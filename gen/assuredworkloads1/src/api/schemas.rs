use super::*;
/// Request for acknowledging the violation Next Id: 4
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations workloads violations acknowledge organizations](OrganizationLocationWorkloadViolationAcknowledgeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssuredworkloadsV1AcknowledgeViolationRequest {
    /// Required. Business justification explaining the need for violation acknowledgement
    
    pub comment: Option<String>,
    /// Optional. This field is deprecated and will be removed in future version of the API. Name of the OrgPolicy which was modified with non-compliant change and resulted in this violation. Format: projects/{project_number}/policies/{constraint_name} folders/{folder_id}/policies/{constraint_name} organizations/{organization_id}/policies/{constraint_name}
    #[serde(rename="nonCompliantOrgPolicy")]
    
    pub non_compliant_org_policy: Option<String>,
}

impl client::RequestValue for GoogleCloudAssuredworkloadsV1AcknowledgeViolationRequest {}


/// Response for violation acknowledgement
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations workloads violations acknowledge organizations](OrganizationLocationWorkloadViolationAcknowledgeCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssuredworkloadsV1AcknowledgeViolationResponse { _never_set: Option<bool> }

impl client::ResponseResult for GoogleCloudAssuredworkloadsV1AcknowledgeViolationResponse {}


/// Response of ListViolations endpoint.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations workloads violations list organizations](OrganizationLocationWorkloadViolationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssuredworkloadsV1ListViolationsResponse {
    /// The next page token. Returns empty if reached the last page.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of Violations under a Workload.
    
    pub violations: Option<Vec<GoogleCloudAssuredworkloadsV1Violation>>,
}

impl client::ResponseResult for GoogleCloudAssuredworkloadsV1ListViolationsResponse {}


/// Response of ListWorkloads endpoint.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations workloads list organizations](OrganizationLocationWorkloadListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssuredworkloadsV1ListWorkloadsResponse {
    /// The next page token. Return empty if reached the last page.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of Workloads under a given parent.
    
    pub workloads: Option<Vec<GoogleCloudAssuredworkloadsV1Workload>>,
}

impl client::ResponseResult for GoogleCloudAssuredworkloadsV1ListWorkloadsResponse {}


/// Request of updating permission settings for a partner workload.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations workloads mutate partner permissions organizations](OrganizationLocationWorkloadMutatePartnerPermissionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssuredworkloadsV1MutatePartnerPermissionsRequest {
    /// Optional. The etag of the workload. If this is provided, it must match the server's etag.
    
    pub etag: Option<String>,
    /// Required. The partner permissions to be updated.
    #[serde(rename="partnerPermissions")]
    
    pub partner_permissions: Option<GoogleCloudAssuredworkloadsV1WorkloadPartnerPermissions>,
    /// Required. The list of fields to be updated. E.g. update_mask { paths: "partner_permissions.data_logs_viewer"}
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for GoogleCloudAssuredworkloadsV1MutatePartnerPermissionsRequest {}


/// Request for restricting list of available resources in Workload environment.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations workloads restrict allowed resources organizations](OrganizationLocationWorkloadRestrictAllowedResourceCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssuredworkloadsV1RestrictAllowedResourcesRequest {
    /// Required. The type of restriction for using gcp products in the Workload environment.
    #[serde(rename="restrictionType")]
    
    pub restriction_type: Option<GoogleCloudAssuredworkloadsV1RestrictAllowedResourcesRequestRestrictionTypeEnum>,
}

impl client::RequestValue for GoogleCloudAssuredworkloadsV1RestrictAllowedResourcesRequest {}


/// Response for restricting the list of allowed resources.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations workloads restrict allowed resources organizations](OrganizationLocationWorkloadRestrictAllowedResourceCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssuredworkloadsV1RestrictAllowedResourcesResponse { _never_set: Option<bool> }

impl client::ResponseResult for GoogleCloudAssuredworkloadsV1RestrictAllowedResourcesResponse {}


/// Workload monitoring Violation. Next Id: 22
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations workloads violations get organizations](OrganizationLocationWorkloadViolationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssuredworkloadsV1Violation {
    /// A boolean that indicates if the violation is acknowledged
    
    pub acknowledged: Option<bool>,
    /// Optional. Timestamp when this violation was acknowledged last. This will be absent when acknowledged field is marked as false.
    #[serde(rename="acknowledgementTime")]
    
    pub acknowledgement_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Immutable. Audit Log Link for violated resource Format: https://console.cloud.google.com/logs/query;query={logName}{protoPayload.resourceName}{timeRange}{folder}
    #[serde(rename="auditLogLink")]
    
    pub audit_log_link: Option<String>,
    /// Output only. Time of the event which triggered the Violation.
    #[serde(rename="beginTime")]
    
    pub begin_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Category under which this violation is mapped. e.g. Location, Service Usage, Access, Encryption, etc.
    
    pub category: Option<String>,
    /// Output only. Description for the Violation. e.g. OrgPolicy gcp.resourceLocations has non compliant value.
    
    pub description: Option<String>,
    /// Output only. Immutable. Audit Log link to find business justification provided for violation exception. Format: https://console.cloud.google.com/logs/query;query={logName}{protoPayload.resourceName}{protoPayload.methodName}{timeRange}{organization}
    #[serde(rename="exceptionAuditLogLink")]
    
    pub exception_audit_log_link: Option<String>,
    /// Output only. Immutable. Name of the Violation. Format: organizations/{organization}/locations/{location}/workloads/{workload_id}/violations/{violations_id}
    
    pub name: Option<String>,
    /// Output only. Immutable. Name of the OrgPolicy which was modified with non-compliant change and resulted this violation. Format: projects/{project_number}/policies/{constraint_name} folders/{folder_id}/policies/{constraint_name} organizations/{organization_id}/policies/{constraint_name}
    #[serde(rename="nonCompliantOrgPolicy")]
    
    pub non_compliant_org_policy: Option<String>,
    /// Output only. Immutable. The org-policy-constraint that was incorrectly changed, which resulted in this violation.
    #[serde(rename="orgPolicyConstraint")]
    
    pub org_policy_constraint: Option<String>,
    /// Output only. Compliance violation remediation
    
    pub remediation: Option<GoogleCloudAssuredworkloadsV1ViolationRemediation>,
    /// Output only. Time of the event which fixed the Violation. If the violation is ACTIVE this will be empty.
    #[serde(rename="resolveTime")]
    
    pub resolve_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. State of the violation
    
    pub state: Option<GoogleCloudAssuredworkloadsV1ViolationStateEnum>,
    /// Output only. The last time when the Violation record was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for GoogleCloudAssuredworkloadsV1Violation {}


/// Represents remediation guidance to resolve compliance violation for AssuredWorkload
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssuredworkloadsV1ViolationRemediation {
    /// Values that can resolve the violation For example: for list org policy violations, this will either be the list of allowed or denied values
    #[serde(rename="compliantValues")]
    
    pub compliant_values: Option<Vec<String>>,
    /// Required. Remediation instructions to resolve violations
    
    pub instructions: Option<GoogleCloudAssuredworkloadsV1ViolationRemediationInstructions>,
    /// Output only. Reemediation type based on the type of org policy values violated
    #[serde(rename="remediationType")]
    
    pub remediation_type: Option<GoogleCloudAssuredworkloadsV1ViolationRemediationRemediationTypeEnum>,
}

impl client::Part for GoogleCloudAssuredworkloadsV1ViolationRemediation {}


/// Instructions to remediate violation
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssuredworkloadsV1ViolationRemediationInstructions {
    /// Remediation instructions to resolve violation via cloud console
    #[serde(rename="consoleInstructions")]
    
    pub console_instructions: Option<GoogleCloudAssuredworkloadsV1ViolationRemediationInstructionsConsole>,
    /// Remediation instructions to resolve violation via gcloud cli
    #[serde(rename="gcloudInstructions")]
    
    pub gcloud_instructions: Option<GoogleCloudAssuredworkloadsV1ViolationRemediationInstructionsGcloud>,
}

impl client::Part for GoogleCloudAssuredworkloadsV1ViolationRemediationInstructions {}


/// Remediation instructions to resolve violation via cloud console
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssuredworkloadsV1ViolationRemediationInstructionsConsole {
    /// Additional urls for more information about steps
    #[serde(rename="additionalLinks")]
    
    pub additional_links: Option<Vec<String>>,
    /// Link to console page where violations can be resolved
    #[serde(rename="consoleUris")]
    
    pub console_uris: Option<Vec<String>>,
    /// Steps to resolve violation via cloud console
    
    pub steps: Option<Vec<String>>,
}

impl client::Part for GoogleCloudAssuredworkloadsV1ViolationRemediationInstructionsConsole {}


/// Remediation instructions to resolve violation via gcloud cli
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssuredworkloadsV1ViolationRemediationInstructionsGcloud {
    /// Additional urls for more information about steps
    #[serde(rename="additionalLinks")]
    
    pub additional_links: Option<Vec<String>>,
    /// Gcloud command to resolve violation
    #[serde(rename="gcloudCommands")]
    
    pub gcloud_commands: Option<Vec<String>>,
    /// Steps to resolve violation via gcloud cli
    
    pub steps: Option<Vec<String>>,
}

impl client::Part for GoogleCloudAssuredworkloadsV1ViolationRemediationInstructionsGcloud {}


/// A Workload object for managing highly regulated workloads of cloud customers.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations workloads create organizations](OrganizationLocationWorkloadCreateCall) (request)
/// * [locations workloads get organizations](OrganizationLocationWorkloadGetCall) (response)
/// * [locations workloads mutate partner permissions organizations](OrganizationLocationWorkloadMutatePartnerPermissionCall) (response)
/// * [locations workloads patch organizations](OrganizationLocationWorkloadPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssuredworkloadsV1Workload {
    /// Optional. The billing account used for the resources which are direct children of workload. This billing account is initially associated with the resources created as part of Workload creation. After the initial creation of these resources, the customer can change the assigned billing account. The resource name has the form `billingAccounts/{billing_account_id}`. For example, `billingAccounts/012345-567890-ABCDEF`.
    #[serde(rename="billingAccount")]
    
    pub billing_account: Option<String>,
    /// Required. Immutable. Compliance Regime associated with this workload.
    #[serde(rename="complianceRegime")]
    
    pub compliance_regime: Option<GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum>,
    /// Output only. Count of active Violations in the Workload.
    #[serde(rename="complianceStatus")]
    
    pub compliance_status: Option<GoogleCloudAssuredworkloadsV1WorkloadComplianceStatus>,
    /// Output only. Urls for services which are compliant for this Assured Workload, but which are currently disallowed by the ResourceUsageRestriction org policy. Invoke RestrictAllowedResources endpoint to allow your project developers to use these services in their environment."
    #[serde(rename="compliantButDisallowedServices")]
    
    pub compliant_but_disallowed_services: Option<Vec<String>>,
    /// Output only. Immutable. The Workload creation timestamp.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. The user-assigned display name of the Workload. When present it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, and spaces. Example: My Workload
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Optional. Indicates the sovereignty status of the given workload. Currently meant to be used by Europe/Canada customers.
    #[serde(rename="enableSovereignControls")]
    
    pub enable_sovereign_controls: Option<bool>,
    /// Optional. ETag of the workload, it is calculated on the basis of the Workload contents. It will be used in Update & Delete operations.
    
    pub etag: Option<String>,
    /// Output only. Represents the KAJ enrollment state of the given workload.
    #[serde(rename="kajEnrollmentState")]
    
    pub kaj_enrollment_state: Option<GoogleCloudAssuredworkloadsV1WorkloadKajEnrollmentStateEnum>,
    /// Input only. Settings used to create a CMEK crypto key. When set, a project with a KMS CMEK key is provisioned. This field is deprecated as of Feb 28, 2022. In order to create a Keyring, callers should specify, ENCRYPTION_KEYS_PROJECT or KEYRING in ResourceSettings.resource_type field.
    #[serde(rename="kmsSettings")]
    
    pub kms_settings: Option<GoogleCloudAssuredworkloadsV1WorkloadKMSSettings>,
    /// Optional. Labels applied to the workload.
    
    pub labels: Option<HashMap<String, String>>,
    /// Optional. The resource name of the workload. Format: organizations/{organization}/locations/{location}/workloads/{workload} Read-only.
    
    pub name: Option<String>,
    /// Optional. Partner regime associated with this workload.
    
    pub partner: Option<GoogleCloudAssuredworkloadsV1WorkloadPartnerEnum>,
    /// Input only. The parent resource for the resources managed by this Assured Workload. May be either empty or a folder resource which is a child of the Workload parent. If not specified all resources are created under the parent organization. Format: folders/{folder_id}
    #[serde(rename="provisionedResourcesParent")]
    
    pub provisioned_resources_parent: Option<String>,
    /// Input only. Resource properties that are used to customize workload resources. These properties (such as custom project id) will be used to create workload resources if possible. This field is optional.
    #[serde(rename="resourceSettings")]
    
    pub resource_settings: Option<Vec<GoogleCloudAssuredworkloadsV1WorkloadResourceSettings>>,
    /// Output only. The resources associated with this workload. These resources will be created when creating the workload. If any of the projects already exist, the workload creation will fail. Always read only.
    
    pub resources: Option<Vec<GoogleCloudAssuredworkloadsV1WorkloadResourceInfo>>,
    /// Output only. Represents the SAA enrollment response of the given workload. SAA enrollment response is queried during GetWorkload call. In failure cases, user friendly error message is shown in SAA details page.
    #[serde(rename="saaEnrollmentResponse")]
    
    pub saa_enrollment_response: Option<GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponse>,
}

impl client::RequestValue for GoogleCloudAssuredworkloadsV1Workload {}
impl client::ResponseResult for GoogleCloudAssuredworkloadsV1Workload {}


/// Represents the Compliance Status of this workload
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssuredworkloadsV1WorkloadComplianceStatus {
    /// Count of active Violations which are acknowledged in the Workload.
    #[serde(rename="acknowledgedViolationCount")]
    
    pub acknowledged_violation_count: Option<i32>,
    /// Count of active Violations which haven't been acknowledged.
    #[serde(rename="activeViolationCount")]
    
    pub active_violation_count: Option<i32>,
}

impl client::Part for GoogleCloudAssuredworkloadsV1WorkloadComplianceStatus {}


/// Settings specific to the Key Management Service. This message is deprecated. In order to create a Keyring, callers should specify, ENCRYPTION_KEYS_PROJECT or KEYRING in ResourceSettings.resource_type field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssuredworkloadsV1WorkloadKMSSettings {
    /// Required. Input only. Immutable. The time at which the Key Management Service will automatically create a new version of the crypto key and mark it as the primary.
    #[serde(rename="nextRotationTime")]
    
    pub next_rotation_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. Input only. Immutable. [next_rotation_time] will be advanced by this period when the Key Management Service automatically rotates a key. Must be at least 24 hours and at most 876,000 hours.
    #[serde(rename="rotationPeriod")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub rotation_period: Option<client::chrono::Duration>,
}

impl client::Part for GoogleCloudAssuredworkloadsV1WorkloadKMSSettings {}


/// Permissions granted to the AW Partner SA account for the customer workload
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssuredworkloadsV1WorkloadPartnerPermissions {
    /// Allow partner to view data and logs
    #[serde(rename="dataLogsViewer")]
    
    pub data_logs_viewer: Option<bool>,
    /// Allow partner to monitor folder and remediate violations
    #[serde(rename="remediateFolderViolations")]
    
    pub remediate_folder_violations: Option<bool>,
    /// Allow partner to approve or reject Service Access requests
    #[serde(rename="serviceAccessApprover")]
    
    pub service_access_approver: Option<bool>,
}

impl client::Part for GoogleCloudAssuredworkloadsV1WorkloadPartnerPermissions {}


/// Represent the resources that are children of this Workload.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssuredworkloadsV1WorkloadResourceInfo {
    /// Resource identifier. For a project this represents project_number.
    #[serde(rename="resourceId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub resource_id: Option<i64>,
    /// Indicates the type of resource.
    #[serde(rename="resourceType")]
    
    pub resource_type: Option<GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceTypeEnum>,
}

impl client::Part for GoogleCloudAssuredworkloadsV1WorkloadResourceInfo {}


/// Represent the custom settings for the resources to be created.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssuredworkloadsV1WorkloadResourceSettings {
    /// User-assigned resource display name. If not empty it will be used to create a resource with the specified name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Resource identifier. For a project this represents project_id. If the project is already taken, the workload creation will fail. For KeyRing, this represents the keyring_id. For a folder, don't set this value as folder_id is assigned by Google.
    #[serde(rename="resourceId")]
    
    pub resource_id: Option<String>,
    /// Indicates the type of resource. This field should be specified to correspond the id to the right resource type (CONSUMER_FOLDER or ENCRYPTION_KEYS_PROJECT)
    #[serde(rename="resourceType")]
    
    pub resource_type: Option<GoogleCloudAssuredworkloadsV1WorkloadResourceSettingResourceTypeEnum>,
}

impl client::Part for GoogleCloudAssuredworkloadsV1WorkloadResourceSettings {}


/// Signed Access Approvals (SAA) enrollment response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponse {
    /// Indicates SAA enrollment setup error if any.
    #[serde(rename="setupErrors")]
    
    pub setup_errors: Option<Vec<GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsEnum>>,
    /// Indicates SAA enrollment status of a given workload.
    #[serde(rename="setupStatus")]
    
    pub setup_status: Option<GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupStatusEnum>,
}

impl client::Part for GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponse {}


/// The response message for Operations.ListOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations list organizations](OrganizationLocationOperationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleLongrunningListOperationsResponse {
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of operations that matches the specified filter in the request.
    
    pub operations: Option<Vec<GoogleLongrunningOperation>>,
}

impl client::ResponseResult for GoogleLongrunningListOperationsResponse {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations get organizations](OrganizationLocationOperationGetCall) (response)
/// * [locations workloads create organizations](OrganizationLocationWorkloadCreateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleLongrunningOperation {
    /// If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available.
    
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    
    pub error: Option<GoogleRpcStatus>,
    /// Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`.
    
    pub name: Option<String>,
    /// The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`.
    
    pub response: Option<HashMap<String, json::Value>>,
}

impl client::ResponseResult for GoogleLongrunningOperation {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations workloads delete organizations](OrganizationLocationWorkloadDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleProtobufEmpty { _never_set: Option<bool> }

impl client::ResponseResult for GoogleProtobufEmpty {}


/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleRpcStatus {
    /// The status code, which should be an enum value of google.rpc.Code.
    
    pub code: Option<i32>,
    /// A list of messages that carry the error details. There is a common set of message types for APIs to use.
    
    pub details: Option<Vec<HashMap<String, json::Value>>>,
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client.
    
    pub message: Option<String>,
}

impl client::Part for GoogleRpcStatus {}


