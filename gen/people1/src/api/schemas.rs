use super::*;
/// A person's physical address. May be a P.O. box or street address. All fields are optional.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Address {
    /// The city of the address.
    
    pub city: Option<String>,
    /// The country of the address.
    
    pub country: Option<String>,
    /// The [ISO 3166-1 alpha-2](http://www.iso.org/iso/country_codes.htm) country code of the address.
    #[serde(rename="countryCode")]
    
    pub country_code: Option<String>,
    /// The extended address of the address; for example, the apartment number.
    #[serde(rename="extendedAddress")]
    
    pub extended_address: Option<String>,
    /// Output only. The type of the address translated and formatted in the viewer's account locale or the `Accept-Language` HTTP header locale.
    #[serde(rename="formattedType")]
    
    pub formatted_type: Option<String>,
    /// The unstructured value of the address. If this is not set by the user it will be automatically constructed from structured values.
    #[serde(rename="formattedValue")]
    
    pub formatted_value: Option<String>,
    /// Metadata about the address.
    
    pub metadata: Option<FieldMetadata>,
    /// The P.O. box of the address.
    #[serde(rename="poBox")]
    
    pub po_box: Option<String>,
    /// The postal code of the address.
    #[serde(rename="postalCode")]
    
    pub postal_code: Option<String>,
    /// The region of the address; for example, the state or province.
    
    pub region: Option<String>,
    /// The street address.
    #[serde(rename="streetAddress")]
    
    pub street_address: Option<String>,
    /// The type of the address. The type can be custom or one of these predefined values: * `home` * `work` * `other`
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for Address {}


/// A person's age range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AgeRangeType {
    /// The age range.
    #[serde(rename="ageRange")]
    
    pub age_range: Option<AgeRangeTypeAgeRangeEnum>,
    /// Metadata about the age range.
    
    pub metadata: Option<FieldMetadata>,
}

impl client::Part for AgeRangeType {}


/// A request to create a batch of contacts.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch create contacts people](PersonBatchCreateContactCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchCreateContactsRequest {
    /// Required. The contact to create. Allows up to 200 contacts in a single request.
    
    pub contacts: Option<Vec<ContactToCreate>>,
    /// Required. A field mask to restrict which fields on each person are returned in the response. Multiple fields can be specified by separating them with commas. If read mask is left empty, the post-mutate-get is skipped and no data will be returned in the response. Valid values are: * addresses * ageRanges * biographies * birthdays * calendarUrls * clientData * coverPhotos * emailAddresses * events * externalIds * genders * imClients * interests * locales * locations * memberships * metadata * miscKeywords * names * nicknames * occupations * organizations * phoneNumbers * photos * relations * sipAddresses * skills * urls * userDefined
    #[serde(rename="readMask")]
    
    pub read_mask: Option<client::FieldMask>,
    /// Optional. A mask of what source types to return in the post mutate read. Defaults to READ_SOURCE_TYPE_CONTACT and READ_SOURCE_TYPE_PROFILE if not set.
    
    pub sources: Option<Vec<BatchCreateContactsRequestSourcesEnum>>,
}

impl client::RequestValue for BatchCreateContactsRequest {}


/// If not successful, returns BatchCreateContactsErrorDetails which contains a list of errors for each invalid contact. The response to a request to create a batch of contacts.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch create contacts people](PersonBatchCreateContactCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchCreateContactsResponse {
    /// The contacts that were created, unless the request `read_mask` is empty.
    #[serde(rename="createdPeople")]
    
    pub created_people: Option<Vec<PersonResponse>>,
}

impl client::ResponseResult for BatchCreateContactsResponse {}


/// A request to delete a batch of existing contacts.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch delete contacts people](PersonBatchDeleteContactCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchDeleteContactsRequest {
    /// Required. The resource names of the contact to delete. It's repeatable. Allows up to 500 resource names in a single request.
    #[serde(rename="resourceNames")]
    
    pub resource_names: Option<Vec<String>>,
}

impl client::RequestValue for BatchDeleteContactsRequest {}


/// The response to a batch get contact groups request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch get contact groups](ContactGroupBatchGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchGetContactGroupsResponse {
    /// The list of responses for each requested contact group resource.
    
    pub responses: Option<Vec<ContactGroupResponse>>,
}

impl client::ResponseResult for BatchGetContactGroupsResponse {}


/// A request to update a batch of contacts.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch update contacts people](PersonBatchUpdateContactCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchUpdateContactsRequest {
    /// Required. A map of resource names to the person data to be updated. Allows up to 200 contacts in a single request.
    
    pub contacts: Option<HashMap<String, Person>>,
    /// Required. A field mask to restrict which fields on each person are returned. Multiple fields can be specified by separating them with commas. If read mask is left empty, the post-mutate-get is skipped and no data will be returned in the response. Valid values are: * addresses * ageRanges * biographies * birthdays * calendarUrls * clientData * coverPhotos * emailAddresses * events * externalIds * genders * imClients * interests * locales * locations * memberships * metadata * miscKeywords * names * nicknames * occupations * organizations * phoneNumbers * photos * relations * sipAddresses * skills * urls * userDefined
    #[serde(rename="readMask")]
    
    pub read_mask: Option<client::FieldMask>,
    /// Optional. A mask of what source types to return. Defaults to READ_SOURCE_TYPE_CONTACT and READ_SOURCE_TYPE_PROFILE if not set.
    
    pub sources: Option<Vec<BatchUpdateContactsRequestSourcesEnum>>,
    /// Required. A field mask to restrict which fields on the person are updated. Multiple fields can be specified by separating them with commas. All specified fields will be replaced, or cleared if left empty for each person. Valid values are: * addresses * biographies * birthdays * calendarUrls * clientData * emailAddresses * events * externalIds * genders * imClients * interests * locales * locations * memberships * miscKeywords * names * nicknames * occupations * organizations * phoneNumbers * relations * sipAddresses * urls * userDefined
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for BatchUpdateContactsRequest {}


/// If not successful, returns BatchUpdateContactsErrorDetails, a list of errors corresponding to each contact. The response to a request to update a batch of contacts.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch update contacts people](PersonBatchUpdateContactCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchUpdateContactsResponse {
    /// A map of resource names to the contacts that were updated, unless the request `read_mask` is empty.
    #[serde(rename="updateResult")]
    
    pub update_result: Option<HashMap<String, PersonResponse>>,
}

impl client::ResponseResult for BatchUpdateContactsResponse {}


/// A person's short biography.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Biography {
    /// The content type of the biography.
    #[serde(rename="contentType")]
    
    pub content_type: Option<BiographyContentTypeEnum>,
    /// Metadata about the biography.
    
    pub metadata: Option<FieldMetadata>,
    /// The short biography.
    
    pub value: Option<String>,
}

impl client::Part for Biography {}


/// A person's birthday. At least one of the `date` and `text` fields are specified. The `date` and `text` fields typically represent the same date, but are not guaranteed to. Clients should always set the `date` field when mutating birthdays.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Birthday {
    /// The structured date of the birthday.
    
    pub date: Option<Date>,
    /// Metadata about the birthday.
    
    pub metadata: Option<FieldMetadata>,
    /// Prefer to use the `date` field if set. A free-form string representing the user's birthday. This value is not validated.
    
    pub text: Option<String>,
}

impl client::Part for Birthday {}


/// **DEPRECATED**: No data will be returned A person's bragging rights.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BraggingRights {
    /// Metadata about the bragging rights.
    
    pub metadata: Option<FieldMetadata>,
    /// The bragging rights; for example, `climbed mount everest`.
    
    pub value: Option<String>,
}

impl client::Part for BraggingRights {}


/// A person's calendar URL.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CalendarUrl {
    /// Output only. The type of the calendar URL translated and formatted in the viewer's account locale or the `Accept-Language` HTTP header locale.
    #[serde(rename="formattedType")]
    
    pub formatted_type: Option<String>,
    /// Metadata about the calendar URL.
    
    pub metadata: Option<FieldMetadata>,
    /// The type of the calendar URL. The type can be custom or one of these predefined values: * `home` * `freeBusy` * `work`
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The calendar URL.
    
    pub url: Option<String>,
}

impl client::Part for CalendarUrl {}


/// Arbitrary client data that is populated by clients. Duplicate keys and values are allowed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClientData {
    /// The client specified key of the client data.
    
    pub key: Option<String>,
    /// Metadata about the client data.
    
    pub metadata: Option<FieldMetadata>,
    /// The client specified value of the client data.
    
    pub value: Option<String>,
}

impl client::Part for ClientData {}


/// A contact group.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [members modify contact groups](ContactGroupMemberModifyCall) (none)
/// * [batch get contact groups](ContactGroupBatchGetCall) (none)
/// * [create contact groups](ContactGroupCreateCall) (response)
/// * [delete contact groups](ContactGroupDeleteCall) (none)
/// * [get contact groups](ContactGroupGetCall) (response)
/// * [list contact groups](ContactGroupListCall) (none)
/// * [update contact groups](ContactGroupUpdateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContactGroup {
    /// The group's client data.
    #[serde(rename="clientData")]
    
    pub client_data: Option<Vec<GroupClientData>>,
    /// The [HTTP entity tag](https://en.wikipedia.org/wiki/HTTP_ETag) of the resource. Used for web cache validation.
    
    pub etag: Option<String>,
    /// Output only. The name translated and formatted in the viewer's account locale or the `Accept-Language` HTTP header locale for system groups names. Group names set by the owner are the same as name.
    #[serde(rename="formattedName")]
    
    pub formatted_name: Option<String>,
    /// Output only. The contact group type.
    #[serde(rename="groupType")]
    
    pub group_type: Option<ContactGroupGroupTypeEnum>,
    /// Output only. The total number of contacts in the group irrespective of max members in specified in the request.
    #[serde(rename="memberCount")]
    
    pub member_count: Option<i32>,
    /// Output only. The list of contact person resource names that are members of the contact group. The field is only populated for GET requests and will only return as many members as `maxMembers` in the get request.
    #[serde(rename="memberResourceNames")]
    
    pub member_resource_names: Option<Vec<String>>,
    /// Output only. Metadata about the contact group.
    
    pub metadata: Option<ContactGroupMetadata>,
    /// The contact group name set by the group owner or a system provided name for system groups. For [`contactGroups.create`](https://developers.google.com/people/api/rest/v1/contactGroups/create) or [`contactGroups.update`](https://developers.google.com/people/api/rest/v1/contactGroups/update) the name must be unique to the users contact groups. Attempting to create a group with a duplicate name will return a HTTP 409 error.
    
    pub name: Option<String>,
    /// The resource name for the contact group, assigned by the server. An ASCII string, in the form of `contactGroups/{contact_group_id}`.
    #[serde(rename="resourceName")]
    
    pub resource_name: Option<String>,
}

impl client::Resource for ContactGroup {}
impl client::ResponseResult for ContactGroup {}


/// A Google contact group membership.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContactGroupMembership {
    /// Output only. The contact group ID for the contact group membership.
    #[serde(rename="contactGroupId")]
    
    pub contact_group_id: Option<String>,
    /// The resource name for the contact group, assigned by the server. An ASCII string, in the form of `contactGroups/{contact_group_id}`. Only contact_group_resource_name can be used for modifying memberships. Any contact group membership can be removed, but only user group or "myContacts" or "starred" system groups memberships can be added. A contact must always have at least one contact group membership.
    #[serde(rename="contactGroupResourceName")]
    
    pub contact_group_resource_name: Option<String>,
}

impl client::Part for ContactGroupMembership {}


/// The metadata about a contact group.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContactGroupMetadata {
    /// Output only. True if the contact group resource has been deleted. Populated only for [`ListContactGroups`](https://developers.google.com/people/api/rest/v1/contactgroups/list) requests that include a sync token.
    
    pub deleted: Option<bool>,
    /// Output only. The time the group was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for ContactGroupMetadata {}


/// The response for a specific contact group.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContactGroupResponse {
    /// The contact group.
    #[serde(rename="contactGroup")]
    
    pub contact_group: Option<ContactGroup>,
    /// The original requested resource name.
    #[serde(rename="requestedResourceName")]
    
    pub requested_resource_name: Option<String>,
    /// The status of the response.
    
    pub status: Option<Status>,
}

impl client::Part for ContactGroupResponse {}


/// A wrapper that contains the person data to populate a newly created source.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContactToCreate {
    /// Required. The person data to populate a newly created source.
    #[serde(rename="contactPerson")]
    
    pub contact_person: Option<Person>,
}

impl client::Part for ContactToCreate {}


/// A request to copy an “Other contact” to my contacts group.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [copy other contact to my contacts group other contacts](OtherContactCopyOtherContactToMyContactsGroupCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CopyOtherContactToMyContactsGroupRequest {
    /// Required. A field mask to restrict which fields are copied into the new contact. Valid values are: * emailAddresses * names * phoneNumbers
    #[serde(rename="copyMask")]
    
    pub copy_mask: Option<client::FieldMask>,
    /// Optional. A field mask to restrict which fields on the person are returned. Multiple fields can be specified by separating them with commas. Defaults to the copy mask with metadata and membership fields if not set. Valid values are: * addresses * ageRanges * biographies * birthdays * calendarUrls * clientData * coverPhotos * emailAddresses * events * externalIds * genders * imClients * interests * locales * locations * memberships * metadata * miscKeywords * names * nicknames * occupations * organizations * phoneNumbers * photos * relations * sipAddresses * skills * urls * userDefined
    #[serde(rename="readMask")]
    
    pub read_mask: Option<client::FieldMask>,
    /// Optional. A mask of what source types to return. Defaults to READ_SOURCE_TYPE_CONTACT and READ_SOURCE_TYPE_PROFILE if not set.
    
    pub sources: Option<Vec<CopyOtherContactToMyContactsGroupRequestSourcesEnum>>,
}

impl client::RequestValue for CopyOtherContactToMyContactsGroupRequest {}


/// A person's cover photo. A large image shown on the person's profile page that represents who they are or what they care about.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CoverPhoto {
    /// True if the cover photo is the default cover photo; false if the cover photo is a user-provided cover photo.
    
    pub default: Option<bool>,
    /// Metadata about the cover photo.
    
    pub metadata: Option<FieldMetadata>,
    /// The URL of the cover photo.
    
    pub url: Option<String>,
}

impl client::Part for CoverPhoto {}


/// A request to create a new contact group.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create contact groups](ContactGroupCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateContactGroupRequest {
    /// Required. The contact group to create.
    #[serde(rename="contactGroup")]
    
    pub contact_group: Option<ContactGroup>,
    /// Optional. A field mask to restrict which fields on the group are returned. Defaults to `metadata`, `groupType`, and `name` if not set or set to empty. Valid fields are: * clientData * groupType * metadata * name
    #[serde(rename="readGroupFields")]
    
    pub read_group_fields: Option<client::FieldMask>,
}

impl client::RequestValue for CreateContactGroupRequest {}


/// Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values. * A month and day, with a zero year (for example, an anniversary). * A year on its own, with a zero month and a zero day. * A year and month, with a zero day (for example, a credit card expiration date). Related types: * google.type.TimeOfDay * google.type.DateTime * google.protobuf.Timestamp
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Date {
    /// Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant.
    
    pub day: Option<i32>,
    /// Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day.
    
    pub month: Option<i32>,
    /// Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year.
    
    pub year: Option<i32>,
}

impl client::Part for Date {}


/// The response for deleting a contact’s photo.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete contact photo people](PersonDeleteContactPhotoCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteContactPhotoResponse {
    /// The updated person, if person_fields is set in the DeleteContactPhotoRequest; otherwise this will be unset.
    
    pub person: Option<Person>,
}

impl client::ResponseResult for DeleteContactPhotoResponse {}


/// A Google Workspace Domain membership.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DomainMembership {
    /// True if the person is in the viewer's Google Workspace domain.
    #[serde(rename="inViewerDomain")]
    
    pub in_viewer_domain: Option<bool>,
}

impl client::Part for DomainMembership {}


/// A person's email address.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EmailAddress {
    /// The display name of the email.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. The type of the email address translated and formatted in the viewer's account locale or the `Accept-Language` HTTP header locale.
    #[serde(rename="formattedType")]
    
    pub formatted_type: Option<String>,
    /// Metadata about the email address.
    
    pub metadata: Option<FieldMetadata>,
    /// The type of the email address. The type can be custom or one of these predefined values: * `home` * `work` * `other`
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The email address.
    
    pub value: Option<String>,
}

impl client::Part for EmailAddress {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete contact groups](ContactGroupDeleteCall) (response)
/// * [batch delete contacts people](PersonBatchDeleteContactCall) (response)
/// * [delete contact people](PersonDeleteContactCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// An event related to the person.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    /// The date of the event.
    
    pub date: Option<Date>,
    /// Output only. The type of the event translated and formatted in the viewer's account locale or the `Accept-Language` HTTP header locale.
    #[serde(rename="formattedType")]
    
    pub formatted_type: Option<String>,
    /// Metadata about the event.
    
    pub metadata: Option<FieldMetadata>,
    /// The type of the event. The type can be custom or one of these predefined values: * `anniversary` * `other`
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for Event {}


/// An identifier from an external entity related to the person.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExternalId {
    /// Output only. The type of the event translated and formatted in the viewer's account locale or the `Accept-Language` HTTP header locale.
    #[serde(rename="formattedType")]
    
    pub formatted_type: Option<String>,
    /// Metadata about the external ID.
    
    pub metadata: Option<FieldMetadata>,
    /// The type of the external ID. The type can be custom or one of these predefined values: * `account` * `customer` * `loginId` * `network` * `organization`
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The value of the external ID.
    
    pub value: Option<String>,
}

impl client::Part for ExternalId {}


/// Metadata about a field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FieldMetadata {
    /// Output only. True if the field is the primary field for all sources in the person. Each person will have at most one field with `primary` set to true.
    
    pub primary: Option<bool>,
    /// The source of the field.
    
    pub source: Option<Source>,
    /// True if the field is the primary field for the source. Each source must have at most one field with `source_primary` set to true.
    #[serde(rename="sourcePrimary")]
    
    pub source_primary: Option<bool>,
    /// Output only. True if the field is verified; false if the field is unverified. A verified field is typically a name, email address, phone number, or website that has been confirmed to be owned by the person.
    
    pub verified: Option<bool>,
}

impl client::Part for FieldMetadata {}


/// The name that should be used to sort the person in a list.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileAs {
    /// Metadata about the file-as.
    
    pub metadata: Option<FieldMetadata>,
    /// The file-as value
    
    pub value: Option<String>,
}

impl client::Part for FileAs {}


/// A person's gender.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Gender {
    /// Free form text field for pronouns that should be used to address the person. Common values are: * `he`/`him` * `she`/`her` * `they`/`them`
    #[serde(rename="addressMeAs")]
    
    pub address_me_as: Option<String>,
    /// Output only. The value of the gender translated and formatted in the viewer's account locale or the `Accept-Language` HTTP header locale. Unspecified or custom value are not localized.
    #[serde(rename="formattedValue")]
    
    pub formatted_value: Option<String>,
    /// Metadata about the gender.
    
    pub metadata: Option<FieldMetadata>,
    /// The gender for the person. The gender can be custom or one of these predefined values: * `male` * `female` * `unspecified`
    
    pub value: Option<String>,
}

impl client::Part for Gender {}


/// The response to a get request for a list of people by resource name.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get batch get people](PersonGetBatchGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetPeopleResponse {
    /// The response for each requested resource name.
    
    pub responses: Option<Vec<PersonResponse>>,
}

impl client::ResponseResult for GetPeopleResponse {}


/// Arbitrary client data that is populated by clients. Duplicate keys and values are allowed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GroupClientData {
    /// The client specified key of the client data.
    
    pub key: Option<String>,
    /// The client specified value of the client data.
    
    pub value: Option<String>,
}

impl client::Part for GroupClientData {}


/// A person's instant messaging client.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImClient {
    /// Output only. The protocol of the IM client formatted in the viewer's account locale or the `Accept-Language` HTTP header locale.
    #[serde(rename="formattedProtocol")]
    
    pub formatted_protocol: Option<String>,
    /// Output only. The type of the IM client translated and formatted in the viewer's account locale or the `Accept-Language` HTTP header locale.
    #[serde(rename="formattedType")]
    
    pub formatted_type: Option<String>,
    /// Metadata about the IM client.
    
    pub metadata: Option<FieldMetadata>,
    /// The protocol of the IM client. The protocol can be custom or one of these predefined values: * `aim` * `msn` * `yahoo` * `skype` * `qq` * `googleTalk` * `icq` * `jabber` * `netMeeting`
    
    pub protocol: Option<String>,
    /// The type of the IM client. The type can be custom or one of these predefined values: * `home` * `work` * `other`
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The user name used in the IM client.
    
    pub username: Option<String>,
}

impl client::Part for ImClient {}


/// One of the person's interests.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Interest {
    /// Metadata about the interest.
    
    pub metadata: Option<FieldMetadata>,
    /// The interest; for example, `stargazing`.
    
    pub value: Option<String>,
}

impl client::Part for Interest {}


/// The response to a request for the authenticated user’s connections.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [connections list people](PersonConnectionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListConnectionsResponse {
    /// The list of people that the requestor is connected to.
    
    pub connections: Option<Vec<Person>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A token, which can be sent as `sync_token` to retrieve changes since the last request. Request must set `request_sync_token` to return the sync token. When the response is paginated, only the last page will contain `nextSyncToken`.
    #[serde(rename="nextSyncToken")]
    
    pub next_sync_token: Option<String>,
    /// The total number of items in the list without pagination.
    #[serde(rename="totalItems")]
    
    pub total_items: Option<i32>,
    /// **DEPRECATED** (Please use totalItems) The total number of people in the list without pagination.
    #[serde(rename="totalPeople")]
    
    pub total_people: Option<i32>,
}

impl client::ResponseResult for ListConnectionsResponse {}


/// The response to a list contact groups request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list contact groups](ContactGroupListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListContactGroupsResponse {
    /// The list of contact groups. Members of the contact groups are not populated.
    #[serde(rename="contactGroups")]
    
    pub contact_groups: Option<Vec<ContactGroup>>,
    /// The token that can be used to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The token that can be used to retrieve changes since the last request.
    #[serde(rename="nextSyncToken")]
    
    pub next_sync_token: Option<String>,
    /// The total number of items in the list without pagination.
    #[serde(rename="totalItems")]
    
    pub total_items: Option<i32>,
}

impl client::ResponseResult for ListContactGroupsResponse {}


/// The response to a request for the authenticated user’s domain directory.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list directory people people](PersonListDirectoryPersonCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListDirectoryPeopleResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A token, which can be sent as `sync_token` to retrieve changes since the last request. Request must set `request_sync_token` to return the sync token.
    #[serde(rename="nextSyncToken")]
    
    pub next_sync_token: Option<String>,
    /// The list of people in the domain directory.
    
    pub people: Option<Vec<Person>>,
}

impl client::ResponseResult for ListDirectoryPeopleResponse {}


/// The response to a request for the authenticated user’s “Other contacts”.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list other contacts](OtherContactListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListOtherContactsResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A token, which can be sent as `sync_token` to retrieve changes since the last request. Request must set `request_sync_token` to return the sync token.
    #[serde(rename="nextSyncToken")]
    
    pub next_sync_token: Option<String>,
    /// The list of "Other contacts" returned as Person resources. "Other contacts" support a limited subset of fields. See ListOtherContactsRequest.request_mask for more detailed information.
    #[serde(rename="otherContacts")]
    
    pub other_contacts: Option<Vec<Person>>,
    /// The total number of other contacts in the list without pagination.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
}

impl client::ResponseResult for ListOtherContactsResponse {}


/// A person's locale preference.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Locale {
    /// Metadata about the locale.
    
    pub metadata: Option<FieldMetadata>,
    /// The well-formed [IETF BCP 47](https://tools.ietf.org/html/bcp47) language tag representing the locale.
    
    pub value: Option<String>,
}

impl client::Part for Locale {}


/// A person's location.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    /// The building identifier.
    #[serde(rename="buildingId")]
    
    pub building_id: Option<String>,
    /// Whether the location is the current location.
    
    pub current: Option<bool>,
    /// The individual desk location.
    #[serde(rename="deskCode")]
    
    pub desk_code: Option<String>,
    /// The floor name or number.
    
    pub floor: Option<String>,
    /// The floor section in `floor_name`.
    #[serde(rename="floorSection")]
    
    pub floor_section: Option<String>,
    /// Metadata about the location.
    
    pub metadata: Option<FieldMetadata>,
    /// The type of the location. The type can be custom or one of these predefined values: * `desk` * `grewUp`
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The free-form value of the location.
    
    pub value: Option<String>,
}

impl client::Part for Location {}


/// A person's membership in a group. Only contact group memberships can be modified.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Membership {
    /// The contact group membership.
    #[serde(rename="contactGroupMembership")]
    
    pub contact_group_membership: Option<ContactGroupMembership>,
    /// Output only. The domain membership.
    #[serde(rename="domainMembership")]
    
    pub domain_membership: Option<DomainMembership>,
    /// Metadata about the membership.
    
    pub metadata: Option<FieldMetadata>,
}

impl client::Part for Membership {}


/// A person's miscellaneous keyword.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MiscKeyword {
    /// Output only. The type of the miscellaneous keyword translated and formatted in the viewer's account locale or the `Accept-Language` HTTP header locale.
    #[serde(rename="formattedType")]
    
    pub formatted_type: Option<String>,
    /// Metadata about the miscellaneous keyword.
    
    pub metadata: Option<FieldMetadata>,
    /// The miscellaneous keyword type.
    #[serde(rename="type")]
    
    pub type_: Option<MiscKeywordTypeEnum>,
    /// The value of the miscellaneous keyword.
    
    pub value: Option<String>,
}

impl client::Part for MiscKeyword {}


/// A request to modify an existing contact group’s members. Contacts can be removed from any group but they can only be added to a user group or “myContacts” or “starred” system groups.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [members modify contact groups](ContactGroupMemberModifyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ModifyContactGroupMembersRequest {
    /// Optional. The resource names of the contact people to add in the form of `people/{person_id}`. The total number of resource names in `resource_names_to_add` and `resource_names_to_remove` must be less than or equal to 1000.
    #[serde(rename="resourceNamesToAdd")]
    
    pub resource_names_to_add: Option<Vec<String>>,
    /// Optional. The resource names of the contact people to remove in the form of `people/{person_id}`. The total number of resource names in `resource_names_to_add` and `resource_names_to_remove` must be less than or equal to 1000.
    #[serde(rename="resourceNamesToRemove")]
    
    pub resource_names_to_remove: Option<Vec<String>>,
}

impl client::RequestValue for ModifyContactGroupMembersRequest {}


/// The response to a modify contact group members request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [members modify contact groups](ContactGroupMemberModifyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ModifyContactGroupMembersResponse {
    /// The contact people resource names that cannot be removed from their last contact group.
    #[serde(rename="canNotRemoveLastContactGroupResourceNames")]
    
    pub can_not_remove_last_contact_group_resource_names: Option<Vec<String>>,
    /// The contact people resource names that were not found.
    #[serde(rename="notFoundResourceNames")]
    
    pub not_found_resource_names: Option<Vec<String>>,
}

impl client::ResponseResult for ModifyContactGroupMembersResponse {}


/// A person's name. If the name is a mononym, the family name is empty.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Name {
    /// Output only. The display name formatted according to the locale specified by the viewer's account or the `Accept-Language` HTTP header.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. The display name with the last name first formatted according to the locale specified by the viewer's account or the `Accept-Language` HTTP header.
    #[serde(rename="displayNameLastFirst")]
    
    pub display_name_last_first: Option<String>,
    /// The family name.
    #[serde(rename="familyName")]
    
    pub family_name: Option<String>,
    /// The given name.
    #[serde(rename="givenName")]
    
    pub given_name: Option<String>,
    /// The honorific prefixes, such as `Mrs.` or `Dr.`
    #[serde(rename="honorificPrefix")]
    
    pub honorific_prefix: Option<String>,
    /// The honorific suffixes, such as `Jr.`
    #[serde(rename="honorificSuffix")]
    
    pub honorific_suffix: Option<String>,
    /// Metadata about the name.
    
    pub metadata: Option<FieldMetadata>,
    /// The middle name(s).
    #[serde(rename="middleName")]
    
    pub middle_name: Option<String>,
    /// The family name spelled as it sounds.
    #[serde(rename="phoneticFamilyName")]
    
    pub phonetic_family_name: Option<String>,
    /// The full name spelled as it sounds.
    #[serde(rename="phoneticFullName")]
    
    pub phonetic_full_name: Option<String>,
    /// The given name spelled as it sounds.
    #[serde(rename="phoneticGivenName")]
    
    pub phonetic_given_name: Option<String>,
    /// The honorific prefixes spelled as they sound.
    #[serde(rename="phoneticHonorificPrefix")]
    
    pub phonetic_honorific_prefix: Option<String>,
    /// The honorific suffixes spelled as they sound.
    #[serde(rename="phoneticHonorificSuffix")]
    
    pub phonetic_honorific_suffix: Option<String>,
    /// The middle name(s) spelled as they sound.
    #[serde(rename="phoneticMiddleName")]
    
    pub phonetic_middle_name: Option<String>,
    /// The free form name value.
    #[serde(rename="unstructuredName")]
    
    pub unstructured_name: Option<String>,
}

impl client::Part for Name {}


/// A person's nickname.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Nickname {
    /// Metadata about the nickname.
    
    pub metadata: Option<FieldMetadata>,
    /// The type of the nickname.
    #[serde(rename="type")]
    
    pub type_: Option<NicknameTypeEnum>,
    /// The nickname.
    
    pub value: Option<String>,
}

impl client::Part for Nickname {}


/// A person's occupation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Occupation {
    /// Metadata about the occupation.
    
    pub metadata: Option<FieldMetadata>,
    /// The occupation; for example, `carpenter`.
    
    pub value: Option<String>,
}

impl client::Part for Occupation {}


/// A person's past or current organization. Overlapping date ranges are permitted.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Organization {
    /// The person's cost center at the organization.
    #[serde(rename="costCenter")]
    
    pub cost_center: Option<String>,
    /// True if the organization is the person's current organization; false if the organization is a past organization.
    
    pub current: Option<bool>,
    /// The person's department at the organization.
    
    pub department: Option<String>,
    /// The domain name associated with the organization; for example, `google.com`.
    
    pub domain: Option<String>,
    /// The end date when the person left the organization.
    #[serde(rename="endDate")]
    
    pub end_date: Option<Date>,
    /// Output only. The type of the organization translated and formatted in the viewer's account locale or the `Accept-Language` HTTP header locale.
    #[serde(rename="formattedType")]
    
    pub formatted_type: Option<String>,
    /// The person's full-time equivalent millipercent within the organization (100000 = 100%).
    #[serde(rename="fullTimeEquivalentMillipercent")]
    
    pub full_time_equivalent_millipercent: Option<i32>,
    /// The person's job description at the organization.
    #[serde(rename="jobDescription")]
    
    pub job_description: Option<String>,
    /// The location of the organization office the person works at.
    
    pub location: Option<String>,
    /// Metadata about the organization.
    
    pub metadata: Option<FieldMetadata>,
    /// The name of the organization.
    
    pub name: Option<String>,
    /// The phonetic name of the organization.
    #[serde(rename="phoneticName")]
    
    pub phonetic_name: Option<String>,
    /// The start date when the person joined the organization.
    #[serde(rename="startDate")]
    
    pub start_date: Option<Date>,
    /// The symbol associated with the organization; for example, a stock ticker symbol, abbreviation, or acronym.
    
    pub symbol: Option<String>,
    /// The person's job title at the organization.
    
    pub title: Option<String>,
    /// The type of the organization. The type can be custom or one of these predefined values: * `work` * `school`
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for Organization {}


/// Information about a person merged from various data sources such as the authenticated user’s contacts and profile data. Most fields can have multiple items. The items in a field have no guaranteed order, but each non-empty field is guaranteed to have exactly one field with `metadata.primary` set to true.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [copy other contact to my contacts group other contacts](OtherContactCopyOtherContactToMyContactsGroupCall) (response)
/// * [create contact people](PersonCreateContactCall) (request|response)
/// * [get people](PersonGetCall) (response)
/// * [update contact people](PersonUpdateContactCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Person {
    /// The person's street addresses.
    
    pub addresses: Option<Vec<Address>>,
    /// Output only. **DEPRECATED** (Please use `person.ageRanges` instead) The person's age range.
    #[serde(rename="ageRange")]
    
    pub age_range: Option<PersonAgeRangeEnum>,
    /// Output only. The person's age ranges.
    #[serde(rename="ageRanges")]
    
    pub age_ranges: Option<Vec<AgeRangeType>>,
    /// The person's biographies. This field is a singleton for contact sources.
    
    pub biographies: Option<Vec<Biography>>,
    /// The person's birthdays. This field is a singleton for contact sources.
    
    pub birthdays: Option<Vec<Birthday>>,
    /// **DEPRECATED**: No data will be returned The person's bragging rights.
    #[serde(rename="braggingRights")]
    
    pub bragging_rights: Option<Vec<BraggingRights>>,
    /// The person's calendar URLs.
    #[serde(rename="calendarUrls")]
    
    pub calendar_urls: Option<Vec<CalendarUrl>>,
    /// The person's client data.
    #[serde(rename="clientData")]
    
    pub client_data: Option<Vec<ClientData>>,
    /// Output only. The person's cover photos.
    #[serde(rename="coverPhotos")]
    
    pub cover_photos: Option<Vec<CoverPhoto>>,
    /// The person's email addresses. For `people.connections.list` and `otherContacts.list` the number of email addresses is limited to 100. If a Person has more email addresses the entire set can be obtained by calling GetPeople.
    #[serde(rename="emailAddresses")]
    
    pub email_addresses: Option<Vec<EmailAddress>>,
    /// The [HTTP entity tag](https://en.wikipedia.org/wiki/HTTP_ETag) of the resource. Used for web cache validation.
    
    pub etag: Option<String>,
    /// The person's events.
    
    pub events: Option<Vec<Event>>,
    /// The person's external IDs.
    #[serde(rename="externalIds")]
    
    pub external_ids: Option<Vec<ExternalId>>,
    /// The person's file-ases.
    #[serde(rename="fileAses")]
    
    pub file_ases: Option<Vec<FileAs>>,
    /// The person's genders. This field is a singleton for contact sources.
    
    pub genders: Option<Vec<Gender>>,
    /// The person's instant messaging clients.
    #[serde(rename="imClients")]
    
    pub im_clients: Option<Vec<ImClient>>,
    /// The person's interests.
    
    pub interests: Option<Vec<Interest>>,
    /// The person's locale preferences.
    
    pub locales: Option<Vec<Locale>>,
    /// The person's locations.
    
    pub locations: Option<Vec<Location>>,
    /// The person's group memberships.
    
    pub memberships: Option<Vec<Membership>>,
    /// Output only. Metadata about the person.
    
    pub metadata: Option<PersonMetadata>,
    /// The person's miscellaneous keywords.
    #[serde(rename="miscKeywords")]
    
    pub misc_keywords: Option<Vec<MiscKeyword>>,
    /// The person's names. This field is a singleton for contact sources.
    
    pub names: Option<Vec<Name>>,
    /// The person's nicknames.
    
    pub nicknames: Option<Vec<Nickname>>,
    /// The person's occupations.
    
    pub occupations: Option<Vec<Occupation>>,
    /// The person's past or current organizations.
    
    pub organizations: Option<Vec<Organization>>,
    /// The person's phone numbers. For `people.connections.list` and `otherContacts.list` the number of phone numbers is limited to 100. If a Person has more phone numbers the entire set can be obtained by calling GetPeople.
    #[serde(rename="phoneNumbers")]
    
    pub phone_numbers: Option<Vec<PhoneNumber>>,
    /// Output only. The person's photos.
    
    pub photos: Option<Vec<Photo>>,
    /// The person's relations.
    
    pub relations: Option<Vec<Relation>>,
    /// Output only. **DEPRECATED**: No data will be returned The person's relationship interests.
    #[serde(rename="relationshipInterests")]
    
    pub relationship_interests: Option<Vec<RelationshipInterest>>,
    /// Output only. **DEPRECATED**: No data will be returned The person's relationship statuses.
    #[serde(rename="relationshipStatuses")]
    
    pub relationship_statuses: Option<Vec<RelationshipStatus>>,
    /// **DEPRECATED**: (Please use `person.locations` instead) The person's residences.
    
    pub residences: Option<Vec<Residence>>,
    /// The resource name for the person, assigned by the server. An ASCII string in the form of `people/{person_id}`.
    #[serde(rename="resourceName")]
    
    pub resource_name: Option<String>,
    /// The person's SIP addresses.
    #[serde(rename="sipAddresses")]
    
    pub sip_addresses: Option<Vec<SipAddress>>,
    /// The person's skills.
    
    pub skills: Option<Vec<Skill>>,
    /// Output only. **DEPRECATED**: No data will be returned The person's taglines.
    
    pub taglines: Option<Vec<Tagline>>,
    /// The person's associated URLs.
    
    pub urls: Option<Vec<Url>>,
    /// The person's user defined data.
    #[serde(rename="userDefined")]
    
    pub user_defined: Option<Vec<UserDefined>>,
}

impl client::RequestValue for Person {}
impl client::ResponseResult for Person {}


/// The metadata about a person.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PersonMetadata {
    /// Output only. True if the person resource has been deleted. Populated only for `people.connections.list` and `otherContacts.list` sync requests.
    
    pub deleted: Option<bool>,
    /// Output only. Resource names of people linked to this resource.
    #[serde(rename="linkedPeopleResourceNames")]
    
    pub linked_people_resource_names: Option<Vec<String>>,
    /// Output only. **DEPRECATED** (Please use `person.metadata.sources.profileMetadata.objectType` instead) The type of the person object.
    #[serde(rename="objectType")]
    
    pub object_type: Option<PersonMetadataObjectTypeEnum>,
    /// Output only. Any former resource names this person has had. Populated only for `people.connections.list` requests that include a sync token. The resource name may change when adding or removing fields that link a contact and profile such as a verified email, verified phone number, or profile URL.
    #[serde(rename="previousResourceNames")]
    
    pub previous_resource_names: Option<Vec<String>>,
    /// The sources of data for the person.
    
    pub sources: Option<Vec<Source>>,
}

impl client::Part for PersonMetadata {}


/// The response for a single person
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PersonResponse {
    /// **DEPRECATED** (Please use status instead) [HTTP 1.1 status code] (http://www.w3.org/Protocols/rfc2616/rfc2616-sec10.html).
    #[serde(rename="httpStatusCode")]
    
    pub http_status_code: Option<i32>,
    /// The person.
    
    pub person: Option<Person>,
    /// The original requested resource name. May be different than the resource name on the returned person. The resource name can change when adding or removing fields that link a contact and profile such as a verified email, verified phone number, or a profile URL.
    #[serde(rename="requestedResourceName")]
    
    pub requested_resource_name: Option<String>,
    /// The status of the response.
    
    pub status: Option<Status>,
}

impl client::Part for PersonResponse {}


/// A person's phone number.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PhoneNumber {
    /// Output only. The canonicalized [ITU-T E.164](https://law.resource.org/pub/us/cfr/ibr/004/itu-t.E.164.1.2008.pdf) form of the phone number.
    #[serde(rename="canonicalForm")]
    
    pub canonical_form: Option<String>,
    /// Output only. The type of the phone number translated and formatted in the viewer's account locale or the `Accept-Language` HTTP header locale.
    #[serde(rename="formattedType")]
    
    pub formatted_type: Option<String>,
    /// Metadata about the phone number.
    
    pub metadata: Option<FieldMetadata>,
    /// The type of the phone number. The type can be custom or one of these predefined values: * `home` * `work` * `mobile` * `homeFax` * `workFax` * `otherFax` * `pager` * `workMobile` * `workPager` * `main` * `googleVoice` * `other`
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The phone number.
    
    pub value: Option<String>,
}

impl client::Part for PhoneNumber {}


/// A person's photo. A picture shown next to the person's name to help others recognize the person.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Photo {
    /// True if the photo is a default photo; false if the photo is a user-provided photo.
    
    pub default: Option<bool>,
    /// Metadata about the photo.
    
    pub metadata: Option<FieldMetadata>,
    /// The URL of the photo. You can change the desired size by appending a query parameter `sz={size}` at the end of the url, where {size} is the size in pixels. Example: https://lh3.googleusercontent.com/-T_wVWLlmg7w/AAAAAAAAAAI/AAAAAAAABa8/00gzXvDBYqw/s100/photo.jpg?sz=50
    
    pub url: Option<String>,
}

impl client::Part for Photo {}


/// The metadata about a profile.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProfileMetadata {
    /// Output only. The profile object type.
    #[serde(rename="objectType")]
    
    pub object_type: Option<ProfileMetadataObjectTypeEnum>,
    /// Output only. The user types.
    #[serde(rename="userTypes")]
    
    pub user_types: Option<Vec<ProfileMetadataUserTypesEnum>>,
}

impl client::Part for ProfileMetadata {}


/// A person's relation to another person.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Relation {
    /// Output only. The type of the relation translated and formatted in the viewer's account locale or the locale specified in the Accept-Language HTTP header.
    #[serde(rename="formattedType")]
    
    pub formatted_type: Option<String>,
    /// Metadata about the relation.
    
    pub metadata: Option<FieldMetadata>,
    /// The name of the other person this relation refers to.
    
    pub person: Option<String>,
    /// The person's relation to the other person. The type can be custom or one of these predefined values: * `spouse` * `child` * `mother` * `father` * `parent` * `brother` * `sister` * `friend` * `relative` * `domesticPartner` * `manager` * `assistant` * `referredBy` * `partner`
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for Relation {}


/// **DEPRECATED**: No data will be returned A person's relationship interest .
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RelationshipInterest {
    /// Output only. The value of the relationship interest translated and formatted in the viewer's account locale or the locale specified in the Accept-Language HTTP header.
    #[serde(rename="formattedValue")]
    
    pub formatted_value: Option<String>,
    /// Metadata about the relationship interest.
    
    pub metadata: Option<FieldMetadata>,
    /// The kind of relationship the person is looking for. The value can be custom or one of these predefined values: * `friend` * `date` * `relationship` * `networking`
    
    pub value: Option<String>,
}

impl client::Part for RelationshipInterest {}


/// **DEPRECATED**: No data will be returned A person's relationship status.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RelationshipStatus {
    /// Output only. The value of the relationship status translated and formatted in the viewer's account locale or the `Accept-Language` HTTP header locale.
    #[serde(rename="formattedValue")]
    
    pub formatted_value: Option<String>,
    /// Metadata about the relationship status.
    
    pub metadata: Option<FieldMetadata>,
    /// The relationship status. The value can be custom or one of these predefined values: * `single` * `inARelationship` * `engaged` * `married` * `itsComplicated` * `openRelationship` * `widowed` * `inDomesticPartnership` * `inCivilUnion`
    
    pub value: Option<String>,
}

impl client::Part for RelationshipStatus {}


/// **DEPRECATED**: Please use `person.locations` instead. A person's past or current residence.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Residence {
    /// True if the residence is the person's current residence; false if the residence is a past residence.
    
    pub current: Option<bool>,
    /// Metadata about the residence.
    
    pub metadata: Option<FieldMetadata>,
    /// The address of the residence.
    
    pub value: Option<String>,
}

impl client::Part for Residence {}


/// The response to a request for people in the authenticated user’s domain directory that match the specified query.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search directory people people](PersonSearchDirectoryPersonCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchDirectoryPeopleResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of people in the domain directory that match the query.
    
    pub people: Option<Vec<Person>>,
    /// The total number of items in the list without pagination.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
}

impl client::ResponseResult for SearchDirectoryPeopleResponse {}


/// The response to a search request for the authenticated user, given a query.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search other contacts](OtherContactSearchCall) (response)
/// * [search contacts people](PersonSearchContactCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    /// The results of the request.
    
    pub results: Option<Vec<SearchResult>>,
}

impl client::ResponseResult for SearchResponse {}


/// A result of a search query.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchResult {
    /// The matched Person.
    
    pub person: Option<Person>,
}

impl client::Part for SearchResult {}


/// A person's SIP address. Session Initial Protocol addresses are used for VoIP communications to make voice or video calls over the internet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SipAddress {
    /// Output only. The type of the SIP address translated and formatted in the viewer's account locale or the `Accept-Language` HTTP header locale.
    #[serde(rename="formattedType")]
    
    pub formatted_type: Option<String>,
    /// Metadata about the SIP address.
    
    pub metadata: Option<FieldMetadata>,
    /// The type of the SIP address. The type can be custom or or one of these predefined values: * `home` * `work` * `mobile` * `other`
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The SIP address in the [RFC 3261 19.1](https://tools.ietf.org/html/rfc3261#section-19.1) SIP URI format.
    
    pub value: Option<String>,
}

impl client::Part for SipAddress {}


/// A skill that the person has.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Skill {
    /// Metadata about the skill.
    
    pub metadata: Option<FieldMetadata>,
    /// The skill; for example, `underwater basket weaving`.
    
    pub value: Option<String>,
}

impl client::Part for Skill {}


/// The source of a field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Source {
    /// **Only populated in `person.metadata.sources`.** The [HTTP entity tag](https://en.wikipedia.org/wiki/HTTP_ETag) of the source. Used for web cache validation.
    
    pub etag: Option<String>,
    /// The unique identifier within the source type generated by the server.
    
    pub id: Option<String>,
    /// Output only. **Only populated in `person.metadata.sources`.** Metadata about a source of type PROFILE.
    #[serde(rename="profileMetadata")]
    
    pub profile_metadata: Option<ProfileMetadata>,
    /// The source type.
    #[serde(rename="type")]
    
    pub type_: Option<SourceTypeEnum>,
    /// Output only. **Only populated in `person.metadata.sources`.** Last update timestamp of this source.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for Source {}


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


/// **DEPRECATED**: No data will be returned A brief one-line description of the person.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Tagline {
    /// Metadata about the tagline.
    
    pub metadata: Option<FieldMetadata>,
    /// The tagline.
    
    pub value: Option<String>,
}

impl client::Part for Tagline {}


/// A request to update an existing user contact group. All updated fields will be replaced.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [update contact groups](ContactGroupUpdateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateContactGroupRequest {
    /// Required. The contact group to update.
    #[serde(rename="contactGroup")]
    
    pub contact_group: Option<ContactGroup>,
    /// Optional. A field mask to restrict which fields on the group are returned. Defaults to `metadata`, `groupType`, and `name` if not set or set to empty. Valid fields are: * clientData * groupType * memberCount * metadata * name
    #[serde(rename="readGroupFields")]
    
    pub read_group_fields: Option<client::FieldMask>,
    /// Optional. A field mask to restrict which fields on the group are updated. Multiple fields can be specified by separating them with commas. Defaults to `name` if not set or set to empty. Updated fields are replaced. Valid values are: * clientData * name
    #[serde(rename="updateGroupFields")]
    
    pub update_group_fields: Option<client::FieldMask>,
}

impl client::RequestValue for UpdateContactGroupRequest {}


/// A request to update an existing contact’s photo. All requests must have a valid photo format: JPEG or PNG.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [update contact photo people](PersonUpdateContactPhotoCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateContactPhotoRequest {
    /// Optional. A field mask to restrict which fields on the person are returned. Multiple fields can be specified by separating them with commas. Defaults to empty if not set, which will skip the post mutate get. Valid values are: * addresses * ageRanges * biographies * birthdays * calendarUrls * clientData * coverPhotos * emailAddresses * events * externalIds * genders * imClients * interests * locales * locations * memberships * metadata * miscKeywords * names * nicknames * occupations * organizations * phoneNumbers * photos * relations * sipAddresses * skills * urls * userDefined
    #[serde(rename="personFields")]
    
    pub person_fields: Option<client::FieldMask>,
    /// Required. Raw photo bytes
    #[serde(rename="photoBytes")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub photo_bytes: Option<Vec<u8>>,
    /// Optional. A mask of what source types to return. Defaults to READ_SOURCE_TYPE_CONTACT and READ_SOURCE_TYPE_PROFILE if not set.
    
    pub sources: Option<Vec<UpdateContactPhotoRequestSourcesEnum>>,
}

impl client::RequestValue for UpdateContactPhotoRequest {}


/// The response for updating a contact’s photo.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [update contact photo people](PersonUpdateContactPhotoCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateContactPhotoResponse {
    /// The updated person, if person_fields is set in the UpdateContactPhotoRequest; otherwise this will be unset.
    
    pub person: Option<Person>,
}

impl client::ResponseResult for UpdateContactPhotoResponse {}


/// A person's associated URLs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Url {
    /// Output only. The type of the URL translated and formatted in the viewer's account locale or the `Accept-Language` HTTP header locale.
    #[serde(rename="formattedType")]
    
    pub formatted_type: Option<String>,
    /// Metadata about the URL.
    
    pub metadata: Option<FieldMetadata>,
    /// The type of the URL. The type can be custom or one of these predefined values: * `home` * `work` * `blog` * `profile` * `homePage` * `ftp` * `reservations` * `appInstallPage`: website for a Currents application. * `other`
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The URL.
    
    pub value: Option<String>,
}

impl client::Part for Url {}


/// Arbitrary user data that is populated by the end users.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserDefined {
    /// The end user specified key of the user defined data.
    
    pub key: Option<String>,
    /// Metadata about the user defined data.
    
    pub metadata: Option<FieldMetadata>,
    /// The end user specified value of the user defined data.
    
    pub value: Option<String>,
}

impl client::Part for UserDefined {}


