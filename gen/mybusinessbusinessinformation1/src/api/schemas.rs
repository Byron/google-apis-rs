use super::*;
/// Additional information that is surfaced in AdWords.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdWordsLocationExtensions {
    /// Required. An alternate phone number to display on AdWords location extensions instead of the location's primary phone number.
    #[serde(rename="adPhone")]
    
    pub ad_phone: Option<String>,
}

impl client::Part for AdWordsLocationExtensions {}


/// Request message for Locations.AssociateLocationRequest.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [associate locations](LocationAssociateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AssociateLocationRequest {
    /// The association to establish. If not set, it indicates no match.
    #[serde(rename="placeId")]
    
    pub place_id: Option<String>,
}

impl client::RequestValue for AssociateLocationRequest {}


/// A location attribute. Attributes provide additional information about a location. The attributes that can be set on a location may vary based on the properties of that location (for example, category). Available attributes are determined by Google and may be added and removed without API changes.
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
    /// Required. The resource name for this attribute.
    
    pub name: Option<String>,
    /// When the attribute value type is REPEATED_ENUM, this contains the attribute value, and the other values fields must be empty.
    #[serde(rename="repeatedEnumValue")]
    
    pub repeated_enum_value: Option<RepeatedEnumAttributeValue>,
    /// When the attribute value type is URL, this field contains the value(s) for this attribute, and the other values fields must be empty.
    #[serde(rename="uriValues")]
    
    pub uri_values: Option<Vec<UriAttributeValue>>,
    /// Output only. The type of value that this attribute contains. This should be used to determine how to interpret the value.
    #[serde(rename="valueType")]
    
    pub value_type: Option<String>,
    /// The values for this attribute. The type of the values supplied must match that expected for that attribute. This is a repeated field where multiple attribute values may be provided. Attribute types only support one value.
    
    pub values: Option<Vec<json::Value>>,
}

impl client::Resource for Attribute {}


/// Metadata for an attribute. Contains display information for the attribute, including a localized name and a heading for grouping related attributes together.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AttributeMetadata {
    /// If true, the attribute is deprecated and should no longer be used. If deprecated, updating this attribute will not result in an error, but updates will not be saved. At some point after being deprecated, the attribute will be removed entirely and it will become an error.
    
    pub deprecated: Option<bool>,
    /// The localized display name for the attribute, if available; otherwise, the English display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The localized display name of the group that contains this attribute, if available; otherwise, the English group name. Related attributes are collected into a group and should be displayed together under the heading given here.
    #[serde(rename="groupDisplayName")]
    
    pub group_display_name: Option<String>,
    /// The unique identifier for the attribute.
    
    pub parent: Option<String>,
    /// If true, the attribute supports multiple values. If false, only a single value should be provided.
    
    pub repeatable: Option<bool>,
    /// For some types of attributes (for example, enums), a list of supported values and corresponding display names for those values is provided.
    #[serde(rename="valueMetadata")]
    
    pub value_metadata: Option<Vec<AttributeValueMetadata>>,
    /// The value type for the attribute. Values set and retrieved should be expected to be of this type.
    #[serde(rename="valueType")]
    
    pub value_type: Option<String>,
}

impl client::Part for AttributeMetadata {}


/// Metadata for supported attribute values.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AttributeValueMetadata {
    /// The display name for this value, localized where available; otherwise, in English. The value display name is intended to be used in context with the attribute display name. For example, for a "WiFi" enum attribute, this could contain "Paid" to represent paid Wi-Fi.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The attribute value.
    
    pub value: Option<json::Value>,
}

impl client::Part for AttributeValueMetadata {}


/// A container for all the attributes for a given location.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [attributes get google updated locations](LocationAttributeGetGoogleUpdatedCall) (response)
/// * [get attributes locations](LocationGetAttributeCall) (response)
/// * [update attributes locations](LocationUpdateAttributeCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Attributes {
    /// A collection of attributes that need to be updated.
    
    pub attributes: Option<Vec<Attribute>>,
    /// Required. Google identifier for this location in the form of `locations/{location_id}/attributes`.
    
    pub name: Option<String>,
}

impl client::RequestValue for Attributes {}
impl client::ResponseResult for Attributes {}


/// Response message for BusinessCategories.BatchGetBusinessCategories.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch get categories](CategoryBatchGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchGetCategoriesResponse {
    /// Categories that match the GConcept ids provided in the request. They will not come in the same order as category ids in the request.
    
    pub categories: Option<Vec<Category>>,
}

impl client::ResponseResult for BatchGetCategoriesResponse {}


/// Represents the time periods that this location is open for business. Holds a collection of TimePeriod instances.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BusinessHours {
    /// Required. A collection of times that this location is open for business. Each period represents a range of hours when the location is open during the week.
    
    pub periods: Option<Vec<TimePeriod>>,
}

impl client::Part for BusinessHours {}


/// A collection of categories that describes the business. During updates, both fields must be set. Clients are prohibited from individually updating the primary or additional categories using the update mask.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Categories {
    /// Optional. Additional categories to describe your business. Categories help your customers find accurate, specific results for services they're interested in. To keep your business information accurate and live, make sure that you use as few categories as possible to describe your overall core business. Choose categories that are as specific as possible, but representative of your main business.
    #[serde(rename="additionalCategories")]
    
    pub additional_categories: Option<Vec<Category>>,
    /// Required. Category that best describes the core business this location engages in.
    #[serde(rename="primaryCategory")]
    
    pub primary_category: Option<Category>,
}

impl client::Part for Categories {}


/// A category describing what this business is (not what it does). For a list of valid category IDs, and the mappings to their human-readable names, see `categories.list`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Category {
    /// Output only. The human-readable name of the category. This is set when reading the location. When modifying the location, `category_id` must be set.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. More hours types that are available for this business category.
    #[serde(rename="moreHoursTypes")]
    
    pub more_hours_types: Option<Vec<MoreHoursType>>,
    /// Required. A stable ID (provided by Google) for this category. The value must be specified when modifying the category (when creating or updating a location).
    
    pub name: Option<String>,
    /// Output only. A list of all the service types that are available for this business category.
    #[serde(rename="serviceTypes")]
    
    pub service_types: Option<Vec<ServiceType>>,
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
    /// Required. The chain's resource name, in the format `chains/{chain_id}`.
    
    pub name: Option<String>,
    /// Websites of the chain.
    
    pub websites: Option<Vec<ChainUri>>,
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
pub struct ChainUri {
    /// The uri for this chain.
    
    pub uri: Option<String>,
}

impl client::Part for ChainUri {}


/// Request message for Locations.ClearLocationAssociationRequest.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [clear location association locations](LocationClearLocationAssociationCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClearLocationAssociationRequest { _never_set: Option<bool> }

impl client::RequestValue for ClearLocationAssociationRequest {}


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


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [associate locations](LocationAssociateCall) (response)
/// * [clear location association locations](LocationClearLocationAssociationCall) (response)
/// * [delete locations](LocationDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// Represents a free-form service offered by the merchant. These are services that are not exposed as part of our structure service data. The merchant manually enters the names for of such services via a geomerchant surface.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FreeFormServiceItem {
    /// Required. This field represents the category name (i.e. the category's stable ID). The `category` and `service_type_id` should match the possible combinations provided in the `Category` message.
    
    pub category: Option<String>,
    /// Required. Language-tagged labels for the item. We recommend that item names be 140 characters or less, and descriptions 250 characters or less. This field should only be set if the input is a custom service item. Standardized service types should be updated via service_type_id.
    
    pub label: Option<Label>,
}

impl client::Part for FreeFormServiceItem {}


/// Represents a Location that is present on Google. This can be a location that has been claimed by the user, someone else, or could be unclaimed.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search google locations](GoogleLocationSearchCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleLocation {
    /// The sparsely populated Location information. This field can be re-used in CreateLocation if it is not currently claimed by a user.
    
    pub location: Option<Location>,
    /// Resource name of this GoogleLocation, in the format `googleLocations/{googleLocationId}`.
    
    pub name: Option<String>,
    /// A URL that will redirect the user to the request admin rights UI. This field is only present if the location has already been claimed by any user, including the current user.
    #[serde(rename="requestAdminRightsUri")]
    
    pub request_admin_rights_uri: Option<String>,
}

impl client::Resource for GoogleLocation {}


/// Represents a location that was modified by Google.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get google updated locations](LocationGetGoogleUpdatedCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleUpdatedLocation {
    /// The fields that Google updated.
    #[serde(rename="diffMask")]
    
    pub diff_mask: Option<client::FieldMask>,
    /// The Google-updated version of this location.
    
    pub location: Option<Location>,
    /// The fields that have pending edits that haven't yet been pushed to Maps and Search.
    #[serde(rename="pendingMask")]
    
    pub pending_mask: Option<client::FieldMask>,
}

impl client::ResponseResult for GoogleUpdatedLocation {}


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
    /// Optional. The BCP-47 language code that these strings apply for. Only one set of labels may be set per language.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
}

impl client::Part for Label {}


/// An object that represents a latitude/longitude pair. This is expressed as a pair of doubles to represent degrees latitude and degrees longitude. Unless specified otherwise, this object must conform to the WGS84 standard. Values must be within normalized ranges.
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


/// Response for AttributesService.ListAttributeMetadata.
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
    #[serde(rename="attributeMetadata")]
    
    pub attribute_metadata: Option<Vec<AttributeMetadata>>,
    /// If the number of attributes exceeded the requested page size, this field will be populated with a token to fetch the next page of attributes on a subsequent call to `attributes.list`. If there are no more attributes, this field will not be present in the response.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListAttributeMetadataResponse {}


/// Response message for BusinessCategories.ListCategories.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list categories](CategoryListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCategoriesResponse {
    /// The matching categories based on the requested parameters.
    
    pub categories: Option<Vec<Category>>,
    /// If the number of categories exceeded the requested page size, this field will be populated with a token to fetch the next page of categories on a subsequent call to `ListCategories`.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListCategoriesResponse {}


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
    /// If the number of locations exceeded the requested page size, this field is populated with a token to fetch the next page of locations on a subsequent call to `ListLocations`. If there are no more locations, this field is not present in the response.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The approximate number of Locations in the list irrespective of pagination.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
}

impl client::ResponseResult for ListLocationsResponse {}


/// A location. See the \[help center article\] (https://support.google.com/business/answer/3038177) for a detailed description of these fields, or the [category endpoint](https://developers.google.com/my-business/reference/rest/v4/categories) for a list of valid business categories.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations create accounts](AccountLocationCreateCall) (request|response)
/// * [attributes get google updated locations](LocationAttributeGetGoogleUpdatedCall) (none)
/// * [associate locations](LocationAssociateCall) (none)
/// * [clear location association locations](LocationClearLocationAssociationCall) (none)
/// * [delete locations](LocationDeleteCall) (none)
/// * [get locations](LocationGetCall) (response)
/// * [get attributes locations](LocationGetAttributeCall) (none)
/// * [get google updated locations](LocationGetGoogleUpdatedCall) (none)
/// * [patch locations](LocationPatchCall) (request|response)
/// * [update attributes locations](LocationUpdateAttributeCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    /// Optional. Additional information that is surfaced in AdWords.
    #[serde(rename="adWordsLocationExtensions")]
    
    pub ad_words_location_extensions: Option<AdWordsLocationExtensions>,
    /// Optional. The different categories that describe the business.
    
    pub categories: Option<Categories>,
    /// Optional. A collection of free-form strings to allow you to tag your business. These labels are NOT user facing; only you can see them. Must be between 1-255 characters per label.
    
    pub labels: Option<Vec<String>>,
    /// Immutable. The language of the location. Set during creation and not updateable.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Optional. User-provided latitude and longitude. When creating a location, this field is ignored if the provided address geocodes successfully. This field is only returned on get requests if the user-provided `latlng` value was accepted during create, or the `latlng` value was updated through the Google Business Profile website. This field can only be updated by approved clients.
    
    pub latlng: Option<LatLng>,
    /// Output only. Additional non-user-editable information.
    
    pub metadata: Option<Metadata>,
    /// Optional. More hours for a business's different departments or specific customers.
    #[serde(rename="moreHours")]
    
    pub more_hours: Option<Vec<MoreHours>>,
    /// Google identifier for this location in the form: `locations/{location_id}`.
    
    pub name: Option<String>,
    /// Optional. A flag that indicates whether the location is currently open for business.
    #[serde(rename="openInfo")]
    
    pub open_info: Option<OpenInfo>,
    /// Optional. The different phone numbers that customers can use to get in touch with the business.
    #[serde(rename="phoneNumbers")]
    
    pub phone_numbers: Option<PhoneNumbers>,
    /// Optional. Describes your business in your own voice and shares with users the unique story of your business and offerings. This field is required for all categories except lodging categories (e.g. hotels, motels, inns).
    
    pub profile: Option<Profile>,
    /// Optional. Operating hours for the business.
    #[serde(rename="regularHours")]
    
    pub regular_hours: Option<BusinessHours>,
    /// Optional. All locations and chain related to this one.
    #[serde(rename="relationshipData")]
    
    pub relationship_data: Option<RelationshipData>,
    /// Optional. Service area businesses provide their service at the customer's location. If this business is a service area business, this field describes the area(s) serviced by the business.
    #[serde(rename="serviceArea")]
    
    pub service_area: Option<ServiceAreaBusiness>,
    /// Optional. List of services supported by merchants. A service can be haircut, install water heater, etc. Duplicated service items will be removed automatically.
    #[serde(rename="serviceItems")]
    
    pub service_items: Option<Vec<ServiceItem>>,
    /// Optional. Special hours for the business. This typically includes holiday hours, and other times outside of regular operating hours. These override regular business hours. This field cannot be set without regular hours.
    #[serde(rename="specialHours")]
    
    pub special_hours: Option<SpecialHours>,
    /// Optional. External identifier for this location, which must be unique within a given account. This is a means of associating the location with your own records.
    #[serde(rename="storeCode")]
    
    pub store_code: Option<String>,
    /// Optional. A precise, accurate address to describe your business location. PO boxes or mailboxes located at remote locations are not acceptable. At this time, you can specify a maximum of five `address_lines` values in the address. This field should only be set for businesses that have a storefront. This field should not be set for locations of type `CUSTOMER_LOCATION_ONLY`.
    #[serde(rename="storefrontAddress")]
    
    pub storefront_address: Option<PostalAddress>,
    /// Required. Location name should reflect your business's real-world name, as used consistently on your storefront, website, and stationery, and as known to customers. Any additional information, when relevant, can be included in other fields of the resource (for example, `Address`, `Categories`). Don't add unnecessary information to your name (for example, prefer "Google" over "Google Inc. - Mountain View Corporate Headquarters"). Don't include marketing taglines, store codes, special characters, hours or closed/open status, phone numbers, website URLs, service/product information, location/address or directions, or containment information (for example, "Chase ATM in Duane Reade").
    
    pub title: Option<String>,
    /// Optional. A URL for this business. If possible, use a URL that represents this individual business location instead of a generic website/URL that represents all locations, or the brand.
    #[serde(rename="websiteUri")]
    
    pub website_uri: Option<String>,
}

impl client::RequestValue for Location {}
impl client::Resource for Location {}
impl client::ResponseResult for Location {}


/// Additional non-user-editable information about the location.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    /// Output only. Indicates whether the location can be deleted using the API.
    #[serde(rename="canDelete")]
    
    pub can_delete: Option<bool>,
    /// Output only. Indicates if the listing is eligible for business calls.
    #[serde(rename="canHaveBusinessCalls")]
    
    pub can_have_business_calls: Option<bool>,
    /// Output only. Indicates if the listing is eligible for food menu.
    #[serde(rename="canHaveFoodMenus")]
    
    pub can_have_food_menus: Option<bool>,
    /// Output only. Indicates if the listing can modify the service list.
    #[serde(rename="canModifyServiceList")]
    
    pub can_modify_service_list: Option<bool>,
    /// Output only. Indicates whether the location can operate on Health data.
    #[serde(rename="canOperateHealthData")]
    
    pub can_operate_health_data: Option<bool>,
    /// Output only. Indicates if the listing can manage local posts.
    #[serde(rename="canOperateLocalPost")]
    
    pub can_operate_local_post: Option<bool>,
    /// Output only. Indicates whether the location can operate on Lodging data.
    #[serde(rename="canOperateLodgingData")]
    
    pub can_operate_lodging_data: Option<bool>,
    /// Output only. The location resource that this location duplicates.
    #[serde(rename="duplicateLocation")]
    
    pub duplicate_location: Option<String>,
    /// Output only. Indicates whether the place ID associated with this location has updates that need to be updated or rejected by the client. If this boolean is set, you should call the `getGoogleUpdated` method to lookup information that's needs to be verified.
    #[serde(rename="hasGoogleUpdated")]
    
    pub has_google_updated: Option<bool>,
    /// Output only. Indicates whether any of this Location's properties are in the edit pending state.
    #[serde(rename="hasPendingEdits")]
    
    pub has_pending_edits: Option<bool>,
    /// Output only. Indicates if the listing has Voice of Merchant. If this boolean is false, you should call the locations.getVoiceOfMerchantState API to get details as to why they do not have Voice of Merchant.
    #[serde(rename="hasVoiceOfMerchant")]
    
    pub has_voice_of_merchant: Option<bool>,
    /// Output only. A link to the location on Maps.
    #[serde(rename="mapsUri")]
    
    pub maps_uri: Option<String>,
    /// Output only. A link to the page on Google Search where a customer can leave a review for the location.
    #[serde(rename="newReviewUri")]
    
    pub new_review_uri: Option<String>,
    /// Output only. If this locationappears on Google Maps, this field is populated with the place ID for the location. This ID can be used in various Places APIs. This field can be set during Create calls, but not for Update.
    #[serde(rename="placeId")]
    
    pub place_id: Option<String>,
}

impl client::Part for Metadata {}


/// Represents an amount of money with its currency type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Money {
    /// The three-letter currency code defined in ISO 4217.
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// Number of nano (10^-9) units of the amount. The value must be between -999,999,999 and +999,999,999 inclusive. If `units` is positive, `nanos` must be positive or zero. If `units` is zero, `nanos` can be positive, zero, or negative. If `units` is negative, `nanos` must be negative or zero. For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
    
    pub nanos: Option<i32>,
    /// The whole units of the amount. For example if `currencyCode` is `"USD"`, then 1 unit is one US dollar.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub units: Option<i64>,
}

impl client::Part for Money {}


/// The time periods during which a location is open for certain types of business.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MoreHours {
    /// Required. Type of hours. Clients should call {#link businessCategories:BatchGet} to get supported hours types for categories of their locations.
    #[serde(rename="hoursTypeId")]
    
    pub hours_type_id: Option<String>,
    /// Required. A collection of times that this location is open. Each period represents a range of hours when the location is open during the week.
    
    pub periods: Option<Vec<TimePeriod>>,
}

impl client::Part for MoreHours {}


/// More hours types that a business can offers, in addition to its regular hours.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MoreHoursType {
    /// Output only. The human-readable English display name for the hours type.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. A stable ID provided by Google for this hours type.
    #[serde(rename="hoursTypeId")]
    
    pub hours_type_id: Option<String>,
    /// Output only. The human-readable localized display name for the hours type.
    #[serde(rename="localizedDisplayName")]
    
    pub localized_display_name: Option<String>,
}

impl client::Part for MoreHoursType {}


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
    /// Optional. The date on which the location first opened. If the exact day is not known, month and year only can be provided. The date must be in the past or be no more than one year in the future.
    #[serde(rename="openingDate")]
    
    pub opening_date: Option<Date>,
    /// Required. Indicates whether or not the Location is currently open for business. All locations are open by default, unless updated to be closed.
    
    pub status: Option<String>,
}

impl client::Part for OpenInfo {}


/// A collection of phone numbers for the business. During updates, both fields must be set. Clients may not update just the primary or additional phone numbers using the update mask. International phone format is preferred, such as "+1 415 555 0132", see more in (https://developers.google.com/style/phone-numbers#international-phone-numbers).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PhoneNumbers {
    /// Optional. Up to two phone numbers (mobile or landline, no fax) at which your business can be called, in addition to your primary phone number.
    #[serde(rename="additionalPhones")]
    
    pub additional_phones: Option<Vec<String>>,
    /// Required. A phone number that connects to your individual business location as directly as possible. Use a local phone number instead of a central, call center helpline number whenever possible.
    #[serde(rename="primaryPhone")]
    
    pub primary_phone: Option<String>,
}

impl client::Part for PhoneNumbers {}


/// Defines an area that's represented by a place ID.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlaceInfo {
    /// Required. The ID of the place. Must correspond to a region. (https://developers.google.com/places/web-service/supported_types#table3)
    #[serde(rename="placeId")]
    
    pub place_id: Option<String>,
    /// Required. The localized name of the place. For example, `Scottsdale, AZ`.
    #[serde(rename="placeName")]
    
    pub place_name: Option<String>,
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


/// Represents a postal address, e.g. for postal delivery or payments addresses. Given a postal address, a postal service can deliver items to a premise, P.O. Box or similar. It is not intended to model geographical locations (roads, towns, mountains). In typical usage an address would be created via user input or from importing existing data, depending on the type of process. Advice on address input / editing: - Use an internationalization-ready address widget such as https://github.com/google/libaddressinput) - Users should not be presented with UI elements for input or editing of fields outside countries where that field is used. For more guidance on how to use this schema, please see: https://support.google.com/business/answer/6397478
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PostalAddress {
    /// Unstructured address lines describing the lower levels of an address. Because values in address_lines do not have type information and may sometimes contain multiple values in a single field (e.g. "Austin, TX"), it is important that the line order is clear. The order of address lines should be "envelope order" for the country/region of the address. In places where this can vary (e.g. Japan), address_language is used to make it explicit (e.g. "ja" for large-to-small ordering and "ja-Latn" or "en" for small-to-large). This way, the most specific line of an address can be selected based on the language. The minimum permitted structural representation of an address consists of a region_code with all remaining information placed in the address_lines. It would be possible to format such an address very approximately without geocoding, but no semantic reasoning could be made about any of the address components until it was at least partially resolved. Creating an address only containing a region_code and address_lines, and then geocoding is the recommended way to handle completely unstructured addresses (as opposed to guessing which parts of the address should be localities or administrative areas).
    #[serde(rename="addressLines")]
    
    pub address_lines: Option<Vec<String>>,
    /// Optional. Highest administrative subdivision which is used for postal addresses of a country or region. For example, this can be a state, a province, an oblast, or a prefecture. Specifically, for Spain this is the province and not the autonomous community (e.g. "Barcelona" and not "Catalonia"). Many countries don't use an administrative area in postal addresses. E.g. in Switzerland this should be left unpopulated.
    #[serde(rename="administrativeArea")]
    
    pub administrative_area: Option<String>,
    /// Optional. BCP-47 language code of the contents of this address (if known). This is often the UI language of the input form or is expected to match one of the languages used in the address' country/region, or their transliterated equivalents. This can affect formatting in certain countries, but is not critical to the correctness of the data and will never affect any validation or other non-formatting related operations. If this value is not known, it should be omitted (rather than specifying a possibly incorrect default). Examples: "zh-Hant", "ja", "ja-Latn", "en".
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Optional. Generally refers to the city/town portion of the address. Examples: US city, IT comune, UK post town. In regions of the world where localities are not well defined or do not fit into this structure well, leave locality empty and use address_lines.
    
    pub locality: Option<String>,
    /// Optional. The name of the organization at the address.
    
    pub organization: Option<String>,
    /// Optional. Postal code of the address. Not all countries use or require postal codes to be present, but where they are used, they may trigger additional validation with other parts of the address (e.g. state/zip validation in the U.S.A.).
    #[serde(rename="postalCode")]
    
    pub postal_code: Option<String>,
    /// Optional. The recipient at the address. This field may, under certain circumstances, contain multiline information. For example, it might contain "care of" information.
    
    pub recipients: Option<Vec<String>>,
    /// Required. CLDR region code of the country/region of the address. This is never inferred and it is up to the user to ensure the value is correct. See https://cldr.unicode.org/ and https://www.unicode.org/cldr/charts/30/supplemental/territory_information.html for details. Example: "CH" for Switzerland.
    #[serde(rename="regionCode")]
    
    pub region_code: Option<String>,
    /// The schema revision of the `PostalAddress`. This must be set to 0, which is the latest revision. All new revisions **must** be backward compatible with old revisions.
    
    pub revision: Option<i32>,
    /// Optional. Additional, country-specific, sorting code. This is not used in most regions. Where it is used, the value is either a string like "CEDEX", optionally followed by a number (e.g. "CEDEX 7"), or just a number alone, representing the "sector code" (Jamaica), "delivery area indicator" (Malawi) or "post office indicator" (e.g. Côte d'Ivoire).
    #[serde(rename="sortingCode")]
    
    pub sorting_code: Option<String>,
    /// Optional. Sublocality of the address. For example, this can be neighborhoods, boroughs, districts.
    
    pub sublocality: Option<String>,
}

impl client::Part for PostalAddress {}


/// All information pertaining to the location's profile.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Profile {
    /// Required. Description of the location in your own voice, not editable by anyone else.
    
    pub description: Option<String>,
}

impl client::Part for Profile {}


/// Information of all parent and children locations related to this one.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RelationshipData {
    /// The list of children locations that this location has relations with.
    #[serde(rename="childrenLocations")]
    
    pub children_locations: Option<Vec<RelevantLocation>>,
    /// The resource name of the Chain that this location is member of. How to find Chain ID
    #[serde(rename="parentChain")]
    
    pub parent_chain: Option<String>,
    /// The parent location that this location has relations with.
    #[serde(rename="parentLocation")]
    
    pub parent_location: Option<RelevantLocation>,
}

impl client::Part for RelationshipData {}


/// Information about another location that is related to current one. The relation can be any one of DEPARTMENT_OF or INDEPENDENT_ESTABLISHMENT_OF, and the location specified here can be on either side (parent/child) of the location.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RelevantLocation {
    /// Required. Specify the location that is on the other side of the relation by its placeID.
    #[serde(rename="placeId")]
    
    pub place_id: Option<String>,
    /// Required. The type of the relationship.
    #[serde(rename="relationType")]
    
    pub relation_type: Option<String>,
}

impl client::Part for RelevantLocation {}


/// Values for an attribute with a `value_type` of REPEATED_ENUM. This consists of two lists of value IDs: those that are set (true) and those that are unset (false). Values absent are considered unknown. At least one value must be specified.
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
    /// Chains that match the queried chain_display_name in SearchChainsRequest. If there are no matches, this field will be empty. Results are listed in order of relevance.
    
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
    /// Location to search for. If provided, will find locations which match the provided location details.
    
    pub location: Option<Location>,
    /// The number of matches to return. The default value is 3, with a maximum of 10. Note that latency may increase if more are requested. There is no pagination.
    #[serde(rename="pageSize")]
    
    pub page_size: Option<i32>,
    /// Text query to search for. The search results from a query string will be less accurate than if providing an exact location, but can provide more inexact matches.
    
    pub query: Option<String>,
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
    /// A collection of GoogleLocations that are potential matches to the specified request, listed in order from most to least accuracy.
    #[serde(rename="googleLocations")]
    
    pub google_locations: Option<Vec<GoogleLocation>>,
}

impl client::ResponseResult for SearchGoogleLocationsResponse {}


/// Service area businesses provide their service at the customer's location (for example, a locksmith or plumber).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceAreaBusiness {
    /// Required. Indicates the type of the service area business.
    #[serde(rename="businessType")]
    
    pub business_type: Option<String>,
    /// The area that this business serves defined through a set of places.
    
    pub places: Option<Places>,
    /// Immutable. CLDR region code of the country/region that this service area business is based in. See http://cldr.unicode.org/ and http://www.unicode.org/cldr/charts/30/supplemental/territory_information.html for details. Example: "CH" for Switzerland. This field is required for CUSTOMER_LOCATION_ONLY businesses, and is ignored otherwise. The region specified here can be different from regions for the areas that this business serves (e.g. service area businesses that provide services in regions other than the one that they are based in). If this location requires verification after creation, the address provided for verification purposes *must* be located within this region, and the business owner or their authorized representative *must* be able to receive postal mail at the provided verification address.
    #[serde(rename="regionCode")]
    
    pub region_code: Option<String>,
}

impl client::Part for ServiceAreaBusiness {}


/// A message that describes a single service item. It is used to describe the type of service that the merchant provides. For example, haircut can be a service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceItem {
    /// Optional. This field will be set case of free-form services data.
    #[serde(rename="freeFormServiceItem")]
    
    pub free_form_service_item: Option<FreeFormServiceItem>,
    /// Optional. Represents the monetary price of the service item. We recommend that currency_code and units should be set when including a price. This will be treated as a fixed price for the service item.
    
    pub price: Option<Money>,
    /// Optional. This field will be set case of structured services data.
    #[serde(rename="structuredServiceItem")]
    
    pub structured_service_item: Option<StructuredServiceItem>,
}

impl client::Part for ServiceItem {}


/// A message describing a service type that the business offers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceType {
    /// Output only. The human-readable display name for the service type.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. A stable ID (provided by Google) for this service type.
    #[serde(rename="serviceTypeId")]
    
    pub service_type_id: Option<String>,
}

impl client::Part for ServiceType {}


/// Represents a single time period when a location's operational hours differ from its normal business hours. A special hour period must represent a range of less than 24 hours. The `open_time` and `start_date` must predate the `close_time` and `end_date`. The `close_time` and `end_date` can extend to 11:59 a.m. on the day after the specified `start_date`. For example, the following inputs are valid: start_date=2015-11-23, open_time=08:00, close_time=18:00 start_date=2015-11-23, end_date=2015-11-23, open_time=08:00, close_time=18:00 start_date=2015-11-23, end_date=2015-11-24, open_time=13:00, close_time=11:59 The following inputs are not valid: start_date=2015-11-23, open_time=13:00, close_time=11:59 start_date=2015-11-23, end_date=2015-11-24, open_time=13:00, close_time=12:00 start_date=2015-11-23, end_date=2015-11-25, open_time=08:00, close_time=18:00
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpecialHourPeriod {
    /// Optional. Valid values are 00:00-24:00, where 24:00 represents midnight at the end of the specified day field. Must be specified if `closed` is false.
    #[serde(rename="closeTime")]
    
    pub close_time: Option<TimeOfDay>,
    /// Optional. If true, `end_date`, `open_time`, and `close_time` are ignored, and the date specified in `start_date` is treated as the location being closed for the entire day.
    
    pub closed: Option<bool>,
    /// Optional. The calendar date this special hour period ends on. If `end_date` field is not set, default to the date specified in `start_date`. If set, this field must be equal to or at most 1 day after `start_date`.
    #[serde(rename="endDate")]
    
    pub end_date: Option<Date>,
    /// Optional. Valid values are 00:00-24:00 where 24:00 represents midnight at the end of the specified day field. Must be specified if `closed` is false.
    #[serde(rename="openTime")]
    
    pub open_time: Option<TimeOfDay>,
    /// Required. The calendar date this special hour period starts on.
    #[serde(rename="startDate")]
    
    pub start_date: Option<Date>,
}

impl client::Part for SpecialHourPeriod {}


/// Represents a set of time periods when a location's operational hours differ from its normal business hours.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpecialHours {
    /// Required. A list of exceptions to the business's regular hours.
    #[serde(rename="specialHourPeriods")]
    
    pub special_hour_periods: Option<Vec<SpecialHourPeriod>>,
}

impl client::Part for SpecialHours {}


/// Represents a structured service offered by the merchant. For eg: toilet_installation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StructuredServiceItem {
    /// Optional. Description of structured service item. The character limit is 300.
    
    pub description: Option<String>,
    /// Required. The `service_type_id` field is a Google provided unique ID that can be found in `ServiceType`. This information is provided by `BatchGetCategories` rpc service.
    #[serde(rename="serviceTypeId")]
    
    pub service_type_id: Option<String>,
}

impl client::Part for StructuredServiceItem {}


/// Represents a time of day. The date and time zone are either not significant or are specified elsewhere. An API may choose to allow leap seconds. Related types are google.type.Date and `google.protobuf.Timestamp`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeOfDay {
    /// Hours of day in 24 hour format. Should be from 0 to 23. An API may choose to allow the value "24:00:00" for scenarios like business closing time.
    
    pub hours: Option<i32>,
    /// Minutes of hour of day. Must be from 0 to 59.
    
    pub minutes: Option<i32>,
    /// Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999.
    
    pub nanos: Option<i32>,
    /// Seconds of minutes of the time. Must normally be from 0 to 59. An API may allow the value 60 if it allows leap-seconds.
    
    pub seconds: Option<i32>,
}

impl client::Part for TimeOfDay {}


/// Represents a span of time that the business is open, starting on the specified open day/time and closing on the specified close day/time. The closing time must occur after the opening time, for example later in the same day, or on a subsequent day.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimePeriod {
    /// Required. Indicates the day of the week this period ends on.
    #[serde(rename="closeDay")]
    
    pub close_day: Option<String>,
    /// Required. Valid values are 00:00-24:00, where 24:00 represents midnight at the end of the specified day field.
    #[serde(rename="closeTime")]
    
    pub close_time: Option<TimeOfDay>,
    /// Required. Indicates the day of the week this period starts on.
    #[serde(rename="openDay")]
    
    pub open_day: Option<String>,
    /// Required. Valid values are 00:00-24:00, where 24:00 represents midnight at the end of the specified day field.
    #[serde(rename="openTime")]
    
    pub open_time: Option<TimeOfDay>,
}

impl client::Part for TimePeriod {}


/// Values for an attribute with a `value_type` of URL.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UriAttributeValue {
    /// Required. The proposed URI value for this attribute.
    
    pub uri: Option<String>,
}

impl client::Part for UriAttributeValue {}


