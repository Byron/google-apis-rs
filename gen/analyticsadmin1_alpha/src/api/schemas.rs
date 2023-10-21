use super::*;
/// A resource message representing a Google Analytics account.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get accounts](AccountGetCall) (response)
/// * [patch accounts](AccountPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaAccount {
    /// Output only. Time when this account was originally created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Indicates whether this Account is soft-deleted or not. Deleted accounts are excluded from List results unless specifically requested.
    
    pub deleted: Option<bool>,
    /// Required. Human-readable display name for this account.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Resource name of this account. Format: accounts/{account} Example: "accounts/100"
    
    pub name: Option<String>,
    /// Country of business. Must be a Unicode CLDR region code.
    #[serde(rename="regionCode")]
    
    pub region_code: Option<String>,
    /// Output only. Time when account payload fields were last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GoogleAnalyticsAdminV1alphaAccount {}
impl client::ResponseResult for GoogleAnalyticsAdminV1alphaAccount {}


/// A virtual resource representing an overview of an account and all its child GA4 properties.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaAccountSummary {
    /// Resource name of account referred to by this account summary Format: accounts/{account_id} Example: "accounts/1000"
    
    pub account: Option<String>,
    /// Display name for the account referred to in this account summary.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Resource name for this account summary. Format: accountSummaries/{account_id} Example: "accountSummaries/1000"
    
    pub name: Option<String>,
    /// List of summaries for child accounts of this account.
    #[serde(rename="propertySummaries")]
    
    pub property_summaries: Option<Vec<GoogleAnalyticsAdminV1alphaPropertySummary>>,
}

impl client::Part for GoogleAnalyticsAdminV1alphaAccountSummary {}


/// Request message for AcknowledgeUserDataCollection RPC.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [acknowledge user data collection properties](PropertyAcknowledgeUserDataCollectionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaAcknowledgeUserDataCollectionRequest {
    /// Required. An acknowledgement that the caller of this method understands the terms of user data collection. This field must contain the exact value: "I acknowledge that I have the necessary privacy disclosures and rights from my end users for the collection and processing of their data, including the association of such data with the visitation information Google Analytics collects from my site and/or app property."
    
    pub acknowledgement: Option<String>,
}

impl client::RequestValue for GoogleAnalyticsAdminV1alphaAcknowledgeUserDataCollectionRequest {}


/// Response message for AcknowledgeUserDataCollection RPC.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [acknowledge user data collection properties](PropertyAcknowledgeUserDataCollectionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaAcknowledgeUserDataCollectionResponse { _never_set: Option<bool> }

impl client::ResponseResult for GoogleAnalyticsAdminV1alphaAcknowledgeUserDataCollectionResponse {}


/// Request message for ApproveDisplayVideo360AdvertiserLinkProposal RPC.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [display video360 advertiser link proposals approve properties](PropertyDisplayVideo360AdvertiserLinkProposalApproveCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaApproveDisplayVideo360AdvertiserLinkProposalRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleAnalyticsAdminV1alphaApproveDisplayVideo360AdvertiserLinkProposalRequest {}


/// Response message for ApproveDisplayVideo360AdvertiserLinkProposal RPC.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [display video360 advertiser link proposals approve properties](PropertyDisplayVideo360AdvertiserLinkProposalApproveCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaApproveDisplayVideo360AdvertiserLinkProposalResponse {
    /// The DisplayVideo360AdvertiserLink created as a result of approving the proposal.
    #[serde(rename="displayVideo360AdvertiserLink")]
    
    pub display_video360_advertiser_link: Option<GoogleAnalyticsAdminV1alphaDisplayVideo360AdvertiserLink>,
}

impl client::ResponseResult for GoogleAnalyticsAdminV1alphaApproveDisplayVideo360AdvertiserLinkProposalResponse {}


/// Request message for ArchiveCustomDimension RPC.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custom dimensions archive properties](PropertyCustomDimensionArchiveCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaArchiveCustomDimensionRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleAnalyticsAdminV1alphaArchiveCustomDimensionRequest {}


/// Request message for ArchiveCustomMetric RPC.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custom metrics archive properties](PropertyCustomMetricArchiveCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaArchiveCustomMetricRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleAnalyticsAdminV1alphaArchiveCustomMetricRequest {}


/// Read-only resource used to summarize a principal's effective roles.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaAuditUserLink {
    /// Roles directly assigned to this user for this entity. Format: predefinedRoles/viewer Excludes roles that are inherited from an account (if this is for a property), group, or organization admin role.
    #[serde(rename="directRoles")]
    
    pub direct_roles: Option<Vec<String>>,
    /// Union of all permissions a user has at this account or property (includes direct permissions, group-inherited permissions, etc.). Format: predefinedRoles/viewer
    #[serde(rename="effectiveRoles")]
    
    pub effective_roles: Option<Vec<String>>,
    /// Email address of the linked user
    #[serde(rename="emailAddress")]
    
    pub email_address: Option<String>,
    /// Example format: properties/1234/userLinks/5678
    
    pub name: Option<String>,
}

impl client::Part for GoogleAnalyticsAdminV1alphaAuditUserLink {}


/// Request message for AuditUserLinks RPC.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [user links audit accounts](AccountUserLinkAuditCall) (request)
/// * [user links audit properties](PropertyUserLinkAuditCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaAuditUserLinksRequest {
    /// The maximum number of user links to return. The service may return fewer than this value. If unspecified, at most 1000 user links will be returned. The maximum value is 5000; values above 5000 will be coerced to 5000.
    #[serde(rename="pageSize")]
    
    pub page_size: Option<i32>,
    /// A page token, received from a previous `AuditUserLinks` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `AuditUserLinks` must match the call that provided the page token.
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
}

impl client::RequestValue for GoogleAnalyticsAdminV1alphaAuditUserLinksRequest {}


/// Response message for AuditUserLinks RPC.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [user links audit accounts](AccountUserLinkAuditCall) (response)
/// * [user links audit properties](PropertyUserLinkAuditCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaAuditUserLinksResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of AuditUserLinks. These will be ordered stably, but in an arbitrary order.
    #[serde(rename="userLinks")]
    
    pub user_links: Option<Vec<GoogleAnalyticsAdminV1alphaAuditUserLink>>,
}

impl client::ResponseResult for GoogleAnalyticsAdminV1alphaAuditUserLinksResponse {}


/// Request message for BatchCreateUserLinks RPC.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [user links batch create accounts](AccountUserLinkBatchCreateCall) (request)
/// * [user links batch create properties](PropertyUserLinkBatchCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaBatchCreateUserLinksRequest {
    /// Optional. If set, then email the new users notifying them that they've been granted permissions to the resource. Regardless of whether this is set or not, notify_new_user field inside each individual request is ignored.
    #[serde(rename="notifyNewUsers")]
    
    pub notify_new_users: Option<bool>,
    /// Required. The requests specifying the user links to create. A maximum of 1000 user links can be created in a batch.
    
    pub requests: Option<Vec<GoogleAnalyticsAdminV1alphaCreateUserLinkRequest>>,
}

impl client::RequestValue for GoogleAnalyticsAdminV1alphaBatchCreateUserLinksRequest {}


/// Response message for BatchCreateUserLinks RPC.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [user links batch create accounts](AccountUserLinkBatchCreateCall) (response)
/// * [user links batch create properties](PropertyUserLinkBatchCreateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaBatchCreateUserLinksResponse {
    /// The user links created.
    #[serde(rename="userLinks")]
    
    pub user_links: Option<Vec<GoogleAnalyticsAdminV1alphaUserLink>>,
}

impl client::ResponseResult for GoogleAnalyticsAdminV1alphaBatchCreateUserLinksResponse {}


/// Request message for BatchDeleteUserLinks RPC.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [user links batch delete accounts](AccountUserLinkBatchDeleteCall) (request)
/// * [user links batch delete properties](PropertyUserLinkBatchDeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaBatchDeleteUserLinksRequest {
    /// Required. The requests specifying the user links to update. A maximum of 1000 user links can be updated in a batch.
    
    pub requests: Option<Vec<GoogleAnalyticsAdminV1alphaDeleteUserLinkRequest>>,
}

impl client::RequestValue for GoogleAnalyticsAdminV1alphaBatchDeleteUserLinksRequest {}


/// Response message for BatchGetUserLinks RPC.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [user links batch get accounts](AccountUserLinkBatchGetCall) (response)
/// * [user links batch get properties](PropertyUserLinkBatchGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaBatchGetUserLinksResponse {
    /// The requested user links.
    #[serde(rename="userLinks")]
    
    pub user_links: Option<Vec<GoogleAnalyticsAdminV1alphaUserLink>>,
}

impl client::ResponseResult for GoogleAnalyticsAdminV1alphaBatchGetUserLinksResponse {}


/// Request message for BatchUpdateUserLinks RPC.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [user links batch update accounts](AccountUserLinkBatchUpdateCall) (request)
/// * [user links batch update properties](PropertyUserLinkBatchUpdateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaBatchUpdateUserLinksRequest {
    /// Required. The requests specifying the user links to update. A maximum of 1000 user links can be updated in a batch.
    
    pub requests: Option<Vec<GoogleAnalyticsAdminV1alphaUpdateUserLinkRequest>>,
}

impl client::RequestValue for GoogleAnalyticsAdminV1alphaBatchUpdateUserLinksRequest {}


/// Response message for BatchUpdateUserLinks RPC.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [user links batch update accounts](AccountUserLinkBatchUpdateCall) (response)
/// * [user links batch update properties](PropertyUserLinkBatchUpdateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaBatchUpdateUserLinksResponse {
    /// The user links updated.
    #[serde(rename="userLinks")]
    
    pub user_links: Option<Vec<GoogleAnalyticsAdminV1alphaUserLink>>,
}

impl client::ResponseResult for GoogleAnalyticsAdminV1alphaBatchUpdateUserLinksResponse {}


/// Request message for CancelDisplayVideo360AdvertiserLinkProposal RPC.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [display video360 advertiser link proposals cancel properties](PropertyDisplayVideo360AdvertiserLinkProposalCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaCancelDisplayVideo360AdvertiserLinkProposalRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleAnalyticsAdminV1alphaCancelDisplayVideo360AdvertiserLinkProposalRequest {}


/// A description of a change to a single Google Analytics resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaChangeHistoryChange {
    /// The type of action that changed this resource.
    
    pub action: Option<String>,
    /// Resource name of the resource whose changes are described by this entry.
    
    pub resource: Option<String>,
    /// Resource contents from after the change was made. If this resource was deleted in this change, this field will be missing.
    #[serde(rename="resourceAfterChange")]
    
    pub resource_after_change: Option<GoogleAnalyticsAdminV1alphaChangeHistoryChangeChangeHistoryResource>,
    /// Resource contents from before the change was made. If this resource was created in this change, this field will be missing.
    #[serde(rename="resourceBeforeChange")]
    
    pub resource_before_change: Option<GoogleAnalyticsAdminV1alphaChangeHistoryChangeChangeHistoryResource>,
}

impl client::Part for GoogleAnalyticsAdminV1alphaChangeHistoryChange {}


/// A snapshot of a resource as before or after the result of a change in change history.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaChangeHistoryChangeChangeHistoryResource {
    /// A snapshot of an Account resource in change history.
    
    pub account: Option<GoogleAnalyticsAdminV1alphaAccount>,
    /// A snapshot of a ConversionEvent resource in change history.
    #[serde(rename="conversionEvent")]
    
    pub conversion_event: Option<GoogleAnalyticsAdminV1alphaConversionEvent>,
    /// A snapshot of a CustomDimension resource in change history.
    #[serde(rename="customDimension")]
    
    pub custom_dimension: Option<GoogleAnalyticsAdminV1alphaCustomDimension>,
    /// A snapshot of a CustomMetric resource in change history.
    #[serde(rename="customMetric")]
    
    pub custom_metric: Option<GoogleAnalyticsAdminV1alphaCustomMetric>,
    /// A snapshot of a data retention settings resource in change history.
    #[serde(rename="dataRetentionSettings")]
    
    pub data_retention_settings: Option<GoogleAnalyticsAdminV1alphaDataRetentionSettings>,
    /// A snapshot of a DataStream resource in change history.
    #[serde(rename="dataStream")]
    
    pub data_stream: Option<GoogleAnalyticsAdminV1alphaDataStream>,
    /// A snapshot of a DisplayVideo360AdvertiserLink resource in change history.
    #[serde(rename="displayVideo360AdvertiserLink")]
    
    pub display_video360_advertiser_link: Option<GoogleAnalyticsAdminV1alphaDisplayVideo360AdvertiserLink>,
    /// A snapshot of a DisplayVideo360AdvertiserLinkProposal resource in change history.
    #[serde(rename="displayVideo360AdvertiserLinkProposal")]
    
    pub display_video360_advertiser_link_proposal: Option<GoogleAnalyticsAdminV1alphaDisplayVideo360AdvertiserLinkProposal>,
    /// A snapshot of a FirebaseLink resource in change history.
    #[serde(rename="firebaseLink")]
    
    pub firebase_link: Option<GoogleAnalyticsAdminV1alphaFirebaseLink>,
    /// A snapshot of a GoogleAdsLink resource in change history.
    #[serde(rename="googleAdsLink")]
    
    pub google_ads_link: Option<GoogleAnalyticsAdminV1alphaGoogleAdsLink>,
    /// A snapshot of a GoogleSignalsSettings resource in change history.
    #[serde(rename="googleSignalsSettings")]
    
    pub google_signals_settings: Option<GoogleAnalyticsAdminV1alphaGoogleSignalsSettings>,
    /// A snapshot of a MeasurementProtocolSecret resource in change history.
    #[serde(rename="measurementProtocolSecret")]
    
    pub measurement_protocol_secret: Option<GoogleAnalyticsAdminV1alphaMeasurementProtocolSecret>,
    /// A snapshot of a Property resource in change history.
    
    pub property: Option<GoogleAnalyticsAdminV1alphaProperty>,
}

impl client::Part for GoogleAnalyticsAdminV1alphaChangeHistoryChangeChangeHistoryResource {}


/// A set of changes within a Google Analytics account or its child properties that resulted from the same cause. Common causes would be updates made in the Google Analytics UI, changes from customer support, or automatic Google Analytics system changes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaChangeHistoryEvent {
    /// The type of actor that made this change.
    #[serde(rename="actorType")]
    
    pub actor_type: Option<String>,
    /// Time when change was made.
    #[serde(rename="changeTime")]
    
    pub change_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A list of changes made in this change history event that fit the filters specified in SearchChangeHistoryEventsRequest.
    
    pub changes: Option<Vec<GoogleAnalyticsAdminV1alphaChangeHistoryChange>>,
    /// If true, then the list of changes returned was filtered, and does not represent all changes that occurred in this event.
    #[serde(rename="changesFiltered")]
    
    pub changes_filtered: Option<bool>,
    /// ID of this change history event. This ID is unique across Google Analytics.
    
    pub id: Option<String>,
    /// Email address of the Google account that made the change. This will be a valid email address if the actor field is set to USER, and empty otherwise. Google accounts that have been deleted will cause an error.
    #[serde(rename="userActorEmail")]
    
    pub user_actor_email: Option<String>,
}

impl client::Part for GoogleAnalyticsAdminV1alphaChangeHistoryEvent {}


/// A conversion event in a Google Analytics property.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [conversion events create properties](PropertyConversionEventCreateCall) (request|response)
/// * [conversion events get properties](PropertyConversionEventGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaConversionEvent {
    /// Output only. Time when this conversion event was created in the property.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. If set to true, this conversion event refers to a custom event. If set to false, this conversion event refers to a default event in GA. Default events typically have special meaning in GA. Default events are usually created for you by the GA system, but in some cases can be created by property admins. Custom events count towards the maximum number of custom conversion events that may be created per property.
    
    pub custom: Option<bool>,
    /// Output only. If set, this event can currently be deleted via DeleteConversionEvent.
    
    pub deletable: Option<bool>,
    /// Immutable. The event name for this conversion event. Examples: 'click', 'purchase'
    #[serde(rename="eventName")]
    
    pub event_name: Option<String>,
    /// Output only. Resource name of this conversion event. Format: properties/{property}/conversionEvents/{conversion_event}
    
    pub name: Option<String>,
}

impl client::RequestValue for GoogleAnalyticsAdminV1alphaConversionEvent {}
impl client::ResponseResult for GoogleAnalyticsAdminV1alphaConversionEvent {}


/// Request message for CreateUserLink RPC. Users can have multiple email addresses associated with their Google account, and one of these email addresses is the "primary" email address. Any of the email addresses associated with a Google account may be used for a new UserLink, but the returned UserLink will always contain the "primary" email address. As a result, the input and output email address for this request may differ.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaCreateUserLinkRequest {
    /// Optional. If set, then email the new user notifying them that they've been granted permissions to the resource.
    #[serde(rename="notifyNewUser")]
    
    pub notify_new_user: Option<bool>,
    /// Required. Example format: accounts/1234
    
    pub parent: Option<String>,
    /// Required. The user link to create.
    #[serde(rename="userLink")]
    
    pub user_link: Option<GoogleAnalyticsAdminV1alphaUserLink>,
}

impl client::Part for GoogleAnalyticsAdminV1alphaCreateUserLinkRequest {}


/// A definition for a CustomDimension.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custom dimensions create properties](PropertyCustomDimensionCreateCall) (request|response)
/// * [custom dimensions get properties](PropertyCustomDimensionGetCall) (response)
/// * [custom dimensions patch properties](PropertyCustomDimensionPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaCustomDimension {
    /// Optional. Description for this custom dimension. Max length of 150 characters.
    
    pub description: Option<String>,
    /// Optional. If set to true, sets this dimension as NPA and excludes it from ads personalization. This is currently only supported by user-scoped custom dimensions.
    #[serde(rename="disallowAdsPersonalization")]
    
    pub disallow_ads_personalization: Option<bool>,
    /// Required. Display name for this custom dimension as shown in the Analytics UI. Max length of 82 characters, alphanumeric plus space and underscore starting with a letter. Legacy system-generated display names may contain square brackets, but updates to this field will never permit square brackets.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Resource name for this CustomDimension resource. Format: properties/{property}/customDimensions/{customDimension}
    
    pub name: Option<String>,
    /// Required. Immutable. Tagging parameter name for this custom dimension. If this is a user-scoped dimension, then this is the user property name. If this is an event-scoped dimension, then this is the event parameter name. May only contain alphanumeric and underscore characters, starting with a letter. Max length of 24 characters for user-scoped dimensions, 40 characters for event-scoped dimensions.
    #[serde(rename="parameterName")]
    
    pub parameter_name: Option<String>,
    /// Required. Immutable. The scope of this dimension.
    
    pub scope: Option<String>,
}

impl client::RequestValue for GoogleAnalyticsAdminV1alphaCustomDimension {}
impl client::ResponseResult for GoogleAnalyticsAdminV1alphaCustomDimension {}


/// A definition for a custom metric.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custom metrics create properties](PropertyCustomMetricCreateCall) (request|response)
/// * [custom metrics get properties](PropertyCustomMetricGetCall) (response)
/// * [custom metrics patch properties](PropertyCustomMetricPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaCustomMetric {
    /// Optional. Description for this custom dimension. Max length of 150 characters.
    
    pub description: Option<String>,
    /// Required. Display name for this custom metric as shown in the Analytics UI. Max length of 82 characters, alphanumeric plus space and underscore starting with a letter. Legacy system-generated display names may contain square brackets, but updates to this field will never permit square brackets.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Required. The type for the custom metric's value.
    #[serde(rename="measurementUnit")]
    
    pub measurement_unit: Option<String>,
    /// Output only. Resource name for this CustomMetric resource. Format: properties/{property}/customMetrics/{customMetric}
    
    pub name: Option<String>,
    /// Required. Immutable. Tagging name for this custom metric. If this is an event-scoped metric, then this is the event parameter name. May only contain alphanumeric and underscore charactes, starting with a letter. Max length of 40 characters for event-scoped metrics.
    #[serde(rename="parameterName")]
    
    pub parameter_name: Option<String>,
    /// Optional. Types of restricted data that this metric may contain. Required for metrics with CURRENCY measurement unit. Must be empty for metrics with a non-CURRENCY measurement unit.
    #[serde(rename="restrictedMetricType")]
    
    pub restricted_metric_type: Option<Vec<String>>,
    /// Required. Immutable. The scope of this custom metric.
    
    pub scope: Option<String>,
}

impl client::RequestValue for GoogleAnalyticsAdminV1alphaCustomMetric {}
impl client::ResponseResult for GoogleAnalyticsAdminV1alphaCustomMetric {}


/// Settings values for data retention. This is a singleton resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get data retention settings properties](PropertyGetDataRetentionSettingCall) (response)
/// * [update data retention settings properties](PropertyUpdateDataRetentionSettingCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaDataRetentionSettings {
    /// The length of time that event-level data is retained.
    #[serde(rename="eventDataRetention")]
    
    pub event_data_retention: Option<String>,
    /// Output only. Resource name for this DataRetentionSetting resource. Format: properties/{property}/dataRetentionSettings
    
    pub name: Option<String>,
    /// If true, reset the retention period for the user identifier with every event from that user.
    #[serde(rename="resetUserDataOnNewActivity")]
    
    pub reset_user_data_on_new_activity: Option<bool>,
}

impl client::RequestValue for GoogleAnalyticsAdminV1alphaDataRetentionSettings {}
impl client::ResponseResult for GoogleAnalyticsAdminV1alphaDataRetentionSettings {}


/// A resource message representing data sharing settings of a Google Analytics account.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get data sharing settings accounts](AccountGetDataSharingSettingCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaDataSharingSettings {
    /// Output only. Resource name. Format: accounts/{account}/dataSharingSettings Example: "accounts/1000/dataSharingSettings"
    
    pub name: Option<String>,
    /// Allows any of Google sales to access the data in order to suggest configuration changes to improve results.
    #[serde(rename="sharingWithGoogleAnySalesEnabled")]
    
    pub sharing_with_google_any_sales_enabled: Option<bool>,
    /// Allows Google sales teams that are assigned to the customer to access the data in order to suggest configuration changes to improve results. Sales team restrictions still apply when enabled.
    #[serde(rename="sharingWithGoogleAssignedSalesEnabled")]
    
    pub sharing_with_google_assigned_sales_enabled: Option<bool>,
    /// Allows Google to use the data to improve other Google products or services.
    #[serde(rename="sharingWithGoogleProductsEnabled")]
    
    pub sharing_with_google_products_enabled: Option<bool>,
    /// Allows Google support to access the data in order to help troubleshoot issues.
    #[serde(rename="sharingWithGoogleSupportEnabled")]
    
    pub sharing_with_google_support_enabled: Option<bool>,
    /// Allows Google to share the data anonymously in aggregate form with others.
    #[serde(rename="sharingWithOthersEnabled")]
    
    pub sharing_with_others_enabled: Option<bool>,
}

impl client::ResponseResult for GoogleAnalyticsAdminV1alphaDataSharingSettings {}


/// A resource message representing a data stream.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [data streams create properties](PropertyDataStreamCreateCall) (request|response)
/// * [data streams get properties](PropertyDataStreamGetCall) (response)
/// * [data streams patch properties](PropertyDataStreamPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaDataStream {
    /// Data specific to Android app streams. Must be populated if type is ANDROID_APP_DATA_STREAM.
    #[serde(rename="androidAppStreamData")]
    
    pub android_app_stream_data: Option<GoogleAnalyticsAdminV1alphaDataStreamAndroidAppStreamData>,
    /// Output only. Time when this stream was originally created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Human-readable display name for the Data Stream. Required for web data streams. The max allowed display name length is 255 UTF-16 code units.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Data specific to iOS app streams. Must be populated if type is IOS_APP_DATA_STREAM.
    #[serde(rename="iosAppStreamData")]
    
    pub ios_app_stream_data: Option<GoogleAnalyticsAdminV1alphaDataStreamIosAppStreamData>,
    /// Output only. Resource name of this Data Stream. Format: properties/{property_id}/dataStreams/{stream_id} Example: "properties/1000/dataStreams/2000"
    
    pub name: Option<String>,
    /// Required. Immutable. The type of this DataStream resource.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Output only. Time when stream payload fields were last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Data specific to web streams. Must be populated if type is WEB_DATA_STREAM.
    #[serde(rename="webStreamData")]
    
    pub web_stream_data: Option<GoogleAnalyticsAdminV1alphaDataStreamWebStreamData>,
}

impl client::RequestValue for GoogleAnalyticsAdminV1alphaDataStream {}
impl client::ResponseResult for GoogleAnalyticsAdminV1alphaDataStream {}


/// Data specific to Android app streams.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaDataStreamAndroidAppStreamData {
    /// Output only. ID of the corresponding Android app in Firebase, if any. This ID can change if the Android app is deleted and recreated.
    #[serde(rename="firebaseAppId")]
    
    pub firebase_app_id: Option<String>,
    /// Immutable. The package name for the app being measured. Example: "com.example.myandroidapp"
    #[serde(rename="packageName")]
    
    pub package_name: Option<String>,
}

impl client::Part for GoogleAnalyticsAdminV1alphaDataStreamAndroidAppStreamData {}


/// Data specific to iOS app streams.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaDataStreamIosAppStreamData {
    /// Required. Immutable. The Apple App Store Bundle ID for the app Example: "com.example.myiosapp"
    #[serde(rename="bundleId")]
    
    pub bundle_id: Option<String>,
    /// Output only. ID of the corresponding iOS app in Firebase, if any. This ID can change if the iOS app is deleted and recreated.
    #[serde(rename="firebaseAppId")]
    
    pub firebase_app_id: Option<String>,
}

impl client::Part for GoogleAnalyticsAdminV1alphaDataStreamIosAppStreamData {}


/// Data specific to web streams.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaDataStreamWebStreamData {
    /// Immutable. Domain name of the web app being measured, or empty. Example: "http://www.google.com", "https://www.google.com"
    #[serde(rename="defaultUri")]
    
    pub default_uri: Option<String>,
    /// Output only. ID of the corresponding web app in Firebase, if any. This ID can change if the web app is deleted and recreated.
    #[serde(rename="firebaseAppId")]
    
    pub firebase_app_id: Option<String>,
    /// Output only. Analytics "Measurement ID", without the "G-" prefix. Example: "G-1A2BCD345E" would just be "1A2BCD345E"
    #[serde(rename="measurementId")]
    
    pub measurement_id: Option<String>,
}

impl client::Part for GoogleAnalyticsAdminV1alphaDataStreamWebStreamData {}


/// Request message for DeleteUserLink RPC.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaDeleteUserLinkRequest {
    /// Required. Example format: accounts/1234/userLinks/5678
    
    pub name: Option<String>,
}

impl client::Part for GoogleAnalyticsAdminV1alphaDeleteUserLinkRequest {}


/// A link between a GA4 property and a Display & Video 360 advertiser.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [display video360 advertiser links create properties](PropertyDisplayVideo360AdvertiserLinkCreateCall) (request|response)
/// * [display video360 advertiser links get properties](PropertyDisplayVideo360AdvertiserLinkGetCall) (response)
/// * [display video360 advertiser links patch properties](PropertyDisplayVideo360AdvertiserLinkPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaDisplayVideo360AdvertiserLink {
    /// Enables personalized advertising features with this integration. If this field is not set on create/update, it will be defaulted to true.
    #[serde(rename="adsPersonalizationEnabled")]
    
    pub ads_personalization_enabled: Option<bool>,
    /// Output only. The display name of the Display & Video 360 Advertiser.
    #[serde(rename="advertiserDisplayName")]
    
    pub advertiser_display_name: Option<String>,
    /// Immutable. The Display & Video 360 Advertiser's advertiser ID.
    #[serde(rename="advertiserId")]
    
    pub advertiser_id: Option<String>,
    /// Immutable. Enables the import of campaign data from Display & Video 360 into the GA4 property. After link creation, this can only be updated from the Display & Video 360 product. If this field is not set on create, it will be defaulted to true.
    #[serde(rename="campaignDataSharingEnabled")]
    
    pub campaign_data_sharing_enabled: Option<bool>,
    /// Immutable. Enables the import of cost data from Display & Video 360 into the GA4 property. This can only be enabled if campaign_data_sharing_enabled is enabled. After link creation, this can only be updated from the Display & Video 360 product. If this field is not set on create, it will be defaulted to true.
    #[serde(rename="costDataSharingEnabled")]
    
    pub cost_data_sharing_enabled: Option<bool>,
    /// Output only. The resource name for this DisplayVideo360AdvertiserLink resource. Format: properties/{propertyId}/displayVideo360AdvertiserLinks/{linkId} Note: linkId is not the Display & Video 360 Advertiser ID
    
    pub name: Option<String>,
}

impl client::RequestValue for GoogleAnalyticsAdminV1alphaDisplayVideo360AdvertiserLink {}
impl client::ResponseResult for GoogleAnalyticsAdminV1alphaDisplayVideo360AdvertiserLink {}


/// A proposal for a link between a GA4 property and a Display & Video 360 advertiser. A proposal is converted to a DisplayVideo360AdvertiserLink once approved. Google Analytics admins approve inbound proposals while Display & Video 360 admins approve outbound proposals.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [display video360 advertiser link proposals cancel properties](PropertyDisplayVideo360AdvertiserLinkProposalCancelCall) (response)
/// * [display video360 advertiser link proposals create properties](PropertyDisplayVideo360AdvertiserLinkProposalCreateCall) (request|response)
/// * [display video360 advertiser link proposals get properties](PropertyDisplayVideo360AdvertiserLinkProposalGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaDisplayVideo360AdvertiserLinkProposal {
    /// Immutable. Enables personalized advertising features with this integration. If this field is not set on create, it will be defaulted to true.
    #[serde(rename="adsPersonalizationEnabled")]
    
    pub ads_personalization_enabled: Option<bool>,
    /// Output only. The display name of the Display & Video Advertiser. Only populated for proposals that originated from Display & Video 360.
    #[serde(rename="advertiserDisplayName")]
    
    pub advertiser_display_name: Option<String>,
    /// Immutable. The Display & Video 360 Advertiser's advertiser ID.
    #[serde(rename="advertiserId")]
    
    pub advertiser_id: Option<String>,
    /// Immutable. Enables the import of campaign data from Display & Video 360. If this field is not set on create, it will be defaulted to true.
    #[serde(rename="campaignDataSharingEnabled")]
    
    pub campaign_data_sharing_enabled: Option<bool>,
    /// Immutable. Enables the import of cost data from Display & Video 360. This can only be enabled if campaign_data_sharing_enabled is enabled. If this field is not set on create, it will be defaulted to true.
    #[serde(rename="costDataSharingEnabled")]
    
    pub cost_data_sharing_enabled: Option<bool>,
    /// Output only. The status information for this link proposal.
    #[serde(rename="linkProposalStatusDetails")]
    
    pub link_proposal_status_details: Option<GoogleAnalyticsAdminV1alphaLinkProposalStatusDetails>,
    /// Output only. The resource name for this DisplayVideo360AdvertiserLinkProposal resource. Format: properties/{propertyId}/displayVideo360AdvertiserLinkProposals/{proposalId} Note: proposalId is not the Display & Video 360 Advertiser ID
    
    pub name: Option<String>,
    /// Input only. On a proposal being sent to Display & Video 360, this field must be set to the email address of an admin on the target advertiser. This is used to verify that the Google Analytics admin is aware of at least one admin on the Display & Video 360 Advertiser. This does not restrict approval of the proposal to a single user. Any admin on the Display & Video 360 Advertiser may approve the proposal.
    #[serde(rename="validationEmail")]
    
    pub validation_email: Option<String>,
}

impl client::RequestValue for GoogleAnalyticsAdminV1alphaDisplayVideo360AdvertiserLinkProposal {}
impl client::ResponseResult for GoogleAnalyticsAdminV1alphaDisplayVideo360AdvertiserLinkProposal {}


/// A link between a GA4 property and a Firebase project.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [firebase links create properties](PropertyFirebaseLinkCreateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaFirebaseLink {
    /// Output only. Time when this FirebaseLink was originally created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Example format: properties/1234/firebaseLinks/5678
    
    pub name: Option<String>,
    /// Immutable. Firebase project resource name. When creating a FirebaseLink, you may provide this resource name using either a project number or project ID. Once this resource has been created, returned FirebaseLinks will always have a project_name that contains a project number. Format: 'projects/{project number}' Example: 'projects/1234'
    
    pub project: Option<String>,
}

impl client::RequestValue for GoogleAnalyticsAdminV1alphaFirebaseLink {}
impl client::ResponseResult for GoogleAnalyticsAdminV1alphaFirebaseLink {}


/// Read-only resource with the tag for sending data from a website to a DataStream. Only present for web DataStream resources.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [data streams get global site tag properties](PropertyDataStreamGetGlobalSiteTagCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaGlobalSiteTag {
    /// Output only. Resource name for this GlobalSiteTag resource. Format: properties/{property_id}/dataStreams/{stream_id}/globalSiteTag Example: "properties/123/dataStreams/456/globalSiteTag"
    
    pub name: Option<String>,
    /// Immutable. JavaScript code snippet to be pasted as the first item into the head tag of every webpage to measure.
    
    pub snippet: Option<String>,
}

impl client::ResponseResult for GoogleAnalyticsAdminV1alphaGlobalSiteTag {}


/// A link between a GA4 property and a Google Ads account.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [google ads links create properties](PropertyGoogleAdsLinkCreateCall) (request|response)
/// * [google ads links patch properties](PropertyGoogleAdsLinkPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaGoogleAdsLink {
    /// Enable personalized advertising features with this integration. Automatically publish my Google Analytics audience lists and Google Analytics remarketing events/parameters to the linked Google Ads account. If this field is not set on create/update, it will be defaulted to true.
    #[serde(rename="adsPersonalizationEnabled")]
    
    pub ads_personalization_enabled: Option<bool>,
    /// Output only. If true, this link is for a Google Ads manager account.
    #[serde(rename="canManageClients")]
    
    pub can_manage_clients: Option<bool>,
    /// Output only. Time when this link was originally created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Email address of the user that created the link. An empty string will be returned if the email address can't be retrieved.
    #[serde(rename="creatorEmailAddress")]
    
    pub creator_email_address: Option<String>,
    /// Immutable. Google Ads customer ID.
    #[serde(rename="customerId")]
    
    pub customer_id: Option<String>,
    /// Output only. Format: properties/{propertyId}/googleAdsLinks/{googleAdsLinkId} Note: googleAdsLinkId is not the Google Ads customer ID.
    
    pub name: Option<String>,
    /// Output only. Time when this link was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GoogleAnalyticsAdminV1alphaGoogleAdsLink {}
impl client::ResponseResult for GoogleAnalyticsAdminV1alphaGoogleAdsLink {}


/// Settings values for Google Signals. This is a singleton resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get google signals settings properties](PropertyGetGoogleSignalsSettingCall) (response)
/// * [update google signals settings properties](PropertyUpdateGoogleSignalsSettingCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaGoogleSignalsSettings {
    /// Output only. Terms of Service acceptance.
    
    pub consent: Option<String>,
    /// Output only. Resource name of this setting. Format: properties/{property_id}/googleSignalsSettings Example: "properties/1000/googleSignalsSettings"
    
    pub name: Option<String>,
    /// Status of this setting.
    
    pub state: Option<String>,
}

impl client::RequestValue for GoogleAnalyticsAdminV1alphaGoogleSignalsSettings {}
impl client::ResponseResult for GoogleAnalyticsAdminV1alphaGoogleSignalsSettings {}


/// Status information for a link proposal.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaLinkProposalStatusDetails {
    /// Output only. The source of this proposal.
    #[serde(rename="linkProposalInitiatingProduct")]
    
    pub link_proposal_initiating_product: Option<String>,
    /// Output only. The state of this proposal.
    #[serde(rename="linkProposalState")]
    
    pub link_proposal_state: Option<String>,
    /// Output only. The email address of the user that proposed this linkage.
    #[serde(rename="requestorEmail")]
    
    pub requestor_email: Option<String>,
}

impl client::Part for GoogleAnalyticsAdminV1alphaLinkProposalStatusDetails {}


/// Response message for ListAccountSummaries RPC.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list account summaries](AccountSummaryListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaListAccountSummariesResponse {
    /// Account summaries of all accounts the caller has access to.
    #[serde(rename="accountSummaries")]
    
    pub account_summaries: Option<Vec<GoogleAnalyticsAdminV1alphaAccountSummary>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleAnalyticsAdminV1alphaListAccountSummariesResponse {}


/// Request message for ListAccounts RPC.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list accounts](AccountListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaListAccountsResponse {
    /// Results that were accessible to the caller.
    
    pub accounts: Option<Vec<GoogleAnalyticsAdminV1alphaAccount>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleAnalyticsAdminV1alphaListAccountsResponse {}


/// Response message for ListConversionEvents RPC.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [conversion events list properties](PropertyConversionEventListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaListConversionEventsResponse {
    /// The requested conversion events
    #[serde(rename="conversionEvents")]
    
    pub conversion_events: Option<Vec<GoogleAnalyticsAdminV1alphaConversionEvent>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleAnalyticsAdminV1alphaListConversionEventsResponse {}


/// Response message for ListCustomDimensions RPC.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custom dimensions list properties](PropertyCustomDimensionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaListCustomDimensionsResponse {
    /// List of CustomDimensions.
    #[serde(rename="customDimensions")]
    
    pub custom_dimensions: Option<Vec<GoogleAnalyticsAdminV1alphaCustomDimension>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleAnalyticsAdminV1alphaListCustomDimensionsResponse {}


/// Response message for ListCustomMetrics RPC.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custom metrics list properties](PropertyCustomMetricListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaListCustomMetricsResponse {
    /// List of CustomMetrics.
    #[serde(rename="customMetrics")]
    
    pub custom_metrics: Option<Vec<GoogleAnalyticsAdminV1alphaCustomMetric>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleAnalyticsAdminV1alphaListCustomMetricsResponse {}


/// Response message for ListDataStreams RPC.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [data streams list properties](PropertyDataStreamListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaListDataStreamsResponse {
    /// List of DataStreams.
    #[serde(rename="dataStreams")]
    
    pub data_streams: Option<Vec<GoogleAnalyticsAdminV1alphaDataStream>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleAnalyticsAdminV1alphaListDataStreamsResponse {}


/// Response message for ListDisplayVideo360AdvertiserLinkProposals RPC.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [display video360 advertiser link proposals list properties](PropertyDisplayVideo360AdvertiserLinkProposalListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaListDisplayVideo360AdvertiserLinkProposalsResponse {
    /// List of DisplayVideo360AdvertiserLinkProposals.
    #[serde(rename="displayVideo360AdvertiserLinkProposals")]
    
    pub display_video360_advertiser_link_proposals: Option<Vec<GoogleAnalyticsAdminV1alphaDisplayVideo360AdvertiserLinkProposal>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleAnalyticsAdminV1alphaListDisplayVideo360AdvertiserLinkProposalsResponse {}


/// Response message for ListDisplayVideo360AdvertiserLinks RPC.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [display video360 advertiser links list properties](PropertyDisplayVideo360AdvertiserLinkListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaListDisplayVideo360AdvertiserLinksResponse {
    /// List of DisplayVideo360AdvertiserLinks.
    #[serde(rename="displayVideo360AdvertiserLinks")]
    
    pub display_video360_advertiser_links: Option<Vec<GoogleAnalyticsAdminV1alphaDisplayVideo360AdvertiserLink>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleAnalyticsAdminV1alphaListDisplayVideo360AdvertiserLinksResponse {}


/// Response message for ListFirebaseLinks RPC
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [firebase links list properties](PropertyFirebaseLinkListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaListFirebaseLinksResponse {
    /// List of FirebaseLinks. This will have at most one value.
    #[serde(rename="firebaseLinks")]
    
    pub firebase_links: Option<Vec<GoogleAnalyticsAdminV1alphaFirebaseLink>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. Currently, Google Analytics supports only one FirebaseLink per property, so this will never be populated.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleAnalyticsAdminV1alphaListFirebaseLinksResponse {}


/// Response message for ListGoogleAdsLinks RPC.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [google ads links list properties](PropertyGoogleAdsLinkListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaListGoogleAdsLinksResponse {
    /// List of GoogleAdsLinks.
    #[serde(rename="googleAdsLinks")]
    
    pub google_ads_links: Option<Vec<GoogleAnalyticsAdminV1alphaGoogleAdsLink>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleAnalyticsAdminV1alphaListGoogleAdsLinksResponse {}


/// Response message for ListMeasurementProtocolSecret RPC
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [data streams measurement protocol secrets list properties](PropertyDataStreamMeasurementProtocolSecretListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaListMeasurementProtocolSecretsResponse {
    /// A list of secrets for the parent stream specified in the request.
    #[serde(rename="measurementProtocolSecrets")]
    
    pub measurement_protocol_secrets: Option<Vec<GoogleAnalyticsAdminV1alphaMeasurementProtocolSecret>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleAnalyticsAdminV1alphaListMeasurementProtocolSecretsResponse {}


/// Response message for ListProperties RPC.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list properties](PropertyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaListPropertiesResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Results that matched the filter criteria and were accessible to the caller.
    
    pub properties: Option<Vec<GoogleAnalyticsAdminV1alphaProperty>>,
}

impl client::ResponseResult for GoogleAnalyticsAdminV1alphaListPropertiesResponse {}


/// Response message for ListUserLinks RPC.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [user links list accounts](AccountUserLinkListCall) (response)
/// * [user links list properties](PropertyUserLinkListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaListUserLinksResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of UserLinks. These will be ordered stably, but in an arbitrary order.
    #[serde(rename="userLinks")]
    
    pub user_links: Option<Vec<GoogleAnalyticsAdminV1alphaUserLink>>,
}

impl client::ResponseResult for GoogleAnalyticsAdminV1alphaListUserLinksResponse {}


/// A secret value used for sending hits to Measurement Protocol.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [data streams measurement protocol secrets create properties](PropertyDataStreamMeasurementProtocolSecretCreateCall) (request|response)
/// * [data streams measurement protocol secrets get properties](PropertyDataStreamMeasurementProtocolSecretGetCall) (response)
/// * [data streams measurement protocol secrets patch properties](PropertyDataStreamMeasurementProtocolSecretPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaMeasurementProtocolSecret {
    /// Required. Human-readable display name for this secret.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Resource name of this secret. This secret may be a child of any type of stream. Format: properties/{property}/webDataStreams/{webDataStream}/measurementProtocolSecrets/{measurementProtocolSecret}
    
    pub name: Option<String>,
    /// Output only. The measurement protocol secret value. Pass this value to the api_secret field of the Measurement Protocol API when sending hits to this secret's parent property.
    #[serde(rename="secretValue")]
    
    pub secret_value: Option<String>,
}

impl client::RequestValue for GoogleAnalyticsAdminV1alphaMeasurementProtocolSecret {}
impl client::ResponseResult for GoogleAnalyticsAdminV1alphaMeasurementProtocolSecret {}


/// A resource message representing a Google Analytics GA4 property.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create properties](PropertyCreateCall) (request|response)
/// * [delete properties](PropertyDeleteCall) (response)
/// * [get properties](PropertyGetCall) (response)
/// * [patch properties](PropertyPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaProperty {
    /// Immutable. The resource name of the parent account Format: accounts/{account_id} Example: "accounts/123"
    
    pub account: Option<String>,
    /// Output only. Time when the entity was originally created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The currency type used in reports involving monetary values. Format: https://en.wikipedia.org/wiki/ISO_4217 Examples: "USD", "EUR", "JPY"
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// Output only. If set, the time at which this property was trashed. If not set, then this property is not currently in the trash can.
    #[serde(rename="deleteTime")]
    
    pub delete_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. Human-readable display name for this property. The max allowed display name length is 100 UTF-16 code units.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. If set, the time at which this trashed property will be permanently deleted. If not set, then this property is not currently in the trash can and is not slated to be deleted.
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Industry associated with this property Example: AUTOMOTIVE, FOOD_AND_DRINK
    #[serde(rename="industryCategory")]
    
    pub industry_category: Option<String>,
    /// Output only. Resource name of this property. Format: properties/{property_id} Example: "properties/1000"
    
    pub name: Option<String>,
    /// Immutable. Resource name of this property's logical parent. Note: The Property-Moving UI can be used to change the parent. Format: accounts/{account} Example: "accounts/100"
    
    pub parent: Option<String>,
    /// Output only. The Google Analytics service level that applies to this property.
    #[serde(rename="serviceLevel")]
    
    pub service_level: Option<String>,
    /// Required. Reporting Time Zone, used as the day boundary for reports, regardless of where the data originates. If the time zone honors DST, Analytics will automatically adjust for the changes. NOTE: Changing the time zone only affects data going forward, and is not applied retroactively. Format: https://www.iana.org/time-zones Example: "America/Los_Angeles"
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<String>,
    /// Output only. Time when entity payload fields were last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GoogleAnalyticsAdminV1alphaProperty {}
impl client::ResponseResult for GoogleAnalyticsAdminV1alphaProperty {}


/// A virtual resource representing metadata for a GA4 property.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaPropertySummary {
    /// Display name for the property referred to in this property summary.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Resource name of property referred to by this property summary Format: properties/{property_id} Example: "properties/1000"
    
    pub property: Option<String>,
}

impl client::Part for GoogleAnalyticsAdminV1alphaPropertySummary {}


/// Request message for ProvisionAccountTicket RPC.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [provision account ticket accounts](AccountProvisionAccountTicketCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaProvisionAccountTicketRequest {
    /// The account to create.
    
    pub account: Option<GoogleAnalyticsAdminV1alphaAccount>,
    /// Redirect URI where the user will be sent after accepting Terms of Service. Must be configured in Developers Console as a Redirect URI
    #[serde(rename="redirectUri")]
    
    pub redirect_uri: Option<String>,
}

impl client::RequestValue for GoogleAnalyticsAdminV1alphaProvisionAccountTicketRequest {}


/// Response message for ProvisionAccountTicket RPC.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [provision account ticket accounts](AccountProvisionAccountTicketCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaProvisionAccountTicketResponse {
    /// The param to be passed in the ToS link.
    #[serde(rename="accountTicketId")]
    
    pub account_ticket_id: Option<String>,
}

impl client::ResponseResult for GoogleAnalyticsAdminV1alphaProvisionAccountTicketResponse {}


/// Request message for SearchChangeHistoryEvents RPC.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search change history events accounts](AccountSearchChangeHistoryEventCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaSearchChangeHistoryEventsRequest {
    /// Optional. If set, only return changes that match one or more of these types of actions.
    
    pub action: Option<Vec<String>>,
    /// Optional. If set, only return changes if they are made by a user in this list.
    #[serde(rename="actorEmail")]
    
    pub actor_email: Option<Vec<String>>,
    /// Optional. If set, only return changes made after this time (inclusive).
    #[serde(rename="earliestChangeTime")]
    
    pub earliest_change_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. If set, only return changes made before this time (inclusive).
    #[serde(rename="latestChangeTime")]
    
    pub latest_change_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. The maximum number of ChangeHistoryEvent items to return. The service may return fewer than this value, even if there are additional pages. If unspecified, at most 50 items will be returned. The maximum value is 200 (higher values will be coerced to the maximum).
    #[serde(rename="pageSize")]
    
    pub page_size: Option<i32>,
    /// Optional. A page token, received from a previous `SearchChangeHistoryEvents` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `SearchChangeHistoryEvents` must match the call that provided the page token.
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
    /// Optional. Resource name for a child property. If set, only return changes made to this property or its child resources.
    
    pub property: Option<String>,
    /// Optional. If set, only return changes if they are for a resource that matches at least one of these types.
    #[serde(rename="resourceType")]
    
    pub resource_type: Option<Vec<String>>,
}

impl client::RequestValue for GoogleAnalyticsAdminV1alphaSearchChangeHistoryEventsRequest {}


/// Response message for SearchAccounts RPC.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search change history events accounts](AccountSearchChangeHistoryEventCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaSearchChangeHistoryEventsResponse {
    /// Results that were accessible to the caller.
    #[serde(rename="changeHistoryEvents")]
    
    pub change_history_events: Option<Vec<GoogleAnalyticsAdminV1alphaChangeHistoryEvent>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleAnalyticsAdminV1alphaSearchChangeHistoryEventsResponse {}


/// Request message for UpdateUserLink RPC.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaUpdateUserLinkRequest {
    /// Required. The user link to update.
    #[serde(rename="userLink")]
    
    pub user_link: Option<GoogleAnalyticsAdminV1alphaUserLink>,
}

impl client::Part for GoogleAnalyticsAdminV1alphaUpdateUserLinkRequest {}


/// A resource message representing a user’s permissions on an Account or Property resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [user links create accounts](AccountUserLinkCreateCall) (request|response)
/// * [user links get accounts](AccountUserLinkGetCall) (response)
/// * [user links patch accounts](AccountUserLinkPatchCall) (request|response)
/// * [user links create properties](PropertyUserLinkCreateCall) (request|response)
/// * [user links get properties](PropertyUserLinkGetCall) (response)
/// * [user links patch properties](PropertyUserLinkPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAnalyticsAdminV1alphaUserLink {
    /// Roles directly assigned to this user for this account or property. Valid values: predefinedRoles/viewer predefinedRoles/analyst predefinedRoles/editor predefinedRoles/admin predefinedRoles/no-cost-data predefinedRoles/no-revenue-data Excludes roles that are inherited from a higher-level entity, group, or organization admin role. A UserLink that is updated to have an empty list of direct_roles will be deleted.
    #[serde(rename="directRoles")]
    
    pub direct_roles: Option<Vec<String>>,
    /// Immutable. Email address of the user to link
    #[serde(rename="emailAddress")]
    
    pub email_address: Option<String>,
    /// Output only. Example format: properties/1234/userLinks/5678
    
    pub name: Option<String>,
}

impl client::RequestValue for GoogleAnalyticsAdminV1alphaUserLink {}
impl client::ResponseResult for GoogleAnalyticsAdminV1alphaUserLink {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [user links batch delete accounts](AccountUserLinkBatchDeleteCall) (response)
/// * [user links delete accounts](AccountUserLinkDeleteCall) (response)
/// * [delete accounts](AccountDeleteCall) (response)
/// * [conversion events delete properties](PropertyConversionEventDeleteCall) (response)
/// * [custom dimensions archive properties](PropertyCustomDimensionArchiveCall) (response)
/// * [custom metrics archive properties](PropertyCustomMetricArchiveCall) (response)
/// * [data streams measurement protocol secrets delete properties](PropertyDataStreamMeasurementProtocolSecretDeleteCall) (response)
/// * [data streams delete properties](PropertyDataStreamDeleteCall) (response)
/// * [display video360 advertiser link proposals delete properties](PropertyDisplayVideo360AdvertiserLinkProposalDeleteCall) (response)
/// * [display video360 advertiser links delete properties](PropertyDisplayVideo360AdvertiserLinkDeleteCall) (response)
/// * [firebase links delete properties](PropertyFirebaseLinkDeleteCall) (response)
/// * [google ads links delete properties](PropertyGoogleAdsLinkDeleteCall) (response)
/// * [user links batch delete properties](PropertyUserLinkBatchDeleteCall) (response)
/// * [user links delete properties](PropertyUserLinkDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleProtobufEmpty { _never_set: Option<bool> }

impl client::ResponseResult for GoogleProtobufEmpty {}


