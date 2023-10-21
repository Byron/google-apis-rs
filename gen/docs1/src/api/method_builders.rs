use super::*;
/// A builder providing access to all methods supported on *document* resources.
/// It is not used directly, but through the [`Docs`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_docs1 as docs1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use docs1::{Docs, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Docs::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `batch_update(...)`, `create(...)` and `get(...)`
/// // to build up your call.
/// let rb = hub.documents();
/// # }
/// ```
pub struct DocumentMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Docs<S>,
}

impl<'a, S> client::MethodsBuilder for DocumentMethods<'a, S> {}

impl<'a, S> DocumentMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Applies one or more updates to the document. Each request is validated before being applied. If any request is not valid, then the entire request will fail and nothing will be applied. Some requests have replies to give you some information about how they are applied. Other requests do not need to return information; these each return an empty reply. The order of replies matches that of the requests. For example, suppose you call batchUpdate with four updates, and only the third one returns information. The response would have two empty replies, the reply to the third request, and another empty reply, in that order. Because other users may be editing the document, the document might not exactly reflect your changes: your changes may be altered with respect to collaborator changes. If there are no collaborators, the document should reflect your changes. In any case, the updates in your request are guaranteed to be applied together atomically.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `documentId` - The ID of the document to update.
    pub fn batch_update(&self, request: BatchUpdateDocumentRequest, document_id: &str) -> DocumentBatchUpdateCall<'a, S> {
        DocumentBatchUpdateCall {
            hub: self.hub,
            _request: request,
            _document_id: document_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a blank document using the title given in the request. Other fields in the request, including any provided content, are ignored. Returns the created document.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: Document) -> DocumentCreateCall<'a, S> {
        DocumentCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the latest version of the specified document.
    /// 
    /// # Arguments
    ///
    /// * `documentId` - The ID of the document to retrieve.
    pub fn get(&self, document_id: &str) -> DocumentGetCall<'a, S> {
        DocumentGetCall {
            hub: self.hub,
            _document_id: document_id.to_string(),
            _suggestions_view_mode: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



