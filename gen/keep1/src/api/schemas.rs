use super::*;
/// An attachment to a note.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [download media](MediaDownloadCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Attachment {
    /// The MIME types (IANA media types) in which the attachment is available.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<Vec<String>>,
    /// The resource name;
    
    pub name: Option<String>,
}

impl client::ResponseResult for Attachment {}


/// The request to add one or more permissions on the note. Currently, only the `WRITER` role may be specified. If adding a permission fails, then the entire request fails and no changes are made.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [permissions batch create notes](NotePermissionBatchCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchCreatePermissionsRequest {
    /// The request message specifying the resources to create.
    
    pub requests: Option<Vec<CreatePermissionRequest>>,
}

impl client::RequestValue for BatchCreatePermissionsRequest {}


/// The response for creating permissions on a note.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [permissions batch create notes](NotePermissionBatchCreateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchCreatePermissionsResponse {
    /// Permissions created.
    
    pub permissions: Option<Vec<Permission>>,
}

impl client::ResponseResult for BatchCreatePermissionsResponse {}


/// The request to remove one or more permissions from a note. A permission with the `OWNER` role can’t be removed. If removing a permission fails, then the entire request fails and no changes are made. Returns a 400 bad request error if a specified permission does not exist on the note.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [permissions batch delete notes](NotePermissionBatchDeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchDeletePermissionsRequest {
    /// Required. The names of the permissions to delete. Format: `notes/{note}/permissions/{permission}`
    
    pub names: Option<Vec<String>>,
}

impl client::RequestValue for BatchDeletePermissionsRequest {}


/// The request to add a single permission on the note.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreatePermissionRequest {
    /// Required. The parent note where this permission will be created. Format: `notes/{note}`
    
    pub parent: Option<String>,
    /// Required. The permission to create. One of Permission.email, User.email or Group.email must be supplied.
    
    pub permission: Option<Permission>,
}

impl client::Part for CreatePermissionRequest {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [permissions batch delete notes](NotePermissionBatchDeleteCall) (response)
/// * [delete notes](NoteDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// Describes a single Google Family.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Family { _never_set: Option<bool> }

impl client::Part for Family {}


/// Describes a single Group.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Group {
    /// The group email.
    
    pub email: Option<String>,
}

impl client::Part for Group {}


/// The list of items for a single list note.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListContent {
    /// The items in the list. The number of items must be less than 1,000.
    #[serde(rename="listItems")]
    
    pub list_items: Option<Vec<ListItem>>,
}

impl client::Part for ListContent {}


/// A single list item in a note's list.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListItem {
    /// Whether this item has been checked off or not.
    
    pub checked: Option<bool>,
    /// If set, list of list items nested under this list item. Only one level of nesting is allowed.
    #[serde(rename="childListItems")]
    
    pub child_list_items: Option<Vec<ListItem>>,
    /// The text of this item. Length must be less than 1,000 characters.
    
    pub text: Option<TextContent>,
}

impl client::Part for ListItem {}


/// The response when listing a page of notes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list notes](NoteListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListNotesResponse {
    /// Next page's `page_token` field.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A page of notes.
    
    pub notes: Option<Vec<Note>>,
}

impl client::ResponseResult for ListNotesResponse {}


/// A single note.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [permissions batch create notes](NotePermissionBatchCreateCall) (none)
/// * [permissions batch delete notes](NotePermissionBatchDeleteCall) (none)
/// * [create notes](NoteCreateCall) (request|response)
/// * [delete notes](NoteDeleteCall) (none)
/// * [get notes](NoteGetCall) (response)
/// * [list notes](NoteListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Note {
    /// Output only. The attachments attached to this note.
    
    pub attachments: Option<Vec<Attachment>>,
    /// The body of the note.
    
    pub body: Option<Section>,
    /// Output only. When this note was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The resource name of this note. See general note on identifiers in KeepService.
    
    pub name: Option<String>,
    /// Output only. The list of permissions set on the note. Contains at least one entry for the note owner.
    
    pub permissions: Option<Vec<Permission>>,
    /// The title of the note. Length must be less than 1,000 characters.
    
    pub title: Option<String>,
    /// Output only. When this note was trashed. If `trashed`, the note is eventually deleted. If the note is not trashed, this field is not set (and the trashed field is `false`).
    #[serde(rename="trashTime")]
    
    pub trash_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. `true` if this note has been trashed. If trashed, the note is eventually deleted.
    
    pub trashed: Option<bool>,
    /// Output only. When this note was last modified.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Note {}
impl client::Resource for Note {}
impl client::ResponseResult for Note {}


/// A single permission on the note. Associates a `member` with a `role`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Permission {
    /// Output only. Whether this member has been deleted. If the member is recovered, this value is set to false and the recovered member retains the role on the note.
    
    pub deleted: Option<bool>,
    /// The email associated with the member. If set on create, the `email` field in the `User` or `Group` message must either be empty or match this field. On read, may be unset if the member does not have an associated email.
    
    pub email: Option<String>,
    /// Output only. The Google Family to which this role applies.
    
    pub family: Option<Family>,
    /// Output only. The group to which this role applies.
    
    pub group: Option<Group>,
    /// Output only. The resource name.
    
    pub name: Option<String>,
    /// The role granted by this permission. The role determines the entity’s ability to read, write, and share notes.
    
    pub role: Option<PermissionRoleEnum>,
    /// Output only. The user to whom this role applies.
    
    pub user: Option<User>,
}

impl client::Part for Permission {}


/// The content of the note.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Section {
    /// Used if this section's content is a list.
    
    pub list: Option<ListContent>,
    /// Used if this section's content is a block of text. The length of the text content must be less than 20,000 characters.
    
    pub text: Option<TextContent>,
}

impl client::Part for Section {}


/// The block of text for a single text section or list item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TextContent {
    /// The text of the note. The limits on this vary with the specific field using this type.
    
    pub text: Option<String>,
}

impl client::Part for TextContent {}


/// Describes a single user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct User {
    /// The user's email.
    
    pub email: Option<String>,
}

impl client::Part for User {}


