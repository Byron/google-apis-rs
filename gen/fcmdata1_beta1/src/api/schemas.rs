use super::*;
/// Message delivery data for a given date, app, and analytics label combination.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseFcmDataV1beta1AndroidDeliveryData {
    /// The analytics label associated with the messages sent. All messages sent without an analytics label will be grouped together in a single entry.
    #[serde(rename="analyticsLabel")]
    
    pub analytics_label: Option<String>,
    /// The app ID to which the messages were sent.
    #[serde(rename="appId")]
    
    pub app_id: Option<String>,
    /// The data for the specified appId, date, and analyticsLabel.
    
    pub data: Option<GoogleFirebaseFcmDataV1beta1Data>,
    /// The date represented by this entry.
    
    pub date: Option<GoogleTypeDate>,
}

impl client::Part for GoogleFirebaseFcmDataV1beta1AndroidDeliveryData {}


/// Data detailing messaging delivery
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseFcmDataV1beta1Data {
    /// Count of messages accepted by FCM intended to Android devices. The targeted device must have opted in to the collection of usage and diagnostic information.
    #[serde(rename="countMessagesAccepted")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count_messages_accepted: Option<i64>,
    /// Additional information about delivery performance for messages that were successfully delivered.
    #[serde(rename="deliveryPerformancePercents")]
    
    pub delivery_performance_percents: Option<GoogleFirebaseFcmDataV1beta1DeliveryPerformancePercents>,
    /// Additional general insights about message delivery.
    #[serde(rename="messageInsightPercents")]
    
    pub message_insight_percents: Option<GoogleFirebaseFcmDataV1beta1MessageInsightPercents>,
    /// Mutually exclusive breakdown of message delivery outcomes.
    #[serde(rename="messageOutcomePercents")]
    
    pub message_outcome_percents: Option<GoogleFirebaseFcmDataV1beta1MessageOutcomePercents>,
}

impl client::Part for GoogleFirebaseFcmDataV1beta1Data {}


/// Overview of delivery performance for messages that were successfully delivered. All percentages are calculated with countMessagesAccepted as the denominator. These categories are not mutually exclusive; a message can be delayed for multiple reasons.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseFcmDataV1beta1DeliveryPerformancePercents {
    /// The percentage of accepted messages that were delayed because the device was in doze mode. Only [normal priority messages](https://firebase.google.com/docs/cloud-messaging/concept-options#setting-the-priority-of-a-message) should be delayed due to doze mode.
    #[serde(rename="delayedDeviceDoze")]
    
    pub delayed_device_doze: Option<f32>,
    /// The percentage of accepted messages that were delayed because the target device was not connected at the time of sending. These messages were eventually delivered when the device reconnected.
    #[serde(rename="delayedDeviceOffline")]
    
    pub delayed_device_offline: Option<f32>,
    /// The percentage of accepted messages that were delayed due to message throttling, such as [collapsible message throttling](https://firebase.google.com/docs/cloud-messaging/concept-options#collapsible_throttling) or [maximum message rate throttling](https://firebase.google.com/docs/cloud-messaging/concept-options#device_throttling).
    #[serde(rename="delayedMessageThrottled")]
    
    pub delayed_message_throttled: Option<f32>,
    /// The percentage of accepted messages that were delayed because the intended device user-profile was [stopped](https://firebase.google.com/docs/cloud-messaging/android/receive#handling_messages) on the target device at the time of the send. The messages were eventually delivered when the user-profile was started again.
    #[serde(rename="delayedUserStopped")]
    
    pub delayed_user_stopped: Option<f32>,
    /// The percentage of accepted messages that were delivered to the device without delay from the FCM system.
    #[serde(rename="deliveredNoDelay")]
    
    pub delivered_no_delay: Option<f32>,
}

impl client::Part for GoogleFirebaseFcmDataV1beta1DeliveryPerformancePercents {}


/// Response message for ListAndroidDeliveryData.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [android apps delivery data list projects](ProjectAndroidAppDeliveryDataListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseFcmDataV1beta1ListAndroidDeliveryDataResponse {
    /// The delivery data for the provided app. There will be one entry per combination of app, date, and analytics label.
    #[serde(rename="androidDeliveryData")]
    
    pub android_delivery_data: Option<Vec<GoogleFirebaseFcmDataV1beta1AndroidDeliveryData>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleFirebaseFcmDataV1beta1ListAndroidDeliveryDataResponse {}


/// Additional information about message delivery. All percentages are calculated with countMessagesAccepted as the denominator.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseFcmDataV1beta1MessageInsightPercents {
    /// The percentage of accepted messages that had their priority lowered from high to normal. See [documentation for setting message priority](https://firebase.google.com/docs/cloud-messaging/android/message-priority).
    #[serde(rename="priorityLowered")]
    
    pub priority_lowered: Option<f32>,
}

impl client::Part for GoogleFirebaseFcmDataV1beta1MessageInsightPercents {}


/// Percentage breakdown of message delivery outcomes. These categories are mutually exclusive. All percentages are calculated with countMessagesAccepted as the denominator. These categories may not account for all message outcomes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseFcmDataV1beta1MessageOutcomePercents {
    /// The percentage of all accepted messages that were successfully delivered to the device.
    
    pub delivered: Option<f32>,
    /// The percentage of accepted messages that were dropped because the application was force stopped on the device at the time of delivery and retries were unsuccessful.
    #[serde(rename="droppedAppForceStopped")]
    
    pub dropped_app_force_stopped: Option<f32>,
    /// The percentage of accepted messages that were dropped because the target device is inactive. FCM will drop messages if the target device is deemed inactive by our servers. If a device does reconnect, we call [OnDeletedMessages()](https://firebase.google.com/docs/cloud-messaging/android/receive#override-ondeletedmessages) in our SDK instead of delivering the messages.
    #[serde(rename="droppedDeviceInactive")]
    
    pub dropped_device_inactive: Option<f32>,
    /// The percentage of accepted messages that were dropped due to [too many undelivered non-collapsible messages](https://firebase.google.com/docs/cloud-messaging/concept-options#collapsible_and_non-collapsible_messages). Specifically, each app instance can only have 100 pending messages stored on our servers for a device which is disconnected. When that device reconnects, those messages are delivered. When there are more than the maximum pending messages, we call [OnDeletedMessages()](https://firebase.google.com/docs/cloud-messaging/android/receive#override-ondeletedmessages) in our SDK instead of delivering the messages.
    #[serde(rename="droppedTooManyPendingMessages")]
    
    pub dropped_too_many_pending_messages: Option<f32>,
    /// The percentage of messages accepted on this day that were not dropped and not delivered, due to the device being disconnected (as of the end of the America/Los_Angeles day when the message was sent to FCM). A portion of these messages will be delivered the next day when the device connects but others may be destined to devices that ultimately never reconnect.
    
    pub pending: Option<f32>,
}

impl client::Part for GoogleFirebaseFcmDataV1beta1MessageOutcomePercents {}


/// Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values. * A month and day, with a zero year (for example, an anniversary). * A year on its own, with a zero month and a zero day. * A year and month, with a zero day (for example, a credit card expiration date). Related types: * google.type.TimeOfDay * google.type.DateTime * google.protobuf.Timestamp
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeDate {
    /// Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant.
    
    pub day: Option<i32>,
    /// Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day.
    
    pub month: Option<i32>,
    /// Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year.
    
    pub year: Option<i32>,
}

impl client::Part for GoogleTypeDate {}


