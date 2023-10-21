use super::*;
/// Input configuration for BatchTranslateDocument request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchDocumentInputConfig {
    /// Google Cloud Storage location for the source input. This can be a single file (for example, `gs://translation-test/input.docx`) or a wildcard (for example, `gs://translation-test/*`). File mime type is determined based on extension. Supported mime type includes: - `pdf`, application/pdf - `docx`, application/vnd.openxmlformats-officedocument.wordprocessingml.document - `pptx`, application/vnd.openxmlformats-officedocument.presentationml.presentation - `xlsx`, application/vnd.openxmlformats-officedocument.spreadsheetml.sheet The max file size to support for `.docx`, `.pptx` and `.xlsx` is 100MB. The max file size to support for `.pdf` is 1GB and the max page limit is 1000 pages. The max file size to support for all input documents is 1GB.
    #[serde(rename="gcsSource")]
    
    pub gcs_source: Option<GcsSource>,
}

impl client::Part for BatchDocumentInputConfig {}


/// Output configuration for BatchTranslateDocument request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchDocumentOutputConfig {
    /// Google Cloud Storage destination for output content. For every single input document (for example, gs://a/b/c.[extension]), we generate at most 2 * n output files. (n is the # of target_language_codes in the BatchTranslateDocumentRequest). While the input documents are being processed, we write/update an index file `index.csv` under `gcs_destination.output_uri_prefix` (for example, gs://translation_output/index.csv) The index file is generated/updated as new files are being translated. The format is: input_document,target_language_code,translation_output,error_output, glossary_translation_output,glossary_error_output `input_document` is one file we matched using gcs_source.input_uri. `target_language_code` is provided in the request. `translation_output` contains the translations. (details provided below) `error_output` contains the error message during processing of the file. Both translations_file and errors_file could be empty strings if we have no content to output. `glossary_translation_output` and `glossary_error_output` are the translated output/error when we apply glossaries. They could also be empty if we have no content to output. Once a row is present in index.csv, the input/output matching never changes. Callers should also expect all the content in input_file are processed and ready to be consumed (that is, no partial output file is written). Since index.csv will be keeping updated during the process, please make sure there is no custom retention policy applied on the output bucket that may avoid file updating. (https://cloud.google.com/storage/docs/bucket-lock#retention-policy) The naming format of translation output files follows (for target language code [trg]): `translation_output`: gs://translation_output/a_b_c_[trg]_translation.[extension] `glossary_translation_output`: gs://translation_test/a_b_c_[trg]_glossary_translation.[extension] The output document will maintain the same file format as the input document. The naming format of error output files follows (for target language code [trg]): `error_output`: gs://translation_test/a_b_c_[trg]_errors.txt `glossary_error_output`: gs://translation_test/a_b_c_[trg]_glossary_translation.txt The error output is a txt file containing error details.
    #[serde(rename="gcsDestination")]
    
    pub gcs_destination: Option<GcsDestination>,
}

impl client::Part for BatchDocumentOutputConfig {}


/// The BatchTranslateDocument request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations batch translate document projects](ProjectLocationBatchTranslateDocumentCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchTranslateDocumentRequest {
    /// Optional. This flag is to support user customized attribution. If not provided, the default is `Machine Translated by Google`. Customized attribution should follow rules in https://cloud.google.com/translate/attribution#attribution_and_logos
    #[serde(rename="customizedAttribution")]
    
    pub customized_attribution: Option<String>,
    /// Optional.
    #[serde(rename="formatConversions")]
    
    pub format_conversions: Option<HashMap<String, String>>,
    /// Optional. Glossaries to be applied. It's keyed by target language code.
    
    pub glossaries: Option<HashMap<String, TranslateTextGlossaryConfig>>,
    /// Required. Input configurations. The total number of files matched should be <= 100. The total content size to translate should be <= 100M Unicode codepoints. The files must use UTF-8 encoding.
    #[serde(rename="inputConfigs")]
    
    pub input_configs: Option<Vec<BatchDocumentInputConfig>>,
    /// Optional. The models to use for translation. Map's key is target language code. Map's value is the model name. Value can be a built-in general model, or an AutoML Translation model. The value format depends on model type: - AutoML Translation models: `projects/{project-number-or-id}/locations/{location-id}/models/{model-id}` - General (built-in) models: `projects/{project-number-or-id}/locations/{location-id}/models/general/nmt`, If the map is empty or a specific model is not requested for a language pair, then default google model (nmt) is used.
    
    pub models: Option<HashMap<String, String>>,
    /// Required. Output configuration. If 2 input configs match to the same file (that is, same input path), we don't generate output for duplicate inputs.
    #[serde(rename="outputConfig")]
    
    pub output_config: Option<BatchDocumentOutputConfig>,
    /// Required. The BCP-47 language code of the input document if known, for example, "en-US" or "sr-Latn". Supported language codes are listed in [Language Support](https://cloud.google.com/translate/docs/languages).
    #[serde(rename="sourceLanguageCode")]
    
    pub source_language_code: Option<String>,
    /// Required. The BCP-47 language code to use for translation of the input document. Specify up to 10 language codes here.
    #[serde(rename="targetLanguageCodes")]
    
    pub target_language_codes: Option<Vec<String>>,
}

impl client::RequestValue for BatchTranslateDocumentRequest {}


/// The batch translation request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations batch translate text projects](ProjectLocationBatchTranslateTextCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchTranslateTextRequest {
    /// Optional. Glossaries to be applied for translation. It's keyed by target language code.
    
    pub glossaries: Option<HashMap<String, TranslateTextGlossaryConfig>>,
    /// Required. Input configurations. The total number of files matched should be <= 100. The total content size should be <= 100M Unicode codepoints. The files must use UTF-8 encoding.
    #[serde(rename="inputConfigs")]
    
    pub input_configs: Option<Vec<InputConfig>>,
    /// Optional. The labels with user-defined metadata for the request. Label keys and values can be no longer than 63 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter. See https://cloud.google.com/translate/docs/advanced/labels for more information.
    
    pub labels: Option<HashMap<String, String>>,
    /// Optional. The models to use for translation. Map's key is target language code. Map's value is model name. Value can be a built-in general model, or an AutoML Translation model. The value format depends on model type: - AutoML Translation models: `projects/{project-number-or-id}/locations/{location-id}/models/{model-id}` - General (built-in) models: `projects/{project-number-or-id}/locations/{location-id}/models/general/nmt`, If the map is empty or a specific model is not requested for a language pair, then default google model (nmt) is used.
    
    pub models: Option<HashMap<String, String>>,
    /// Required. Output configuration. If 2 input configs match to the same file (that is, same input path), we don't generate output for duplicate inputs.
    #[serde(rename="outputConfig")]
    
    pub output_config: Option<OutputConfig>,
    /// Required. Source language code.
    #[serde(rename="sourceLanguageCode")]
    
    pub source_language_code: Option<String>,
    /// Required. Specify up to 10 language codes here.
    #[serde(rename="targetLanguageCodes")]
    
    pub target_language_codes: Option<Vec<String>>,
}

impl client::RequestValue for BatchTranslateTextRequest {}


/// The request message for Operations.CancelOperation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations cancel projects](ProjectLocationOperationCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelOperationRequest { _never_set: Option<bool> }

impl client::RequestValue for CancelOperationRequest {}


/// A dataset that hosts the examples (sentence pairs) used for translation models.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets create projects](ProjectLocationDatasetCreateCall) (request)
/// * [locations datasets get projects](ProjectLocationDatasetGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Dataset {
    /// Output only. Timestamp when this dataset was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The name of the dataset to show in the interface. The name can be up to 32 characters long and can consist only of ASCII Latin letters A-Z and a-z, underscores (_), and ASCII digits 0-9.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. The number of examples in the dataset.
    #[serde(rename="exampleCount")]
    
    pub example_count: Option<i32>,
    /// The resource name of the dataset, in form of `projects/{project-number-or-id}/locations/{location_id}/datasets/{dataset_id}`
    
    pub name: Option<String>,
    /// The BCP-47 language code of the source language.
    #[serde(rename="sourceLanguageCode")]
    
    pub source_language_code: Option<String>,
    /// The BCP-47 language code of the target language.
    #[serde(rename="targetLanguageCode")]
    
    pub target_language_code: Option<String>,
    /// Output only. Number of test examples (sentence pairs).
    #[serde(rename="testExampleCount")]
    
    pub test_example_count: Option<i32>,
    /// Output only. Number of training examples (sentence pairs).
    #[serde(rename="trainExampleCount")]
    
    pub train_example_count: Option<i32>,
    /// Output only. Timestamp when this dataset was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Number of validation examples (sentence pairs).
    #[serde(rename="validateExampleCount")]
    
    pub validate_example_count: Option<i32>,
}

impl client::RequestValue for Dataset {}
impl client::ResponseResult for Dataset {}


/// Input configuration for datasets.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatasetInputConfig {
    /// Files containing the sentence pairs to be imported to the dataset.
    #[serde(rename="inputFiles")]
    
    pub input_files: Option<Vec<InputFile>>,
}

impl client::Part for DatasetInputConfig {}


/// Output configuration for datasets.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatasetOutputConfig {
    /// Google Cloud Storage destination to write the output.
    #[serde(rename="gcsDestination")]
    
    pub gcs_destination: Option<GcsOutputDestination>,
}

impl client::Part for DatasetOutputConfig {}


/// The request message for language detection.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations detect language projects](ProjectLocationDetectLanguageCall) (request)
/// * [detect language projects](ProjectDetectLanguageCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DetectLanguageRequest {
    /// The content of the input stored as a string.
    
    pub content: Option<String>,
    /// Optional. The labels with user-defined metadata for the request. Label keys and values can be no longer than 63 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter. See https://cloud.google.com/translate/docs/advanced/labels for more information.
    
    pub labels: Option<HashMap<String, String>>,
    /// Optional. The format of the source text, for example, "text/html", "text/plain". If left blank, the MIME type defaults to "text/html".
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
    /// Optional. The language detection model to be used. Format: `projects/{project-number-or-id}/locations/{location-id}/models/language-detection/{model-id}` Only one language detection model is currently supported: `projects/{project-number-or-id}/locations/{location-id}/models/language-detection/default`. If not specified, the default model is used.
    
    pub model: Option<String>,
}

impl client::RequestValue for DetectLanguageRequest {}


/// The response message for language detection.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations detect language projects](ProjectLocationDetectLanguageCall) (response)
/// * [detect language projects](ProjectDetectLanguageCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DetectLanguageResponse {
    /// The most probable language detected by the Translation API. For each request, the Translation API will always return only one result.
    
    pub languages: Option<Vec<DetectedLanguage>>,
}

impl client::ResponseResult for DetectLanguageResponse {}


/// The response message for language detection.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DetectedLanguage {
    /// The confidence of the detection result for this language.
    
    pub confidence: Option<f32>,
    /// The BCP-47 language code of the source content in the request, detected automatically.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
}

impl client::Part for DetectedLanguage {}


/// A document translation request input config.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DocumentInputConfig {
    /// Document's content represented as a stream of bytes.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub content: Option<Vec<u8>>,
    /// Google Cloud Storage location. This must be a single file. For example: gs://example_bucket/example_file.pdf
    #[serde(rename="gcsSource")]
    
    pub gcs_source: Option<GcsSource>,
    /// Specifies the input document's mime_type. If not specified it will be determined using the file extension for gcs_source provided files. For a file provided through bytes content the mime_type must be provided. Currently supported mime types are: - application/pdf - application/vnd.openxmlformats-officedocument.wordprocessingml.document - application/vnd.openxmlformats-officedocument.presentationml.presentation - application/vnd.openxmlformats-officedocument.spreadsheetml.sheet
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
}

impl client::Part for DocumentInputConfig {}


/// A document translation request output config.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DocumentOutputConfig {
    /// Optional. Google Cloud Storage destination for the translation output, e.g., `gs://my_bucket/my_directory/`. The destination directory provided does not have to be empty, but the bucket must exist. If a file with the same name as the output file already exists in the destination an error will be returned. For a DocumentInputConfig.contents provided document, the output file will have the name "output_[trg]_translations.[ext]", where - [trg] corresponds to the translated file's language code, - [ext] corresponds to the translated file's extension according to its mime type. For a DocumentInputConfig.gcs_uri provided document, the output file will have a name according to its URI. For example: an input file with URI: "gs://a/b/c.[extension]" stored in a gcs_destination bucket with name "my_bucket" will have an output URI: "gs://my_bucket/a_b_c_[trg]_translations.[ext]", where - [trg] corresponds to the translated file's language code, - [ext] corresponds to the translated file's extension according to its mime type. If the document was directly provided through the request, then the output document will have the format: "gs://my_bucket/translated_document_[trg]_translations.[ext], where - [trg] corresponds to the translated file's language code, - [ext] corresponds to the translated file's extension according to its mime type. If a glossary was provided, then the output URI for the glossary translation will be equal to the default output URI but have `glossary_translations` instead of `translations`. For the previous example, its glossary URI would be: "gs://my_bucket/a_b_c_[trg]_glossary_translations.[ext]". Thus the max number of output files will be 2 (Translated document, Glossary translated document). Callers should expect no partial outputs. If there is any error during document translation, no output will be stored in the Cloud Storage bucket.
    #[serde(rename="gcsDestination")]
    
    pub gcs_destination: Option<GcsDestination>,
    /// Optional. Specifies the translated document's mime_type. If not specified, the translated file's mime type will be the same as the input file's mime type. Currently only support the output mime type to be the same as input mime type. - application/pdf - application/vnd.openxmlformats-officedocument.wordprocessingml.document - application/vnd.openxmlformats-officedocument.presentationml.presentation - application/vnd.openxmlformats-officedocument.spreadsheetml.sheet
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
}

impl client::Part for DocumentOutputConfig {}


/// A translated document message.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DocumentTranslation {
    /// The array of translated documents. It is expected to be size 1 for now. We may produce multiple translated documents in the future for other type of file formats.
    #[serde(rename="byteStreamOutputs")]
    
    #[serde_as(as = "Option<Vec<::client::serde::urlsafe_base64::Wrapper>>")]
    pub byte_stream_outputs: Option<Vec<Vec<u8>>>,
    /// The detected language for the input document. If the user did not provide the source language for the input document, this field will have the language code automatically detected. If the source language was passed, auto-detection of the language does not occur and this field is empty.
    #[serde(rename="detectedLanguageCode")]
    
    pub detected_language_code: Option<String>,
    /// The translated document's mime type.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
}

impl client::Part for DocumentTranslation {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations glossaries glossary entries delete projects](ProjectLocationGlossaryGlossaryEntryDeleteCall) (response)
/// * [locations operations cancel projects](ProjectLocationOperationCancelCall) (response)
/// * [locations operations delete projects](ProjectLocationOperationDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// A sentence pair.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Example {
    /// Output only. The resource name of the example, in form of `projects/{project-number-or-id}/locations/{location_id}/datasets/{dataset_id}/examples/{example_id}'
    
    pub name: Option<String>,
    /// Sentence in source language.
    #[serde(rename="sourceText")]
    
    pub source_text: Option<String>,
    /// Sentence in target language.
    #[serde(rename="targetText")]
    
    pub target_text: Option<String>,
    /// Output only. Usage of the sentence pair. Options are TRAIN|VALIDATION|TEST.
    
    pub usage: Option<String>,
}

impl client::Part for Example {}


/// Request message for ExportData.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets export data projects](ProjectLocationDatasetExportDataCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExportDataRequest {
    /// Required. The config for the output content.
    #[serde(rename="outputConfig")]
    
    pub output_config: Option<DatasetOutputConfig>,
}

impl client::RequestValue for ExportDataRequest {}


/// The Google Cloud Storage location for the output content.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GcsDestination {
    /// Required. The bucket used in 'output_uri_prefix' must exist and there must be no files under 'output_uri_prefix'. 'output_uri_prefix' must end with "/" and start with "gs://". One 'output_uri_prefix' can only be used by one batch translation job at a time. Otherwise an INVALID_ARGUMENT (400) error is returned.
    #[serde(rename="outputUriPrefix")]
    
    pub output_uri_prefix: Option<String>,
}

impl client::Part for GcsDestination {}


/// The Google Cloud Storage location for the input content.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GcsInputSource {
    /// Required. Source data URI. For example, `gs://my_bucket/my_object`.
    #[serde(rename="inputUri")]
    
    pub input_uri: Option<String>,
}

impl client::Part for GcsInputSource {}


/// The Google Cloud Storage location for the output content.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GcsOutputDestination {
    /// Required. Google Cloud Storage URI to output directory. For example, `gs://bucket/directory`. The requesting user must have write permission to the bucket. The directory will be created if it doesn't exist.
    #[serde(rename="outputUriPrefix")]
    
    pub output_uri_prefix: Option<String>,
}

impl client::Part for GcsOutputDestination {}


/// The Google Cloud Storage location for the input content.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GcsSource {
    /// Required. Source data URI. For example, `gs://my_bucket/my_object`.
    #[serde(rename="inputUri")]
    
    pub input_uri: Option<String>,
}

impl client::Part for GcsSource {}


/// Represents a glossary built from user-provided data.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations glossaries create projects](ProjectLocationGlossaryCreateCall) (request)
/// * [locations glossaries get projects](ProjectLocationGlossaryGetCall) (response)
/// * [locations glossaries patch projects](ProjectLocationGlossaryPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Glossary {
    /// Optional. The display name of the glossary.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. When the glossary creation was finished.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The number of entries defined in the glossary.
    #[serde(rename="entryCount")]
    
    pub entry_count: Option<i32>,
    /// Required. Provides examples to build the glossary from. Total glossary must not exceed 10M Unicode codepoints.
    #[serde(rename="inputConfig")]
    
    pub input_config: Option<GlossaryInputConfig>,
    /// Used with equivalent term set glossaries.
    #[serde(rename="languageCodesSet")]
    
    pub language_codes_set: Option<LanguageCodesSet>,
    /// Used with unidirectional glossaries.
    #[serde(rename="languagePair")]
    
    pub language_pair: Option<LanguageCodePair>,
    /// Required. The resource name of the glossary. Glossary names have the form `projects/{project-number-or-id}/locations/{location-id}/glossaries/{glossary-id}`.
    
    pub name: Option<String>,
    /// Output only. When CreateGlossary was called.
    #[serde(rename="submitTime")]
    
    pub submit_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Glossary {}
impl client::ResponseResult for Glossary {}


/// Represents a single entry in a glossary.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations glossaries glossary entries create projects](ProjectLocationGlossaryGlossaryEntryCreateCall) (request|response)
/// * [locations glossaries glossary entries get projects](ProjectLocationGlossaryGlossaryEntryGetCall) (response)
/// * [locations glossaries glossary entries patch projects](ProjectLocationGlossaryGlossaryEntryPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GlossaryEntry {
    /// Describes the glossary entry.
    
    pub description: Option<String>,
    /// Required. The resource name of the entry. Format: "projects/*/locations/*/glossaries/*/glossaryEntries/*"
    
    pub name: Option<String>,
    /// Used for an unidirectional glossary.
    #[serde(rename="termsPair")]
    
    pub terms_pair: Option<GlossaryTermsPair>,
    /// Used for an equivalent term sets glossary.
    #[serde(rename="termsSet")]
    
    pub terms_set: Option<GlossaryTermsSet>,
}

impl client::RequestValue for GlossaryEntry {}
impl client::ResponseResult for GlossaryEntry {}


/// Input configuration for glossaries.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GlossaryInputConfig {
    /// Required. Google Cloud Storage location of glossary data. File format is determined based on the filename extension. API returns [google.rpc.Code.INVALID_ARGUMENT] for unsupported URI-s and file formats. Wildcards are not allowed. This must be a single file in one of the following formats: For unidirectional glossaries: - TSV/CSV (`.tsv`/`.csv`): Two column file, tab- or comma-separated. The first column is source text. The second column is target text. No headers in this file. The first row contains data and not column names. - TMX (`.tmx`): TMX file with parallel data defining source/target term pairs. For equivalent term sets glossaries: - CSV (`.csv`): Multi-column CSV file defining equivalent glossary terms in multiple languages. See documentation for more information - [glossaries](https://cloud.google.com/translate/docs/advanced/glossary).
    #[serde(rename="gcsSource")]
    
    pub gcs_source: Option<GcsSource>,
}

impl client::Part for GlossaryInputConfig {}


/// Represents a single glossary term
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GlossaryTerm {
    /// The language for this glossary term.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// The text for the glossary term.
    
    pub text: Option<String>,
}

impl client::Part for GlossaryTerm {}


/// Represents a single entry for an unidirectional glossary.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GlossaryTermsPair {
    /// The source term is the term that will get match in the text,
    #[serde(rename="sourceTerm")]
    
    pub source_term: Option<GlossaryTerm>,
    /// The term that will replace the match source term.
    #[serde(rename="targetTerm")]
    
    pub target_term: Option<GlossaryTerm>,
}

impl client::Part for GlossaryTermsPair {}


/// Represents a single entry for an equivalent term set glossary. This is used for equivalent term sets where each term can be replaced by the other terms in the set.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GlossaryTermsSet {
    /// Each term in the set represents a term that can be replaced by the other terms.
    
    pub terms: Option<Vec<GlossaryTerm>>,
}

impl client::Part for GlossaryTermsSet {}


/// Request message for ImportData.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets import data projects](ProjectLocationDatasetImportDataCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImportDataRequest {
    /// Required. The config for the input content.
    #[serde(rename="inputConfig")]
    
    pub input_config: Option<DatasetInputConfig>,
}

impl client::RequestValue for ImportDataRequest {}


/// Input configuration for BatchTranslateText request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InputConfig {
    /// Required. Google Cloud Storage location for the source input. This can be a single file (for example, `gs://translation-test/input.tsv`) or a wildcard (for example, `gs://translation-test/*`). If a file extension is `.tsv`, it can contain either one or two columns. The first column (optional) is the id of the text request. If the first column is missing, we use the row number (0-based) from the input file as the ID in the output file. The second column is the actual text to be translated. We recommend each row be <= 10K Unicode codepoints, otherwise an error might be returned. Note that the input tsv must be RFC 4180 compliant. You could use https://github.com/Clever/csvlint to check potential formatting errors in your tsv file. csvlint --delimiter='\t' your_input_file.tsv The other supported file extensions are `.txt` or `.html`, which is treated as a single large chunk of text.
    #[serde(rename="gcsSource")]
    
    pub gcs_source: Option<GcsSource>,
    /// Optional. Can be "text/plain" or "text/html". For `.tsv`, "text/html" is used if mime_type is missing. For `.html`, this field must be "text/html" or empty. For `.txt`, this field must be "text/plain" or empty.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
}

impl client::Part for InputConfig {}


/// An input file.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InputFile {
    /// Google Cloud Storage file source.
    #[serde(rename="gcsSource")]
    
    pub gcs_source: Option<GcsInputSource>,
    /// Optional. Usage of the file contents. Options are TRAIN|VALIDATION|TEST, or UNASSIGNED (by default) for auto split.
    
    pub usage: Option<String>,
}

impl client::Part for InputFile {}


/// Used with unidirectional glossaries.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LanguageCodePair {
    /// Required. The BCP-47 language code of the input text, for example, "en-US". Expected to be an exact match for GlossaryTerm.language_code.
    #[serde(rename="sourceLanguageCode")]
    
    pub source_language_code: Option<String>,
    /// Required. The BCP-47 language code for translation output, for example, "zh-CN". Expected to be an exact match for GlossaryTerm.language_code.
    #[serde(rename="targetLanguageCode")]
    
    pub target_language_code: Option<String>,
}

impl client::Part for LanguageCodePair {}


/// Used with equivalent term set glossaries.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LanguageCodesSet {
    /// The BCP-47 language code(s) for terms defined in the glossary. All entries are unique. The list contains at least two entries. Expected to be an exact match for GlossaryTerm.language_code.
    #[serde(rename="languageCodes")]
    
    pub language_codes: Option<Vec<String>>,
}

impl client::Part for LanguageCodesSet {}


/// Response message for ListDatasets.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets list projects](ProjectLocationDatasetListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListDatasetsResponse {
    /// The datasets read.
    
    pub datasets: Option<Vec<Dataset>>,
    /// A token to retrieve next page of results. Pass this token to the page_token field in the ListDatasetsRequest to obtain the corresponding page.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListDatasetsResponse {}


/// Response message for ListExamples.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets examples list projects](ProjectLocationDatasetExampleListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListExamplesResponse {
    /// The sentence pairs.
    
    pub examples: Option<Vec<Example>>,
    /// A token to retrieve next page of results. Pass this token to the page_token field in the ListExamplesRequest to obtain the corresponding page.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListExamplesResponse {}


/// Response message for ListGlossaries.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations glossaries list projects](ProjectLocationGlossaryListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListGlossariesResponse {
    /// The list of glossaries for a project.
    
    pub glossaries: Option<Vec<Glossary>>,
    /// A token to retrieve a page of results. Pass this value in the [ListGlossariesRequest.page_token] field in the subsequent call to `ListGlossaries` method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListGlossariesResponse {}


/// Response message for ListGlossaryEntries
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations glossaries glossary entries list projects](ProjectLocationGlossaryGlossaryEntryListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListGlossaryEntriesResponse {
    /// Optional. The Glossary Entries
    #[serde(rename="glossaryEntries")]
    
    pub glossary_entries: Option<Vec<GlossaryEntry>>,
    /// Optional. A token to retrieve a page of results. Pass this value in the [ListGLossaryEntriesRequest.page_token] field in the subsequent calls.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListGlossaryEntriesResponse {}


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
pub struct ListLocationsResponse {
    /// A list of locations that matches the specified filter in the request.
    
    pub locations: Option<Vec<Location>>,
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListLocationsResponse {}


/// Response message for ListModels.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations models list projects](ProjectLocationModelListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListModelsResponse {
    /// The models read.
    
    pub models: Option<Vec<Model>>,
    /// A token to retrieve next page of results. Pass this token to the page_token field in the ListModelsRequest to obtain the corresponding page.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListModelsResponse {}


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
pub struct ListOperationsResponse {
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of operations that matches the specified filter in the request.
    
    pub operations: Option<Vec<Operation>>,
}

impl client::ResponseResult for ListOperationsResponse {}


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
pub struct Location {
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

impl client::ResponseResult for Location {}


/// A trained translation model.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations models create projects](ProjectLocationModelCreateCall) (request)
/// * [locations models get projects](ProjectLocationModelGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Model {
    /// Output only. Timestamp when the model resource was created, which is also when the training started.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The dataset from which the model is trained, in form of `projects/{project-number-or-id}/locations/{location_id}/datasets/{dataset_id}`
    
    pub dataset: Option<String>,
    /// Output only. Timestamp when the model training finished and ready to be used for translation.
    #[serde(rename="deployTime")]
    
    pub deploy_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The name of the model to show in the interface. The name can be up to 32 characters long and can consist only of ASCII Latin letters A-Z and a-z, underscores (_), and ASCII digits 0-9.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The resource name of the model, in form of `projects/{project-number-or-id}/locations/{location_id}/models/{model_id}`
    
    pub name: Option<String>,
    /// Output only. The BCP-47 language code of the source language.
    #[serde(rename="sourceLanguageCode")]
    
    pub source_language_code: Option<String>,
    /// Output only. The BCP-47 language code of the target language.
    #[serde(rename="targetLanguageCode")]
    
    pub target_language_code: Option<String>,
    /// Output only. Number of examples (sentence pairs) used to test the model.
    #[serde(rename="testExampleCount")]
    
    pub test_example_count: Option<i32>,
    /// Output only. Number of examples (sentence pairs) used to train the model.
    #[serde(rename="trainExampleCount")]
    
    pub train_example_count: Option<i32>,
    /// Output only. Timestamp when this model was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Number of examples (sentence pairs) used to validate the model.
    #[serde(rename="validateExampleCount")]
    
    pub validate_example_count: Option<i32>,
}

impl client::RequestValue for Model {}
impl client::ResponseResult for Model {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets create projects](ProjectLocationDatasetCreateCall) (response)
/// * [locations datasets delete projects](ProjectLocationDatasetDeleteCall) (response)
/// * [locations datasets export data projects](ProjectLocationDatasetExportDataCall) (response)
/// * [locations datasets import data projects](ProjectLocationDatasetImportDataCall) (response)
/// * [locations glossaries create projects](ProjectLocationGlossaryCreateCall) (response)
/// * [locations glossaries delete projects](ProjectLocationGlossaryDeleteCall) (response)
/// * [locations glossaries patch projects](ProjectLocationGlossaryPatchCall) (response)
/// * [locations models create projects](ProjectLocationModelCreateCall) (response)
/// * [locations models delete projects](ProjectLocationModelDeleteCall) (response)
/// * [locations operations get projects](ProjectLocationOperationGetCall) (response)
/// * [locations operations wait projects](ProjectLocationOperationWaitCall) (response)
/// * [locations batch translate document projects](ProjectLocationBatchTranslateDocumentCall) (response)
/// * [locations batch translate text projects](ProjectLocationBatchTranslateTextCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available.
    
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    
    pub error: Option<Status>,
    /// Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`.
    
    pub name: Option<String>,
    /// The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`.
    
    pub response: Option<HashMap<String, json::Value>>,
}

impl client::ResponseResult for Operation {}


/// Output configuration for BatchTranslateText request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OutputConfig {
    /// Google Cloud Storage destination for output content. For every single input file (for example, gs://a/b/c.[extension]), we generate at most 2 * n output files. (n is the # of target_language_codes in the BatchTranslateTextRequest). Output files (tsv) generated are compliant with RFC 4180 except that record delimiters are '\n' instead of '\r\n'. We don't provide any way to change record delimiters. While the input files are being processed, we write/update an index file 'index.csv' under 'output_uri_prefix' (for example, gs://translation-test/index.csv) The index file is generated/updated as new files are being translated. The format is: input_file,target_language_code,translations_file,errors_file, glossary_translations_file,glossary_errors_file input_file is one file we matched using gcs_source.input_uri. target_language_code is provided in the request. translations_file contains the translations. (details provided below) errors_file contains the errors during processing of the file. (details below). Both translations_file and errors_file could be empty strings if we have no content to output. glossary_translations_file and glossary_errors_file are always empty strings if the input_file is tsv. They could also be empty if we have no content to output. Once a row is present in index.csv, the input/output matching never changes. Callers should also expect all the content in input_file are processed and ready to be consumed (that is, no partial output file is written). Since index.csv will be keeping updated during the process, please make sure there is no custom retention policy applied on the output bucket that may avoid file updating. (https://cloud.google.com/storage/docs/bucket-lock#retention-policy) The format of translations_file (for target language code 'trg') is: `gs://translation_test/a_b_c_'trg'_translations.[extension]` If the input file extension is tsv, the output has the following columns: Column 1: ID of the request provided in the input, if it's not provided in the input, then the input row number is used (0-based). Column 2: source sentence. Column 3: translation without applying a glossary. Empty string if there is an error. Column 4 (only present if a glossary is provided in the request): translation after applying the glossary. Empty string if there is an error applying the glossary. Could be same string as column 3 if there is no glossary applied. If input file extension is a txt or html, the translation is directly written to the output file. If glossary is requested, a separate glossary_translations_file has format of gs://translation_test/a_b_c_'trg'_glossary_translations.[extension] The format of errors file (for target language code 'trg') is: gs://translation_test/a_b_c_'trg'_errors.[extension] If the input file extension is tsv, errors_file contains the following: Column 1: ID of the request provided in the input, if it's not provided in the input, then the input row number is used (0-based). Column 2: source sentence. Column 3: Error detail for the translation. Could be empty. Column 4 (only present if a glossary is provided in the request): Error when applying the glossary. If the input file extension is txt or html, glossary_error_file will be generated that contains error details. glossary_error_file has format of gs://translation_test/a_b_c_'trg'_glossary_errors.[extension]
    #[serde(rename="gcsDestination")]
    
    pub gcs_destination: Option<GcsDestination>,
}

impl client::Part for OutputConfig {}


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


/// A single supported language response corresponds to information related to one supported language.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SupportedLanguage {
    /// Human-readable name of the language localized in the display language specified in the request.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Supported language code, generally consisting of its ISO 639-1 identifier, for example, 'en', 'ja'. In certain cases, BCP-47 codes including language and region identifiers are returned (for example, 'zh-TW' and 'zh-CN').
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Can be used as a source language.
    #[serde(rename="supportSource")]
    
    pub support_source: Option<bool>,
    /// Can be used as a target language.
    #[serde(rename="supportTarget")]
    
    pub support_target: Option<bool>,
}

impl client::Part for SupportedLanguage {}


/// The response message for discovering supported languages.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations get supported languages projects](ProjectLocationGetSupportedLanguageCall) (response)
/// * [get supported languages projects](ProjectGetSupportedLanguageCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SupportedLanguages {
    /// A list of supported language responses. This list contains an entry for each language the Translation API supports.
    
    pub languages: Option<Vec<SupportedLanguage>>,
}

impl client::ResponseResult for SupportedLanguages {}


/// A document translation request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations translate document projects](ProjectLocationTranslateDocumentCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TranslateDocumentRequest {
    /// Optional. This flag is to support user customized attribution. If not provided, the default is `Machine Translated by Google`. Customized attribution should follow rules in https://cloud.google.com/translate/attribution#attribution_and_logos
    #[serde(rename="customizedAttribution")]
    
    pub customized_attribution: Option<String>,
    /// Required. Input configurations.
    #[serde(rename="documentInputConfig")]
    
    pub document_input_config: Option<DocumentInputConfig>,
    /// Optional. Output configurations. Defines if the output file should be stored within Cloud Storage as well as the desired output format. If not provided the translated file will only be returned through a byte-stream and its output mime type will be the same as the input file's mime type.
    #[serde(rename="documentOutputConfig")]
    
    pub document_output_config: Option<DocumentOutputConfig>,
    /// Optional. If true, use the text removal server to remove the shadow text on background image for native pdf translation.
    #[serde(rename="enableShadowRemovalNativePdf")]
    
    pub enable_shadow_removal_native_pdf: Option<bool>,
    /// Optional. Glossary to be applied. The glossary must be within the same region (have the same location-id) as the model, otherwise an INVALID_ARGUMENT (400) error is returned.
    #[serde(rename="glossaryConfig")]
    
    pub glossary_config: Option<TranslateTextGlossaryConfig>,
    /// Optional. is_translate_native_pdf_only field for external customers. If true, the page limit of online native pdf translation is 300 and only native pdf pages will be translated.
    #[serde(rename="isTranslateNativePdfOnly")]
    
    pub is_translate_native_pdf_only: Option<bool>,
    /// Optional. The labels with user-defined metadata for the request. Label keys and values can be no longer than 63 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter. See https://cloud.google.com/translate/docs/advanced/labels for more information.
    
    pub labels: Option<HashMap<String, String>>,
    /// Optional. The `model` type requested for this translation. The format depends on model type: - AutoML Translation models: `projects/{project-number-or-id}/locations/{location-id}/models/{model-id}` - General (built-in) models: `projects/{project-number-or-id}/locations/{location-id}/models/general/nmt`, If not provided, the default Google model (NMT) will be used for translation.
    
    pub model: Option<String>,
    /// Optional. The BCP-47 language code of the input document if known, for example, "en-US" or "sr-Latn". Supported language codes are listed in Language Support. If the source language isn't specified, the API attempts to identify the source language automatically and returns the source language within the response. Source language must be specified if the request contains a glossary or a custom model.
    #[serde(rename="sourceLanguageCode")]
    
    pub source_language_code: Option<String>,
    /// Required. The BCP-47 language code to use for translation of the input document, set to one of the language codes listed in Language Support.
    #[serde(rename="targetLanguageCode")]
    
    pub target_language_code: Option<String>,
}

impl client::RequestValue for TranslateDocumentRequest {}


/// A translated document response message.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations translate document projects](ProjectLocationTranslateDocumentCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TranslateDocumentResponse {
    /// Translated document.
    #[serde(rename="documentTranslation")]
    
    pub document_translation: Option<DocumentTranslation>,
    /// The `glossary_config` used for this translation.
    #[serde(rename="glossaryConfig")]
    
    pub glossary_config: Option<TranslateTextGlossaryConfig>,
    /// The document's translation output if a glossary is provided in the request. This can be the same as [TranslateDocumentResponse.document_translation] if no glossary terms apply.
    #[serde(rename="glossaryDocumentTranslation")]
    
    pub glossary_document_translation: Option<DocumentTranslation>,
    /// Only present when 'model' is present in the request. 'model' is normalized to have a project number. For example: If the 'model' field in TranslateDocumentRequest is: `projects/{project-id}/locations/{location-id}/models/general/nmt` then `model` here would be normalized to `projects/{project-number}/locations/{location-id}/models/general/nmt`.
    
    pub model: Option<String>,
}

impl client::ResponseResult for TranslateDocumentResponse {}


/// Configures which glossary is used for a specific target language and defines options for applying that glossary.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TranslateTextGlossaryConfig {
    /// Required. The `glossary` to be applied for this translation. The format depends on the glossary: - User-provided custom glossary: `projects/{project-number-or-id}/locations/{location-id}/glossaries/{glossary-id}`
    
    pub glossary: Option<String>,
    /// Optional. Indicates match is case insensitive. The default value is `false` if missing.
    #[serde(rename="ignoreCase")]
    
    pub ignore_case: Option<bool>,
}

impl client::Part for TranslateTextGlossaryConfig {}


/// The request message for synchronous translation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations translate text projects](ProjectLocationTranslateTextCall) (request)
/// * [translate text projects](ProjectTranslateTextCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TranslateTextRequest {
    /// Required. The content of the input in string format. We recommend the total content be less than 30,000 codepoints. The max length of this field is 1024. Use BatchTranslateText for larger text.
    
    pub contents: Option<Vec<String>>,
    /// Optional. Glossary to be applied. The glossary must be within the same region (have the same location-id) as the model, otherwise an INVALID_ARGUMENT (400) error is returned.
    #[serde(rename="glossaryConfig")]
    
    pub glossary_config: Option<TranslateTextGlossaryConfig>,
    /// Optional. The labels with user-defined metadata for the request. Label keys and values can be no longer than 63 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter. See https://cloud.google.com/translate/docs/advanced/labels for more information.
    
    pub labels: Option<HashMap<String, String>>,
    /// Optional. The format of the source text, for example, "text/html", "text/plain". If left blank, the MIME type defaults to "text/html".
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
    /// Optional. The `model` type requested for this translation. The format depends on model type: - AutoML Translation models: `projects/{project-number-or-id}/locations/{location-id}/models/{model-id}` - General (built-in) models: `projects/{project-number-or-id}/locations/{location-id}/models/general/nmt`, For global (non-regionalized) requests, use `location-id` `global`. For example, `projects/{project-number-or-id}/locations/global/models/general/nmt`. If not provided, the default Google model (NMT) will be used
    
    pub model: Option<String>,
    /// Optional. The BCP-47 language code of the input text if known, for example, "en-US" or "sr-Latn". Supported language codes are listed in Language Support. If the source language isn't specified, the API attempts to identify the source language automatically and returns the source language within the response.
    #[serde(rename="sourceLanguageCode")]
    
    pub source_language_code: Option<String>,
    /// Required. The BCP-47 language code to use for translation of the input text, set to one of the language codes listed in Language Support.
    #[serde(rename="targetLanguageCode")]
    
    pub target_language_code: Option<String>,
}

impl client::RequestValue for TranslateTextRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations translate text projects](ProjectLocationTranslateTextCall) (response)
/// * [translate text projects](ProjectTranslateTextCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TranslateTextResponse {
    /// Text translation responses if a glossary is provided in the request. This can be the same as `translations` if no terms apply. This field has the same length as `contents`.
    #[serde(rename="glossaryTranslations")]
    
    pub glossary_translations: Option<Vec<Translation>>,
    /// Text translation responses with no glossary applied. This field has the same length as `contents`.
    
    pub translations: Option<Vec<Translation>>,
}

impl client::ResponseResult for TranslateTextResponse {}


/// A single translation response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Translation {
    /// The BCP-47 language code of source text in the initial request, detected automatically, if no source language was passed within the initial request. If the source language was passed, auto-detection of the language does not occur and this field is empty.
    #[serde(rename="detectedLanguageCode")]
    
    pub detected_language_code: Option<String>,
    /// The `glossary_config` used for this translation.
    #[serde(rename="glossaryConfig")]
    
    pub glossary_config: Option<TranslateTextGlossaryConfig>,
    /// Only present when `model` is present in the request. `model` here is normalized to have project number. For example: If the `model` requested in TranslationTextRequest is `projects/{project-id}/locations/{location-id}/models/general/nmt` then `model` here would be normalized to `projects/{project-number}/locations/{location-id}/models/general/nmt`.
    
    pub model: Option<String>,
    /// Text translated into the target language. If an error occurs during translation, this field might be excluded from the response.
    #[serde(rename="translatedText")]
    
    pub translated_text: Option<String>,
}

impl client::Part for Translation {}


/// The request message for Operations.WaitOperation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations wait projects](ProjectLocationOperationWaitCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WaitOperationRequest {
    /// The maximum duration to wait before timing out. If left blank, the wait will be at most the time permitted by the underlying HTTP/RPC protocol. If RPC context deadline is also specified, the shorter one will be used.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub timeout: Option<client::chrono::Duration>,
}

impl client::RequestValue for WaitOperationRequest {}


