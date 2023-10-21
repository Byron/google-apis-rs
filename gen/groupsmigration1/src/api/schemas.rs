use super::*;
/// JSON response template for groups migration API.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insert archive](ArchiveInsertCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Groups {
    /// The kind of insert resource this is.
    
    pub kind: Option<String>,
    /// The status of the insert request.
    #[serde(rename="responseCode")]
    
    pub response_code: Option<String>,
}

impl client::ResponseResult for Groups {}


