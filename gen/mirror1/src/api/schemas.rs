use super::*;
/// Represents an account passed into the Account Manager on Glass.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insert accounts](AccountInsertCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    /// no description provided
    #[serde(rename="authTokens")]
    
    pub auth_tokens: Option<Vec<AuthToken>>,
    /// no description provided
    
    pub features: Option<Vec<String>>,
    /// no description provided
    
    pub password: Option<String>,
    /// no description provided
    #[serde(rename="userData")]
    
    pub user_data: Option<Vec<UserData>>,
}

impl client::RequestValue for Account {}
impl client::Resource for Account {}
impl client::ResponseResult for Account {}


/// Represents media content, such as a photo, that can be attached to a timeline item.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [attachments get timeline](TimelineAttachmentGetCall) (response)
/// * [attachments insert timeline](TimelineAttachmentInsertCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Attachment {
    /// The MIME type of the attachment.
    #[serde(rename="contentType")]
    
    pub content_type: Option<String>,
    /// The URL for the content.
    #[serde(rename="contentUrl")]
    
    pub content_url: Option<String>,
    /// The ID of the attachment.
    
    pub id: Option<String>,
    /// Indicates that the contentUrl is not available because the attachment content is still being processed. If the caller wishes to retrieve the content, it should try again later.
    #[serde(rename="isProcessingContent")]
    
    pub is_processing_content: Option<bool>,
}

impl client::ResponseResult for Attachment {}


/// A list of Attachments. This is the response from the server to GET requests on the attachments collection.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [attachments list timeline](TimelineAttachmentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AttachmentsListResponse {
    /// The list of attachments.
    
    pub items: Option<Vec<Attachment>>,
    /// The type of resource. This is always mirror#attachmentsList.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for AttachmentsListResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuthToken {
    /// no description provided
    #[serde(rename="authToken")]
    
    pub auth_token: Option<String>,
    /// no description provided
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for AuthToken {}


/// A single menu command that is part of a Contact.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Command {
    /// The type of operation this command corresponds to. Allowed values are:  
    /// - TAKE_A_NOTE - Shares a timeline item with the transcription of user speech from the "Take a note" voice menu command.  
    /// - POST_AN_UPDATE - Shares a timeline item with the transcription of user speech from the "Post an update" voice menu command.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for Command {}


/// A person or group that can be used as a creator or a contact.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete contacts](ContactDeleteCall) (none)
/// * [get contacts](ContactGetCall) (response)
/// * [insert contacts](ContactInsertCall) (request|response)
/// * [list contacts](ContactListCall) (none)
/// * [patch contacts](ContactPatchCall) (request|response)
/// * [update contacts](ContactUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Contact {
    /// A list of voice menu commands that a contact can handle. Glass shows up to three contacts for each voice menu command. If there are more than that, the three contacts with the highest priority are shown for that particular command.
    #[serde(rename="acceptCommands")]
    
    pub accept_commands: Option<Vec<Command>>,
    /// A list of MIME types that a contact supports. The contact will be shown to the user if any of its acceptTypes matches any of the types of the attachments on the item. If no acceptTypes are given, the contact will be shown for all items.
    #[serde(rename="acceptTypes")]
    
    pub accept_types: Option<Vec<String>>,
    /// The name to display for this contact.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// An ID for this contact. This is generated by the application and is treated as an opaque token.
    
    pub id: Option<String>,
    /// Set of image URLs to display for a contact. Most contacts will have a single image, but a "group" contact may include up to 8 image URLs and they will be resized and cropped into a mosaic on the client.
    #[serde(rename="imageUrls")]
    
    pub image_urls: Option<Vec<String>>,
    /// The type of resource. This is always mirror#contact.
    
    pub kind: Option<String>,
    /// Primary phone number for the contact. This can be a fully-qualified number, with country calling code and area code, or a local number.
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<String>,
    /// Priority for the contact to determine ordering in a list of contacts. Contacts with higher priorities will be shown before ones with lower priorities.
    
    pub priority: Option<u32>,
    /// A list of sharing features that a contact can handle. Allowed values are:  
    /// - ADD_CAPTION
    #[serde(rename="sharingFeatures")]
    
    pub sharing_features: Option<Vec<String>>,
    /// The ID of the application that created this contact. This is populated by the API
    
    pub source: Option<String>,
    /// Name of this contact as it should be pronounced. If this contact's name must be spoken as part of a voice disambiguation menu, this name is used as the expected pronunciation. This is useful for contact names with unpronounceable characters or whose display spelling is otherwise not phonetic.
    #[serde(rename="speakableName")]
    
    pub speakable_name: Option<String>,
    /// The type for this contact. This is used for sorting in UIs. Allowed values are:  
    /// - INDIVIDUAL - Represents a single person. This is the default. 
    /// - GROUP - Represents more than a single person.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::RequestValue for Contact {}
impl client::Resource for Contact {}
impl client::ResponseResult for Contact {}


/// A list of Contacts representing contacts. This is the response from the server to GET requests on the contacts collection.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list contacts](ContactListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContactsListResponse {
    /// Contact list.
    
    pub items: Option<Vec<Contact>>,
    /// The type of resource. This is always mirror#contacts.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for ContactsListResponse {}


/// A geographic location that can be associated with a timeline item.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get locations](LocationGetCall) (response)
/// * [list locations](LocationListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    /// The accuracy of the location fix in meters.
    
    pub accuracy: Option<f64>,
    /// The full address of the location.
    
    pub address: Option<String>,
    /// The name to be displayed. This may be a business name or a user-defined place, such as "Home".
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The ID of the location.
    
    pub id: Option<String>,
    /// The type of resource. This is always mirror#location.
    
    pub kind: Option<String>,
    /// The latitude, in degrees.
    
    pub latitude: Option<f64>,
    /// The longitude, in degrees.
    
    pub longitude: Option<f64>,
    /// The time at which this location was captured, formatted according to RFC 3339.
    
    pub timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Resource for Location {}
impl client::ResponseResult for Location {}


/// A list of Locations. This is the response from the server to GET requests on the locations collection.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list locations](LocationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocationsListResponse {
    /// The list of locations.
    
    pub items: Option<Vec<Location>>,
    /// The type of resource. This is always mirror#locationsList.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for LocationsListResponse {}


/// A custom menu item that can be presented to the user by a timeline item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MenuItem {
    /// Controls the behavior when the user picks the menu option. Allowed values are:  
    /// - CUSTOM - Custom action set by the service. When the user selects this menuItem, the API triggers a notification to your callbackUrl with the userActions.type set to CUSTOM and the userActions.payload set to the ID of this menu item. This is the default value. 
    /// - Built-in actions:  
    /// - REPLY - Initiate a reply to the timeline item using the voice recording UI. The creator attribute must be set in the timeline item for this menu to be available. 
    /// - REPLY_ALL - Same behavior as REPLY. The original timeline item's recipients will be added to the reply item. 
    /// - DELETE - Delete the timeline item. 
    /// - SHARE - Share the timeline item with the available contacts. 
    /// - READ_ALOUD - Read the timeline item's speakableText aloud; if this field is not set, read the text field; if none of those fields are set, this menu item is ignored. 
    /// - GET_MEDIA_INPUT - Allow users to provide media payloads to Glassware from a menu item (currently, only transcribed text from voice input is supported). Subscribe to notifications when users invoke this menu item to receive the timeline item ID. Retrieve the media from the timeline item in the payload property. 
    /// - VOICE_CALL - Initiate a phone call using the timeline item's creator.phoneNumber attribute as recipient. 
    /// - NAVIGATE - Navigate to the timeline item's location. 
    /// - TOGGLE_PINNED - Toggle the isPinned state of the timeline item. 
    /// - OPEN_URI - Open the payload of the menu item in the browser. 
    /// - PLAY_VIDEO - Open the payload of the menu item in the Glass video player. 
    /// - SEND_MESSAGE - Initiate sending a message to the timeline item's creator:  
    /// - If the creator.phoneNumber is set and Glass is connected to an Android phone, the message is an SMS. 
    /// - Otherwise, if the creator.email is set, the message is an email.
    
    pub action: Option<String>,
    /// The ContextualMenus.Command associated with this MenuItem (e.g. READ_ALOUD). The voice label for this command will be displayed in the voice menu and the touch label will be displayed in the touch menu. Note that the default menu value's display name will be overriden if you specify this property. Values that do not correspond to a ContextualMenus.Command name will be ignored.
    
    pub contextual_command: Option<String>,
    /// The ID for this menu item. This is generated by the application and is treated as an opaque token.
    
    pub id: Option<String>,
    /// A generic payload whose meaning changes depending on this MenuItem's action.  
    /// - When the action is OPEN_URI, the payload is the URL of the website to view. 
    /// - When the action is PLAY_VIDEO, the payload is the streaming URL of the video 
    /// - When the action is GET_MEDIA_INPUT, the payload is the text transcription of a user's speech input
    
    pub payload: Option<String>,
    /// If set to true on a CUSTOM menu item, that item will be removed from the menu after it is selected.
    #[serde(rename="removeWhenSelected")]
    
    pub remove_when_selected: Option<bool>,
    /// For CUSTOM items, a list of values controlling the appearance of the menu item in each of its states. A value for the DEFAULT state must be provided. If the PENDING or CONFIRMED states are missing, they will not be shown.
    
    pub values: Option<Vec<MenuValue>>,
}

impl client::Part for MenuItem {}


/// A single value that is part of a MenuItem.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MenuValue {
    /// The name to display for the menu item. If you specify this property for a built-in menu item, the default contextual voice command for that menu item is not shown.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// URL of an icon to display with the menu item.
    #[serde(rename="iconUrl")]
    
    pub icon_url: Option<String>,
    /// The state that this value applies to. Allowed values are:  
    /// - DEFAULT - Default value shown when displayed in the menuItems list. 
    /// - PENDING - Value shown when the menuItem has been selected by the user but can still be cancelled. 
    /// - CONFIRMED - Value shown when the menuItem has been selected by the user and can no longer be cancelled.
    
    pub state: Option<String>,
}

impl client::Part for MenuValue {}


/// A notification delivered by the API.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Notification {
    /// The collection that generated the notification.
    
    pub collection: Option<String>,
    /// The ID of the item that generated the notification.
    #[serde(rename="itemId")]
    
    pub item_id: Option<String>,
    /// The type of operation that generated the notification.
    
    pub operation: Option<String>,
    /// A list of actions taken by the user that triggered the notification.
    #[serde(rename="userActions")]
    
    pub user_actions: Option<Vec<UserAction>>,
    /// The user token provided by the service when it subscribed for notifications.
    #[serde(rename="userToken")]
    
    pub user_token: Option<String>,
    /// The secret verify token provided by the service when it subscribed for notifications.
    #[serde(rename="verifyToken")]
    
    pub verify_token: Option<String>,
}

impl client::Part for Notification {}


/// Controls how notifications for a timeline item are presented to the user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NotificationConfig {
    /// The time at which the notification should be delivered.
    #[serde(rename="deliveryTime")]
    
    pub delivery_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Describes how important the notification is. Allowed values are:  
    /// - DEFAULT - Notifications of default importance. A chime will be played to alert users.
    
    pub level: Option<String>,
}

impl client::Part for NotificationConfig {}


/// A setting for Glass.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get settings](SettingGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Setting {
    /// The setting's ID. The following IDs are valid:  
    /// - locale - The key to the user’s language/locale (BCP 47 identifier) that Glassware should use to render localized content.  
    /// - timezone - The key to the user’s current time zone region as defined in the tz database. Example: America/Los_Angeles.
    
    pub id: Option<String>,
    /// The type of resource. This is always mirror#setting.
    
    pub kind: Option<String>,
    /// The setting value, as a string.
    
    pub value: Option<String>,
}

impl client::Resource for Setting {}
impl client::ResponseResult for Setting {}


/// A subscription to events on a collection.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete subscriptions](SubscriptionDeleteCall) (none)
/// * [insert subscriptions](SubscriptionInsertCall) (request|response)
/// * [list subscriptions](SubscriptionListCall) (none)
/// * [update subscriptions](SubscriptionUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Subscription {
    /// The URL where notifications should be delivered (must start with https://).
    #[serde(rename="callbackUrl")]
    
    pub callback_url: Option<String>,
    /// The collection to subscribe to. Allowed values are:  
    /// - timeline - Changes in the timeline including insertion, deletion, and updates. 
    /// - locations - Location updates. 
    /// - settings - Settings updates.
    
    pub collection: Option<String>,
    /// The ID of the subscription.
    
    pub id: Option<String>,
    /// The type of resource. This is always mirror#subscription.
    
    pub kind: Option<String>,
    /// Container object for notifications. This is not populated in the Subscription resource.
    
    pub notification: Option<Notification>,
    /// A list of operations that should be subscribed to. An empty list indicates that all operations on the collection should be subscribed to. Allowed values are:  
    /// - UPDATE - The item has been updated. 
    /// - INSERT - A new item has been inserted. 
    /// - DELETE - The item has been deleted. 
    /// - MENU_ACTION - A custom menu item has been triggered by the user.
    
    pub operation: Option<Vec<String>>,
    /// The time at which this subscription was last modified, formatted according to RFC 3339.
    
    pub updated: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// An opaque token sent to the subscriber in notifications so that it can determine the ID of the user.
    #[serde(rename="userToken")]
    
    pub user_token: Option<String>,
    /// A secret token sent to the subscriber in notifications so that it can verify that the notification was generated by Google.
    #[serde(rename="verifyToken")]
    
    pub verify_token: Option<String>,
}

impl client::RequestValue for Subscription {}
impl client::Resource for Subscription {}
impl client::ResponseResult for Subscription {}


/// A list of Subscriptions. This is the response from the server to GET requests on the subscription collection.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list subscriptions](SubscriptionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionsListResponse {
    /// The list of subscriptions.
    
    pub items: Option<Vec<Subscription>>,
    /// The type of resource. This is always mirror#subscriptionsList.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for SubscriptionsListResponse {}


/// Each item in the user’s timeline is represented as a TimelineItem JSON structure, described below.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get timeline](TimelineGetCall) (response)
/// * [insert timeline](TimelineInsertCall) (request|response)
/// * [patch timeline](TimelinePatchCall) (request|response)
/// * [update timeline](TimelineUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimelineItem {
    /// A list of media attachments associated with this item. As a convenience, you can refer to attachments in your HTML payloads with the attachment or cid scheme. For example:  
    /// - attachment: <img src="attachment:attachment_index"> where attachment_index is the 0-based index of this array. 
    /// - cid: <img src="cid:attachment_id"> where attachment_id is the ID of the attachment.
    
    pub attachments: Option<Vec<Attachment>>,
    /// The bundle ID for this item. Services can specify a bundleId to group many items together. They appear under a single top-level item on the device.
    #[serde(rename="bundleId")]
    
    pub bundle_id: Option<String>,
    /// A canonical URL pointing to the canonical/high quality version of the data represented by the timeline item.
    #[serde(rename="canonicalUrl")]
    
    pub canonical_url: Option<String>,
    /// The time at which this item was created, formatted according to RFC 3339.
    
    pub created: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The user or group that created this item.
    
    pub creator: Option<Contact>,
    /// The time that should be displayed when this item is viewed in the timeline, formatted according to RFC 3339. This user's timeline is sorted chronologically on display time, so this will also determine where the item is displayed in the timeline. If not set by the service, the display time defaults to the updated time.
    #[serde(rename="displayTime")]
    
    pub display_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// ETag for this item.
    
    pub etag: Option<String>,
    /// HTML content for this item. If both text and html are provided for an item, the html will be rendered in the timeline.
    /// Allowed HTML elements - You can use these elements in your timeline cards.
    ///  
    /// - Headers: h1, h2, h3, h4, h5, h6 
    /// - Images: img 
    /// - Lists: li, ol, ul 
    /// - HTML5 semantics: article, aside, details, figure, figcaption, footer, header, nav, section, summary, time 
    /// - Structural: blockquote, br, div, hr, p, span 
    /// - Style: b, big, center, em, i, u, s, small, strike, strong, style, sub, sup 
    /// - Tables: table, tbody, td, tfoot, th, thead, tr  
    /// Blocked HTML elements: These elements and their contents are removed from HTML payloads.
    ///  
    /// - Document headers: head, title 
    /// - Embeds: audio, embed, object, source, video 
    /// - Frames: frame, frameset 
    /// - Scripting: applet, script  
    /// Other elements: Any elements that aren't listed are removed, but their contents are preserved.
    
    pub html: Option<String>,
    /// The ID of the timeline item. This is unique within a user's timeline.
    
    pub id: Option<String>,
    /// If this item was generated as a reply to another item, this field will be set to the ID of the item being replied to. This can be used to attach a reply to the appropriate conversation or post.
    #[serde(rename="inReplyTo")]
    
    pub in_reply_to: Option<String>,
    /// Whether this item is a bundle cover.
    /// 
    /// If an item is marked as a bundle cover, it will be the entry point to the bundle of items that have the same bundleId as that item. It will be shown only on the main timeline — not within the opened bundle.
    /// 
    /// On the main timeline, items that are shown are:  
    /// - Items that have isBundleCover set to true  
    /// - Items that do not have a bundleId  In a bundle sub-timeline, items that are shown are:  
    /// - Items that have the bundleId in question AND isBundleCover set to false
    #[serde(rename="isBundleCover")]
    
    pub is_bundle_cover: Option<bool>,
    /// When true, indicates this item is deleted, and only the ID property is set.
    #[serde(rename="isDeleted")]
    
    pub is_deleted: Option<bool>,
    /// When true, indicates this item is pinned, which means it's grouped alongside "active" items like navigation and hangouts, on the opposite side of the home screen from historical (non-pinned) timeline items. You can allow the user to toggle the value of this property with the TOGGLE_PINNED built-in menu item.
    #[serde(rename="isPinned")]
    
    pub is_pinned: Option<bool>,
    /// The type of resource. This is always mirror#timelineItem.
    
    pub kind: Option<String>,
    /// The geographic location associated with this item.
    
    pub location: Option<Location>,
    /// A list of menu items that will be presented to the user when this item is selected in the timeline.
    #[serde(rename="menuItems")]
    
    pub menu_items: Option<Vec<MenuItem>>,
    /// Controls how notifications for this item are presented on the device. If this is missing, no notification will be generated.
    
    pub notification: Option<NotificationConfig>,
    /// For pinned items, this determines the order in which the item is displayed in the timeline, with a higher score appearing closer to the clock. Note: setting this field is currently not supported.
    #[serde(rename="pinScore")]
    
    pub pin_score: Option<i32>,
    /// A list of users or groups that this item has been shared with.
    
    pub recipients: Option<Vec<Contact>>,
    /// A URL that can be used to retrieve this item.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Opaque string you can use to map a timeline item to data in your own service.
    #[serde(rename="sourceItemId")]
    
    pub source_item_id: Option<String>,
    /// The speakable version of the content of this item. Along with the READ_ALOUD menu item, use this field to provide text that would be clearer when read aloud, or to provide extended information to what is displayed visually on Glass.
    /// 
    /// Glassware should also specify the speakableType field, which will be spoken before this text in cases where the additional context is useful, for example when the user requests that the item be read aloud following a notification.
    #[serde(rename="speakableText")]
    
    pub speakable_text: Option<String>,
    /// A speakable description of the type of this item. This will be announced to the user prior to reading the content of the item in cases where the additional context is useful, for example when the user requests that the item be read aloud following a notification.
    /// 
    /// This should be a short, simple noun phrase such as "Email", "Text message", or "Daily Planet News Update".
    /// 
    /// Glassware are encouraged to populate this field for every timeline item, even if the item does not contain speakableText or text so that the user can learn the type of the item without looking at the screen.
    #[serde(rename="speakableType")]
    
    pub speakable_type: Option<String>,
    /// Text content of this item.
    
    pub text: Option<String>,
    /// The title of this item.
    
    pub title: Option<String>,
    /// The time at which this item was last modified, formatted according to RFC 3339.
    
    pub updated: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for TimelineItem {}
impl client::ResponseResult for TimelineItem {}


/// A list of timeline items. This is the response from the server to GET requests on the timeline collection.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list timeline](TimelineListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimelineListResponse {
    /// Items in the timeline.
    
    pub items: Option<Vec<TimelineItem>>,
    /// The type of resource. This is always mirror#timeline.
    
    pub kind: Option<String>,
    /// The next page token. Provide this as the pageToken parameter in the request to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for TimelineListResponse {}


/// Represents an action taken by the user that triggered a notification.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserAction {
    /// An optional payload for the action.
    /// 
    /// For actions of type CUSTOM, this is the ID of the custom menu item that was selected.
    
    pub payload: Option<String>,
    /// The type of action. The value of this can be:  
    /// - SHARE - the user shared an item. 
    /// - REPLY - the user replied to an item. 
    /// - REPLY_ALL - the user replied to all recipients of an item. 
    /// - CUSTOM - the user selected a custom menu item on the timeline item. 
    /// - DELETE - the user deleted the item. 
    /// - PIN - the user pinned the item. 
    /// - UNPIN - the user unpinned the item. 
    /// - LAUNCH - the user initiated a voice command.  In the future, additional types may be added. UserActions with unrecognized types should be ignored.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for UserAction {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserData {
    /// no description provided
    
    pub key: Option<String>,
    /// no description provided
    
    pub value: Option<String>,
}

impl client::Part for UserData {}


