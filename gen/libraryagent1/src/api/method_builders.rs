use super::*;
/// A builder providing access to all methods supported on *shelf* resources.
/// It is not used directly, but through the [`Libraryagent`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_libraryagent1 as libraryagent1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use libraryagent1::{Libraryagent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Libraryagent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `books_borrow(...)`, `books_get(...)`, `books_list(...)`, `books_return(...)`, `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.shelves();
/// # }
/// ```
pub struct ShelfMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Libraryagent<S>,
}

impl<'a, S> client::MethodsBuilder for ShelfMethods<'a, S> {}

impl<'a, S> ShelfMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Borrow a book from the library. Returns the book if it is borrowed successfully. Returns NOT_FOUND if the book does not exist in the library. Returns quota exceeded error if the amount of books borrowed exceeds allocation quota in any dimensions.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the book to borrow.
    pub fn books_borrow(&self, name: &str) -> ShelfBookBorrowCall<'a, S> {
        ShelfBookBorrowCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a book. Returns NOT_FOUND if the book does not exist.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the book to retrieve.
    pub fn books_get(&self, name: &str) -> ShelfBookGetCall<'a, S> {
        ShelfBookGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists books in a shelf. The order is unspecified but deterministic. Newly created books will not necessarily be added to the end of this list. Returns NOT_FOUND if the shelf does not exist.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the shelf whose books we'd like to list.
    pub fn books_list(&self, parent: &str) -> ShelfBookListCall<'a, S> {
        ShelfBookListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Return a book to the library. Returns the book if it is returned to the library successfully. Returns error if the book does not belong to the library or the users didn't borrow before.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the book to return.
    pub fn books_return(&self, name: &str) -> ShelfBookReturnCall<'a, S> {
        ShelfBookReturnCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a shelf. Returns NOT_FOUND if the shelf does not exist.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the shelf to retrieve.
    pub fn get(&self, name: &str) -> ShelfGetCall<'a, S> {
        ShelfGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists shelves. The order is unspecified but deterministic. Newly created shelves will not necessarily be added to the end of this list.
    pub fn list(&self) -> ShelfListCall<'a, S> {
        ShelfListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



