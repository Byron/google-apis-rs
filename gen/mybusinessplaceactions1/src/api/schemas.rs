use super::*;
/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [place action links delete locations](LocationPlaceActionLinkDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// Response message for PlaceActions.ListPlaceActionLinks.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [place action links list locations](LocationPlaceActionLinkListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListPlaceActionLinksResponse {
    /// If there are more place action links than the requested page size, then this field is populated with a token to fetch the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The returned list of place action links.
    #[serde(rename="placeActionLinks")]
    
    pub place_action_links: Option<Vec<PlaceActionLink>>,
}

impl client::ResponseResult for ListPlaceActionLinksResponse {}


/// Response message for PlaceActions.ListPlaceActionTypeMetadata.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list place action type metadata](PlaceActionTypeMetadataListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListPlaceActionTypeMetadataResponse {
    /// If the number of action types exceeded the requested page size, this field will be populated with a token to fetch the next page on a subsequent call to `placeActionTypeMetadata.list`. If there are no more results, this field will not be present in the response.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A collection of metadata for the available place action types.
    #[serde(rename="placeActionTypeMetadata")]
    
    pub place_action_type_metadata: Option<Vec<PlaceActionTypeMetadata>>,
}

impl client::ResponseResult for ListPlaceActionTypeMetadataResponse {}


/// Represents a place action link and its attributes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [place action links create locations](LocationPlaceActionLinkCreateCall) (request|response)
/// * [place action links get locations](LocationPlaceActionLinkGetCall) (response)
/// * [place action links patch locations](LocationPlaceActionLinkPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlaceActionLink {
    /// Output only. The time when the place action link was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Indicates whether this link can be edited by the client.
    #[serde(rename="isEditable")]
    
    pub is_editable: Option<bool>,
    /// Optional. Whether this link is preferred by the merchant. Only one link can be marked as preferred per place action type at a location. If a future request marks a different link as preferred for the same place action type, then the current preferred link (if any exists) will lose its preference.
    #[serde(rename="isPreferred")]
    
    pub is_preferred: Option<bool>,
    /// Optional. The resource name, in the format `locations/{location_id}/placeActionLinks/{place_action_link_id}`. The name field will only be considered in UpdatePlaceActionLink and DeletePlaceActionLink requests for updating and deleting links respectively. However, it will be ignored in CreatePlaceActionLink request, where `place_action_link_id` will be assigned by the server on successful creation of a new link and returned as part of the response.
    
    pub name: Option<String>,
    /// Required. The type of place action that can be performed using this link.
    #[serde(rename="placeActionType")]
    
    pub place_action_type: Option<PlaceActionLinkPlaceActionTypeEnum>,
    /// Output only. Specifies the provider type.
    #[serde(rename="providerType")]
    
    pub provider_type: Option<PlaceActionLinkProviderTypeEnum>,
    /// Output only. The time when the place action link was last modified.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. The link uri. The same uri can be reused for different action types across different locations. However, only one place action link is allowed for each unique combination of (uri, place action type, location).
    
    pub uri: Option<String>,
}

impl client::RequestValue for PlaceActionLink {}
impl client::ResponseResult for PlaceActionLink {}


/// Metadata for supported place action types.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlaceActionTypeMetadata {
    /// The localized display name for the attribute, if available; otherwise, the English display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The place action type.
    #[serde(rename="placeActionType")]
    
    pub place_action_type: Option<PlaceActionTypeMetadataPlaceActionTypeEnum>,
}

impl client::Part for PlaceActionTypeMetadata {}


