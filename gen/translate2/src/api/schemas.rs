use super::*;
/// The request message for language detection.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [detect detections](DetectionDetectCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DetectLanguageRequest {
    /// The input text upon which to perform language detection. Repeat this
    /// parameter to perform language detection on multiple text inputs.
    
    pub q: Option<Vec<String>>,
}

impl client::RequestValue for DetectLanguageRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [detect detections](DetectionDetectCall) (response)
/// * [list detections](DetectionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DetectionsListResponse {
    /// A detections contains detection results of several text
    
    pub detections: Option<Vec<DetectionsResource>>,
}

impl client::ResponseResult for DetectionsListResponse {}


/// An array of languages which we detect for the given text The most likely language list first.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
/// The contained type is `Option<Vec<DetectionsResourceDetectionsResource>>`.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DetectionsResource {
    /// The confidence of the detection result of this language.
    
    pub confidence: Option<f32>,
    /// A boolean to indicate is the language detection result reliable.
    #[serde(rename="isReliable")]
    
    pub is_reliable: Option<bool>,
    /// The language we detected.
    
    pub language: Option<String>,
}

impl client::Part for DetectionsResource {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list languages](LanguageListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LanguagesListResponse {
    /// List of source/target languages supported by the translation API. If target parameter is unspecified, the list is sorted by the ASCII code point order of the language code. If target parameter is specified, the list is sorted by the collation order of the language name in the target language.
    
    pub languages: Option<Vec<LanguagesResource>>,
}

impl client::ResponseResult for LanguagesListResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LanguagesResource {
    /// Supported language code, generally consisting of its ISO 639-1
    /// identifier. (E.g. 'en', 'ja'). In certain cases, BCP-47 codes including
    /// language + region identifiers are returned (e.g. 'zh-TW' and 'zh-CH')
    
    pub language: Option<String>,
    /// Human readable name of the language localized to the target language.
    
    pub name: Option<String>,
}

impl client::Part for LanguagesResource {}


/// The main translation request message for the Cloud Translation API.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [translate translations](TranslationTranslateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TranslateTextRequest {
    /// The format of the source text, in either HTML (default) or plain-text. A
    /// value of "html" indicates HTML and a value of "text" indicates plain-text.
    
    pub format: Option<String>,
    /// The `model` type requested for this translation. Valid values are
    /// listed in public documentation.
    
    pub model: Option<String>,
    /// The input text to translate. Repeat this parameter to perform translation
    /// operations on multiple text inputs.
    
    pub q: Option<Vec<String>>,
    /// The language of the source text, set to one of the language codes listed in
    /// Language Support. If the source language is not specified, the API will
    /// attempt to identify the source language automatically and return it within
    /// the response.
    
    pub source: Option<String>,
    /// The language to use for translation of the input text, set to one of the
    /// language codes listed in Language Support.
    
    pub target: Option<String>,
}

impl client::RequestValue for TranslateTextRequest {}


/// The main language translation response message.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list translations](TranslationListCall) (response)
/// * [translate translations](TranslationTranslateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TranslationsListResponse {
    /// Translations contains list of translation results of given text
    
    pub translations: Option<Vec<TranslationsResource>>,
}

impl client::ResponseResult for TranslationsListResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TranslationsResource {
    /// The source language of the initial request, detected automatically, if
    /// no source language was passed within the initial request. If the
    /// source language was passed, auto-detection of the language will not
    /// occur and this field will be empty.
    #[serde(rename="detectedSourceLanguage")]
    
    pub detected_source_language: Option<String>,
    /// The `model` type used for this translation. Valid values are
    /// listed in public documentation. Can be different from requested `model`.
    /// Present only if specific model type was explicitly requested.
    
    pub model: Option<String>,
    /// Text translated into the target language.
    #[serde(rename="translatedText")]
    
    pub translated_text: Option<String>,
}

impl client::Part for TranslationsResource {}


/// An array of languages which we detect for the given text The most likely language list first.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DetectionsResourceNested {
    /// The confidence of the detection result of this language.
    
    pub confidence: Option<f32>,
    /// A boolean to indicate is the language detection result reliable.
    #[serde(rename="isReliable")]
    
    pub is_reliable: Option<bool>,
    /// The language we detected.
    
    pub language: Option<String>,
}

impl client::NestedType for DetectionsResourceNested {}
impl client::Part for DetectionsResourceNested {}


