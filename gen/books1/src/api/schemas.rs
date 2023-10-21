use super::*;
/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [annotations insert mylibrary](MylibraryAnnotationInsertCall) (request|response)
/// * [annotations update mylibrary](MylibraryAnnotationUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Annotation {
    /// Anchor text after excerpt. For requests, if the user bookmarked a screen that has no flowing text on it, then this field should be empty.
    #[serde(rename="afterSelectedText")]
    
    pub after_selected_text: Option<String>,
    /// Anchor text before excerpt. For requests, if the user bookmarked a screen that has no flowing text on it, then this field should be empty.
    #[serde(rename="beforeSelectedText")]
    
    pub before_selected_text: Option<String>,
    /// Selection ranges sent from the client.
    #[serde(rename="clientVersionRanges")]
    
    pub client_version_ranges: Option<AnnotationClientVersionRanges>,
    /// Timestamp for the created time of this annotation.
    
    pub created: Option<String>,
    /// Selection ranges for the most recent content version.
    #[serde(rename="currentVersionRanges")]
    
    pub current_version_ranges: Option<AnnotationCurrentVersionRanges>,
    /// User-created data for this annotation.
    
    pub data: Option<String>,
    /// Indicates that this annotation is deleted.
    
    pub deleted: Option<bool>,
    /// The highlight style for this annotation.
    #[serde(rename="highlightStyle")]
    
    pub highlight_style: Option<String>,
    /// Id of this annotation, in the form of a GUID.
    
    pub id: Option<String>,
    /// Resource type.
    
    pub kind: Option<String>,
    /// The layer this annotation is for.
    #[serde(rename="layerId")]
    
    pub layer_id: Option<String>,
    /// no description provided
    #[serde(rename="layerSummary")]
    
    pub layer_summary: Option<AnnotationLayerSummary>,
    /// Pages that this annotation spans.
    #[serde(rename="pageIds")]
    
    pub page_ids: Option<Vec<String>>,
    /// Excerpt from the volume.
    #[serde(rename="selectedText")]
    
    pub selected_text: Option<String>,
    /// URL to this resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Timestamp for the last time this annotation was modified.
    
    pub updated: Option<String>,
    /// The volume that this annotation belongs to.
    #[serde(rename="volumeId")]
    
    pub volume_id: Option<String>,
}

impl client::RequestValue for Annotation {}
impl client::ResponseResult for Annotation {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [annotations list mylibrary](MylibraryAnnotationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Annotations {
    /// A list of annotations.
    
    pub items: Option<Vec<Annotation>>,
    /// Resource type.
    
    pub kind: Option<String>,
    /// Token to pass in for pagination for the next page. This will not be present if this request does not have more results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Total number of annotations found. This may be greater than the number of notes returned in this response if results have been paginated.
    #[serde(rename="totalItems")]
    
    pub total_items: Option<i32>,
}

impl client::ResponseResult for Annotations {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [annotations summary mylibrary](MylibraryAnnotationSummaryCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnnotationsSummary {
    /// no description provided
    
    pub kind: Option<String>,
    /// no description provided
    
    pub layers: Option<Vec<AnnotationsSummaryLayers>>,
}

impl client::ResponseResult for AnnotationsSummary {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [annotation data list layers](LayerAnnotationDataListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Annotationsdata {
    /// A list of Annotation Data.
    
    pub items: Option<Vec<GeoAnnotationdata>>,
    /// Resource type
    
    pub kind: Option<String>,
    /// Token to pass in for pagination for the next page. This will not be present if this request does not have more results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The total number of volume annotations found.
    #[serde(rename="totalItems")]
    
    pub total_items: Option<i32>,
}

impl client::ResponseResult for Annotationsdata {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BooksAnnotationsRange {
    /// The offset from the ending position.
    #[serde(rename="endOffset")]
    
    pub end_offset: Option<String>,
    /// The ending position for the range.
    #[serde(rename="endPosition")]
    
    pub end_position: Option<String>,
    /// The offset from the starting position.
    #[serde(rename="startOffset")]
    
    pub start_offset: Option<String>,
    /// The starting position for the range.
    #[serde(rename="startPosition")]
    
    pub start_position: Option<String>,
}

impl client::Part for BooksAnnotationsRange {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [add book cloudloading](CloudloadingAddBookCall) (response)
/// * [update book cloudloading](CloudloadingUpdateBookCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BooksCloudloadingResource {
    /// no description provided
    
    pub author: Option<String>,
    /// no description provided
    #[serde(rename="processingState")]
    
    pub processing_state: Option<String>,
    /// no description provided
    
    pub title: Option<String>,
    /// no description provided
    #[serde(rename="volumeId")]
    
    pub volume_id: Option<String>,
}

impl client::RequestValue for BooksCloudloadingResource {}
impl client::ResponseResult for BooksCloudloadingResource {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [recommended rate volumes](VolumeRecommendedRateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BooksVolumesRecommendedRateResponse {
    /// no description provided
    
    pub consistency_token: Option<String>,
}

impl client::ResponseResult for BooksVolumesRecommendedRateResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get bookshelves](BookshelfGetCall) (response)
/// * [bookshelves get mylibrary](MylibraryBookshelfGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Bookshelf {
    /// Whether this bookshelf is PUBLIC or PRIVATE.
    
    pub access: Option<String>,
    /// Created time for this bookshelf (formatted UTC timestamp with millisecond resolution).
    
    pub created: Option<String>,
    /// Description of this bookshelf.
    
    pub description: Option<String>,
    /// Id of this bookshelf, only unique by user.
    
    pub id: Option<i32>,
    /// Resource type for bookshelf metadata.
    
    pub kind: Option<String>,
    /// URL to this resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Title of this bookshelf.
    
    pub title: Option<String>,
    /// Last modified time of this bookshelf (formatted UTC timestamp with millisecond resolution).
    
    pub updated: Option<String>,
    /// Number of volumes in this bookshelf.
    #[serde(rename="volumeCount")]
    
    pub volume_count: Option<i32>,
    /// Last time a volume was added or removed from this bookshelf (formatted UTC timestamp with millisecond resolution).
    #[serde(rename="volumesLastUpdated")]
    
    pub volumes_last_updated: Option<String>,
}

impl client::ResponseResult for Bookshelf {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list bookshelves](BookshelfListCall) (response)
/// * [bookshelves list mylibrary](MylibraryBookshelfListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Bookshelves {
    /// A list of bookshelves.
    
    pub items: Option<Vec<Bookshelf>>,
    /// Resource type.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for Bookshelves {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list categories onboarding](OnboardingListCategoryCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Category {
    /// A list of onboarding categories.
    
    pub items: Option<Vec<CategoryItems>>,
    /// Resource type.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for Category {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConcurrentAccessRestriction {
    /// Whether access is granted for this (user, device, volume).
    #[serde(rename="deviceAllowed")]
    
    pub device_allowed: Option<bool>,
    /// Resource type.
    
    pub kind: Option<String>,
    /// The maximum number of concurrent access licenses for this volume.
    #[serde(rename="maxConcurrentDevices")]
    
    pub max_concurrent_devices: Option<i32>,
    /// Error/warning message.
    
    pub message: Option<String>,
    /// Client nonce for verification. Download access and client-validation only.
    
    pub nonce: Option<String>,
    /// Error/warning reason code.
    #[serde(rename="reasonCode")]
    
    pub reason_code: Option<String>,
    /// Whether this volume has any concurrent access restrictions.
    
    pub restricted: Option<bool>,
    /// Response signature.
    
    pub signature: Option<String>,
    /// Client app identifier for verification. Download access and client-validation only.
    
    pub source: Option<String>,
    /// Time in seconds for license auto-expiration.
    #[serde(rename="timeWindowSeconds")]
    
    pub time_window_seconds: Option<i32>,
    /// Identifies the volume for which this entry applies.
    #[serde(rename="volumeId")]
    
    pub volume_id: Option<String>,
}

impl client::Part for ConcurrentAccessRestriction {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [annotation data get layers](LayerAnnotationDataGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DictionaryAnnotationdata {
    /// The type of annotation this data is for.
    #[serde(rename="annotationType")]
    
    pub annotation_type: Option<String>,
    /// JSON encoded data for this dictionary annotation data. Emitted with name 'data' in JSON output. Either this or geo_data will be populated.
    
    pub data: Option<Dictlayerdata>,
    /// Base64 encoded data for this annotation data.
    #[serde(rename="encodedData")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub encoded_data: Option<Vec<u8>>,
    /// Unique id for this annotation data.
    
    pub id: Option<String>,
    /// Resource Type
    
    pub kind: Option<String>,
    /// The Layer id for this data. *
    #[serde(rename="layerId")]
    
    pub layer_id: Option<String>,
    /// URL for this resource. *
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Timestamp for the last time this data was updated. (RFC 3339 UTC date-time format).
    
    pub updated: Option<String>,
    /// The volume id for this data. *
    #[serde(rename="volumeId")]
    
    pub volume_id: Option<String>,
}

impl client::ResponseResult for DictionaryAnnotationdata {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Dictlayerdata {
    /// no description provided
    
    pub common: Option<DictlayerdataCommon>,
    /// no description provided
    
    pub dict: Option<DictlayerdataDict>,
    /// no description provided
    
    pub kind: Option<String>,
}

impl client::Part for Dictlayerdata {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get personalizedstream](PersonalizedstreamGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Discoveryclusters {
    /// no description provided
    
    pub clusters: Option<Vec<DiscoveryclustersClusters>>,
    /// Resorce type.
    
    pub kind: Option<String>,
    /// no description provided
    #[serde(rename="totalClusters")]
    
    pub total_clusters: Option<i32>,
}

impl client::ResponseResult for Discoveryclusters {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DownloadAccessRestriction {
    /// If restricted, whether access is granted for this (user, device, volume).
    #[serde(rename="deviceAllowed")]
    
    pub device_allowed: Option<bool>,
    /// If restricted, the number of content download licenses already acquired (including the requesting client, if licensed).
    #[serde(rename="downloadsAcquired")]
    
    pub downloads_acquired: Option<i32>,
    /// If deviceAllowed, whether access was just acquired with this request.
    #[serde(rename="justAcquired")]
    
    pub just_acquired: Option<bool>,
    /// Resource type.
    
    pub kind: Option<String>,
    /// If restricted, the maximum number of content download licenses for this volume.
    #[serde(rename="maxDownloadDevices")]
    
    pub max_download_devices: Option<i32>,
    /// Error/warning message.
    
    pub message: Option<String>,
    /// Client nonce for verification. Download access and client-validation only.
    
    pub nonce: Option<String>,
    /// Error/warning reason code. Additional codes may be added in the future. 0 OK 100 ACCESS_DENIED_PUBLISHER_LIMIT 101 ACCESS_DENIED_LIMIT 200 WARNING_USED_LAST_ACCESS
    #[serde(rename="reasonCode")]
    
    pub reason_code: Option<String>,
    /// Whether this volume has any download access restrictions.
    
    pub restricted: Option<bool>,
    /// Response signature.
    
    pub signature: Option<String>,
    /// Client app identifier for verification. Download access and client-validation only.
    
    pub source: Option<String>,
    /// Identifies the volume for which this entry applies.
    #[serde(rename="volumeId")]
    
    pub volume_id: Option<String>,
}

impl client::Part for DownloadAccessRestriction {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [release download access myconfig](MyconfigReleaseDownloadAccesCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DownloadAccesses {
    /// A list of download access responses.
    #[serde(rename="downloadAccessList")]
    
    pub download_access_list: Option<Vec<DownloadAccessRestriction>>,
    /// Resource type.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for DownloadAccesses {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete book cloudloading](CloudloadingDeleteBookCall) (response)
/// * [share familysharing](FamilysharingShareCall) (response)
/// * [unshare familysharing](FamilysharingUnshareCall) (response)
/// * [annotations delete mylibrary](MylibraryAnnotationDeleteCall) (response)
/// * [bookshelves add volume mylibrary](MylibraryBookshelfAddVolumeCall) (response)
/// * [bookshelves clear volumes mylibrary](MylibraryBookshelfClearVolumeCall) (response)
/// * [bookshelves move volume mylibrary](MylibraryBookshelfMoveVolumeCall) (response)
/// * [bookshelves remove volume mylibrary](MylibraryBookshelfRemoveVolumeCall) (response)
/// * [readingpositions set position mylibrary](MylibraryReadingpositionSetPositionCall) (response)
/// * [accept promooffer](PromoofferAcceptCall) (response)
/// * [dismiss promooffer](PromoofferDismisCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get family info familysharing](FamilysharingGetFamilyInfoCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FamilyInfo {
    /// Resource type.
    
    pub kind: Option<String>,
    /// Family membership info of the user that made the request.
    
    pub membership: Option<FamilyInfoMembership>,
}

impl client::ResponseResult for FamilyInfo {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GeoAnnotationdata {
    /// The type of annotation this data is for.
    #[serde(rename="annotationType")]
    
    pub annotation_type: Option<String>,
    /// JSON encoded data for this geo annotation data. Emitted with name 'data' in JSON output. Either this or dict_data will be populated.
    
    pub data: Option<Geolayerdata>,
    /// Base64 encoded data for this annotation data.
    #[serde(rename="encodedData")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub encoded_data: Option<Vec<u8>>,
    /// Unique id for this annotation data.
    
    pub id: Option<String>,
    /// Resource Type
    
    pub kind: Option<String>,
    /// The Layer id for this data. *
    #[serde(rename="layerId")]
    
    pub layer_id: Option<String>,
    /// URL for this resource. *
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Timestamp for the last time this data was updated. (RFC 3339 UTC date-time format).
    
    pub updated: Option<String>,
    /// The volume id for this data. *
    #[serde(rename="volumeId")]
    
    pub volume_id: Option<String>,
}

impl client::Part for GeoAnnotationdata {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Geolayerdata {
    /// no description provided
    
    pub common: Option<GeolayerdataCommon>,
    /// no description provided
    
    pub geo: Option<GeolayerdataGeo>,
    /// no description provided
    
    pub kind: Option<String>,
}

impl client::Part for Geolayerdata {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list layers](LayerListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Layersummaries {
    /// A list of layer summary items.
    
    pub items: Option<Vec<Layersummary>>,
    /// Resource type.
    
    pub kind: Option<String>,
    /// The total number of layer summaries found.
    #[serde(rename="totalItems")]
    
    pub total_items: Option<i32>,
}

impl client::ResponseResult for Layersummaries {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get layers](LayerGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Layersummary {
    /// The number of annotations for this layer.
    #[serde(rename="annotationCount")]
    
    pub annotation_count: Option<i32>,
    /// The list of annotation types contained for this layer.
    #[serde(rename="annotationTypes")]
    
    pub annotation_types: Option<Vec<String>>,
    /// Link to get data for this annotation.
    #[serde(rename="annotationsDataLink")]
    
    pub annotations_data_link: Option<String>,
    /// The link to get the annotations for this layer.
    #[serde(rename="annotationsLink")]
    
    pub annotations_link: Option<String>,
    /// The content version this resource is for.
    #[serde(rename="contentVersion")]
    
    pub content_version: Option<String>,
    /// The number of data items for this layer.
    #[serde(rename="dataCount")]
    
    pub data_count: Option<i32>,
    /// Unique id of this layer summary.
    
    pub id: Option<String>,
    /// Resource Type
    
    pub kind: Option<String>,
    /// The layer id for this summary.
    #[serde(rename="layerId")]
    
    pub layer_id: Option<String>,
    /// URL to this resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Timestamp for the last time an item in this layer was updated. (RFC 3339 UTC date-time format).
    
    pub updated: Option<String>,
    /// The current version of this layer's volume annotations. Note that this version applies only to the data in the books.layers.volumeAnnotations.* responses. The actual annotation data is versioned separately.
    #[serde(rename="volumeAnnotationsVersion")]
    
    pub volume_annotations_version: Option<String>,
    /// The volume id this resource is for.
    #[serde(rename="volumeId")]
    
    pub volume_id: Option<String>,
}

impl client::ResponseResult for Layersummary {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list offline metadata dictionary](DictionaryListOfflineMetadataCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    /// A list of offline dictionary metadata.
    
    pub items: Option<Vec<MetadataItems>>,
    /// Resource type.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for Metadata {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get notification](NotificationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Notification {
    /// no description provided
    
    pub body: Option<String>,
    /// The list of crm experiment ids.
    #[serde(rename="crmExperimentIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub crm_experiment_ids: Option<Vec<i64>>,
    /// no description provided
    
    pub doc_id: Option<String>,
    /// no description provided
    
    pub doc_type: Option<String>,
    /// no description provided
    
    pub dont_show_notification: Option<bool>,
    /// no description provided
    #[serde(rename="iconUrl")]
    
    pub icon_url: Option<String>,
    /// no description provided
    
    pub is_document_mature: Option<bool>,
    /// Resource type.
    
    pub kind: Option<String>,
    /// no description provided
    #[serde(rename="notificationGroup")]
    
    pub notification_group: Option<String>,
    /// no description provided
    
    pub notification_type: Option<String>,
    /// no description provided
    
    pub pcampaign_id: Option<String>,
    /// no description provided
    
    pub reason: Option<String>,
    /// no description provided
    
    pub show_notification_settings_action: Option<bool>,
    /// no description provided
    #[serde(rename="targetUrl")]
    
    pub target_url: Option<String>,
    /// no description provided
    #[serde(rename="timeToExpireMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub time_to_expire_ms: Option<i64>,
    /// no description provided
    
    pub title: Option<String>,
}

impl client::ResponseResult for Notification {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get promooffer](PromoofferGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Offers {
    /// A list of offers.
    
    pub items: Option<Vec<OffersItems>>,
    /// Resource type.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for Offers {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [readingpositions get mylibrary](MylibraryReadingpositionGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReadingPosition {
    /// Position in an EPUB as a CFI.
    #[serde(rename="epubCfiPosition")]
    
    pub epub_cfi_position: Option<String>,
    /// Position in a volume for image-based content.
    #[serde(rename="gbImagePosition")]
    
    pub gb_image_position: Option<String>,
    /// Position in a volume for text-based content.
    #[serde(rename="gbTextPosition")]
    
    pub gb_text_position: Option<String>,
    /// Resource type for a reading position.
    
    pub kind: Option<String>,
    /// Position in a PDF file.
    #[serde(rename="pdfPosition")]
    
    pub pdf_position: Option<String>,
    /// Timestamp when this reading position was last updated (formatted UTC timestamp with millisecond resolution).
    
    pub updated: Option<String>,
    /// Volume id associated with this reading position.
    #[serde(rename="volumeId")]
    
    pub volume_id: Option<String>,
}

impl client::ResponseResult for ReadingPosition {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [request access myconfig](MyconfigRequestAccesCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RequestAccessData {
    /// A concurrent access response.
    #[serde(rename="concurrentAccess")]
    
    pub concurrent_access: Option<ConcurrentAccessRestriction>,
    /// A download access response.
    #[serde(rename="downloadAccess")]
    
    pub download_access: Option<DownloadAccessRestriction>,
    /// Resource type.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for RequestAccessData {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Review {
    /// Author of this review.
    
    pub author: Option<ReviewAuthor>,
    /// Review text.
    
    pub content: Option<String>,
    /// Date of this review.
    
    pub date: Option<String>,
    /// URL for the full review text, for reviews gathered from the web.
    #[serde(rename="fullTextUrl")]
    
    pub full_text_url: Option<String>,
    /// Resource type for a review.
    
    pub kind: Option<String>,
    /// Star rating for this review. Possible values are ONE, TWO, THREE, FOUR, FIVE or NOT_RATED.
    
    pub rating: Option<String>,
    /// Information regarding the source of this review, when the review is not from a Google Books user.
    
    pub source: Option<ReviewSource>,
    /// Title for this review.
    
    pub title: Option<String>,
    /// Source type for this review. Possible values are EDITORIAL, WEB_USER or GOOGLE_USER.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Volume that this review is for.
    #[serde(rename="volumeId")]
    
    pub volume_id: Option<String>,
}

impl client::Part for Review {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get series](SeriesGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Series {
    /// Resource type.
    
    pub kind: Option<String>,
    /// no description provided
    
    pub series: Option<Vec<SeriesSeries>>,
}

impl client::ResponseResult for Series {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [membership get series](SeriesMembershipGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Seriesmembership {
    /// Resorce type.
    
    pub kind: Option<String>,
    /// no description provided
    
    pub member: Option<Vec<Volume>>,
    /// no description provided
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for Seriesmembership {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get user settings myconfig](MyconfigGetUserSettingCall) (response)
/// * [update user settings myconfig](MyconfigUpdateUserSettingCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Usersettings {
    /// Resource type.
    
    pub kind: Option<String>,
    /// User settings in sub-objects, each for different purposes.
    #[serde(rename="notesExport")]
    
    pub notes_export: Option<UsersettingsNotesExport>,
    /// no description provided
    
    pub notification: Option<UsersettingsNotification>,
}

impl client::RequestValue for Usersettings {}
impl client::ResponseResult for Usersettings {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [associated list volumes](VolumeAssociatedListCall) (none)
/// * [mybooks list volumes](VolumeMybookListCall) (none)
/// * [recommended list volumes](VolumeRecommendedListCall) (none)
/// * [recommended rate volumes](VolumeRecommendedRateCall) (none)
/// * [useruploaded list volumes](VolumeUseruploadedListCall) (none)
/// * [get volumes](VolumeGetCall) (response)
/// * [list volumes](VolumeListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Volume {
    /// Any information about a volume related to reading or obtaining that volume text. This information can depend on country (books may be public domain in one country but not in another, e.g.).
    #[serde(rename="accessInfo")]
    
    pub access_info: Option<VolumeAccessInfo>,
    /// Opaque identifier for a specific version of a volume resource. (In LITE projection)
    
    pub etag: Option<String>,
    /// Unique identifier for a volume. (In LITE projection.)
    
    pub id: Option<String>,
    /// Resource type for a volume. (In LITE projection.)
    
    pub kind: Option<String>,
    /// What layers exist in this volume and high level information about them.
    #[serde(rename="layerInfo")]
    
    pub layer_info: Option<VolumeLayerInfo>,
    /// Recommendation related information for this volume.
    #[serde(rename="recommendedInfo")]
    
    pub recommended_info: Option<VolumeRecommendedInfo>,
    /// Any information about a volume related to the eBookstore and/or purchaseability. This information can depend on the country where the request originates from (i.e. books may not be for sale in certain countries).
    #[serde(rename="saleInfo")]
    
    pub sale_info: Option<VolumeSaleInfo>,
    /// Search result information related to this volume.
    #[serde(rename="searchInfo")]
    
    pub search_info: Option<VolumeSearchInfo>,
    /// URL to this resource. (In LITE projection.)
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// User specific information related to this volume. (e.g. page this user last read or whether they purchased this book)
    #[serde(rename="userInfo")]
    
    pub user_info: Option<VolumeUserInfo>,
    /// General volume information.
    #[serde(rename="volumeInfo")]
    
    pub volume_info: Option<VolumeVolumeInfo>,
}

impl client::Resource for Volume {}
impl client::ResponseResult for Volume {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list category volumes onboarding](OnboardingListCategoryVolumeCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Volume2 {
    /// A list of volumes.
    
    pub items: Option<Vec<Volume>>,
    /// Resource type.
    
    pub kind: Option<String>,
    /// no description provided
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for Volume2 {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [volume annotations get layers](LayerVolumeAnnotationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Volumeannotation {
    /// The annotation data id for this volume annotation.
    #[serde(rename="annotationDataId")]
    
    pub annotation_data_id: Option<String>,
    /// Link to get data for this annotation.
    #[serde(rename="annotationDataLink")]
    
    pub annotation_data_link: Option<String>,
    /// The type of annotation this is.
    #[serde(rename="annotationType")]
    
    pub annotation_type: Option<String>,
    /// The content ranges to identify the selected text.
    #[serde(rename="contentRanges")]
    
    pub content_ranges: Option<VolumeannotationContentRanges>,
    /// Data for this annotation.
    
    pub data: Option<String>,
    /// Indicates that this annotation is deleted.
    
    pub deleted: Option<bool>,
    /// Unique id of this volume annotation.
    
    pub id: Option<String>,
    /// Resource Type
    
    pub kind: Option<String>,
    /// The Layer this annotation is for.
    #[serde(rename="layerId")]
    
    pub layer_id: Option<String>,
    /// Pages the annotation spans.
    #[serde(rename="pageIds")]
    
    pub page_ids: Option<Vec<String>>,
    /// Excerpt from the volume.
    #[serde(rename="selectedText")]
    
    pub selected_text: Option<String>,
    /// URL to this resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Timestamp for the last time this anntoation was updated. (RFC 3339 UTC date-time format).
    
    pub updated: Option<String>,
    /// The Volume this annotation is for.
    #[serde(rename="volumeId")]
    
    pub volume_id: Option<String>,
}

impl client::ResponseResult for Volumeannotation {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [volume annotations list layers](LayerVolumeAnnotationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Volumeannotations {
    /// A list of volume annotations.
    
    pub items: Option<Vec<Volumeannotation>>,
    /// Resource type
    
    pub kind: Option<String>,
    /// Token to pass in for pagination for the next page. This will not be present if this request does not have more results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The total number of volume annotations found.
    #[serde(rename="totalItems")]
    
    pub total_items: Option<i32>,
    /// The version string for all of the volume annotations in this layer (not just the ones in this response). Note: the version string doesn't apply to the annotation data, just the information in this response (e.g. the location of annotations in the book).
    
    pub version: Option<String>,
}

impl client::ResponseResult for Volumeannotations {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [volumes list bookshelves](BookshelfVolumeListCall) (response)
/// * [sync volume licenses myconfig](MyconfigSyncVolumeLicenseCall) (response)
/// * [bookshelves volumes list mylibrary](MylibraryBookshelfVolumeListCall) (response)
/// * [associated list volumes](VolumeAssociatedListCall) (response)
/// * [mybooks list volumes](VolumeMybookListCall) (response)
/// * [recommended list volumes](VolumeRecommendedListCall) (response)
/// * [useruploaded list volumes](VolumeUseruploadedListCall) (response)
/// * [list volumes](VolumeListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Volumes {
    /// A list of volumes.
    
    pub items: Option<Vec<Volume>>,
    /// Resource type.
    
    pub kind: Option<String>,
    /// Total number of volumes found. This might be greater than the number of volumes returned in this response if results have been paginated.
    #[serde(rename="totalItems")]
    
    pub total_items: Option<i32>,
}

impl client::ResponseResult for Volumes {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Volumeseriesinfo {
    /// The display number string. This should be used only for display purposes and the actual sequence should be inferred from the below orderNumber.
    #[serde(rename="bookDisplayNumber")]
    
    pub book_display_number: Option<String>,
    /// Resource type.
    
    pub kind: Option<String>,
    /// Short book title in the context of the series.
    #[serde(rename="shortSeriesBookTitle")]
    
    pub short_series_book_title: Option<String>,
    /// no description provided
    #[serde(rename="volumeSeries")]
    
    pub volume_series: Option<Vec<VolumeseriesinfoVolumeSeries>>,
}

impl client::Part for Volumeseriesinfo {}


/// Selection ranges sent from the client.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnnotationClientVersionRanges {
    /// Range in CFI format for this annotation sent by client.
    #[serde(rename="cfiRange")]
    
    pub cfi_range: Option<BooksAnnotationsRange>,
    /// Content version the client sent in.
    #[serde(rename="contentVersion")]
    
    pub content_version: Option<String>,
    /// Range in GB image format for this annotation sent by client.
    #[serde(rename="gbImageRange")]
    
    pub gb_image_range: Option<BooksAnnotationsRange>,
    /// Range in GB text format for this annotation sent by client.
    #[serde(rename="gbTextRange")]
    
    pub gb_text_range: Option<BooksAnnotationsRange>,
    /// Range in image CFI format for this annotation sent by client.
    #[serde(rename="imageCfiRange")]
    
    pub image_cfi_range: Option<BooksAnnotationsRange>,
}

impl client::NestedType for AnnotationClientVersionRanges {}
impl client::Part for AnnotationClientVersionRanges {}


/// Selection ranges for the most recent content version.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnnotationCurrentVersionRanges {
    /// Range in CFI format for this annotation for version above.
    #[serde(rename="cfiRange")]
    
    pub cfi_range: Option<BooksAnnotationsRange>,
    /// Content version applicable to ranges below.
    #[serde(rename="contentVersion")]
    
    pub content_version: Option<String>,
    /// Range in GB image format for this annotation for version above.
    #[serde(rename="gbImageRange")]
    
    pub gb_image_range: Option<BooksAnnotationsRange>,
    /// Range in GB text format for this annotation for version above.
    #[serde(rename="gbTextRange")]
    
    pub gb_text_range: Option<BooksAnnotationsRange>,
    /// Range in image CFI format for this annotation for version above.
    #[serde(rename="imageCfiRange")]
    
    pub image_cfi_range: Option<BooksAnnotationsRange>,
}

impl client::NestedType for AnnotationCurrentVersionRanges {}
impl client::Part for AnnotationCurrentVersionRanges {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnnotationLayerSummary {
    /// Maximum allowed characters on this layer, especially for the "copy" layer.
    #[serde(rename="allowedCharacterCount")]
    
    pub allowed_character_count: Option<i32>,
    /// Type of limitation on this layer. "limited" or "unlimited" for the "copy" layer.
    #[serde(rename="limitType")]
    
    pub limit_type: Option<String>,
    /// Remaining allowed characters on this layer, especially for the "copy" layer.
    #[serde(rename="remainingCharacterCount")]
    
    pub remaining_character_count: Option<i32>,
}

impl client::NestedType for AnnotationLayerSummary {}
impl client::Part for AnnotationLayerSummary {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnnotationsSummaryLayers {
    /// no description provided
    #[serde(rename="allowedCharacterCount")]
    
    pub allowed_character_count: Option<i32>,
    /// no description provided
    #[serde(rename="layerId")]
    
    pub layer_id: Option<String>,
    /// no description provided
    #[serde(rename="limitType")]
    
    pub limit_type: Option<String>,
    /// no description provided
    #[serde(rename="remainingCharacterCount")]
    
    pub remaining_character_count: Option<i32>,
    /// no description provided
    
    pub updated: Option<String>,
}

impl client::NestedType for AnnotationsSummaryLayers {}
impl client::Part for AnnotationsSummaryLayers {}


/// A list of onboarding categories.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CategoryItems {
    /// no description provided
    #[serde(rename="badgeUrl")]
    
    pub badge_url: Option<String>,
    /// no description provided
    #[serde(rename="categoryId")]
    
    pub category_id: Option<String>,
    /// no description provided
    
    pub name: Option<String>,
}

impl client::NestedType for CategoryItems {}
impl client::Part for CategoryItems {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DictlayerdataCommon {
    /// The display title and localized canonical name to use when searching for this entity on Google search.
    
    pub title: Option<String>,
}

impl client::NestedType for DictlayerdataCommon {}
impl client::Part for DictlayerdataCommon {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DictlayerdataDict {
    /// The source, url and attribution for this dictionary data.
    
    pub source: Option<DictlayerdataDictSource>,
    /// no description provided
    
    pub words: Option<Vec<DictlayerdataDictWords>>,
}

impl client::NestedType for DictlayerdataDict {}
impl client::Part for DictlayerdataDict {}


/// The source, url and attribution for this dictionary data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DictlayerdataDictSource {
    /// no description provided
    
    pub attribution: Option<String>,
    /// no description provided
    
    pub url: Option<String>,
}

impl client::NestedType for DictlayerdataDictSource {}
impl client::Part for DictlayerdataDictSource {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DictlayerdataDictWords {
    /// no description provided
    
    pub derivatives: Option<Vec<DictlayerdataDictWordsDerivatives>>,
    /// no description provided
    
    pub examples: Option<Vec<DictlayerdataDictWordsExamples>>,
    /// no description provided
    
    pub senses: Option<Vec<DictlayerdataDictWordsSenses>>,
    /// The words with different meanings but not related words, e.g. "go" (game) and "go" (verb).
    
    pub source: Option<DictlayerdataDictWordsSource>,
}

impl client::NestedType for DictlayerdataDictWords {}
impl client::Part for DictlayerdataDictWords {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DictlayerdataDictWordsDerivatives {
    /// no description provided
    
    pub source: Option<DictlayerdataDictWordsDerivativesSource>,
    /// no description provided
    
    pub text: Option<String>,
}

impl client::NestedType for DictlayerdataDictWordsDerivatives {}
impl client::Part for DictlayerdataDictWordsDerivatives {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DictlayerdataDictWordsDerivativesSource {
    /// no description provided
    
    pub attribution: Option<String>,
    /// no description provided
    
    pub url: Option<String>,
}

impl client::NestedType for DictlayerdataDictWordsDerivativesSource {}
impl client::Part for DictlayerdataDictWordsDerivativesSource {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DictlayerdataDictWordsExamples {
    /// no description provided
    
    pub source: Option<DictlayerdataDictWordsExamplesSource>,
    /// no description provided
    
    pub text: Option<String>,
}

impl client::NestedType for DictlayerdataDictWordsExamples {}
impl client::Part for DictlayerdataDictWordsExamples {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DictlayerdataDictWordsExamplesSource {
    /// no description provided
    
    pub attribution: Option<String>,
    /// no description provided
    
    pub url: Option<String>,
}

impl client::NestedType for DictlayerdataDictWordsExamplesSource {}
impl client::Part for DictlayerdataDictWordsExamplesSource {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DictlayerdataDictWordsSenses {
    /// no description provided
    
    pub conjugations: Option<Vec<DictlayerdataDictWordsSensesConjugations>>,
    /// no description provided
    
    pub definitions: Option<Vec<DictlayerdataDictWordsSensesDefinitions>>,
    /// no description provided
    #[serde(rename="partOfSpeech")]
    
    pub part_of_speech: Option<String>,
    /// no description provided
    
    pub pronunciation: Option<String>,
    /// no description provided
    #[serde(rename="pronunciationUrl")]
    
    pub pronunciation_url: Option<String>,
    /// no description provided
    
    pub source: Option<DictlayerdataDictWordsSensesSource>,
    /// no description provided
    
    pub syllabification: Option<String>,
    /// no description provided
    
    pub synonyms: Option<Vec<DictlayerdataDictWordsSensesSynonyms>>,
}

impl client::NestedType for DictlayerdataDictWordsSenses {}
impl client::Part for DictlayerdataDictWordsSenses {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DictlayerdataDictWordsSensesConjugations {
    /// no description provided
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// no description provided
    
    pub value: Option<String>,
}

impl client::NestedType for DictlayerdataDictWordsSensesConjugations {}
impl client::Part for DictlayerdataDictWordsSensesConjugations {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DictlayerdataDictWordsSensesDefinitions {
    /// no description provided
    
    pub definition: Option<String>,
    /// no description provided
    
    pub examples: Option<Vec<DictlayerdataDictWordsSensesDefinitionsExamples>>,
}

impl client::NestedType for DictlayerdataDictWordsSensesDefinitions {}
impl client::Part for DictlayerdataDictWordsSensesDefinitions {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DictlayerdataDictWordsSensesDefinitionsExamples {
    /// no description provided
    
    pub source: Option<DictlayerdataDictWordsSensesDefinitionsExamplesSource>,
    /// no description provided
    
    pub text: Option<String>,
}

impl client::NestedType for DictlayerdataDictWordsSensesDefinitionsExamples {}
impl client::Part for DictlayerdataDictWordsSensesDefinitionsExamples {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DictlayerdataDictWordsSensesDefinitionsExamplesSource {
    /// no description provided
    
    pub attribution: Option<String>,
    /// no description provided
    
    pub url: Option<String>,
}

impl client::NestedType for DictlayerdataDictWordsSensesDefinitionsExamplesSource {}
impl client::Part for DictlayerdataDictWordsSensesDefinitionsExamplesSource {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DictlayerdataDictWordsSensesSource {
    /// no description provided
    
    pub attribution: Option<String>,
    /// no description provided
    
    pub url: Option<String>,
}

impl client::NestedType for DictlayerdataDictWordsSensesSource {}
impl client::Part for DictlayerdataDictWordsSensesSource {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DictlayerdataDictWordsSensesSynonyms {
    /// no description provided
    
    pub source: Option<DictlayerdataDictWordsSensesSynonymsSource>,
    /// no description provided
    
    pub text: Option<String>,
}

impl client::NestedType for DictlayerdataDictWordsSensesSynonyms {}
impl client::Part for DictlayerdataDictWordsSensesSynonyms {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DictlayerdataDictWordsSensesSynonymsSource {
    /// no description provided
    
    pub attribution: Option<String>,
    /// no description provided
    
    pub url: Option<String>,
}

impl client::NestedType for DictlayerdataDictWordsSensesSynonymsSource {}
impl client::Part for DictlayerdataDictWordsSensesSynonymsSource {}


/// The words with different meanings but not related words, e.g. "go" (game) and "go" (verb).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DictlayerdataDictWordsSource {
    /// no description provided
    
    pub attribution: Option<String>,
    /// no description provided
    
    pub url: Option<String>,
}

impl client::NestedType for DictlayerdataDictWordsSource {}
impl client::Part for DictlayerdataDictWordsSource {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DiscoveryclustersClusters {
    /// no description provided
    
    pub banner_with_content_container: Option<DiscoveryclustersClustersBannerWithContentContainer>,
    /// no description provided
    #[serde(rename="subTitle")]
    
    pub sub_title: Option<String>,
    /// no description provided
    
    pub title: Option<String>,
    /// no description provided
    #[serde(rename="totalVolumes")]
    
    pub total_volumes: Option<i32>,
    /// no description provided
    
    pub uid: Option<String>,
    /// no description provided
    
    pub volumes: Option<Vec<Volume>>,
}

impl client::NestedType for DiscoveryclustersClusters {}
impl client::Part for DiscoveryclustersClusters {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DiscoveryclustersClustersBannerWithContentContainer {
    /// no description provided
    #[serde(rename="fillColorArgb")]
    
    pub fill_color_argb: Option<String>,
    /// no description provided
    #[serde(rename="imageUrl")]
    
    pub image_url: Option<String>,
    /// no description provided
    #[serde(rename="maskColorArgb")]
    
    pub mask_color_argb: Option<String>,
    /// no description provided
    #[serde(rename="moreButtonText")]
    
    pub more_button_text: Option<String>,
    /// no description provided
    #[serde(rename="moreButtonUrl")]
    
    pub more_button_url: Option<String>,
    /// no description provided
    #[serde(rename="textColorArgb")]
    
    pub text_color_argb: Option<String>,
}

impl client::NestedType for DiscoveryclustersClustersBannerWithContentContainer {}
impl client::Part for DiscoveryclustersClustersBannerWithContentContainer {}


/// Family membership info of the user that made the request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FamilyInfoMembership {
    /// Restrictions on user buying and acquiring content.
    #[serde(rename="acquirePermission")]
    
    pub acquire_permission: Option<String>,
    /// The age group of the user.
    #[serde(rename="ageGroup")]
    
    pub age_group: Option<String>,
    /// The maximum allowed maturity rating for the user.
    #[serde(rename="allowedMaturityRating")]
    
    pub allowed_maturity_rating: Option<String>,
    /// no description provided
    #[serde(rename="isInFamily")]
    
    pub is_in_family: Option<bool>,
    /// The role of the user in the family.
    
    pub role: Option<String>,
}

impl client::NestedType for FamilyInfoMembership {}
impl client::Part for FamilyInfoMembership {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GeolayerdataCommon {
    /// The language of the information url and description.
    
    pub lang: Option<String>,
    /// The URL for the preview image information.
    #[serde(rename="previewImageUrl")]
    
    pub preview_image_url: Option<String>,
    /// The description for this location.
    
    pub snippet: Option<String>,
    /// The URL for information for this location. Ex: wikipedia link.
    #[serde(rename="snippetUrl")]
    
    pub snippet_url: Option<String>,
    /// The display title and localized canonical name to use when searching for this entity on Google search.
    
    pub title: Option<String>,
}

impl client::NestedType for GeolayerdataCommon {}
impl client::Part for GeolayerdataCommon {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GeolayerdataGeo {
    /// The boundary of the location as a set of loops containing pairs of latitude, longitude coordinates.
    
    pub boundary: Option<Vec<String>>,
    /// The cache policy active for this data. EX: UNRESTRICTED, RESTRICTED, NEVER
    #[serde(rename="cachePolicy")]
    
    pub cache_policy: Option<String>,
    /// The country code of the location.
    #[serde(rename="countryCode")]
    
    pub country_code: Option<String>,
    /// The latitude of the location.
    
    pub latitude: Option<f64>,
    /// The longitude of the location.
    
    pub longitude: Option<f64>,
    /// The type of map that should be used for this location. EX: HYBRID, ROADMAP, SATELLITE, TERRAIN
    #[serde(rename="mapType")]
    
    pub map_type: Option<String>,
    /// The viewport for showing this location. This is a latitude, longitude rectangle.
    
    pub viewport: Option<GeolayerdataGeoViewport>,
    /// The Zoom level to use for the map. Zoom levels between 0 (the lowest zoom level, in which the entire world can be seen on one map) to 21+ (down to individual buildings). See: https: //developers.google.com/maps/documentation/staticmaps/#Zoomlevels
    
    pub zoom: Option<i32>,
}

impl client::NestedType for GeolayerdataGeo {}
impl client::Part for GeolayerdataGeo {}


/// The viewport for showing this location. This is a latitude, longitude rectangle.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GeolayerdataGeoViewport {
    /// no description provided
    
    pub hi: Option<GeolayerdataGeoViewportHi>,
    /// no description provided
    
    pub lo: Option<GeolayerdataGeoViewportLo>,
}

impl client::NestedType for GeolayerdataGeoViewport {}
impl client::Part for GeolayerdataGeoViewport {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GeolayerdataGeoViewportHi {
    /// no description provided
    
    pub latitude: Option<f64>,
    /// no description provided
    
    pub longitude: Option<f64>,
}

impl client::NestedType for GeolayerdataGeoViewportHi {}
impl client::Part for GeolayerdataGeoViewportHi {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GeolayerdataGeoViewportLo {
    /// no description provided
    
    pub latitude: Option<f64>,
    /// no description provided
    
    pub longitude: Option<f64>,
}

impl client::NestedType for GeolayerdataGeoViewportLo {}
impl client::Part for GeolayerdataGeoViewportLo {}


/// A list of offline dictionary metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetadataItems {
    /// no description provided
    
    pub download_url: Option<String>,
    /// no description provided
    
    pub encrypted_key: Option<String>,
    /// no description provided
    
    pub language: Option<String>,
    /// no description provided
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub size: Option<i64>,
    /// no description provided
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub version: Option<i64>,
}

impl client::NestedType for MetadataItems {}
impl client::Part for MetadataItems {}


/// A list of offers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OffersItems {
    /// no description provided
    #[serde(rename="artUrl")]
    
    pub art_url: Option<String>,
    /// no description provided
    #[serde(rename="gservicesKey")]
    
    pub gservices_key: Option<String>,
    /// no description provided
    
    pub id: Option<String>,
    /// no description provided
    
    pub items: Option<Vec<OffersItemsItems>>,
}

impl client::NestedType for OffersItems {}
impl client::Part for OffersItems {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OffersItemsItems {
    /// no description provided
    
    pub author: Option<String>,
    /// no description provided
    #[serde(rename="canonicalVolumeLink")]
    
    pub canonical_volume_link: Option<String>,
    /// no description provided
    #[serde(rename="coverUrl")]
    
    pub cover_url: Option<String>,
    /// no description provided
    
    pub description: Option<String>,
    /// no description provided
    
    pub title: Option<String>,
    /// no description provided
    #[serde(rename="volumeId")]
    
    pub volume_id: Option<String>,
}

impl client::NestedType for OffersItemsItems {}
impl client::Part for OffersItemsItems {}


/// Author of this review.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReviewAuthor {
    /// Name of this person.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
}

impl client::NestedType for ReviewAuthor {}
impl client::Part for ReviewAuthor {}


/// Information regarding the source of this review, when the review is not from a Google Books user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReviewSource {
    /// Name of the source.
    
    pub description: Option<String>,
    /// Extra text about the source of the review.
    #[serde(rename="extraDescription")]
    
    pub extra_description: Option<String>,
    /// URL of the source of the review.
    
    pub url: Option<String>,
}

impl client::NestedType for ReviewSource {}
impl client::Part for ReviewSource {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SeriesSeries {
    /// no description provided
    #[serde(rename="bannerImageUrl")]
    
    pub banner_image_url: Option<String>,
    /// no description provided
    #[serde(rename="eligibleForSubscription")]
    
    pub eligible_for_subscription: Option<bool>,
    /// no description provided
    #[serde(rename="imageUrl")]
    
    pub image_url: Option<String>,
    /// no description provided
    #[serde(rename="isComplete")]
    
    pub is_complete: Option<bool>,
    /// no description provided
    #[serde(rename="seriesFormatType")]
    
    pub series_format_type: Option<String>,
    /// no description provided
    #[serde(rename="seriesId")]
    
    pub series_id: Option<String>,
    /// no description provided
    #[serde(rename="seriesSubscriptionReleaseInfo")]
    
    pub series_subscription_release_info: Option<SeriesSeriesSeriesSubscriptionReleaseInfo>,
    /// no description provided
    #[serde(rename="seriesType")]
    
    pub series_type: Option<String>,
    /// no description provided
    #[serde(rename="subscriptionId")]
    
    pub subscription_id: Option<String>,
    /// no description provided
    
    pub title: Option<String>,
}

impl client::NestedType for SeriesSeries {}
impl client::Part for SeriesSeries {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SeriesSeriesSeriesSubscriptionReleaseInfo {
    /// no description provided
    #[serde(rename="cancelTime")]
    
    pub cancel_time: Option<String>,
    /// no description provided
    #[serde(rename="currentReleaseInfo")]
    
    pub current_release_info: Option<SeriesSeriesSeriesSubscriptionReleaseInfoCurrentReleaseInfo>,
    /// no description provided
    #[serde(rename="nextReleaseInfo")]
    
    pub next_release_info: Option<SeriesSeriesSeriesSubscriptionReleaseInfoNextReleaseInfo>,
    /// no description provided
    #[serde(rename="seriesSubscriptionType")]
    
    pub series_subscription_type: Option<String>,
}

impl client::NestedType for SeriesSeriesSeriesSubscriptionReleaseInfo {}
impl client::Part for SeriesSeriesSeriesSubscriptionReleaseInfo {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SeriesSeriesSeriesSubscriptionReleaseInfoCurrentReleaseInfo {
    /// no description provided
    #[serde(rename="amountInMicros")]
    
    pub amount_in_micros: Option<f64>,
    /// no description provided
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// no description provided
    #[serde(rename="releaseNumber")]
    
    pub release_number: Option<String>,
    /// no description provided
    #[serde(rename="releaseTime")]
    
    pub release_time: Option<String>,
}

impl client::NestedType for SeriesSeriesSeriesSubscriptionReleaseInfoCurrentReleaseInfo {}
impl client::Part for SeriesSeriesSeriesSubscriptionReleaseInfoCurrentReleaseInfo {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SeriesSeriesSeriesSubscriptionReleaseInfoNextReleaseInfo {
    /// no description provided
    #[serde(rename="amountInMicros")]
    
    pub amount_in_micros: Option<f64>,
    /// no description provided
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// no description provided
    #[serde(rename="releaseNumber")]
    
    pub release_number: Option<String>,
    /// no description provided
    #[serde(rename="releaseTime")]
    
    pub release_time: Option<String>,
}

impl client::NestedType for SeriesSeriesSeriesSubscriptionReleaseInfoNextReleaseInfo {}
impl client::Part for SeriesSeriesSeriesSubscriptionReleaseInfoNextReleaseInfo {}


/// User settings in sub-objects, each for different purposes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UsersettingsNotesExport {
    /// no description provided
    #[serde(rename="folderName")]
    
    pub folder_name: Option<String>,
    /// no description provided
    #[serde(rename="isEnabled")]
    
    pub is_enabled: Option<bool>,
}

impl client::NestedType for UsersettingsNotesExport {}
impl client::Part for UsersettingsNotesExport {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UsersettingsNotification {
    /// no description provided
    #[serde(rename="matchMyInterests")]
    
    pub match_my_interests: Option<UsersettingsNotificationMatchMyInterests>,
    /// no description provided
    #[serde(rename="moreFromAuthors")]
    
    pub more_from_authors: Option<UsersettingsNotificationMoreFromAuthors>,
    /// no description provided
    #[serde(rename="moreFromSeries")]
    
    pub more_from_series: Option<UsersettingsNotificationMoreFromSeries>,
    /// no description provided
    #[serde(rename="priceDrop")]
    
    pub price_drop: Option<UsersettingsNotificationPriceDrop>,
    /// no description provided
    #[serde(rename="rewardExpirations")]
    
    pub reward_expirations: Option<UsersettingsNotificationRewardExpirations>,
}

impl client::NestedType for UsersettingsNotification {}
impl client::Part for UsersettingsNotification {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UsersettingsNotificationMatchMyInterests {
    /// no description provided
    
    pub opted_state: Option<String>,
}

impl client::NestedType for UsersettingsNotificationMatchMyInterests {}
impl client::Part for UsersettingsNotificationMatchMyInterests {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UsersettingsNotificationMoreFromAuthors {
    /// no description provided
    
    pub opted_state: Option<String>,
}

impl client::NestedType for UsersettingsNotificationMoreFromAuthors {}
impl client::Part for UsersettingsNotificationMoreFromAuthors {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UsersettingsNotificationMoreFromSeries {
    /// no description provided
    
    pub opted_state: Option<String>,
}

impl client::NestedType for UsersettingsNotificationMoreFromSeries {}
impl client::Part for UsersettingsNotificationMoreFromSeries {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UsersettingsNotificationPriceDrop {
    /// no description provided
    
    pub opted_state: Option<String>,
}

impl client::NestedType for UsersettingsNotificationPriceDrop {}
impl client::Part for UsersettingsNotificationPriceDrop {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UsersettingsNotificationRewardExpirations {
    /// no description provided
    
    pub opted_state: Option<String>,
}

impl client::NestedType for UsersettingsNotificationRewardExpirations {}
impl client::Part for UsersettingsNotificationRewardExpirations {}


/// Any information about a volume related to reading or obtaining that volume text. This information can depend on country (books may be public domain in one country but not in another, e.g.).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeAccessInfo {
    /// Combines the access and viewability of this volume into a single status field for this user. Values can be FULL_PURCHASED, FULL_PUBLIC_DOMAIN, SAMPLE or NONE. (In LITE projection.)
    #[serde(rename="accessViewStatus")]
    
    pub access_view_status: Option<String>,
    /// The two-letter ISO_3166-1 country code for which this access information is valid. (In LITE projection.)
    
    pub country: Option<String>,
    /// Information about a volume's download license access restrictions.
    #[serde(rename="downloadAccess")]
    
    pub download_access: Option<DownloadAccessRestriction>,
    /// URL to the Google Drive viewer if this volume is uploaded by the user by selecting the file from Google Drive.
    #[serde(rename="driveImportedContentLink")]
    
    pub drive_imported_content_link: Option<String>,
    /// Whether this volume can be embedded in a viewport using the Embedded Viewer API.
    
    pub embeddable: Option<bool>,
    /// Information about epub content. (In LITE projection.)
    
    pub epub: Option<VolumeAccessInfoEpub>,
    /// Whether this volume requires that the client explicitly request offline download license rather than have it done automatically when loading the content, if the client supports it.
    #[serde(rename="explicitOfflineLicenseManagement")]
    
    pub explicit_offline_license_management: Option<bool>,
    /// Information about pdf content. (In LITE projection.)
    
    pub pdf: Option<VolumeAccessInfoPdf>,
    /// Whether or not this book is public domain in the country listed above.
    #[serde(rename="publicDomain")]
    
    pub public_domain: Option<bool>,
    /// Whether quote sharing is allowed for this volume.
    #[serde(rename="quoteSharingAllowed")]
    
    pub quote_sharing_allowed: Option<bool>,
    /// Whether text-to-speech is permitted for this volume. Values can be ALLOWED, ALLOWED_FOR_ACCESSIBILITY, or NOT_ALLOWED.
    #[serde(rename="textToSpeechPermission")]
    
    pub text_to_speech_permission: Option<String>,
    /// For ordered but not yet processed orders, we give a URL that can be used to go to the appropriate Google Wallet page.
    #[serde(rename="viewOrderUrl")]
    
    pub view_order_url: Option<String>,
    /// The read access of a volume. Possible values are PARTIAL, ALL_PAGES, NO_PAGES or UNKNOWN. This value depends on the country listed above. A value of PARTIAL means that the publisher has allowed some portion of the volume to be viewed publicly, without purchase. This can apply to eBooks as well as non-eBooks. Public domain books will always have a value of ALL_PAGES.
    
    pub viewability: Option<String>,
    /// URL to read this volume on the Google Books site. Link will not allow users to read non-viewable volumes.
    #[serde(rename="webReaderLink")]
    
    pub web_reader_link: Option<String>,
}

impl client::NestedType for VolumeAccessInfo {}
impl client::Part for VolumeAccessInfo {}


/// Information about epub content. (In LITE projection.)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeAccessInfoEpub {
    /// URL to retrieve ACS token for epub download. (In LITE projection.)
    #[serde(rename="acsTokenLink")]
    
    pub acs_token_link: Option<String>,
    /// URL to download epub. (In LITE projection.)
    #[serde(rename="downloadLink")]
    
    pub download_link: Option<String>,
    /// Is a flowing text epub available either as public domain or for purchase. (In LITE projection.)
    #[serde(rename="isAvailable")]
    
    pub is_available: Option<bool>,
}

impl client::NestedType for VolumeAccessInfoEpub {}
impl client::Part for VolumeAccessInfoEpub {}


/// Information about pdf content. (In LITE projection.)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeAccessInfoPdf {
    /// URL to retrieve ACS token for pdf download. (In LITE projection.)
    #[serde(rename="acsTokenLink")]
    
    pub acs_token_link: Option<String>,
    /// URL to download pdf. (In LITE projection.)
    #[serde(rename="downloadLink")]
    
    pub download_link: Option<String>,
    /// Is a scanned image pdf available either as public domain or for purchase. (In LITE projection.)
    #[serde(rename="isAvailable")]
    
    pub is_available: Option<bool>,
}

impl client::NestedType for VolumeAccessInfoPdf {}
impl client::Part for VolumeAccessInfoPdf {}


/// What layers exist in this volume and high level information about them.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeLayerInfo {
    /// A layer should appear here if and only if the layer exists for this book.
    
    pub layers: Option<Vec<VolumeLayerInfoLayers>>,
}

impl client::NestedType for VolumeLayerInfo {}
impl client::Part for VolumeLayerInfo {}


/// A layer should appear here if and only if the layer exists for this book.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeLayerInfoLayers {
    /// The layer id of this layer (e.g. "geo").
    #[serde(rename="layerId")]
    
    pub layer_id: Option<String>,
    /// The current version of this layer's volume annotations. Note that this version applies only to the data in the books.layers.volumeAnnotations.* responses. The actual annotation data is versioned separately.
    #[serde(rename="volumeAnnotationsVersion")]
    
    pub volume_annotations_version: Option<String>,
}

impl client::NestedType for VolumeLayerInfoLayers {}
impl client::Part for VolumeLayerInfoLayers {}


/// Recommendation related information for this volume.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeRecommendedInfo {
    /// A text explaining why this volume is recommended.
    
    pub explanation: Option<String>,
}

impl client::NestedType for VolumeRecommendedInfo {}
impl client::Part for VolumeRecommendedInfo {}


/// Any information about a volume related to the eBookstore and/or purchaseability. This information can depend on the country where the request originates from (i.e. books may not be for sale in certain countries).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeSaleInfo {
    /// URL to purchase this volume on the Google Books site. (In LITE projection)
    #[serde(rename="buyLink")]
    
    pub buy_link: Option<String>,
    /// The two-letter ISO_3166-1 country code for which this sale information is valid. (In LITE projection.)
    
    pub country: Option<String>,
    /// Whether or not this volume is an eBook (can be added to the My eBooks shelf).
    #[serde(rename="isEbook")]
    
    pub is_ebook: Option<bool>,
    /// Suggested retail price. (In LITE projection.)
    #[serde(rename="listPrice")]
    
    pub list_price: Option<VolumeSaleInfoListPrice>,
    /// Offers available for this volume (sales and rentals).
    
    pub offers: Option<Vec<VolumeSaleInfoOffers>>,
    /// The date on which this book is available for sale.
    #[serde(rename="onSaleDate")]
    
    pub on_sale_date: Option<String>,
    /// The actual selling price of the book. This is the same as the suggested retail or list price unless there are offers or discounts on this volume. (In LITE projection.)
    #[serde(rename="retailPrice")]
    
    pub retail_price: Option<VolumeSaleInfoRetailPrice>,
    /// Whether or not this book is available for sale or offered for free in the Google eBookstore for the country listed above. Possible values are FOR_SALE, FOR_RENTAL_ONLY, FOR_SALE_AND_RENTAL, FREE, NOT_FOR_SALE, or FOR_PREORDER.
    
    pub saleability: Option<String>,
}

impl client::NestedType for VolumeSaleInfo {}
impl client::Part for VolumeSaleInfo {}


/// Suggested retail price. (In LITE projection.)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeSaleInfoListPrice {
    /// Amount in the currency listed below. (In LITE projection.)
    
    pub amount: Option<f64>,
    /// An ISO 4217, three-letter currency code. (In LITE projection.)
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
}

impl client::NestedType for VolumeSaleInfoListPrice {}
impl client::Part for VolumeSaleInfoListPrice {}


/// Offers available for this volume (sales and rentals).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeSaleInfoOffers {
    /// The finsky offer type (e.g., PURCHASE=0 RENTAL=3)
    #[serde(rename="finskyOfferType")]
    
    pub finsky_offer_type: Option<i32>,
    /// Indicates whether the offer is giftable.
    
    pub giftable: Option<bool>,
    /// Offer list (=undiscounted) price in Micros.
    #[serde(rename="listPrice")]
    
    pub list_price: Option<VolumeSaleInfoOffersListPrice>,
    /// The rental duration (for rental offers only).
    #[serde(rename="rentalDuration")]
    
    pub rental_duration: Option<VolumeSaleInfoOffersRentalDuration>,
    /// Offer retail (=discounted) price in Micros
    #[serde(rename="retailPrice")]
    
    pub retail_price: Option<VolumeSaleInfoOffersRetailPrice>,
}

impl client::NestedType for VolumeSaleInfoOffers {}
impl client::Part for VolumeSaleInfoOffers {}


/// Offer list (=undiscounted) price in Micros.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeSaleInfoOffersListPrice {
    /// no description provided
    #[serde(rename="amountInMicros")]
    
    pub amount_in_micros: Option<f64>,
    /// no description provided
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
}

impl client::NestedType for VolumeSaleInfoOffersListPrice {}
impl client::Part for VolumeSaleInfoOffersListPrice {}


/// The rental duration (for rental offers only).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeSaleInfoOffersRentalDuration {
    /// no description provided
    
    pub count: Option<f64>,
    /// no description provided
    
    pub unit: Option<String>,
}

impl client::NestedType for VolumeSaleInfoOffersRentalDuration {}
impl client::Part for VolumeSaleInfoOffersRentalDuration {}


/// Offer retail (=discounted) price in Micros
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeSaleInfoOffersRetailPrice {
    /// no description provided
    #[serde(rename="amountInMicros")]
    
    pub amount_in_micros: Option<f64>,
    /// no description provided
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
}

impl client::NestedType for VolumeSaleInfoOffersRetailPrice {}
impl client::Part for VolumeSaleInfoOffersRetailPrice {}


/// The actual selling price of the book. This is the same as the suggested retail or list price unless there are offers or discounts on this volume. (In LITE projection.)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeSaleInfoRetailPrice {
    /// Amount in the currency listed below. (In LITE projection.)
    
    pub amount: Option<f64>,
    /// An ISO 4217, three-letter currency code. (In LITE projection.)
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
}

impl client::NestedType for VolumeSaleInfoRetailPrice {}
impl client::Part for VolumeSaleInfoRetailPrice {}


/// Search result information related to this volume.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeSearchInfo {
    /// A text snippet containing the search query.
    #[serde(rename="textSnippet")]
    
    pub text_snippet: Option<String>,
}

impl client::NestedType for VolumeSearchInfo {}
impl client::Part for VolumeSearchInfo {}


/// User specific information related to this volume. (e.g. page this user last read or whether they purchased this book)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeUserInfo {
    /// Timestamp when this volume was acquired by the user. (RFC 3339 UTC date-time format) Acquiring includes purchase, user upload, receiving family sharing, etc.
    #[serde(rename="acquiredTime")]
    
    pub acquired_time: Option<String>,
    /// How this volume was acquired.
    #[serde(rename="acquisitionType")]
    
    pub acquisition_type: Option<i32>,
    /// Copy/Paste accounting information.
    
    pub copy: Option<VolumeUserInfoCopy>,
    /// Whether this volume is purchased, sample, pd download etc.
    #[serde(rename="entitlementType")]
    
    pub entitlement_type: Option<i32>,
    /// Information on the ability to share with the family.
    #[serde(rename="familySharing")]
    
    pub family_sharing: Option<VolumeUserInfoFamilySharing>,
    /// Whether or not the user shared this volume with the family.
    #[serde(rename="isFamilySharedFromUser")]
    
    pub is_family_shared_from_user: Option<bool>,
    /// Whether or not the user received this volume through family sharing.
    #[serde(rename="isFamilySharedToUser")]
    
    pub is_family_shared_to_user: Option<bool>,
    /// Deprecated: Replaced by familySharing.
    #[serde(rename="isFamilySharingAllowed")]
    
    pub is_family_sharing_allowed: Option<bool>,
    /// Deprecated: Replaced by familySharing.
    #[serde(rename="isFamilySharingDisabledByFop")]
    
    pub is_family_sharing_disabled_by_fop: Option<bool>,
    /// Whether or not this volume is currently in "my books."
    #[serde(rename="isInMyBooks")]
    
    pub is_in_my_books: Option<bool>,
    /// Whether or not this volume was pre-ordered by the authenticated user making the request. (In LITE projection.)
    #[serde(rename="isPreordered")]
    
    pub is_preordered: Option<bool>,
    /// Whether or not this volume was purchased by the authenticated user making the request. (In LITE projection.)
    #[serde(rename="isPurchased")]
    
    pub is_purchased: Option<bool>,
    /// Whether or not this volume was user uploaded.
    #[serde(rename="isUploaded")]
    
    pub is_uploaded: Option<bool>,
    /// The user's current reading position in the volume, if one is available. (In LITE projection.)
    #[serde(rename="readingPosition")]
    
    pub reading_position: Option<ReadingPosition>,
    /// Period during this book is/was a valid rental.
    #[serde(rename="rentalPeriod")]
    
    pub rental_period: Option<VolumeUserInfoRentalPeriod>,
    /// Whether this book is an active or an expired rental.
    #[serde(rename="rentalState")]
    
    pub rental_state: Option<String>,
    /// This user's review of this volume, if one exists.
    
    pub review: Option<Review>,
    /// Timestamp when this volume was last modified by a user action, such as a reading position update, volume purchase or writing a review. (RFC 3339 UTC date-time format).
    
    pub updated: Option<String>,
    /// no description provided
    #[serde(rename="userUploadedVolumeInfo")]
    
    pub user_uploaded_volume_info: Option<VolumeUserInfoUserUploadedVolumeInfo>,
}

impl client::NestedType for VolumeUserInfo {}
impl client::Part for VolumeUserInfo {}


/// Copy/Paste accounting information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeUserInfoCopy {
    /// no description provided
    #[serde(rename="allowedCharacterCount")]
    
    pub allowed_character_count: Option<i32>,
    /// no description provided
    #[serde(rename="limitType")]
    
    pub limit_type: Option<String>,
    /// no description provided
    #[serde(rename="remainingCharacterCount")]
    
    pub remaining_character_count: Option<i32>,
    /// no description provided
    
    pub updated: Option<String>,
}

impl client::NestedType for VolumeUserInfoCopy {}
impl client::Part for VolumeUserInfoCopy {}


/// Information on the ability to share with the family.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeUserInfoFamilySharing {
    /// The role of the user in the family.
    #[serde(rename="familyRole")]
    
    pub family_role: Option<String>,
    /// Whether or not this volume can be shared with the family by the user. This includes sharing eligibility of both the volume and the user. If the value is true, the user can initiate a family sharing action.
    #[serde(rename="isSharingAllowed")]
    
    pub is_sharing_allowed: Option<bool>,
    /// Whether or not sharing this volume is temporarily disabled due to issues with the Family Wallet.
    #[serde(rename="isSharingDisabledByFop")]
    
    pub is_sharing_disabled_by_fop: Option<bool>,
}

impl client::NestedType for VolumeUserInfoFamilySharing {}
impl client::Part for VolumeUserInfoFamilySharing {}


/// Period during this book is/was a valid rental.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeUserInfoRentalPeriod {
    /// no description provided
    #[serde(rename="endUtcSec")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub end_utc_sec: Option<i64>,
    /// no description provided
    #[serde(rename="startUtcSec")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub start_utc_sec: Option<i64>,
}

impl client::NestedType for VolumeUserInfoRentalPeriod {}
impl client::Part for VolumeUserInfoRentalPeriod {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeUserInfoUserUploadedVolumeInfo {
    /// no description provided
    #[serde(rename="processingState")]
    
    pub processing_state: Option<String>,
}

impl client::NestedType for VolumeUserInfoUserUploadedVolumeInfo {}
impl client::Part for VolumeUserInfoUserUploadedVolumeInfo {}


/// General volume information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeVolumeInfo {
    /// Whether anonymous logging should be allowed.
    #[serde(rename="allowAnonLogging")]
    
    pub allow_anon_logging: Option<bool>,
    /// The names of the authors and/or editors for this volume. (In LITE projection)
    
    pub authors: Option<Vec<String>>,
    /// The mean review rating for this volume. (min = 1.0, max = 5.0)
    #[serde(rename="averageRating")]
    
    pub average_rating: Option<f64>,
    /// Canonical URL for a volume. (In LITE projection.)
    #[serde(rename="canonicalVolumeLink")]
    
    pub canonical_volume_link: Option<String>,
    /// A list of subject categories, such as "Fiction", "Suspense", etc.
    
    pub categories: Option<Vec<String>>,
    /// Whether the volume has comics content.
    #[serde(rename="comicsContent")]
    
    pub comics_content: Option<bool>,
    /// An identifier for the version of the volume content (text & images). (In LITE projection)
    #[serde(rename="contentVersion")]
    
    pub content_version: Option<String>,
    /// A synopsis of the volume. The text of the description is formatted in HTML and includes simple formatting elements, such as b, i, and br tags. (In LITE projection.)
    
    pub description: Option<String>,
    /// Physical dimensions of this volume.
    
    pub dimensions: Option<VolumeVolumeInfoDimensions>,
    /// A list of image links for all the sizes that are available. (In LITE projection.)
    #[serde(rename="imageLinks")]
    
    pub image_links: Option<VolumeVolumeInfoImageLinks>,
    /// Industry standard identifiers for this volume.
    #[serde(rename="industryIdentifiers")]
    
    pub industry_identifiers: Option<Vec<VolumeVolumeInfoIndustryIdentifiers>>,
    /// URL to view information about this volume on the Google Books site. (In LITE projection)
    #[serde(rename="infoLink")]
    
    pub info_link: Option<String>,
    /// Best language for this volume (based on content). It is the two-letter ISO 639-1 code such as 'fr', 'en', etc.
    
    pub language: Option<String>,
    /// The main category to which this volume belongs. It will be the category from the categories list returned below that has the highest weight.
    #[serde(rename="mainCategory")]
    
    pub main_category: Option<String>,
    /// no description provided
    #[serde(rename="maturityRating")]
    
    pub maturity_rating: Option<String>,
    /// Total number of pages as per publisher metadata.
    #[serde(rename="pageCount")]
    
    pub page_count: Option<i32>,
    /// A top-level summary of the panelization info in this volume.
    #[serde(rename="panelizationSummary")]
    
    pub panelization_summary: Option<VolumeVolumeInfoPanelizationSummary>,
    /// URL to preview this volume on the Google Books site.
    #[serde(rename="previewLink")]
    
    pub preview_link: Option<String>,
    /// Type of publication of this volume. Possible values are BOOK or MAGAZINE.
    #[serde(rename="printType")]
    
    pub print_type: Option<String>,
    /// Total number of printed pages in generated pdf representation.
    #[serde(rename="printedPageCount")]
    
    pub printed_page_count: Option<i32>,
    /// Date of publication. (In LITE projection.)
    #[serde(rename="publishedDate")]
    
    pub published_date: Option<String>,
    /// Publisher of this volume. (In LITE projection.)
    
    pub publisher: Option<String>,
    /// The number of review ratings for this volume.
    #[serde(rename="ratingsCount")]
    
    pub ratings_count: Option<i32>,
    /// The reading modes available for this volume.
    #[serde(rename="readingModes")]
    
    pub reading_modes: Option<VolumeVolumeInfoReadingModes>,
    /// Total number of sample pages as per publisher metadata.
    #[serde(rename="samplePageCount")]
    
    pub sample_page_count: Option<i32>,
    /// no description provided
    #[serde(rename="seriesInfo")]
    
    pub series_info: Option<Volumeseriesinfo>,
    /// Volume subtitle. (In LITE projection.)
    
    pub subtitle: Option<String>,
    /// Volume title. (In LITE projection.)
    
    pub title: Option<String>,
}

impl client::NestedType for VolumeVolumeInfo {}
impl client::Part for VolumeVolumeInfo {}


/// Physical dimensions of this volume.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeVolumeInfoDimensions {
    /// Height or length of this volume (in cm).
    
    pub height: Option<String>,
    /// Thickness of this volume (in cm).
    
    pub thickness: Option<String>,
    /// Width of this volume (in cm).
    
    pub width: Option<String>,
}

impl client::NestedType for VolumeVolumeInfoDimensions {}
impl client::Part for VolumeVolumeInfoDimensions {}


/// A list of image links for all the sizes that are available. (In LITE projection.)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeVolumeInfoImageLinks {
    /// Image link for extra large size (width of ~1280 pixels). (In LITE projection)
    #[serde(rename="extraLarge")]
    
    pub extra_large: Option<String>,
    /// Image link for large size (width of ~800 pixels). (In LITE projection)
    
    pub large: Option<String>,
    /// Image link for medium size (width of ~575 pixels). (In LITE projection)
    
    pub medium: Option<String>,
    /// Image link for small size (width of ~300 pixels). (In LITE projection)
    
    pub small: Option<String>,
    /// Image link for small thumbnail size (width of ~80 pixels). (In LITE projection)
    #[serde(rename="smallThumbnail")]
    
    pub small_thumbnail: Option<String>,
    /// Image link for thumbnail size (width of ~128 pixels). (In LITE projection)
    
    pub thumbnail: Option<String>,
}

impl client::NestedType for VolumeVolumeInfoImageLinks {}
impl client::Part for VolumeVolumeInfoImageLinks {}


/// Industry standard identifiers for this volume.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeVolumeInfoIndustryIdentifiers {
    /// Industry specific volume identifier.
    
    pub identifier: Option<String>,
    /// Identifier type. Possible values are ISBN_10, ISBN_13, ISSN and OTHER.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::NestedType for VolumeVolumeInfoIndustryIdentifiers {}
impl client::Part for VolumeVolumeInfoIndustryIdentifiers {}


/// A top-level summary of the panelization info in this volume.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeVolumeInfoPanelizationSummary {
    /// no description provided
    #[serde(rename="containsEpubBubbles")]
    
    pub contains_epub_bubbles: Option<bool>,
    /// no description provided
    #[serde(rename="containsImageBubbles")]
    
    pub contains_image_bubbles: Option<bool>,
    /// no description provided
    #[serde(rename="epubBubbleVersion")]
    
    pub epub_bubble_version: Option<String>,
    /// no description provided
    #[serde(rename="imageBubbleVersion")]
    
    pub image_bubble_version: Option<String>,
}

impl client::NestedType for VolumeVolumeInfoPanelizationSummary {}
impl client::Part for VolumeVolumeInfoPanelizationSummary {}


/// The reading modes available for this volume.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeVolumeInfoReadingModes {
    /// no description provided
    
    pub image: Option<bool>,
    /// no description provided
    
    pub text: Option<bool>,
}

impl client::NestedType for VolumeVolumeInfoReadingModes {}
impl client::Part for VolumeVolumeInfoReadingModes {}


/// The content ranges to identify the selected text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeannotationContentRanges {
    /// Range in CFI format for this annotation for version above.
    #[serde(rename="cfiRange")]
    
    pub cfi_range: Option<BooksAnnotationsRange>,
    /// Content version applicable to ranges below.
    #[serde(rename="contentVersion")]
    
    pub content_version: Option<String>,
    /// Range in GB image format for this annotation for version above.
    #[serde(rename="gbImageRange")]
    
    pub gb_image_range: Option<BooksAnnotationsRange>,
    /// Range in GB text format for this annotation for version above.
    #[serde(rename="gbTextRange")]
    
    pub gb_text_range: Option<BooksAnnotationsRange>,
}

impl client::NestedType for VolumeannotationContentRanges {}
impl client::Part for VolumeannotationContentRanges {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeseriesinfoVolumeSeries {
    /// List of issues. Applicable only for Collection Edition and Omnibus.
    
    pub issue: Option<Vec<VolumeseriesinfoVolumeSeriesIssue>>,
    /// The book order number in the series.
    #[serde(rename="orderNumber")]
    
    pub order_number: Option<i32>,
    /// The book type in the context of series. Examples - Single Issue, Collection Edition, etc.
    #[serde(rename="seriesBookType")]
    
    pub series_book_type: Option<String>,
    /// The series id.
    #[serde(rename="seriesId")]
    
    pub series_id: Option<String>,
}

impl client::NestedType for VolumeseriesinfoVolumeSeries {}
impl client::Part for VolumeseriesinfoVolumeSeries {}


/// List of issues. Applicable only for Collection Edition and Omnibus.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeseriesinfoVolumeSeriesIssue {
    /// no description provided
    #[serde(rename="issueDisplayNumber")]
    
    pub issue_display_number: Option<String>,
    /// no description provided
    #[serde(rename="issueOrderNumber")]
    
    pub issue_order_number: Option<i32>,
}

impl client::NestedType for VolumeseriesinfoVolumeSeriesIssue {}
impl client::Part for VolumeseriesinfoVolumeSeriesIssue {}


