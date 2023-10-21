use super::*;
/// This resource represents a custom app.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custom apps create accounts](AccountCustomAppCreateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomApp {
    /// Default listing language in BCP 47 format.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Organizations to which the custom app should be made available. If the request contains any organizations, then the app will be restricted to only these organizations. To support the organization linked to the developer account, the organization ID should be provided explicitly together with other organizations. If no organizations are provided, then the app is only available to the organization linked to the developer account.
    
    pub organizations: Option<Vec<Organization>>,
    /// Output only. Package name of the created Android app. Only present in the API response.
    #[serde(rename="packageName")]
    
    pub package_name: Option<String>,
    /// Title for the Android app.
    
    pub title: Option<String>,
}

impl client::RequestValue for CustomApp {}
impl client::ResponseResult for CustomApp {}


/// Represents an organization that can access a custom app.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Organization {
    /// Required. ID of the organization.
    #[serde(rename="organizationId")]
    
    pub organization_id: Option<String>,
    /// Optional. A human-readable name of the organization, to help recognize the organization.
    #[serde(rename="organizationName")]
    
    pub organization_name: Option<String>,
}

impl client::Part for Organization {}


