use super::*;
/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list acl](AclListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Acl {
    /// ETag of the collection.
    
    pub etag: Option<String>,
    /// List of rules on the access control list.
    
    pub items: Option<Vec<AclRule>>,
    /// Type of the collection ("calendar#acl").
    
    pub kind: Option<String>,
    /// Token used to access the next page of this result. Omitted if no further results are available, in which case nextSyncToken is provided.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Token used at a later point in time to retrieve only the entries that have changed since this result was returned. Omitted if further results are available, in which case nextPageToken is provided.
    #[serde(rename="nextSyncToken")]
    
    pub next_sync_token: Option<String>,
}

impl client::ResponseResult for Acl {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get acl](AclGetCall) (response)
/// * [insert acl](AclInsertCall) (request|response)
/// * [patch acl](AclPatchCall) (request|response)
/// * [update acl](AclUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AclRule {
    /// ETag of the resource.
    
    pub etag: Option<String>,
    /// Identifier of the Access Control List (ACL) rule. See Sharing calendars.
    
    pub id: Option<String>,
    /// Type of the resource ("calendar#aclRule").
    
    pub kind: Option<String>,
    /// The role assigned to the scope. Possible values are:  
    /// - "none" - Provides no access. 
    /// - "freeBusyReader" - Provides read access to free/busy information. 
    /// - "reader" - Provides read access to the calendar. Private events will appear to users with reader access, but event details will be hidden. 
    /// - "writer" - Provides read and write access to the calendar. Private events will appear to users with writer access, and event details will be visible. 
    /// - "owner" - Provides ownership of the calendar. This role has all of the permissions of the writer role with the additional ability to see and manipulate ACLs.
    
    pub role: Option<String>,
    /// The extent to which calendar access is granted by this ACL rule.
    
    pub scope: Option<AclRuleScope>,
}

impl client::RequestValue for AclRule {}
impl client::ResponseResult for AclRule {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [clear calendars](CalendarClearCall) (none)
/// * [delete calendars](CalendarDeleteCall) (none)
/// * [get calendars](CalendarGetCall) (response)
/// * [insert calendars](CalendarInsertCall) (request|response)
/// * [patch calendars](CalendarPatchCall) (request|response)
/// * [update calendars](CalendarUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Calendar {
    /// Conferencing properties for this calendar, for example what types of conferences are allowed.
    #[serde(rename="conferenceProperties")]
    
    pub conference_properties: Option<ConferenceProperties>,
    /// Description of the calendar. Optional.
    
    pub description: Option<String>,
    /// ETag of the resource.
    
    pub etag: Option<String>,
    /// Identifier of the calendar. To retrieve IDs call the calendarList.list() method.
    
    pub id: Option<String>,
    /// Type of the resource ("calendar#calendar").
    
    pub kind: Option<String>,
    /// Geographic location of the calendar as free-form text. Optional.
    
    pub location: Option<String>,
    /// Title of the calendar.
    
    pub summary: Option<String>,
    /// The time zone of the calendar. (Formatted as an IANA Time Zone Database name, e.g. "Europe/Zurich".) Optional.
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<String>,
}

impl client::RequestValue for Calendar {}
impl client::Resource for Calendar {}
impl client::ResponseResult for Calendar {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list calendar list](CalendarListListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CalendarList {
    /// ETag of the collection.
    
    pub etag: Option<String>,
    /// Calendars that are present on the user's calendar list.
    
    pub items: Option<Vec<CalendarListEntry>>,
    /// Type of the collection ("calendar#calendarList").
    
    pub kind: Option<String>,
    /// Token used to access the next page of this result. Omitted if no further results are available, in which case nextSyncToken is provided.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Token used at a later point in time to retrieve only the entries that have changed since this result was returned. Omitted if further results are available, in which case nextPageToken is provided.
    #[serde(rename="nextSyncToken")]
    
    pub next_sync_token: Option<String>,
}

impl client::ResponseResult for CalendarList {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get calendar list](CalendarListGetCall) (response)
/// * [insert calendar list](CalendarListInsertCall) (request|response)
/// * [patch calendar list](CalendarListPatchCall) (request|response)
/// * [update calendar list](CalendarListUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CalendarListEntry {
    /// The effective access role that the authenticated user has on the calendar. Read-only. Possible values are:  
    /// - "freeBusyReader" - Provides read access to free/busy information. 
    /// - "reader" - Provides read access to the calendar. Private events will appear to users with reader access, but event details will be hidden. 
    /// - "writer" - Provides read and write access to the calendar. Private events will appear to users with writer access, and event details will be visible. 
    /// - "owner" - Provides ownership of the calendar. This role has all of the permissions of the writer role with the additional ability to see and manipulate ACLs.
    #[serde(rename="accessRole")]
    
    pub access_role: Option<String>,
    /// The main color of the calendar in the hexadecimal format "#0088aa". This property supersedes the index-based colorId property. To set or change this property, you need to specify colorRgbFormat=true in the parameters of the insert, update and patch methods. Optional.
    #[serde(rename="backgroundColor")]
    
    pub background_color: Option<String>,
    /// The color of the calendar. This is an ID referring to an entry in the calendar section of the colors definition (see the colors endpoint). This property is superseded by the backgroundColor and foregroundColor properties and can be ignored when using these properties. Optional.
    #[serde(rename="colorId")]
    
    pub color_id: Option<String>,
    /// Conferencing properties for this calendar, for example what types of conferences are allowed.
    #[serde(rename="conferenceProperties")]
    
    pub conference_properties: Option<ConferenceProperties>,
    /// The default reminders that the authenticated user has for this calendar.
    #[serde(rename="defaultReminders")]
    
    pub default_reminders: Option<Vec<EventReminder>>,
    /// Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.
    
    pub deleted: Option<bool>,
    /// Description of the calendar. Optional. Read-only.
    
    pub description: Option<String>,
    /// ETag of the resource.
    
    pub etag: Option<String>,
    /// The foreground color of the calendar in the hexadecimal format "#ffffff". This property supersedes the index-based colorId property. To set or change this property, you need to specify colorRgbFormat=true in the parameters of the insert, update and patch methods. Optional.
    #[serde(rename="foregroundColor")]
    
    pub foreground_color: Option<String>,
    /// Whether the calendar has been hidden from the list. Optional. The attribute is only returned when the calendar is hidden, in which case the value is true.
    
    pub hidden: Option<bool>,
    /// Identifier of the calendar.
    
    pub id: Option<String>,
    /// Type of the resource ("calendar#calendarListEntry").
    
    pub kind: Option<String>,
    /// Geographic location of the calendar as free-form text. Optional. Read-only.
    
    pub location: Option<String>,
    /// The notifications that the authenticated user is receiving for this calendar.
    #[serde(rename="notificationSettings")]
    
    pub notification_settings: Option<CalendarListEntryNotificationSettings>,
    /// Whether the calendar is the primary calendar of the authenticated user. Read-only. Optional. The default is False.
    
    pub primary: Option<bool>,
    /// Whether the calendar content shows up in the calendar UI. Optional. The default is False.
    
    pub selected: Option<bool>,
    /// Title of the calendar. Read-only.
    
    pub summary: Option<String>,
    /// The summary that the authenticated user has set for this calendar. Optional.
    #[serde(rename="summaryOverride")]
    
    pub summary_override: Option<String>,
    /// The time zone of the calendar. Optional. Read-only.
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<String>,
}

impl client::RequestValue for CalendarListEntry {}
impl client::ResponseResult for CalendarListEntry {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CalendarNotification {
    /// The method used to deliver the notification. The possible value is:  
    /// - "email" - Notifications are sent via email.  
    /// Required when adding a notification.
    
    pub method: Option<String>,
    /// The type of notification. Possible values are:  
    /// - "eventCreation" - Notification sent when a new event is put on the calendar. 
    /// - "eventChange" - Notification sent when an event is changed. 
    /// - "eventCancellation" - Notification sent when an event is cancelled. 
    /// - "eventResponse" - Notification sent when an attendee responds to the event invitation. 
    /// - "agenda" - An agenda with the events of the day (sent out in the morning).  
    /// Required when adding a notification.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for CalendarNotification {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [watch acl](AclWatchCall) (request|response)
/// * [watch calendar list](CalendarListWatchCall) (request|response)
/// * [stop channels](ChannelStopCall) (request)
/// * [watch events](EventWatchCall) (request|response)
/// * [watch settings](SettingWatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Channel {
    /// The address where notifications are delivered for this channel.
    
    pub address: Option<String>,
    /// Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub expiration: Option<i64>,
    /// A UUID or similar unique string that identifies this channel.
    
    pub id: Option<String>,
    /// Identifies this as a notification channel used to watch for changes to a resource, which is "api#channel".
    
    pub kind: Option<String>,
    /// Additional parameters controlling delivery channel behavior. Optional.
    
    pub params: Option<HashMap<String, String>>,
    /// A Boolean value to indicate whether payload is wanted. Optional.
    
    pub payload: Option<bool>,
    /// An opaque ID that identifies the resource being watched on this channel. Stable across different API versions.
    #[serde(rename="resourceId")]
    
    pub resource_id: Option<String>,
    /// A version-specific identifier for the watched resource.
    #[serde(rename="resourceUri")]
    
    pub resource_uri: Option<String>,
    /// An arbitrary string delivered to the target address with each notification delivered over this channel. Optional.
    
    pub token: Option<String>,
    /// The type of delivery mechanism used for this channel. Valid values are "web_hook" (or "webhook"). Both values refer to a channel where Http requests are used to deliver messages.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::RequestValue for Channel {}
impl client::Resource for Channel {}
impl client::ResponseResult for Channel {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ColorDefinition {
    /// The background color associated with this color definition.
    
    pub background: Option<String>,
    /// The foreground color that can be used to write on top of a background with 'background' color.
    
    pub foreground: Option<String>,
}

impl client::Part for ColorDefinition {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get colors](ColorGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Colors {
    /// A global palette of calendar colors, mapping from the color ID to its definition. A calendarListEntry resource refers to one of these color IDs in its colorId field. Read-only.
    
    pub calendar: Option<HashMap<String, ColorDefinition>>,
    /// A global palette of event colors, mapping from the color ID to its definition. An event resource may refer to one of these color IDs in its colorId field. Read-only.
    
    pub event: Option<HashMap<String, ColorDefinition>>,
    /// Type of the resource ("calendar#colors").
    
    pub kind: Option<String>,
    /// Last modification time of the color palette (as a RFC3339 timestamp). Read-only.
    
    pub updated: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for Colors {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConferenceData {
    /// The ID of the conference.
    /// Can be used by developers to keep track of conferences, should not be displayed to users.
    /// The ID value is formed differently for each conference solution type:  
    /// - eventHangout: ID is not set. (This conference type is deprecated.)
    /// - eventNamedHangout: ID is the name of the Hangout. (This conference type is deprecated.)
    /// - hangoutsMeet: ID is the 10-letter meeting code, for example aaa-bbbb-ccc.
    /// - addOn: ID is defined by the third-party provider.  Optional.
    #[serde(rename="conferenceId")]
    
    pub conference_id: Option<String>,
    /// The conference solution, such as Google Meet.
    /// Unset for a conference with a failed create request.
    /// Either conferenceSolution and at least one entryPoint, or createRequest is required.
    #[serde(rename="conferenceSolution")]
    
    pub conference_solution: Option<ConferenceSolution>,
    /// A request to generate a new conference and attach it to the event. The data is generated asynchronously. To see whether the data is present check the status field.
    /// Either conferenceSolution and at least one entryPoint, or createRequest is required.
    #[serde(rename="createRequest")]
    
    pub create_request: Option<CreateConferenceRequest>,
    /// Information about individual conference entry points, such as URLs or phone numbers.
    /// All of them must belong to the same conference.
    /// Either conferenceSolution and at least one entryPoint, or createRequest is required.
    #[serde(rename="entryPoints")]
    
    pub entry_points: Option<Vec<EntryPoint>>,
    /// Additional notes (such as instructions from the domain administrator, legal notices) to display to the user. Can contain HTML. The maximum length is 2048 characters. Optional.
    
    pub notes: Option<String>,
    /// Additional properties related to a conference. An example would be a solution-specific setting for enabling video streaming.
    
    pub parameters: Option<ConferenceParameters>,
    /// The signature of the conference data.
    /// Generated on server side.
    /// Unset for a conference with a failed create request.
    /// Optional for a conference with a pending create request.
    
    pub signature: Option<String>,
}

impl client::Part for ConferenceData {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConferenceParameters {
    /// Additional add-on specific data.
    #[serde(rename="addOnParameters")]
    
    pub add_on_parameters: Option<ConferenceParametersAddOnParameters>,
}

impl client::Part for ConferenceParameters {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConferenceParametersAddOnParameters {
    /// no description provided
    
    pub parameters: Option<HashMap<String, String>>,
}

impl client::Part for ConferenceParametersAddOnParameters {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConferenceProperties {
    /// The types of conference solutions that are supported for this calendar.
    /// The possible values are:  
    /// - "eventHangout" 
    /// - "eventNamedHangout" 
    /// - "hangoutsMeet"  Optional.
    #[serde(rename="allowedConferenceSolutionTypes")]
    
    pub allowed_conference_solution_types: Option<Vec<String>>,
}

impl client::Part for ConferenceProperties {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConferenceRequestStatus {
    /// The current status of the conference create request. Read-only.
    /// The possible values are:  
    /// - "pending": the conference create request is still being processed.
    /// - "success": the conference create request succeeded, the entry points are populated.
    /// - "failure": the conference create request failed, there are no entry points.
    #[serde(rename="statusCode")]
    
    pub status_code: Option<String>,
}

impl client::Part for ConferenceRequestStatus {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConferenceSolution {
    /// The user-visible icon for this solution.
    #[serde(rename="iconUri")]
    
    pub icon_uri: Option<String>,
    /// The key which can uniquely identify the conference solution for this event.
    
    pub key: Option<ConferenceSolutionKey>,
    /// The user-visible name of this solution. Not localized.
    
    pub name: Option<String>,
}

impl client::Part for ConferenceSolution {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConferenceSolutionKey {
    /// The conference solution type.
    /// If a client encounters an unfamiliar or empty type, it should still be able to display the entry points. However, it should disallow modifications.
    /// The possible values are:  
    /// - "eventHangout" for Hangouts for consumers (deprecated; existing events may show this conference solution type but new conferences cannot be created)
    /// - "eventNamedHangout" for classic Hangouts for Google Workspace users (deprecated; existing events may show this conference solution type but new conferences cannot be created)
    /// - "hangoutsMeet" for Google Meet (http://meet.google.com)
    /// - "addOn" for 3P conference providers
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for ConferenceSolutionKey {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateConferenceRequest {
    /// The conference solution, such as Hangouts or Google Meet.
    #[serde(rename="conferenceSolutionKey")]
    
    pub conference_solution_key: Option<ConferenceSolutionKey>,
    /// The client-generated unique ID for this request.
    /// Clients should regenerate this ID for every new request. If an ID provided is the same as for the previous request, the request is ignored.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
    /// The status of the conference create request.
    
    pub status: Option<ConferenceRequestStatus>,
}

impl client::Part for CreateConferenceRequest {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EntryPoint {
    /// The access code to access the conference. The maximum length is 128 characters.
    /// When creating new conference data, populate only the subset of {meetingCode, accessCode, passcode, password, pin} fields that match the terminology that the conference provider uses. Only the populated fields should be displayed.
    /// Optional.
    #[serde(rename="accessCode")]
    
    pub access_code: Option<String>,
    /// Features of the entry point, such as being toll or toll-free. One entry point can have multiple features. However, toll and toll-free cannot be both set on the same entry point.
    #[serde(rename="entryPointFeatures")]
    
    pub entry_point_features: Option<Vec<String>>,
    /// The type of the conference entry point.
    /// Possible values are:  
    /// - "video" - joining a conference over HTTP. A conference can have zero or one video entry point.
    /// - "phone" - joining a conference by dialing a phone number. A conference can have zero or more phone entry points.
    /// - "sip" - joining a conference over SIP. A conference can have zero or one sip entry point.
    /// - "more" - further conference joining instructions, for example additional phone numbers. A conference can have zero or one more entry point. A conference with only a more entry point is not a valid conference.
    #[serde(rename="entryPointType")]
    
    pub entry_point_type: Option<String>,
    /// The label for the URI. Visible to end users. Not localized. The maximum length is 512 characters.
    /// Examples:  
    /// - for video: meet.google.com/aaa-bbbb-ccc
    /// - for phone: +1 123 268 2601
    /// - for sip: 12345678@altostrat.com
    /// - for more: should not be filled  
    /// Optional.
    
    pub label: Option<String>,
    /// The meeting code to access the conference. The maximum length is 128 characters.
    /// When creating new conference data, populate only the subset of {meetingCode, accessCode, passcode, password, pin} fields that match the terminology that the conference provider uses. Only the populated fields should be displayed.
    /// Optional.
    #[serde(rename="meetingCode")]
    
    pub meeting_code: Option<String>,
    /// The passcode to access the conference. The maximum length is 128 characters.
    /// When creating new conference data, populate only the subset of {meetingCode, accessCode, passcode, password, pin} fields that match the terminology that the conference provider uses. Only the populated fields should be displayed.
    
    pub passcode: Option<String>,
    /// The password to access the conference. The maximum length is 128 characters.
    /// When creating new conference data, populate only the subset of {meetingCode, accessCode, passcode, password, pin} fields that match the terminology that the conference provider uses. Only the populated fields should be displayed.
    /// Optional.
    
    pub password: Option<String>,
    /// The PIN to access the conference. The maximum length is 128 characters.
    /// When creating new conference data, populate only the subset of {meetingCode, accessCode, passcode, password, pin} fields that match the terminology that the conference provider uses. Only the populated fields should be displayed.
    /// Optional.
    
    pub pin: Option<String>,
    /// The CLDR/ISO 3166 region code for the country associated with this phone access. Example: "SE" for Sweden.
    /// Calendar backend will populate this field only for EntryPointType.PHONE.
    #[serde(rename="regionCode")]
    
    pub region_code: Option<String>,
    /// The URI of the entry point. The maximum length is 1300 characters.
    /// Format:  
    /// - for video, http: or https: schema is required.
    /// - for phone, tel: schema is required. The URI should include the entire dial sequence (e.g., tel:+12345678900,,,123456789;1234).
    /// - for sip, sip: schema is required, e.g., sip:12345678@myprovider.com.
    /// - for more, http: or https: schema is required.
    
    pub uri: Option<String>,
}

impl client::Part for EntryPoint {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Error {
    /// Domain, or broad category, of the error.
    
    pub domain: Option<String>,
    /// Specific reason for the error. Some of the possible values are:  
    /// - "groupTooBig" - The group of users requested is too large for a single query. 
    /// - "tooManyCalendarsRequested" - The number of calendars requested is too large for a single query. 
    /// - "notFound" - The requested resource was not found. 
    /// - "internalError" - The API service has encountered an internal error.  Additional error types may be added in the future, so clients should gracefully handle additional error statuses not included in this list.
    
    pub reason: Option<String>,
}

impl client::Part for Error {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete events](EventDeleteCall) (none)
/// * [get events](EventGetCall) (response)
/// * [import events](EventImportCall) (request|response)
/// * [insert events](EventInsertCall) (request|response)
/// * [instances events](EventInstanceCall) (none)
/// * [list events](EventListCall) (none)
/// * [move events](EventMoveCall) (response)
/// * [patch events](EventPatchCall) (request|response)
/// * [quick add events](EventQuickAddCall) (response)
/// * [update events](EventUpdateCall) (request|response)
/// * [watch events](EventWatchCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    /// Whether anyone can invite themselves to the event (deprecated). Optional. The default is False.
    #[serde(rename="anyoneCanAddSelf")]
    
    pub anyone_can_add_self: Option<bool>,
    /// File attachments for the event.
    /// In order to modify attachments the supportsAttachments request parameter should be set to true.
    /// There can be at most 25 attachments per event,
    
    pub attachments: Option<Vec<EventAttachment>>,
    /// The attendees of the event. See the Events with attendees guide for more information on scheduling events with other calendar users. Service accounts need to use domain-wide delegation of authority to populate the attendee list.
    
    pub attendees: Option<Vec<EventAttendee>>,
    /// Whether attendees may have been omitted from the event's representation. When retrieving an event, this may be due to a restriction specified by the maxAttendee query parameter. When updating an event, this can be used to only update the participant's response. Optional. The default is False.
    #[serde(rename="attendeesOmitted")]
    
    pub attendees_omitted: Option<bool>,
    /// The color of the event. This is an ID referring to an entry in the event section of the colors definition (see the  colors endpoint). Optional.
    #[serde(rename="colorId")]
    
    pub color_id: Option<String>,
    /// The conference-related information, such as details of a Google Meet conference. To create new conference details use the createRequest field. To persist your changes, remember to set the conferenceDataVersion request parameter to 1 for all event modification requests.
    #[serde(rename="conferenceData")]
    
    pub conference_data: Option<ConferenceData>,
    /// Creation time of the event (as a RFC3339 timestamp). Read-only.
    
    pub created: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The creator of the event. Read-only.
    
    pub creator: Option<EventCreator>,
    /// Description of the event. Can contain HTML. Optional.
    
    pub description: Option<String>,
    /// The (exclusive) end time of the event. For a recurring event, this is the end time of the first instance.
    
    pub end: Option<EventDateTime>,
    /// Whether the end time is actually unspecified. An end time is still provided for compatibility reasons, even if this attribute is set to True. The default is False.
    #[serde(rename="endTimeUnspecified")]
    
    pub end_time_unspecified: Option<bool>,
    /// ETag of the resource.
    
    pub etag: Option<String>,
    /// Specific type of the event. Read-only. Possible values are:  
    /// - "default" - A regular event or not further specified. 
    /// - "outOfOffice" - An out-of-office event. 
    /// - "focusTime" - A focus-time event.
    #[serde(rename="eventType")]
    
    pub event_type: Option<String>,
    /// Extended properties of the event.
    #[serde(rename="extendedProperties")]
    
    pub extended_properties: Option<EventExtendedProperties>,
    /// A gadget that extends this event. Gadgets are deprecated; this structure is instead only used for returning birthday calendar metadata.
    
    pub gadget: Option<EventGadget>,
    /// Whether attendees other than the organizer can invite others to the event. Optional. The default is True.
    #[serde(rename="guestsCanInviteOthers")]
    
    pub guests_can_invite_others: Option<bool>,
    /// Whether attendees other than the organizer can modify the event. Optional. The default is False.
    #[serde(rename="guestsCanModify")]
    
    pub guests_can_modify: Option<bool>,
    /// Whether attendees other than the organizer can see who the event's attendees are. Optional. The default is True.
    #[serde(rename="guestsCanSeeOtherGuests")]
    
    pub guests_can_see_other_guests: Option<bool>,
    /// An absolute link to the Google Hangout associated with this event. Read-only.
    #[serde(rename="hangoutLink")]
    
    pub hangout_link: Option<String>,
    /// An absolute link to this event in the Google Calendar Web UI. Read-only.
    #[serde(rename="htmlLink")]
    
    pub html_link: Option<String>,
    /// Event unique identifier as defined in RFC5545. It is used to uniquely identify events accross calendaring systems and must be supplied when importing events via the import method.
    /// Note that the iCalUID and the id are not identical and only one of them should be supplied at event creation time. One difference in their semantics is that in recurring events, all occurrences of one event have different ids while they all share the same iCalUIDs. To retrieve an event using its iCalUID, call the events.list method using the iCalUID parameter. To retrieve an event using its id, call the events.get method.
    #[serde(rename="iCalUID")]
    
    pub i_cal_uid: Option<String>,
    /// Opaque identifier of the event. When creating new single or recurring events, you can specify their IDs. Provided IDs must follow these rules:  
    /// - characters allowed in the ID are those used in base32hex encoding, i.e. lowercase letters a-v and digits 0-9, see section 3.1.2 in RFC2938 
    /// - the length of the ID must be between 5 and 1024 characters 
    /// - the ID must be unique per calendar  Due to the globally distributed nature of the system, we cannot guarantee that ID collisions will be detected at event creation time. To minimize the risk of collisions we recommend using an established UUID algorithm such as one described in RFC4122.
    /// If you do not specify an ID, it will be automatically generated by the server.
    /// Note that the icalUID and the id are not identical and only one of them should be supplied at event creation time. One difference in their semantics is that in recurring events, all occurrences of one event have different ids while they all share the same icalUIDs.
    
    pub id: Option<String>,
    /// Type of the resource ("calendar#event").
    
    pub kind: Option<String>,
    /// Geographic location of the event as free-form text. Optional.
    
    pub location: Option<String>,
    /// Whether this is a locked event copy where no changes can be made to the main event fields "summary", "description", "location", "start", "end" or "recurrence". The default is False. Read-Only.
    
    pub locked: Option<bool>,
    /// The organizer of the event. If the organizer is also an attendee, this is indicated with a separate entry in attendees with the organizer field set to True. To change the organizer, use the move operation. Read-only, except when importing an event.
    
    pub organizer: Option<EventOrganizer>,
    /// For an instance of a recurring event, this is the time at which this event would start according to the recurrence data in the recurring event identified by recurringEventId. It uniquely identifies the instance within the recurring event series even if the instance was moved to a different time. Immutable.
    #[serde(rename="originalStartTime")]
    
    pub original_start_time: Option<EventDateTime>,
    /// If set to True, Event propagation is disabled. Note that it is not the same thing as Private event properties. Optional. Immutable. The default is False.
    #[serde(rename="privateCopy")]
    
    pub private_copy: Option<bool>,
    /// List of RRULE, EXRULE, RDATE and EXDATE lines for a recurring event, as specified in RFC5545. Note that DTSTART and DTEND lines are not allowed in this field; event start and end times are specified in the start and end fields. This field is omitted for single events or instances of recurring events.
    
    pub recurrence: Option<Vec<String>>,
    /// For an instance of a recurring event, this is the id of the recurring event to which this instance belongs. Immutable.
    #[serde(rename="recurringEventId")]
    
    pub recurring_event_id: Option<String>,
    /// Information about the event's reminders for the authenticated user.
    
    pub reminders: Option<EventReminders>,
    /// Sequence number as per iCalendar.
    
    pub sequence: Option<i32>,
    /// Source from which the event was created. For example, a web page, an email message or any document identifiable by an URL with HTTP or HTTPS scheme. Can only be seen or modified by the creator of the event.
    
    pub source: Option<EventSource>,
    /// The (inclusive) start time of the event. For a recurring event, this is the start time of the first instance.
    
    pub start: Option<EventDateTime>,
    /// Status of the event. Optional. Possible values are:  
    /// - "confirmed" - The event is confirmed. This is the default status. 
    /// - "tentative" - The event is tentatively confirmed. 
    /// - "cancelled" - The event is cancelled (deleted). The list method returns cancelled events only on incremental sync (when syncToken or updatedMin are specified) or if the showDeleted flag is set to true. The get method always returns them.
    /// A cancelled status represents two different states depending on the event type:  
    /// - Cancelled exceptions of an uncancelled recurring event indicate that this instance should no longer be presented to the user. Clients should store these events for the lifetime of the parent recurring event.
    /// Cancelled exceptions are only guaranteed to have values for the id, recurringEventId and originalStartTime fields populated. The other fields might be empty.  
    /// - All other cancelled events represent deleted events. Clients should remove their locally synced copies. Such cancelled events will eventually disappear, so do not rely on them being available indefinitely.
    /// Deleted events are only guaranteed to have the id field populated.   On the organizer's calendar, cancelled events continue to expose event details (summary, location, etc.) so that they can be restored (undeleted). Similarly, the events to which the user was invited and that they manually removed continue to provide details. However, incremental sync requests with showDeleted set to false will not return these details.
    /// If an event changes its organizer (for example via the move operation) and the original organizer is not on the attendee list, it will leave behind a cancelled event where only the id field is guaranteed to be populated.
    
    pub status: Option<String>,
    /// Title of the event.
    
    pub summary: Option<String>,
    /// Whether the event blocks time on the calendar. Optional. Possible values are:  
    /// - "opaque" - Default value. The event does block time on the calendar. This is equivalent to setting Show me as to Busy in the Calendar UI. 
    /// - "transparent" - The event does not block time on the calendar. This is equivalent to setting Show me as to Available in the Calendar UI.
    
    pub transparency: Option<String>,
    /// Last modification time of the event (as a RFC3339 timestamp). Read-only.
    
    pub updated: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Visibility of the event. Optional. Possible values are:  
    /// - "default" - Uses the default visibility for events on the calendar. This is the default value. 
    /// - "public" - The event is public and event details are visible to all readers of the calendar. 
    /// - "private" - The event is private and only event attendees may view event details. 
    /// - "confidential" - The event is private. This value is provided for compatibility reasons.
    
    pub visibility: Option<String>,
}

impl client::RequestValue for Event {}
impl client::Resource for Event {}
impl client::ResponseResult for Event {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventAttachment {
    /// ID of the attached file. Read-only.
    /// For Google Drive files, this is the ID of the corresponding Files resource entry in the Drive API.
    #[serde(rename="fileId")]
    
    pub file_id: Option<String>,
    /// URL link to the attachment.
    /// For adding Google Drive file attachments use the same format as in alternateLink property of the Files resource in the Drive API.
    /// Required when adding an attachment.
    #[serde(rename="fileUrl")]
    
    pub file_url: Option<String>,
    /// URL link to the attachment's icon. This field can only be modified for custom third-party attachments.
    #[serde(rename="iconLink")]
    
    pub icon_link: Option<String>,
    /// Internet media type (MIME type) of the attachment.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
    /// Attachment title.
    
    pub title: Option<String>,
}

impl client::Part for EventAttachment {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventAttendee {
    /// Number of additional guests. Optional. The default is 0.
    #[serde(rename="additionalGuests")]
    
    pub additional_guests: Option<i32>,
    /// The attendee's response comment. Optional.
    
    pub comment: Option<String>,
    /// The attendee's name, if available. Optional.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The attendee's email address, if available. This field must be present when adding an attendee. It must be a valid email address as per RFC5322.
    /// Required when adding an attendee.
    
    pub email: Option<String>,
    /// The attendee's Profile ID, if available.
    
    pub id: Option<String>,
    /// Whether this is an optional attendee. Optional. The default is False.
    
    pub optional: Option<bool>,
    /// Whether the attendee is the organizer of the event. Read-only. The default is False.
    
    pub organizer: Option<bool>,
    /// Whether the attendee is a resource. Can only be set when the attendee is added to the event for the first time. Subsequent modifications are ignored. Optional. The default is False.
    
    pub resource: Option<bool>,
    /// The attendee's response status. Possible values are:  
    /// - "needsAction" - The attendee has not responded to the invitation (recommended for new events). 
    /// - "declined" - The attendee has declined the invitation. 
    /// - "tentative" - The attendee has tentatively accepted the invitation. 
    /// - "accepted" - The attendee has accepted the invitation.  Warning: If you add an event using the values declined, tentative, or accepted, attendees with the "Add invitations to my calendar" setting set to "When I respond to invitation in email" won't see an event on their calendar unless they choose to change their invitation response in the event invitation email.
    #[serde(rename="responseStatus")]
    
    pub response_status: Option<String>,
    /// Whether this entry represents the calendar on which this copy of the event appears. Read-only. The default is False.
    #[serde(rename="self")]
    
    pub self_: Option<bool>,
}

impl client::Part for EventAttendee {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventDateTime {
    /// The date, in the format "yyyy-mm-dd", if this is an all-day event.
    
    pub date: Option<client::chrono::NaiveDate>,
    /// The time, as a combined date-time value (formatted according to RFC3339). A time zone offset is required unless a time zone is explicitly specified in timeZone.
    #[serde(rename="dateTime")]
    
    pub date_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The time zone in which the time is specified. (Formatted as an IANA Time Zone Database name, e.g. "Europe/Zurich".) For recurring events this field is required and specifies the time zone in which the recurrence is expanded. For single events this field is optional and indicates a custom time zone for the event start/end.
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<String>,
}

impl client::Part for EventDateTime {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventReminder {
    /// The method used by this reminder. Possible values are:  
    /// - "email" - Reminders are sent via email. 
    /// - "popup" - Reminders are sent via a UI popup.  
    /// Required when adding a reminder.
    
    pub method: Option<String>,
    /// Number of minutes before the start of the event when the reminder should trigger. Valid values are between 0 and 40320 (4 weeks in minutes).
    /// Required when adding a reminder.
    
    pub minutes: Option<i32>,
}

impl client::Part for EventReminder {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances events](EventInstanceCall) (response)
/// * [list events](EventListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Events {
    /// The user's access role for this calendar. Read-only. Possible values are:  
    /// - "none" - The user has no access. 
    /// - "freeBusyReader" - The user has read access to free/busy information. 
    /// - "reader" - The user has read access to the calendar. Private events will appear to users with reader access, but event details will be hidden. 
    /// - "writer" - The user has read and write access to the calendar. Private events will appear to users with writer access, and event details will be visible. 
    /// - "owner" - The user has ownership of the calendar. This role has all of the permissions of the writer role with the additional ability to see and manipulate ACLs.
    #[serde(rename="accessRole")]
    
    pub access_role: Option<String>,
    /// The default reminders on the calendar for the authenticated user. These reminders apply to all events on this calendar that do not explicitly override them (i.e. do not have reminders.useDefault set to True).
    #[serde(rename="defaultReminders")]
    
    pub default_reminders: Option<Vec<EventReminder>>,
    /// Description of the calendar. Read-only.
    
    pub description: Option<String>,
    /// ETag of the collection.
    
    pub etag: Option<String>,
    /// List of events on the calendar.
    
    pub items: Option<Vec<Event>>,
    /// Type of the collection ("calendar#events").
    
    pub kind: Option<String>,
    /// Token used to access the next page of this result. Omitted if no further results are available, in which case nextSyncToken is provided.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Token used at a later point in time to retrieve only the entries that have changed since this result was returned. Omitted if further results are available, in which case nextPageToken is provided.
    #[serde(rename="nextSyncToken")]
    
    pub next_sync_token: Option<String>,
    /// Title of the calendar. Read-only.
    
    pub summary: Option<String>,
    /// The time zone of the calendar. Read-only.
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<String>,
    /// Last modification time of the calendar (as a RFC3339 timestamp). Read-only.
    
    pub updated: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for Events {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FreeBusyCalendar {
    /// List of time ranges during which this calendar should be regarded as busy.
    
    pub busy: Option<Vec<TimePeriod>>,
    /// Optional error(s) (if computation for the calendar failed).
    
    pub errors: Option<Vec<Error>>,
}

impl client::Part for FreeBusyCalendar {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FreeBusyGroup {
    /// List of calendars' identifiers within a group.
    
    pub calendars: Option<Vec<String>>,
    /// Optional error(s) (if computation for the group failed).
    
    pub errors: Option<Vec<Error>>,
}

impl client::Part for FreeBusyGroup {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [query freebusy](FreebusyQueryCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FreeBusyRequest {
    /// Maximal number of calendars for which FreeBusy information is to be provided. Optional. Maximum value is 50.
    #[serde(rename="calendarExpansionMax")]
    
    pub calendar_expansion_max: Option<i32>,
    /// Maximal number of calendar identifiers to be provided for a single group. Optional. An error is returned for a group with more members than this value. Maximum value is 100.
    #[serde(rename="groupExpansionMax")]
    
    pub group_expansion_max: Option<i32>,
    /// List of calendars and/or groups to query.
    
    pub items: Option<Vec<FreeBusyRequestItem>>,
    /// The end of the interval for the query formatted as per RFC3339.
    #[serde(rename="timeMax")]
    
    pub time_max: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The start of the interval for the query formatted as per RFC3339.
    #[serde(rename="timeMin")]
    
    pub time_min: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Time zone used in the response. Optional. The default is UTC.
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<String>,
}

impl client::RequestValue for FreeBusyRequest {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FreeBusyRequestItem {
    /// The identifier of a calendar or a group.
    
    pub id: Option<String>,
}

impl client::Part for FreeBusyRequestItem {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [query freebusy](FreebusyQueryCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FreeBusyResponse {
    /// List of free/busy information for calendars.
    
    pub calendars: Option<HashMap<String, FreeBusyCalendar>>,
    /// Expansion of groups.
    
    pub groups: Option<HashMap<String, FreeBusyGroup>>,
    /// Type of the resource ("calendar#freeBusy").
    
    pub kind: Option<String>,
    /// The end of the interval.
    #[serde(rename="timeMax")]
    
    pub time_max: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The start of the interval.
    #[serde(rename="timeMin")]
    
    pub time_min: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for FreeBusyResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get settings](SettingGetCall) (response)
/// * [list settings](SettingListCall) (none)
/// * [watch settings](SettingWatchCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Setting {
    /// ETag of the resource.
    
    pub etag: Option<String>,
    /// The id of the user setting.
    
    pub id: Option<String>,
    /// Type of the resource ("calendar#setting").
    
    pub kind: Option<String>,
    /// Value of the user setting. The format of the value depends on the ID of the setting. It must always be a UTF-8 string of length up to 1024 characters.
    
    pub value: Option<String>,
}

impl client::Resource for Setting {}
impl client::ResponseResult for Setting {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list settings](SettingListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    /// Etag of the collection.
    
    pub etag: Option<String>,
    /// List of user settings.
    
    pub items: Option<Vec<Setting>>,
    /// Type of the collection ("calendar#settings").
    
    pub kind: Option<String>,
    /// Token used to access the next page of this result. Omitted if no further results are available, in which case nextSyncToken is provided.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Token used at a later point in time to retrieve only the entries that have changed since this result was returned. Omitted if further results are available, in which case nextPageToken is provided.
    #[serde(rename="nextSyncToken")]
    
    pub next_sync_token: Option<String>,
}

impl client::ResponseResult for Settings {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimePeriod {
    /// The (exclusive) end of the time period.
    
    pub end: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The (inclusive) start of the time period.
    
    pub start: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for TimePeriod {}


/// The extent to which calendar access is granted by this ACL rule.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AclRuleScope {
    /// The type of the scope. Possible values are:  
    /// - "default" - The public scope. This is the default value. 
    /// - "user" - Limits the scope to a single user. 
    /// - "group" - Limits the scope to a group. 
    /// - "domain" - Limits the scope to a domain.  Note: The permissions granted to the "default", or public, scope apply to any user, authenticated or not.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The email address of a user or group, or the name of a domain, depending on the scope type. Omitted for type "default".
    
    pub value: Option<String>,
}

impl client::NestedType for AclRuleScope {}
impl client::Part for AclRuleScope {}


/// The notifications that the authenticated user is receiving for this calendar.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CalendarListEntryNotificationSettings {
    /// The list of notifications set for this calendar.
    
    pub notifications: Option<Vec<CalendarNotification>>,
}

impl client::NestedType for CalendarListEntryNotificationSettings {}
impl client::Part for CalendarListEntryNotificationSettings {}


/// The creator of the event. Read-only.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventCreator {
    /// The creator's name, if available.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The creator's email address, if available.
    
    pub email: Option<String>,
    /// The creator's Profile ID, if available.
    
    pub id: Option<String>,
    /// Whether the creator corresponds to the calendar on which this copy of the event appears. Read-only. The default is False.
    #[serde(rename="self")]
    
    pub self_: Option<bool>,
}

impl client::NestedType for EventCreator {}
impl client::Part for EventCreator {}


/// Extended properties of the event.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventExtendedProperties {
    /// Properties that are private to the copy of the event that appears on this calendar.
    
    pub private: Option<HashMap<String, String>>,
    /// Properties that are shared between copies of the event on other attendees' calendars.
    
    pub shared: Option<HashMap<String, String>>,
}

impl client::NestedType for EventExtendedProperties {}
impl client::Part for EventExtendedProperties {}


/// A gadget that extends this event. Gadgets are deprecated; this structure is instead only used for returning birthday calendar metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventGadget {
    /// The gadget's display mode. Deprecated. Possible values are:  
    /// - "icon" - The gadget displays next to the event's title in the calendar view. 
    /// - "chip" - The gadget displays when the event is clicked.
    
    pub display: Option<String>,
    /// The gadget's height in pixels. The height must be an integer greater than 0. Optional. Deprecated.
    
    pub height: Option<i32>,
    /// The gadget's icon URL. The URL scheme must be HTTPS. Deprecated.
    #[serde(rename="iconLink")]
    
    pub icon_link: Option<String>,
    /// The gadget's URL. The URL scheme must be HTTPS. Deprecated.
    
    pub link: Option<String>,
    /// Preferences.
    
    pub preferences: Option<HashMap<String, String>>,
    /// The gadget's title. Deprecated.
    
    pub title: Option<String>,
    /// The gadget's type. Deprecated.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The gadget's width in pixels. The width must be an integer greater than 0. Optional. Deprecated.
    
    pub width: Option<i32>,
}

impl client::NestedType for EventGadget {}
impl client::Part for EventGadget {}


/// The organizer of the event. If the organizer is also an attendee, this is indicated with a separate entry in attendees with the organizer field set to True. To change the organizer, use the move operation. Read-only, except when importing an event.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventOrganizer {
    /// The organizer's name, if available.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The organizer's email address, if available. It must be a valid email address as per RFC5322.
    
    pub email: Option<String>,
    /// The organizer's Profile ID, if available.
    
    pub id: Option<String>,
    /// Whether the organizer corresponds to the calendar on which this copy of the event appears. Read-only. The default is False.
    #[serde(rename="self")]
    
    pub self_: Option<bool>,
}

impl client::NestedType for EventOrganizer {}
impl client::Part for EventOrganizer {}


/// Information about the event's reminders for the authenticated user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventReminders {
    /// If the event doesn't use the default reminders, this lists the reminders specific to the event, or, if not set, indicates that no reminders are set for this event. The maximum number of override reminders is 5.
    
    pub overrides: Option<Vec<EventReminder>>,
    /// Whether the default reminders of the calendar apply to the event.
    #[serde(rename="useDefault")]
    
    pub use_default: Option<bool>,
}

impl client::NestedType for EventReminders {}
impl client::Part for EventReminders {}


/// Source from which the event was created. For example, a web page, an email message or any document identifiable by an URL with HTTP or HTTPS scheme. Can only be seen or modified by the creator of the event.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventSource {
    /// Title of the source; for example a title of a web page or an email subject.
    
    pub title: Option<String>,
    /// URL of the source pointing to a resource. The URL scheme must be HTTP or HTTPS.
    
    pub url: Option<String>,
}

impl client::NestedType for EventSource {}
impl client::Part for EventSource {}


