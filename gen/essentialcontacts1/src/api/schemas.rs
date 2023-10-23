use super::*;
/// Response message for the ComputeContacts method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [contacts compute folders](FolderContactComputeCall) (response)
/// * [contacts compute organizations](OrganizationContactComputeCall) (response)
/// * [contacts compute projects](ProjectContactComputeCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudEssentialcontactsV1ComputeContactsResponse {
    /// All contacts for the resource that are subscribed to the specified notification categories, including contacts inherited from any parent resources.
    
    pub contacts: Option<Vec<GoogleCloudEssentialcontactsV1Contact>>,
    /// If there are more results than those appearing in this response, then `next_page_token` is included. To get the next set of results, call this method again using the value of `next_page_token` as `page_token` and the rest of the parameters the same as the original request.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudEssentialcontactsV1ComputeContactsResponse {}


/// A contact that will receive notifications from Google Cloud.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [contacts create folders](FolderContactCreateCall) (request|response)
/// * [contacts get folders](FolderContactGetCall) (response)
/// * [contacts patch folders](FolderContactPatchCall) (request|response)
/// * [contacts create organizations](OrganizationContactCreateCall) (request|response)
/// * [contacts get organizations](OrganizationContactGetCall) (response)
/// * [contacts patch organizations](OrganizationContactPatchCall) (request|response)
/// * [contacts create projects](ProjectContactCreateCall) (request|response)
/// * [contacts get projects](ProjectContactGetCall) (response)
/// * [contacts patch projects](ProjectContactPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudEssentialcontactsV1Contact {
    /// Required. The email address to send notifications to. The email address does not need to be a Google Account.
    
    pub email: Option<String>,
    /// Required. The preferred language for notifications, as a ISO 639-1 language code. See [Supported languages](https://cloud.google.com/resource-manager/docs/managing-notification-contacts#supported-languages) for a list of supported languages.
    #[serde(rename="languageTag")]
    
    pub language_tag: Option<String>,
    /// Output only. The identifier for the contact. Format: {resource_type}/{resource_id}/contacts/{contact_id}
    
    pub name: Option<String>,
    /// Required. The categories of notifications that the contact will receive communications for.
    #[serde(rename="notificationCategorySubscriptions")]
    
    pub notification_category_subscriptions: Option<Vec<GoogleCloudEssentialcontactsV1ContactNotificationCategorySubscriptionsEnum>>,
    /// The last time the validation_state was updated, either manually or automatically. A contact is considered stale if its validation state was updated more than 1 year ago.
    #[serde(rename="validateTime")]
    
    pub validate_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The validity of the contact. A contact is considered valid if it is the correct recipient for notifications for a particular resource.
    #[serde(rename="validationState")]
    
    pub validation_state: Option<GoogleCloudEssentialcontactsV1ContactValidationStateEnum>,
}

impl client::RequestValue for GoogleCloudEssentialcontactsV1Contact {}
impl client::ResponseResult for GoogleCloudEssentialcontactsV1Contact {}


/// Response message for the ListContacts method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [contacts list folders](FolderContactListCall) (response)
/// * [contacts list organizations](OrganizationContactListCall) (response)
/// * [contacts list projects](ProjectContactListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudEssentialcontactsV1ListContactsResponse {
    /// The contacts for the specified resource.
    
    pub contacts: Option<Vec<GoogleCloudEssentialcontactsV1Contact>>,
    /// If there are more results than those appearing in this response, then `next_page_token` is included. To get the next set of results, call this method again using the value of `next_page_token` as `page_token` and the rest of the parameters the same as the original request.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudEssentialcontactsV1ListContactsResponse {}


/// Request message for the SendTestMessage method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [contacts send test message folders](FolderContactSendTestMessageCall) (request)
/// * [contacts send test message organizations](OrganizationContactSendTestMessageCall) (request)
/// * [contacts send test message projects](ProjectContactSendTestMessageCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudEssentialcontactsV1SendTestMessageRequest {
    /// Required. The list of names of the contacts to send a test message to. Format: organizations/{organization_id}/contacts/{contact_id}, folders/{folder_id}/contacts/{contact_id} or projects/{project_id}/contacts/{contact_id}
    
    pub contacts: Option<Vec<String>>,
    /// Required. The notification category to send the test message for. All contacts must be subscribed to this category.
    #[serde(rename="notificationCategory")]
    
    pub notification_category: Option<GoogleCloudEssentialcontactsV1SendTestMessageRequestNotificationCategoryEnum>,
}

impl client::RequestValue for GoogleCloudEssentialcontactsV1SendTestMessageRequest {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [contacts delete folders](FolderContactDeleteCall) (response)
/// * [contacts send test message folders](FolderContactSendTestMessageCall) (response)
/// * [contacts delete organizations](OrganizationContactDeleteCall) (response)
/// * [contacts send test message organizations](OrganizationContactSendTestMessageCall) (response)
/// * [contacts delete projects](ProjectContactDeleteCall) (response)
/// * [contacts send test message projects](ProjectContactSendTestMessageCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleProtobufEmpty { _never_set: Option<bool> }

impl client::ResponseResult for GoogleProtobufEmpty {}


