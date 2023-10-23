use super::*;
/// An Activity resource is a combined view of multiple events. An activity has a list of individual events and a combined view of the common fields among all events.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Activity {
    /// The fields common to all of the singleEvents that make up the Activity.
    #[serde(rename="combinedEvent")]
    
    pub combined_event: Option<Event>,
    /// A list of all the Events that make up the Activity.
    #[serde(rename="singleEvents")]
    
    pub single_events: Option<Vec<Event>>,
}

impl client::Part for Activity {}


/// Represents the changes associated with an action taken by a user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    /// Additional event types. Some events may have multiple types when multiple actions are part of a single event. For example, creating a document, renaming it, and sharing it may be part of a single file-creation event.
    #[serde(rename="additionalEventTypes")]
    
    pub additional_event_types: Option<Vec<EventAdditionalEventTypesEnum>>,
    /// The time at which the event occurred formatted as Unix time in milliseconds.
    #[serde(rename="eventTimeMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub event_time_millis: Option<u64>,
    /// Whether this event is caused by a user being deleted.
    #[serde(rename="fromUserDeletion")]
    
    pub from_user_deletion: Option<bool>,
    /// Extra information for move type events, such as changes in an object's parents.
    #[serde(rename="move")]
    
    pub move_: Option<Move>,
    /// Extra information for permissionChange type events, such as the user or group the new permission applies to.
    #[serde(rename="permissionChanges")]
    
    pub permission_changes: Option<Vec<PermissionChange>>,
    /// The main type of event that occurred.
    #[serde(rename="primaryEventType")]
    
    pub primary_event_type: Option<EventPrimaryEventTypeEnum>,
    /// Extra information for rename type events, such as the old and new names.
    
    pub rename: Option<Rename>,
    /// Information specific to the Target object modified by the event.
    
    pub target: Option<Target>,
    /// Represents the user responsible for the event.
    
    pub user: Option<User>,
}

impl client::Part for Event {}


/// The response from the list request. Contains a list of activities and a token to retrieve the next page of results.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list activities](ActivityListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListActivitiesResponse {
    /// List of activities.
    
    pub activities: Option<Vec<Activity>>,
    /// Token for the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListActivitiesResponse {}


/// Contains information about changes in an object's parents as a result of a move type event.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Move {
    /// The added parent(s).
    #[serde(rename="addedParents")]
    
    pub added_parents: Option<Vec<Parent>>,
    /// The removed parent(s).
    #[serde(rename="removedParents")]
    
    pub removed_parents: Option<Vec<Parent>>,
}

impl client::Part for Move {}


/// Contains information about a parent object. For example, a folder in Drive is a parent for all files within it.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Parent {
    /// The parent's ID.
    
    pub id: Option<String>,
    /// Whether this is the root folder.
    #[serde(rename="isRoot")]
    
    pub is_root: Option<bool>,
    /// The parent's title.
    
    pub title: Option<String>,
}

impl client::Part for Parent {}


/// Contains information about the permissions and type of access allowed with regards to a Google Drive object. This is a subset of the fields contained in a corresponding Drive Permissions object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Permission {
    /// The name of the user or group the permission applies to.
    
    pub name: Option<String>,
    /// The ID for this permission. Corresponds to the Drive API's permission ID returned as part of the Drive Permissions resource.
    #[serde(rename="permissionId")]
    
    pub permission_id: Option<String>,
    /// Indicates the Google Drive permissions role. The role determines a user's ability to read, write, or comment on the file.
    
    pub role: Option<PermissionRoleEnum>,
    /// Indicates how widely permissions are granted.
    #[serde(rename="type")]
    
    pub type_: Option<PermissionTypeEnum>,
    /// The user's information if the type is USER.
    
    pub user: Option<User>,
    /// Whether the permission requires a link to the file.
    #[serde(rename="withLink")]
    
    pub with_link: Option<bool>,
}

impl client::Part for Permission {}


/// Contains information about a Drive object's permissions that changed as a result of a permissionChange type event.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PermissionChange {
    /// Lists all Permission objects added.
    #[serde(rename="addedPermissions")]
    
    pub added_permissions: Option<Vec<Permission>>,
    /// Lists all Permission objects removed.
    #[serde(rename="removedPermissions")]
    
    pub removed_permissions: Option<Vec<Permission>>,
}

impl client::Part for PermissionChange {}


/// Photo information for a user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Photo {
    /// The URL of the photo.
    
    pub url: Option<String>,
}

impl client::Part for Photo {}


/// Contains information about a renametype event.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Rename {
    /// The new title.
    #[serde(rename="newTitle")]
    
    pub new_title: Option<String>,
    /// The old title.
    #[serde(rename="oldTitle")]
    
    pub old_title: Option<String>,
}

impl client::Part for Rename {}


/// Information about the object modified by the event.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Target {
    /// The ID of the target. For example, in Google Drive, this is the file or folder ID.
    
    pub id: Option<String>,
    /// The MIME type of the target.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
    /// The name of the target. For example, in Google Drive, this is the title of the file.
    
    pub name: Option<String>,
}

impl client::Part for Target {}


/// A representation of a user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct User {
    /// A boolean which indicates whether the specified User was deleted. If true, name, photo and permission_id will be omitted.
    #[serde(rename="isDeleted")]
    
    pub is_deleted: Option<bool>,
    /// Whether the user is the authenticated user.
    #[serde(rename="isMe")]
    
    pub is_me: Option<bool>,
    /// The displayable name of the user.
    
    pub name: Option<String>,
    /// The permission ID associated with this user. Equivalent to the Drive API's permission ID for this user, returned as part of the Drive Permissions resource.
    #[serde(rename="permissionId")]
    
    pub permission_id: Option<String>,
    /// The profile photo of the user. Not present if the user has no profile photo.
    
    pub photo: Option<Photo>,
}

impl client::Part for User {}


