use super::*;
/// Represents a Google Tag Manager Account.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers environments create accounts](AccountContainerEnvironmentCreateCall) (none)
/// * [containers environments delete accounts](AccountContainerEnvironmentDeleteCall) (none)
/// * [containers environments get accounts](AccountContainerEnvironmentGetCall) (none)
/// * [containers environments list accounts](AccountContainerEnvironmentListCall) (none)
/// * [containers environments update accounts](AccountContainerEnvironmentUpdateCall) (none)
/// * [containers folders entities list accounts](AccountContainerFolderEntityListCall) (none)
/// * [containers folders create accounts](AccountContainerFolderCreateCall) (none)
/// * [containers folders delete accounts](AccountContainerFolderDeleteCall) (none)
/// * [containers folders get accounts](AccountContainerFolderGetCall) (none)
/// * [containers folders list accounts](AccountContainerFolderListCall) (none)
/// * [containers folders update accounts](AccountContainerFolderUpdateCall) (none)
/// * [containers move_folders update accounts](AccountContainerMoveFolderUpdateCall) (none)
/// * [containers reauthorize_environments update accounts](AccountContainerReauthorizeEnvironmentUpdateCall) (none)
/// * [containers tags create accounts](AccountContainerTagCreateCall) (none)
/// * [containers tags delete accounts](AccountContainerTagDeleteCall) (none)
/// * [containers tags get accounts](AccountContainerTagGetCall) (none)
/// * [containers tags list accounts](AccountContainerTagListCall) (none)
/// * [containers tags update accounts](AccountContainerTagUpdateCall) (none)
/// * [containers triggers create accounts](AccountContainerTriggerCreateCall) (none)
/// * [containers triggers delete accounts](AccountContainerTriggerDeleteCall) (none)
/// * [containers triggers get accounts](AccountContainerTriggerGetCall) (none)
/// * [containers triggers list accounts](AccountContainerTriggerListCall) (none)
/// * [containers triggers update accounts](AccountContainerTriggerUpdateCall) (none)
/// * [containers variables create accounts](AccountContainerVariableCreateCall) (none)
/// * [containers variables delete accounts](AccountContainerVariableDeleteCall) (none)
/// * [containers variables get accounts](AccountContainerVariableGetCall) (none)
/// * [containers variables list accounts](AccountContainerVariableListCall) (none)
/// * [containers variables update accounts](AccountContainerVariableUpdateCall) (none)
/// * [containers versions create accounts](AccountContainerVersionCreateCall) (none)
/// * [containers versions delete accounts](AccountContainerVersionDeleteCall) (none)
/// * [containers versions get accounts](AccountContainerVersionGetCall) (none)
/// * [containers versions list accounts](AccountContainerVersionListCall) (none)
/// * [containers versions publish accounts](AccountContainerVersionPublishCall) (none)
/// * [containers versions restore accounts](AccountContainerVersionRestoreCall) (none)
/// * [containers versions undelete accounts](AccountContainerVersionUndeleteCall) (none)
/// * [containers versions update accounts](AccountContainerVersionUpdateCall) (none)
/// * [containers create accounts](AccountContainerCreateCall) (none)
/// * [containers delete accounts](AccountContainerDeleteCall) (none)
/// * [containers get accounts](AccountContainerGetCall) (none)
/// * [containers list accounts](AccountContainerListCall) (none)
/// * [containers update accounts](AccountContainerUpdateCall) (none)
/// * [permissions create accounts](AccountPermissionCreateCall) (none)
/// * [permissions delete accounts](AccountPermissionDeleteCall) (none)
/// * [permissions get accounts](AccountPermissionGetCall) (none)
/// * [permissions list accounts](AccountPermissionListCall) (none)
/// * [permissions update accounts](AccountPermissionUpdateCall) (none)
/// * [get accounts](AccountGetCall) (response)
/// * [list accounts](AccountListCall) (none)
/// * [update accounts](AccountUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    /// The Account ID uniquely identifies the GTM Account.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// The fingerprint of the GTM Account as computed at storage time. This value is recomputed whenever the account is modified.
    
    pub fingerprint: Option<String>,
    /// Account display name. @mutable tagmanager.accounts.create @mutable tagmanager.accounts.update
    
    pub name: Option<String>,
    /// Whether the account shares data anonymously with Google and others. @mutable tagmanager.accounts.create @mutable tagmanager.accounts.update
    #[serde(rename="shareData")]
    
    pub share_data: Option<bool>,
}

impl client::RequestValue for Account {}
impl client::Resource for Account {}
impl client::ResponseResult for Account {}


/// Defines the Google Tag Manager Account access permissions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountAccess {
    /// List of Account permissions. Valid account permissions are read and manage. @mutable tagmanager.accounts.permissions.create @mutable tagmanager.accounts.permissions.update
    
    pub permission: Option<Vec<AccountAccesPermissionEnum>>,
}

impl client::Part for AccountAccess {}


/// Represents a predicate.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Condition {
    /// A list of named parameters (key/value), depending on the condition's type. Notes: - For binary operators, include parameters named arg0 and arg1 for specifying the left and right operands, respectively. - At this time, the left operand (arg0) must be a reference to a variable. - For case-insensitive Regex matching, include a boolean parameter named ignore_case that is set to true. If not specified or set to any other value, the matching will be case sensitive. - To negate an operator, include a boolean parameter named negate boolean parameter that is set to true. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update
    
    pub parameter: Option<Vec<Parameter>>,
    /// The type of operator for this condition. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update
    #[serde(rename="type")]
    
    pub type_: Option<ConditionTypeEnum>,
}

impl client::Part for Condition {}


/// Represents a Google Tag Manager Container.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers create accounts](AccountContainerCreateCall) (request|response)
/// * [containers get accounts](AccountContainerGetCall) (response)
/// * [containers update accounts](AccountContainerUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Container {
    /// GTM Account ID.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// The Container ID uniquely identifies the GTM Container.
    #[serde(rename="containerId")]
    
    pub container_id: Option<String>,
    /// Optional list of domain names associated with the Container. @mutable tagmanager.accounts.containers.create @mutable tagmanager.accounts.containers.update
    #[serde(rename="domainName")]
    
    pub domain_name: Option<Vec<String>>,
    /// List of enabled built-in variables. Valid values include: pageUrl, pageHostname, pagePath, referrer, event, clickElement, clickClasses, clickId, clickTarget, clickUrl, clickText, formElement, formClasses, formId, formTarget, formUrl, formText, errorMessage, errorUrl, errorLine, newHistoryFragment, oldHistoryFragment, newHistoryState, oldHistoryState, historySource, containerVersion, debugMode, randomNumber, containerId. @mutable tagmanager.accounts.containers.create @mutable tagmanager.accounts.containers.update
    #[serde(rename="enabledBuiltInVariable")]
    
    pub enabled_built_in_variable: Option<Vec<ContainerEnabledBuiltInVariableEnum>>,
    /// The fingerprint of the GTM Container as computed at storage time. This value is recomputed whenever the account is modified.
    
    pub fingerprint: Option<String>,
    /// Container display name. @mutable tagmanager.accounts.containers.create @mutable tagmanager.accounts.containers.update
    
    pub name: Option<String>,
    /// Container Notes. @mutable tagmanager.accounts.containers.create @mutable tagmanager.accounts.containers.update
    
    pub notes: Option<String>,
    /// Container Public ID.
    #[serde(rename="publicId")]
    
    pub public_id: Option<String>,
    /// Container Country ID. @mutable tagmanager.accounts.containers.create @mutable tagmanager.accounts.containers.update
    #[serde(rename="timeZoneCountryId")]
    
    pub time_zone_country_id: Option<String>,
    /// Container Time Zone ID. @mutable tagmanager.accounts.containers.create @mutable tagmanager.accounts.containers.update
    #[serde(rename="timeZoneId")]
    
    pub time_zone_id: Option<String>,
    /// List of Usage Contexts for the Container. Valid values include: web, android, ios. @mutable tagmanager.accounts.containers.create @mutable tagmanager.accounts.containers.update
    #[serde(rename="usageContext")]
    
    pub usage_context: Option<Vec<ContainerUsageContextEnum>>,
}

impl client::RequestValue for Container {}
impl client::ResponseResult for Container {}


/// Defines the Google Tag Manager Container access permissions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContainerAccess {
    /// GTM Container ID. @mutable tagmanager.accounts.permissions.create @mutable tagmanager.accounts.permissions.update
    #[serde(rename="containerId")]
    
    pub container_id: Option<String>,
    /// List of Container permissions. Valid container permissions are: read, edit, delete, publish. @mutable tagmanager.accounts.permissions.create @mutable tagmanager.accounts.permissions.update
    
    pub permission: Option<Vec<ContainerAccesPermissionEnum>>,
}

impl client::Part for ContainerAccess {}


/// Represents a Google Tag Manager Container Version.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers versions get accounts](AccountContainerVersionGetCall) (response)
/// * [containers versions restore accounts](AccountContainerVersionRestoreCall) (response)
/// * [containers versions undelete accounts](AccountContainerVersionUndeleteCall) (response)
/// * [containers versions update accounts](AccountContainerVersionUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContainerVersion {
    /// GTM Account ID.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// The container that this version was taken from.
    
    pub container: Option<Container>,
    /// GTM Container ID.
    #[serde(rename="containerId")]
    
    pub container_id: Option<String>,
    /// The Container Version ID uniquely identifies the GTM Container Version.
    #[serde(rename="containerVersionId")]
    
    pub container_version_id: Option<String>,
    /// A value of true indicates this container version has been deleted.
    
    pub deleted: Option<bool>,
    /// The fingerprint of the GTM Container Version as computed at storage time. This value is recomputed whenever the container version is modified.
    
    pub fingerprint: Option<String>,
    /// The folders in the container that this version was taken from.
    
    pub folder: Option<Vec<Folder>>,
    /// The macros in the container that this version was taken from.
    #[serde(rename="macro")]
    
    pub macro_: Option<Vec<Macro>>,
    /// Container version display name. @mutable tagmanager.accounts.containers.versions.update
    
    pub name: Option<String>,
    /// User notes on how to apply this container version in the container. @mutable tagmanager.accounts.containers.versions.update
    
    pub notes: Option<String>,
    /// The rules in the container that this version was taken from.
    
    pub rule: Option<Vec<Rule>>,
    /// The tags in the container that this version was taken from.
    
    pub tag: Option<Vec<Tag>>,
    /// The triggers in the container that this version was taken from.
    
    pub trigger: Option<Vec<Trigger>>,
    /// The variables in the container that this version was taken from.
    
    pub variable: Option<Vec<Variable>>,
}

impl client::RequestValue for ContainerVersion {}
impl client::ResponseResult for ContainerVersion {}


/// Represents a Google Tag Manager Container Version Header.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContainerVersionHeader {
    /// GTM Account ID.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// GTM Container ID.
    #[serde(rename="containerId")]
    
    pub container_id: Option<String>,
    /// The Container Version ID uniquely identifies the GTM Container Version.
    #[serde(rename="containerVersionId")]
    
    pub container_version_id: Option<String>,
    /// A value of true indicates this container version has been deleted.
    
    pub deleted: Option<bool>,
    /// Container version display name.
    
    pub name: Option<String>,
    /// Number of macros in the container version.
    #[serde(rename="numMacros")]
    
    pub num_macros: Option<String>,
    /// Number of rules in the container version.
    #[serde(rename="numRules")]
    
    pub num_rules: Option<String>,
    /// Number of tags in the container version.
    #[serde(rename="numTags")]
    
    pub num_tags: Option<String>,
    /// Number of triggers in the container version.
    #[serde(rename="numTriggers")]
    
    pub num_triggers: Option<String>,
    /// Number of variables in the container version.
    #[serde(rename="numVariables")]
    
    pub num_variables: Option<String>,
}

impl client::Part for ContainerVersionHeader {}


/// Options for new container versions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers versions create accounts](AccountContainerVersionCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateContainerVersionRequestVersionOptions {
    /// The name of the container version to be created.
    
    pub name: Option<String>,
    /// The notes of the container version to be created.
    
    pub notes: Option<String>,
    /// The creation of this version may be for quick preview and shouldn't be saved.
    #[serde(rename="quickPreview")]
    
    pub quick_preview: Option<bool>,
}

impl client::RequestValue for CreateContainerVersionRequestVersionOptions {}


/// Create container versions response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers versions create accounts](AccountContainerVersionCreateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateContainerVersionResponse {
    /// Compiler errors or not.
    #[serde(rename="compilerError")]
    
    pub compiler_error: Option<bool>,
    /// The container version created.
    #[serde(rename="containerVersion")]
    
    pub container_version: Option<ContainerVersion>,
}

impl client::ResponseResult for CreateContainerVersionResponse {}


/// Represents a Google Tag Manager Environment. Note that a user can create, delete and update environments of type USER, but can only update the enable_debug and url fields of environments of other types.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers environments create accounts](AccountContainerEnvironmentCreateCall) (request|response)
/// * [containers environments get accounts](AccountContainerEnvironmentGetCall) (response)
/// * [containers environments update accounts](AccountContainerEnvironmentUpdateCall) (request|response)
/// * [containers reauthorize_environments update accounts](AccountContainerReauthorizeEnvironmentUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Environment {
    /// GTM Account ID.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// The environment authorization code.
    #[serde(rename="authorizationCode")]
    
    pub authorization_code: Option<String>,
    /// The last update time-stamp for the authorization code.
    #[serde(rename="authorizationTimestampMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub authorization_timestamp_ms: Option<i64>,
    /// GTM Container ID.
    #[serde(rename="containerId")]
    
    pub container_id: Option<String>,
    /// no description provided
    #[serde(rename="containerVersionId")]
    
    pub container_version_id: Option<String>,
    /// The environment description. Can be set or changed only on USER type environments. @mutable tagmanager.accounts.containers.environments.create @mutable tagmanager.accounts.containers.environments.update
    
    pub description: Option<String>,
    /// Whether or not to enable debug by default on for the environment. @mutable tagmanager.accounts.containers.environments.create @mutable tagmanager.accounts.containers.environments.update
    #[serde(rename="enableDebug")]
    
    pub enable_debug: Option<bool>,
    /// GTM Environment ID uniquely identifies the GTM Environment.
    #[serde(rename="environmentId")]
    
    pub environment_id: Option<String>,
    /// The fingerprint of the GTM environment as computed at storage time. This value is recomputed whenever the environment is modified.
    
    pub fingerprint: Option<String>,
    /// The environment display name. Can be set or changed only on USER type environments. @mutable tagmanager.accounts.containers.environments.create @mutable tagmanager.accounts.containers.environments.update
    
    pub name: Option<String>,
    /// The type of this environment.
    #[serde(rename="type")]
    
    pub type_: Option<EnvironmentTypeEnum>,
    /// Default preview page url for the environment. @mutable tagmanager.accounts.containers.environments.create @mutable tagmanager.accounts.containers.environments.update
    
    pub url: Option<String>,
}

impl client::RequestValue for Environment {}
impl client::ResponseResult for Environment {}


/// Represents a Google Tag Manager Folder.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers folders create accounts](AccountContainerFolderCreateCall) (request|response)
/// * [containers folders get accounts](AccountContainerFolderGetCall) (response)
/// * [containers folders update accounts](AccountContainerFolderUpdateCall) (request|response)
/// * [containers move_folders update accounts](AccountContainerMoveFolderUpdateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Folder {
    /// GTM Account ID.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// GTM Container ID.
    #[serde(rename="containerId")]
    
    pub container_id: Option<String>,
    /// The fingerprint of the GTM Folder as computed at storage time. This value is recomputed whenever the folder is modified.
    
    pub fingerprint: Option<String>,
    /// The Folder ID uniquely identifies the GTM Folder.
    #[serde(rename="folderId")]
    
    pub folder_id: Option<String>,
    /// Folder display name. @mutable tagmanager.accounts.containers.folders.create @mutable tagmanager.accounts.containers.folders.update
    
    pub name: Option<String>,
}

impl client::RequestValue for Folder {}
impl client::ResponseResult for Folder {}


/// Represents a Google Tag Manager Folder’s contents.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers folders entities list accounts](AccountContainerFolderEntityListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FolderEntities {
    /// The list of tags inside the folder.
    
    pub tag: Option<Vec<Tag>>,
    /// The list of triggers inside the folder.
    
    pub trigger: Option<Vec<Trigger>>,
    /// The list of variables inside the folder.
    
    pub variable: Option<Vec<Variable>>,
}

impl client::ResponseResult for FolderEntities {}


/// List AccountUsers Response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [permissions list accounts](AccountPermissionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAccountUsersResponse {
    /// All GTM AccountUsers of a GTM Account.
    #[serde(rename="userAccess")]
    
    pub user_access: Option<Vec<UserAccess>>,
}

impl client::ResponseResult for ListAccountUsersResponse {}


/// List Accounts Response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list accounts](AccountListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAccountsResponse {
    /// List of GTM Accounts that a user has access to.
    
    pub accounts: Option<Vec<Account>>,
}

impl client::ResponseResult for ListAccountsResponse {}


/// List container versions response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers versions list accounts](AccountContainerVersionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListContainerVersionsResponse {
    /// All versions of a GTM Container.
    #[serde(rename="containerVersion")]
    
    pub container_version: Option<Vec<ContainerVersion>>,
    /// All container version headers of a GTM Container.
    #[serde(rename="containerVersionHeader")]
    
    pub container_version_header: Option<Vec<ContainerVersionHeader>>,
}

impl client::ResponseResult for ListContainerVersionsResponse {}


/// List Containers Response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers list accounts](AccountContainerListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListContainersResponse {
    /// All Containers of a GTM Account.
    
    pub containers: Option<Vec<Container>>,
}

impl client::ResponseResult for ListContainersResponse {}


/// List Environments Response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers environments list accounts](AccountContainerEnvironmentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListEnvironmentsResponse {
    /// All Environments of a GTM Container.
    
    pub environments: Option<Vec<Environment>>,
}

impl client::ResponseResult for ListEnvironmentsResponse {}


/// List Folders Response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers folders list accounts](AccountContainerFolderListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListFoldersResponse {
    /// All GTM Folders of a GTM Container.
    
    pub folders: Option<Vec<Folder>>,
}

impl client::ResponseResult for ListFoldersResponse {}


/// List Tags Response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers tags list accounts](AccountContainerTagListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTagsResponse {
    /// All GTM Tags of a GTM Container.
    
    pub tags: Option<Vec<Tag>>,
}

impl client::ResponseResult for ListTagsResponse {}


/// List triggers response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers triggers list accounts](AccountContainerTriggerListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTriggersResponse {
    /// All GTM Triggers of a GTM Container.
    
    pub triggers: Option<Vec<Trigger>>,
}

impl client::ResponseResult for ListTriggersResponse {}


/// List Variables Response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers variables list accounts](AccountContainerVariableListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListVariablesResponse {
    /// All GTM Variables of a GTM Container.
    
    pub variables: Option<Vec<Variable>>,
}

impl client::ResponseResult for ListVariablesResponse {}


/// Represents a Google Tag Manager Macro.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Macro {
    /// GTM Account ID.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// GTM Container ID.
    #[serde(rename="containerId")]
    
    pub container_id: Option<String>,
    /// For mobile containers only: A list of rule IDs for disabling conditional macros; the macro is enabled if one of the enabling rules is true while all the disabling rules are false. Treated as an unordered set. @mutable tagmanager.accounts.containers.macros.create @mutable tagmanager.accounts.containers.macros.update
    #[serde(rename="disablingRuleId")]
    
    pub disabling_rule_id: Option<Vec<String>>,
    /// For mobile containers only: A list of rule IDs for enabling conditional macros; the macro is enabled if one of the enabling rules is true while all the disabling rules are false. Treated as an unordered set. @mutable tagmanager.accounts.containers.macros.create @mutable tagmanager.accounts.containers.macros.update
    #[serde(rename="enablingRuleId")]
    
    pub enabling_rule_id: Option<Vec<String>>,
    /// The fingerprint of the GTM Macro as computed at storage time. This value is recomputed whenever the macro is modified.
    
    pub fingerprint: Option<String>,
    /// The Macro ID uniquely identifies the GTM Macro.
    #[serde(rename="macroId")]
    
    pub macro_id: Option<String>,
    /// Macro display name. @mutable tagmanager.accounts.containers.macros.create @mutable tagmanager.accounts.containers.macros.update
    
    pub name: Option<String>,
    /// User notes on how to apply this macro in the container. @mutable tagmanager.accounts.containers.macros.create @mutable tagmanager.accounts.containers.macros.update
    
    pub notes: Option<String>,
    /// The macro's parameters. @mutable tagmanager.accounts.containers.macros.create @mutable tagmanager.accounts.containers.macros.update
    
    pub parameter: Option<Vec<Parameter>>,
    /// Parent folder id.
    #[serde(rename="parentFolderId")]
    
    pub parent_folder_id: Option<String>,
    /// The end timestamp in milliseconds to schedule a macro. @mutable tagmanager.accounts.containers.macros.create @mutable tagmanager.accounts.containers.macros.update
    #[serde(rename="scheduleEndMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub schedule_end_ms: Option<i64>,
    /// The start timestamp in milliseconds to schedule a macro. @mutable tagmanager.accounts.containers.macros.create @mutable tagmanager.accounts.containers.macros.update
    #[serde(rename="scheduleStartMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub schedule_start_ms: Option<i64>,
    /// GTM Macro Type. @mutable tagmanager.accounts.containers.macros.create @mutable tagmanager.accounts.containers.macros.update
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for Macro {}


/// Represents a Google Tag Manager Parameter.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Parameter {
    /// The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values. @mutable tagmanager.accounts.containers.variables.create @mutable tagmanager.accounts.containers.variables.update @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update
    
    pub key: Option<String>,
    /// This list parameter's parameters (keys will be ignored). @mutable tagmanager.accounts.containers.variables.create @mutable tagmanager.accounts.containers.variables.update @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update
    
    pub list: Option<Vec<Parameter>>,
    /// This map parameter's parameters (must have keys; keys must be unique). @mutable tagmanager.accounts.containers.variables.create @mutable tagmanager.accounts.containers.variables.update @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update
    
    pub map: Option<Vec<Parameter>>,
    /// The parameter type. Valid values are: - boolean: The value represents a boolean, represented as 'true' or 'false' - integer: The value represents a 64-bit signed integer value, in base 10 - list: A list of parameters should be specified - map: A map of parameters should be specified - template: The value represents any text; this can include variable references (even variable references that might return non-string types) - trigger_reference: The value represents a trigger, represented as the trigger id - tag_reference: The value represents a tag, represented as the tag name @mutable tagmanager.accounts.containers.variables.create @mutable tagmanager.accounts.containers.variables.update @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update
    #[serde(rename="type")]
    
    pub type_: Option<ParameterTypeEnum>,
    /// A parameter's value (may contain variable references such as "{{myVariable}}") as appropriate to the specified type. @mutable tagmanager.accounts.containers.variables.create @mutable tagmanager.accounts.containers.variables.update @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update
    
    pub value: Option<String>,
}

impl client::Part for Parameter {}


/// Publish container version response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers versions publish accounts](AccountContainerVersionPublishCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PublishContainerVersionResponse {
    /// Compiler errors or not.
    #[serde(rename="compilerError")]
    
    pub compiler_error: Option<bool>,
    /// The container version created.
    #[serde(rename="containerVersion")]
    
    pub container_version: Option<ContainerVersion>,
}

impl client::ResponseResult for PublishContainerVersionResponse {}


/// Represents a Google Tag Manager Rule.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Rule {
    /// GTM Account ID.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// The list of conditions that make up this rule (implicit AND between them). @mutable tagmanager.accounts.containers.rules.create @mutable tagmanager.accounts.containers.rules.update
    
    pub condition: Option<Vec<Condition>>,
    /// GTM Container ID.
    #[serde(rename="containerId")]
    
    pub container_id: Option<String>,
    /// The fingerprint of the GTM Rule as computed at storage time. This value is recomputed whenever the rule is modified.
    
    pub fingerprint: Option<String>,
    /// Rule display name. @mutable tagmanager.accounts.containers.rules.create @mutable tagmanager.accounts.containers.rules.update
    
    pub name: Option<String>,
    /// User notes on how to apply this rule in the container. @mutable tagmanager.accounts.containers.rules.create @mutable tagmanager.accounts.containers.rules.update
    
    pub notes: Option<String>,
    /// The Rule ID uniquely identifies the GTM Rule.
    #[serde(rename="ruleId")]
    
    pub rule_id: Option<String>,
}

impl client::Part for Rule {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetupTag {
    /// If true, fire the main tag if and only if the setup tag fires successfully. If false, fire the main tag regardless of setup tag firing status.
    #[serde(rename="stopOnSetupFailure")]
    
    pub stop_on_setup_failure: Option<bool>,
    /// The name of the setup tag.
    #[serde(rename="tagName")]
    
    pub tag_name: Option<String>,
}

impl client::Part for SetupTag {}


/// Represents a Google Tag Manager Tag.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers tags create accounts](AccountContainerTagCreateCall) (request|response)
/// * [containers tags get accounts](AccountContainerTagGetCall) (response)
/// * [containers tags update accounts](AccountContainerTagUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Tag {
    /// GTM Account ID.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// Blocking rule IDs. If any of the listed rules evaluate to true, the tag will not fire. @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update
    #[serde(rename="blockingRuleId")]
    
    pub blocking_rule_id: Option<Vec<String>>,
    /// Blocking trigger IDs. If any of the listed triggers evaluate to true, the tag will not fire. @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update
    #[serde(rename="blockingTriggerId")]
    
    pub blocking_trigger_id: Option<Vec<String>>,
    /// GTM Container ID.
    #[serde(rename="containerId")]
    
    pub container_id: Option<String>,
    /// The fingerprint of the GTM Tag as computed at storage time. This value is recomputed whenever the tag is modified.
    
    pub fingerprint: Option<String>,
    /// Firing rule IDs. A tag will fire when any of the listed rules are true and all of its blockingRuleIds (if any specified) are false. @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update
    #[serde(rename="firingRuleId")]
    
    pub firing_rule_id: Option<Vec<String>>,
    /// Firing trigger IDs. A tag will fire when any of the listed triggers are true and all of its blockingTriggerIds (if any specified) are false. @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update
    #[serde(rename="firingTriggerId")]
    
    pub firing_trigger_id: Option<Vec<String>>,
    /// If set to true, this tag will only fire in the live environment (e.g. not in preview or debug mode). @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update
    #[serde(rename="liveOnly")]
    
    pub live_only: Option<bool>,
    /// Tag display name. @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update
    
    pub name: Option<String>,
    /// User notes on how to apply this tag in the container. @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update
    
    pub notes: Option<String>,
    /// The tag's parameters. @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update
    
    pub parameter: Option<Vec<Parameter>>,
    /// Parent folder id.
    #[serde(rename="parentFolderId")]
    
    pub parent_folder_id: Option<String>,
    /// True if the tag is paused. @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update
    
    pub paused: Option<bool>,
    /// User defined numeric priority of the tag. Tags are fired asynchronously in order of priority. Tags with higher numeric value fire first. A tag's priority can be a positive or negative value. The default value is 0. @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update
    
    pub priority: Option<Parameter>,
    /// The end timestamp in milliseconds to schedule a tag. @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update
    #[serde(rename="scheduleEndMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub schedule_end_ms: Option<i64>,
    /// The start timestamp in milliseconds to schedule a tag. @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update
    #[serde(rename="scheduleStartMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub schedule_start_ms: Option<i64>,
    /// The list of setup tags. Currently we only allow one.
    #[serde(rename="setupTag")]
    
    pub setup_tag: Option<Vec<SetupTag>>,
    /// Option to fire this tag.
    #[serde(rename="tagFiringOption")]
    
    pub tag_firing_option: Option<TagTagFiringOptionEnum>,
    /// The Tag ID uniquely identifies the GTM Tag.
    #[serde(rename="tagId")]
    
    pub tag_id: Option<String>,
    /// The list of teardown tags. Currently we only allow one.
    #[serde(rename="teardownTag")]
    
    pub teardown_tag: Option<Vec<TeardownTag>>,
    /// GTM Tag Type. @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::RequestValue for Tag {}
impl client::ResponseResult for Tag {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TeardownTag {
    /// If true, fire the teardown tag if and only if the main tag fires successfully. If false, fire the teardown tag regardless of main tag firing status.
    #[serde(rename="stopTeardownOnFailure")]
    
    pub stop_teardown_on_failure: Option<bool>,
    /// The name of the teardown tag.
    #[serde(rename="tagName")]
    
    pub tag_name: Option<String>,
}

impl client::Part for TeardownTag {}


/// Represents a Google Tag Manager Trigger
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers triggers create accounts](AccountContainerTriggerCreateCall) (request|response)
/// * [containers triggers get accounts](AccountContainerTriggerGetCall) (response)
/// * [containers triggers update accounts](AccountContainerTriggerUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Trigger {
    /// GTM Account ID.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// Used in the case of auto event tracking. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update
    #[serde(rename="autoEventFilter")]
    
    pub auto_event_filter: Option<Vec<Condition>>,
    /// Whether or not we should only fire tags if the form submit or link click event is not cancelled by some other event handler (e.g. because of validation). Only valid for Form Submission and Link Click triggers. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update
    #[serde(rename="checkValidation")]
    
    pub check_validation: Option<Parameter>,
    /// GTM Container ID.
    #[serde(rename="containerId")]
    
    pub container_id: Option<String>,
    /// A visibility trigger minimum continuous visible time (in milliseconds). Only valid for AMP Visibility trigger. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update
    #[serde(rename="continuousTimeMinMilliseconds")]
    
    pub continuous_time_min_milliseconds: Option<Parameter>,
    /// Used in the case of custom event, which is fired iff all Conditions are true. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update
    #[serde(rename="customEventFilter")]
    
    pub custom_event_filter: Option<Vec<Condition>>,
    /// Name of the GTM event that is fired. Only valid for Timer triggers. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update
    #[serde(rename="eventName")]
    
    pub event_name: Option<Parameter>,
    /// The trigger will only fire iff all Conditions are true. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update
    
    pub filter: Option<Vec<Condition>>,
    /// The fingerprint of the GTM Trigger as computed at storage time. This value is recomputed whenever the trigger is modified.
    
    pub fingerprint: Option<String>,
    /// List of integer percentage values for scroll triggers. The trigger will fire when each percentage is reached when the view is scrolled horizontally. Only valid for AMP scroll triggers. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update
    #[serde(rename="horizontalScrollPercentageList")]
    
    pub horizontal_scroll_percentage_list: Option<Parameter>,
    /// Time between triggering recurring Timer Events (in milliseconds). Only valid for Timer triggers. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update
    
    pub interval: Option<Parameter>,
    /// Time between Timer Events to fire (in seconds). Only valid for AMP Timer trigger. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update
    #[serde(rename="intervalSeconds")]
    
    pub interval_seconds: Option<Parameter>,
    /// Limit of the number of GTM events this Timer Trigger will fire. If no limit is set, we will continue to fire GTM events until the user leaves the page. Only valid for Timer triggers. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update
    
    pub limit: Option<Parameter>,
    /// Max time to fire Timer Events (in seconds). Only valid for AMP Timer trigger. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update
    #[serde(rename="maxTimerLengthSeconds")]
    
    pub max_timer_length_seconds: Option<Parameter>,
    /// Trigger display name. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update
    
    pub name: Option<String>,
    /// Additional parameters. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update
    
    pub parameter: Option<Vec<Parameter>>,
    /// Parent folder id.
    #[serde(rename="parentFolderId")]
    
    pub parent_folder_id: Option<String>,
    /// A click trigger CSS selector (i.e. "a", "button" etc.). Only valid for AMP Click trigger. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update
    
    pub selector: Option<Parameter>,
    /// A visibility trigger minimum total visible time (in milliseconds). Only valid for AMP Visibility trigger. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update
    #[serde(rename="totalTimeMinMilliseconds")]
    
    pub total_time_min_milliseconds: Option<Parameter>,
    /// The Trigger ID uniquely identifies the GTM Trigger.
    #[serde(rename="triggerId")]
    
    pub trigger_id: Option<String>,
    /// Defines the data layer event that causes this trigger. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update
    #[serde(rename="type")]
    
    pub type_: Option<TriggerTypeEnum>,
    /// Globally unique id of the trigger that auto-generates this (a Form Submit, Link Click or Timer listener) if any. Used to make incompatible auto-events work together with trigger filtering based on trigger ids. This value is populated during output generation since the tags implied by triggers don't exist until then. Only valid for Form Submit, Link Click and Timer triggers. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update
    #[serde(rename="uniqueTriggerId")]
    
    pub unique_trigger_id: Option<Parameter>,
    /// List of integer percentage values for scroll triggers. The trigger will fire when each percentage is reached when the view is scrolled vertically. Only valid for AMP scroll triggers. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update
    #[serde(rename="verticalScrollPercentageList")]
    
    pub vertical_scroll_percentage_list: Option<Parameter>,
    /// A visibility trigger CSS selector (i.e. "#id"). Only valid for AMP Visibility trigger. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update
    #[serde(rename="visibilitySelector")]
    
    pub visibility_selector: Option<Parameter>,
    /// A visibility trigger maximum percent visibility. Only valid for AMP Visibility trigger. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update
    #[serde(rename="visiblePercentageMax")]
    
    pub visible_percentage_max: Option<Parameter>,
    /// A visibility trigger minimum percent visibility. Only valid for AMP Visibility trigger. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update
    #[serde(rename="visiblePercentageMin")]
    
    pub visible_percentage_min: Option<Parameter>,
    /// Whether or not we should delay the form submissions or link opening until all of the tags have fired (by preventing the default action and later simulating the default action). Only valid for Form Submission and Link Click triggers. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update
    #[serde(rename="waitForTags")]
    
    pub wait_for_tags: Option<Parameter>,
    /// How long to wait (in milliseconds) for tags to fire when 'waits_for_tags' above evaluates to true. Only valid for Form Submission and Link Click triggers. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update
    #[serde(rename="waitForTagsTimeout")]
    
    pub wait_for_tags_timeout: Option<Parameter>,
}

impl client::RequestValue for Trigger {}
impl client::ResponseResult for Trigger {}


/// Represents a user’s permissions to an account and its container.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [permissions create accounts](AccountPermissionCreateCall) (request|response)
/// * [permissions get accounts](AccountPermissionGetCall) (response)
/// * [permissions update accounts](AccountPermissionUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserAccess {
    /// GTM Account access permissions. @mutable tagmanager.accounts.permissions.create @mutable tagmanager.accounts.permissions.update
    #[serde(rename="accountAccess")]
    
    pub account_access: Option<AccountAccess>,
    /// GTM Account ID.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// GTM Container access permissions. @mutable tagmanager.accounts.permissions.create @mutable tagmanager.accounts.permissions.update
    #[serde(rename="containerAccess")]
    
    pub container_access: Option<Vec<ContainerAccess>>,
    /// User's email address. @mutable tagmanager.accounts.permissions.create
    #[serde(rename="emailAddress")]
    
    pub email_address: Option<String>,
    /// Account Permission ID.
    #[serde(rename="permissionId")]
    
    pub permission_id: Option<String>,
}

impl client::RequestValue for UserAccess {}
impl client::ResponseResult for UserAccess {}


/// Represents a Google Tag Manager Variable.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers variables create accounts](AccountContainerVariableCreateCall) (request|response)
/// * [containers variables get accounts](AccountContainerVariableGetCall) (response)
/// * [containers variables update accounts](AccountContainerVariableUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Variable {
    /// GTM Account ID.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// GTM Container ID.
    #[serde(rename="containerId")]
    
    pub container_id: Option<String>,
    /// For mobile containers only: A list of trigger IDs for disabling conditional variables; the variable is enabled if one of the enabling trigger is true while all the disabling trigger are false. Treated as an unordered set. @mutable tagmanager.accounts.containers.variables.create @mutable tagmanager.accounts.containers.variables.update
    #[serde(rename="disablingTriggerId")]
    
    pub disabling_trigger_id: Option<Vec<String>>,
    /// For mobile containers only: A list of trigger IDs for enabling conditional variables; the variable is enabled if one of the enabling triggers is true while all the disabling triggers are false. Treated as an unordered set. @mutable tagmanager.accounts.containers.variables.create @mutable tagmanager.accounts.containers.variables.update
    #[serde(rename="enablingTriggerId")]
    
    pub enabling_trigger_id: Option<Vec<String>>,
    /// The fingerprint of the GTM Variable as computed at storage time. This value is recomputed whenever the variable is modified.
    
    pub fingerprint: Option<String>,
    /// Variable display name. @mutable tagmanager.accounts.containers.variables.create @mutable tagmanager.accounts.containers.variables.update
    
    pub name: Option<String>,
    /// User notes on how to apply this variable in the container. @mutable tagmanager.accounts.containers.variables.create @mutable tagmanager.accounts.containers.variables.update
    
    pub notes: Option<String>,
    /// The variable's parameters. @mutable tagmanager.accounts.containers.variables.create @mutable tagmanager.accounts.containers.variables.update
    
    pub parameter: Option<Vec<Parameter>>,
    /// Parent folder id.
    #[serde(rename="parentFolderId")]
    
    pub parent_folder_id: Option<String>,
    /// The end timestamp in milliseconds to schedule a variable. @mutable tagmanager.accounts.containers.variables.create @mutable tagmanager.accounts.containers.variables.update
    #[serde(rename="scheduleEndMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub schedule_end_ms: Option<i64>,
    /// The start timestamp in milliseconds to schedule a variable. @mutable tagmanager.accounts.containers.variables.create @mutable tagmanager.accounts.containers.variables.update
    #[serde(rename="scheduleStartMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub schedule_start_ms: Option<i64>,
    /// GTM Variable Type. @mutable tagmanager.accounts.containers.variables.create @mutable tagmanager.accounts.containers.variables.update
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The Variable ID uniquely identifies the GTM Variable.
    #[serde(rename="variableId")]
    
    pub variable_id: Option<String>,
}

impl client::RequestValue for Variable {}
impl client::ResponseResult for Variable {}


