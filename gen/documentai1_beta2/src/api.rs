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
    /// See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account.
    CloudPlatform,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::CloudPlatform
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all Document related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_documentai1_beta2 as documentai1_beta2;
/// use documentai1_beta2::api::GoogleCloudDocumentaiV1beta2BatchProcessDocumentsRequest;
/// use documentai1_beta2::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use documentai1_beta2::{Document, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
/// let mut hub = Document::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudDocumentaiV1beta2BatchProcessDocumentsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().documents_batch_process(req, "parent")
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
pub struct Document<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: Box<dyn client::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, S> client::Hub for Document<S> {}

impl<'a, S> Document<S> {

    pub fn new<A: 'static + client::GetToken>(client: hyper::Client<S, hyper::body::Body>, auth: A) -> Document<S> {
        Document {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/5.0.5".to_string(),
            _base_url: "https://documentai.googleapis.com/".to_string(),
            _root_url: "https://documentai.googleapis.com/".to_string(),
        }
    }

    pub fn projects(&'a self) -> ProjectMethods<'a, S> {
        ProjectMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/5.0.5`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://documentai.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://documentai.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// Parameters to control AutoML model prediction behavior.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2AutoMlParams {
    /// Resource name of the AutoML model. Format: `projects/{project-id}/locations/{location-id}/models/{model-id}`.
    
    pub model: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2AutoMlParams {}


/// Encodes the detailed information of a barcode.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2Barcode {
    /// Format of a barcode. The supported formats are: - `CODE_128`: Code 128 type. - `CODE_39`: Code 39 type. - `CODE_93`: Code 93 type. - `CODABAR`: Codabar type. - `DATA_MATRIX`: 2D Data Matrix type. - `ITF`: ITF type. - `EAN_13`: EAN-13 type. - `EAN_8`: EAN-8 type. - `QR_CODE`: 2D QR code type. - `UPC_A`: UPC-A type. - `UPC_E`: UPC-E type. - `PDF417`: PDF417 type. - `AZTEC`: 2D Aztec code type. - `DATABAR`: GS1 DataBar code type.
    
    pub format: Option<String>,
    /// Raw value encoded in the barcode. For example: `'MEBKM:TITLE:Google;URL:https://www.google.com;;'`.
    #[serde(rename="rawValue")]
    
    pub raw_value: Option<String>,
    /// Value format describes the format of the value that a barcode encodes. The supported formats are: - `CONTACT_INFO`: Contact information. - `EMAIL`: Email address. - `ISBN`: ISBN identifier. - `PHONE`: Phone number. - `PRODUCT`: Product. - `SMS`: SMS message. - `TEXT`: Text string. - `URL`: URL address. - `WIFI`: Wifi information. - `GEO`: Geo-localization. - `CALENDAR_EVENT`: Calendar event. - `DRIVER_LICENSE`: Driver's license.
    #[serde(rename="valueFormat")]
    
    pub value_format: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2Barcode {}


/// Request to batch process documents as an asynchronous operation. The output is written to Cloud Storage as JSON in the \[Document\] format.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [documents batch process projects](ProjectDocumentBatchProcesCall) (request)
/// * [locations documents batch process projects](ProjectLocationDocumentBatchProcesCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2BatchProcessDocumentsRequest {
    /// Required. Individual requests for each document.
    
    pub requests: Option<Vec<GoogleCloudDocumentaiV1beta2ProcessDocumentRequest>>,
}

impl client::RequestValue for GoogleCloudDocumentaiV1beta2BatchProcessDocumentsRequest {}


/// A bounding polygon for the detected image annotation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2BoundingPoly {
    /// The bounding polygon normalized vertices.
    #[serde(rename="normalizedVertices")]
    
    pub normalized_vertices: Option<Vec<GoogleCloudDocumentaiV1beta2NormalizedVertex>>,
    /// The bounding polygon vertices.
    
    pub vertices: Option<Vec<GoogleCloudDocumentaiV1beta2Vertex>>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2BoundingPoly {}


/// Document represents the canonical document resource in Document AI. It is an interchange format that provides insights into documents and allows for collaboration between users and Document AI to iterate and optimize for quality.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [documents process projects](ProjectDocumentProcesCall) (response)
/// * [locations documents process projects](ProjectLocationDocumentProcesCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2Document {
    /// Document chunked based on chunking config.
    #[serde(rename="chunkedDocument")]
    
    pub chunked_document: Option<GoogleCloudDocumentaiV1beta2DocumentChunkedDocument>,
    /// Optional. Inline document content, represented as a stream of bytes. Note: As with all `bytes` fields, protobuffers use a pure binary representation, whereas JSON representations use base64.
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub content: Option<Vec<u8>>,
    /// Parsed layout of the document.
    #[serde(rename="documentLayout")]
    
    pub document_layout: Option<GoogleCloudDocumentaiV1beta2DocumentDocumentLayout>,
    /// A list of entities detected on Document.text. For document shards, entities in this list may cross shard boundaries.
    
    pub entities: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentEntity>>,
    /// Placeholder. Relationship among Document.entities.
    #[serde(rename="entityRelations")]
    
    pub entity_relations: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentEntityRelation>>,
    /// Any error that occurred while processing this document.
    
    pub error: Option<GoogleRpcStatus>,
    /// Labels for this document.
    
    pub labels: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentLabel>>,
    /// An IANA published [media type (MIME type)](https://www.iana.org/assignments/media-types/media-types.xhtml).
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
    /// Visual page layout for the Document.
    
    pub pages: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPage>>,
    /// Placeholder. Revision history of this document.
    
    pub revisions: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentRevision>>,
    /// Information about the sharding if this document is sharded part of a larger document. If the document is not sharded, this message is not specified.
    #[serde(rename="shardInfo")]
    
    pub shard_info: Option<GoogleCloudDocumentaiV1beta2DocumentShardInfo>,
    /// Optional. UTF-8 encoded text in reading order from the document.
    
    pub text: Option<String>,
    /// Placeholder. A list of text corrections made to Document.text. This is usually used for annotating corrections to OCR mistakes. Text changes for a given revision may not overlap with each other.
    #[serde(rename="textChanges")]
    
    pub text_changes: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentTextChange>>,
    /// Styles for the Document.text.
    #[serde(rename="textStyles")]
    
    pub text_styles: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentStyle>>,
    /// Optional. Currently supports Google Cloud Storage URI of the form `gs://bucket_name/object_name`. Object versioning is not supported. For more information, refer to [Google Cloud Storage Request URIs](https://cloud.google.com/storage/docs/reference-uris).
    
    pub uri: Option<String>,
}

impl client::ResponseResult for GoogleCloudDocumentaiV1beta2Document {}


/// Represents the chunks that the document is divided into.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentChunkedDocument {
    /// List of chunks.
    
    pub chunks: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentChunkedDocumentChunk>>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentChunkedDocument {}


/// Represents a chunk.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentChunkedDocumentChunk {
    /// ID of the chunk.
    #[serde(rename="chunkId")]
    
    pub chunk_id: Option<String>,
    /// Text content of the chunk.
    
    pub content: Option<String>,
    /// Page footers associated with the chunk.
    #[serde(rename="pageFooters")]
    
    pub page_footers: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentChunkedDocumentChunkChunkPageFooter>>,
    /// Page headers associated with the chunk.
    #[serde(rename="pageHeaders")]
    
    pub page_headers: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentChunkedDocumentChunkChunkPageHeader>>,
    /// Page span of the chunk.
    #[serde(rename="pageSpan")]
    
    pub page_span: Option<GoogleCloudDocumentaiV1beta2DocumentChunkedDocumentChunkChunkPageSpan>,
    /// Unused.
    #[serde(rename="sourceBlockIds")]
    
    pub source_block_ids: Option<Vec<String>>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentChunkedDocumentChunk {}


/// Represents the page footer associated with the chunk.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentChunkedDocumentChunkChunkPageFooter {
    /// Page span of the footer.
    #[serde(rename="pageSpan")]
    
    pub page_span: Option<GoogleCloudDocumentaiV1beta2DocumentChunkedDocumentChunkChunkPageSpan>,
    /// Footer in text format.
    
    pub text: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentChunkedDocumentChunkChunkPageFooter {}


/// Represents the page header associated with the chunk.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentChunkedDocumentChunkChunkPageHeader {
    /// Page span of the header.
    #[serde(rename="pageSpan")]
    
    pub page_span: Option<GoogleCloudDocumentaiV1beta2DocumentChunkedDocumentChunkChunkPageSpan>,
    /// Header in text format.
    
    pub text: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentChunkedDocumentChunkChunkPageHeader {}


/// Represents where the chunk starts and ends in the document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentChunkedDocumentChunkChunkPageSpan {
    /// Page where chunk ends in the document.
    #[serde(rename="pageEnd")]
    
    pub page_end: Option<i32>,
    /// Page where chunk starts in the document.
    #[serde(rename="pageStart")]
    
    pub page_start: Option<i32>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentChunkedDocumentChunkChunkPageSpan {}


/// Represents the parsed layout of a document as a collection of blocks that the document is divided into.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentDocumentLayout {
    /// List of blocks in the document.
    
    pub blocks: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentDocumentLayoutDocumentLayoutBlock>>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentDocumentLayout {}


/// Represents a block. A block could be one of the various types (text, table, list) supported.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentDocumentLayoutDocumentLayoutBlock {
    /// ID of the block.
    #[serde(rename="blockId")]
    
    pub block_id: Option<String>,
    /// Block consisting of list content/structure.
    #[serde(rename="listBlock")]
    
    pub list_block: Option<GoogleCloudDocumentaiV1beta2DocumentDocumentLayoutDocumentLayoutBlockLayoutListBlock>,
    /// Page span of the block.
    #[serde(rename="pageSpan")]
    
    pub page_span: Option<GoogleCloudDocumentaiV1beta2DocumentDocumentLayoutDocumentLayoutBlockLayoutPageSpan>,
    /// Block consisting of table content/structure.
    #[serde(rename="tableBlock")]
    
    pub table_block: Option<GoogleCloudDocumentaiV1beta2DocumentDocumentLayoutDocumentLayoutBlockLayoutTableBlock>,
    /// Block consisting of text content.
    #[serde(rename="textBlock")]
    
    pub text_block: Option<GoogleCloudDocumentaiV1beta2DocumentDocumentLayoutDocumentLayoutBlockLayoutTextBlock>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentDocumentLayoutDocumentLayoutBlock {}


/// Represents a list type block.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentDocumentLayoutDocumentLayoutBlockLayoutListBlock {
    /// List entries that constitute a list block.
    #[serde(rename="listEntries")]
    
    pub list_entries: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentDocumentLayoutDocumentLayoutBlockLayoutListEntry>>,
    /// Type of the list_entries (if exist). Available options are `ordered` and `unordered`.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentDocumentLayoutDocumentLayoutBlockLayoutListBlock {}


/// Represents an entry in the list.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentDocumentLayoutDocumentLayoutBlockLayoutListEntry {
    /// A list entry is a list of blocks. Repeated blocks support further hierarchies and nested blocks.
    
    pub blocks: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentDocumentLayoutDocumentLayoutBlock>>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentDocumentLayoutDocumentLayoutBlockLayoutListEntry {}


/// Represents where the block starts and ends in the document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentDocumentLayoutDocumentLayoutBlockLayoutPageSpan {
    /// Page where block ends in the document.
    #[serde(rename="pageEnd")]
    
    pub page_end: Option<i32>,
    /// Page where block starts in the document.
    #[serde(rename="pageStart")]
    
    pub page_start: Option<i32>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentDocumentLayoutDocumentLayoutBlockLayoutPageSpan {}


/// Represents a table type block.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentDocumentLayoutDocumentLayoutBlockLayoutTableBlock {
    /// Body rows containing main table content.
    #[serde(rename="bodyRows")]
    
    pub body_rows: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentDocumentLayoutDocumentLayoutBlockLayoutTableRow>>,
    /// Table caption/title.
    
    pub caption: Option<String>,
    /// Header rows at the top of the table.
    #[serde(rename="headerRows")]
    
    pub header_rows: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentDocumentLayoutDocumentLayoutBlockLayoutTableRow>>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentDocumentLayoutDocumentLayoutBlockLayoutTableBlock {}


/// Represents a cell in a table row.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentDocumentLayoutDocumentLayoutBlockLayoutTableCell {
    /// A table cell is a list of blocks. Repeated blocks support further hierarchies and nested blocks.
    
    pub blocks: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentDocumentLayoutDocumentLayoutBlock>>,
    /// How many columns this cell spans.
    #[serde(rename="colSpan")]
    
    pub col_span: Option<i32>,
    /// How many rows this cell spans.
    #[serde(rename="rowSpan")]
    
    pub row_span: Option<i32>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentDocumentLayoutDocumentLayoutBlockLayoutTableCell {}


/// Represents a row in a table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentDocumentLayoutDocumentLayoutBlockLayoutTableRow {
    /// A table row is a list of table cells.
    
    pub cells: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentDocumentLayoutDocumentLayoutBlockLayoutTableCell>>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentDocumentLayoutDocumentLayoutBlockLayoutTableRow {}


/// Represents a text type block.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentDocumentLayoutDocumentLayoutBlockLayoutTextBlock {
    /// A text block could further have child blocks. Repeated blocks support further hierarchies and nested blocks.
    
    pub blocks: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentDocumentLayoutDocumentLayoutBlock>>,
    /// Text content stored in the block.
    
    pub text: Option<String>,
    /// Type of the text in the block. Available options are: `paragraph`, `subtitle`, `heading-1`, `heading-2`, `heading-3`, `heading-4`, `heading-5`, `header`, `footer`.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentDocumentLayoutDocumentLayoutBlockLayoutTextBlock {}


/// An entity that could be a phrase in the text or a property that belongs to the document. It is a known entity type, such as a person, an organization, or location.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentEntity {
    /// Optional. Confidence of detected Schema entity. Range `[0, 1]`.
    
    pub confidence: Option<f32>,
    /// Optional. Canonical id. This will be a unique value in the entity list for this document.
    
    pub id: Option<String>,
    /// Optional. Deprecated. Use `id` field instead.
    #[serde(rename="mentionId")]
    
    pub mention_id: Option<String>,
    /// Optional. Text value of the entity e.g. `1600 Amphitheatre Pkwy`.
    #[serde(rename="mentionText")]
    
    pub mention_text: Option<String>,
    /// Optional. Normalized entity value. Absent if the extracted value could not be converted or the type (e.g. address) is not supported for certain parsers. This field is also only populated for certain supported document types.
    #[serde(rename="normalizedValue")]
    
    pub normalized_value: Option<GoogleCloudDocumentaiV1beta2DocumentEntityNormalizedValue>,
    /// Optional. Represents the provenance of this entity wrt. the location on the page where it was found.
    #[serde(rename="pageAnchor")]
    
    pub page_anchor: Option<GoogleCloudDocumentaiV1beta2DocumentPageAnchor>,
    /// Optional. Entities can be nested to form a hierarchical data structure representing the content in the document.
    
    pub properties: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentEntity>>,
    /// Optional. The history of this annotation.
    
    pub provenance: Option<GoogleCloudDocumentaiV1beta2DocumentProvenance>,
    /// Optional. Whether the entity will be redacted for de-identification purposes.
    
    pub redacted: Option<bool>,
    /// Optional. Provenance of the entity. Text anchor indexing into the Document.text.
    #[serde(rename="textAnchor")]
    
    pub text_anchor: Option<GoogleCloudDocumentaiV1beta2DocumentTextAnchor>,
    /// Required. Entity type from a schema e.g. `Address`.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentEntity {}


/// Parsed and normalized entity value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentEntityNormalizedValue {
    /// Postal address. See also: https://github.com/googleapis/googleapis/blob/master/google/type/postal_address.proto
    #[serde(rename="addressValue")]
    
    pub address_value: Option<GoogleTypePostalAddress>,
    /// Boolean value. Can be used for entities with binary values, or for checkboxes.
    #[serde(rename="booleanValue")]
    
    pub boolean_value: Option<bool>,
    /// Date value. Includes year, month, day. See also: https://github.com/googleapis/googleapis/blob/master/google/type/date.proto
    #[serde(rename="dateValue")]
    
    pub date_value: Option<GoogleTypeDate>,
    /// DateTime value. Includes date, time, and timezone. See also: https://github.com/googleapis/googleapis/blob/master/google/type/datetime.proto
    #[serde(rename="datetimeValue")]
    
    pub datetime_value: Option<GoogleTypeDateTime>,
    /// Float value.
    #[serde(rename="floatValue")]
    
    pub float_value: Option<f32>,
    /// Integer value.
    #[serde(rename="integerValue")]
    
    pub integer_value: Option<i32>,
    /// Money value. See also: https://github.com/googleapis/googleapis/blob/master/google/type/money.proto
    #[serde(rename="moneyValue")]
    
    pub money_value: Option<GoogleTypeMoney>,
    /// Optional. An optional field to store a normalized string. For some entity types, one of respective `structured_value` fields may also be populated. Also not all the types of `structured_value` will be normalized. For example, some processors may not generate `float` or `integer` normalized text by default. Below are sample formats mapped to structured values. - Money/Currency type (`money_value`) is in the ISO 4217 text format. - Date type (`date_value`) is in the ISO 8601 text format. - Datetime type (`datetime_value`) is in the ISO 8601 text format.
    
    pub text: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentEntityNormalizedValue {}


/// Relationship between Entities.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentEntityRelation {
    /// Object entity id.
    #[serde(rename="objectId")]
    
    pub object_id: Option<String>,
    /// Relationship description.
    
    pub relation: Option<String>,
    /// Subject entity id.
    #[serde(rename="subjectId")]
    
    pub subject_id: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentEntityRelation {}


/// Label attaches schema information and/or other metadata to segments within a Document. Multiple Labels on a single field can denote either different labels, different instances of the same label created at different times, or some combination of both.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentLabel {
    /// Label is generated AutoML model. This field stores the full resource name of the AutoML model. Format: `projects/{project-id}/locations/{location-id}/models/{model-id}`
    #[serde(rename="automlModel")]
    
    pub automl_model: Option<String>,
    /// Confidence score between 0 and 1 for label assignment.
    
    pub confidence: Option<f32>,
    /// Name of the label. When the label is generated from AutoML Text Classification model, this field represents the name of the category.
    
    pub name: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentLabel {}


/// A page in a Document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentPage {
    /// A list of visually detected text blocks on the page. A block has a set of lines (collected into paragraphs) that have a common line-spacing and orientation.
    
    pub blocks: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageBlock>>,
    /// A list of detected barcodes.
    #[serde(rename="detectedBarcodes")]
    
    pub detected_barcodes: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageDetectedBarcode>>,
    /// A list of detected languages together with confidence.
    #[serde(rename="detectedLanguages")]
    
    pub detected_languages: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguage>>,
    /// Physical dimension of the page.
    
    pub dimension: Option<GoogleCloudDocumentaiV1beta2DocumentPageDimension>,
    /// A list of visually detected form fields on the page.
    #[serde(rename="formFields")]
    
    pub form_fields: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageFormField>>,
    /// Rendered image for this page. This image is preprocessed to remove any skew, rotation, and distortions such that the annotation bounding boxes can be upright and axis-aligned.
    
    pub image: Option<GoogleCloudDocumentaiV1beta2DocumentPageImage>,
    /// Image quality scores.
    #[serde(rename="imageQualityScores")]
    
    pub image_quality_scores: Option<GoogleCloudDocumentaiV1beta2DocumentPageImageQualityScores>,
    /// Layout for the page.
    
    pub layout: Option<GoogleCloudDocumentaiV1beta2DocumentPageLayout>,
    /// A list of visually detected text lines on the page. A collection of tokens that a human would perceive as a line.
    
    pub lines: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageLine>>,
    /// 1-based index for current Page in a parent Document. Useful when a page is taken out of a Document for individual processing.
    #[serde(rename="pageNumber")]
    
    pub page_number: Option<i32>,
    /// A list of visually detected text paragraphs on the page. A collection of lines that a human would perceive as a paragraph.
    
    pub paragraphs: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageParagraph>>,
    /// The history of this page.
    
    pub provenance: Option<GoogleCloudDocumentaiV1beta2DocumentProvenance>,
    /// A list of visually detected symbols on the page.
    
    pub symbols: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageSymbol>>,
    /// A list of visually detected tables on the page.
    
    pub tables: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageTable>>,
    /// A list of visually detected tokens on the page.
    
    pub tokens: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageToken>>,
    /// Transformation matrices that were applied to the original document image to produce Page.image.
    
    pub transforms: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageMatrix>>,
    /// A list of detected non-text visual elements e.g. checkbox, signature etc. on the page.
    #[serde(rename="visualElements")]
    
    pub visual_elements: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageVisualElement>>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentPage {}


/// Referencing the visual context of the entity in the Document.pages. Page anchors can be cross-page, consist of multiple bounding polygons and optionally reference specific layout element types.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentPageAnchor {
    /// One or more references to visual page elements
    #[serde(rename="pageRefs")]
    
    pub page_refs: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageAnchorPageRef>>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentPageAnchor {}


/// Represents a weak reference to a page element within a document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentPageAnchorPageRef {
    /// Optional. Identifies the bounding polygon of a layout element on the page. If `layout_type` is set, the bounding polygon must be exactly the same to the layout element it's referring to.
    #[serde(rename="boundingPoly")]
    
    pub bounding_poly: Option<GoogleCloudDocumentaiV1beta2BoundingPoly>,
    /// Optional. Confidence of detected page element, if applicable. Range `[0, 1]`.
    
    pub confidence: Option<f32>,
    /// Optional. Deprecated. Use PageRef.bounding_poly instead.
    #[serde(rename="layoutId")]
    
    pub layout_id: Option<String>,
    /// Optional. The type of the layout element that is being referenced if any.
    #[serde(rename="layoutType")]
    
    pub layout_type: Option<String>,
    /// Required. Index into the Document.pages element, for example using `Document.pages` to locate the related page element. This field is skipped when its value is the default `0`. See https://developers.google.com/protocol-buffers/docs/proto3#json.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub page: Option<i64>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentPageAnchorPageRef {}


/// A block has a set of lines (collected into paragraphs) that have a common line-spacing and orientation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentPageBlock {
    /// A list of detected languages together with confidence.
    #[serde(rename="detectedLanguages")]
    
    pub detected_languages: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguage>>,
    /// Layout for Block.
    
    pub layout: Option<GoogleCloudDocumentaiV1beta2DocumentPageLayout>,
    /// The history of this annotation.
    
    pub provenance: Option<GoogleCloudDocumentaiV1beta2DocumentProvenance>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentPageBlock {}


/// A detected barcode.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentPageDetectedBarcode {
    /// Detailed barcode information of the DetectedBarcode.
    
    pub barcode: Option<GoogleCloudDocumentaiV1beta2Barcode>,
    /// Layout for DetectedBarcode.
    
    pub layout: Option<GoogleCloudDocumentaiV1beta2DocumentPageLayout>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentPageDetectedBarcode {}


/// Detected language for a structural component.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguage {
    /// Confidence of detected language. Range `[0, 1]`.
    
    pub confidence: Option<f32>,
    /// The [BCP-47 language code](https://www.unicode.org/reports/tr35/#Unicode_locale_identifier), such as `en-US` or `sr-Latn`.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguage {}


/// Dimension for the page.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentPageDimension {
    /// Page height.
    
    pub height: Option<f32>,
    /// Dimension unit.
    
    pub unit: Option<String>,
    /// Page width.
    
    pub width: Option<f32>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentPageDimension {}


/// A form field detected on the page.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentPageFormField {
    /// Created for Labeling UI to export key text. If corrections were made to the text identified by the `field_name.text_anchor`, this field will contain the correction.
    #[serde(rename="correctedKeyText")]
    
    pub corrected_key_text: Option<String>,
    /// Created for Labeling UI to export value text. If corrections were made to the text identified by the `field_value.text_anchor`, this field will contain the correction.
    #[serde(rename="correctedValueText")]
    
    pub corrected_value_text: Option<String>,
    /// Layout for the FormField name. e.g. `Address`, `Email`, `Grand total`, `Phone number`, etc.
    #[serde(rename="fieldName")]
    
    pub field_name: Option<GoogleCloudDocumentaiV1beta2DocumentPageLayout>,
    /// Layout for the FormField value.
    #[serde(rename="fieldValue")]
    
    pub field_value: Option<GoogleCloudDocumentaiV1beta2DocumentPageLayout>,
    /// A list of detected languages for name together with confidence.
    #[serde(rename="nameDetectedLanguages")]
    
    pub name_detected_languages: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguage>>,
    /// The history of this annotation.
    
    pub provenance: Option<GoogleCloudDocumentaiV1beta2DocumentProvenance>,
    /// A list of detected languages for value together with confidence.
    #[serde(rename="valueDetectedLanguages")]
    
    pub value_detected_languages: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguage>>,
    /// If the value is non-textual, this field represents the type. Current valid values are: - blank (this indicates the `field_value` is normal text) - `unfilled_checkbox` - `filled_checkbox`
    #[serde(rename="valueType")]
    
    pub value_type: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentPageFormField {}


/// Rendered image contents for this page.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentPageImage {
    /// Raw byte content of the image.
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub content: Option<Vec<u8>>,
    /// Height of the image in pixels.
    
    pub height: Option<i32>,
    /// Encoding [media type (MIME type)](https://www.iana.org/assignments/media-types/media-types.xhtml) for the image.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
    /// Width of the image in pixels.
    
    pub width: Option<i32>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentPageImage {}


/// Image quality scores for the page image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentPageImageQualityScores {
    /// A list of detected defects.
    #[serde(rename="detectedDefects")]
    
    pub detected_defects: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageImageQualityScoresDetectedDefect>>,
    /// The overall quality score. Range `[0, 1]` where `1` is perfect quality.
    #[serde(rename="qualityScore")]
    
    pub quality_score: Option<f32>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentPageImageQualityScores {}


/// Image Quality Defects
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentPageImageQualityScoresDetectedDefect {
    /// Confidence of detected defect. Range `[0, 1]` where `1` indicates strong confidence that the defect exists.
    
    pub confidence: Option<f32>,
    /// Name of the defect type. Supported values are: - `quality/defect_blurry` - `quality/defect_noisy` - `quality/defect_dark` - `quality/defect_faint` - `quality/defect_text_too_small` - `quality/defect_document_cutoff` - `quality/defect_text_cutoff` - `quality/defect_glare`
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentPageImageQualityScoresDetectedDefect {}


/// Visual element describing a layout unit on a page.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentPageLayout {
    /// The bounding polygon for the Layout.
    #[serde(rename="boundingPoly")]
    
    pub bounding_poly: Option<GoogleCloudDocumentaiV1beta2BoundingPoly>,
    /// Confidence of the current Layout within context of the object this layout is for. e.g. confidence can be for a single token, a table, a visual element, etc. depending on context. Range `[0, 1]`.
    
    pub confidence: Option<f32>,
    /// Detected orientation for the Layout.
    
    pub orientation: Option<String>,
    /// Text anchor indexing into the Document.text.
    #[serde(rename="textAnchor")]
    
    pub text_anchor: Option<GoogleCloudDocumentaiV1beta2DocumentTextAnchor>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentPageLayout {}


/// A collection of tokens that a human would perceive as a line. Does not cross column boundaries, can be horizontal, vertical, etc.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentPageLine {
    /// A list of detected languages together with confidence.
    #[serde(rename="detectedLanguages")]
    
    pub detected_languages: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguage>>,
    /// Layout for Line.
    
    pub layout: Option<GoogleCloudDocumentaiV1beta2DocumentPageLayout>,
    /// The history of this annotation.
    
    pub provenance: Option<GoogleCloudDocumentaiV1beta2DocumentProvenance>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentPageLine {}


/// Representation for transformation matrix, intended to be compatible and used with OpenCV format for image manipulation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentPageMatrix {
    /// Number of columns in the matrix.
    
    pub cols: Option<i32>,
    /// The matrix data.
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub data: Option<Vec<u8>>,
    /// Number of rows in the matrix.
    
    pub rows: Option<i32>,
    /// This encodes information about what data type the matrix uses. For example, 0 (CV_8U) is an unsigned 8-bit image. For the full list of OpenCV primitive data types, please refer to https://docs.opencv.org/4.3.0/d1/d1b/group__core__hal__interface.html
    #[serde(rename="type")]
    
    pub type_: Option<i32>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentPageMatrix {}


/// A collection of lines that a human would perceive as a paragraph.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentPageParagraph {
    /// A list of detected languages together with confidence.
    #[serde(rename="detectedLanguages")]
    
    pub detected_languages: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguage>>,
    /// Layout for Paragraph.
    
    pub layout: Option<GoogleCloudDocumentaiV1beta2DocumentPageLayout>,
    /// The history of this annotation.
    
    pub provenance: Option<GoogleCloudDocumentaiV1beta2DocumentProvenance>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentPageParagraph {}


/// A detected symbol.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentPageSymbol {
    /// A list of detected languages together with confidence.
    #[serde(rename="detectedLanguages")]
    
    pub detected_languages: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguage>>,
    /// Layout for Symbol.
    
    pub layout: Option<GoogleCloudDocumentaiV1beta2DocumentPageLayout>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentPageSymbol {}


/// A table representation similar to HTML table structure.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentPageTable {
    /// Body rows of the table.
    #[serde(rename="bodyRows")]
    
    pub body_rows: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageTableTableRow>>,
    /// A list of detected languages together with confidence.
    #[serde(rename="detectedLanguages")]
    
    pub detected_languages: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguage>>,
    /// Header rows of the table.
    #[serde(rename="headerRows")]
    
    pub header_rows: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageTableTableRow>>,
    /// Layout for Table.
    
    pub layout: Option<GoogleCloudDocumentaiV1beta2DocumentPageLayout>,
    /// The history of this table.
    
    pub provenance: Option<GoogleCloudDocumentaiV1beta2DocumentProvenance>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentPageTable {}


/// A cell representation inside the table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentPageTableTableCell {
    /// How many columns this cell spans.
    #[serde(rename="colSpan")]
    
    pub col_span: Option<i32>,
    /// A list of detected languages together with confidence.
    #[serde(rename="detectedLanguages")]
    
    pub detected_languages: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguage>>,
    /// Layout for TableCell.
    
    pub layout: Option<GoogleCloudDocumentaiV1beta2DocumentPageLayout>,
    /// How many rows this cell spans.
    #[serde(rename="rowSpan")]
    
    pub row_span: Option<i32>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentPageTableTableCell {}


/// A row of table cells.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentPageTableTableRow {
    /// Cells that make up this row.
    
    pub cells: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageTableTableCell>>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentPageTableTableRow {}


/// A detected token.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentPageToken {
    /// Detected break at the end of a Token.
    #[serde(rename="detectedBreak")]
    
    pub detected_break: Option<GoogleCloudDocumentaiV1beta2DocumentPageTokenDetectedBreak>,
    /// A list of detected languages together with confidence.
    #[serde(rename="detectedLanguages")]
    
    pub detected_languages: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguage>>,
    /// Layout for Token.
    
    pub layout: Option<GoogleCloudDocumentaiV1beta2DocumentPageLayout>,
    /// The history of this annotation.
    
    pub provenance: Option<GoogleCloudDocumentaiV1beta2DocumentProvenance>,
    /// Text style attributes.
    #[serde(rename="styleInfo")]
    
    pub style_info: Option<GoogleCloudDocumentaiV1beta2DocumentPageTokenStyleInfo>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentPageToken {}


/// Detected break at the end of a Token.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentPageTokenDetectedBreak {
    /// Detected break type.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentPageTokenDetectedBreak {}


/// Font and other text style attributes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentPageTokenStyleInfo {
    /// Color of the background.
    #[serde(rename="backgroundColor")]
    
    pub background_color: Option<GoogleTypeColor>,
    /// Whether the text is bold (equivalent to font_weight is at least `700`).
    
    pub bold: Option<bool>,
    /// Font size in points (`1` point is `¹⁄₇₂` inches).
    #[serde(rename="fontSize")]
    
    pub font_size: Option<i32>,
    /// Name or style of the font.
    #[serde(rename="fontType")]
    
    pub font_type: Option<String>,
    /// TrueType weight on a scale `100` (thin) to `1000` (ultra-heavy). Normal is `400`, bold is `700`.
    #[serde(rename="fontWeight")]
    
    pub font_weight: Option<i32>,
    /// Whether the text is handwritten.
    
    pub handwritten: Option<bool>,
    /// Whether the text is italic.
    
    pub italic: Option<bool>,
    /// Letter spacing in points.
    #[serde(rename="letterSpacing")]
    
    pub letter_spacing: Option<f64>,
    /// Font size in pixels, equal to _unrounded font_size_ * _resolution_ ÷ `72.0`.
    #[serde(rename="pixelFontSize")]
    
    pub pixel_font_size: Option<f64>,
    /// Whether the text is in small caps. This feature is not supported yet.
    
    pub smallcaps: Option<bool>,
    /// Whether the text is strikethrough. This feature is not supported yet.
    
    pub strikeout: Option<bool>,
    /// Whether the text is a subscript. This feature is not supported yet.
    
    pub subscript: Option<bool>,
    /// Whether the text is a superscript. This feature is not supported yet.
    
    pub superscript: Option<bool>,
    /// Color of the text.
    #[serde(rename="textColor")]
    
    pub text_color: Option<GoogleTypeColor>,
    /// Whether the text is underlined.
    
    pub underlined: Option<bool>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentPageTokenStyleInfo {}


/// Detected non-text visual elements e.g. checkbox, signature etc. on the page.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentPageVisualElement {
    /// A list of detected languages together with confidence.
    #[serde(rename="detectedLanguages")]
    
    pub detected_languages: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguage>>,
    /// Layout for VisualElement.
    
    pub layout: Option<GoogleCloudDocumentaiV1beta2DocumentPageLayout>,
    /// Type of the VisualElement.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentPageVisualElement {}


/// Structure to identify provenance relationships between annotations in different revisions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentProvenance {
    /// The Id of this operation. Needs to be unique within the scope of the revision.
    
    pub id: Option<i32>,
    /// References to the original elements that are replaced.
    
    pub parents: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentProvenanceParent>>,
    /// The index of the revision that produced this element.
    
    pub revision: Option<i32>,
    /// The type of provenance operation.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentProvenance {}


/// The parent element the current element is based on. Used for referencing/aligning, removal and replacement operations.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentProvenanceParent {
    /// The id of the parent provenance.
    
    pub id: Option<i32>,
    /// The index of the parent item in the corresponding item list (eg. list of entities, properties within entities, etc.) in the parent revision.
    
    pub index: Option<i32>,
    /// The index of the index into current revision's parent_ids list.
    
    pub revision: Option<i32>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentProvenanceParent {}


/// Contains past or forward revisions of this document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentRevision {
    /// If the change was made by a person specify the name or id of that person.
    
    pub agent: Option<String>,
    /// The time that the revision was created, internally generated by doc proto storage at the time of create.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Human Review information of this revision.
    #[serde(rename="humanReview")]
    
    pub human_review: Option<GoogleCloudDocumentaiV1beta2DocumentRevisionHumanReview>,
    /// Id of the revision, internally generated by doc proto storage. Unique within the context of the document.
    
    pub id: Option<String>,
    /// The revisions that this revision is based on. This can include one or more parent (when documents are merged.) This field represents the index into the `revisions` field.
    
    pub parent: Option<Vec<i32>>,
    /// The revisions that this revision is based on. Must include all the ids that have anything to do with this revision - eg. there are `provenance.parent.revision` fields that index into this field.
    #[serde(rename="parentIds")]
    
    pub parent_ids: Option<Vec<String>>,
    /// If the annotation was made by processor identify the processor by its resource name.
    
    pub processor: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentRevision {}


/// Human Review information of the document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentRevisionHumanReview {
    /// Human review state. e.g. `requested`, `succeeded`, `rejected`.
    
    pub state: Option<String>,
    /// A message providing more details about the current state of processing. For example, the rejection reason when the state is `rejected`.
    #[serde(rename="stateMessage")]
    
    pub state_message: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentRevisionHumanReview {}


/// For a large document, sharding may be performed to produce several document shards. Each document shard contains this field to detail which shard it is.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentShardInfo {
    /// Total number of shards.
    #[serde(rename="shardCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub shard_count: Option<i64>,
    /// The 0-based index of this shard.
    #[serde(rename="shardIndex")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub shard_index: Option<i64>,
    /// The index of the first character in Document.text in the overall document global text.
    #[serde(rename="textOffset")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub text_offset: Option<i64>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentShardInfo {}


/// Annotation for common text style attributes. This adheres to CSS conventions as much as possible.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentStyle {
    /// Text background color.
    #[serde(rename="backgroundColor")]
    
    pub background_color: Option<GoogleTypeColor>,
    /// Text color.
    
    pub color: Option<GoogleTypeColor>,
    /// Font family such as `Arial`, `Times New Roman`. https://www.w3schools.com/cssref/pr_font_font-family.asp
    #[serde(rename="fontFamily")]
    
    pub font_family: Option<String>,
    /// Font size.
    #[serde(rename="fontSize")]
    
    pub font_size: Option<GoogleCloudDocumentaiV1beta2DocumentStyleFontSize>,
    /// [Font weight](https://www.w3schools.com/cssref/pr_font_weight.asp). Possible values are `normal`, `bold`, `bolder`, and `lighter`.
    #[serde(rename="fontWeight")]
    
    pub font_weight: Option<String>,
    /// Text anchor indexing into the Document.text.
    #[serde(rename="textAnchor")]
    
    pub text_anchor: Option<GoogleCloudDocumentaiV1beta2DocumentTextAnchor>,
    /// [Text decoration](https://www.w3schools.com/cssref/pr_text_text-decoration.asp). Follows CSS standard. 
    #[serde(rename="textDecoration")]
    
    pub text_decoration: Option<String>,
    /// [Text style](https://www.w3schools.com/cssref/pr_font_font-style.asp). Possible values are `normal`, `italic`, and `oblique`.
    #[serde(rename="textStyle")]
    
    pub text_style: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentStyle {}


/// Font size with unit.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentStyleFontSize {
    /// Font size for the text.
    
    pub size: Option<f32>,
    /// Unit for the font size. Follows CSS naming (such as `in`, `px`, and `pt`).
    
    pub unit: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentStyleFontSize {}


/// Text reference indexing into the Document.text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentTextAnchor {
    /// Contains the content of the text span so that users do not have to look it up in the text_segments. It is always populated for formFields.
    
    pub content: Option<String>,
    /// The text segments from the Document.text.
    #[serde(rename="textSegments")]
    
    pub text_segments: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentTextAnchorTextSegment>>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentTextAnchor {}


/// A text segment in the Document.text. The indices may be out of bounds which indicate that the text extends into another document shard for large sharded documents. See ShardInfo.text_offset
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentTextAnchorTextSegment {
    /// TextSegment half open end UTF-8 char index in the Document.text.
    #[serde(rename="endIndex")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub end_index: Option<i64>,
    /// TextSegment start UTF-8 char index in the Document.text.
    #[serde(rename="startIndex")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub start_index: Option<i64>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentTextAnchorTextSegment {}


/// This message is used for text changes aka. OCR corrections.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentTextChange {
    /// The text that replaces the text identified in the `text_anchor`.
    #[serde(rename="changedText")]
    
    pub changed_text: Option<String>,
    /// The history of this annotation.
    
    pub provenance: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentProvenance>>,
    /// Provenance of the correction. Text anchor indexing into the Document.text. There can only be a single `TextAnchor.text_segments` element. If the start and end index of the text segment are the same, the text change is inserted before that index.
    #[serde(rename="textAnchor")]
    
    pub text_anchor: Option<GoogleCloudDocumentaiV1beta2DocumentTextAnchor>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2DocumentTextChange {}


/// Parameters to control entity extraction behavior.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2EntityExtractionParams {
    /// Whether to enable entity extraction.
    
    pub enabled: Option<bool>,
    /// Model version of the entity extraction. Default is "builtin/stable". Specify "builtin/latest" for the latest model.
    #[serde(rename="modelVersion")]
    
    pub model_version: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2EntityExtractionParams {}


/// Parameters to control form extraction behavior.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2FormExtractionParams {
    /// Whether to enable form extraction.
    
    pub enabled: Option<bool>,
    /// Reserved for future use.
    #[serde(rename="keyValuePairHints")]
    
    pub key_value_pair_hints: Option<Vec<GoogleCloudDocumentaiV1beta2KeyValuePairHint>>,
    /// Model version of the form extraction system. Default is "builtin/stable". Specify "builtin/latest" for the latest model. For custom form models, specify: "custom/{model_name}". Model name format is "bucket_name/path/to/modeldir" corresponding to "gs://bucket_name/path/to/modeldir" where annotated examples are stored.
    #[serde(rename="modelVersion")]
    
    pub model_version: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2FormExtractionParams {}


/// The Google Cloud Storage location where the output file will be written to.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2GcsDestination {
    /// no description provided
    
    pub uri: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2GcsDestination {}


/// The Google Cloud Storage location where the input file will be read from.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2GcsSource {
    /// no description provided
    
    pub uri: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2GcsSource {}


/// The desired input location and metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2InputConfig {
    /// Content in bytes, represented as a stream of bytes. Note: As with all `bytes` fields, proto buffer messages use a pure binary representation, whereas JSON representations use base64. This field only works for synchronous ProcessDocument method.
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub contents: Option<Vec<u8>>,
    /// The Google Cloud Storage location to read the input from. This must be a single file.
    #[serde(rename="gcsSource")]
    
    pub gcs_source: Option<GoogleCloudDocumentaiV1beta2GcsSource>,
    /// Required. Mimetype of the input. Current supported mimetypes are application/pdf, image/tiff, and image/gif. In addition, application/json type is supported for requests with ProcessDocumentRequest.automl_params field set. The JSON file needs to be in Document format.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2InputConfig {}


/// Reserved for future use.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2KeyValuePairHint {
    /// The key text for the hint.
    
    pub key: Option<String>,
    /// Type of the value. This is case-insensitive, and could be one of: ADDRESS, LOCATION, ORGANIZATION, PERSON, PHONE_NUMBER, ID, NUMBER, EMAIL, PRICE, TERMS, DATE, NAME. Types not in this list will be ignored.
    #[serde(rename="valueTypes")]
    
    pub value_types: Option<Vec<String>>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2KeyValuePairHint {}


/// A vertex represents a 2D point in the image. NOTE: the normalized vertex coordinates are relative to the original image and range from 0 to 1.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2NormalizedVertex {
    /// X coordinate.
    
    pub x: Option<f32>,
    /// Y coordinate (starts from the top of the image).
    
    pub y: Option<f32>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2NormalizedVertex {}


/// Parameters to control Optical Character Recognition (OCR) behavior.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2OcrParams {
    /// List of languages to use for OCR. In most cases, an empty value yields the best results since it enables automatic language detection. For languages based on the Latin alphabet, setting `language_hints` is not needed. In rare cases, when the language of the text in the image is known, setting a hint will help get better results (although it will be a significant hindrance if the hint is wrong). Document processing returns an error if one or more of the specified languages is not one of the supported languages.
    #[serde(rename="languageHints")]
    
    pub language_hints: Option<Vec<String>>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2OcrParams {}


/// The desired output location and metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2OutputConfig {
    /// The Google Cloud Storage location to write the output to.
    #[serde(rename="gcsDestination")]
    
    pub gcs_destination: Option<GoogleCloudDocumentaiV1beta2GcsDestination>,
    /// The max number of pages to include into each output Document shard JSON on Google Cloud Storage. The valid range is [1, 100]. If not specified, the default value is 20. For example, for one pdf file with 100 pages, 100 parsed pages will be produced. If `pages_per_shard` = 20, then 5 Document shard JSON files each containing 20 parsed pages will be written under the prefix OutputConfig.gcs_destination.uri and suffix pages-x-to-y.json where x and y are 1-indexed page numbers. Example GCS outputs with 157 pages and pages_per_shard = 50: pages-001-to-050.json pages-051-to-100.json pages-101-to-150.json pages-151-to-157.json
    #[serde(rename="pagesPerShard")]
    
    pub pages_per_shard: Option<i32>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2OutputConfig {}


/// Request to process one document.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [documents process projects](ProjectDocumentProcesCall) (request)
/// * [locations documents process projects](ProjectLocationDocumentProcesCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2ProcessDocumentRequest {
    /// Controls AutoML model prediction behavior. AutoMlParams cannot be used together with other Params.
    #[serde(rename="automlParams")]
    
    pub automl_params: Option<GoogleCloudDocumentaiV1beta2AutoMlParams>,
    /// Specifies a known document type for deeper structure detection. Valid values are currently "general" and "invoice". If not provided, "general"\ is used as default. If any other value is given, the request is rejected.
    #[serde(rename="documentType")]
    
    pub document_type: Option<String>,
    /// Controls entity extraction behavior. If not specified, the system will decide reasonable defaults.
    #[serde(rename="entityExtractionParams")]
    
    pub entity_extraction_params: Option<GoogleCloudDocumentaiV1beta2EntityExtractionParams>,
    /// Controls form extraction behavior. If not specified, the system will decide reasonable defaults.
    #[serde(rename="formExtractionParams")]
    
    pub form_extraction_params: Option<GoogleCloudDocumentaiV1beta2FormExtractionParams>,
    /// Required. Information about the input file.
    #[serde(rename="inputConfig")]
    
    pub input_config: Option<GoogleCloudDocumentaiV1beta2InputConfig>,
    /// Controls OCR behavior. If not specified, the system will decide reasonable defaults.
    #[serde(rename="ocrParams")]
    
    pub ocr_params: Option<GoogleCloudDocumentaiV1beta2OcrParams>,
    /// The desired output location. This field is only needed in BatchProcessDocumentsRequest.
    #[serde(rename="outputConfig")]
    
    pub output_config: Option<GoogleCloudDocumentaiV1beta2OutputConfig>,
    /// Target project and location to make a call. Format: `projects/{project-id}/locations/{location-id}`. If no location is specified, a region will be chosen automatically. This field is only populated when used in ProcessDocument method.
    
    pub parent: Option<String>,
    /// Controls table extraction behavior. If not specified, the system will decide reasonable defaults.
    #[serde(rename="tableExtractionParams")]
    
    pub table_extraction_params: Option<GoogleCloudDocumentaiV1beta2TableExtractionParams>,
}

impl client::RequestValue for GoogleCloudDocumentaiV1beta2ProcessDocumentRequest {}


/// A hint for a table bounding box on the page for table parsing.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2TableBoundHint {
    /// Bounding box hint for a table on this page. The coordinates must be normalized to [0,1] and the bounding box must be an axis-aligned rectangle.
    #[serde(rename="boundingBox")]
    
    pub bounding_box: Option<GoogleCloudDocumentaiV1beta2BoundingPoly>,
    /// Optional. Page number for multi-paged inputs this hint applies to. If not provided, this hint will apply to all pages by default. This value is 1-based.
    #[serde(rename="pageNumber")]
    
    pub page_number: Option<i32>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2TableBoundHint {}


/// Parameters to control table extraction behavior.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2TableExtractionParams {
    /// Whether to enable table extraction.
    
    pub enabled: Option<bool>,
    /// Optional. Reserved for future use.
    #[serde(rename="headerHints")]
    
    pub header_hints: Option<Vec<String>>,
    /// Model version of the table extraction system. Default is "builtin/stable". Specify "builtin/latest" for the latest model.
    #[serde(rename="modelVersion")]
    
    pub model_version: Option<String>,
    /// Optional. Table bounding box hints that can be provided to complex cases which our algorithm cannot locate the table(s) in.
    #[serde(rename="tableBoundHints")]
    
    pub table_bound_hints: Option<Vec<GoogleCloudDocumentaiV1beta2TableBoundHint>>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2TableExtractionParams {}


/// A vertex represents a 2D point in the image. NOTE: the vertex coordinates are in the same scale as the original image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2Vertex {
    /// X coordinate.
    
    pub x: Option<i32>,
    /// Y coordinate (starts from the top of the image).
    
    pub y: Option<i32>,
}

impl client::Part for GoogleCloudDocumentaiV1beta2Vertex {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [documents batch process projects](ProjectDocumentBatchProcesCall) (response)
/// * [locations documents batch process projects](ProjectLocationDocumentBatchProcesCall) (response)
/// * [locations operations get projects](ProjectLocationOperationGetCall) (response)
/// * [operations get projects](ProjectOperationGetCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleLongrunningOperation {
    /// If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available.
    
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    
    pub error: Option<GoogleRpcStatus>,
    /// Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`.
    
    pub name: Option<String>,
    /// The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`.
    
    pub response: Option<HashMap<String, json::Value>>,
}

impl client::ResponseResult for GoogleLongrunningOperation {}


/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleRpcStatus {
    /// The status code, which should be an enum value of google.rpc.Code.
    
    pub code: Option<i32>,
    /// A list of messages that carry the error details. There is a common set of message types for APIs to use.
    
    pub details: Option<Vec<HashMap<String, json::Value>>>,
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client.
    
    pub message: Option<String>,
}

impl client::Part for GoogleRpcStatus {}


/// Represents a color in the RGBA color space. This representation is designed for simplicity of conversion to and from color representations in various languages over compactness. For example, the fields of this representation can be trivially provided to the constructor of `java.awt.Color` in Java; it can also be trivially provided to UIColor's `+colorWithRed:green:blue:alpha` method in iOS; and, with just a little work, it can be easily formatted into a CSS `rgba()` string in JavaScript. This reference page doesn't have information about the absolute color space that should be used to interpret the RGB value—for example, sRGB, Adobe RGB, DCI-P3, and BT.2020. By default, applications should assume the sRGB color space. When color equality needs to be decided, implementations, unless documented otherwise, treat two colors as equal if all their red, green, blue, and alpha values each differ by at most `1e-5`. Example (Java): import com.google.type.Color; // ... public static java.awt.Color fromProto(Color protocolor) { float alpha = protocolor.hasAlpha() ? protocolor.getAlpha().getValue() : 1.0; return new java.awt.Color( protocolor.getRed(), protocolor.getGreen(), protocolor.getBlue(), alpha); } public static Color toProto(java.awt.Color color) { float red = (float) color.getRed(); float green = (float) color.getGreen(); float blue = (float) color.getBlue(); float denominator = 255.0; Color.Builder resultBuilder = Color .newBuilder() .setRed(red / denominator) .setGreen(green / denominator) .setBlue(blue / denominator); int alpha = color.getAlpha(); if (alpha != 255) { result.setAlpha( FloatValue .newBuilder() .setValue(((float) alpha) / denominator) .build()); } return resultBuilder.build(); } // ... Example (iOS / Obj-C): // ... static UIColor* fromProto(Color* protocolor) { float red = [protocolor red]; float green = [protocolor green]; float blue = [protocolor blue]; FloatValue* alpha_wrapper = [protocolor alpha]; float alpha = 1.0; if (alpha_wrapper != nil) { alpha = [alpha_wrapper value]; } return [UIColor colorWithRed:red green:green blue:blue alpha:alpha]; } static Color* toProto(UIColor* color) { CGFloat red, green, blue, alpha; if (![color getRed:&red green:&green blue:&blue alpha:&alpha]) { return nil; } Color* result = [[Color alloc] init]; [result setRed:red]; [result setGreen:green]; [result setBlue:blue]; if (alpha <= 0.9999) { [result setAlpha:floatWrapperWithValue(alpha)]; } [result autorelease]; return result; } // ... Example (JavaScript): // ... var protoToCssColor = function(rgb_color) { var redFrac = rgb_color.red || 0.0; var greenFrac = rgb_color.green || 0.0; var blueFrac = rgb_color.blue || 0.0; var red = Math.floor(redFrac * 255); var green = Math.floor(greenFrac * 255); var blue = Math.floor(blueFrac * 255); if (!('alpha' in rgb_color)) { return rgbToCssColor(red, green, blue); } var alphaFrac = rgb_color.alpha.value || 0.0; var rgbParams = [red, green, blue].join(','); return ['rgba(', rgbParams, ',', alphaFrac, ')'].join(''); }; var rgbToCssColor = function(red, green, blue) { var rgbNumber = new Number((red << 16) | (green << 8) | blue); var hexString = rgbNumber.toString(16); var missingZeros = 6 - hexString.length; var resultBuilder = ['#']; for (var i = 0; i < missingZeros; i++) { resultBuilder.push('0'); } resultBuilder.push(hexString); return resultBuilder.join(''); }; // ...
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeColor {
    /// The fraction of this color that should be applied to the pixel. That is, the final pixel color is defined by the equation: `pixel color = alpha * (this color) + (1.0 - alpha) * (background color)` This means that a value of 1.0 corresponds to a solid color, whereas a value of 0.0 corresponds to a completely transparent color. This uses a wrapper message rather than a simple float scalar so that it is possible to distinguish between a default value and the value being unset. If omitted, this color object is rendered as a solid color (as if the alpha value had been explicitly given a value of 1.0).
    
    pub alpha: Option<f32>,
    /// The amount of blue in the color as a value in the interval [0, 1].
    
    pub blue: Option<f32>,
    /// The amount of green in the color as a value in the interval [0, 1].
    
    pub green: Option<f32>,
    /// The amount of red in the color as a value in the interval [0, 1].
    
    pub red: Option<f32>,
}

impl client::Part for GoogleTypeColor {}


/// Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values. * A month and day, with a zero year (for example, an anniversary). * A year on its own, with a zero month and a zero day. * A year and month, with a zero day (for example, a credit card expiration date). Related types: * google.type.TimeOfDay * google.type.DateTime * google.protobuf.Timestamp
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeDate {
    /// Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant.
    
    pub day: Option<i32>,
    /// Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day.
    
    pub month: Option<i32>,
    /// Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year.
    
    pub year: Option<i32>,
}

impl client::Part for GoogleTypeDate {}


/// Represents civil time (or occasionally physical time). This type can represent a civil time in one of a few possible ways: * When utc_offset is set and time_zone is unset: a civil time on a calendar day with a particular offset from UTC. * When time_zone is set and utc_offset is unset: a civil time on a calendar day in a particular time zone. * When neither time_zone nor utc_offset is set: a civil time on a calendar day in local time. The date is relative to the Proleptic Gregorian Calendar. If year, month, or day are 0, the DateTime is considered not to have a specific year, month, or day respectively. This type may also be used to represent a physical time if all the date and time fields are set and either case of the `time_offset` oneof is set. Consider using `Timestamp` message for physical time instead. If your use case also would like to store the user's timezone, that can be done in another field. This type is more flexible than some applications may want. Make sure to document and validate your application's limitations.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeDateTime {
    /// Optional. Day of month. Must be from 1 to 31 and valid for the year and month, or 0 if specifying a datetime without a day.
    
    pub day: Option<i32>,
    /// Optional. Hours of day in 24 hour format. Should be from 0 to 23, defaults to 0 (midnight). An API may choose to allow the value "24:00:00" for scenarios like business closing time.
    
    pub hours: Option<i32>,
    /// Optional. Minutes of hour of day. Must be from 0 to 59, defaults to 0.
    
    pub minutes: Option<i32>,
    /// Optional. Month of year. Must be from 1 to 12, or 0 if specifying a datetime without a month.
    
    pub month: Option<i32>,
    /// Optional. Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999, defaults to 0.
    
    pub nanos: Option<i32>,
    /// Optional. Seconds of minutes of the time. Must normally be from 0 to 59, defaults to 0. An API may allow the value 60 if it allows leap-seconds.
    
    pub seconds: Option<i32>,
    /// Time zone.
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<GoogleTypeTimeZone>,
    /// UTC offset. Must be whole seconds, between -18 hours and +18 hours. For example, a UTC offset of -4:00 would be represented as { seconds: -14400 }.
    #[serde(rename="utcOffset")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub utc_offset: Option<client::chrono::Duration>,
    /// Optional. Year of date. Must be from 1 to 9999, or 0 if specifying a datetime without a year.
    
    pub year: Option<i32>,
}

impl client::Part for GoogleTypeDateTime {}


/// Represents an amount of money with its currency type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeMoney {
    /// The three-letter currency code defined in ISO 4217.
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// Number of nano (10^-9) units of the amount. The value must be between -999,999,999 and +999,999,999 inclusive. If `units` is positive, `nanos` must be positive or zero. If `units` is zero, `nanos` can be positive, zero, or negative. If `units` is negative, `nanos` must be negative or zero. For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
    
    pub nanos: Option<i32>,
    /// The whole units of the amount. For example if `currencyCode` is `"USD"`, then 1 unit is one US dollar.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub units: Option<i64>,
}

impl client::Part for GoogleTypeMoney {}


/// Represents a postal address, e.g. for postal delivery or payments addresses. Given a postal address, a postal service can deliver items to a premise, P.O. Box or similar. It is not intended to model geographical locations (roads, towns, mountains). In typical usage an address would be created via user input or from importing existing data, depending on the type of process. Advice on address input / editing: - Use an internationalization-ready address widget such as https://github.com/google/libaddressinput) - Users should not be presented with UI elements for input or editing of fields outside countries where that field is used. For more guidance on how to use this schema, please see: https://support.google.com/business/answer/6397478
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypePostalAddress {
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

impl client::Part for GoogleTypePostalAddress {}


/// Represents a time zone from the [IANA Time Zone Database](https://www.iana.org/time-zones).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeTimeZone {
    /// IANA Time Zone Database time zone, e.g. "America/New_York".
    
    pub id: Option<String>,
    /// Optional. IANA Time Zone Database version number, e.g. "2019a".
    
    pub version: Option<String>,
}

impl client::Part for GoogleTypeTimeZone {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Document`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_documentai1_beta2 as documentai1_beta2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use documentai1_beta2::{Document, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Document::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `documents_batch_process(...)`, `documents_process(...)`, `locations_documents_batch_process(...)`, `locations_documents_process(...)`, `locations_operations_get(...)` and `operations_get(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

    hub: &'a Document<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// LRO endpoint to batch process many documents. The output is written to Cloud Storage as JSON in the [Document] format.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Target project and location to make a call. Format: `projects/{project-id}/locations/{location-id}`. If no location is specified, a region will be chosen automatically.
    pub fn documents_batch_process(&self, request: GoogleCloudDocumentaiV1beta2BatchProcessDocumentsRequest, parent: &str) -> ProjectDocumentBatchProcesCall<'a, S> {
        ProjectDocumentBatchProcesCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Processes a single document.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Target project and location to make a call. Format: `projects/{project-id}/locations/{location-id}`. If no location is specified, a region will be chosen automatically. This field is only populated when used in ProcessDocument method.
    pub fn documents_process(&self, request: GoogleCloudDocumentaiV1beta2ProcessDocumentRequest, parent: &str) -> ProjectDocumentProcesCall<'a, S> {
        ProjectDocumentProcesCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// LRO endpoint to batch process many documents. The output is written to Cloud Storage as JSON in the [Document] format.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Target project and location to make a call. Format: `projects/{project-id}/locations/{location-id}`. If no location is specified, a region will be chosen automatically.
    pub fn locations_documents_batch_process(&self, request: GoogleCloudDocumentaiV1beta2BatchProcessDocumentsRequest, parent: &str) -> ProjectLocationDocumentBatchProcesCall<'a, S> {
        ProjectLocationDocumentBatchProcesCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Processes a single document.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Target project and location to make a call. Format: `projects/{project-id}/locations/{location-id}`. If no location is specified, a region will be chosen automatically. This field is only populated when used in ProcessDocument method.
    pub fn locations_documents_process(&self, request: GoogleCloudDocumentaiV1beta2ProcessDocumentRequest, parent: &str) -> ProjectLocationDocumentProcesCall<'a, S> {
        ProjectLocationDocumentProcesCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn locations_operations_get(&self, name: &str) -> ProjectLocationOperationGetCall<'a, S> {
        ProjectLocationOperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn operations_get(&self, name: &str) -> ProjectOperationGetCall<'a, S> {
        ProjectOperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// LRO endpoint to batch process many documents. The output is written to Cloud Storage as JSON in the [Document] format.
///
/// A builder for the *documents.batchProcess* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_documentai1_beta2 as documentai1_beta2;
/// use documentai1_beta2::api::GoogleCloudDocumentaiV1beta2BatchProcessDocumentsRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use documentai1_beta2::{Document, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Document::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudDocumentaiV1beta2BatchProcessDocumentsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().documents_batch_process(req, "parent")
///              .doit().await;
/// # }
/// ```
pub struct ProjectDocumentBatchProcesCall<'a, S>
    where S: 'a {

    hub: &'a Document<S>,
    _request: GoogleCloudDocumentaiV1beta2BatchProcessDocumentsRequest,
    _parent: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for ProjectDocumentBatchProcesCall<'a, S> {}

impl<'a, S> ProjectDocumentBatchProcesCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleLongrunningOperation)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "documentai.projects.documents.batchProcess",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "parent"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("parent", self._parent);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1beta2/{+parent}/documents:batchProcess";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+parent}", "parent")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["parent"];
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
    pub fn request(mut self, new_value: GoogleCloudDocumentaiV1beta2BatchProcessDocumentsRequest) -> ProjectDocumentBatchProcesCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Target project and location to make a call. Format: `projects/{project-id}/locations/{location-id}`. If no location is specified, a region will be chosen automatically.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> ProjectDocumentBatchProcesCall<'a, S> {
        self._parent = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ProjectDocumentBatchProcesCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectDocumentBatchProcesCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectDocumentBatchProcesCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectDocumentBatchProcesCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ProjectDocumentBatchProcesCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Processes a single document.
///
/// A builder for the *documents.process* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_documentai1_beta2 as documentai1_beta2;
/// use documentai1_beta2::api::GoogleCloudDocumentaiV1beta2ProcessDocumentRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use documentai1_beta2::{Document, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Document::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudDocumentaiV1beta2ProcessDocumentRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().documents_process(req, "parent")
///              .doit().await;
/// # }
/// ```
pub struct ProjectDocumentProcesCall<'a, S>
    where S: 'a {

    hub: &'a Document<S>,
    _request: GoogleCloudDocumentaiV1beta2ProcessDocumentRequest,
    _parent: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for ProjectDocumentProcesCall<'a, S> {}

impl<'a, S> ProjectDocumentProcesCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleCloudDocumentaiV1beta2Document)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "documentai.projects.documents.process",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "parent"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("parent", self._parent);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1beta2/{+parent}/documents:process";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+parent}", "parent")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["parent"];
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
    pub fn request(mut self, new_value: GoogleCloudDocumentaiV1beta2ProcessDocumentRequest) -> ProjectDocumentProcesCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Target project and location to make a call. Format: `projects/{project-id}/locations/{location-id}`. If no location is specified, a region will be chosen automatically. This field is only populated when used in ProcessDocument method.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> ProjectDocumentProcesCall<'a, S> {
        self._parent = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ProjectDocumentProcesCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectDocumentProcesCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectDocumentProcesCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectDocumentProcesCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ProjectDocumentProcesCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// LRO endpoint to batch process many documents. The output is written to Cloud Storage as JSON in the [Document] format.
///
/// A builder for the *locations.documents.batchProcess* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_documentai1_beta2 as documentai1_beta2;
/// use documentai1_beta2::api::GoogleCloudDocumentaiV1beta2BatchProcessDocumentsRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use documentai1_beta2::{Document, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Document::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudDocumentaiV1beta2BatchProcessDocumentsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().locations_documents_batch_process(req, "parent")
///              .doit().await;
/// # }
/// ```
pub struct ProjectLocationDocumentBatchProcesCall<'a, S>
    where S: 'a {

    hub: &'a Document<S>,
    _request: GoogleCloudDocumentaiV1beta2BatchProcessDocumentsRequest,
    _parent: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for ProjectLocationDocumentBatchProcesCall<'a, S> {}

impl<'a, S> ProjectLocationDocumentBatchProcesCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleLongrunningOperation)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "documentai.projects.locations.documents.batchProcess",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "parent"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("parent", self._parent);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1beta2/{+parent}/documents:batchProcess";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+parent}", "parent")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["parent"];
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
    pub fn request(mut self, new_value: GoogleCloudDocumentaiV1beta2BatchProcessDocumentsRequest) -> ProjectLocationDocumentBatchProcesCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Target project and location to make a call. Format: `projects/{project-id}/locations/{location-id}`. If no location is specified, a region will be chosen automatically.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> ProjectLocationDocumentBatchProcesCall<'a, S> {
        self._parent = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ProjectLocationDocumentBatchProcesCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectLocationDocumentBatchProcesCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectLocationDocumentBatchProcesCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectLocationDocumentBatchProcesCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ProjectLocationDocumentBatchProcesCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Processes a single document.
///
/// A builder for the *locations.documents.process* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_documentai1_beta2 as documentai1_beta2;
/// use documentai1_beta2::api::GoogleCloudDocumentaiV1beta2ProcessDocumentRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use documentai1_beta2::{Document, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Document::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudDocumentaiV1beta2ProcessDocumentRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().locations_documents_process(req, "parent")
///              .doit().await;
/// # }
/// ```
pub struct ProjectLocationDocumentProcesCall<'a, S>
    where S: 'a {

    hub: &'a Document<S>,
    _request: GoogleCloudDocumentaiV1beta2ProcessDocumentRequest,
    _parent: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for ProjectLocationDocumentProcesCall<'a, S> {}

impl<'a, S> ProjectLocationDocumentProcesCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleCloudDocumentaiV1beta2Document)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "documentai.projects.locations.documents.process",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "parent"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("parent", self._parent);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1beta2/{+parent}/documents:process";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+parent}", "parent")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["parent"];
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
    pub fn request(mut self, new_value: GoogleCloudDocumentaiV1beta2ProcessDocumentRequest) -> ProjectLocationDocumentProcesCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Target project and location to make a call. Format: `projects/{project-id}/locations/{location-id}`. If no location is specified, a region will be chosen automatically. This field is only populated when used in ProcessDocument method.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> ProjectLocationDocumentProcesCall<'a, S> {
        self._parent = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ProjectLocationDocumentProcesCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectLocationDocumentProcesCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectLocationDocumentProcesCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectLocationDocumentProcesCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ProjectLocationDocumentProcesCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.
///
/// A builder for the *locations.operations.get* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_documentai1_beta2 as documentai1_beta2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use documentai1_beta2::{Document, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Document::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().locations_operations_get("name")
///              .doit().await;
/// # }
/// ```
pub struct ProjectLocationOperationGetCall<'a, S>
    where S: 'a {

    hub: &'a Document<S>,
    _name: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for ProjectLocationOperationGetCall<'a, S> {}

impl<'a, S> ProjectLocationOperationGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleLongrunningOperation)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "documentai.projects.locations.operations.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        params.push("name", self._name);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1beta2/{+name}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["name"];
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


    /// The name of the operation resource.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> ProjectLocationOperationGetCall<'a, S> {
        self._name = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ProjectLocationOperationGetCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectLocationOperationGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectLocationOperationGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectLocationOperationGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ProjectLocationOperationGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.
///
/// A builder for the *operations.get* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_documentai1_beta2 as documentai1_beta2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use documentai1_beta2::{Document, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Document::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().operations_get("name")
///              .doit().await;
/// # }
/// ```
pub struct ProjectOperationGetCall<'a, S>
    where S: 'a {

    hub: &'a Document<S>,
    _name: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for ProjectOperationGetCall<'a, S> {}

impl<'a, S> ProjectOperationGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleLongrunningOperation)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "documentai.projects.operations.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        params.push("name", self._name);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1beta2/{+name}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["name"];
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


    /// The name of the operation resource.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> ProjectOperationGetCall<'a, S> {
        self._name = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ProjectOperationGetCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectOperationGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectOperationGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectOperationGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ProjectOperationGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


