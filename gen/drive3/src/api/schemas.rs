use super::*;
/// Information about the user, the user’s Drive, and system capabilities.
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
    /// Whether the user has installed the requesting app.
    #[serde(rename="appInstalled")]
    
    pub app_installed: Option<bool>,
    /// Whether the user can create shared drives.
    #[serde(rename="canCreateDrives")]
    
    pub can_create_drives: Option<bool>,
    /// Deprecated - use canCreateDrives instead.
    #[serde(rename="canCreateTeamDrives")]
    
    pub can_create_team_drives: Option<bool>,
    /// A list of themes that are supported for shared drives.
    #[serde(rename="driveThemes")]
    
    pub drive_themes: Option<Vec<AboutDriveThemes>>,
    /// A map of source MIME type to possible targets for all supported exports.
    #[serde(rename="exportFormats")]
    
    pub export_formats: Option<HashMap<String, Vec<String>>>,
    /// The currently supported folder colors as RGB hex strings.
    #[serde(rename="folderColorPalette")]
    
    pub folder_color_palette: Option<Vec<String>>,
    /// A map of source MIME type to possible targets for all supported imports.
    #[serde(rename="importFormats")]
    
    pub import_formats: Option<HashMap<String, Vec<String>>>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#about".
    
    pub kind: Option<String>,
    /// A map of maximum import sizes by MIME type, in bytes.
    #[serde(rename="maxImportSizes")]
    
    #[serde_as(as = "Option<HashMap<_, ::client::serde_with::DisplayFromStr>>")]
    pub max_import_sizes: Option<HashMap<String, i64>>,
    /// The maximum upload size in bytes.
    #[serde(rename="maxUploadSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_upload_size: Option<i64>,
    /// The user's storage quota limits and usage. All fields are measured in bytes.
    #[serde(rename="storageQuota")]
    
    pub storage_quota: Option<AboutStorageQuota>,
    /// Deprecated - use driveThemes instead.
    #[serde(rename="teamDriveThemes")]
    
    pub team_drive_themes: Option<Vec<AboutTeamDriveThemes>>,
    /// The authenticated user.
    
    pub user: Option<User>,
}

impl client::ResponseResult for About {}


/// A change to a file or shared drive.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get start page token changes](ChangeGetStartPageTokenCall) (none)
/// * [list changes](ChangeListCall) (none)
/// * [watch changes](ChangeWatchCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Change {
    /// The type of the change. Possible values are file and drive.
    #[serde(rename="changeType")]
    
    pub change_type: Option<String>,
    /// The updated state of the shared drive. Present if the changeType is drive, the user is still a member of the shared drive, and the shared drive has not been deleted.
    
    pub drive: Option<Drive>,
    /// The ID of the shared drive associated with this change.
    #[serde(rename="driveId")]
    
    pub drive_id: Option<String>,
    /// The updated state of the file. Present if the type is file and the file has not been removed from this list of changes.
    
    pub file: Option<File>,
    /// The ID of the file which has changed.
    #[serde(rename="fileId")]
    
    pub file_id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#change".
    
    pub kind: Option<String>,
    /// Whether the file or shared drive has been removed from this list of changes, for example by deletion or loss of access.
    
    pub removed: Option<bool>,
    /// Deprecated - use drive instead.
    #[serde(rename="teamDrive")]
    
    pub team_drive: Option<TeamDrive>,
    /// Deprecated - use driveId instead.
    #[serde(rename="teamDriveId")]
    
    pub team_drive_id: Option<String>,
    /// The time of this change (RFC 3339 date-time).
    
    pub time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Deprecated - use changeType instead.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Resource for Change {}


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
    /// The list of changes. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched.
    
    pub changes: Option<Vec<Change>>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#changeList".
    
    pub kind: Option<String>,
    /// The starting page token for future changes. This will be present only if the end of the current changes list has been reached.
    #[serde(rename="newStartPageToken")]
    
    pub new_start_page_token: Option<String>,
    /// The page token for the next page of changes. This will be absent if the end of the changes list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
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


/// A comment on a file.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create comments](CommentCreateCall) (request|response)
/// * [delete comments](CommentDeleteCall) (none)
/// * [get comments](CommentGetCall) (response)
/// * [list comments](CommentListCall) (none)
/// * [update comments](CommentUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Comment {
    /// A region of the document represented as a JSON string. For details on defining anchor properties, refer to  Add comments and replies.
    
    pub anchor: Option<String>,
    /// The author of the comment. The author's email address and permission ID will not be populated.
    
    pub author: Option<User>,
    /// The plain text content of the comment. This field is used for setting the content, while htmlContent should be displayed.
    
    pub content: Option<String>,
    /// The time at which the comment was created (RFC 3339 date-time).
    #[serde(rename="createdTime")]
    
    pub created_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Whether the comment has been deleted. A deleted comment has no content.
    
    pub deleted: Option<bool>,
    /// The content of the comment with HTML formatting.
    #[serde(rename="htmlContent")]
    
    pub html_content: Option<String>,
    /// The ID of the comment.
    
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#comment".
    
    pub kind: Option<String>,
    /// The last time the comment or any of its replies was modified (RFC 3339 date-time).
    #[serde(rename="modifiedTime")]
    
    pub modified_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The file content to which the comment refers, typically within the anchor region. For a text file, for example, this would be the text at the location of the comment.
    #[serde(rename="quotedFileContent")]
    
    pub quoted_file_content: Option<CommentQuotedFileContent>,
    /// The full list of replies to the comment in chronological order.
    
    pub replies: Option<Vec<Reply>>,
    /// Whether the comment has been resolved by one of its replies.
    
    pub resolved: Option<bool>,
}

impl client::RequestValue for Comment {}
impl client::Resource for Comment {}
impl client::ResponseResult for Comment {}


/// A list of comments on a file.
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
    
    pub comments: Option<Vec<Comment>>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#commentList".
    
    pub kind: Option<String>,
    /// The page token for the next page of comments. This will be absent if the end of the comments list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for CommentList {}


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
    #[serde(rename="restrictionTime")]
    
    pub restriction_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
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
/// * [create drives](DriveCreateCall) (request|response)
/// * [delete drives](DriveDeleteCall) (none)
/// * [get drives](DriveGetCall) (response)
/// * [hide drives](DriveHideCall) (response)
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
    #[serde(rename="createdTime")]
    
    pub created_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Whether the shared drive is hidden from default view.
    
    pub hidden: Option<bool>,
    /// The ID of this shared drive which is also the ID of the top level folder of this shared drive.
    
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#drive".
    
    pub kind: Option<String>,
    /// The name of this shared drive.
    
    pub name: Option<String>,
    /// The organizational unit of this shared drive. This field is only populated on drives.list responses when the useDomainAdminAccess parameter is set to true.
    #[serde(rename="orgUnitId")]
    
    pub org_unit_id: Option<String>,
    /// A set of restrictions that apply to this shared drive or items inside this shared drive.
    
    pub restrictions: Option<DriveRestrictions>,
    /// The ID of the theme from which the background image and color will be set. The set of possible driveThemes can be retrieved from a drive.about.get response. When not specified on a drive.drives.create request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set colorRgb or backgroundImageFile.
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
    
    pub drives: Option<Vec<Drive>>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#driveList".
    
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
/// * [create files](FileCreateCall) (request|response)
/// * [delete files](FileDeleteCall) (none)
/// * [empty trash files](FileEmptyTrashCall) (none)
/// * [export files](FileExportCall) (none)
/// * [generate ids files](FileGenerateIdCall) (none)
/// * [get files](FileGetCall) (response)
/// * [list files](FileListCall) (none)
/// * [list labels files](FileListLabelCall) (none)
/// * [modify labels files](FileModifyLabelCall) (none)
/// * [update files](FileUpdateCall) (request|response)
/// * [watch files](FileWatchCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct File {
    /// A collection of arbitrary key-value pairs which are private to the requesting app.
    /// Entries with null values are cleared in update and copy requests. These properties can only be retrieved using an authenticated request. An authenticated request uses an access token obtained with a OAuth 2 client ID. You cannot use an API key to retrieve private properties.
    #[serde(rename="appProperties")]
    
    pub app_properties: Option<HashMap<String, String>>,
    /// Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
    
    pub capabilities: Option<FileCapabilities>,
    /// Additional information about the content of the file. These fields are never populated in responses.
    #[serde(rename="contentHints")]
    
    pub content_hints: Option<FileContentHints>,
    /// Restrictions for accessing the content of the file. Only populated if such a restriction exists.
    #[serde(rename="contentRestrictions")]
    
    pub content_restrictions: Option<Vec<ContentRestriction>>,
    /// Whether the options to copy, print, or download this file, should be disabled for readers and commenters.
    #[serde(rename="copyRequiresWriterPermission")]
    
    pub copy_requires_writer_permission: Option<bool>,
    /// The time at which the file was created (RFC 3339 date-time).
    #[serde(rename="createdTime")]
    
    pub created_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A short description of the file.
    
    pub description: Option<String>,
    /// ID of the shared drive the file resides in. Only populated for items in shared drives.
    #[serde(rename="driveId")]
    
    pub drive_id: Option<String>,
    /// Whether the file has been explicitly trashed, as opposed to recursively trashed from a parent folder.
    #[serde(rename="explicitlyTrashed")]
    
    pub explicitly_trashed: Option<bool>,
    /// Links for exporting Docs Editors files to specific formats.
    #[serde(rename="exportLinks")]
    
    pub export_links: Option<HashMap<String, String>>,
    /// The final component of fullFileExtension. This is only available for files with binary content in Google Drive.
    #[serde(rename="fileExtension")]
    
    pub file_extension: Option<String>,
    /// The color for a folder or shortcut to a folder as an RGB hex string. The supported colors are published in the folderColorPalette field of the About resource.
    /// If an unsupported color is specified, the closest color in the palette will be used instead.
    #[serde(rename="folderColorRgb")]
    
    pub folder_color_rgb: Option<String>,
    /// The full file extension extracted from the name field. May contain multiple concatenated extensions, such as "tar.gz". This is only available for files with binary content in Google Drive.
    /// This is automatically updated when the name field changes, however it isn't cleared if the new name does not contain a valid extension.
    #[serde(rename="fullFileExtension")]
    
    pub full_file_extension: Option<String>,
    /// Whether there are permissions directly on this file. This field is only populated for items in shared drives.
    #[serde(rename="hasAugmentedPermissions")]
    
    pub has_augmented_permissions: Option<bool>,
    /// Whether this file has a thumbnail. This does not indicate whether the requesting app has access to the thumbnail. To check access, look for the presence of the thumbnailLink field.
    #[serde(rename="hasThumbnail")]
    
    pub has_thumbnail: Option<bool>,
    /// The ID of the file's head revision. This is currently only available for files with binary content in Google Drive.
    #[serde(rename="headRevisionId")]
    
    pub head_revision_id: Option<String>,
    /// A static, unauthenticated link to the file's icon.
    #[serde(rename="iconLink")]
    
    pub icon_link: Option<String>,
    /// The ID of the file.
    
    pub id: Option<String>,
    /// Additional metadata about image media, if available.
    #[serde(rename="imageMediaMetadata")]
    
    pub image_media_metadata: Option<FileImageMediaMetadata>,
    /// Whether the file was created or opened by the requesting app.
    #[serde(rename="isAppAuthorized")]
    
    pub is_app_authorized: Option<bool>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#file".
    
    pub kind: Option<String>,
    /// An overview of the labels on the file.
    #[serde(rename="labelInfo")]
    
    pub label_info: Option<FileLabelInfo>,
    /// The last user to modify the file.
    #[serde(rename="lastModifyingUser")]
    
    pub last_modifying_user: Option<User>,
    /// Contains details about the link URLs that clients are using to refer to this item.
    #[serde(rename="linkShareMetadata")]
    
    pub link_share_metadata: Option<FileLinkShareMetadata>,
    /// The MD5 checksum for the content of the file. This is only applicable to files with binary content in Google Drive.
    #[serde(rename="md5Checksum")]
    
    pub md5_checksum: Option<String>,
    /// The MIME type of the file.
    /// Google Drive will attempt to automatically detect an appropriate value from uploaded content if no value is provided. The value cannot be changed unless a new revision is uploaded.
    /// If a file is created with a Google Doc MIME type, the uploaded content will be imported if possible. The supported import formats are published in the About resource.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
    /// Whether the file has been modified by this user.
    #[serde(rename="modifiedByMe")]
    
    pub modified_by_me: Option<bool>,
    /// The last time the file was modified by the user (RFC 3339 date-time).
    #[serde(rename="modifiedByMeTime")]
    
    pub modified_by_me_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The last time the file was modified by anyone (RFC 3339 date-time).
    /// Note that setting modifiedTime will also update modifiedByMeTime for the user.
    #[serde(rename="modifiedTime")]
    
    pub modified_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The name of the file. This is not necessarily unique within a folder. Note that for immutable items such as the top level folders of shared drives, My Drive root folder, and Application Data folder the name is constant.
    
    pub name: Option<String>,
    /// The original filename of the uploaded content if available, or else the original value of the name field. This is only available for files with binary content in Google Drive.
    #[serde(rename="originalFilename")]
    
    pub original_filename: Option<String>,
    /// Whether the user owns the file. Not populated for items in shared drives.
    #[serde(rename="ownedByMe")]
    
    pub owned_by_me: Option<bool>,
    /// The owner of this file. Only certain legacy files may have more than one owner. This field isn't populated for items in shared drives.
    
    pub owners: Option<Vec<User>>,
    /// The IDs of the parent folders which contain the file.
    /// If not specified as part of a create request, the file will be placed directly in the user's My Drive folder. If not specified as part of a copy request, the file will inherit any discoverable parents of the source file. Update requests must use the addParents and removeParents parameters to modify the parents list.
    
    pub parents: Option<Vec<String>>,
    /// List of permission IDs for users with access to this file.
    #[serde(rename="permissionIds")]
    
    pub permission_ids: Option<Vec<String>>,
    /// The full list of permissions for the file. This is only available if the requesting user can share the file. Not populated for items in shared drives.
    
    pub permissions: Option<Vec<Permission>>,
    /// A collection of arbitrary key-value pairs which are visible to all apps.
    /// Entries with null values are cleared in update and copy requests.
    
    pub properties: Option<HashMap<String, String>>,
    /// The number of storage quota bytes used by the file. This includes the head revision as well as previous revisions with keepForever enabled.
    #[serde(rename="quotaBytesUsed")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub quota_bytes_used: Option<i64>,
    /// A key needed to access the item via a shared link.
    #[serde(rename="resourceKey")]
    
    pub resource_key: Option<String>,
    /// The SHA1 checksum associated with this file, if available. This field is only populated for files with content stored in Google Drive; it isn't populated for Docs Editors or shortcut files.
    #[serde(rename="sha1Checksum")]
    
    pub sha1_checksum: Option<String>,
    /// The SHA256 checksum associated with this file, if available. This field is only populated for files with content stored in Google Drive; it isn't populated for Docs Editors or shortcut files.
    #[serde(rename="sha256Checksum")]
    
    pub sha256_checksum: Option<String>,
    /// Whether the file has been shared. Not populated for items in shared drives.
    
    pub shared: Option<bool>,
    /// The time at which the file was shared with the user, if applicable (RFC 3339 date-time).
    #[serde(rename="sharedWithMeTime")]
    
    pub shared_with_me_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The user who shared the file with the requesting user, if applicable.
    #[serde(rename="sharingUser")]
    
    pub sharing_user: Option<User>,
    /// Shortcut file details. Only populated for shortcut files, which have the mimeType field set to application/vnd.google-apps.shortcut.
    #[serde(rename="shortcutDetails")]
    
    pub shortcut_details: Option<FileShortcutDetails>,
    /// The size of the file's content in bytes. This is applicable to binary files in Google Drive and Google Docs files.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub size: Option<i64>,
    /// The list of spaces which contain the file. The currently supported values are 'drive', 'appDataFolder' and 'photos'.
    
    pub spaces: Option<Vec<String>>,
    /// Whether the user has starred the file.
    
    pub starred: Option<bool>,
    /// Deprecated - use driveId instead.
    #[serde(rename="teamDriveId")]
    
    pub team_drive_id: Option<String>,
    /// A short-lived link to the file's thumbnail, if available. Typically lasts on the order of hours. Only populated when the requesting app can access the file's content. If the file isn't shared publicly, the URL returned in Files.thumbnailLink must be fetched using a credentialed request.
    #[serde(rename="thumbnailLink")]
    
    pub thumbnail_link: Option<String>,
    /// The thumbnail version for use in thumbnail cache invalidation.
    #[serde(rename="thumbnailVersion")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub thumbnail_version: Option<i64>,
    /// Whether the file has been trashed, either explicitly or from a trashed parent folder. Only the owner may trash a file. The trashed item is excluded from all files.list responses returned for any user who does not own the file. However, all users with access to the file can see the trashed item metadata in an API response. All users with access can copy, download, export, and share the file.
    
    pub trashed: Option<bool>,
    /// The time that the item was trashed (RFC 3339 date-time). Only populated for items in shared drives.
    #[serde(rename="trashedTime")]
    
    pub trashed_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// If the file has been explicitly trashed, the user who trashed it. Only populated for items in shared drives.
    #[serde(rename="trashingUser")]
    
    pub trashing_user: Option<User>,
    /// A monotonically increasing version number for the file. This reflects every change made to the file on the server, even those not visible to the user.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub version: Option<i64>,
    /// Additional metadata about video media. This may not be available immediately upon upload.
    #[serde(rename="videoMediaMetadata")]
    
    pub video_media_metadata: Option<FileVideoMediaMetadata>,
    /// Whether the file has been viewed by this user.
    #[serde(rename="viewedByMe")]
    
    pub viewed_by_me: Option<bool>,
    /// The last time the file was viewed by the user (RFC 3339 date-time).
    #[serde(rename="viewedByMeTime")]
    
    pub viewed_by_me_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Deprecated - use copyRequiresWriterPermission instead.
    #[serde(rename="viewersCanCopyContent")]
    
    pub viewers_can_copy_content: Option<bool>,
    /// A link for downloading the content of the file in a browser. This is only available for files with binary content in Google Drive.
    #[serde(rename="webContentLink")]
    
    pub web_content_link: Option<String>,
    /// A link for opening the file in a relevant Google editor or viewer in a browser.
    #[serde(rename="webViewLink")]
    
    pub web_view_link: Option<String>,
    /// Whether users with only writer permission can modify the file's permissions. Not populated for items in shared drives.
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
    /// The list of files. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched.
    
    pub files: Option<Vec<File>>,
    /// Whether the search process was incomplete. If true, then some search results may be missing, since all documents were not searched. This may occur when searching multiple drives with the "allDrives" corpora, but all corpora could not be searched. When this happens, it is suggested that clients narrow their query by choosing a different corpus such as "user" or "drive".
    #[serde(rename="incompleteSearch")]
    
    pub incomplete_search: Option<bool>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#fileList".
    
    pub kind: Option<String>,
    /// The page token for the next page of files. This will be absent if the end of the files list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for FileList {}


/// A list of generated file IDs which can be provided in create requests.
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
    /// Identifies what kind of resource this is. Value: the fixed string "drive#generatedIds".
    
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
    /// The ID of the Field to be modified.
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
    /// This is always drive#labelList
    
    pub kind: Option<String>,
    /// The list of labels.
    
    pub labels: Option<Vec<Label>>,
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


/// A permission for a file. A permission grants a user, group, domain or the world access to a file or a folder hierarchy.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create permissions](PermissionCreateCall) (request|response)
/// * [delete permissions](PermissionDeleteCall) (none)
/// * [get permissions](PermissionGetCall) (response)
/// * [list permissions](PermissionListCall) (none)
/// * [update permissions](PermissionUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Permission {
    /// Whether the permission allows the file to be discovered through search. This is only applicable for permissions of type domain or anyone.
    #[serde(rename="allowFileDiscovery")]
    
    pub allow_file_discovery: Option<bool>,
    /// Whether the account associated with this permission has been deleted. This field only pertains to user and group permissions.
    
    pub deleted: Option<bool>,
    /// The "pretty" name of the value of the permission. The following is a list of examples for each type of permission:  
    /// - user - User's full name, as defined for their Google account, such as "Joe Smith." 
    /// - group - Name of the Google Group, such as "The Company Administrators." 
    /// - domain - String domain name, such as "thecompany.com." 
    /// - anyone - No displayName is present.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The domain to which this permission refers.
    
    pub domain: Option<String>,
    /// The email address of the user or group to which this permission refers.
    #[serde(rename="emailAddress")]
    
    pub email_address: Option<String>,
    /// The time at which this permission will expire (RFC 3339 date-time). Expiration times have the following restrictions:  
    /// - They cannot be set on shared drive items 
    /// - They can only be set on user and group permissions 
    /// - The time must be in the future 
    /// - The time cannot be more than a year in the future
    #[serde(rename="expirationTime")]
    
    pub expiration_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The ID of this permission. This is a unique identifier for the grantee, and is published in User resources as permissionId. IDs should be treated as opaque values.
    
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#permission".
    
    pub kind: Option<String>,
    /// Whether the account associated with this permission is a pending owner. Only populated for user type permissions for files that are not in a shared drive.
    #[serde(rename="pendingOwner")]
    
    pub pending_owner: Option<bool>,
    /// Details of whether the permissions on this shared drive item are inherited or directly on this item. This is an output-only field which is present only for shared drive items.
    #[serde(rename="permissionDetails")]
    
    pub permission_details: Option<Vec<PermissionPermissionDetails>>,
    /// A link to the user's profile photo, if available.
    #[serde(rename="photoLink")]
    
    pub photo_link: Option<String>,
    /// The role granted by this permission. While new values may be supported in the future, the following are currently allowed:  
    /// - owner 
    /// - organizer 
    /// - fileOrganizer 
    /// - writer 
    /// - commenter 
    /// - reader
    
    pub role: Option<String>,
    /// Deprecated - use permissionDetails instead.
    #[serde(rename="teamDrivePermissionDetails")]
    
    pub team_drive_permission_details: Option<Vec<PermissionTeamDrivePermissionDetails>>,
    /// The type of the grantee. Valid values are:  
    /// - user 
    /// - group 
    /// - domain 
    /// - anyone  When creating a permission, if type is user or group, you must provide an emailAddress for the user or group. When type is domain, you must provide a domain. There isn't extra information required for a anyone type.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Indicates the view for this permission. Only populated for permissions that belong to a view. published is the only supported value.
    
    pub view: Option<String>,
}

impl client::RequestValue for Permission {}
impl client::Resource for Permission {}
impl client::ResponseResult for Permission {}


/// A list of permissions for a file.
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
    /// Identifies what kind of resource this is. Value: the fixed string "drive#permissionList".
    
    pub kind: Option<String>,
    /// The page token for the next page of permissions. This field will be absent if the end of the permissions list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of permissions. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched.
    
    pub permissions: Option<Vec<Permission>>,
}

impl client::ResponseResult for PermissionList {}


/// A reply to a comment on a file.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create replies](ReplyCreateCall) (request|response)
/// * [get replies](ReplyGetCall) (response)
/// * [update replies](ReplyUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Reply {
    /// The action the reply performed to the parent comment. Valid values are:  
    /// - resolve 
    /// - reopen
    
    pub action: Option<String>,
    /// The author of the reply. The author's email address and permission ID will not be populated.
    
    pub author: Option<User>,
    /// The plain text content of the reply. This field is used for setting the content, while htmlContent should be displayed. This is required on creates if no action is specified.
    
    pub content: Option<String>,
    /// The time at which the reply was created (RFC 3339 date-time).
    #[serde(rename="createdTime")]
    
    pub created_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Whether the reply has been deleted. A deleted reply has no content.
    
    pub deleted: Option<bool>,
    /// The content of the reply with HTML formatting.
    #[serde(rename="htmlContent")]
    
    pub html_content: Option<String>,
    /// The ID of the reply.
    
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#reply".
    
    pub kind: Option<String>,
    /// The last time the reply was modified (RFC 3339 date-time).
    #[serde(rename="modifiedTime")]
    
    pub modified_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Reply {}
impl client::ResponseResult for Reply {}


/// A list of replies to a comment on a file.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list replies](ReplyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReplyList {
    /// Identifies what kind of resource this is. Value: the fixed string "drive#replyList".
    
    pub kind: Option<String>,
    /// The page token for the next page of replies. This will be absent if the end of the replies list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of replies. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched.
    
    pub replies: Option<Vec<Reply>>,
}

impl client::ResponseResult for ReplyList {}


/// The metadata for a revision to a file.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete revisions](RevisionDeleteCall) (none)
/// * [get revisions](RevisionGetCall) (response)
/// * [list revisions](RevisionListCall) (none)
/// * [update revisions](RevisionUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Revision {
    /// Links for exporting Docs Editors files to specific formats.
    #[serde(rename="exportLinks")]
    
    pub export_links: Option<HashMap<String, String>>,
    /// The ID of the revision.
    
    pub id: Option<String>,
    /// Whether to keep this revision forever, even if it is no longer the head revision. If not set, the revision will be automatically purged 30 days after newer content is uploaded. This can be set on a maximum of 200 revisions for a file.
    /// This field is only applicable to files with binary content in Drive.
    #[serde(rename="keepForever")]
    
    pub keep_forever: Option<bool>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#revision".
    
    pub kind: Option<String>,
    /// The last user to modify this revision.
    #[serde(rename="lastModifyingUser")]
    
    pub last_modifying_user: Option<User>,
    /// The MD5 checksum of the revision's content. This is only applicable to files with binary content in Drive.
    #[serde(rename="md5Checksum")]
    
    pub md5_checksum: Option<String>,
    /// The MIME type of the revision.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
    /// The last time the revision was modified (RFC 3339 date-time).
    #[serde(rename="modifiedTime")]
    
    pub modified_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The original filename used to create this revision. This is only applicable to files with binary content in Drive.
    #[serde(rename="originalFilename")]
    
    pub original_filename: Option<String>,
    /// Whether subsequent revisions will be automatically republished. This is only applicable to Docs Editors files.
    #[serde(rename="publishAuto")]
    
    pub publish_auto: Option<bool>,
    /// Whether this revision is published. This is only applicable to Docs Editors files.
    
    pub published: Option<bool>,
    /// A link to the published revision. This is only populated for Google Sites files.
    #[serde(rename="publishedLink")]
    
    pub published_link: Option<String>,
    /// Whether this revision is published outside the domain. This is only applicable to Docs Editors files.
    #[serde(rename="publishedOutsideDomain")]
    
    pub published_outside_domain: Option<bool>,
    /// The size of the revision's content in bytes. This is only applicable to files with binary content in Drive.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub size: Option<i64>,
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
    /// Identifies what kind of resource this is. Value: the fixed string "drive#revisionList".
    
    pub kind: Option<String>,
    /// The page token for the next page of revisions. This will be absent if the end of the revisions list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of revisions. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched.
    
    pub revisions: Option<Vec<Revision>>,
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
/// * [create teamdrives](TeamdriveCreateCall) (request|response)
/// * [get teamdrives](TeamdriveGetCall) (response)
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
    #[serde(rename="createdTime")]
    
    pub created_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The ID of this Team Drive which is also the ID of the top level folder of this Team Drive.
    
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#teamDrive".
    
    pub kind: Option<String>,
    /// The name of this Team Drive.
    
    pub name: Option<String>,
    /// The organizational unit of this shared drive. This field is only populated on drives.list responses when the useDomainAdminAccess parameter is set to true.
    #[serde(rename="orgUnitId")]
    
    pub org_unit_id: Option<String>,
    /// A set of restrictions that apply to this Team Drive or items inside this Team Drive.
    
    pub restrictions: Option<TeamDriveRestrictions>,
    /// The ID of the theme from which the background image and color will be set. The set of possible teamDriveThemes can be retrieved from a drive.about.get response. When not specified on a drive.teamdrives.create request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set colorRgb or backgroundImageFile.
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
    /// Identifies what kind of resource this is. Value: the fixed string "drive#teamDriveList".
    
    pub kind: Option<String>,
    /// The page token for the next page of Team Drives. This will be absent if the end of the Team Drives list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of Team Drives. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched.
    #[serde(rename="teamDrives")]
    
    pub team_drives: Option<Vec<TeamDrive>>,
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
    /// The email address of the user. This may not be present in certain contexts if the user has not made their email address visible to the requester.
    #[serde(rename="emailAddress")]
    
    pub email_address: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#user".
    
    pub kind: Option<String>,
    /// Whether this user is the requesting user.
    
    pub me: Option<bool>,
    /// The user's ID as visible in Permission resources.
    #[serde(rename="permissionId")]
    
    pub permission_id: Option<String>,
    /// A link to the user's profile photo, if available.
    #[serde(rename="photoLink")]
    
    pub photo_link: Option<String>,
}

impl client::Part for User {}


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


/// The user's storage quota limits and usage. All fields are measured in bytes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AboutStorageQuota {
    /// The usage limit, if applicable. This will not be present if the user has unlimited storage.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub limit: Option<i64>,
    /// The total usage across all services.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub usage: Option<i64>,
    /// The usage by all files in Google Drive.
    #[serde(rename="usageInDrive")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub usage_in_drive: Option<i64>,
    /// The usage by trashed files in Google Drive.
    #[serde(rename="usageInDriveTrash")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub usage_in_drive_trash: Option<i64>,
}

impl client::NestedType for AboutStorageQuota {}
impl client::Part for AboutStorageQuota {}


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


/// The file content to which the comment refers, typically within the anchor region. For a text file, for example, this would be the text at the location of the comment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommentQuotedFileContent {
    /// The MIME type of the quoted content.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
    /// The quoted content itself. This is interpreted as plain text if set through the API.
    
    pub value: Option<String>,
}

impl client::NestedType for CommentQuotedFileContent {}
impl client::Part for CommentQuotedFileContent {}


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
    /// Whether the current user can change the securityUpdateEnabled field on link share metadata.
    #[serde(rename="canChangeSecurityUpdateEnabled")]
    
    pub can_change_security_update_enabled: Option<bool>,
    /// Deprecated
    #[serde(rename="canChangeViewersCanCopyContent")]
    
    pub can_change_viewers_can_copy_content: Option<bool>,
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


/// Additional information about the content of the file. These fields are never populated in responses.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileContentHints {
    /// Text to be indexed for the file to improve fullText queries. This is limited to 128 KB in length and may contain HTML elements. For more information, see Manage file metadata.
    #[serde(rename="indexableText")]
    
    pub indexable_text: Option<String>,
    /// A thumbnail for the file. This will only be used if Google Drive cannot generate a standard thumbnail.
    
    pub thumbnail: Option<FileContentHintsThumbnail>,
}

impl client::NestedType for FileContentHints {}
impl client::Part for FileContentHints {}


/// A thumbnail for the file. This will only be used if Google Drive cannot generate a standard thumbnail.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileContentHintsThumbnail {
    /// The thumbnail data encoded with URL-safe Base64 (RFC 4648 section 5).
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub image: Option<Vec<u8>>,
    /// The MIME type of the thumbnail.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
}

impl client::NestedType for FileContentHintsThumbnail {}
impl client::Part for FileContentHintsThumbnail {}


/// Additional metadata about image media, if available.
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
    /// The date and time the photo was taken (EXIF DateTime).
    
    pub time: Option<String>,
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


/// Additional metadata about video media. This may not be available immediately upon upload.
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
    /// - commenter 
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


