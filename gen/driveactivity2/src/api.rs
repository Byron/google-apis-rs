use std::collections::HashMap;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::default::Default;
use std::collections::BTreeMap;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;
use std::thread::sleep;

use crate::client;

// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// View and add to the activity record of files in your Google Drive
    DriveActivity,

    /// View the activity record of files in your Google Drive
    DriveActivityReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::DriveActivity => "https://www.googleapis.com/auth/drive.activity",
            Scope::DriveActivityReadonly => "https://www.googleapis.com/auth/drive.activity.readonly",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::DriveActivityReadonly
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all DriveActivityHub related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_driveactivity2 as driveactivity2;
/// use driveactivity2::api::QueryDriveActivityRequest;
/// use driveactivity2::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use driveactivity2::DriveActivityHub;
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DriveActivityHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = QueryDriveActivityRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.activity().query(req)
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
pub struct DriveActivityHub<C> {
    client: RefCell<C>,
    auth: RefCell<oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>>>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C> client::Hub for DriveActivityHub<C> {}

impl<'a, C> DriveActivityHub<C>
    where  C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {

    pub fn new(client: C, authenticator: oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>>) -> DriveActivityHub<C> {
        DriveActivityHub {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/2.0.0".to_string(),
            _base_url: "https://driveactivity.googleapis.com/".to_string(),
            _root_url: "https://driveactivity.googleapis.com/".to_string(),
        }
    }

    pub fn activity(&'a self) -> ActivityMethods<'a, C> {
        ActivityMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/2.0.0`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://driveactivity.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://driveactivity.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// Information about the action.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Action {
    /// The actor responsible for this action (or empty if all actors are responsible).
    pub actor: Option<Actor>,
    /// The type and detailed information about the action.
    pub detail: Option<ActionDetail>,
    /// The target this action affects (or empty if affecting all targets). This represents the state of the target immediately after this action occurred.
    pub target: Option<Target>,
    /// The action occurred over this time range.
    #[serde(rename="timeRange")]
    pub time_range: Option<TimeRange>,
    /// The action occurred at this specific time.
    pub timestamp: Option<String>,
}

impl client::Part for Action {}


/// Data describing the type and additional information of an action.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActionDetail {
    /// A change about comments was made.
    pub comment: Option<Comment>,
    /// An object was created.
    pub create: Option<Create>,
    /// An object was deleted.
    pub delete: Option<Delete>,
    /// A change happened in data leak prevention status.
    #[serde(rename="dlpChange")]
    pub dlp_change: Option<DataLeakPreventionChange>,
    /// An object was edited.
    pub edit: Option<Edit>,
    /// An object was moved.
    #[serde(rename="move")]
    pub move_: Option<Move>,
    /// The permission on an object was changed.
    #[serde(rename="permissionChange")]
    pub permission_change: Option<PermissionChange>,
    /// An object was referenced in an application outside of Drive/Docs.
    pub reference: Option<ApplicationReference>,
    /// An object was renamed.
    pub rename: Option<Rename>,
    /// A deleted object was restored.
    pub restore: Option<Restore>,
    /// Settings were changed.
    #[serde(rename="settingsChange")]
    pub settings_change: Option<SettingsChange>,
}

impl client::Part for ActionDetail {}


/// The actor of a Drive activity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Actor {
    /// An administrator.
    pub administrator: Option<Administrator>,
    /// An anonymous user.
    pub anonymous: Option<AnonymousUser>,
    /// An account acting on behalf of another.
    pub impersonation: Option<Impersonation>,
    /// A non-user actor (i.e. system triggered).
    pub system: Option<SystemEvent>,
    /// An end user.
    pub user: Option<User>,
}

impl client::Part for Actor {}


/// Empty message representing an administrator.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Administrator { _never_set: Option<bool> }

impl client::Part for Administrator {}


/// Empty message representing an anonymous user or indicating the authenticated user should be anonymized.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnonymousUser { _never_set: Option<bool> }

impl client::Part for AnonymousUser {}


/// Represents any user (including a logged out user).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Anyone { _never_set: Option<bool> }

impl client::Part for Anyone {}


/// Activity in applications other than Drive.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApplicationReference {
    /// The reference type corresponding to this event.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl client::Part for ApplicationReference {}


/// A comment with an assignment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Assignment {
    /// The user to whom the comment was assigned.
    #[serde(rename="assignedUser")]
    pub assigned_user: Option<User>,
    /// The sub-type of this event.
    pub subtype: Option<String>,
}

impl client::Part for Assignment {}


/// A change about comments on an object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Comment {
    /// A change on an assignment.
    pub assignment: Option<Assignment>,
    /// Users who are mentioned in this comment.
    #[serde(rename="mentionedUsers")]
    pub mentioned_users: Option<Vec<User>>,
    /// A change on a regular posted comment.
    pub post: Option<Post>,
    /// A change on a suggestion.
    pub suggestion: Option<Suggestion>,
}

impl client::Part for Comment {}


/// How the individual activities are consolidated. A set of activities may be consolidated into one combined activity if they are related in some way, such as one actor performing the same action on multiple targets, or multiple actors performing the same action on a single target. The strategy defines the rules for which activities are related.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConsolidationStrategy {
    /// The individual activities are consolidated using the legacy strategy.
    pub legacy: Option<Legacy>,
    /// The individual activities are not consolidated.
    pub none: Option<NoConsolidation>,
}

impl client::Part for ConsolidationStrategy {}


/// An object was created by copying an existing object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Copy {
    /// The original object.
    #[serde(rename="originalObject")]
    pub original_object: Option<TargetReference>,
}

impl client::Part for Copy {}


/// An object was created.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Create {
    /// If present, indicates the object was created by copying an existing Drive object.
    pub copy: Option<Copy>,
    /// If present, indicates the object was newly created (e.g. as a blank document), not derived from a Drive object or external object.
    pub new: Option<New>,
    /// If present, indicates the object originated externally and was uploaded to Drive.
    pub upload: Option<Upload>,
}

impl client::Part for Create {}


/// A change in the object's data leak prevention status.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataLeakPreventionChange {
    /// The type of Data Leak Prevention (DLP) change.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl client::Part for DataLeakPreventionChange {}


/// An object was deleted.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Delete {
    /// The type of delete action taken.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl client::Part for Delete {}


/// A user whose account has since been deleted.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeletedUser { _never_set: Option<bool> }

impl client::Part for DeletedUser {}


/// Information about a domain.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Domain {
    /// An opaque string used to identify this domain.
    #[serde(rename="legacyId")]
    pub legacy_id: Option<String>,
    /// The name of the domain, e.g. `google.com`.
    pub name: Option<String>,
}

impl client::Part for Domain {}


/// Information about a shared drive.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Drive {
    /// The resource name of the shared drive. The format is `COLLECTION_ID/DRIVE_ID`. Clients should not assume a specific collection ID for this resource name.
    pub name: Option<String>,
    /// The root of this shared drive.
    pub root: Option<DriveItem>,
    /// The title of the shared drive.
    pub title: Option<String>,
}

impl client::Part for Drive {}


/// A single Drive activity comprising one or more Actions by one or more Actors on one or more Targets. Some Action groupings occur spontaneously, such as moving an item into a shared folder triggering a permission change. Other groupings of related Actions, such as multiple Actors editing one item or moving multiple files into a new folder, are controlled by the selection of a ConsolidationStrategy in the QueryDriveActivityRequest.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DriveActivity {
    /// Details on all actions in this activity.
    pub actions: Option<Vec<Action>>,
    /// All actor(s) responsible for the activity.
    pub actors: Option<Vec<Actor>>,
    /// Key information about the primary action for this activity. This is either representative, or the most important, of all actions in the activity, according to the ConsolidationStrategy in the request.
    #[serde(rename="primaryActionDetail")]
    pub primary_action_detail: Option<ActionDetail>,
    /// All Google Drive objects this activity is about (e.g. file, folder, drive). This represents the state of the target immediately after the actions occurred.
    pub targets: Option<Vec<Target>>,
    /// The activity occurred over this time range.
    #[serde(rename="timeRange")]
    pub time_range: Option<TimeRange>,
    /// The activity occurred at this specific time.
    pub timestamp: Option<String>,
}

impl client::Part for DriveActivity {}


/// A Drive item which is a file.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DriveFile { _never_set: Option<bool> }

impl client::Part for DriveFile {}


/// A Drive item which is a folder.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DriveFolder {
    /// The type of Drive folder.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl client::Part for DriveFolder {}


/// A Drive item, such as a file or folder.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DriveItem {
    /// The Drive item is a file.
    #[serde(rename="driveFile")]
    pub drive_file: Option<DriveFile>,
    /// The Drive item is a folder. Includes information about the type of folder.
    #[serde(rename="driveFolder")]
    pub drive_folder: Option<DriveFolder>,
    /// This field is deprecated; please use the `driveFile` field instead.
    pub file: Option<File>,
    /// This field is deprecated; please use the `driveFolder` field instead.
    pub folder: Option<Folder>,
    /// The MIME type of the Drive item. See https://developers.google.com/drive/v3/web/mime-types.
    #[serde(rename="mimeType")]
    pub mime_type: Option<String>,
    /// The target Drive item. The format is `items/ITEM_ID`.
    pub name: Option<String>,
    /// Information about the owner of this Drive item.
    pub owner: Option<Owner>,
    /// The title of the Drive item.
    pub title: Option<String>,
}

impl client::Part for DriveItem {}


/// A lightweight reference to a Drive item, such as a file or folder.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DriveItemReference {
    /// The Drive item is a file.
    #[serde(rename="driveFile")]
    pub drive_file: Option<DriveFile>,
    /// The Drive item is a folder. Includes information about the type of folder.
    #[serde(rename="driveFolder")]
    pub drive_folder: Option<DriveFolder>,
    /// This field is deprecated; please use the `driveFile` field instead.
    pub file: Option<File>,
    /// This field is deprecated; please use the `driveFolder` field instead.
    pub folder: Option<Folder>,
    /// The target Drive item. The format is `items/ITEM_ID`.
    pub name: Option<String>,
    /// The title of the Drive item.
    pub title: Option<String>,
}

impl client::Part for DriveItemReference {}


/// A lightweight reference to a shared drive.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DriveReference {
    /// The resource name of the shared drive. The format is `COLLECTION_ID/DRIVE_ID`. Clients should not assume a specific collection ID for this resource name.
    pub name: Option<String>,
    /// The title of the shared drive.
    pub title: Option<String>,
}

impl client::Part for DriveReference {}


/// An empty message indicating an object was edited.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Edit { _never_set: Option<bool> }

impl client::Part for Edit {}


/// This item is deprecated; please see `DriveFile` instead.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct File { _never_set: Option<bool> }

impl client::Part for File {}


/// A comment on a file.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileComment {
    /// The comment in the discussion thread. This identifier is an opaque string compatible with the Drive API; see https://developers.google.com/drive/v3/reference/comments/get
    #[serde(rename="legacyCommentId")]
    pub legacy_comment_id: Option<String>,
    /// The discussion thread to which the comment was added. This identifier is an opaque string compatible with the Drive API and references the first comment in a discussion; see https://developers.google.com/drive/v3/reference/comments/get
    #[serde(rename="legacyDiscussionId")]
    pub legacy_discussion_id: Option<String>,
    /// The link to the discussion thread containing this comment, for example, `https://docs.google.com/DOCUMENT_ID/edit?disco=THREAD_ID`.
    #[serde(rename="linkToDiscussion")]
    pub link_to_discussion: Option<String>,
    /// The Drive item containing this comment.
    pub parent: Option<DriveItem>,
}

impl client::Part for FileComment {}


/// This item is deprecated; please see `DriveFolder` instead.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Folder {
    /// This field is deprecated; please see `DriveFolder.type` instead.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl client::Part for Folder {}


/// Information about a group.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Group {
    /// The email address of the group.
    pub email: Option<String>,
    /// The title of the group.
    pub title: Option<String>,
}

impl client::Part for Group {}


/// Information about an impersonation, where an admin acts on behalf of an end user. Information about the acting admin is not currently available.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Impersonation {
    /// The impersonated user.
    #[serde(rename="impersonatedUser")]
    pub impersonated_user: Option<User>,
}

impl client::Part for Impersonation {}


/// A known user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KnownUser {
    /// True if this is the user making the request.
    #[serde(rename="isCurrentUser")]
    pub is_current_user: Option<bool>,
    /// The identifier for this user that can be used with the People API to get more information. The format is `people/ACCOUNT_ID`. See https://developers.google.com/people/.
    #[serde(rename="personName")]
    pub person_name: Option<String>,
}

impl client::Part for KnownUser {}


/// A strategy which consolidates activities using the grouping rules from the legacy V1 Activity API. Similar actions occurring within a window of time can be grouped across multiple targets (such as moving a set of files at once) or multiple actors (such as several users editing the same item). Grouping rules for this strategy are specific to each type of action.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Legacy { _never_set: Option<bool> }

impl client::Part for Legacy {}


/// An object was moved.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Move {
    /// The added parent object(s).
    #[serde(rename="addedParents")]
    pub added_parents: Option<Vec<TargetReference>>,
    /// The removed parent object(s).
    #[serde(rename="removedParents")]
    pub removed_parents: Option<Vec<TargetReference>>,
}

impl client::Part for Move {}


/// An object was created from scratch.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct New { _never_set: Option<bool> }

impl client::Part for New {}


/// A strategy which does no consolidation of individual activities.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NoConsolidation { _never_set: Option<bool> }

impl client::Part for NoConsolidation {}


/// Information about the owner of a Drive item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Owner {
    /// The domain of the Drive item owner.
    pub domain: Option<Domain>,
    /// The drive that owns the item.
    pub drive: Option<DriveReference>,
    /// This field is deprecated; please use the `drive` field instead.
    #[serde(rename="teamDrive")]
    pub team_drive: Option<TeamDriveReference>,
    /// The user that owns the Drive item.
    pub user: Option<User>,
}

impl client::Part for Owner {}


/// The permission setting of an object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Permission {
    /// If true, the item can be discovered (e.g. in the user's "Shared with me" collection) without needing a link to the item.
    #[serde(rename="allowDiscovery")]
    pub allow_discovery: Option<bool>,
    /// If set, this permission applies to anyone, even logged out users.
    pub anyone: Option<Anyone>,
    /// The domain to whom this permission applies.
    pub domain: Option<Domain>,
    /// The group to whom this permission applies.
    pub group: Option<Group>,
    /// Indicates the [Google Drive permissions role](https://developers.google.com/drive/web/manage-sharing#roles). The role determines a user's ability to read, write, and comment on items.
    pub role: Option<String>,
    /// The user to whom this permission applies.
    pub user: Option<User>,
}

impl client::Part for Permission {}


/// A change of the permission setting on an item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PermissionChange {
    /// The set of permissions added by this change.
    #[serde(rename="addedPermissions")]
    pub added_permissions: Option<Vec<Permission>>,
    /// The set of permissions removed by this change.
    #[serde(rename="removedPermissions")]
    pub removed_permissions: Option<Vec<Permission>>,
}

impl client::Part for PermissionChange {}


/// A regular posted comment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Post {
    /// The sub-type of this event.
    pub subtype: Option<String>,
}

impl client::Part for Post {}


/// The request message for querying Drive activity.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [query activity](ActivityQueryCall) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryDriveActivityRequest {
    /// Return activities for this Drive folder and all children and descendants. The format is `items/ITEM_ID`.
    #[serde(rename="ancestorName")]
    pub ancestor_name: Option<String>,
    /// Details on how to consolidate related actions that make up the activity. If not set, then related actions are not consolidated.
    #[serde(rename="consolidationStrategy")]
    pub consolidation_strategy: Option<ConsolidationStrategy>,
    /// The filtering for items returned from this query request. The format of the filter string is a sequence of expressions, joined by an optional "AND", where each expression is of the form "field operator value". Supported fields: - `time`: Uses numerical operators on date values either in terms of milliseconds since Jan 1, 1970 or in RFC 3339 format. Examples: - `time > 1452409200000 AND time <= 1492812924310` - `time >= "2016-01-10T01:02:03-05:00"` - `detail.action_detail_case`: Uses the "has" operator (:) and either a singular value or a list of allowed action types enclosed in parentheses. Examples: - `detail.action_detail_case: RENAME` - `detail.action_detail_case:(CREATE EDIT)` - `-detail.action_detail_case:MOVE` 
    pub filter: Option<String>,
    /// Return activities for this Drive item. The format is `items/ITEM_ID`.
    #[serde(rename="itemName")]
    pub item_name: Option<String>,
    /// The miminum number of activities desired in the response; the server will attempt to return at least this quanitity. The server may also return fewer activities if it has a partial response ready before the request times out. If not set, a default value is used.
    #[serde(rename="pageSize")]
    pub page_size: Option<i32>,
    /// The token identifying which page of results to return. Set this to the next_page_token value returned from a previous query to obtain the following page of results. If not set, the first page of results will be returned.
    #[serde(rename="pageToken")]
    pub page_token: Option<String>,
}

impl client::RequestValue for QueryDriveActivityRequest {}


/// Response message for querying Drive activity.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [query activity](ActivityQueryCall) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryDriveActivityResponse {
    /// List of activity requested.
    pub activities: Option<Vec<DriveActivity>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for QueryDriveActivityResponse {}


/// An object was renamed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Rename {
    /// The new title of the drive object.
    #[serde(rename="newTitle")]
    pub new_title: Option<String>,
    /// The previous title of the drive object.
    #[serde(rename="oldTitle")]
    pub old_title: Option<String>,
}

impl client::Part for Rename {}


/// A deleted object was restored.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Restore {
    /// The type of restore action taken.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl client::Part for Restore {}


/// Information about restriction policy changes to a feature.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RestrictionChange {
    /// The feature which had a change in restriction policy.
    pub feature: Option<String>,
    /// The restriction in place after the change.
    #[serde(rename="newRestriction")]
    pub new_restriction: Option<String>,
}

impl client::Part for RestrictionChange {}


/// Information about settings changes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SettingsChange {
    /// The set of changes made to restrictions.
    #[serde(rename="restrictionChanges")]
    pub restriction_changes: Option<Vec<RestrictionChange>>,
}

impl client::Part for SettingsChange {}


/// A suggestion.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Suggestion {
    /// The sub-type of this event.
    pub subtype: Option<String>,
}

impl client::Part for Suggestion {}


/// Event triggered by system operations instead of end users.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SystemEvent {
    /// The type of the system event that may triggered activity.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl client::Part for SystemEvent {}


/// Information about the target of activity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Target {
    /// The target is a shared drive.
    pub drive: Option<Drive>,
    /// The target is a Drive item.
    #[serde(rename="driveItem")]
    pub drive_item: Option<DriveItem>,
    /// The target is a comment on a Drive file.
    #[serde(rename="fileComment")]
    pub file_comment: Option<FileComment>,
    /// This field is deprecated; please use the `drive` field instead.
    #[serde(rename="teamDrive")]
    pub team_drive: Option<TeamDrive>,
}

impl client::Part for Target {}


/// A lightweight reference to the target of activity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TargetReference {
    /// The target is a shared drive.
    pub drive: Option<DriveReference>,
    /// The target is a Drive item.
    #[serde(rename="driveItem")]
    pub drive_item: Option<DriveItemReference>,
    /// This field is deprecated; please use the `drive` field instead.
    #[serde(rename="teamDrive")]
    pub team_drive: Option<TeamDriveReference>,
}

impl client::Part for TargetReference {}


/// This item is deprecated; please see `Drive` instead.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TeamDrive {
    /// This field is deprecated; please see `Drive.name` instead.
    pub name: Option<String>,
    /// This field is deprecated; please see `Drive.root` instead.
    pub root: Option<DriveItem>,
    /// This field is deprecated; please see `Drive.title` instead.
    pub title: Option<String>,
}

impl client::Part for TeamDrive {}


/// This item is deprecated; please see `DriveReference` instead.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TeamDriveReference {
    /// This field is deprecated; please see `DriveReference.name` instead.
    pub name: Option<String>,
    /// This field is deprecated; please see `DriveReference.title` instead.
    pub title: Option<String>,
}

impl client::Part for TeamDriveReference {}


/// Information about time ranges.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeRange {
    /// The end of the time range.
    #[serde(rename="endTime")]
    pub end_time: Option<String>,
    /// The start of the time range.
    #[serde(rename="startTime")]
    pub start_time: Option<String>,
}

impl client::Part for TimeRange {}


/// A user about whom nothing is currently known.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UnknownUser { _never_set: Option<bool> }

impl client::Part for UnknownUser {}


/// An object was uploaded into Drive.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Upload { _never_set: Option<bool> }

impl client::Part for Upload {}


/// Information about an end user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct User {
    /// A user whose account has since been deleted.
    #[serde(rename="deletedUser")]
    pub deleted_user: Option<DeletedUser>,
    /// A known user.
    #[serde(rename="knownUser")]
    pub known_user: Option<KnownUser>,
    /// A user about whom nothing is currently known.
    #[serde(rename="unknownUser")]
    pub unknown_user: Option<UnknownUser>,
}

impl client::Part for User {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *activity* resources.
/// It is not used directly, but through the `DriveActivityHub` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_driveactivity2 as driveactivity2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use driveactivity2::DriveActivityHub;
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DriveActivityHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `query(...)`
/// // to build up your call.
/// let rb = hub.activity();
/// # }
/// ```
pub struct ActivityMethods<'a, C>
    where C: 'a {

    hub: &'a DriveActivityHub<C>,
}

impl<'a, C> client::MethodsBuilder for ActivityMethods<'a, C> {}

impl<'a, C> ActivityMethods<'a, C> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Query past activity in Google Drive.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn query(&self, request: QueryDriveActivityRequest) -> ActivityQueryCall<'a, C> {
        ActivityQueryCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Query past activity in Google Drive.
///
/// A builder for the *query* method supported by a *activity* resource.
/// It is not used directly, but through a `ActivityMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_driveactivity2 as driveactivity2;
/// use driveactivity2::api::QueryDriveActivityRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use driveactivity2::DriveActivityHub;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = DriveActivityHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = QueryDriveActivityRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.activity().query(req)
///              .doit().await;
/// # }
/// ```
pub struct ActivityQueryCall<'a, C>
    where C: 'a {

    hub: &'a DriveActivityHub<C>,
    _request: QueryDriveActivityRequest,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for ActivityQueryCall<'a, C> {}

impl<'a, C> ActivityQueryCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, QueryDriveActivityResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "driveactivity.activity.query",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
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

        let mut url = self.hub._base_url.clone() + "v2/activity:query";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveActivity.as_ref().to_string(), ());
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
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
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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
    pub fn request(mut self, new_value: QueryDriveActivityRequest) -> ActivityQueryCall<'a, C> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ActivityQueryCall<'a, C> {
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
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ActivityQueryCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::DriveActivity`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ActivityQueryCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


