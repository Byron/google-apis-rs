use super::*;
/// Encodes the detailed information of a barcode.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1Barcode {
    /// Format of a barcode. The supported formats are: - `CODE_128`: Code 128 type. - `CODE_39`: Code 39 type. - `CODE_93`: Code 93 type. - `CODABAR`: Codabar type. - `DATA_MATRIX`: 2D Data Matrix type. - `ITF`: ITF type. - `EAN_13`: EAN-13 type. - `EAN_8`: EAN-8 type. - `QR_CODE`: 2D QR code type. - `UPC_A`: UPC-A type. - `UPC_E`: UPC-E type. - `PDF417`: PDF417 type. - `AZTEC`: 2D Aztec code type. - `DATABAR`: GS1 DataBar code type.
    
    pub format: Option<String>,
    /// Raw value encoded in the barcode. For example: `'MEBKM:TITLE:Google;URL:https://www.google.com;;'`.
    #[serde(rename="rawValue")]
    
    pub raw_value: Option<String>,
    /// Value format describes the format of the value that a barcode encodes. The supported formats are: - `CONTACT_INFO`: Contact information. - `EMAIL`: Email address. - `ISBN`: ISBN identifier. - `PHONE`: Phone number. - `PRODUCT`: Product. - `SMS`: SMS message. - `TEXT`: Text string. - `URL`: URL address. - `WIFI`: Wifi information. - `GEO`: Geo-localization. - `CALENDAR_EVENT`: Calendar event. - `DRIVER_LICENSE`: Driver's license.
    #[serde(rename="valueFormat")]
    
    pub value_format: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1Barcode {}


/// The common config to specify a set of documents used as input.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1BatchDocumentsInputConfig {
    /// The set of documents individually specified on Cloud Storage.
    #[serde(rename="gcsDocuments")]
    
    pub gcs_documents: Option<GoogleCloudDocumentaiV1GcsDocuments>,
    /// The set of documents that match the specified Cloud Storage `gcs_prefix`.
    #[serde(rename="gcsPrefix")]
    
    pub gcs_prefix: Option<GoogleCloudDocumentaiV1GcsPrefix>,
}

impl client::Part for GoogleCloudDocumentaiV1BatchDocumentsInputConfig {}


/// Request message for batch process document method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations processors processor versions batch process projects](ProjectLocationProcessorProcessorVersionBatchProcesCall) (request)
/// * [locations processors batch process projects](ProjectLocationProcessorBatchProcesCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1BatchProcessRequest {
    /// The overall output config for batch process.
    #[serde(rename="documentOutputConfig")]
    
    pub document_output_config: Option<GoogleCloudDocumentaiV1DocumentOutputConfig>,
    /// The input documents for batch process.
    #[serde(rename="inputDocuments")]
    
    pub input_documents: Option<GoogleCloudDocumentaiV1BatchDocumentsInputConfig>,
    /// Whether Human Review feature should be skipped for this request. Default to false.
    #[serde(rename="skipHumanReview")]
    
    pub skip_human_review: Option<bool>,
}

impl client::RequestValue for GoogleCloudDocumentaiV1BatchProcessRequest {}


/// A bounding polygon for the detected image annotation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1BoundingPoly {
    /// The bounding polygon normalized vertices.
    #[serde(rename="normalizedVertices")]
    
    pub normalized_vertices: Option<Vec<GoogleCloudDocumentaiV1NormalizedVertex>>,
    /// The bounding polygon vertices.
    
    pub vertices: Option<Vec<GoogleCloudDocumentaiV1Vertex>>,
}

impl client::Part for GoogleCloudDocumentaiV1BoundingPoly {}


/// Request message for the deploy processor version method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations processors processor versions deploy projects](ProjectLocationProcessorProcessorVersionDeployCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DeployProcessorVersionRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleCloudDocumentaiV1DeployProcessorVersionRequest {}


/// Request message for the disable processor method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations processors disable projects](ProjectLocationProcessorDisableCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DisableProcessorRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleCloudDocumentaiV1DisableProcessorRequest {}


/// Document represents the canonical document resource in Document AI. It is an interchange format that provides insights into documents and allows for collaboration between users and Document AI to iterate and optimize for quality.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1Document {
    /// Optional. Inline document content, represented as a stream of bytes. Note: As with all `bytes` fields, protobuffers use a pure binary representation, whereas JSON representations use base64.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub content: Option<Vec<u8>>,
    /// A list of entities detected on Document.text. For document shards, entities in this list may cross shard boundaries.
    
    pub entities: Option<Vec<GoogleCloudDocumentaiV1DocumentEntity>>,
    /// Placeholder. Relationship among Document.entities.
    #[serde(rename="entityRelations")]
    
    pub entity_relations: Option<Vec<GoogleCloudDocumentaiV1DocumentEntityRelation>>,
    /// Any error that occurred while processing this document.
    
    pub error: Option<GoogleRpcStatus>,
    /// An IANA published MIME type (also referred to as media type). For more information, see https://www.iana.org/assignments/media-types/media-types.xhtml.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
    /// Visual page layout for the Document.
    
    pub pages: Option<Vec<GoogleCloudDocumentaiV1DocumentPage>>,
    /// Placeholder. Revision history of this document.
    
    pub revisions: Option<Vec<GoogleCloudDocumentaiV1DocumentRevision>>,
    /// Information about the sharding if this document is sharded part of a larger document. If the document is not sharded, this message is not specified.
    #[serde(rename="shardInfo")]
    
    pub shard_info: Option<GoogleCloudDocumentaiV1DocumentShardInfo>,
    /// Optional. UTF-8 encoded text in reading order from the document.
    
    pub text: Option<String>,
    /// Placeholder. A list of text corrections made to Document.text. This is usually used for annotating corrections to OCR mistakes. Text changes for a given revision may not overlap with each other.
    #[serde(rename="textChanges")]
    
    pub text_changes: Option<Vec<GoogleCloudDocumentaiV1DocumentTextChange>>,
    /// Styles for the Document.text.
    #[serde(rename="textStyles")]
    
    pub text_styles: Option<Vec<GoogleCloudDocumentaiV1DocumentStyle>>,
    /// Optional. Currently supports Google Cloud Storage URI of the form `gs://bucket_name/object_name`. Object versioning is not supported. See [Google Cloud Storage Request URIs](https://cloud.google.com/storage/docs/reference-uris) for more info.
    
    pub uri: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1Document {}


/// An entity that could be a phrase in the text or a property that belongs to the document. It is a known entity type, such as a person, an organization, or location.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentEntity {
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
    
    pub normalized_value: Option<GoogleCloudDocumentaiV1DocumentEntityNormalizedValue>,
    /// Optional. Represents the provenance of this entity wrt. the location on the page where it was found.
    #[serde(rename="pageAnchor")]
    
    pub page_anchor: Option<GoogleCloudDocumentaiV1DocumentPageAnchor>,
    /// Optional. Entities can be nested to form a hierarchical data structure representing the content in the document.
    
    pub properties: Option<Vec<GoogleCloudDocumentaiV1DocumentEntity>>,
    /// Optional. The history of this annotation.
    
    pub provenance: Option<GoogleCloudDocumentaiV1DocumentProvenance>,
    /// Optional. Whether the entity will be redacted for de-identification purposes.
    
    pub redacted: Option<bool>,
    /// Optional. Provenance of the entity. Text anchor indexing into the Document.text.
    #[serde(rename="textAnchor")]
    
    pub text_anchor: Option<GoogleCloudDocumentaiV1DocumentTextAnchor>,
    /// Required. Entity type from a schema e.g. `Address`.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentEntity {}


/// Parsed and normalized entity value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentEntityNormalizedValue {
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

impl client::Part for GoogleCloudDocumentaiV1DocumentEntityNormalizedValue {}


/// Relationship between Entities.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentEntityRelation {
    /// Object entity id.
    #[serde(rename="objectId")]
    
    pub object_id: Option<String>,
    /// Relationship description.
    
    pub relation: Option<String>,
    /// Subject entity id.
    #[serde(rename="subjectId")]
    
    pub subject_id: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentEntityRelation {}


/// Config that controls the output of documents. All documents will be written as a JSON file.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentOutputConfig {
    /// Output config to write the results to Cloud Storage.
    #[serde(rename="gcsOutputConfig")]
    
    pub gcs_output_config: Option<GoogleCloudDocumentaiV1DocumentOutputConfigGcsOutputConfig>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentOutputConfig {}


/// The configuration used when outputting documents.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentOutputConfigGcsOutputConfig {
    /// Specifies which fields to include in the output documents. Only supports top level document and pages field so it must be in the form of `{document_field_name}` or `pages.{page_field_name}`.
    #[serde(rename="fieldMask")]
    
    pub field_mask: Option<client::FieldMask>,
    /// The Cloud Storage uri (a directory) of the output.
    #[serde(rename="gcsUri")]
    
    pub gcs_uri: Option<String>,
    /// Specifies the sharding config for the output document.
    #[serde(rename="shardingConfig")]
    
    pub sharding_config: Option<GoogleCloudDocumentaiV1DocumentOutputConfigGcsOutputConfigShardingConfig>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentOutputConfigGcsOutputConfig {}


/// The sharding config for the output document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentOutputConfigGcsOutputConfigShardingConfig {
    /// The number of overlapping pages between consecutive shards.
    #[serde(rename="pagesOverlap")]
    
    pub pages_overlap: Option<i32>,
    /// The number of pages per shard.
    #[serde(rename="pagesPerShard")]
    
    pub pages_per_shard: Option<i32>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentOutputConfigGcsOutputConfigShardingConfig {}


/// A page in a Document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentPage {
    /// A list of visually detected text blocks on the page. A block has a set of lines (collected into paragraphs) that have a common line-spacing and orientation.
    
    pub blocks: Option<Vec<GoogleCloudDocumentaiV1DocumentPageBlock>>,
    /// A list of detected barcodes.
    #[serde(rename="detectedBarcodes")]
    
    pub detected_barcodes: Option<Vec<GoogleCloudDocumentaiV1DocumentPageDetectedBarcode>>,
    /// A list of detected languages together with confidence.
    #[serde(rename="detectedLanguages")]
    
    pub detected_languages: Option<Vec<GoogleCloudDocumentaiV1DocumentPageDetectedLanguage>>,
    /// Physical dimension of the page.
    
    pub dimension: Option<GoogleCloudDocumentaiV1DocumentPageDimension>,
    /// A list of visually detected form fields on the page.
    #[serde(rename="formFields")]
    
    pub form_fields: Option<Vec<GoogleCloudDocumentaiV1DocumentPageFormField>>,
    /// Rendered image for this page. This image is preprocessed to remove any skew, rotation, and distortions such that the annotation bounding boxes can be upright and axis-aligned.
    
    pub image: Option<GoogleCloudDocumentaiV1DocumentPageImage>,
    /// Image Quality Scores.
    #[serde(rename="imageQualityScores")]
    
    pub image_quality_scores: Option<GoogleCloudDocumentaiV1DocumentPageImageQualityScores>,
    /// Layout for the page.
    
    pub layout: Option<GoogleCloudDocumentaiV1DocumentPageLayout>,
    /// A list of visually detected text lines on the page. A collection of tokens that a human would perceive as a line.
    
    pub lines: Option<Vec<GoogleCloudDocumentaiV1DocumentPageLine>>,
    /// 1-based index for current Page in a parent Document. Useful when a page is taken out of a Document for individual processing.
    #[serde(rename="pageNumber")]
    
    pub page_number: Option<i32>,
    /// A list of visually detected text paragraphs on the page. A collection of lines that a human would perceive as a paragraph.
    
    pub paragraphs: Option<Vec<GoogleCloudDocumentaiV1DocumentPageParagraph>>,
    /// The history of this page.
    
    pub provenance: Option<GoogleCloudDocumentaiV1DocumentProvenance>,
    /// A list of visually detected symbols on the page.
    
    pub symbols: Option<Vec<GoogleCloudDocumentaiV1DocumentPageSymbol>>,
    /// A list of visually detected tables on the page.
    
    pub tables: Option<Vec<GoogleCloudDocumentaiV1DocumentPageTable>>,
    /// A list of visually detected tokens on the page.
    
    pub tokens: Option<Vec<GoogleCloudDocumentaiV1DocumentPageToken>>,
    /// Transformation matrices that were applied to the original document image to produce Page.image.
    
    pub transforms: Option<Vec<GoogleCloudDocumentaiV1DocumentPageMatrix>>,
    /// A list of detected non-text visual elements e.g. checkbox, signature etc. on the page.
    #[serde(rename="visualElements")]
    
    pub visual_elements: Option<Vec<GoogleCloudDocumentaiV1DocumentPageVisualElement>>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentPage {}


/// Referencing the visual context of the entity in the Document.pages. Page anchors can be cross-page, consist of multiple bounding polygons and optionally reference specific layout element types.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentPageAnchor {
    /// One or more references to visual page elements
    #[serde(rename="pageRefs")]
    
    pub page_refs: Option<Vec<GoogleCloudDocumentaiV1DocumentPageAnchorPageRef>>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentPageAnchor {}


/// Represents a weak reference to a page element within a document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentPageAnchorPageRef {
    /// Optional. Identifies the bounding polygon of a layout element on the page.
    #[serde(rename="boundingPoly")]
    
    pub bounding_poly: Option<GoogleCloudDocumentaiV1BoundingPoly>,
    /// Optional. Confidence of detected page element, if applicable. Range `[0, 1]`.
    
    pub confidence: Option<f32>,
    /// Optional. Deprecated. Use PageRef.bounding_poly instead.
    #[serde(rename="layoutId")]
    
    pub layout_id: Option<String>,
    /// Optional. The type of the layout element that is being referenced if any.
    #[serde(rename="layoutType")]
    
    pub layout_type: Option<GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutTypeEnum>,
    /// Required. Index into the Document.pages element, for example using `Document.pages` to locate the related page element. This field is skipped when its value is the default `0`. See https://developers.google.com/protocol-buffers/docs/proto3#json.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub page: Option<i64>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentPageAnchorPageRef {}


/// A block has a set of lines (collected into paragraphs) that have a common line-spacing and orientation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentPageBlock {
    /// A list of detected languages together with confidence.
    #[serde(rename="detectedLanguages")]
    
    pub detected_languages: Option<Vec<GoogleCloudDocumentaiV1DocumentPageDetectedLanguage>>,
    /// Layout for Block.
    
    pub layout: Option<GoogleCloudDocumentaiV1DocumentPageLayout>,
    /// The history of this annotation.
    
    pub provenance: Option<GoogleCloudDocumentaiV1DocumentProvenance>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentPageBlock {}


/// A detected barcode.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentPageDetectedBarcode {
    /// Detailed barcode information of the DetectedBarcode.
    
    pub barcode: Option<GoogleCloudDocumentaiV1Barcode>,
    /// Layout for DetectedBarcode.
    
    pub layout: Option<GoogleCloudDocumentaiV1DocumentPageLayout>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentPageDetectedBarcode {}


/// Detected language for a structural component.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentPageDetectedLanguage {
    /// Confidence of detected language. Range `[0, 1]`.
    
    pub confidence: Option<f32>,
    /// The BCP-47 language code, such as `en-US` or `sr-Latn`. For more information, see https://www.unicode.org/reports/tr35/#Unicode_locale_identifier.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentPageDetectedLanguage {}


/// Dimension for the page.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentPageDimension {
    /// Page height.
    
    pub height: Option<f32>,
    /// Dimension unit.
    
    pub unit: Option<String>,
    /// Page width.
    
    pub width: Option<f32>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentPageDimension {}


/// A form field detected on the page.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentPageFormField {
    /// Created for Labeling UI to export key text. If corrections were made to the text identified by the `field_name.text_anchor`, this field will contain the correction.
    #[serde(rename="correctedKeyText")]
    
    pub corrected_key_text: Option<String>,
    /// Created for Labeling UI to export value text. If corrections were made to the text identified by the `field_value.text_anchor`, this field will contain the correction.
    #[serde(rename="correctedValueText")]
    
    pub corrected_value_text: Option<String>,
    /// Layout for the FormField name. e.g. `Address`, `Email`, `Grand total`, `Phone number`, etc.
    #[serde(rename="fieldName")]
    
    pub field_name: Option<GoogleCloudDocumentaiV1DocumentPageLayout>,
    /// Layout for the FormField value.
    #[serde(rename="fieldValue")]
    
    pub field_value: Option<GoogleCloudDocumentaiV1DocumentPageLayout>,
    /// A list of detected languages for name together with confidence.
    #[serde(rename="nameDetectedLanguages")]
    
    pub name_detected_languages: Option<Vec<GoogleCloudDocumentaiV1DocumentPageDetectedLanguage>>,
    /// The history of this annotation.
    
    pub provenance: Option<GoogleCloudDocumentaiV1DocumentProvenance>,
    /// A list of detected languages for value together with confidence.
    #[serde(rename="valueDetectedLanguages")]
    
    pub value_detected_languages: Option<Vec<GoogleCloudDocumentaiV1DocumentPageDetectedLanguage>>,
    /// If the value is non-textual, this field represents the type. Current valid values are: - blank (this indicates the `field_value` is normal text) - `unfilled_checkbox` - `filled_checkbox`
    #[serde(rename="valueType")]
    
    pub value_type: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentPageFormField {}


/// Rendered image contents for this page.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentPageImage {
    /// Raw byte content of the image.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub content: Option<Vec<u8>>,
    /// Height of the image in pixels.
    
    pub height: Option<i32>,
    /// Encoding mime type for the image.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
    /// Width of the image in pixels.
    
    pub width: Option<i32>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentPageImage {}


/// Image Quality Scores for the page image
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentPageImageQualityScores {
    /// A list of detected defects.
    #[serde(rename="detectedDefects")]
    
    pub detected_defects: Option<Vec<GoogleCloudDocumentaiV1DocumentPageImageQualityScoresDetectedDefect>>,
    /// The overall quality score. Range `[0, 1]` where 1 is perfect quality.
    #[serde(rename="qualityScore")]
    
    pub quality_score: Option<f32>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentPageImageQualityScores {}


/// Image Quality Defects
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentPageImageQualityScoresDetectedDefect {
    /// Confidence of detected defect. Range `[0, 1]` where 1 indicates strong confidence of that the defect exists.
    
    pub confidence: Option<f32>,
    /// Name of the defect type. Supported values are: - `quality/defect_blurry` - `quality/defect_noisy` - `quality/defect_dark` - `quality/defect_faint` - `quality/defect_text_too_small` - `quality/defect_document_cutoff` - `quality/defect_text_cutoff` - `quality/defect_glare`
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentPageImageQualityScoresDetectedDefect {}


/// Visual element describing a layout unit on a page.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentPageLayout {
    /// The bounding polygon for the Layout.
    #[serde(rename="boundingPoly")]
    
    pub bounding_poly: Option<GoogleCloudDocumentaiV1BoundingPoly>,
    /// Confidence of the current Layout within context of the object this layout is for. e.g. confidence can be for a single token, a table, a visual element, etc. depending on context. Range `[0, 1]`.
    
    pub confidence: Option<f32>,
    /// Detected orientation for the Layout.
    
    pub orientation: Option<GoogleCloudDocumentaiV1DocumentPageLayoutOrientationEnum>,
    /// Text anchor indexing into the Document.text.
    #[serde(rename="textAnchor")]
    
    pub text_anchor: Option<GoogleCloudDocumentaiV1DocumentTextAnchor>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentPageLayout {}


/// A collection of tokens that a human would perceive as a line. Does not cross column boundaries, can be horizontal, vertical, etc.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentPageLine {
    /// A list of detected languages together with confidence.
    #[serde(rename="detectedLanguages")]
    
    pub detected_languages: Option<Vec<GoogleCloudDocumentaiV1DocumentPageDetectedLanguage>>,
    /// Layout for Line.
    
    pub layout: Option<GoogleCloudDocumentaiV1DocumentPageLayout>,
    /// The history of this annotation.
    
    pub provenance: Option<GoogleCloudDocumentaiV1DocumentProvenance>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentPageLine {}


/// Representation for transformation matrix, intended to be compatible and used with OpenCV format for image manipulation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentPageMatrix {
    /// Number of columns in the matrix.
    
    pub cols: Option<i32>,
    /// The matrix data.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub data: Option<Vec<u8>>,
    /// Number of rows in the matrix.
    
    pub rows: Option<i32>,
    /// This encodes information about what data type the matrix uses. For example, 0 (CV_8U) is an unsigned 8-bit image. For the full list of OpenCV primitive data types, please refer to https://docs.opencv.org/4.3.0/d1/d1b/group__core__hal__interface.html
    #[serde(rename="type")]
    
    pub type_: Option<i32>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentPageMatrix {}


/// A collection of lines that a human would perceive as a paragraph.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentPageParagraph {
    /// A list of detected languages together with confidence.
    #[serde(rename="detectedLanguages")]
    
    pub detected_languages: Option<Vec<GoogleCloudDocumentaiV1DocumentPageDetectedLanguage>>,
    /// Layout for Paragraph.
    
    pub layout: Option<GoogleCloudDocumentaiV1DocumentPageLayout>,
    /// The history of this annotation.
    
    pub provenance: Option<GoogleCloudDocumentaiV1DocumentProvenance>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentPageParagraph {}


/// A detected symbol.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentPageSymbol {
    /// A list of detected languages together with confidence.
    #[serde(rename="detectedLanguages")]
    
    pub detected_languages: Option<Vec<GoogleCloudDocumentaiV1DocumentPageDetectedLanguage>>,
    /// Layout for Symbol.
    
    pub layout: Option<GoogleCloudDocumentaiV1DocumentPageLayout>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentPageSymbol {}


/// A table representation similar to HTML table structure.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentPageTable {
    /// Body rows of the table.
    #[serde(rename="bodyRows")]
    
    pub body_rows: Option<Vec<GoogleCloudDocumentaiV1DocumentPageTableTableRow>>,
    /// A list of detected languages together with confidence.
    #[serde(rename="detectedLanguages")]
    
    pub detected_languages: Option<Vec<GoogleCloudDocumentaiV1DocumentPageDetectedLanguage>>,
    /// Header rows of the table.
    #[serde(rename="headerRows")]
    
    pub header_rows: Option<Vec<GoogleCloudDocumentaiV1DocumentPageTableTableRow>>,
    /// Layout for Table.
    
    pub layout: Option<GoogleCloudDocumentaiV1DocumentPageLayout>,
    /// The history of this table.
    
    pub provenance: Option<GoogleCloudDocumentaiV1DocumentProvenance>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentPageTable {}


/// A cell representation inside the table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentPageTableTableCell {
    /// How many columns this cell spans.
    #[serde(rename="colSpan")]
    
    pub col_span: Option<i32>,
    /// A list of detected languages together with confidence.
    #[serde(rename="detectedLanguages")]
    
    pub detected_languages: Option<Vec<GoogleCloudDocumentaiV1DocumentPageDetectedLanguage>>,
    /// Layout for TableCell.
    
    pub layout: Option<GoogleCloudDocumentaiV1DocumentPageLayout>,
    /// How many rows this cell spans.
    #[serde(rename="rowSpan")]
    
    pub row_span: Option<i32>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentPageTableTableCell {}


/// A row of table cells.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentPageTableTableRow {
    /// Cells that make up this row.
    
    pub cells: Option<Vec<GoogleCloudDocumentaiV1DocumentPageTableTableCell>>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentPageTableTableRow {}


/// A detected token.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentPageToken {
    /// Detected break at the end of a Token.
    #[serde(rename="detectedBreak")]
    
    pub detected_break: Option<GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreak>,
    /// A list of detected languages together with confidence.
    #[serde(rename="detectedLanguages")]
    
    pub detected_languages: Option<Vec<GoogleCloudDocumentaiV1DocumentPageDetectedLanguage>>,
    /// Layout for Token.
    
    pub layout: Option<GoogleCloudDocumentaiV1DocumentPageLayout>,
    /// The history of this annotation.
    
    pub provenance: Option<GoogleCloudDocumentaiV1DocumentProvenance>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentPageToken {}


/// Detected break at the end of a Token.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreak {
    /// Detected break type.
    #[serde(rename="type")]
    
    pub type_: Option<GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreakTypeEnum>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreak {}


/// Detected non-text visual elements e.g. checkbox, signature etc. on the page.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentPageVisualElement {
    /// A list of detected languages together with confidence.
    #[serde(rename="detectedLanguages")]
    
    pub detected_languages: Option<Vec<GoogleCloudDocumentaiV1DocumentPageDetectedLanguage>>,
    /// Layout for VisualElement.
    
    pub layout: Option<GoogleCloudDocumentaiV1DocumentPageLayout>,
    /// Type of the VisualElement.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentPageVisualElement {}


/// Structure to identify provenance relationships between annotations in different revisions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentProvenance {
    /// The Id of this operation. Needs to be unique within the scope of the revision.
    
    pub id: Option<i32>,
    /// References to the original elements that are replaced.
    
    pub parents: Option<Vec<GoogleCloudDocumentaiV1DocumentProvenanceParent>>,
    /// The index of the revision that produced this element.
    
    pub revision: Option<i32>,
    /// The type of provenance operation.
    #[serde(rename="type")]
    
    pub type_: Option<GoogleCloudDocumentaiV1DocumentProvenanceTypeEnum>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentProvenance {}


/// The parent element the current element is based on. Used for referencing/aligning, removal and replacement operations.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentProvenanceParent {
    /// The id of the parent provenance.
    
    pub id: Option<i32>,
    /// The index of the parent item in the corresponding item list (eg. list of entities, properties within entities, etc.) in the parent revision.
    
    pub index: Option<i32>,
    /// The index of the index into current revision's parent_ids list.
    
    pub revision: Option<i32>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentProvenanceParent {}


/// Contains past or forward revisions of this document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentRevision {
    /// If the change was made by a person specify the name or id of that person.
    
    pub agent: Option<String>,
    /// The time that the revision was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Human Review information of this revision.
    #[serde(rename="humanReview")]
    
    pub human_review: Option<GoogleCloudDocumentaiV1DocumentRevisionHumanReview>,
    /// Id of the revision. Unique within the context of the document.
    
    pub id: Option<String>,
    /// The revisions that this revision is based on. This can include one or more parent (when documents are merged.) This field represents the index into the `revisions` field.
    
    pub parent: Option<Vec<i32>>,
    /// The revisions that this revision is based on. Must include all the ids that have anything to do with this revision - eg. there are `provenance.parent.revision` fields that index into this field.
    #[serde(rename="parentIds")]
    
    pub parent_ids: Option<Vec<String>>,
    /// If the annotation was made by processor identify the processor by its resource name.
    
    pub processor: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentRevision {}


/// Human Review information of the document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentRevisionHumanReview {
    /// Human review state. e.g. `requested`, `succeeded`, `rejected`.
    
    pub state: Option<String>,
    /// A message providing more details about the current state of processing. For example, the rejection reason when the state is `rejected`.
    #[serde(rename="stateMessage")]
    
    pub state_message: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentRevisionHumanReview {}


/// The schema defines the output of the processed document by a processor.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentSchema {
    /// Description of the schema.
    
    pub description: Option<String>,
    /// Display name to show to users.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Entity types of the schema.
    #[serde(rename="entityTypes")]
    
    pub entity_types: Option<Vec<GoogleCloudDocumentaiV1DocumentSchemaEntityType>>,
    /// Metadata of the schema.
    
    pub metadata: Option<GoogleCloudDocumentaiV1DocumentSchemaMetadata>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentSchema {}


/// EntityType is the wrapper of a label of the corresponding model with detailed attributes and limitations for entity-based processors. Multiple types can also compose a dependency tree to represent nested types.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentSchemaEntityType {
    /// The entity type that this type is derived from. For now, one and only one should be set.
    #[serde(rename="baseTypes")]
    
    pub base_types: Option<Vec<String>>,
    /// User defined name for the type.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// If specified, lists all the possible values for this entity. This should not be more than a handful of values. If the number of values is >10 or could change frequently use the `EntityType.value_ontology` field and specify a list of all possible values in a value ontology file.
    #[serde(rename="enumValues")]
    
    pub enum_values: Option<GoogleCloudDocumentaiV1DocumentSchemaEntityTypeEnumValues>,
    /// Name of the type. It must be unique within the schema file and cannot be a 'Common Type'. Besides that we use the following naming conventions: - *use `snake_casing`* - name matching is case-sensitive - Maximum 64 characters. - Must start with a letter. - Allowed characters: ASCII letters `[a-z0-9_-]`. (For backward compatibility internal infrastructure and tooling can handle any ascii character) - The `/` is sometimes used to denote a property of a type. For example `line_item/amount`. This convention is deprecated, but will still be honored for backward compatibility.
    
    pub name: Option<String>,
    /// Describing the nested structure, or composition of an entity.
    
    pub properties: Option<Vec<GoogleCloudDocumentaiV1DocumentSchemaEntityTypeProperty>>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentSchemaEntityType {}


/// Defines the a list of enum values.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentSchemaEntityTypeEnumValues {
    /// The individual values that this enum values type can include.
    
    pub values: Option<Vec<String>>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentSchemaEntityTypeEnumValues {}


/// Defines properties that can be part of the entity type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentSchemaEntityTypeProperty {
    /// The name of the property. Follows the same guidelines as the EntityType name.
    
    pub name: Option<String>,
    /// Occurrence type limits the number of instances an entity type appears in the document.
    #[serde(rename="occurrenceType")]
    
    pub occurrence_type: Option<GoogleCloudDocumentaiV1DocumentSchemaEntityTypePropertyOccurrenceTypeEnum>,
    /// A reference to the value type of the property. This type is subject to the same conventions as the `Entity.base_types` field.
    #[serde(rename="valueType")]
    
    pub value_type: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentSchemaEntityTypeProperty {}


/// Metadata for global schema behavior.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentSchemaMetadata {
    /// If true, on a given page, there can be multiple `document` annotations covering it.
    #[serde(rename="documentAllowMultipleLabels")]
    
    pub document_allow_multiple_labels: Option<bool>,
    /// If true, a `document` entity type can be applied to subdocument ( splitting). Otherwise, it can only be applied to the entire document (classification).
    #[serde(rename="documentSplitter")]
    
    pub document_splitter: Option<bool>,
    /// If set, all the nested entities must be prefixed with the parents.
    #[serde(rename="prefixedNamingOnProperties")]
    
    pub prefixed_naming_on_properties: Option<bool>,
    /// If set, we will skip the naming format validation in the schema. So the string values in `DocumentSchema.EntityType.name` and `DocumentSchema.EntityType.Property.name` will not be checked.
    #[serde(rename="skipNamingValidation")]
    
    pub skip_naming_validation: Option<bool>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentSchemaMetadata {}


/// For a large document, sharding may be performed to produce several document shards. Each document shard contains this field to detail which shard it is.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentShardInfo {
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

impl client::Part for GoogleCloudDocumentaiV1DocumentShardInfo {}


/// Annotation for common text style attributes. This adheres to CSS conventions as much as possible.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentStyle {
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
    
    pub font_size: Option<GoogleCloudDocumentaiV1DocumentStyleFontSize>,
    /// Font weight. Possible values are normal, bold, bolder, and lighter. https://www.w3schools.com/cssref/pr_font_weight.asp
    #[serde(rename="fontWeight")]
    
    pub font_weight: Option<String>,
    /// Text anchor indexing into the Document.text.
    #[serde(rename="textAnchor")]
    
    pub text_anchor: Option<GoogleCloudDocumentaiV1DocumentTextAnchor>,
    /// Text decoration. Follows CSS standard. https://www.w3schools.com/cssref/pr_text_text-decoration.asp
    #[serde(rename="textDecoration")]
    
    pub text_decoration: Option<String>,
    /// Text style. Possible values are normal, italic, and oblique. https://www.w3schools.com/cssref/pr_font_font-style.asp
    #[serde(rename="textStyle")]
    
    pub text_style: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentStyle {}


/// Font size with unit.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentStyleFontSize {
    /// Font size for the text.
    
    pub size: Option<f32>,
    /// Unit for the font size. Follows CSS naming (in, px, pt, etc.).
    
    pub unit: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentStyleFontSize {}


/// Text reference indexing into the Document.text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentTextAnchor {
    /// Contains the content of the text span so that users do not have to look it up in the text_segments. It is always populated for formFields.
    
    pub content: Option<String>,
    /// The text segments from the Document.text.
    #[serde(rename="textSegments")]
    
    pub text_segments: Option<Vec<GoogleCloudDocumentaiV1DocumentTextAnchorTextSegment>>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentTextAnchor {}


/// A text segment in the Document.text. The indices may be out of bounds which indicate that the text extends into another document shard for large sharded documents. See ShardInfo.text_offset
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentTextAnchorTextSegment {
    /// TextSegment half open end UTF-8 char index in the Document.text.
    #[serde(rename="endIndex")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub end_index: Option<i64>,
    /// TextSegment start UTF-8 char index in the Document.text.
    #[serde(rename="startIndex")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub start_index: Option<i64>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentTextAnchorTextSegment {}


/// This message is used for text changes aka. OCR corrections.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1DocumentTextChange {
    /// The text that replaces the text identified in the `text_anchor`.
    #[serde(rename="changedText")]
    
    pub changed_text: Option<String>,
    /// The history of this annotation.
    
    pub provenance: Option<Vec<GoogleCloudDocumentaiV1DocumentProvenance>>,
    /// Provenance of the correction. Text anchor indexing into the Document.text. There can only be a single `TextAnchor.text_segments` element. If the start and end index of the text segment are the same, the text change is inserted before that index.
    #[serde(rename="textAnchor")]
    
    pub text_anchor: Option<GoogleCloudDocumentaiV1DocumentTextAnchor>,
}

impl client::Part for GoogleCloudDocumentaiV1DocumentTextChange {}


/// Request message for the enable processor method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations processors enable projects](ProjectLocationProcessorEnableCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1EnableProcessorRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleCloudDocumentaiV1EnableProcessorRequest {}


/// Evaluates the given ProcessorVersion against the supplied documents.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations processors processor versions evaluate processor version projects](ProjectLocationProcessorProcessorVersionEvaluateProcessorVersionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1EvaluateProcessorVersionRequest {
    /// Optional. The documents used in the evaluation. If unspecified, use the processor's dataset as evaluation input.
    #[serde(rename="evaluationDocuments")]
    
    pub evaluation_documents: Option<GoogleCloudDocumentaiV1BatchDocumentsInputConfig>,
}

impl client::RequestValue for GoogleCloudDocumentaiV1EvaluateProcessorVersionRequest {}


/// An evaluation of a ProcessorVersion’s performance.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations processors processor versions evaluations get projects](ProjectLocationProcessorProcessorVersionEvaluationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1Evaluation {
    /// Metrics for all the entities in aggregate.
    #[serde(rename="allEntitiesMetrics")]
    
    pub all_entities_metrics: Option<GoogleCloudDocumentaiV1EvaluationMultiConfidenceMetrics>,
    /// The time that the evaluation was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Counters for the documents used in the evaluation.
    #[serde(rename="documentCounters")]
    
    pub document_counters: Option<GoogleCloudDocumentaiV1EvaluationCounters>,
    /// Metrics across confidence levels, for different entities.
    #[serde(rename="entityMetrics")]
    
    pub entity_metrics: Option<HashMap<String, GoogleCloudDocumentaiV1EvaluationMultiConfidenceMetrics>>,
    /// The KMS key name used for encryption.
    #[serde(rename="kmsKeyName")]
    
    pub kms_key_name: Option<String>,
    /// The KMS key version with which data is encrypted.
    #[serde(rename="kmsKeyVersionName")]
    
    pub kms_key_version_name: Option<String>,
    /// The resource name of the evaluation. Format: `projects/{project}/locations/{location}/processors/{processor}/processorVersions/{processor_version}/evaluations/{evaluation}`
    
    pub name: Option<String>,
}

impl client::ResponseResult for GoogleCloudDocumentaiV1Evaluation {}


/// Evaluations metrics, at a specific confidence level.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1EvaluationConfidenceLevelMetrics {
    /// The confidence level.
    #[serde(rename="confidenceLevel")]
    
    pub confidence_level: Option<f32>,
    /// The metrics at the specific confidence level.
    
    pub metrics: Option<GoogleCloudDocumentaiV1EvaluationMetrics>,
}

impl client::Part for GoogleCloudDocumentaiV1EvaluationConfidenceLevelMetrics {}


/// Evaluation counters for the documents that were used.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1EvaluationCounters {
    /// How many documents were used in the evaluation.
    #[serde(rename="evaluatedDocumentsCount")]
    
    pub evaluated_documents_count: Option<i32>,
    /// How many documents were not included in the evaluation as Document AI failed to process them.
    #[serde(rename="failedDocumentsCount")]
    
    pub failed_documents_count: Option<i32>,
    /// How many documents were sent for evaluation.
    #[serde(rename="inputDocumentsCount")]
    
    pub input_documents_count: Option<i32>,
    /// How many documents were not included in the evaluation as they didn't pass validation.
    #[serde(rename="invalidDocumentsCount")]
    
    pub invalid_documents_count: Option<i32>,
}

impl client::Part for GoogleCloudDocumentaiV1EvaluationCounters {}


/// Evaluation metrics, either in aggregate or about a specific entity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1EvaluationMetrics {
    /// The calculated f1 score.
    #[serde(rename="f1Score")]
    
    pub f1_score: Option<f32>,
    /// The amount of false negatives.
    #[serde(rename="falseNegativesCount")]
    
    pub false_negatives_count: Option<i32>,
    /// The amount of false positives.
    #[serde(rename="falsePositivesCount")]
    
    pub false_positives_count: Option<i32>,
    /// The amount of documents with a ground truth occurrence.
    #[serde(rename="groundTruthDocumentCount")]
    
    pub ground_truth_document_count: Option<i32>,
    /// The amount of occurrences in ground truth documents.
    #[serde(rename="groundTruthOccurrencesCount")]
    
    pub ground_truth_occurrences_count: Option<i32>,
    /// The calculated precision.
    
    pub precision: Option<f32>,
    /// The amount of documents with a predicted occurrence.
    #[serde(rename="predictedDocumentCount")]
    
    pub predicted_document_count: Option<i32>,
    /// The amount of occurrences in predicted documents.
    #[serde(rename="predictedOccurrencesCount")]
    
    pub predicted_occurrences_count: Option<i32>,
    /// The calculated recall.
    
    pub recall: Option<f32>,
    /// The amount of documents that had an occurrence of this label.
    #[serde(rename="totalDocumentsCount")]
    
    pub total_documents_count: Option<i32>,
    /// The amount of true positives.
    #[serde(rename="truePositivesCount")]
    
    pub true_positives_count: Option<i32>,
}

impl client::Part for GoogleCloudDocumentaiV1EvaluationMetrics {}


/// Metrics across multiple confidence levels.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1EvaluationMultiConfidenceMetrics {
    /// The calculated area under the precision recall curve (AUPRC), computed by integrating over all confidence thresholds.
    
    pub auprc: Option<f32>,
    /// The AUPRC for metrics with fuzzy matching disabled, i.e., exact matching only.
    #[serde(rename="auprcExact")]
    
    pub auprc_exact: Option<f32>,
    /// Metrics across confidence levels with fuzzy matching enabled.
    #[serde(rename="confidenceLevelMetrics")]
    
    pub confidence_level_metrics: Option<Vec<GoogleCloudDocumentaiV1EvaluationConfidenceLevelMetrics>>,
    /// Metrics across confidence levels with only exact matching.
    #[serde(rename="confidenceLevelMetricsExact")]
    
    pub confidence_level_metrics_exact: Option<Vec<GoogleCloudDocumentaiV1EvaluationConfidenceLevelMetrics>>,
    /// The Estimated Calibration Error (ECE) of the confidence of the predicted entities.
    #[serde(rename="estimatedCalibrationError")]
    
    pub estimated_calibration_error: Option<f32>,
    /// The ECE for the predicted entities with fuzzy matching disabled, i.e., exact matching only.
    #[serde(rename="estimatedCalibrationErrorExact")]
    
    pub estimated_calibration_error_exact: Option<f32>,
    /// The metrics type for the label.
    #[serde(rename="metricsType")]
    
    pub metrics_type: Option<GoogleCloudDocumentaiV1EvaluationMultiConfidenceMetricMetricsTypeEnum>,
}

impl client::Part for GoogleCloudDocumentaiV1EvaluationMultiConfidenceMetrics {}


/// Gives a short summary of an evaluation, and links to the evaluation itself.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1EvaluationReference {
    /// An aggregate of the statistics for the evaluation with fuzzy matching on.
    #[serde(rename="aggregateMetrics")]
    
    pub aggregate_metrics: Option<GoogleCloudDocumentaiV1EvaluationMetrics>,
    /// An aggregate of the statistics for the evaluation with fuzzy matching off.
    #[serde(rename="aggregateMetricsExact")]
    
    pub aggregate_metrics_exact: Option<GoogleCloudDocumentaiV1EvaluationMetrics>,
    /// The resource name of the evaluation.
    
    pub evaluation: Option<String>,
    /// The resource name of the Long Running Operation for the evaluation.
    
    pub operation: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1EvaluationReference {}


/// Response message for fetch processor types.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations fetch processor types projects](ProjectLocationFetchProcessorTypeCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1FetchProcessorTypesResponse {
    /// The list of processor types.
    #[serde(rename="processorTypes")]
    
    pub processor_types: Option<Vec<GoogleCloudDocumentaiV1ProcessorType>>,
}

impl client::ResponseResult for GoogleCloudDocumentaiV1FetchProcessorTypesResponse {}


/// Specifies a document stored on Cloud Storage.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1GcsDocument {
    /// The Cloud Storage object uri.
    #[serde(rename="gcsUri")]
    
    pub gcs_uri: Option<String>,
    /// An IANA MIME type (RFC6838) of the content.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1GcsDocument {}


/// Specifies a set of documents on Cloud Storage.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1GcsDocuments {
    /// The list of documents.
    
    pub documents: Option<Vec<GoogleCloudDocumentaiV1GcsDocument>>,
}

impl client::Part for GoogleCloudDocumentaiV1GcsDocuments {}


/// Specifies all documents on Cloud Storage with a common prefix.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1GcsPrefix {
    /// The URI prefix.
    #[serde(rename="gcsUriPrefix")]
    
    pub gcs_uri_prefix: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1GcsPrefix {}


/// The status of human review on a processed document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1HumanReviewStatus {
    /// The name of the operation triggered by the processed document. This field is populated only when the [state] is [HUMAN_REVIEW_IN_PROGRESS]. It has the same response type and metadata as the long running operation returned by [ReviewDocument] method.
    #[serde(rename="humanReviewOperation")]
    
    pub human_review_operation: Option<String>,
    /// The state of human review on the processing request.
    
    pub state: Option<GoogleCloudDocumentaiV1HumanReviewStatuStateEnum>,
    /// A message providing more details about the human review state.
    #[serde(rename="stateMessage")]
    
    pub state_message: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1HumanReviewStatus {}


/// The response from ListEvaluations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations processors processor versions evaluations list projects](ProjectLocationProcessorProcessorVersionEvaluationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1ListEvaluationsResponse {
    /// The evaluations requested.
    
    pub evaluations: Option<Vec<GoogleCloudDocumentaiV1Evaluation>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDocumentaiV1ListEvaluationsResponse {}


/// Response message for list processor types.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations processor types list projects](ProjectLocationProcessorTypeListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1ListProcessorTypesResponse {
    /// Points to the next page, otherwise empty.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The processor types.
    #[serde(rename="processorTypes")]
    
    pub processor_types: Option<Vec<GoogleCloudDocumentaiV1ProcessorType>>,
}

impl client::ResponseResult for GoogleCloudDocumentaiV1ListProcessorTypesResponse {}


/// Response message for list processors.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations processors processor versions list projects](ProjectLocationProcessorProcessorVersionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1ListProcessorVersionsResponse {
    /// Points to the next processor, otherwise empty.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of processors.
    #[serde(rename="processorVersions")]
    
    pub processor_versions: Option<Vec<GoogleCloudDocumentaiV1ProcessorVersion>>,
}

impl client::ResponseResult for GoogleCloudDocumentaiV1ListProcessorVersionsResponse {}


/// Response message for list processors.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations processors list projects](ProjectLocationProcessorListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1ListProcessorsResponse {
    /// Points to the next processor, otherwise empty.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of processors.
    
    pub processors: Option<Vec<GoogleCloudDocumentaiV1Processor>>,
}

impl client::ResponseResult for GoogleCloudDocumentaiV1ListProcessorsResponse {}


/// A vertex represents a 2D point in the image. NOTE: the normalized vertex coordinates are relative to the original image and range from 0 to 1.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1NormalizedVertex {
    /// X coordinate.
    
    pub x: Option<f32>,
    /// Y coordinate (starts from the top of the image).
    
    pub y: Option<f32>,
}

impl client::Part for GoogleCloudDocumentaiV1NormalizedVertex {}


/// Request message for the process document method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations processors processor versions process projects](ProjectLocationProcessorProcessorVersionProcesCall) (request)
/// * [locations processors process projects](ProjectLocationProcessorProcesCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1ProcessRequest {
    /// Specifies which fields to include in ProcessResponse's document. Only supports top level document and pages field so it must be in the form of `{document_field_name}` or `pages.{page_field_name}`.
    #[serde(rename="fieldMask")]
    
    pub field_mask: Option<client::FieldMask>,
    /// An inline document proto.
    #[serde(rename="inlineDocument")]
    
    pub inline_document: Option<GoogleCloudDocumentaiV1Document>,
    /// A raw document content (bytes).
    #[serde(rename="rawDocument")]
    
    pub raw_document: Option<GoogleCloudDocumentaiV1RawDocument>,
    /// Whether Human Review feature should be skipped for this request. Default to false.
    #[serde(rename="skipHumanReview")]
    
    pub skip_human_review: Option<bool>,
}

impl client::RequestValue for GoogleCloudDocumentaiV1ProcessRequest {}


/// Response message for the process document method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations processors processor versions process projects](ProjectLocationProcessorProcessorVersionProcesCall) (response)
/// * [locations processors process projects](ProjectLocationProcessorProcesCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1ProcessResponse {
    /// The document payload, will populate fields based on the processor's behavior.
    
    pub document: Option<GoogleCloudDocumentaiV1Document>,
    /// The status of human review on the processed document.
    #[serde(rename="humanReviewStatus")]
    
    pub human_review_status: Option<GoogleCloudDocumentaiV1HumanReviewStatus>,
}

impl client::ResponseResult for GoogleCloudDocumentaiV1ProcessResponse {}


/// The first-class citizen for Document AI. Each processor defines how to extract structural information from a document.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations processors create projects](ProjectLocationProcessorCreateCall) (request|response)
/// * [locations processors get projects](ProjectLocationProcessorGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1Processor {
    /// The time the processor was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The default processor version.
    #[serde(rename="defaultProcessorVersion")]
    
    pub default_processor_version: Option<String>,
    /// The display name of the processor.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The KMS key used for encryption/decryption in CMEK scenarios. See https://cloud.google.com/security-key-management.
    #[serde(rename="kmsKeyName")]
    
    pub kms_key_name: Option<String>,
    /// Output only. Immutable. The resource name of the processor. Format: `projects/{project}/locations/{location}/processors/{processor}`
    
    pub name: Option<String>,
    /// Output only. Immutable. The http endpoint that can be called to invoke processing.
    #[serde(rename="processEndpoint")]
    
    pub process_endpoint: Option<String>,
    /// Output only. The state of the processor.
    
    pub state: Option<GoogleCloudDocumentaiV1ProcessorStateEnum>,
    /// The processor type, e.g., `OCR_PROCESSOR`, `INVOICE_PROCESSOR`, etc. To get a list of processors types, see FetchProcessorTypes.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::RequestValue for GoogleCloudDocumentaiV1Processor {}
impl client::ResponseResult for GoogleCloudDocumentaiV1Processor {}


/// A processor type is responsible for performing a certain document understanding task on a certain type of document.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations processor types get projects](ProjectLocationProcessorTypeGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1ProcessorType {
    /// Whether the processor type allows creation. If true, users can create a processor of this processor type. Otherwise, users need to request access.
    #[serde(rename="allowCreation")]
    
    pub allow_creation: Option<bool>,
    /// The locations in which this processor is available.
    #[serde(rename="availableLocations")]
    
    pub available_locations: Option<Vec<GoogleCloudDocumentaiV1ProcessorTypeLocationInfo>>,
    /// The processor category, used by UI to group processor types.
    
    pub category: Option<String>,
    /// Launch stage of the processor type
    #[serde(rename="launchStage")]
    
    pub launch_stage: Option<GoogleCloudDocumentaiV1ProcessorTypeLaunchStageEnum>,
    /// The resource name of the processor type. Format: `projects/{project}/processorTypes/{processor_type}`
    
    pub name: Option<String>,
    /// A set of Cloud Storage URIs of sample documents for this processor.
    #[serde(rename="sampleDocumentUris")]
    
    pub sample_document_uris: Option<Vec<String>>,
    /// The processor type, e.g., `OCR_PROCESSOR`, `INVOICE_PROCESSOR`, etc.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::ResponseResult for GoogleCloudDocumentaiV1ProcessorType {}


/// The location information about where the processor is available.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1ProcessorTypeLocationInfo {
    /// The location id, currently must be one of [us, eu].
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1ProcessorTypeLocationInfo {}


/// A processor version is an implementation of a processor. Each processor can have multiple versions, pre-trained by Google internally or up-trained by the customer. At a time, a processor can only have one default version version. So the processor’s behavior (when processing documents) is defined by a default version
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations processors processor versions get projects](ProjectLocationProcessorProcessorVersionGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1ProcessorVersion {
    /// The time the processor version was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// If set, information about the eventual deprecation of this version.
    #[serde(rename="deprecationInfo")]
    
    pub deprecation_info: Option<GoogleCloudDocumentaiV1ProcessorVersionDeprecationInfo>,
    /// The display name of the processor version.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The schema of the processor version. Describes the output.
    #[serde(rename="documentSchema")]
    
    pub document_schema: Option<GoogleCloudDocumentaiV1DocumentSchema>,
    /// Denotes that this ProcessorVersion is managed by google.
    #[serde(rename="googleManaged")]
    
    pub google_managed: Option<bool>,
    /// The KMS key name used for encryption.
    #[serde(rename="kmsKeyName")]
    
    pub kms_key_name: Option<String>,
    /// The KMS key version with which data is encrypted.
    #[serde(rename="kmsKeyVersionName")]
    
    pub kms_key_version_name: Option<String>,
    /// The most recently invoked evaluation for the processor version.
    #[serde(rename="latestEvaluation")]
    
    pub latest_evaluation: Option<GoogleCloudDocumentaiV1EvaluationReference>,
    /// The resource name of the processor version. Format: `projects/{project}/locations/{location}/processors/{processor}/processorVersions/{processor_version}`
    
    pub name: Option<String>,
    /// The state of the processor version.
    
    pub state: Option<GoogleCloudDocumentaiV1ProcessorVersionStateEnum>,
}

impl client::ResponseResult for GoogleCloudDocumentaiV1ProcessorVersion {}


/// Information about the upcoming deprecation of this processor version.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1ProcessorVersionDeprecationInfo {
    /// The time at which this processor version will be deprecated.
    #[serde(rename="deprecationTime")]
    
    pub deprecation_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// If set, the processor version that will be used as a replacement.
    #[serde(rename="replacementProcessorVersion")]
    
    pub replacement_processor_version: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1ProcessorVersionDeprecationInfo {}


/// Payload message of raw document content (bytes).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1RawDocument {
    /// Inline document content.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub content: Option<Vec<u8>>,
    /// An IANA MIME type (RFC6838) indicating the nature and format of the content.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
}

impl client::Part for GoogleCloudDocumentaiV1RawDocument {}


/// Request message for review document method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations processors human review config review document projects](ProjectLocationProcessorHumanReviewConfigReviewDocumentCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1ReviewDocumentRequest {
    /// The document schema of the human review task.
    #[serde(rename="documentSchema")]
    
    pub document_schema: Option<GoogleCloudDocumentaiV1DocumentSchema>,
    /// Whether the validation should be performed on the ad-hoc review request.
    #[serde(rename="enableSchemaValidation")]
    
    pub enable_schema_validation: Option<bool>,
    /// An inline document proto.
    #[serde(rename="inlineDocument")]
    
    pub inline_document: Option<GoogleCloudDocumentaiV1Document>,
    /// The priority of the human review task.
    
    pub priority: Option<GoogleCloudDocumentaiV1ReviewDocumentRequestPriorityEnum>,
}

impl client::RequestValue for GoogleCloudDocumentaiV1ReviewDocumentRequest {}


/// Request message for the set default processor version method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations processors set default processor version projects](ProjectLocationProcessorSetDefaultProcessorVersionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1SetDefaultProcessorVersionRequest {
    /// Required. The resource name of child ProcessorVersion to use as default. Format: `projects/{project}/locations/{location}/processors/{processor}/processorVersions/{version}`
    #[serde(rename="defaultProcessorVersion")]
    
    pub default_processor_version: Option<String>,
}

impl client::RequestValue for GoogleCloudDocumentaiV1SetDefaultProcessorVersionRequest {}


/// Request message for the create processor version method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations processors processor versions train projects](ProjectLocationProcessorProcessorVersionTrainCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1TrainProcessorVersionRequest {
    /// Optional. The processor version to use as a base for training. This processor version must be a child of `parent`. Format: `projects/{project}/locations/{location}/processors/{processor}/processorVersions/{processorVersion}`.
    #[serde(rename="baseProcessorVersion")]
    
    pub base_processor_version: Option<String>,
    /// Optional. The schema the processor version will be trained with.
    #[serde(rename="documentSchema")]
    
    pub document_schema: Option<GoogleCloudDocumentaiV1DocumentSchema>,
    /// Optional. The input data used to train the `ProcessorVersion`.
    #[serde(rename="inputData")]
    
    pub input_data: Option<GoogleCloudDocumentaiV1TrainProcessorVersionRequestInputData>,
    /// Required. The processor version to be created.
    #[serde(rename="processorVersion")]
    
    pub processor_version: Option<GoogleCloudDocumentaiV1ProcessorVersion>,
}

impl client::RequestValue for GoogleCloudDocumentaiV1TrainProcessorVersionRequest {}


/// The input data used to train a new `ProcessorVersion`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1TrainProcessorVersionRequestInputData {
    /// The documents used for testing the trained version.
    #[serde(rename="testDocuments")]
    
    pub test_documents: Option<GoogleCloudDocumentaiV1BatchDocumentsInputConfig>,
    /// The documents used for training the new version.
    #[serde(rename="trainingDocuments")]
    
    pub training_documents: Option<GoogleCloudDocumentaiV1BatchDocumentsInputConfig>,
}

impl client::Part for GoogleCloudDocumentaiV1TrainProcessorVersionRequestInputData {}


/// Request message for the undeploy processor version method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations processors processor versions undeploy projects](ProjectLocationProcessorProcessorVersionUndeployCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1UndeployProcessorVersionRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleCloudDocumentaiV1UndeployProcessorVersionRequest {}


/// A vertex represents a 2D point in the image. NOTE: the vertex coordinates are in the same scale as the original image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1Vertex {
    /// X coordinate.
    
    pub x: Option<i32>,
    /// Y coordinate (starts from the top of the image).
    
    pub y: Option<i32>,
}

impl client::Part for GoogleCloudDocumentaiV1Vertex {}


/// The response message for Locations.ListLocations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations list projects](ProjectLocationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudLocationListLocationsResponse {
    /// A list of locations that matches the specified filter in the request.
    
    pub locations: Option<Vec<GoogleCloudLocationLocation>>,
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudLocationListLocationsResponse {}


/// A resource that represents Google Cloud Platform location.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations get projects](ProjectLocationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudLocationLocation {
    /// The friendly name for this location, typically a nearby city name. For example, "Tokyo".
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"}
    
    pub labels: Option<HashMap<String, String>>,
    /// The canonical id for this location. For example: `"us-east1"`.
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
    /// Service-specific metadata. For example the available capacity at the given location.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"`
    
    pub name: Option<String>,
}

impl client::ResponseResult for GoogleCloudLocationLocation {}


/// The response message for Operations.ListOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations list projects](ProjectLocationOperationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleLongrunningListOperationsResponse {
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of operations that matches the specified filter in the request.
    
    pub operations: Option<Vec<GoogleLongrunningOperation>>,
}

impl client::ResponseResult for GoogleLongrunningListOperationsResponse {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations get projects](ProjectLocationOperationGetCall) (response)
/// * [locations processors human review config review document projects](ProjectLocationProcessorHumanReviewConfigReviewDocumentCall) (response)
/// * [locations processors processor versions batch process projects](ProjectLocationProcessorProcessorVersionBatchProcesCall) (response)
/// * [locations processors processor versions delete projects](ProjectLocationProcessorProcessorVersionDeleteCall) (response)
/// * [locations processors processor versions deploy projects](ProjectLocationProcessorProcessorVersionDeployCall) (response)
/// * [locations processors processor versions evaluate processor version projects](ProjectLocationProcessorProcessorVersionEvaluateProcessorVersionCall) (response)
/// * [locations processors processor versions train projects](ProjectLocationProcessorProcessorVersionTrainCall) (response)
/// * [locations processors processor versions undeploy projects](ProjectLocationProcessorProcessorVersionUndeployCall) (response)
/// * [locations processors batch process projects](ProjectLocationProcessorBatchProcesCall) (response)
/// * [locations processors delete projects](ProjectLocationProcessorDeleteCall) (response)
/// * [locations processors disable projects](ProjectLocationProcessorDisableCall) (response)
/// * [locations processors enable projects](ProjectLocationProcessorEnableCall) (response)
/// * [locations processors set default processor version projects](ProjectLocationProcessorSetDefaultProcessorVersionCall) (response)
/// * [operations get projects](ProjectOperationGetCall) (response)
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
    /// The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`.
    
    pub response: Option<HashMap<String, json::Value>>,
}

impl client::ResponseResult for GoogleLongrunningOperation {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete operations](OperationDeleteCall) (response)
/// * [locations operations cancel projects](ProjectLocationOperationCancelCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleProtobufEmpty { _never_set: Option<bool> }

impl client::ResponseResult for GoogleProtobufEmpty {}


/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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


/// Represents a color in the RGBA color space. This representation is designed for simplicity of conversion to/from color representations in various languages over compactness. For example, the fields of this representation can be trivially provided to the constructor of `java.awt.Color` in Java; it can also be trivially provided to UIColor's `+colorWithRed:green:blue:alpha` method in iOS; and, with just a little work, it can be easily formatted into a CSS `rgba()` string in JavaScript. This reference page doesn't carry information about the absolute color space that should be used to interpret the RGB value (e.g. sRGB, Adobe RGB, DCI-P3, BT.2020, etc.). By default, applications should assume the sRGB color space. When color equality needs to be decided, implementations, unless documented otherwise, treat two colors as equal if all their red, green, blue, and alpha values each differ by at most 1e-5. Example (Java): import com.google.type.Color; // ... public static java.awt.Color fromProto(Color protocolor) { float alpha = protocolor.hasAlpha() ? protocolor.getAlpha().getValue() : 1.0; return new java.awt.Color( protocolor.getRed(), protocolor.getGreen(), protocolor.getBlue(), alpha); } public static Color toProto(java.awt.Color color) { float red = (float) color.getRed(); float green = (float) color.getGreen(); float blue = (float) color.getBlue(); float denominator = 255.0; Color.Builder resultBuilder = Color .newBuilder() .setRed(red / denominator) .setGreen(green / denominator) .setBlue(blue / denominator); int alpha = color.getAlpha(); if (alpha != 255) { result.setAlpha( FloatValue .newBuilder() .setValue(((float) alpha) / denominator) .build()); } return resultBuilder.build(); } // ... Example (iOS / Obj-C): // ... static UIColor* fromProto(Color* protocolor) { float red = [protocolor red]; float green = [protocolor green]; float blue = [protocolor blue]; FloatValue* alpha_wrapper = [protocolor alpha]; float alpha = 1.0; if (alpha_wrapper != nil) { alpha = [alpha_wrapper value]; } return [UIColor colorWithRed:red green:green blue:blue alpha:alpha]; } static Color* toProto(UIColor* color) { CGFloat red, green, blue, alpha; if (![color getRed:&red green:&green blue:&blue alpha:&alpha]) { return nil; } Color* result = [[Color alloc] init]; [result setRed:red]; [result setGreen:green]; [result setBlue:blue]; if (alpha <= 0.9999) { [result setAlpha:floatWrapperWithValue(alpha)]; } [result autorelease]; return result; } // ... Example (JavaScript): // ... var protoToCssColor = function(rgb_color) { var redFrac = rgb_color.red || 0.0; var greenFrac = rgb_color.green || 0.0; var blueFrac = rgb_color.blue || 0.0; var red = Math.floor(redFrac * 255); var green = Math.floor(greenFrac * 255); var blue = Math.floor(blueFrac * 255); if (!('alpha' in rgb_color)) { return rgbToCssColor(red, green, blue); } var alphaFrac = rgb_color.alpha.value || 0.0; var rgbParams = [red, green, blue].join(','); return ['rgba(', rgbParams, ',', alphaFrac, ')'].join(''); }; var rgbToCssColor = function(red, green, blue) { var rgbNumber = new Number((red << 16) | (green << 8) | blue); var hexString = rgbNumber.toString(16); var missingZeros = 6 - hexString.length; var resultBuilder = ['#']; for (var i = 0; i < missingZeros; i++) { resultBuilder.push('0'); } resultBuilder.push(hexString); return resultBuilder.join(''); }; // ...
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeTimeZone {
    /// IANA Time Zone Database time zone, e.g. "America/New_York".
    
    pub id: Option<String>,
    /// Optional. IANA Time Zone Database version number, e.g. "2019a".
    
    pub version: Option<String>,
}

impl client::Part for GoogleTypeTimeZone {}


