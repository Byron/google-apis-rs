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


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigFile {
    /// The contents of the file.
    
    pub content: Option<String>,
}

impl client::Part for ConfigFile {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [cancel preview deployments](DeploymentCancelPreviewCall) (none)
/// * [delete deployments](DeploymentDeleteCall) (none)
/// * [get deployments](DeploymentGetCall) (response)
/// * [get iam policy deployments](DeploymentGetIamPolicyCall) (none)
/// * [insert deployments](DeploymentInsertCall) (request)
/// * [list deployments](DeploymentListCall) (none)
/// * [patch deployments](DeploymentPatchCall) (request)
/// * [set iam policy deployments](DeploymentSetIamPolicyCall) (none)
/// * [stop deployments](DeploymentStopCall) (none)
/// * [test iam permissions deployments](DeploymentTestIamPermissionCall) (none)
/// * [update deployments](DeploymentUpdateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Deployment {
    /// An optional user-provided description of the deployment.
    
    pub description: Option<String>,
    /// Provides a fingerprint to use in requests to modify a deployment, such as `update()`, `stop()`, and `cancelPreview()` requests. A fingerprint is a randomly generated value that must be provided with `update()`, `stop()`, and `cancelPreview()` requests to perform optimistic locking. This ensures optimistic concurrency so that only one request happens at a time. The fingerprint is initially generated by Deployment Manager and changes after every request to modify data. To get the latest fingerprint value, perform a `get()` request to a deployment.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub fingerprint: Option<Vec<u8>>,
    /// no description provided
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<u64>,
    /// Output only. Creation timestamp in RFC3339 text format.
    #[serde(rename="insertTime")]
    
    pub insert_time: Option<String>,
    /// Map of One Platform labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`.
    
    pub labels: Option<Vec<DeploymentLabelEntry>>,
    /// Output only. URL of the manifest representing the last manifest that was successfully deployed. If no manifest has been successfully deployed, this field will be absent.
    
    pub manifest: Option<String>,
    /// Name of the resource; provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash.
    
    pub name: Option<String>,
    /// Output only. The Operation that most recently ran, or is currently running, on this deployment.
    
    pub operation: Option<Operation>,
    /// Output only. Server defined URL for the resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// [Input Only] The parameters that define your deployment, including the deployment configuration and relevant templates.
    
    pub target: Option<TargetConfiguration>,
    /// Output only. If Deployment Manager is currently updating or previewing an update to this deployment, the updated configuration appears here.
    
    pub update: Option<DeploymentUpdate>,
    /// Output only. Update timestamp in RFC3339 text format.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<String>,
}

impl client::RequestValue for Deployment {}
impl client::Resource for Deployment {}
impl client::ResponseResult for Deployment {}


/// Label object for Deployments
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeploymentLabelEntry {
    /// Key of the label
    
    pub key: Option<String>,
    /// Value of the label
    
    pub value: Option<String>,
}

impl client::Part for DeploymentLabelEntry {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeploymentUpdate {
    /// Output only. An optional user-provided description of the deployment after the current update has been applied.
    
    pub description: Option<String>,
    /// Map of One Platform labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`.
    
    pub labels: Option<Vec<DeploymentUpdateLabelEntry>>,
    /// Output only. URL of the manifest representing the update configuration of this deployment.
    
    pub manifest: Option<String>,
}

impl client::Part for DeploymentUpdate {}


/// Label object for DeploymentUpdate
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeploymentUpdateLabelEntry {
    /// Key of the label
    
    pub key: Option<String>,
    /// Value of the label
    
    pub value: Option<String>,
}

impl client::Part for DeploymentUpdateLabelEntry {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [cancel preview deployments](DeploymentCancelPreviewCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeploymentsCancelPreviewRequest {
    /// Specifies a fingerprint for `cancelPreview()` requests. A fingerprint is a randomly generated value that must be provided in `cancelPreview()` requests to perform optimistic locking. This ensures optimistic concurrency so that the deployment does not have conflicting requests (e.g. if someone attempts to make a new update request while another user attempts to cancel a preview, this would prevent one of the requests). The fingerprint is initially generated by Deployment Manager and changes after every request to modify a deployment. To get the latest fingerprint value, perform a `get()` request on the deployment.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub fingerprint: Option<Vec<u8>>,
}

impl client::RequestValue for DeploymentsCancelPreviewRequest {}


/// A response containing a partial list of deployments and a page token used to build the next request if the request has been truncated.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list deployments](DeploymentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeploymentsListResponse {
    /// Output only. The deployments contained in this response.
    
    pub deployments: Option<Vec<Deployment>>,
    /// Output only. A token used to continue a truncated list request.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for DeploymentsListResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [stop deployments](DeploymentStopCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeploymentsStopRequest {
    /// Specifies a fingerprint for `stop()` requests. A fingerprint is a randomly generated value that must be provided in `stop()` requests to perform optimistic locking. This ensures optimistic concurrency so that the deployment does not have conflicting requests (e.g. if someone attempts to make a new update request while another user attempts to stop an ongoing update request, this would prevent a collision). The fingerprint is initially generated by Deployment Manager and changes after every request to modify a deployment. To get the latest fingerprint value, perform a `get()` request on the deployment.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub fingerprint: Option<Vec<u8>>,
}

impl client::RequestValue for DeploymentsStopRequest {}


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


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [set iam policy deployments](DeploymentSetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GlobalSetPolicyRequest {
    /// Flatten Policy to create a backward compatible wire-format. Deprecated. Use 'policy' to specify bindings.
    
    pub bindings: Option<Vec<Binding>>,
    /// Flatten Policy to create a backward compatible wire-format. Deprecated. Use 'policy' to specify the etag.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
    /// REQUIRED: The complete policy to be applied to the 'resource'. The size of the policy is limited to a few 10s of KB. An empty policy is in general a valid policy but certain services (like Projects) might reject them.
    
    pub policy: Option<Policy>,
}

impl client::RequestValue for GlobalSetPolicyRequest {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImportFile {
    /// The contents of the file.
    
    pub content: Option<String>,
    /// The name of the file.
    
    pub name: Option<String>,
}

impl client::Part for ImportFile {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get manifests](ManifestGetCall) (response)
/// * [list manifests](ManifestListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Manifest {
    /// Output only. The YAML configuration for this manifest.
    
    pub config: Option<ConfigFile>,
    /// Output only. The fully-expanded configuration file, including any templates and references.
    #[serde(rename="expandedConfig")]
    
    pub expanded_config: Option<String>,
    /// no description provided
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<u64>,
    /// Output only. The imported files for this manifest.
    
    pub imports: Option<Vec<ImportFile>>,
    /// Output only. Creation timestamp in RFC3339 text format.
    #[serde(rename="insertTime")]
    
    pub insert_time: Option<String>,
    /// Output only. The YAML layout for this manifest.
    
    pub layout: Option<String>,
    /// Output only. The computed size of the fully expanded manifest.
    #[serde(rename="manifestSizeBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub manifest_size_bytes: Option<i64>,
    /// Output only. The size limit for expanded manifests in the project.
    #[serde(rename="manifestSizeLimitBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub manifest_size_limit_bytes: Option<i64>,
    /// Output only. The name of the manifest.
    
    pub name: Option<String>,
    /// Output only. Self link for the manifest.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::Resource for Manifest {}
impl client::ResponseResult for Manifest {}


/// A response containing a partial list of manifests and a page token used to build the next request if the request has been truncated.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list manifests](ManifestListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManifestsListResponse {
    /// Output only. Manifests contained in this list response.
    
    pub manifests: Option<Vec<Manifest>>,
    /// Output only. A token used to continue a truncated list request.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ManifestsListResponse {}


/// Represents an Operation resource. Google Compute Engine has three Operation resources: * [Global](https://cloud.google.com/compute/docs/reference/rest/{$api_version}/globalOperations) * [Regional](https://cloud.google.com/compute/docs/reference/rest/{$api_version}/regionOperations) * [Zonal](https://cloud.google.com/compute/docs/reference/rest/{$api_version}/zoneOperations) You can use an operation resource to manage asynchronous API requests. For more information, read Handling API responses. Operations can be global, regional or zonal. - For global operations, use the `globalOperations` resource. - For regional operations, use the `regionOperations` resource. - For zonal operations, use the `zonalOperations` resource. For more information, read Global, Regional, and Zonal Resources.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [cancel preview deployments](DeploymentCancelPreviewCall) (response)
/// * [delete deployments](DeploymentDeleteCall) (response)
/// * [insert deployments](DeploymentInsertCall) (response)
/// * [patch deployments](DeploymentPatchCall) (response)
/// * [stop deployments](DeploymentStopCall) (response)
/// * [update deployments](DeploymentUpdateCall) (response)
/// * [get operations](OperationGetCall) (response)
/// * [list operations](OperationListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// [Output Only] The value of `requestId` if you provided it in the request. Not present otherwise.
    #[serde(rename="clientOperationId")]
    
    pub client_operation_id: Option<String>,
    /// [Deprecated] This field is deprecated.
    #[serde(rename="creationTimestamp")]
    
    pub creation_timestamp: Option<String>,
    /// [Output Only] A textual description of the operation, which is set when the operation is created.
    
    pub description: Option<String>,
    /// [Output Only] The time that this operation was completed. This value is in RFC3339 text format.
    #[serde(rename="endTime")]
    
    pub end_time: Option<String>,
    /// [Output Only] If errors are generated during processing of the operation, this field will be populated.
    
    pub error: Option<OperationError>,
    /// [Output Only] If the operation fails, this field contains the HTTP error message that was returned, such as `NOT FOUND`.
    #[serde(rename="httpErrorMessage")]
    
    pub http_error_message: Option<String>,
    /// [Output Only] If the operation fails, this field contains the HTTP error status code that was returned. For example, a `404` means the resource was not found.
    #[serde(rename="httpErrorStatusCode")]
    
    pub http_error_status_code: Option<i32>,
    /// [Output Only] The unique identifier for the operation. This identifier is defined by the server.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<u64>,
    /// [Output Only] The time that this operation was requested. This value is in RFC3339 text format.
    #[serde(rename="insertTime")]
    
    pub insert_time: Option<String>,
    /// [Output Only] Type of the resource. Always `compute#operation` for Operation resources.
    
    pub kind: Option<String>,
    /// [Output Only] Name of the operation.
    
    pub name: Option<String>,
    /// [Output Only] An ID that represents a group of operations, such as when a group of operations results from a `bulkInsert` API request.
    #[serde(rename="operationGroupId")]
    
    pub operation_group_id: Option<String>,
    /// [Output Only] The type of operation, such as `insert`, `update`, or `delete`, and so on.
    #[serde(rename="operationType")]
    
    pub operation_type: Option<String>,
    /// [Output Only] An optional progress indicator that ranges from 0 to 100. There is no requirement that this be linear or support any granularity of operations. This should not be used to guess when the operation will be complete. This number should monotonically increase as the operation progresses.
    
    pub progress: Option<i32>,
    /// [Output Only] The URL of the region where the operation resides. Only applicable when performing regional operations.
    
    pub region: Option<String>,
    /// [Output Only] Server-defined URL for the resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// [Output Only] The time that this operation was started by the server. This value is in RFC3339 text format.
    #[serde(rename="startTime")]
    
    pub start_time: Option<String>,
    /// [Output Only] The status of the operation, which can be one of the following: `PENDING`, `RUNNING`, or `DONE`.
    
    pub status: Option<OperationStatusEnum>,
    /// [Output Only] An optional textual description of the current status of the operation.
    #[serde(rename="statusMessage")]
    
    pub status_message: Option<String>,
    /// [Output Only] The unique target ID, which identifies a specific incarnation of the target resource.
    #[serde(rename="targetId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub target_id: Option<u64>,
    /// [Output Only] The URL of the resource that the operation modifies. For operations related to creating a snapshot, this points to the persistent disk that the snapshot was created from.
    #[serde(rename="targetLink")]
    
    pub target_link: Option<String>,
    /// [Output Only] User who requested the operation, for example: `user@example.com`.
    
    pub user: Option<String>,
    /// [Output Only] If warning messages are generated during processing of the operation, this field will be populated.
    
    pub warnings: Option<Vec<OperationWarnings>>,
    /// [Output Only] The URL of the zone where the operation resides. Only applicable when performing per-zone operations.
    
    pub zone: Option<String>,
}

impl client::Resource for Operation {}
impl client::ResponseResult for Operation {}


/// A response containing a partial list of operations and a page token used to build the next request if the request has been truncated.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list operations](OperationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperationsListResponse {
    /// Output only. A token used to continue a truncated list request.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Output only. Operations contained in this list response.
    
    pub operations: Option<Vec<Operation>>,
}

impl client::ResponseResult for OperationsListResponse {}


/// An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members`, or principals, to a single `role`. Principals can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { “bindings”: \[ { “role”: “roles/resourcemanager.organizationAdmin”, “members”: \[ “user:mike@example.com”, “group:admins@example.com”, “domain:google.com”, “serviceAccount:my-project-id@appspot.gserviceaccount.com” \] }, { “role”: “roles/resourcemanager.organizationViewer”, “members”: \[ “user:eve@example.com” \], “condition”: { “title”: “expirable access”, “description”: “Does not grant access after Sep 2020”, “expression”: “request.time \< timestamp(‘2020-10-01T00:00:00.000Z’)”, } } \], “etag”: “BwWWja0YfJA=”, “version”: 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time \< timestamp(‘2020-10-01T00:00:00.000Z’) etag: BwWWja0YfJA= version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get iam policy deployments](DeploymentGetIamPolicyCall) (response)
/// * [set iam policy deployments](DeploymentSetIamPolicyCall) (response)
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


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get resources](ResourceGetCall) (response)
/// * [list resources](ResourceListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Resource {
    /// The Access Control Policy set on this resource.
    #[serde(rename="accessControl")]
    
    pub access_control: Option<ResourceAccessControl>,
    /// Output only. The evaluated properties of the resource with references expanded. Returned as serialized YAML.
    #[serde(rename="finalProperties")]
    
    pub final_properties: Option<String>,
    /// no description provided
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<u64>,
    /// Output only. Creation timestamp in RFC3339 text format.
    #[serde(rename="insertTime")]
    
    pub insert_time: Option<String>,
    /// Output only. URL of the manifest representing the current configuration of this resource.
    
    pub manifest: Option<String>,
    /// Output only. The name of the resource as it appears in the YAML config.
    
    pub name: Option<String>,
    /// Output only. The current properties of the resource before any references have been filled in. Returned as serialized YAML.
    
    pub properties: Option<String>,
    /// Output only. The type of the resource, for example `compute.v1.instance`, or `cloudfunctions.v1beta1.function`.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Output only. If Deployment Manager is currently updating or previewing an update to this resource, the updated configuration appears here.
    
    pub update: Option<ResourceUpdate>,
    /// Output only. Update timestamp in RFC3339 text format.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<String>,
    /// Output only. The URL of the actual resource.
    
    pub url: Option<String>,
    /// Output only. If warning messages are generated during processing of this resource, this field will be populated.
    
    pub warnings: Option<Vec<ResourceWarnings>>,
}

impl client::Resource for Resource {}
impl client::ResponseResult for Resource {}


/// The access controls set on the resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResourceAccessControl {
    /// The GCP IAM Policy to set on the resource.
    #[serde(rename="gcpIamPolicy")]
    
    pub gcp_iam_policy: Option<String>,
}

impl client::Part for ResourceAccessControl {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResourceUpdate {
    /// The Access Control Policy to set on this resource after updating the resource itself.
    #[serde(rename="accessControl")]
    
    pub access_control: Option<ResourceAccessControl>,
    /// Output only. If errors are generated during update of the resource, this field will be populated.
    
    pub error: Option<ResourceUpdateError>,
    /// Output only. The expanded properties of the resource with reference values expanded. Returned as serialized YAML.
    #[serde(rename="finalProperties")]
    
    pub final_properties: Option<String>,
    /// Output only. The intent of the resource: `PREVIEW`, `UPDATE`, or `CANCEL`.
    
    pub intent: Option<ResourceUpdateIntentEnum>,
    /// Output only. URL of the manifest representing the update configuration of this resource.
    
    pub manifest: Option<String>,
    /// Output only. The set of updated properties for this resource, before references are expanded. Returned as serialized YAML.
    
    pub properties: Option<String>,
    /// Output only. The state of the resource.
    
    pub state: Option<ResourceUpdateStateEnum>,
    /// Output only. If warning messages are generated during processing of this resource, this field will be populated.
    
    pub warnings: Option<Vec<ResourceUpdateWarnings>>,
}

impl client::Part for ResourceUpdate {}


/// A response containing a partial list of resources and a page token used to build the next request if the request has been truncated.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list resources](ResourceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResourcesListResponse {
    /// A token used to continue a truncated list request.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Resources contained in this list response.
    
    pub resources: Option<Vec<Resource>>,
}

impl client::ResponseResult for ResourcesListResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TargetConfiguration {
    /// The configuration to use for this deployment.
    
    pub config: Option<ConfigFile>,
    /// Specifies any files to import for this configuration. This can be used to import templates or other files. For example, you might import a text file in order to use the file in a template.
    
    pub imports: Option<Vec<ImportFile>>,
}

impl client::Part for TargetConfiguration {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [test iam permissions deployments](DeploymentTestIamPermissionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestPermissionsRequest {
    /// The set of permissions to check for the 'resource'. Permissions with wildcards (such as '*' or 'storage.*') are not allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::RequestValue for TestPermissionsRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [test iam permissions deployments](DeploymentTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestPermissionsResponse {
    /// A subset of `TestPermissionsRequest.permissions` that the caller is allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for TestPermissionsResponse {}


/// A resource type supported by Deployment Manager.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list types](TypeListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Type {
    /// no description provided
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<u64>,
    /// Output only. Creation timestamp in RFC3339 text format.
    #[serde(rename="insertTime")]
    
    pub insert_time: Option<String>,
    /// Name of the type.
    
    pub name: Option<String>,
    /// Output only. The Operation that most recently ran, or is currently running, on this type.
    
    pub operation: Option<Operation>,
    /// Output only. Server defined URL for the resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::Resource for Type {}


/// A response that returns all Types supported by Deployment Manager
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list types](TypeListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TypesListResponse {
    /// A token used to continue a truncated list request.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Output only. A list of resource types supported by Deployment Manager.
    
    pub types: Option<Vec<Type>>,
}

impl client::ResponseResult for TypesListResponse {}


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
    /// [Output Only] Metadata about this warning in key: value format. For example: "data": [ { "key": "scope", "value": "zones/us-east1-d" } 
    
    pub data: Option<Vec<OperationWarningsData>>,
    /// [Output Only] A human-readable description of the warning code.
    
    pub message: Option<String>,
}

impl client::NestedType for OperationWarnings {}
impl client::Part for OperationWarnings {}


/// [Output Only] Metadata about this warning in key: value format. For example: "data": [ { "key": "scope", "value": "zones/us-east1-d" } 
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


/// Output only. If warning messages are generated during processing of this resource, this field will be populated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResourceWarnings {
    /// [Output Only] A warning code, if applicable. For example, Compute Engine returns NO_RESULTS_ON_PAGE if there are no results in the response.
    
    pub code: Option<ResourceWarningCodeEnum>,
    /// [Output Only] Metadata about this warning in key: value format. For example: "data": [ { "key": "scope", "value": "zones/us-east1-d" } 
    
    pub data: Option<Vec<ResourceWarningsData>>,
    /// [Output Only] A human-readable description of the warning code.
    
    pub message: Option<String>,
}

impl client::NestedType for ResourceWarnings {}
impl client::Part for ResourceWarnings {}


/// [Output Only] Metadata about this warning in key: value format. For example: "data": [ { "key": "scope", "value": "zones/us-east1-d" } 
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResourceWarningsData {
    /// [Output Only] A key that provides more detail on the warning being returned. For example, for warnings where there are no results in a list request for a particular zone, this key might be scope and the key value might be the zone name. Other examples might be a key indicating a deprecated resource and a suggested replacement, or a warning about invalid network settings (for example, if an instance attempts to perform IP forwarding but is not enabled for IP forwarding).
    
    pub key: Option<String>,
    /// [Output Only] A warning data value corresponding to the key.
    
    pub value: Option<String>,
}

impl client::NestedType for ResourceWarningsData {}
impl client::Part for ResourceWarningsData {}


/// Output only. If errors are generated during update of the resource, this field will be populated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResourceUpdateError {
    /// [Output Only] The array of errors encountered while processing this operation.
    
    pub errors: Option<Vec<ResourceUpdateErrorErrors>>,
}

impl client::NestedType for ResourceUpdateError {}
impl client::Part for ResourceUpdateError {}


/// [Output Only] The array of errors encountered while processing this operation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResourceUpdateErrorErrors {
    /// [Output Only] The error type identifier for this error.
    
    pub code: Option<String>,
    /// [Output Only] Indicates the field in the request that caused the error. This property is optional.
    
    pub location: Option<String>,
    /// [Output Only] An optional, human-readable error message.
    
    pub message: Option<String>,
}

impl client::NestedType for ResourceUpdateErrorErrors {}
impl client::Part for ResourceUpdateErrorErrors {}


/// Output only. If warning messages are generated during processing of this resource, this field will be populated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResourceUpdateWarnings {
    /// [Output Only] A warning code, if applicable. For example, Compute Engine returns NO_RESULTS_ON_PAGE if there are no results in the response.
    
    pub code: Option<ResourceUpdateWarningCodeEnum>,
    /// [Output Only] Metadata about this warning in key: value format. For example: "data": [ { "key": "scope", "value": "zones/us-east1-d" } 
    
    pub data: Option<Vec<ResourceUpdateWarningsData>>,
    /// [Output Only] A human-readable description of the warning code.
    
    pub message: Option<String>,
}

impl client::NestedType for ResourceUpdateWarnings {}
impl client::Part for ResourceUpdateWarnings {}


/// [Output Only] Metadata about this warning in key: value format. For example: "data": [ { "key": "scope", "value": "zones/us-east1-d" } 
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResourceUpdateWarningsData {
    /// [Output Only] A key that provides more detail on the warning being returned. For example, for warnings where there are no results in a list request for a particular zone, this key might be scope and the key value might be the zone name. Other examples might be a key indicating a deprecated resource and a suggested replacement, or a warning about invalid network settings (for example, if an instance attempts to perform IP forwarding but is not enabled for IP forwarding).
    
    pub key: Option<String>,
    /// [Output Only] A warning data value corresponding to the key.
    
    pub value: Option<String>,
}

impl client::NestedType for ResourceUpdateWarningsData {}
impl client::Part for ResourceUpdateWarningsData {}


