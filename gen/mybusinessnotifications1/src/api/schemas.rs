use super::*;
/// A Google Pub/Sub topic where notifications can be published when a location is updated or has a new review. There will be only one notification setting resource per-account.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get notification setting accounts](AccountGetNotificationSettingCall) (response)
/// * [update notification setting accounts](AccountUpdateNotificationSettingCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NotificationSetting {
    /// Required. The resource name this setting is for. This is of the form `accounts/{account_id}/notificationSetting`.
    
    pub name: Option<String>,
    /// The types of notifications that will be sent to the Pub/Sub topic. To stop receiving notifications entirely, use NotificationSettings.UpdateNotificationSetting with an empty notification_types or set the pubsub_topic to an empty string.
    #[serde(rename="notificationTypes")]
    
    pub notification_types: Option<Vec<NotificationSettingNotificationTypesEnum>>,
    /// Optional. The Google Pub/Sub topic that will receive notifications when locations managed by this account are updated. If unset, no notifications will be posted. The account mybusiness-api-pubsub@system.gserviceaccount.com must have at least Publish permissions on the Pub/Sub topic.
    #[serde(rename="pubsubTopic")]
    
    pub pubsub_topic: Option<String>,
}

impl client::RequestValue for NotificationSetting {}
impl client::ResponseResult for NotificationSetting {}


