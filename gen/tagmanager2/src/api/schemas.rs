use super::*;
/// Represents a Google Tag Manager Account.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers destinations get accounts](AccountContainerDestinationGetCall) (none)
/// * [containers destinations link accounts](AccountContainerDestinationLinkCall) (none)
/// * [containers destinations list accounts](AccountContainerDestinationListCall) (none)
/// * [containers environments create accounts](AccountContainerEnvironmentCreateCall) (none)
/// * [containers environments delete accounts](AccountContainerEnvironmentDeleteCall) (none)
/// * [containers environments get accounts](AccountContainerEnvironmentGetCall) (none)
/// * [containers environments list accounts](AccountContainerEnvironmentListCall) (none)
/// * [containers environments reauthorize accounts](AccountContainerEnvironmentReauthorizeCall) (none)
/// * [containers environments update accounts](AccountContainerEnvironmentUpdateCall) (none)
/// * [containers version_headers latest accounts](AccountContainerVersionHeaderLatestCall) (none)
/// * [containers version_headers list accounts](AccountContainerVersionHeaderListCall) (none)
/// * [containers versions delete accounts](AccountContainerVersionDeleteCall) (none)
/// * [containers versions get accounts](AccountContainerVersionGetCall) (none)
/// * [containers versions live accounts](AccountContainerVersionLiveCall) (none)
/// * [containers versions publish accounts](AccountContainerVersionPublishCall) (none)
/// * [containers versions set_latest accounts](AccountContainerVersionSetLatestCall) (none)
/// * [containers versions undelete accounts](AccountContainerVersionUndeleteCall) (none)
/// * [containers versions update accounts](AccountContainerVersionUpdateCall) (none)
/// * [containers workspaces built_in_variables create accounts](AccountContainerWorkspaceBuiltInVariableCreateCall) (none)
/// * [containers workspaces built_in_variables delete accounts](AccountContainerWorkspaceBuiltInVariableDeleteCall) (none)
/// * [containers workspaces built_in_variables list accounts](AccountContainerWorkspaceBuiltInVariableListCall) (none)
/// * [containers workspaces built_in_variables revert accounts](AccountContainerWorkspaceBuiltInVariableRevertCall) (none)
/// * [containers workspaces clients create accounts](AccountContainerWorkspaceClientCreateCall) (none)
/// * [containers workspaces clients delete accounts](AccountContainerWorkspaceClientDeleteCall) (none)
/// * [containers workspaces clients get accounts](AccountContainerWorkspaceClientGetCall) (none)
/// * [containers workspaces clients list accounts](AccountContainerWorkspaceClientListCall) (none)
/// * [containers workspaces clients revert accounts](AccountContainerWorkspaceClientRevertCall) (none)
/// * [containers workspaces clients update accounts](AccountContainerWorkspaceClientUpdateCall) (none)
/// * [containers workspaces folders create accounts](AccountContainerWorkspaceFolderCreateCall) (none)
/// * [containers workspaces folders delete accounts](AccountContainerWorkspaceFolderDeleteCall) (none)
/// * [containers workspaces folders entities accounts](AccountContainerWorkspaceFolderEntityCall) (none)
/// * [containers workspaces folders get accounts](AccountContainerWorkspaceFolderGetCall) (none)
/// * [containers workspaces folders list accounts](AccountContainerWorkspaceFolderListCall) (none)
/// * [containers workspaces folders move_entities_to_folder accounts](AccountContainerWorkspaceFolderMoveEntitiesToFolderCall) (none)
/// * [containers workspaces folders revert accounts](AccountContainerWorkspaceFolderRevertCall) (none)
/// * [containers workspaces folders update accounts](AccountContainerWorkspaceFolderUpdateCall) (none)
/// * [containers workspaces gtag_config create accounts](AccountContainerWorkspaceGtagConfigCreateCall) (none)
/// * [containers workspaces gtag_config delete accounts](AccountContainerWorkspaceGtagConfigDeleteCall) (none)
/// * [containers workspaces gtag_config get accounts](AccountContainerWorkspaceGtagConfigGetCall) (none)
/// * [containers workspaces gtag_config list accounts](AccountContainerWorkspaceGtagConfigListCall) (none)
/// * [containers workspaces gtag_config update accounts](AccountContainerWorkspaceGtagConfigUpdateCall) (none)
/// * [containers workspaces tags create accounts](AccountContainerWorkspaceTagCreateCall) (none)
/// * [containers workspaces tags delete accounts](AccountContainerWorkspaceTagDeleteCall) (none)
/// * [containers workspaces tags get accounts](AccountContainerWorkspaceTagGetCall) (none)
/// * [containers workspaces tags list accounts](AccountContainerWorkspaceTagListCall) (none)
/// * [containers workspaces tags revert accounts](AccountContainerWorkspaceTagRevertCall) (none)
/// * [containers workspaces tags update accounts](AccountContainerWorkspaceTagUpdateCall) (none)
/// * [containers workspaces templates create accounts](AccountContainerWorkspaceTemplateCreateCall) (none)
/// * [containers workspaces templates delete accounts](AccountContainerWorkspaceTemplateDeleteCall) (none)
/// * [containers workspaces templates get accounts](AccountContainerWorkspaceTemplateGetCall) (none)
/// * [containers workspaces templates list accounts](AccountContainerWorkspaceTemplateListCall) (none)
/// * [containers workspaces templates revert accounts](AccountContainerWorkspaceTemplateRevertCall) (none)
/// * [containers workspaces templates update accounts](AccountContainerWorkspaceTemplateUpdateCall) (none)
/// * [containers workspaces triggers create accounts](AccountContainerWorkspaceTriggerCreateCall) (none)
/// * [containers workspaces triggers delete accounts](AccountContainerWorkspaceTriggerDeleteCall) (none)
/// * [containers workspaces triggers get accounts](AccountContainerWorkspaceTriggerGetCall) (none)
/// * [containers workspaces triggers list accounts](AccountContainerWorkspaceTriggerListCall) (none)
/// * [containers workspaces triggers revert accounts](AccountContainerWorkspaceTriggerRevertCall) (none)
/// * [containers workspaces triggers update accounts](AccountContainerWorkspaceTriggerUpdateCall) (none)
/// * [containers workspaces variables create accounts](AccountContainerWorkspaceVariableCreateCall) (none)
/// * [containers workspaces variables delete accounts](AccountContainerWorkspaceVariableDeleteCall) (none)
/// * [containers workspaces variables get accounts](AccountContainerWorkspaceVariableGetCall) (none)
/// * [containers workspaces variables list accounts](AccountContainerWorkspaceVariableListCall) (none)
/// * [containers workspaces variables revert accounts](AccountContainerWorkspaceVariableRevertCall) (none)
/// * [containers workspaces variables update accounts](AccountContainerWorkspaceVariableUpdateCall) (none)
/// * [containers workspaces zones create accounts](AccountContainerWorkspaceZoneCreateCall) (none)
/// * [containers workspaces zones delete accounts](AccountContainerWorkspaceZoneDeleteCall) (none)
/// * [containers workspaces zones get accounts](AccountContainerWorkspaceZoneGetCall) (none)
/// * [containers workspaces zones list accounts](AccountContainerWorkspaceZoneListCall) (none)
/// * [containers workspaces zones revert accounts](AccountContainerWorkspaceZoneRevertCall) (none)
/// * [containers workspaces zones update accounts](AccountContainerWorkspaceZoneUpdateCall) (none)
/// * [containers workspaces create accounts](AccountContainerWorkspaceCreateCall) (none)
/// * [containers workspaces create_version accounts](AccountContainerWorkspaceCreateVersionCall) (none)
/// * [containers workspaces delete accounts](AccountContainerWorkspaceDeleteCall) (none)
/// * [containers workspaces get accounts](AccountContainerWorkspaceGetCall) (none)
/// * [containers workspaces get status accounts](AccountContainerWorkspaceGetStatuCall) (none)
/// * [containers workspaces list accounts](AccountContainerWorkspaceListCall) (none)
/// * [containers workspaces quick_preview accounts](AccountContainerWorkspaceQuickPreviewCall) (none)
/// * [containers workspaces resolve_conflict accounts](AccountContainerWorkspaceResolveConflictCall) (none)
/// * [containers workspaces sync accounts](AccountContainerWorkspaceSyncCall) (none)
/// * [containers workspaces update accounts](AccountContainerWorkspaceUpdateCall) (none)
/// * [containers combine accounts](AccountContainerCombineCall) (none)
/// * [containers create accounts](AccountContainerCreateCall) (none)
/// * [containers delete accounts](AccountContainerDeleteCall) (none)
/// * [containers get accounts](AccountContainerGetCall) (none)
/// * [containers list accounts](AccountContainerListCall) (none)
/// * [containers lookup accounts](AccountContainerLookupCall) (none)
/// * [containers move_tag_id accounts](AccountContainerMoveTagIdCall) (none)
/// * [containers snippet accounts](AccountContainerSnippetCall) (none)
/// * [containers update accounts](AccountContainerUpdateCall) (none)
/// * [user_permissions create accounts](AccountUserPermissionCreateCall) (none)
/// * [user_permissions delete accounts](AccountUserPermissionDeleteCall) (none)
/// * [user_permissions get accounts](AccountUserPermissionGetCall) (none)
/// * [user_permissions list accounts](AccountUserPermissionListCall) (none)
/// * [user_permissions update accounts](AccountUserPermissionUpdateCall) (none)
/// * [get accounts](AccountGetCall) (response)
/// * [list accounts](AccountListCall) (none)
/// * [update accounts](AccountUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    /// The Account ID uniquely identifies the GTM Account.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// Read-only Account feature set
    
    pub features: Option<AccountFeatures>,
    /// The fingerprint of the GTM Account as computed at storage time. This value is recomputed whenever the account is modified.
    
    pub fingerprint: Option<String>,
    /// Account display name. @mutable tagmanager.accounts.create @mutable tagmanager.accounts.update
    
    pub name: Option<String>,
    /// GTM Account's API relative path.
    
    pub path: Option<String>,
    /// Whether the account shares data anonymously with Google and others. This flag enables benchmarking by sharing your data in an anonymous form. Google will remove all identifiable information about your website, combine the data with hundreds of other anonymous sites and report aggregate trends in the benchmarking service. @mutable tagmanager.accounts.create @mutable tagmanager.accounts.update
    #[serde(rename="shareData")]
    
    pub share_data: Option<bool>,
    /// Auto generated link to the tag manager UI
    #[serde(rename="tagManagerUrl")]
    
    pub tag_manager_url: Option<String>,
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
    /// Whether the user has no access, user access, or admin access to an account. @mutable tagmanager.accounts.permissions.create @mutable tagmanager.accounts.permissions.update
    
    pub permission: Option<AccountAccesPermissionEnum>,
}

impl client::Part for AccountAccess {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountFeatures {
    /// Whether this Account supports multiple Containers.
    #[serde(rename="supportMultipleContainers")]
    
    pub support_multiple_containers: Option<bool>,
    /// Whether this Account supports user permissions managed by GTM.
    #[serde(rename="supportUserPermissions")]
    
    pub support_user_permissions: Option<bool>,
}

impl client::Part for AccountFeatures {}


/// Built-in variables are a special category of variables that are pre-created and non-customizable. They provide common functionality like accessing properties of the gtm data layer, monitoring clicks, or accessing elements of a page URL.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuiltInVariable {
    /// GTM Account ID.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// GTM Container ID.
    #[serde(rename="containerId")]
    
    pub container_id: Option<String>,
    /// Name of the built-in variable to be used to refer to the built-in variable.
    
    pub name: Option<String>,
    /// GTM BuiltInVariable's API relative path.
    
    pub path: Option<String>,
    /// Type of built-in variable. @required.tagmanager.accounts.containers.workspaces.built_in_variable.update @mutable tagmanager.accounts.containers.workspaces.built_in_variable.update
    #[serde(rename="type")]
    
    pub type_: Option<BuiltInVariableTypeEnum>,
    /// GTM Workspace ID.
    #[serde(rename="workspaceId")]
    
    pub workspace_id: Option<String>,
}

impl client::Part for BuiltInVariable {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers workspaces clients create accounts](AccountContainerWorkspaceClientCreateCall) (request|response)
/// * [containers workspaces clients get accounts](AccountContainerWorkspaceClientGetCall) (response)
/// * [containers workspaces clients update accounts](AccountContainerWorkspaceClientUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Client {
    /// GTM Account ID.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// The Client ID uniquely identifies the GTM client.
    #[serde(rename="clientId")]
    
    pub client_id: Option<String>,
    /// GTM Container ID.
    #[serde(rename="containerId")]
    
    pub container_id: Option<String>,
    /// The fingerprint of the GTM Client as computed at storage time. This value is recomputed whenever the client is modified.
    
    pub fingerprint: Option<String>,
    /// Client display name. @mutable tagmanager.accounts.containers.workspaces.clients.create @mutable tagmanager.accounts.containers.workspaces.clients.update
    
    pub name: Option<String>,
    /// User notes on how to apply this tag in the container. @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update
    
    pub notes: Option<String>,
    /// The client's parameters. @mutable tagmanager.accounts.containers.workspaces.clients.create @mutable tagmanager.accounts.containers.workspaces.clients.update
    
    pub parameter: Option<Vec<Parameter>>,
    /// Parent folder id.
    #[serde(rename="parentFolderId")]
    
    pub parent_folder_id: Option<String>,
    /// GTM client's API relative path.
    
    pub path: Option<String>,
    /// Priority determines relative firing order. @mutable tagmanager.accounts.containers.workspaces.clients.create @mutable tagmanager.accounts.containers.workspaces.clients.update
    
    pub priority: Option<i32>,
    /// Auto generated link to the tag manager UI
    #[serde(rename="tagManagerUrl")]
    
    pub tag_manager_url: Option<String>,
    /// Client type. @mutable tagmanager.accounts.containers.workspaces.clients.create @mutable tagmanager.accounts.containers.workspaces.clients.update
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// GTM Workspace ID.
    #[serde(rename="workspaceId")]
    
    pub workspace_id: Option<String>,
}

impl client::RequestValue for Client {}
impl client::ResponseResult for Client {}


/// Represents a predicate.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Condition {
    /// A list of named parameters (key/value), depending on the condition's type. Notes: - For binary operators, include parameters named arg0 and arg1 for specifying the left and right operands, respectively. - At this time, the left operand (arg0) must be a reference to a variable. - For case-insensitive Regex matching, include a boolean parameter named ignore_case that is set to true. If not specified or set to any other value, the matching will be case sensitive. - To negate an operator, include a boolean parameter named negate boolean parameter that is set to true. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update
    
    pub parameter: Option<Vec<Parameter>>,
    /// The type of operator for this condition. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update
    #[serde(rename="type")]
    
    pub type_: Option<ConditionTypeEnum>,
}

impl client::Part for Condition {}


/// Represents a Google Tag Manager Container, which specifies the platform tags will run on, manages workspaces, and retains container versions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers combine accounts](AccountContainerCombineCall) (response)
/// * [containers create accounts](AccountContainerCreateCall) (request|response)
/// * [containers get accounts](AccountContainerGetCall) (response)
/// * [containers lookup accounts](AccountContainerLookupCall) (response)
/// * [containers move_tag_id accounts](AccountContainerMoveTagIdCall) (response)
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
    /// List of domain names associated with the Container. @mutable tagmanager.accounts.containers.create @mutable tagmanager.accounts.containers.update
    #[serde(rename="domainName")]
    
    pub domain_name: Option<Vec<String>>,
    /// Read-only Container feature set.
    
    pub features: Option<ContainerFeatures>,
    /// The fingerprint of the GTM Container as computed at storage time. This value is recomputed whenever the account is modified.
    
    pub fingerprint: Option<String>,
    /// Container display name. @mutable tagmanager.accounts.containers.create @mutable tagmanager.accounts.containers.update
    
    pub name: Option<String>,
    /// Container Notes. @mutable tagmanager.accounts.containers.create @mutable tagmanager.accounts.containers.update
    
    pub notes: Option<String>,
    /// GTM Container's API relative path.
    
    pub path: Option<String>,
    /// Container Public ID.
    #[serde(rename="publicId")]
    
    pub public_id: Option<String>,
    /// All Tag IDs that refer to this Container.
    #[serde(rename="tagIds")]
    
    pub tag_ids: Option<Vec<String>>,
    /// Auto generated link to the tag manager UI
    #[serde(rename="tagManagerUrl")]
    
    pub tag_manager_url: Option<String>,
    /// List of server-side container URLs for the Container. If multiple URLs are provided, all URL paths must match. @mutable tagmanager.accounts.containers.create @mutable tagmanager.accounts.containers.update
    #[serde(rename="taggingServerUrls")]
    
    pub tagging_server_urls: Option<Vec<String>>,
    /// List of Usage Contexts for the Container. Valid values include: web, android, or ios. @mutable tagmanager.accounts.containers.create @mutable tagmanager.accounts.containers.update
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
    /// List of Container permissions. @mutable tagmanager.accounts.permissions.create @mutable tagmanager.accounts.permissions.update
    
    pub permission: Option<ContainerAccesPermissionEnum>,
}

impl client::Part for ContainerAccess {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContainerFeatures {
    /// Whether this Container supports built-in variables
    #[serde(rename="supportBuiltInVariables")]
    
    pub support_built_in_variables: Option<bool>,
    /// Whether this Container supports clients.
    #[serde(rename="supportClients")]
    
    pub support_clients: Option<bool>,
    /// Whether this Container supports environments.
    #[serde(rename="supportEnvironments")]
    
    pub support_environments: Option<bool>,
    /// Whether this Container supports folders.
    #[serde(rename="supportFolders")]
    
    pub support_folders: Option<bool>,
    /// Whether this Container supports Google tag config.
    #[serde(rename="supportGtagConfigs")]
    
    pub support_gtag_configs: Option<bool>,
    /// Whether this Container supports tags.
    #[serde(rename="supportTags")]
    
    pub support_tags: Option<bool>,
    /// Whether this Container supports templates.
    #[serde(rename="supportTemplates")]
    
    pub support_templates: Option<bool>,
    /// Whether this Container supports triggers.
    #[serde(rename="supportTriggers")]
    
    pub support_triggers: Option<bool>,
    /// Whether this Container supports user permissions managed by GTM.
    #[serde(rename="supportUserPermissions")]
    
    pub support_user_permissions: Option<bool>,
    /// Whether this Container supports variables.
    #[serde(rename="supportVariables")]
    
    pub support_variables: Option<bool>,
    /// Whether this Container supports Container versions.
    #[serde(rename="supportVersions")]
    
    pub support_versions: Option<bool>,
    /// Whether this Container supports workspaces.
    #[serde(rename="supportWorkspaces")]
    
    pub support_workspaces: Option<bool>,
    /// Whether this Container supports zones.
    #[serde(rename="supportZones")]
    
    pub support_zones: Option<bool>,
}

impl client::Part for ContainerFeatures {}


/// Represents a Google Tag Manager Container Version.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers versions get accounts](AccountContainerVersionGetCall) (response)
/// * [containers versions live accounts](AccountContainerVersionLiveCall) (response)
/// * [containers versions set_latest accounts](AccountContainerVersionSetLatestCall) (response)
/// * [containers versions undelete accounts](AccountContainerVersionUndeleteCall) (response)
/// * [containers versions update accounts](AccountContainerVersionUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContainerVersion {
    /// GTM Account ID.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// The built-in variables in the container that this version was taken from.
    #[serde(rename="builtInVariable")]
    
    pub built_in_variable: Option<Vec<BuiltInVariable>>,
    /// The clients in the container that this version was taken from.
    
    pub client: Option<Vec<Client>>,
    /// The container that this version was taken from.
    
    pub container: Option<Container>,
    /// GTM Container ID.
    #[serde(rename="containerId")]
    
    pub container_id: Option<String>,
    /// The Container Version ID uniquely identifies the GTM Container Version.
    #[serde(rename="containerVersionId")]
    
    pub container_version_id: Option<String>,
    /// The custom templates in the container that this version was taken from.
    #[serde(rename="customTemplate")]
    
    pub custom_template: Option<Vec<CustomTemplate>>,
    /// A value of true indicates this container version has been deleted.
    
    pub deleted: Option<bool>,
    /// Container version description. @mutable tagmanager.accounts.containers.versions.update
    
    pub description: Option<String>,
    /// The fingerprint of the GTM Container Version as computed at storage time. This value is recomputed whenever the container version is modified.
    
    pub fingerprint: Option<String>,
    /// The folders in the container that this version was taken from.
    
    pub folder: Option<Vec<Folder>>,
    /// The Google tag configs in the container that this version was taken from.
    #[serde(rename="gtagConfig")]
    
    pub gtag_config: Option<Vec<GtagConfig>>,
    /// Container version display name. @mutable tagmanager.accounts.containers.versions.update
    
    pub name: Option<String>,
    /// GTM Container Version's API relative path.
    
    pub path: Option<String>,
    /// The tags in the container that this version was taken from.
    
    pub tag: Option<Vec<Tag>>,
    /// Auto generated link to the tag manager UI
    #[serde(rename="tagManagerUrl")]
    
    pub tag_manager_url: Option<String>,
    /// The triggers in the container that this version was taken from.
    
    pub trigger: Option<Vec<Trigger>>,
    /// The variables in the container that this version was taken from.
    
    pub variable: Option<Vec<Variable>>,
    /// The zones in the container that this version was taken from.
    
    pub zone: Option<Vec<Zone>>,
}

impl client::RequestValue for ContainerVersion {}
impl client::ResponseResult for ContainerVersion {}


/// Represents a Google Tag Manager Container Version Header.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers version_headers latest accounts](AccountContainerVersionHeaderLatestCall) (response)
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
    /// Number of clients in the container version.
    #[serde(rename="numClients")]
    
    pub num_clients: Option<String>,
    /// Number of custom templates in the container version.
    #[serde(rename="numCustomTemplates")]
    
    pub num_custom_templates: Option<String>,
    /// Number of Google tag configs in the container version.
    #[serde(rename="numGtagConfigs")]
    
    pub num_gtag_configs: Option<String>,
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
    /// Number of zones in the container version.
    #[serde(rename="numZones")]
    
    pub num_zones: Option<String>,
    /// GTM Container Version's API relative path.
    
    pub path: Option<String>,
}

impl client::ResponseResult for ContainerVersionHeader {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers workspaces built_in_variables create accounts](AccountContainerWorkspaceBuiltInVariableCreateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateBuiltInVariableResponse {
    /// List of created built-in variables.
    #[serde(rename="builtInVariable")]
    
    pub built_in_variable: Option<Vec<BuiltInVariable>>,
}

impl client::ResponseResult for CreateBuiltInVariableResponse {}


/// Options for new container versions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers workspaces create_version accounts](AccountContainerWorkspaceCreateVersionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateContainerVersionRequestVersionOptions {
    /// The name of the container version to be created.
    
    pub name: Option<String>,
    /// The notes of the container version to be created.
    
    pub notes: Option<String>,
}

impl client::RequestValue for CreateContainerVersionRequestVersionOptions {}


/// Create container versions response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers workspaces create_version accounts](AccountContainerWorkspaceCreateVersionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateContainerVersionResponse {
    /// Compiler errors or not.
    #[serde(rename="compilerError")]
    
    pub compiler_error: Option<bool>,
    /// The container version created.
    #[serde(rename="containerVersion")]
    
    pub container_version: Option<ContainerVersion>,
    /// Auto generated workspace path created as a result of version creation. This field should only be populated if the created version was not a quick preview.
    #[serde(rename="newWorkspacePath")]
    
    pub new_workspace_path: Option<String>,
    /// Whether version creation failed when syncing the workspace to the latest container version.
    #[serde(rename="syncStatus")]
    
    pub sync_status: Option<SyncStatus>,
}

impl client::ResponseResult for CreateContainerVersionResponse {}


/// Represents a Google Tag Manager Custom Template’s contents.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers workspaces templates create accounts](AccountContainerWorkspaceTemplateCreateCall) (request|response)
/// * [containers workspaces templates get accounts](AccountContainerWorkspaceTemplateGetCall) (response)
/// * [containers workspaces templates update accounts](AccountContainerWorkspaceTemplateUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomTemplate {
    /// GTM Account ID.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// GTM Container ID.
    #[serde(rename="containerId")]
    
    pub container_id: Option<String>,
    /// The fingerprint of the GTM Custom Template as computed at storage time. This value is recomputed whenever the template is modified.
    
    pub fingerprint: Option<String>,
    /// A reference to the Community Template Gallery entry.
    #[serde(rename="galleryReference")]
    
    pub gallery_reference: Option<GalleryReference>,
    /// Custom Template display name.
    
    pub name: Option<String>,
    /// GTM Custom Template's API relative path.
    
    pub path: Option<String>,
    /// Auto generated link to the tag manager UI
    #[serde(rename="tagManagerUrl")]
    
    pub tag_manager_url: Option<String>,
    /// The custom template in text format.
    #[serde(rename="templateData")]
    
    pub template_data: Option<String>,
    /// The Custom Template ID uniquely identifies the GTM custom template.
    #[serde(rename="templateId")]
    
    pub template_id: Option<String>,
    /// GTM Workspace ID.
    #[serde(rename="workspaceId")]
    
    pub workspace_id: Option<String>,
}

impl client::RequestValue for CustomTemplate {}
impl client::ResponseResult for CustomTemplate {}


/// Represents a Google Tag Destination.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers destinations get accounts](AccountContainerDestinationGetCall) (response)
/// * [containers destinations link accounts](AccountContainerDestinationLinkCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Destination {
    /// GTM Account ID.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// GTM Container ID.
    #[serde(rename="containerId")]
    
    pub container_id: Option<String>,
    /// Destination ID.
    #[serde(rename="destinationId")]
    
    pub destination_id: Option<String>,
    /// The Destination link ID uniquely identifies the Destination.
    #[serde(rename="destinationLinkId")]
    
    pub destination_link_id: Option<String>,
    /// The fingerprint of the Google Tag Destination as computed at storage time. This value is recomputed whenever the destination is modified.
    
    pub fingerprint: Option<String>,
    /// Destination display name.
    
    pub name: Option<String>,
    /// Destination's API relative path.
    
    pub path: Option<String>,
    /// Auto generated link to the tag manager UI.
    #[serde(rename="tagManagerUrl")]
    
    pub tag_manager_url: Option<String>,
}

impl client::ResponseResult for Destination {}


/// A workspace entity that may represent a tag, trigger, variable, or folder in addition to its status in the workspace.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers workspaces resolve_conflict accounts](AccountContainerWorkspaceResolveConflictCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Entity {
    /// Represents how the entity has been changed in the workspace.
    #[serde(rename="changeStatus")]
    
    pub change_status: Option<EntityChangeStatusEnum>,
    /// The client being represented by the entity.
    
    pub client: Option<Client>,
    /// The folder being represented by the entity.
    
    pub folder: Option<Folder>,
    /// The tag being represented by the entity.
    
    pub tag: Option<Tag>,
    /// The trigger being represented by the entity.
    
    pub trigger: Option<Trigger>,
    /// The variable being represented by the entity.
    
    pub variable: Option<Variable>,
}

impl client::RequestValue for Entity {}


/// Represents a Google Tag Manager Environment. Note that a user can create, delete and update environments of type USER, but can only update the enable_debug and url fields of environments of other types.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers environments create accounts](AccountContainerEnvironmentCreateCall) (request|response)
/// * [containers environments get accounts](AccountContainerEnvironmentGetCall) (response)
/// * [containers environments reauthorize accounts](AccountContainerEnvironmentReauthorizeCall) (request|response)
/// * [containers environments update accounts](AccountContainerEnvironmentUpdateCall) (request|response)
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
    #[serde(rename="authorizationTimestamp")]
    
    pub authorization_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// GTM Container ID.
    #[serde(rename="containerId")]
    
    pub container_id: Option<String>,
    /// Represents a link to a container version.
    #[serde(rename="containerVersionId")]
    
    pub container_version_id: Option<String>,
    /// The environment description. Can be set or changed only on USER type environments. @mutable tagmanager.accounts.containers.environments.create @mutable tagmanager.accounts.containers.environments.update
    
    pub description: Option<String>,
    /// Whether or not to enable debug by default for the environment. @mutable tagmanager.accounts.containers.environments.create @mutable tagmanager.accounts.containers.environments.update
    #[serde(rename="enableDebug")]
    
    pub enable_debug: Option<bool>,
    /// GTM Environment ID uniquely identifies the GTM Environment.
    #[serde(rename="environmentId")]
    
    pub environment_id: Option<String>,
    /// The fingerprint of the GTM environment as computed at storage time. This value is recomputed whenever the environment is modified.
    
    pub fingerprint: Option<String>,
    /// The environment display name. Can be set or changed only on USER type environments. @mutable tagmanager.accounts.containers.environments.create @mutable tagmanager.accounts.containers.environments.update
    
    pub name: Option<String>,
    /// GTM Environment's API relative path.
    
    pub path: Option<String>,
    /// Auto generated link to the tag manager UI
    #[serde(rename="tagManagerUrl")]
    
    pub tag_manager_url: Option<String>,
    /// The type of this environment.
    #[serde(rename="type")]
    
    pub type_: Option<EnvironmentTypeEnum>,
    /// Default preview page url for the environment. @mutable tagmanager.accounts.containers.environments.create @mutable tagmanager.accounts.containers.environments.update
    
    pub url: Option<String>,
    /// Represents a link to a quick preview of a workspace.
    #[serde(rename="workspaceId")]
    
    pub workspace_id: Option<String>,
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
/// * [containers workspaces folders create accounts](AccountContainerWorkspaceFolderCreateCall) (request|response)
/// * [containers workspaces folders get accounts](AccountContainerWorkspaceFolderGetCall) (response)
/// * [containers workspaces folders move_entities_to_folder accounts](AccountContainerWorkspaceFolderMoveEntitiesToFolderCall) (request)
/// * [containers workspaces folders update accounts](AccountContainerWorkspaceFolderUpdateCall) (request|response)
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
    /// Folder display name. @mutable tagmanager.accounts.containers.workspaces.folders.create @mutable tagmanager.accounts.containers.workspaces.folders.update
    
    pub name: Option<String>,
    /// User notes on how to apply this folder in the container. @mutable tagmanager.accounts.containers.workspaces.folders.create @mutable tagmanager.accounts.containers.workspaces.folders.update
    
    pub notes: Option<String>,
    /// GTM Folder's API relative path.
    
    pub path: Option<String>,
    /// Auto generated link to the tag manager UI
    #[serde(rename="tagManagerUrl")]
    
    pub tag_manager_url: Option<String>,
    /// GTM Workspace ID.
    #[serde(rename="workspaceId")]
    
    pub workspace_id: Option<String>,
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
/// * [containers workspaces folders entities accounts](AccountContainerWorkspaceFolderEntityCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FolderEntities {
    /// Continuation token for fetching the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of tags inside the folder.
    
    pub tag: Option<Vec<Tag>>,
    /// The list of triggers inside the folder.
    
    pub trigger: Option<Vec<Trigger>>,
    /// The list of variables inside the folder.
    
    pub variable: Option<Vec<Variable>>,
}

impl client::ResponseResult for FolderEntities {}


/// Represents the link between a custom template and an entry on the Community Template Gallery site.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GalleryReference {
    /// The name of the host for the community gallery template.
    
    pub host: Option<String>,
    /// If a user has manually edited the community gallery template.
    #[serde(rename="isModified")]
    
    pub is_modified: Option<bool>,
    /// The name of the owner for the community gallery template.
    
    pub owner: Option<String>,
    /// The name of the repository for the community gallery template.
    
    pub repository: Option<String>,
    /// The signature of the community gallery template as computed at import time. This value is recomputed whenever the template is updated from the gallery.
    
    pub signature: Option<String>,
    /// The version of the community gallery template.
    
    pub version: Option<String>,
}

impl client::Part for GalleryReference {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers snippet accounts](AccountContainerSnippetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetContainerSnippetResponse {
    /// Tagging snippet for a Container.
    
    pub snippet: Option<String>,
}

impl client::ResponseResult for GetContainerSnippetResponse {}


/// The changes that have occurred in the workspace since the base container version.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers workspaces get status accounts](AccountContainerWorkspaceGetStatuCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetWorkspaceStatusResponse {
    /// The merge conflict after sync.
    #[serde(rename="mergeConflict")]
    
    pub merge_conflict: Option<Vec<MergeConflict>>,
    /// Entities that have been changed in the workspace.
    #[serde(rename="workspaceChange")]
    
    pub workspace_change: Option<Vec<Entity>>,
}

impl client::ResponseResult for GetWorkspaceStatusResponse {}


/// Represents a Google tag configuration.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers workspaces gtag_config create accounts](AccountContainerWorkspaceGtagConfigCreateCall) (request|response)
/// * [containers workspaces gtag_config get accounts](AccountContainerWorkspaceGtagConfigGetCall) (response)
/// * [containers workspaces gtag_config update accounts](AccountContainerWorkspaceGtagConfigUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GtagConfig {
    /// Google tag account ID.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// Google tag container ID.
    #[serde(rename="containerId")]
    
    pub container_id: Option<String>,
    /// The fingerprint of the Google tag config as computed at storage time. This value is recomputed whenever the config is modified.
    
    pub fingerprint: Option<String>,
    /// The ID uniquely identifies the Google tag config.
    #[serde(rename="gtagConfigId")]
    
    pub gtag_config_id: Option<String>,
    /// The Google tag config's parameters. @mutable tagmanager.accounts.containers.workspaces.gtag_config.create @mutable tagmanager.accounts.containers.workspaces.gtag_config.update
    
    pub parameter: Option<Vec<Parameter>>,
    /// Google tag config's API relative path.
    
    pub path: Option<String>,
    /// Auto generated link to the tag manager UI
    #[serde(rename="tagManagerUrl")]
    
    pub tag_manager_url: Option<String>,
    /// Google tag config type. @required tagmanager.accounts.containers.workspaces.gtag_config.create @required tagmanager.accounts.containers.workspaces.gtag_config.update @mutable tagmanager.accounts.containers.workspaces.gtag_config.create @mutable tagmanager.accounts.containers.workspaces.gtag_config.update
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Google tag workspace ID. Only used by GTM containers. Set to 0 otherwise.
    #[serde(rename="workspaceId")]
    
    pub workspace_id: Option<String>,
}

impl client::RequestValue for GtagConfig {}
impl client::ResponseResult for GtagConfig {}


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
    
    pub account: Option<Vec<Account>>,
    /// Continuation token for fetching the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListAccountsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers workspaces clients list accounts](AccountContainerWorkspaceClientListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListClientsResponse {
    /// All GTM Clients of a GTM Container.
    
    pub client: Option<Vec<Client>>,
    /// Continuation token for fetching the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListClientsResponse {}


/// List container versions response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers version_headers list accounts](AccountContainerVersionHeaderListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListContainerVersionsResponse {
    /// All container version headers of a GTM Container.
    #[serde(rename="containerVersionHeader")]
    
    pub container_version_header: Option<Vec<ContainerVersionHeader>>,
    /// Continuation token for fetching the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
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
    
    pub container: Option<Vec<Container>>,
    /// Continuation token for fetching the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListContainersResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers destinations list accounts](AccountContainerDestinationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListDestinationsResponse {
    /// All Destinations linked to a GTM Container.
    
    pub destination: Option<Vec<Destination>>,
    /// Continuation token for fetching the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListDestinationsResponse {}


/// A list of enabled built-in variables.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers workspaces built_in_variables list accounts](AccountContainerWorkspaceBuiltInVariableListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListEnabledBuiltInVariablesResponse {
    /// All GTM BuiltInVariables of a GTM container.
    #[serde(rename="builtInVariable")]
    
    pub built_in_variable: Option<Vec<BuiltInVariable>>,
    /// Continuation token for fetching the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListEnabledBuiltInVariablesResponse {}


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
    
    pub environment: Option<Vec<Environment>>,
    /// Continuation token for fetching the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListEnvironmentsResponse {}


/// List Folders Response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers workspaces folders list accounts](AccountContainerWorkspaceFolderListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListFoldersResponse {
    /// All GTM Folders of a GTM Container.
    
    pub folder: Option<Vec<Folder>>,
    /// Continuation token for fetching the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListFoldersResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers workspaces gtag_config list accounts](AccountContainerWorkspaceGtagConfigListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListGtagConfigResponse {
    /// All Google tag configs in a Container.
    #[serde(rename="gtagConfig")]
    
    pub gtag_config: Option<Vec<GtagConfig>>,
    /// Continuation token for fetching the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListGtagConfigResponse {}


/// List Tags Response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers workspaces tags list accounts](AccountContainerWorkspaceTagListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTagsResponse {
    /// Continuation token for fetching the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// All GTM Tags of a GTM Container.
    
    pub tag: Option<Vec<Tag>>,
}

impl client::ResponseResult for ListTagsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers workspaces templates list accounts](AccountContainerWorkspaceTemplateListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTemplatesResponse {
    /// Continuation token for fetching the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// All GTM Custom Templates of a GTM Container.
    
    pub template: Option<Vec<CustomTemplate>>,
}

impl client::ResponseResult for ListTemplatesResponse {}


/// List triggers response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers workspaces triggers list accounts](AccountContainerWorkspaceTriggerListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTriggersResponse {
    /// Continuation token for fetching the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// All GTM Triggers of a GTM Container.
    
    pub trigger: Option<Vec<Trigger>>,
}

impl client::ResponseResult for ListTriggersResponse {}


/// List user permissions response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [user_permissions list accounts](AccountUserPermissionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListUserPermissionsResponse {
    /// Continuation token for fetching the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// All GTM UserPermissions of a GTM Account.
    #[serde(rename="userPermission")]
    
    pub user_permission: Option<Vec<UserPermission>>,
}

impl client::ResponseResult for ListUserPermissionsResponse {}


/// List Variables Response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers workspaces variables list accounts](AccountContainerWorkspaceVariableListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListVariablesResponse {
    /// Continuation token for fetching the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// All GTM Variables of a GTM Container.
    
    pub variable: Option<Vec<Variable>>,
}

impl client::ResponseResult for ListVariablesResponse {}


/// A list of workspaces in a container.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers workspaces list accounts](AccountContainerWorkspaceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListWorkspacesResponse {
    /// Continuation token for fetching the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// All Workspaces of a GTM Container.
    
    pub workspace: Option<Vec<Workspace>>,
}

impl client::ResponseResult for ListWorkspacesResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers workspaces zones list accounts](AccountContainerWorkspaceZoneListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListZonesResponse {
    /// Continuation token for fetching the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// All GTM Zones of a GTM Container.
    
    pub zone: Option<Vec<Zone>>,
}

impl client::ResponseResult for ListZonesResponse {}


/// Represents a merge conflict.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MergeConflict {
    /// The base version entity (since the latest sync operation) that has conflicting changes compared to the workspace. If this field is missing, it means the workspace entity is deleted from the base version.
    #[serde(rename="entityInBaseVersion")]
    
    pub entity_in_base_version: Option<Entity>,
    /// The workspace entity that has conflicting changes compared to the base version. If an entity is deleted in a workspace, it will still appear with a deleted change status.
    #[serde(rename="entityInWorkspace")]
    
    pub entity_in_workspace: Option<Entity>,
}

impl client::Part for MergeConflict {}


/// Represents a Google Tag Manager Parameter.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Parameter {
    /// The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values. @mutable tagmanager.accounts.containers.workspaces.variables.create @mutable tagmanager.accounts.containers.workspaces.variables.update @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update
    
    pub key: Option<String>,
    /// This list parameter's parameters (keys will be ignored). @mutable tagmanager.accounts.containers.workspaces.variables.create @mutable tagmanager.accounts.containers.workspaces.variables.update @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update
    
    pub list: Option<Vec<Parameter>>,
    /// This map parameter's parameters (must have keys; keys must be unique). @mutable tagmanager.accounts.containers.workspaces.variables.create @mutable tagmanager.accounts.containers.workspaces.variables.update @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update
    
    pub map: Option<Vec<Parameter>>,
    /// The parameter type. Valid values are: - boolean: The value represents a boolean, represented as 'true' or 'false' - integer: The value represents a 64-bit signed integer value, in base 10 - list: A list of parameters should be specified - map: A map of parameters should be specified - template: The value represents any text; this can include variable references (even variable references that might return non-string types) - trigger_reference: The value represents a trigger, represented as the trigger id - tag_reference: The value represents a tag, represented as the tag name @mutable tagmanager.accounts.containers.workspaces.variables.create @mutable tagmanager.accounts.containers.workspaces.variables.update @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update
    #[serde(rename="type")]
    
    pub type_: Option<ParameterTypeEnum>,
    /// A parameter's value (may contain variable references such as "{{myVariable}}") as appropriate to the specified type. @mutable tagmanager.accounts.containers.workspaces.variables.create @mutable tagmanager.accounts.containers.workspaces.variables.update @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update
    
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


/// Response to quick previewing a workspace.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers workspaces quick_preview accounts](AccountContainerWorkspaceQuickPreviewCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QuickPreviewResponse {
    /// Were there compiler errors or not.
    #[serde(rename="compilerError")]
    
    pub compiler_error: Option<bool>,
    /// The quick previewed container version.
    #[serde(rename="containerVersion")]
    
    pub container_version: Option<ContainerVersion>,
    /// Whether quick previewing failed when syncing the workspace to the latest container version.
    #[serde(rename="syncStatus")]
    
    pub sync_status: Option<SyncStatus>,
}

impl client::ResponseResult for QuickPreviewResponse {}


/// The result of reverting a built-in variable in a workspace.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers workspaces built_in_variables revert accounts](AccountContainerWorkspaceBuiltInVariableRevertCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RevertBuiltInVariableResponse {
    /// Whether the built-in variable is enabled after reversion.
    
    pub enabled: Option<bool>,
}

impl client::ResponseResult for RevertBuiltInVariableResponse {}


/// The result of reverting a client in a workspace.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers workspaces clients revert accounts](AccountContainerWorkspaceClientRevertCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RevertClientResponse {
    /// Client as it appears in the latest container version since the last workspace synchronization operation. If no client is present, that means the client was deleted in the latest container version.
    
    pub client: Option<Client>,
}

impl client::ResponseResult for RevertClientResponse {}


/// The result of reverting folder changes in a workspace.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers workspaces folders revert accounts](AccountContainerWorkspaceFolderRevertCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RevertFolderResponse {
    /// Folder as it appears in the latest container version since the last workspace synchronization operation. If no folder is present, that means the folder was deleted in the latest container version.
    
    pub folder: Option<Folder>,
}

impl client::ResponseResult for RevertFolderResponse {}


/// The result of reverting a tag in a workspace.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers workspaces tags revert accounts](AccountContainerWorkspaceTagRevertCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RevertTagResponse {
    /// Tag as it appears in the latest container version since the last workspace synchronization operation. If no tag is present, that means the tag was deleted in the latest container version.
    
    pub tag: Option<Tag>,
}

impl client::ResponseResult for RevertTagResponse {}


/// The result of reverting a template in a workspace.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers workspaces templates revert accounts](AccountContainerWorkspaceTemplateRevertCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RevertTemplateResponse {
    /// Template as it appears in the latest container version since the last workspace synchronization operation. If no template is present, that means the template was deleted in the latest container version.
    
    pub template: Option<CustomTemplate>,
}

impl client::ResponseResult for RevertTemplateResponse {}


/// The result of reverting a trigger in a workspace.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers workspaces triggers revert accounts](AccountContainerWorkspaceTriggerRevertCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RevertTriggerResponse {
    /// Trigger as it appears in the latest container version since the last workspace synchronization operation. If no trigger is present, that means the trigger was deleted in the latest container version.
    
    pub trigger: Option<Trigger>,
}

impl client::ResponseResult for RevertTriggerResponse {}


/// The result of reverting a variable in a workspace.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers workspaces variables revert accounts](AccountContainerWorkspaceVariableRevertCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RevertVariableResponse {
    /// Variable as it appears in the latest container version since the last workspace synchronization operation. If no variable is present, that means the variable was deleted in the latest container version.
    
    pub variable: Option<Variable>,
}

impl client::ResponseResult for RevertVariableResponse {}


/// The result of reverting a zone in a workspace.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers workspaces zones revert accounts](AccountContainerWorkspaceZoneRevertCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RevertZoneResponse {
    /// Zone as it appears in the latest container version since the last workspace synchronization operation. If no zone is present, that means the zone was deleted in the latest container version.
    
    pub zone: Option<Zone>,
}

impl client::ResponseResult for RevertZoneResponse {}


/// Represents a reference to atag that fires before another tag in order to set up dependencies.
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


/// The status of a workspace after synchronization.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SyncStatus {
    /// Synchornization operation detected a merge conflict.
    #[serde(rename="mergeConflict")]
    
    pub merge_conflict: Option<bool>,
    /// An error occurred during the synchronization operation.
    #[serde(rename="syncError")]
    
    pub sync_error: Option<bool>,
}

impl client::Part for SyncStatus {}


/// A response after synchronizing the workspace to the latest container version.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers workspaces sync accounts](AccountContainerWorkspaceSyncCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SyncWorkspaceResponse {
    /// The merge conflict after sync. If this field is not empty, the sync is still treated as successful. But a version cannot be created until all conflicts are resolved.
    #[serde(rename="mergeConflict")]
    
    pub merge_conflict: Option<Vec<MergeConflict>>,
    /// Indicates whether synchronization caused a merge conflict or sync error.
    #[serde(rename="syncStatus")]
    
    pub sync_status: Option<SyncStatus>,
}

impl client::ResponseResult for SyncWorkspaceResponse {}


/// Represents a Google Tag Manager Tag.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers workspaces tags create accounts](AccountContainerWorkspaceTagCreateCall) (request|response)
/// * [containers workspaces tags get accounts](AccountContainerWorkspaceTagGetCall) (response)
/// * [containers workspaces tags update accounts](AccountContainerWorkspaceTagUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Tag {
    /// GTM Account ID.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// Blocking rule IDs. If any of the listed rules evaluate to true, the tag will not fire. @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update
    #[serde(rename="blockingRuleId")]
    
    pub blocking_rule_id: Option<Vec<String>>,
    /// Blocking trigger IDs. If any of the listed triggers evaluate to true, the tag will not fire. @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update
    #[serde(rename="blockingTriggerId")]
    
    pub blocking_trigger_id: Option<Vec<String>>,
    /// Consent settings of a tag. @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update
    #[serde(rename="consentSettings")]
    
    pub consent_settings: Option<TagConsentSetting>,
    /// GTM Container ID.
    #[serde(rename="containerId")]
    
    pub container_id: Option<String>,
    /// The fingerprint of the GTM Tag as computed at storage time. This value is recomputed whenever the tag is modified.
    
    pub fingerprint: Option<String>,
    /// Firing rule IDs. A tag will fire when any of the listed rules are true and all of its blockingRuleIds (if any specified) are false. @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update
    #[serde(rename="firingRuleId")]
    
    pub firing_rule_id: Option<Vec<String>>,
    /// Firing trigger IDs. A tag will fire when any of the listed triggers are true and all of its blockingTriggerIds (if any specified) are false. @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update
    #[serde(rename="firingTriggerId")]
    
    pub firing_trigger_id: Option<Vec<String>>,
    /// If set to true, this tag will only fire in the live environment (e.g. not in preview or debug mode). @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update
    #[serde(rename="liveOnly")]
    
    pub live_only: Option<bool>,
    /// A map of key-value pairs of tag metadata to be included in the event data for tag monitoring. Notes: - This parameter must be type MAP. - Each parameter in the map are type TEMPLATE, however cannot contain variable references. @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update
    #[serde(rename="monitoringMetadata")]
    
    pub monitoring_metadata: Option<Parameter>,
    /// If non-empty, then the tag display name will be included in the monitoring metadata map using the key specified. @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update
    #[serde(rename="monitoringMetadataTagNameKey")]
    
    pub monitoring_metadata_tag_name_key: Option<String>,
    /// Tag display name. @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update
    
    pub name: Option<String>,
    /// User notes on how to apply this tag in the container. @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update
    
    pub notes: Option<String>,
    /// The tag's parameters. @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update
    
    pub parameter: Option<Vec<Parameter>>,
    /// Parent folder id.
    #[serde(rename="parentFolderId")]
    
    pub parent_folder_id: Option<String>,
    /// GTM Tag's API relative path.
    
    pub path: Option<String>,
    /// Indicates whether the tag is paused, which prevents the tag from firing. @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update
    
    pub paused: Option<bool>,
    /// User defined numeric priority of the tag. Tags are fired asynchronously in order of priority. Tags with higher numeric value fire first. A tag's priority can be a positive or negative value. The default value is 0. @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update
    
    pub priority: Option<Parameter>,
    /// The end timestamp in milliseconds to schedule a tag. @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update
    #[serde(rename="scheduleEndMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub schedule_end_ms: Option<i64>,
    /// The start timestamp in milliseconds to schedule a tag. @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update
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
    /// Auto generated link to the tag manager UI
    #[serde(rename="tagManagerUrl")]
    
    pub tag_manager_url: Option<String>,
    /// The list of teardown tags. Currently we only allow one.
    #[serde(rename="teardownTag")]
    
    pub teardown_tag: Option<Vec<TeardownTag>>,
    /// GTM Tag Type. @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// GTM Workspace ID.
    #[serde(rename="workspaceId")]
    
    pub workspace_id: Option<String>,
}

impl client::RequestValue for Tag {}
impl client::ResponseResult for Tag {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TagConsentSetting {
    /// The tag's consent status. If set to NEEDED, the runtime will check that the consent types specified by the consent_type field have been granted.
    #[serde(rename="consentStatus")]
    
    pub consent_status: Option<TagConsentSettingConsentStatusEnum>,
    /// The type of consents to check for during tag firing if in the consent NEEDED state. This parameter must be of type LIST where each list item is of type STRING.
    #[serde(rename="consentType")]
    
    pub consent_type: Option<Parameter>,
}

impl client::Part for TagConsentSetting {}


/// Represents a tag that fires after another tag in order to tear down dependencies.
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
/// * [containers workspaces triggers create accounts](AccountContainerWorkspaceTriggerCreateCall) (request|response)
/// * [containers workspaces triggers get accounts](AccountContainerWorkspaceTriggerGetCall) (response)
/// * [containers workspaces triggers update accounts](AccountContainerWorkspaceTriggerUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Trigger {
    /// GTM Account ID.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// Used in the case of auto event tracking. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update
    #[serde(rename="autoEventFilter")]
    
    pub auto_event_filter: Option<Vec<Condition>>,
    /// Whether or not we should only fire tags if the form submit or link click event is not cancelled by some other event handler (e.g. because of validation). Only valid for Form Submission and Link Click triggers. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update
    #[serde(rename="checkValidation")]
    
    pub check_validation: Option<Parameter>,
    /// GTM Container ID.
    #[serde(rename="containerId")]
    
    pub container_id: Option<String>,
    /// A visibility trigger minimum continuous visible time (in milliseconds). Only valid for AMP Visibility trigger. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update
    #[serde(rename="continuousTimeMinMilliseconds")]
    
    pub continuous_time_min_milliseconds: Option<Parameter>,
    /// Used in the case of custom event, which is fired iff all Conditions are true. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update
    #[serde(rename="customEventFilter")]
    
    pub custom_event_filter: Option<Vec<Condition>>,
    /// Name of the GTM event that is fired. Only valid for Timer triggers. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update
    #[serde(rename="eventName")]
    
    pub event_name: Option<Parameter>,
    /// The trigger will only fire iff all Conditions are true. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update
    
    pub filter: Option<Vec<Condition>>,
    /// The fingerprint of the GTM Trigger as computed at storage time. This value is recomputed whenever the trigger is modified.
    
    pub fingerprint: Option<String>,
    /// List of integer percentage values for scroll triggers. The trigger will fire when each percentage is reached when the view is scrolled horizontally. Only valid for AMP scroll triggers. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update
    #[serde(rename="horizontalScrollPercentageList")]
    
    pub horizontal_scroll_percentage_list: Option<Parameter>,
    /// Time between triggering recurring Timer Events (in milliseconds). Only valid for Timer triggers. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update
    
    pub interval: Option<Parameter>,
    /// Time between Timer Events to fire (in seconds). Only valid for AMP Timer trigger. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update
    #[serde(rename="intervalSeconds")]
    
    pub interval_seconds: Option<Parameter>,
    /// Limit of the number of GTM events this Timer Trigger will fire. If no limit is set, we will continue to fire GTM events until the user leaves the page. Only valid for Timer triggers. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update
    
    pub limit: Option<Parameter>,
    /// Max time to fire Timer Events (in seconds). Only valid for AMP Timer trigger. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update
    #[serde(rename="maxTimerLengthSeconds")]
    
    pub max_timer_length_seconds: Option<Parameter>,
    /// Trigger display name. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update
    
    pub name: Option<String>,
    /// User notes on how to apply this trigger in the container. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update
    
    pub notes: Option<String>,
    /// Additional parameters. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update
    
    pub parameter: Option<Vec<Parameter>>,
    /// Parent folder id.
    #[serde(rename="parentFolderId")]
    
    pub parent_folder_id: Option<String>,
    /// GTM Trigger's API relative path.
    
    pub path: Option<String>,
    /// A click trigger CSS selector (i.e. "a", "button" etc.). Only valid for AMP Click trigger. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update
    
    pub selector: Option<Parameter>,
    /// Auto generated link to the tag manager UI
    #[serde(rename="tagManagerUrl")]
    
    pub tag_manager_url: Option<String>,
    /// A visibility trigger minimum total visible time (in milliseconds). Only valid for AMP Visibility trigger. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update
    #[serde(rename="totalTimeMinMilliseconds")]
    
    pub total_time_min_milliseconds: Option<Parameter>,
    /// The Trigger ID uniquely identifies the GTM Trigger.
    #[serde(rename="triggerId")]
    
    pub trigger_id: Option<String>,
    /// Defines the data layer event that causes this trigger. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update
    #[serde(rename="type")]
    
    pub type_: Option<TriggerTypeEnum>,
    /// Globally unique id of the trigger that auto-generates this (a Form Submit, Link Click or Timer listener) if any. Used to make incompatible auto-events work together with trigger filtering based on trigger ids. This value is populated during output generation since the tags implied by triggers don't exist until then. Only valid for Form Submit, Link Click and Timer triggers. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update
    #[serde(rename="uniqueTriggerId")]
    
    pub unique_trigger_id: Option<Parameter>,
    /// List of integer percentage values for scroll triggers. The trigger will fire when each percentage is reached when the view is scrolled vertically. Only valid for AMP scroll triggers. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update
    #[serde(rename="verticalScrollPercentageList")]
    
    pub vertical_scroll_percentage_list: Option<Parameter>,
    /// A visibility trigger CSS selector (i.e. "#id"). Only valid for AMP Visibility trigger. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update
    #[serde(rename="visibilitySelector")]
    
    pub visibility_selector: Option<Parameter>,
    /// A visibility trigger maximum percent visibility. Only valid for AMP Visibility trigger. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update
    #[serde(rename="visiblePercentageMax")]
    
    pub visible_percentage_max: Option<Parameter>,
    /// A visibility trigger minimum percent visibility. Only valid for AMP Visibility trigger. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update
    #[serde(rename="visiblePercentageMin")]
    
    pub visible_percentage_min: Option<Parameter>,
    /// Whether or not we should delay the form submissions or link opening until all of the tags have fired (by preventing the default action and later simulating the default action). Only valid for Form Submission and Link Click triggers. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update
    #[serde(rename="waitForTags")]
    
    pub wait_for_tags: Option<Parameter>,
    /// How long to wait (in milliseconds) for tags to fire when 'waits_for_tags' above evaluates to true. Only valid for Form Submission and Link Click triggers. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update
    #[serde(rename="waitForTagsTimeout")]
    
    pub wait_for_tags_timeout: Option<Parameter>,
    /// GTM Workspace ID.
    #[serde(rename="workspaceId")]
    
    pub workspace_id: Option<String>,
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
/// * [user_permissions create accounts](AccountUserPermissionCreateCall) (request|response)
/// * [user_permissions get accounts](AccountUserPermissionGetCall) (response)
/// * [user_permissions update accounts](AccountUserPermissionUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserPermission {
    /// GTM Account access permissions. @mutable tagmanager.accounts.permissions.create @mutable tagmanager.accounts.permissions.update
    #[serde(rename="accountAccess")]
    
    pub account_access: Option<AccountAccess>,
    /// The Account ID uniquely identifies the GTM Account.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// GTM Container access permissions. @mutable tagmanager.accounts.permissions.create @mutable tagmanager.accounts.permissions.update
    #[serde(rename="containerAccess")]
    
    pub container_access: Option<Vec<ContainerAccess>>,
    /// User's email address. @mutable tagmanager.accounts.permissions.create
    #[serde(rename="emailAddress")]
    
    pub email_address: Option<String>,
    /// GTM UserPermission's API relative path.
    
    pub path: Option<String>,
}

impl client::RequestValue for UserPermission {}
impl client::ResponseResult for UserPermission {}


/// Represents a Google Tag Manager Variable.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers workspaces variables create accounts](AccountContainerWorkspaceVariableCreateCall) (request|response)
/// * [containers workspaces variables get accounts](AccountContainerWorkspaceVariableGetCall) (response)
/// * [containers workspaces variables update accounts](AccountContainerWorkspaceVariableUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Variable {
    /// GTM Account ID.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// GTM Container ID.
    #[serde(rename="containerId")]
    
    pub container_id: Option<String>,
    /// For mobile containers only: A list of trigger IDs for disabling conditional variables; the variable is enabled if one of the enabling trigger is true while all the disabling trigger are false. Treated as an unordered set. @mutable tagmanager.accounts.containers.workspaces.variables.create @mutable tagmanager.accounts.containers.workspaces.variables.update
    #[serde(rename="disablingTriggerId")]
    
    pub disabling_trigger_id: Option<Vec<String>>,
    /// For mobile containers only: A list of trigger IDs for enabling conditional variables; the variable is enabled if one of the enabling triggers is true while all the disabling triggers are false. Treated as an unordered set. @mutable tagmanager.accounts.containers.workspaces.variables.create @mutable tagmanager.accounts.containers.workspaces.variables.update
    #[serde(rename="enablingTriggerId")]
    
    pub enabling_trigger_id: Option<Vec<String>>,
    /// The fingerprint of the GTM Variable as computed at storage time. This value is recomputed whenever the variable is modified.
    
    pub fingerprint: Option<String>,
    /// Option to convert a variable value to other value.
    #[serde(rename="formatValue")]
    
    pub format_value: Option<VariableFormatValue>,
    /// Variable display name. @mutable tagmanager.accounts.containers.workspaces.variables.create @mutable tagmanager.accounts.containers.workspaces.variables.update
    
    pub name: Option<String>,
    /// User notes on how to apply this variable in the container. @mutable tagmanager.accounts.containers.workspaces.variables.create @mutable tagmanager.accounts.containers.workspaces.variables.update
    
    pub notes: Option<String>,
    /// The variable's parameters. @mutable tagmanager.accounts.containers.workspaces.variables.create @mutable tagmanager.accounts.containers.workspaces.variables.update
    
    pub parameter: Option<Vec<Parameter>>,
    /// Parent folder id.
    #[serde(rename="parentFolderId")]
    
    pub parent_folder_id: Option<String>,
    /// GTM Variable's API relative path.
    
    pub path: Option<String>,
    /// The end timestamp in milliseconds to schedule a variable. @mutable tagmanager.accounts.containers.workspaces.variables.create @mutable tagmanager.accounts.containers.workspaces.variables.update
    #[serde(rename="scheduleEndMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub schedule_end_ms: Option<i64>,
    /// The start timestamp in milliseconds to schedule a variable. @mutable tagmanager.accounts.containers.workspaces.variables.create @mutable tagmanager.accounts.containers.workspaces.variables.update
    #[serde(rename="scheduleStartMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub schedule_start_ms: Option<i64>,
    /// Auto generated link to the tag manager UI
    #[serde(rename="tagManagerUrl")]
    
    pub tag_manager_url: Option<String>,
    /// GTM Variable Type. @mutable tagmanager.accounts.containers.workspaces.variables.create @mutable tagmanager.accounts.containers.workspaces.variables.update
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The Variable ID uniquely identifies the GTM Variable.
    #[serde(rename="variableId")]
    
    pub variable_id: Option<String>,
    /// GTM Workspace ID.
    #[serde(rename="workspaceId")]
    
    pub workspace_id: Option<String>,
}

impl client::RequestValue for Variable {}
impl client::ResponseResult for Variable {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VariableFormatValue {
    /// The option to convert a string-type variable value to either lowercase or uppercase.
    #[serde(rename="caseConversionType")]
    
    pub case_conversion_type: Option<VariableFormatValueCaseConversionTypeEnum>,
    /// The value to convert if a variable value is false.
    #[serde(rename="convertFalseToValue")]
    
    pub convert_false_to_value: Option<Parameter>,
    /// The value to convert if a variable value is null.
    #[serde(rename="convertNullToValue")]
    
    pub convert_null_to_value: Option<Parameter>,
    /// The value to convert if a variable value is true.
    #[serde(rename="convertTrueToValue")]
    
    pub convert_true_to_value: Option<Parameter>,
    /// The value to convert if a variable value is undefined.
    #[serde(rename="convertUndefinedToValue")]
    
    pub convert_undefined_to_value: Option<Parameter>,
}

impl client::Part for VariableFormatValue {}


/// Represents a Google Tag Manager Container Workspace.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers workspaces create accounts](AccountContainerWorkspaceCreateCall) (request|response)
/// * [containers workspaces get accounts](AccountContainerWorkspaceGetCall) (response)
/// * [containers workspaces update accounts](AccountContainerWorkspaceUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Workspace {
    /// GTM Account ID.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// GTM Container ID.
    #[serde(rename="containerId")]
    
    pub container_id: Option<String>,
    /// Workspace description. @mutable tagmanager.accounts.containers.workspaces.create @mutable tagmanager.accounts.containers.workspaces.update
    
    pub description: Option<String>,
    /// The fingerprint of the GTM Workspace as computed at storage time. This value is recomputed whenever the workspace is modified.
    
    pub fingerprint: Option<String>,
    /// Workspace display name. @mutable tagmanager.accounts.containers.workspaces.create @mutable tagmanager.accounts.containers.workspaces.update
    
    pub name: Option<String>,
    /// GTM Workspace's API relative path.
    
    pub path: Option<String>,
    /// Auto generated link to the tag manager UI
    #[serde(rename="tagManagerUrl")]
    
    pub tag_manager_url: Option<String>,
    /// The Workspace ID uniquely identifies the GTM Workspace.
    #[serde(rename="workspaceId")]
    
    pub workspace_id: Option<String>,
}

impl client::RequestValue for Workspace {}
impl client::ResponseResult for Workspace {}


/// Represents a Google Tag Manager Zone’s contents.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers workspaces zones create accounts](AccountContainerWorkspaceZoneCreateCall) (request|response)
/// * [containers workspaces zones get accounts](AccountContainerWorkspaceZoneGetCall) (response)
/// * [containers workspaces zones update accounts](AccountContainerWorkspaceZoneUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Zone {
    /// GTM Account ID.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// This Zone's boundary.
    
    pub boundary: Option<ZoneBoundary>,
    /// Containers that are children of this Zone.
    #[serde(rename="childContainer")]
    
    pub child_container: Option<Vec<ZoneChildContainer>>,
    /// GTM Container ID.
    #[serde(rename="containerId")]
    
    pub container_id: Option<String>,
    /// The fingerprint of the GTM Zone as computed at storage time. This value is recomputed whenever the zone is modified.
    
    pub fingerprint: Option<String>,
    /// Zone display name.
    
    pub name: Option<String>,
    /// User notes on how to apply this zone in the container.
    
    pub notes: Option<String>,
    /// GTM Zone's API relative path.
    
    pub path: Option<String>,
    /// Auto generated link to the tag manager UI
    #[serde(rename="tagManagerUrl")]
    
    pub tag_manager_url: Option<String>,
    /// This Zone's type restrictions.
    #[serde(rename="typeRestriction")]
    
    pub type_restriction: Option<ZoneTypeRestriction>,
    /// GTM Workspace ID.
    #[serde(rename="workspaceId")]
    
    pub workspace_id: Option<String>,
    /// The Zone ID uniquely identifies the GTM Zone.
    #[serde(rename="zoneId")]
    
    pub zone_id: Option<String>,
}

impl client::RequestValue for Zone {}
impl client::ResponseResult for Zone {}


/// Represents a Zone's boundaries.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ZoneBoundary {
    /// The conditions that, when conjoined, make up the boundary.
    
    pub condition: Option<Vec<Condition>>,
    /// Custom evaluation trigger IDs. A zone will evaluate its boundary conditions when any of the listed triggers are true.
    #[serde(rename="customEvaluationTriggerId")]
    
    pub custom_evaluation_trigger_id: Option<Vec<String>>,
}

impl client::Part for ZoneBoundary {}


/// Represents a child container of a Zone.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ZoneChildContainer {
    /// The zone's nickname for the child container.
    
    pub nickname: Option<String>,
    /// The child container's public id.
    #[serde(rename="publicId")]
    
    pub public_id: Option<String>,
}

impl client::Part for ZoneChildContainer {}


/// Represents a Zone's type restrictions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ZoneTypeRestriction {
    /// True if type restrictions have been enabled for this Zone.
    
    pub enable: Option<bool>,
    /// List of type public ids that have been whitelisted for use in this Zone.
    #[serde(rename="whitelistedTypeId")]
    
    pub whitelisted_type_id: Option<Vec<String>>,
}

impl client::Part for ZoneTypeRestriction {}


