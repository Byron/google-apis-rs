use super::*;
/// Output for PublishUrlNotification
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [publish url notifications](UrlNotificationPublishCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PublishUrlNotificationResponse {
    /// Description of the notification events received for this URL.
    #[serde(rename="urlNotificationMetadata")]
    
    pub url_notification_metadata: Option<UrlNotificationMetadata>,
}

impl client::ResponseResult for PublishUrlNotificationResponse {}


/// `UrlNotification` is the resource used in all Indexing API calls. It describes one event in the life cycle of a Web Document.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get metadata url notifications](UrlNotificationGetMetadataCall) (none)
/// * [publish url notifications](UrlNotificationPublishCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UrlNotification {
    /// Creation timestamp for this notification. Users should _not_ specify it, the field is ignored at the request time.
    #[serde(rename="notifyTime")]
    
    pub notify_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The URL life cycle event that Google is being notified about.
    #[serde(rename="type")]
    
    pub type_: Option<UrlNotificationTypeEnum>,
    /// The object of this notification. The URL must be owned by the publisher of this notification and, in case of `URL_UPDATED` notifications, it _must_ be crawlable by Google.
    
    pub url: Option<String>,
}

impl client::RequestValue for UrlNotification {}
impl client::Resource for UrlNotification {}


/// Summary of the most recent Indexing API notifications successfully received, for a given URL.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get metadata url notifications](UrlNotificationGetMetadataCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UrlNotificationMetadata {
    /// Latest notification received with type `URL_REMOVED`.
    #[serde(rename="latestRemove")]
    
    pub latest_remove: Option<UrlNotification>,
    /// Latest notification received with type `URL_UPDATED`.
    #[serde(rename="latestUpdate")]
    
    pub latest_update: Option<UrlNotification>,
    /// URL to which this metadata refers.
    
    pub url: Option<String>,
}

impl client::ResponseResult for UrlNotificationMetadata {}


