use super::*;
/// A list of authorized public keys for a user account.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuthorizedKeysView {
    /// [Output Only] The list of authorized public keys in SSH format.
    
    pub keys: Option<Vec<String>>,
    /// [Output Only] Whether the user has the ability to elevate on the instance that requested the authorized keys.
    
    pub sudoer: Option<bool>,
}

impl client::Part for AuthorizedKeysView {}


/// A Group resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [add member groups](GroupAddMemberCall) (none)
/// * [delete groups](GroupDeleteCall) (none)
/// * [get groups](GroupGetCall) (response)
/// * [insert groups](GroupInsertCall) (request)
/// * [list groups](GroupListCall) (none)
/// * [remove member groups](GroupRemoveMemberCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Group {
    /// [Output Only] Creation timestamp in RFC3339 text format.
    #[serde(rename="creationTimestamp")]
    
    pub creation_timestamp: Option<String>,
    /// An optional textual description of the resource; provided by the client when the resource is created.
    
    pub description: Option<String>,
    /// [Output Only] Unique identifier for the resource; defined by the server.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<u64>,
    /// [Output Only] Type of the resource. Always clouduseraccounts#group for groups.
    
    pub kind: Option<String>,
    /// [Output Only] A list of URLs to User resources who belong to the group. Users may only be members of groups in the same project.
    
    pub members: Option<Vec<String>>,
    /// Name of the resource; provided by the client when the resource is created.
    
    pub name: Option<String>,
    /// [Output Only] Server defined URL for the resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::RequestValue for Group {}
impl client::Resource for Group {}
impl client::ResponseResult for Group {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list groups](GroupListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GroupList {
    /// [Output Only] Unique identifier for the resource; defined by the server.
    
    pub id: Option<String>,
    /// [Output Only] A list of Group resources.
    
    pub items: Option<Vec<Group>>,
    /// [Output Only] Type of resource. Always clouduseraccounts#groupList for lists of groups.
    
    pub kind: Option<String>,
    /// [Output Only] A token used to continue a truncated list request.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// [Output Only] Server defined URL for this resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::ResponseResult for GroupList {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [add member groups](GroupAddMemberCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GroupsAddMemberRequest {
    /// Fully-qualified URLs of the User resources to add.
    
    pub users: Option<Vec<String>>,
}

impl client::RequestValue for GroupsAddMemberRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [remove member groups](GroupRemoveMemberCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GroupsRemoveMemberRequest {
    /// Fully-qualified URLs of the User resources to remove.
    
    pub users: Option<Vec<String>>,
}

impl client::RequestValue for GroupsRemoveMemberRequest {}


/// A list of all Linux accounts for this project. This API is only used by Compute Engine virtual machines to get information about user accounts for a project or instance. Linux resources are read-only views into users and groups managed by the Compute Engine Accounts API.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LinuxAccountViews {
    /// [Output Only] A list of all groups within a project.
    #[serde(rename="groupViews")]
    
    pub group_views: Option<Vec<LinuxGroupView>>,
    /// [Output Only] Type of the resource. Always clouduseraccounts#linuxAccountViews for Linux resources.
    
    pub kind: Option<String>,
    /// [Output Only] A list of all users within a project.
    #[serde(rename="userViews")]
    
    pub user_views: Option<Vec<LinuxUserView>>,
}

impl client::Part for LinuxAccountViews {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get authorized keys view linux](LinuxGetAuthorizedKeysViewCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LinuxGetAuthorizedKeysViewResponse {
    /// [Output Only] A list of authorized public keys for a user.
    
    pub resource: Option<AuthorizedKeysView>,
}

impl client::ResponseResult for LinuxGetAuthorizedKeysViewResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get linux account views linux](LinuxGetLinuxAccountViewCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LinuxGetLinuxAccountViewsResponse {
    /// [Output Only] A list of authorized user accounts and groups.
    
    pub resource: Option<LinuxAccountViews>,
}

impl client::ResponseResult for LinuxGetLinuxAccountViewsResponse {}


/// A detailed view of a Linux group.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LinuxGroupView {
    /// [Output Only] The Group ID.
    
    pub gid: Option<u32>,
    /// [Output Only] Group name.
    #[serde(rename="groupName")]
    
    pub group_name: Option<String>,
    /// [Output Only] List of user accounts that belong to the group.
    
    pub members: Option<Vec<String>>,
}

impl client::Part for LinuxGroupView {}


/// A detailed view of a Linux user account.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LinuxUserView {
    /// [Output Only] The GECOS (user information) entry for this account.
    
    pub gecos: Option<String>,
    /// [Output Only] User's default group ID.
    
    pub gid: Option<u32>,
    /// [Output Only] The path to the home directory for this account.
    #[serde(rename="homeDirectory")]
    
    pub home_directory: Option<String>,
    /// [Output Only] The path to the login shell for this account.
    
    pub shell: Option<String>,
    /// [Output Only] User ID.
    
    pub uid: Option<u32>,
    /// [Output Only] The username of the account.
    
    pub username: Option<String>,
}

impl client::Part for LinuxUserView {}


/// An Operation resource, used to manage asynchronous API requests.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get global accounts operations](GlobalAccountsOperationGetCall) (response)
/// * [add member groups](GroupAddMemberCall) (response)
/// * [delete groups](GroupDeleteCall) (response)
/// * [insert groups](GroupInsertCall) (response)
/// * [remove member groups](GroupRemoveMemberCall) (response)
/// * [add public key users](UserAddPublicKeyCall) (response)
/// * [delete users](UserDeleteCall) (response)
/// * [insert users](UserInsertCall) (response)
/// * [remove public key users](UserRemovePublicKeyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// [Output Only] Reserved for future use.
    #[serde(rename="clientOperationId")]
    
    pub client_operation_id: Option<String>,
    /// [Output Only] Creation timestamp in RFC3339 text format.
    #[serde(rename="creationTimestamp")]
    
    pub creation_timestamp: Option<String>,
    /// [Output Only] A textual description of the operation, which is set when the operation is created.
    
    pub description: Option<String>,
    /// [Output Only] The time that this operation was completed. This value is in RFC3339 text format.
    #[serde(rename="endTime")]
    
    pub end_time: Option<String>,
    /// [Output Only] If errors are generated during processing of the operation, this field will be populated.
    
    pub error: Option<OperationError>,
    /// [Output Only] If the operation fails, this field contains the HTTP error message that was returned, such as NOT FOUND.
    #[serde(rename="httpErrorMessage")]
    
    pub http_error_message: Option<String>,
    /// [Output Only] If the operation fails, this field contains the HTTP error status code that was returned. For example, a 404 means the resource was not found.
    #[serde(rename="httpErrorStatusCode")]
    
    pub http_error_status_code: Option<i32>,
    /// [Output Only] The unique identifier for the resource. This identifier is defined by the server.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<u64>,
    /// [Output Only] The time that this operation was requested. This value is in RFC3339 text format.
    #[serde(rename="insertTime")]
    
    pub insert_time: Option<String>,
    /// [Output Only] Type of the resource. Always compute#operation for Operation resources.
    
    pub kind: Option<String>,
    /// [Output Only] Name of the resource.
    
    pub name: Option<String>,
    /// [Output Only] The type of operation, such as insert, update, or delete, and so on.
    #[serde(rename="operationType")]
    
    pub operation_type: Option<String>,
    /// [Output Only] An optional progress indicator that ranges from 0 to 100. There is no requirement that this be linear or support any granularity of operations. This should not be used to guess when the operation will be complete. This number should monotonically increase as the operation progresses.
    
    pub progress: Option<i32>,
    /// [Output Only] The URL of the region where the operation resides. Only available when performing regional operations.
    
    pub region: Option<String>,
    /// [Output Only] Server-defined URL for the resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// [Output Only] The time that this operation was started by the server. This value is in RFC3339 text format.
    #[serde(rename="startTime")]
    
    pub start_time: Option<String>,
    /// [Output Only] The status of the operation, which can be one of the following: PENDING, RUNNING, or DONE.
    
    pub status: Option<OperationStatusEnum>,
    /// [Output Only] An optional textual description of the current status of the operation.
    #[serde(rename="statusMessage")]
    
    pub status_message: Option<String>,
    /// [Output Only] The unique target ID, which identifies a specific incarnation of the target resource.
    #[serde(rename="targetId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub target_id: Option<u64>,
    /// [Output Only] The URL of the resource that the operation modifies.
    #[serde(rename="targetLink")]
    
    pub target_link: Option<String>,
    /// [Output Only] User who requested the operation, for example: user@example.com.
    
    pub user: Option<String>,
    /// [Output Only] If warning messages are generated during processing of the operation, this field will be populated.
    
    pub warnings: Option<Vec<OperationWarnings>>,
    /// [Output Only] The URL of the zone where the operation resides. Only available when performing per-zone operations.
    
    pub zone: Option<String>,
}

impl client::ResponseResult for Operation {}


/// Contains a list of Operation resources.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list global accounts operations](GlobalAccountsOperationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperationList {
    /// [Output Only] The unique identifier for the resource. This identifier is defined by the server.
    
    pub id: Option<String>,
    /// [Output Only] A list of Operation resources.
    
    pub items: Option<Vec<Operation>>,
    /// [Output Only] Type of resource. Always compute#operations for Operations resource.
    
    pub kind: Option<String>,
    /// [Output Only] This token allows you to get the next page of results for list requests. If the number of results is larger than maxResults, use the nextPageToken as a value for the query parameter pageToken in the next list request. Subsequent list requests will have their own nextPageToken to continue paging through the results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// [Output Only] Server-defined URL for this resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::ResponseResult for OperationList {}


/// A public key for authenticating to guests.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [add public key users](UserAddPublicKeyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PublicKey {
    /// [Output Only] Creation timestamp in RFC3339 text format.
    #[serde(rename="creationTimestamp")]
    
    pub creation_timestamp: Option<String>,
    /// An optional textual description of the resource; provided by the client when the resource is created.
    
    pub description: Option<String>,
    /// Optional expiration timestamp. If provided, the timestamp must be in RFC3339 text format. If not provided, the public key never expires.
    #[serde(rename="expirationTimestamp")]
    
    pub expiration_timestamp: Option<String>,
    /// [Output Only] The fingerprint of the key is defined by RFC4716 to be the MD5 digest of the public key.
    
    pub fingerprint: Option<String>,
    /// Public key text in SSH format, defined by RFC4253 section 6.6.
    
    pub key: Option<String>,
}

impl client::RequestValue for PublicKey {}


/// A User resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [add public key users](UserAddPublicKeyCall) (none)
/// * [delete users](UserDeleteCall) (none)
/// * [get users](UserGetCall) (response)
/// * [insert users](UserInsertCall) (request)
/// * [list users](UserListCall) (none)
/// * [remove public key users](UserRemovePublicKeyCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct User {
    /// [Output Only] Creation timestamp in RFC3339 text format.
    #[serde(rename="creationTimestamp")]
    
    pub creation_timestamp: Option<String>,
    /// An optional textual description of the resource; provided by the client when the resource is created.
    
    pub description: Option<String>,
    /// [Output Only] A list of URLs to Group resources who contain the user. Users are only members of groups in the same project.
    
    pub groups: Option<Vec<String>>,
    /// [Output Only] Unique identifier for the resource; defined by the server.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<u64>,
    /// [Output Only] Type of the resource. Always clouduseraccounts#user for users.
    
    pub kind: Option<String>,
    /// Name of the resource; provided by the client when the resource is created.
    
    pub name: Option<String>,
    /// Email address of account's owner. This account will be validated to make sure it exists. The email can belong to any domain, but it must be tied to a Google account.
    
    pub owner: Option<String>,
    /// [Output Only] Public keys that this user may use to login.
    #[serde(rename="publicKeys")]
    
    pub public_keys: Option<Vec<PublicKey>>,
    /// [Output Only] Server defined URL for the resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::RequestValue for User {}
impl client::Resource for User {}
impl client::ResponseResult for User {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list users](UserListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserList {
    /// [Output Only] Unique identifier for the resource; defined by the server.
    
    pub id: Option<String>,
    /// [Output Only] A list of User resources.
    
    pub items: Option<Vec<User>>,
    /// [Output Only] Type of resource. Always clouduseraccounts#userList for lists of users.
    
    pub kind: Option<String>,
    /// [Output Only] A token used to continue a truncated list request.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// [Output Only] Server defined URL for this resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::ResponseResult for UserList {}


/// [Output Only] If errors are generated during processing of the operation, this field will be populated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperationError {
    /// [Output Only] The array of errors encountered while processing this operation.
    
    pub errors: Option<Vec<OperationErrorErrors>>,
}

impl client::NestedType for OperationError {}
impl client::Part for OperationError {}


/// [Output Only] The array of errors encountered while processing this operation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperationErrorErrors {
    /// [Output Only] The error type identifier for this error.
    
    pub code: Option<String>,
    /// [Output Only] Indicates the field in the request that caused the error. This property is optional.
    
    pub location: Option<String>,
    /// [Output Only] An optional, human-readable error message.
    
    pub message: Option<String>,
}

impl client::NestedType for OperationErrorErrors {}
impl client::Part for OperationErrorErrors {}


/// [Output Only] If warning messages are generated during processing of the operation, this field will be populated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperationWarnings {
    /// [Output Only] A warning code, if applicable. For example, Compute Engine returns NO_RESULTS_ON_PAGE if there are no results in the response.
    
    pub code: Option<OperationWarningCodeEnum>,
    /// [Output Only] Metadata about this warning in key: value format. For example:
    /// "data": [ { "key": "scope", "value": "zones/us-east1-d" }
    
    pub data: Option<Vec<OperationWarningsData>>,
    /// [Output Only] A human-readable description of the warning code.
    
    pub message: Option<String>,
}

impl client::NestedType for OperationWarnings {}
impl client::Part for OperationWarnings {}


/// [Output Only] Metadata about this warning in key: value format. For example:
/// "data": [ { "key": "scope", "value": "zones/us-east1-d" }
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperationWarningsData {
    /// [Output Only] A key that provides more detail on the warning being returned. For example, for warnings where there are no results in a list request for a particular zone, this key might be scope and the key value might be the zone name. Other examples might be a key indicating a deprecated resource and a suggested replacement, or a warning about invalid network settings (for example, if an instance attempts to perform IP forwarding but is not enabled for IP forwarding).
    
    pub key: Option<String>,
    /// [Output Only] A warning data value corresponding to the key.
    
    pub value: Option<String>,
}

impl client::NestedType for OperationWarningsData {}
impl client::Part for OperationWarningsData {}


