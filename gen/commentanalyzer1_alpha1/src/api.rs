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
    /// View your email address
    UserinfoEmail,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::UserinfoEmail => "https://www.googleapis.com/auth/userinfo.email",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::UserinfoEmail
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all CommentAnalyzer related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_commentanalyzer1_alpha1 as commentanalyzer1_alpha1;
/// use commentanalyzer1_alpha1::api::AnalyzeCommentRequest;
/// use commentanalyzer1_alpha1::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use commentanalyzer1_alpha1::{CommentAnalyzer, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
/// let mut hub = CommentAnalyzer::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = AnalyzeCommentRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.comments().analyze(req)
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
pub struct CommentAnalyzer<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: Box<dyn client::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, S> client::Hub for CommentAnalyzer<S> {}

impl<'a, S> CommentAnalyzer<S> {

    pub fn new<A: 'static + client::GetToken>(client: hyper::Client<S, hyper::body::Body>, auth: A) -> CommentAnalyzer<S> {
        CommentAnalyzer {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/5.0.5".to_string(),
            _base_url: "https://commentanalyzer.googleapis.com/".to_string(),
            _root_url: "https://commentanalyzer.googleapis.com/".to_string(),
        }
    }

    pub fn comments(&'a self) -> CommentMethods<'a, S> {
        CommentMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/5.0.5`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://commentanalyzer.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://commentanalyzer.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// The comment analysis request message.
/// LINT.IfChange
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [analyze comments](CommentAnalyzeCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
    
    pub score_type: Option<String>,
}

impl client::Part for AttributeParameters {}


/// This holds score values for a single attribute. It contains both per-span
/// scores as well as an overall summary score..
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Score {
    /// The type of the above value.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Score value. Semantics described by type below.
    
    pub value: Option<f32>,
}

impl client::Part for Score {}


/// This is a single score for a given span of text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TextEntry {
    /// UTF-8 encoded text.
    
    pub text: Option<String>,
    /// Type of the text field.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for TextEntry {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *comment* resources.
/// It is not used directly, but through the [`CommentAnalyzer`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_commentanalyzer1_alpha1 as commentanalyzer1_alpha1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use commentanalyzer1_alpha1::{CommentAnalyzer, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CommentAnalyzer::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `analyze(...)` and `suggestscore(...)`
/// // to build up your call.
/// let rb = hub.comments();
/// # }
/// ```
pub struct CommentMethods<'a, S>
    where S: 'a {

    hub: &'a CommentAnalyzer<S>,
}

impl<'a, S> client::MethodsBuilder for CommentMethods<'a, S> {}

impl<'a, S> CommentMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Analyzes the provided text and returns scores for requested attributes.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn analyze(&self, request: AnalyzeCommentRequest) -> CommentAnalyzeCall<'a, S> {
        CommentAnalyzeCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Suggest comment scores as training data.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn suggestscore(&self, request: SuggestCommentScoreRequest) -> CommentSuggestscoreCall<'a, S> {
        CommentSuggestscoreCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Analyzes the provided text and returns scores for requested attributes.
///
/// A builder for the *analyze* method supported by a *comment* resource.
/// It is not used directly, but through a [`CommentMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_commentanalyzer1_alpha1 as commentanalyzer1_alpha1;
/// use commentanalyzer1_alpha1::api::AnalyzeCommentRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use commentanalyzer1_alpha1::{CommentAnalyzer, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = CommentAnalyzer::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = AnalyzeCommentRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.comments().analyze(req)
///              .doit().await;
/// # }
/// ```
pub struct CommentAnalyzeCall<'a, S>
    where S: 'a {

    hub: &'a CommentAnalyzer<S>,
    _request: AnalyzeCommentRequest,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for CommentAnalyzeCall<'a, S> {}

impl<'a, S> CommentAnalyzeCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, AnalyzeCommentResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "commentanalyzer.comments.analyze",
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
        let mut url = self.hub._base_url.clone() + "v1alpha1/comments:analyze";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::UserinfoEmail.as_ref().to_string());
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
    pub fn request(mut self, new_value: AnalyzeCommentRequest) -> CommentAnalyzeCall<'a, S> {
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CommentAnalyzeCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> CommentAnalyzeCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::UserinfoEmail`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> CommentAnalyzeCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> CommentAnalyzeCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> CommentAnalyzeCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Suggest comment scores as training data.
///
/// A builder for the *suggestscore* method supported by a *comment* resource.
/// It is not used directly, but through a [`CommentMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_commentanalyzer1_alpha1 as commentanalyzer1_alpha1;
/// use commentanalyzer1_alpha1::api::SuggestCommentScoreRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use commentanalyzer1_alpha1::{CommentAnalyzer, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = CommentAnalyzer::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = SuggestCommentScoreRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.comments().suggestscore(req)
///              .doit().await;
/// # }
/// ```
pub struct CommentSuggestscoreCall<'a, S>
    where S: 'a {

    hub: &'a CommentAnalyzer<S>,
    _request: SuggestCommentScoreRequest,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for CommentSuggestscoreCall<'a, S> {}

impl<'a, S> CommentSuggestscoreCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, SuggestCommentScoreResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "commentanalyzer.comments.suggestscore",
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
        let mut url = self.hub._base_url.clone() + "v1alpha1/comments:suggestscore";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::UserinfoEmail.as_ref().to_string());
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
    pub fn request(mut self, new_value: SuggestCommentScoreRequest) -> CommentSuggestscoreCall<'a, S> {
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CommentSuggestscoreCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> CommentSuggestscoreCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::UserinfoEmail`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> CommentSuggestscoreCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> CommentSuggestscoreCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> CommentSuggestscoreCall<'a, S> {
        self._scopes.clear();
        self
    }
}


