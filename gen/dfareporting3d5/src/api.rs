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
    /// View and manage your DoubleClick Campaign Manager's (DCM) display ad campaigns
    Dfatrafficking,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Dfatrafficking => "https://www.googleapis.com/auth/dfatrafficking",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Dfatrafficking
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all Dfareporting related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d5 as dfareporting3d5;
/// use dfareporting3d5::api::CreativeAssetMetadata;
/// use dfareporting3d5::{Result, Error};
/// use std::fs;
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d5::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = CreativeAssetMetadata::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `upload(...)`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.media().upload(req, -28, -27)
///              .upload(fs::File::open("file.ext").unwrap(), "application/octet-stream".parse().unwrap()).await;
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
pub struct Dfareporting<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: Box<dyn client::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, S> client::Hub for Dfareporting<S> {}

impl<'a, S> Dfareporting<S> {

    pub fn new<A: 'static + client::GetToken>(client: hyper::Client<S, hyper::body::Body>, auth: A) -> Dfareporting<S> {
        Dfareporting {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/5.0.5".to_string(),
            _base_url: "https://dfareporting.googleapis.com/dfareporting/v3.5/".to_string(),
            _root_url: "https://dfareporting.googleapis.com/".to_string(),
        }
    }

    pub fn media(&'a self) -> MediaMethods<'a, S> {
        MediaMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/5.0.5`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://dfareporting.googleapis.com/dfareporting/v3.5/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://dfareporting.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// Creative Click Tag.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClickTag {
    /// Parameter value for the specified click tag. This field contains a click-through url.
    #[serde(rename="clickThroughUrl")]
    
    pub click_through_url: Option<CreativeClickThroughUrl>,
    /// Advertiser event name associated with the click tag. This field is used by DISPLAY_IMAGE_GALLERY and HTML5_BANNER creatives. Applicable to DISPLAY when the primary asset type is not HTML_IMAGE.
    #[serde(rename="eventName")]
    
    pub event_name: Option<String>,
    /// Parameter name for the specified click tag. For DISPLAY_IMAGE_GALLERY creative assets, this field must match the value of the creative asset's creativeAssetId.name field.
    
    pub name: Option<String>,
}

impl client::Part for ClickTag {}


/// Creative Asset ID.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeAssetId {
    /// Name of the creative asset. This is a required field while inserting an asset. After insertion, this assetIdentifier is used to identify the uploaded asset. Characters in the name must be alphanumeric or one of the following: ".-_ ". Spaces are allowed.
    
    pub name: Option<String>,
    /// Type of asset to upload. This is a required field. FLASH and IMAGE are no longer supported for new uploads. All image assets should use HTML_IMAGE.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for CreativeAssetId {}


/// CreativeAssets contains properties of a creative asset file which will be uploaded or has already been uploaded. Refer to the creative sample code for how to upload assets and insert a creative.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [upload media](MediaUploadCall) (request|response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeAssetMetadata {
    /// ID of the creative asset. This is a required field.
    #[serde(rename="assetIdentifier")]
    
    pub asset_identifier: Option<CreativeAssetId>,
    /// List of detected click tags for assets. This is a read-only, auto-generated field. This field is empty for a rich media asset.
    #[serde(rename="clickTags")]
    
    pub click_tags: Option<Vec<ClickTag>>,
    /// List of counter events configured for the asset. This is a read-only, auto-generated field and only applicable to a rich media asset.
    #[serde(rename="counterCustomEvents")]
    
    pub counter_custom_events: Option<Vec<CreativeCustomEvent>>,
    /// List of feature dependencies for the creative asset that are detected by Campaign Manager. Feature dependencies are features that a browser must be able to support in order to render your HTML5 creative correctly. This is a read-only, auto-generated field.
    #[serde(rename="detectedFeatures")]
    
    pub detected_features: Option<Vec<String>>,
    /// List of exit events configured for the asset. This is a read-only, auto-generated field and only applicable to a rich media asset.
    #[serde(rename="exitCustomEvents")]
    
    pub exit_custom_events: Option<Vec<CreativeCustomEvent>>,
    /// Numeric ID of the asset. This is a read-only, auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Dimension value for the numeric ID of the asset. This is a read-only, auto-generated field.
    #[serde(rename="idDimensionValue")]
    
    pub id_dimension_value: Option<DimensionValue>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#creativeAssetMetadata".
    
    pub kind: Option<String>,
    /// no description provided
    #[serde(rename="mediaRequestInfo")]
    
    pub media_request_info: Option<MediaRequestInfo>,
    /// no description provided
    #[serde(rename="mediaResponseInfo")]
    
    pub media_response_info: Option<MediaResponseInfo>,
    /// True if the uploaded asset is a rich media asset. This is a read-only, auto-generated field.
    #[serde(rename="richMedia")]
    
    pub rich_media: Option<bool>,
    /// List of timer events configured for the asset. This is a read-only, auto-generated field and only applicable to a rich media asset.
    #[serde(rename="timerCustomEvents")]
    
    pub timer_custom_events: Option<Vec<CreativeCustomEvent>>,
    /// Rules validated during code generation that generated a warning. This is a read-only, auto-generated field. Possible values are: - "ADMOB_REFERENCED" - "ASSET_FORMAT_UNSUPPORTED_DCM" - "ASSET_INVALID" - "CLICK_TAG_HARD_CODED" - "CLICK_TAG_INVALID" - "CLICK_TAG_IN_GWD" - "CLICK_TAG_MISSING" - "CLICK_TAG_MORE_THAN_ONE" - "CLICK_TAG_NON_TOP_LEVEL" - "COMPONENT_UNSUPPORTED_DCM" - "ENABLER_UNSUPPORTED_METHOD_DCM" - "EXTERNAL_FILE_REFERENCED" - "FILE_DETAIL_EMPTY" - "FILE_TYPE_INVALID" - "GWD_PROPERTIES_INVALID" - "HTML5_FEATURE_UNSUPPORTED" - "LINKED_FILE_NOT_FOUND" - "MAX_FLASH_VERSION_11" - "MRAID_REFERENCED" - "NOT_SSL_COMPLIANT" - "ORPHANED_ASSET" - "PRIMARY_HTML_MISSING" - "SVG_INVALID" - "ZIP_INVALID" 
    #[serde(rename="warnedValidationRules")]
    
    pub warned_validation_rules: Option<Vec<String>>,
}

impl client::RequestValue for CreativeAssetMetadata {}
impl client::ResponseResult for CreativeAssetMetadata {}


/// Click-through URL
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeClickThroughUrl {
    /// Read-only convenience field representing the actual URL that will be used for this click-through. The URL is computed as follows: - If landingPageId is specified then that landing page's URL is assigned to this field. - Otherwise, the customClickThroughUrl is assigned to this field. 
    #[serde(rename="computedClickThroughUrl")]
    
    pub computed_click_through_url: Option<String>,
    /// Custom click-through URL. Applicable if the landingPageId field is left unset.
    #[serde(rename="customClickThroughUrl")]
    
    pub custom_click_through_url: Option<String>,
    /// ID of the landing page for the click-through URL.
    #[serde(rename="landingPageId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub landing_page_id: Option<i64>,
}

impl client::Part for CreativeClickThroughUrl {}


/// Creative Custom Event.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeCustomEvent {
    /// Unique ID of this event used by Reporting and Data Transfer. This is a read-only field.
    #[serde(rename="advertiserCustomEventId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_custom_event_id: Option<i64>,
    /// User-entered name for the event.
    #[serde(rename="advertiserCustomEventName")]
    
    pub advertiser_custom_event_name: Option<String>,
    /// Type of the event. This is a read-only field.
    #[serde(rename="advertiserCustomEventType")]
    
    pub advertiser_custom_event_type: Option<String>,
    /// Artwork label column, used to link events in Campaign Manager back to events in Studio. This is a required field and should not be modified after insertion.
    #[serde(rename="artworkLabel")]
    
    pub artwork_label: Option<String>,
    /// Artwork type used by the creative.This is a read-only field.
    #[serde(rename="artworkType")]
    
    pub artwork_type: Option<String>,
    /// Exit click-through URL for the event. This field is used only for exit events.
    #[serde(rename="exitClickThroughUrl")]
    
    pub exit_click_through_url: Option<CreativeClickThroughUrl>,
    /// ID of this event. This is a required field and should not be modified after insertion.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Properties for rich media popup windows. This field is used only for exit events.
    #[serde(rename="popupWindowProperties")]
    
    pub popup_window_properties: Option<PopupWindowProperties>,
    /// Target type used by the event.
    #[serde(rename="targetType")]
    
    pub target_type: Option<String>,
    /// Video reporting ID, used to differentiate multiple videos in a single creative. This is a read-only field.
    #[serde(rename="videoReportingId")]
    
    pub video_reporting_id: Option<String>,
}

impl client::Part for CreativeCustomEvent {}


/// Represents a DimensionValue resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DimensionValue {
    /// The name of the dimension.
    #[serde(rename="dimensionName")]
    
    pub dimension_name: Option<String>,
    /// The eTag of this response for caching purposes.
    
    pub etag: Option<String>,
    /// The ID associated with the value if available.
    
    pub id: Option<String>,
    /// The kind of resource this is, in this case dfareporting#dimensionValue.
    
    pub kind: Option<String>,
    /// Determines how the 'value' field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, '*' is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions ('dfa:paidSearch*') allow a matchType other than EXACT.
    #[serde(rename="matchType")]
    
    pub match_type: Option<String>,
    /// The value of the dimension.
    
    pub value: Option<String>,
}

impl client::Part for DimensionValue {}


/// Extra information added to operations that support Scotty media requests.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MediaRequestInfo {
    /// The number of current bytes uploaded or downloaded.
    #[serde(rename="currentBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub current_bytes: Option<i64>,
    /// Data to be copied to backend requests. Custom data is returned to Scotty in the agent_state field, which Scotty will then provide in subsequent upload notifications.
    #[serde(rename="customData")]
    
    pub custom_data: Option<String>,
    /// Set if the http request info is diff encoded. The value of this field is the version number of the base revision. This is corresponding to Apiary's mediaDiffObjectVersion (//depot/google3/java/com/google/api/server/media/variable/DiffObjectVersionVariable.java). See go/esf-scotty-diff-upload for more information.
    #[serde(rename="diffObjectVersion")]
    
    pub diff_object_version: Option<String>,
    /// The existence of the final_status field indicates that this is the last call to the agent for this request_id. http://google3/uploader/agent/scotty_agent.proto?l=737&rcl=347601929
    #[serde(rename="finalStatus")]
    
    pub final_status: Option<i32>,
    /// The type of notification received from Scotty.
    #[serde(rename="notificationType")]
    
    pub notification_type: Option<String>,
    /// The Scotty request ID.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
    /// The partition of the Scotty server handling this request. type is uploader_service.RequestReceivedParamsServingInfo LINT.IfChange(request_received_params_serving_info_annotations) LINT.ThenChange()
    #[serde(rename="requestReceivedParamsServingInfo")]
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub request_received_params_serving_info: Option<Vec<u8>>,
    /// The total size of the file.
    #[serde(rename="totalBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_bytes: Option<i64>,
    /// Whether the total bytes field contains an estimated data.
    #[serde(rename="totalBytesIsEstimated")]
    
    pub total_bytes_is_estimated: Option<bool>,
}

impl client::Part for MediaRequestInfo {}


/// This message is for backends to pass their scotty media specific fields to ESF. Backend will include this in their response message to ESF. Example: ExportFile is an rpc defined for upload using scotty from ESF. rpc ExportFile(ExportFileRequest) returns (ExportFileResponse) Message ExportFileResponse will include apiserving.MediaResponseInfo to tell ESF about data like dynamic_dropzone it needs to pass to Scotty. message ExportFileResponse { optional gdata.Media blob = 1; optional apiserving.MediaResponseInfo media_response_info = 2 }
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MediaResponseInfo {
    /// Data to copy from backend response to the next backend requests. Custom data is returned to Scotty in the agent_state field, which Scotty will then provide in subsequent upload notifications.
    #[serde(rename="customData")]
    
    pub custom_data: Option<String>,
    /// Specifies any transformation to be applied to data before persisting it or retrieving from storage. E.g., encryption options for blobstore2. This should be of the form uploader_service.DataStorageTransform.
    #[serde(rename="dataStorageTransform")]
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub data_storage_transform: Option<Vec<u8>>,
    /// Specifies the Scotty Drop Target to use for uploads. If present in a media response, Scotty does not upload to a standard drop zone. Instead, Scotty saves the upload directly to the location specified in this drop target. Unlike drop zones, the drop target is the final storage location for an upload. So, the agent does not need to clone the blob at the end of the upload. The agent is responsible for garbage collecting any orphaned blobs that may occur due to aborted uploads. For more information, see the drop target design doc here: http://goto/ScottyDropTarget This field will be preferred to dynamicDropzone. If provided, the identified field in the response must be of the type uploader.agent.DropTarget.
    #[serde(rename="dynamicDropTarget")]
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub dynamic_drop_target: Option<Vec<u8>>,
    /// Specifies the Scotty dropzone to use for uploads.
    #[serde(rename="dynamicDropzone")]
    
    pub dynamic_dropzone: Option<String>,
    /// Request class to use for all Blobstore operations for this request.
    #[serde(rename="requestClass")]
    
    pub request_class: Option<String>,
    /// Requester ID passed along to be recorded in the Scotty logs
    #[serde(rename="scottyAgentUserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub scotty_agent_user_id: Option<i64>,
    /// Customer-specific data to be recorded in the Scotty logs type is logs_proto_scotty.CustomerLog
    #[serde(rename="scottyCustomerLog")]
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub scotty_customer_log: Option<Vec<u8>>,
    /// Specifies the TrafficClass that Scotty should use for any RPCs to fetch the response bytes. Will override the traffic class GTOS of the incoming http request. This is a temporary field to facilitate whitelisting and experimentation by the bigstore agent only. For instance, this does not apply to RTMP reads. WARNING: DO NOT USE WITHOUT PERMISSION FROM THE SCOTTY TEAM.
    #[serde(rename="trafficClassField")]
    
    pub traffic_class_field: Option<String>,
    /// Tells Scotty to verify hashes on the agent's behalf by parsing out the X-Goog-Hash header.
    #[serde(rename="verifyHashFromHeader")]
    
    pub verify_hash_from_header: Option<bool>,
}

impl client::Part for MediaResponseInfo {}


/// Offset Position.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OffsetPosition {
    /// Offset distance from left side of an asset or a window.
    
    pub left: Option<i32>,
    /// Offset distance from top side of an asset or a window.
    
    pub top: Option<i32>,
}

impl client::Part for OffsetPosition {}


/// Popup Window Properties.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PopupWindowProperties {
    /// Popup dimension for a creative. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA and all VPAID
    
    pub dimension: Option<Size>,
    /// Upper-left corner coordinates of the popup window. Applicable if positionType is COORDINATES.
    
    pub offset: Option<OffsetPosition>,
    /// Popup window position either centered or at specific coordinate.
    #[serde(rename="positionType")]
    
    pub position_type: Option<String>,
    /// Whether to display the browser address bar.
    #[serde(rename="showAddressBar")]
    
    pub show_address_bar: Option<bool>,
    /// Whether to display the browser menu bar.
    #[serde(rename="showMenuBar")]
    
    pub show_menu_bar: Option<bool>,
    /// Whether to display the browser scroll bar.
    #[serde(rename="showScrollBar")]
    
    pub show_scroll_bar: Option<bool>,
    /// Whether to display the browser status bar.
    #[serde(rename="showStatusBar")]
    
    pub show_status_bar: Option<bool>,
    /// Whether to display the browser tool bar.
    #[serde(rename="showToolBar")]
    
    pub show_tool_bar: Option<bool>,
    /// Title of popup window.
    
    pub title: Option<String>,
}

impl client::Part for PopupWindowProperties {}


/// Represents the dimensions of ads, placements, creatives, or creative assets.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Size {
    /// Height of this size. Acceptable values are 0 to 32767, inclusive.
    
    pub height: Option<i32>,
    /// IAB standard size. This is a read-only, auto-generated field.
    
    pub iab: Option<bool>,
    /// ID of this size. This is a read-only, auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#size".
    
    pub kind: Option<String>,
    /// Width of this size. Acceptable values are 0 to 32767, inclusive.
    
    pub width: Option<i32>,
}

impl client::Part for Size {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *media* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d5 as dfareporting3d5;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d5::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `upload(...)`
/// // to build up your call.
/// let rb = hub.media();
/// # }
/// ```
pub struct MediaMethods<'a, S>
    where S: 'a {

    hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for MediaMethods<'a, S> {}

impl<'a, S> MediaMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new creative asset.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    /// * `advertiserId` - Advertiser ID of this creative. This is a required field.
    pub fn upload(&self, request: CreativeAssetMetadata, profile_id: i64, advertiser_id: i64) -> MediaUploadCall<'a, S> {
        MediaUploadCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _advertiser_id: advertiser_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Inserts a new creative asset.
///
/// A builder for the *upload* method supported by a *media* resource.
/// It is not used directly, but through a [`MediaMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_dfareporting3d5 as dfareporting3d5;
/// use dfareporting3d5::api::CreativeAssetMetadata;
/// use std::fs;
/// # async fn dox() {
/// # use std::default::Default;
/// # use dfareporting3d5::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = CreativeAssetMetadata::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `upload(...)`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.media().upload(req, -8, -80)
///              .upload(fs::File::open("file.ext").unwrap(), "application/octet-stream".parse().unwrap()).await;
/// # }
/// ```
pub struct MediaUploadCall<'a, S>
    where S: 'a {

    hub: &'a Dfareporting<S>,
    _request: CreativeAssetMetadata,
    _profile_id: i64,
    _advertiser_id: i64,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for MediaUploadCall<'a, S> {}

impl<'a, S> MediaUploadCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    async fn doit<RS>(mut self, mut reader: RS, reader_mime_type: mime::Mime, protocol: client::UploadProtocol) -> client::Result<(hyper::Response<hyper::body::Body>, CreativeAssetMetadata)>
		where RS: client::ReadSeek {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "dfareporting.media.upload",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "profileId", "advertiserId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("profileId", self._profile_id.to_string());
        params.push("advertiserId", self._advertiser_id.to_string());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let (mut url, upload_type) =
            if protocol == client::UploadProtocol::Simple {
                (self.hub._root_url.clone() + "upload/dfareporting/v3.5/userprofiles/{+profileId}/creativeAssets/{+advertiserId}/creativeAssets", "multipart")
            } else {
                unreachable!()
            };
        params.push("uploadType", upload_type);
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Dfatrafficking.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+profileId}", "profileId"), ("{+advertiserId}", "advertiserId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["advertiserId", "profileId"];
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
                let mut mp_reader: client::MultiPartReader = Default::default();
                let (mut body_reader, content_type) = match protocol {
                    client::UploadProtocol::Simple => {
                        mp_reader.reserve_exact(2);
                        let size = reader.seek(io::SeekFrom::End(0)).unwrap();
                    reader.seek(io::SeekFrom::Start(0)).unwrap();
                    if size > 1073741824 {
                    	return Err(client::Error::UploadSizeLimitExceeded(size, 1073741824))
                    }
                        mp_reader.add_part(&mut request_value_reader, request_size, json_mime_type.clone())
                                 .add_part(&mut reader, size, reader_mime_type.clone());
                        (&mut mp_reader as &mut (dyn io::Read + Send), client::MultiPartReader::mime_type())
                    },
                    _ => (&mut request_value_reader as &mut (dyn io::Read + Send), json_mime_type.clone()),
                };
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let mut body_reader_bytes = vec![];
                        body_reader.read_to_end(&mut body_reader_bytes).unwrap();
                        let request = req_builder
                            .header(CONTENT_TYPE, content_type.to_string())
                            .body(hyper::body::Body::from(body_reader_bytes));

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

    /// Upload media all at once.
    /// If the upload fails for whichever reason, all progress is lost.
    ///
    /// * *multipart*: yes
    /// * *max size*: 1073741824
    /// * *valid mime types*: '*/*'
    pub async fn upload<RS>(self, stream: RS, mime_type: mime::Mime) -> client::Result<(hyper::Response<hyper::body::Body>, CreativeAssetMetadata)>
                where RS: client::ReadSeek {
        self.doit(stream, mime_type, client::UploadProtocol::Simple).await
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: CreativeAssetMetadata) -> MediaUploadCall<'a, S> {
        self._request = new_value;
        self
    }
    /// User profile ID associated with this request.
    ///
    /// Sets the *profile id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn profile_id(mut self, new_value: i64) -> MediaUploadCall<'a, S> {
        self._profile_id = new_value;
        self
    }
    /// Advertiser ID of this creative. This is a required field.
    ///
    /// Sets the *advertiser id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn advertiser_id(mut self, new_value: i64) -> MediaUploadCall<'a, S> {
        self._advertiser_id = new_value;
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> MediaUploadCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> MediaUploadCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Dfatrafficking`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> MediaUploadCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> MediaUploadCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> MediaUploadCall<'a, S> {
        self._scopes.clear();
        self
    }
}


