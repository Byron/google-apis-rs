use super::*;
/// A builder providing access to all methods supported on *document* resources.
/// It is not used directly, but through the [`CloudNaturalLanguage`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_language1 as language1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use language1::{CloudNaturalLanguage, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudNaturalLanguage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `analyze_entities(...)`, `analyze_entity_sentiment(...)`, `analyze_sentiment(...)`, `analyze_syntax(...)`, `annotate_text(...)` and `classify_text(...)`
/// // to build up your call.
/// let rb = hub.documents();
/// # }
/// ```
pub struct DocumentMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudNaturalLanguage<S>,
}

impl<'a, S> client::MethodsBuilder for DocumentMethods<'a, S> {}

impl<'a, S> DocumentMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Finds named entities (currently proper names and common nouns) in the text along with entity types, salience, mentions for each entity, and other properties.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn analyze_entities(&self, request: AnalyzeEntitiesRequest) -> DocumentAnalyzeEntityCall<'a, S> {
        DocumentAnalyzeEntityCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Finds entities, similar to AnalyzeEntities in the text and analyzes sentiment associated with each entity and its mentions.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn analyze_entity_sentiment(&self, request: AnalyzeEntitySentimentRequest) -> DocumentAnalyzeEntitySentimentCall<'a, S> {
        DocumentAnalyzeEntitySentimentCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Analyzes the sentiment of the provided text.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn analyze_sentiment(&self, request: AnalyzeSentimentRequest) -> DocumentAnalyzeSentimentCall<'a, S> {
        DocumentAnalyzeSentimentCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Analyzes the syntax of the text and provides sentence boundaries and tokenization along with part of speech tags, dependency trees, and other properties.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn analyze_syntax(&self, request: AnalyzeSyntaxRequest) -> DocumentAnalyzeSyntaxCall<'a, S> {
        DocumentAnalyzeSyntaxCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// A convenience method that provides all the features that analyzeSentiment, analyzeEntities, and analyzeSyntax provide in one call.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn annotate_text(&self, request: AnnotateTextRequest) -> DocumentAnnotateTextCall<'a, S> {
        DocumentAnnotateTextCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Classifies a document into categories.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn classify_text(&self, request: ClassifyTextRequest) -> DocumentClassifyTextCall<'a, S> {
        DocumentClassifyTextCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



