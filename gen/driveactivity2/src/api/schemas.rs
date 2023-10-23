use super::*;
/// Information about the action.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
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
    
    pub timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for Action {}


/// Data describing the type and additional information of an action.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActionDetail {
    /// Label was changed.
    #[serde(rename="appliedLabelChange")]
    
    pub applied_label_change: Option<AppliedLabelChange>,
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
#[serde_with::serde_as(crate = "::client::serde_with")]
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
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Administrator { _never_set: Option<bool> }

impl client::Part for Administrator {}


/// Empty message representing an anonymous user or indicating the authenticated user should be anonymized.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnonymousUser { _never_set: Option<bool> }

impl client::Part for AnonymousUser {}


/// Represents any user (including a logged out user).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Anyone { _never_set: Option<bool> }

impl client::Part for Anyone {}


/// Activity in applications other than Drive.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApplicationReference {
    /// The reference type corresponding to this event.
    #[serde(rename="type")]
    
    pub type_: Option<ApplicationReferenceTypeEnum>,
}

impl client::Part for ApplicationReference {}


/// Label changes that were made on the Target.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppliedLabelChange {
    /// Changes that were made to the Label on the Target.
    
    pub changes: Option<Vec<AppliedLabelChangeDetail>>,
}

impl client::Part for AppliedLabelChange {}


/// A change made to a Label on the Target.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppliedLabelChangeDetail {
    /// Field Changes. Only present if `types` contains `LABEL_FIELD_VALUE_CHANGED`.
    #[serde(rename="fieldChanges")]
    
    pub field_changes: Option<Vec<FieldValueChange>>,
    /// The Label name representing the Label that changed. This name always contains the revision of the Label that was used when this Action occurred. The format is `labels/id@revision`.
    
    pub label: Option<String>,
    /// The human-readable title of the label that changed.
    
    pub title: Option<String>,
    /// The types of changes made to the Label on the Target.
    
    pub types: Option<Vec<AppliedLabelChangeDetailTypesEnum>>,
}

impl client::Part for AppliedLabelChangeDetail {}


/// A comment with an assignment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Assignment {
    /// The user to whom the comment was assigned.
    #[serde(rename="assignedUser")]
    
    pub assigned_user: Option<User>,
    /// The sub-type of this event.
    
    pub subtype: Option<AssignmentSubtypeEnum>,
}

impl client::Part for Assignment {}


/// A change about comments on an object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
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


/// How the individual activities are consolidated. If a set of activities is related they can be consolidated into one combined activity, such as one actor performing the same action on multiple targets, or multiple actors performing the same action on a single target. The strategy defines the rules for which activities are related.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
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
#[serde_with::serde_as(crate = "::client::serde_with")]
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
#[serde_with::serde_as(crate = "::client::serde_with")]
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
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataLeakPreventionChange {
    /// The type of Data Leak Prevention (DLP) change.
    #[serde(rename="type")]
    
    pub type_: Option<DataLeakPreventionChangeTypeEnum>,
}

impl client::Part for DataLeakPreventionChange {}


/// Wrapper for Date Field value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Date {
    /// Date value.
    
    pub value: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for Date {}


/// An object was deleted.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Delete {
    /// The type of delete action taken.
    #[serde(rename="type")]
    
    pub type_: Option<DeleteTypeEnum>,
}

impl client::Part for Delete {}


/// A user whose account has since been deleted.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeletedUser { _never_set: Option<bool> }

impl client::Part for DeletedUser {}


/// Information about a domain.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
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
#[serde_with::serde_as(crate = "::client::serde_with")]
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
#[serde_with::serde_as(crate = "::client::serde_with")]
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
    
    pub timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for DriveActivity {}


/// A Drive item which is a file.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DriveFile { _never_set: Option<bool> }

impl client::Part for DriveFile {}


/// A Drive item which is a folder.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DriveFolder {
    /// The type of Drive folder.
    #[serde(rename="type")]
    
    pub type_: Option<DriveFolderTypeEnum>,
}

impl client::Part for DriveFolder {}


/// A Drive item, such as a file or folder.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
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
#[serde_with::serde_as(crate = "::client::serde_with")]
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
#[serde_with::serde_as(crate = "::client::serde_with")]
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
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Edit { _never_set: Option<bool> }

impl client::Part for Edit {}


/// Contains a value of a Field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FieldValue {
    /// Date Field value.
    
    pub date: Option<Date>,
    /// Integer Field value.
    
    pub integer: Option<Integer>,
    /// Selection Field value.
    
    pub selection: Option<Selection>,
    /// Selection List Field value.
    #[serde(rename="selectionList")]
    
    pub selection_list: Option<SelectionList>,
    /// Text Field value.
    
    pub text: Option<Text>,
    /// Text List Field value.
    #[serde(rename="textList")]
    
    pub text_list: Option<TextList>,
    /// User Field value.
    
    pub user: Option<SingleUser>,
    /// User List Field value.
    #[serde(rename="userList")]
    
    pub user_list: Option<UserList>,
}

impl client::Part for FieldValue {}


/// Change to a Field value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FieldValueChange {
    /// The human-readable display name for this field.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The ID of this field. Field IDs are unique within a Label.
    #[serde(rename="fieldId")]
    
    pub field_id: Option<String>,
    /// The value that is now set on the field. If not present, the field was cleared. At least one of {old_value|new_value} is always set.
    #[serde(rename="newValue")]
    
    pub new_value: Option<FieldValue>,
    /// The value that was previously set on the field. If not present, the field was newly set. At least one of {old_value|new_value} is always set.
    #[serde(rename="oldValue")]
    
    pub old_value: Option<FieldValue>,
}

impl client::Part for FieldValueChange {}


/// This item is deprecated; please see `DriveFile` instead.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct File { _never_set: Option<bool> }

impl client::Part for File {}


/// A comment on a file.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
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
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Folder {
    /// This field is deprecated; please see `DriveFolder.type` instead.
    #[serde(rename="type")]
    
    pub type_: Option<FolderTypeEnum>,
}

impl client::Part for Folder {}


/// Information about a group.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
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
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Impersonation {
    /// The impersonated user.
    #[serde(rename="impersonatedUser")]
    
    pub impersonated_user: Option<User>,
}

impl client::Part for Impersonation {}


/// Wrapper for Integer Field value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Integer {
    /// Integer value.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub value: Option<i64>,
}

impl client::Part for Integer {}


/// A known user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
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


/// A strategy that consolidates activities using the grouping rules from the legacy V1 Activity API. Similar actions occurring within a window of time can be grouped across multiple targets (such as moving a set of files at once) or multiple actors (such as several users editing the same item). Grouping rules for this strategy are specific to each type of action.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Legacy { _never_set: Option<bool> }

impl client::Part for Legacy {}


/// An object was moved.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
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
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct New { _never_set: Option<bool> }

impl client::Part for New {}


/// A strategy that does no consolidation of individual activities.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NoConsolidation { _never_set: Option<bool> }

impl client::Part for NoConsolidation {}


/// Information about the owner of a Drive item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
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
#[serde_with::serde_as(crate = "::client::serde_with")]
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
    
    pub role: Option<PermissionRoleEnum>,
    /// The user to whom this permission applies.
    
    pub user: Option<User>,
}

impl client::Part for Permission {}


/// A change of the permission setting on an item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
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
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Post {
    /// The sub-type of this event.
    
    pub subtype: Option<PostSubtypeEnum>,
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
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryDriveActivityRequest {
    /// Return activities for this Drive folder, plus all children and descendants. The format is `items/ITEM_ID`.
    #[serde(rename="ancestorName")]
    
    pub ancestor_name: Option<String>,
    /// Details on how to consolidate related actions that make up the activity. If not set, then related actions aren't consolidated.
    #[serde(rename="consolidationStrategy")]
    
    pub consolidation_strategy: Option<ConsolidationStrategy>,
    /// The filtering for items returned from this query request. The format of the filter string is a sequence of expressions, joined by an optional "AND", where each expression is of the form "field operator value". Supported fields: - `time`: Uses numerical operators on date values either in terms of milliseconds since Jan 1, 1970 or in RFC 3339 format. Examples: - `time > 1452409200000 AND time <= 1492812924310` - `time >= "2016-01-10T01:02:03-05:00"` - `detail.action_detail_case`: Uses the "has" operator (:) and either a singular value or a list of allowed action types enclosed in parentheses, separated by a space. To exclude a result from the response, prepend a hyphen (`-`) to the beginning of the filter string. Examples: - `detail.action_detail_case:RENAME` - `detail.action_detail_case:(CREATE RESTORE)` - `-detail.action_detail_case:MOVE` 
    
    pub filter: Option<String>,
    /// Return activities for this Drive item. The format is `items/ITEM_ID`.
    #[serde(rename="itemName")]
    
    pub item_name: Option<String>,
    /// The minimum number of activities desired in the response; the server attempts to return at least this quantity. The server may also return fewer activities if it has a partial response ready before the request times out. If not set, a default value is used.
    #[serde(rename="pageSize")]
    
    pub page_size: Option<i32>,
    /// The token identifies which page of results to return. Set this to the next_page_token value returned from a previous query to obtain the following page of results. If not set, the first page of results is returned.
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
#[serde_with::serde_as(crate = "::client::serde_with")]
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
#[serde_with::serde_as(crate = "::client::serde_with")]
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
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Restore {
    /// The type of restore action taken.
    #[serde(rename="type")]
    
    pub type_: Option<RestoreTypeEnum>,
}

impl client::Part for Restore {}


/// Information about restriction policy changes to a feature.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RestrictionChange {
    /// The feature which had a change in restriction policy.
    
    pub feature: Option<RestrictionChangeFeatureEnum>,
    /// The restriction in place after the change.
    #[serde(rename="newRestriction")]
    
    pub new_restriction: Option<RestrictionChangeNewRestrictionEnum>,
}

impl client::Part for RestrictionChange {}


/// Wrapper for Selection Field value as combined value/display_name pair for selected choice.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Selection {
    /// Selection value as human-readable display string.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Selection value as Field Choice ID.
    
    pub value: Option<String>,
}

impl client::Part for Selection {}


/// Wrapper for SelectionList Field value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SelectionList {
    /// Selection values.
    
    pub values: Option<Vec<Selection>>,
}

impl client::Part for SelectionList {}


/// Information about settings changes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SettingsChange {
    /// The set of changes made to restrictions.
    #[serde(rename="restrictionChanges")]
    
    pub restriction_changes: Option<Vec<RestrictionChange>>,
}

impl client::Part for SettingsChange {}


/// Wrapper for User Field value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SingleUser {
    /// User value as email.
    
    pub value: Option<String>,
}

impl client::Part for SingleUser {}


/// A suggestion.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Suggestion {
    /// The sub-type of this event.
    
    pub subtype: Option<SuggestionSubtypeEnum>,
}

impl client::Part for Suggestion {}


/// Event triggered by system operations instead of end users.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SystemEvent {
    /// The type of the system event that may triggered activity.
    #[serde(rename="type")]
    
    pub type_: Option<SystemEventTypeEnum>,
}

impl client::Part for SystemEvent {}


/// Information about the target of activity. For more information on how activity history is shared with users, see [Activity history visibility](https://developers.google.com/drive/activity/v2#activityhistory).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
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
#[serde_with::serde_as(crate = "::client::serde_with")]
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
#[serde_with::serde_as(crate = "::client::serde_with")]
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
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TeamDriveReference {
    /// This field is deprecated; please see `DriveReference.name` instead.
    
    pub name: Option<String>,
    /// This field is deprecated; please see `DriveReference.title` instead.
    
    pub title: Option<String>,
}

impl client::Part for TeamDriveReference {}


/// Wrapper for Text Field value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Text {
    /// Value of Text Field.
    
    pub value: Option<String>,
}

impl client::Part for Text {}


/// Wrapper for Text List Field value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TextList {
    /// Text values.
    
    pub values: Option<Vec<Text>>,
}

impl client::Part for TextList {}


/// Information about time ranges.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeRange {
    /// The end of the time range.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The start of the time range.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for TimeRange {}


/// A user about whom nothing is currently known.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UnknownUser { _never_set: Option<bool> }

impl client::Part for UnknownUser {}


/// An object was uploaded into Drive.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Upload { _never_set: Option<bool> }

impl client::Part for Upload {}


/// Information about an end user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
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


/// Wrapper for UserList Field value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserList {
    /// User values.
    
    pub values: Option<Vec<SingleUser>>,
}

impl client::Part for UserList {}


