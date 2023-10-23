use super::*;
/// Settings on a Project/Folder/Organization related to Access Approval.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get access approval settings folders](FolderGetAccessApprovalSettingCall) (response)
/// * [update access approval settings folders](FolderUpdateAccessApprovalSettingCall) (request|response)
/// * [get access approval settings organizations](OrganizationGetAccessApprovalSettingCall) (response)
/// * [update access approval settings organizations](OrganizationUpdateAccessApprovalSettingCall) (request|response)
/// * [get access approval settings projects](ProjectGetAccessApprovalSettingCall) (response)
/// * [update access approval settings projects](ProjectUpdateAccessApprovalSettingCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccessApprovalSettings {
    /// Output only. This field is read only (not settable via
    /// UpdateAccessAccessApprovalSettings method). If the field is true, that
    /// indicates that at least one service is enrolled for Access Approval in one
    /// or more ancestors of the Project or Folder (this field will always be
    /// unset for the organization since organizations do not have ancestors).
    #[serde(rename="enrolledAncestor")]
    
    pub enrolled_ancestor: Option<bool>,
    /// A list of Google Cloud Services for which the given resource has Access
    /// Approval enrolled. Access requests for the resource given by name against
    /// any of these services contained here will be required to have explicit
    /// approval. If name refers to an organization, enrollment can be done for
    /// individual services. If name refers to a folder or project, enrollment can
    /// only be done on an all or nothing basis.
    /// 
    /// If a cloud_product is repeated in this list, the first entry will be
    /// honored and all following entries will be discarded. A maximum of 10
    /// enrolled services will be enforced, to be expanded as the set of supported
    /// services is expanded.
    #[serde(rename="enrolledServices")]
    
    pub enrolled_services: Option<Vec<EnrolledService>>,
    /// The resource name of the settings. Format is one of:
    /// <ol>
    ///   <li>"projects/{project_id}/accessApprovalSettings"</li>
    ///   <li>"folders/{folder_id}/accessApprovalSettings"</li>
    ///   <li>"organizations/{organization_id}/accessApprovalSettings"</li>
    /// <ol>
    
    pub name: Option<String>,
    /// A list of email addresses to which notifications relating to approval
    /// requests should be sent. Notifications relating to a resource will be sent
    /// to all emails in the settings of ancestor resources of that resource. A
    /// maximum of 50 email addresses are allowed.
    #[serde(rename="notificationEmails")]
    
    pub notification_emails: Option<Vec<String>>,
}

impl client::RequestValue for AccessApprovalSettings {}
impl client::ResponseResult for AccessApprovalSettings {}


/// Home office and physical location of the principal.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccessLocations {
    /// The "home office" location of the principal. A two-letter country code
    /// (ISO 3166-1 alpha-2), such as "US", "DE" or "GB" or a region code. In some
    /// limited situations Google systems may refer refer to a region code instead
    /// of a country code.
    /// Possible Region Codes:
    /// <ol>
    ///   <li>ASI: Asia</li>
    ///   <li>EUR: Europe</li>
    ///   <li>OCE: Oceania</li>
    ///   <li>AFR: Africa</li>
    ///   <li>NAM: North America</li>
    ///   <li>SAM: South America</li>
    ///   <li>ANT: Antarctica</li>
    ///   <li>ANY: Any location</li>
    /// </ol>
    #[serde(rename="principalOfficeCountry")]
    
    pub principal_office_country: Option<String>,
    /// Physical location of the principal at the time of the access. A
    /// two-letter country code (ISO 3166-1 alpha-2), such as "US", "DE" or "GB" or
    /// a region code. In some limited situations Google systems may refer refer to
    /// a region code instead of a country code.
    /// Possible Region Codes:
    /// <ol>
    ///   <li>ASI: Asia</li>
    ///   <li>EUR: Europe</li>
    ///   <li>OCE: Oceania</li>
    ///   <li>AFR: Africa</li>
    ///   <li>NAM: North America</li>
    ///   <li>SAM: South America</li>
    ///   <li>ANT: Antarctica</li>
    ///   <li>ANY: Any location</li>
    /// </ol>
    #[serde(rename="principalPhysicalLocationCountry")]
    
    pub principal_physical_location_country: Option<String>,
}

impl client::Part for AccessLocations {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccessReason {
    /// More detail about certain reason types. See comments for each type above.
    
    pub detail: Option<String>,
    /// Type of access justification.
    #[serde(rename="type")]
    
    pub type_: Option<AccessReasonTypeEnum>,
}

impl client::Part for AccessReason {}


/// A request for the customer to approve access to a resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [approval requests approve folders](FolderApprovalRequestApproveCall) (response)
/// * [approval requests dismiss folders](FolderApprovalRequestDismisCall) (response)
/// * [approval requests get folders](FolderApprovalRequestGetCall) (response)
/// * [approval requests approve organizations](OrganizationApprovalRequestApproveCall) (response)
/// * [approval requests dismiss organizations](OrganizationApprovalRequestDismisCall) (response)
/// * [approval requests get organizations](OrganizationApprovalRequestGetCall) (response)
/// * [approval requests approve projects](ProjectApprovalRequestApproveCall) (response)
/// * [approval requests dismiss projects](ProjectApprovalRequestDismisCall) (response)
/// * [approval requests get projects](ProjectApprovalRequestGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApprovalRequest {
    /// Access was approved.
    
    pub approve: Option<ApproveDecision>,
    /// The request was dismissed.
    
    pub dismiss: Option<DismissDecision>,
    /// The resource name of the request. Format is
    /// "{projects|folders|organizations}/{id}/approvalRequests/{approval_request_id}".
    
    pub name: Option<String>,
    /// The time at which approval was requested.
    #[serde(rename="requestTime")]
    
    pub request_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The requested expiration for the approval. If the request is approved,
    /// access will be granted from the time of approval until the expiration time.
    #[serde(rename="requestedExpiration")]
    
    pub requested_expiration: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The locations for which approval is being requested.
    #[serde(rename="requestedLocations")]
    
    pub requested_locations: Option<AccessLocations>,
    /// The justification for which approval is being requested.
    #[serde(rename="requestedReason")]
    
    pub requested_reason: Option<AccessReason>,
    /// The resource for which approval is being requested. The format of the
    /// resource name is defined at
    /// https://cloud.google.com/apis/design/resource_names. The resource name here
    /// may either be a "full" resource name (e.g.
    /// "//library.googleapis.com/shelves/shelf1/books/book2") or a "relative"
    /// resource name (e.g. "shelves/shelf1/books/book2") as described in the
    /// resource name specification.
    #[serde(rename="requestedResourceName")]
    
    pub requested_resource_name: Option<String>,
    /// Properties related to the resource represented by requested_resource_name.
    #[serde(rename="requestedResourceProperties")]
    
    pub requested_resource_properties: Option<ResourceProperties>,
}

impl client::ResponseResult for ApprovalRequest {}


/// Request to approve an ApprovalRequest.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [approval requests approve folders](FolderApprovalRequestApproveCall) (request)
/// * [approval requests approve organizations](OrganizationApprovalRequestApproveCall) (request)
/// * [approval requests approve projects](ProjectApprovalRequestApproveCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApproveApprovalRequestMessage {
    /// The expiration time of this approval.
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for ApproveApprovalRequestMessage {}


/// A decision that has been made to approve access to a resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApproveDecision {
    /// The time at which approval was granted.
    #[serde(rename="approveTime")]
    
    pub approve_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The time at which the approval expires.
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for ApproveDecision {}


/// Request to dismiss an approval request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [approval requests dismiss folders](FolderApprovalRequestDismisCall) (request)
/// * [approval requests dismiss organizations](OrganizationApprovalRequestDismisCall) (request)
/// * [approval requests dismiss projects](ProjectApprovalRequestDismisCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DismissApprovalRequestMessage { _never_set: Option<bool> }

impl client::RequestValue for DismissApprovalRequestMessage {}


/// A decision that has been made to dismiss an approval request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DismissDecision {
    /// The time at which the approval request was dismissed.
    #[serde(rename="dismissTime")]
    
    pub dismiss_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for DismissDecision {}


/// A generic empty message that you can re-use to avoid defining duplicated
/// empty messages in your APIs. A typical example is to use it as the request
/// or the response type of an API method. For instance:
/// 
/// ````text
/// service Foo {
///   rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty);
/// }
/// ````
/// 
/// The JSON representation for `Empty` is empty JSON object `{}`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete access approval settings folders](FolderDeleteAccessApprovalSettingCall) (response)
/// * [delete access approval settings organizations](OrganizationDeleteAccessApprovalSettingCall) (response)
/// * [delete access approval settings projects](ProjectDeleteAccessApprovalSettingCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// Represents the enrollment of a cloud resource into a specific service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnrolledService {
    /// The product for which Access Approval will be enrolled. Allowed values are
    /// listed below (case-sensitive):
    /// <ol>
    ///   <li>all</li>
    ///   <li>appengine.googleapis.com</li>
    ///   <li>bigquery.googleapis.com</li>
    ///   <li>bigtable.googleapis.com</li>
    ///   <li>cloudkms.googleapis.com</li>
    ///   <li>compute.googleapis.com</li>
    ///   <li>dataflow.googleapis.com</li>
    ///   <li>iam.googleapis.com</li>
    ///   <li>pubsub.googleapis.com</li>
    ///   <li>storage.googleapis.com</li>
    /// <ol>
    #[serde(rename="cloudProduct")]
    
    pub cloud_product: Option<String>,
    /// The enrollment level of the service.
    #[serde(rename="enrollmentLevel")]
    
    pub enrollment_level: Option<EnrolledServiceEnrollmentLevelEnum>,
}

impl client::Part for EnrolledService {}


/// Response to listing of ApprovalRequest objects.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [approval requests list folders](FolderApprovalRequestListCall) (response)
/// * [approval requests list organizations](OrganizationApprovalRequestListCall) (response)
/// * [approval requests list projects](ProjectApprovalRequestListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListApprovalRequestsResponse {
    /// Approval request details.
    #[serde(rename="approvalRequests")]
    
    pub approval_requests: Option<Vec<ApprovalRequest>>,
    /// Token to retrieve the next page of results, or empty if there are no more.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListApprovalRequestsResponse {}


/// The properties associated with the resource of the request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResourceProperties {
    /// Whether an approval will exclude the descendants of the resource being
    /// requested.
    #[serde(rename="excludesDescendants")]
    
    pub excludes_descendants: Option<bool>,
}

impl client::Part for ResourceProperties {}


