use super::*;
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
/// let mut hub = CommentAnalyzer::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `analyze(...)` and `suggestscore(...)`
/// // to build up your call.
/// let rb = hub.comments();
/// # }
/// ```
pub struct CommentMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CommentAnalyzer<S>,
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



