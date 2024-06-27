use std::collections::HashMap;
use std::cell::RefCell;
use std::default::Default;
use std::collections::BTreeSet;
use std::error::Error as StdError;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;

use hyper::client::connect;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::time::sleep;
use tower_service;
use serde::{Serialize, Deserialize};

use crate::{client, client::GetToken, client::serde_with};

// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Ord, PartialOrd, Hash, Debug, Clone, Copy)]
pub enum Scope {
    /// Manage your books
    Full,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Full => "https://www.googleapis.com/auth/books",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Full
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all Books related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_books1 as books1;
/// use books1::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.promooffer().accept()
///              .volume_id("eos")
///              .serial("dolor")
///              .product("ea")
///              .offer_id("ipsum")
///              .model("invidunt")
///              .manufacturer("amet")
///              .device("duo")
///              .android_id("ipsum")
///              .doit().await;
/// 
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
///         |Error::Io(_)
///         |Error::MissingAPIKey
///         |Error::MissingToken(_)
///         |Error::Cancelled
///         |Error::UploadSizeLimitExceeded(_, _)
///         |Error::Failure(_)
///         |Error::BadRequest(_)
///         |Error::FieldClash(_)
///         |Error::JsonDecodeError(_, _) => println!("{}", e),
///     },
///     Ok(res) => println!("Success: {:?}", res),
/// }
/// # }
/// ```
#[derive(Clone)]
pub struct Books<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: Box<dyn client::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, S> client::Hub for Books<S> {}

impl<'a, S> Books<S> {

    pub fn new<A: 'static + client::GetToken>(client: hyper::Client<S, hyper::body::Body>, auth: A) -> Books<S> {
        Books {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/5.0.5".to_string(),
            _base_url: "https://books.googleapis.com/".to_string(),
            _root_url: "https://books.googleapis.com/".to_string(),
        }
    }

    pub fn bookshelves(&'a self) -> BookshelfMethods<'a, S> {
        BookshelfMethods { hub: &self }
    }
    pub fn cloudloading(&'a self) -> CloudloadingMethods<'a, S> {
        CloudloadingMethods { hub: &self }
    }
    pub fn dictionary(&'a self) -> DictionaryMethods<'a, S> {
        DictionaryMethods { hub: &self }
    }
    pub fn familysharing(&'a self) -> FamilysharingMethods<'a, S> {
        FamilysharingMethods { hub: &self }
    }
    pub fn layers(&'a self) -> LayerMethods<'a, S> {
        LayerMethods { hub: &self }
    }
    pub fn myconfig(&'a self) -> MyconfigMethods<'a, S> {
        MyconfigMethods { hub: &self }
    }
    pub fn mylibrary(&'a self) -> MylibraryMethods<'a, S> {
        MylibraryMethods { hub: &self }
    }
    pub fn notification(&'a self) -> NotificationMethods<'a, S> {
        NotificationMethods { hub: &self }
    }
    pub fn onboarding(&'a self) -> OnboardingMethods<'a, S> {
        OnboardingMethods { hub: &self }
    }
    pub fn personalizedstream(&'a self) -> PersonalizedstreamMethods<'a, S> {
        PersonalizedstreamMethods { hub: &self }
    }
    pub fn promooffer(&'a self) -> PromoofferMethods<'a, S> {
        PromoofferMethods { hub: &self }
    }
    pub fn series(&'a self) -> SeriesMethods<'a, S> {
        SeriesMethods { hub: &self }
    }
    pub fn volumes(&'a self) -> VolumeMethods<'a, S> {
        VolumeMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/5.0.5`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://books.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://books.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [annotations insert mylibrary](MylibraryAnnotationInsertCall) (request|response)
/// * [annotations update mylibrary](MylibraryAnnotationUpdateCall) (request|response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *bookshelf* resources.
/// It is not used directly, but through the [`Books`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_books1 as books1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `list(...)` and `volumes_list(...)`
/// // to build up your call.
/// let rb = hub.bookshelves();
/// # }
/// ```
pub struct BookshelfMethods<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
}

impl<'a, S> client::MethodsBuilder for BookshelfMethods<'a, S> {}

impl<'a, S> BookshelfMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves volumes in a specific bookshelf for the specified user.
    /// 
    /// # Arguments
    ///
    /// * `userId` - ID of user for whom to retrieve bookshelf volumes.
    /// * `shelf` - ID of bookshelf to retrieve volumes.
    pub fn volumes_list(&self, user_id: &str, shelf: &str) -> BookshelfVolumeListCall<'a, S> {
        BookshelfVolumeListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _shelf: shelf.to_string(),
            _start_index: Default::default(),
            _source: Default::default(),
            _show_preorders: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves metadata for a specific bookshelf for the specified user.
    /// 
    /// # Arguments
    ///
    /// * `userId` - ID of user for whom to retrieve bookshelves.
    /// * `shelf` - ID of bookshelf to retrieve.
    pub fn get(&self, user_id: &str, shelf: &str) -> BookshelfGetCall<'a, S> {
        BookshelfGetCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _shelf: shelf.to_string(),
            _source: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of public bookshelves for the specified user.
    /// 
    /// # Arguments
    ///
    /// * `userId` - ID of user for whom to retrieve bookshelves.
    pub fn list(&self, user_id: &str) -> BookshelfListCall<'a, S> {
        BookshelfListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _source: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *cloudloading* resources.
/// It is not used directly, but through the [`Books`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_books1 as books1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `add_book(...)`, `delete_book(...)` and `update_book(...)`
/// // to build up your call.
/// let rb = hub.cloudloading();
/// # }
/// ```
pub struct CloudloadingMethods<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
}

impl<'a, S> client::MethodsBuilder for CloudloadingMethods<'a, S> {}

impl<'a, S> CloudloadingMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Add a user-upload volume and triggers processing.
    pub fn add_book(&self) -> CloudloadingAddBookCall<'a, S> {
        CloudloadingAddBookCall {
            hub: self.hub,
            _upload_client_token: Default::default(),
            _name: Default::default(),
            _mime_type: Default::default(),
            _drive_document_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Remove the book and its contents
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - The id of the book to be removed.
    pub fn delete_book(&self, volume_id: &str) -> CloudloadingDeleteBookCall<'a, S> {
        CloudloadingDeleteBookCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a user-upload volume.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn update_book(&self, request: BooksCloudloadingResource) -> CloudloadingUpdateBookCall<'a, S> {
        CloudloadingUpdateBookCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *dictionary* resources.
/// It is not used directly, but through the [`Books`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_books1 as books1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list_offline_metadata(...)`
/// // to build up your call.
/// let rb = hub.dictionary();
/// # }
/// ```
pub struct DictionaryMethods<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
}

impl<'a, S> client::MethodsBuilder for DictionaryMethods<'a, S> {}

impl<'a, S> DictionaryMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of offline dictionary metadata available
    /// 
    /// # Arguments
    ///
    /// * `cpksver` - The device/version ID from which to request the data.
    pub fn list_offline_metadata(&self, cpksver: &str) -> DictionaryListOfflineMetadataCall<'a, S> {
        DictionaryListOfflineMetadataCall {
            hub: self.hub,
            _cpksver: cpksver.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *familysharing* resources.
/// It is not used directly, but through the [`Books`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_books1 as books1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_family_info(...)`, `share(...)` and `unshare(...)`
/// // to build up your call.
/// let rb = hub.familysharing();
/// # }
/// ```
pub struct FamilysharingMethods<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
}

impl<'a, S> client::MethodsBuilder for FamilysharingMethods<'a, S> {}

impl<'a, S> FamilysharingMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information regarding the family that the user is part of.
    pub fn get_family_info(&self) -> FamilysharingGetFamilyInfoCall<'a, S> {
        FamilysharingGetFamilyInfoCall {
            hub: self.hub,
            _source: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Initiates sharing of the content with the user's family. Empty response indicates success.
    pub fn share(&self) -> FamilysharingShareCall<'a, S> {
        FamilysharingShareCall {
            hub: self.hub,
            _volume_id: Default::default(),
            _source: Default::default(),
            _doc_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Initiates revoking content that has already been shared with the user's family. Empty response indicates success.
    pub fn unshare(&self) -> FamilysharingUnshareCall<'a, S> {
        FamilysharingUnshareCall {
            hub: self.hub,
            _volume_id: Default::default(),
            _source: Default::default(),
            _doc_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *layer* resources.
/// It is not used directly, but through the [`Books`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_books1 as books1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `annotation_data_get(...)`, `annotation_data_list(...)`, `get(...)`, `list(...)`, `volume_annotations_get(...)` and `volume_annotations_list(...)`
/// // to build up your call.
/// let rb = hub.layers();
/// # }
/// ```
pub struct LayerMethods<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
}

impl<'a, S> client::MethodsBuilder for LayerMethods<'a, S> {}

impl<'a, S> LayerMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the annotation data.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - The volume to retrieve annotations for.
    /// * `layerId` - The ID for the layer to get the annotations.
    /// * `annotationDataId` - The ID of the annotation data to retrieve.
    /// * `contentVersion` - The content version for the volume you are trying to retrieve.
    pub fn annotation_data_get(&self, volume_id: &str, layer_id: &str, annotation_data_id: &str, content_version: &str) -> LayerAnnotationDataGetCall<'a, S> {
        LayerAnnotationDataGetCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _layer_id: layer_id.to_string(),
            _annotation_data_id: annotation_data_id.to_string(),
            _content_version: content_version.to_string(),
            _w: Default::default(),
            _source: Default::default(),
            _scale: Default::default(),
            _locale: Default::default(),
            _h: Default::default(),
            _allow_web_definitions: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the annotation data for a volume and layer.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - The volume to retrieve annotation data for.
    /// * `layerId` - The ID for the layer to get the annotation data.
    /// * `contentVersion` - The content version for the requested volume.
    pub fn annotation_data_list(&self, volume_id: &str, layer_id: &str, content_version: &str) -> LayerAnnotationDataListCall<'a, S> {
        LayerAnnotationDataListCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _layer_id: layer_id.to_string(),
            _content_version: content_version.to_string(),
            _w: Default::default(),
            _updated_min: Default::default(),
            _updated_max: Default::default(),
            _source: Default::default(),
            _scale: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _locale: Default::default(),
            _h: Default::default(),
            _annotation_data_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the volume annotation.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - The volume to retrieve annotations for.
    /// * `layerId` - The ID for the layer to get the annotations.
    /// * `annotationId` - The ID of the volume annotation to retrieve.
    pub fn volume_annotations_get(&self, volume_id: &str, layer_id: &str, annotation_id: &str) -> LayerVolumeAnnotationGetCall<'a, S> {
        LayerVolumeAnnotationGetCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _layer_id: layer_id.to_string(),
            _annotation_id: annotation_id.to_string(),
            _source: Default::default(),
            _locale: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the volume annotations for a volume and layer.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - The volume to retrieve annotations for.
    /// * `layerId` - The ID for the layer to get the annotations.
    /// * `contentVersion` - The content version for the requested volume.
    pub fn volume_annotations_list(&self, volume_id: &str, layer_id: &str, content_version: &str) -> LayerVolumeAnnotationListCall<'a, S> {
        LayerVolumeAnnotationListCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _layer_id: layer_id.to_string(),
            _content_version: content_version.to_string(),
            _volume_annotations_version: Default::default(),
            _updated_min: Default::default(),
            _updated_max: Default::default(),
            _start_position: Default::default(),
            _start_offset: Default::default(),
            _source: Default::default(),
            _show_deleted: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _locale: Default::default(),
            _end_position: Default::default(),
            _end_offset: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the layer summary for a volume.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - The volume to retrieve layers for.
    /// * `summaryId` - The ID for the layer to get the summary for.
    pub fn get(&self, volume_id: &str, summary_id: &str) -> LayerGetCall<'a, S> {
        LayerGetCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _summary_id: summary_id.to_string(),
            _source: Default::default(),
            _content_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List the layer summaries for a volume.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - The volume to retrieve layers for.
    pub fn list(&self, volume_id: &str) -> LayerListCall<'a, S> {
        LayerListCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _source: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _content_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *myconfig* resources.
/// It is not used directly, but through the [`Books`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_books1 as books1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_user_settings(...)`, `release_download_access(...)`, `request_access(...)`, `sync_volume_licenses(...)` and `update_user_settings(...)`
/// // to build up your call.
/// let rb = hub.myconfig();
/// # }
/// ```
pub struct MyconfigMethods<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
}

impl<'a, S> client::MethodsBuilder for MyconfigMethods<'a, S> {}

impl<'a, S> MyconfigMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the current settings for the user.
    pub fn get_user_settings(&self) -> MyconfigGetUserSettingCall<'a, S> {
        MyconfigGetUserSettingCall {
            hub: self.hub,
            _country: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Release downloaded content access restriction.
    /// 
    /// # Arguments
    ///
    /// * `cpksver` - The device/version ID from which to release the restriction.
    /// * `volumeIds` - The volume(s) to release restrictions for.
    pub fn release_download_access(&self, cpksver: &str, volume_ids: &Vec<String>) -> MyconfigReleaseDownloadAccesCall<'a, S> {
        MyconfigReleaseDownloadAccesCall {
            hub: self.hub,
            _cpksver: cpksver.to_string(),
            _volume_ids: volume_ids.clone(),
            _source: Default::default(),
            _locale: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Request concurrent and download access restrictions.
    /// 
    /// # Arguments
    ///
    /// * `cpksver` - The device/version ID from which to request the restrictions.
    /// * `nonce` - The client nonce value.
    /// * `source` - String to identify the originator of this request.
    /// * `volumeId` - The volume to request concurrent/download restrictions for.
    pub fn request_access(&self, cpksver: &str, nonce: &str, source: &str, volume_id: &str) -> MyconfigRequestAccesCall<'a, S> {
        MyconfigRequestAccesCall {
            hub: self.hub,
            _cpksver: cpksver.to_string(),
            _nonce: nonce.to_string(),
            _source: source.to_string(),
            _volume_id: volume_id.to_string(),
            _locale: Default::default(),
            _license_types: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Request downloaded content access for specified volumes on the My eBooks shelf.
    /// 
    /// # Arguments
    ///
    /// * `cpksver` - The device/version ID from which to release the restriction.
    /// * `nonce` - The client nonce value.
    /// * `source` - String to identify the originator of this request.
    pub fn sync_volume_licenses(&self, cpksver: &str, nonce: &str, source: &str) -> MyconfigSyncVolumeLicenseCall<'a, S> {
        MyconfigSyncVolumeLicenseCall {
            hub: self.hub,
            _cpksver: cpksver.to_string(),
            _nonce: nonce.to_string(),
            _source: source.to_string(),
            _volume_ids: Default::default(),
            _show_preorders: Default::default(),
            _locale: Default::default(),
            _include_non_comics_series: Default::default(),
            _features: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the settings for the user. If a sub-object is specified, it will overwrite the existing sub-object stored in the server. Unspecified sub-objects will retain the existing value.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn update_user_settings(&self, request: Usersettings) -> MyconfigUpdateUserSettingCall<'a, S> {
        MyconfigUpdateUserSettingCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *mylibrary* resources.
/// It is not used directly, but through the [`Books`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_books1 as books1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `annotations_delete(...)`, `annotations_insert(...)`, `annotations_list(...)`, `annotations_summary(...)`, `annotations_update(...)`, `bookshelves_add_volume(...)`, `bookshelves_clear_volumes(...)`, `bookshelves_get(...)`, `bookshelves_list(...)`, `bookshelves_move_volume(...)`, `bookshelves_remove_volume(...)`, `bookshelves_volumes_list(...)`, `readingpositions_get(...)` and `readingpositions_set_position(...)`
/// // to build up your call.
/// let rb = hub.mylibrary();
/// # }
/// ```
pub struct MylibraryMethods<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
}

impl<'a, S> client::MethodsBuilder for MylibraryMethods<'a, S> {}

impl<'a, S> MylibraryMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an annotation.
    /// 
    /// # Arguments
    ///
    /// * `annotationId` - The ID for the annotation to delete.
    pub fn annotations_delete(&self, annotation_id: &str) -> MylibraryAnnotationDeleteCall<'a, S> {
        MylibraryAnnotationDeleteCall {
            hub: self.hub,
            _annotation_id: annotation_id.to_string(),
            _source: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new annotation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn annotations_insert(&self, request: Annotation) -> MylibraryAnnotationInsertCall<'a, S> {
        MylibraryAnnotationInsertCall {
            hub: self.hub,
            _request: request,
            _source: Default::default(),
            _show_only_summary_in_response: Default::default(),
            _country: Default::default(),
            _annotation_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of annotations, possibly filtered.
    pub fn annotations_list(&self) -> MylibraryAnnotationListCall<'a, S> {
        MylibraryAnnotationListCall {
            hub: self.hub,
            _volume_id: Default::default(),
            _updated_min: Default::default(),
            _updated_max: Default::default(),
            _source: Default::default(),
            _show_deleted: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _layer_ids: Default::default(),
            _layer_id: Default::default(),
            _content_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the summary of specified layers.
    /// 
    /// # Arguments
    ///
    /// * `layerIds` - Array of layer IDs to get the summary for.
    /// * `volumeId` - Volume id to get the summary for.
    pub fn annotations_summary(&self, layer_ids: &Vec<String>, volume_id: &str) -> MylibraryAnnotationSummaryCall<'a, S> {
        MylibraryAnnotationSummaryCall {
            hub: self.hub,
            _layer_ids: layer_ids.clone(),
            _volume_id: volume_id.to_string(),
            _source: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing annotation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `annotationId` - The ID for the annotation to update.
    pub fn annotations_update(&self, request: Annotation, annotation_id: &str) -> MylibraryAnnotationUpdateCall<'a, S> {
        MylibraryAnnotationUpdateCall {
            hub: self.hub,
            _request: request,
            _annotation_id: annotation_id.to_string(),
            _source: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets volume information for volumes on a bookshelf.
    /// 
    /// # Arguments
    ///
    /// * `shelf` - The bookshelf ID or name retrieve volumes for.
    pub fn bookshelves_volumes_list(&self, shelf: &str) -> MylibraryBookshelfVolumeListCall<'a, S> {
        MylibraryBookshelfVolumeListCall {
            hub: self.hub,
            _shelf: shelf.to_string(),
            _start_index: Default::default(),
            _source: Default::default(),
            _show_preorders: Default::default(),
            _q: Default::default(),
            _projection: Default::default(),
            _max_results: Default::default(),
            _country: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a volume to a bookshelf.
    /// 
    /// # Arguments
    ///
    /// * `shelf` - ID of bookshelf to which to add a volume.
    /// * `volumeId` - ID of volume to add.
    pub fn bookshelves_add_volume(&self, shelf: &str, volume_id: &str) -> MylibraryBookshelfAddVolumeCall<'a, S> {
        MylibraryBookshelfAddVolumeCall {
            hub: self.hub,
            _shelf: shelf.to_string(),
            _volume_id: volume_id.to_string(),
            _source: Default::default(),
            _reason: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Clears all volumes from a bookshelf.
    /// 
    /// # Arguments
    ///
    /// * `shelf` - ID of bookshelf from which to remove a volume.
    pub fn bookshelves_clear_volumes(&self, shelf: &str) -> MylibraryBookshelfClearVolumeCall<'a, S> {
        MylibraryBookshelfClearVolumeCall {
            hub: self.hub,
            _shelf: shelf.to_string(),
            _source: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves metadata for a specific bookshelf belonging to the authenticated user.
    /// 
    /// # Arguments
    ///
    /// * `shelf` - ID of bookshelf to retrieve.
    pub fn bookshelves_get(&self, shelf: &str) -> MylibraryBookshelfGetCall<'a, S> {
        MylibraryBookshelfGetCall {
            hub: self.hub,
            _shelf: shelf.to_string(),
            _source: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of bookshelves belonging to the authenticated user.
    pub fn bookshelves_list(&self) -> MylibraryBookshelfListCall<'a, S> {
        MylibraryBookshelfListCall {
            hub: self.hub,
            _source: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Moves a volume within a bookshelf.
    /// 
    /// # Arguments
    ///
    /// * `shelf` - ID of bookshelf with the volume.
    /// * `volumeId` - ID of volume to move.
    /// * `volumePosition` - Position on shelf to move the item (0 puts the item before the current first item, 1 puts it between the first and the second and so on.)
    pub fn bookshelves_move_volume(&self, shelf: &str, volume_id: &str, volume_position: i32) -> MylibraryBookshelfMoveVolumeCall<'a, S> {
        MylibraryBookshelfMoveVolumeCall {
            hub: self.hub,
            _shelf: shelf.to_string(),
            _volume_id: volume_id.to_string(),
            _volume_position: volume_position,
            _source: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes a volume from a bookshelf.
    /// 
    /// # Arguments
    ///
    /// * `shelf` - ID of bookshelf from which to remove a volume.
    /// * `volumeId` - ID of volume to remove.
    pub fn bookshelves_remove_volume(&self, shelf: &str, volume_id: &str) -> MylibraryBookshelfRemoveVolumeCall<'a, S> {
        MylibraryBookshelfRemoveVolumeCall {
            hub: self.hub,
            _shelf: shelf.to_string(),
            _volume_id: volume_id.to_string(),
            _source: Default::default(),
            _reason: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves my reading position information for a volume.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - ID of volume for which to retrieve a reading position.
    pub fn readingpositions_get(&self, volume_id: &str) -> MylibraryReadingpositionGetCall<'a, S> {
        MylibraryReadingpositionGetCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _source: Default::default(),
            _content_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets my reading position information for a volume.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - ID of volume for which to update the reading position.
    /// * `position` - Position string for the new volume reading position.
    /// * `timestamp` - RFC 3339 UTC format timestamp associated with this reading position.
    pub fn readingpositions_set_position(&self, volume_id: &str, position: &str, timestamp: &str) -> MylibraryReadingpositionSetPositionCall<'a, S> {
        MylibraryReadingpositionSetPositionCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _position: position.to_string(),
            _timestamp: timestamp.to_string(),
            _source: Default::default(),
            _device_cookie: Default::default(),
            _content_version: Default::default(),
            _action: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *notification* resources.
/// It is not used directly, but through the [`Books`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_books1 as books1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.notification();
/// # }
/// ```
pub struct NotificationMethods<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
}

impl<'a, S> client::MethodsBuilder for NotificationMethods<'a, S> {}

impl<'a, S> NotificationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns notification details for a given notification id.
    /// 
    /// # Arguments
    ///
    /// * `notification_id` - String to identify the notification.
    pub fn get(&self, notification_id: &str) -> NotificationGetCall<'a, S> {
        NotificationGetCall {
            hub: self.hub,
            _notification_id: notification_id.to_string(),
            _source: Default::default(),
            _locale: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *onboarding* resources.
/// It is not used directly, but through the [`Books`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_books1 as books1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list_categories(...)` and `list_category_volumes(...)`
/// // to build up your call.
/// let rb = hub.onboarding();
/// # }
/// ```
pub struct OnboardingMethods<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
}

impl<'a, S> client::MethodsBuilder for OnboardingMethods<'a, S> {}

impl<'a, S> OnboardingMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// List categories for onboarding experience.
    pub fn list_categories(&self) -> OnboardingListCategoryCall<'a, S> {
        OnboardingListCategoryCall {
            hub: self.hub,
            _locale: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List available volumes under categories for onboarding experience.
    pub fn list_category_volumes(&self) -> OnboardingListCategoryVolumeCall<'a, S> {
        OnboardingListCategoryVolumeCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _max_allowed_maturity_rating: Default::default(),
            _locale: Default::default(),
            _category_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *personalizedstream* resources.
/// It is not used directly, but through the [`Books`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_books1 as books1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.personalizedstream();
/// # }
/// ```
pub struct PersonalizedstreamMethods<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
}

impl<'a, S> client::MethodsBuilder for PersonalizedstreamMethods<'a, S> {}

impl<'a, S> PersonalizedstreamMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a stream of personalized book clusters
    pub fn get(&self) -> PersonalizedstreamGetCall<'a, S> {
        PersonalizedstreamGetCall {
            hub: self.hub,
            _source: Default::default(),
            _max_allowed_maturity_rating: Default::default(),
            _locale: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *promooffer* resources.
/// It is not used directly, but through the [`Books`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_books1 as books1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `accept(...)`, `dismiss(...)` and `get(...)`
/// // to build up your call.
/// let rb = hub.promooffer();
/// # }
/// ```
pub struct PromoofferMethods<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
}

impl<'a, S> client::MethodsBuilder for PromoofferMethods<'a, S> {}

impl<'a, S> PromoofferMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Accepts the promo offer.
    pub fn accept(&self) -> PromoofferAcceptCall<'a, S> {
        PromoofferAcceptCall {
            hub: self.hub,
            _volume_id: Default::default(),
            _serial: Default::default(),
            _product: Default::default(),
            _offer_id: Default::default(),
            _model: Default::default(),
            _manufacturer: Default::default(),
            _device: Default::default(),
            _android_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Marks the promo offer as dismissed.
    pub fn dismiss(&self) -> PromoofferDismisCall<'a, S> {
        PromoofferDismisCall {
            hub: self.hub,
            _serial: Default::default(),
            _product: Default::default(),
            _offer_id: Default::default(),
            _model: Default::default(),
            _manufacturer: Default::default(),
            _device: Default::default(),
            _android_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of promo offers available to the user
    pub fn get(&self) -> PromoofferGetCall<'a, S> {
        PromoofferGetCall {
            hub: self.hub,
            _serial: Default::default(),
            _product: Default::default(),
            _model: Default::default(),
            _manufacturer: Default::default(),
            _device: Default::default(),
            _android_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *series* resources.
/// It is not used directly, but through the [`Books`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_books1 as books1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `membership_get(...)`
/// // to build up your call.
/// let rb = hub.series();
/// # }
/// ```
pub struct SeriesMethods<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
}

impl<'a, S> client::MethodsBuilder for SeriesMethods<'a, S> {}

impl<'a, S> SeriesMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns Series membership data given the series id.
    /// 
    /// # Arguments
    ///
    /// * `series_id` - String that identifies the series
    pub fn membership_get(&self, series_id: &str) -> SeriesMembershipGetCall<'a, S> {
        SeriesMembershipGetCall {
            hub: self.hub,
            _series_id: series_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns Series metadata for the given series ids.
    /// 
    /// # Arguments
    ///
    /// * `series_id` - String that identifies the series
    pub fn get(&self, series_id: &Vec<String>) -> SeriesGetCall<'a, S> {
        SeriesGetCall {
            hub: self.hub,
            _series_id: series_id.clone(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *volume* resources.
/// It is not used directly, but through the [`Books`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_books1 as books1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `associated_list(...)`, `get(...)`, `list(...)`, `mybooks_list(...)`, `recommended_list(...)`, `recommended_rate(...)` and `useruploaded_list(...)`
/// // to build up your call.
/// let rb = hub.volumes();
/// # }
/// ```
pub struct VolumeMethods<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
}

impl<'a, S> client::MethodsBuilder for VolumeMethods<'a, S> {}

impl<'a, S> VolumeMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Return a list of associated books.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - ID of the source volume.
    pub fn associated_list(&self, volume_id: &str) -> VolumeAssociatedListCall<'a, S> {
        VolumeAssociatedListCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _source: Default::default(),
            _max_allowed_maturity_rating: Default::default(),
            _locale: Default::default(),
            _association: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Return a list of books in My Library.
    pub fn mybooks_list(&self) -> VolumeMybookListCall<'a, S> {
        VolumeMybookListCall {
            hub: self.hub,
            _start_index: Default::default(),
            _source: Default::default(),
            _processing_state: Default::default(),
            _max_results: Default::default(),
            _locale: Default::default(),
            _country: Default::default(),
            _acquire_method: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Return a list of recommended books for the current user.
    pub fn recommended_list(&self) -> VolumeRecommendedListCall<'a, S> {
        VolumeRecommendedListCall {
            hub: self.hub,
            _source: Default::default(),
            _max_allowed_maturity_rating: Default::default(),
            _locale: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Rate a recommended book for the current user.
    /// 
    /// # Arguments
    ///
    /// * `rating` - Rating to be given to the volume.
    /// * `volumeId` - ID of the source volume.
    pub fn recommended_rate(&self, rating: &str, volume_id: &str) -> VolumeRecommendedRateCall<'a, S> {
        VolumeRecommendedRateCall {
            hub: self.hub,
            _rating: rating.to_string(),
            _volume_id: volume_id.to_string(),
            _source: Default::default(),
            _locale: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Return a list of books uploaded by the current user.
    pub fn useruploaded_list(&self) -> VolumeUseruploadedListCall<'a, S> {
        VolumeUseruploadedListCall {
            hub: self.hub,
            _volume_id: Default::default(),
            _start_index: Default::default(),
            _source: Default::default(),
            _processing_state: Default::default(),
            _max_results: Default::default(),
            _locale: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets volume information for a single volume.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - ID of volume to retrieve.
    pub fn get(&self, volume_id: &str) -> VolumeGetCall<'a, S> {
        VolumeGetCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _user_library_consistent_read: Default::default(),
            _source: Default::default(),
            _projection: Default::default(),
            _partner: Default::default(),
            _include_non_comics_series: Default::default(),
            _country: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Performs a book search.
    /// 
    /// # Arguments
    ///
    /// * `q` - Full-text search query string.
    pub fn list(&self, q: &str) -> VolumeListCall<'a, S> {
        VolumeListCall {
            hub: self.hub,
            _q: q.to_string(),
            _start_index: Default::default(),
            _source: Default::default(),
            _show_preorders: Default::default(),
            _projection: Default::default(),
            _print_type: Default::default(),
            _partner: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _max_allowed_maturity_rating: Default::default(),
            _library_restrict: Default::default(),
            _lang_restrict: Default::default(),
            _filter: Default::default(),
            _download: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Retrieves volumes in a specific bookshelf for the specified user.
///
/// A builder for the *volumes.list* method supported by a *bookshelf* resource.
/// It is not used directly, but through a [`BookshelfMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.bookshelves().volumes_list("userId", "shelf")
///              .start_index(89)
///              .source("rebum.")
///              .show_preorders(true)
///              .max_results(51)
///              .doit().await;
/// # }
/// ```
pub struct BookshelfVolumeListCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _user_id: String,
    _shelf: String,
    _start_index: Option<u32>,
    _source: Option<String>,
    _show_preorders: Option<bool>,
    _max_results: Option<u32>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for BookshelfVolumeListCall<'a, S> {}

impl<'a, S> BookshelfVolumeListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Volumes)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.bookshelves.volumes.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "userId", "shelf", "startIndex", "source", "showPreorders", "maxResults"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(8 + self._additional_params.len());
        params.push("userId", self._user_id);
        params.push("shelf", self._shelf);
        if let Some(value) = self._start_index.as_ref() {
            params.push("startIndex", value.to_string());
        }
        if let Some(value) = self._source.as_ref() {
            params.push("source", value);
        }
        if let Some(value) = self._show_preorders.as_ref() {
            params.push("showPreorders", value.to_string());
        }
        if let Some(value) = self._max_results.as_ref() {
            params.push("maxResults", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/users/{userId}/bookshelves/{shelf}/volumes";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{userId}", "userId"), ("{shelf}", "shelf")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["shelf", "userId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// ID of user for whom to retrieve bookshelf volumes.
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> BookshelfVolumeListCall<'a, S> {
        self._user_id = new_value.to_string();
        self
    }
    /// ID of bookshelf to retrieve volumes.
    ///
    /// Sets the *shelf* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn shelf(mut self, new_value: &str) -> BookshelfVolumeListCall<'a, S> {
        self._shelf = new_value.to_string();
        self
    }
    /// Index of the first element to return (starts at 0)
    ///
    /// Sets the *start index* query property to the given value.
    pub fn start_index(mut self, new_value: u32) -> BookshelfVolumeListCall<'a, S> {
        self._start_index = Some(new_value);
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> BookshelfVolumeListCall<'a, S> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Set to true to show pre-ordered books. Defaults to false.
    ///
    /// Sets the *show preorders* query property to the given value.
    pub fn show_preorders(mut self, new_value: bool) -> BookshelfVolumeListCall<'a, S> {
        self._show_preorders = Some(new_value);
        self
    }
    /// Maximum number of results to return
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> BookshelfVolumeListCall<'a, S> {
        self._max_results = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BookshelfVolumeListCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> BookshelfVolumeListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> BookshelfVolumeListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> BookshelfVolumeListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> BookshelfVolumeListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Retrieves metadata for a specific bookshelf for the specified user.
///
/// A builder for the *get* method supported by a *bookshelf* resource.
/// It is not used directly, but through a [`BookshelfMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.bookshelves().get("userId", "shelf")
///              .source("gubergren")
///              .doit().await;
/// # }
/// ```
pub struct BookshelfGetCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _user_id: String,
    _shelf: String,
    _source: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for BookshelfGetCall<'a, S> {}

impl<'a, S> BookshelfGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Bookshelf)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.bookshelves.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "userId", "shelf", "source"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("userId", self._user_id);
        params.push("shelf", self._shelf);
        if let Some(value) = self._source.as_ref() {
            params.push("source", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/users/{userId}/bookshelves/{shelf}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{userId}", "userId"), ("{shelf}", "shelf")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["shelf", "userId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// ID of user for whom to retrieve bookshelves.
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> BookshelfGetCall<'a, S> {
        self._user_id = new_value.to_string();
        self
    }
    /// ID of bookshelf to retrieve.
    ///
    /// Sets the *shelf* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn shelf(mut self, new_value: &str) -> BookshelfGetCall<'a, S> {
        self._shelf = new_value.to_string();
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> BookshelfGetCall<'a, S> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BookshelfGetCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> BookshelfGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> BookshelfGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> BookshelfGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> BookshelfGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Retrieves a list of public bookshelves for the specified user.
///
/// A builder for the *list* method supported by a *bookshelf* resource.
/// It is not used directly, but through a [`BookshelfMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.bookshelves().list("userId")
///              .source("dolor")
///              .doit().await;
/// # }
/// ```
pub struct BookshelfListCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _user_id: String,
    _source: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for BookshelfListCall<'a, S> {}

impl<'a, S> BookshelfListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Bookshelves)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.bookshelves.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "userId", "source"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("userId", self._user_id);
        if let Some(value) = self._source.as_ref() {
            params.push("source", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/users/{userId}/bookshelves";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{userId}", "userId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["userId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// ID of user for whom to retrieve bookshelves.
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> BookshelfListCall<'a, S> {
        self._user_id = new_value.to_string();
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> BookshelfListCall<'a, S> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BookshelfListCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> BookshelfListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> BookshelfListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> BookshelfListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> BookshelfListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Add a user-upload volume and triggers processing.
///
/// A builder for the *addBook* method supported by a *cloudloading* resource.
/// It is not used directly, but through a [`CloudloadingMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.cloudloading().add_book()
///              .upload_client_token("Lorem")
///              .name("eos")
///              .mime_type("labore")
///              .drive_document_id("sed")
///              .doit().await;
/// # }
/// ```
pub struct CloudloadingAddBookCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _upload_client_token: Option<String>,
    _name: Option<String>,
    _mime_type: Option<String>,
    _drive_document_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for CloudloadingAddBookCall<'a, S> {}

impl<'a, S> CloudloadingAddBookCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, BooksCloudloadingResource)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.cloudloading.addBook",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "upload_client_token", "name", "mime_type", "drive_document_id"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        if let Some(value) = self._upload_client_token.as_ref() {
            params.push("upload_client_token", value);
        }
        if let Some(value) = self._name.as_ref() {
            params.push("name", value);
        }
        if let Some(value) = self._mime_type.as_ref() {
            params.push("mime_type", value);
        }
        if let Some(value) = self._drive_document_id.as_ref() {
            params.push("drive_document_id", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/cloudloading/addBook";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Scotty upload token.
    ///
    /// Sets the *upload_client_token* query property to the given value.
    pub fn upload_client_token(mut self, new_value: &str) -> CloudloadingAddBookCall<'a, S> {
        self._upload_client_token = Some(new_value.to_string());
        self
    }
    /// The document name. It can be set only if the drive_document_id is set.
    ///
    /// Sets the *name* query property to the given value.
    pub fn name(mut self, new_value: &str) -> CloudloadingAddBookCall<'a, S> {
        self._name = Some(new_value.to_string());
        self
    }
    /// The document MIME type. It can be set only if the drive_document_id is set.
    ///
    /// Sets the *mime_type* query property to the given value.
    pub fn mime_type(mut self, new_value: &str) -> CloudloadingAddBookCall<'a, S> {
        self._mime_type = Some(new_value.to_string());
        self
    }
    /// A drive document id. The upload_client_token must not be set.
    ///
    /// Sets the *drive_document_id* query property to the given value.
    pub fn drive_document_id(mut self, new_value: &str) -> CloudloadingAddBookCall<'a, S> {
        self._drive_document_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CloudloadingAddBookCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> CloudloadingAddBookCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> CloudloadingAddBookCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> CloudloadingAddBookCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> CloudloadingAddBookCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Remove the book and its contents
///
/// A builder for the *deleteBook* method supported by a *cloudloading* resource.
/// It is not used directly, but through a [`CloudloadingMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.cloudloading().delete_book("volumeId")
///              .doit().await;
/// # }
/// ```
pub struct CloudloadingDeleteBookCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _volume_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for CloudloadingDeleteBookCall<'a, S> {}

impl<'a, S> CloudloadingDeleteBookCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Empty)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.cloudloading.deleteBook",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "volumeId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        params.push("volumeId", self._volume_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/cloudloading/deleteBook";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The id of the book to be removed.
    ///
    /// Sets the *volume id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn volume_id(mut self, new_value: &str) -> CloudloadingDeleteBookCall<'a, S> {
        self._volume_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CloudloadingDeleteBookCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> CloudloadingDeleteBookCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> CloudloadingDeleteBookCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> CloudloadingDeleteBookCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> CloudloadingDeleteBookCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Updates a user-upload volume.
///
/// A builder for the *updateBook* method supported by a *cloudloading* resource.
/// It is not used directly, but through a [`CloudloadingMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// use books1::api::BooksCloudloadingResource;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BooksCloudloadingResource::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.cloudloading().update_book(req)
///              .doit().await;
/// # }
/// ```
pub struct CloudloadingUpdateBookCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _request: BooksCloudloadingResource,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for CloudloadingUpdateBookCall<'a, S> {}

impl<'a, S> CloudloadingUpdateBookCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, BooksCloudloadingResource)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.cloudloading.updateBook",
                               http_method: hyper::Method::POST });

        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/cloudloading/updateBook";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: BooksCloudloadingResource) -> CloudloadingUpdateBookCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CloudloadingUpdateBookCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> CloudloadingUpdateBookCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> CloudloadingUpdateBookCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> CloudloadingUpdateBookCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> CloudloadingUpdateBookCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Returns a list of offline dictionary metadata available
///
/// A builder for the *listOfflineMetadata* method supported by a *dictionary* resource.
/// It is not used directly, but through a [`DictionaryMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.dictionary().list_offline_metadata("cpksver")
///              .doit().await;
/// # }
/// ```
pub struct DictionaryListOfflineMetadataCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _cpksver: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for DictionaryListOfflineMetadataCall<'a, S> {}

impl<'a, S> DictionaryListOfflineMetadataCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Metadata)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.dictionary.listOfflineMetadata",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "cpksver"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        params.push("cpksver", self._cpksver);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/dictionary/listOfflineMetadata";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The device/version ID from which to request the data.
    ///
    /// Sets the *cpksver* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn cpksver(mut self, new_value: &str) -> DictionaryListOfflineMetadataCall<'a, S> {
        self._cpksver = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> DictionaryListOfflineMetadataCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> DictionaryListOfflineMetadataCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> DictionaryListOfflineMetadataCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> DictionaryListOfflineMetadataCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> DictionaryListOfflineMetadataCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Gets information regarding the family that the user is part of.
///
/// A builder for the *getFamilyInfo* method supported by a *familysharing* resource.
/// It is not used directly, but through a [`FamilysharingMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.familysharing().get_family_info()
///              .source("no")
///              .doit().await;
/// # }
/// ```
pub struct FamilysharingGetFamilyInfoCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _source: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for FamilysharingGetFamilyInfoCall<'a, S> {}

impl<'a, S> FamilysharingGetFamilyInfoCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, FamilyInfo)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.familysharing.getFamilyInfo",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "source"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        if let Some(value) = self._source.as_ref() {
            params.push("source", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/familysharing/getFamilyInfo";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> FamilysharingGetFamilyInfoCall<'a, S> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> FamilysharingGetFamilyInfoCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> FamilysharingGetFamilyInfoCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> FamilysharingGetFamilyInfoCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> FamilysharingGetFamilyInfoCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> FamilysharingGetFamilyInfoCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Initiates sharing of the content with the user's family. Empty response indicates success.
///
/// A builder for the *share* method supported by a *familysharing* resource.
/// It is not used directly, but through a [`FamilysharingMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.familysharing().share()
///              .volume_id("Stet")
///              .source("kasd")
///              .doc_id("et")
///              .doit().await;
/// # }
/// ```
pub struct FamilysharingShareCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _volume_id: Option<String>,
    _source: Option<String>,
    _doc_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for FamilysharingShareCall<'a, S> {}

impl<'a, S> FamilysharingShareCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Empty)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.familysharing.share",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "volumeId", "source", "docId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        if let Some(value) = self._volume_id.as_ref() {
            params.push("volumeId", value);
        }
        if let Some(value) = self._source.as_ref() {
            params.push("source", value);
        }
        if let Some(value) = self._doc_id.as_ref() {
            params.push("docId", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/familysharing/share";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The volume to share.
    ///
    /// Sets the *volume id* query property to the given value.
    pub fn volume_id(mut self, new_value: &str) -> FamilysharingShareCall<'a, S> {
        self._volume_id = Some(new_value.to_string());
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> FamilysharingShareCall<'a, S> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The docid to share.
    ///
    /// Sets the *doc id* query property to the given value.
    pub fn doc_id(mut self, new_value: &str) -> FamilysharingShareCall<'a, S> {
        self._doc_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> FamilysharingShareCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> FamilysharingShareCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> FamilysharingShareCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> FamilysharingShareCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> FamilysharingShareCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Initiates revoking content that has already been shared with the user's family. Empty response indicates success.
///
/// A builder for the *unshare* method supported by a *familysharing* resource.
/// It is not used directly, but through a [`FamilysharingMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.familysharing().unshare()
///              .volume_id("sed")
///              .source("et")
///              .doc_id("et")
///              .doit().await;
/// # }
/// ```
pub struct FamilysharingUnshareCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _volume_id: Option<String>,
    _source: Option<String>,
    _doc_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for FamilysharingUnshareCall<'a, S> {}

impl<'a, S> FamilysharingUnshareCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Empty)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.familysharing.unshare",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "volumeId", "source", "docId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        if let Some(value) = self._volume_id.as_ref() {
            params.push("volumeId", value);
        }
        if let Some(value) = self._source.as_ref() {
            params.push("source", value);
        }
        if let Some(value) = self._doc_id.as_ref() {
            params.push("docId", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/familysharing/unshare";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The volume to unshare.
    ///
    /// Sets the *volume id* query property to the given value.
    pub fn volume_id(mut self, new_value: &str) -> FamilysharingUnshareCall<'a, S> {
        self._volume_id = Some(new_value.to_string());
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> FamilysharingUnshareCall<'a, S> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The docid to unshare.
    ///
    /// Sets the *doc id* query property to the given value.
    pub fn doc_id(mut self, new_value: &str) -> FamilysharingUnshareCall<'a, S> {
        self._doc_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> FamilysharingUnshareCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> FamilysharingUnshareCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> FamilysharingUnshareCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> FamilysharingUnshareCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> FamilysharingUnshareCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Gets the annotation data.
///
/// A builder for the *annotationData.get* method supported by a *layer* resource.
/// It is not used directly, but through a [`LayerMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.layers().annotation_data_get("volumeId", "layerId", "annotationDataId", "contentVersion")
///              .w(-34)
///              .source("et")
///              .scale(-28)
///              .locale("amet.")
///              .h(-96)
///              .allow_web_definitions(false)
///              .doit().await;
/// # }
/// ```
pub struct LayerAnnotationDataGetCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _volume_id: String,
    _layer_id: String,
    _annotation_data_id: String,
    _content_version: String,
    _w: Option<i32>,
    _source: Option<String>,
    _scale: Option<i32>,
    _locale: Option<String>,
    _h: Option<i32>,
    _allow_web_definitions: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for LayerAnnotationDataGetCall<'a, S> {}

impl<'a, S> LayerAnnotationDataGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, DictionaryAnnotationdata)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.layers.annotationData.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "volumeId", "layerId", "annotationDataId", "contentVersion", "w", "source", "scale", "locale", "h", "allowWebDefinitions"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(12 + self._additional_params.len());
        params.push("volumeId", self._volume_id);
        params.push("layerId", self._layer_id);
        params.push("annotationDataId", self._annotation_data_id);
        params.push("contentVersion", self._content_version);
        if let Some(value) = self._w.as_ref() {
            params.push("w", value.to_string());
        }
        if let Some(value) = self._source.as_ref() {
            params.push("source", value);
        }
        if let Some(value) = self._scale.as_ref() {
            params.push("scale", value.to_string());
        }
        if let Some(value) = self._locale.as_ref() {
            params.push("locale", value);
        }
        if let Some(value) = self._h.as_ref() {
            params.push("h", value.to_string());
        }
        if let Some(value) = self._allow_web_definitions.as_ref() {
            params.push("allowWebDefinitions", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/volumes/{volumeId}/layers/{layerId}/data/{annotationDataId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{volumeId}", "volumeId"), ("{layerId}", "layerId"), ("{annotationDataId}", "annotationDataId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["annotationDataId", "layerId", "volumeId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The volume to retrieve annotations for.
    ///
    /// Sets the *volume id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn volume_id(mut self, new_value: &str) -> LayerAnnotationDataGetCall<'a, S> {
        self._volume_id = new_value.to_string();
        self
    }
    /// The ID for the layer to get the annotations.
    ///
    /// Sets the *layer id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn layer_id(mut self, new_value: &str) -> LayerAnnotationDataGetCall<'a, S> {
        self._layer_id = new_value.to_string();
        self
    }
    /// The ID of the annotation data to retrieve.
    ///
    /// Sets the *annotation data id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn annotation_data_id(mut self, new_value: &str) -> LayerAnnotationDataGetCall<'a, S> {
        self._annotation_data_id = new_value.to_string();
        self
    }
    /// The content version for the volume you are trying to retrieve.
    ///
    /// Sets the *content version* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn content_version(mut self, new_value: &str) -> LayerAnnotationDataGetCall<'a, S> {
        self._content_version = new_value.to_string();
        self
    }
    /// The requested pixel width for any images. If width is provided height must also be provided.
    ///
    /// Sets the *w* query property to the given value.
    pub fn w(mut self, new_value: i32) -> LayerAnnotationDataGetCall<'a, S> {
        self._w = Some(new_value);
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> LayerAnnotationDataGetCall<'a, S> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The requested scale for the image.
    ///
    /// Sets the *scale* query property to the given value.
    pub fn scale(mut self, new_value: i32) -> LayerAnnotationDataGetCall<'a, S> {
        self._scale = Some(new_value);
        self
    }
    /// The locale information for the data. ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'.
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> LayerAnnotationDataGetCall<'a, S> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// The requested pixel height for any images. If height is provided width must also be provided.
    ///
    /// Sets the *h* query property to the given value.
    pub fn h(mut self, new_value: i32) -> LayerAnnotationDataGetCall<'a, S> {
        self._h = Some(new_value);
        self
    }
    /// For the dictionary layer. Whether or not to allow web definitions.
    ///
    /// Sets the *allow web definitions* query property to the given value.
    pub fn allow_web_definitions(mut self, new_value: bool) -> LayerAnnotationDataGetCall<'a, S> {
        self._allow_web_definitions = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> LayerAnnotationDataGetCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> LayerAnnotationDataGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> LayerAnnotationDataGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> LayerAnnotationDataGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> LayerAnnotationDataGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Gets the annotation data for a volume and layer.
///
/// A builder for the *annotationData.list* method supported by a *layer* resource.
/// It is not used directly, but through a [`LayerMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.layers().annotation_data_list("volumeId", "layerId", "contentVersion")
///              .w(-95)
///              .updated_min("Stet")
///              .updated_max("dolor")
///              .source("duo")
///              .scale(-76)
///              .page_token("vero")
///              .max_results(13)
///              .locale("Stet")
///              .h(-76)
///              .add_annotation_data_id("elitr")
///              .doit().await;
/// # }
/// ```
pub struct LayerAnnotationDataListCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _volume_id: String,
    _layer_id: String,
    _content_version: String,
    _w: Option<i32>,
    _updated_min: Option<String>,
    _updated_max: Option<String>,
    _source: Option<String>,
    _scale: Option<i32>,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _locale: Option<String>,
    _h: Option<i32>,
    _annotation_data_id: Vec<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for LayerAnnotationDataListCall<'a, S> {}

impl<'a, S> LayerAnnotationDataListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Annotationsdata)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.layers.annotationData.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "volumeId", "layerId", "contentVersion", "w", "updatedMin", "updatedMax", "source", "scale", "pageToken", "maxResults", "locale", "h", "annotationDataId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(15 + self._additional_params.len());
        params.push("volumeId", self._volume_id);
        params.push("layerId", self._layer_id);
        params.push("contentVersion", self._content_version);
        if let Some(value) = self._w.as_ref() {
            params.push("w", value.to_string());
        }
        if let Some(value) = self._updated_min.as_ref() {
            params.push("updatedMin", value);
        }
        if let Some(value) = self._updated_max.as_ref() {
            params.push("updatedMax", value);
        }
        if let Some(value) = self._source.as_ref() {
            params.push("source", value);
        }
        if let Some(value) = self._scale.as_ref() {
            params.push("scale", value.to_string());
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._max_results.as_ref() {
            params.push("maxResults", value.to_string());
        }
        if let Some(value) = self._locale.as_ref() {
            params.push("locale", value);
        }
        if let Some(value) = self._h.as_ref() {
            params.push("h", value.to_string());
        }
        if self._annotation_data_id.len() > 0 {
            for f in self._annotation_data_id.iter() {
                params.push("annotationDataId", f);
            }
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/volumes/{volumeId}/layers/{layerId}/data";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{volumeId}", "volumeId"), ("{layerId}", "layerId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["layerId", "volumeId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The volume to retrieve annotation data for.
    ///
    /// Sets the *volume id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn volume_id(mut self, new_value: &str) -> LayerAnnotationDataListCall<'a, S> {
        self._volume_id = new_value.to_string();
        self
    }
    /// The ID for the layer to get the annotation data.
    ///
    /// Sets the *layer id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn layer_id(mut self, new_value: &str) -> LayerAnnotationDataListCall<'a, S> {
        self._layer_id = new_value.to_string();
        self
    }
    /// The content version for the requested volume.
    ///
    /// Sets the *content version* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn content_version(mut self, new_value: &str) -> LayerAnnotationDataListCall<'a, S> {
        self._content_version = new_value.to_string();
        self
    }
    /// The requested pixel width for any images. If width is provided height must also be provided.
    ///
    /// Sets the *w* query property to the given value.
    pub fn w(mut self, new_value: i32) -> LayerAnnotationDataListCall<'a, S> {
        self._w = Some(new_value);
        self
    }
    /// RFC 3339 timestamp to restrict to items updated since this timestamp (inclusive).
    ///
    /// Sets the *updated min* query property to the given value.
    pub fn updated_min(mut self, new_value: &str) -> LayerAnnotationDataListCall<'a, S> {
        self._updated_min = Some(new_value.to_string());
        self
    }
    /// RFC 3339 timestamp to restrict to items updated prior to this timestamp (exclusive).
    ///
    /// Sets the *updated max* query property to the given value.
    pub fn updated_max(mut self, new_value: &str) -> LayerAnnotationDataListCall<'a, S> {
        self._updated_max = Some(new_value.to_string());
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> LayerAnnotationDataListCall<'a, S> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The requested scale for the image.
    ///
    /// Sets the *scale* query property to the given value.
    pub fn scale(mut self, new_value: i32) -> LayerAnnotationDataListCall<'a, S> {
        self._scale = Some(new_value);
        self
    }
    /// The value of the nextToken from the previous page.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> LayerAnnotationDataListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of results to return
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> LayerAnnotationDataListCall<'a, S> {
        self._max_results = Some(new_value);
        self
    }
    /// The locale information for the data. ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'.
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> LayerAnnotationDataListCall<'a, S> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// The requested pixel height for any images. If height is provided width must also be provided.
    ///
    /// Sets the *h* query property to the given value.
    pub fn h(mut self, new_value: i32) -> LayerAnnotationDataListCall<'a, S> {
        self._h = Some(new_value);
        self
    }
    /// The list of Annotation Data Ids to retrieve. Pagination is ignored if this is set.
    ///
    /// Append the given value to the *annotation data id* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_annotation_data_id(mut self, new_value: &str) -> LayerAnnotationDataListCall<'a, S> {
        self._annotation_data_id.push(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> LayerAnnotationDataListCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> LayerAnnotationDataListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> LayerAnnotationDataListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> LayerAnnotationDataListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> LayerAnnotationDataListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Gets the volume annotation.
///
/// A builder for the *volumeAnnotations.get* method supported by a *layer* resource.
/// It is not used directly, but through a [`LayerMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.layers().volume_annotations_get("volumeId", "layerId", "annotationId")
///              .source("ipsum")
///              .locale("accusam")
///              .doit().await;
/// # }
/// ```
pub struct LayerVolumeAnnotationGetCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _volume_id: String,
    _layer_id: String,
    _annotation_id: String,
    _source: Option<String>,
    _locale: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for LayerVolumeAnnotationGetCall<'a, S> {}

impl<'a, S> LayerVolumeAnnotationGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Volumeannotation)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.layers.volumeAnnotations.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "volumeId", "layerId", "annotationId", "source", "locale"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(7 + self._additional_params.len());
        params.push("volumeId", self._volume_id);
        params.push("layerId", self._layer_id);
        params.push("annotationId", self._annotation_id);
        if let Some(value) = self._source.as_ref() {
            params.push("source", value);
        }
        if let Some(value) = self._locale.as_ref() {
            params.push("locale", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/volumes/{volumeId}/layers/{layerId}/annotations/{annotationId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{volumeId}", "volumeId"), ("{layerId}", "layerId"), ("{annotationId}", "annotationId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["annotationId", "layerId", "volumeId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The volume to retrieve annotations for.
    ///
    /// Sets the *volume id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn volume_id(mut self, new_value: &str) -> LayerVolumeAnnotationGetCall<'a, S> {
        self._volume_id = new_value.to_string();
        self
    }
    /// The ID for the layer to get the annotations.
    ///
    /// Sets the *layer id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn layer_id(mut self, new_value: &str) -> LayerVolumeAnnotationGetCall<'a, S> {
        self._layer_id = new_value.to_string();
        self
    }
    /// The ID of the volume annotation to retrieve.
    ///
    /// Sets the *annotation id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn annotation_id(mut self, new_value: &str) -> LayerVolumeAnnotationGetCall<'a, S> {
        self._annotation_id = new_value.to_string();
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> LayerVolumeAnnotationGetCall<'a, S> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The locale information for the data. ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'.
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> LayerVolumeAnnotationGetCall<'a, S> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> LayerVolumeAnnotationGetCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> LayerVolumeAnnotationGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> LayerVolumeAnnotationGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> LayerVolumeAnnotationGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> LayerVolumeAnnotationGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Gets the volume annotations for a volume and layer.
///
/// A builder for the *volumeAnnotations.list* method supported by a *layer* resource.
/// It is not used directly, but through a [`LayerMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.layers().volume_annotations_list("volumeId", "layerId", "contentVersion")
///              .volume_annotations_version("et")
///              .updated_min("erat")
///              .updated_max("consetetur")
///              .start_position("amet.")
///              .start_offset("sed")
///              .source("takimata")
///              .show_deleted(true)
///              .page_token("et")
///              .max_results(78)
///              .locale("voluptua.")
///              .end_position("dolore")
///              .end_offset("dolore")
///              .doit().await;
/// # }
/// ```
pub struct LayerVolumeAnnotationListCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _volume_id: String,
    _layer_id: String,
    _content_version: String,
    _volume_annotations_version: Option<String>,
    _updated_min: Option<String>,
    _updated_max: Option<String>,
    _start_position: Option<String>,
    _start_offset: Option<String>,
    _source: Option<String>,
    _show_deleted: Option<bool>,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _locale: Option<String>,
    _end_position: Option<String>,
    _end_offset: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for LayerVolumeAnnotationListCall<'a, S> {}

impl<'a, S> LayerVolumeAnnotationListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Volumeannotations)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.layers.volumeAnnotations.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "volumeId", "layerId", "contentVersion", "volumeAnnotationsVersion", "updatedMin", "updatedMax", "startPosition", "startOffset", "source", "showDeleted", "pageToken", "maxResults", "locale", "endPosition", "endOffset"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(17 + self._additional_params.len());
        params.push("volumeId", self._volume_id);
        params.push("layerId", self._layer_id);
        params.push("contentVersion", self._content_version);
        if let Some(value) = self._volume_annotations_version.as_ref() {
            params.push("volumeAnnotationsVersion", value);
        }
        if let Some(value) = self._updated_min.as_ref() {
            params.push("updatedMin", value);
        }
        if let Some(value) = self._updated_max.as_ref() {
            params.push("updatedMax", value);
        }
        if let Some(value) = self._start_position.as_ref() {
            params.push("startPosition", value);
        }
        if let Some(value) = self._start_offset.as_ref() {
            params.push("startOffset", value);
        }
        if let Some(value) = self._source.as_ref() {
            params.push("source", value);
        }
        if let Some(value) = self._show_deleted.as_ref() {
            params.push("showDeleted", value.to_string());
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._max_results.as_ref() {
            params.push("maxResults", value.to_string());
        }
        if let Some(value) = self._locale.as_ref() {
            params.push("locale", value);
        }
        if let Some(value) = self._end_position.as_ref() {
            params.push("endPosition", value);
        }
        if let Some(value) = self._end_offset.as_ref() {
            params.push("endOffset", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/volumes/{volumeId}/layers/{layerId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{volumeId}", "volumeId"), ("{layerId}", "layerId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["layerId", "volumeId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The volume to retrieve annotations for.
    ///
    /// Sets the *volume id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn volume_id(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, S> {
        self._volume_id = new_value.to_string();
        self
    }
    /// The ID for the layer to get the annotations.
    ///
    /// Sets the *layer id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn layer_id(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, S> {
        self._layer_id = new_value.to_string();
        self
    }
    /// The content version for the requested volume.
    ///
    /// Sets the *content version* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn content_version(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, S> {
        self._content_version = new_value.to_string();
        self
    }
    /// The version of the volume annotations that you are requesting.
    ///
    /// Sets the *volume annotations version* query property to the given value.
    pub fn volume_annotations_version(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, S> {
        self._volume_annotations_version = Some(new_value.to_string());
        self
    }
    /// RFC 3339 timestamp to restrict to items updated since this timestamp (inclusive).
    ///
    /// Sets the *updated min* query property to the given value.
    pub fn updated_min(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, S> {
        self._updated_min = Some(new_value.to_string());
        self
    }
    /// RFC 3339 timestamp to restrict to items updated prior to this timestamp (exclusive).
    ///
    /// Sets the *updated max* query property to the given value.
    pub fn updated_max(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, S> {
        self._updated_max = Some(new_value.to_string());
        self
    }
    /// The start position to start retrieving data from.
    ///
    /// Sets the *start position* query property to the given value.
    pub fn start_position(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, S> {
        self._start_position = Some(new_value.to_string());
        self
    }
    /// The start offset to start retrieving data from.
    ///
    /// Sets the *start offset* query property to the given value.
    pub fn start_offset(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, S> {
        self._start_offset = Some(new_value.to_string());
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, S> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Set to true to return deleted annotations. updatedMin must be in the request to use this. Defaults to false.
    ///
    /// Sets the *show deleted* query property to the given value.
    pub fn show_deleted(mut self, new_value: bool) -> LayerVolumeAnnotationListCall<'a, S> {
        self._show_deleted = Some(new_value);
        self
    }
    /// The value of the nextToken from the previous page.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of results to return
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> LayerVolumeAnnotationListCall<'a, S> {
        self._max_results = Some(new_value);
        self
    }
    /// The locale information for the data. ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'.
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, S> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// The end position to end retrieving data from.
    ///
    /// Sets the *end position* query property to the given value.
    pub fn end_position(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, S> {
        self._end_position = Some(new_value.to_string());
        self
    }
    /// The end offset to end retrieving data from.
    ///
    /// Sets the *end offset* query property to the given value.
    pub fn end_offset(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, S> {
        self._end_offset = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> LayerVolumeAnnotationListCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> LayerVolumeAnnotationListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> LayerVolumeAnnotationListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> LayerVolumeAnnotationListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> LayerVolumeAnnotationListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Gets the layer summary for a volume.
///
/// A builder for the *get* method supported by a *layer* resource.
/// It is not used directly, but through a [`LayerMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.layers().get("volumeId", "summaryId")
///              .source("amet.")
///              .content_version("ea")
///              .doit().await;
/// # }
/// ```
pub struct LayerGetCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _volume_id: String,
    _summary_id: String,
    _source: Option<String>,
    _content_version: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for LayerGetCall<'a, S> {}

impl<'a, S> LayerGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Layersummary)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.layers.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "volumeId", "summaryId", "source", "contentVersion"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("volumeId", self._volume_id);
        params.push("summaryId", self._summary_id);
        if let Some(value) = self._source.as_ref() {
            params.push("source", value);
        }
        if let Some(value) = self._content_version.as_ref() {
            params.push("contentVersion", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/volumes/{volumeId}/layersummary/{summaryId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{volumeId}", "volumeId"), ("{summaryId}", "summaryId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["summaryId", "volumeId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The volume to retrieve layers for.
    ///
    /// Sets the *volume id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn volume_id(mut self, new_value: &str) -> LayerGetCall<'a, S> {
        self._volume_id = new_value.to_string();
        self
    }
    /// The ID for the layer to get the summary for.
    ///
    /// Sets the *summary id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn summary_id(mut self, new_value: &str) -> LayerGetCall<'a, S> {
        self._summary_id = new_value.to_string();
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> LayerGetCall<'a, S> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The content version for the requested volume.
    ///
    /// Sets the *content version* query property to the given value.
    pub fn content_version(mut self, new_value: &str) -> LayerGetCall<'a, S> {
        self._content_version = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> LayerGetCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> LayerGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> LayerGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> LayerGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> LayerGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// List the layer summaries for a volume.
///
/// A builder for the *list* method supported by a *layer* resource.
/// It is not used directly, but through a [`LayerMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.layers().list("volumeId")
///              .source("Lorem")
///              .page_token("invidunt")
///              .max_results(90)
///              .content_version("est")
///              .doit().await;
/// # }
/// ```
pub struct LayerListCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _volume_id: String,
    _source: Option<String>,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _content_version: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for LayerListCall<'a, S> {}

impl<'a, S> LayerListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Layersummaries)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.layers.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "volumeId", "source", "pageToken", "maxResults", "contentVersion"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(7 + self._additional_params.len());
        params.push("volumeId", self._volume_id);
        if let Some(value) = self._source.as_ref() {
            params.push("source", value);
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._max_results.as_ref() {
            params.push("maxResults", value.to_string());
        }
        if let Some(value) = self._content_version.as_ref() {
            params.push("contentVersion", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/volumes/{volumeId}/layersummary";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{volumeId}", "volumeId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["volumeId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The volume to retrieve layers for.
    ///
    /// Sets the *volume id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn volume_id(mut self, new_value: &str) -> LayerListCall<'a, S> {
        self._volume_id = new_value.to_string();
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> LayerListCall<'a, S> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The value of the nextToken from the previous page.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> LayerListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of results to return
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> LayerListCall<'a, S> {
        self._max_results = Some(new_value);
        self
    }
    /// The content version for the requested volume.
    ///
    /// Sets the *content version* query property to the given value.
    pub fn content_version(mut self, new_value: &str) -> LayerListCall<'a, S> {
        self._content_version = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> LayerListCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> LayerListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> LayerListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> LayerListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> LayerListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Gets the current settings for the user.
///
/// A builder for the *getUserSettings* method supported by a *myconfig* resource.
/// It is not used directly, but through a [`MyconfigMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.myconfig().get_user_settings()
///              .country("At")
///              .doit().await;
/// # }
/// ```
pub struct MyconfigGetUserSettingCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _country: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for MyconfigGetUserSettingCall<'a, S> {}

impl<'a, S> MyconfigGetUserSettingCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Usersettings)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.myconfig.getUserSettings",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "country"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        if let Some(value) = self._country.as_ref() {
            params.push("country", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/myconfig/getUserSettings";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Unused. Added only to workaround TEX mandatory request template requirement
    ///
    /// Sets the *country* query property to the given value.
    pub fn country(mut self, new_value: &str) -> MyconfigGetUserSettingCall<'a, S> {
        self._country = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> MyconfigGetUserSettingCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> MyconfigGetUserSettingCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> MyconfigGetUserSettingCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> MyconfigGetUserSettingCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> MyconfigGetUserSettingCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Release downloaded content access restriction.
///
/// A builder for the *releaseDownloadAccess* method supported by a *myconfig* resource.
/// It is not used directly, but through a [`MyconfigMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.myconfig().release_download_access("cpksver", &vec!["sit".into()])
///              .source("et")
///              .locale("tempor")
///              .doit().await;
/// # }
/// ```
pub struct MyconfigReleaseDownloadAccesCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _cpksver: String,
    _volume_ids: Vec<String>,
    _source: Option<String>,
    _locale: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for MyconfigReleaseDownloadAccesCall<'a, S> {}

impl<'a, S> MyconfigReleaseDownloadAccesCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, DownloadAccesses)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.myconfig.releaseDownloadAccess",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "cpksver", "volumeIds", "source", "locale"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("cpksver", self._cpksver);
        if self._volume_ids.len() > 0 {
            for f in self._volume_ids.iter() {
                params.push("volumeIds", f);
            }
        }
        if let Some(value) = self._source.as_ref() {
            params.push("source", value);
        }
        if let Some(value) = self._locale.as_ref() {
            params.push("locale", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/myconfig/releaseDownloadAccess";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The device/version ID from which to release the restriction.
    ///
    /// Sets the *cpksver* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn cpksver(mut self, new_value: &str) -> MyconfigReleaseDownloadAccesCall<'a, S> {
        self._cpksver = new_value.to_string();
        self
    }
    /// The volume(s) to release restrictions for.
    ///
    /// Append the given value to the *volume ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn add_volume_ids(mut self, new_value: &str) -> MyconfigReleaseDownloadAccesCall<'a, S> {
        self._volume_ids.push(new_value.to_string());
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> MyconfigReleaseDownloadAccesCall<'a, S> {
        self._source = Some(new_value.to_string());
        self
    }
    /// ISO-639-1, ISO-3166-1 codes for message localization, i.e. en_US.
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> MyconfigReleaseDownloadAccesCall<'a, S> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> MyconfigReleaseDownloadAccesCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> MyconfigReleaseDownloadAccesCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> MyconfigReleaseDownloadAccesCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> MyconfigReleaseDownloadAccesCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> MyconfigReleaseDownloadAccesCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Request concurrent and download access restrictions.
///
/// A builder for the *requestAccess* method supported by a *myconfig* resource.
/// It is not used directly, but through a [`MyconfigMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.myconfig().request_access("cpksver", "nonce", "source", "volumeId")
///              .locale("Lorem")
///              .license_types("est")
///              .doit().await;
/// # }
/// ```
pub struct MyconfigRequestAccesCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _cpksver: String,
    _nonce: String,
    _source: String,
    _volume_id: String,
    _locale: Option<String>,
    _license_types: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for MyconfigRequestAccesCall<'a, S> {}

impl<'a, S> MyconfigRequestAccesCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, RequestAccessData)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.myconfig.requestAccess",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "cpksver", "nonce", "source", "volumeId", "locale", "licenseTypes"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(8 + self._additional_params.len());
        params.push("cpksver", self._cpksver);
        params.push("nonce", self._nonce);
        params.push("source", self._source);
        params.push("volumeId", self._volume_id);
        if let Some(value) = self._locale.as_ref() {
            params.push("locale", value);
        }
        if let Some(value) = self._license_types.as_ref() {
            params.push("licenseTypes", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/myconfig/requestAccess";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The device/version ID from which to request the restrictions.
    ///
    /// Sets the *cpksver* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn cpksver(mut self, new_value: &str) -> MyconfigRequestAccesCall<'a, S> {
        self._cpksver = new_value.to_string();
        self
    }
    /// The client nonce value.
    ///
    /// Sets the *nonce* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn nonce(mut self, new_value: &str) -> MyconfigRequestAccesCall<'a, S> {
        self._nonce = new_value.to_string();
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn source(mut self, new_value: &str) -> MyconfigRequestAccesCall<'a, S> {
        self._source = new_value.to_string();
        self
    }
    /// The volume to request concurrent/download restrictions for.
    ///
    /// Sets the *volume id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn volume_id(mut self, new_value: &str) -> MyconfigRequestAccesCall<'a, S> {
        self._volume_id = new_value.to_string();
        self
    }
    /// ISO-639-1, ISO-3166-1 codes for message localization, i.e. en_US.
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> MyconfigRequestAccesCall<'a, S> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// The type of access license to request. If not specified, the default is BOTH.
    ///
    /// Sets the *license types* query property to the given value.
    pub fn license_types(mut self, new_value: &str) -> MyconfigRequestAccesCall<'a, S> {
        self._license_types = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> MyconfigRequestAccesCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> MyconfigRequestAccesCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> MyconfigRequestAccesCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> MyconfigRequestAccesCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> MyconfigRequestAccesCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Request downloaded content access for specified volumes on the My eBooks shelf.
///
/// A builder for the *syncVolumeLicenses* method supported by a *myconfig* resource.
/// It is not used directly, but through a [`MyconfigMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.myconfig().sync_volume_licenses("cpksver", "nonce", "source")
///              .add_volume_ids("dolores")
///              .show_preorders(true)
///              .locale("sed")
///              .include_non_comics_series(false)
///              .add_features("elitr")
///              .doit().await;
/// # }
/// ```
pub struct MyconfigSyncVolumeLicenseCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _cpksver: String,
    _nonce: String,
    _source: String,
    _volume_ids: Vec<String>,
    _show_preorders: Option<bool>,
    _locale: Option<String>,
    _include_non_comics_series: Option<bool>,
    _features: Vec<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for MyconfigSyncVolumeLicenseCall<'a, S> {}

impl<'a, S> MyconfigSyncVolumeLicenseCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Volumes)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.myconfig.syncVolumeLicenses",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "cpksver", "nonce", "source", "volumeIds", "showPreorders", "locale", "includeNonComicsSeries", "features"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(10 + self._additional_params.len());
        params.push("cpksver", self._cpksver);
        params.push("nonce", self._nonce);
        params.push("source", self._source);
        if self._volume_ids.len() > 0 {
            for f in self._volume_ids.iter() {
                params.push("volumeIds", f);
            }
        }
        if let Some(value) = self._show_preorders.as_ref() {
            params.push("showPreorders", value.to_string());
        }
        if let Some(value) = self._locale.as_ref() {
            params.push("locale", value);
        }
        if let Some(value) = self._include_non_comics_series.as_ref() {
            params.push("includeNonComicsSeries", value.to_string());
        }
        if self._features.len() > 0 {
            for f in self._features.iter() {
                params.push("features", f);
            }
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/myconfig/syncVolumeLicenses";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The device/version ID from which to release the restriction.
    ///
    /// Sets the *cpksver* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn cpksver(mut self, new_value: &str) -> MyconfigSyncVolumeLicenseCall<'a, S> {
        self._cpksver = new_value.to_string();
        self
    }
    /// The client nonce value.
    ///
    /// Sets the *nonce* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn nonce(mut self, new_value: &str) -> MyconfigSyncVolumeLicenseCall<'a, S> {
        self._nonce = new_value.to_string();
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn source(mut self, new_value: &str) -> MyconfigSyncVolumeLicenseCall<'a, S> {
        self._source = new_value.to_string();
        self
    }
    /// The volume(s) to request download restrictions for.
    ///
    /// Append the given value to the *volume ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_volume_ids(mut self, new_value: &str) -> MyconfigSyncVolumeLicenseCall<'a, S> {
        self._volume_ids.push(new_value.to_string());
        self
    }
    /// Set to true to show pre-ordered books. Defaults to false.
    ///
    /// Sets the *show preorders* query property to the given value.
    pub fn show_preorders(mut self, new_value: bool) -> MyconfigSyncVolumeLicenseCall<'a, S> {
        self._show_preorders = Some(new_value);
        self
    }
    /// ISO-639-1, ISO-3166-1 codes for message localization, i.e. en_US.
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> MyconfigSyncVolumeLicenseCall<'a, S> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// Set to true to include non-comics series. Defaults to false.
    ///
    /// Sets the *include non comics series* query property to the given value.
    pub fn include_non_comics_series(mut self, new_value: bool) -> MyconfigSyncVolumeLicenseCall<'a, S> {
        self._include_non_comics_series = Some(new_value);
        self
    }
    /// List of features supported by the client, i.e., 'RENTALS'
    ///
    /// Append the given value to the *features* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_features(mut self, new_value: &str) -> MyconfigSyncVolumeLicenseCall<'a, S> {
        self._features.push(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> MyconfigSyncVolumeLicenseCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> MyconfigSyncVolumeLicenseCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> MyconfigSyncVolumeLicenseCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> MyconfigSyncVolumeLicenseCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> MyconfigSyncVolumeLicenseCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Sets the settings for the user. If a sub-object is specified, it will overwrite the existing sub-object stored in the server. Unspecified sub-objects will retain the existing value.
///
/// A builder for the *updateUserSettings* method supported by a *myconfig* resource.
/// It is not used directly, but through a [`MyconfigMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// use books1::api::Usersettings;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Usersettings::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.myconfig().update_user_settings(req)
///              .doit().await;
/// # }
/// ```
pub struct MyconfigUpdateUserSettingCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _request: Usersettings,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for MyconfigUpdateUserSettingCall<'a, S> {}

impl<'a, S> MyconfigUpdateUserSettingCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Usersettings)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.myconfig.updateUserSettings",
                               http_method: hyper::Method::POST });

        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/myconfig/updateUserSettings";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Usersettings) -> MyconfigUpdateUserSettingCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> MyconfigUpdateUserSettingCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> MyconfigUpdateUserSettingCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> MyconfigUpdateUserSettingCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> MyconfigUpdateUserSettingCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> MyconfigUpdateUserSettingCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Deletes an annotation.
///
/// A builder for the *annotations.delete* method supported by a *mylibrary* resource.
/// It is not used directly, but through a [`MylibraryMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().annotations_delete("annotationId")
///              .source("no")
///              .doit().await;
/// # }
/// ```
pub struct MylibraryAnnotationDeleteCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _annotation_id: String,
    _source: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for MylibraryAnnotationDeleteCall<'a, S> {}

impl<'a, S> MylibraryAnnotationDeleteCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Empty)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.mylibrary.annotations.delete",
                               http_method: hyper::Method::DELETE });

        for &field in ["alt", "annotationId", "source"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("annotationId", self._annotation_id);
        if let Some(value) = self._source.as_ref() {
            params.push("source", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/mylibrary/annotations/{annotationId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{annotationId}", "annotationId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["annotationId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::DELETE)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID for the annotation to delete.
    ///
    /// Sets the *annotation id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn annotation_id(mut self, new_value: &str) -> MylibraryAnnotationDeleteCall<'a, S> {
        self._annotation_id = new_value.to_string();
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> MylibraryAnnotationDeleteCall<'a, S> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> MylibraryAnnotationDeleteCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryAnnotationDeleteCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> MylibraryAnnotationDeleteCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> MylibraryAnnotationDeleteCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> MylibraryAnnotationDeleteCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Inserts a new annotation.
///
/// A builder for the *annotations.insert* method supported by a *mylibrary* resource.
/// It is not used directly, but through a [`MylibraryMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// use books1::api::Annotation;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Annotation::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().annotations_insert(req)
///              .source("nonumy")
///              .show_only_summary_in_response(false)
///              .country("sadipscing")
///              .annotation_id("aliquyam")
///              .doit().await;
/// # }
/// ```
pub struct MylibraryAnnotationInsertCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _request: Annotation,
    _source: Option<String>,
    _show_only_summary_in_response: Option<bool>,
    _country: Option<String>,
    _annotation_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for MylibraryAnnotationInsertCall<'a, S> {}

impl<'a, S> MylibraryAnnotationInsertCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Annotation)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.mylibrary.annotations.insert",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "source", "showOnlySummaryInResponse", "country", "annotationId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(7 + self._additional_params.len());
        if let Some(value) = self._source.as_ref() {
            params.push("source", value);
        }
        if let Some(value) = self._show_only_summary_in_response.as_ref() {
            params.push("showOnlySummaryInResponse", value.to_string());
        }
        if let Some(value) = self._country.as_ref() {
            params.push("country", value);
        }
        if let Some(value) = self._annotation_id.as_ref() {
            params.push("annotationId", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/mylibrary/annotations";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Annotation) -> MylibraryAnnotationInsertCall<'a, S> {
        self._request = new_value;
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> MylibraryAnnotationInsertCall<'a, S> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Requests that only the summary of the specified layer be provided in the response.
    ///
    /// Sets the *show only summary in response* query property to the given value.
    pub fn show_only_summary_in_response(mut self, new_value: bool) -> MylibraryAnnotationInsertCall<'a, S> {
        self._show_only_summary_in_response = Some(new_value);
        self
    }
    /// ISO-3166-1 code to override the IP-based location.
    ///
    /// Sets the *country* query property to the given value.
    pub fn country(mut self, new_value: &str) -> MylibraryAnnotationInsertCall<'a, S> {
        self._country = Some(new_value.to_string());
        self
    }
    /// The ID for the annotation to insert.
    ///
    /// Sets the *annotation id* query property to the given value.
    pub fn annotation_id(mut self, new_value: &str) -> MylibraryAnnotationInsertCall<'a, S> {
        self._annotation_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> MylibraryAnnotationInsertCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryAnnotationInsertCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> MylibraryAnnotationInsertCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> MylibraryAnnotationInsertCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> MylibraryAnnotationInsertCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Retrieves a list of annotations, possibly filtered.
///
/// A builder for the *annotations.list* method supported by a *mylibrary* resource.
/// It is not used directly, but through a [`MylibraryMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().annotations_list()
///              .volume_id("dolores")
///              .updated_min("sadipscing")
///              .updated_max("erat")
///              .source("aliquyam")
///              .show_deleted(true)
///              .page_token("est")
///              .max_results(77)
///              .add_layer_ids("sea")
///              .layer_id("consetetur")
///              .content_version("consetetur")
///              .doit().await;
/// # }
/// ```
pub struct MylibraryAnnotationListCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _volume_id: Option<String>,
    _updated_min: Option<String>,
    _updated_max: Option<String>,
    _source: Option<String>,
    _show_deleted: Option<bool>,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _layer_ids: Vec<String>,
    _layer_id: Option<String>,
    _content_version: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for MylibraryAnnotationListCall<'a, S> {}

impl<'a, S> MylibraryAnnotationListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Annotations)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.mylibrary.annotations.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "volumeId", "updatedMin", "updatedMax", "source", "showDeleted", "pageToken", "maxResults", "layerIds", "layerId", "contentVersion"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(12 + self._additional_params.len());
        if let Some(value) = self._volume_id.as_ref() {
            params.push("volumeId", value);
        }
        if let Some(value) = self._updated_min.as_ref() {
            params.push("updatedMin", value);
        }
        if let Some(value) = self._updated_max.as_ref() {
            params.push("updatedMax", value);
        }
        if let Some(value) = self._source.as_ref() {
            params.push("source", value);
        }
        if let Some(value) = self._show_deleted.as_ref() {
            params.push("showDeleted", value.to_string());
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._max_results.as_ref() {
            params.push("maxResults", value.to_string());
        }
        if self._layer_ids.len() > 0 {
            for f in self._layer_ids.iter() {
                params.push("layerIds", f);
            }
        }
        if let Some(value) = self._layer_id.as_ref() {
            params.push("layerId", value);
        }
        if let Some(value) = self._content_version.as_ref() {
            params.push("contentVersion", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/mylibrary/annotations";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The volume to restrict annotations to.
    ///
    /// Sets the *volume id* query property to the given value.
    pub fn volume_id(mut self, new_value: &str) -> MylibraryAnnotationListCall<'a, S> {
        self._volume_id = Some(new_value.to_string());
        self
    }
    /// RFC 3339 timestamp to restrict to items updated since this timestamp (inclusive).
    ///
    /// Sets the *updated min* query property to the given value.
    pub fn updated_min(mut self, new_value: &str) -> MylibraryAnnotationListCall<'a, S> {
        self._updated_min = Some(new_value.to_string());
        self
    }
    /// RFC 3339 timestamp to restrict to items updated prior to this timestamp (exclusive).
    ///
    /// Sets the *updated max* query property to the given value.
    pub fn updated_max(mut self, new_value: &str) -> MylibraryAnnotationListCall<'a, S> {
        self._updated_max = Some(new_value.to_string());
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> MylibraryAnnotationListCall<'a, S> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Set to true to return deleted annotations. updatedMin must be in the request to use this. Defaults to false.
    ///
    /// Sets the *show deleted* query property to the given value.
    pub fn show_deleted(mut self, new_value: bool) -> MylibraryAnnotationListCall<'a, S> {
        self._show_deleted = Some(new_value);
        self
    }
    /// The value of the nextToken from the previous page.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> MylibraryAnnotationListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of results to return
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> MylibraryAnnotationListCall<'a, S> {
        self._max_results = Some(new_value);
        self
    }
    /// The layer ID(s) to limit annotation by.
    ///
    /// Append the given value to the *layer ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_layer_ids(mut self, new_value: &str) -> MylibraryAnnotationListCall<'a, S> {
        self._layer_ids.push(new_value.to_string());
        self
    }
    /// The layer ID to limit annotation by.
    ///
    /// Sets the *layer id* query property to the given value.
    pub fn layer_id(mut self, new_value: &str) -> MylibraryAnnotationListCall<'a, S> {
        self._layer_id = Some(new_value.to_string());
        self
    }
    /// The content version for the requested volume.
    ///
    /// Sets the *content version* query property to the given value.
    pub fn content_version(mut self, new_value: &str) -> MylibraryAnnotationListCall<'a, S> {
        self._content_version = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> MylibraryAnnotationListCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryAnnotationListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> MylibraryAnnotationListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> MylibraryAnnotationListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> MylibraryAnnotationListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Gets the summary of specified layers.
///
/// A builder for the *annotations.summary* method supported by a *mylibrary* resource.
/// It is not used directly, but through a [`MylibraryMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().annotations_summary(&vec!["Stet".into()], "volumeId")
///              .source("aliquyam")
///              .doit().await;
/// # }
/// ```
pub struct MylibraryAnnotationSummaryCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _layer_ids: Vec<String>,
    _volume_id: String,
    _source: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for MylibraryAnnotationSummaryCall<'a, S> {}

impl<'a, S> MylibraryAnnotationSummaryCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, AnnotationsSummary)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.mylibrary.annotations.summary",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "layerIds", "volumeId", "source"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        if self._layer_ids.len() > 0 {
            for f in self._layer_ids.iter() {
                params.push("layerIds", f);
            }
        }
        params.push("volumeId", self._volume_id);
        if let Some(value) = self._source.as_ref() {
            params.push("source", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/mylibrary/annotations/summary";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Array of layer IDs to get the summary for.
    ///
    /// Append the given value to the *layer ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn add_layer_ids(mut self, new_value: &str) -> MylibraryAnnotationSummaryCall<'a, S> {
        self._layer_ids.push(new_value.to_string());
        self
    }
    /// Volume id to get the summary for.
    ///
    /// Sets the *volume id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn volume_id(mut self, new_value: &str) -> MylibraryAnnotationSummaryCall<'a, S> {
        self._volume_id = new_value.to_string();
        self
    }
    /// Optional. String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> MylibraryAnnotationSummaryCall<'a, S> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> MylibraryAnnotationSummaryCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryAnnotationSummaryCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> MylibraryAnnotationSummaryCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> MylibraryAnnotationSummaryCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> MylibraryAnnotationSummaryCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Updates an existing annotation.
///
/// A builder for the *annotations.update* method supported by a *mylibrary* resource.
/// It is not used directly, but through a [`MylibraryMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// use books1::api::Annotation;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Annotation::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().annotations_update(req, "annotationId")
///              .source("duo")
///              .doit().await;
/// # }
/// ```
pub struct MylibraryAnnotationUpdateCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _request: Annotation,
    _annotation_id: String,
    _source: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for MylibraryAnnotationUpdateCall<'a, S> {}

impl<'a, S> MylibraryAnnotationUpdateCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Annotation)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.mylibrary.annotations.update",
                               http_method: hyper::Method::PUT });

        for &field in ["alt", "annotationId", "source"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("annotationId", self._annotation_id);
        if let Some(value) = self._source.as_ref() {
            params.push("source", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/mylibrary/annotations/{annotationId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{annotationId}", "annotationId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["annotationId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::PUT)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Annotation) -> MylibraryAnnotationUpdateCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID for the annotation to update.
    ///
    /// Sets the *annotation id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn annotation_id(mut self, new_value: &str) -> MylibraryAnnotationUpdateCall<'a, S> {
        self._annotation_id = new_value.to_string();
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> MylibraryAnnotationUpdateCall<'a, S> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> MylibraryAnnotationUpdateCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryAnnotationUpdateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> MylibraryAnnotationUpdateCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> MylibraryAnnotationUpdateCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> MylibraryAnnotationUpdateCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Gets volume information for volumes on a bookshelf.
///
/// A builder for the *bookshelves.volumes.list* method supported by a *mylibrary* resource.
/// It is not used directly, but through a [`MylibraryMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().bookshelves_volumes_list("shelf")
///              .start_index(44)
///              .source("sit")
///              .show_preorders(false)
///              .q("eos")
///              .projection("Lorem")
///              .max_results(84)
///              .country("Stet")
///              .doit().await;
/// # }
/// ```
pub struct MylibraryBookshelfVolumeListCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _shelf: String,
    _start_index: Option<u32>,
    _source: Option<String>,
    _show_preorders: Option<bool>,
    _q: Option<String>,
    _projection: Option<String>,
    _max_results: Option<u32>,
    _country: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for MylibraryBookshelfVolumeListCall<'a, S> {}

impl<'a, S> MylibraryBookshelfVolumeListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Volumes)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.mylibrary.bookshelves.volumes.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "shelf", "startIndex", "source", "showPreorders", "q", "projection", "maxResults", "country"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(10 + self._additional_params.len());
        params.push("shelf", self._shelf);
        if let Some(value) = self._start_index.as_ref() {
            params.push("startIndex", value.to_string());
        }
        if let Some(value) = self._source.as_ref() {
            params.push("source", value);
        }
        if let Some(value) = self._show_preorders.as_ref() {
            params.push("showPreorders", value.to_string());
        }
        if let Some(value) = self._q.as_ref() {
            params.push("q", value);
        }
        if let Some(value) = self._projection.as_ref() {
            params.push("projection", value);
        }
        if let Some(value) = self._max_results.as_ref() {
            params.push("maxResults", value.to_string());
        }
        if let Some(value) = self._country.as_ref() {
            params.push("country", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/mylibrary/bookshelves/{shelf}/volumes";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{shelf}", "shelf")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["shelf"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The bookshelf ID or name retrieve volumes for.
    ///
    /// Sets the *shelf* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn shelf(mut self, new_value: &str) -> MylibraryBookshelfVolumeListCall<'a, S> {
        self._shelf = new_value.to_string();
        self
    }
    /// Index of the first element to return (starts at 0)
    ///
    /// Sets the *start index* query property to the given value.
    pub fn start_index(mut self, new_value: u32) -> MylibraryBookshelfVolumeListCall<'a, S> {
        self._start_index = Some(new_value);
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> MylibraryBookshelfVolumeListCall<'a, S> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Set to true to show pre-ordered books. Defaults to false.
    ///
    /// Sets the *show preorders* query property to the given value.
    pub fn show_preorders(mut self, new_value: bool) -> MylibraryBookshelfVolumeListCall<'a, S> {
        self._show_preorders = Some(new_value);
        self
    }
    /// Full-text search query string in this bookshelf.
    ///
    /// Sets the *q* query property to the given value.
    pub fn q(mut self, new_value: &str) -> MylibraryBookshelfVolumeListCall<'a, S> {
        self._q = Some(new_value.to_string());
        self
    }
    /// Restrict information returned to a set of selected fields.
    ///
    /// Sets the *projection* query property to the given value.
    pub fn projection(mut self, new_value: &str) -> MylibraryBookshelfVolumeListCall<'a, S> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Maximum number of results to return
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> MylibraryBookshelfVolumeListCall<'a, S> {
        self._max_results = Some(new_value);
        self
    }
    /// ISO-3166-1 code to override the IP-based location.
    ///
    /// Sets the *country* query property to the given value.
    pub fn country(mut self, new_value: &str) -> MylibraryBookshelfVolumeListCall<'a, S> {
        self._country = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> MylibraryBookshelfVolumeListCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryBookshelfVolumeListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> MylibraryBookshelfVolumeListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> MylibraryBookshelfVolumeListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> MylibraryBookshelfVolumeListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Adds a volume to a bookshelf.
///
/// A builder for the *bookshelves.addVolume* method supported by a *mylibrary* resource.
/// It is not used directly, but through a [`MylibraryMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().bookshelves_add_volume("shelf", "volumeId")
///              .source("et")
///              .reason("sea")
///              .doit().await;
/// # }
/// ```
pub struct MylibraryBookshelfAddVolumeCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _shelf: String,
    _volume_id: String,
    _source: Option<String>,
    _reason: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for MylibraryBookshelfAddVolumeCall<'a, S> {}

impl<'a, S> MylibraryBookshelfAddVolumeCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Empty)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.mylibrary.bookshelves.addVolume",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "shelf", "volumeId", "source", "reason"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("shelf", self._shelf);
        params.push("volumeId", self._volume_id);
        if let Some(value) = self._source.as_ref() {
            params.push("source", value);
        }
        if let Some(value) = self._reason.as_ref() {
            params.push("reason", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/mylibrary/bookshelves/{shelf}/addVolume";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{shelf}", "shelf")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["shelf"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// ID of bookshelf to which to add a volume.
    ///
    /// Sets the *shelf* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn shelf(mut self, new_value: &str) -> MylibraryBookshelfAddVolumeCall<'a, S> {
        self._shelf = new_value.to_string();
        self
    }
    /// ID of volume to add.
    ///
    /// Sets the *volume id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn volume_id(mut self, new_value: &str) -> MylibraryBookshelfAddVolumeCall<'a, S> {
        self._volume_id = new_value.to_string();
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> MylibraryBookshelfAddVolumeCall<'a, S> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The reason for which the book is added to the library.
    ///
    /// Sets the *reason* query property to the given value.
    pub fn reason(mut self, new_value: &str) -> MylibraryBookshelfAddVolumeCall<'a, S> {
        self._reason = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> MylibraryBookshelfAddVolumeCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryBookshelfAddVolumeCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> MylibraryBookshelfAddVolumeCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> MylibraryBookshelfAddVolumeCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> MylibraryBookshelfAddVolumeCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Clears all volumes from a bookshelf.
///
/// A builder for the *bookshelves.clearVolumes* method supported by a *mylibrary* resource.
/// It is not used directly, but through a [`MylibraryMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().bookshelves_clear_volumes("shelf")
///              .source("At")
///              .doit().await;
/// # }
/// ```
pub struct MylibraryBookshelfClearVolumeCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _shelf: String,
    _source: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for MylibraryBookshelfClearVolumeCall<'a, S> {}

impl<'a, S> MylibraryBookshelfClearVolumeCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Empty)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.mylibrary.bookshelves.clearVolumes",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "shelf", "source"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("shelf", self._shelf);
        if let Some(value) = self._source.as_ref() {
            params.push("source", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/mylibrary/bookshelves/{shelf}/clearVolumes";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{shelf}", "shelf")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["shelf"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// ID of bookshelf from which to remove a volume.
    ///
    /// Sets the *shelf* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn shelf(mut self, new_value: &str) -> MylibraryBookshelfClearVolumeCall<'a, S> {
        self._shelf = new_value.to_string();
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> MylibraryBookshelfClearVolumeCall<'a, S> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> MylibraryBookshelfClearVolumeCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryBookshelfClearVolumeCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> MylibraryBookshelfClearVolumeCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> MylibraryBookshelfClearVolumeCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> MylibraryBookshelfClearVolumeCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Retrieves metadata for a specific bookshelf belonging to the authenticated user.
///
/// A builder for the *bookshelves.get* method supported by a *mylibrary* resource.
/// It is not used directly, but through a [`MylibraryMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().bookshelves_get("shelf")
///              .source("eirmod")
///              .doit().await;
/// # }
/// ```
pub struct MylibraryBookshelfGetCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _shelf: String,
    _source: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for MylibraryBookshelfGetCall<'a, S> {}

impl<'a, S> MylibraryBookshelfGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Bookshelf)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.mylibrary.bookshelves.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "shelf", "source"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("shelf", self._shelf);
        if let Some(value) = self._source.as_ref() {
            params.push("source", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/mylibrary/bookshelves/{shelf}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{shelf}", "shelf")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["shelf"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// ID of bookshelf to retrieve.
    ///
    /// Sets the *shelf* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn shelf(mut self, new_value: &str) -> MylibraryBookshelfGetCall<'a, S> {
        self._shelf = new_value.to_string();
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> MylibraryBookshelfGetCall<'a, S> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> MylibraryBookshelfGetCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryBookshelfGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> MylibraryBookshelfGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> MylibraryBookshelfGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> MylibraryBookshelfGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Retrieves a list of bookshelves belonging to the authenticated user.
///
/// A builder for the *bookshelves.list* method supported by a *mylibrary* resource.
/// It is not used directly, but through a [`MylibraryMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().bookshelves_list()
///              .source("Lorem")
///              .doit().await;
/// # }
/// ```
pub struct MylibraryBookshelfListCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _source: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for MylibraryBookshelfListCall<'a, S> {}

impl<'a, S> MylibraryBookshelfListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Bookshelves)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.mylibrary.bookshelves.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "source"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        if let Some(value) = self._source.as_ref() {
            params.push("source", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/mylibrary/bookshelves";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> MylibraryBookshelfListCall<'a, S> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> MylibraryBookshelfListCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryBookshelfListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> MylibraryBookshelfListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> MylibraryBookshelfListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> MylibraryBookshelfListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Moves a volume within a bookshelf.
///
/// A builder for the *bookshelves.moveVolume* method supported by a *mylibrary* resource.
/// It is not used directly, but through a [`MylibraryMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().bookshelves_move_volume("shelf", "volumeId", -31)
///              .source("dolores")
///              .doit().await;
/// # }
/// ```
pub struct MylibraryBookshelfMoveVolumeCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _shelf: String,
    _volume_id: String,
    _volume_position: i32,
    _source: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for MylibraryBookshelfMoveVolumeCall<'a, S> {}

impl<'a, S> MylibraryBookshelfMoveVolumeCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Empty)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.mylibrary.bookshelves.moveVolume",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "shelf", "volumeId", "volumePosition", "source"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("shelf", self._shelf);
        params.push("volumeId", self._volume_id);
        params.push("volumePosition", self._volume_position.to_string());
        if let Some(value) = self._source.as_ref() {
            params.push("source", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/mylibrary/bookshelves/{shelf}/moveVolume";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{shelf}", "shelf")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["shelf"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// ID of bookshelf with the volume.
    ///
    /// Sets the *shelf* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn shelf(mut self, new_value: &str) -> MylibraryBookshelfMoveVolumeCall<'a, S> {
        self._shelf = new_value.to_string();
        self
    }
    /// ID of volume to move.
    ///
    /// Sets the *volume id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn volume_id(mut self, new_value: &str) -> MylibraryBookshelfMoveVolumeCall<'a, S> {
        self._volume_id = new_value.to_string();
        self
    }
    /// Position on shelf to move the item (0 puts the item before the current first item, 1 puts it between the first and the second and so on.)
    ///
    /// Sets the *volume position* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn volume_position(mut self, new_value: i32) -> MylibraryBookshelfMoveVolumeCall<'a, S> {
        self._volume_position = new_value;
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> MylibraryBookshelfMoveVolumeCall<'a, S> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> MylibraryBookshelfMoveVolumeCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryBookshelfMoveVolumeCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> MylibraryBookshelfMoveVolumeCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> MylibraryBookshelfMoveVolumeCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> MylibraryBookshelfMoveVolumeCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Removes a volume from a bookshelf.
///
/// A builder for the *bookshelves.removeVolume* method supported by a *mylibrary* resource.
/// It is not used directly, but through a [`MylibraryMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().bookshelves_remove_volume("shelf", "volumeId")
///              .source("sea")
///              .reason("takimata")
///              .doit().await;
/// # }
/// ```
pub struct MylibraryBookshelfRemoveVolumeCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _shelf: String,
    _volume_id: String,
    _source: Option<String>,
    _reason: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for MylibraryBookshelfRemoveVolumeCall<'a, S> {}

impl<'a, S> MylibraryBookshelfRemoveVolumeCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Empty)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.mylibrary.bookshelves.removeVolume",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "shelf", "volumeId", "source", "reason"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("shelf", self._shelf);
        params.push("volumeId", self._volume_id);
        if let Some(value) = self._source.as_ref() {
            params.push("source", value);
        }
        if let Some(value) = self._reason.as_ref() {
            params.push("reason", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/mylibrary/bookshelves/{shelf}/removeVolume";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{shelf}", "shelf")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["shelf"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// ID of bookshelf from which to remove a volume.
    ///
    /// Sets the *shelf* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn shelf(mut self, new_value: &str) -> MylibraryBookshelfRemoveVolumeCall<'a, S> {
        self._shelf = new_value.to_string();
        self
    }
    /// ID of volume to remove.
    ///
    /// Sets the *volume id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn volume_id(mut self, new_value: &str) -> MylibraryBookshelfRemoveVolumeCall<'a, S> {
        self._volume_id = new_value.to_string();
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> MylibraryBookshelfRemoveVolumeCall<'a, S> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The reason for which the book is removed from the library.
    ///
    /// Sets the *reason* query property to the given value.
    pub fn reason(mut self, new_value: &str) -> MylibraryBookshelfRemoveVolumeCall<'a, S> {
        self._reason = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> MylibraryBookshelfRemoveVolumeCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryBookshelfRemoveVolumeCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> MylibraryBookshelfRemoveVolumeCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> MylibraryBookshelfRemoveVolumeCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> MylibraryBookshelfRemoveVolumeCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Retrieves my reading position information for a volume.
///
/// A builder for the *readingpositions.get* method supported by a *mylibrary* resource.
/// It is not used directly, but through a [`MylibraryMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().readingpositions_get("volumeId")
///              .source("et")
///              .content_version("At")
///              .doit().await;
/// # }
/// ```
pub struct MylibraryReadingpositionGetCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _volume_id: String,
    _source: Option<String>,
    _content_version: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for MylibraryReadingpositionGetCall<'a, S> {}

impl<'a, S> MylibraryReadingpositionGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ReadingPosition)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.mylibrary.readingpositions.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "volumeId", "source", "contentVersion"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("volumeId", self._volume_id);
        if let Some(value) = self._source.as_ref() {
            params.push("source", value);
        }
        if let Some(value) = self._content_version.as_ref() {
            params.push("contentVersion", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/mylibrary/readingpositions/{volumeId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{volumeId}", "volumeId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["volumeId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// ID of volume for which to retrieve a reading position.
    ///
    /// Sets the *volume id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn volume_id(mut self, new_value: &str) -> MylibraryReadingpositionGetCall<'a, S> {
        self._volume_id = new_value.to_string();
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> MylibraryReadingpositionGetCall<'a, S> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Volume content version for which this reading position is requested.
    ///
    /// Sets the *content version* query property to the given value.
    pub fn content_version(mut self, new_value: &str) -> MylibraryReadingpositionGetCall<'a, S> {
        self._content_version = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> MylibraryReadingpositionGetCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryReadingpositionGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> MylibraryReadingpositionGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> MylibraryReadingpositionGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> MylibraryReadingpositionGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Sets my reading position information for a volume.
///
/// A builder for the *readingpositions.setPosition* method supported by a *mylibrary* resource.
/// It is not used directly, but through a [`MylibraryMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().readingpositions_set_position("volumeId", "position", "timestamp")
///              .source("erat")
///              .device_cookie("sea")
///              .content_version("nonumy")
///              .action("et")
///              .doit().await;
/// # }
/// ```
pub struct MylibraryReadingpositionSetPositionCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _volume_id: String,
    _position: String,
    _timestamp: String,
    _source: Option<String>,
    _device_cookie: Option<String>,
    _content_version: Option<String>,
    _action: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for MylibraryReadingpositionSetPositionCall<'a, S> {}

impl<'a, S> MylibraryReadingpositionSetPositionCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Empty)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.mylibrary.readingpositions.setPosition",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "volumeId", "position", "timestamp", "source", "deviceCookie", "contentVersion", "action"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(9 + self._additional_params.len());
        params.push("volumeId", self._volume_id);
        params.push("position", self._position);
        params.push("timestamp", self._timestamp);
        if let Some(value) = self._source.as_ref() {
            params.push("source", value);
        }
        if let Some(value) = self._device_cookie.as_ref() {
            params.push("deviceCookie", value);
        }
        if let Some(value) = self._content_version.as_ref() {
            params.push("contentVersion", value);
        }
        if let Some(value) = self._action.as_ref() {
            params.push("action", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/mylibrary/readingpositions/{volumeId}/setPosition";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{volumeId}", "volumeId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["volumeId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// ID of volume for which to update the reading position.
    ///
    /// Sets the *volume id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn volume_id(mut self, new_value: &str) -> MylibraryReadingpositionSetPositionCall<'a, S> {
        self._volume_id = new_value.to_string();
        self
    }
    /// Position string for the new volume reading position.
    ///
    /// Sets the *position* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn position(mut self, new_value: &str) -> MylibraryReadingpositionSetPositionCall<'a, S> {
        self._position = new_value.to_string();
        self
    }
    /// RFC 3339 UTC format timestamp associated with this reading position.
    ///
    /// Sets the *timestamp* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn timestamp(mut self, new_value: &str) -> MylibraryReadingpositionSetPositionCall<'a, S> {
        self._timestamp = new_value.to_string();
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> MylibraryReadingpositionSetPositionCall<'a, S> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Random persistent device cookie optional on set position.
    ///
    /// Sets the *device cookie* query property to the given value.
    pub fn device_cookie(mut self, new_value: &str) -> MylibraryReadingpositionSetPositionCall<'a, S> {
        self._device_cookie = Some(new_value.to_string());
        self
    }
    /// Volume content version for which this reading position applies.
    ///
    /// Sets the *content version* query property to the given value.
    pub fn content_version(mut self, new_value: &str) -> MylibraryReadingpositionSetPositionCall<'a, S> {
        self._content_version = Some(new_value.to_string());
        self
    }
    /// Action that caused this reading position to be set.
    ///
    /// Sets the *action* query property to the given value.
    pub fn action(mut self, new_value: &str) -> MylibraryReadingpositionSetPositionCall<'a, S> {
        self._action = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> MylibraryReadingpositionSetPositionCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryReadingpositionSetPositionCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> MylibraryReadingpositionSetPositionCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> MylibraryReadingpositionSetPositionCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> MylibraryReadingpositionSetPositionCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Returns notification details for a given notification id.
///
/// A builder for the *get* method supported by a *notification* resource.
/// It is not used directly, but through a [`NotificationMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.notification().get("notification_id")
///              .source("justo")
///              .locale("sea")
///              .doit().await;
/// # }
/// ```
pub struct NotificationGetCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _notification_id: String,
    _source: Option<String>,
    _locale: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for NotificationGetCall<'a, S> {}

impl<'a, S> NotificationGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Notification)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.notification.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "notification_id", "source", "locale"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("notification_id", self._notification_id);
        if let Some(value) = self._source.as_ref() {
            params.push("source", value);
        }
        if let Some(value) = self._locale.as_ref() {
            params.push("locale", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/notification/get";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// String to identify the notification.
    ///
    /// Sets the *notification_id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn notification_id(mut self, new_value: &str) -> NotificationGetCall<'a, S> {
        self._notification_id = new_value.to_string();
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> NotificationGetCall<'a, S> {
        self._source = Some(new_value.to_string());
        self
    }
    /// ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'. Used for generating notification title and body.
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> NotificationGetCall<'a, S> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> NotificationGetCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> NotificationGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> NotificationGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> NotificationGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> NotificationGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// List categories for onboarding experience.
///
/// A builder for the *listCategories* method supported by a *onboarding* resource.
/// It is not used directly, but through a [`OnboardingMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.onboarding().list_categories()
///              .locale("consetetur")
///              .doit().await;
/// # }
/// ```
pub struct OnboardingListCategoryCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _locale: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for OnboardingListCategoryCall<'a, S> {}

impl<'a, S> OnboardingListCategoryCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Category)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.onboarding.listCategories",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "locale"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        if let Some(value) = self._locale.as_ref() {
            params.push("locale", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/onboarding/listCategories";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// ISO-639-1 language and ISO-3166-1 country code. Default is en-US if unset.
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> OnboardingListCategoryCall<'a, S> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OnboardingListCategoryCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> OnboardingListCategoryCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> OnboardingListCategoryCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> OnboardingListCategoryCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> OnboardingListCategoryCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// List available volumes under categories for onboarding experience.
///
/// A builder for the *listCategoryVolumes* method supported by a *onboarding* resource.
/// It is not used directly, but through a [`OnboardingMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.onboarding().list_category_volumes()
///              .page_token("sit")
///              .page_size(69)
///              .max_allowed_maturity_rating("eos")
///              .locale("At")
///              .add_category_id("dolores")
///              .doit().await;
/// # }
/// ```
pub struct OnboardingListCategoryVolumeCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _page_token: Option<String>,
    _page_size: Option<u32>,
    _max_allowed_maturity_rating: Option<String>,
    _locale: Option<String>,
    _category_id: Vec<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for OnboardingListCategoryVolumeCall<'a, S> {}

impl<'a, S> OnboardingListCategoryVolumeCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Volume2)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.onboarding.listCategoryVolumes",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "pageToken", "pageSize", "maxAllowedMaturityRating", "locale", "categoryId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(7 + self._additional_params.len());
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }
        if let Some(value) = self._max_allowed_maturity_rating.as_ref() {
            params.push("maxAllowedMaturityRating", value);
        }
        if let Some(value) = self._locale.as_ref() {
            params.push("locale", value);
        }
        if self._category_id.len() > 0 {
            for f in self._category_id.iter() {
                params.push("categoryId", f);
            }
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/onboarding/listCategoryVolumes";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The value of the nextToken from the previous page.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> OnboardingListCategoryVolumeCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Number of maximum results per page to be included in the response.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: u32) -> OnboardingListCategoryVolumeCall<'a, S> {
        self._page_size = Some(new_value);
        self
    }
    /// The maximum allowed maturity rating of returned volumes. Books with a higher maturity rating are filtered out.
    ///
    /// Sets the *max allowed maturity rating* query property to the given value.
    pub fn max_allowed_maturity_rating(mut self, new_value: &str) -> OnboardingListCategoryVolumeCall<'a, S> {
        self._max_allowed_maturity_rating = Some(new_value.to_string());
        self
    }
    /// ISO-639-1 language and ISO-3166-1 country code. Default is en-US if unset.
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> OnboardingListCategoryVolumeCall<'a, S> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// List of category ids requested.
    ///
    /// Append the given value to the *category id* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_category_id(mut self, new_value: &str) -> OnboardingListCategoryVolumeCall<'a, S> {
        self._category_id.push(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OnboardingListCategoryVolumeCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> OnboardingListCategoryVolumeCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> OnboardingListCategoryVolumeCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> OnboardingListCategoryVolumeCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> OnboardingListCategoryVolumeCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Returns a stream of personalized book clusters
///
/// A builder for the *get* method supported by a *personalizedstream* resource.
/// It is not used directly, but through a [`PersonalizedstreamMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.personalizedstream().get()
///              .source("consetetur")
///              .max_allowed_maturity_rating("gubergren")
///              .locale("dolor")
///              .doit().await;
/// # }
/// ```
pub struct PersonalizedstreamGetCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _source: Option<String>,
    _max_allowed_maturity_rating: Option<String>,
    _locale: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for PersonalizedstreamGetCall<'a, S> {}

impl<'a, S> PersonalizedstreamGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Discoveryclusters)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.personalizedstream.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "source", "maxAllowedMaturityRating", "locale"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        if let Some(value) = self._source.as_ref() {
            params.push("source", value);
        }
        if let Some(value) = self._max_allowed_maturity_rating.as_ref() {
            params.push("maxAllowedMaturityRating", value);
        }
        if let Some(value) = self._locale.as_ref() {
            params.push("locale", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/personalizedstream/get";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> PersonalizedstreamGetCall<'a, S> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The maximum allowed maturity rating of returned recommendations. Books with a higher maturity rating are filtered out.
    ///
    /// Sets the *max allowed maturity rating* query property to the given value.
    pub fn max_allowed_maturity_rating(mut self, new_value: &str) -> PersonalizedstreamGetCall<'a, S> {
        self._max_allowed_maturity_rating = Some(new_value.to_string());
        self
    }
    /// ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'. Used for generating recommendations.
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> PersonalizedstreamGetCall<'a, S> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> PersonalizedstreamGetCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> PersonalizedstreamGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> PersonalizedstreamGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> PersonalizedstreamGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> PersonalizedstreamGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Accepts the promo offer.
///
/// A builder for the *accept* method supported by a *promooffer* resource.
/// It is not used directly, but through a [`PromoofferMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.promooffer().accept()
///              .volume_id("aliquyam")
///              .serial("no")
///              .product("amet.")
///              .offer_id("ipsum")
///              .model("Lorem")
///              .manufacturer("accusam")
///              .device("gubergren")
///              .android_id("sadipscing")
///              .doit().await;
/// # }
/// ```
pub struct PromoofferAcceptCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _volume_id: Option<String>,
    _serial: Option<String>,
    _product: Option<String>,
    _offer_id: Option<String>,
    _model: Option<String>,
    _manufacturer: Option<String>,
    _device: Option<String>,
    _android_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for PromoofferAcceptCall<'a, S> {}

impl<'a, S> PromoofferAcceptCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Empty)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.promooffer.accept",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "volumeId", "serial", "product", "offerId", "model", "manufacturer", "device", "androidId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(10 + self._additional_params.len());
        if let Some(value) = self._volume_id.as_ref() {
            params.push("volumeId", value);
        }
        if let Some(value) = self._serial.as_ref() {
            params.push("serial", value);
        }
        if let Some(value) = self._product.as_ref() {
            params.push("product", value);
        }
        if let Some(value) = self._offer_id.as_ref() {
            params.push("offerId", value);
        }
        if let Some(value) = self._model.as_ref() {
            params.push("model", value);
        }
        if let Some(value) = self._manufacturer.as_ref() {
            params.push("manufacturer", value);
        }
        if let Some(value) = self._device.as_ref() {
            params.push("device", value);
        }
        if let Some(value) = self._android_id.as_ref() {
            params.push("androidId", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/promooffer/accept";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Volume id to exercise the offer
    ///
    /// Sets the *volume id* query property to the given value.
    pub fn volume_id(mut self, new_value: &str) -> PromoofferAcceptCall<'a, S> {
        self._volume_id = Some(new_value.to_string());
        self
    }
    /// device serial
    ///
    /// Sets the *serial* query property to the given value.
    pub fn serial(mut self, new_value: &str) -> PromoofferAcceptCall<'a, S> {
        self._serial = Some(new_value.to_string());
        self
    }
    /// device product
    ///
    /// Sets the *product* query property to the given value.
    pub fn product(mut self, new_value: &str) -> PromoofferAcceptCall<'a, S> {
        self._product = Some(new_value.to_string());
        self
    }
    ///
    /// Sets the *offer id* query property to the given value.
    pub fn offer_id(mut self, new_value: &str) -> PromoofferAcceptCall<'a, S> {
        self._offer_id = Some(new_value.to_string());
        self
    }
    /// device model
    ///
    /// Sets the *model* query property to the given value.
    pub fn model(mut self, new_value: &str) -> PromoofferAcceptCall<'a, S> {
        self._model = Some(new_value.to_string());
        self
    }
    /// device manufacturer
    ///
    /// Sets the *manufacturer* query property to the given value.
    pub fn manufacturer(mut self, new_value: &str) -> PromoofferAcceptCall<'a, S> {
        self._manufacturer = Some(new_value.to_string());
        self
    }
    /// device device
    ///
    /// Sets the *device* query property to the given value.
    pub fn device(mut self, new_value: &str) -> PromoofferAcceptCall<'a, S> {
        self._device = Some(new_value.to_string());
        self
    }
    /// device android_id
    ///
    /// Sets the *android id* query property to the given value.
    pub fn android_id(mut self, new_value: &str) -> PromoofferAcceptCall<'a, S> {
        self._android_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> PromoofferAcceptCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> PromoofferAcceptCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> PromoofferAcceptCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> PromoofferAcceptCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> PromoofferAcceptCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Marks the promo offer as dismissed.
///
/// A builder for the *dismiss* method supported by a *promooffer* resource.
/// It is not used directly, but through a [`PromoofferMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.promooffer().dismiss()
///              .serial("At")
///              .product("sit")
///              .offer_id("duo")
///              .model("sit")
///              .manufacturer("magna")
///              .device("et")
///              .android_id("rebum.")
///              .doit().await;
/// # }
/// ```
pub struct PromoofferDismisCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _serial: Option<String>,
    _product: Option<String>,
    _offer_id: Option<String>,
    _model: Option<String>,
    _manufacturer: Option<String>,
    _device: Option<String>,
    _android_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for PromoofferDismisCall<'a, S> {}

impl<'a, S> PromoofferDismisCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Empty)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.promooffer.dismiss",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "serial", "product", "offerId", "model", "manufacturer", "device", "androidId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(9 + self._additional_params.len());
        if let Some(value) = self._serial.as_ref() {
            params.push("serial", value);
        }
        if let Some(value) = self._product.as_ref() {
            params.push("product", value);
        }
        if let Some(value) = self._offer_id.as_ref() {
            params.push("offerId", value);
        }
        if let Some(value) = self._model.as_ref() {
            params.push("model", value);
        }
        if let Some(value) = self._manufacturer.as_ref() {
            params.push("manufacturer", value);
        }
        if let Some(value) = self._device.as_ref() {
            params.push("device", value);
        }
        if let Some(value) = self._android_id.as_ref() {
            params.push("androidId", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/promooffer/dismiss";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// device serial
    ///
    /// Sets the *serial* query property to the given value.
    pub fn serial(mut self, new_value: &str) -> PromoofferDismisCall<'a, S> {
        self._serial = Some(new_value.to_string());
        self
    }
    /// device product
    ///
    /// Sets the *product* query property to the given value.
    pub fn product(mut self, new_value: &str) -> PromoofferDismisCall<'a, S> {
        self._product = Some(new_value.to_string());
        self
    }
    /// Offer to dimiss
    ///
    /// Sets the *offer id* query property to the given value.
    pub fn offer_id(mut self, new_value: &str) -> PromoofferDismisCall<'a, S> {
        self._offer_id = Some(new_value.to_string());
        self
    }
    /// device model
    ///
    /// Sets the *model* query property to the given value.
    pub fn model(mut self, new_value: &str) -> PromoofferDismisCall<'a, S> {
        self._model = Some(new_value.to_string());
        self
    }
    /// device manufacturer
    ///
    /// Sets the *manufacturer* query property to the given value.
    pub fn manufacturer(mut self, new_value: &str) -> PromoofferDismisCall<'a, S> {
        self._manufacturer = Some(new_value.to_string());
        self
    }
    /// device device
    ///
    /// Sets the *device* query property to the given value.
    pub fn device(mut self, new_value: &str) -> PromoofferDismisCall<'a, S> {
        self._device = Some(new_value.to_string());
        self
    }
    /// device android_id
    ///
    /// Sets the *android id* query property to the given value.
    pub fn android_id(mut self, new_value: &str) -> PromoofferDismisCall<'a, S> {
        self._android_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> PromoofferDismisCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> PromoofferDismisCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> PromoofferDismisCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> PromoofferDismisCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> PromoofferDismisCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Returns a list of promo offers available to the user
///
/// A builder for the *get* method supported by a *promooffer* resource.
/// It is not used directly, but through a [`PromoofferMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.promooffer().get()
///              .serial("dolor")
///              .product("Lorem")
///              .model("justo")
///              .manufacturer("amet.")
///              .device("no")
///              .android_id("nonumy")
///              .doit().await;
/// # }
/// ```
pub struct PromoofferGetCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _serial: Option<String>,
    _product: Option<String>,
    _model: Option<String>,
    _manufacturer: Option<String>,
    _device: Option<String>,
    _android_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for PromoofferGetCall<'a, S> {}

impl<'a, S> PromoofferGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Offers)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.promooffer.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "serial", "product", "model", "manufacturer", "device", "androidId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(8 + self._additional_params.len());
        if let Some(value) = self._serial.as_ref() {
            params.push("serial", value);
        }
        if let Some(value) = self._product.as_ref() {
            params.push("product", value);
        }
        if let Some(value) = self._model.as_ref() {
            params.push("model", value);
        }
        if let Some(value) = self._manufacturer.as_ref() {
            params.push("manufacturer", value);
        }
        if let Some(value) = self._device.as_ref() {
            params.push("device", value);
        }
        if let Some(value) = self._android_id.as_ref() {
            params.push("androidId", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/promooffer/get";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// device serial
    ///
    /// Sets the *serial* query property to the given value.
    pub fn serial(mut self, new_value: &str) -> PromoofferGetCall<'a, S> {
        self._serial = Some(new_value.to_string());
        self
    }
    /// device product
    ///
    /// Sets the *product* query property to the given value.
    pub fn product(mut self, new_value: &str) -> PromoofferGetCall<'a, S> {
        self._product = Some(new_value.to_string());
        self
    }
    /// device model
    ///
    /// Sets the *model* query property to the given value.
    pub fn model(mut self, new_value: &str) -> PromoofferGetCall<'a, S> {
        self._model = Some(new_value.to_string());
        self
    }
    /// device manufacturer
    ///
    /// Sets the *manufacturer* query property to the given value.
    pub fn manufacturer(mut self, new_value: &str) -> PromoofferGetCall<'a, S> {
        self._manufacturer = Some(new_value.to_string());
        self
    }
    /// device device
    ///
    /// Sets the *device* query property to the given value.
    pub fn device(mut self, new_value: &str) -> PromoofferGetCall<'a, S> {
        self._device = Some(new_value.to_string());
        self
    }
    /// device android_id
    ///
    /// Sets the *android id* query property to the given value.
    pub fn android_id(mut self, new_value: &str) -> PromoofferGetCall<'a, S> {
        self._android_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> PromoofferGetCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> PromoofferGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> PromoofferGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> PromoofferGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> PromoofferGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Returns Series membership data given the series id.
///
/// A builder for the *membership.get* method supported by a *series* resource.
/// It is not used directly, but through a [`SeriesMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.series().membership_get("series_id")
///              .page_token("kasd")
///              .page_size(0)
///              .doit().await;
/// # }
/// ```
pub struct SeriesMembershipGetCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _series_id: String,
    _page_token: Option<String>,
    _page_size: Option<u32>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for SeriesMembershipGetCall<'a, S> {}

impl<'a, S> SeriesMembershipGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Seriesmembership)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.series.membership.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "series_id", "page_token", "page_size"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("series_id", self._series_id);
        if let Some(value) = self._page_token.as_ref() {
            params.push("page_token", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("page_size", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/series/membership/get";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// String that identifies the series
    ///
    /// Sets the *series_id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn series_id(mut self, new_value: &str) -> SeriesMembershipGetCall<'a, S> {
        self._series_id = new_value.to_string();
        self
    }
    /// The value of the nextToken from the previous page.
    ///
    /// Sets the *page_token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> SeriesMembershipGetCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Number of maximum results per page to be included in the response.
    ///
    /// Sets the *page_size* query property to the given value.
    pub fn page_size(mut self, new_value: u32) -> SeriesMembershipGetCall<'a, S> {
        self._page_size = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> SeriesMembershipGetCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> SeriesMembershipGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> SeriesMembershipGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> SeriesMembershipGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> SeriesMembershipGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Returns Series metadata for the given series ids.
///
/// A builder for the *get* method supported by a *series* resource.
/// It is not used directly, but through a [`SeriesMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.series().get(&vec!["sanctus".into()])
///              .doit().await;
/// # }
/// ```
pub struct SeriesGetCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _series_id: Vec<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for SeriesGetCall<'a, S> {}

impl<'a, S> SeriesGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Series)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.series.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "series_id"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        if self._series_id.len() > 0 {
            for f in self._series_id.iter() {
                params.push("series_id", f);
            }
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/series/get";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// String that identifies the series
    ///
    /// Append the given value to the *series_id* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn add_series_id(mut self, new_value: &str) -> SeriesGetCall<'a, S> {
        self._series_id.push(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> SeriesGetCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> SeriesGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> SeriesGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> SeriesGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> SeriesGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Return a list of associated books.
///
/// A builder for the *associated.list* method supported by a *volume* resource.
/// It is not used directly, but through a [`VolumeMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.volumes().associated_list("volumeId")
///              .source("rebum.")
///              .max_allowed_maturity_rating("tempor")
///              .locale("dolore")
///              .association("eos")
///              .doit().await;
/// # }
/// ```
pub struct VolumeAssociatedListCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _volume_id: String,
    _source: Option<String>,
    _max_allowed_maturity_rating: Option<String>,
    _locale: Option<String>,
    _association: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for VolumeAssociatedListCall<'a, S> {}

impl<'a, S> VolumeAssociatedListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Volumes)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.volumes.associated.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "volumeId", "source", "maxAllowedMaturityRating", "locale", "association"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(7 + self._additional_params.len());
        params.push("volumeId", self._volume_id);
        if let Some(value) = self._source.as_ref() {
            params.push("source", value);
        }
        if let Some(value) = self._max_allowed_maturity_rating.as_ref() {
            params.push("maxAllowedMaturityRating", value);
        }
        if let Some(value) = self._locale.as_ref() {
            params.push("locale", value);
        }
        if let Some(value) = self._association.as_ref() {
            params.push("association", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/volumes/{volumeId}/associated";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{volumeId}", "volumeId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["volumeId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// ID of the source volume.
    ///
    /// Sets the *volume id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn volume_id(mut self, new_value: &str) -> VolumeAssociatedListCall<'a, S> {
        self._volume_id = new_value.to_string();
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> VolumeAssociatedListCall<'a, S> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The maximum allowed maturity rating of returned recommendations. Books with a higher maturity rating are filtered out.
    ///
    /// Sets the *max allowed maturity rating* query property to the given value.
    pub fn max_allowed_maturity_rating(mut self, new_value: &str) -> VolumeAssociatedListCall<'a, S> {
        self._max_allowed_maturity_rating = Some(new_value.to_string());
        self
    }
    /// ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'. Used for generating recommendations.
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> VolumeAssociatedListCall<'a, S> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// Association type.
    ///
    /// Sets the *association* query property to the given value.
    pub fn association(mut self, new_value: &str) -> VolumeAssociatedListCall<'a, S> {
        self._association = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> VolumeAssociatedListCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> VolumeAssociatedListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> VolumeAssociatedListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> VolumeAssociatedListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> VolumeAssociatedListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Return a list of books in My Library.
///
/// A builder for the *mybooks.list* method supported by a *volume* resource.
/// It is not used directly, but through a [`VolumeMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.volumes().mybooks_list()
///              .start_index(49)
///              .source("dolore")
///              .add_processing_state("amet")
///              .max_results(64)
///              .locale("At")
///              .country("sit")
///              .add_acquire_method("vero")
///              .doit().await;
/// # }
/// ```
pub struct VolumeMybookListCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _start_index: Option<u32>,
    _source: Option<String>,
    _processing_state: Vec<String>,
    _max_results: Option<u32>,
    _locale: Option<String>,
    _country: Option<String>,
    _acquire_method: Vec<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for VolumeMybookListCall<'a, S> {}

impl<'a, S> VolumeMybookListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Volumes)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.volumes.mybooks.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "startIndex", "source", "processingState", "maxResults", "locale", "country", "acquireMethod"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(9 + self._additional_params.len());
        if let Some(value) = self._start_index.as_ref() {
            params.push("startIndex", value.to_string());
        }
        if let Some(value) = self._source.as_ref() {
            params.push("source", value);
        }
        if self._processing_state.len() > 0 {
            for f in self._processing_state.iter() {
                params.push("processingState", f);
            }
        }
        if let Some(value) = self._max_results.as_ref() {
            params.push("maxResults", value.to_string());
        }
        if let Some(value) = self._locale.as_ref() {
            params.push("locale", value);
        }
        if let Some(value) = self._country.as_ref() {
            params.push("country", value);
        }
        if self._acquire_method.len() > 0 {
            for f in self._acquire_method.iter() {
                params.push("acquireMethod", f);
            }
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/volumes/mybooks";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Index of the first result to return (starts at 0)
    ///
    /// Sets the *start index* query property to the given value.
    pub fn start_index(mut self, new_value: u32) -> VolumeMybookListCall<'a, S> {
        self._start_index = Some(new_value);
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> VolumeMybookListCall<'a, S> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The processing state of the user uploaded volumes to be returned. Applicable only if the UPLOADED is specified in the acquireMethod.
    ///
    /// Append the given value to the *processing state* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_processing_state(mut self, new_value: &str) -> VolumeMybookListCall<'a, S> {
        self._processing_state.push(new_value.to_string());
        self
    }
    /// Maximum number of results to return.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> VolumeMybookListCall<'a, S> {
        self._max_results = Some(new_value);
        self
    }
    /// ISO-639-1 language and ISO-3166-1 country code. Ex:'en_US'. Used for generating recommendations.
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> VolumeMybookListCall<'a, S> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// ISO-3166-1 code to override the IP-based location.
    ///
    /// Sets the *country* query property to the given value.
    pub fn country(mut self, new_value: &str) -> VolumeMybookListCall<'a, S> {
        self._country = Some(new_value.to_string());
        self
    }
    /// How the book was acquired
    ///
    /// Append the given value to the *acquire method* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_acquire_method(mut self, new_value: &str) -> VolumeMybookListCall<'a, S> {
        self._acquire_method.push(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> VolumeMybookListCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> VolumeMybookListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> VolumeMybookListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> VolumeMybookListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> VolumeMybookListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Return a list of recommended books for the current user.
///
/// A builder for the *recommended.list* method supported by a *volume* resource.
/// It is not used directly, but through a [`VolumeMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.volumes().recommended_list()
///              .source("duo")
///              .max_allowed_maturity_rating("sadipscing")
///              .locale("ut")
///              .doit().await;
/// # }
/// ```
pub struct VolumeRecommendedListCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _source: Option<String>,
    _max_allowed_maturity_rating: Option<String>,
    _locale: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for VolumeRecommendedListCall<'a, S> {}

impl<'a, S> VolumeRecommendedListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Volumes)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.volumes.recommended.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "source", "maxAllowedMaturityRating", "locale"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        if let Some(value) = self._source.as_ref() {
            params.push("source", value);
        }
        if let Some(value) = self._max_allowed_maturity_rating.as_ref() {
            params.push("maxAllowedMaturityRating", value);
        }
        if let Some(value) = self._locale.as_ref() {
            params.push("locale", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/volumes/recommended";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> VolumeRecommendedListCall<'a, S> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The maximum allowed maturity rating of returned recommendations. Books with a higher maturity rating are filtered out.
    ///
    /// Sets the *max allowed maturity rating* query property to the given value.
    pub fn max_allowed_maturity_rating(mut self, new_value: &str) -> VolumeRecommendedListCall<'a, S> {
        self._max_allowed_maturity_rating = Some(new_value.to_string());
        self
    }
    /// ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'. Used for generating recommendations.
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> VolumeRecommendedListCall<'a, S> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> VolumeRecommendedListCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> VolumeRecommendedListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> VolumeRecommendedListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> VolumeRecommendedListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> VolumeRecommendedListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Rate a recommended book for the current user.
///
/// A builder for the *recommended.rate* method supported by a *volume* resource.
/// It is not used directly, but through a [`VolumeMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.volumes().recommended_rate("rating", "volumeId")
///              .source("kasd")
///              .locale("sadipscing")
///              .doit().await;
/// # }
/// ```
pub struct VolumeRecommendedRateCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _rating: String,
    _volume_id: String,
    _source: Option<String>,
    _locale: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for VolumeRecommendedRateCall<'a, S> {}

impl<'a, S> VolumeRecommendedRateCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, BooksVolumesRecommendedRateResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.volumes.recommended.rate",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "rating", "volumeId", "source", "locale"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("rating", self._rating);
        params.push("volumeId", self._volume_id);
        if let Some(value) = self._source.as_ref() {
            params.push("source", value);
        }
        if let Some(value) = self._locale.as_ref() {
            params.push("locale", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/volumes/recommended/rate";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Rating to be given to the volume.
    ///
    /// Sets the *rating* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn rating(mut self, new_value: &str) -> VolumeRecommendedRateCall<'a, S> {
        self._rating = new_value.to_string();
        self
    }
    /// ID of the source volume.
    ///
    /// Sets the *volume id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn volume_id(mut self, new_value: &str) -> VolumeRecommendedRateCall<'a, S> {
        self._volume_id = new_value.to_string();
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> VolumeRecommendedRateCall<'a, S> {
        self._source = Some(new_value.to_string());
        self
    }
    /// ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'. Used for generating recommendations.
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> VolumeRecommendedRateCall<'a, S> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> VolumeRecommendedRateCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> VolumeRecommendedRateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> VolumeRecommendedRateCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> VolumeRecommendedRateCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> VolumeRecommendedRateCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Return a list of books uploaded by the current user.
///
/// A builder for the *useruploaded.list* method supported by a *volume* resource.
/// It is not used directly, but through a [`VolumeMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.volumes().useruploaded_list()
///              .add_volume_id("tempor")
///              .start_index(91)
///              .source("et")
///              .add_processing_state("Lorem")
///              .max_results(68)
///              .locale("takimata")
///              .doit().await;
/// # }
/// ```
pub struct VolumeUseruploadedListCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _volume_id: Vec<String>,
    _start_index: Option<u32>,
    _source: Option<String>,
    _processing_state: Vec<String>,
    _max_results: Option<u32>,
    _locale: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for VolumeUseruploadedListCall<'a, S> {}

impl<'a, S> VolumeUseruploadedListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Volumes)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.volumes.useruploaded.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "volumeId", "startIndex", "source", "processingState", "maxResults", "locale"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(8 + self._additional_params.len());
        if self._volume_id.len() > 0 {
            for f in self._volume_id.iter() {
                params.push("volumeId", f);
            }
        }
        if let Some(value) = self._start_index.as_ref() {
            params.push("startIndex", value.to_string());
        }
        if let Some(value) = self._source.as_ref() {
            params.push("source", value);
        }
        if self._processing_state.len() > 0 {
            for f in self._processing_state.iter() {
                params.push("processingState", f);
            }
        }
        if let Some(value) = self._max_results.as_ref() {
            params.push("maxResults", value.to_string());
        }
        if let Some(value) = self._locale.as_ref() {
            params.push("locale", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/volumes/useruploaded";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ids of the volumes to be returned. If not specified all that match the processingState are returned.
    ///
    /// Append the given value to the *volume id* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_volume_id(mut self, new_value: &str) -> VolumeUseruploadedListCall<'a, S> {
        self._volume_id.push(new_value.to_string());
        self
    }
    /// Index of the first result to return (starts at 0)
    ///
    /// Sets the *start index* query property to the given value.
    pub fn start_index(mut self, new_value: u32) -> VolumeUseruploadedListCall<'a, S> {
        self._start_index = Some(new_value);
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> VolumeUseruploadedListCall<'a, S> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The processing state of the user uploaded volumes to be returned.
    ///
    /// Append the given value to the *processing state* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_processing_state(mut self, new_value: &str) -> VolumeUseruploadedListCall<'a, S> {
        self._processing_state.push(new_value.to_string());
        self
    }
    /// Maximum number of results to return.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> VolumeUseruploadedListCall<'a, S> {
        self._max_results = Some(new_value);
        self
    }
    /// ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'. Used for generating recommendations.
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> VolumeUseruploadedListCall<'a, S> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> VolumeUseruploadedListCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> VolumeUseruploadedListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> VolumeUseruploadedListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> VolumeUseruploadedListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> VolumeUseruploadedListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Gets volume information for a single volume.
///
/// A builder for the *get* method supported by a *volume* resource.
/// It is not used directly, but through a [`VolumeMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.volumes().get("volumeId")
///              .user_library_consistent_read(false)
///              .source("clita")
///              .projection("Stet")
///              .partner("aliquyam")
///              .include_non_comics_series(false)
///              .country("dolores")
///              .doit().await;
/// # }
/// ```
pub struct VolumeGetCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _volume_id: String,
    _user_library_consistent_read: Option<bool>,
    _source: Option<String>,
    _projection: Option<String>,
    _partner: Option<String>,
    _include_non_comics_series: Option<bool>,
    _country: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for VolumeGetCall<'a, S> {}

impl<'a, S> VolumeGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Volume)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.volumes.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "volumeId", "user_library_consistent_read", "source", "projection", "partner", "includeNonComicsSeries", "country"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(9 + self._additional_params.len());
        params.push("volumeId", self._volume_id);
        if let Some(value) = self._user_library_consistent_read.as_ref() {
            params.push("user_library_consistent_read", value.to_string());
        }
        if let Some(value) = self._source.as_ref() {
            params.push("source", value);
        }
        if let Some(value) = self._projection.as_ref() {
            params.push("projection", value);
        }
        if let Some(value) = self._partner.as_ref() {
            params.push("partner", value);
        }
        if let Some(value) = self._include_non_comics_series.as_ref() {
            params.push("includeNonComicsSeries", value.to_string());
        }
        if let Some(value) = self._country.as_ref() {
            params.push("country", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/volumes/{volumeId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{volumeId}", "volumeId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["volumeId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// ID of volume to retrieve.
    ///
    /// Sets the *volume id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn volume_id(mut self, new_value: &str) -> VolumeGetCall<'a, S> {
        self._volume_id = new_value.to_string();
        self
    }
    ///
    /// Sets the *user_library_consistent_read* query property to the given value.
    pub fn user_library_consistent_read(mut self, new_value: bool) -> VolumeGetCall<'a, S> {
        self._user_library_consistent_read = Some(new_value);
        self
    }
    /// string to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> VolumeGetCall<'a, S> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Restrict information returned to a set of selected fields.
    ///
    /// Sets the *projection* query property to the given value.
    pub fn projection(mut self, new_value: &str) -> VolumeGetCall<'a, S> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Brand results for partner ID.
    ///
    /// Sets the *partner* query property to the given value.
    pub fn partner(mut self, new_value: &str) -> VolumeGetCall<'a, S> {
        self._partner = Some(new_value.to_string());
        self
    }
    /// Set to true to include non-comics series. Defaults to false.
    ///
    /// Sets the *include non comics series* query property to the given value.
    pub fn include_non_comics_series(mut self, new_value: bool) -> VolumeGetCall<'a, S> {
        self._include_non_comics_series = Some(new_value);
        self
    }
    /// ISO-3166-1 code to override the IP-based location.
    ///
    /// Sets the *country* query property to the given value.
    pub fn country(mut self, new_value: &str) -> VolumeGetCall<'a, S> {
        self._country = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> VolumeGetCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> VolumeGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> VolumeGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> VolumeGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> VolumeGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Performs a book search.
///
/// A builder for the *list* method supported by a *volume* resource.
/// It is not used directly, but through a [`VolumeMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_books1 as books1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.volumes().list("q")
///              .start_index(63)
///              .source("clita")
///              .show_preorders(false)
///              .projection("aliquyam")
///              .print_type("magna")
///              .partner("diam")
///              .order_by("nonumy")
///              .max_results(83)
///              .max_allowed_maturity_rating("sanctus")
///              .library_restrict("accusam")
///              .lang_restrict("tempor")
///              .filter("sed")
///              .download("est")
///              .doit().await;
/// # }
/// ```
pub struct VolumeListCall<'a, S>
    where S: 'a {

    hub: &'a Books<S>,
    _q: String,
    _start_index: Option<u32>,
    _source: Option<String>,
    _show_preorders: Option<bool>,
    _projection: Option<String>,
    _print_type: Option<String>,
    _partner: Option<String>,
    _order_by: Option<String>,
    _max_results: Option<u32>,
    _max_allowed_maturity_rating: Option<String>,
    _library_restrict: Option<String>,
    _lang_restrict: Option<String>,
    _filter: Option<String>,
    _download: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for VolumeListCall<'a, S> {}

impl<'a, S> VolumeListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Volumes)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "books.volumes.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "q", "startIndex", "source", "showPreorders", "projection", "printType", "partner", "orderBy", "maxResults", "maxAllowedMaturityRating", "libraryRestrict", "langRestrict", "filter", "download"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(16 + self._additional_params.len());
        params.push("q", self._q);
        if let Some(value) = self._start_index.as_ref() {
            params.push("startIndex", value.to_string());
        }
        if let Some(value) = self._source.as_ref() {
            params.push("source", value);
        }
        if let Some(value) = self._show_preorders.as_ref() {
            params.push("showPreorders", value.to_string());
        }
        if let Some(value) = self._projection.as_ref() {
            params.push("projection", value);
        }
        if let Some(value) = self._print_type.as_ref() {
            params.push("printType", value);
        }
        if let Some(value) = self._partner.as_ref() {
            params.push("partner", value);
        }
        if let Some(value) = self._order_by.as_ref() {
            params.push("orderBy", value);
        }
        if let Some(value) = self._max_results.as_ref() {
            params.push("maxResults", value.to_string());
        }
        if let Some(value) = self._max_allowed_maturity_rating.as_ref() {
            params.push("maxAllowedMaturityRating", value);
        }
        if let Some(value) = self._library_restrict.as_ref() {
            params.push("libraryRestrict", value);
        }
        if let Some(value) = self._lang_restrict.as_ref() {
            params.push("langRestrict", value);
        }
        if let Some(value) = self._filter.as_ref() {
            params.push("filter", value);
        }
        if let Some(value) = self._download.as_ref() {
            params.push("download", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "books/v1/volumes";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Full-text search query string.
    ///
    /// Sets the *q* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn q(mut self, new_value: &str) -> VolumeListCall<'a, S> {
        self._q = new_value.to_string();
        self
    }
    /// Index of the first result to return (starts at 0)
    ///
    /// Sets the *start index* query property to the given value.
    pub fn start_index(mut self, new_value: u32) -> VolumeListCall<'a, S> {
        self._start_index = Some(new_value);
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> VolumeListCall<'a, S> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Set to true to show books available for preorder. Defaults to false.
    ///
    /// Sets the *show preorders* query property to the given value.
    pub fn show_preorders(mut self, new_value: bool) -> VolumeListCall<'a, S> {
        self._show_preorders = Some(new_value);
        self
    }
    /// Restrict information returned to a set of selected fields.
    ///
    /// Sets the *projection* query property to the given value.
    pub fn projection(mut self, new_value: &str) -> VolumeListCall<'a, S> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Restrict to books or magazines.
    ///
    /// Sets the *print type* query property to the given value.
    pub fn print_type(mut self, new_value: &str) -> VolumeListCall<'a, S> {
        self._print_type = Some(new_value.to_string());
        self
    }
    /// Restrict and brand results for partner ID.
    ///
    /// Sets the *partner* query property to the given value.
    pub fn partner(mut self, new_value: &str) -> VolumeListCall<'a, S> {
        self._partner = Some(new_value.to_string());
        self
    }
    /// Sort search results.
    ///
    /// Sets the *order by* query property to the given value.
    pub fn order_by(mut self, new_value: &str) -> VolumeListCall<'a, S> {
        self._order_by = Some(new_value.to_string());
        self
    }
    /// Maximum number of results to return.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> VolumeListCall<'a, S> {
        self._max_results = Some(new_value);
        self
    }
    /// The maximum allowed maturity rating of returned recommendations. Books with a higher maturity rating are filtered out.
    ///
    /// Sets the *max allowed maturity rating* query property to the given value.
    pub fn max_allowed_maturity_rating(mut self, new_value: &str) -> VolumeListCall<'a, S> {
        self._max_allowed_maturity_rating = Some(new_value.to_string());
        self
    }
    /// Restrict search to this user's library.
    ///
    /// Sets the *library restrict* query property to the given value.
    pub fn library_restrict(mut self, new_value: &str) -> VolumeListCall<'a, S> {
        self._library_restrict = Some(new_value.to_string());
        self
    }
    /// Restrict results to books with this language code.
    ///
    /// Sets the *lang restrict* query property to the given value.
    pub fn lang_restrict(mut self, new_value: &str) -> VolumeListCall<'a, S> {
        self._lang_restrict = Some(new_value.to_string());
        self
    }
    /// Filter search results.
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> VolumeListCall<'a, S> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// Restrict to volumes by download availability.
    ///
    /// Sets the *download* query property to the given value.
    pub fn download(mut self, new_value: &str) -> VolumeListCall<'a, S> {
        self._download = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> VolumeListCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> VolumeListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> VolumeListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> VolumeListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> VolumeListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


