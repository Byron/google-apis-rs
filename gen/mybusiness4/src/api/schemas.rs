use super::*;
/// Request message for AccessControl.AcceptInvitation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [invitations accept accounts](AccountInvitationAcceptCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AcceptInvitationRequest { _never_set: Option<bool> }

impl client::RequestValue for AcceptInvitationRequest {}


/// An account is a container for your business’s locations. If you are the only
/// user who manages locations for your business, you can use your personal
/// Google Account. To share management of locations with multiple users,
/// \[create a business account\]
/// (https://support.google.com/business/answer/6085339?ref_topic=6085325).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [admins create accounts](AccountAdminCreateCall) (none)
/// * [admins delete accounts](AccountAdminDeleteCall) (none)
/// * [admins list accounts](AccountAdminListCall) (none)
/// * [admins patch accounts](AccountAdminPatchCall) (none)
/// * [invitations accept accounts](AccountInvitationAcceptCall) (none)
/// * [invitations decline accounts](AccountInvitationDeclineCall) (none)
/// * [invitations list accounts](AccountInvitationListCall) (none)
/// * [locations admins create accounts](AccountLocationAdminCreateCall) (none)
/// * [locations admins delete accounts](AccountLocationAdminDeleteCall) (none)
/// * [locations admins list accounts](AccountLocationAdminListCall) (none)
/// * [locations admins patch accounts](AccountLocationAdminPatchCall) (none)
/// * [locations followers get metadata accounts](AccountLocationFollowerGetMetadataCall) (none)
/// * [locations local posts create accounts](AccountLocationLocalPostCreateCall) (none)
/// * [locations local posts delete accounts](AccountLocationLocalPostDeleteCall) (none)
/// * [locations local posts get accounts](AccountLocationLocalPostGetCall) (none)
/// * [locations local posts list accounts](AccountLocationLocalPostListCall) (none)
/// * [locations local posts patch accounts](AccountLocationLocalPostPatchCall) (none)
/// * [locations local posts report insights accounts](AccountLocationLocalPostReportInsightCall) (none)
/// * [locations media customers get accounts](AccountLocationMediaCustomerGetCall) (none)
/// * [locations media customers list accounts](AccountLocationMediaCustomerListCall) (none)
/// * [locations media create accounts](AccountLocationMediaCreateCall) (none)
/// * [locations media delete accounts](AccountLocationMediaDeleteCall) (none)
/// * [locations media get accounts](AccountLocationMediaGetCall) (none)
/// * [locations media list accounts](AccountLocationMediaListCall) (none)
/// * [locations media patch accounts](AccountLocationMediaPatchCall) (none)
/// * [locations media start upload accounts](AccountLocationMediaStartUploadCall) (none)
/// * [locations questions answers delete accounts](AccountLocationQuestionAnswerDeleteCall) (none)
/// * [locations questions answers list accounts](AccountLocationQuestionAnswerListCall) (none)
/// * [locations questions answers upsert accounts](AccountLocationQuestionAnswerUpsertCall) (none)
/// * [locations questions create accounts](AccountLocationQuestionCreateCall) (none)
/// * [locations questions delete accounts](AccountLocationQuestionDeleteCall) (none)
/// * [locations questions list accounts](AccountLocationQuestionListCall) (none)
/// * [locations questions patch accounts](AccountLocationQuestionPatchCall) (none)
/// * [locations reviews delete reply accounts](AccountLocationReviewDeleteReplyCall) (none)
/// * [locations reviews get accounts](AccountLocationReviewGetCall) (none)
/// * [locations reviews list accounts](AccountLocationReviewListCall) (none)
/// * [locations reviews update reply accounts](AccountLocationReviewUpdateReplyCall) (none)
/// * [locations verifications complete accounts](AccountLocationVerificationCompleteCall) (none)
/// * [locations verifications list accounts](AccountLocationVerificationListCall) (none)
/// * [locations associate accounts](AccountLocationAssociateCall) (none)
/// * [locations batch get accounts](AccountLocationBatchGetCall) (none)
/// * [locations batch get reviews accounts](AccountLocationBatchGetReviewCall) (none)
/// * [locations clear association accounts](AccountLocationClearAssociationCall) (none)
/// * [locations create accounts](AccountLocationCreateCall) (none)
/// * [locations delete accounts](AccountLocationDeleteCall) (none)
/// * [locations fetch verification options accounts](AccountLocationFetchVerificationOptionCall) (none)
/// * [locations find matches accounts](AccountLocationFindMatchCall) (none)
/// * [locations get accounts](AccountLocationGetCall) (none)
/// * [locations get google updated accounts](AccountLocationGetGoogleUpdatedCall) (none)
/// * [locations list accounts](AccountLocationListCall) (none)
/// * [locations patch accounts](AccountLocationPatchCall) (none)
/// * [locations report insights accounts](AccountLocationReportInsightCall) (none)
/// * [locations transfer accounts](AccountLocationTransferCall) (none)
/// * [locations verify accounts](AccountLocationVerifyCall) (none)
/// * [create accounts](AccountCreateCall) (request|response)
/// * [delete notifications accounts](AccountDeleteNotificationCall) (none)
/// * [generate account number accounts](AccountGenerateAccountNumberCall) (response)
/// * [get accounts](AccountGetCall) (response)
/// * [get notifications accounts](AccountGetNotificationCall) (none)
/// * [list accounts](AccountListCall) (none)
/// * [list recommend google locations accounts](AccountListRecommendGoogleLocationCall) (none)
/// * [update accounts](AccountUpdateCall) (request|response)
/// * [update notifications accounts](AccountUpdateNotificationCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    /// The name of the account. <aside class="note"><b>Note:</b> For an account
    /// with AccountType `PERSONAL`,
    /// this is the first and last name of the user account.</aside>
    #[serde(rename="accountName")]
    
    pub account_name: Option<String>,
    /// Account reference number if provisioned.
    #[serde(rename="accountNumber")]
    
    pub account_number: Option<String>,
    /// The resource name, in the format `accounts/{account_id}`.
    
    pub name: Option<String>,
    /// Additional info for an organization. This is populated only for an
    /// organization account.
    #[serde(rename="organizationInfo")]
    
    pub organization_info: Option<OrganizationInfo>,
    /// Output only. Specifies the PermissionLevel the caller has for this account.
    #[serde(rename="permissionLevel")]
    
    pub permission_level: Option<AccountPermissionLevelEnum>,
    /// Output only. Specifies the AccountRole
    /// the caller has for this account.
    
    pub role: Option<AccountRoleEnum>,
    /// Output only. Indicates the AccountState of this account.
    
    pub state: Option<AccountState>,
    /// Output only. Specifies the AccountType
    /// of this account.
    #[serde(rename="type")]
    
    pub type_: Option<AccountTypeEnum>,
}

impl client::RequestValue for Account {}
impl client::Resource for Account {}
impl client::ResponseResult for Account {}


/// Indicates status of the account, such as whether the account
/// has been verified by Google.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountState {
    /// If verified, future locations that are created are automatically
    /// connected to Google Maps, and have Google+ pages created, without requiring
    /// moderation.
    
    pub status: Option<AccountStateStatusEnum>,
    /// Indicates whether the account is vetted by Google. A vetted account is able
    /// to verify locations via the VETTED_PARTNER method.
    #[serde(rename="vettedStatus")]
    
    pub vetted_status: Option<AccountStateVettedStatusEnum>,
}

impl client::Part for AccountState {}


/// Additional information that is surfaced in AdWords.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdWordsLocationExtensions {
    /// An alternate phone number to display on AdWords location extensions
    /// instead of the location's primary phone number.
    #[serde(rename="adPhone")]
    
    pub ad_phone: Option<String>,
}

impl client::Part for AdWordsLocationExtensions {}


/// Input for ADDRESS verification.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddressInput {
    /// Contact name the mail should be sent to.
    #[serde(rename="mailerContactName")]
    
    pub mailer_contact_name: Option<String>,
}

impl client::Part for AddressInput {}


/// Display data for verifications through postcard.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddressVerificationData {
    /// Address that a postcard can be sent to.
    
    pub address: Option<PostalAddress>,
    /// Merchant's business name.
    #[serde(rename="businessName")]
    
    pub business_name: Option<String>,
}

impl client::Part for AddressVerificationData {}


/// An administrator of an Account or a
/// Location.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [admins create accounts](AccountAdminCreateCall) (request|response)
/// * [admins patch accounts](AccountAdminPatchCall) (request|response)
/// * [locations admins create accounts](AccountLocationAdminCreateCall) (request|response)
/// * [locations admins patch accounts](AccountLocationAdminPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Admin {
    /// The name of the admin. When making the initial invitation, this is the
    /// invitee's email address. On `GET` calls, the user's email address is
    /// returned if the invitation is still pending. Otherwise, it contains
    /// the user's first and last names.
    #[serde(rename="adminName")]
    
    pub admin_name: Option<String>,
    /// The resource name. For account admins, this is in the form:
    /// `accounts/{account_id}/admins/{admin_id}`
    /// 
    /// For location admins, this is in the form:
    /// `accounts/{account_id}/locations/{location_id}/admins/{admin_id}`
    
    pub name: Option<String>,
    /// Output only. Indicates whether this admin has a pending invitation for the
    /// specified resource.
    #[serde(rename="pendingInvitation")]
    
    pub pending_invitation: Option<bool>,
    /// Specifies the AdminRole that this
    /// admin uses with the specified Account
    /// or Location resource.
    
    pub role: Option<AdminRoleEnum>,
}

impl client::RequestValue for Admin {}
impl client::ResponseResult for Admin {}


/// Represents an answer to a question
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations questions answers upsert accounts](AccountLocationQuestionAnswerUpsertCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Answer {
    /// Output only. The author of the answer.
    
    pub author: Option<Author>,
    /// Output only. The timestamp for when the answer was written.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The unique name for the answer
    /// accounts/*/locations/*/questions/*/answers/*
    
    pub name: Option<String>,
    /// The text of the answer. It should contain at least one non-whitespace
    /// character. The maximum length is 4096 characters.
    
    pub text: Option<String>,
    /// Output only. The timestamp for when the answer was last modified.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The number of upvotes for the answer.
    #[serde(rename="upvoteCount")]
    
    pub upvote_count: Option<i32>,
}

impl client::ResponseResult for Answer {}


/// Request message for Locations.AssociateLocationRequest.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations associate accounts](AccountLocationAssociateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AssociateLocationRequest {
    /// The association to establish. If not set, it indicates no match.
    #[serde(rename="placeId")]
    
    pub place_id: Option<String>,
}

impl client::RequestValue for AssociateLocationRequest {}


/// A location attribute. Attributes provide additional information about a
/// location. The attributes that can be set on a location may vary based on
/// the properties of that location (for example, category). Available attributes
/// are determined by Google and may be added and removed without API changes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list attributes](AttributeListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Attribute {
    /// The ID of the attribute. Attribute IDs are provided by Google.
    #[serde(rename="attributeId")]
    
    pub attribute_id: Option<String>,
    /// When the attribute value type is REPEATED_ENUM, this contains the attribute
    /// value, and the other values fields must be empty.
    #[serde(rename="repeatedEnumValue")]
    
    pub repeated_enum_value: Option<RepeatedEnumAttributeValue>,
    /// When the attribute value type is URL, this field contains the value(s) for
    /// this attribute, and the other values fields must be empty.
    #[serde(rename="urlValues")]
    
    pub url_values: Option<Vec<UrlAttributeValue>>,
    /// Output only. The type of value that this attribute contains. This should be
    /// used to determine how to interpret the value.
    #[serde(rename="valueType")]
    
    pub value_type: Option<AttributeValueTypeEnum>,
    /// The values for this attribute. The type of the values supplied must match
    /// that expected for that attribute; see
    /// [AttributeValueType](https://developers.google.com/my-business/reference/rest/v4/AttributeValueType).
    /// This is a repeated field where multiple attribute values may be provided.
    /// Attribute types only support one value.
    
    pub values: Option<Vec<json::Value>>,
}

impl client::Resource for Attribute {}


/// Metadata for an attribute. Contains display information for the attribute,
/// including a localized name and a heading for grouping related attributes
/// together.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AttributeMetadata {
    /// The ID of the attribute.
    #[serde(rename="attributeId")]
    
    pub attribute_id: Option<String>,
    /// The localized display name for the attribute, if available; otherwise,
    /// the English display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The localized display name of the group that contains this attribute, if
    /// available; otherwise, the English group name. Related attributes are
    /// collected into a group and should be displayed together under the heading
    /// given here.
    #[serde(rename="groupDisplayName")]
    
    pub group_display_name: Option<String>,
    /// If true, the attribute is deprecated and should no longer be used. If
    /// deprecated, updating this attribute will not result in an error, but
    /// updates will not be saved. At some point after being deprecated, the
    /// attribute will be removed entirely and it will become an error.
    #[serde(rename="isDeprecated")]
    
    pub is_deprecated: Option<bool>,
    /// If true, the attribute supports multiple values. If false, only a single
    /// value should be provided.
    #[serde(rename="isRepeatable")]
    
    pub is_repeatable: Option<bool>,
    /// For some types of attributes (for example, enums), a list of supported
    /// values and corresponding display names for those values is provided.
    #[serde(rename="valueMetadata")]
    
    pub value_metadata: Option<Vec<AttributeValueMetadata>>,
    /// The value type for the attribute. Values set and retrieved should be
    /// expected to be of this type.
    #[serde(rename="valueType")]
    
    pub value_type: Option<AttributeMetadataValueTypeEnum>,
}

impl client::Part for AttributeMetadata {}


/// Metadata for supported attribute values.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AttributeValueMetadata {
    /// The display name for this value, localized where available; otherwise, in
    /// English. The value display name is intended to be used in context with
    /// the attribute display name.
    /// 
    /// For example, for a "WiFi" enum attribute, this could contain "Paid" to
    /// represent paid Wi-Fi.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The attribute value.
    
    pub value: Option<json::Value>,
}

impl client::Part for AttributeValueMetadata {}


/// Attribution information for customer media items, such as the contributor's
/// name and profile picture.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Attribution {
    /// The user name to attribute the media item to.
    #[serde(rename="profileName")]
    
    pub profile_name: Option<String>,
    /// URL of the attributed user's profile photo thumbnail.
    #[serde(rename="profilePhotoUrl")]
    
    pub profile_photo_url: Option<String>,
    /// The URL of the attributed user's Google Maps profile page.
    #[serde(rename="profileUrl")]
    
    pub profile_url: Option<String>,
    /// The URL of the takedown page, where the media item can be reported if it
    /// is inappropriate.
    #[serde(rename="takedownUrl")]
    
    pub takedown_url: Option<String>,
}

impl client::Part for Attribution {}


/// Represents the author of a question or answer
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Author {
    /// The display name of the user
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The profile photo URL of the user.
    #[serde(rename="profilePhotoUrl")]
    
    pub profile_photo_url: Option<String>,
    /// The type of user the author is.
    #[serde(rename="type")]
    
    pub type_: Option<AuthorTypeEnum>,
}

impl client::Part for Author {}


/// A request for basic metric insights.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BasicMetricsRequest {
    /// A collection of metrics to return values for including the options for
    /// how the data should be returned.
    #[serde(rename="metricRequests")]
    
    pub metric_requests: Option<Vec<MetricRequest>>,
    /// The range to gather metrics for. The maximum range is 18 months from the
    /// request date. In some cases, the data may still be missing for days close
    /// to the request date. Missing data will be specified in the `metricValues`
    /// in the response.
    #[serde(rename="timeRange")]
    
    pub time_range: Option<TimeRange>,
}

impl client::Part for BasicMetricsRequest {}


/// Request message for Locations.BatchGetLocations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations batch get accounts](AccountLocationBatchGetCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchGetLocationsRequest {
    /// A collection of locations to fetch, specified by their names.
    #[serde(rename="locationNames")]
    
    pub location_names: Option<Vec<String>>,
}

impl client::RequestValue for BatchGetLocationsRequest {}


/// Response message for Locations.BatchGetLocations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations batch get accounts](AccountLocationBatchGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchGetLocationsResponse {
    /// A collection of locations.
    
    pub locations: Option<Vec<Location>>,
}

impl client::ResponseResult for BatchGetLocationsResponse {}


/// Request message for Reviews.BatchGetReviews.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations batch get reviews accounts](AccountLocationBatchGetReviewCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchGetReviewsRequest {
    /// Whether to ignore rating-only reviews.
    #[serde(rename="ignoreRatingOnlyReviews")]
    
    pub ignore_rating_only_reviews: Option<bool>,
    /// A collection of locations to fetch reviews for, specified by their names.
    #[serde(rename="locationNames")]
    
    pub location_names: Option<Vec<String>>,
    /// Optional. Specifies the field to sort reviews by.
    /// If unspecified, the order of reviews returned will
    /// default to `update_time desc`.
    /// Valid orders to sort by are `rating`, `rating desc` and `update_time desc`.
    /// `rating` will return reviews in ascending order.
    /// `update_time`(i.e. ascending order) is not supported.
    #[serde(rename="orderBy")]
    
    pub order_by: Option<String>,
    /// How many reviews to fetch per page. The default value is 200.
    #[serde(rename="pageSize")]
    
    pub page_size: Option<i32>,
    /// If specified, it fetches the next page of reviews.
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
}

impl client::RequestValue for BatchGetReviewsRequest {}


/// Response message for Reviews.BatchGetReviews.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations batch get reviews accounts](AccountLocationBatchGetReviewCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchGetReviewsResponse {
    /// Reviews with location information.
    #[serde(rename="locationReviews")]
    
    pub location_reviews: Option<Vec<LocationReview>>,
    /// If the number of reviews exceeded the requested page size, this field
    /// is populated with a token to fetch the next page of reviews on a subsequent
    /// calls. If there are no more reviews, this field will not be present in the
    /// response.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for BatchGetReviewsResponse {}


/// Represents the time periods that this location is open for business.
/// Holds a collection of TimePeriod
/// instances.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BusinessHours {
    /// A collection of times that this location is open for business. Each period
    /// represents a range of hours when the location is open during the week.
    
    pub periods: Option<Vec<TimePeriod>>,
}

impl client::Part for BusinessHours {}


/// An action that is performed when the user clicks through the post
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CallToAction {
    /// The type of action that will be performed.
    #[serde(rename="actionType")]
    
    pub action_type: Option<CallToActionActionTypeEnum>,
    /// The URL the user will be directed to upon clicking. This field should be
    /// left unset for Call CTA.
    
    pub url: Option<String>,
}

impl client::Part for CallToAction {}


/// A category describing what this business is (not what it does). For a list of
/// valid category IDs, and the mappings to their human-readable names, see
/// [categories.list](https://developers.google.com/my-business/reference/rest/v4/categories/list).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Category {
    /// @OutputOnly.
    /// A stable ID (provided by Google) for this category.
    /// The `category_id` must be specified when modifying the category (when
    /// creating or updating a location).
    #[serde(rename="categoryId")]
    
    pub category_id: Option<String>,
    /// @OutputOnly.
    /// The human-readable name of the category. This is set when
    /// reading the location. When modifying the location, `category_id` must be
    /// set.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
}

impl client::Part for Category {}


/// A chain is a brand that your business’s locations can be affiliated with.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get chains](ChainGetCall) (response)
/// * [search chains](ChainSearchCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Chain {
    /// Names of the chain.
    #[serde(rename="chainNames")]
    
    pub chain_names: Option<Vec<ChainName>>,
    /// Number of locations that are part of this chain.
    #[serde(rename="locationCount")]
    
    pub location_count: Option<i32>,
    /// The chain's resource name, in the format `chains/{chain_place_id}`.
    
    pub name: Option<String>,
    /// Websites of the chain.
    
    pub websites: Option<Vec<ChainUrl>>,
}

impl client::Resource for Chain {}
impl client::ResponseResult for Chain {}


/// Name to be used when displaying the chain.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChainName {
    /// The display name for this chain.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The BCP 47 code of language of the name.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
}

impl client::Part for ChainName {}


/// Url to be used when displaying the chain.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChainUrl {
    /// The url for this chain.
    
    pub url: Option<String>,
}

impl client::Part for ChainUrl {}


/// Request message for Locations.ClearLocationAssociationRequest.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations clear association accounts](AccountLocationClearAssociationCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClearLocationAssociationRequest { _never_set: Option<bool> }

impl client::RequestValue for ClearLocationAssociationRequest {}


/// Request message for Verifications.CompleteVerificationAction.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations verifications complete accounts](AccountLocationVerificationCompleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CompleteVerificationRequest {
    /// PIN code received by the merchant to complete the verification.
    
    pub pin: Option<String>,
}

impl client::RequestValue for CompleteVerificationRequest {}


/// Response message for Verifications.CompleteVerificationAction.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations verifications complete accounts](AccountLocationVerificationCompleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CompleteVerificationResponse {
    /// The completed verification.
    
    pub verification: Option<Verification>,
}

impl client::ResponseResult for CompleteVerificationResponse {}


/// Represents a whole or partial calendar date, e.g. a birthday. The time of day
/// and time zone are either specified elsewhere or are not significant. The date
/// is relative to the Proleptic Gregorian Calendar. This can represent:
/// 
/// * A full date, with non-zero year, month and day values
/// * A month and day value, with a zero year, e.g. an anniversary
/// * A year on its own, with zero month and day values
/// * A year and month value, with a zero day, e.g. a credit card expiration date
/// 
/// Related types are google.type.TimeOfDay and `google.protobuf.Timestamp`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Date {
    /// Day of month. Must be from 1 to 31 and valid for the year and month, or 0
    /// if specifying a year by itself or a year and month where the day is not
    /// significant.
    
    pub day: Option<i32>,
    /// Month of year. Must be from 1 to 12, or 0 if specifying a year without a
    /// month and day.
    
    pub month: Option<i32>,
    /// Year of date. Must be from 1 to 9999, or 0 if specifying a date without
    /// a year.
    
    pub year: Option<i32>,
}

impl client::Part for Date {}


/// Request message for AccessControl.DeclineInvitation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [invitations decline accounts](AccountInvitationDeclineCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeclineInvitationRequest { _never_set: Option<bool> }

impl client::RequestValue for DeclineInvitationRequest {}


/// A value for a single metric with a given time dimension.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DimensionalMetricValue {
    /// The option that requested this dimensional value.
    #[serde(rename="metricOption")]
    
    pub metric_option: Option<DimensionalMetricValueMetricOptionEnum>,
    /// The dimension for the value.
    #[serde(rename="timeDimension")]
    
    pub time_dimension: Option<TimeDimension>,
    /// The value. If no value is set, then the requested data is missing.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub value: Option<i64>,
}

impl client::Part for DimensionalMetricValue {}


/// Dimensions of the media item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Dimensions {
    /// Height of the media item, in pixels.
    #[serde(rename="heightPixels")]
    
    pub height_pixels: Option<i32>,
    /// Width of the media item, in pixels.
    #[serde(rename="widthPixels")]
    
    pub width_pixels: Option<i32>,
}

impl client::Part for Dimensions {}


/// A request for driving direction insights.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DrivingDirectionMetricsRequest {
    /// The BCP 47 code for the language. If a language code is not provided,
    /// it defaults to English.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// The number of days to aggregate data for. Results returned will
    /// be available data over the last number of requested days.
    /// Valid values are 7, 30, and 90.
    #[serde(rename="numDays")]
    
    pub num_days: Option<DrivingDirectionMetricsRequestNumDaysEnum>,
}

impl client::Part for DrivingDirectionMetricsRequest {}


/// Information about the location that this location duplicates.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Duplicate {
    /// Indicates whether the user has access to the location it duplicates.
    
    pub access: Option<DuplicateAccessEnum>,
    /// The resource name of the location that this duplicates. Only populated
    /// if the authenticated user has access rights to that location and that
    /// location is not deleted.
    #[serde(rename="locationName")]
    
    pub location_name: Option<String>,
    /// The place ID of the location that this duplicates.
    #[serde(rename="placeId")]
    
    pub place_id: Option<String>,
}

impl client::Part for Duplicate {}


/// Input for EMAIL verification.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EmailInput {
    /// Email address where the PIN should be sent to.
    /// 
    /// An email address is accepted only if it is one of the addresses provided
    /// by FetchVerificationOptions. If the EmailVerificationData has
    /// is_user_name_editable set to true, the client may specify a different
    /// user name (local-part) but must match the domain name.
    #[serde(rename="emailAddress")]
    
    pub email_address: Option<String>,
}

impl client::Part for EmailInput {}


/// Display data for verifications through email.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EmailVerificationData {
    /// Domain name in the email address. e.g. "gmail.com" in foo@gmail.com
    #[serde(rename="domainName")]
    
    pub domain_name: Option<String>,
    /// Whether client is allowed to provide a different user name.
    #[serde(rename="isUserNameEditable")]
    
    pub is_user_name_editable: Option<bool>,
    /// User name in the email address. e.g. "foo" in foo@gmail.com
    #[serde(rename="userName")]
    
    pub user_name: Option<String>,
}

impl client::Part for EmailVerificationData {}


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
/// * [admins delete accounts](AccountAdminDeleteCall) (response)
/// * [invitations accept accounts](AccountInvitationAcceptCall) (response)
/// * [invitations decline accounts](AccountInvitationDeclineCall) (response)
/// * [locations admins delete accounts](AccountLocationAdminDeleteCall) (response)
/// * [locations local posts delete accounts](AccountLocationLocalPostDeleteCall) (response)
/// * [locations media delete accounts](AccountLocationMediaDeleteCall) (response)
/// * [locations questions answers delete accounts](AccountLocationQuestionAnswerDeleteCall) (response)
/// * [locations questions delete accounts](AccountLocationQuestionDeleteCall) (response)
/// * [locations reviews delete reply accounts](AccountLocationReviewDeleteReplyCall) (response)
/// * [locations associate accounts](AccountLocationAssociateCall) (response)
/// * [locations clear association accounts](AccountLocationClearAssociationCall) (response)
/// * [locations delete accounts](AccountLocationDeleteCall) (response)
/// * [delete notifications accounts](AccountDeleteNotificationCall) (response)
/// * [report google locations](GoogleLocationReportCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// Request message for Verifications.FetchVerificationOptions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations fetch verification options accounts](AccountLocationFetchVerificationOptionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FetchVerificationOptionsRequest {
    /// Extra context information for the verification of service businesses.
    /// Required for the locations whose business type is CUSTOMER_LOCATION_ONLY.
    /// INVALID_ARGUMENT will be thrown if it is set for other business types of
    /// locations.
    
    pub context: Option<ServiceBusinessContext>,
    /// The BCP 47 language code representing the language that is to be used for
    /// the verification process. Available options vary by language.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
}

impl client::RequestValue for FetchVerificationOptionsRequest {}


/// Response message for Verifications.FetchVerificationOptions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations fetch verification options accounts](AccountLocationFetchVerificationOptionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FetchVerificationOptionsResponse {
    /// The available verification options.
    
    pub options: Option<Vec<VerificationOption>>,
}

impl client::ResponseResult for FetchVerificationOptionsResponse {}


/// Request message for Locations.FindMatchingLocations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations find matches accounts](AccountLocationFindMatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FindMatchingLocationsRequest {
    /// The preferred language for the matching location (in BCP-47 format).
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Deprecated. This field is ignored for all requests.
    #[serde(rename="maxCacheDuration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub max_cache_duration: Option<client::chrono::Duration>,
    /// The number of matches to return. The default value is 3, with a maximum
    /// of 10. Note that latency may increase if more are requested. There is no
    /// pagination.
    #[serde(rename="numResults")]
    
    pub num_results: Option<i32>,
}

impl client::RequestValue for FindMatchingLocationsRequest {}


/// Response message for Locations.FindMatchingLocations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations find matches accounts](AccountLocationFindMatchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FindMatchingLocationsResponse {
    /// When the matching algorithm was last executed for this location.
    #[serde(rename="matchTime")]
    
    pub match_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A collection of locations that are potential matches to the specified
    /// location, listed in order from best to least match. If there is an exact
    /// match, it will be in the first position.
    #[serde(rename="matchedLocations")]
    
    pub matched_locations: Option<Vec<MatchedLocation>>,
}

impl client::ResponseResult for FindMatchingLocationsResponse {}


/// Follower metadata for a location.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations followers get metadata accounts](AccountLocationFollowerGetMetadataCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FollowersMetadata {
    /// Total number of followers for the location.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<u64>,
    /// The resource name for this.
    /// accounts/{account_id}/locations/{location_id}/followers/metadata
    
    pub name: Option<String>,
}

impl client::ResponseResult for FollowersMetadata {}


/// Request message for Accounts.GenerateAccountNumber.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [generate account number accounts](AccountGenerateAccountNumberCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GenerateAccountNumberRequest { _never_set: Option<bool> }

impl client::RequestValue for GenerateAccountNumberRequest {}


/// Request message for Verifications.GenerateVerificationToken.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [generate verification tokens](VerificationTokenGenerateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GenerateVerificationTokenRequest {
    /// The target location.
    
    pub location: Option<Location>,
}

impl client::RequestValue for GenerateVerificationTokenRequest {}


/// Response message for Verifications.GenerateVerificationToken.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [generate verification tokens](VerificationTokenGenerateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GenerateVerificationTokenResponse {
    /// The generated token to verify the location.
    
    pub token: Option<VerificationToken>,
}

impl client::ResponseResult for GenerateVerificationTokenResponse {}


/// Represents a Location that is present on
/// Google. This can be a location that has been claimed by the user, someone
/// else, or could be unclaimed.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [report google locations](GoogleLocationReportCall) (none)
/// * [search google locations](GoogleLocationSearchCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleLocation {
    /// The sparsely populated Location
    /// information. This field can be re-used in
    /// CreateLocation if it is
    /// not currently claimed by a user.
    
    pub location: Option<Location>,
    /// Resource name of this GoogleLocation, in the format
    /// `googleLocations/{googleLocationId}`.
    
    pub name: Option<String>,
    /// A URL that will redirect the user to the request admin rights UI.
    /// This field is only present if the location has already been claimed by
    /// any user, including the current user.
    #[serde(rename="requestAdminRightsUrl")]
    
    pub request_admin_rights_url: Option<String>,
}

impl client::Resource for GoogleLocation {}


/// Represents a location that was modified by Google.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations get google updated accounts](AccountLocationGetGoogleUpdatedCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleUpdatedLocation {
    /// The fields that Google updated.
    #[serde(rename="diffMask")]
    
    pub diff_mask: Option<client::FieldMask>,
    /// The Google-updated version of this location.
    
    pub location: Option<Location>,
}

impl client::ResponseResult for GoogleUpdatedLocation {}


/// Output only. Represents a pending invitation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Invitation {
    /// The resource name for the invitation.
    
    pub name: Option<String>,
    /// The invited role on the account.
    
    pub role: Option<InvitationRoleEnum>,
    /// The sparsely populated account this invitation is for.
    #[serde(rename="targetAccount")]
    
    pub target_account: Option<Account>,
    /// The target location this invitation is for.
    #[serde(rename="targetLocation")]
    
    pub target_location: Option<TargetLocation>,
}

impl client::Part for Invitation {}


/// A single list item. Each variation of an item in the price list should
/// have its own Item with its own price data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    /// Required. ID for the item. Price list, section, and item IDs cannot be
    /// duplicated within this Location.
    #[serde(rename="itemId")]
    
    pub item_id: Option<String>,
    /// Required. Language-tagged labels for the item. We recommend that
    /// item names be 140 characters or less, and descriptions 250 characters
    /// or less. At least one set of labels is required.
    
    pub labels: Option<Vec<Label>>,
    /// Optional. Price of the item.
    
    pub price: Option<Money>,
}

impl client::Part for Item {}


/// Label to be used when displaying the price list, section, or item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Label {
    /// Optional. Description of the price list, section, or item.
    
    pub description: Option<String>,
    /// Required. Display name for the price list, section, or item.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Optional. The BCP-47 language code that these strings apply for.
    /// Only one set of labels may be set per language.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
}

impl client::Part for Label {}


/// An object representing a latitude/longitude pair. This is expressed as a pair
/// of doubles representing degrees latitude and degrees longitude. Unless
/// specified otherwise, this must conform to the
/// <a href="http://www.unoosa.org/pdf/icg/2012/template/WGS_84.pdf">WGS84
/// standard</a>. Values must be within normalized ranges.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LatLng {
    /// The latitude in degrees. It must be in the range [-90.0, +90.0].
    
    pub latitude: Option<f64>,
    /// The longitude in degrees. It must be in the range [-180.0, +180.0].
    
    pub longitude: Option<f64>,
}

impl client::Part for LatLng {}


/// Response message for AccessControl.ListAccountAdmins.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [admins list accounts](AccountAdminListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAccountAdminsResponse {
    /// A collection of Admin instances.
    
    pub admins: Option<Vec<Admin>>,
}

impl client::ResponseResult for ListAccountAdminsResponse {}


/// Response message for Accounts.ListAccounts.
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
    /// A collection of accounts to which the user has access. The personal
    /// account of the user doing the query will always be the first item of the
    /// result, unless it is filtered out.
    
    pub accounts: Option<Vec<Account>>,
    /// If the number of accounts exceeds the requested page size, this field is
    /// populated with a token to fetch the next page of accounts on a
    /// subsequent call to `accounts.list`. If there are no more accounts, this
    /// field is not present in the response.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListAccountsResponse {}


/// Response message for QuestionsAndAnswers.ListAnswers
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations questions answers list accounts](AccountLocationQuestionAnswerListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAnswersResponse {
    /// The requested answers.
    
    pub answers: Option<Vec<Answer>>,
    /// If the number of answers exceeds the requested max page size, this field
    /// is populated with a token to fetch the next page of answers on a subsequent
    /// call. If there are no more answers, this field is not present in the
    /// response.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The total number of answers posted for this question across all pages.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
}

impl client::ResponseResult for ListAnswersResponse {}


/// Response message for Locations.ListAttributeMetadata.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list attributes](AttributeListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAttributeMetadataResponse {
    /// A collection of attribute metadata for the available attributes.
    
    pub attributes: Option<Vec<AttributeMetadata>>,
    /// If the number of attributes exceeded the requested page size, this field
    /// will be populated with a token to fetch the next page of attributes on a
    /// subsequent call to `attributes.list`. If there are no more attributes, this
    /// field will not be present in the response.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListAttributeMetadataResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list categories](CategoryListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListBusinessCategoriesResponse {
    /// The categories. Categories are BASIC view. They don't contain any
    /// ServiceType information.
    
    pub categories: Option<Vec<Category>>,
    /// If the number of categories exceeded the requested page size, this field
    /// will be populated with a token to fetch the next page of categories
    /// on a subsequent call to `ListBusinessCategories`.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The total number of categories for the request parameters.
    #[serde(rename="totalCategoryCount")]
    
    pub total_category_count: Option<i32>,
}

impl client::ResponseResult for ListBusinessCategoriesResponse {}


/// Response message for Media.ListCustomerMediaItems.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations media customers list accounts](AccountLocationMediaCustomerListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCustomerMediaItemsResponse {
    /// The returned list of media items.
    #[serde(rename="mediaItems")]
    
    pub media_items: Option<Vec<MediaItem>>,
    /// If there are more media items than the requested page size, then this field
    /// is populated with a token to fetch the next page of media items on a
    /// subsequent call to ListCustomerMediaItems.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The total number of media items for this location, irrespective of
    /// pagination. This number is approximate, particularly when there are
    /// multiple pages of results.
    #[serde(rename="totalMediaItemCount")]
    
    pub total_media_item_count: Option<i32>,
}

impl client::ResponseResult for ListCustomerMediaItemsResponse {}


/// Response message for AccessControl.ListInvitations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [invitations list accounts](AccountInvitationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListInvitationsResponse {
    /// A collection of location invitations that are pending for the account. The
    /// number of invitations listed here cannot exceed 1000.
    
    pub invitations: Option<Vec<Invitation>>,
}

impl client::ResponseResult for ListInvitationsResponse {}


/// Response message for ListLocalPosts
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations local posts list accounts](AccountLocationLocalPostListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListLocalPostsResponse {
    /// The returned list of local posts.
    #[serde(rename="localPosts")]
    
    pub local_posts: Option<Vec<LocalPost>>,
    /// If there are more local posts than the requested page size, then this field
    /// is populated with a token to fetch the next page of local posts on a
    /// subsequent call to `ListLocalPosts`.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListLocalPostsResponse {}


/// Response message for AccessControl.ListLocationAdmins.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations admins list accounts](AccountLocationAdminListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListLocationAdminsResponse {
    /// A collection of Admin instances.
    
    pub admins: Option<Vec<Admin>>,
}

impl client::ResponseResult for ListLocationAdminsResponse {}


/// Response message for Locations.ListLocations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations list accounts](AccountLocationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListLocationsResponse {
    /// The locations.
    
    pub locations: Option<Vec<Location>>,
    /// If the number of locations exceeded the requested page size, this field
    /// is populated with a token to fetch the next page of locations on a
    /// subsequent call to `ListLocations`. If there are no more locations, this
    /// field is not present in the response.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The approximate number of Locations in the list irrespective of pagination.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
}

impl client::ResponseResult for ListLocationsResponse {}


/// Response message for Media.ListMediaItems.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations media list accounts](AccountLocationMediaListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListMediaItemsResponse {
    /// The returned list of media items.
    #[serde(rename="mediaItems")]
    
    pub media_items: Option<Vec<MediaItem>>,
    /// If there are more media items than the requested page size, then this field
    /// is populated with a token to fetch the next page of media items on a
    /// subsequent call to ListMediaItems.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The total number of media items for this location, irrespective of
    /// pagination.
    #[serde(rename="totalMediaItemCount")]
    
    pub total_media_item_count: Option<i32>,
}

impl client::ResponseResult for ListMediaItemsResponse {}


/// Response message for QuestionsAndAnswers.ListQuestions
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations questions list accounts](AccountLocationQuestionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListQuestionsResponse {
    /// If the number of questions exceeds the requested max page size, this field
    /// is populated with a token to fetch the next page of questions on a
    /// subsequent call. If there are no more questions, this field is not present
    /// in the response.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The requested questions,
    
    pub questions: Option<Vec<Question>>,
    /// The total number of questions posted for this location across all pages.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
}

impl client::ResponseResult for ListQuestionsResponse {}


/// Response message for GoogleLocations.ListRecommendedGoogleLocations.
/// 
/// It also contains some locations that have been claimed by other GMB users
/// since the last time they were recommended to this GMB account.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list recommend google locations accounts](AccountListRecommendGoogleLocationCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListRecommendedGoogleLocationsResponse {
    /// The locations recommended to a GMB account.
    /// Each of these represents a GoogleLocation that
    /// is present on Maps.
    /// The locations are sorted in decreasing order of relevance to the GMB
    /// account.
    #[serde(rename="googleLocations")]
    
    pub google_locations: Option<Vec<GoogleLocation>>,
    /// During pagination, if there are more locations available to be fetched in
    /// the next page, this field is populated with a token to fetch the next page
    /// of locations in a subsequent call. If there are no more locations to be
    /// fetched, this field is not present in the response.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The total number of recommended locations for this GMB account,
    /// irrespective of pagination.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
}

impl client::ResponseResult for ListRecommendedGoogleLocationsResponse {}


/// Response message for Reviews.ListReviews.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations reviews list accounts](AccountLocationReviewListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListReviewsResponse {
    /// The average star rating of all reviews for this location
    /// on a scale of 1 to 5, where 5 is the highest rating.
    #[serde(rename="averageRating")]
    
    pub average_rating: Option<f64>,
    /// If the number of reviews exceeded the requested page size, this field
    /// is populated with a token to fetch the next page of reviews on a
    /// subsequent call to ListReviews. If there are no more reviews, this
    /// field is not present in the response.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The reviews.
    
    pub reviews: Option<Vec<Review>>,
    /// The total number of reviews for this location.
    #[serde(rename="totalReviewCount")]
    
    pub total_review_count: Option<i32>,
}

impl client::ResponseResult for ListReviewsResponse {}


/// Response message for Verifications.ListVerifications.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations verifications list accounts](AccountLocationVerificationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListVerificationsResponse {
    /// If the number of verifications exceeded the requested page size, this field
    /// will be populated with a token to fetch the next page of verification on a
    /// subsequent call. If there are no more attributes, this field will not be
    /// present in the response.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of the verifications.
    
    pub verifications: Option<Vec<Verification>>,
}

impl client::ResponseResult for ListVerificationsResponse {}


/// Represents a [local post](https://support.google.com/business/answer/7662907)
/// for a location.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations local posts create accounts](AccountLocationLocalPostCreateCall) (request|response)
/// * [locations local posts get accounts](AccountLocationLocalPostGetCall) (response)
/// * [locations local posts patch accounts](AccountLocationLocalPostPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocalPost {
    /// The type of alert the post is created for. This field is only applicable
    /// for posts of topic_type Alert, and behaves as a sub-type of Alerts.
    #[serde(rename="alertType")]
    
    pub alert_type: Option<LocalPostAlertTypeEnum>,
    /// The URL that users are sent to when clicking through the promotion. Ignored
    /// for topic type `OFFER`.
    #[serde(rename="callToAction")]
    
    pub call_to_action: Option<CallToAction>,
    /// Output only. Time of the creation of the post.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Event information. Required for topic types `EVENT` and `OFFER`.
    
    pub event: Option<LocalPostEvent>,
    /// The language of the local post.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// The media associated with the post. source_url is the only supported data
    /// field for a LocalPost MediaItem.
    
    pub media: Option<Vec<MediaItem>>,
    /// Output only. Google identifier for this local post in the form:
    ///  `accounts/{account_id}/locations/{location_id}/localPosts/{local_post_id}`
    
    pub name: Option<String>,
    /// Additional data for offer posts. This should only be set when the
    /// topic_type is OFFER.
    
    pub offer: Option<LocalPostOffer>,
    /// Output only. The link to the local post in Google search. This link can
    /// be used to share the post via social media, email, text, etc.
    #[serde(rename="searchUrl")]
    
    pub search_url: Option<String>,
    /// Output only. The state of the post, indicating what part of its lifecycle
    /// it is in.
    
    pub state: Option<LocalPostStateEnum>,
    /// Description/body of the local post.
    
    pub summary: Option<String>,
    /// Required. The topic type of the post: standard, event, offer, or alert.
    #[serde(rename="topicType")]
    
    pub topic_type: Option<LocalPostTopicTypeEnum>,
    /// Output only. Time of the last modification of the post made by the user.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for LocalPost {}
impl client::ResponseResult for LocalPost {}


/// All the information pertaining to an event featured in a local post.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocalPostEvent {
    /// Event start and end date/time.
    
    pub schedule: Option<TimeInterval>,
    /// Name of the event.
    
    pub title: Option<String>,
}

impl client::Part for LocalPostEvent {}


/// All the metrics requested for a Local Post.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocalPostMetrics {
    /// no description provided
    #[serde(rename="localPostName")]
    
    pub local_post_name: Option<String>,
    /// A list of values for the requested metrics.
    #[serde(rename="metricValues")]
    
    pub metric_values: Option<Vec<MetricValue>>,
}

impl client::Part for LocalPostMetrics {}


/// Specific fields for offer posts.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocalPostOffer {
    /// Optional. Offer code that is usable in store or online.
    #[serde(rename="couponCode")]
    
    pub coupon_code: Option<String>,
    /// Optional. Online link to redeem offer.
    #[serde(rename="redeemOnlineUrl")]
    
    pub redeem_online_url: Option<String>,
    /// Optional. Offer terms and conditions.
    #[serde(rename="termsConditions")]
    
    pub terms_conditions: Option<String>,
}

impl client::Part for LocalPostOffer {}


/// A location.
/// See the \[help center article\]
/// (https://support.google.com/business/answer/3038177) for a detailed
/// description of these fields, or the [category
/// endpoint](https://developers.google.com/my-business/reference/rest/v4/categories) for a list of valid
/// business categories.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations create accounts](AccountLocationCreateCall) (request|response)
/// * [locations get accounts](AccountLocationGetCall) (response)
/// * [locations patch accounts](AccountLocationPatchCall) (request|response)
/// * [locations transfer accounts](AccountLocationTransferCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    /// Additional information that is surfaced in AdWords.
    #[serde(rename="adWordsLocationExtensions")]
    
    pub ad_words_location_extensions: Option<AdWordsLocationExtensions>,
    /// Additional categories to describe your business.
    /// Categories help your customers find accurate, specific results for services
    /// they're interested in. To keep your business information accurate
    /// and live, make sure that you use as few categories as possible to describe
    /// your overall core business.
    /// Choose categories that are as specific as possible, but representative of
    /// your main business.
    #[serde(rename="additionalCategories")]
    
    pub additional_categories: Option<Vec<Category>>,
    /// Up to two phone numbers (mobile or landline, no fax) at which your business
    /// can be called, in addition to your primary phone number.
    #[serde(rename="additionalPhones")]
    
    pub additional_phones: Option<Vec<String>>,
    /// A precise, accurate address to describe your business location.
    /// PO boxes or mailboxes located at remote locations are not acceptable.
    /// At this time, you can specify a maximum of five `address_lines` values in
    /// the address.
    
    pub address: Option<PostalAddress>,
    /// Attributes for this location.
    
    pub attributes: Option<Vec<Attribute>>,
    /// A collection of free-form strings to allow you to tag your business. These
    /// labels are NOT user facing; only you can see them.
    /// Limited to 255 characters (per label).
    
    pub labels: Option<Vec<String>>,
    /// The language of the location. Set during creation and not updateable.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// User-provided latitude and longitude.
    /// When creating a location, this field is ignored if the provided
    /// address geocodes successfully.
    /// This field is only returned on get requests if the user-provided
    /// `latlng`
    /// value was accepted during create, or the `latlng` value was updated through
    /// the Google My Business website.
    /// This field cannot be updated.
    
    pub latlng: Option<LatLng>,
    /// A collection of keys that link this business to
    /// other Google properties, such as a Google+ page and Google Maps Places.
    #[serde(rename="locationKey")]
    
    pub location_key: Option<LocationKey>,
    /// Location name should reflect your business's real-world name, as used
    /// consistently on your storefront, website, and stationery, and as known to
    /// customers.
    /// Any additional information, when relevant, can be included in other
    /// fields of the resource (for example, `Address`, `Categories`).
    /// Don't add unnecessary information to your name (for example, prefer
    /// "Google" over "Google Inc. - Mountain View Corporate Headquarters"). Don't
    /// include marketing taglines, store codes, special characters, hours or
    /// closed/open status, phone numbers, website URLs, service/product
    /// information, location/address or directions, or containment information
    /// (for example, "Chase ATM in Duane Reade").
    #[serde(rename="locationName")]
    
    pub location_name: Option<String>,
    /// Output only. A set of booleans that reflects the [state of a
    /// location.](https://support.google.com/business/answer/3480862)
    #[serde(rename="locationState")]
    
    pub location_state: Option<LocationState>,
    /// Output Only. Additional non-user-editable information.
    
    pub metadata: Option<Metadata>,
    /// Google identifier for this location in the form:
    ///   `accounts/{account_id}/locations/{location_id}`
    /// 
    /// In the context of matches, this field will not be populated.
    
    pub name: Option<String>,
    /// A flag that indicates whether the location is currently open for
    /// business.
    #[serde(rename="openInfo")]
    
    pub open_info: Option<OpenInfo>,
    /// Price list information for this location.
    #[serde(rename="priceLists")]
    
    pub price_lists: Option<Vec<PriceList>>,
    /// Category that best describes the core business this location engages in.
    #[serde(rename="primaryCategory")]
    
    pub primary_category: Option<Category>,
    /// A phone number that connects to your individual business location
    /// as directly as possible. Use a local phone number instead of a central,
    /// call center helpline number whenever possible.
    #[serde(rename="primaryPhone")]
    
    pub primary_phone: Option<String>,
    /// Describes your business in your own voice and shares with users the unique
    /// story of your business and offerings.
    
    pub profile: Option<Profile>,
    /// Operating hours for the business.
    #[serde(rename="regularHours")]
    
    pub regular_hours: Option<BusinessHours>,
    /// All locations and chain related to this one.
    #[serde(rename="relationshipData")]
    
    pub relationship_data: Option<RelationshipData>,
    /// Service area businesses provide their service at the customer's location.
    /// If this business is a service area business, this field describes the
    /// area(s) serviced by the business.
    #[serde(rename="serviceArea")]
    
    pub service_area: Option<ServiceAreaBusiness>,
    /// Special hours for the business. This typically includes holiday hours,
    /// and other times outside of regular operating hours.
    /// These override regular business hours.
    #[serde(rename="specialHours")]
    
    pub special_hours: Option<SpecialHours>,
    /// External identifier for this location, which must be unique inside a given
    /// account. This is a means of associating the location with your own records.
    #[serde(rename="storeCode")]
    
    pub store_code: Option<String>,
    /// A URL for this business. If possible, use a URL that represents this
    /// individual business location instead of a generic website/URL that
    /// represents all locations, or the brand.
    #[serde(rename="websiteUrl")]
    
    pub website_url: Option<String>,
}

impl client::RequestValue for Location {}
impl client::ResponseResult for Location {}


/// How the media item is associated with its location.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocationAssociation {
    /// The category that this location photo belongs to.
    
    pub category: Option<LocationAssociationCategoryEnum>,
    /// The ID of a price list item that this location photo is associated
    /// with.
    #[serde(rename="priceListItemId")]
    
    pub price_list_item_id: Option<String>,
}

impl client::Part for LocationAssociation {}


/// A location indexed with the regions that people usually come from.
/// This is captured by counting how many driving-direction requests to
/// this location are from each region.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocationDrivingDirectionMetrics {
    /// The location resource name this metric value belongs to.
    #[serde(rename="locationName")]
    
    pub location_name: Option<String>,
    /// Time zone (IANA timezone IDs, for example, 'Europe/London') of the
    /// location.
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<String>,
    /// Driving-direction requests by source region. By convention, these
    /// are sorted by count with at most 10 results.
    #[serde(rename="topDirectionSources")]
    
    pub top_direction_sources: Option<Vec<TopDirectionSources>>,
}

impl client::Part for LocationDrivingDirectionMetrics {}


/// Alternate/surrogate key references for a location.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocationKey {
    /// Output only. A value of true indicates that an unset place ID is
    /// deliberate, which is different from no association being made yet.
    #[serde(rename="explicitNoPlaceId")]
    
    pub explicit_no_place_id: Option<bool>,
    /// If this location has been verified and is connected to/appears on Google
    /// Maps, this field is populated with the place ID for the location.
    /// This ID can be used in various Places APIs.
    /// 
    /// If this location is unverified, this field may be populated if the location
    /// has been associated with a place that appears on Google Maps.
    /// 
    /// This field can be set during Create calls, but not for Update.
    /// 
    /// The additional `explicit_no_place_id` bool qualifies whether an unset
    /// place ID is deliberate or not.
    #[serde(rename="placeId")]
    
    pub place_id: Option<String>,
    /// Output only. If this location has a Google+ page associated with it, this
    /// is populated with the Google+ page ID for this location.
    #[serde(rename="plusPageId")]
    
    pub plus_page_id: Option<String>,
    /// Output only. The `request_id` used to create this location. May be empty if
    /// this location was created outside of the GMB API or Google My Business
    /// Locations.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::Part for LocationKey {}


/// A series of Metrics and BreakdownMetrics associated with a Location over
/// some time range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocationMetrics {
    /// The location resource name these values belong to.
    #[serde(rename="locationName")]
    
    pub location_name: Option<String>,
    /// A list of values for the requested metrics.
    #[serde(rename="metricValues")]
    
    pub metric_values: Option<Vec<MetricValue>>,
    /// IANA timezone for the location.
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<String>,
}

impl client::Part for LocationMetrics {}


/// Represents a review with location information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocationReview {
    /// Location resource name.
    
    pub name: Option<String>,
    /// A review for the location.
    
    pub review: Option<Review>,
}

impl client::Part for LocationReview {}


/// Contains a set of booleans that reflect the [state of a
/// Location.](https://support.google.com/business/answer/3480862)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocationState {
    /// Output only. Indicates whether the location can be deleted using the Google
    /// My Business API.
    #[serde(rename="canDelete")]
    
    pub can_delete: Option<bool>,
    /// Output only. Indicates whether the location can be updated.
    #[serde(rename="canUpdate")]
    
    pub can_update: Option<bool>,
    /// Output only. Indicates whether any of this Location's properties are in the
    /// edit pending state.
    #[serde(rename="hasPendingEdits")]
    
    pub has_pending_edits: Option<bool>,
    /// Output only. Indicates whether the location has pending verification
    /// requests.
    #[serde(rename="hasPendingVerification")]
    
    pub has_pending_verification: Option<bool>,
    /// Output only. Indicates whether the location is disabled.
    #[serde(rename="isDisabled")]
    
    pub is_disabled: Option<bool>,
    /// Output only. Indicates whether the location is disconnected from a place on
    /// Google Maps.
    #[serde(rename="isDisconnected")]
    
    pub is_disconnected: Option<bool>,
    /// Output only. Indicates whether the location is a duplicate of another
    /// location. For more information, see
    /// [metadata.duplicate](https://developers.google.com/my-business/reference/rest/v4/accounts.locations#Duplicate).
    #[serde(rename="isDuplicate")]
    
    pub is_duplicate: Option<bool>,
    /// Output only. Indicates whether the place ID associated with this location
    /// has updates.
    #[serde(rename="isGoogleUpdated")]
    
    pub is_google_updated: Option<bool>,
    /// Output only. Indicates whether
    /// [accounts.locations.localPosts](https://developers.google.com/my-business/reference/rest/v4/accounts.locations.localPosts)
    /// is disabled for this location.
    #[serde(rename="isLocalPostApiDisabled")]
    
    pub is_local_post_api_disabled: Option<bool>,
    /// Output only. Indicates whether the review of the location is pending.
    #[serde(rename="isPendingReview")]
    
    pub is_pending_review: Option<bool>,
    /// Output only. Indicates whether the location is published.
    #[serde(rename="isPublished")]
    
    pub is_published: Option<bool>,
    /// Output only. Indicates whether the location is suspended.
    /// Suspended locations are not visible to end users in Google products.
    /// If you believe this was a mistake, see the [help center article]
    /// (https://support.google.com/business/answer/4569145).
    #[serde(rename="isSuspended")]
    
    pub is_suspended: Option<bool>,
    /// Output only. Indicates whether the location is verified.
    #[serde(rename="isVerified")]
    
    pub is_verified: Option<bool>,
    /// Output only. Indicates whether the location requires reverification.
    #[serde(rename="needsReverification")]
    
    pub needs_reverification: Option<bool>,
}

impl client::Part for LocationState {}


/// Represents a possible match to a location.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MatchedLocation {
    /// Is this an exact match?
    #[serde(rename="isExactMatch")]
    
    pub is_exact_match: Option<bool>,
    /// The sparsely populated location that is a potential match. Unpopulated
    /// fields include, but may not be limited to:
    /// name (the matched location cannot be retrieved via `GetLocation` nor
    /// `BatchGetLocations`); `store_code`; `service_area` coverage area details;
    /// `labels`; `ad_words_location_extensions`; `photos`
    
    pub location: Option<Location>,
}

impl client::Part for MatchedLocation {}


/// Insights and statistics for the media item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MediaInsights {
    /// Output only. The number of times the media item has been viewed.
    #[serde(rename="viewCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub view_count: Option<i64>,
}

impl client::Part for MediaInsights {}


/// A single media item.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations media customers get accounts](AccountLocationMediaCustomerGetCall) (response)
/// * [locations media create accounts](AccountLocationMediaCreateCall) (request|response)
/// * [locations media get accounts](AccountLocationMediaGetCall) (response)
/// * [locations media patch accounts](AccountLocationMediaPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MediaItem {
    /// Output only. Attribution information for customer media items. You must
    /// display this attribution as provided to your users and must not delete or
    /// alter the attribution.
    
    pub attribution: Option<Attribution>,
    /// Output only. Creation time of this media item.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Input only. A reference to media item binary data as obtained by the
    /// `StartUploadMediaItemData` method.
    /// 
    /// When creating a media item, either <code>sourceUrl</code> or
    /// <code>dataRef</code> must be set.
    #[serde(rename="dataRef")]
    
    pub data_ref: Option<MediaItemDataRef>,
    /// Description for this media item. Descriptions cannot be modified through
    /// the My Business API, but can be set when creating a new media item that is
    /// not a cover photo.
    
    pub description: Option<String>,
    /// Output only. The dimensions (width and height) in pixels.
    
    pub dimensions: Option<Dimensions>,
    /// Output only. Google-hosted URL for this media item. This URL is not static
    /// since it may change over time. For video this will be a preview image with
    /// an overlaid play icon.
    #[serde(rename="googleUrl")]
    
    pub google_url: Option<String>,
    /// Output only. Statistics for this media item.
    
    pub insights: Option<MediaInsights>,
    /// Required when calling `CreatePhoto`. Describes how this media item is
    /// connected to its location. Must be either a category (for example,
    /// EXTERIOR) or the ID of a price list item.
    /// 
    /// This is required when adding new media to a location with
    /// `CreateMediaItem`. For other types of media, for example, photos on local
    /// posts, this will not be present.
    #[serde(rename="locationAssociation")]
    
    pub location_association: Option<LocationAssociation>,
    /// The format of this media item. Must be set when the media item is created,
    /// and is read-only on all other requests. Cannot be updated.
    #[serde(rename="mediaFormat")]
    
    pub media_format: Option<MediaItemMediaFormatEnum>,
    /// The resource name for this media item.
    /// `accounts/{account_id}/locations/{location_id}/media/{media_key}`
    
    pub name: Option<String>,
    /// A publicly accessible URL where the media item can be retrieved
    /// from.
    /// 
    /// When creating one of this or data_ref must be set to specify the
    /// source of the media item.
    /// 
    /// If `source_url` was used when creating a media item, it will be populated
    /// with that source URL when the media item is retrieved.
    /// 
    /// This field cannot be updated.
    #[serde(rename="sourceUrl")]
    
    pub source_url: Option<String>,
    /// Output only. Where provided, the URL of a thumbnail image for this media
    /// item.
    #[serde(rename="thumbnailUrl")]
    
    pub thumbnail_url: Option<String>,
}

impl client::RequestValue for MediaItem {}
impl client::ResponseResult for MediaItem {}


/// Reference to the photo binary data of a `MediaItem` uploaded through the My
/// Business API.
/// 
/// Create a data ref using StartUploadMediaItemData, and use this ref
/// when uploading bytes to \[UpdateMedia\] and subsequently calling
/// CreateMediaItem.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations media start upload accounts](AccountLocationMediaStartUploadCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MediaItemDataRef {
    /// The unique ID for this media item's binary data.
    /// Used to upload the photo data with [UpdateMedia] and when
    /// creating a new media item from those bytes with CreateMediaItem.
    /// 
    /// Example of uploading bytes:
    /// `curl -X POST -T{path_to_file}
    /// "http://mybusiness.googleapis.com/upload/v1/media/{resource_name}?upload_type=media"`
    /// 
    /// For CreateMediaItem
    /// calls, set this as the `MediaItem` `data_ref`.
    #[serde(rename="resourceName")]
    
    pub resource_name: Option<String>,
}

impl client::ResponseResult for MediaItemDataRef {}


/// Additional non-user-editable information about the location.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    /// Information about the location that this location duplicates. Only
    /// present when `location_state.is_duplicate` is true.
    
    pub duplicate: Option<Duplicate>,
    /// A link to the location on Maps.
    #[serde(rename="mapsUrl")]
    
    pub maps_url: Option<String>,
    /// A link to the page on Google Search where a customer can leave a review
    /// for the location.
    #[serde(rename="newReviewUrl")]
    
    pub new_review_url: Option<String>,
}

impl client::Part for Metadata {}


/// A request to return values for one metric and the options for how those
/// values should be returned.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricRequest {
    /// The requested metric.
    
    pub metric: Option<MetricRequestMetricEnum>,
    /// How the values should appear when returned.
    
    pub options: Option<Vec<MetricRequestOptionsEnum>>,
}

impl client::Part for MetricRequest {}


/// A value for a single Metric from a starting time.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricValue {
    /// Dimensional values for this metric.
    #[serde(rename="dimensionalValues")]
    
    pub dimensional_values: Option<Vec<DimensionalMetricValue>>,
    /// The metric for which the value applies.
    
    pub metric: Option<MetricValueMetricEnum>,
    /// The total aggregated value for this metric.
    /// Set for the AGGREGATED_TOTAL option.
    #[serde(rename="totalValue")]
    
    pub total_value: Option<DimensionalMetricValue>,
}

impl client::Part for MetricValue {}


/// Represents an amount of money with its currency type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Money {
    /// The 3-letter currency code defined in ISO 4217.
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// Number of nano (10^-9) units of the amount.
    /// The value must be between -999,999,999 and +999,999,999 inclusive.
    /// If `units` is positive, `nanos` must be positive or zero.
    /// If `units` is zero, `nanos` can be positive, zero, or negative.
    /// If `units` is negative, `nanos` must be negative or zero.
    /// For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
    
    pub nanos: Option<i32>,
    /// The whole units of the amount.
    /// For example if `currencyCode` is `"USD"`, then 1 unit is one US dollar.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub units: Option<i64>,
}

impl client::Part for Money {}


/// A Google Cloud Pub/Sub topic where notifications can be published when a
/// location is updated or has a new review. There will be only one notification
/// settings resource per-account.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get notifications accounts](AccountGetNotificationCall) (response)
/// * [update notifications accounts](AccountUpdateNotificationCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Notifications {
    /// Output only. The notifications resource name.
    
    pub name: Option<String>,
    /// The types of notifications that will be sent to the Cloud Pub/Sub topic.
    /// At least one must be specified. To stop receiving notifications entirely,
    /// use DeleteNotifications.
    #[serde(rename="notificationTypes")]
    
    pub notification_types: Option<Vec<NotificationNotificationTypesEnum>>,
    /// The Google Cloud Pub/Sub topic that will receive notifications when
    /// locations managed by this account are updated. If unset, no notifications
    /// will be posted.
    /// 
    /// The account mybusiness-api-pubsub@system.gserviceaccount.com must have at
    /// least Publish permissions on the Cloud Pub/Sub topic.
    #[serde(rename="topicName")]
    
    pub topic_name: Option<String>,
}

impl client::RequestValue for Notifications {}
impl client::ResponseResult for Notifications {}


/// Information related to the opening state of the business.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OpenInfo {
    /// Output only. Indicates whether this business is eligible for re-open.
    #[serde(rename="canReopen")]
    
    pub can_reopen: Option<bool>,
    /// The date on which the location first opened. If the exact day is not known,
    /// month and year only can be provided. The date must be in the past or be
    /// no more than one year in the future.
    #[serde(rename="openingDate")]
    
    pub opening_date: Option<Date>,
    /// Indicates whether or not the Location is currently open for business.
    /// All locations are open by default, unless updated to be closed.
    
    pub status: Option<OpenInfoStatusEnum>,
}

impl client::Part for OpenInfo {}


/// Additional Info stored for an organization.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrganizationInfo {
    /// The contact number for the organization.
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<String>,
    /// The postal address for the account.
    #[serde(rename="postalAddress")]
    
    pub postal_address: Option<PostalAddress>,
    /// The registered domain for the account.
    #[serde(rename="registeredDomain")]
    
    pub registered_domain: Option<String>,
}

impl client::Part for OrganizationInfo {}


/// Input for PHONE_CALL/SMS verification.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PhoneInput {
    /// The phone number that should be called or be sent SMS to. It must be one of
    /// the phone numbers in the eligible options.
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<String>,
}

impl client::Part for PhoneInput {}


/// Display Data for verifications through phone, e.g. phone call, sms.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PhoneVerificationData {
    /// Phone number that the PIN will be sent to.
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<String>,
}

impl client::Part for PhoneVerificationData {}


/// Defines an area that's represented by a place ID.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlaceInfo {
    /// The localized name of the place. For example, `Scottsdale, AZ`.
    
    pub name: Option<String>,
    /// The ID of the place. Must correspond to a [region.]
    /// (https://developers.google.com/places/web-service/supported_types#table3)
    #[serde(rename="placeId")]
    
    pub place_id: Option<String>,
}

impl client::Part for PlaceInfo {}


/// Defines the union of areas represented by a set of places.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Places {
    /// The areas represented by place IDs. Limited to a maximum of 20 places.
    #[serde(rename="placeInfos")]
    
    pub place_infos: Option<Vec<PlaceInfo>>,
}

impl client::Part for Places {}


/// A radius around a particular point (latitude/longitude).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PointRadius {
    /// The latitude/longitude that specifies the center of an area defined by the
    /// radius.
    
    pub latlng: Option<LatLng>,
    /// The distance in kilometers of the area around the point.
    #[serde(rename="radiusKm")]
    
    pub radius_km: Option<f32>,
}

impl client::Part for PointRadius {}


/// Represents a postal address, e.g. for postal delivery or payments addresses.
/// Given a postal address, a postal service can deliver items to a premise, P.O.
/// Box or similar.
/// It is not intended to model geographical locations (roads, towns,
/// mountains).
/// 
/// In typical usage an address would be created via user input or from importing
/// existing data, depending on the type of process.
/// 
/// Advice on address input / editing:
///  - Use an i18n-ready address widget such as
///    https://github.com/google/libaddressinput)
/// - Users should not be presented with UI elements for input or editing of
///   fields outside countries where that field is used.
/// 
/// For more guidance on how to use this schema, please see:
/// https://support.google.com/business/answer/6397478
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PostalAddress {
    /// Unstructured address lines describing the lower levels of an address.
    /// 
    /// Because values in address_lines do not have type information and may
    /// sometimes contain multiple values in a single field (e.g.
    /// "Austin, TX"), it is important that the line order is clear. The order of
    /// address lines should be "envelope order" for the country/region of the
    /// address. In places where this can vary (e.g. Japan), address_language is
    /// used to make it explicit (e.g. "ja" for large-to-small ordering and
    /// "ja-Latn" or "en" for small-to-large). This way, the most specific line of
    /// an address can be selected based on the language.
    /// 
    /// The minimum permitted structural representation of an address consists
    /// of a region_code with all remaining information placed in the
    /// address_lines. It would be possible to format such an address very
    /// approximately without geocoding, but no semantic reasoning could be
    /// made about any of the address components until it was at least
    /// partially resolved.
    /// 
    /// Creating an address only containing a region_code and address_lines, and
    /// then geocoding is the recommended way to handle completely unstructured
    /// addresses (as opposed to guessing which parts of the address should be
    /// localities or administrative areas).
    #[serde(rename="addressLines")]
    
    pub address_lines: Option<Vec<String>>,
    /// Optional. Highest administrative subdivision which is used for postal
    /// addresses of a country or region.
    /// For example, this can be a state, a province, an oblast, or a prefecture.
    /// Specifically, for Spain this is the province and not the autonomous
    /// community (e.g. "Barcelona" and not "Catalonia").
    /// Many countries don't use an administrative area in postal addresses. E.g.
    /// in Switzerland this should be left unpopulated.
    #[serde(rename="administrativeArea")]
    
    pub administrative_area: Option<String>,
    /// Optional. BCP-47 language code of the contents of this address (if
    /// known). This is often the UI language of the input form or is expected
    /// to match one of the languages used in the address' country/region, or their
    /// transliterated equivalents.
    /// This can affect formatting in certain countries, but is not critical
    /// to the correctness of the data and will never affect any validation or
    /// other non-formatting related operations.
    /// 
    /// If this value is not known, it should be omitted (rather than specifying a
    /// possibly incorrect default).
    /// 
    /// Examples: "zh-Hant", "ja", "ja-Latn", "en".
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Optional. Generally refers to the city/town portion of the address.
    /// Examples: US city, IT comune, UK post town.
    /// In regions of the world where localities are not well defined or do not fit
    /// into this structure well, leave locality empty and use address_lines.
    
    pub locality: Option<String>,
    /// Optional. The name of the organization at the address.
    
    pub organization: Option<String>,
    /// Optional. Postal code of the address. Not all countries use or require
    /// postal codes to be present, but where they are used, they may trigger
    /// additional validation with other parts of the address (e.g. state/zip
    /// validation in the U.S.A.).
    #[serde(rename="postalCode")]
    
    pub postal_code: Option<String>,
    /// Optional. The recipient at the address.
    /// This field may, under certain circumstances, contain multiline information.
    /// For example, it might contain "care of" information.
    
    pub recipients: Option<Vec<String>>,
    /// Required. CLDR region code of the country/region of the address. This
    /// is never inferred and it is up to the user to ensure the value is
    /// correct. See http://cldr.unicode.org/ and
    /// http://www.unicode.org/cldr/charts/30/supplemental/territory_information.html
    /// for details. Example: "CH" for Switzerland.
    #[serde(rename="regionCode")]
    
    pub region_code: Option<String>,
    /// The schema revision of the `PostalAddress`. This must be set to 0, which is
    /// the latest revision.
    /// 
    /// All new revisions **must** be backward compatible with old revisions.
    
    pub revision: Option<i32>,
    /// Optional. Additional, country-specific, sorting code. This is not used
    /// in most regions. Where it is used, the value is either a string like
    /// "CEDEX", optionally followed by a number (e.g. "CEDEX 7"), or just a number
    /// alone, representing the "sector code" (Jamaica), "delivery area indicator"
    /// (Malawi) or "post office indicator" (e.g. Côte d'Ivoire).
    #[serde(rename="sortingCode")]
    
    pub sorting_code: Option<String>,
    /// Optional. Sublocality of the address.
    /// For example, this can be neighborhoods, boroughs, districts.
    
    pub sublocality: Option<String>,
}

impl client::Part for PostalAddress {}


/// A list of item price information. Price lists are structured as one or more
/// price lists, each containing one or more sections with one or more items.
/// For example, food price lists may represent breakfast/lunch/dinner menus,
/// with sections for burgers/steak/seafood.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PriceList {
    /// Required. Language-tagged labels for the price list.
    
    pub labels: Option<Vec<Label>>,
    /// Required. ID for the price list. Price list, section, and item IDs cannot
    /// be duplicated within this Location.
    #[serde(rename="priceListId")]
    
    pub price_list_id: Option<String>,
    /// Required. Sections for this price list. Each price list must contain at
    /// least one section.
    
    pub sections: Option<Vec<Section>>,
    /// Optional source URL of where the price list was retrieved from. For
    /// example, this could be the URL of the page that was automatically scraped
    /// to populate the menu information.
    #[serde(rename="sourceUrl")]
    
    pub source_url: Option<String>,
}

impl client::Part for PriceList {}


/// All information pertaining to the location's profile.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Profile {
    /// Description of the location in your own voice, not editable by anyone else.
    
    pub description: Option<String>,
}

impl client::Part for Profile {}


/// Represents a single question and some of its answers.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations questions create accounts](AccountLocationQuestionCreateCall) (request|response)
/// * [locations questions patch accounts](AccountLocationQuestionPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Question {
    /// Output only. The author of the question.
    
    pub author: Option<Author>,
    /// Output only. The timestamp for when the question was written.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The unique name for the question.
    /// accounts/*/locations/*/questions/*
    
    pub name: Option<String>,
    /// The text of the question. It should contain at least two words and the
    /// total length should be greater than or equal to 10 characters. The maximum
    /// length is 4096 characters.
    
    pub text: Option<String>,
    /// Output only. A list of answers to the question, sorted by upvotes. This
    /// may not be a complete list of answers depending on the request parameters
    /// (answers_per_question)
    #[serde(rename="topAnswers")]
    
    pub top_answers: Option<Vec<Answer>>,
    /// Output only. The total number of answers posted for this question.
    #[serde(rename="totalAnswerCount")]
    
    pub total_answer_count: Option<i32>,
    /// Output only. The timestamp for when the question was last modified.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The number of upvotes for the question.
    #[serde(rename="upvoteCount")]
    
    pub upvote_count: Option<i32>,
}

impl client::RequestValue for Question {}
impl client::ResponseResult for Question {}


/// A region with its associated request count.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RegionCount {
    /// Number of driving-direction requests from this region.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
    /// Human-readable label for the region.
    
    pub label: Option<String>,
    /// Center of region.
    
    pub latlng: Option<LatLng>,
}

impl client::Part for RegionCount {}


/// Information of all parent and children locations related to this one.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RelationshipData {
    /// The resource name of the Chain that this location is member of.
    /// How to find Chain ID
    #[serde(rename="parentChain")]
    
    pub parent_chain: Option<String>,
}

impl client::Part for RelationshipData {}


/// Values for an attribute with a `value_type` of REPEATED_ENUM. This consists
/// of two lists of value IDs: those that are set (true) and those that are unset
/// (false). Values absent are considered unknown. At least one value must be
/// specified.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RepeatedEnumAttributeValue {
    /// Enum values that are set.
    #[serde(rename="setValues")]
    
    pub set_values: Option<Vec<String>>,
    /// Enum values that are unset.
    #[serde(rename="unsetValues")]
    
    pub unset_values: Option<Vec<String>>,
}

impl client::Part for RepeatedEnumAttributeValue {}


/// Request message for reporting a GoogleLocation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [report google locations](GoogleLocationReportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportGoogleLocationRequest {
    /// Optional. The resource name of the location group that this Google Location
    /// is being reported for, in the format `accounts/{account_id}`.
    #[serde(rename="locationGroupName")]
    
    pub location_group_name: Option<String>,
    /// The reason for which the user is reporting this location when the issue
    /// is with the location itself.
    #[serde(rename="reportReasonBadLocation")]
    
    pub report_reason_bad_location: Option<ReportGoogleLocationRequestReportReasonBadLocationEnum>,
    /// The reason for which the user is reporting this location when the issue
    /// is with the recommendation. This report is useful if the location has
    /// been recommended to the GMB account.
    #[serde(rename="reportReasonBadRecommendation")]
    
    pub report_reason_bad_recommendation: Option<ReportGoogleLocationRequestReportReasonBadRecommendationEnum>,
    /// Optional. A text entry for elaborating on the reason for which the user is
    /// reporting this location. The maximum length is 512 characters.
    #[serde(rename="reportReasonElaboration")]
    
    pub report_reason_elaboration: Option<String>,
    /// Optional. The BCP 47 code of language used in `report_reason_elaboration`.
    #[serde(rename="reportReasonLanguageCode")]
    
    pub report_reason_language_code: Option<String>,
}

impl client::RequestValue for ReportGoogleLocationRequest {}


/// Request message for Insights.ReportLocalPostInsights
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations local posts report insights accounts](AccountLocationLocalPostReportInsightCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportLocalPostInsightsRequest {
    /// A request to include basic metric insights in the report. This request
    /// applies to all posts requested.
    #[serde(rename="basicRequest")]
    
    pub basic_request: Option<BasicMetricsRequest>,
    /// Required. The list of posts for which to fetch insights data. All posts
    /// have to belong to the location whose name is specified in the `name` field.
    #[serde(rename="localPostNames")]
    
    pub local_post_names: Option<Vec<String>>,
}

impl client::RequestValue for ReportLocalPostInsightsRequest {}


/// Response message for Insights.ReportLocalPostInsights
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations local posts report insights accounts](AccountLocationLocalPostReportInsightCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportLocalPostInsightsResponse {
    /// One entry per requested post corresponding to this location.
    #[serde(rename="localPostMetrics")]
    
    pub local_post_metrics: Option<Vec<LocalPostMetrics>>,
    /// no description provided
    
    pub name: Option<String>,
    /// Time zone (IANA timezone IDs, eg, 'Europe/London') of the location.
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<String>,
}

impl client::ResponseResult for ReportLocalPostInsightsResponse {}


/// Request message for Insights.ReportLocationInsights.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations report insights accounts](AccountLocationReportInsightCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportLocationInsightsRequest {
    /// A request to include basic metric insights in the report.
    #[serde(rename="basicRequest")]
    
    pub basic_request: Option<BasicMetricsRequest>,
    /// A request to include insights about driving-direction requests
    /// in the report.
    #[serde(rename="drivingDirectionsRequest")]
    
    pub driving_directions_request: Option<DrivingDirectionMetricsRequest>,
    /// A collection of locations to fetch insights for, specified by their names.
    #[serde(rename="locationNames")]
    
    pub location_names: Option<Vec<String>>,
}

impl client::RequestValue for ReportLocationInsightsRequest {}


/// Response message for `Insights.ReportLocationInsights`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations report insights accounts](AccountLocationReportInsightCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportLocationInsightsResponse {
    /// A collection of values for driving direction-related metrics.
    #[serde(rename="locationDrivingDirectionMetrics")]
    
    pub location_driving_direction_metrics: Option<Vec<LocationDrivingDirectionMetrics>>,
    /// A collection of metric values by location.
    #[serde(rename="locationMetrics")]
    
    pub location_metrics: Option<Vec<LocationMetrics>>,
}

impl client::ResponseResult for ReportLocationInsightsResponse {}


/// Output only. Represents a review for a location.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations reviews get accounts](AccountLocationReviewGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Review {
    /// The body of the review as plain text with markups.
    
    pub comment: Option<String>,
    /// The timestamp for when the review was written.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The resource name. For Review it is of the form
    /// `accounts/{account_id}/locations/{location_id}/reviews/{review_id}`
    
    pub name: Option<String>,
    /// The encrypted unique identifier.
    #[serde(rename="reviewId")]
    
    pub review_id: Option<String>,
    /// The owner/manager of this location's reply to this review.
    #[serde(rename="reviewReply")]
    
    pub review_reply: Option<ReviewReply>,
    /// The author of the review.
    
    pub reviewer: Option<Reviewer>,
    /// The star rating of the review.
    #[serde(rename="starRating")]
    
    pub star_rating: Option<ReviewStarRatingEnum>,
    /// The timestamp for when the review was last modified.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for Review {}


/// Represents the location owner/manager’s reply to a review.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations reviews update reply accounts](AccountLocationReviewUpdateReplyCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReviewReply {
    /// The body of the reply as plain text with markups.
    /// The maximum length is 4096 bytes.
    
    pub comment: Option<String>,
    /// Output only. The timestamp for when the reply was last modified.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for ReviewReply {}
impl client::ResponseResult for ReviewReply {}


/// Represents the author of the review.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Reviewer {
    /// The name of the reviewer.
    /// Only populated with the reviewer's real name if `is_anonymous` is false.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Indicates whether the reviewer has opted to remain anonymous.
    #[serde(rename="isAnonymous")]
    
    pub is_anonymous: Option<bool>,
    /// The profile photo link of the reviewer.
    /// Only populated if `is_anonymous` is false.
    #[serde(rename="profilePhotoUrl")]
    
    pub profile_photo_url: Option<String>,
}

impl client::Part for Reviewer {}


/// Response message for Locations.SearchChains.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search chains](ChainSearchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchChainsResponse {
    /// Chains that match the queried chain_display_name in SearchChainsRequest.
    /// If there are no matches, this field will be empty.
    /// Results are listed in order of relevance.
    
    pub chains: Option<Vec<Chain>>,
}

impl client::ResponseResult for SearchChainsResponse {}


/// Request message for GoogleLocations.SearchGoogleLocations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search google locations](GoogleLocationSearchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchGoogleLocationsRequest {
    /// Location to search for. If provided, will find locations which match the
    /// provided location details.
    
    pub location: Option<Location>,
    /// Text query to search for. The search results from a query string will be
    /// less accurate than if providing an exact location, but can provide more
    /// inexact matches.
    
    pub query: Option<String>,
    /// The number of matches to return. The default value is 3, with a maximum
    /// of 10. Note that latency may increase if more are requested. There is no
    /// pagination.
    #[serde(rename="resultCount")]
    
    pub result_count: Option<i32>,
}

impl client::RequestValue for SearchGoogleLocationsRequest {}


/// Response message for GoogleLocations.SearchGoogleLocations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search google locations](GoogleLocationSearchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchGoogleLocationsResponse {
    /// A collection of GoogleLocations that are potential matches to the specified
    /// request, listed in order from most to least accuracy.
    #[serde(rename="googleLocations")]
    
    pub google_locations: Option<Vec<GoogleLocation>>,
}

impl client::ResponseResult for SearchGoogleLocationsResponse {}


/// A section of the price list containing one or more items.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Section {
    /// Items that are contained within this section of the price list.
    
    pub items: Option<Vec<Item>>,
    /// Required. Language-tagged labels for the section. We recommend that
    /// section names and descriptions be 140 characters or less. At least one
    /// set of labels is required.
    
    pub labels: Option<Vec<Label>>,
    /// Required. ID for the section. Price list, section, and item IDs cannot be
    /// duplicated within this Location.
    #[serde(rename="sectionId")]
    
    pub section_id: Option<String>,
    /// Optional. Type of the current price list section. Default value is FOOD.
    #[serde(rename="sectionType")]
    
    pub section_type: Option<SectionSectionTypeEnum>,
}

impl client::Part for Section {}


/// Service area businesses provide their service at the customer's location (for
/// example, a locksmith or plumber).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceAreaBusiness {
    /// Indicates the type of the service area
    /// business.
    #[serde(rename="businessType")]
    
    pub business_type: Option<ServiceAreaBusinesBusinessTypeEnum>,
    /// The area that this business serves defined through a set of places.
    
    pub places: Option<Places>,
    /// Output only. The area that this business serves centered around a point.
    
    pub radius: Option<PointRadius>,
}

impl client::Part for ServiceAreaBusiness {}


/// Additional data for service business verification.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceBusinessContext {
    /// The verification address of the location. It is used to either enable more
    /// verification options or send a postcard.
    
    pub address: Option<PostalAddress>,
}

impl client::Part for ServiceBusinessContext {}


/// Represents a single time period when a location’s operational hours differ
/// from its normal business hours.
/// A special hour period must represent a range of less than 24 hours.
/// The `open_time` and `start_date` must predate the `close_time` and
/// `end_date`. The `close_time` and `end_date` can extend to 11:59 a.m. on the
/// day after the specified `start_date`. For example, the following inputs are
/// valid:
/// 
/// ````text
/// start_date=2015-11-23, open_time=08:00, close_time=18:00
/// start_date=2015-11-23, end_date=2015-11-23, open_time=08:00,
/// close_time=18:00 start_date=2015-11-23, end_date=2015-11-24,
/// open_time=13:00, close_time=11:59
/// ````
/// 
/// The following inputs are not valid:
/// 
/// ````text
/// start_date=2015-11-23, open_time=13:00, close_time=11:59
/// start_date=2015-11-23, end_date=2015-11-24, open_time=13:00,
/// close_time=12:00 start_date=2015-11-23, end_date=2015-11-25,
/// open_time=08:00, close_time=18:00
/// ````
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpecialHourPeriod {
    /// The wall time on `end_date` when a location closes, expressed in
    /// 24hr ISO 8601 extended format. (hh:mm) Valid values are 00:00-24:00,
    /// where 24:00 represents midnight at the end of the specified day field.
    /// Must be specified if `is_closed` is false.
    #[serde(rename="closeTime")]
    
    pub close_time: Option<String>,
    /// The calendar date this special hour period ends on.
    /// If `end_date` field is not set, default to the date specified in
    /// `start_date`.
    /// If set, this field must be equal to or at most 1 day after `start_date`.
    #[serde(rename="endDate")]
    
    pub end_date: Option<Date>,
    /// If true, `end_date`, `open_time`, and `close_time` are ignored,
    /// and the date specified in `start_date` is treated as the location being
    /// closed for the entire day.
    #[serde(rename="isClosed")]
    
    pub is_closed: Option<bool>,
    /// The wall time on `start_date` when a location opens, expressed in
    /// 24hr ISO 8601 extended format. (hh:mm) Valid values are 00:00-24:00,
    /// where 24:00 represents midnight at the end of the specified day field.
    /// Must be specified if `is_closed` is false.
    #[serde(rename="openTime")]
    
    pub open_time: Option<String>,
    /// The calendar date this special hour period starts on.
    #[serde(rename="startDate")]
    
    pub start_date: Option<Date>,
}

impl client::Part for SpecialHourPeriod {}


/// Represents a set of time periods when a location's operational hours differ
/// from its normal business hours.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpecialHours {
    /// A list of exceptions to the business's regular hours.
    #[serde(rename="specialHourPeriods")]
    
    pub special_hour_periods: Option<Vec<SpecialHourPeriod>>,
}

impl client::Part for SpecialHours {}


/// Request message for Media.StartUploadMediaItemData.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations media start upload accounts](AccountLocationMediaStartUploadCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StartUploadMediaItemDataRequest { _never_set: Option<bool> }

impl client::RequestValue for StartUploadMediaItemDataRequest {}


/// Represents a target location for a pending invitation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TargetLocation {
    /// The address of the location to which the user is invited.
    #[serde(rename="locationAddress")]
    
    pub location_address: Option<String>,
    /// The name of the location to which the user is invited.
    #[serde(rename="locationName")]
    
    pub location_name: Option<String>,
}

impl client::Part for TargetLocation {}


/// The dimension for which data is divided over.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeDimension {
    /// The day of the week ("MONDAY" to "SUNDAY") this value corresponds to.
    /// Set for BREAKDOWN_DAY_OF_WEEK option.
    #[serde(rename="dayOfWeek")]
    
    pub day_of_week: Option<TimeDimensionDayOfWeekEnum>,
    /// The hour of the day (0 to 23) this value corresponds to.
    /// Set for BREAKDOWN_HOUR_OF_DAY option.
    #[serde(rename="timeOfDay")]
    
    pub time_of_day: Option<TimeOfDay>,
    /// The range of time this value covers.
    /// Set for AGGREGATED_TOTAL and AGGREGATED_DAILY options.
    #[serde(rename="timeRange")]
    
    pub time_range: Option<TimeRange>,
}

impl client::Part for TimeDimension {}


/// An interval of time, inclusive. It must contain all fields to be valid.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeInterval {
    /// The end date of this period.
    #[serde(rename="endDate")]
    
    pub end_date: Option<Date>,
    /// The end time of this period.
    #[serde(rename="endTime")]
    
    pub end_time: Option<TimeOfDay>,
    /// The start date of this period.
    #[serde(rename="startDate")]
    
    pub start_date: Option<Date>,
    /// The start time of this period.
    #[serde(rename="startTime")]
    
    pub start_time: Option<TimeOfDay>,
}

impl client::Part for TimeInterval {}


/// Represents a time of day. The date and time zone are either not significant
/// or are specified elsewhere. An API may choose to allow leap seconds. Related
/// types are google.type.Date and `google.protobuf.Timestamp`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeOfDay {
    /// Hours of day in 24 hour format. Should be from 0 to 23. An API may choose
    /// to allow the value "24:00:00" for scenarios like business closing time.
    
    pub hours: Option<i32>,
    /// Minutes of hour of day. Must be from 0 to 59.
    
    pub minutes: Option<i32>,
    /// Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999.
    
    pub nanos: Option<i32>,
    /// Seconds of minutes of the time. Must normally be from 0 to 59. An API may
    /// allow the value 60 if it allows leap-seconds.
    
    pub seconds: Option<i32>,
}

impl client::Part for TimeOfDay {}


/// Represents a span of time that the business is open, starting on the
/// specified open
/// day/time and closing on the specified close day/time.
/// The closing time must occur after the opening time, for example later in the
/// same day, or on a subsequent day.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimePeriod {
    /// Indicates the day of the week this period ends
    /// on.
    #[serde(rename="closeDay")]
    
    pub close_day: Option<TimePeriodCloseDayEnum>,
    /// Time in 24hr ISO 8601 extended format (hh:mm). Valid values are
    /// 00:00-24:00, where 24:00 represents midnight at the end of the specified
    /// day field.
    #[serde(rename="closeTime")]
    
    pub close_time: Option<String>,
    /// Indicates the day of the week this period starts
    /// on.
    #[serde(rename="openDay")]
    
    pub open_day: Option<TimePeriodOpenDayEnum>,
    /// Time in 24hr ISO 8601 extended format (hh:mm). Valid values are
    /// 00:00-24:00, where 24:00 represents midnight at the end of the specified
    /// day field.
    #[serde(rename="openTime")]
    
    pub open_time: Option<String>,
}

impl client::Part for TimePeriod {}


/// A range of time. Data will be pulled over the range as a half-open inverval
/// (that is, [start_time, end_time)).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeRange {
    /// Epoch timestamp for the end of the range (exclusive).
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Epoch timestamp for the start of the range (inclusive).
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for TimeRange {}


/// Top regions where driving-direction requests originated from.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TopDirectionSources {
    /// The number of days data is aggregated over.
    #[serde(rename="dayCount")]
    
    pub day_count: Option<i32>,
    /// Regions sorted in descending order by count.
    #[serde(rename="regionCounts")]
    
    pub region_counts: Option<Vec<RegionCount>>,
}

impl client::Part for TopDirectionSources {}


/// Request message for Locations.TransferLocation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations transfer accounts](AccountLocationTransferCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TransferLocationRequest {
    /// Name of the account resource to transfer the location to (for example,
    /// "accounts/8675309").
    #[serde(rename="toAccount")]
    
    pub to_account: Option<String>,
}

impl client::RequestValue for TransferLocationRequest {}


/// Request message for QuestionsAndAnswers.UpsertAnswer
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations questions answers upsert accounts](AccountLocationQuestionAnswerUpsertCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpsertAnswerRequest {
    /// The new answer.
    
    pub answer: Option<Answer>,
}

impl client::RequestValue for UpsertAnswerRequest {}


/// Values for an attribute with a `value_type` of URL.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UrlAttributeValue {
    /// The URL.
    
    pub url: Option<String>,
}

impl client::Part for UrlAttributeValue {}


/// A verification represents a verification attempt on a location.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Verification {
    /// The timestamp when the verification is requested.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The method of the verification.
    
    pub method: Option<VerificationMethodEnum>,
    /// Resource name of the verification.
    
    pub name: Option<String>,
    /// The state of the verification.
    
    pub state: Option<VerificationStateEnum>,
}

impl client::Part for Verification {}


/// The verification option represents how to verify the location (indicated by
/// verification method) and where the verification will be sent to (indicated by
/// display data).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VerificationOption {
    /// Set only if the method is MAIL.
    #[serde(rename="addressData")]
    
    pub address_data: Option<AddressVerificationData>,
    /// Set only if the method is EMAIL.
    #[serde(rename="emailData")]
    
    pub email_data: Option<EmailVerificationData>,
    /// Set only if the method is PHONE_CALL or SMS.
    #[serde(rename="phoneData")]
    
    pub phone_data: Option<PhoneVerificationData>,
    /// Method to verify the location.
    #[serde(rename="verificationMethod")]
    
    pub verification_method: Option<VerificationOptionVerificationMethodEnum>,
}

impl client::Part for VerificationOption {}


/// Token generated by a vetted partner.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [generate verification tokens](VerificationTokenGenerateCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VerificationToken {
    /// The token string.
    #[serde(rename="tokenString")]
    
    pub token_string: Option<String>,
}

impl client::Resource for VerificationToken {}


/// Request message for Verifications.VerifyLocation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations verify accounts](AccountLocationVerifyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VerifyLocationRequest {
    /// The input for ADDRESS method.
    #[serde(rename="addressInput")]
    
    pub address_input: Option<AddressInput>,
    /// Extra context information for the verification of service businesses.
    /// Required for the locations whose business type is CUSTOMER_LOCATION_ONLY.
    /// For ADDRESS verification, the address will be used to send out postcard.
    /// For other methods, it should be the same as the one that is passed to
    /// FetchVerificationOptions.
    /// INVALID_ARGUMENT will be thrown if it is set for other types of business
    /// locations.
    
    pub context: Option<ServiceBusinessContext>,
    /// The input for EMAIL method.
    #[serde(rename="emailInput")]
    
    pub email_input: Option<EmailInput>,
    /// The BCP 47 language code representing the language that is to be used for
    /// the verification process.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Verification method.
    
    pub method: Option<VerifyLocationRequestMethodEnum>,
    /// The input for PHONE_CALL/SMS method
    #[serde(rename="phoneInput")]
    
    pub phone_input: Option<PhoneInput>,
    /// The input for VETTED_PARTNER method. The input is not needed for a
    /// vetted account.
    #[serde(rename="vettedPartnerInput")]
    
    pub vetted_partner_input: Option<VettedPartnerInput>,
}

impl client::RequestValue for VerifyLocationRequest {}


/// Response message for Verifications.VerifyLocation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations verify accounts](AccountLocationVerifyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VerifyLocationResponse {
    /// The created verification request.
    
    pub verification: Option<Verification>,
}

impl client::ResponseResult for VerifyLocationResponse {}


/// Input for VETTED_PARTNER verification.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VettedPartnerInput {
    /// Token that is associated to the location.
    
    pub token: Option<VerificationToken>,
}

impl client::Part for VettedPartnerInput {}


