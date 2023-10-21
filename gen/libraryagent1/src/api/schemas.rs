use super::*;
/// A single book in the library.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [books borrow shelves](ShelfBookBorrowCall) (response)
/// * [books get shelves](ShelfBookGetCall) (response)
/// * [books return shelves](ShelfBookReturnCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleExampleLibraryagentV1Book {
    /// The name of the book author.
    
    pub author: Option<String>,
    /// The resource name of the book. Book names have the form `shelves/{shelf_id}/books/{book_id}`. The name is ignored when creating a book.
    
    pub name: Option<String>,
    /// Value indicating whether the book has been read.
    
    pub read: Option<bool>,
    /// The title of the book.
    
    pub title: Option<String>,
}

impl client::ResponseResult for GoogleExampleLibraryagentV1Book {}


/// Response message for LibraryAgent.ListBooks.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [books list shelves](ShelfBookListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleExampleLibraryagentV1ListBooksResponse {
    /// The list of books.
    
    pub books: Option<Vec<GoogleExampleLibraryagentV1Book>>,
    /// A token to retrieve next page of results. Pass this value in the ListBooksRequest.page_token field in the subsequent call to `ListBooks` method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleExampleLibraryagentV1ListBooksResponse {}


/// Response message for LibraryAgent.ListShelves.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list shelves](ShelfListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleExampleLibraryagentV1ListShelvesResponse {
    /// A token to retrieve next page of results. Pass this value in the ListShelvesRequest.page_token field in the subsequent call to `ListShelves` method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of shelves.
    
    pub shelves: Option<Vec<GoogleExampleLibraryagentV1Shelf>>,
}

impl client::ResponseResult for GoogleExampleLibraryagentV1ListShelvesResponse {}


/// A Shelf contains a collection of books with a theme.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get shelves](ShelfGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleExampleLibraryagentV1Shelf {
    /// Output only. The resource name of the shelf. Shelf names have the form `shelves/{shelf_id}`. The name is ignored when creating a shelf.
    
    pub name: Option<String>,
    /// The theme of the shelf
    
    pub theme: Option<String>,
}

impl client::ResponseResult for GoogleExampleLibraryagentV1Shelf {}


