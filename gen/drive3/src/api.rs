use std::collections::HashMap;
use std::cell::RefCell;
use std::default::Default;
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::sync::Arc;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;
use std::thread::sleep;

use http::Uri;
use hyper::client::connect;
use tokio::io::{AsyncRead, AsyncWrite};
use tower_service;
use serde::{Serialize, Deserialize};

use crate::{client, client::GetToken, client::oauth2};

// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// See, edit, create, and delete all of your Google Drive files
    Full,

    /// See, create, and delete its own configuration data in your Google Drive
    Appdata,

    /// See, edit, create, and delete only the specific Google Drive files you use with this app
    File,

    /// View and manage metadata of files in your Google Drive
    Metadata,

    /// See information about your Google Drive files
    MetadataReadonly,

    /// View the photos, videos and albums in your Google Photos
    PhotoReadonly,

    /// See and download all your Google Drive files
    Readonly,

    /// Modify your Google Apps Script scripts' behavior
    Script,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Full => "https://www.googleapis.com/auth/drive",
            Scope::Appdata => "https://www.googleapis.com/auth/drive.appdata",
            Scope::File => "https://www.googleapis.com/auth/drive.file",
            Scope::Metadata => "https://www.googleapis.com/auth/drive.metadata",
            Scope::MetadataReadonly => "https://www.googleapis.com/auth/drive.metadata.readonly",
            Scope::PhotoReadonly => "https://www.googleapis.com/auth/drive.photos.readonly",
            Scope::Readonly => "https://www.googleapis.com/auth/drive.readonly",
            Scope::Script => "https://www.googleapis.com/auth/drive.scripts",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::MetadataReadonly
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all DriveHub related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_drive3 as drive3;
/// use drive3::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.files().list()
///              .team_drive_id("eos")
///              .supports_team_drives(false)
///              .supports_all_drives(true)
///              .spaces("duo")
///              .q("sed")
///              .page_token("no")
///              .page_size(-15)
///              .order_by("kasd")
///              .include_team_drive_items(true)
///              .include_permissions_for_view("et")
///              .include_items_from_all_drives(true)
///              .drive_id("vero")
///              .corpus("erat")
///              .corpora("sed")
///              .doit().await;
/// 
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
///         |Error::Io(_)
///         |Error::MissingAPIKey
///         |Error::MissingToken(_)
///         |Error::Cancelled
///         |Error::UploadSizeLimitExceeded(_, _)
///         |Error::Failure(_)
///         |Error::BadRequest(_)
///         |Error::FieldClash(_)
///         |Error::JsonDecodeError(_, _) => println!("{}", e),
///     },
///     Ok(res) => println!("Success: {:?}", res),
/// }
/// # }
/// ```
#[derive(Clone)]
pub struct DriveHub<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: Arc<Box<dyn client::GetToken + Send + Sync>>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, S> client::Hub for DriveHub<S> {}

impl<'a, S> DriveHub<S> {

    pub fn new<A: 'static + client::GetToken + Send + Sync>(client: hyper::Client<S, hyper::body::Body>, auth: A) -> DriveHub<S> {
        DriveHub {
            client,
            auth: Arc::new(Box::new(auth)),
            _user_agent: "google-api-rust-client/4.0.2".to_string(),
            _base_url: "https://www.googleapis.com/drive/v3/".to_string(),
            _root_url: "https://www.googleapis.com/".to_string(),
        }
    }

    pub fn about(&'a self) -> AboutMethods<'a, S> {
        AboutMethods { hub: &self }
    }
    pub fn changes(&'a self) -> ChangeMethods<'a, S> {
        ChangeMethods { hub: &self }
    }
    pub fn channels(&'a self) -> ChannelMethods<'a, S> {
        ChannelMethods { hub: &self }
    }
    pub fn comments(&'a self) -> CommentMethods<'a, S> {
        CommentMethods { hub: &self }
    }
    pub fn drives(&'a self) -> DriveMethods<'a, S> {
        DriveMethods { hub: &self }
    }
    pub fn files(&'a self) -> FileMethods<'a, S> {
        FileMethods { hub: &self }
    }
    pub fn permissions(&'a self) -> PermissionMethods<'a, S> {
        PermissionMethods { hub: &self }
    }
    pub fn replies(&'a self) -> ReplyMethods<'a, S> {
        ReplyMethods { hub: &self }
    }
    pub fn revisions(&'a self) -> RevisionMethods<'a, S> {
        RevisionMethods { hub: &self }
    }
    pub fn teamdrives(&'a self) -> TeamdriveMethods<'a, S> {
        TeamdriveMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/4.0.2`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/drive/v3/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// Information about the user, the user's Drive, and system capabilities.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get about](AboutGetCall) (response)
/// 
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
    pub max_import_sizes: Option<HashMap<String, String>>,
    /// The maximum upload size in bytes.
    #[serde(rename="maxUploadSize")]
    pub max_upload_size: Option<String>,
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
/// 
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
    pub time: Option<String>,
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
/// 
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
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Channel {
    /// The address where notifications are delivered for this channel.
    pub address: Option<String>,
    /// Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional.
    pub expiration: Option<String>,
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
/// 
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
    pub created_time: Option<String>,
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
    pub modified_time: Option<String>,
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
/// 
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
    pub restriction_time: Option<String>,
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
/// 
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
    pub created_time: Option<String>,
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
/// 
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
/// * [update files](FileUpdateCall) (request|response)
/// * [watch files](FileWatchCall) (none)
/// 
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
    pub created_time: Option<String>,
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
    /// This is automatically updated when the name field changes, however it is not cleared if the new name does not contain a valid extension.
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
    pub modified_by_me_time: Option<String>,
    /// The last time the file was modified by anyone (RFC 3339 date-time).
    /// Note that setting modifiedTime will also update modifiedByMeTime for the user.
    #[serde(rename="modifiedTime")]
    pub modified_time: Option<String>,
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
    pub quota_bytes_used: Option<String>,
    /// A key needed to access the item via a shared link.
    #[serde(rename="resourceKey")]
    pub resource_key: Option<String>,
    /// Whether the file has been shared. Not populated for items in shared drives.
    pub shared: Option<bool>,
    /// The time at which the file was shared with the user, if applicable (RFC 3339 date-time).
    #[serde(rename="sharedWithMeTime")]
    pub shared_with_me_time: Option<String>,
    /// The user who shared the file with the requesting user, if applicable.
    #[serde(rename="sharingUser")]
    pub sharing_user: Option<User>,
    /// Shortcut file details. Only populated for shortcut files, which have the mimeType field set to application/vnd.google-apps.shortcut.
    #[serde(rename="shortcutDetails")]
    pub shortcut_details: Option<FileShortcutDetails>,
    /// The size of the file's content in bytes. This is applicable to binary files in Google Drive and Google Docs files.
    pub size: Option<String>,
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
    pub thumbnail_version: Option<String>,
    /// Whether the file has been trashed, either explicitly or from a trashed parent folder. Only the owner may trash a file. The trashed item is excluded from all files.list responses returned for any user who does not own the file. However, all users with access to the file can see the trashed item metadata in an API response. All users with access can copy, download, export, and share the file.
    pub trashed: Option<bool>,
    /// The time that the item was trashed (RFC 3339 date-time). Only populated for items in shared drives.
    #[serde(rename="trashedTime")]
    pub trashed_time: Option<String>,
    /// If the file has been explicitly trashed, the user who trashed it. Only populated for items in shared drives.
    #[serde(rename="trashingUser")]
    pub trashing_user: Option<User>,
    /// A monotonically increasing version number for the file. This reflects every change made to the file on the server, even those not visible to the user.
    pub version: Option<String>,
    /// Additional metadata about video media. This may not be available immediately upon upload.
    #[serde(rename="videoMediaMetadata")]
    pub video_media_metadata: Option<FileVideoMediaMetadata>,
    /// Whether the file has been viewed by this user.
    #[serde(rename="viewedByMe")]
    pub viewed_by_me: Option<bool>,
    /// The last time the file was viewed by the user (RFC 3339 date-time).
    #[serde(rename="viewedByMeTime")]
    pub viewed_by_me_time: Option<String>,
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
/// 
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
/// 
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
/// 
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
    /// - They can only be set on user and group permissions 
    /// - The time must be in the future 
    /// - The time cannot be more than a year in the future
    #[serde(rename="expirationTime")]
    pub expiration_time: Option<String>,
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
/// 
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
/// 
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
    pub created_time: Option<String>,
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
    pub modified_time: Option<String>,
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
/// 
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
/// 
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
    pub modified_time: Option<String>,
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
    pub size: Option<String>,
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
/// 
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
/// 
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
/// 
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
    pub created_time: Option<String>,
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
/// 
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
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AboutStorageQuota {
    /// The usage limit, if applicable. This will not be present if the user has unlimited storage.
    pub limit: Option<String>,
    /// The total usage across all services.
    pub usage: Option<String>,
    /// The usage by all files in Google Drive.
    #[serde(rename="usageInDrive")]
    pub usage_in_drive: Option<String>,
    /// The usage by trashed files in Google Drive.
    #[serde(rename="usageInDriveTrash")]
    pub usage_in_drive_trash: Option<String>,
}

impl client::NestedType for AboutStorageQuota {}
impl client::Part for AboutStorageQuota {}


/// Deprecated - use driveThemes instead.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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
    /// Whether the current user can read the revisions resource of this file. For a shared drive item, whether revisions of non-folder descendants of this item, or this item itself if it is not a folder, can be read.
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
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileContentHints {
    /// Text to be indexed for the file to improve fullText queries. This is limited to 128KB in length and may contain HTML elements.
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
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileContentHintsThumbnail {
    /// The thumbnail data encoded with URL-safe Base64 (RFC 4648 section 5).
    pub image: Option<String>,
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


/// Contains details about the link URLs that clients are using to refer to this item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileVideoMediaMetadata {
    /// The duration of the video in milliseconds.
    #[serde(rename="durationMillis")]
    pub duration_millis: Option<String>,
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



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *about* resources.
/// It is not used directly, but through the `DriveHub` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_drive3 as drive3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.about();
/// # }
/// ```
pub struct AboutMethods<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
}

impl<'a, S> client::MethodsBuilder for AboutMethods<'a, S> {}

impl<'a, S> AboutMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information about the user, the user's Drive, and system capabilities.
    pub fn get(&self) -> AboutGetCall<'a, S> {
        AboutGetCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *change* resources.
/// It is not used directly, but through the `DriveHub` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_drive3 as drive3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_start_page_token(...)`, `list(...)` and `watch(...)`
/// // to build up your call.
/// let rb = hub.changes();
/// # }
/// ```
pub struct ChangeMethods<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
}

impl<'a, S> client::MethodsBuilder for ChangeMethods<'a, S> {}

impl<'a, S> ChangeMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the starting pageToken for listing future changes.
    pub fn get_start_page_token(&self) -> ChangeGetStartPageTokenCall<'a, S> {
        ChangeGetStartPageTokenCall {
            hub: self.hub,
            _team_drive_id: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _drive_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the changes for a user or shared drive.
    /// 
    /// # Arguments
    ///
    /// * `pageToken` - The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response or to the response from the getStartPageToken method.
    pub fn list(&self, page_token: &str) -> ChangeListCall<'a, S> {
        ChangeListCall {
            hub: self.hub,
            _page_token: page_token.to_string(),
            _team_drive_id: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _spaces: Default::default(),
            _restrict_to_my_drive: Default::default(),
            _page_size: Default::default(),
            _include_team_drive_items: Default::default(),
            _include_removed: Default::default(),
            _include_permissions_for_view: Default::default(),
            _include_items_from_all_drives: Default::default(),
            _include_corpus_removals: Default::default(),
            _drive_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Subscribes to changes for a user.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `pageToken` - The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response or to the response from the getStartPageToken method.
    pub fn watch(&self, request: Channel, page_token: &str) -> ChangeWatchCall<'a, S> {
        ChangeWatchCall {
            hub: self.hub,
            _request: request,
            _page_token: page_token.to_string(),
            _team_drive_id: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _spaces: Default::default(),
            _restrict_to_my_drive: Default::default(),
            _page_size: Default::default(),
            _include_team_drive_items: Default::default(),
            _include_removed: Default::default(),
            _include_permissions_for_view: Default::default(),
            _include_items_from_all_drives: Default::default(),
            _include_corpus_removals: Default::default(),
            _drive_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *channel* resources.
/// It is not used directly, but through the `DriveHub` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_drive3 as drive3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `stop(...)`
/// // to build up your call.
/// let rb = hub.channels();
/// # }
/// ```
pub struct ChannelMethods<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
}

impl<'a, S> client::MethodsBuilder for ChannelMethods<'a, S> {}

impl<'a, S> ChannelMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Stop watching resources through this channel
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn stop(&self, request: Channel) -> ChannelStopCall<'a, S> {
        ChannelStopCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *comment* resources.
/// It is not used directly, but through the `DriveHub` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_drive3 as drive3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `delete(...)`, `get(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.comments();
/// # }
/// ```
pub struct CommentMethods<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
}

impl<'a, S> client::MethodsBuilder for CommentMethods<'a, S> {}

impl<'a, S> CommentMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new comment on a file.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID of the file.
    pub fn create(&self, request: Comment, file_id: &str) -> CommentCreateCall<'a, S> {
        CommentCreateCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a comment.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    /// * `commentId` - The ID of the comment.
    pub fn delete(&self, file_id: &str, comment_id: &str) -> CommentDeleteCall<'a, S> {
        CommentDeleteCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _comment_id: comment_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a comment by ID.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    /// * `commentId` - The ID of the comment.
    pub fn get(&self, file_id: &str, comment_id: &str) -> CommentGetCall<'a, S> {
        CommentGetCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _comment_id: comment_id.to_string(),
            _include_deleted: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists a file's comments.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    pub fn list(&self, file_id: &str) -> CommentListCall<'a, S> {
        CommentListCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _start_modified_time: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _include_deleted: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a comment with patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID of the file.
    /// * `commentId` - The ID of the comment.
    pub fn update(&self, request: Comment, file_id: &str, comment_id: &str) -> CommentUpdateCall<'a, S> {
        CommentUpdateCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _comment_id: comment_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *drive* resources.
/// It is not used directly, but through the `DriveHub` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_drive3 as drive3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `delete(...)`, `get(...)`, `hide(...)`, `list(...)`, `unhide(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.drives();
/// # }
/// ```
pub struct DriveMethods<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
}

impl<'a, S> client::MethodsBuilder for DriveMethods<'a, S> {}

impl<'a, S> DriveMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new shared drive.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `requestId` - An ID, such as a random UUID, which uniquely identifies this user's request for idempotent creation of a shared drive. A repeated request by the same user and with the same request ID will avoid creating duplicates by attempting to create the same shared drive. If the shared drive already exists a 409 error will be returned.
    pub fn create(&self, request: Drive, request_id: &str) -> DriveCreateCall<'a, S> {
        DriveCreateCall {
            hub: self.hub,
            _request: request,
            _request_id: request_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Permanently deletes a shared drive for which the user is an organizer. The shared drive cannot contain any untrashed items.
    /// 
    /// # Arguments
    ///
    /// * `driveId` - The ID of the shared drive.
    pub fn delete(&self, drive_id: &str) -> DriveDeleteCall<'a, S> {
        DriveDeleteCall {
            hub: self.hub,
            _drive_id: drive_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a shared drive's metadata by ID.
    /// 
    /// # Arguments
    ///
    /// * `driveId` - The ID of the shared drive.
    pub fn get(&self, drive_id: &str) -> DriveGetCall<'a, S> {
        DriveGetCall {
            hub: self.hub,
            _drive_id: drive_id.to_string(),
            _use_domain_admin_access: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Hides a shared drive from the default view.
    /// 
    /// # Arguments
    ///
    /// * `driveId` - The ID of the shared drive.
    pub fn hide(&self, drive_id: &str) -> DriveHideCall<'a, S> {
        DriveHideCall {
            hub: self.hub,
            _drive_id: drive_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the user's shared drives.
    pub fn list(&self) -> DriveListCall<'a, S> {
        DriveListCall {
            hub: self.hub,
            _use_domain_admin_access: Default::default(),
            _q: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Restores a shared drive to the default view.
    /// 
    /// # Arguments
    ///
    /// * `driveId` - The ID of the shared drive.
    pub fn unhide(&self, drive_id: &str) -> DriveUnhideCall<'a, S> {
        DriveUnhideCall {
            hub: self.hub,
            _drive_id: drive_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the metadate for a shared drive.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `driveId` - The ID of the shared drive.
    pub fn update(&self, request: Drive, drive_id: &str) -> DriveUpdateCall<'a, S> {
        DriveUpdateCall {
            hub: self.hub,
            _request: request,
            _drive_id: drive_id.to_string(),
            _use_domain_admin_access: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *file* resources.
/// It is not used directly, but through the `DriveHub` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_drive3 as drive3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `copy(...)`, `create(...)`, `delete(...)`, `empty_trash(...)`, `export(...)`, `generate_ids(...)`, `get(...)`, `list(...)`, `update(...)` and `watch(...)`
/// // to build up your call.
/// let rb = hub.files();
/// # }
/// ```
pub struct FileMethods<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
}

impl<'a, S> client::MethodsBuilder for FileMethods<'a, S> {}

impl<'a, S> FileMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a copy of a file and applies any requested updates with patch semantics. Folders cannot be copied.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID of the file.
    pub fn copy(&self, request: File, file_id: &str) -> FileCopyCall<'a, S> {
        FileCopyCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _ocr_language: Default::default(),
            _keep_revision_forever: Default::default(),
            _include_permissions_for_view: Default::default(),
            _ignore_default_visibility: Default::default(),
            _enforce_single_parent: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new file.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: File) -> FileCreateCall<'a, S> {
        FileCreateCall {
            hub: self.hub,
            _request: request,
            _use_content_as_indexable_text: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _ocr_language: Default::default(),
            _keep_revision_forever: Default::default(),
            _include_permissions_for_view: Default::default(),
            _ignore_default_visibility: Default::default(),
            _enforce_single_parent: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Permanently deletes a file owned by the user without moving it to the trash. If the file belongs to a shared drive the user must be an organizer on the parent. If the target is a folder, all descendants owned by the user are also deleted.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    pub fn delete(&self, file_id: &str) -> FileDeleteCall<'a, S> {
        FileDeleteCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _enforce_single_parent: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Permanently deletes all of the user's trashed files.
    pub fn empty_trash(&self) -> FileEmptyTrashCall<'a, S> {
        FileEmptyTrashCall {
            hub: self.hub,
            _enforce_single_parent: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Exports a Google Workspace document to the requested MIME type and returns exported byte content. Note that the exported content is limited to 10MB.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    /// * `mimeType` - The MIME type of the format requested for this export.
    pub fn export(&self, file_id: &str, mime_type: &str) -> FileExportCall<'a, S> {
        FileExportCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _mime_type: mime_type.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generates a set of file IDs which can be provided in create or copy requests.
    pub fn generate_ids(&self) -> FileGenerateIdCall<'a, S> {
        FileGenerateIdCall {
            hub: self.hub,
            _type_: Default::default(),
            _space: Default::default(),
            _count: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a file's metadata or content by ID.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    pub fn get(&self, file_id: &str) -> FileGetCall<'a, S> {
        FileGetCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _include_permissions_for_view: Default::default(),
            _acknowledge_abuse: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists or searches files.
    pub fn list(&self) -> FileListCall<'a, S> {
        FileListCall {
            hub: self.hub,
            _team_drive_id: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _spaces: Default::default(),
            _q: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _include_team_drive_items: Default::default(),
            _include_permissions_for_view: Default::default(),
            _include_items_from_all_drives: Default::default(),
            _drive_id: Default::default(),
            _corpus: Default::default(),
            _corpora: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a file's metadata and/or content. When calling this method, only populate fields in the request that you want to modify. When updating fields, some fields might change automatically, such as modifiedDate. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID of the file.
    pub fn update(&self, request: File, file_id: &str) -> FileUpdateCall<'a, S> {
        FileUpdateCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _use_content_as_indexable_text: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _remove_parents: Default::default(),
            _ocr_language: Default::default(),
            _keep_revision_forever: Default::default(),
            _include_permissions_for_view: Default::default(),
            _enforce_single_parent: Default::default(),
            _add_parents: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Subscribes to changes to a file. While you can establish a channel forchanges to a file on a shared drive, a change to a shared drive file won't create a notification.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID of the file.
    pub fn watch(&self, request: Channel, file_id: &str) -> FileWatchCall<'a, S> {
        FileWatchCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _include_permissions_for_view: Default::default(),
            _acknowledge_abuse: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *permission* resources.
/// It is not used directly, but through the `DriveHub` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_drive3 as drive3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `delete(...)`, `get(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.permissions();
/// # }
/// ```
pub struct PermissionMethods<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
}

impl<'a, S> client::MethodsBuilder for PermissionMethods<'a, S> {}

impl<'a, S> PermissionMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a permission for a file or shared drive.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID of the file or shared drive.
    pub fn create(&self, request: Permission, file_id: &str) -> PermissionCreateCall<'a, S> {
        PermissionCreateCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _use_domain_admin_access: Default::default(),
            _transfer_ownership: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _send_notification_email: Default::default(),
            _move_to_new_owners_root: Default::default(),
            _enforce_single_parent: Default::default(),
            _email_message: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a permission.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file or shared drive.
    /// * `permissionId` - The ID of the permission.
    pub fn delete(&self, file_id: &str, permission_id: &str) -> PermissionDeleteCall<'a, S> {
        PermissionDeleteCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _permission_id: permission_id.to_string(),
            _use_domain_admin_access: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a permission by ID.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    /// * `permissionId` - The ID of the permission.
    pub fn get(&self, file_id: &str, permission_id: &str) -> PermissionGetCall<'a, S> {
        PermissionGetCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _permission_id: permission_id.to_string(),
            _use_domain_admin_access: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists a file's or shared drive's permissions.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file or shared drive.
    pub fn list(&self, file_id: &str) -> PermissionListCall<'a, S> {
        PermissionListCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _use_domain_admin_access: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _include_permissions_for_view: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a permission with patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID of the file or shared drive.
    /// * `permissionId` - The ID of the permission.
    pub fn update(&self, request: Permission, file_id: &str, permission_id: &str) -> PermissionUpdateCall<'a, S> {
        PermissionUpdateCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _permission_id: permission_id.to_string(),
            _use_domain_admin_access: Default::default(),
            _transfer_ownership: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _remove_expiration: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *reply* resources.
/// It is not used directly, but through the `DriveHub` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_drive3 as drive3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `delete(...)`, `get(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.replies();
/// # }
/// ```
pub struct ReplyMethods<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
}

impl<'a, S> client::MethodsBuilder for ReplyMethods<'a, S> {}

impl<'a, S> ReplyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new reply to a comment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID of the file.
    /// * `commentId` - The ID of the comment.
    pub fn create(&self, request: Reply, file_id: &str, comment_id: &str) -> ReplyCreateCall<'a, S> {
        ReplyCreateCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _comment_id: comment_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a reply.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    /// * `commentId` - The ID of the comment.
    /// * `replyId` - The ID of the reply.
    pub fn delete(&self, file_id: &str, comment_id: &str, reply_id: &str) -> ReplyDeleteCall<'a, S> {
        ReplyDeleteCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _comment_id: comment_id.to_string(),
            _reply_id: reply_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a reply by ID.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    /// * `commentId` - The ID of the comment.
    /// * `replyId` - The ID of the reply.
    pub fn get(&self, file_id: &str, comment_id: &str, reply_id: &str) -> ReplyGetCall<'a, S> {
        ReplyGetCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _comment_id: comment_id.to_string(),
            _reply_id: reply_id.to_string(),
            _include_deleted: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists a comment's replies.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    /// * `commentId` - The ID of the comment.
    pub fn list(&self, file_id: &str, comment_id: &str) -> ReplyListCall<'a, S> {
        ReplyListCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _comment_id: comment_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _include_deleted: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a reply with patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID of the file.
    /// * `commentId` - The ID of the comment.
    /// * `replyId` - The ID of the reply.
    pub fn update(&self, request: Reply, file_id: &str, comment_id: &str, reply_id: &str) -> ReplyUpdateCall<'a, S> {
        ReplyUpdateCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _comment_id: comment_id.to_string(),
            _reply_id: reply_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *revision* resources.
/// It is not used directly, but through the `DriveHub` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_drive3 as drive3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.revisions();
/// # }
/// ```
pub struct RevisionMethods<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
}

impl<'a, S> client::MethodsBuilder for RevisionMethods<'a, S> {}

impl<'a, S> RevisionMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Permanently deletes a file version. You can only delete revisions for files with binary content in Google Drive, like images or videos. Revisions for other files, like Google Docs or Sheets, and the last remaining file version can't be deleted.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    /// * `revisionId` - The ID of the revision.
    pub fn delete(&self, file_id: &str, revision_id: &str) -> RevisionDeleteCall<'a, S> {
        RevisionDeleteCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _revision_id: revision_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a revision's metadata or content by ID.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    /// * `revisionId` - The ID of the revision.
    pub fn get(&self, file_id: &str, revision_id: &str) -> RevisionGetCall<'a, S> {
        RevisionGetCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _revision_id: revision_id.to_string(),
            _acknowledge_abuse: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists a file's revisions.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    pub fn list(&self, file_id: &str) -> RevisionListCall<'a, S> {
        RevisionListCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a revision with patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID of the file.
    /// * `revisionId` - The ID of the revision.
    pub fn update(&self, request: Revision, file_id: &str, revision_id: &str) -> RevisionUpdateCall<'a, S> {
        RevisionUpdateCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _revision_id: revision_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *teamdrive* resources.
/// It is not used directly, but through the `DriveHub` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_drive3 as drive3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `delete(...)`, `get(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.teamdrives();
/// # }
/// ```
pub struct TeamdriveMethods<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
}

impl<'a, S> client::MethodsBuilder for TeamdriveMethods<'a, S> {}

impl<'a, S> TeamdriveMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deprecated use drives.create instead.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `requestId` - An ID, such as a random UUID, which uniquely identifies this user's request for idempotent creation of a Team Drive. A repeated request by the same user and with the same request ID will avoid creating duplicates by attempting to create the same Team Drive. If the Team Drive already exists a 409 error will be returned.
    pub fn create(&self, request: TeamDrive, request_id: &str) -> TeamdriveCreateCall<'a, S> {
        TeamdriveCreateCall {
            hub: self.hub,
            _request: request,
            _request_id: request_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deprecated use drives.delete instead.
    /// 
    /// # Arguments
    ///
    /// * `teamDriveId` - The ID of the Team Drive
    pub fn delete(&self, team_drive_id: &str) -> TeamdriveDeleteCall<'a, S> {
        TeamdriveDeleteCall {
            hub: self.hub,
            _team_drive_id: team_drive_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deprecated use drives.get instead.
    /// 
    /// # Arguments
    ///
    /// * `teamDriveId` - The ID of the Team Drive
    pub fn get(&self, team_drive_id: &str) -> TeamdriveGetCall<'a, S> {
        TeamdriveGetCall {
            hub: self.hub,
            _team_drive_id: team_drive_id.to_string(),
            _use_domain_admin_access: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deprecated use drives.list instead.
    pub fn list(&self) -> TeamdriveListCall<'a, S> {
        TeamdriveListCall {
            hub: self.hub,
            _use_domain_admin_access: Default::default(),
            _q: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deprecated use drives.update instead
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `teamDriveId` - The ID of the Team Drive
    pub fn update(&self, request: TeamDrive, team_drive_id: &str) -> TeamdriveUpdateCall<'a, S> {
        TeamdriveUpdateCall {
            hub: self.hub,
            _request: request,
            _team_drive_id: team_drive_id.to_string(),
            _use_domain_admin_access: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Gets information about the user, the user's Drive, and system capabilities.
///
/// A builder for the *get* method supported by a *about* resource.
/// It is not used directly, but through a `AboutMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.about().get()
///              .doit().await;
/// # }
/// ```
pub struct AboutGetCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for AboutGetCall<'a, S> {}

impl<'a, S> AboutGetCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, About)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.about.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(2 + self._additional_params.len());
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "about";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::MetadataReadonly.as_ref().to_string(), ());
        }


        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AboutGetCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> AboutGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::MetadataReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> AboutGetCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets the starting pageToken for listing future changes.
///
/// A builder for the *getStartPageToken* method supported by a *change* resource.
/// It is not used directly, but through a `ChangeMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.changes().get_start_page_token()
///              .team_drive_id("duo")
///              .supports_team_drives(false)
///              .supports_all_drives(false)
///              .drive_id("dolor")
///              .doit().await;
/// # }
/// ```
pub struct ChangeGetStartPageTokenCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _team_drive_id: Option<String>,
    _supports_team_drives: Option<bool>,
    _supports_all_drives: Option<bool>,
    _drive_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for ChangeGetStartPageTokenCall<'a, S> {}

impl<'a, S> ChangeGetStartPageTokenCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, StartPageToken)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.changes.getStartPageToken",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        if let Some(value) = self._team_drive_id {
            params.push(("teamDriveId", value.to_string()));
        }
        if let Some(value) = self._supports_team_drives {
            params.push(("supportsTeamDrives", value.to_string()));
        }
        if let Some(value) = self._supports_all_drives {
            params.push(("supportsAllDrives", value.to_string()));
        }
        if let Some(value) = self._drive_id {
            params.push(("driveId", value.to_string()));
        }
        for &field in ["alt", "teamDriveId", "supportsTeamDrives", "supportsAllDrives", "driveId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "changes/startPageToken";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::MetadataReadonly.as_ref().to_string(), ());
        }


        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Deprecated use driveId instead.
    ///
    /// Sets the *team drive id* query property to the given value.
    pub fn team_drive_id(mut self, new_value: &str) -> ChangeGetStartPageTokenCall<'a, S> {
        self._team_drive_id = Some(new_value.to_string());
        self
    }
    /// Deprecated use supportsAllDrives instead.
    ///
    /// Sets the *supports team drives* query property to the given value.
    pub fn supports_team_drives(mut self, new_value: bool) -> ChangeGetStartPageTokenCall<'a, S> {
        self._supports_team_drives = Some(new_value);
        self
    }
    /// Whether the requesting application supports both My Drives and shared drives.
    ///
    /// Sets the *supports all drives* query property to the given value.
    pub fn supports_all_drives(mut self, new_value: bool) -> ChangeGetStartPageTokenCall<'a, S> {
        self._supports_all_drives = Some(new_value);
        self
    }
    /// The ID of the shared drive for which the starting pageToken for listing future changes from that shared drive is returned.
    ///
    /// Sets the *drive id* query property to the given value.
    pub fn drive_id(mut self, new_value: &str) -> ChangeGetStartPageTokenCall<'a, S> {
        self._drive_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ChangeGetStartPageTokenCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ChangeGetStartPageTokenCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::MetadataReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> ChangeGetStartPageTokenCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists the changes for a user or shared drive.
///
/// A builder for the *list* method supported by a *change* resource.
/// It is not used directly, but through a `ChangeMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.changes().list("pageToken")
///              .team_drive_id("et")
///              .supports_team_drives(false)
///              .supports_all_drives(false)
///              .spaces("duo")
///              .restrict_to_my_drive(false)
///              .page_size(-76)
///              .include_team_drive_items(false)
///              .include_removed(true)
///              .include_permissions_for_view("vero")
///              .include_items_from_all_drives(true)
///              .include_corpus_removals(true)
///              .drive_id("ipsum")
///              .doit().await;
/// # }
/// ```
pub struct ChangeListCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _page_token: String,
    _team_drive_id: Option<String>,
    _supports_team_drives: Option<bool>,
    _supports_all_drives: Option<bool>,
    _spaces: Option<String>,
    _restrict_to_my_drive: Option<bool>,
    _page_size: Option<i32>,
    _include_team_drive_items: Option<bool>,
    _include_removed: Option<bool>,
    _include_permissions_for_view: Option<String>,
    _include_items_from_all_drives: Option<bool>,
    _include_corpus_removals: Option<bool>,
    _drive_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for ChangeListCall<'a, S> {}

impl<'a, S> ChangeListCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ChangeList)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.changes.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(15 + self._additional_params.len());
        params.push(("pageToken", self._page_token.to_string()));
        if let Some(value) = self._team_drive_id {
            params.push(("teamDriveId", value.to_string()));
        }
        if let Some(value) = self._supports_team_drives {
            params.push(("supportsTeamDrives", value.to_string()));
        }
        if let Some(value) = self._supports_all_drives {
            params.push(("supportsAllDrives", value.to_string()));
        }
        if let Some(value) = self._spaces {
            params.push(("spaces", value.to_string()));
        }
        if let Some(value) = self._restrict_to_my_drive {
            params.push(("restrictToMyDrive", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._include_team_drive_items {
            params.push(("includeTeamDriveItems", value.to_string()));
        }
        if let Some(value) = self._include_removed {
            params.push(("includeRemoved", value.to_string()));
        }
        if let Some(value) = self._include_permissions_for_view {
            params.push(("includePermissionsForView", value.to_string()));
        }
        if let Some(value) = self._include_items_from_all_drives {
            params.push(("includeItemsFromAllDrives", value.to_string()));
        }
        if let Some(value) = self._include_corpus_removals {
            params.push(("includeCorpusRemovals", value.to_string()));
        }
        if let Some(value) = self._drive_id {
            params.push(("driveId", value.to_string()));
        }
        for &field in ["alt", "pageToken", "teamDriveId", "supportsTeamDrives", "supportsAllDrives", "spaces", "restrictToMyDrive", "pageSize", "includeTeamDriveItems", "includeRemoved", "includePermissionsForView", "includeItemsFromAllDrives", "includeCorpusRemovals", "driveId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "changes";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::MetadataReadonly.as_ref().to_string(), ());
        }


        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response or to the response from the getStartPageToken method.
    ///
    /// Sets the *page token* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn page_token(mut self, new_value: &str) -> ChangeListCall<'a, S> {
        self._page_token = new_value.to_string();
        self
    }
    /// Deprecated use driveId instead.
    ///
    /// Sets the *team drive id* query property to the given value.
    pub fn team_drive_id(mut self, new_value: &str) -> ChangeListCall<'a, S> {
        self._team_drive_id = Some(new_value.to_string());
        self
    }
    /// Deprecated use supportsAllDrives instead.
    ///
    /// Sets the *supports team drives* query property to the given value.
    pub fn supports_team_drives(mut self, new_value: bool) -> ChangeListCall<'a, S> {
        self._supports_team_drives = Some(new_value);
        self
    }
    /// Whether the requesting application supports both My Drives and shared drives.
    ///
    /// Sets the *supports all drives* query property to the given value.
    pub fn supports_all_drives(mut self, new_value: bool) -> ChangeListCall<'a, S> {
        self._supports_all_drives = Some(new_value);
        self
    }
    /// A comma-separated list of spaces to query within the user corpus. Supported values are 'drive', 'appDataFolder' and 'photos'.
    ///
    /// Sets the *spaces* query property to the given value.
    pub fn spaces(mut self, new_value: &str) -> ChangeListCall<'a, S> {
        self._spaces = Some(new_value.to_string());
        self
    }
    /// Whether to restrict the results to changes inside the My Drive hierarchy. This omits changes to files such as those in the Application Data folder or shared files which have not been added to My Drive.
    ///
    /// Sets the *restrict to my drive* query property to the given value.
    pub fn restrict_to_my_drive(mut self, new_value: bool) -> ChangeListCall<'a, S> {
        self._restrict_to_my_drive = Some(new_value);
        self
    }
    /// The maximum number of changes to return per page.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> ChangeListCall<'a, S> {
        self._page_size = Some(new_value);
        self
    }
    /// Deprecated use includeItemsFromAllDrives instead.
    ///
    /// Sets the *include team drive items* query property to the given value.
    pub fn include_team_drive_items(mut self, new_value: bool) -> ChangeListCall<'a, S> {
        self._include_team_drive_items = Some(new_value);
        self
    }
    /// Whether to include changes indicating that items have been removed from the list of changes, for example by deletion or loss of access.
    ///
    /// Sets the *include removed* query property to the given value.
    pub fn include_removed(mut self, new_value: bool) -> ChangeListCall<'a, S> {
        self._include_removed = Some(new_value);
        self
    }
    /// Specifies which additional view's permissions to include in the response. Only 'published' is supported.
    ///
    /// Sets the *include permissions for view* query property to the given value.
    pub fn include_permissions_for_view(mut self, new_value: &str) -> ChangeListCall<'a, S> {
        self._include_permissions_for_view = Some(new_value.to_string());
        self
    }
    /// Whether both My Drive and shared drive items should be included in results.
    ///
    /// Sets the *include items from all drives* query property to the given value.
    pub fn include_items_from_all_drives(mut self, new_value: bool) -> ChangeListCall<'a, S> {
        self._include_items_from_all_drives = Some(new_value);
        self
    }
    /// Whether changes should include the file resource if the file is still accessible by the user at the time of the request, even when a file was removed from the list of changes and there will be no further change entries for this file.
    ///
    /// Sets the *include corpus removals* query property to the given value.
    pub fn include_corpus_removals(mut self, new_value: bool) -> ChangeListCall<'a, S> {
        self._include_corpus_removals = Some(new_value);
        self
    }
    /// The shared drive from which changes are returned. If specified the change IDs will be reflective of the shared drive; use the combined drive ID and change ID as an identifier.
    ///
    /// Sets the *drive id* query property to the given value.
    pub fn drive_id(mut self, new_value: &str) -> ChangeListCall<'a, S> {
        self._drive_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ChangeListCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ChangeListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::MetadataReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> ChangeListCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Subscribes to changes for a user.
///
/// A builder for the *watch* method supported by a *change* resource.
/// It is not used directly, but through a `ChangeMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// use drive3::api::Channel;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Channel::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.changes().watch(req, "pageToken")
///              .team_drive_id("takimata")
///              .supports_team_drives(true)
///              .supports_all_drives(false)
///              .spaces("erat")
///              .restrict_to_my_drive(false)
///              .page_size(-2)
///              .include_team_drive_items(true)
///              .include_removed(false)
///              .include_permissions_for_view("accusam")
///              .include_items_from_all_drives(false)
///              .include_corpus_removals(false)
///              .drive_id("amet.")
///              .doit().await;
/// # }
/// ```
pub struct ChangeWatchCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _request: Channel,
    _page_token: String,
    _team_drive_id: Option<String>,
    _supports_team_drives: Option<bool>,
    _supports_all_drives: Option<bool>,
    _spaces: Option<String>,
    _restrict_to_my_drive: Option<bool>,
    _page_size: Option<i32>,
    _include_team_drive_items: Option<bool>,
    _include_removed: Option<bool>,
    _include_permissions_for_view: Option<String>,
    _include_items_from_all_drives: Option<bool>,
    _include_corpus_removals: Option<bool>,
    _drive_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for ChangeWatchCall<'a, S> {}

impl<'a, S> ChangeWatchCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Channel)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.changes.watch",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(16 + self._additional_params.len());
        params.push(("pageToken", self._page_token.to_string()));
        if let Some(value) = self._team_drive_id {
            params.push(("teamDriveId", value.to_string()));
        }
        if let Some(value) = self._supports_team_drives {
            params.push(("supportsTeamDrives", value.to_string()));
        }
        if let Some(value) = self._supports_all_drives {
            params.push(("supportsAllDrives", value.to_string()));
        }
        if let Some(value) = self._spaces {
            params.push(("spaces", value.to_string()));
        }
        if let Some(value) = self._restrict_to_my_drive {
            params.push(("restrictToMyDrive", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._include_team_drive_items {
            params.push(("includeTeamDriveItems", value.to_string()));
        }
        if let Some(value) = self._include_removed {
            params.push(("includeRemoved", value.to_string()));
        }
        if let Some(value) = self._include_permissions_for_view {
            params.push(("includePermissionsForView", value.to_string()));
        }
        if let Some(value) = self._include_items_from_all_drives {
            params.push(("includeItemsFromAllDrives", value.to_string()));
        }
        if let Some(value) = self._include_corpus_removals {
            params.push(("includeCorpusRemovals", value.to_string()));
        }
        if let Some(value) = self._drive_id {
            params.push(("driveId", value.to_string()));
        }
        for &field in ["alt", "pageToken", "teamDriveId", "supportsTeamDrives", "supportsAllDrives", "spaces", "restrictToMyDrive", "pageSize", "includeTeamDriveItems", "includeRemoved", "includePermissionsForView", "includeItemsFromAllDrives", "includeCorpusRemovals", "driveId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "changes/watch";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }


        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Channel) -> ChangeWatchCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response or to the response from the getStartPageToken method.
    ///
    /// Sets the *page token* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn page_token(mut self, new_value: &str) -> ChangeWatchCall<'a, S> {
        self._page_token = new_value.to_string();
        self
    }
    /// Deprecated use driveId instead.
    ///
    /// Sets the *team drive id* query property to the given value.
    pub fn team_drive_id(mut self, new_value: &str) -> ChangeWatchCall<'a, S> {
        self._team_drive_id = Some(new_value.to_string());
        self
    }
    /// Deprecated use supportsAllDrives instead.
    ///
    /// Sets the *supports team drives* query property to the given value.
    pub fn supports_team_drives(mut self, new_value: bool) -> ChangeWatchCall<'a, S> {
        self._supports_team_drives = Some(new_value);
        self
    }
    /// Whether the requesting application supports both My Drives and shared drives.
    ///
    /// Sets the *supports all drives* query property to the given value.
    pub fn supports_all_drives(mut self, new_value: bool) -> ChangeWatchCall<'a, S> {
        self._supports_all_drives = Some(new_value);
        self
    }
    /// A comma-separated list of spaces to query within the user corpus. Supported values are 'drive', 'appDataFolder' and 'photos'.
    ///
    /// Sets the *spaces* query property to the given value.
    pub fn spaces(mut self, new_value: &str) -> ChangeWatchCall<'a, S> {
        self._spaces = Some(new_value.to_string());
        self
    }
    /// Whether to restrict the results to changes inside the My Drive hierarchy. This omits changes to files such as those in the Application Data folder or shared files which have not been added to My Drive.
    ///
    /// Sets the *restrict to my drive* query property to the given value.
    pub fn restrict_to_my_drive(mut self, new_value: bool) -> ChangeWatchCall<'a, S> {
        self._restrict_to_my_drive = Some(new_value);
        self
    }
    /// The maximum number of changes to return per page.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> ChangeWatchCall<'a, S> {
        self._page_size = Some(new_value);
        self
    }
    /// Deprecated use includeItemsFromAllDrives instead.
    ///
    /// Sets the *include team drive items* query property to the given value.
    pub fn include_team_drive_items(mut self, new_value: bool) -> ChangeWatchCall<'a, S> {
        self._include_team_drive_items = Some(new_value);
        self
    }
    /// Whether to include changes indicating that items have been removed from the list of changes, for example by deletion or loss of access.
    ///
    /// Sets the *include removed* query property to the given value.
    pub fn include_removed(mut self, new_value: bool) -> ChangeWatchCall<'a, S> {
        self._include_removed = Some(new_value);
        self
    }
    /// Specifies which additional view's permissions to include in the response. Only 'published' is supported.
    ///
    /// Sets the *include permissions for view* query property to the given value.
    pub fn include_permissions_for_view(mut self, new_value: &str) -> ChangeWatchCall<'a, S> {
        self._include_permissions_for_view = Some(new_value.to_string());
        self
    }
    /// Whether both My Drive and shared drive items should be included in results.
    ///
    /// Sets the *include items from all drives* query property to the given value.
    pub fn include_items_from_all_drives(mut self, new_value: bool) -> ChangeWatchCall<'a, S> {
        self._include_items_from_all_drives = Some(new_value);
        self
    }
    /// Whether changes should include the file resource if the file is still accessible by the user at the time of the request, even when a file was removed from the list of changes and there will be no further change entries for this file.
    ///
    /// Sets the *include corpus removals* query property to the given value.
    pub fn include_corpus_removals(mut self, new_value: bool) -> ChangeWatchCall<'a, S> {
        self._include_corpus_removals = Some(new_value);
        self
    }
    /// The shared drive from which changes are returned. If specified the change IDs will be reflective of the shared drive; use the combined drive ID and change ID as an identifier.
    ///
    /// Sets the *drive id* query property to the given value.
    pub fn drive_id(mut self, new_value: &str) -> ChangeWatchCall<'a, S> {
        self._drive_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ChangeWatchCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ChangeWatchCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> ChangeWatchCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Stop watching resources through this channel
///
/// A builder for the *stop* method supported by a *channel* resource.
/// It is not used directly, but through a `ChannelMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// use drive3::api::Channel;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Channel::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.channels().stop(req)
///              .doit().await;
/// # }
/// ```
pub struct ChannelStopCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _request: Channel,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for ChannelStopCall<'a, S> {}

impl<'a, S> ChannelStopCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<hyper::Response<hyper::body::Body>> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.channels.stop",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(2 + self._additional_params.len());
        for &field in [].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "channels/stop";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }


        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Channel) -> ChannelStopCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ChannelStopCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ChannelStopCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> ChannelStopCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a new comment on a file.
///
/// A builder for the *create* method supported by a *comment* resource.
/// It is not used directly, but through a `CommentMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// use drive3::api::Comment;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Comment::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.comments().create(req, "fileId")
///              .doit().await;
/// # }
/// ```
pub struct CommentCreateCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _request: Comment,
    _file_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for CommentCreateCall<'a, S> {}

impl<'a, S> CommentCreateCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Comment)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.comments.create",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        for &field in ["alt", "fileId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "files/{fileId}/comments";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["fileId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Comment) -> CommentCreateCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> CommentCreateCall<'a, S> {
        self._file_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CommentCreateCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> CommentCreateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> CommentCreateCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deletes a comment.
///
/// A builder for the *delete* method supported by a *comment* resource.
/// It is not used directly, but through a `CommentMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.comments().delete("fileId", "commentId")
///              .doit().await;
/// # }
/// ```
pub struct CommentDeleteCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _file_id: String,
    _comment_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for CommentDeleteCall<'a, S> {}

impl<'a, S> CommentDeleteCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<hyper::Response<hyper::body::Body>> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.comments.delete",
                               http_method: hyper::Method::DELETE });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        params.push(("commentId", self._comment_id.to_string()));
        for &field in ["fileId", "commentId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "files/{fileId}/comments/{commentId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId"), ("{commentId}", "commentId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["commentId", "fileId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::DELETE).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> CommentDeleteCall<'a, S> {
        self._file_id = new_value.to_string();
        self
    }
    /// The ID of the comment.
    ///
    /// Sets the *comment id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn comment_id(mut self, new_value: &str) -> CommentDeleteCall<'a, S> {
        self._comment_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CommentDeleteCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> CommentDeleteCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> CommentDeleteCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets a comment by ID.
///
/// A builder for the *get* method supported by a *comment* resource.
/// It is not used directly, but through a `CommentMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.comments().get("fileId", "commentId")
///              .include_deleted(true)
///              .doit().await;
/// # }
/// ```
pub struct CommentGetCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _file_id: String,
    _comment_id: String,
    _include_deleted: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for CommentGetCall<'a, S> {}

impl<'a, S> CommentGetCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Comment)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.comments.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        params.push(("commentId", self._comment_id.to_string()));
        if let Some(value) = self._include_deleted {
            params.push(("includeDeleted", value.to_string()));
        }
        for &field in ["alt", "fileId", "commentId", "includeDeleted"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "files/{fileId}/comments/{commentId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId"), ("{commentId}", "commentId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["commentId", "fileId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> CommentGetCall<'a, S> {
        self._file_id = new_value.to_string();
        self
    }
    /// The ID of the comment.
    ///
    /// Sets the *comment id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn comment_id(mut self, new_value: &str) -> CommentGetCall<'a, S> {
        self._comment_id = new_value.to_string();
        self
    }
    /// Whether to return deleted comments. Deleted comments will not include their original content.
    ///
    /// Sets the *include deleted* query property to the given value.
    pub fn include_deleted(mut self, new_value: bool) -> CommentGetCall<'a, S> {
        self._include_deleted = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CommentGetCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> CommentGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Readonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> CommentGetCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists a file's comments.
///
/// A builder for the *list* method supported by a *comment* resource.
/// It is not used directly, but through a `CommentMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.comments().list("fileId")
///              .start_modified_time("et")
///              .page_token("tempor")
///              .page_size(-32)
///              .include_deleted(true)
///              .doit().await;
/// # }
/// ```
pub struct CommentListCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _file_id: String,
    _start_modified_time: Option<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _include_deleted: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for CommentListCall<'a, S> {}

impl<'a, S> CommentListCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, CommentList)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.comments.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        if let Some(value) = self._start_modified_time {
            params.push(("startModifiedTime", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._include_deleted {
            params.push(("includeDeleted", value.to_string()));
        }
        for &field in ["alt", "fileId", "startModifiedTime", "pageToken", "pageSize", "includeDeleted"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "files/{fileId}/comments";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["fileId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> CommentListCall<'a, S> {
        self._file_id = new_value.to_string();
        self
    }
    /// The minimum value of 'modifiedTime' for the result comments (RFC 3339 date-time).
    ///
    /// Sets the *start modified time* query property to the given value.
    pub fn start_modified_time(mut self, new_value: &str) -> CommentListCall<'a, S> {
        self._start_modified_time = Some(new_value.to_string());
        self
    }
    /// The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> CommentListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of comments to return per page.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> CommentListCall<'a, S> {
        self._page_size = Some(new_value);
        self
    }
    /// Whether to include deleted comments. Deleted comments will not include their original content.
    ///
    /// Sets the *include deleted* query property to the given value.
    pub fn include_deleted(mut self, new_value: bool) -> CommentListCall<'a, S> {
        self._include_deleted = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CommentListCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> CommentListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Readonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> CommentListCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates a comment with patch semantics.
///
/// A builder for the *update* method supported by a *comment* resource.
/// It is not used directly, but through a `CommentMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// use drive3::api::Comment;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Comment::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.comments().update(req, "fileId", "commentId")
///              .doit().await;
/// # }
/// ```
pub struct CommentUpdateCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _request: Comment,
    _file_id: String,
    _comment_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for CommentUpdateCall<'a, S> {}

impl<'a, S> CommentUpdateCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Comment)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.comments.update",
                               http_method: hyper::Method::PATCH });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        params.push(("commentId", self._comment_id.to_string()));
        for &field in ["alt", "fileId", "commentId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "files/{fileId}/comments/{commentId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId"), ("{commentId}", "commentId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["commentId", "fileId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::PATCH).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Comment) -> CommentUpdateCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> CommentUpdateCall<'a, S> {
        self._file_id = new_value.to_string();
        self
    }
    /// The ID of the comment.
    ///
    /// Sets the *comment id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn comment_id(mut self, new_value: &str) -> CommentUpdateCall<'a, S> {
        self._comment_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CommentUpdateCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> CommentUpdateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> CommentUpdateCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a new shared drive.
///
/// A builder for the *create* method supported by a *drive* resource.
/// It is not used directly, but through a `DriveMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// use drive3::api::Drive;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Drive::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.drives().create(req, "requestId")
///              .doit().await;
/// # }
/// ```
pub struct DriveCreateCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _request: Drive,
    _request_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for DriveCreateCall<'a, S> {}

impl<'a, S> DriveCreateCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Drive)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.drives.create",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("requestId", self._request_id.to_string()));
        for &field in ["alt", "requestId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "drives";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }


        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Drive) -> DriveCreateCall<'a, S> {
        self._request = new_value;
        self
    }
    /// An ID, such as a random UUID, which uniquely identifies this user's request for idempotent creation of a shared drive. A repeated request by the same user and with the same request ID will avoid creating duplicates by attempting to create the same shared drive. If the shared drive already exists a 409 error will be returned.
    ///
    /// Sets the *request id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request_id(mut self, new_value: &str) -> DriveCreateCall<'a, S> {
        self._request_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> DriveCreateCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> DriveCreateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> DriveCreateCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Permanently deletes a shared drive for which the user is an organizer. The shared drive cannot contain any untrashed items.
///
/// A builder for the *delete* method supported by a *drive* resource.
/// It is not used directly, but through a `DriveMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.drives().delete("driveId")
///              .doit().await;
/// # }
/// ```
pub struct DriveDeleteCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _drive_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for DriveDeleteCall<'a, S> {}

impl<'a, S> DriveDeleteCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<hyper::Response<hyper::body::Body>> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.drives.delete",
                               http_method: hyper::Method::DELETE });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(2 + self._additional_params.len());
        params.push(("driveId", self._drive_id.to_string()));
        for &field in ["driveId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "drives/{driveId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{driveId}", "driveId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["driveId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::DELETE).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the shared drive.
    ///
    /// Sets the *drive id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn drive_id(mut self, new_value: &str) -> DriveDeleteCall<'a, S> {
        self._drive_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> DriveDeleteCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> DriveDeleteCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> DriveDeleteCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets a shared drive's metadata by ID.
///
/// A builder for the *get* method supported by a *drive* resource.
/// It is not used directly, but through a `DriveMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.drives().get("driveId")
///              .use_domain_admin_access(true)
///              .doit().await;
/// # }
/// ```
pub struct DriveGetCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _drive_id: String,
    _use_domain_admin_access: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for DriveGetCall<'a, S> {}

impl<'a, S> DriveGetCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Drive)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.drives.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("driveId", self._drive_id.to_string()));
        if let Some(value) = self._use_domain_admin_access {
            params.push(("useDomainAdminAccess", value.to_string()));
        }
        for &field in ["alt", "driveId", "useDomainAdminAccess"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "drives/{driveId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{driveId}", "driveId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["driveId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the shared drive.
    ///
    /// Sets the *drive id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn drive_id(mut self, new_value: &str) -> DriveGetCall<'a, S> {
        self._drive_id = new_value.to_string();
        self
    }
    /// Issue the request as a domain administrator; if set to true, then the requester will be granted access if they are an administrator of the domain to which the shared drive belongs.
    ///
    /// Sets the *use domain admin access* query property to the given value.
    pub fn use_domain_admin_access(mut self, new_value: bool) -> DriveGetCall<'a, S> {
        self._use_domain_admin_access = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> DriveGetCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> DriveGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Readonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> DriveGetCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Hides a shared drive from the default view.
///
/// A builder for the *hide* method supported by a *drive* resource.
/// It is not used directly, but through a `DriveMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.drives().hide("driveId")
///              .doit().await;
/// # }
/// ```
pub struct DriveHideCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _drive_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for DriveHideCall<'a, S> {}

impl<'a, S> DriveHideCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Drive)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.drives.hide",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        params.push(("driveId", self._drive_id.to_string()));
        for &field in ["alt", "driveId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "drives/{driveId}/hide";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{driveId}", "driveId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["driveId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the shared drive.
    ///
    /// Sets the *drive id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn drive_id(mut self, new_value: &str) -> DriveHideCall<'a, S> {
        self._drive_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> DriveHideCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> DriveHideCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> DriveHideCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists the user's shared drives.
///
/// A builder for the *list* method supported by a *drive* resource.
/// It is not used directly, but through a `DriveMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.drives().list()
///              .use_domain_admin_access(false)
///              .q("elitr")
///              .page_token("sed")
///              .page_size(-61)
///              .doit().await;
/// # }
/// ```
pub struct DriveListCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _use_domain_admin_access: Option<bool>,
    _q: Option<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for DriveListCall<'a, S> {}

impl<'a, S> DriveListCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, DriveList)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.drives.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        if let Some(value) = self._use_domain_admin_access {
            params.push(("useDomainAdminAccess", value.to_string()));
        }
        if let Some(value) = self._q {
            params.push(("q", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        for &field in ["alt", "useDomainAdminAccess", "q", "pageToken", "pageSize"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "drives";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_ref().to_string(), ());
        }


        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Issue the request as a domain administrator; if set to true, then all shared drives of the domain in which the requester is an administrator are returned.
    ///
    /// Sets the *use domain admin access* query property to the given value.
    pub fn use_domain_admin_access(mut self, new_value: bool) -> DriveListCall<'a, S> {
        self._use_domain_admin_access = Some(new_value);
        self
    }
    /// Query string for searching shared drives.
    ///
    /// Sets the *q* query property to the given value.
    pub fn q(mut self, new_value: &str) -> DriveListCall<'a, S> {
        self._q = Some(new_value.to_string());
        self
    }
    /// Page token for shared drives.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> DriveListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of shared drives to return per page.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> DriveListCall<'a, S> {
        self._page_size = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> DriveListCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> DriveListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Readonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> DriveListCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Restores a shared drive to the default view.
///
/// A builder for the *unhide* method supported by a *drive* resource.
/// It is not used directly, but through a `DriveMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.drives().unhide("driveId")
///              .doit().await;
/// # }
/// ```
pub struct DriveUnhideCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _drive_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for DriveUnhideCall<'a, S> {}

impl<'a, S> DriveUnhideCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Drive)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.drives.unhide",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        params.push(("driveId", self._drive_id.to_string()));
        for &field in ["alt", "driveId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "drives/{driveId}/unhide";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{driveId}", "driveId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["driveId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the shared drive.
    ///
    /// Sets the *drive id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn drive_id(mut self, new_value: &str) -> DriveUnhideCall<'a, S> {
        self._drive_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> DriveUnhideCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> DriveUnhideCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> DriveUnhideCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates the metadate for a shared drive.
///
/// A builder for the *update* method supported by a *drive* resource.
/// It is not used directly, but through a `DriveMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// use drive3::api::Drive;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Drive::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.drives().update(req, "driveId")
///              .use_domain_admin_access(true)
///              .doit().await;
/// # }
/// ```
pub struct DriveUpdateCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _request: Drive,
    _drive_id: String,
    _use_domain_admin_access: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for DriveUpdateCall<'a, S> {}

impl<'a, S> DriveUpdateCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Drive)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.drives.update",
                               http_method: hyper::Method::PATCH });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("driveId", self._drive_id.to_string()));
        if let Some(value) = self._use_domain_admin_access {
            params.push(("useDomainAdminAccess", value.to_string()));
        }
        for &field in ["alt", "driveId", "useDomainAdminAccess"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "drives/{driveId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{driveId}", "driveId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["driveId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::PATCH).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Drive) -> DriveUpdateCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the shared drive.
    ///
    /// Sets the *drive id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn drive_id(mut self, new_value: &str) -> DriveUpdateCall<'a, S> {
        self._drive_id = new_value.to_string();
        self
    }
    /// Issue the request as a domain administrator; if set to true, then the requester will be granted access if they are an administrator of the domain to which the shared drive belongs.
    ///
    /// Sets the *use domain admin access* query property to the given value.
    pub fn use_domain_admin_access(mut self, new_value: bool) -> DriveUpdateCall<'a, S> {
        self._use_domain_admin_access = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> DriveUpdateCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> DriveUpdateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> DriveUpdateCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a copy of a file and applies any requested updates with patch semantics. Folders cannot be copied.
///
/// A builder for the *copy* method supported by a *file* resource.
/// It is not used directly, but through a `FileMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// use drive3::api::File;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = File::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.files().copy(req, "fileId")
///              .supports_team_drives(true)
///              .supports_all_drives(false)
///              .ocr_language("erat")
///              .keep_revision_forever(false)
///              .include_permissions_for_view("amet")
///              .ignore_default_visibility(true)
///              .enforce_single_parent(false)
///              .doit().await;
/// # }
/// ```
pub struct FileCopyCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _request: File,
    _file_id: String,
    _supports_team_drives: Option<bool>,
    _supports_all_drives: Option<bool>,
    _ocr_language: Option<String>,
    _keep_revision_forever: Option<bool>,
    _include_permissions_for_view: Option<String>,
    _ignore_default_visibility: Option<bool>,
    _enforce_single_parent: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for FileCopyCall<'a, S> {}

impl<'a, S> FileCopyCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, File)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.files.copy",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(11 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        if let Some(value) = self._supports_team_drives {
            params.push(("supportsTeamDrives", value.to_string()));
        }
        if let Some(value) = self._supports_all_drives {
            params.push(("supportsAllDrives", value.to_string()));
        }
        if let Some(value) = self._ocr_language {
            params.push(("ocrLanguage", value.to_string()));
        }
        if let Some(value) = self._keep_revision_forever {
            params.push(("keepRevisionForever", value.to_string()));
        }
        if let Some(value) = self._include_permissions_for_view {
            params.push(("includePermissionsForView", value.to_string()));
        }
        if let Some(value) = self._ignore_default_visibility {
            params.push(("ignoreDefaultVisibility", value.to_string()));
        }
        if let Some(value) = self._enforce_single_parent {
            params.push(("enforceSingleParent", value.to_string()));
        }
        for &field in ["alt", "fileId", "supportsTeamDrives", "supportsAllDrives", "ocrLanguage", "keepRevisionForever", "includePermissionsForView", "ignoreDefaultVisibility", "enforceSingleParent"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "files/{fileId}/copy";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["fileId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: File) -> FileCopyCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> FileCopyCall<'a, S> {
        self._file_id = new_value.to_string();
        self
    }
    /// Deprecated use supportsAllDrives instead.
    ///
    /// Sets the *supports team drives* query property to the given value.
    pub fn supports_team_drives(mut self, new_value: bool) -> FileCopyCall<'a, S> {
        self._supports_team_drives = Some(new_value);
        self
    }
    /// Whether the requesting application supports both My Drives and shared drives.
    ///
    /// Sets the *supports all drives* query property to the given value.
    pub fn supports_all_drives(mut self, new_value: bool) -> FileCopyCall<'a, S> {
        self._supports_all_drives = Some(new_value);
        self
    }
    /// A language hint for OCR processing during image import (ISO 639-1 code).
    ///
    /// Sets the *ocr language* query property to the given value.
    pub fn ocr_language(mut self, new_value: &str) -> FileCopyCall<'a, S> {
        self._ocr_language = Some(new_value.to_string());
        self
    }
    /// Whether to set the 'keepForever' field in the new head revision. This is only applicable to files with binary content in Google Drive. Only 200 revisions for the file can be kept forever. If the limit is reached, try deleting pinned revisions.
    ///
    /// Sets the *keep revision forever* query property to the given value.
    pub fn keep_revision_forever(mut self, new_value: bool) -> FileCopyCall<'a, S> {
        self._keep_revision_forever = Some(new_value);
        self
    }
    /// Specifies which additional view's permissions to include in the response. Only 'published' is supported.
    ///
    /// Sets the *include permissions for view* query property to the given value.
    pub fn include_permissions_for_view(mut self, new_value: &str) -> FileCopyCall<'a, S> {
        self._include_permissions_for_view = Some(new_value.to_string());
        self
    }
    /// Whether to ignore the domain's default visibility settings for the created file. Domain administrators can choose to make all uploaded files visible to the domain by default; this parameter bypasses that behavior for the request. Permissions are still inherited from parent folders.
    ///
    /// Sets the *ignore default visibility* query property to the given value.
    pub fn ignore_default_visibility(mut self, new_value: bool) -> FileCopyCall<'a, S> {
        self._ignore_default_visibility = Some(new_value);
        self
    }
    /// Deprecated. Copying files into multiple folders is no longer supported. Use shortcuts instead.
    ///
    /// Sets the *enforce single parent* query property to the given value.
    pub fn enforce_single_parent(mut self, new_value: bool) -> FileCopyCall<'a, S> {
        self._enforce_single_parent = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> FileCopyCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> FileCopyCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> FileCopyCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a new file.
///
/// A builder for the *create* method supported by a *file* resource.
/// It is not used directly, but through a `FileMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// use drive3::api::File;
/// use std::fs;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = File::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `upload(...)`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.files().create(req)
///              .use_content_as_indexable_text(true)
///              .supports_team_drives(true)
///              .supports_all_drives(false)
///              .ocr_language("elitr")
///              .keep_revision_forever(true)
///              .include_permissions_for_view("est")
///              .ignore_default_visibility(true)
///              .enforce_single_parent(false)
///              .upload(fs::File::open("file.ext").unwrap(), "application/octet-stream".parse().unwrap()).await;
/// # }
/// ```
pub struct FileCreateCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _request: File,
    _use_content_as_indexable_text: Option<bool>,
    _supports_team_drives: Option<bool>,
    _supports_all_drives: Option<bool>,
    _ocr_language: Option<String>,
    _keep_revision_forever: Option<bool>,
    _include_permissions_for_view: Option<String>,
    _ignore_default_visibility: Option<bool>,
    _enforce_single_parent: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for FileCreateCall<'a, S> {}

impl<'a, S> FileCreateCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    async fn doit<RS>(mut self, mut reader: RS, reader_mime_type: mime::Mime, protocol: &'static str) -> client::Result<(hyper::Response<hyper::body::Body>, File)>
		where RS: client::ReadSeek {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.files.create",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(11 + self._additional_params.len());
        if let Some(value) = self._use_content_as_indexable_text {
            params.push(("useContentAsIndexableText", value.to_string()));
        }
        if let Some(value) = self._supports_team_drives {
            params.push(("supportsTeamDrives", value.to_string()));
        }
        if let Some(value) = self._supports_all_drives {
            params.push(("supportsAllDrives", value.to_string()));
        }
        if let Some(value) = self._ocr_language {
            params.push(("ocrLanguage", value.to_string()));
        }
        if let Some(value) = self._keep_revision_forever {
            params.push(("keepRevisionForever", value.to_string()));
        }
        if let Some(value) = self._include_permissions_for_view {
            params.push(("includePermissionsForView", value.to_string()));
        }
        if let Some(value) = self._ignore_default_visibility {
            params.push(("ignoreDefaultVisibility", value.to_string()));
        }
        if let Some(value) = self._enforce_single_parent {
            params.push(("enforceSingleParent", value.to_string()));
        }
        for &field in ["alt", "useContentAsIndexableText", "supportsTeamDrives", "supportsAllDrives", "ocrLanguage", "keepRevisionForever", "includePermissionsForView", "ignoreDefaultVisibility", "enforceSingleParent"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let (mut url, upload_type) =
            if protocol == "resumable" {
                (self.hub._root_url.clone() + "resumable/upload/drive/v3/files", "resumable")
            } else if protocol == "simple" {
                (self.hub._root_url.clone() + "upload/drive/v3/files", "multipart")
            } else {
                unreachable!()
            };
        params.push(("uploadType", upload_type.to_string()));
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }


        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();

        let mut should_ask_dlg_for_url = false;
        let mut upload_url_from_server;
        let mut upload_url: Option<String> = None;

        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                if should_ask_dlg_for_url && (upload_url = dlg.upload_url()) == () && upload_url.is_some() {
                    should_ask_dlg_for_url = false;
                    upload_url_from_server = false;
                    Ok(hyper::Response::builder()
                        .status(hyper::StatusCode::OK)
                        .header("Location", upload_url.as_ref().unwrap().clone())
                        .body(hyper::body::Body::empty())
                        .unwrap())
                } else {
                    let mut mp_reader: client::MultiPartReader = Default::default();
                    let (mut body_reader, content_type) = match protocol {
                        "simple" => {
                            mp_reader.reserve_exact(2);
                            let size = reader.seek(io::SeekFrom::End(0)).unwrap();
                        reader.seek(io::SeekFrom::Start(0)).unwrap();
                        if size > 5497558138880 {
                        	return Err(client::Error::UploadSizeLimitExceeded(size, 5497558138880))
                        }
                            mp_reader.add_part(&mut request_value_reader, request_size, json_mime_type.clone())
                                     .add_part(&mut reader, size, reader_mime_type.clone());
                            let mime_type = mp_reader.mime_type();
                            (&mut mp_reader as &mut (dyn io::Read + Send), (CONTENT_TYPE, mime_type.to_string()))
                        },
                        _ => (&mut request_value_reader as &mut (dyn io::Read + Send), (CONTENT_TYPE, json_mime_type.to_string())),
                    };
                    let client = &self.hub.client;
                    dlg.pre_request();
                    let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                            .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));
    
                    upload_url_from_server = true;
                    if protocol == "resumable" {
                        req_builder = req_builder.header("X-Upload-Content-Type", format!("{}", reader_mime_type));
                    }
    
                            let mut body_reader_bytes = vec![];
                            body_reader.read_to_end(&mut body_reader_bytes).unwrap();
                            let request = req_builder
                            .header(content_type.0, content_type.1.to_string())
                            .body(hyper::body::Body::from(body_reader_bytes));
    
                    client.request(request.unwrap()).await
                    
                }
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    if protocol == "resumable" {
                        let size = reader.seek(io::SeekFrom::End(0)).unwrap();
                        reader.seek(io::SeekFrom::Start(0)).unwrap();
                        if size > 5497558138880 {
                        	return Err(client::Error::UploadSizeLimitExceeded(size, 5497558138880))
                        }
                        let upload_result = {
                            let url_str = &res.headers().get("Location").expect("LOCATION header is part of protocol").to_str().unwrap();
                            if upload_url_from_server {
                                dlg.store_upload_url(Some(url_str));
                            }

                            client::ResumableUploadHelper {
                                client: &self.hub.client,
                                delegate: dlg,
                                start_at: if upload_url_from_server { Some(0) } else { None },
                                auth: &self.hub.auth,
                                user_agent: &self.hub._user_agent,
                                auth_header: format!("Bearer {}", token.as_str()),
                                url: url_str,
                                reader: &mut reader,
                                media_type: reader_mime_type.clone(),
                                content_length: size
                            }.upload().await
                        };
                        match upload_result {
                            None => {
                                dlg.finished(false);
                                return Err(client::Error::Cancelled)
                            }
                            Some(Err(err)) => {
                                dlg.finished(false);
                                return Err(client::Error::HttpError(err))
                            }
                            Some(Ok(upload_result)) => {
                                res = upload_result;
                                if !res.status().is_success() {
                                    dlg.store_upload_url(None);
                                    dlg.finished(false);
                                    return Err(client::Error::Failure(res))
                                }
                            }
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }

    /// Upload media in a resumable fashion.
    /// Even if the upload fails or is interrupted, it can be resumed for a
    /// certain amount of time as the server maintains state temporarily.
    /// 
    /// The delegate will be asked for an `upload_url()`, and if not provided, will be asked to store an upload URL
    /// that was provided by the server, using `store_upload_url(...)`. The upload will be done in chunks, the delegate
    /// may specify the `chunk_size()` and may cancel the operation before each chunk is uploaded, using
    /// `cancel_chunk_upload(...)`.
    ///
    /// * *multipart*: yes
    /// * *max size*: 5120GB
    /// * *valid mime types*: '*/*'
    pub async fn upload_resumable<RS>(self, resumeable_stream: RS, mime_type: mime::Mime) -> client::Result<(hyper::Response<hyper::body::Body>, File)>
                where RS: client::ReadSeek {
        self.doit(resumeable_stream, mime_type, "resumable").await
    }
    /// Upload media all at once.
    /// If the upload fails for whichever reason, all progress is lost.
    ///
    /// * *multipart*: yes
    /// * *max size*: 5120GB
    /// * *valid mime types*: '*/*'
    pub async fn upload<RS>(self, stream: RS, mime_type: mime::Mime) -> client::Result<(hyper::Response<hyper::body::Body>, File)>
                where RS: client::ReadSeek {
        self.doit(stream, mime_type, "simple").await
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: File) -> FileCreateCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Whether to use the uploaded content as indexable text.
    ///
    /// Sets the *use content as indexable text* query property to the given value.
    pub fn use_content_as_indexable_text(mut self, new_value: bool) -> FileCreateCall<'a, S> {
        self._use_content_as_indexable_text = Some(new_value);
        self
    }
    /// Deprecated use supportsAllDrives instead.
    ///
    /// Sets the *supports team drives* query property to the given value.
    pub fn supports_team_drives(mut self, new_value: bool) -> FileCreateCall<'a, S> {
        self._supports_team_drives = Some(new_value);
        self
    }
    /// Whether the requesting application supports both My Drives and shared drives.
    ///
    /// Sets the *supports all drives* query property to the given value.
    pub fn supports_all_drives(mut self, new_value: bool) -> FileCreateCall<'a, S> {
        self._supports_all_drives = Some(new_value);
        self
    }
    /// A language hint for OCR processing during image import (ISO 639-1 code).
    ///
    /// Sets the *ocr language* query property to the given value.
    pub fn ocr_language(mut self, new_value: &str) -> FileCreateCall<'a, S> {
        self._ocr_language = Some(new_value.to_string());
        self
    }
    /// Whether to set the 'keepForever' field in the new head revision. This is only applicable to files with binary content in Google Drive. Only 200 revisions for the file can be kept forever. If the limit is reached, try deleting pinned revisions.
    ///
    /// Sets the *keep revision forever* query property to the given value.
    pub fn keep_revision_forever(mut self, new_value: bool) -> FileCreateCall<'a, S> {
        self._keep_revision_forever = Some(new_value);
        self
    }
    /// Specifies which additional view's permissions to include in the response. Only 'published' is supported.
    ///
    /// Sets the *include permissions for view* query property to the given value.
    pub fn include_permissions_for_view(mut self, new_value: &str) -> FileCreateCall<'a, S> {
        self._include_permissions_for_view = Some(new_value.to_string());
        self
    }
    /// Whether to ignore the domain's default visibility settings for the created file. Domain administrators can choose to make all uploaded files visible to the domain by default; this parameter bypasses that behavior for the request. Permissions are still inherited from parent folders.
    ///
    /// Sets the *ignore default visibility* query property to the given value.
    pub fn ignore_default_visibility(mut self, new_value: bool) -> FileCreateCall<'a, S> {
        self._ignore_default_visibility = Some(new_value);
        self
    }
    /// Deprecated. Creating files in multiple folders is no longer supported.
    ///
    /// Sets the *enforce single parent* query property to the given value.
    pub fn enforce_single_parent(mut self, new_value: bool) -> FileCreateCall<'a, S> {
        self._enforce_single_parent = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> FileCreateCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> FileCreateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> FileCreateCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Permanently deletes a file owned by the user without moving it to the trash. If the file belongs to a shared drive the user must be an organizer on the parent. If the target is a folder, all descendants owned by the user are also deleted.
///
/// A builder for the *delete* method supported by a *file* resource.
/// It is not used directly, but through a `FileMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.files().delete("fileId")
///              .supports_team_drives(true)
///              .supports_all_drives(true)
///              .enforce_single_parent(false)
///              .doit().await;
/// # }
/// ```
pub struct FileDeleteCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _file_id: String,
    _supports_team_drives: Option<bool>,
    _supports_all_drives: Option<bool>,
    _enforce_single_parent: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for FileDeleteCall<'a, S> {}

impl<'a, S> FileDeleteCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<hyper::Response<hyper::body::Body>> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.files.delete",
                               http_method: hyper::Method::DELETE });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        if let Some(value) = self._supports_team_drives {
            params.push(("supportsTeamDrives", value.to_string()));
        }
        if let Some(value) = self._supports_all_drives {
            params.push(("supportsAllDrives", value.to_string()));
        }
        if let Some(value) = self._enforce_single_parent {
            params.push(("enforceSingleParent", value.to_string()));
        }
        for &field in ["fileId", "supportsTeamDrives", "supportsAllDrives", "enforceSingleParent"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "files/{fileId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["fileId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::DELETE).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> FileDeleteCall<'a, S> {
        self._file_id = new_value.to_string();
        self
    }
    /// Deprecated use supportsAllDrives instead.
    ///
    /// Sets the *supports team drives* query property to the given value.
    pub fn supports_team_drives(mut self, new_value: bool) -> FileDeleteCall<'a, S> {
        self._supports_team_drives = Some(new_value);
        self
    }
    /// Whether the requesting application supports both My Drives and shared drives.
    ///
    /// Sets the *supports all drives* query property to the given value.
    pub fn supports_all_drives(mut self, new_value: bool) -> FileDeleteCall<'a, S> {
        self._supports_all_drives = Some(new_value);
        self
    }
    /// Deprecated. If an item is not in a shared drive and its last parent is deleted but the item itself is not, the item will be placed under its owner's root.
    ///
    /// Sets the *enforce single parent* query property to the given value.
    pub fn enforce_single_parent(mut self, new_value: bool) -> FileDeleteCall<'a, S> {
        self._enforce_single_parent = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> FileDeleteCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> FileDeleteCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> FileDeleteCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Permanently deletes all of the user's trashed files.
///
/// A builder for the *emptyTrash* method supported by a *file* resource.
/// It is not used directly, but through a `FileMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.files().empty_trash()
///              .enforce_single_parent(false)
///              .doit().await;
/// # }
/// ```
pub struct FileEmptyTrashCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _enforce_single_parent: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for FileEmptyTrashCall<'a, S> {}

impl<'a, S> FileEmptyTrashCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<hyper::Response<hyper::body::Body>> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.files.emptyTrash",
                               http_method: hyper::Method::DELETE });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(2 + self._additional_params.len());
        if let Some(value) = self._enforce_single_parent {
            params.push(("enforceSingleParent", value.to_string()));
        }
        for &field in ["enforceSingleParent"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "files/trash";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }


        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::DELETE).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Deprecated. If an item is not in a shared drive and its last parent is deleted but the item itself is not, the item will be placed under its owner's root.
    ///
    /// Sets the *enforce single parent* query property to the given value.
    pub fn enforce_single_parent(mut self, new_value: bool) -> FileEmptyTrashCall<'a, S> {
        self._enforce_single_parent = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> FileEmptyTrashCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> FileEmptyTrashCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> FileEmptyTrashCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Exports a Google Workspace document to the requested MIME type and returns exported byte content. Note that the exported content is limited to 10MB.
///
/// This method supports **media download**. To enable it, adjust the builder like this:
/// `.param("alt", "media")`.
///
/// A builder for the *export* method supported by a *file* resource.
/// It is not used directly, but through a `FileMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.files().export("fileId", "mimeType")
///              .doit().await;
/// # }
/// ```
pub struct FileExportCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _file_id: String,
    _mime_type: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for FileExportCall<'a, S> {}

impl<'a, S> FileExportCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<hyper::Response<hyper::body::Body>> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.files.export",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        params.push(("mimeType", self._mime_type.to_string()));
        for &field in ["fileId", "mimeType"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "files/{fileId}/export";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["fileId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> FileExportCall<'a, S> {
        self._file_id = new_value.to_string();
        self
    }
    /// The MIME type of the format requested for this export.
    ///
    /// Sets the *mime type* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn mime_type(mut self, new_value: &str) -> FileExportCall<'a, S> {
        self._mime_type = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> FileExportCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> FileExportCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Readonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> FileExportCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Generates a set of file IDs which can be provided in create or copy requests.
///
/// A builder for the *generateIds* method supported by a *file* resource.
/// It is not used directly, but through a `FileMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.files().generate_ids()
///              .type_("Lorem")
///              .space("accusam")
///              .count(-47)
///              .doit().await;
/// # }
/// ```
pub struct FileGenerateIdCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _type_: Option<String>,
    _space: Option<String>,
    _count: Option<i32>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for FileGenerateIdCall<'a, S> {}

impl<'a, S> FileGenerateIdCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GeneratedIds)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.files.generateIds",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        if let Some(value) = self._type_ {
            params.push(("type", value.to_string()));
        }
        if let Some(value) = self._space {
            params.push(("space", value.to_string()));
        }
        if let Some(value) = self._count {
            params.push(("count", value.to_string()));
        }
        for &field in ["alt", "type", "space", "count"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "files/generateIds";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }


        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The type of items which the IDs can be used for. Supported values are 'files' and 'shortcuts'. Note that 'shortcuts' are only supported in the drive 'space'. (Default: 'files')
    ///
    /// Sets the *type* query property to the given value.
    pub fn type_(mut self, new_value: &str) -> FileGenerateIdCall<'a, S> {
        self._type_ = Some(new_value.to_string());
        self
    }
    /// The space in which the IDs can be used to create new files. Supported values are 'drive' and 'appDataFolder'. (Default: 'drive')
    ///
    /// Sets the *space* query property to the given value.
    pub fn space(mut self, new_value: &str) -> FileGenerateIdCall<'a, S> {
        self._space = Some(new_value.to_string());
        self
    }
    /// The number of IDs to return.
    ///
    /// Sets the *count* query property to the given value.
    pub fn count(mut self, new_value: i32) -> FileGenerateIdCall<'a, S> {
        self._count = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> FileGenerateIdCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> FileGenerateIdCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> FileGenerateIdCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets a file's metadata or content by ID.
///
/// This method supports **media download**. To enable it, adjust the builder like this:
/// `.param("alt", "media")`.
/// Please note that due to missing multi-part support on the server side, you will only receive the media,
/// but not the `File` structure that you would usually get. The latter will be a default value.
///
/// A builder for the *get* method supported by a *file* resource.
/// It is not used directly, but through a `FileMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.files().get("fileId")
///              .supports_team_drives(true)
///              .supports_all_drives(false)
///              .include_permissions_for_view("accusam")
///              .acknowledge_abuse(true)
///              .doit().await;
/// # }
/// ```
pub struct FileGetCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _file_id: String,
    _supports_team_drives: Option<bool>,
    _supports_all_drives: Option<bool>,
    _include_permissions_for_view: Option<String>,
    _acknowledge_abuse: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for FileGetCall<'a, S> {}

impl<'a, S> FileGetCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, File)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.files.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        if let Some(value) = self._supports_team_drives {
            params.push(("supportsTeamDrives", value.to_string()));
        }
        if let Some(value) = self._supports_all_drives {
            params.push(("supportsAllDrives", value.to_string()));
        }
        if let Some(value) = self._include_permissions_for_view {
            params.push(("includePermissionsForView", value.to_string()));
        }
        if let Some(value) = self._acknowledge_abuse {
            params.push(("acknowledgeAbuse", value.to_string()));
        }
        for &field in ["fileId", "supportsTeamDrives", "supportsAllDrives", "includePermissionsForView", "acknowledgeAbuse"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let (json_field_missing, enable_resource_parsing) = {
            let mut enable = true;
            let mut field_present = true;
            for &(name, ref value) in params.iter() {
                if name == "alt" {
                    field_present = false;
                    if <String as AsRef<str>>::as_ref(&value) != "json" {
                        enable = false;
                    }
                    break;
                }
            }
            (field_present, enable)
        };
        if json_field_missing {
            params.push(("alt", "json".to_string()));
        }

        let mut url = self.hub._base_url.clone() + "files/{fileId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::MetadataReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["fileId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = if enable_resource_parsing {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    } else { (res, Default::default()) };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> FileGetCall<'a, S> {
        self._file_id = new_value.to_string();
        self
    }
    /// Deprecated use supportsAllDrives instead.
    ///
    /// Sets the *supports team drives* query property to the given value.
    pub fn supports_team_drives(mut self, new_value: bool) -> FileGetCall<'a, S> {
        self._supports_team_drives = Some(new_value);
        self
    }
    /// Whether the requesting application supports both My Drives and shared drives.
    ///
    /// Sets the *supports all drives* query property to the given value.
    pub fn supports_all_drives(mut self, new_value: bool) -> FileGetCall<'a, S> {
        self._supports_all_drives = Some(new_value);
        self
    }
    /// Specifies which additional view's permissions to include in the response. Only 'published' is supported.
    ///
    /// Sets the *include permissions for view* query property to the given value.
    pub fn include_permissions_for_view(mut self, new_value: &str) -> FileGetCall<'a, S> {
        self._include_permissions_for_view = Some(new_value.to_string());
        self
    }
    /// Whether the user is acknowledging the risk of downloading known malware or other abusive files. This is only applicable when alt=media.
    ///
    /// Sets the *acknowledge abuse* query property to the given value.
    pub fn acknowledge_abuse(mut self, new_value: bool) -> FileGetCall<'a, S> {
        self._acknowledge_abuse = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> FileGetCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> FileGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::MetadataReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> FileGetCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists or searches files.
///
/// A builder for the *list* method supported by a *file* resource.
/// It is not used directly, but through a `FileMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.files().list()
///              .team_drive_id("Lorem")
///              .supports_team_drives(false)
///              .supports_all_drives(true)
///              .spaces("erat")
///              .q("sea")
///              .page_token("nonumy")
///              .page_size(-22)
///              .order_by("gubergren")
///              .include_team_drive_items(true)
///              .include_permissions_for_view("consetetur")
///              .include_items_from_all_drives(false)
///              .drive_id("aliquyam")
///              .corpus("eos")
///              .corpora("At")
///              .doit().await;
/// # }
/// ```
pub struct FileListCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _team_drive_id: Option<String>,
    _supports_team_drives: Option<bool>,
    _supports_all_drives: Option<bool>,
    _spaces: Option<String>,
    _q: Option<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _order_by: Option<String>,
    _include_team_drive_items: Option<bool>,
    _include_permissions_for_view: Option<String>,
    _include_items_from_all_drives: Option<bool>,
    _drive_id: Option<String>,
    _corpus: Option<String>,
    _corpora: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for FileListCall<'a, S> {}

impl<'a, S> FileListCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, FileList)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.files.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(16 + self._additional_params.len());
        if let Some(value) = self._team_drive_id {
            params.push(("teamDriveId", value.to_string()));
        }
        if let Some(value) = self._supports_team_drives {
            params.push(("supportsTeamDrives", value.to_string()));
        }
        if let Some(value) = self._supports_all_drives {
            params.push(("supportsAllDrives", value.to_string()));
        }
        if let Some(value) = self._spaces {
            params.push(("spaces", value.to_string()));
        }
        if let Some(value) = self._q {
            params.push(("q", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._order_by {
            params.push(("orderBy", value.to_string()));
        }
        if let Some(value) = self._include_team_drive_items {
            params.push(("includeTeamDriveItems", value.to_string()));
        }
        if let Some(value) = self._include_permissions_for_view {
            params.push(("includePermissionsForView", value.to_string()));
        }
        if let Some(value) = self._include_items_from_all_drives {
            params.push(("includeItemsFromAllDrives", value.to_string()));
        }
        if let Some(value) = self._drive_id {
            params.push(("driveId", value.to_string()));
        }
        if let Some(value) = self._corpus {
            params.push(("corpus", value.to_string()));
        }
        if let Some(value) = self._corpora {
            params.push(("corpora", value.to_string()));
        }
        for &field in ["alt", "teamDriveId", "supportsTeamDrives", "supportsAllDrives", "spaces", "q", "pageToken", "pageSize", "orderBy", "includeTeamDriveItems", "includePermissionsForView", "includeItemsFromAllDrives", "driveId", "corpus", "corpora"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "files";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::MetadataReadonly.as_ref().to_string(), ());
        }


        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Deprecated use driveId instead.
    ///
    /// Sets the *team drive id* query property to the given value.
    pub fn team_drive_id(mut self, new_value: &str) -> FileListCall<'a, S> {
        self._team_drive_id = Some(new_value.to_string());
        self
    }
    /// Deprecated use supportsAllDrives instead.
    ///
    /// Sets the *supports team drives* query property to the given value.
    pub fn supports_team_drives(mut self, new_value: bool) -> FileListCall<'a, S> {
        self._supports_team_drives = Some(new_value);
        self
    }
    /// Whether the requesting application supports both My Drives and shared drives.
    ///
    /// Sets the *supports all drives* query property to the given value.
    pub fn supports_all_drives(mut self, new_value: bool) -> FileListCall<'a, S> {
        self._supports_all_drives = Some(new_value);
        self
    }
    /// A comma-separated list of spaces to query within the corpus. Supported values are 'drive' and 'appDataFolder'.
    ///
    /// Sets the *spaces* query property to the given value.
    pub fn spaces(mut self, new_value: &str) -> FileListCall<'a, S> {
        self._spaces = Some(new_value.to_string());
        self
    }
    /// A query for filtering the file results. See the "Search for Files" guide for supported syntax.
    ///
    /// Sets the *q* query property to the given value.
    pub fn q(mut self, new_value: &str) -> FileListCall<'a, S> {
        self._q = Some(new_value.to_string());
        self
    }
    /// The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> FileListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of files to return per page. Partial or empty result pages are possible even before the end of the files list has been reached.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> FileListCall<'a, S> {
        self._page_size = Some(new_value);
        self
    }
    /// A comma-separated list of sort keys. Valid keys are 'createdTime', 'folder', 'modifiedByMeTime', 'modifiedTime', 'name', 'name_natural', 'quotaBytesUsed', 'recency', 'sharedWithMeTime', 'starred', and 'viewedByMeTime'. Each key sorts ascending by default, but may be reversed with the 'desc' modifier. Example usage: ?orderBy=folder,modifiedTime desc,name. Please note that there is a current limitation for users with approximately one million files in which the requested sort order is ignored.
    ///
    /// Sets the *order by* query property to the given value.
    pub fn order_by(mut self, new_value: &str) -> FileListCall<'a, S> {
        self._order_by = Some(new_value.to_string());
        self
    }
    /// Deprecated use includeItemsFromAllDrives instead.
    ///
    /// Sets the *include team drive items* query property to the given value.
    pub fn include_team_drive_items(mut self, new_value: bool) -> FileListCall<'a, S> {
        self._include_team_drive_items = Some(new_value);
        self
    }
    /// Specifies which additional view's permissions to include in the response. Only 'published' is supported.
    ///
    /// Sets the *include permissions for view* query property to the given value.
    pub fn include_permissions_for_view(mut self, new_value: &str) -> FileListCall<'a, S> {
        self._include_permissions_for_view = Some(new_value.to_string());
        self
    }
    /// Whether both My Drive and shared drive items should be included in results.
    ///
    /// Sets the *include items from all drives* query property to the given value.
    pub fn include_items_from_all_drives(mut self, new_value: bool) -> FileListCall<'a, S> {
        self._include_items_from_all_drives = Some(new_value);
        self
    }
    /// ID of the shared drive to search.
    ///
    /// Sets the *drive id* query property to the given value.
    pub fn drive_id(mut self, new_value: &str) -> FileListCall<'a, S> {
        self._drive_id = Some(new_value.to_string());
        self
    }
    /// The source of files to list. Deprecated: use 'corpora' instead.
    ///
    /// Sets the *corpus* query property to the given value.
    pub fn corpus(mut self, new_value: &str) -> FileListCall<'a, S> {
        self._corpus = Some(new_value.to_string());
        self
    }
    /// Groupings of files to which the query applies. Supported groupings are: 'user' (files created by, opened by, or shared directly with the user), 'drive' (files in the specified shared drive as indicated by the 'driveId'), 'domain' (files shared to the user's domain), and 'allDrives' (A combination of 'user' and 'drive' for all drives where the user is a member). When able, use 'user' or 'drive', instead of 'allDrives', for efficiency.
    ///
    /// Sets the *corpora* query property to the given value.
    pub fn corpora(mut self, new_value: &str) -> FileListCall<'a, S> {
        self._corpora = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> FileListCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> FileListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::MetadataReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> FileListCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates a file's metadata and/or content. When calling this method, only populate fields in the request that you want to modify. When updating fields, some fields might change automatically, such as modifiedDate. This method supports patch semantics.
///
/// A builder for the *update* method supported by a *file* resource.
/// It is not used directly, but through a `FileMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// use drive3::api::File;
/// use std::fs;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = File::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `upload(...)`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.files().update(req, "fileId")
///              .use_content_as_indexable_text(true)
///              .supports_team_drives(true)
///              .supports_all_drives(true)
///              .remove_parents("amet.")
///              .ocr_language("ipsum")
///              .keep_revision_forever(true)
///              .include_permissions_for_view("accusam")
///              .enforce_single_parent(true)
///              .add_parents("sadipscing")
///              .upload(fs::File::open("file.ext").unwrap(), "application/octet-stream".parse().unwrap()).await;
/// # }
/// ```
pub struct FileUpdateCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _request: File,
    _file_id: String,
    _use_content_as_indexable_text: Option<bool>,
    _supports_team_drives: Option<bool>,
    _supports_all_drives: Option<bool>,
    _remove_parents: Option<String>,
    _ocr_language: Option<String>,
    _keep_revision_forever: Option<bool>,
    _include_permissions_for_view: Option<String>,
    _enforce_single_parent: Option<bool>,
    _add_parents: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for FileUpdateCall<'a, S> {}

impl<'a, S> FileUpdateCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{

    /// Perform the operation you have build so far, but without uploading. This is used to e.g. renaming or updating the description for a file
    pub async fn doit_without_upload(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, File)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.files.update",
                               http_method: hyper::Method::PATCH });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(13 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        if let Some(value) = self._use_content_as_indexable_text {
            params.push(("useContentAsIndexableText", value.to_string()));
        }
        if let Some(value) = self._supports_team_drives {
            params.push(("supportsTeamDrives", value.to_string()));
        }
        if let Some(value) = self._supports_all_drives {
            params.push(("supportsAllDrives", value.to_string()));
        }
        if let Some(value) = self._remove_parents {
            params.push(("removeParents", value.to_string()));
        }
        if let Some(value) = self._ocr_language {
            params.push(("ocrLanguage", value.to_string()));
        }
        if let Some(value) = self._keep_revision_forever {
            params.push(("keepRevisionForever", value.to_string()));
        }
        if let Some(value) = self._include_permissions_for_view {
            params.push(("includePermissionsForView", value.to_string()));
        }
        if let Some(value) = self._enforce_single_parent {
            params.push(("enforceSingleParent", value.to_string()));
        }
        if let Some(value) = self._add_parents {
            params.push(("addParents", value.to_string()));
        }
        for &field in ["alt", "fileId", "useContentAsIndexableText", "supportsTeamDrives", "supportsAllDrives", "removeParents", "ocrLanguage", "keepRevisionForever", "includePermissionsForView", "enforceSingleParent", "addParents"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "files/{fileId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["fileId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::PATCH).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }



    /// Perform the operation you have build so far.
    async fn doit<RS>(mut self, mut reader: RS, reader_mime_type: mime::Mime, protocol: &'static str) -> client::Result<(hyper::Response<hyper::body::Body>, File)>
		where RS: client::ReadSeek {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.files.update",
                               http_method: hyper::Method::PATCH });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(13 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        if let Some(value) = self._use_content_as_indexable_text {
            params.push(("useContentAsIndexableText", value.to_string()));
        }
        if let Some(value) = self._supports_team_drives {
            params.push(("supportsTeamDrives", value.to_string()));
        }
        if let Some(value) = self._supports_all_drives {
            params.push(("supportsAllDrives", value.to_string()));
        }
        if let Some(value) = self._remove_parents {
            params.push(("removeParents", value.to_string()));
        }
        if let Some(value) = self._ocr_language {
            params.push(("ocrLanguage", value.to_string()));
        }
        if let Some(value) = self._keep_revision_forever {
            params.push(("keepRevisionForever", value.to_string()));
        }
        if let Some(value) = self._include_permissions_for_view {
            params.push(("includePermissionsForView", value.to_string()));
        }
        if let Some(value) = self._enforce_single_parent {
            params.push(("enforceSingleParent", value.to_string()));
        }
        if let Some(value) = self._add_parents {
            params.push(("addParents", value.to_string()));
        }
        for &field in ["alt", "fileId", "useContentAsIndexableText", "supportsTeamDrives", "supportsAllDrives", "removeParents", "ocrLanguage", "keepRevisionForever", "includePermissionsForView", "enforceSingleParent", "addParents"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let (mut url, upload_type) =
            if protocol == "resumable" {
                (self.hub._root_url.clone() + "resumable/upload/drive/v3/files/{fileId}", "resumable")
            } else if protocol == "simple" {
                (self.hub._root_url.clone() + "upload/drive/v3/files/{fileId}", "multipart")
            } else {
                unreachable!()
            };
        params.push(("uploadType", upload_type.to_string()));
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["fileId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();

        let mut should_ask_dlg_for_url = false;
        let mut upload_url_from_server;
        let mut upload_url: Option<String> = None;

        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                if should_ask_dlg_for_url && (upload_url = dlg.upload_url()) == () && upload_url.is_some() {
                    should_ask_dlg_for_url = false;
                    upload_url_from_server = false;
                    Ok(hyper::Response::builder()
                        .status(hyper::StatusCode::OK)
                        .header("Location", upload_url.as_ref().unwrap().clone())
                        .body(hyper::body::Body::empty())
                        .unwrap())
                } else {
                    let mut mp_reader: client::MultiPartReader = Default::default();
                    let (mut body_reader, content_type) = match protocol {
                        "simple" => {
                            mp_reader.reserve_exact(2);
                            let size = reader.seek(io::SeekFrom::End(0)).unwrap();
                        reader.seek(io::SeekFrom::Start(0)).unwrap();
                        if size > 5497558138880 {
                        	return Err(client::Error::UploadSizeLimitExceeded(size, 5497558138880))
                        }
                            mp_reader.add_part(&mut request_value_reader, request_size, json_mime_type.clone())
                                     .add_part(&mut reader, size, reader_mime_type.clone());
                            let mime_type = mp_reader.mime_type();
                            (&mut mp_reader as &mut (dyn io::Read + Send), (CONTENT_TYPE, mime_type.to_string()))
                        },
                        _ => (&mut request_value_reader as &mut (dyn io::Read + Send), (CONTENT_TYPE, json_mime_type.to_string())),
                    };
                    let client = &self.hub.client;
                    dlg.pre_request();
                    let mut req_builder = hyper::Request::builder().method(hyper::Method::PATCH).uri(url.clone().into_string())
                            .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));
    
                    upload_url_from_server = true;
                    if protocol == "resumable" {
                        req_builder = req_builder.header("X-Upload-Content-Type", format!("{}", reader_mime_type));
                    }
    
                            let mut body_reader_bytes = vec![];
                            body_reader.read_to_end(&mut body_reader_bytes).unwrap();
                            let request = req_builder
                            .header(content_type.0, content_type.1.to_string())
                            .body(hyper::body::Body::from(body_reader_bytes));
    
                    client.request(request.unwrap()).await
                    
                }
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    if protocol == "resumable" {
                        let size = reader.seek(io::SeekFrom::End(0)).unwrap();
                        reader.seek(io::SeekFrom::Start(0)).unwrap();
                        if size > 5497558138880 {
                        	return Err(client::Error::UploadSizeLimitExceeded(size, 5497558138880))
                        }
                        let upload_result = {
                            let url_str = &res.headers().get("Location").expect("LOCATION header is part of protocol").to_str().unwrap();
                            if upload_url_from_server {
                                dlg.store_upload_url(Some(url_str));
                            }

                            client::ResumableUploadHelper {
                                client: &self.hub.client,
                                delegate: dlg,
                                start_at: if upload_url_from_server { Some(0) } else { None },
                                auth: &self.hub.auth,
                                user_agent: &self.hub._user_agent,
                                auth_header: format!("Bearer {}", token.as_str()),
                                url: url_str,
                                reader: &mut reader,
                                media_type: reader_mime_type.clone(),
                                content_length: size
                            }.upload().await
                        };
                        match upload_result {
                            None => {
                                dlg.finished(false);
                                return Err(client::Error::Cancelled)
                            }
                            Some(Err(err)) => {
                                dlg.finished(false);
                                return Err(client::Error::HttpError(err))
                            }
                            Some(Ok(upload_result)) => {
                                res = upload_result;
                                if !res.status().is_success() {
                                    dlg.store_upload_url(None);
                                    dlg.finished(false);
                                    return Err(client::Error::Failure(res))
                                }
                            }
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }

    /// Upload media in a resumable fashion.
    /// Even if the upload fails or is interrupted, it can be resumed for a
    /// certain amount of time as the server maintains state temporarily.
    /// 
    /// The delegate will be asked for an `upload_url()`, and if not provided, will be asked to store an upload URL
    /// that was provided by the server, using `store_upload_url(...)`. The upload will be done in chunks, the delegate
    /// may specify the `chunk_size()` and may cancel the operation before each chunk is uploaded, using
    /// `cancel_chunk_upload(...)`.
    ///
    /// * *multipart*: yes
    /// * *max size*: 5120GB
    /// * *valid mime types*: '*/*'
    pub async fn upload_resumable<RS>(self, resumeable_stream: RS, mime_type: mime::Mime) -> client::Result<(hyper::Response<hyper::body::Body>, File)>
                where RS: client::ReadSeek {
        self.doit(resumeable_stream, mime_type, "resumable").await
    }
    /// Upload media all at once.
    /// If the upload fails for whichever reason, all progress is lost.
    ///
    /// * *multipart*: yes
    /// * *max size*: 5120GB
    /// * *valid mime types*: '*/*'
    pub async fn upload<RS>(self, stream: RS, mime_type: mime::Mime) -> client::Result<(hyper::Response<hyper::body::Body>, File)>
                where RS: client::ReadSeek {
        self.doit(stream, mime_type, "simple").await
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: File) -> FileUpdateCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> FileUpdateCall<'a, S> {
        self._file_id = new_value.to_string();
        self
    }
    /// Whether to use the uploaded content as indexable text.
    ///
    /// Sets the *use content as indexable text* query property to the given value.
    pub fn use_content_as_indexable_text(mut self, new_value: bool) -> FileUpdateCall<'a, S> {
        self._use_content_as_indexable_text = Some(new_value);
        self
    }
    /// Deprecated use supportsAllDrives instead.
    ///
    /// Sets the *supports team drives* query property to the given value.
    pub fn supports_team_drives(mut self, new_value: bool) -> FileUpdateCall<'a, S> {
        self._supports_team_drives = Some(new_value);
        self
    }
    /// Whether the requesting application supports both My Drives and shared drives.
    ///
    /// Sets the *supports all drives* query property to the given value.
    pub fn supports_all_drives(mut self, new_value: bool) -> FileUpdateCall<'a, S> {
        self._supports_all_drives = Some(new_value);
        self
    }
    /// A comma-separated list of parent IDs to remove.
    ///
    /// Sets the *remove parents* query property to the given value.
    pub fn remove_parents(mut self, new_value: &str) -> FileUpdateCall<'a, S> {
        self._remove_parents = Some(new_value.to_string());
        self
    }
    /// A language hint for OCR processing during image import (ISO 639-1 code).
    ///
    /// Sets the *ocr language* query property to the given value.
    pub fn ocr_language(mut self, new_value: &str) -> FileUpdateCall<'a, S> {
        self._ocr_language = Some(new_value.to_string());
        self
    }
    /// Whether to set the 'keepForever' field in the new head revision. This is only applicable to files with binary content in Google Drive. Only 200 revisions for the file can be kept forever. If the limit is reached, try deleting pinned revisions.
    ///
    /// Sets the *keep revision forever* query property to the given value.
    pub fn keep_revision_forever(mut self, new_value: bool) -> FileUpdateCall<'a, S> {
        self._keep_revision_forever = Some(new_value);
        self
    }
    /// Specifies which additional view's permissions to include in the response. Only 'published' is supported.
    ///
    /// Sets the *include permissions for view* query property to the given value.
    pub fn include_permissions_for_view(mut self, new_value: &str) -> FileUpdateCall<'a, S> {
        self._include_permissions_for_view = Some(new_value.to_string());
        self
    }
    /// Deprecated. Adding files to multiple folders is no longer supported. Use shortcuts instead.
    ///
    /// Sets the *enforce single parent* query property to the given value.
    pub fn enforce_single_parent(mut self, new_value: bool) -> FileUpdateCall<'a, S> {
        self._enforce_single_parent = Some(new_value);
        self
    }
    /// A comma-separated list of parent IDs to add.
    ///
    /// Sets the *add parents* query property to the given value.
    pub fn add_parents(mut self, new_value: &str) -> FileUpdateCall<'a, S> {
        self._add_parents = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> FileUpdateCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> FileUpdateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> FileUpdateCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Subscribes to changes to a file. While you can establish a channel forchanges to a file on a shared drive, a change to a shared drive file won't create a notification.
///
/// This method supports **media download**. To enable it, adjust the builder like this:
/// `.param("alt", "media")`.
/// Please note that due to missing multi-part support on the server side, you will only receive the media,
/// but not the `Channel` structure that you would usually get. The latter will be a default value.
///
/// A builder for the *watch* method supported by a *file* resource.
/// It is not used directly, but through a `FileMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// use drive3::api::Channel;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Channel::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.files().watch(req, "fileId")
///              .supports_team_drives(true)
///              .supports_all_drives(true)
///              .include_permissions_for_view("magna")
///              .acknowledge_abuse(true)
///              .doit().await;
/// # }
/// ```
pub struct FileWatchCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _request: Channel,
    _file_id: String,
    _supports_team_drives: Option<bool>,
    _supports_all_drives: Option<bool>,
    _include_permissions_for_view: Option<String>,
    _acknowledge_abuse: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for FileWatchCall<'a, S> {}

impl<'a, S> FileWatchCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Channel)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.files.watch",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        if let Some(value) = self._supports_team_drives {
            params.push(("supportsTeamDrives", value.to_string()));
        }
        if let Some(value) = self._supports_all_drives {
            params.push(("supportsAllDrives", value.to_string()));
        }
        if let Some(value) = self._include_permissions_for_view {
            params.push(("includePermissionsForView", value.to_string()));
        }
        if let Some(value) = self._acknowledge_abuse {
            params.push(("acknowledgeAbuse", value.to_string()));
        }
        for &field in ["fileId", "supportsTeamDrives", "supportsAllDrives", "includePermissionsForView", "acknowledgeAbuse"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let (json_field_missing, enable_resource_parsing) = {
            let mut enable = true;
            let mut field_present = true;
            for &(name, ref value) in params.iter() {
                if name == "alt" {
                    field_present = false;
                    if <String as AsRef<str>>::as_ref(&value) != "json" {
                        enable = false;
                    }
                    break;
                }
            }
            (field_present, enable)
        };
        if json_field_missing {
            params.push(("alt", "json".to_string()));
        }

        let mut url = self.hub._base_url.clone() + "files/{fileId}/watch";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["fileId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = if enable_resource_parsing {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    } else { (res, Default::default()) };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Channel) -> FileWatchCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> FileWatchCall<'a, S> {
        self._file_id = new_value.to_string();
        self
    }
    /// Deprecated use supportsAllDrives instead.
    ///
    /// Sets the *supports team drives* query property to the given value.
    pub fn supports_team_drives(mut self, new_value: bool) -> FileWatchCall<'a, S> {
        self._supports_team_drives = Some(new_value);
        self
    }
    /// Whether the requesting application supports both My Drives and shared drives.
    ///
    /// Sets the *supports all drives* query property to the given value.
    pub fn supports_all_drives(mut self, new_value: bool) -> FileWatchCall<'a, S> {
        self._supports_all_drives = Some(new_value);
        self
    }
    /// Specifies which additional view's permissions to include in the response. Only 'published' is supported.
    ///
    /// Sets the *include permissions for view* query property to the given value.
    pub fn include_permissions_for_view(mut self, new_value: &str) -> FileWatchCall<'a, S> {
        self._include_permissions_for_view = Some(new_value.to_string());
        self
    }
    /// Whether the user is acknowledging the risk of downloading known malware or other abusive files. This is only applicable when alt=media.
    ///
    /// Sets the *acknowledge abuse* query property to the given value.
    pub fn acknowledge_abuse(mut self, new_value: bool) -> FileWatchCall<'a, S> {
        self._acknowledge_abuse = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> FileWatchCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> FileWatchCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> FileWatchCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a permission for a file or shared drive.
///
/// A builder for the *create* method supported by a *permission* resource.
/// It is not used directly, but through a `PermissionMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// use drive3::api::Permission;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Permission::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.permissions().create(req, "fileId")
///              .use_domain_admin_access(false)
///              .transfer_ownership(true)
///              .supports_team_drives(false)
///              .supports_all_drives(true)
///              .send_notification_email(false)
///              .move_to_new_owners_root(true)
///              .enforce_single_parent(false)
///              .email_message("rebum.")
///              .doit().await;
/// # }
/// ```
pub struct PermissionCreateCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _request: Permission,
    _file_id: String,
    _use_domain_admin_access: Option<bool>,
    _transfer_ownership: Option<bool>,
    _supports_team_drives: Option<bool>,
    _supports_all_drives: Option<bool>,
    _send_notification_email: Option<bool>,
    _move_to_new_owners_root: Option<bool>,
    _enforce_single_parent: Option<bool>,
    _email_message: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for PermissionCreateCall<'a, S> {}

impl<'a, S> PermissionCreateCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Permission)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.permissions.create",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(12 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        if let Some(value) = self._use_domain_admin_access {
            params.push(("useDomainAdminAccess", value.to_string()));
        }
        if let Some(value) = self._transfer_ownership {
            params.push(("transferOwnership", value.to_string()));
        }
        if let Some(value) = self._supports_team_drives {
            params.push(("supportsTeamDrives", value.to_string()));
        }
        if let Some(value) = self._supports_all_drives {
            params.push(("supportsAllDrives", value.to_string()));
        }
        if let Some(value) = self._send_notification_email {
            params.push(("sendNotificationEmail", value.to_string()));
        }
        if let Some(value) = self._move_to_new_owners_root {
            params.push(("moveToNewOwnersRoot", value.to_string()));
        }
        if let Some(value) = self._enforce_single_parent {
            params.push(("enforceSingleParent", value.to_string()));
        }
        if let Some(value) = self._email_message {
            params.push(("emailMessage", value.to_string()));
        }
        for &field in ["alt", "fileId", "useDomainAdminAccess", "transferOwnership", "supportsTeamDrives", "supportsAllDrives", "sendNotificationEmail", "moveToNewOwnersRoot", "enforceSingleParent", "emailMessage"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "files/{fileId}/permissions";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["fileId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Permission) -> PermissionCreateCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the file or shared drive.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> PermissionCreateCall<'a, S> {
        self._file_id = new_value.to_string();
        self
    }
    /// Issue the request as a domain administrator; if set to true, then the requester will be granted access if the file ID parameter refers to a shared drive and the requester is an administrator of the domain to which the shared drive belongs.
    ///
    /// Sets the *use domain admin access* query property to the given value.
    pub fn use_domain_admin_access(mut self, new_value: bool) -> PermissionCreateCall<'a, S> {
        self._use_domain_admin_access = Some(new_value);
        self
    }
    /// Whether to transfer ownership to the specified user and downgrade the current owner to a writer. This parameter is required as an acknowledgement of the side effect. File owners can only transfer ownership of files existing on My Drive. Files existing in a shared drive are owned by the organization that owns that shared drive. Ownership transfers are not supported for files and folders in shared drives. Organizers of a shared drive can move items from that shared drive into their My Drive which transfers the ownership to them.
    ///
    /// Sets the *transfer ownership* query property to the given value.
    pub fn transfer_ownership(mut self, new_value: bool) -> PermissionCreateCall<'a, S> {
        self._transfer_ownership = Some(new_value);
        self
    }
    /// Deprecated use supportsAllDrives instead.
    ///
    /// Sets the *supports team drives* query property to the given value.
    pub fn supports_team_drives(mut self, new_value: bool) -> PermissionCreateCall<'a, S> {
        self._supports_team_drives = Some(new_value);
        self
    }
    /// Whether the requesting application supports both My Drives and shared drives.
    ///
    /// Sets the *supports all drives* query property to the given value.
    pub fn supports_all_drives(mut self, new_value: bool) -> PermissionCreateCall<'a, S> {
        self._supports_all_drives = Some(new_value);
        self
    }
    /// Whether to send a notification email when sharing to users or groups. This defaults to true for users and groups, and is not allowed for other requests. It must not be disabled for ownership transfers.
    ///
    /// Sets the *send notification email* query property to the given value.
    pub fn send_notification_email(mut self, new_value: bool) -> PermissionCreateCall<'a, S> {
        self._send_notification_email = Some(new_value);
        self
    }
    /// This parameter will only take effect if the item is not in a shared drive and the request is attempting to transfer the ownership of the item. If set to true, the item will be moved to the new owner's My Drive root folder and all prior parents removed. If set to false, parents are not changed.
    ///
    /// Sets the *move to new owners root* query property to the given value.
    pub fn move_to_new_owners_root(mut self, new_value: bool) -> PermissionCreateCall<'a, S> {
        self._move_to_new_owners_root = Some(new_value);
        self
    }
    /// Deprecated. See moveToNewOwnersRoot for details.
    ///
    /// Sets the *enforce single parent* query property to the given value.
    pub fn enforce_single_parent(mut self, new_value: bool) -> PermissionCreateCall<'a, S> {
        self._enforce_single_parent = Some(new_value);
        self
    }
    /// A plain text custom message to include in the notification email.
    ///
    /// Sets the *email message* query property to the given value.
    pub fn email_message(mut self, new_value: &str) -> PermissionCreateCall<'a, S> {
        self._email_message = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> PermissionCreateCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> PermissionCreateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> PermissionCreateCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deletes a permission.
///
/// A builder for the *delete* method supported by a *permission* resource.
/// It is not used directly, but through a `PermissionMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.permissions().delete("fileId", "permissionId")
///              .use_domain_admin_access(true)
///              .supports_team_drives(false)
///              .supports_all_drives(false)
///              .doit().await;
/// # }
/// ```
pub struct PermissionDeleteCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _file_id: String,
    _permission_id: String,
    _use_domain_admin_access: Option<bool>,
    _supports_team_drives: Option<bool>,
    _supports_all_drives: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for PermissionDeleteCall<'a, S> {}

impl<'a, S> PermissionDeleteCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<hyper::Response<hyper::body::Body>> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.permissions.delete",
                               http_method: hyper::Method::DELETE });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        params.push(("permissionId", self._permission_id.to_string()));
        if let Some(value) = self._use_domain_admin_access {
            params.push(("useDomainAdminAccess", value.to_string()));
        }
        if let Some(value) = self._supports_team_drives {
            params.push(("supportsTeamDrives", value.to_string()));
        }
        if let Some(value) = self._supports_all_drives {
            params.push(("supportsAllDrives", value.to_string()));
        }
        for &field in ["fileId", "permissionId", "useDomainAdminAccess", "supportsTeamDrives", "supportsAllDrives"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "files/{fileId}/permissions/{permissionId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId"), ("{permissionId}", "permissionId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["permissionId", "fileId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::DELETE).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the file or shared drive.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> PermissionDeleteCall<'a, S> {
        self._file_id = new_value.to_string();
        self
    }
    /// The ID of the permission.
    ///
    /// Sets the *permission id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn permission_id(mut self, new_value: &str) -> PermissionDeleteCall<'a, S> {
        self._permission_id = new_value.to_string();
        self
    }
    /// Issue the request as a domain administrator; if set to true, then the requester will be granted access if the file ID parameter refers to a shared drive and the requester is an administrator of the domain to which the shared drive belongs.
    ///
    /// Sets the *use domain admin access* query property to the given value.
    pub fn use_domain_admin_access(mut self, new_value: bool) -> PermissionDeleteCall<'a, S> {
        self._use_domain_admin_access = Some(new_value);
        self
    }
    /// Deprecated use supportsAllDrives instead.
    ///
    /// Sets the *supports team drives* query property to the given value.
    pub fn supports_team_drives(mut self, new_value: bool) -> PermissionDeleteCall<'a, S> {
        self._supports_team_drives = Some(new_value);
        self
    }
    /// Whether the requesting application supports both My Drives and shared drives.
    ///
    /// Sets the *supports all drives* query property to the given value.
    pub fn supports_all_drives(mut self, new_value: bool) -> PermissionDeleteCall<'a, S> {
        self._supports_all_drives = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> PermissionDeleteCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> PermissionDeleteCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> PermissionDeleteCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets a permission by ID.
///
/// A builder for the *get* method supported by a *permission* resource.
/// It is not used directly, but through a `PermissionMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.permissions().get("fileId", "permissionId")
///              .use_domain_admin_access(true)
///              .supports_team_drives(false)
///              .supports_all_drives(true)
///              .doit().await;
/// # }
/// ```
pub struct PermissionGetCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _file_id: String,
    _permission_id: String,
    _use_domain_admin_access: Option<bool>,
    _supports_team_drives: Option<bool>,
    _supports_all_drives: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for PermissionGetCall<'a, S> {}

impl<'a, S> PermissionGetCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Permission)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.permissions.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        params.push(("permissionId", self._permission_id.to_string()));
        if let Some(value) = self._use_domain_admin_access {
            params.push(("useDomainAdminAccess", value.to_string()));
        }
        if let Some(value) = self._supports_team_drives {
            params.push(("supportsTeamDrives", value.to_string()));
        }
        if let Some(value) = self._supports_all_drives {
            params.push(("supportsAllDrives", value.to_string()));
        }
        for &field in ["alt", "fileId", "permissionId", "useDomainAdminAccess", "supportsTeamDrives", "supportsAllDrives"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "files/{fileId}/permissions/{permissionId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::MetadataReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId"), ("{permissionId}", "permissionId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["permissionId", "fileId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> PermissionGetCall<'a, S> {
        self._file_id = new_value.to_string();
        self
    }
    /// The ID of the permission.
    ///
    /// Sets the *permission id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn permission_id(mut self, new_value: &str) -> PermissionGetCall<'a, S> {
        self._permission_id = new_value.to_string();
        self
    }
    /// Issue the request as a domain administrator; if set to true, then the requester will be granted access if the file ID parameter refers to a shared drive and the requester is an administrator of the domain to which the shared drive belongs.
    ///
    /// Sets the *use domain admin access* query property to the given value.
    pub fn use_domain_admin_access(mut self, new_value: bool) -> PermissionGetCall<'a, S> {
        self._use_domain_admin_access = Some(new_value);
        self
    }
    /// Deprecated use supportsAllDrives instead.
    ///
    /// Sets the *supports team drives* query property to the given value.
    pub fn supports_team_drives(mut self, new_value: bool) -> PermissionGetCall<'a, S> {
        self._supports_team_drives = Some(new_value);
        self
    }
    /// Whether the requesting application supports both My Drives and shared drives.
    ///
    /// Sets the *supports all drives* query property to the given value.
    pub fn supports_all_drives(mut self, new_value: bool) -> PermissionGetCall<'a, S> {
        self._supports_all_drives = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> PermissionGetCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> PermissionGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::MetadataReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> PermissionGetCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists a file's or shared drive's permissions.
///
/// A builder for the *list* method supported by a *permission* resource.
/// It is not used directly, but through a `PermissionMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.permissions().list("fileId")
///              .use_domain_admin_access(false)
///              .supports_team_drives(true)
///              .supports_all_drives(false)
///              .page_token("tempor")
///              .page_size(-10)
///              .include_permissions_for_view("et")
///              .doit().await;
/// # }
/// ```
pub struct PermissionListCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _file_id: String,
    _use_domain_admin_access: Option<bool>,
    _supports_team_drives: Option<bool>,
    _supports_all_drives: Option<bool>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _include_permissions_for_view: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for PermissionListCall<'a, S> {}

impl<'a, S> PermissionListCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, PermissionList)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.permissions.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(9 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        if let Some(value) = self._use_domain_admin_access {
            params.push(("useDomainAdminAccess", value.to_string()));
        }
        if let Some(value) = self._supports_team_drives {
            params.push(("supportsTeamDrives", value.to_string()));
        }
        if let Some(value) = self._supports_all_drives {
            params.push(("supportsAllDrives", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._include_permissions_for_view {
            params.push(("includePermissionsForView", value.to_string()));
        }
        for &field in ["alt", "fileId", "useDomainAdminAccess", "supportsTeamDrives", "supportsAllDrives", "pageToken", "pageSize", "includePermissionsForView"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "files/{fileId}/permissions";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::MetadataReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["fileId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the file or shared drive.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> PermissionListCall<'a, S> {
        self._file_id = new_value.to_string();
        self
    }
    /// Issue the request as a domain administrator; if set to true, then the requester will be granted access if the file ID parameter refers to a shared drive and the requester is an administrator of the domain to which the shared drive belongs.
    ///
    /// Sets the *use domain admin access* query property to the given value.
    pub fn use_domain_admin_access(mut self, new_value: bool) -> PermissionListCall<'a, S> {
        self._use_domain_admin_access = Some(new_value);
        self
    }
    /// Deprecated use supportsAllDrives instead.
    ///
    /// Sets the *supports team drives* query property to the given value.
    pub fn supports_team_drives(mut self, new_value: bool) -> PermissionListCall<'a, S> {
        self._supports_team_drives = Some(new_value);
        self
    }
    /// Whether the requesting application supports both My Drives and shared drives.
    ///
    /// Sets the *supports all drives* query property to the given value.
    pub fn supports_all_drives(mut self, new_value: bool) -> PermissionListCall<'a, S> {
        self._supports_all_drives = Some(new_value);
        self
    }
    /// The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> PermissionListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of permissions to return per page. When not set for files in a shared drive, at most 100 results will be returned. When not set for files that are not in a shared drive, the entire list will be returned.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> PermissionListCall<'a, S> {
        self._page_size = Some(new_value);
        self
    }
    /// Specifies which additional view's permissions to include in the response. Only 'published' is supported.
    ///
    /// Sets the *include permissions for view* query property to the given value.
    pub fn include_permissions_for_view(mut self, new_value: &str) -> PermissionListCall<'a, S> {
        self._include_permissions_for_view = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> PermissionListCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> PermissionListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::MetadataReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> PermissionListCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates a permission with patch semantics.
///
/// A builder for the *update* method supported by a *permission* resource.
/// It is not used directly, but through a `PermissionMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// use drive3::api::Permission;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Permission::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.permissions().update(req, "fileId", "permissionId")
///              .use_domain_admin_access(true)
///              .transfer_ownership(true)
///              .supports_team_drives(false)
///              .supports_all_drives(false)
///              .remove_expiration(false)
///              .doit().await;
/// # }
/// ```
pub struct PermissionUpdateCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _request: Permission,
    _file_id: String,
    _permission_id: String,
    _use_domain_admin_access: Option<bool>,
    _transfer_ownership: Option<bool>,
    _supports_team_drives: Option<bool>,
    _supports_all_drives: Option<bool>,
    _remove_expiration: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for PermissionUpdateCall<'a, S> {}

impl<'a, S> PermissionUpdateCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Permission)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.permissions.update",
                               http_method: hyper::Method::PATCH });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(10 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        params.push(("permissionId", self._permission_id.to_string()));
        if let Some(value) = self._use_domain_admin_access {
            params.push(("useDomainAdminAccess", value.to_string()));
        }
        if let Some(value) = self._transfer_ownership {
            params.push(("transferOwnership", value.to_string()));
        }
        if let Some(value) = self._supports_team_drives {
            params.push(("supportsTeamDrives", value.to_string()));
        }
        if let Some(value) = self._supports_all_drives {
            params.push(("supportsAllDrives", value.to_string()));
        }
        if let Some(value) = self._remove_expiration {
            params.push(("removeExpiration", value.to_string()));
        }
        for &field in ["alt", "fileId", "permissionId", "useDomainAdminAccess", "transferOwnership", "supportsTeamDrives", "supportsAllDrives", "removeExpiration"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "files/{fileId}/permissions/{permissionId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId"), ("{permissionId}", "permissionId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["permissionId", "fileId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::PATCH).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Permission) -> PermissionUpdateCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the file or shared drive.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> PermissionUpdateCall<'a, S> {
        self._file_id = new_value.to_string();
        self
    }
    /// The ID of the permission.
    ///
    /// Sets the *permission id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn permission_id(mut self, new_value: &str) -> PermissionUpdateCall<'a, S> {
        self._permission_id = new_value.to_string();
        self
    }
    /// Issue the request as a domain administrator; if set to true, then the requester will be granted access if the file ID parameter refers to a shared drive and the requester is an administrator of the domain to which the shared drive belongs.
    ///
    /// Sets the *use domain admin access* query property to the given value.
    pub fn use_domain_admin_access(mut self, new_value: bool) -> PermissionUpdateCall<'a, S> {
        self._use_domain_admin_access = Some(new_value);
        self
    }
    /// Whether to transfer ownership to the specified user and downgrade the current owner to a writer. This parameter is required as an acknowledgement of the side effect. File owners can only transfer ownership of files existing on My Drive. Files existing in a shared drive are owned by the organization that owns that shared drive. Ownership transfers are not supported for files and folders in shared drives. Organizers of a shared drive can move items from that shared drive into their My Drive which transfers the ownership to them.
    ///
    /// Sets the *transfer ownership* query property to the given value.
    pub fn transfer_ownership(mut self, new_value: bool) -> PermissionUpdateCall<'a, S> {
        self._transfer_ownership = Some(new_value);
        self
    }
    /// Deprecated use supportsAllDrives instead.
    ///
    /// Sets the *supports team drives* query property to the given value.
    pub fn supports_team_drives(mut self, new_value: bool) -> PermissionUpdateCall<'a, S> {
        self._supports_team_drives = Some(new_value);
        self
    }
    /// Whether the requesting application supports both My Drives and shared drives.
    ///
    /// Sets the *supports all drives* query property to the given value.
    pub fn supports_all_drives(mut self, new_value: bool) -> PermissionUpdateCall<'a, S> {
        self._supports_all_drives = Some(new_value);
        self
    }
    /// Whether to remove the expiration date.
    ///
    /// Sets the *remove expiration* query property to the given value.
    pub fn remove_expiration(mut self, new_value: bool) -> PermissionUpdateCall<'a, S> {
        self._remove_expiration = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> PermissionUpdateCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> PermissionUpdateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> PermissionUpdateCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a new reply to a comment.
///
/// A builder for the *create* method supported by a *reply* resource.
/// It is not used directly, but through a `ReplyMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// use drive3::api::Reply;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Reply::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.replies().create(req, "fileId", "commentId")
///              .doit().await;
/// # }
/// ```
pub struct ReplyCreateCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _request: Reply,
    _file_id: String,
    _comment_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for ReplyCreateCall<'a, S> {}

impl<'a, S> ReplyCreateCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Reply)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.replies.create",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        params.push(("commentId", self._comment_id.to_string()));
        for &field in ["alt", "fileId", "commentId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "files/{fileId}/comments/{commentId}/replies";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId"), ("{commentId}", "commentId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["commentId", "fileId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Reply) -> ReplyCreateCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> ReplyCreateCall<'a, S> {
        self._file_id = new_value.to_string();
        self
    }
    /// The ID of the comment.
    ///
    /// Sets the *comment id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn comment_id(mut self, new_value: &str) -> ReplyCreateCall<'a, S> {
        self._comment_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ReplyCreateCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ReplyCreateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> ReplyCreateCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deletes a reply.
///
/// A builder for the *delete* method supported by a *reply* resource.
/// It is not used directly, but through a `ReplyMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.replies().delete("fileId", "commentId", "replyId")
///              .doit().await;
/// # }
/// ```
pub struct ReplyDeleteCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _file_id: String,
    _comment_id: String,
    _reply_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for ReplyDeleteCall<'a, S> {}

impl<'a, S> ReplyDeleteCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<hyper::Response<hyper::body::Body>> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.replies.delete",
                               http_method: hyper::Method::DELETE });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        params.push(("commentId", self._comment_id.to_string()));
        params.push(("replyId", self._reply_id.to_string()));
        for &field in ["fileId", "commentId", "replyId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "files/{fileId}/comments/{commentId}/replies/{replyId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId"), ("{commentId}", "commentId"), ("{replyId}", "replyId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["replyId", "commentId", "fileId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::DELETE).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> ReplyDeleteCall<'a, S> {
        self._file_id = new_value.to_string();
        self
    }
    /// The ID of the comment.
    ///
    /// Sets the *comment id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn comment_id(mut self, new_value: &str) -> ReplyDeleteCall<'a, S> {
        self._comment_id = new_value.to_string();
        self
    }
    /// The ID of the reply.
    ///
    /// Sets the *reply id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn reply_id(mut self, new_value: &str) -> ReplyDeleteCall<'a, S> {
        self._reply_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ReplyDeleteCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ReplyDeleteCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> ReplyDeleteCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets a reply by ID.
///
/// A builder for the *get* method supported by a *reply* resource.
/// It is not used directly, but through a `ReplyMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.replies().get("fileId", "commentId", "replyId")
///              .include_deleted(false)
///              .doit().await;
/// # }
/// ```
pub struct ReplyGetCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _file_id: String,
    _comment_id: String,
    _reply_id: String,
    _include_deleted: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for ReplyGetCall<'a, S> {}

impl<'a, S> ReplyGetCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Reply)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.replies.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        params.push(("commentId", self._comment_id.to_string()));
        params.push(("replyId", self._reply_id.to_string()));
        if let Some(value) = self._include_deleted {
            params.push(("includeDeleted", value.to_string()));
        }
        for &field in ["alt", "fileId", "commentId", "replyId", "includeDeleted"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "files/{fileId}/comments/{commentId}/replies/{replyId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId"), ("{commentId}", "commentId"), ("{replyId}", "replyId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["replyId", "commentId", "fileId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> ReplyGetCall<'a, S> {
        self._file_id = new_value.to_string();
        self
    }
    /// The ID of the comment.
    ///
    /// Sets the *comment id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn comment_id(mut self, new_value: &str) -> ReplyGetCall<'a, S> {
        self._comment_id = new_value.to_string();
        self
    }
    /// The ID of the reply.
    ///
    /// Sets the *reply id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn reply_id(mut self, new_value: &str) -> ReplyGetCall<'a, S> {
        self._reply_id = new_value.to_string();
        self
    }
    /// Whether to return deleted replies. Deleted replies will not include their original content.
    ///
    /// Sets the *include deleted* query property to the given value.
    pub fn include_deleted(mut self, new_value: bool) -> ReplyGetCall<'a, S> {
        self._include_deleted = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ReplyGetCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ReplyGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Readonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> ReplyGetCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists a comment's replies.
///
/// A builder for the *list* method supported by a *reply* resource.
/// It is not used directly, but through a `ReplyMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.replies().list("fileId", "commentId")
///              .page_token("accusam")
///              .page_size(-39)
///              .include_deleted(true)
///              .doit().await;
/// # }
/// ```
pub struct ReplyListCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _file_id: String,
    _comment_id: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _include_deleted: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for ReplyListCall<'a, S> {}

impl<'a, S> ReplyListCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ReplyList)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.replies.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        params.push(("commentId", self._comment_id.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._include_deleted {
            params.push(("includeDeleted", value.to_string()));
        }
        for &field in ["alt", "fileId", "commentId", "pageToken", "pageSize", "includeDeleted"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "files/{fileId}/comments/{commentId}/replies";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId"), ("{commentId}", "commentId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["commentId", "fileId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> ReplyListCall<'a, S> {
        self._file_id = new_value.to_string();
        self
    }
    /// The ID of the comment.
    ///
    /// Sets the *comment id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn comment_id(mut self, new_value: &str) -> ReplyListCall<'a, S> {
        self._comment_id = new_value.to_string();
        self
    }
    /// The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ReplyListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of replies to return per page.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> ReplyListCall<'a, S> {
        self._page_size = Some(new_value);
        self
    }
    /// Whether to include deleted replies. Deleted replies will not include their original content.
    ///
    /// Sets the *include deleted* query property to the given value.
    pub fn include_deleted(mut self, new_value: bool) -> ReplyListCall<'a, S> {
        self._include_deleted = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ReplyListCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ReplyListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Readonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> ReplyListCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates a reply with patch semantics.
///
/// A builder for the *update* method supported by a *reply* resource.
/// It is not used directly, but through a `ReplyMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// use drive3::api::Reply;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Reply::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.replies().update(req, "fileId", "commentId", "replyId")
///              .doit().await;
/// # }
/// ```
pub struct ReplyUpdateCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _request: Reply,
    _file_id: String,
    _comment_id: String,
    _reply_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for ReplyUpdateCall<'a, S> {}

impl<'a, S> ReplyUpdateCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Reply)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.replies.update",
                               http_method: hyper::Method::PATCH });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        params.push(("commentId", self._comment_id.to_string()));
        params.push(("replyId", self._reply_id.to_string()));
        for &field in ["alt", "fileId", "commentId", "replyId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "files/{fileId}/comments/{commentId}/replies/{replyId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId"), ("{commentId}", "commentId"), ("{replyId}", "replyId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["replyId", "commentId", "fileId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::PATCH).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Reply) -> ReplyUpdateCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> ReplyUpdateCall<'a, S> {
        self._file_id = new_value.to_string();
        self
    }
    /// The ID of the comment.
    ///
    /// Sets the *comment id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn comment_id(mut self, new_value: &str) -> ReplyUpdateCall<'a, S> {
        self._comment_id = new_value.to_string();
        self
    }
    /// The ID of the reply.
    ///
    /// Sets the *reply id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn reply_id(mut self, new_value: &str) -> ReplyUpdateCall<'a, S> {
        self._reply_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ReplyUpdateCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ReplyUpdateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> ReplyUpdateCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Permanently deletes a file version. You can only delete revisions for files with binary content in Google Drive, like images or videos. Revisions for other files, like Google Docs or Sheets, and the last remaining file version can't be deleted.
///
/// A builder for the *delete* method supported by a *revision* resource.
/// It is not used directly, but through a `RevisionMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.revisions().delete("fileId", "revisionId")
///              .doit().await;
/// # }
/// ```
pub struct RevisionDeleteCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _file_id: String,
    _revision_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for RevisionDeleteCall<'a, S> {}

impl<'a, S> RevisionDeleteCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<hyper::Response<hyper::body::Body>> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.revisions.delete",
                               http_method: hyper::Method::DELETE });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        params.push(("revisionId", self._revision_id.to_string()));
        for &field in ["fileId", "revisionId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "files/{fileId}/revisions/{revisionId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId"), ("{revisionId}", "revisionId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["revisionId", "fileId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::DELETE).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> RevisionDeleteCall<'a, S> {
        self._file_id = new_value.to_string();
        self
    }
    /// The ID of the revision.
    ///
    /// Sets the *revision id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn revision_id(mut self, new_value: &str) -> RevisionDeleteCall<'a, S> {
        self._revision_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> RevisionDeleteCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> RevisionDeleteCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> RevisionDeleteCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets a revision's metadata or content by ID.
///
/// This method supports **media download**. To enable it, adjust the builder like this:
/// `.param("alt", "media")`.
/// Please note that due to missing multi-part support on the server side, you will only receive the media,
/// but not the `Revision` structure that you would usually get. The latter will be a default value.
///
/// A builder for the *get* method supported by a *revision* resource.
/// It is not used directly, but through a `RevisionMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.revisions().get("fileId", "revisionId")
///              .acknowledge_abuse(false)
///              .doit().await;
/// # }
/// ```
pub struct RevisionGetCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _file_id: String,
    _revision_id: String,
    _acknowledge_abuse: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for RevisionGetCall<'a, S> {}

impl<'a, S> RevisionGetCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Revision)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.revisions.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        params.push(("revisionId", self._revision_id.to_string()));
        if let Some(value) = self._acknowledge_abuse {
            params.push(("acknowledgeAbuse", value.to_string()));
        }
        for &field in ["fileId", "revisionId", "acknowledgeAbuse"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let (json_field_missing, enable_resource_parsing) = {
            let mut enable = true;
            let mut field_present = true;
            for &(name, ref value) in params.iter() {
                if name == "alt" {
                    field_present = false;
                    if <String as AsRef<str>>::as_ref(&value) != "json" {
                        enable = false;
                    }
                    break;
                }
            }
            (field_present, enable)
        };
        if json_field_missing {
            params.push(("alt", "json".to_string()));
        }

        let mut url = self.hub._base_url.clone() + "files/{fileId}/revisions/{revisionId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::MetadataReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId"), ("{revisionId}", "revisionId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["revisionId", "fileId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = if enable_resource_parsing {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    } else { (res, Default::default()) };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> RevisionGetCall<'a, S> {
        self._file_id = new_value.to_string();
        self
    }
    /// The ID of the revision.
    ///
    /// Sets the *revision id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn revision_id(mut self, new_value: &str) -> RevisionGetCall<'a, S> {
        self._revision_id = new_value.to_string();
        self
    }
    /// Whether the user is acknowledging the risk of downloading known malware or other abusive files. This is only applicable when alt=media.
    ///
    /// Sets the *acknowledge abuse* query property to the given value.
    pub fn acknowledge_abuse(mut self, new_value: bool) -> RevisionGetCall<'a, S> {
        self._acknowledge_abuse = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> RevisionGetCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> RevisionGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::MetadataReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> RevisionGetCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists a file's revisions.
///
/// A builder for the *list* method supported by a *revision* resource.
/// It is not used directly, but through a `RevisionMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.revisions().list("fileId")
///              .page_token("consetetur")
///              .page_size(-11)
///              .doit().await;
/// # }
/// ```
pub struct RevisionListCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _file_id: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for RevisionListCall<'a, S> {}

impl<'a, S> RevisionListCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, RevisionList)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.revisions.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        for &field in ["alt", "fileId", "pageToken", "pageSize"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "files/{fileId}/revisions";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::MetadataReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["fileId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> RevisionListCall<'a, S> {
        self._file_id = new_value.to_string();
        self
    }
    /// The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> RevisionListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of revisions to return per page.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> RevisionListCall<'a, S> {
        self._page_size = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> RevisionListCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> RevisionListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::MetadataReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> RevisionListCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates a revision with patch semantics.
///
/// A builder for the *update* method supported by a *revision* resource.
/// It is not used directly, but through a `RevisionMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// use drive3::api::Revision;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Revision::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.revisions().update(req, "fileId", "revisionId")
///              .doit().await;
/// # }
/// ```
pub struct RevisionUpdateCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _request: Revision,
    _file_id: String,
    _revision_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for RevisionUpdateCall<'a, S> {}

impl<'a, S> RevisionUpdateCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Revision)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.revisions.update",
                               http_method: hyper::Method::PATCH });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        params.push(("revisionId", self._revision_id.to_string()));
        for &field in ["alt", "fileId", "revisionId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "files/{fileId}/revisions/{revisionId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId"), ("{revisionId}", "revisionId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["revisionId", "fileId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::PATCH).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Revision) -> RevisionUpdateCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> RevisionUpdateCall<'a, S> {
        self._file_id = new_value.to_string();
        self
    }
    /// The ID of the revision.
    ///
    /// Sets the *revision id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn revision_id(mut self, new_value: &str) -> RevisionUpdateCall<'a, S> {
        self._revision_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> RevisionUpdateCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> RevisionUpdateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> RevisionUpdateCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deprecated use drives.create instead.
///
/// A builder for the *create* method supported by a *teamdrive* resource.
/// It is not used directly, but through a `TeamdriveMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// use drive3::api::TeamDrive;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = TeamDrive::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.teamdrives().create(req, "requestId")
///              .doit().await;
/// # }
/// ```
pub struct TeamdriveCreateCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _request: TeamDrive,
    _request_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for TeamdriveCreateCall<'a, S> {}

impl<'a, S> TeamdriveCreateCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, TeamDrive)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.teamdrives.create",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("requestId", self._request_id.to_string()));
        for &field in ["alt", "requestId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "teamdrives";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }


        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: TeamDrive) -> TeamdriveCreateCall<'a, S> {
        self._request = new_value;
        self
    }
    /// An ID, such as a random UUID, which uniquely identifies this user's request for idempotent creation of a Team Drive. A repeated request by the same user and with the same request ID will avoid creating duplicates by attempting to create the same Team Drive. If the Team Drive already exists a 409 error will be returned.
    ///
    /// Sets the *request id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request_id(mut self, new_value: &str) -> TeamdriveCreateCall<'a, S> {
        self._request_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> TeamdriveCreateCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> TeamdriveCreateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> TeamdriveCreateCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deprecated use drives.delete instead.
///
/// A builder for the *delete* method supported by a *teamdrive* resource.
/// It is not used directly, but through a `TeamdriveMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.teamdrives().delete("teamDriveId")
///              .doit().await;
/// # }
/// ```
pub struct TeamdriveDeleteCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _team_drive_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for TeamdriveDeleteCall<'a, S> {}

impl<'a, S> TeamdriveDeleteCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<hyper::Response<hyper::body::Body>> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.teamdrives.delete",
                               http_method: hyper::Method::DELETE });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(2 + self._additional_params.len());
        params.push(("teamDriveId", self._team_drive_id.to_string()));
        for &field in ["teamDriveId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "teamdrives/{teamDriveId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{teamDriveId}", "teamDriveId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["teamDriveId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::DELETE).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the Team Drive
    ///
    /// Sets the *team drive id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn team_drive_id(mut self, new_value: &str) -> TeamdriveDeleteCall<'a, S> {
        self._team_drive_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> TeamdriveDeleteCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> TeamdriveDeleteCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> TeamdriveDeleteCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deprecated use drives.get instead.
///
/// A builder for the *get* method supported by a *teamdrive* resource.
/// It is not used directly, but through a `TeamdriveMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.teamdrives().get("teamDriveId")
///              .use_domain_admin_access(true)
///              .doit().await;
/// # }
/// ```
pub struct TeamdriveGetCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _team_drive_id: String,
    _use_domain_admin_access: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for TeamdriveGetCall<'a, S> {}

impl<'a, S> TeamdriveGetCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, TeamDrive)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.teamdrives.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("teamDriveId", self._team_drive_id.to_string()));
        if let Some(value) = self._use_domain_admin_access {
            params.push(("useDomainAdminAccess", value.to_string()));
        }
        for &field in ["alt", "teamDriveId", "useDomainAdminAccess"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "teamdrives/{teamDriveId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{teamDriveId}", "teamDriveId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["teamDriveId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the Team Drive
    ///
    /// Sets the *team drive id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn team_drive_id(mut self, new_value: &str) -> TeamdriveGetCall<'a, S> {
        self._team_drive_id = new_value.to_string();
        self
    }
    /// Issue the request as a domain administrator; if set to true, then the requester will be granted access if they are an administrator of the domain to which the Team Drive belongs.
    ///
    /// Sets the *use domain admin access* query property to the given value.
    pub fn use_domain_admin_access(mut self, new_value: bool) -> TeamdriveGetCall<'a, S> {
        self._use_domain_admin_access = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> TeamdriveGetCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> TeamdriveGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Readonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> TeamdriveGetCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deprecated use drives.list instead.
///
/// A builder for the *list* method supported by a *teamdrive* resource.
/// It is not used directly, but through a `TeamdriveMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.teamdrives().list()
///              .use_domain_admin_access(true)
///              .q("sit")
///              .page_token("kasd")
///              .page_size(-47)
///              .doit().await;
/// # }
/// ```
pub struct TeamdriveListCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _use_domain_admin_access: Option<bool>,
    _q: Option<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for TeamdriveListCall<'a, S> {}

impl<'a, S> TeamdriveListCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, TeamDriveList)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.teamdrives.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        if let Some(value) = self._use_domain_admin_access {
            params.push(("useDomainAdminAccess", value.to_string()));
        }
        if let Some(value) = self._q {
            params.push(("q", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        for &field in ["alt", "useDomainAdminAccess", "q", "pageToken", "pageSize"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "teamdrives";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_ref().to_string(), ());
        }


        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Issue the request as a domain administrator; if set to true, then all Team Drives of the domain in which the requester is an administrator are returned.
    ///
    /// Sets the *use domain admin access* query property to the given value.
    pub fn use_domain_admin_access(mut self, new_value: bool) -> TeamdriveListCall<'a, S> {
        self._use_domain_admin_access = Some(new_value);
        self
    }
    /// Query string for searching Team Drives.
    ///
    /// Sets the *q* query property to the given value.
    pub fn q(mut self, new_value: &str) -> TeamdriveListCall<'a, S> {
        self._q = Some(new_value.to_string());
        self
    }
    /// Page token for Team Drives.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> TeamdriveListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of Team Drives to return.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> TeamdriveListCall<'a, S> {
        self._page_size = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> TeamdriveListCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> TeamdriveListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Readonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> TeamdriveListCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deprecated use drives.update instead
///
/// A builder for the *update* method supported by a *teamdrive* resource.
/// It is not used directly, but through a `TeamdriveMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_drive3 as drive3;
/// use drive3::api::TeamDrive;
/// # async fn dox() {
/// # use std::default::Default;
/// # use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = TeamDrive::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.teamdrives().update(req, "teamDriveId")
///              .use_domain_admin_access(false)
///              .doit().await;
/// # }
/// ```
pub struct TeamdriveUpdateCall<'a, S>
    where S: 'a {

    hub: &'a DriveHub<S>,
    _request: TeamDrive,
    _team_drive_id: String,
    _use_domain_admin_access: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for TeamdriveUpdateCall<'a, S> {}

impl<'a, S> TeamdriveUpdateCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, TeamDrive)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "drive.teamdrives.update",
                               http_method: hyper::Method::PATCH });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("teamDriveId", self._team_drive_id.to_string()));
        if let Some(value) = self._use_domain_admin_access {
            params.push(("useDomainAdminAccess", value.to_string()));
        }
        for &field in ["alt", "teamDriveId", "useDomainAdminAccess"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "teamdrives/{teamDriveId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{teamDriveId}", "teamDriveId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["teamDriveId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.keys().map(String::as_str).collect::<Vec<_>>()[..]).await {
                // TODO: remove Ok / Err branches
                Ok(Some(token)) => token.clone(),
                Ok(None) => {
                    let err = oauth2::Error::OtherError(anyhow::Error::msg("unknown error occurred while generating oauth2 token"));
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
                Err(err) => {
                    match dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::PATCH).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: TeamDrive) -> TeamdriveUpdateCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the Team Drive
    ///
    /// Sets the *team drive id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn team_drive_id(mut self, new_value: &str) -> TeamdriveUpdateCall<'a, S> {
        self._team_drive_id = new_value.to_string();
        self
    }
    /// Issue the request as a domain administrator; if set to true, then the requester will be granted access if they are an administrator of the domain to which the Team Drive belongs.
    ///
    /// Sets the *use domain admin access* query property to the given value.
    pub fn use_domain_admin_access(mut self, new_value: bool) -> TeamdriveUpdateCall<'a, S> {
        self._use_domain_admin_access = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> TeamdriveUpdateCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> TeamdriveUpdateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> TeamdriveUpdateCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


