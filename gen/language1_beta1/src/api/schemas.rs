use super::*;
/// The entity analysis request message.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [analyze entities documents](DocumentAnalyzeEntityCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyzeEntitiesRequest {
    /// Input document.
    
    pub document: Option<Document>,
    /// The encoding type used by the API to calculate offsets.
    #[serde(rename="encodingType")]
    
    pub encoding_type: Option<AnalyzeEntitiesRequestEncodingTypeEnum>,
}

impl client::RequestValue for AnalyzeEntitiesRequest {}


/// The entity analysis response message.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [analyze entities documents](DocumentAnalyzeEntityCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyzeEntitiesResponse {
    /// The recognized entities in the input document.
    
    pub entities: Option<Vec<Entity>>,
    /// The language of the text, which will be the same as the language specified in the request or, if not specified, the automatically-detected language. See Document.language field for more details.
    
    pub language: Option<String>,
}

impl client::ResponseResult for AnalyzeEntitiesResponse {}


/// The sentiment analysis request message.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [analyze sentiment documents](DocumentAnalyzeSentimentCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyzeSentimentRequest {
    /// Input document.
    
    pub document: Option<Document>,
    /// The encoding type used by the API to calculate sentence offsets for the sentence sentiment.
    #[serde(rename="encodingType")]
    
    pub encoding_type: Option<AnalyzeSentimentRequestEncodingTypeEnum>,
}

impl client::RequestValue for AnalyzeSentimentRequest {}


/// The sentiment analysis response message.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [analyze sentiment documents](DocumentAnalyzeSentimentCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyzeSentimentResponse {
    /// The overall sentiment of the input document.
    #[serde(rename="documentSentiment")]
    
    pub document_sentiment: Option<Sentiment>,
    /// The language of the text, which will be the same as the language specified in the request or, if not specified, the automatically-detected language. See Document.language field for more details.
    
    pub language: Option<String>,
    /// The sentiment for all the sentences in the document.
    
    pub sentences: Option<Vec<Sentence>>,
}

impl client::ResponseResult for AnalyzeSentimentResponse {}


/// The syntax analysis request message.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [analyze syntax documents](DocumentAnalyzeSyntaxCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyzeSyntaxRequest {
    /// Input document.
    
    pub document: Option<Document>,
    /// The encoding type used by the API to calculate offsets.
    #[serde(rename="encodingType")]
    
    pub encoding_type: Option<AnalyzeSyntaxRequestEncodingTypeEnum>,
}

impl client::RequestValue for AnalyzeSyntaxRequest {}


/// The syntax analysis response message.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [analyze syntax documents](DocumentAnalyzeSyntaxCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyzeSyntaxResponse {
    /// The language of the text, which will be the same as the language specified in the request or, if not specified, the automatically-detected language. See Document.language field for more details.
    
    pub language: Option<String>,
    /// Sentences in the input document.
    
    pub sentences: Option<Vec<Sentence>>,
    /// Tokens, along with their syntactic information, in the input document.
    
    pub tokens: Option<Vec<Token>>,
}

impl client::ResponseResult for AnalyzeSyntaxResponse {}


/// The request message for the text annotation API, which can perform multiple analysis types (sentiment, entities, and syntax) in one call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [annotate text documents](DocumentAnnotateTextCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnnotateTextRequest {
    /// Input document.
    
    pub document: Option<Document>,
    /// The encoding type used by the API to calculate offsets.
    #[serde(rename="encodingType")]
    
    pub encoding_type: Option<AnnotateTextRequestEncodingTypeEnum>,
    /// The enabled features.
    
    pub features: Option<Features>,
}

impl client::RequestValue for AnnotateTextRequest {}


/// The text annotations response message.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [annotate text documents](DocumentAnnotateTextCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnnotateTextResponse {
    /// The overall sentiment for the document. Populated if the user enables AnnotateTextRequest.Features.extract_document_sentiment.
    #[serde(rename="documentSentiment")]
    
    pub document_sentiment: Option<Sentiment>,
    /// Entities, along with their semantic information, in the input document. Populated if the user enables AnnotateTextRequest.Features.extract_entities.
    
    pub entities: Option<Vec<Entity>>,
    /// The language of the text, which will be the same as the language specified in the request or, if not specified, the automatically-detected language. See Document.language field for more details.
    
    pub language: Option<String>,
    /// Sentences in the input document. Populated if the user enables AnnotateTextRequest.Features.extract_syntax.
    
    pub sentences: Option<Vec<Sentence>>,
    /// Tokens, along with their syntactic information, in the input document. Populated if the user enables AnnotateTextRequest.Features.extract_syntax.
    
    pub tokens: Option<Vec<Token>>,
}

impl client::ResponseResult for AnnotateTextResponse {}


/// Represents dependency parse tree information for a token.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DependencyEdge {
    /// Represents the head of this token in the dependency tree. This is the index of the token which has an arc going to this token. The index is the position of the token in the array of tokens returned by the API method. If this token is a root token, then the `head_token_index` is its own index.
    #[serde(rename="headTokenIndex")]
    
    pub head_token_index: Option<i32>,
    /// The parse label for the token.
    
    pub label: Option<DependencyEdgeLabelEnum>,
}

impl client::Part for DependencyEdge {}


/// \################################################################ # Represents the input to API methods.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [analyze entities documents](DocumentAnalyzeEntityCall) (none)
/// * [analyze sentiment documents](DocumentAnalyzeSentimentCall) (none)
/// * [analyze syntax documents](DocumentAnalyzeSyntaxCall) (none)
/// * [annotate text documents](DocumentAnnotateTextCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Document {
    /// The content of the input in string format. Cloud audit logging exempt since it is based on user data.
    
    pub content: Option<String>,
    /// The Google Cloud Storage URI where the file content is located. This URI must be of the form: gs://bucket_name/object_name. For more details, see https://cloud.google.com/storage/docs/reference-uris. NOTE: Cloud Storage object versioning is not supported.
    #[serde(rename="gcsContentUri")]
    
    pub gcs_content_uri: Option<String>,
    /// The language of the document (if not specified, the language is automatically detected). Both ISO and BCP-47 language codes are accepted. [Language Support](https://cloud.google.com/natural-language/docs/languages) lists currently supported languages for each API method. If the language (either specified by the caller or automatically detected) is not supported by the called API method, an `INVALID_ARGUMENT` error is returned.
    
    pub language: Option<String>,
    /// Required. If the type is not set or is `TYPE_UNSPECIFIED`, returns an `INVALID_ARGUMENT` error.
    #[serde(rename="type")]
    
    pub type_: Option<DocumentTypeEnum>,
}

impl client::Resource for Document {}


/// Represents a phrase in the text that is a known entity, such as a person, an organization, or location. The API associates information, such as salience and mentions, with entities.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Entity {
    /// The mentions of this entity in the input document. The API currently supports proper noun mentions.
    
    pub mentions: Option<Vec<EntityMention>>,
    /// Metadata associated with the entity. Currently, Wikipedia URLs and Knowledge Graph MIDs are provided, if available. The associated keys are "wikipedia_url" and "mid", respectively.
    
    pub metadata: Option<HashMap<String, String>>,
    /// The representative name for the entity.
    
    pub name: Option<String>,
    /// The salience score associated with the entity in the [0, 1.0] range. The salience score for an entity provides information about the importance or centrality of that entity to the entire document text. Scores closer to 0 are less salient, while scores closer to 1.0 are highly salient.
    
    pub salience: Option<f32>,
    /// The entity type.
    #[serde(rename="type")]
    
    pub type_: Option<EntityTypeEnum>,
}

impl client::Part for Entity {}


/// Represents a mention for an entity in the text. Currently, proper noun mentions are supported.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EntityMention {
    /// The mention text.
    
    pub text: Option<TextSpan>,
    /// The type of the entity mention.
    #[serde(rename="type")]
    
    pub type_: Option<EntityMentionTypeEnum>,
}

impl client::Part for EntityMention {}


/// All available features for sentiment, syntax, and semantic analysis. Setting each one to true will enable that specific analysis for the input.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Features {
    /// Extract document-level sentiment.
    #[serde(rename="extractDocumentSentiment")]
    
    pub extract_document_sentiment: Option<bool>,
    /// Extract entities.
    #[serde(rename="extractEntities")]
    
    pub extract_entities: Option<bool>,
    /// Extract syntax information.
    #[serde(rename="extractSyntax")]
    
    pub extract_syntax: Option<bool>,
}

impl client::Part for Features {}


/// Represents part of speech information for a token.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PartOfSpeech {
    /// The grammatical aspect.
    
    pub aspect: Option<PartOfSpeechAspectEnum>,
    /// The grammatical case.
    
    pub case: Option<PartOfSpeechCaseEnum>,
    /// The grammatical form.
    
    pub form: Option<PartOfSpeechFormEnum>,
    /// The grammatical gender.
    
    pub gender: Option<PartOfSpeechGenderEnum>,
    /// The grammatical mood.
    
    pub mood: Option<PartOfSpeechMoodEnum>,
    /// The grammatical number.
    
    pub number: Option<PartOfSpeechNumberEnum>,
    /// The grammatical person.
    
    pub person: Option<PartOfSpeechPersonEnum>,
    /// The grammatical properness.
    
    pub proper: Option<PartOfSpeechProperEnum>,
    /// The grammatical reciprocity.
    
    pub reciprocity: Option<PartOfSpeechReciprocityEnum>,
    /// The part of speech tag.
    
    pub tag: Option<PartOfSpeechTagEnum>,
    /// The grammatical tense.
    
    pub tense: Option<PartOfSpeechTenseEnum>,
    /// The grammatical voice.
    
    pub voice: Option<PartOfSpeechVoiceEnum>,
}

impl client::Part for PartOfSpeech {}


/// Represents a sentence in the input document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Sentence {
    /// For calls to AnalyzeSentiment or if AnnotateTextRequest.Features.extract_document_sentiment is set to true, this field will contain the sentiment for the sentence.
    
    pub sentiment: Option<Sentiment>,
    /// The sentence text.
    
    pub text: Option<TextSpan>,
}

impl client::Part for Sentence {}


/// Represents the feeling associated with the entire text or entities in the text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Sentiment {
    /// A non-negative number in the [0, +inf) range, which represents the absolute magnitude of sentiment regardless of score (positive or negative).
    
    pub magnitude: Option<f32>,
    /// DEPRECATED FIELD - This field is being deprecated in favor of score. Please refer to our documentation at https://cloud.google.com/natural-language/docs for more information.
    
    pub polarity: Option<f32>,
    /// Sentiment score between -1.0 (negative sentiment) and 1.0 (positive sentiment).
    
    pub score: Option<f32>,
}

impl client::Part for Sentiment {}


/// Represents an output piece of text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TextSpan {
    /// The API calculates the beginning offset of the content in the original document according to the EncodingType specified in the API request.
    #[serde(rename="beginOffset")]
    
    pub begin_offset: Option<i32>,
    /// The content of the output text.
    
    pub content: Option<String>,
}

impl client::Part for TextSpan {}


/// Represents the smallest syntactic building block of the text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Token {
    /// Dependency tree parse for this token.
    #[serde(rename="dependencyEdge")]
    
    pub dependency_edge: Option<DependencyEdge>,
    /// [Lemma](https://en.wikipedia.org/wiki/Lemma_%28morphology%29) of the token.
    
    pub lemma: Option<String>,
    /// Parts of speech tag for this token.
    #[serde(rename="partOfSpeech")]
    
    pub part_of_speech: Option<PartOfSpeech>,
    /// The token text.
    
    pub text: Option<TextSpan>,
}

impl client::Part for Token {}


