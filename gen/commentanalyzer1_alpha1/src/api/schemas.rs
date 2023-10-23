use super::*;
/// The comment analysis request message.
/// LINT.IfChange
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [analyze comments](CommentAnalyzeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyzeCommentRequest {
    /// Opaque token that is echoed from the request to the response.
    #[serde(rename="clientToken")]
    
    pub client_token: Option<String>,
    /// The comment to analyze.
    
    pub comment: Option<TextEntry>,
    /// Optional identifier associating this AnalyzeCommentRequest with a
    /// particular client's community. Different communities may have different
    /// norms and rules. Specifying this value enables us to explore building
    /// community-specific models for clients.
    #[serde(rename="communityId")]
    
    pub community_id: Option<String>,
    /// The context of the comment.
    
    pub context: Option<Context>,
    /// Do not store the comment or context sent in this request. By default, the
    /// service may store comments/context for debugging purposes.
    #[serde(rename="doNotStore")]
    
    pub do_not_store: Option<bool>,
    /// The language(s) of the comment and context. If none are specified, we
    /// attempt to automatically detect the language. Specifying multiple languages
    /// means the text contains multiple lanugages. Both ISO and BCP-47 language
    /// codes are accepted.
    /// 
    /// The server returns an error if no language was specified and language
    /// detection fails. The server also returns an error if the languages (either
    /// specified by the caller, or auto-detected) are not *all* supported by the
    /// service.
    
    pub languages: Option<Vec<String>>,
    /// Specification of requested attributes. The AttributeParameters serve as
    /// configuration for each associated attribute. The map keys are attribute
    /// names. The available attributes may be different on each RFE installation,
    /// and can be seen by calling ListAttributes (see above).
    /// For the prod installation, known as Perspective API, at
    /// blade:commentanalyzer-esf and commentanalyzer.googleapis.com, see
    /// go/checker-models (internal) and
    /// https://github.com/conversationai/perspectiveapi/blob/master/2-api/models.md#all-attribute-types.
    #[serde(rename="requestedAttributes")]
    
    pub requested_attributes: Option<HashMap<String, AttributeParameters>>,
    /// Session ID. Used to join related RPCs into a single session. For example,
    /// an interactive tool that calls both the AnalyzeComment and
    /// SuggestCommentScore RPCs should set all invocations of both RPCs to the
    /// same Session ID, typically a random 64-bit integer.
    #[serde(rename="sessionId")]
    
    pub session_id: Option<String>,
    /// An advisory parameter that will return span annotations if the model
    /// is capable of providing scores with sub-comment resolution. This will
    /// likely increase the size of the returned message.
    #[serde(rename="spanAnnotations")]
    
    pub span_annotations: Option<bool>,
}

impl client::RequestValue for AnalyzeCommentRequest {}


/// The comment analysis response message.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [analyze comments](CommentAnalyzeCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyzeCommentResponse {
    /// Scores for the requested attributes. The map keys are attribute names (same
    /// as the requested_attribute field in AnalyzeCommentRequest, for example
    /// "ATTACK_ON_AUTHOR", "INFLAMMATORY", etc).
    #[serde(rename="attributeScores")]
    
    pub attribute_scores: Option<HashMap<String, AttributeScores>>,
    /// Same token from the original AnalyzeCommentRequest.
    #[serde(rename="clientToken")]
    
    pub client_token: Option<String>,
    /// Contains the languages detected from the text content, sorted in order of
    /// likelihood.
    #[serde(rename="detectedLanguages")]
    
    pub detected_languages: Option<Vec<String>>,
    /// The language(s) used by CommentAnalyzer service to choose which Model to
    /// use when analyzing the comment. Might better be called
    /// “effective_languages”. The logic used to make the choice is as follows:
    /// if !Request.languages.empty()
    /// effective_languages = Request.languages
    /// else
    /// effective_languages = detected_languages\[0\]
    
    pub languages: Option<Vec<String>>,
}

impl client::ResponseResult for AnalyzeCommentResponse {}


/// A type of context specific to a comment left on a single-threaded comment
/// message board, where comments are either a top level comment or the child of
/// a top level comment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ArticleAndParentComment {
    /// The source content about which the comment was made (article text, article
    /// summary, video transcript, etc).
    
    pub article: Option<TextEntry>,
    /// Refers to text that is a direct parent of the source comment, such as in a
    /// one-deep threaded message board. This field will only be present for
    /// comments that are replies to other comments and will not be populated for
    /// direct comments on the article_text.
    #[serde(rename="parentComment")]
    
    pub parent_comment: Option<TextEntry>,
}

impl client::Part for ArticleAndParentComment {}


/// Configurable parameters for attribute scoring.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AttributeParameters {
    /// Don't return scores for this attribute that are below this threshold. If
    /// unset, a default threshold will be applied. A FloatValue wrapper is used to
    /// distinguish between 0 vs. default/unset.
    #[serde(rename="scoreThreshold")]
    
    pub score_threshold: Option<f32>,
    /// What type of scores to return. If unset, defaults to probability scores.
    #[serde(rename="scoreType")]
    
    pub score_type: Option<AttributeParameterScoreTypeEnum>,
}

impl client::Part for AttributeParameters {}


/// This holds score values for a single attribute. It contains both per-span
/// scores as well as an overall summary score..
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AttributeScores {
    /// Per-span scores.
    #[serde(rename="spanScores")]
    
    pub span_scores: Option<Vec<SpanScore>>,
    /// Overall score for comment as a whole.
    #[serde(rename="summaryScore")]
    
    pub summary_score: Option<Score>,
}

impl client::Part for AttributeScores {}


/// Context is typically something that a Comment is referencing or replying to
/// (such as an article, or previous comment).
/// Note: Populate only ONE OF the following fields. The oneof syntax cannot be
/// used because that would require nesting entries inside another message and
/// breaking backwards compatibility. The server will return an error if more
/// than one of the following fields is present.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Context {
    /// Information about the source for which the original comment was made, and
    /// any parent comment info.
    #[serde(rename="articleAndParentComment")]
    
    pub article_and_parent_comment: Option<ArticleAndParentComment>,
    /// A list of messages. For example, a linear comments section or forum thread.
    
    pub entries: Option<Vec<TextEntry>>,
}

impl client::Part for Context {}


/// Analysis scores are described by a value and a ScoreType.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Score {
    /// The type of the above value.
    #[serde(rename="type")]
    
    pub type_: Option<ScoreTypeEnum>,
    /// Score value. Semantics described by type below.
    
    pub value: Option<f32>,
}

impl client::Part for Score {}


/// This is a single score for a given span of text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpanScore {
    /// "begin" and "end" describe the span of the original text that the attribute
    /// score applies to. The values are the UTF-16 codepoint range. "end" is
    /// exclusive. For example, with the text "Hi there", the begin/end pair (0,2)
    /// describes the text "Hi".
    /// 
    /// If "begin" and "end" are unset, the score applies to the full text.
    
    pub begin: Option<i32>,
    /// no description provided
    
    pub end: Option<i32>,
    /// The score value.
    
    pub score: Option<Score>,
}

impl client::Part for SpanScore {}


/// The comment score suggestion request message.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [suggestscore comments](CommentSuggestscoreCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SuggestCommentScoreRequest {
    /// Attribute scores for the comment. The map keys are attribute names, same as
    /// the requested_attribute field in AnalyzeCommentRequest (for example
    /// "ATTACK_ON_AUTHOR", "INFLAMMATORY", etc.). This field has the same type as
    /// the `attribute_scores` field in AnalyzeCommentResponse.
    /// 
    /// To specify an overall attribute score for the entire comment as a whole,
    /// use the `summary_score` field of the mapped AttributeScores object. To
    /// specify scores on specific subparts of the comment, use the `span_scores`
    /// field. All SpanScore objects must have begin and end fields set.
    /// 
    /// All Score objects must be explicitly set (for binary classification, use
    /// the score values 0 and 1). If Score objects don't include a ScoreType,
    /// `PROBABILITY` is assumed.
    /// 
    /// `attribute_scores` must not be empty. The mapped AttributeScores objects
    /// also must not be empty. An `INVALID_ARGUMENT` error is returned for all
    /// malformed requests.
    #[serde(rename="attributeScores")]
    
    pub attribute_scores: Option<HashMap<String, AttributeScores>>,
    /// Opaque token that is echoed from the request to the response.
    #[serde(rename="clientToken")]
    
    pub client_token: Option<String>,
    /// The comment being scored.
    
    pub comment: Option<TextEntry>,
    /// Optional identifier associating this comment score suggestion with a
    /// particular sub-community. Different communities may have different norms
    /// and rules. Specifying this value enables training community-specific
    /// models.
    #[serde(rename="communityId")]
    
    pub community_id: Option<String>,
    /// The context of the comment.
    
    pub context: Option<Context>,
    /// The language(s) of the comment and context. If none are specified, we
    /// attempt to automatically detect the language. Both ISO and BCP-47 language
    /// codes are accepted.
    
    pub languages: Option<Vec<String>>,
    /// Session ID. Used to join related RPCs into a single session. For example,
    /// an interactive tool that calls both the AnalyzeComment and
    /// SuggestCommentScore RPCs should set all invocations of both RPCs to the
    /// same Session ID, typically a random 64-bit integer.
    #[serde(rename="sessionId")]
    
    pub session_id: Option<String>,
}

impl client::RequestValue for SuggestCommentScoreRequest {}


/// The comment score suggestion response message.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [suggestscore comments](CommentSuggestscoreCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SuggestCommentScoreResponse {
    /// Same token from the original SuggestCommentScoreRequest.
    #[serde(rename="clientToken")]
    
    pub client_token: Option<String>,
    /// The list of languages detected from the comment text.
    #[serde(rename="detectedLanguages")]
    
    pub detected_languages: Option<Vec<String>>,
    /// The list of languages provided in the request.
    #[serde(rename="requestedLanguages")]
    
    pub requested_languages: Option<Vec<String>>,
}

impl client::ResponseResult for SuggestCommentScoreResponse {}


/// Represents a body of text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TextEntry {
    /// UTF-8 encoded text.
    
    pub text: Option<String>,
    /// Type of the text field.
    #[serde(rename="type")]
    
    pub type_: Option<TextEntryTypeEnum>,
}

impl client::Part for TextEntry {}


