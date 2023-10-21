use super::*;
/// An item with user information and settings.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get about](AboutGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct About {
    /// Information about supported additional roles per file type. The most specific type takes precedence.
    #[serde(rename="additionalRoleInfo")]
    
    pub additional_role_info: Option<Vec<AboutAdditionalRoleInfo>>,
    /// Whether the user can create shared drives.
    #[serde(rename="canCreateDrives")]
    
    pub can_create_drives: Option<bool>,
    /// Deprecated - use canCreateDrives instead.
    #[serde(rename="canCreateTeamDrives")]
    
    pub can_create_team_drives: Option<bool>,
    /// The domain sharing policy for the current user. Possible values are:  
    /// - allowed 
    /// - allowedWithWarning 
    /// - incomingOnly 
    /// - disallowed
    #[serde(rename="domainSharingPolicy")]
    
    pub domain_sharing_policy: Option<String>,
    /// A list of themes that are supported for shared drives.
    #[serde(rename="driveThemes")]
    
    pub drive_themes: Option<Vec<AboutDriveThemes>>,
    /// The ETag of the item.
    
    pub etag: Option<String>,
    /// The allowable export formats.
    #[serde(rename="exportFormats")]
    
    pub export_formats: Option<Vec<AboutExportFormats>>,
    /// List of additional features enabled on this account.
    
    pub features: Option<Vec<AboutFeatures>>,
    /// The palette of allowable folder colors as RGB hex strings.
    #[serde(rename="folderColorPalette")]
    
    pub folder_color_palette: Option<Vec<String>>,
    /// The allowable import formats.
    #[serde(rename="importFormats")]
    
    pub import_formats: Option<Vec<AboutImportFormats>>,
    /// A boolean indicating whether the authenticated app is installed by the authenticated user.
    #[serde(rename="isCurrentAppInstalled")]
    
    pub is_current_app_installed: Option<bool>,
    /// This is always drive#about.
    
    pub kind: Option<String>,
    /// The user's language or locale code, as defined by BCP 47, with some extensions from Unicode's LDML format (http://www.unicode.org/reports/tr35/).
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// The largest change id.
    #[serde(rename="largestChangeId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub largest_change_id: Option<i64>,
    /// List of max upload sizes for each file type. The most specific type takes precedence.
    #[serde(rename="maxUploadSizes")]
    
    pub max_upload_sizes: Option<Vec<AboutMaxUploadSizes>>,
    /// The name of the current user.
    
    pub name: Option<String>,
    /// The current user's ID as visible in the permissions collection.
    #[serde(rename="permissionId")]
    
    pub permission_id: Option<String>,
    /// The amount of storage quota used by different Google services.
    #[serde(rename="quotaBytesByService")]
    
    pub quota_bytes_by_service: Option<Vec<AboutQuotaBytesByService>>,
    /// The total number of quota bytes. This is only relevant when quotaType is LIMITED.
    #[serde(rename="quotaBytesTotal")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub quota_bytes_total: Option<i64>,
    /// The number of quota bytes used by Google Drive.
    #[serde(rename="quotaBytesUsed")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub quota_bytes_used: Option<i64>,
    /// The number of quota bytes used by all Google apps (Drive, Picasa, etc.).
    #[serde(rename="quotaBytesUsedAggregate")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub quota_bytes_used_aggregate: Option<i64>,
    /// The number of quota bytes used by trashed items.
    #[serde(rename="quotaBytesUsedInTrash")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub quota_bytes_used_in_trash: Option<i64>,
    /// The type of the user's storage quota. Possible values are:  
    /// - LIMITED 
    /// - UNLIMITED
    #[serde(rename="quotaType")]
    
    pub quota_type: Option<String>,
    /// The number of remaining change ids, limited to no more than 2500.
    #[serde(rename="remainingChangeIds")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub remaining_change_ids: Option<i64>,
    /// The id of the root folder.
    #[serde(rename="rootFolderId")]
    
    pub root_folder_id: Option<String>,
    /// A link back to this item.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Deprecated - use driveThemes instead.
    #[serde(rename="teamDriveThemes")]
    
    pub team_drive_themes: Option<Vec<AboutTeamDriveThemes>>,
    /// The authenticated user.
    
    pub user: Option<User>,
}

impl client::ResponseResult for About {}


/// The apps resource provides a list of the apps that a user has installed, with information about each app’s supported MIME types, file extensions, and other details.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get apps](AppGetCall) (response)
/// * [list apps](AppListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct App {
    /// Whether the app is authorized to access data on the user's Drive.
    
    pub authorized: Option<bool>,
    /// The template url to create a new file with this app in a given folder. The template will contain {folderId} to be replaced by the folder to create the new file in.
    #[serde(rename="createInFolderTemplate")]
    
    pub create_in_folder_template: Option<String>,
    /// The url to create a new file with this app.
    #[serde(rename="createUrl")]
    
    pub create_url: Option<String>,
    /// Whether the app has drive-wide scope. An app with drive-wide scope can access all files in the user's drive.
    #[serde(rename="hasDriveWideScope")]
    
    pub has_drive_wide_scope: Option<bool>,
    /// The various icons for the app.
    
    pub icons: Option<Vec<AppIcons>>,
    /// The ID of the app.
    
    pub id: Option<String>,
    /// Whether the app is installed.
    
    pub installed: Option<bool>,
    /// This is always drive#app.
    
    pub kind: Option<String>,
    /// A long description of the app.
    #[serde(rename="longDescription")]
    
    pub long_description: Option<String>,
    /// The name of the app.
    
    pub name: Option<String>,
    /// The type of object this app creates (e.g. Chart). If empty, the app name should be used instead.
    #[serde(rename="objectType")]
    
    pub object_type: Option<String>,
    /// The template url for opening files with this app. The template will contain {ids} and/or {exportIds} to be replaced by the actual file ids. See  Open Files  for the full documentation.
    #[serde(rename="openUrlTemplate")]
    
    pub open_url_template: Option<String>,
    /// The list of primary file extensions.
    #[serde(rename="primaryFileExtensions")]
    
    pub primary_file_extensions: Option<Vec<String>>,
    /// The list of primary mime types.
    #[serde(rename="primaryMimeTypes")]
    
    pub primary_mime_types: Option<Vec<String>>,
    /// The ID of the product listing for this app.
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
    /// A link to the product listing for this app.
    #[serde(rename="productUrl")]
    
    pub product_url: Option<String>,
    /// The list of secondary file extensions.
    #[serde(rename="secondaryFileExtensions")]
    
    pub secondary_file_extensions: Option<Vec<String>>,
    /// The list of secondary mime types.
    #[serde(rename="secondaryMimeTypes")]
    
    pub secondary_mime_types: Option<Vec<String>>,
    /// A short description of the app.
    #[serde(rename="shortDescription")]
    
    pub short_description: Option<String>,
    /// Whether this app supports creating new objects.
    #[serde(rename="supportsCreate")]
    
    pub supports_create: Option<bool>,
    /// Whether this app supports importing from Docs Editors.
    #[serde(rename="supportsImport")]
    
    pub supports_import: Option<bool>,
    /// Whether this app supports opening more than one file.
    #[serde(rename="supportsMultiOpen")]
    
    pub supports_multi_open: Option<bool>,
    /// Whether this app supports creating new files when offline.
    #[serde(rename="supportsOfflineCreate")]
    
    pub supports_offline_create: Option<bool>,
    /// Whether the app is selected as the default handler for the types it supports.
    #[serde(rename="useByDefault")]
    
    pub use_by_default: Option<bool>,
}

impl client::Resource for App {}
impl client::ResponseResult for App {}


/// A list of third-party applications which the user has installed or given access to Google Drive.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list apps](AppListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppList {
    /// List of app IDs that the user has specified to use by default. The list is in reverse-priority order (lowest to highest).
    #[serde(rename="defaultAppIds")]
    
    pub default_app_ids: Option<Vec<String>>,
    /// The ETag of the list.
    
    pub etag: Option<String>,
    /// The list of apps.
    
    pub items: Option<Vec<App>>,
    /// This is always drive#appList.
    
    pub kind: Option<String>,
    /// A link back to this list.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::ResponseResult for AppList {}


/// Representation of a change to a file or shared drive.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get changes](ChangeGetCall) (response)
/// * [get start page token changes](ChangeGetStartPageTokenCall) (none)
/// * [list changes](ChangeListCall) (none)
/// * [watch changes](ChangeWatchCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Change {
    /// The type of the change. Possible values are file and drive.
    #[serde(rename="changeType")]
    
    pub change_type: Option<String>,
    /// Whether the file or shared drive has been removed from this list of changes, for example by deletion or loss of access.
    
    pub deleted: Option<bool>,
    /// The updated state of the shared drive. Present if the changeType is drive, the user is still a member of the shared drive, and the shared drive has not been deleted.
    
    pub drive: Option<Drive>,
    /// The ID of the shared drive associated with this change.
    #[serde(rename="driveId")]
    
    pub drive_id: Option<String>,
    /// The updated state of the file. Present if the type is file and the file has not been removed from this list of changes.
    
    pub file: Option<File>,
    /// The ID of the file associated with this change.
    #[serde(rename="fileId")]
    
    pub file_id: Option<String>,
    /// The ID of the change.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// This is always drive#change.
    
    pub kind: Option<String>,
    /// The time of this modification.
    #[serde(rename="modificationDate")]
    
    pub modification_date: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A link back to this change.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Deprecated - use drive instead.
    #[serde(rename="teamDrive")]
    
    pub team_drive: Option<TeamDrive>,
    /// Deprecated - use driveId instead.
    #[serde(rename="teamDriveId")]
    
    pub team_drive_id: Option<String>,
    /// Deprecated - use changeType instead.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Resource for Change {}
impl client::ResponseResult for Change {}


/// A list of changes for a user.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list changes](ChangeListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChangeList {
    /// The ETag of the list.
    
    pub etag: Option<String>,
    /// The list of changes. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched.
    
    pub items: Option<Vec<Change>>,
    /// This is always drive#changeList.
    
    pub kind: Option<String>,
    /// The current largest change ID.
    #[serde(rename="largestChangeId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub largest_change_id: Option<i64>,
    /// The starting page token for future changes. This will be present only if the end of the current changes list has been reached.
    #[serde(rename="newStartPageToken")]
    
    pub new_start_page_token: Option<String>,
    /// A link to the next page of changes.
    #[serde(rename="nextLink")]
    
    pub next_link: Option<String>,
    /// The page token for the next page of changes. This will be absent if the end of the changes list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A link back to this list.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::ResponseResult for ChangeList {}


/// An notification channel used to watch for resource changes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [watch changes](ChangeWatchCall) (request|response)
/// * [stop channels](ChannelStopCall) (request)
/// * [watch files](FileWatchCall) (request|response)
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


/// A list of children of a file.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list children](ChildListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChildList {
    /// The ETag of the list.
    
    pub etag: Option<String>,
    /// The list of children. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched.
    
    pub items: Option<Vec<ChildReference>>,
    /// This is always drive#childList.
    
    pub kind: Option<String>,
    /// A link to the next page of children.
    #[serde(rename="nextLink")]
    
    pub next_link: Option<String>,
    /// The page token for the next page of children. This will be absent if the end of the children list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A link back to this list.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::ResponseResult for ChildList {}


/// A reference to a folder’s child.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get children](ChildGetCall) (response)
/// * [insert children](ChildInsertCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChildReference {
    /// A link to the child.
    #[serde(rename="childLink")]
    
    pub child_link: Option<String>,
    /// The ID of the child.
    
    pub id: Option<String>,
    /// This is always drive#childReference.
    
    pub kind: Option<String>,
    /// A link back to this reference.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::RequestValue for ChildReference {}
impl client::ResponseResult for ChildReference {}


/// A comment on a file in Google Drive.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete comments](CommentDeleteCall) (none)
/// * [get comments](CommentGetCall) (response)
/// * [insert comments](CommentInsertCall) (request|response)
/// * [list comments](CommentListCall) (none)
/// * [patch comments](CommentPatchCall) (request|response)
/// * [update comments](CommentUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Comment {
    /// A region of the document represented as a JSON string. For details on defining anchor properties, refer to  Add comments and replies.
    
    pub anchor: Option<String>,
    /// The author of the comment. The author's email address and permission ID will not be populated.
    
    pub author: Option<User>,
    /// The ID of the comment.
    #[serde(rename="commentId")]
    
    pub comment_id: Option<String>,
    /// The plain text content used to create this comment. This is not HTML safe and should only be used as a starting point to make edits to a comment's content.
    
    pub content: Option<String>,
    /// The context of the file which is being commented on.
    
    pub context: Option<CommentContext>,
    /// The date when this comment was first created.
    #[serde(rename="createdDate")]
    
    pub created_date: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Whether this comment has been deleted. If a comment has been deleted the content will be cleared and this will only represent a comment that once existed.
    
    pub deleted: Option<bool>,
    /// The file which this comment is addressing.
    #[serde(rename="fileId")]
    
    pub file_id: Option<String>,
    /// The title of the file which this comment is addressing.
    #[serde(rename="fileTitle")]
    
    pub file_title: Option<String>,
    /// HTML formatted content for this comment.
    #[serde(rename="htmlContent")]
    
    pub html_content: Option<String>,
    /// This is always drive#comment.
    
    pub kind: Option<String>,
    /// The date when this comment or any of its replies were last modified.
    #[serde(rename="modifiedDate")]
    
    pub modified_date: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Replies to this post.
    
    pub replies: Option<Vec<CommentReply>>,
    /// A link back to this comment.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// The status of this comment. Status can be changed by posting a reply to a comment with the desired status.  
    /// - "open" - The comment is still open. 
    /// - "resolved" - The comment has been resolved by one of its replies.
    
    pub status: Option<String>,
}

impl client::RequestValue for Comment {}
impl client::Resource for Comment {}
impl client::ResponseResult for Comment {}


/// A list of comments on a file in Google Drive.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list comments](CommentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommentList {
    /// The list of comments. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched.
    
    pub items: Option<Vec<Comment>>,
    /// This is always drive#commentList.
    
    pub kind: Option<String>,
    /// A link to the next page of comments.
    #[serde(rename="nextLink")]
    
    pub next_link: Option<String>,
    /// The page token for the next page of comments. This will be absent if the end of the comments list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A link back to this list.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::ResponseResult for CommentList {}


/// A comment on a file in Google Drive.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get replies](ReplyGetCall) (response)
/// * [insert replies](ReplyInsertCall) (request|response)
/// * [patch replies](ReplyPatchCall) (request|response)
/// * [update replies](ReplyUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommentReply {
    /// The author of the reply. The author's email address and permission ID will not be populated.
    
    pub author: Option<User>,
    /// The plain text content used to create this reply. This is not HTML safe and should only be used as a starting point to make edits to a reply's content. This field is required on inserts if no verb is specified (resolve/reopen).
    
    pub content: Option<String>,
    /// The date when this reply was first created.
    #[serde(rename="createdDate")]
    
    pub created_date: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Whether this reply has been deleted. If a reply has been deleted the content will be cleared and this will only represent a reply that once existed.
    
    pub deleted: Option<bool>,
    /// HTML formatted content for this reply.
    #[serde(rename="htmlContent")]
    
    pub html_content: Option<String>,
    /// This is always drive#commentReply.
    
    pub kind: Option<String>,
    /// The date when this reply was last modified.
    #[serde(rename="modifiedDate")]
    
    pub modified_date: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The ID of the reply.
    #[serde(rename="replyId")]
    
    pub reply_id: Option<String>,
    /// The action this reply performed to the parent comment. When creating a new reply this is the action to be perform to the parent comment. Possible values are:  
    /// - "resolve" - To resolve a comment. 
    /// - "reopen" - To reopen (un-resolve) a comment.
    
    pub verb: Option<String>,
}

impl client::RequestValue for CommentReply {}
impl client::ResponseResult for CommentReply {}


/// A list of replies to a comment on a file in Google Drive.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list replies](ReplyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommentReplyList {
    /// The list of replies. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched.
    
    pub items: Option<Vec<CommentReply>>,
    /// This is always drive#commentReplyList.
    
    pub kind: Option<String>,
    /// A link to the next page of replies.
    #[serde(rename="nextLink")]
    
    pub next_link: Option<String>,
    /// The page token for the next page of replies. This will be absent if the end of the replies list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A link back to this list.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::ResponseResult for CommentReplyList {}


/// A restriction for accessing the content of the file.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContentRestriction {
    /// Whether the content of the file is read-only. If a file is read-only, a new revision of the file may not be added, comments may not be added or modified, and the title of the file may not be modified.
    #[serde(rename="readOnly")]
    
    pub read_only: Option<bool>,
    /// Reason for why the content of the file is restricted. This is only mutable on requests that also set readOnly=true.
    
    pub reason: Option<String>,
    /// The user who set the content restriction. Only populated if readOnly is true.
    #[serde(rename="restrictingUser")]
    
    pub restricting_user: Option<User>,
    /// The time at which the content restriction was set (formatted RFC 3339 timestamp). Only populated if readOnly is true.
    #[serde(rename="restrictionDate")]
    
    pub restriction_date: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The type of the content restriction. Currently the only possible value is globalContentRestriction.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for ContentRestriction {}


/// Representation of a shared drive.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete drives](DriveDeleteCall) (none)
/// * [get drives](DriveGetCall) (response)
/// * [hide drives](DriveHideCall) (response)
/// * [insert drives](DriveInsertCall) (request|response)
/// * [list drives](DriveListCall) (none)
/// * [unhide drives](DriveUnhideCall) (response)
/// * [update drives](DriveUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Drive {
    /// An image file and cropping parameters from which a background image for this shared drive is set. This is a write only field; it can only be set on drive.drives.update requests that don't set themeId. When specified, all fields of the backgroundImageFile must be set.
    #[serde(rename="backgroundImageFile")]
    
    pub background_image_file: Option<DriveBackgroundImageFile>,
    /// A short-lived link to this shared drive's background image.
    #[serde(rename="backgroundImageLink")]
    
    pub background_image_link: Option<String>,
    /// Capabilities the current user has on this shared drive.
    
    pub capabilities: Option<DriveCapabilities>,
    /// The color of this shared drive as an RGB hex string. It can only be set on a drive.drives.update request that does not set themeId.
    #[serde(rename="colorRgb")]
    
    pub color_rgb: Option<String>,
    /// The time at which the shared drive was created (RFC 3339 date-time).
    #[serde(rename="createdDate")]
    
    pub created_date: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Whether the shared drive is hidden from default view.
    
    pub hidden: Option<bool>,
    /// The ID of this shared drive which is also the ID of the top level folder of this shared drive.
    
    pub id: Option<String>,
    /// This is always drive#drive
    
    pub kind: Option<String>,
    /// The name of this shared drive.
    
    pub name: Option<String>,
    /// The organizational unit of this shared drive. This field is only populated on drives.list responses when the useDomainAdminAccess parameter is set to true.
    #[serde(rename="orgUnitId")]
    
    pub org_unit_id: Option<String>,
    /// A set of restrictions that apply to this shared drive or items inside this shared drive.
    
    pub restrictions: Option<DriveRestrictions>,
    /// The ID of the theme from which the background image and color will be set. The set of possible driveThemes can be retrieved from a drive.about.get response. When not specified on a drive.drives.insert request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set colorRgb or backgroundImageFile.
    #[serde(rename="themeId")]
    
    pub theme_id: Option<String>,
}

impl client::RequestValue for Drive {}
impl client::Resource for Drive {}
impl client::ResponseResult for Drive {}


/// A list of shared drives.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list drives](DriveListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DriveList {
    /// The list of shared drives. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched.
    
    pub items: Option<Vec<Drive>>,
    /// This is always drive#driveList
    
    pub kind: Option<String>,
    /// The page token for the next page of shared drives. This will be absent if the end of the list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for DriveList {}


/// The metadata for a file.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [copy files](FileCopyCall) (request|response)
/// * [delete files](FileDeleteCall) (none)
/// * [empty trash files](FileEmptyTrashCall) (none)
/// * [export files](FileExportCall) (none)
/// * [generate ids files](FileGenerateIdCall) (none)
/// * [get files](FileGetCall) (response)
/// * [insert files](FileInsertCall) (request|response)
/// * [list files](FileListCall) (none)
/// * [list labels files](FileListLabelCall) (none)
/// * [modify labels files](FileModifyLabelCall) (none)
/// * [patch files](FilePatchCall) (request|response)
/// * [touch files](FileTouchCall) (response)
/// * [trash files](FileTrashCall) (response)
/// * [untrash files](FileUntrashCall) (response)
/// * [update files](FileUpdateCall) (request|response)
/// * [watch files](FileWatchCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct File {
    /// A link for opening the file in a relevant Google editor or viewer.
    #[serde(rename="alternateLink")]
    
    pub alternate_link: Option<String>,
    /// Whether this file is in the Application Data folder.
    #[serde(rename="appDataContents")]
    
    pub app_data_contents: Option<bool>,
    /// Deprecated: use capabilities/canComment.
    #[serde(rename="canComment")]
    
    pub can_comment: Option<bool>,
    /// Deprecated: use capabilities/canReadRevisions.
    #[serde(rename="canReadRevisions")]
    
    pub can_read_revisions: Option<bool>,
    /// Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
    
    pub capabilities: Option<FileCapabilities>,
    /// Restrictions for accessing the content of the file. Only populated if such a restriction exists.
    #[serde(rename="contentRestrictions")]
    
    pub content_restrictions: Option<Vec<ContentRestriction>>,
    /// Whether the options to copy, print, or download this file, should be disabled for readers and commenters.
    #[serde(rename="copyRequiresWriterPermission")]
    
    pub copy_requires_writer_permission: Option<bool>,
    /// Deprecated: use capabilities/canCopy.
    
    pub copyable: Option<bool>,
    /// Create time for this file (formatted RFC 3339 timestamp).
    #[serde(rename="createdDate")]
    
    pub created_date: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A link to open this file with the user's default app for this file. Only populated when the drive.apps.readonly scope is used.
    #[serde(rename="defaultOpenWithLink")]
    
    pub default_open_with_link: Option<String>,
    /// A short description of the file.
    
    pub description: Option<String>,
    /// Short lived download URL for the file. This field is only populated for files with content stored in Google Drive; it isn't populated for Docs Editors or shortcut files.
    #[serde(rename="downloadUrl")]
    
    pub download_url: Option<String>,
    /// ID of the shared drive the file resides in. Only populated for items in shared drives.
    #[serde(rename="driveId")]
    
    pub drive_id: Option<String>,
    /// Deprecated: use capabilities/canEdit.
    
    pub editable: Option<bool>,
    /// A link for embedding the file.
    #[serde(rename="embedLink")]
    
    pub embed_link: Option<String>,
    /// ETag of the file.
    
    pub etag: Option<String>,
    /// Whether this file has been explicitly trashed, as opposed to recursively trashed.
    #[serde(rename="explicitlyTrashed")]
    
    pub explicitly_trashed: Option<bool>,
    /// Links for exporting Docs Editors files to specific formats.
    #[serde(rename="exportLinks")]
    
    pub export_links: Option<HashMap<String, String>>,
    /// The final component of fullFileExtension with trailing text that does not appear to be part of the extension removed. This field is only populated for files with content stored in Google Drive; it isn't populated for Docs Editors or shortcut files.
    #[serde(rename="fileExtension")]
    
    pub file_extension: Option<String>,
    /// The size of the file in bytes. This field is populated for files with content stored in Google Drive and for files in Docs Editors; it isn't populated for shortcut files.
    #[serde(rename="fileSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub file_size: Option<i64>,
    /// Folder color as an RGB hex string if the file is a folder or a shortcut to a folder. The list of supported colors is available in the folderColorPalette field of the About resource. If an unsupported color is specified, it will be changed to the closest color in the palette.
    #[serde(rename="folderColorRgb")]
    
    pub folder_color_rgb: Option<String>,
    /// The full file extension; extracted from the title. May contain multiple concatenated extensions, such as "tar.gz". Removing an extension from the title does not clear this field; however, changing the extension on the title does update this field. This field is only populated for files with content stored in Google Drive; it isn't populated for Docs Editors or shortcut files.
    #[serde(rename="fullFileExtension")]
    
    pub full_file_extension: Option<String>,
    /// Whether there are permissions directly on this file. This field is only populated for items in shared drives.
    #[serde(rename="hasAugmentedPermissions")]
    
    pub has_augmented_permissions: Option<bool>,
    /// Whether this file has a thumbnail. This does not indicate whether the requesting app has access to the thumbnail. To check access, look for the presence of the thumbnailLink field.
    #[serde(rename="hasThumbnail")]
    
    pub has_thumbnail: Option<bool>,
    /// The ID of the file's head revision. This field is only populated for files with content stored in Google Drive; it isn't populated for Docs Editors or shortcut files.
    #[serde(rename="headRevisionId")]
    
    pub head_revision_id: Option<String>,
    /// A link to the file's icon.
    #[serde(rename="iconLink")]
    
    pub icon_link: Option<String>,
    /// The ID of the file.
    
    pub id: Option<String>,
    /// Metadata about image media. This will only be present for image types, and its contents will depend on what can be parsed from the image content.
    #[serde(rename="imageMediaMetadata")]
    
    pub image_media_metadata: Option<FileImageMediaMetadata>,
    /// Indexable text attributes for the file (can only be written). For more information, see Manage file metadata.
    #[serde(rename="indexableText")]
    
    pub indexable_text: Option<FileIndexableText>,
    /// Whether the file was created or opened by the requesting app.
    #[serde(rename="isAppAuthorized")]
    
    pub is_app_authorized: Option<bool>,
    /// The type of file. This is always drive#file.
    
    pub kind: Option<String>,
    /// An overview of the labels on the file.
    #[serde(rename="labelInfo")]
    
    pub label_info: Option<FileLabelInfo>,
    /// A group of labels for the file.
    
    pub labels: Option<FileLabels>,
    /// The last user to modify this file.
    #[serde(rename="lastModifyingUser")]
    
    pub last_modifying_user: Option<User>,
    /// Name of the last user to modify this file.
    #[serde(rename="lastModifyingUserName")]
    
    pub last_modifying_user_name: Option<String>,
    /// Last time this file was viewed by the user (formatted RFC 3339 timestamp).
    #[serde(rename="lastViewedByMeDate")]
    
    pub last_viewed_by_me_date: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Contains details about the link URLs that clients are using to refer to this item.
    #[serde(rename="linkShareMetadata")]
    
    pub link_share_metadata: Option<FileLinkShareMetadata>,
    /// Deprecated.
    #[serde(rename="markedViewedByMeDate")]
    
    pub marked_viewed_by_me_date: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// An MD5 checksum for the content of this file. This field is only populated for files with content stored in Google Drive; it isn't populated for Docs Editors or shortcut files.
    #[serde(rename="md5Checksum")]
    
    pub md5_checksum: Option<String>,
    /// The MIME type of the file. This is only mutable on update when uploading new content. This field can be left blank, and the mimetype will be determined from the uploaded content's MIME type.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
    /// Last time this file was modified by the user (formatted RFC 3339 timestamp). Note that setting modifiedDate will also update the modifiedByMe date for the user which set the date.
    #[serde(rename="modifiedByMeDate")]
    
    pub modified_by_me_date: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Last time this file was modified by anyone (formatted RFC 3339 timestamp). This is only mutable on update when the setModifiedDate parameter is set.
    #[serde(rename="modifiedDate")]
    
    pub modified_date: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A map of the id of each of the user's apps to a link to open this file with that app. Only populated when the drive.apps.readonly scope is used.
    #[serde(rename="openWithLinks")]
    
    pub open_with_links: Option<HashMap<String, String>>,
    /// The original filename of the uploaded content if available, or else the original value of the title field. This is only available for files with binary content in Google Drive.
    #[serde(rename="originalFilename")]
    
    pub original_filename: Option<String>,
    /// Whether the file is owned by the current user. Not populated for items in shared drives.
    #[serde(rename="ownedByMe")]
    
    pub owned_by_me: Option<bool>,
    /// Name(s) of the owner(s) of this file. Not populated for items in shared drives.
    #[serde(rename="ownerNames")]
    
    pub owner_names: Option<Vec<String>>,
    /// The owner of this file. Only certain legacy files may have more than one owner. This field isn't populated for items in shared drives.
    
    pub owners: Option<Vec<User>>,
    /// Collection of parent folders which contain this file.
    /// If not specified as part of an insert request, the file will be placed directly in the user's My Drive folder. If not specified as part of a copy request, the file will inherit any discoverable parents of the source file. Update requests can also use the addParents and removeParents parameters to modify the parents list.
    
    pub parents: Option<Vec<ParentReference>>,
    /// List of permission IDs for users with access to this file.
    #[serde(rename="permissionIds")]
    
    pub permission_ids: Option<Vec<String>>,
    /// The list of permissions for users with access to this file. Not populated for items in shared drives.
    
    pub permissions: Option<Vec<Permission>>,
    /// The list of properties.
    
    pub properties: Option<Vec<Property>>,
    /// The number of quota bytes used by this file.
    #[serde(rename="quotaBytesUsed")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub quota_bytes_used: Option<i64>,
    /// A key needed to access the item via a shared link.
    #[serde(rename="resourceKey")]
    
    pub resource_key: Option<String>,
    /// A link back to this file.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// The SHA1 checksum associated with this file, if available. This field is only populated for files with content stored in Google Drive; it isn't populated for Docs Editors or shortcut files.
    #[serde(rename="sha1Checksum")]
    
    pub sha1_checksum: Option<String>,
    /// The SHA256 checksum associated with this file, if available. This field is only populated for files with content stored in Google Drive; it isn't populated for Docs Editors or shortcut files.
    #[serde(rename="sha256Checksum")]
    
    pub sha256_checksum: Option<String>,
    /// Deprecated: use capabilities/canShare.
    
    pub shareable: Option<bool>,
    /// Whether the file has been shared. Not populated for items in shared drives.
    
    pub shared: Option<bool>,
    /// Time at which this file was shared with the user (formatted RFC 3339 timestamp).
    #[serde(rename="sharedWithMeDate")]
    
    pub shared_with_me_date: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// User that shared the item with the current user, if available.
    #[serde(rename="sharingUser")]
    
    pub sharing_user: Option<User>,
    /// Shortcut file details. Only populated for shortcut files, which have the mimeType field set to application/vnd.google-apps.shortcut.
    #[serde(rename="shortcutDetails")]
    
    pub shortcut_details: Option<FileShortcutDetails>,
    /// The list of spaces which contain the file. Supported values are 'drive', 'appDataFolder' and 'photos'.
    
    pub spaces: Option<Vec<String>>,
    /// Deprecated - use driveId instead.
    #[serde(rename="teamDriveId")]
    
    pub team_drive_id: Option<String>,
    /// A thumbnail for the file. This will only be used if a standard thumbnail cannot be generated.
    
    pub thumbnail: Option<FileThumbnail>,
    /// A short-lived link to the file's thumbnail. Typically lasts on the order of hours. Only populated when the requesting app can access the file's content. If the file isn't shared publicly, the URL returned in Files.thumbnailLink must be fetched using a credentialed request.
    #[serde(rename="thumbnailLink")]
    
    pub thumbnail_link: Option<String>,
    /// The thumbnail version for use in thumbnail cache invalidation.
    #[serde(rename="thumbnailVersion")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub thumbnail_version: Option<i64>,
    /// The title of this file. Note that for immutable items such as the top level folders of shared drives, My Drive root folder, and Application Data folder the title is constant.
    
    pub title: Option<String>,
    /// The time that the item was trashed (formatted RFC 3339 timestamp). Only populated for items in shared drives.
    #[serde(rename="trashedDate")]
    
    pub trashed_date: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// If the file has been explicitly trashed, the user who trashed it. Only populated for items in shared drives.
    #[serde(rename="trashingUser")]
    
    pub trashing_user: Option<User>,
    /// The permissions for the authenticated user on this file.
    #[serde(rename="userPermission")]
    
    pub user_permission: Option<Permission>,
    /// A monotonically increasing version number for the file. This reflects every change made to the file on the server, even those not visible to the requesting user.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub version: Option<i64>,
    /// Metadata about video media. This will only be present for video types.
    #[serde(rename="videoMediaMetadata")]
    
    pub video_media_metadata: Option<FileVideoMediaMetadata>,
    /// A link for downloading the content of the file in a browser using cookie based authentication. In cases where the content is shared publicly, the content can be downloaded without any credentials.
    #[serde(rename="webContentLink")]
    
    pub web_content_link: Option<String>,
    /// A link only available on public folders for viewing their static web assets (HTML, CSS, JS, etc) via Google Drive's Website Hosting.
    #[serde(rename="webViewLink")]
    
    pub web_view_link: Option<String>,
    /// Whether writers can share the document with other users. Not populated for items in shared drives.
    #[serde(rename="writersCanShare")]
    
    pub writers_can_share: Option<bool>,
}

impl client::RequestValue for File {}
impl client::Resource for File {}
impl client::ResponseResult for File {}


/// A list of files.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list files](FileListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileList {
    /// The ETag of the list.
    
    pub etag: Option<String>,
    /// Whether the search process was incomplete. If true, then some search results may be missing, since all documents were not searched. This may occur when searching multiple drives with the "allDrives" corpora, but all corpora could not be searched. When this happens, it is suggested that clients narrow their query by choosing a different corpus such as "default" or "drive".
    #[serde(rename="incompleteSearch")]
    
    pub incomplete_search: Option<bool>,
    /// The list of files. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched.
    
    pub items: Option<Vec<File>>,
    /// This is always drive#fileList.
    
    pub kind: Option<String>,
    /// A link to the next page of files.
    #[serde(rename="nextLink")]
    
    pub next_link: Option<String>,
    /// The page token for the next page of files. This will be absent if the end of the files list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A link back to this list.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::ResponseResult for FileList {}


/// A list of generated IDs which can be provided in insert requests
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [generate ids files](FileGenerateIdCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GeneratedIds {
    /// The IDs generated for the requesting user in the specified space.
    
    pub ids: Option<Vec<String>>,
    /// This is always drive#generatedIds
    
    pub kind: Option<String>,
    /// The type of file that can be created with these IDs.
    
    pub space: Option<String>,
}

impl client::ResponseResult for GeneratedIds {}


/// Representation of a label and its fields.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Label {
    /// A map of the label's fields keyed by the field ID.
    
    pub fields: Option<HashMap<String, LabelField>>,
    /// The ID of the label.
    
    pub id: Option<String>,
    /// This is always drive#label
    
    pub kind: Option<String>,
    /// The revision ID of the label.
    #[serde(rename="revisionId")]
    
    pub revision_id: Option<String>,
}

impl client::Part for Label {}


/// Representation of a label field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LabelField {
    /// Only present if valueType is dateString. RFC 3339 formatted date: YYYY-MM-DD.
    #[serde(rename="dateString")]
    
    pub date_string: Option<Vec<client::chrono::NaiveDate>>,
    /// The identifier of this field.
    
    pub id: Option<String>,
    /// Only present if valueType is integer.
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub integer: Option<Vec<i64>>,
    /// This is always drive#labelField.
    
    pub kind: Option<String>,
    /// Only present if valueType is selection.
    
    pub selection: Option<Vec<String>>,
    /// Only present if valueType is text.
    
    pub text: Option<Vec<String>>,
    /// Only present if valueType is user.
    
    pub user: Option<Vec<User>>,
    /// The field type. While new values may be supported in the future, the following are currently allowed:  
    /// - dateString 
    /// - integer 
    /// - selection 
    /// - text 
    /// - user
    #[serde(rename="valueType")]
    
    pub value_type: Option<String>,
}

impl client::Part for LabelField {}


/// A modification to a label's field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LabelFieldModification {
    /// The ID of the field to be modified.
    #[serde(rename="fieldId")]
    
    pub field_id: Option<String>,
    /// This is always drive#labelFieldModification.
    
    pub kind: Option<String>,
    /// Replaces a dateString field with these new values. The values must be strings in the RFC 3339 full-date format: YYYY-MM-DD.
    #[serde(rename="setDateValues")]
    
    pub set_date_values: Option<Vec<client::chrono::NaiveDate>>,
    /// Replaces an integer field with these new values.
    #[serde(rename="setIntegerValues")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub set_integer_values: Option<Vec<i64>>,
    /// Replaces a selection field with these new values.
    #[serde(rename="setSelectionValues")]
    
    pub set_selection_values: Option<Vec<String>>,
    /// Replaces a text field with these new values.
    #[serde(rename="setTextValues")]
    
    pub set_text_values: Option<Vec<String>>,
    /// Replaces a user field with these new values. The values must be valid email addresses.
    #[serde(rename="setUserValues")]
    
    pub set_user_values: Option<Vec<String>>,
    /// Unsets the values for this field.
    #[serde(rename="unsetValues")]
    
    pub unset_values: Option<bool>,
}

impl client::Part for LabelFieldModification {}


/// A list of labels.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list labels files](FileListLabelCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LabelList {
    /// The list of labels.
    
    pub items: Option<Vec<Label>>,
    /// This is always drive#labelList
    
    pub kind: Option<String>,
    /// The page token for the next page of labels. This field will be absent if the end of the list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for LabelList {}


/// A modification to a label on a file. A LabelModification can be used to apply a label to a file, update an existing label on a file, or remove a label from a file.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LabelModification {
    /// The list of modifications to this label's fields.
    #[serde(rename="fieldModifications")]
    
    pub field_modifications: Option<Vec<LabelFieldModification>>,
    /// This is always drive#labelModification.
    
    pub kind: Option<String>,
    /// The ID of the label to modify.
    #[serde(rename="labelId")]
    
    pub label_id: Option<String>,
    /// If true, the label will be removed from the file.
    #[serde(rename="removeLabel")]
    
    pub remove_label: Option<bool>,
}

impl client::Part for LabelModification {}


/// A request to modify the set of labels on a file. This request may contain many modifications that will either all succeed or all fail transactionally.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [modify labels files](FileModifyLabelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ModifyLabelsRequest {
    /// This is always drive#modifyLabelsRequest
    
    pub kind: Option<String>,
    /// The list of modifications to apply to the labels on the file.
    #[serde(rename="labelModifications")]
    
    pub label_modifications: Option<Vec<LabelModification>>,
}

impl client::RequestValue for ModifyLabelsRequest {}


/// Response to a ModifyLabels request. This contains only those labels which were added or updated by the request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [modify labels files](FileModifyLabelCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ModifyLabelsResponse {
    /// This is always drive#modifyLabelsResponse
    
    pub kind: Option<String>,
    /// The list of labels which were added or updated by the request.
    #[serde(rename="modifiedLabels")]
    
    pub modified_labels: Option<Vec<Label>>,
}

impl client::ResponseResult for ModifyLabelsResponse {}


/// A list of a file’s parents.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list parents](ParentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ParentList {
    /// The ETag of the list.
    
    pub etag: Option<String>,
    /// The list of parents.
    
    pub items: Option<Vec<ParentReference>>,
    /// This is always drive#parentList.
    
    pub kind: Option<String>,
    /// A link back to this list.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::ResponseResult for ParentList {}


/// A reference to a file’s parent.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get parents](ParentGetCall) (response)
/// * [insert parents](ParentInsertCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ParentReference {
    /// The ID of the parent.
    
    pub id: Option<String>,
    /// Whether or not the parent is the root folder.
    #[serde(rename="isRoot")]
    
    pub is_root: Option<bool>,
    /// This is always drive#parentReference.
    
    pub kind: Option<String>,
    /// A link to the parent.
    #[serde(rename="parentLink")]
    
    pub parent_link: Option<String>,
    /// A link back to this reference.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::RequestValue for ParentReference {}
impl client::ResponseResult for ParentReference {}


/// A permission for a file.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete permissions](PermissionDeleteCall) (none)
/// * [get permissions](PermissionGetCall) (response)
/// * [get id for email permissions](PermissionGetIdForEmailCall) (none)
/// * [insert permissions](PermissionInsertCall) (request|response)
/// * [list permissions](PermissionListCall) (none)
/// * [patch permissions](PermissionPatchCall) (request|response)
/// * [update permissions](PermissionUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Permission {
    /// Additional roles for this user. Only commenter is currently allowed, though more may be supported in the future.
    #[serde(rename="additionalRoles")]
    
    pub additional_roles: Option<Vec<String>>,
    /// Deprecated.
    #[serde(rename="authKey")]
    
    pub auth_key: Option<String>,
    /// Whether the account associated with this permission has been deleted. This field only pertains to user and group permissions.
    
    pub deleted: Option<bool>,
    /// The domain name of the entity this permission refers to. This is an output-only field which is present when the permission type is user, group or domain.
    
    pub domain: Option<String>,
    /// The email address of the user or group this permission refers to. This is an output-only field which is present when the permission type is user or group.
    #[serde(rename="emailAddress")]
    
    pub email_address: Option<String>,
    /// The ETag of the permission.
    
    pub etag: Option<String>,
    /// The time at which this permission will expire (RFC 3339 date-time). Expiration dates have the following restrictions:  
    /// - They cannot be set on shared drive items 
    /// - They can only be set on user and group permissions 
    /// - The date must be in the future 
    /// - The date cannot be more than a year in the future
    #[serde(rename="expirationDate")]
    
    pub expiration_date: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The ID of the user this permission refers to, and identical to the permissionId in the About and Files resources. When making a drive.permissions.insert request, exactly one of the id or value fields must be specified unless the permission type is anyone, in which case both id and value are ignored.
    
    pub id: Option<String>,
    /// This is always drive#permission.
    
    pub kind: Option<String>,
    /// The name for this permission.
    
    pub name: Option<String>,
    /// Whether the account associated with this permission is a pending owner. Only populated for user type permissions for files that are not in a shared drive.
    #[serde(rename="pendingOwner")]
    
    pub pending_owner: Option<bool>,
    /// Details of whether the permissions on this shared drive item are inherited or directly on this item. This is an output-only field which is present only for shared drive items.
    #[serde(rename="permissionDetails")]
    
    pub permission_details: Option<Vec<PermissionPermissionDetails>>,
    /// A link to the profile photo, if available.
    #[serde(rename="photoLink")]
    
    pub photo_link: Option<String>,
    /// The primary role for this user. While new values may be supported in the future, the following are currently allowed:  
    /// - owner 
    /// - organizer 
    /// - fileOrganizer 
    /// - writer 
    /// - reader
    
    pub role: Option<String>,
    /// A link back to this permission.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Deprecated - use permissionDetails instead.
    #[serde(rename="teamDrivePermissionDetails")]
    
    pub team_drive_permission_details: Option<Vec<PermissionTeamDrivePermissionDetails>>,
    /// The account type. Allowed values are:  
    /// - user 
    /// - group 
    /// - domain 
    /// - anyone
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The email address or domain name for the entity. This is used during inserts and is not populated in responses. When making a drive.permissions.insert request, exactly one of the id or value fields must be specified unless the permission type is anyone, in which case both id and value are ignored.
    
    pub value: Option<String>,
    /// Indicates the view for this permission. Only populated for permissions that belong to a view. published is the only supported value.
    
    pub view: Option<String>,
    /// Whether the link is required for this permission.
    #[serde(rename="withLink")]
    
    pub with_link: Option<bool>,
}

impl client::RequestValue for Permission {}
impl client::Resource for Permission {}
impl client::ResponseResult for Permission {}


/// An ID for a user or group as seen in Permission items.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get id for email permissions](PermissionGetIdForEmailCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PermissionId {
    /// The permission ID.
    
    pub id: Option<String>,
    /// This is always drive#permissionId.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for PermissionId {}


/// A list of permissions associated with a file.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list permissions](PermissionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PermissionList {
    /// The ETag of the list.
    
    pub etag: Option<String>,
    /// The list of permissions.
    
    pub items: Option<Vec<Permission>>,
    /// This is always drive#permissionList.
    
    pub kind: Option<String>,
    /// The page token for the next page of permissions. This field will be absent if the end of the permissions list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A link back to this list.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::ResponseResult for PermissionList {}


/// A key-value pair attached to a file that is either public or private to an application.
/// The following limits apply to file properties:
/// 
/// * Maximum of 100 properties total per file
/// * Maximum of 30 private properties per app
/// * Maximum of 30 public properties
/// * Maximum of 124 bytes size limit on (key + value) string in UTF-8 encoding for a single property.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get properties](PropertyGetCall) (response)
/// * [insert properties](PropertyInsertCall) (request|response)
/// * [patch properties](PropertyPatchCall) (request|response)
/// * [update properties](PropertyUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Property {
    /// ETag of the property.
    
    pub etag: Option<String>,
    /// The key of this property.
    
    pub key: Option<String>,
    /// This is always drive#property.
    
    pub kind: Option<String>,
    /// The link back to this property.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// The value of this property.
    
    pub value: Option<String>,
    /// The visibility of this property. Allowed values are PRIVATE and PUBLIC. (Default: PRIVATE). Private properties can only be retrieved using an authenticated request. An authenticated request uses an access token obtained with a OAuth 2 client ID. You cannot use an API key to retrieve private properties.
    
    pub visibility: Option<String>,
}

impl client::RequestValue for Property {}
impl client::ResponseResult for Property {}


/// A collection of properties, key-value pairs that are either public or private to an application.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list properties](PropertyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PropertyList {
    /// The ETag of the list.
    
    pub etag: Option<String>,
    /// The list of properties.
    
    pub items: Option<Vec<Property>>,
    /// This is always drive#propertyList.
    
    pub kind: Option<String>,
    /// The link back to this list.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::ResponseResult for PropertyList {}


/// A revision of a file.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete revisions](RevisionDeleteCall) (none)
/// * [get revisions](RevisionGetCall) (response)
/// * [list revisions](RevisionListCall) (none)
/// * [patch revisions](RevisionPatchCall) (request|response)
/// * [update revisions](RevisionUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Revision {
    /// no description provided
    #[serde(rename="downloadUrl")]
    
    pub download_url: Option<String>,
    /// The ETag of the revision.
    
    pub etag: Option<String>,
    /// Links for exporting Docs Editors files to specific formats.
    #[serde(rename="exportLinks")]
    
    pub export_links: Option<HashMap<String, String>>,
    /// The size of the revision in bytes. This will only be populated on files with content stored in Drive.
    #[serde(rename="fileSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub file_size: Option<i64>,
    /// The ID of the revision.
    
    pub id: Option<String>,
    /// This is always drive#revision.
    
    pub kind: Option<String>,
    /// The last user to modify this revision.
    #[serde(rename="lastModifyingUser")]
    
    pub last_modifying_user: Option<User>,
    /// Name of the last user to modify this revision.
    #[serde(rename="lastModifyingUserName")]
    
    pub last_modifying_user_name: Option<String>,
    /// An MD5 checksum for the content of this revision. This will only be populated on files with content stored in Drive.
    #[serde(rename="md5Checksum")]
    
    pub md5_checksum: Option<String>,
    /// The MIME type of the revision.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
    /// Last time this revision was modified (formatted RFC 3339 timestamp).
    #[serde(rename="modifiedDate")]
    
    pub modified_date: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The original filename when this revision was created. This will only be populated on files with content stored in Drive.
    #[serde(rename="originalFilename")]
    
    pub original_filename: Option<String>,
    /// Whether this revision is pinned to prevent automatic purging. If not set, the revision is automatically purged 30 days after newer content is uploaded. This field can only be modified on files with content stored in Drive, excluding Docs Editors files. Revisions can also be pinned when they are created through the drive.files.insert/update/copy by using the pinned query parameter. Pinned revisions are stored indefinitely using additional storage quota, up to a maximum of 200 revisions.
    
    pub pinned: Option<bool>,
    /// Whether subsequent revisions will be automatically republished. This is only populated and can only be modified for Docs Editors files.
    #[serde(rename="publishAuto")]
    
    pub publish_auto: Option<bool>,
    /// Whether this revision is published. This is only populated and can only be modified for Docs Editors files.
    
    pub published: Option<bool>,
    /// A link to the published revision. This is only populated for Google Sites files.
    #[serde(rename="publishedLink")]
    
    pub published_link: Option<String>,
    /// Whether this revision is published outside the domain. This is only populated and can only be modified for Docs Editors files.
    #[serde(rename="publishedOutsideDomain")]
    
    pub published_outside_domain: Option<bool>,
    /// A link back to this revision.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::RequestValue for Revision {}
impl client::Resource for Revision {}
impl client::ResponseResult for Revision {}


/// A list of revisions of a file.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list revisions](RevisionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RevisionList {
    /// The ETag of the list.
    
    pub etag: Option<String>,
    /// The list of revisions. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched.
    
    pub items: Option<Vec<Revision>>,
    /// This is always drive#revisionList.
    
    pub kind: Option<String>,
    /// The page token for the next page of revisions. This field will be absent if the end of the revisions list has been reached. If the token is rejected for any reason, it should be discarded and pagination should be restarted from the first page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A link back to this list.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::ResponseResult for RevisionList {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get start page token changes](ChangeGetStartPageTokenCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StartPageToken {
    /// Identifies what kind of resource this is. Value: the fixed string "drive#startPageToken".
    
    pub kind: Option<String>,
    /// The starting page token for listing changes.
    #[serde(rename="startPageToken")]
    
    pub start_page_token: Option<String>,
}

impl client::ResponseResult for StartPageToken {}


/// Deprecated: use the drive collection instead.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get teamdrives](TeamdriveGetCall) (response)
/// * [insert teamdrives](TeamdriveInsertCall) (request|response)
/// * [update teamdrives](TeamdriveUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TeamDrive {
    /// An image file and cropping parameters from which a background image for this Team Drive is set. This is a write only field; it can only be set on drive.teamdrives.update requests that don't set themeId. When specified, all fields of the backgroundImageFile must be set.
    #[serde(rename="backgroundImageFile")]
    
    pub background_image_file: Option<TeamDriveBackgroundImageFile>,
    /// A short-lived link to this Team Drive's background image.
    #[serde(rename="backgroundImageLink")]
    
    pub background_image_link: Option<String>,
    /// Capabilities the current user has on this Team Drive.
    
    pub capabilities: Option<TeamDriveCapabilities>,
    /// The color of this Team Drive as an RGB hex string. It can only be set on a drive.teamdrives.update request that does not set themeId.
    #[serde(rename="colorRgb")]
    
    pub color_rgb: Option<String>,
    /// The time at which the Team Drive was created (RFC 3339 date-time).
    #[serde(rename="createdDate")]
    
    pub created_date: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The ID of this Team Drive which is also the ID of the top level folder of this Team Drive.
    
    pub id: Option<String>,
    /// This is always drive#teamDrive
    
    pub kind: Option<String>,
    /// The name of this Team Drive.
    
    pub name: Option<String>,
    /// The organizational unit of this shared drive. This field is only populated on drives.list responses when the useDomainAdminAccess parameter is set to true.
    #[serde(rename="orgUnitId")]
    
    pub org_unit_id: Option<String>,
    /// A set of restrictions that apply to this Team Drive or items inside this Team Drive.
    
    pub restrictions: Option<TeamDriveRestrictions>,
    /// The ID of the theme from which the background image and color will be set. The set of possible teamDriveThemes can be retrieved from a drive.about.get response. When not specified on a drive.teamdrives.insert request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set colorRgb or backgroundImageFile.
    #[serde(rename="themeId")]
    
    pub theme_id: Option<String>,
}

impl client::RequestValue for TeamDrive {}
impl client::Resource for TeamDrive {}
impl client::ResponseResult for TeamDrive {}


/// A list of Team Drives.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list teamdrives](TeamdriveListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TeamDriveList {
    /// The list of Team Drives.
    
    pub items: Option<Vec<TeamDrive>>,
    /// This is always drive#teamDriveList
    
    pub kind: Option<String>,
    /// The page token for the next page of Team Drives.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for TeamDriveList {}


/// Information about a Drive user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct User {
    /// A plain text displayable name for this user.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The email address of the user.
    #[serde(rename="emailAddress")]
    
    pub email_address: Option<String>,
    /// Whether this user is the same as the authenticated user for whom the request was made.
    #[serde(rename="isAuthenticatedUser")]
    
    pub is_authenticated_user: Option<bool>,
    /// This is always drive#user.
    
    pub kind: Option<String>,
    /// The user's ID as visible in the permissions collection.
    #[serde(rename="permissionId")]
    
    pub permission_id: Option<String>,
    /// The user's profile picture.
    
    pub picture: Option<UserPicture>,
}

impl client::Part for User {}


/// Information about supported additional roles per file type. The most specific type takes precedence.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AboutAdditionalRoleInfo {
    /// The supported additional roles per primary role.
    #[serde(rename="roleSets")]
    
    pub role_sets: Option<Vec<AboutAdditionalRoleInfoRoleSets>>,
    /// The content type that this additional role info applies to.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::NestedType for AboutAdditionalRoleInfo {}
impl client::Part for AboutAdditionalRoleInfo {}


/// The supported additional roles per primary role.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AboutAdditionalRoleInfoRoleSets {
    /// The supported additional roles with the primary role.
    #[serde(rename="additionalRoles")]
    
    pub additional_roles: Option<Vec<String>>,
    /// A primary permission role.
    #[serde(rename="primaryRole")]
    
    pub primary_role: Option<String>,
}

impl client::NestedType for AboutAdditionalRoleInfoRoleSets {}
impl client::Part for AboutAdditionalRoleInfoRoleSets {}


/// A list of themes that are supported for shared drives.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AboutDriveThemes {
    /// A link to this theme's background image.
    #[serde(rename="backgroundImageLink")]
    
    pub background_image_link: Option<String>,
    /// The color of this theme as an RGB hex string.
    #[serde(rename="colorRgb")]
    
    pub color_rgb: Option<String>,
    /// The ID of the theme.
    
    pub id: Option<String>,
}

impl client::NestedType for AboutDriveThemes {}
impl client::Part for AboutDriveThemes {}


/// The allowable export formats.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AboutExportFormats {
    /// The content type to convert from.
    
    pub source: Option<String>,
    /// The possible content types to convert to.
    
    pub targets: Option<Vec<String>>,
}

impl client::NestedType for AboutExportFormats {}
impl client::Part for AboutExportFormats {}


/// List of additional features enabled on this account.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AboutFeatures {
    /// The name of the feature.
    #[serde(rename="featureName")]
    
    pub feature_name: Option<String>,
    /// The request limit rate for this feature, in queries per second.
    #[serde(rename="featureRate")]
    
    pub feature_rate: Option<f64>,
}

impl client::NestedType for AboutFeatures {}
impl client::Part for AboutFeatures {}


/// The allowable import formats.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AboutImportFormats {
    /// The imported file's content type to convert from.
    
    pub source: Option<String>,
    /// The possible content types to convert to.
    
    pub targets: Option<Vec<String>>,
}

impl client::NestedType for AboutImportFormats {}
impl client::Part for AboutImportFormats {}


/// List of max upload sizes for each file type. The most specific type takes precedence.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AboutMaxUploadSizes {
    /// The max upload size for this type.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub size: Option<i64>,
    /// The file type.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::NestedType for AboutMaxUploadSizes {}
impl client::Part for AboutMaxUploadSizes {}


/// The amount of storage quota used by different Google services.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AboutQuotaBytesByService {
    /// The storage quota bytes used by the service.
    #[serde(rename="bytesUsed")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bytes_used: Option<i64>,
    /// The service's name, e.g. DRIVE, GMAIL, or PHOTOS.
    #[serde(rename="serviceName")]
    
    pub service_name: Option<String>,
}

impl client::NestedType for AboutQuotaBytesByService {}
impl client::Part for AboutQuotaBytesByService {}


/// Deprecated - use driveThemes instead.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AboutTeamDriveThemes {
    /// Deprecated - use driveThemes/backgroundImageLink instead.
    #[serde(rename="backgroundImageLink")]
    
    pub background_image_link: Option<String>,
    /// Deprecated - use driveThemes/colorRgb instead.
    #[serde(rename="colorRgb")]
    
    pub color_rgb: Option<String>,
    /// Deprecated - use driveThemes/id instead.
    
    pub id: Option<String>,
}

impl client::NestedType for AboutTeamDriveThemes {}
impl client::Part for AboutTeamDriveThemes {}


/// The various icons for the app.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppIcons {
    /// Category of the icon. Allowed values are:  
    /// - application - icon for the application 
    /// - document - icon for a file associated with the app 
    /// - documentShared - icon for a shared file associated with the app
    
    pub category: Option<String>,
    /// URL for the icon.
    #[serde(rename="iconUrl")]
    
    pub icon_url: Option<String>,
    /// Size of the icon. Represented as the maximum of the width and height.
    
    pub size: Option<i32>,
}

impl client::NestedType for AppIcons {}
impl client::Part for AppIcons {}


/// The context of the file which is being commented on.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommentContext {
    /// The MIME type of the context snippet.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Data representation of the segment of the file being commented on. In the case of a text file for example, this would be the actual text that the comment is about.
    
    pub value: Option<String>,
}

impl client::NestedType for CommentContext {}
impl client::Part for CommentContext {}


/// An image file and cropping parameters from which a background image for this shared drive is set. This is a write only field; it can only be set on drive.drives.update requests that don't set themeId. When specified, all fields of the backgroundImageFile must be set.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DriveBackgroundImageFile {
    /// The ID of an image file in Google Drive to use for the background image.
    
    pub id: Option<String>,
    /// The width of the cropped image in the closed range of 0 to 1. This value represents the width of the cropped image divided by the width of the entire image. The height is computed by applying a width to height aspect ratio of 80 to 9. The resulting image must be at least 1280 pixels wide and 144 pixels high.
    
    pub width: Option<f32>,
    /// The X coordinate of the upper left corner of the cropping area in the background image. This is a value in the closed range of 0 to 1. This value represents the horizontal distance from the left side of the entire image to the left side of the cropping area divided by the width of the entire image.
    #[serde(rename="xCoordinate")]
    
    pub x_coordinate: Option<f32>,
    /// The Y coordinate of the upper left corner of the cropping area in the background image. This is a value in the closed range of 0 to 1. This value represents the vertical distance from the top side of the entire image to the top side of the cropping area divided by the height of the entire image.
    #[serde(rename="yCoordinate")]
    
    pub y_coordinate: Option<f32>,
}

impl client::NestedType for DriveBackgroundImageFile {}
impl client::Part for DriveBackgroundImageFile {}


/// Capabilities the current user has on this shared drive.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DriveCapabilities {
    /// Whether the current user can add children to folders in this shared drive.
    #[serde(rename="canAddChildren")]
    
    pub can_add_children: Option<bool>,
    /// Whether the current user can change the copyRequiresWriterPermission restriction of this shared drive.
    #[serde(rename="canChangeCopyRequiresWriterPermissionRestriction")]
    
    pub can_change_copy_requires_writer_permission_restriction: Option<bool>,
    /// Whether the current user can change the domainUsersOnly restriction of this shared drive.
    #[serde(rename="canChangeDomainUsersOnlyRestriction")]
    
    pub can_change_domain_users_only_restriction: Option<bool>,
    /// Whether the current user can change the background of this shared drive.
    #[serde(rename="canChangeDriveBackground")]
    
    pub can_change_drive_background: Option<bool>,
    /// Whether the current user can change the driveMembersOnly restriction of this shared drive.
    #[serde(rename="canChangeDriveMembersOnlyRestriction")]
    
    pub can_change_drive_members_only_restriction: Option<bool>,
    /// Whether the current user can comment on files in this shared drive.
    #[serde(rename="canComment")]
    
    pub can_comment: Option<bool>,
    /// Whether the current user can copy files in this shared drive.
    #[serde(rename="canCopy")]
    
    pub can_copy: Option<bool>,
    /// Whether the current user can delete children from folders in this shared drive.
    #[serde(rename="canDeleteChildren")]
    
    pub can_delete_children: Option<bool>,
    /// Whether the current user can delete this shared drive. Attempting to delete the shared drive may still fail if there are untrashed items inside the shared drive.
    #[serde(rename="canDeleteDrive")]
    
    pub can_delete_drive: Option<bool>,
    /// Whether the current user can download files in this shared drive.
    #[serde(rename="canDownload")]
    
    pub can_download: Option<bool>,
    /// Whether the current user can edit files in this shared drive
    #[serde(rename="canEdit")]
    
    pub can_edit: Option<bool>,
    /// Whether the current user can list the children of folders in this shared drive.
    #[serde(rename="canListChildren")]
    
    pub can_list_children: Option<bool>,
    /// Whether the current user can add members to this shared drive or remove them or change their role.
    #[serde(rename="canManageMembers")]
    
    pub can_manage_members: Option<bool>,
    /// Whether the current user can read the revisions resource of files in this shared drive.
    #[serde(rename="canReadRevisions")]
    
    pub can_read_revisions: Option<bool>,
    /// Whether the current user can rename files or folders in this shared drive.
    #[serde(rename="canRename")]
    
    pub can_rename: Option<bool>,
    /// Whether the current user can rename this shared drive.
    #[serde(rename="canRenameDrive")]
    
    pub can_rename_drive: Option<bool>,
    /// Whether the current user can reset the shared drive restrictions to defaults.
    #[serde(rename="canResetDriveRestrictions")]
    
    pub can_reset_drive_restrictions: Option<bool>,
    /// Whether the current user can share files or folders in this shared drive.
    #[serde(rename="canShare")]
    
    pub can_share: Option<bool>,
    /// Whether the current user can trash children from folders in this shared drive.
    #[serde(rename="canTrashChildren")]
    
    pub can_trash_children: Option<bool>,
}

impl client::NestedType for DriveCapabilities {}
impl client::Part for DriveCapabilities {}


/// A set of restrictions that apply to this shared drive or items inside this shared drive.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DriveRestrictions {
    /// Whether administrative privileges on this shared drive are required to modify restrictions.
    #[serde(rename="adminManagedRestrictions")]
    
    pub admin_managed_restrictions: Option<bool>,
    /// Whether the options to copy, print, or download files inside this shared drive, should be disabled for readers and commenters. When this restriction is set to true, it will override the similarly named field to true for any file inside this shared drive.
    #[serde(rename="copyRequiresWriterPermission")]
    
    pub copy_requires_writer_permission: Option<bool>,
    /// Whether access to this shared drive and items inside this shared drive is restricted to users of the domain to which this shared drive belongs. This restriction may be overridden by other sharing policies controlled outside of this shared drive.
    #[serde(rename="domainUsersOnly")]
    
    pub domain_users_only: Option<bool>,
    /// Whether access to items inside this shared drive is restricted to its members.
    #[serde(rename="driveMembersOnly")]
    
    pub drive_members_only: Option<bool>,
}

impl client::NestedType for DriveRestrictions {}
impl client::Part for DriveRestrictions {}


/// Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileCapabilities {
    /// Whether the current user is the pending owner of the file. Not populated for shared drive files.
    #[serde(rename="canAcceptOwnership")]
    
    pub can_accept_ownership: Option<bool>,
    /// Whether the current user can add children to this folder. This is always false when the item is not a folder.
    #[serde(rename="canAddChildren")]
    
    pub can_add_children: Option<bool>,
    /// Whether the current user can add a folder from another drive (different shared drive or My Drive) to this folder. This is false when the item is not a folder. Only populated for items in shared drives.
    #[serde(rename="canAddFolderFromAnotherDrive")]
    
    pub can_add_folder_from_another_drive: Option<bool>,
    /// Whether the current user can add a parent for the item without removing an existing parent in the same request. Not populated for shared drive files.
    #[serde(rename="canAddMyDriveParent")]
    
    pub can_add_my_drive_parent: Option<bool>,
    /// Whether the current user can change the copyRequiresWriterPermission restriction of this file.
    #[serde(rename="canChangeCopyRequiresWriterPermission")]
    
    pub can_change_copy_requires_writer_permission: Option<bool>,
    /// Deprecated
    #[serde(rename="canChangeRestrictedDownload")]
    
    pub can_change_restricted_download: Option<bool>,
    /// Whether the current user can change the securityUpdateEnabled field on link share metadata.
    #[serde(rename="canChangeSecurityUpdateEnabled")]
    
    pub can_change_security_update_enabled: Option<bool>,
    /// Whether the current user can comment on this file.
    #[serde(rename="canComment")]
    
    pub can_comment: Option<bool>,
    /// Whether the current user can copy this file. For an item in a shared drive, whether the current user can copy non-folder descendants of this item, or this item itself if it is not a folder.
    #[serde(rename="canCopy")]
    
    pub can_copy: Option<bool>,
    /// Whether the current user can delete this file.
    #[serde(rename="canDelete")]
    
    pub can_delete: Option<bool>,
    /// Whether the current user can delete children of this folder. This is false when the item is not a folder. Only populated for items in shared drives.
    #[serde(rename="canDeleteChildren")]
    
    pub can_delete_children: Option<bool>,
    /// Whether the current user can download this file.
    #[serde(rename="canDownload")]
    
    pub can_download: Option<bool>,
    /// Whether the current user can edit this file. Other factors may limit the type of changes a user can make to a file. For example, see canChangeCopyRequiresWriterPermission or canModifyContent.
    #[serde(rename="canEdit")]
    
    pub can_edit: Option<bool>,
    /// Whether the current user can list the children of this folder. This is always false when the item is not a folder.
    #[serde(rename="canListChildren")]
    
    pub can_list_children: Option<bool>,
    /// Whether the current user can modify the content of this file.
    #[serde(rename="canModifyContent")]
    
    pub can_modify_content: Option<bool>,
    /// Whether the current user can modify restrictions on content of this file.
    #[serde(rename="canModifyContentRestriction")]
    
    pub can_modify_content_restriction: Option<bool>,
    /// Whether the current user can modify the labels on this file.
    #[serde(rename="canModifyLabels")]
    
    pub can_modify_labels: Option<bool>,
    /// Whether the current user can move children of this folder outside of the shared drive. This is false when the item is not a folder. Only populated for items in shared drives.
    #[serde(rename="canMoveChildrenOutOfDrive")]
    
    pub can_move_children_out_of_drive: Option<bool>,
    /// Deprecated - use canMoveChildrenOutOfDrive instead.
    #[serde(rename="canMoveChildrenOutOfTeamDrive")]
    
    pub can_move_children_out_of_team_drive: Option<bool>,
    /// Whether the current user can move children of this folder within this drive. This is false when the item is not a folder. Note that a request to move the child may still fail depending on the current user's access to the child and to the destination folder.
    #[serde(rename="canMoveChildrenWithinDrive")]
    
    pub can_move_children_within_drive: Option<bool>,
    /// Deprecated - use canMoveChildrenWithinDrive instead.
    #[serde(rename="canMoveChildrenWithinTeamDrive")]
    
    pub can_move_children_within_team_drive: Option<bool>,
    /// Deprecated - use canMoveItemOutOfDrive instead.
    #[serde(rename="canMoveItemIntoTeamDrive")]
    
    pub can_move_item_into_team_drive: Option<bool>,
    /// Whether the current user can move this item outside of this drive by changing its parent. Note that a request to change the parent of the item may still fail depending on the new parent that is being added.
    #[serde(rename="canMoveItemOutOfDrive")]
    
    pub can_move_item_out_of_drive: Option<bool>,
    /// Deprecated - use canMoveItemOutOfDrive instead.
    #[serde(rename="canMoveItemOutOfTeamDrive")]
    
    pub can_move_item_out_of_team_drive: Option<bool>,
    /// Whether the current user can move this item within this drive. Note that a request to change the parent of the item may still fail depending on the new parent that is being added and the parent that is being removed.
    #[serde(rename="canMoveItemWithinDrive")]
    
    pub can_move_item_within_drive: Option<bool>,
    /// Deprecated - use canMoveItemWithinDrive instead.
    #[serde(rename="canMoveItemWithinTeamDrive")]
    
    pub can_move_item_within_team_drive: Option<bool>,
    /// Deprecated - use canMoveItemWithinDrive or canMoveItemOutOfDrive instead.
    #[serde(rename="canMoveTeamDriveItem")]
    
    pub can_move_team_drive_item: Option<bool>,
    /// Whether the current user can read the shared drive to which this file belongs. Only populated for items in shared drives.
    #[serde(rename="canReadDrive")]
    
    pub can_read_drive: Option<bool>,
    /// Whether the current user can read the labels on this file.
    #[serde(rename="canReadLabels")]
    
    pub can_read_labels: Option<bool>,
    /// Whether the current user can read the revisions resource of this file. For a shared drive item, whether revisions of non-folder descendants of this item, or this item itself if it isn't a folder, can be read.
    #[serde(rename="canReadRevisions")]
    
    pub can_read_revisions: Option<bool>,
    /// Deprecated - use canReadDrive instead.
    #[serde(rename="canReadTeamDrive")]
    
    pub can_read_team_drive: Option<bool>,
    /// Whether the current user can remove children from this folder. This is always false when the item is not a folder. For a folder in a shared drive, use canDeleteChildren or canTrashChildren instead.
    #[serde(rename="canRemoveChildren")]
    
    pub can_remove_children: Option<bool>,
    /// Whether the current user can remove a parent from the item without adding another parent in the same request. Not populated for shared drive files.
    #[serde(rename="canRemoveMyDriveParent")]
    
    pub can_remove_my_drive_parent: Option<bool>,
    /// Whether the current user can rename this file.
    #[serde(rename="canRename")]
    
    pub can_rename: Option<bool>,
    /// Whether the current user can modify the sharing settings for this file.
    #[serde(rename="canShare")]
    
    pub can_share: Option<bool>,
    /// Whether the current user can move this file to trash.
    #[serde(rename="canTrash")]
    
    pub can_trash: Option<bool>,
    /// Whether the current user can trash children of this folder. This is false when the item is not a folder. Only populated for items in shared drives.
    #[serde(rename="canTrashChildren")]
    
    pub can_trash_children: Option<bool>,
    /// Whether the current user can restore this file from trash.
    #[serde(rename="canUntrash")]
    
    pub can_untrash: Option<bool>,
}

impl client::NestedType for FileCapabilities {}
impl client::Part for FileCapabilities {}


/// Metadata about image media. This will only be present for image types, and its contents will depend on what can be parsed from the image content.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileImageMediaMetadata {
    /// The aperture used to create the photo (f-number).
    
    pub aperture: Option<f32>,
    /// The make of the camera used to create the photo.
    #[serde(rename="cameraMake")]
    
    pub camera_make: Option<String>,
    /// The model of the camera used to create the photo.
    #[serde(rename="cameraModel")]
    
    pub camera_model: Option<String>,
    /// The color space of the photo.
    #[serde(rename="colorSpace")]
    
    pub color_space: Option<String>,
    /// The date and time the photo was taken (EXIF format timestamp).
    
    pub date: Option<String>,
    /// The exposure bias of the photo (APEX value).
    #[serde(rename="exposureBias")]
    
    pub exposure_bias: Option<f32>,
    /// The exposure mode used to create the photo.
    #[serde(rename="exposureMode")]
    
    pub exposure_mode: Option<String>,
    /// The length of the exposure, in seconds.
    #[serde(rename="exposureTime")]
    
    pub exposure_time: Option<f32>,
    /// Whether a flash was used to create the photo.
    #[serde(rename="flashUsed")]
    
    pub flash_used: Option<bool>,
    /// The focal length used to create the photo, in millimeters.
    #[serde(rename="focalLength")]
    
    pub focal_length: Option<f32>,
    /// The height of the image in pixels.
    
    pub height: Option<i32>,
    /// The ISO speed used to create the photo.
    #[serde(rename="isoSpeed")]
    
    pub iso_speed: Option<i32>,
    /// The lens used to create the photo.
    
    pub lens: Option<String>,
    /// Geographic location information stored in the image.
    
    pub location: Option<FileImageMediaMetadataLocation>,
    /// The smallest f-number of the lens at the focal length used to create the photo (APEX value).
    #[serde(rename="maxApertureValue")]
    
    pub max_aperture_value: Option<f32>,
    /// The metering mode used to create the photo.
    #[serde(rename="meteringMode")]
    
    pub metering_mode: Option<String>,
    /// The number of clockwise 90 degree rotations applied from the image's original orientation.
    
    pub rotation: Option<i32>,
    /// The type of sensor used to create the photo.
    
    pub sensor: Option<String>,
    /// The distance to the subject of the photo, in meters.
    #[serde(rename="subjectDistance")]
    
    pub subject_distance: Option<i32>,
    /// The white balance mode used to create the photo.
    #[serde(rename="whiteBalance")]
    
    pub white_balance: Option<String>,
    /// The width of the image in pixels.
    
    pub width: Option<i32>,
}

impl client::NestedType for FileImageMediaMetadata {}
impl client::Part for FileImageMediaMetadata {}


/// Geographic location information stored in the image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileImageMediaMetadataLocation {
    /// The altitude stored in the image.
    
    pub altitude: Option<f64>,
    /// The latitude stored in the image.
    
    pub latitude: Option<f64>,
    /// The longitude stored in the image.
    
    pub longitude: Option<f64>,
}

impl client::NestedType for FileImageMediaMetadataLocation {}
impl client::Part for FileImageMediaMetadataLocation {}


/// Indexable text attributes for the file (can only be written). For more information, see Manage file metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileIndexableText {
    /// The text to be indexed for this file.
    
    pub text: Option<String>,
}

impl client::NestedType for FileIndexableText {}
impl client::Part for FileIndexableText {}


/// An overview of the labels on the file.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileLabelInfo {
    /// The set of labels on the file as requested by the label IDs in the includeLabels parameter. By default, no labels are returned.
    
    pub labels: Option<Vec<Label>>,
}

impl client::NestedType for FileLabelInfo {}
impl client::Part for FileLabelInfo {}


/// A group of labels for the file.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileLabels {
    /// Deprecated.
    
    pub hidden: Option<bool>,
    /// Whether the file has been modified by this user.
    
    pub modified: Option<bool>,
    /// Deprecated - use copyRequiresWriterPermission instead.
    
    pub restricted: Option<bool>,
    /// Whether this file is starred by the user.
    
    pub starred: Option<bool>,
    /// Whether the file has been trashed, either explicitly or from a trashed parent folder. Only the owner may trash a file. The trashed item is excluded from all files.list responses returned for any user who does not own the file. However, all users with access to the file can see the trashed item metadata in an API response. All users with access can copy, download, export, and share the file.
    
    pub trashed: Option<bool>,
    /// Whether this file has been viewed by this user.
    
    pub viewed: Option<bool>,
}

impl client::NestedType for FileLabels {}
impl client::Part for FileLabels {}


/// Contains details about the link URLs that clients are using to refer to this item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileLinkShareMetadata {
    /// Whether the file is eligible for security update.
    #[serde(rename="securityUpdateEligible")]
    
    pub security_update_eligible: Option<bool>,
    /// Whether the security update is enabled for this file.
    #[serde(rename="securityUpdateEnabled")]
    
    pub security_update_enabled: Option<bool>,
}

impl client::NestedType for FileLinkShareMetadata {}
impl client::Part for FileLinkShareMetadata {}


/// Shortcut file details. Only populated for shortcut files, which have the mimeType field set to application/vnd.google-apps.shortcut.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileShortcutDetails {
    /// The ID of the file that this shortcut points to.
    #[serde(rename="targetId")]
    
    pub target_id: Option<String>,
    /// The MIME type of the file that this shortcut points to. The value of this field is a snapshot of the target's MIME type, captured when the shortcut is created.
    #[serde(rename="targetMimeType")]
    
    pub target_mime_type: Option<String>,
    /// The ResourceKey for the target file.
    #[serde(rename="targetResourceKey")]
    
    pub target_resource_key: Option<String>,
}

impl client::NestedType for FileShortcutDetails {}
impl client::Part for FileShortcutDetails {}


/// A thumbnail for the file. This will only be used if a standard thumbnail cannot be generated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileThumbnail {
    /// The URL-safe Base64 encoded bytes of the thumbnail image. It should conform to RFC 4648 section 5.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub image: Option<Vec<u8>>,
    /// The MIME type of the thumbnail.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
}

impl client::NestedType for FileThumbnail {}
impl client::Part for FileThumbnail {}


/// Metadata about video media. This will only be present for video types.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileVideoMediaMetadata {
    /// The duration of the video in milliseconds.
    #[serde(rename="durationMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub duration_millis: Option<i64>,
    /// The height of the video in pixels.
    
    pub height: Option<i32>,
    /// The width of the video in pixels.
    
    pub width: Option<i32>,
}

impl client::NestedType for FileVideoMediaMetadata {}
impl client::Part for FileVideoMediaMetadata {}


/// Details of whether the permissions on this shared drive item are inherited or directly on this item. This is an output-only field which is present only for shared drive items.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PermissionPermissionDetails {
    /// Additional roles for this user. Only commenter is currently possible, though more may be supported in the future.
    #[serde(rename="additionalRoles")]
    
    pub additional_roles: Option<Vec<String>>,
    /// Whether this permission is inherited. This field is always populated. This is an output-only field.
    
    pub inherited: Option<bool>,
    /// The ID of the item from which this permission is inherited. This is an output-only field.
    #[serde(rename="inheritedFrom")]
    
    pub inherited_from: Option<String>,
    /// The permission type for this user. While new values may be added in future, the following are currently possible:  
    /// - file 
    /// - member
    #[serde(rename="permissionType")]
    
    pub permission_type: Option<String>,
    /// The primary role for this user. While new values may be added in the future, the following are currently possible:  
    /// - organizer 
    /// - fileOrganizer 
    /// - writer 
    /// - reader
    
    pub role: Option<String>,
}

impl client::NestedType for PermissionPermissionDetails {}
impl client::Part for PermissionPermissionDetails {}


/// Deprecated - use permissionDetails instead.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PermissionTeamDrivePermissionDetails {
    /// Deprecated - use permissionDetails/additionalRoles instead.
    #[serde(rename="additionalRoles")]
    
    pub additional_roles: Option<Vec<String>>,
    /// Deprecated - use permissionDetails/inherited instead.
    
    pub inherited: Option<bool>,
    /// Deprecated - use permissionDetails/inheritedFrom instead.
    #[serde(rename="inheritedFrom")]
    
    pub inherited_from: Option<String>,
    /// Deprecated - use permissionDetails/role instead.
    
    pub role: Option<String>,
    /// Deprecated - use permissionDetails/permissionType instead.
    #[serde(rename="teamDrivePermissionType")]
    
    pub team_drive_permission_type: Option<String>,
}

impl client::NestedType for PermissionTeamDrivePermissionDetails {}
impl client::Part for PermissionTeamDrivePermissionDetails {}


/// An image file and cropping parameters from which a background image for this Team Drive is set. This is a write only field; it can only be set on drive.teamdrives.update requests that don't set themeId. When specified, all fields of the backgroundImageFile must be set.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TeamDriveBackgroundImageFile {
    /// The ID of an image file in Drive to use for the background image.
    
    pub id: Option<String>,
    /// The width of the cropped image in the closed range of 0 to 1. This value represents the width of the cropped image divided by the width of the entire image. The height is computed by applying a width to height aspect ratio of 80 to 9. The resulting image must be at least 1280 pixels wide and 144 pixels high.
    
    pub width: Option<f32>,
    /// The X coordinate of the upper left corner of the cropping area in the background image. This is a value in the closed range of 0 to 1. This value represents the horizontal distance from the left side of the entire image to the left side of the cropping area divided by the width of the entire image.
    #[serde(rename="xCoordinate")]
    
    pub x_coordinate: Option<f32>,
    /// The Y coordinate of the upper left corner of the cropping area in the background image. This is a value in the closed range of 0 to 1. This value represents the vertical distance from the top side of the entire image to the top side of the cropping area divided by the height of the entire image.
    #[serde(rename="yCoordinate")]
    
    pub y_coordinate: Option<f32>,
}

impl client::NestedType for TeamDriveBackgroundImageFile {}
impl client::Part for TeamDriveBackgroundImageFile {}


/// Capabilities the current user has on this Team Drive.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TeamDriveCapabilities {
    /// Whether the current user can add children to folders in this Team Drive.
    #[serde(rename="canAddChildren")]
    
    pub can_add_children: Option<bool>,
    /// Whether the current user can change the copyRequiresWriterPermission restriction of this Team Drive.
    #[serde(rename="canChangeCopyRequiresWriterPermissionRestriction")]
    
    pub can_change_copy_requires_writer_permission_restriction: Option<bool>,
    /// Whether the current user can change the domainUsersOnly restriction of this Team Drive.
    #[serde(rename="canChangeDomainUsersOnlyRestriction")]
    
    pub can_change_domain_users_only_restriction: Option<bool>,
    /// Whether the current user can change the background of this Team Drive.
    #[serde(rename="canChangeTeamDriveBackground")]
    
    pub can_change_team_drive_background: Option<bool>,
    /// Whether the current user can change the teamMembersOnly restriction of this Team Drive.
    #[serde(rename="canChangeTeamMembersOnlyRestriction")]
    
    pub can_change_team_members_only_restriction: Option<bool>,
    /// Whether the current user can comment on files in this Team Drive.
    #[serde(rename="canComment")]
    
    pub can_comment: Option<bool>,
    /// Whether the current user can copy files in this Team Drive.
    #[serde(rename="canCopy")]
    
    pub can_copy: Option<bool>,
    /// Whether the current user can delete children from folders in this Team Drive.
    #[serde(rename="canDeleteChildren")]
    
    pub can_delete_children: Option<bool>,
    /// Whether the current user can delete this Team Drive. Attempting to delete the Team Drive may still fail if there are untrashed items inside the Team Drive.
    #[serde(rename="canDeleteTeamDrive")]
    
    pub can_delete_team_drive: Option<bool>,
    /// Whether the current user can download files in this Team Drive.
    #[serde(rename="canDownload")]
    
    pub can_download: Option<bool>,
    /// Whether the current user can edit files in this Team Drive
    #[serde(rename="canEdit")]
    
    pub can_edit: Option<bool>,
    /// Whether the current user can list the children of folders in this Team Drive.
    #[serde(rename="canListChildren")]
    
    pub can_list_children: Option<bool>,
    /// Whether the current user can add members to this Team Drive or remove them or change their role.
    #[serde(rename="canManageMembers")]
    
    pub can_manage_members: Option<bool>,
    /// Whether the current user can read the revisions resource of files in this Team Drive.
    #[serde(rename="canReadRevisions")]
    
    pub can_read_revisions: Option<bool>,
    /// Deprecated - use canDeleteChildren or canTrashChildren instead.
    #[serde(rename="canRemoveChildren")]
    
    pub can_remove_children: Option<bool>,
    /// Whether the current user can rename files or folders in this Team Drive.
    #[serde(rename="canRename")]
    
    pub can_rename: Option<bool>,
    /// Whether the current user can rename this Team Drive.
    #[serde(rename="canRenameTeamDrive")]
    
    pub can_rename_team_drive: Option<bool>,
    /// Whether the current user can reset the Team Drive restrictions to defaults.
    #[serde(rename="canResetTeamDriveRestrictions")]
    
    pub can_reset_team_drive_restrictions: Option<bool>,
    /// Whether the current user can share files or folders in this Team Drive.
    #[serde(rename="canShare")]
    
    pub can_share: Option<bool>,
    /// Whether the current user can trash children from folders in this Team Drive.
    #[serde(rename="canTrashChildren")]
    
    pub can_trash_children: Option<bool>,
}

impl client::NestedType for TeamDriveCapabilities {}
impl client::Part for TeamDriveCapabilities {}


/// A set of restrictions that apply to this Team Drive or items inside this Team Drive.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TeamDriveRestrictions {
    /// Whether administrative privileges on this Team Drive are required to modify restrictions.
    #[serde(rename="adminManagedRestrictions")]
    
    pub admin_managed_restrictions: Option<bool>,
    /// Whether the options to copy, print, or download files inside this Team Drive, should be disabled for readers and commenters. When this restriction is set to true, it will override the similarly named field to true for any file inside this Team Drive.
    #[serde(rename="copyRequiresWriterPermission")]
    
    pub copy_requires_writer_permission: Option<bool>,
    /// Whether access to this Team Drive and items inside this Team Drive is restricted to users of the domain to which this Team Drive belongs. This restriction may be overridden by other sharing policies controlled outside of this Team Drive.
    #[serde(rename="domainUsersOnly")]
    
    pub domain_users_only: Option<bool>,
    /// Whether access to items inside this Team Drive is restricted to members of this Team Drive.
    #[serde(rename="teamMembersOnly")]
    
    pub team_members_only: Option<bool>,
}

impl client::NestedType for TeamDriveRestrictions {}
impl client::Part for TeamDriveRestrictions {}


/// The user's profile picture.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserPicture {
    /// A URL that points to a profile picture of this user.
    
    pub url: Option<String>,
}

impl client::NestedType for UserPicture {}
impl client::Part for UserPicture {}


