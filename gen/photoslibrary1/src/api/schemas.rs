use super::*;
/// Request to add an enrichment to a specific album at a specific position.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [add enrichment albums](AlbumAddEnrichmentCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddEnrichmentToAlbumRequest {
    /// Required. The position in the album where the enrichment is to be inserted.
    #[serde(rename="albumPosition")]
    
    pub album_position: Option<AlbumPosition>,
    /// Required. The enrichment to be added.
    #[serde(rename="newEnrichmentItem")]
    
    pub new_enrichment_item: Option<NewEnrichmentItem>,
}

impl client::RequestValue for AddEnrichmentToAlbumRequest {}


/// The enrichment item that’s created.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [add enrichment albums](AlbumAddEnrichmentCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddEnrichmentToAlbumResponse {
    /// Output only. Enrichment which was added.
    #[serde(rename="enrichmentItem")]
    
    pub enrichment_item: Option<EnrichmentItem>,
}

impl client::ResponseResult for AddEnrichmentToAlbumResponse {}


/// Representation of an album in Google Photos. Albums are containers for media items. If an album has been shared by the application, it contains an extra `shareInfo` property.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [add enrichment albums](AlbumAddEnrichmentCall) (none)
/// * [batch add media items albums](AlbumBatchAddMediaItemCall) (none)
/// * [batch remove media items albums](AlbumBatchRemoveMediaItemCall) (none)
/// * [create albums](AlbumCreateCall) (response)
/// * [get albums](AlbumGetCall) (response)
/// * [list albums](AlbumListCall) (none)
/// * [patch albums](AlbumPatchCall) (request|response)
/// * [share albums](AlbumShareCall) (none)
/// * [unshare albums](AlbumUnshareCall) (none)
/// * [get shared albums](SharedAlbumGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Album {
    /// [Output only] A URL to the cover photo's bytes. This shouldn't be used as is. Parameters should be appended to this URL before use. See the [developer documentation](https://developers.google.com/photos/library/guides/access-media-items#base-urls) for a complete list of supported parameters. For example, `'=w2048-h1024'` sets the dimensions of the cover photo to have a width of 2048 px and height of 1024 px.
    #[serde(rename="coverPhotoBaseUrl")]
    
    pub cover_photo_base_url: Option<String>,
    /// Identifier for the media item associated with the cover photo.
    #[serde(rename="coverPhotoMediaItemId")]
    
    pub cover_photo_media_item_id: Option<String>,
    /// Identifier for the album. This is a persistent identifier that can be used between sessions to identify this album.
    
    pub id: Option<String>,
    /// [Output only] True if you can create media items in this album. This field is based on the scopes granted and permissions of the album. If the scopes are changed or permissions of the album are changed, this field is updated.
    #[serde(rename="isWriteable")]
    
    pub is_writeable: Option<bool>,
    /// [Output only] The number of media items in the album.
    #[serde(rename="mediaItemsCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub media_items_count: Option<i64>,
    /// [Output only] Google Photos URL for the album. The user needs to be signed in to their Google Photos account to access this link.
    #[serde(rename="productUrl")]
    
    pub product_url: Option<String>,
    /// [Output only] Information related to shared albums. This field is only populated if the album is a shared album, the developer created the album and the user has granted the `photoslibrary.sharing` scope.
    #[serde(rename="shareInfo")]
    
    pub share_info: Option<ShareInfo>,
    /// Name of the album displayed to the user in their Google Photos account. This string shouldn't be more than 500 characters.
    
    pub title: Option<String>,
}

impl client::RequestValue for Album {}
impl client::Resource for Album {}
impl client::ResponseResult for Album {}


/// Specifies a position in an album.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AlbumPosition {
    /// Type of position, for a media or enrichment item.
    
    pub position: Option<AlbumPositionPositionEnum>,
    /// The enrichment item to which the position is relative to. Only used when position type is AFTER_ENRICHMENT_ITEM.
    #[serde(rename="relativeEnrichmentItemId")]
    
    pub relative_enrichment_item_id: Option<String>,
    /// The media item to which the position is relative to. Only used when position type is AFTER_MEDIA_ITEM.
    #[serde(rename="relativeMediaItemId")]
    
    pub relative_media_item_id: Option<String>,
}

impl client::Part for AlbumPosition {}


/// Request to add media items to an album.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch add media items albums](AlbumBatchAddMediaItemCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchAddMediaItemsToAlbumRequest {
    /// Required. Identifiers of the MediaItems to be added. The maximum number of media items that can be added in one call is 50.
    #[serde(rename="mediaItemIds")]
    
    pub media_item_ids: Option<Vec<String>>,
}

impl client::RequestValue for BatchAddMediaItemsToAlbumRequest {}


/// Response for adding media items to an album.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch add media items albums](AlbumBatchAddMediaItemCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchAddMediaItemsToAlbumResponse { _never_set: Option<bool> }

impl client::ResponseResult for BatchAddMediaItemsToAlbumResponse {}


/// Request to create one or more media items in a user’s Google Photos library. If an `albumid` is specified, the media items are also added to that album. `albumPosition` is optional and can only be specified if an `albumId` is set.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch create media items](MediaItemBatchCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchCreateMediaItemsRequest {
    /// Identifier of the album where the media items are added. The media items are also added to the user's library. This is an optional field.
    #[serde(rename="albumId")]
    
    pub album_id: Option<String>,
    /// Position in the album where the media items are added. If not specified, the media items are added to the end of the album (as per the default value, that is, `LAST_IN_ALBUM`). The request fails if this field is set and the `albumId` is not specified. The request will also fail if you set the field and are not the owner of the shared album.
    #[serde(rename="albumPosition")]
    
    pub album_position: Option<AlbumPosition>,
    /// Required. List of media items to be created. Maximum 50 media items per call.
    #[serde(rename="newMediaItems")]
    
    pub new_media_items: Option<Vec<NewMediaItem>>,
}

impl client::RequestValue for BatchCreateMediaItemsRequest {}


/// List of media items created.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch create media items](MediaItemBatchCreateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchCreateMediaItemsResponse {
    /// Output only. List of media items created.
    #[serde(rename="newMediaItemResults")]
    
    pub new_media_item_results: Option<Vec<NewMediaItemResult>>,
}

impl client::ResponseResult for BatchCreateMediaItemsResponse {}


/// Response to retrieve a list of media items.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch get media items](MediaItemBatchGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchGetMediaItemsResponse {
    /// Output only. List of media items retrieved. Note that even if the call to BatchGetMediaItems succeeds, there may have been failures for some media items in the batch. These failures are indicated in each MediaItemResult.status.
    #[serde(rename="mediaItemResults")]
    
    pub media_item_results: Option<Vec<MediaItemResult>>,
}

impl client::ResponseResult for BatchGetMediaItemsResponse {}


/// Request to remove a list of media items from an album.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch remove media items albums](AlbumBatchRemoveMediaItemCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchRemoveMediaItemsFromAlbumRequest {
    /// Required. Identifiers of the MediaItems to be removed. Must not contain repeated identifiers and cannot be empty. The maximum number of media items that can be removed in one call is 50.
    #[serde(rename="mediaItemIds")]
    
    pub media_item_ids: Option<Vec<String>>,
}

impl client::RequestValue for BatchRemoveMediaItemsFromAlbumRequest {}


/// Response for successfully removing all specified media items from the album.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch remove media items albums](AlbumBatchRemoveMediaItemCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchRemoveMediaItemsFromAlbumResponse { _never_set: Option<bool> }

impl client::ResponseResult for BatchRemoveMediaItemsFromAlbumResponse {}


/// This filter allows you to return media items based on the content type. It's possible to specify a list of categories to include, and/or a list of categories to exclude. Within each list, the categories are combined with an OR. The content filter `includedContentCategories`: [c1, c2, c3] would get media items that contain (c1 OR c2 OR c3). The content filter `excludedContentCategories`: [c1, c2, c3] would NOT get media items that contain (c1 OR c2 OR c3). You can also include some categories while excluding others, as in this example: `includedContentCategories`: [c1, c2], `excludedContentCategories`: [c3, c4] The previous example would get media items that contain (c1 OR c2) AND NOT (c3 OR c4). A category that appears in `includedContentategories` must not appear in `excludedContentCategories`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContentFilter {
    /// The set of categories which are not to be included in the media item search results. The items in the set are ORed. There's a maximum of 10 `excludedContentCategories` per request.
    #[serde(rename="excludedContentCategories")]
    
    pub excluded_content_categories: Option<Vec<ContentFilterExcludedContentCategoriesEnum>>,
    /// The set of categories to be included in the media item search results. The items in the set are ORed. There's a maximum of 10 `includedContentCategories` per request.
    #[serde(rename="includedContentCategories")]
    
    pub included_content_categories: Option<Vec<ContentFilterIncludedContentCategoriesEnum>>,
}

impl client::Part for ContentFilter {}


/// Information about the user who added the media item. Note that this information is included only if the media item is within a shared album created by your app and you have the sharing scope.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContributorInfo {
    /// Display name of the contributor.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// URL to the profile picture of the contributor.
    #[serde(rename="profilePictureBaseUrl")]
    
    pub profile_picture_base_url: Option<String>,
}

impl client::Part for ContributorInfo {}


/// Request to create an album in Google Photos.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create albums](AlbumCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateAlbumRequest {
    /// Required. The album to be created.
    
    pub album: Option<Album>,
}

impl client::RequestValue for CreateAlbumRequest {}


/// Represents a whole calendar date. Set `day` to 0 when only the month and year are significant, for example, all of December 2018. Set `day` and `month` to 0 if only the year is significant, for example, the entire of 2018. Set `year` to 0 when only the day and month are significant, for example, an anniversary or birthday. Unsupported: Setting all values to 0, only `month` to 0, or both `day` and `year` to 0 at the same time.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Date {
    /// Day of month. Must be from 1 to 31 and valid for the year and month, or 0 if specifying a year/month where the day isn't significant.
    
    pub day: Option<i32>,
    /// Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day.
    
    pub month: Option<i32>,
    /// Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year.
    
    pub year: Option<i32>,
}

impl client::Part for Date {}


/// This filter defines the allowed dates or date ranges for the media returned. It's possible to pick a set of specific dates and a set of date ranges.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DateFilter {
    /// List of dates that match the media items' creation date. A maximum of 5 dates can be included per request.
    
    pub dates: Option<Vec<Date>>,
    /// List of dates ranges that match the media items' creation date. A maximum of 5 dates ranges can be included per request.
    
    pub ranges: Option<Vec<DateRange>>,
}

impl client::Part for DateFilter {}


/// Defines a range of dates. Both dates must be of the same format. For more information, see Date.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DateRange {
    /// The end date (included as part of the range). It must be specified in the same format as the start date.
    #[serde(rename="endDate")]
    
    pub end_date: Option<Date>,
    /// The start date (included as part of the range) in one of the formats described.
    #[serde(rename="startDate")]
    
    pub start_date: Option<Date>,
}

impl client::Part for DateRange {}


/// An enrichment item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnrichmentItem {
    /// Identifier of the enrichment item.
    
    pub id: Option<String>,
}

impl client::Part for EnrichmentItem {}


/// This filter defines the features that the media items should have.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FeatureFilter {
    /// The set of features to be included in the media item search results. The items in the set are ORed and may match any of the specified features.
    #[serde(rename="includedFeatures")]
    
    pub included_features: Option<Vec<FeatureFilterIncludedFeaturesEnum>>,
}

impl client::Part for FeatureFilter {}


/// Filters that can be applied to a media item search. If multiple filter options are specified, they're treated as AND with each other.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Filters {
    /// Filters the media items based on their content.
    #[serde(rename="contentFilter")]
    
    pub content_filter: Option<ContentFilter>,
    /// Filters the media items based on their creation date.
    #[serde(rename="dateFilter")]
    
    pub date_filter: Option<DateFilter>,
    /// If set, the results exclude media items that were not created by this app. Defaults to false (all media items are returned). This field is ignored if the photoslibrary.readonly.appcreateddata scope is used.
    #[serde(rename="excludeNonAppCreatedData")]
    
    pub exclude_non_app_created_data: Option<bool>,
    /// Filters the media items based on their features.
    #[serde(rename="featureFilter")]
    
    pub feature_filter: Option<FeatureFilter>,
    /// If set, the results include media items that the user has archived. Defaults to false (archived media items aren't included).
    #[serde(rename="includeArchivedMedia")]
    
    pub include_archived_media: Option<bool>,
    /// Filters the media items based on the type of media.
    #[serde(rename="mediaTypeFilter")]
    
    pub media_type_filter: Option<MediaTypeFilter>,
}

impl client::Part for Filters {}


/// Request to join a shared album on behalf of the user. This uses a shareToken which can be acquired via the shareAlbum or listSharedAlbums calls.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [join shared albums](SharedAlbumJoinCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JoinSharedAlbumRequest {
    /// Required. Token to join the shared album on behalf of the user.
    #[serde(rename="shareToken")]
    
    pub share_token: Option<String>,
}

impl client::RequestValue for JoinSharedAlbumRequest {}


/// Response to successfully joining the shared album on behalf of the user.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [join shared albums](SharedAlbumJoinCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JoinSharedAlbumResponse {
    /// Shared album that the user has joined.
    
    pub album: Option<Album>,
}

impl client::ResponseResult for JoinSharedAlbumResponse {}


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


/// Request to leave a shared album on behalf of the user. This uses a shareToken which can be acquired via the or listSharedAlbums or getAlbum calls.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [leave shared albums](SharedAlbumLeaveCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LeaveSharedAlbumRequest {
    /// Required. Token to leave the shared album on behalf of the user.
    #[serde(rename="shareToken")]
    
    pub share_token: Option<String>,
}

impl client::RequestValue for LeaveSharedAlbumRequest {}


/// Response to successfully leaving the shared album on behalf of the user.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [leave shared albums](SharedAlbumLeaveCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LeaveSharedAlbumResponse { _never_set: Option<bool> }

impl client::ResponseResult for LeaveSharedAlbumResponse {}


/// List of albums requested.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list albums](AlbumListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAlbumsResponse {
    /// Output only. List of albums shown in the Albums tab of the user's Google Photos app.
    
    pub albums: Option<Vec<Album>>,
    /// Output only. Token to use to get the next set of albums. Populated if there are more albums to retrieve for this request.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListAlbumsResponse {}


/// List of all media items from the user’s Google Photos library.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list media items](MediaItemListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListMediaItemsResponse {
    /// Output only. List of media items in the user's library.
    #[serde(rename="mediaItems")]
    
    pub media_items: Option<Vec<MediaItem>>,
    /// Output only. Token to use to get the next set of media items. Its presence is the only reliable indicator of more media items being available in the next request.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListMediaItemsResponse {}


/// List of shared albums requested.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list shared albums](SharedAlbumListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListSharedAlbumsResponse {
    /// Output only. Token to use to get the next set of shared albums. Populated if there are more shared albums to retrieve for this request.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Output only. List of shared albums.
    #[serde(rename="sharedAlbums")]
    
    pub shared_albums: Option<Vec<Album>>,
}

impl client::ResponseResult for ListSharedAlbumsResponse {}


/// Represents a physical location.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    /// Position of the location on the map.
    
    pub latlng: Option<LatLng>,
    /// Name of the location to be displayed.
    #[serde(rename="locationName")]
    
    pub location_name: Option<String>,
}

impl client::Part for Location {}


/// An enrichment containing a single location.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocationEnrichment {
    /// Location for this enrichment item.
    
    pub location: Option<Location>,
}

impl client::Part for LocationEnrichment {}


/// An enrichment containing a map, showing origin and destination locations.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MapEnrichment {
    /// Destination location for this enrichemt item.
    
    pub destination: Option<Location>,
    /// Origin location for this enrichment item.
    
    pub origin: Option<Location>,
}

impl client::Part for MapEnrichment {}


/// Representation of a media item (such as a photo or video) in Google Photos.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch create media items](MediaItemBatchCreateCall) (none)
/// * [batch get media items](MediaItemBatchGetCall) (none)
/// * [get media items](MediaItemGetCall) (response)
/// * [list media items](MediaItemListCall) (none)
/// * [patch media items](MediaItemPatchCall) (request|response)
/// * [search media items](MediaItemSearchCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MediaItem {
    /// A URL to the media item's bytes. This shouldn't be used as is. Parameters should be appended to this URL before use. See the [developer documentation](https://developers.google.com/photos/library/guides/access-media-items#base-urls) for a complete list of supported parameters. For example, `'=w2048-h1024'` will set the dimensions of a media item of type photo to have a width of 2048 px and height of 1024 px.
    #[serde(rename="baseUrl")]
    
    pub base_url: Option<String>,
    /// Information about the user who added this media item. Note that this is only included when using mediaItems.search with the ID of a shared album. The album must be created by your app and you must have the sharing scope.
    #[serde(rename="contributorInfo")]
    
    pub contributor_info: Option<ContributorInfo>,
    /// Description of the media item. This is shown to the user in the item's info section in the Google Photos app. Must be shorter than 1000 characters. Only include text written by users. Descriptions should add context and help users understand media. Do not include any auto-generated strings such as filenames, tags, and other metadata.
    
    pub description: Option<String>,
    /// Filename of the media item. This is shown to the user in the item's info section in the Google Photos app.
    
    pub filename: Option<String>,
    /// Identifier for the media item. This is a persistent identifier that can be used between sessions to identify this media item.
    
    pub id: Option<String>,
    /// Metadata related to the media item, such as, height, width, or creation time.
    #[serde(rename="mediaMetadata")]
    
    pub media_metadata: Option<MediaMetadata>,
    /// MIME type of the media item. For example, `image/jpeg`.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
    /// Google Photos URL for the media item. This link is available to the user only if they're signed in. When retrieved from an album search, the URL points to the item inside the album.
    #[serde(rename="productUrl")]
    
    pub product_url: Option<String>,
}

impl client::RequestValue for MediaItem {}
impl client::Resource for MediaItem {}
impl client::ResponseResult for MediaItem {}


/// Result of retrieving a media item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MediaItemResult {
    /// Media item retrieved from the user's library. It's populated if no errors occurred and the media item was fetched successfully.
    #[serde(rename="mediaItem")]
    
    pub media_item: Option<MediaItem>,
    /// If an error occurred while accessing this media item, this field is populated with information related to the error. For details regarding this field, see Status.
    
    pub status: Option<Status>,
}

impl client::Part for MediaItemResult {}


/// Metadata for a media item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MediaMetadata {
    /// Time when the media item was first created (not when it was uploaded to Google Photos).
    #[serde(rename="creationTime")]
    
    pub creation_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Original height (in pixels) of the media item.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub height: Option<i64>,
    /// Metadata for a photo media type.
    
    pub photo: Option<Photo>,
    /// Metadata for a video media type.
    
    pub video: Option<Video>,
    /// Original width (in pixels) of the media item.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub width: Option<i64>,
}

impl client::Part for MediaMetadata {}


/// This filter defines the type of media items to be returned, for example, videos or photos. Only one media type is supported.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MediaTypeFilter {
    /// The types of media items to be included. This field should be populated with only one media type. If you specify multiple media types, it results in an error.
    #[serde(rename="mediaTypes")]
    
    pub media_types: Option<Vec<MediaTypeFilterMediaTypesEnum>>,
}

impl client::Part for MediaTypeFilter {}


/// A new enrichment item to be added to an album, used by the `albums.addEnrichment` call.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NewEnrichmentItem {
    /// Location to be added to the album.
    #[serde(rename="locationEnrichment")]
    
    pub location_enrichment: Option<LocationEnrichment>,
    /// Map to be added to the album.
    #[serde(rename="mapEnrichment")]
    
    pub map_enrichment: Option<MapEnrichment>,
    /// Text to be added to the album.
    #[serde(rename="textEnrichment")]
    
    pub text_enrichment: Option<TextEnrichment>,
}

impl client::Part for NewEnrichmentItem {}


/// New media item that's created in a user's Google Photos account.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NewMediaItem {
    /// Description of the media item. This is shown to the user in the item's info section in the Google Photos app. Must be shorter than 1000 characters. Only include text written by users. Descriptions should add context and help users understand media. Do not include any auto-generated strings such as filenames, tags, and other metadata.
    
    pub description: Option<String>,
    /// A new media item that has been uploaded via the included `uploadToken`.
    #[serde(rename="simpleMediaItem")]
    
    pub simple_media_item: Option<SimpleMediaItem>,
}

impl client::Part for NewMediaItem {}


/// Result of creating a new media item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NewMediaItemResult {
    /// Media item created with the upload token. It's populated if no errors occurred and the media item was created successfully.
    #[serde(rename="mediaItem")]
    
    pub media_item: Option<MediaItem>,
    /// If an error occurred during the creation of this media item, this field is populated with information related to the error. For details regarding this field, see Status.
    
    pub status: Option<Status>,
    /// The upload token used to create this new (simple) media item. Only populated if the media item is simple and required a single upload token.
    #[serde(rename="uploadToken")]
    
    pub upload_token: Option<String>,
}

impl client::Part for NewMediaItemResult {}


/// Metadata that is specific to a photo, such as, ISO, focal length and exposure time. Some of these fields may be null or not included.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Photo {
    /// Aperture f number of the camera lens with which the photo was taken.
    #[serde(rename="apertureFNumber")]
    
    pub aperture_f_number: Option<f32>,
    /// Brand of the camera with which the photo was taken.
    #[serde(rename="cameraMake")]
    
    pub camera_make: Option<String>,
    /// Model of the camera with which the photo was taken.
    #[serde(rename="cameraModel")]
    
    pub camera_model: Option<String>,
    /// Exposure time of the camera aperture when the photo was taken.
    #[serde(rename="exposureTime")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub exposure_time: Option<client::chrono::Duration>,
    /// Focal length of the camera lens with which the photo was taken.
    #[serde(rename="focalLength")]
    
    pub focal_length: Option<f32>,
    /// ISO of the camera with which the photo was taken.
    #[serde(rename="isoEquivalent")]
    
    pub iso_equivalent: Option<i32>,
}

impl client::Part for Photo {}


/// Request to search for media items in a user’s library. If the album id is specified, this call will return the list of media items in the album. If neither filters nor album id are specified, this call will return all media items in a user’s Google Photos library. If filters are specified, this call will return all media items in the user’s library that fulfill the filter criteria. Filters and album id must not both be set, as this will result in an invalid request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search media items](MediaItemSearchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchMediaItemsRequest {
    /// Identifier of an album. If populated, lists all media items in specified album. Can't set in conjunction with any filters.
    #[serde(rename="albumId")]
    
    pub album_id: Option<String>,
    /// Filters to apply to the request. Can't be set in conjunction with an `albumId`.
    
    pub filters: Option<Filters>,
    /// An optional field to specify the sort order of the search results. The `orderBy` field only works when a dateFilter is used. When this field is not specified, results are displayed newest first, oldest last by their creationTime. Providing `MediaMetadata.creation_time` displays search results in the opposite order, oldest first then newest last. To display results newest first then oldest last, include the `desc` argument as follows: `MediaMetadata.creation_time desc`. The only additional filters that can be used with this parameter are includeArchivedMedia and excludeNonAppCreatedData. No other filters are supported.
    #[serde(rename="orderBy")]
    
    pub order_by: Option<String>,
    /// Maximum number of media items to return in the response. Fewer media items might be returned than the specified number. The default `pageSize` is 25, the maximum is 100.
    #[serde(rename="pageSize")]
    
    pub page_size: Option<i32>,
    /// A continuation token to get the next page of the results. Adding this to the request returns the rows after the `pageToken`. The `pageToken` should be the value returned in the `nextPageToken` parameter in the response to the `searchMediaItems` request.
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
}

impl client::RequestValue for SearchMediaItemsRequest {}


/// List of media items that match the search parameters.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search media items](MediaItemSearchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchMediaItemsResponse {
    /// Output only. List of media items that match the search parameters.
    #[serde(rename="mediaItems")]
    
    pub media_items: Option<Vec<MediaItem>>,
    /// Output only. Use this token to get the next set of media items. Its presence is the only reliable indicator of more media items being available in the next request.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for SearchMediaItemsResponse {}


/// Request to make an album shared in Google Photos.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [share albums](AlbumShareCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ShareAlbumRequest {
    /// Options to be set when converting the album to a shared album.
    #[serde(rename="sharedAlbumOptions")]
    
    pub shared_album_options: Option<SharedAlbumOptions>,
}

impl client::RequestValue for ShareAlbumRequest {}


/// Response to successfully sharing an album.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [share albums](AlbumShareCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ShareAlbumResponse {
    /// Output only. Information about the shared album.
    #[serde(rename="shareInfo")]
    
    pub share_info: Option<ShareInfo>,
}

impl client::ResponseResult for ShareAlbumResponse {}


/// Information about albums that are shared. This information is only included if you created the album, it is shared and you have the sharing scope.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ShareInfo {
    /// True if the album can be joined by users.
    #[serde(rename="isJoinable")]
    
    pub is_joinable: Option<bool>,
    /// True if the user is joined to the album. This is always true for the owner of the album.
    #[serde(rename="isJoined")]
    
    pub is_joined: Option<bool>,
    /// True if the user owns the album.
    #[serde(rename="isOwned")]
    
    pub is_owned: Option<bool>,
    /// A token that is used to join, leave, or retrieve the details of a shared album on behalf of a user who isn't the owner. A `shareToken` is invalidated if the owner turns off link sharing in the Google Photos app, or if the album is unshared.
    #[serde(rename="shareToken")]
    
    pub share_token: Option<String>,
    /// A link to the shared Google Photos album. Anyone with the link can view the contents of the album, so it should be treated with care. The `shareableUrl` parameter is only returned if the album has link sharing turned on. If a user is already joined to an album that isn't link-shared, they can use the album's [`productUrl`](https://developers.google.com/photos/library/reference/rest/v1/albums#Album) to access it instead. A `shareableUrl` is invalidated if the owner turns off link sharing in the Google Photos app, or if the album is unshared.
    #[serde(rename="shareableUrl")]
    
    pub shareable_url: Option<String>,
    /// Options that control whether someone can add media items to, or comment on a shared album.
    #[serde(rename="sharedAlbumOptions")]
    
    pub shared_album_options: Option<SharedAlbumOptions>,
}

impl client::Part for ShareInfo {}


/// Options that control the sharing of an album.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SharedAlbumOptions {
    /// True if the shared album allows collaborators (users who have joined the album) to add media items to it. Defaults to false.
    #[serde(rename="isCollaborative")]
    
    pub is_collaborative: Option<bool>,
    /// True if the shared album allows collaborators (users who have joined the album) to add comments to the album. Defaults to false.
    #[serde(rename="isCommentable")]
    
    pub is_commentable: Option<bool>,
}

impl client::Part for SharedAlbumOptions {}


/// A simple media item to be created in Google Photos via an upload token.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SimpleMediaItem {
    /// File name with extension of the media item. This is shown to the user in Google Photos. The file name specified during the byte upload process is ignored if this field is set. The file name, including the file extension, shouldn't be more than 255 characters. This is an optional field.
    #[serde(rename="fileName")]
    
    pub file_name: Option<String>,
    /// Token identifying the media bytes that have been uploaded to Google.
    #[serde(rename="uploadToken")]
    
    pub upload_token: Option<String>,
}

impl client::Part for SimpleMediaItem {}


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


/// An enrichment containing text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TextEnrichment {
    /// Text for this enrichment item.
    
    pub text: Option<String>,
}

impl client::Part for TextEnrichment {}


/// Request to unshare a shared album in Google Photos.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [unshare albums](AlbumUnshareCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UnshareAlbumRequest { _never_set: Option<bool> }

impl client::RequestValue for UnshareAlbumRequest {}


/// Response of a successful unshare of a shared album.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [unshare albums](AlbumUnshareCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UnshareAlbumResponse { _never_set: Option<bool> }

impl client::ResponseResult for UnshareAlbumResponse {}


/// Metadata that is specific to a video, for example, fps and processing status. Some of these fields may be null or not included.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Video {
    /// Brand of the camera with which the video was taken.
    #[serde(rename="cameraMake")]
    
    pub camera_make: Option<String>,
    /// Model of the camera with which the video was taken.
    #[serde(rename="cameraModel")]
    
    pub camera_model: Option<String>,
    /// Frame rate of the video.
    
    pub fps: Option<f64>,
    /// Processing status of the video.
    
    pub status: Option<VideoStatusEnum>,
}

impl client::Part for Video {}


