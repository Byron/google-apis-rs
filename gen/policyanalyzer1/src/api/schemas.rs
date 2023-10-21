use super::*;
/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPolicyanalyzerV1Activity {
    /// A struct of custom fields to explain the activity.
    
    pub activity: Option<HashMap<String, json::Value>>,
    /// The type of the activity.
    #[serde(rename="activityType")]
    
    pub activity_type: Option<String>,
    /// The full resource name that identifies the resource. For examples of full resource names for Google Cloud services, see https://cloud.google.com/iam/help/troubleshooter/full-resource-names.
    #[serde(rename="fullResourceName")]
    
    pub full_resource_name: Option<String>,
    /// The data observation period to build the activity.
    #[serde(rename="observationPeriod")]
    
    pub observation_period: Option<GoogleCloudPolicyanalyzerV1ObservationPeriod>,
}

impl client::Part for GoogleCloudPolicyanalyzerV1Activity {}


/// Represents data observation period.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPolicyanalyzerV1ObservationPeriod {
    /// The observation end time. The time in this timestamp is always `07:00:00Z`.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The observation start time. The time in this timestamp is always `07:00:00Z`.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GoogleCloudPolicyanalyzerV1ObservationPeriod {}


/// Response to the `QueryActivity` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations activity types activities query projects](ProjectLocationActivityTypeActivityQueryCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPolicyanalyzerV1QueryActivityResponse {
    /// The set of activities that match the filter included in the request.
    
    pub activities: Option<Vec<GoogleCloudPolicyanalyzerV1Activity>>,
    /// If there might be more results than those appearing in this response, then `nextPageToken` is included. To get the next set of results, call this method again using the value of `nextPageToken` as `pageToken`.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudPolicyanalyzerV1QueryActivityResponse {}


