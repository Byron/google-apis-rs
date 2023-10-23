use super::*;
/// Specifies the audit configuration for a service. The configuration determines which permission types are logged, and what identities, if any, are exempted from logging. An AuditConfig must have one or more AuditLogConfigs. If there are AuditConfigs for both `allServices` and a specific service, the union of the two AuditConfigs is used for that service: the log_types specified in each AuditConfig are enabled, and the exempted_members in each AuditLogConfig are exempted. Example Policy with multiple AuditConfigs: { "audit_configs": [ { "service": "allServices", "audit_log_configs": [ { "log_type": "DATA_READ", "exempted_members": [ "user:jose@example.com" ] }, { "log_type": "DATA_WRITE" }, { "log_type": "ADMIN_READ" } ] }, { "service": "sampleservice.googleapis.com", "audit_log_configs": [ { "log_type": "DATA_READ" }, { "log_type": "DATA_WRITE", "exempted_members": [ "user:aliya@example.com" ] } ] } ] } For sampleservice, this policy enables DATA_READ, DATA_WRITE and ADMIN_READ logging. It also exempts `jose@example.com` from DATA_READ logging, and `aliya@example.com` from DATA_WRITE logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuditConfig {
    /// The configuration for logging of each type of permission.
    #[serde(rename="auditLogConfigs")]
    
    pub audit_log_configs: Option<Vec<AuditLogConfig>>,
    /// Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services.
    
    pub service: Option<String>,
}

impl client::Part for AuditConfig {}


/// Provides the configuration for logging a type of permissions. Example: { "audit_log_configs": [ { "log_type": "DATA_READ", "exempted_members": [ "user:jose@example.com" ] }, { "log_type": "DATA_WRITE" } ] } This enables 'DATA_READ' and 'DATA_WRITE' logging, while exempting jose@example.com from DATA_READ logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuditLogConfig {
    /// Specifies the identities that do not cause logging for this type of permission. Follows the same format of Binding.members.
    #[serde(rename="exemptedMembers")]
    
    pub exempted_members: Option<Vec<String>>,
    /// The log type that this config enables.
    #[serde(rename="logType")]
    
    pub log_type: Option<AuditLogConfigLogTypeEnum>,
}

impl client::Part for AuditLogConfig {}


/// Associates `members`, or principals, with a `role`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Binding {
    /// The condition that is associated with this binding. If the condition evaluates to `true`, then this binding applies to the current request. If the condition evaluates to `false`, then this binding does not apply to the current request. However, a different role binding might grant the same role to one or more of the principals in this binding. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub condition: Option<Expr>,
    /// Specifies the principals requesting access for a Google Cloud resource. `members` can have the following values: * `allUsers`: A special identifier that represents anyone who is on the internet; with or without a Google account. * `allAuthenticatedUsers`: A special identifier that represents anyone who is authenticated with a Google account or a service account. Does not include identities that come from external identity providers (IdPs) through identity federation. * `user:{emailid}`: An email address that represents a specific Google account. For example, `alice@example.com` . * `serviceAccount:{emailid}`: An email address that represents a Google service account. For example, `my-other-app@appspot.gserviceaccount.com`. * `serviceAccount:{projectid}.svc.id.goog[{namespace}/{kubernetes-sa}]`: An identifier for a [Kubernetes service account](https://cloud.google.com/kubernetes-engine/docs/how-to/kubernetes-service-accounts). For example, `my-project.svc.id.goog[my-namespace/my-kubernetes-sa]`. * `group:{emailid}`: An email address that represents a Google group. For example, `admins@example.com`. * `deleted:user:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a user that has been recently deleted. For example, `alice@example.com?uid=123456789012345678901`. If the user is recovered, this value reverts to `user:{emailid}` and the recovered user retains the role in the binding. * `deleted:serviceAccount:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a service account that has been recently deleted. For example, `my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901`. If the service account is undeleted, this value reverts to `serviceAccount:{emailid}` and the undeleted service account retains the role in the binding. * `deleted:group:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a Google group that has been recently deleted. For example, `admins@example.com?uid=123456789012345678901`. If the group is recovered, this value reverts to `group:{emailid}` and the recovered group retains the role in the binding. * `domain:{domain}`: The G Suite domain (primary) that represents all the users of that domain. For example, `google.com` or `example.com`. 
    
    pub members: Option<Vec<String>>,
    /// Role that is assigned to the list of `members`, or principals. For example, `roles/viewer`, `roles/editor`, or `roles/owner`.
    
    pub role: Option<String>,
}

impl client::Part for Binding {}


/// An EffectiveTag represents a tag that applies to a resource during policy evaluation. Tags can be either directly bound to a resource or inherited from its ancestor. EffectiveTag contains the name and namespaced_name of the tag value and tag key, with additional fields of `inherited` to indicate the inheritance status of the effective tag.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list effective tags](EffectiveTagListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EffectiveTag {
    /// Indicates the inheritance status of a tag value attached to the given resource. If the tag value is inherited from one of the resource's ancestors, inherited will be true. If false, then the tag value is directly attached to the resource, inherited will be false.
    
    pub inherited: Option<bool>,
    /// The namespaced_name of the TagKey. Now only supported in the format of `{organization_id}/{tag_key_short_name}`. Other formats will be supported when we add non-org parented tags.
    #[serde(rename="namespacedTagKey")]
    
    pub namespaced_tag_key: Option<String>,
    /// Namespaced name of the TagValue. Now only supported in the format `{organization_id}/{tag_key_short_name}/{tag_value_short_name}`. Other formats will be supported when we add non-org parented tags.
    #[serde(rename="namespacedTagValue")]
    
    pub namespaced_tag_value: Option<String>,
    /// The name of the TagKey, in the format `tagKeys/{id}`, such as `tagKeys/123`.
    #[serde(rename="tagKey")]
    
    pub tag_key: Option<String>,
    /// Resource name for TagValue in the format `tagValues/456`.
    #[serde(rename="tagValue")]
    
    pub tag_value: Option<String>,
}

impl client::Resource for EffectiveTag {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete liens](LienDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language. The syntax and semantics of CEL are documented at https://github.com/google/cel-spec. Example (Comparison): title: "Summary size limit" description: "Determines if a summary is less than 100 chars" expression: "document.summary.size() < 100" Example (Equality): title: "Requestor is owner" description: "Determines if requestor is the document owner" expression: "document.owner == request.auth.claims.email" Example (Logic): title: "Public documents" description: "Determine whether the document should be publicly visible" expression: "document.type != 'private' && document.type != 'internal'" Example (Data Manipulation): title: "Notification string" description: "Create a notification string with a timestamp." expression: "'New message received at ' + string(document.create_time)" The exact variables and functions that may be referenced within an expression are determined by the service that evaluates it. See the service documentation for additional information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Expr {
    /// Optional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI.
    
    pub description: Option<String>,
    /// Textual representation of an expression in Common Expression Language syntax.
    
    pub expression: Option<String>,
    /// Optional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file.
    
    pub location: Option<String>,
    /// Optional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression.
    
    pub title: Option<String>,
}

impl client::Part for Expr {}


/// A folder in an organization’s resource hierarchy, used to organize that organization’s resources.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create folders](FolderCreateCall) (request)
/// * [delete folders](FolderDeleteCall) (none)
/// * [get folders](FolderGetCall) (response)
/// * [get iam policy folders](FolderGetIamPolicyCall) (none)
/// * [list folders](FolderListCall) (none)
/// * [move folders](FolderMoveCall) (none)
/// * [patch folders](FolderPatchCall) (request)
/// * [search folders](FolderSearchCall) (none)
/// * [set iam policy folders](FolderSetIamPolicyCall) (none)
/// * [test iam permissions folders](FolderTestIamPermissionCall) (none)
/// * [undelete folders](FolderUndeleteCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Folder {
    /// Output only. Timestamp when the folder was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Timestamp when the folder was requested to be deleted.
    #[serde(rename="deleteTime")]
    
    pub delete_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The folder’s display name. A folder’s display name must be unique amongst its siblings. For example, no two folders with the same parent can share the same display name. The display name must start and end with a letter or digit, may contain letters, digits, spaces, hyphens and underscores and can be no longer than 30 characters. This is captured by the regular expression: `[\p{L}\p{N}]([\p{L}\p{N}_- ]{0,28}[\p{L}\p{N}])?`.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. A checksum computed by the server based on the current value of the folder resource. This may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
    
    pub etag: Option<String>,
    /// Output only. The resource name of the folder. Its format is `folders/{folder_id}`, for example: "folders/1234".
    
    pub name: Option<String>,
    /// Required. The folder's parent's resource name. Updates to the folder's parent must be performed using MoveFolder.
    
    pub parent: Option<String>,
    /// Output only. The lifecycle state of the folder. Updates to the state must be performed using DeleteFolder and UndeleteFolder.
    
    pub state: Option<FolderStateEnum>,
    /// Output only. Timestamp when the folder was last modified.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Folder {}
impl client::Resource for Folder {}
impl client::ResponseResult for Folder {}


/// Request message for `GetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get iam policy folders](FolderGetIamPolicyCall) (request)
/// * [get iam policy organizations](OrganizationGetIamPolicyCall) (request)
/// * [get iam policy projects](ProjectGetIamPolicyCall) (request)
/// * [get iam policy tag keys](TagKeyGetIamPolicyCall) (request)
/// * [get iam policy tag values](TagValueGetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetIamPolicyRequest {
    /// OPTIONAL: A `GetPolicyOptions` object for specifying options to `GetIamPolicy`.
    
    pub options: Option<GetPolicyOptions>,
}

impl client::RequestValue for GetIamPolicyRequest {}


/// Encapsulates settings provided to GetIamPolicy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetPolicyOptions {
    /// Optional. The maximum policy version that will be used to format the policy. Valid values are 0, 1, and 3. Requests specifying an invalid value will be rejected. Requests for policies with any conditional role bindings must specify version 3. Policies with no conditional role bindings may specify any valid value or leave the field unset. The policy in the response might use the policy version that you specified, or it might use a lower policy version. For example, if you specify version 3, but the policy has no conditional role bindings, the response uses version 1. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    #[serde(rename="requestedPolicyVersion")]
    
    pub requested_policy_version: Option<i32>,
}

impl client::Part for GetPolicyOptions {}


/// A Lien represents an encumbrance on the actions that can be performed on a resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create liens](LienCreateCall) (request|response)
/// * [delete liens](LienDeleteCall) (none)
/// * [get liens](LienGetCall) (response)
/// * [list liens](LienListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Lien {
    /// The creation time of this Lien.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A system-generated unique identifier for this Lien. Example: `liens/1234abcd`
    
    pub name: Option<String>,
    /// A stable, user-visible/meaningful string identifying the origin of the Lien, intended to be inspected programmatically. Maximum length of 200 characters. Example: 'compute.googleapis.com'
    
    pub origin: Option<String>,
    /// A reference to the resource this Lien is attached to. The server will validate the parent against those for which Liens are supported. Example: `projects/1234`
    
    pub parent: Option<String>,
    /// Concise user-visible strings indicating why an action cannot be performed on a resource. Maximum length of 200 characters. Example: 'Holds production API key'
    
    pub reason: Option<String>,
    /// The types of operations which should be blocked as a result of this Lien. Each value should correspond to an IAM permission. The server will validate the permissions against those for which Liens are supported. An empty list is meaningless and will be rejected. Example: ['resourcemanager.projects.delete']
    
    pub restrictions: Option<Vec<String>>,
}

impl client::RequestValue for Lien {}
impl client::Resource for Lien {}
impl client::ResponseResult for Lien {}


/// The response of ListEffectiveTags.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list effective tags](EffectiveTagListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListEffectiveTagsResponse {
    /// A possibly paginated list of effective tags for the specified resource.
    #[serde(rename="effectiveTags")]
    
    pub effective_tags: Option<Vec<EffectiveTag>>,
    /// Pagination token. If the result set is too large to fit in a single response, this token is returned. It encodes the position of the current result cursor. Feeding this value into a new list request with the `page_token` parameter gives the next page of the results. When `next_page_token` is not filled in, there is no next page and the list returned is the last page in the result set. Pagination tokens have a limited lifetime.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListEffectiveTagsResponse {}


/// The ListFolders response message.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list folders](FolderListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListFoldersResponse {
    /// A possibly paginated list of folders that are direct descendants of the specified parent resource.
    
    pub folders: Option<Vec<Folder>>,
    /// A pagination token returned from a previous call to `ListFolders` that indicates from where listing should continue.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListFoldersResponse {}


/// The response message for Liens.ListLiens.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list liens](LienListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListLiensResponse {
    /// A list of Liens.
    
    pub liens: Option<Vec<Lien>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListLiensResponse {}


/// A page of the response received from the ListProjects method. A paginated response where more pages are available has `next_page_token` set. This token can be used in a subsequent request to retrieve the next request page. NOTE: A response may contain fewer elements than the request `page_size` and still have a `next_page_token`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list projects](ProjectListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListProjectsResponse {
    /// Pagination token. If the result set is too large to fit in a single response, this token is returned. It encodes the position of the current result cursor. Feeding this value into a new list request with the `page_token` parameter gives the next page of the results. When `next_page_token` is not filled in, there is no next page and the list returned is the last page in the result set. Pagination tokens have a limited lifetime.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of Projects under the parent. This list can be paginated.
    
    pub projects: Option<Vec<Project>>,
}

impl client::ResponseResult for ListProjectsResponse {}


/// The ListTagBindings response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list tag bindings](TagBindingListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTagBindingsResponse {
    /// Pagination token. If the result set is too large to fit in a single response, this token is returned. It encodes the position of the current result cursor. Feeding this value into a new list request with the `page_token` parameter gives the next page of the results. When `next_page_token` is not filled in, there is no next page and the list returned is the last page in the result set. Pagination tokens have a limited lifetime.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A possibly paginated list of TagBindings for the specified resource.
    #[serde(rename="tagBindings")]
    
    pub tag_bindings: Option<Vec<TagBinding>>,
}

impl client::ResponseResult for ListTagBindingsResponse {}


/// The ListTagHolds response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [tag holds list tag values](TagValueTagHoldListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTagHoldsResponse {
    /// Pagination token. If the result set is too large to fit in a single response, this token is returned. It encodes the position of the current result cursor. Feeding this value into a new list request with the `page_token` parameter gives the next page of the results. When `next_page_token` is not filled in, there is no next page and the list returned is the last page in the result set. Pagination tokens have a limited lifetime.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A possibly paginated list of TagHolds.
    #[serde(rename="tagHolds")]
    
    pub tag_holds: Option<Vec<TagHold>>,
}

impl client::ResponseResult for ListTagHoldsResponse {}


/// The ListTagKeys response message.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list tag keys](TagKeyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTagKeysResponse {
    /// A pagination token returned from a previous call to `ListTagKeys` that indicates from where listing should continue.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of TagKeys that live under the specified parent in the request.
    #[serde(rename="tagKeys")]
    
    pub tag_keys: Option<Vec<TagKey>>,
}

impl client::ResponseResult for ListTagKeysResponse {}


/// The ListTagValues response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list tag values](TagValueListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTagValuesResponse {
    /// A pagination token returned from a previous call to `ListTagValues` that indicates from where listing should continue. This is currently not used, but the server may at any point start supplying a valid token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A possibly paginated list of TagValues that are direct descendants of the specified parent TagKey.
    #[serde(rename="tagValues")]
    
    pub tag_values: Option<Vec<TagValue>>,
}

impl client::ResponseResult for ListTagValuesResponse {}


/// The MoveFolder request message.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [move folders](FolderMoveCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MoveFolderRequest {
    /// Required. The resource name of the folder or organization which should be the folder's new parent. Must be of the form `folders/{folder_id}` or `organizations/{org_id}`.
    #[serde(rename="destinationParent")]
    
    pub destination_parent: Option<String>,
}

impl client::RequestValue for MoveFolderRequest {}


/// The request sent to MoveProject method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [move projects](ProjectMoveCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MoveProjectRequest {
    /// Required. The new parent to move the Project under.
    #[serde(rename="destinationParent")]
    
    pub destination_parent: Option<String>,
}

impl client::RequestValue for MoveProjectRequest {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create folders](FolderCreateCall) (response)
/// * [delete folders](FolderDeleteCall) (response)
/// * [move folders](FolderMoveCall) (response)
/// * [patch folders](FolderPatchCall) (response)
/// * [undelete folders](FolderUndeleteCall) (response)
/// * [get operations](OperationGetCall) (response)
/// * [create projects](ProjectCreateCall) (response)
/// * [delete projects](ProjectDeleteCall) (response)
/// * [move projects](ProjectMoveCall) (response)
/// * [patch projects](ProjectPatchCall) (response)
/// * [undelete projects](ProjectUndeleteCall) (response)
/// * [create tag bindings](TagBindingCreateCall) (response)
/// * [delete tag bindings](TagBindingDeleteCall) (response)
/// * [create tag keys](TagKeyCreateCall) (response)
/// * [delete tag keys](TagKeyDeleteCall) (response)
/// * [patch tag keys](TagKeyPatchCall) (response)
/// * [tag holds create tag values](TagValueTagHoldCreateCall) (response)
/// * [tag holds delete tag values](TagValueTagHoldDeleteCall) (response)
/// * [create tag values](TagValueCreateCall) (response)
/// * [delete tag values](TagValueDeleteCall) (response)
/// * [patch tag values](TagValuePatchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available.
    
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    
    pub error: Option<Status>,
    /// Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`.
    
    pub name: Option<String>,
    /// The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`.
    
    pub response: Option<HashMap<String, json::Value>>,
}

impl client::Resource for Operation {}
impl client::ResponseResult for Operation {}


/// The root node in the resource hierarchy to which a particular entity’s (a company, for example) resources belong.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get organizations](OrganizationGetCall) (response)
/// * [get iam policy organizations](OrganizationGetIamPolicyCall) (none)
/// * [search organizations](OrganizationSearchCall) (none)
/// * [set iam policy organizations](OrganizationSetIamPolicyCall) (none)
/// * [test iam permissions organizations](OrganizationTestIamPermissionCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Organization {
    /// Output only. Timestamp when the Organization was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Timestamp when the Organization was requested for deletion.
    #[serde(rename="deleteTime")]
    
    pub delete_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Immutable. The G Suite / Workspace customer id used in the Directory API.
    #[serde(rename="directoryCustomerId")]
    
    pub directory_customer_id: Option<String>,
    /// Output only. A human-readable string that refers to the organization in the Google Cloud Console. This string is set by the server and cannot be changed. The string will be set to the primary domain (for example, "google.com") of the Google Workspace customer that owns the organization.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. A checksum computed by the server based on the current value of the Organization resource. This may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
    
    pub etag: Option<String>,
    /// Output only. The resource name of the organization. This is the organization's relative path in the API. Its format is "organizations/[organization_id]". For example, "organizations/1234".
    
    pub name: Option<String>,
    /// Output only. The organization's current lifecycle state.
    
    pub state: Option<OrganizationStateEnum>,
    /// Output only. Timestamp when the Organization was last modified.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Resource for Organization {}
impl client::ResponseResult for Organization {}


/// An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members`, or principals, to a single `role`. Principals can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { “bindings”: \[ { “role”: “roles/resourcemanager.organizationAdmin”, “members”: \[ “user:mike@example.com”, “group:admins@example.com”, “domain:google.com”, “serviceAccount:my-project-id@appspot.gserviceaccount.com” \] }, { “role”: “roles/resourcemanager.organizationViewer”, “members”: \[ “user:eve@example.com” \], “condition”: { “title”: “expirable access”, “description”: “Does not grant access after Sep 2020”, “expression”: “request.time \< timestamp(‘2020-10-01T00:00:00.000Z’)”, } } \], “etag”: “BwWWja0YfJA=”, “version”: 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time \< timestamp(‘2020-10-01T00:00:00.000Z’) etag: BwWWja0YfJA= version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get iam policy folders](FolderGetIamPolicyCall) (response)
/// * [set iam policy folders](FolderSetIamPolicyCall) (response)
/// * [get iam policy organizations](OrganizationGetIamPolicyCall) (response)
/// * [set iam policy organizations](OrganizationSetIamPolicyCall) (response)
/// * [get iam policy projects](ProjectGetIamPolicyCall) (response)
/// * [set iam policy projects](ProjectSetIamPolicyCall) (response)
/// * [get iam policy tag keys](TagKeyGetIamPolicyCall) (response)
/// * [set iam policy tag keys](TagKeySetIamPolicyCall) (response)
/// * [get iam policy tag values](TagValueGetIamPolicyCall) (response)
/// * [set iam policy tag values](TagValueSetIamPolicyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Policy {
    /// Specifies cloud audit logging configuration for this policy.
    #[serde(rename="auditConfigs")]
    
    pub audit_configs: Option<Vec<AuditConfig>>,
    /// Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`.
    
    pub bindings: Option<Vec<Binding>>,
    /// `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
    /// Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub version: Option<i32>,
}

impl client::ResponseResult for Policy {}


/// A project is a high-level Google Cloud entity. It is a container for ACLs, APIs, App Engine Apps, VMs, and other Google Cloud Platform resources.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create projects](ProjectCreateCall) (request)
/// * [delete projects](ProjectDeleteCall) (none)
/// * [get projects](ProjectGetCall) (response)
/// * [get iam policy projects](ProjectGetIamPolicyCall) (none)
/// * [list projects](ProjectListCall) (none)
/// * [move projects](ProjectMoveCall) (none)
/// * [patch projects](ProjectPatchCall) (request)
/// * [search projects](ProjectSearchCall) (none)
/// * [set iam policy projects](ProjectSetIamPolicyCall) (none)
/// * [test iam permissions projects](ProjectTestIamPermissionCall) (none)
/// * [undelete projects](ProjectUndeleteCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Project {
    /// Output only. Creation time.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The time at which this resource was requested for deletion.
    #[serde(rename="deleteTime")]
    
    pub delete_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. A user-assigned display name of the project. When present it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, single-quote, double-quote, space, and exclamation point. Example: `My Project`
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. A checksum computed by the server based on the current value of the Project resource. This may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
    
    pub etag: Option<String>,
    /// Optional. The labels associated with this project. Label keys must be between 1 and 63 characters long and must conform to the following regular expression: \[a-z\](\[-a-z0-9\]*\[a-z0-9\])?. Label values must be between 0 and 63 characters long and must conform to the regular expression (\[a-z\](\[-a-z0-9\]*\[a-z0-9\])?)?. No more than 64 labels can be associated with a given resource. Clients should store labels in a representation such as JSON that does not depend on specific characters being disallowed. Example: `"myBusinessDimension" : "businessValue"`
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. The unique resource name of the project. It is an int64 generated number prefixed by "projects/". Example: `projects/415104041262`
    
    pub name: Option<String>,
    /// Optional. A reference to a parent Resource. eg., `organizations/123` or `folders/876`.
    
    pub parent: Option<String>,
    /// Immutable. The unique, user-assigned id of the project. It must be 6 to 30 lowercase ASCII letters, digits, or hyphens. It must start with a letter. Trailing hyphens are prohibited. Example: `tokyo-rain-123`
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Output only. The project lifecycle state.
    
    pub state: Option<ProjectStateEnum>,
    /// Output only. The most recent time this resource was modified.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Project {}
impl client::Resource for Project {}
impl client::ResponseResult for Project {}


/// The response message for searching folders.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search folders](FolderSearchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchFoldersResponse {
    /// A possibly paginated folder search results. the specified parent resource.
    
    pub folders: Option<Vec<Folder>>,
    /// A pagination token returned from a previous call to `SearchFolders` that indicates from where searching should continue.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for SearchFoldersResponse {}


/// The response returned from the `SearchOrganizations` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search organizations](OrganizationSearchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchOrganizationsResponse {
    /// A pagination token to be used to retrieve the next page of results. If the result is too large to fit within the page size specified in the request, this field will be set with a token that can be used to fetch the next page of results. If this field is empty, it indicates that this response contains the last page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of Organizations that matched the search query, possibly paginated.
    
    pub organizations: Option<Vec<Organization>>,
}

impl client::ResponseResult for SearchOrganizationsResponse {}


/// A page of the response received from the SearchProjects method. A paginated response where more pages are available has `next_page_token` set. This token can be used in a subsequent request to retrieve the next request page.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search projects](ProjectSearchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchProjectsResponse {
    /// Pagination token. If the result set is too large to fit in a single response, this token is returned. It encodes the position of the current result cursor. Feeding this value into a new list request with the `page_token` parameter gives the next page of the results. When `next_page_token` is not filled in, there is no next page and the list returned is the last page in the result set. Pagination tokens have a limited lifetime.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of Projects that matched the list filter query. This list can be paginated.
    
    pub projects: Option<Vec<Project>>,
}

impl client::ResponseResult for SearchProjectsResponse {}


/// Request message for `SetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [set iam policy folders](FolderSetIamPolicyCall) (request)
/// * [set iam policy organizations](OrganizationSetIamPolicyCall) (request)
/// * [set iam policy projects](ProjectSetIamPolicyCall) (request)
/// * [set iam policy tag keys](TagKeySetIamPolicyCall) (request)
/// * [set iam policy tag values](TagValueSetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetIamPolicyRequest {
    /// REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them.
    
    pub policy: Option<Policy>,
    /// OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: "bindings, etag"`
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for SetIamPolicyRequest {}


/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Status {
    /// The status code, which should be an enum value of google.rpc.Code.
    
    pub code: Option<i32>,
    /// A list of messages that carry the error details. There is a common set of message types for APIs to use.
    
    pub details: Option<Vec<HashMap<String, json::Value>>>,
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client.
    
    pub message: Option<String>,
}

impl client::Part for Status {}


/// A TagBinding represents a connection between a TagValue and a cloud resource Once a TagBinding is created, the TagValue is applied to all the descendants of the Google Cloud resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create tag bindings](TagBindingCreateCall) (request)
/// * [delete tag bindings](TagBindingDeleteCall) (none)
/// * [list tag bindings](TagBindingListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TagBinding {
    /// Output only. The name of the TagBinding. This is a String of the form: `tagBindings/{full-resource-name}/{tag-value-name}` (e.g. `tagBindings/%2F%2Fcloudresourcemanager.googleapis.com%2Fprojects%2F123/tagValues/456`).
    
    pub name: Option<String>,
    /// The full resource name of the resource the TagValue is bound to. E.g. `//cloudresourcemanager.googleapis.com/projects/123`
    
    pub parent: Option<String>,
    /// The TagValue of the TagBinding. Must be of the form `tagValues/456`.
    #[serde(rename="tagValue")]
    
    pub tag_value: Option<String>,
}

impl client::RequestValue for TagBinding {}
impl client::Resource for TagBinding {}


/// A TagHold represents the use of a TagValue that is not captured by TagBindings. If a TagValue has any TagHolds, deletion will be blocked. This resource is intended to be created in the same cloud location as the `holder`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [tag holds create tag values](TagValueTagHoldCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TagHold {
    /// Output only. The time this TagHold was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. A URL where an end user can learn more about removing this hold. E.g. `https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing`
    #[serde(rename="helpLink")]
    
    pub help_link: Option<String>,
    /// Required. The name of the resource where the TagValue is being used. Must be less than 200 characters. E.g. `//compute.googleapis.com/compute/projects/myproject/regions/us-east-1/instanceGroupManagers/instance-group`
    
    pub holder: Option<String>,
    /// Output only. The resource name of a TagHold. This is a String of the form: `tagValues/{tag-value-id}/tagHolds/{tag-hold-id}` (e.g. `tagValues/123/tagHolds/456`). This resource name is generated by the server.
    
    pub name: Option<String>,
    /// Optional. An optional string representing the origin of this request. This field should include human-understandable information to distinguish origins from each other. Must be less than 200 characters. E.g. `migs-35678234`
    
    pub origin: Option<String>,
}

impl client::RequestValue for TagHold {}


/// A TagKey, used to group a set of TagValues.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create tag keys](TagKeyCreateCall) (request)
/// * [delete tag keys](TagKeyDeleteCall) (none)
/// * [get tag keys](TagKeyGetCall) (response)
/// * [get iam policy tag keys](TagKeyGetIamPolicyCall) (none)
/// * [list tag keys](TagKeyListCall) (none)
/// * [patch tag keys](TagKeyPatchCall) (request)
/// * [set iam policy tag keys](TagKeySetIamPolicyCall) (none)
/// * [test iam permissions tag keys](TagKeyTestIamPermissionCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TagKey {
    /// Output only. Creation time.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. User-assigned description of the TagKey. Must not exceed 256 characters. Read-write.
    
    pub description: Option<String>,
    /// Optional. Entity tag which users can pass to prevent race conditions. This field is always set in server responses. See UpdateTagKeyRequest for details.
    
    pub etag: Option<String>,
    /// Immutable. The resource name for a TagKey. Must be in the format `tagKeys/{tag_key_id}`, where `tag_key_id` is the generated numeric id for the TagKey.
    
    pub name: Option<String>,
    /// Output only. Immutable. Namespaced name of the TagKey.
    #[serde(rename="namespacedName")]
    
    pub namespaced_name: Option<String>,
    /// Immutable. The resource name of the new TagKey's parent. Must be of the form `organizations/{org_id}`.
    
    pub parent: Option<String>,
    /// Optional. A purpose denotes that this Tag is intended for use in policies of a specific policy engine, and will involve that policy engine in management operations involving this Tag. A purpose does not grant a policy engine exclusive rights to the Tag, and it may be referenced by other policy engines. A purpose cannot be changed once set.
    
    pub purpose: Option<TagKeyPurposeEnum>,
    /// Optional. Purpose data corresponds to the policy system that the tag is intended for. See documentation for `Purpose` for formatting of this field. Purpose data cannot be changed once set.
    #[serde(rename="purposeData")]
    
    pub purpose_data: Option<HashMap<String, String>>,
    /// Required. Immutable. The user friendly name for a TagKey. The short name should be unique for TagKeys within the same tag namespace. The short name must be 1-63 characters, beginning and ending with an alphanumeric character ([a-z0-9A-Z]) with dashes (-), underscores (_), dots (.), and alphanumerics between.
    #[serde(rename="shortName")]
    
    pub short_name: Option<String>,
    /// Output only. Update time.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for TagKey {}
impl client::Resource for TagKey {}
impl client::ResponseResult for TagKey {}


/// A TagValue is a child of a particular TagKey. This is used to group cloud resources for the purpose of controlling them using policies.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [tag holds create tag values](TagValueTagHoldCreateCall) (none)
/// * [tag holds delete tag values](TagValueTagHoldDeleteCall) (none)
/// * [tag holds list tag values](TagValueTagHoldListCall) (none)
/// * [create tag values](TagValueCreateCall) (request)
/// * [delete tag values](TagValueDeleteCall) (none)
/// * [get tag values](TagValueGetCall) (response)
/// * [get iam policy tag values](TagValueGetIamPolicyCall) (none)
/// * [list tag values](TagValueListCall) (none)
/// * [patch tag values](TagValuePatchCall) (request)
/// * [set iam policy tag values](TagValueSetIamPolicyCall) (none)
/// * [test iam permissions tag values](TagValueTestIamPermissionCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TagValue {
    /// Output only. Creation time.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. User-assigned description of the TagValue. Must not exceed 256 characters. Read-write.
    
    pub description: Option<String>,
    /// Optional. Entity tag which users can pass to prevent race conditions. This field is always set in server responses. See UpdateTagValueRequest for details.
    
    pub etag: Option<String>,
    /// Immutable. Resource name for TagValue in the format `tagValues/456`.
    
    pub name: Option<String>,
    /// Output only. Namespaced name of the TagValue. Now only supported in the format `{organization_id}/{tag_key_short_name}/{short_name}`. Other formats will be supported when we add non-org parented tags.
    #[serde(rename="namespacedName")]
    
    pub namespaced_name: Option<String>,
    /// Immutable. The resource name of the new TagValue's parent TagKey. Must be of the form `tagKeys/{tag_key_id}`.
    
    pub parent: Option<String>,
    /// Required. Immutable. User-assigned short name for TagValue. The short name should be unique for TagValues within the same parent TagKey. The short name must be 63 characters or less, beginning and ending with an alphanumeric character ([a-z0-9A-Z]) with dashes (-), underscores (_), dots (.), and alphanumerics between.
    #[serde(rename="shortName")]
    
    pub short_name: Option<String>,
    /// Output only. Update time.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for TagValue {}
impl client::Resource for TagValue {}
impl client::ResponseResult for TagValue {}


/// Request message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [test iam permissions folders](FolderTestIamPermissionCall) (request)
/// * [test iam permissions organizations](OrganizationTestIamPermissionCall) (request)
/// * [test iam permissions projects](ProjectTestIamPermissionCall) (request)
/// * [test iam permissions tag keys](TagKeyTestIamPermissionCall) (request)
/// * [test iam permissions tag values](TagValueTestIamPermissionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsRequest {
    /// The set of permissions to check for the `resource`. Permissions with wildcards (such as `*` or `storage.*`) are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions).
    
    pub permissions: Option<Vec<String>>,
}

impl client::RequestValue for TestIamPermissionsRequest {}


/// Response message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [test iam permissions folders](FolderTestIamPermissionCall) (response)
/// * [test iam permissions organizations](OrganizationTestIamPermissionCall) (response)
/// * [test iam permissions projects](ProjectTestIamPermissionCall) (response)
/// * [test iam permissions tag keys](TagKeyTestIamPermissionCall) (response)
/// * [test iam permissions tag values](TagValueTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsResponse {
    /// A subset of `TestPermissionsRequest.permissions` that the caller is allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for TestIamPermissionsResponse {}


/// The UndeleteFolder request message.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [undelete folders](FolderUndeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UndeleteFolderRequest { _never_set: Option<bool> }

impl client::RequestValue for UndeleteFolderRequest {}


/// The request sent to the UndeleteProject method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [undelete projects](ProjectUndeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UndeleteProjectRequest { _never_set: Option<bool> }

impl client::RequestValue for UndeleteProjectRequest {}


