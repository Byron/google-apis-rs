// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Docs* crate version *1.0.10+20190627*, where *20190627* is the exact revision of the *docs:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.10*.
//! 
//! Everything else about the *Docs* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/docs/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/docs1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Docs.html) ... 
//! 
//! * [documents](struct.Document.html)
//!  * [*batch update*](struct.DocumentBatchUpdateCall.html), [*create*](struct.DocumentCreateCall.html) and [*get*](struct.DocumentGetCall.html)
//! 
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.Docs.html)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](trait.MethodsBuilder.html) which in turn
//!       allow access to individual [*Call Builders*](trait.CallBuilder.html)
//! * **[Resources](trait.Resource.html)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](trait.Part.html)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](trait.CallBuilder.html)**
//!     * operations to apply to *Resources*
//! 
//! All *structures* are marked with applicable traits to further categorize them and ease browsing.
//! 
//! Generally speaking, you can invoke *Activities* like this:
//! 
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit()
//! ```
//! 
//! Or specifically ...
//! 
//! ```ignore
//! let r = hub.documents().create(...).doit()
//! let r = hub.documents().get(...).doit()
//! let r = hub.documents().batch_update(...).doit()
//! ```
//! 
//! The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
//! supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
//! specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
//! The `doit()` method performs the actual communication with the server and returns the respective result.
//! 
//! # Usage
//! 
//! ## Setting up your Project
//! 
//! To use this library, you would put the following lines into your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! google-docs1 = "*"
//! # This project intentionally uses an old version of Hyper. See
//! # https://github.com/Byron/google-apis-rs/issues/173 for more
//! # information.
//! hyper = "^0.10"
//! hyper-rustls = "^0.6"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! yup-oauth2 = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_docs1 as docs1;
//! use docs1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use docs1::Docs;
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
//!                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
//!                               <MemoryStorage as Default>::default(), None);
//! let mut hub = Docs::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.documents().get("documentId")
//!              .suggestions_view_mode("sed")
//!              .doit();
//! 
//! match result {
//!     Err(e) => match e {
//!         // The Error enum provides details about what exactly happened.
//!         // You can also just use its `Debug`, `Display` or `Error` traits
//!          Error::HttpError(_)
//!         |Error::MissingAPIKey
//!         |Error::MissingToken(_)
//!         |Error::Cancelled
//!         |Error::UploadSizeLimitExceeded(_, _)
//!         |Error::Failure(_)
//!         |Error::BadRequest(_)
//!         |Error::FieldClash(_)
//!         |Error::JsonDecodeError(_, _) => println!("{}", e),
//!     },
//!     Ok(res) => println!("Success: {:?}", res),
//! }
//! # }
//! ```
//! ## Handling Errors
//! 
//! All errors produced by the system are provided either as [Result](enum.Result.html) enumeration as return value of 
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](enum.Result.html), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](trait.ResponseResult.html), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//! 
//! Methods supporting uploads can do so using up to 2 different protocols: 
//! *simple* and *resumable*. The distinctiveness of each is represented by customized 
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//! 
//! ## Customization and Callbacks
//! 
//! You may alter the way an `doit()` method is called by providing a [delegate](trait.Delegate.html) to the 
//! [Method Builder](trait.CallBuilder.html) before making the final `doit()` call. 
//! Respective methods will be called to provide progress information, as well as determine whether the system should 
//! retry on failure.
//! 
//! The [delegate trait](trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.
//! 
//! ## Optional Parts in Server-Requests
//! 
//! All structures provided by this library are made to be [enocodable](trait.RequestValue.html) and 
//! [decodable](trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
//! are valid.
//! Most optionals are are considered [Parts](trait.Part.html) which are identifiable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](trait.RequestValue.html) are moved
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 

// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]

// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate yup_oauth2 as oauth2;
extern crate mime;
extern crate url;

mod cmn;

use std::collections::HashMap;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::default::Default;
use std::collections::BTreeMap;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;
use std::thread::sleep;
use std::time::Duration;

pub use cmn::*;


// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// See, edit, create, and delete all of your Google Drive files
    Drive,

    /// View your Google Docs documents
    DocumentReadonly,

    /// View and manage Google Drive files and folders that you have opened or created with this app
    DriveFile,

    /// View and manage your Google Docs documents
    Document,

    /// See and download all your Google Drive files
    DriveReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Drive => "https://www.googleapis.com/auth/drive",
            Scope::DocumentReadonly => "https://www.googleapis.com/auth/documents.readonly",
            Scope::DriveFile => "https://www.googleapis.com/auth/drive.file",
            Scope::Document => "https://www.googleapis.com/auth/documents",
            Scope::DriveReadonly => "https://www.googleapis.com/auth/drive.readonly",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::DocumentReadonly
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all Docs related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_docs1 as docs1;
/// use docs1::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use docs1::Docs;
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Docs::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.documents().get("documentId")
///              .suggestions_view_mode("dolores")
///              .doit();
/// 
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
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
pub struct Docs<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for Docs<C, A> {}

impl<'a, C, A> Docs<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Docs<C, A> {
        Docs {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.10".to_string(),
            _base_url: "https://docs.googleapis.com/".to_string(),
            _root_url: "https://docs.googleapis.com/".to_string(),
        }
    }

    pub fn documents(&'a self) -> DocumentMethods<'a, C, A> {
        DocumentMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.10`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://docs.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://docs.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// A mask that indicates which of the fields on the base
/// EmbeddedDrawingProperties
/// have been changed in this suggestion. For any field set to true, there is a
/// new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EmbeddedDrawingPropertiesSuggestionState { _never_set: Option<bool> }

impl Part for EmbeddedDrawingPropertiesSuggestionState {}


/// Deletes content from the document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteContentRangeRequest {
    /// The range of content to delete.
    /// 
    /// Deleting text that crosses a paragraph boundary may result in changes
    /// to paragraph styles, lists, positioned objects and bookmarks as the two
    /// paragraphs are merged.
    /// 
    /// Attempting to delete certain ranges can result in an invalid document
    /// structure in which case a 400 bad request error is returned.
    /// 
    /// Some examples of invalid delete requests include:
    /// 
    /// * Deleting one code unit of a surrogate pair.
    /// * Deleting the last newline character of a Body, Header,
    ///   Footer, Footnote, TableCell or TableOfContents.
    /// * Deleting the start or end of a Table,
    ///   TableOfContents or Equation without deleting the entire element.
    /// * Deleting the newline character before a
    ///   Table,
    ///   TableOfContents or
    ///   SectionBreak without deleting the
    ///   element.
    /// * Deleting individual rows or cells of a table. Deleting the content within
    ///   a table cell is allowed.
    pub range: Option<Range>,
}

impl Part for DeleteContentRangeRequest {}


/// A border around an EmbeddedObject.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EmbeddedObjectBorder {
    /// The color of the border.
    pub color: Option<OptionalColor>,
    /// The width of the border.
    pub width: Option<Dimension>,
    /// The dash style of the border.
    #[serde(rename="dashStyle")]
    pub dash_style: Option<String>,
    /// The property state of the border property.
    #[serde(rename="propertyState")]
    pub property_state: Option<String>,
}

impl Part for EmbeddedObjectBorder {}


/// A ParagraphElement representing a
/// column break. A column break makes the subsequent text start at the top of
/// the next column.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ColumnBreak {
    /// The text style of this ColumnBreak.
    /// 
    /// Similar to text content, like text runs and footnote references, the text
    /// style of a column break can affect content layout as well as the styling of
    /// text inserted adjacent to it.
    #[serde(rename="textStyle")]
    pub text_style: Option<TextStyle>,
    /// The suggested deletion IDs. If empty, then there are no suggested deletions
    /// of this content.
    #[serde(rename="suggestedDeletionIds")]
    pub suggested_deletion_ids: Option<Vec<String>>,
    /// The suggested text style changes to this ColumnBreak, keyed by suggestion
    /// ID.
    #[serde(rename="suggestedTextStyleChanges")]
    pub suggested_text_style_changes: Option<HashMap<String, SuggestedTextStyle>>,
    /// The suggested insertion IDs. A ColumnBreak may have multiple insertion IDs if it is
    /// a nested suggested change. If empty, then this is not a suggested
    /// insertion.
    #[serde(rename="suggestedInsertionIds")]
    pub suggested_insertion_ids: Option<Vec<String>>,
}

impl Part for ColumnBreak {}


/// Replaces all instances of text matching a criteria with replace text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReplaceAllTextRequest {
    /// Finds text in the document matching this substring.
    #[serde(rename="containsText")]
    pub contains_text: Option<SubstringMatchCriteria>,
    /// The text that will replace the matched text.
    #[serde(rename="replaceText")]
    pub replace_text: Option<String>,
}

impl Part for ReplaceAllTextRequest {}


/// The crop properties of an image.
/// 
/// The crop rectangle is represented using fractional offsets from the original
/// content's four edges.
/// 
/// - If the offset is in the interval (0, 1), the corresponding edge of crop
/// rectangle is positioned inside of the image's original bounding rectangle.
/// - If the offset is negative or greater than 1, the corresponding edge of crop
/// rectangle is positioned outside of the image's original bounding rectangle.
/// - If all offsets and rotation angle are 0, the image is not cropped.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CropProperties {
    /// The offset specifies how far inwards the bottom edge of the crop rectangle
    /// is from the bottom edge of the original content as a fraction of the
    /// original content's height.
    #[serde(rename="offsetBottom")]
    pub offset_bottom: Option<f32>,
    /// The clockwise rotation angle of the crop rectangle around its center, in
    /// radians. Rotation is applied after the offsets.
    pub angle: Option<f32>,
    /// The offset specifies how far inwards the left edge of the crop rectangle is
    /// from the left edge of the original content as a fraction of the original
    /// content's width.
    #[serde(rename="offsetLeft")]
    pub offset_left: Option<f32>,
    /// The offset specifies how far inwards the right edge of the crop rectangle
    /// is from the right edge of the original content as a fraction of the
    /// original content's width.
    #[serde(rename="offsetRight")]
    pub offset_right: Option<f32>,
    /// The offset specifies how far inwards the top edge of the crop rectangle is
    /// from the top edge of the original content as a fraction of the original
    /// content's height.
    #[serde(rename="offsetTop")]
    pub offset_top: Option<f32>,
}

impl Part for CropProperties {}


/// A Google Docs document.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create documents](struct.DocumentCreateCall.html) (request|response)
/// * [get documents](struct.DocumentGetCall.html) (response)
/// * [batch update documents](struct.DocumentBatchUpdateCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Document {
    /// Output only. The main body of the document.
    pub body: Option<Body>,
    /// Output only. The style of the document.
    #[serde(rename="documentStyle")]
    pub document_style: Option<DocumentStyle>,
    /// Output only. The suggested changes to the style of the document, keyed by
    /// suggestion ID.
    #[serde(rename="suggestedDocumentStyleChanges")]
    pub suggested_document_style_changes: Option<HashMap<String, SuggestedDocumentStyle>>,
    /// Output only. The suggested changes to the named styles of the document,
    /// keyed by suggestion ID.
    #[serde(rename="suggestedNamedStylesChanges")]
    pub suggested_named_styles_changes: Option<HashMap<String, SuggestedNamedStyles>>,
    /// Output only. The lists in the document, keyed by list ID.
    pub lists: Option<HashMap<String, List>>,
    /// Output only. The footers in the document, keyed by footer ID.
    pub footers: Option<HashMap<String, Footer>>,
    /// Output only. The suggestions view mode applied to the document.
    /// 
    /// Note: When editing a document, changes must be based on a document with
    /// SUGGESTIONS_INLINE.
    #[serde(rename="suggestionsViewMode")]
    pub suggestions_view_mode: Option<String>,
    /// Output only. The positioned objects in the document, keyed by object ID.
    #[serde(rename="positionedObjects")]
    pub positioned_objects: Option<HashMap<String, PositionedObject>>,
    /// Output only. The named ranges in the document, keyed by name.
    #[serde(rename="namedRanges")]
    pub named_ranges: Option<HashMap<String, NamedRanges>>,
    /// Output only. The ID of the document.
    #[serde(rename="documentId")]
    pub document_id: Option<String>,
    /// The title of the document.
    pub title: Option<String>,
    /// Output only. The revision ID of the document. Can be used in update
    /// requests to specify which revision of a document to apply updates to and
    /// how the request should behave if the document has been edited since that
    /// revision. Only populated if the user has edit access to the document.
    /// 
    /// The format of the revision ID may change over time, so it should be treated
    /// opaquely. A returned revision ID is only guaranteed to be valid for 24
    /// hours after it has been returned and cannot be shared across users. If the
    /// revision ID is unchanged between calls, then the document has not changed.
    /// Conversely, a changed ID (for the same document and user) usually means the
    /// document has been updated; however, a changed ID can also be due to
    /// internal factors such as ID format changes.
    #[serde(rename="revisionId")]
    pub revision_id: Option<String>,
    /// Output only. The headers in the document, keyed by header ID.
    pub headers: Option<HashMap<String, Header>>,
    /// Output only. The footnotes in the document, keyed by footnote ID.
    pub footnotes: Option<HashMap<String, Footnote>>,
    /// Output only. The inline objects in the document, keyed by object ID.
    #[serde(rename="inlineObjects")]
    pub inline_objects: Option<HashMap<String, InlineObject>>,
    /// Output only. The named styles of the document.
    #[serde(rename="namedStyles")]
    pub named_styles: Option<NamedStyles>,
}

impl RequestValue for Document {}
impl Resource for Document {}
impl ResponseResult for Document {}


/// A suggestion state of a NamedStyle message.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NamedStyleSuggestionState {
    /// A mask that indicates which of the fields in paragraph style have been changed in this
    /// suggestion.
    #[serde(rename="paragraphStyleSuggestionState")]
    pub paragraph_style_suggestion_state: Option<ParagraphStyleSuggestionState>,
    /// A mask that indicates which of the fields in text style have been changed in this
    /// suggestion.
    #[serde(rename="textStyleSuggestionState")]
    pub text_style_suggestion_state: Option<TextStyleSuggestionState>,
    /// The named style type that this suggestion state corresponds to.
    /// 
    /// This field is provided as a convenience for matching the
    /// NamedStyleSuggestionState with its corresponding NamedStyle.
    #[serde(rename="namedStyleType")]
    pub named_style_type: Option<String>,
}

impl Part for NamedStyleSuggestionState {}


/// A ParagraphElement representing a
/// footnote reference. A footnote reference is the inline content rendered with
/// a number and is used to identify the footnote.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FootnoteReference {
    /// The text style of this FootnoteReference.
    #[serde(rename="textStyle")]
    pub text_style: Option<TextStyle>,
    /// The ID of the footnote that
    /// contains the content of this footnote reference.
    #[serde(rename="footnoteId")]
    pub footnote_id: Option<String>,
    /// The rendered number of this footnote.
    #[serde(rename="footnoteNumber")]
    pub footnote_number: Option<String>,
    /// The suggested deletion IDs. If empty, then there are no suggested deletions
    /// of this content.
    #[serde(rename="suggestedDeletionIds")]
    pub suggested_deletion_ids: Option<Vec<String>>,
    /// The suggested text style changes to this FootnoteReference, keyed by
    /// suggestion ID.
    #[serde(rename="suggestedTextStyleChanges")]
    pub suggested_text_style_changes: Option<HashMap<String, SuggestedTextStyle>>,
    /// The suggested insertion IDs. A FootnoteReference may have multiple insertion IDs if
    /// it is a nested suggested change. If empty, then this is not a suggested
    /// insertion.
    #[serde(rename="suggestedInsertionIds")]
    pub suggested_insertion_ids: Option<Vec<String>>,
}

impl Part for FootnoteReference {}


/// The document body.
/// 
/// The body typically contains the full document contents except for
/// headers, footers
/// and footnotes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Body {
    /// The contents of the body.
    /// 
    /// The indexes for the body's content begin at zero.
    pub content: Option<Vec<StructuralElement>>,
}

impl Part for Body {}


/// Request message for BatchUpdateDocument.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch update documents](struct.DocumentBatchUpdateCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchUpdateDocumentRequest {
    /// A list of updates to apply to the document.
    pub requests: Option<Vec<Request>>,
    /// Provides control over how write requests are executed.
    #[serde(rename="writeControl")]
    pub write_control: Option<WriteControl>,
}

impl RequestValue for BatchUpdateDocumentRequest {}


/// A mask that indicates which of the fields on the base SheetsChartReference have been changed in this
/// suggestion. For any field set to true, there is a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SheetsChartReferenceSuggestionState {
    /// Indicates if there was a suggested change to spreadsheet_id.
    #[serde(rename="spreadsheetIdSuggested")]
    pub spreadsheet_id_suggested: Option<bool>,
    /// Indicates if there was a suggested change to chart_id.
    #[serde(rename="chartIdSuggested")]
    pub chart_id_suggested: Option<bool>,
}

impl Part for SheetsChartReferenceSuggestionState {}


/// Styles that apply to a table row.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableRowStyle {
    /// The minimum height of the row. The row will be rendered in the Docs editor
    /// at a height equal to or greater than this value in order to show all the
    /// content in the row's cells.
    #[serde(rename="minRowHeight")]
    pub min_row_height: Option<Dimension>,
}

impl Part for TableRowStyle {}


/// A border around a paragraph.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ParagraphBorder {
    /// The color of the border.
    pub color: Option<OptionalColor>,
    /// The padding of the border.
    pub padding: Option<Dimension>,
    /// The dash style of the border.
    #[serde(rename="dashStyle")]
    pub dash_style: Option<String>,
    /// The width of the border.
    pub width: Option<Dimension>,
}

impl Part for ParagraphBorder {}


/// A ParagraphElement representing a
/// horizontal line.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HorizontalRule {
    /// The text style of this HorizontalRule.
    /// 
    /// Similar to text content, like text runs and footnote references, the text
    /// style of a horizontal rule can affect content layout as well as the styling
    /// of text inserted adjacent to it.
    #[serde(rename="textStyle")]
    pub text_style: Option<TextStyle>,
    /// The suggested deletion IDs. If empty, then there are no suggested deletions
    /// of this content.
    #[serde(rename="suggestedDeletionIds")]
    pub suggested_deletion_ids: Option<Vec<String>>,
    /// The suggested text style changes to this HorizontalRule, keyed by
    /// suggestion ID.
    #[serde(rename="suggestedTextStyleChanges")]
    pub suggested_text_style_changes: Option<HashMap<String, SuggestedTextStyle>>,
    /// The suggested insertion IDs. A HorizontalRule may have multiple insertion IDs if it
    /// is a nested suggested change. If empty, then this is not a suggested
    /// insertion.
    #[serde(rename="suggestedInsertionIds")]
    pub suggested_insertion_ids: Option<Vec<String>>,
}

impl Part for HorizontalRule {}


/// Deletes a column from a table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteTableColumnRequest {
    /// The reference table cell location from which the column will be deleted.
    /// 
    /// The column this cell spans will be deleted. If this is a merged cell that
    /// spans multiple columns, all columns that the cell spans will be deleted. If
    /// no columns remain in the table after this deletion, the whole table is
    /// deleted.
    #[serde(rename="tableCellLocation")]
    pub table_cell_location: Option<TableCellLocation>,
}

impl Part for DeleteTableColumnRequest {}


/// The shading of a paragraph.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Shading {
    /// The background color of this paragraph shading.
    #[serde(rename="backgroundColor")]
    pub background_color: Option<OptionalColor>,
}

impl Part for Shading {}


/// A StructuralElement describes content that provides structure to the
/// document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StructuralElement {
    /// The zero-based end index of this structural element, exclusive, in UTF-16
    /// code units.
    #[serde(rename="endIndex")]
    pub end_index: Option<i32>,
    /// A section break type of structural element.
    #[serde(rename="sectionBreak")]
    pub section_break: Option<SectionBreak>,
    /// A paragraph type of structural element.
    pub paragraph: Option<Paragraph>,
    /// A table type of structural element.
    pub table: Option<Table>,
    /// The zero-based start index of this structural element, in UTF-16 code
    /// units.
    #[serde(rename="startIndex")]
    pub start_index: Option<i32>,
    /// A table of contents type of structural element.
    #[serde(rename="tableOfContents")]
    pub table_of_contents: Option<TableOfContents>,
}

impl Part for StructuralElement {}


/// A suggested change to a TableCellStyle.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SuggestedTableCellStyle {
    /// A TableCellStyle that only includes
    /// the changes made in this suggestion. This can be used along with the
    /// table_cell_style_suggestion_state
    /// to see which fields have changed and their new values.
    #[serde(rename="tableCellStyle")]
    pub table_cell_style: Option<TableCellStyle>,
    /// A mask that indicates which of the fields on the base TableCellStyle have been changed in this suggestion.
    #[serde(rename="tableCellStyleSuggestionState")]
    pub table_cell_style_suggestion_state: Option<TableCellStyleSuggestionState>,
}

impl Part for SuggestedTableCellStyle {}


/// A mask that indicates which of the fields on the base ListProperties have been changed in this suggestion.
/// For any field set to true, there is a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListPropertiesSuggestionState {
    /// A mask that indicates which of the fields on the corresponding
    /// NestingLevel in nesting_levels have been changed in
    /// this suggestion.
    /// 
    /// The nesting level suggestion states are returned in ascending order of the
    /// nesting level with the least nested returned first.
    #[serde(rename="nestingLevelsSuggestionStates")]
    pub nesting_levels_suggestion_states: Option<Vec<NestingLevelSuggestionState>>,
}

impl Part for ListPropertiesSuggestionState {}


/// A StructuralElement representing a
/// paragraph. A paragraph is a range of content that is terminated with a
/// newline character.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Paragraph {
    /// The content of the paragraph broken down into its component parts.
    pub elements: Option<Vec<ParagraphElement>>,
    /// The IDs of the positioned objects that are suggested to be attached to this
    /// paragraph, keyed by suggestion ID.
    #[serde(rename="suggestedPositionedObjectIds")]
    pub suggested_positioned_object_ids: Option<HashMap<String, ObjectReferences>>,
    /// The bullet for this paragraph. If not present, the paragraph does not
    /// belong to a list.
    pub bullet: Option<Bullet>,
    /// The suggested changes to this paragraph's bullet.
    #[serde(rename="suggestedBulletChanges")]
    pub suggested_bullet_changes: Option<HashMap<String, SuggestedBullet>>,
    /// The IDs of the positioned objects tethered to this paragraph.
    #[serde(rename="positionedObjectIds")]
    pub positioned_object_ids: Option<Vec<String>>,
    /// The suggested paragraph style changes to this paragraph, keyed by
    /// suggestion ID.
    #[serde(rename="suggestedParagraphStyleChanges")]
    pub suggested_paragraph_style_changes: Option<HashMap<String, SuggestedParagraphStyle>>,
    /// The style of this paragraph.
    #[serde(rename="paragraphStyle")]
    pub paragraph_style: Option<ParagraphStyle>,
}

impl Part for Paragraph {}


/// Inserts an empty column into a table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InsertTableColumnRequest {
    /// Whether to insert new column to the right of the reference cell location.
    /// 
    /// - `True`: insert to the right.
    /// - `False`: insert to the left.
    #[serde(rename="insertRight")]
    pub insert_right: Option<bool>,
    /// The reference table cell location from which columns will be inserted.
    /// 
    /// A new column will be inserted to the left (or right) of the column where
    /// the reference cell is. If the reference cell is a merged cell, a new
    /// column will be inserted to the left (or right) of the merged cell.
    #[serde(rename="tableCellLocation")]
    pub table_cell_location: Option<TableCellLocation>,
}

impl Part for InsertTableColumnRequest {}


/// A reference to the external linked source content.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LinkedContentReference {
    /// A reference to the linked chart.
    #[serde(rename="sheetsChartReference")]
    pub sheets_chart_reference: Option<SheetsChartReference>,
}

impl Part for LinkedContentReference {}


/// The named styles. Paragraphs in the document can inherit their
/// TextStyle and
/// ParagraphStyle from these named styles.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NamedStyles {
    /// The named styles.
    /// 
    /// There is an entry for each of the possible named style types.
    pub styles: Option<Vec<NamedStyle>>,
}

impl Part for NamedStyles {}


/// The styling that applies to a section.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SectionStyle {
    /// The style of column separators.
    /// 
    /// This style can be set even when there is one column in the section.
    #[serde(rename="columnSeparatorStyle")]
    pub column_separator_style: Option<String>,
    /// The section's columns properties.
    /// 
    /// If empty, the section contains one column with the default properties in
    /// the Docs editor.
    #[serde(rename="columnProperties")]
    pub column_properties: Option<Vec<SectionColumnProperties>>,
    /// The content direction of this section. If unset, the value defaults to
    /// LEFT_TO_RIGHT.
    #[serde(rename="contentDirection")]
    pub content_direction: Option<String>,
}

impl Part for SectionStyle {}


/// A mask that indicates which of the fields on the base
/// Bullet have been changed in this suggestion.
/// For any field set to true, there is a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BulletSuggestionState {
    /// Indicates if there was a suggested change to the
    /// nesting_level.
    #[serde(rename="nestingLevelSuggested")]
    pub nesting_level_suggested: Option<bool>,
    /// A mask that indicates which of the fields in text style have been changed in this
    /// suggestion.
    #[serde(rename="textStyleSuggestionState")]
    pub text_style_suggestion_state: Option<TextStyleSuggestionState>,
    /// Indicates if there was a suggested change to the
    /// list_id.
    #[serde(rename="listIdSuggested")]
    pub list_id_suggested: Option<bool>,
}

impl Part for BulletSuggestionState {}


/// The properties of an embedded drawing.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EmbeddedDrawingProperties { _never_set: Option<bool> }

impl Part for EmbeddedDrawingProperties {}


/// A mask that indicates which of the fields on the base EmbeddedObject have been changed in this suggestion.
/// For any field set to true, there is a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EmbeddedObjectSuggestionState {
    /// A mask that indicates which of the fields in linked_content_reference have been
    /// changed in this suggestion.
    #[serde(rename="linkedContentReferenceSuggestionState")]
    pub linked_content_reference_suggestion_state: Option<LinkedContentReferenceSuggestionState>,
    /// A mask that indicates which of the fields in size have been changed in this
    /// suggestion.
    #[serde(rename="sizeSuggestionState")]
    pub size_suggestion_state: Option<SizeSuggestionState>,
    /// A mask that indicates which of the fields in embedded_drawing_properties have been
    /// changed in this suggestion.
    #[serde(rename="embeddedDrawingPropertiesSuggestionState")]
    pub embedded_drawing_properties_suggestion_state: Option<EmbeddedDrawingPropertiesSuggestionState>,
    /// Indicates if there was a suggested change to margin_right.
    #[serde(rename="marginRightSuggested")]
    pub margin_right_suggested: Option<bool>,
    /// A mask that indicates which of the fields in image_properties have been changed in
    /// this suggestion.
    #[serde(rename="imagePropertiesSuggestionState")]
    pub image_properties_suggestion_state: Option<ImagePropertiesSuggestionState>,
    /// Indicates if there was a suggested change to description.
    #[serde(rename="descriptionSuggested")]
    pub description_suggested: Option<bool>,
    /// Indicates if there was a suggested change to margin_left.
    #[serde(rename="marginLeftSuggested")]
    pub margin_left_suggested: Option<bool>,
    /// Indicates if there was a suggested change to margin_top.
    #[serde(rename="marginTopSuggested")]
    pub margin_top_suggested: Option<bool>,
    /// Indicates if there was a suggested change to margin_bottom.
    #[serde(rename="marginBottomSuggested")]
    pub margin_bottom_suggested: Option<bool>,
    /// Indicates if there was a suggested change to title.
    #[serde(rename="titleSuggested")]
    pub title_suggested: Option<bool>,
    /// A mask that indicates which of the fields in embedded_object_border have been
    /// changed in this suggestion.
    #[serde(rename="embeddedObjectBorderSuggestionState")]
    pub embedded_object_border_suggestion_state: Option<EmbeddedObjectBorderSuggestionState>,
}

impl Part for EmbeddedObjectSuggestionState {}


/// The result of replacing text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReplaceAllTextResponse {
    /// The number of occurrences changed by replacing all text.
    #[serde(rename="occurrencesChanged")]
    pub occurrences_changed: Option<i32>,
}

impl Part for ReplaceAllTextResponse {}


/// Inserts a table at the specified location.
/// 
/// A newline character will be inserted before the inserted table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InsertTableRequest {
    /// Inserts the table at the end of the given header, footer or document
    /// body. A newline character will be inserted before the inserted table.
    /// 
    /// Tables cannot be inserted inside a footnote.
    #[serde(rename="endOfSegmentLocation")]
    pub end_of_segment_location: Option<EndOfSegmentLocation>,
    /// The number of rows in the table.
    pub rows: Option<i32>,
    /// Inserts the table at a specific model index.
    /// 
    /// A newline character will be inserted before the inserted table, therefore
    /// the table start index will be at the specified location index + 1.
    /// 
    /// The table must be inserted inside the bounds of an existing
    /// Paragraph. For instance, it cannot be
    /// inserted at a table's start index (i.e. between an existing table and its
    /// preceding paragraph).
    /// 
    /// Tables cannot be inserted inside a footnote or equation.
    pub location: Option<Location>,
    /// The number of columns in the table.
    pub columns: Option<i32>,
}

impl Part for InsertTableRequest {}


/// A ParagraphElement representing a
/// spot in the text that is dynamically replaced with content that can change
/// over time, like a page number.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AutoText {
    /// The text style of this AutoText.
    #[serde(rename="textStyle")]
    pub text_style: Option<TextStyle>,
    /// The suggested deletion IDs. If empty, then there are no suggested deletions
    /// of this content.
    #[serde(rename="suggestedDeletionIds")]
    pub suggested_deletion_ids: Option<Vec<String>>,
    /// The suggested text style changes to this AutoText, keyed by suggestion ID.
    #[serde(rename="suggestedTextStyleChanges")]
    pub suggested_text_style_changes: Option<HashMap<String, SuggestedTextStyle>>,
    /// The type of this auto text.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The suggested insertion IDs. An AutoText
    /// may have multiple insertion IDs if it is a nested suggested change. If
    /// empty, then this is not a suggested insertion.
    #[serde(rename="suggestedInsertionIds")]
    pub suggested_insertion_ids: Option<Vec<String>>,
}

impl Part for AutoText {}


/// A ParagraphElement that contains
/// an InlineObject.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InlineObjectElement {
    /// The text style of this InlineObjectElement.
    /// 
    /// Similar to text content, like text runs and footnote references, the text
    /// style of an inline object element can affect content layout as well as the
    /// styling of text inserted adjacent to it.
    #[serde(rename="textStyle")]
    pub text_style: Option<TextStyle>,
    /// The suggested deletion IDs. If empty, then there are no suggested deletions
    /// of this content.
    #[serde(rename="suggestedDeletionIds")]
    pub suggested_deletion_ids: Option<Vec<String>>,
    /// The ID of the InlineObject this
    /// element contains.
    #[serde(rename="inlineObjectId")]
    pub inline_object_id: Option<String>,
    /// The suggested text style changes to this InlineObject, keyed by suggestion
    /// ID.
    #[serde(rename="suggestedTextStyleChanges")]
    pub suggested_text_style_changes: Option<HashMap<String, SuggestedTextStyle>>,
    /// The suggested insertion IDs. An InlineObjectElement may have multiple insertion IDs
    /// if it is a nested suggested change. If empty, then this is not a suggested
    /// insertion.
    #[serde(rename="suggestedInsertionIds")]
    pub suggested_insertion_ids: Option<Vec<String>>,
}

impl Part for InlineObjectElement {}


/// Inserts text at the specified location.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InsertTextRequest {
    /// The text to be inserted.
    /// 
    /// Inserting a newline character will implicitly create a new
    /// Paragraph at that index.
    /// The paragraph style of the new paragraph will be copied from the paragraph
    /// at the current insertion index, including lists and bullets.
    /// 
    /// Text styles for inserted text will be determined automatically, generally
    /// preserving the styling of neighboring text. In most cases, the text style
    /// for the inserted text will match the text immediately before the insertion
    /// index.
    /// 
    /// Some control characters (U+0000-U+0008, U+000C-U+001F) and characters
    /// from the Unicode Basic Multilingual Plane Private Use Area (U+E000-U+F8FF)
    /// will be stripped out of the inserted text.
    pub text: Option<String>,
    /// Inserts the text at the end of a header, footer, footnote or
    /// the document body.
    #[serde(rename="endOfSegmentLocation")]
    pub end_of_segment_location: Option<EndOfSegmentLocation>,
    /// Inserts the text at a specific index in the document.
    /// 
    /// Text must be inserted inside the bounds of an existing
    /// Paragraph. For instance, text cannot be
    /// inserted at a table's start index (i.e. between the table and its
    /// preceding paragraph). The text must be inserted in the preceding
    /// paragraph.
    pub location: Option<Location>,
}

impl Part for InsertTextRequest {}


/// Represents the styling that can be applied to text.
/// 
/// Inherited text styles are represented as unset fields in this message. A
/// text style's parent depends on where the text style is defined:
/// 
/// * The TextStyle of text in a Paragraph
///   inherits from the paragraph's corresponding named style type.
/// * The TextStyle on a named style
///   inherits from the normal text named style.
/// * The TextStyle of the normal text named style inherits
///   from the default text style in the Docs editor.
/// * The TextStyle on a Paragraph element
///   that is contained in a table may inherit its text style from the table
///   style.
/// 
/// If the text style does not inherit from a parent, unsetting fields will
/// revert the style to a value matching the defaults in the Docs editor.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TextStyle {
    /// The foreground color of the text. If set, the color is either an RGB color
    /// or transparent, depending on the `color` field.
    #[serde(rename="foregroundColor")]
    pub foreground_color: Option<OptionalColor>,
    /// Whether or not the text is rendered as bold.
    pub bold: Option<bool>,
    /// The text's vertical offset from its normal position.
    /// 
    /// Text with `SUPERSCRIPT` or `SUBSCRIPT` baseline offsets is automatically
    /// rendered in a smaller font size, computed based on the `font_size` field.
    /// The `font_size` itself is not affected by changes in this field.
    #[serde(rename="baselineOffset")]
    pub baseline_offset: Option<String>,
    /// Whether or not the text is struck through.
    pub strikethrough: Option<bool>,
    /// The font family and rendered weight of the text.
    /// 
    /// If an update request specifies values for both `weighted_font_family` and
    /// `bold`, the `weighted_font_family` is applied first, then `bold`.
    /// 
    /// If `weighted_font_family#weight` is not set, it defaults to `400`.
    /// 
    /// If `weighted_font_family` is set, then `weighted_font_family#font_family`
    /// must also be set with a non-empty value. Otherwise, a 400 bad request error
    /// is returned.
    #[serde(rename="weightedFontFamily")]
    pub weighted_font_family: Option<WeightedFontFamily>,
    /// Whether or not the text is in small capital letters.
    #[serde(rename="smallCaps")]
    pub small_caps: Option<bool>,
    /// The size of the text's font.
    #[serde(rename="fontSize")]
    pub font_size: Option<Dimension>,
    /// Whether or not the text is italicized.
    pub italic: Option<bool>,
    /// The hyperlink destination of the text. If unset, there is no link. Links
    /// are not inherited from parent text.
    /// 
    /// Changing the link in an update request causes some other changes to the
    /// text style of the range:
    /// 
    /// * When setting a link, the text foreground color will be updated to the
    ///   default link color and the text will be underlined. If these fields are
    ///   modified in the same request, those values will be used instead of the
    ///   link defaults.
    /// * Setting a link on a text range that overlaps with an existing link will
    ///   also update the existing link to point to the new URL.
    /// * Links are not settable on newline characters. As a result, setting a link
    ///   on a text range that crosses a paragraph boundary, such as `"ABC\n123"`,
    ///   will separate the newline character(s) into their own text runs. The
    ///   link will be applied separately to the runs before and after the newline.
    /// * Removing a link will update the text style of the range to match the
    ///   style of the preceding text (or the default text styles if the preceding
    ///   text is another link) unless different styles are being set in the same
    ///   request.
    pub link: Option<Link>,
    /// Whether or not the text is underlined.
    pub underline: Option<bool>,
    /// The background color of the text. If set, the color is either an RGB color
    /// or transparent, depending on the `color` field.
    #[serde(rename="backgroundColor")]
    pub background_color: Option<OptionalColor>,
}

impl Part for TextStyle {}


/// The properties of a column in a table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableColumnProperties {
    /// The width of the column. Set when the column's `width_type` is
    /// FIXED_WIDTH.
    pub width: Option<Dimension>,
    /// The width type of the column.
    #[serde(rename="widthType")]
    pub width_type: Option<String>,
}

impl Part for TableColumnProperties {}


/// A mask that indicates which of the fields on the base Size have been changed in this suggestion.
/// For any field set to true, the Size has
/// a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SizeSuggestionState {
    /// Indicates if there was a suggested change to width.
    #[serde(rename="widthSuggested")]
    pub width_suggested: Option<bool>,
    /// Indicates if there was a suggested change to height.
    #[serde(rename="heightSuggested")]
    pub height_suggested: Option<bool>,
}

impl Part for SizeSuggestionState {}


/// A particular location in the document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    /// The zero-based index, in UTF-16 code units.
    /// 
    /// The index is relative to the beginning of the segment specified by
    /// segment_id.
    pub index: Option<i32>,
    /// The ID of the header, footer or footnote the location is in. An empty
    /// segment ID signifies the document's body.
    #[serde(rename="segmentId")]
    pub segment_id: Option<String>,
}

impl Part for Location {}


/// Provides control over how write requests are executed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WriteControl {
    /// The revision ID of the
    /// document that the write request will be applied to. If this is not the
    /// latest revision of the document, the request will not be processed and
    /// will return a 400 bad request error.
    /// 
    /// When a required revision ID is returned in a response, it indicates the
    /// revision ID of the document after the request was applied.
    #[serde(rename="requiredRevisionId")]
    pub required_revision_id: Option<String>,
    /// The target revision ID of the
    /// document that the write request will be applied to.
    /// 
    /// If collaborator changes have occurred after the document was read using
    /// the API, the changes produced by this write request will be transformed
    /// against the collaborator changes. This results in a new revision of the
    /// document which incorporates both the changes in the request and the
    /// collaborator changes, and the Docs server will resolve conflicting
    /// changes. When using `target_revision_id`, the API client can be thought
    /// of as another collaborator of the document.
    /// 
    /// The target revision ID may only be used to write to recent versions of a
    /// document. If the target revision is too far behind the latest revision,
    /// the request will not be processed and will return a 400 bad request error
    /// and the request should be retried after reading the latest version of the
    /// document. In most cases a `revision_id` will remain valid for use as a
    /// target revision for several minutes after it is read, but for
    /// frequently-edited documents this window may be shorter.
    #[serde(rename="targetRevisionId")]
    pub target_revision_id: Option<String>,
}

impl Part for WriteControl {}


/// The properties of an image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImageProperties {
    /// The transparency effect of the image. The value should be in the interval
    /// [0.0, 1.0], where 0 means no effect and 1 means completely transparent.
    pub transparency: Option<f32>,
    /// The clockwise rotation angle of the image, in radians.
    pub angle: Option<f32>,
    /// A URI to the image with a default lifetime of 30 minutes.
    /// This URI is tagged with the account of the requester. Anyone with the URI
    /// effectively accesses the image as the original requester. Access to the
    /// image may be lost if the document's sharing settings change.
    #[serde(rename="contentUri")]
    pub content_uri: Option<String>,
    /// The source URI is the URI used to insert the image. The source URI can be
    /// empty.
    #[serde(rename="sourceUri")]
    pub source_uri: Option<String>,
    /// The brightness effect of the image. The value should be in the interval
    /// [-1.0, 1.0], where 0 means no effect.
    pub brightness: Option<f32>,
    /// The crop properties of the image.
    #[serde(rename="cropProperties")]
    pub crop_properties: Option<CropProperties>,
    /// The contrast effect of the image. The value should be in the interval
    /// [-1.0, 1.0], where 0 means no effect.
    pub contrast: Option<f32>,
}

impl Part for ImageProperties {}


/// A document header.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Header {
    /// The contents of the header.
    /// 
    /// The indexes for a header's content begin at zero.
    pub content: Option<Vec<StructuralElement>>,
    /// The ID of the header.
    #[serde(rename="headerId")]
    pub header_id: Option<String>,
}

impl Part for Header {}


/// A reference to another portion of a document or an external URL resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Link {
    /// The ID of a heading in this document.
    #[serde(rename="headingId")]
    pub heading_id: Option<String>,
    /// An external URL.
    pub url: Option<String>,
    /// The ID of a bookmark in this document.
    #[serde(rename="bookmarkId")]
    pub bookmark_id: Option<String>,
}

impl Part for Link {}


/// A suggested change to a Bullet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SuggestedBullet {
    /// A Bullet that only includes the changes made
    /// in this suggestion. This can be used along with the
    /// bullet_suggestion_state to see which
    /// fields have changed and their new values.
    pub bullet: Option<Bullet>,
    /// A mask that indicates which of the fields on the base
    /// Bullet have been changed in this suggestion.
    #[serde(rename="bulletSuggestionState")]
    pub bullet_suggestion_state: Option<BulletSuggestionState>,
}

impl Part for SuggestedBullet {}


/// A ParagraphElement that represents a
/// run of text that all has the same styling.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TextRun {
    /// The text of this run.
    /// 
    /// Any non-text elements in the run are replaced with the Unicode character
    /// U+E907.
    pub content: Option<String>,
    /// The text style of this run.
    #[serde(rename="textStyle")]
    pub text_style: Option<TextStyle>,
    /// The suggested deletion IDs. If empty, then there are no suggested deletions
    /// of this content.
    #[serde(rename="suggestedDeletionIds")]
    pub suggested_deletion_ids: Option<Vec<String>>,
    /// The suggested text style changes to this run, keyed by suggestion ID.
    #[serde(rename="suggestedTextStyleChanges")]
    pub suggested_text_style_changes: Option<HashMap<String, SuggestedTextStyle>>,
    /// The suggested insertion IDs. A TextRun may
    /// have multiple insertion IDs if it is a nested suggested change. If empty,
    /// then this is not a suggested insertion.
    #[serde(rename="suggestedInsertionIds")]
    pub suggested_insertion_ids: Option<Vec<String>>,
}

impl Part for TextRun {}


/// A mask that indicates which of the fields on the base
/// PositionedObjectPositioning have been changed in this
/// suggestion. For any field set to true, there is a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PositionedObjectPositioningSuggestionState {
    /// Indicates if there was a suggested change to layout.
    #[serde(rename="layoutSuggested")]
    pub layout_suggested: Option<bool>,
    /// Indicates if there was a suggested change to top_offset.
    #[serde(rename="topOffsetSuggested")]
    pub top_offset_suggested: Option<bool>,
    /// Indicates if there was a suggested change to left_offset.
    #[serde(rename="leftOffsetSuggested")]
    pub left_offset_suggested: Option<bool>,
}

impl Part for PositionedObjectPositioningSuggestionState {}


/// A color that can either be fully opaque or fully transparent.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OptionalColor {
    /// If set, this will be used as an opaque color. If unset, this represents
    /// a transparent color.
    pub color: Option<Color>,
}

impl Part for OptionalColor {}


/// A border around a table cell.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableCellBorder {
    /// The color of the border.
    pub color: Option<OptionalColor>,
    /// The width of the border.
    pub width: Option<Dimension>,
    /// The dash style of the border.
    #[serde(rename="dashStyle")]
    pub dash_style: Option<String>,
}

impl Part for TableCellBorder {}


/// Update the styling of text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateTextStyleRequest {
    /// The fields that should be updated.
    /// 
    /// At least one field must be specified. The root `text_style` is implied and
    /// should not be specified. A single `"*"` can be used as short-hand for
    /// listing every field.
    /// 
    /// For example, to update the text style to bold, set `fields` to `"bold"`.
    /// 
    /// To reset a property to its default value, include its field name in the
    /// field mask but leave the field itself unset.
    pub fields: Option<String>,
    /// The range of text to style.
    /// 
    /// The range may be extended to include adjacent newlines.
    /// 
    /// If the range fully contains a paragraph belonging to a list, the
    /// paragraph's bullet is also updated with the matching text style.
    pub range: Option<Range>,
    /// The styles to set on the text.
    /// 
    /// If the value for a particular style matches that of the parent, that style
    /// will be set to inherit.
    /// 
    /// Certain text style changes may cause other changes in order to to mirror
    /// the behavior of the Docs editor. See the documentation of
    /// TextStyle for more information.
    #[serde(rename="textStyle")]
    pub text_style: Option<TextStyle>,
}

impl Part for UpdateTextStyleRequest {}


/// Contains properties describing the look and feel of a list bullet at a given
/// level of nesting.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NestingLevel {
    /// The text style of bullets at this level of nesting.
    #[serde(rename="textStyle")]
    pub text_style: Option<TextStyle>,
    /// The amount of indentation for paragraphs at this level of nesting. Applied
    /// to the side that corresponds to the start of the text, based on the
    /// paragraph's content direction.
    #[serde(rename="indentStart")]
    pub indent_start: Option<Dimension>,
    /// The type of glyph used by bullets when paragraphs at this level of
    /// nesting are ordered.
    /// 
    /// The glyph type determines the type of glyph used to replace placeholders
    /// within the glyph_format
    /// when paragraphs at this level of nesting are ordered. For example, if the
    /// nesting level is 0, the glyph_format is `%0.` and the glyph
    /// type is DECIMAL,
    /// then the rendered glyph would replace the placeholder `%0` in the glyph
    /// format with a number corresponding to list item's order within the list.
    #[serde(rename="glyphType")]
    pub glyph_type: Option<String>,
    /// The format string used by bullets at this level of nesting.
    /// 
    /// The glyph format contains one or more placeholders, and these placeholder
    /// are replaced with the appropriate values depending on the glyph_type or glyph_symbol. The placeholders follow
    /// the pattern `%[nesting_level]`. Furthermore, placeholders can have prefixes
    /// and suffixes. Thus, the glyph format follows the pattern
    /// `<prefix>%[nesting_level]<suffix>`. Note that the prefix and suffix are
    /// optional and can be arbitrary strings.
    /// 
    /// For example, the glyph format `%0.` indicates that the rendered glyph will
    /// replace the placeholder with the corresponding glyph for nesting level 0
    /// followed by a period as the suffix. So a list with a glyph type of
    /// UPPER_ALPHA and
    /// glyph format `%0.` at nesting level 0 will result in a list with rendered
    /// glyphs
    /// <p>`A.`
    /// <p>`B.`
    /// <p>`C.`
    /// 
    /// The glyph format can contain placeholders for the current nesting level as
    /// well as placeholders for parent nesting levels. For example, a
    /// list can have a glyph format of `%0.` at nesting level 0 and a
    /// glyph format of `%0.%1.` at nesting level 1. Assuming both nesting levels
    /// have DECIMAL glyph
    /// types, this would result in a list with rendered glyphs
    /// <p>`1.`
    /// <p>`2.`
    /// <p>`  2.1.`
    /// <p>`  2.2.`
    /// <p>`3.`
    /// 
    /// For nesting levels that are ordered, the string that replaces a placeholder
    /// in the glyph format for a particular paragraph depends on the paragraph's
    /// order within the list.
    #[serde(rename="glyphFormat")]
    pub glyph_format: Option<String>,
    /// A custom glyph symbol used by bullets when paragraphs at this level of
    /// nesting are unordered.
    /// 
    /// The glyph symbol replaces placeholders within the glyph_format. For example, if the
    /// glyph_symbol is the solid circle corresponding to Unicode U+25cf code
    /// point and the glyph_format is `%0`, the rendered
    /// glyph would be the solid circle.
    #[serde(rename="glyphSymbol")]
    pub glyph_symbol: Option<String>,
    /// The alignment of the bullet within the space allotted for rendering the
    /// bullet.
    #[serde(rename="bulletAlignment")]
    pub bullet_alignment: Option<String>,
    /// The number of the first list item at this nesting level.
    /// 
    /// A value of 0 is treated as a value of 1 for lettered lists and roman
    /// numeraled lists, i.e. for values of both 0 and 1, lettered and roman
    /// numeraled lists will begin at `a` and `i` respectively.
    /// 
    /// This value is ignored for nesting levels with unordered glyphs.
    #[serde(rename="startNumber")]
    pub start_number: Option<i32>,
    /// The amount of indentation for the first line of paragraphs at this level of
    /// nesting.
    #[serde(rename="indentFirstLine")]
    pub indent_first_line: Option<Dimension>,
}

impl Part for NestingLevel {}


/// A criteria that matches a specific string of text in the document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubstringMatchCriteria {
    /// The text to search for in the document.
    pub text: Option<String>,
    /// Indicates whether the search should respect case:
    /// 
    /// - `True`: the search is case sensitive.
    /// - `False`: the search is case insensitive.
    #[serde(rename="matchCase")]
    pub match_case: Option<bool>,
}

impl Part for SubstringMatchCriteria {}


/// A mask that indicates which of the fields on the base ImageProperties have been changed in this suggestion.
/// For any field set to true, there is a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImagePropertiesSuggestionState {
    /// A mask that indicates which of the fields in crop_properties have been changed in
    /// this suggestion.
    #[serde(rename="cropPropertiesSuggestionState")]
    pub crop_properties_suggestion_state: Option<CropPropertiesSuggestionState>,
    /// Indicates if there was a suggested change to contrast.
    #[serde(rename="contrastSuggested")]
    pub contrast_suggested: Option<bool>,
    /// Indicates if there was a suggested change to angle.
    #[serde(rename="angleSuggested")]
    pub angle_suggested: Option<bool>,
    /// Indicates if there was a suggested change to brightness.
    #[serde(rename="brightnessSuggested")]
    pub brightness_suggested: Option<bool>,
    /// Indicates if there was a suggested change to transparency.
    #[serde(rename="transparencySuggested")]
    pub transparency_suggested: Option<bool>,
    /// Indicates if there was a suggested change to source_uri.
    #[serde(rename="sourceUriSuggested")]
    pub source_uri_suggested: Option<bool>,
    /// Indicates if there was a suggested change to
    /// content_uri.
    #[serde(rename="contentUriSuggested")]
    pub content_uri_suggested: Option<bool>,
}

impl Part for ImagePropertiesSuggestionState {}


/// A ParagraphElement representing an
/// equation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Equation {
    /// The suggested deletion IDs. If empty, then there are no suggested deletions
    /// of this content.
    #[serde(rename="suggestedDeletionIds")]
    pub suggested_deletion_ids: Option<Vec<String>>,
    /// The suggested insertion IDs. A Equation
    /// may have multiple insertion IDs if it is a nested suggested change. If
    /// empty, then this is not a suggested insertion.
    #[serde(rename="suggestedInsertionIds")]
    pub suggested_insertion_ids: Option<Vec<String>>,
}

impl Part for Equation {}


/// Updates the
/// TableColumnProperties of columns
/// in a table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateTableColumnPropertiesRequest {
    /// The list of zero-based column indices whose property should be updated. If
    /// no indices are specified, all columns will be updated.
    #[serde(rename="columnIndices")]
    pub column_indices: Option<Vec<i32>>,
    /// The fields that should be updated.
    /// 
    /// At least one field must be specified. The root `tableColumnProperties` is
    /// implied and should not be specified. A single `"*"` can be used as
    /// short-hand for listing every field.
    /// 
    /// For example to update the column width, set `fields` to `"width"`.
    pub fields: Option<String>,
    /// The table column properties to update.
    /// 
    /// If the value of `table_column_properties#width` is less than 5 points
    /// (5/72 inch), a 400 bad request error is returned.
    #[serde(rename="tableColumnProperties")]
    pub table_column_properties: Option<TableColumnProperties>,
    /// The location where the table starts in the document.
    #[serde(rename="tableStartLocation")]
    pub table_start_location: Option<Location>,
}

impl Part for UpdateTableColumnPropertiesRequest {}


/// Location at the end of a body, header, footer or footnote. The location is
/// immediately before the last newline in the document segment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EndOfSegmentLocation {
    /// The ID of the header, footer or footnote the location is in. An empty
    /// segment ID signifies the document's body.
    #[serde(rename="segmentId")]
    pub segment_id: Option<String>,
}

impl Part for EndOfSegmentLocation {}


/// A single update to apply to a document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Request {
    /// Inserts text at the specified location.
    #[serde(rename="insertText")]
    pub insert_text: Option<InsertTextRequest>,
    /// Creates bullets for paragraphs.
    #[serde(rename="createParagraphBullets")]
    pub create_paragraph_bullets: Option<CreateParagraphBulletsRequest>,
    /// Updates the row style in a table.
    #[serde(rename="updateTableRowStyle")]
    pub update_table_row_style: Option<UpdateTableRowStyleRequest>,
    /// Updates the paragraph style at the specified range.
    #[serde(rename="updateParagraphStyle")]
    pub update_paragraph_style: Option<UpdateParagraphStyleRequest>,
    /// Replaces all instances of the specified text.
    #[serde(rename="replaceAllText")]
    pub replace_all_text: Option<ReplaceAllTextRequest>,
    /// Inserts an empty column into a table.
    #[serde(rename="insertTableColumn")]
    pub insert_table_column: Option<InsertTableColumnRequest>,
    /// Deletes a row from a table.
    #[serde(rename="deleteTableRow")]
    pub delete_table_row: Option<DeleteTableRowRequest>,
    /// Inserts a page break at the specified location.
    #[serde(rename="insertPageBreak")]
    pub insert_page_break: Option<InsertPageBreakRequest>,
    /// Creates a named range.
    #[serde(rename="createNamedRange")]
    pub create_named_range: Option<CreateNamedRangeRequest>,
    /// Updates the properties of columns in a table.
    #[serde(rename="updateTableColumnProperties")]
    pub update_table_column_properties: Option<UpdateTableColumnPropertiesRequest>,
    /// Deletes content from the document.
    #[serde(rename="deleteContentRange")]
    pub delete_content_range: Option<DeleteContentRangeRequest>,
    /// Deletes bullets from paragraphs.
    #[serde(rename="deleteParagraphBullets")]
    pub delete_paragraph_bullets: Option<DeleteParagraphBulletsRequest>,
    /// Deletes a named range.
    #[serde(rename="deleteNamedRange")]
    pub delete_named_range: Option<DeleteNamedRangeRequest>,
    /// Inserts a table at the specified location.
    #[serde(rename="insertTable")]
    pub insert_table: Option<InsertTableRequest>,
    /// Inserts an empty row into a table.
    #[serde(rename="insertTableRow")]
    pub insert_table_row: Option<InsertTableRowRequest>,
    /// Updates the text style at the specified range.
    #[serde(rename="updateTextStyle")]
    pub update_text_style: Option<UpdateTextStyleRequest>,
    /// Deletes a positioned object from the document.
    #[serde(rename="deletePositionedObject")]
    pub delete_positioned_object: Option<DeletePositionedObjectRequest>,
    /// Deletes a column from a table.
    #[serde(rename="deleteTableColumn")]
    pub delete_table_column: Option<DeleteTableColumnRequest>,
    /// Inserts an inline image at the specified location.
    #[serde(rename="insertInlineImage")]
    pub insert_inline_image: Option<InsertInlineImageRequest>,
}

impl Part for Request {}


/// The result of inserting an inline image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InsertInlineImageResponse {
    /// The ID of the created InlineObject.
    #[serde(rename="objectId")]
    pub object_id: Option<String>,
}

impl Part for InsertInlineImageResponse {}


/// A suggested change to ListProperties.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SuggestedListProperties {
    /// A ListProperties that only includes
    /// the changes made in this suggestion. This can be used along with the
    /// list_properties_suggestion_state
    /// to see which fields have changed and their new values.
    #[serde(rename="listProperties")]
    pub list_properties: Option<ListProperties>,
    /// A mask that indicates which of the fields on the base ListProperties have been changed in this suggestion.
    #[serde(rename="listPropertiesSuggestionState")]
    pub list_properties_suggestion_state: Option<ListPropertiesSuggestionState>,
}

impl Part for SuggestedListProperties {}


/// A magnitude in a single direction in the specified units.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Dimension {
    /// The magnitude.
    pub magnitude: Option<f64>,
    /// The units for magnitude.
    pub unit: Option<String>,
}

impl Part for Dimension {}


/// A mask that indicates which of the fields on the base
/// LinkedContentReference have
/// been changed in this suggestion. For any field set to true, there is a new
/// suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LinkedContentReferenceSuggestionState {
    /// A mask that indicates which of the fields in sheets_chart_reference have
    /// been changed in this suggestion.
    #[serde(rename="sheetsChartReferenceSuggestionState")]
    pub sheets_chart_reference_suggestion_state: Option<SheetsChartReferenceSuggestionState>,
}

impl Part for LinkedContentReferenceSuggestionState {}


/// The style of the document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DocumentStyle {
    /// The ID of the default footer. If not set, there is no default footer.
    #[serde(rename="defaultFooterId")]
    pub default_footer_id: Option<String>,
    /// The ID of the footer used only for even pages. The value of
    /// use_even_page_header_footer determines
    /// whether to use the default_footer_id or this value for the
    /// footer on even pages. If not set, there is no even page footer.
    #[serde(rename="evenPageFooterId")]
    pub even_page_footer_id: Option<String>,
    /// The ID of the footer used only for the first page. If not set then
    /// a unique footer for the first page does not exist. The value of
    /// use_first_page_header_footer determines
    /// whether to use the default_footer_id or this value for the
    /// footer on the first page. If not set, there is no first page footer.
    #[serde(rename="firstPageFooterId")]
    pub first_page_footer_id: Option<String>,
    /// The size of a page in the document.
    #[serde(rename="pageSize")]
    pub page_size: Option<Size>,
    /// The ID of the default header. If not set, there is no default header.
    #[serde(rename="defaultHeaderId")]
    pub default_header_id: Option<String>,
    /// The bottom page margin.
    #[serde(rename="marginBottom")]
    pub margin_bottom: Option<Dimension>,
    /// The ID of the header used only for the first page. If not set then
    /// a unique header for the first page does not exist.
    /// The value of use_first_page_header_footer determines
    /// whether to use the default_header_id or this value for the
    /// header on the first page. If not set, there is no first page header.
    #[serde(rename="firstPageHeaderId")]
    pub first_page_header_id: Option<String>,
    /// The ID of the header used only for even pages. The value of
    /// use_even_page_header_footer determines
    /// whether to use the default_header_id or this value for the
    /// header on even pages. If not set, there is no even page header.
    #[serde(rename="evenPageHeaderId")]
    pub even_page_header_id: Option<String>,
    /// Indicates whether to use the first page header / footer IDs for the first
    /// page.
    #[serde(rename="useFirstPageHeaderFooter")]
    pub use_first_page_header_footer: Option<bool>,
    /// The left page margin.
    #[serde(rename="marginLeft")]
    pub margin_left: Option<Dimension>,
    /// Indicates whether to use the even page header / footer IDs for the even
    /// pages.
    #[serde(rename="useEvenPageHeaderFooter")]
    pub use_even_page_header_footer: Option<bool>,
    /// The background of the document.
    pub background: Option<Background>,
    /// The right page margin.
    #[serde(rename="marginRight")]
    pub margin_right: Option<Dimension>,
    /// The page number from which to start counting the number of pages.
    #[serde(rename="pageNumberStart")]
    pub page_number_start: Option<i32>,
    /// The top page margin.
    #[serde(rename="marginTop")]
    pub margin_top: Option<Dimension>,
}

impl Part for DocumentStyle {}


/// Deletes a PositionedObject from the
/// document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeletePositionedObjectRequest {
    /// The ID of the positioned object to delete.
    #[serde(rename="objectId")]
    pub object_id: Option<String>,
}

impl Part for DeletePositionedObjectRequest {}


/// A mask that indicates which of the fields on the base
/// InlineObjectProperties have
/// been changed in this suggestion. For any field set to true, there is a new
/// suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InlineObjectPropertiesSuggestionState {
    /// A mask that indicates which of the fields in embedded_object have been
    /// changed in this suggestion.
    #[serde(rename="embeddedObjectSuggestionState")]
    pub embedded_object_suggestion_state: Option<EmbeddedObjectSuggestionState>,
}

impl Part for InlineObjectPropertiesSuggestionState {}


/// The contents and style of a row in a Table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableRow {
    /// The contents and style of each cell in this row.
    /// 
    /// It is possible for a table to be non-rectangular, so some rows may have a
    /// different number of cells than other rows in the same table.
    #[serde(rename="tableCells")]
    pub table_cells: Option<Vec<TableCell>>,
    /// The zero-based start index of this row, in UTF-16 code units.
    #[serde(rename="startIndex")]
    pub start_index: Option<i32>,
    /// The zero-based end index of this row, exclusive, in UTF-16 code units.
    #[serde(rename="endIndex")]
    pub end_index: Option<i32>,
    /// The style of the table row.
    #[serde(rename="tableRowStyle")]
    pub table_row_style: Option<TableRowStyle>,
    /// The suggested deletion IDs. If empty, then there are no suggested deletions
    /// of this content.
    #[serde(rename="suggestedDeletionIds")]
    pub suggested_deletion_ids: Option<Vec<String>>,
    /// The suggested style changes to this row, keyed by suggestion ID.
    #[serde(rename="suggestedTableRowStyleChanges")]
    pub suggested_table_row_style_changes: Option<HashMap<String, SuggestedTableRowStyle>>,
    /// The suggested insertion IDs. A TableRow
    /// may have multiple insertion IDs if it is a nested suggested change. If
    /// empty, then this is not a suggested insertion.
    #[serde(rename="suggestedInsertionIds")]
    pub suggested_insertion_ids: Option<Vec<String>>,
}

impl Part for TableRow {}


/// Styles that apply to a table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableStyle {
    /// The properties of each column.
    /// 
    /// Note that in Docs, tables contain rows and rows contain cells, similar to
    /// HTML. So the properties for a row can be found on the row's
    /// table_row_style.
    #[serde(rename="tableColumnProperties")]
    pub table_column_properties: Option<Vec<TableColumnProperties>>,
}

impl Part for TableStyle {}


/// Specifies a contiguous range of text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Range {
    /// The zero-based end index of this range, exclusive, in UTF-16 code units.
    /// 
    /// In all current uses, an end index must be provided. This field is an
    /// Int32Value in order to accommodate future use cases with open-ended ranges.
    #[serde(rename="endIndex")]
    pub end_index: Option<i32>,
    /// The zero-based start index of this range, in UTF-16 code units.
    /// 
    /// In all current uses, a start index must be provided. This field is an
    /// Int32Value in order to accommodate future use cases with open-ended ranges.
    #[serde(rename="startIndex")]
    pub start_index: Option<i32>,
    /// The ID of the header, footer or footnote that this range is contained in.
    /// An empty segment ID signifies the document's body.
    #[serde(rename="segmentId")]
    pub segment_id: Option<String>,
}

impl Part for Range {}


/// Deletes a row from a table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteTableRowRequest {
    /// The reference table cell location from which the row will be deleted.
    /// 
    /// The row this cell spans will be deleted. If this is a merged cell that
    /// spans multiple rows, all rows that the cell spans will be deleted. If no
    /// rows remain in the table after this deletion, the whole table is deleted.
    #[serde(rename="tableCellLocation")]
    pub table_cell_location: Option<TableCellLocation>,
}

impl Part for DeleteTableRowRequest {}


/// A reference to a linked chart embedded from Google Sheets.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SheetsChartReference {
    /// The ID of the specific chart in the Google Sheets spreadsheet that is
    /// embedded.
    #[serde(rename="chartId")]
    pub chart_id: Option<i32>,
    /// The ID of the Google Sheets spreadsheet that contains the source chart.
    #[serde(rename="spreadsheetId")]
    pub spreadsheet_id: Option<String>,
}

impl Part for SheetsChartReference {}


/// Creates bullets for all of the paragraphs that overlap with the given range.
/// 
/// The nesting level of each paragraph will be determined by counting leading
/// tabs in front of each paragraph. To avoid excess space between the bullet and
/// the corresponding paragraph, these leading tabs are removed by this request.
/// This may change the indices of parts of the text.
/// 
/// If the paragraph immediately before paragraphs being updated is in a list
/// with a matching preset, the paragraphs being updated are added to that
/// preceding list.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateParagraphBulletsRequest {
    /// The kinds of bullet glyphs to be used.
    #[serde(rename="bulletPreset")]
    pub bullet_preset: Option<String>,
    /// The range to apply the bullet preset to.
    pub range: Option<Range>,
}

impl Part for CreateParagraphBulletsRequest {}


/// A suggested change to a
/// TableRowStyle.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SuggestedTableRowStyle {
    /// A TableRowStyle that only includes
    /// the changes made in this suggestion. This can be used along with the
    /// table_row_style_suggestion_state
    /// to see which fields have changed and their new values.
    #[serde(rename="tableRowStyle")]
    pub table_row_style: Option<TableRowStyle>,
    /// A mask that indicates which of the fields on the base TableRowStyle have been changed in this suggestion.
    #[serde(rename="tableRowStyleSuggestionState")]
    pub table_row_style_suggestion_state: Option<TableRowStyleSuggestionState>,
}

impl Part for SuggestedTableRowStyle {}


/// A mask that indicates which of the fields on the base Shading have been changed in this
/// suggested change. For any field set to true, there is a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ShadingSuggestionState {
    /// Indicates if there was a suggested change to the Shading.
    #[serde(rename="backgroundColorSuggested")]
    pub background_color_suggested: Option<bool>,
}

impl Part for ShadingSuggestionState {}


/// A mask that indicates which of the fields on the base CropProperties have been changed in this suggestion.
/// For any field set to true, there is a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CropPropertiesSuggestionState {
    /// Indicates if there was a suggested change to offset_bottom.
    #[serde(rename="offsetBottomSuggested")]
    pub offset_bottom_suggested: Option<bool>,
    /// Indicates if there was a suggested change to offset_left.
    #[serde(rename="offsetLeftSuggested")]
    pub offset_left_suggested: Option<bool>,
    /// Indicates if there was a suggested change to offset_right.
    #[serde(rename="offsetRightSuggested")]
    pub offset_right_suggested: Option<bool>,
    /// Indicates if there was a suggested change to angle.
    #[serde(rename="angleSuggested")]
    pub angle_suggested: Option<bool>,
    /// Indicates if there was a suggested change to offset_top.
    #[serde(rename="offsetTopSuggested")]
    pub offset_top_suggested: Option<bool>,
}

impl Part for CropPropertiesSuggestionState {}


/// A collection of object IDs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ObjectReferences {
    /// The object IDs.
    #[serde(rename="objectIds")]
    pub object_ids: Option<Vec<String>>,
}

impl Part for ObjectReferences {}


/// Represents a font family and weight of text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WeightedFontFamily {
    /// The font family of the text.
    /// 
    /// The font family can be any font from the Font menu in Docs or from
    /// [Google Fonts] (https://fonts.google.com/). If the font name is
    /// unrecognized, the text is rendered in `Arial`.
    #[serde(rename="fontFamily")]
    pub font_family: Option<String>,
    /// The weight of the font. This field can have any value that is a multiple of
    /// `100` between `100` and `900`, inclusive. This range corresponds to the
    /// numerical values described in the CSS 2.1 Specification,
    /// [section 15.6](https://www.w3.org/TR/CSS21/fonts.html#font-boldness), with
    /// non-numerical values disallowed.
    /// 
    /// The default value is `400` ("normal").
    /// 
    /// The font weight makes up just one component of the rendered font weight.
    /// The rendered weight is determined by a combination of the `weight` and the
    /// text style's resolved `bold` value, after accounting for inheritance:
    /// 
    /// * If the text is bold and the weight is less than `400`, the rendered
    ///   weight is 400.
    /// * If the text is bold and the weight is greater than or equal to `400` but
    ///   is less than `700`, the rendered weight is `700`.
    /// * If the weight is greater than or equal to `700`, the rendered weight is
    ///   equal to the weight.
    /// * If the text is not bold, the rendered weight is equal to the weight.
    pub weight: Option<i32>,
}

impl Part for WeightedFontFamily {}


/// A StructuralElement representing
/// a table of contents.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableOfContents {
    /// The content of the table of contents.
    pub content: Option<Vec<StructuralElement>>,
    /// The suggested deletion IDs. If empty, then there are no suggested deletions
    /// of this content.
    #[serde(rename="suggestedDeletionIds")]
    pub suggested_deletion_ids: Option<Vec<String>>,
    /// The suggested insertion IDs. A TableOfContents may have multiple insertion IDs if it
    /// is a nested suggested change. If empty, then this is not a suggested
    /// insertion.
    #[serde(rename="suggestedInsertionIds")]
    pub suggested_insertion_ids: Option<Vec<String>>,
}

impl Part for TableOfContents {}


/// Inserts an InlineObject containing an
/// image at the given location.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InsertInlineImageRequest {
    /// The size that the image should appear as in the document. This property is
    /// optional and the final size of the image in the document is determined by
    /// the following rules:
    ///  * If neither width nor height is specified, then a default size of the
    ///  image is calculated based on its resolution.
    ///  * If one dimension is specified then the other dimension is calculated to
    ///  preserve the aspect ratio of the image.
    ///  * If both width and height are specified, the image is scaled to fit
    ///  within the provided dimensions while maintaining its aspect ratio.
    #[serde(rename="objectSize")]
    pub object_size: Option<Size>,
    /// Inserts the text at the end of a header, footer or the document body.
    /// 
    /// Inline images cannot be inserted inside a footnote.
    #[serde(rename="endOfSegmentLocation")]
    pub end_of_segment_location: Option<EndOfSegmentLocation>,
    /// Inserts the image at a specific index in the document.
    /// 
    /// The image must be inserted inside the bounds of an existing
    /// Paragraph. For instance, it cannot be
    /// inserted at a table's start index (i.e. between the table and its
    /// preceding paragraph).
    /// 
    /// Inline images cannot be inserted inside a footnote or equation.
    pub location: Option<Location>,
    /// The image URI.
    /// 
    /// The image is fetched once at insertion time and a copy is stored for
    /// display inside the document. Images must be less than 50MB in size, cannot
    /// exceed 25 megapixels, and must be in one of PNG, JPEG, or GIF format.
    /// 
    /// The provided URI can be at most 2 kB in length. The URI itself is saved
    /// with the image, and exposed via the ImageProperties.content_uri field.
    pub uri: Option<String>,
}

impl Part for InsertInlineImageRequest {}


/// A mask that indicates which of the fields on the base TableCellStyle have been changed in this suggestion.
/// For any field set to true, there is a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableCellStyleSuggestionState {
    /// Indicates if there was a suggested change to border_bottom.
    #[serde(rename="borderBottomSuggested")]
    pub border_bottom_suggested: Option<bool>,
    /// Indicates if there was a suggested change to border_right.
    #[serde(rename="borderRightSuggested")]
    pub border_right_suggested: Option<bool>,
    /// Indicates if there was a suggested change to padding_bottom.
    #[serde(rename="paddingBottomSuggested")]
    pub padding_bottom_suggested: Option<bool>,
    /// Indicates if there was a suggested change to content_alignment.
    #[serde(rename="contentAlignmentSuggested")]
    pub content_alignment_suggested: Option<bool>,
    /// Indicates if there was a suggested change to padding_left.
    #[serde(rename="paddingLeftSuggested")]
    pub padding_left_suggested: Option<bool>,
    /// Indicates if there was a suggested change to border_left.
    #[serde(rename="borderLeftSuggested")]
    pub border_left_suggested: Option<bool>,
    /// Indicates if there was a suggested change to border_top.
    #[serde(rename="borderTopSuggested")]
    pub border_top_suggested: Option<bool>,
    /// Indicates if there was a suggested change to column_span.
    #[serde(rename="columnSpanSuggested")]
    pub column_span_suggested: Option<bool>,
    /// Indicates if there was a suggested change to padding_right.
    #[serde(rename="paddingRightSuggested")]
    pub padding_right_suggested: Option<bool>,
    /// Indicates if there was a suggested change to background_color.
    #[serde(rename="backgroundColorSuggested")]
    pub background_color_suggested: Option<bool>,
    /// Indicates if there was a suggested change to row_span.
    #[serde(rename="rowSpanSuggested")]
    pub row_span_suggested: Option<bool>,
    /// Indicates if there was a suggested change to padding_top.
    #[serde(rename="paddingTopSuggested")]
    pub padding_top_suggested: Option<bool>,
}

impl Part for TableCellStyleSuggestionState {}


/// Updates the TableRowStyle of rows in a
/// table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateTableRowStyleRequest {
    /// The fields that should be updated.
    /// 
    /// At least one field must be specified. The root `tableRowStyle` is implied
    /// and should not be specified. A single `"*"` can be used as short-hand for
    /// listing every field.
    /// 
    /// For example to update the minimum row height, set `fields` to
    /// `"min_row_height"`.
    pub fields: Option<String>,
    /// The styles to be set on the rows.
    #[serde(rename="tableRowStyle")]
    pub table_row_style: Option<TableRowStyle>,
    /// The list of zero-based row indices whose style should be updated. If no
    /// indices are specified, all rows will be updated.
    #[serde(rename="rowIndices")]
    pub row_indices: Option<Vec<i32>>,
    /// The location where the table starts in the document.
    #[serde(rename="tableStartLocation")]
    pub table_start_location: Option<Location>,
}

impl Part for UpdateTableRowStyleRequest {}


/// A mask that indicates which of the fields on the base DocumentStyle have been changed in this suggestion.
/// For any field set to true, there is a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DocumentStyleSuggestionState {
    /// Indicates if there was a suggested change to margin_bottom.
    #[serde(rename="marginBottomSuggested")]
    pub margin_bottom_suggested: Option<bool>,
    /// Indicates if there was a suggested change to even_page_header_id.
    #[serde(rename="evenPageHeaderIdSuggested")]
    pub even_page_header_id_suggested: Option<bool>,
    /// Indicates if there was a suggested change to first_page_header_id.
    #[serde(rename="firstPageHeaderIdSuggested")]
    pub first_page_header_id_suggested: Option<bool>,
    /// Indicates if there was a suggested change to margin_right.
    #[serde(rename="marginRightSuggested")]
    pub margin_right_suggested: Option<bool>,
    /// Indicates if there was a suggested change to page_number_start.
    #[serde(rename="pageNumberStartSuggested")]
    pub page_number_start_suggested: Option<bool>,
    /// Indicates if there was a suggested change to default_header_id.
    #[serde(rename="defaultHeaderIdSuggested")]
    pub default_header_id_suggested: Option<bool>,
    /// Indicates if there was a suggested change to default_footer_id.
    #[serde(rename="defaultFooterIdSuggested")]
    pub default_footer_id_suggested: Option<bool>,
    /// Indicates if there was a suggested change to margin_left.
    #[serde(rename="marginLeftSuggested")]
    pub margin_left_suggested: Option<bool>,
    /// Indicates if there was a suggested change to margin_top.
    #[serde(rename="marginTopSuggested")]
    pub margin_top_suggested: Option<bool>,
    /// Indicates if there was a suggested change to first_page_footer_id.
    #[serde(rename="firstPageFooterIdSuggested")]
    pub first_page_footer_id_suggested: Option<bool>,
    /// A mask that indicates which of the fields in size have been changed in this
    /// suggestion.
    #[serde(rename="pageSizeSuggestionState")]
    pub page_size_suggestion_state: Option<SizeSuggestionState>,
    /// Indicates if there was a suggested change to use_first_page_header_footer.
    #[serde(rename="useFirstPageHeaderFooterSuggested")]
    pub use_first_page_header_footer_suggested: Option<bool>,
    /// Indicates if there was a suggested change to even_page_footer_id.
    #[serde(rename="evenPageFooterIdSuggested")]
    pub even_page_footer_id_suggested: Option<bool>,
    /// A mask that indicates which of the fields in background have been changed in this
    /// suggestion.
    #[serde(rename="backgroundSuggestionState")]
    pub background_suggestion_state: Option<BackgroundSuggestionState>,
    /// Indicates if there was a suggested change to use_even_page_header_footer.
    #[serde(rename="useEvenPageHeaderFooterSuggested")]
    pub use_even_page_header_footer_suggested: Option<bool>,
}

impl Part for DocumentStyleSuggestionState {}


/// The result of inserting an embedded Google Sheets chart.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InsertInlineSheetsChartResponse {
    /// The object ID of the inserted chart.
    #[serde(rename="objectId")]
    pub object_id: Option<String>,
}

impl Part for InsertInlineSheetsChartResponse {}


/// The positioning of a PositionedObject. The positioned object is positioned
/// relative to the beginning of the Paragraph
/// it is tethered to.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PositionedObjectPositioning {
    /// The offset of the left edge of the positioned object relative to the
    /// beginning of the Paragraph it is tethered
    /// to. The exact positioning of the object can depend on other content in the
    /// document and the document's styling.
    #[serde(rename="leftOffset")]
    pub left_offset: Option<Dimension>,
    /// The offset of the top edge of the positioned object relative to the
    /// beginning of the Paragraph it is tethered
    /// to. The exact positioning of the object can depend on other content in the
    /// document and the document's styling.
    #[serde(rename="topOffset")]
    pub top_offset: Option<Dimension>,
    /// The layout of this positioned object.
    pub layout: Option<String>,
}

impl Part for PositionedObjectPositioning {}


/// The style of a TableCell.
/// 
/// Inherited table cell styles are represented as unset fields in this message.
/// A table cell style can inherit from the table's style.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableCellStyle {
    /// The bottom padding of the cell.
    #[serde(rename="paddingBottom")]
    pub padding_bottom: Option<Dimension>,
    /// The bottom border of the cell.
    #[serde(rename="borderBottom")]
    pub border_bottom: Option<TableCellBorder>,
    /// The top padding of the cell.
    #[serde(rename="paddingTop")]
    pub padding_top: Option<Dimension>,
    /// The row span of the cell. This property is read-only.
    #[serde(rename="rowSpan")]
    pub row_span: Option<i32>,
    /// The column span of the cell. This property is read-only.
    #[serde(rename="columnSpan")]
    pub column_span: Option<i32>,
    /// The background color of the cell.
    #[serde(rename="backgroundColor")]
    pub background_color: Option<OptionalColor>,
    /// The right border of the cell.
    #[serde(rename="borderRight")]
    pub border_right: Option<TableCellBorder>,
    /// The left padding of the cell.
    #[serde(rename="paddingLeft")]
    pub padding_left: Option<Dimension>,
    /// The top border of the cell.
    #[serde(rename="borderTop")]
    pub border_top: Option<TableCellBorder>,
    /// The right padding of the cell.
    #[serde(rename="paddingRight")]
    pub padding_right: Option<Dimension>,
    /// The left border of the cell.
    #[serde(rename="borderLeft")]
    pub border_left: Option<TableCellBorder>,
    /// The alignment of the content in the table cell. The default alignment
    /// matches the alignment for newly created table cells in the Docs editor.
    #[serde(rename="contentAlignment")]
    pub content_alignment: Option<String>,
}

impl Part for TableCellStyle {}


/// Inserts an empty row into a table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InsertTableRowRequest {
    /// The reference table cell location from which rows will be inserted.
    /// 
    /// A new row will be inserted above (or below) the row where the reference
    /// cell is. If the reference cell is a merged cell, a new row will be
    /// inserted above (or below) the merged cell.
    #[serde(rename="tableCellLocation")]
    pub table_cell_location: Option<TableCellLocation>,
    /// Whether to insert new row below the reference cell location.
    /// 
    /// - `True`: insert below the cell.
    /// - `False`: insert above the cell.
    #[serde(rename="insertBelow")]
    pub insert_below: Option<bool>,
}

impl Part for InsertTableRowRequest {}


/// A single response from an update.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Response {
    /// The result of creating a named range.
    #[serde(rename="createNamedRange")]
    pub create_named_range: Option<CreateNamedRangeResponse>,
    /// The result of inserting an inline Google Sheets chart.
    #[serde(rename="insertInlineSheetsChart")]
    pub insert_inline_sheets_chart: Option<InsertInlineSheetsChartResponse>,
    /// The result of replacing text.
    #[serde(rename="replaceAllText")]
    pub replace_all_text: Option<ReplaceAllTextResponse>,
    /// The result of inserting an inline image.
    #[serde(rename="insertInlineImage")]
    pub insert_inline_image: Option<InsertInlineImageResponse>,
}

impl Part for Response {}


/// Styles that apply to a whole paragraph.
/// 
/// Inherited paragraph styles are represented as unset fields in this message.
/// A paragraph style's parent depends on where the paragraph style is defined:
/// 
/// * The ParagraphStyle on a Paragraph
///   inherits from the paragraph's corresponding named style type.
/// * The ParagraphStyle on a named style
///   inherits from the normal text named style.
/// * The ParagraphStyle of the normal text named style inherits
///   from the default paragraph style in the Docs editor.
/// * The ParagraphStyle on a Paragraph
///   element that is contained in a table may inherit its paragraph style from
///   the table style.
/// 
/// If the paragraph style does not inherit from a parent, unsetting fields will
/// revert the style to a value matching the defaults in the Docs editor.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ParagraphStyle {
    /// The spacing mode for the paragraph.
    #[serde(rename="spacingMode")]
    pub spacing_mode: Option<String>,
    /// The text direction of this paragraph. If unset, the value defaults to
    /// LEFT_TO_RIGHT since
    /// paragraph direction is not inherited.
    pub direction: Option<String>,
    /// Whether at least a part of this paragraph should be laid out on the same
    /// page or column as the next paragraph if possible. If unset, the value is
    /// inherited from the parent.
    #[serde(rename="keepWithNext")]
    pub keep_with_next: Option<bool>,
    /// The amount of space between lines, as a percentage of normal, where normal
    /// is represented as 100.0. If unset, the value is inherited from the parent.
    #[serde(rename="lineSpacing")]
    pub line_spacing: Option<f32>,
    /// The amount of indentation for the paragraph on the side that corresponds to
    /// the start of the text, based on the current paragraph direction. If unset,
    /// the value is inherited from the parent.
    #[serde(rename="indentStart")]
    pub indent_start: Option<Dimension>,
    /// The border at the bottom of this paragraph. If unset, the value is
    /// inherited from the parent.
    /// 
    /// The bottom border is rendered when the paragraph below has different border
    /// and indent properties.
    /// 
    /// Paragraph borders cannot be partially updated. When making
    /// changes to a paragraph border the new border must be specified in
    /// its entirety.
    #[serde(rename="borderBottom")]
    pub border_bottom: Option<ParagraphBorder>,
    /// The border to the right of this paragraph. If unset, the value is inherited
    /// from the parent.
    /// 
    /// Paragraph borders cannot be partially updated. When making
    /// changes to a paragraph border the new border must be specified in
    /// its entirety.
    #[serde(rename="borderRight")]
    pub border_right: Option<ParagraphBorder>,
    /// The border to the left of this paragraph. If unset, the value is inherited
    /// from the parent.
    /// 
    /// Paragraph borders cannot be partially updated. When making
    /// changes to a paragraph border the new border must be specified in
    /// its entirety.
    #[serde(rename="borderLeft")]
    pub border_left: Option<ParagraphBorder>,
    /// The heading ID of the paragraph. If empty, then this paragraph is not a
    /// heading. This property is read-only.
    #[serde(rename="headingId")]
    pub heading_id: Option<String>,
    /// The shading of the paragraph. If unset, the value is inherited from the
    /// parent.
    pub shading: Option<Shading>,
    /// The border at the top of this paragraph. If unset, the value is inherited
    /// from the parent.
    /// 
    /// The top border is rendered when the paragraph above has different border
    /// and indent properties.
    /// 
    /// Paragraph borders cannot be partially updated. When making
    /// changes to a paragraph border the new border must be specified in
    /// its entirety.
    #[serde(rename="borderTop")]
    pub border_top: Option<ParagraphBorder>,
    /// A list of the tab stops for this paragraph. The list of tab stops is not
    /// inherited. This property is read-only.
    #[serde(rename="tabStops")]
    pub tab_stops: Option<Vec<TabStop>>,
    /// The amount of indentation for the first line of the paragraph. If unset,
    /// the value is inherited from the parent.
    #[serde(rename="indentFirstLine")]
    pub indent_first_line: Option<Dimension>,
    /// The text alignment for this paragraph.
    pub alignment: Option<String>,
    /// The amount of extra space below the paragraph. If unset, the value is
    /// inherited from the parent.
    #[serde(rename="spaceBelow")]
    pub space_below: Option<Dimension>,
    /// Whether to avoid widows and orphans for the paragraph. If unset, the value
    /// is inherited from the parent.
    #[serde(rename="avoidWidowAndOrphan")]
    pub avoid_widow_and_orphan: Option<bool>,
    /// The border between this paragraph and the next and previous paragraphs.
    /// If unset, the value is inherited from the parent.
    /// 
    /// The between border is rendered when the adjacent paragraph has the same
    /// border and indent properties.
    /// 
    /// Paragraph borders cannot be partially updated. When making
    /// changes to a paragraph border the new border must be specified in
    /// its entirety.
    #[serde(rename="borderBetween")]
    pub border_between: Option<ParagraphBorder>,
    /// The amount of extra space above the paragraph. If unset, the value is
    /// inherited from the parent.
    #[serde(rename="spaceAbove")]
    pub space_above: Option<Dimension>,
    /// The amount of indentation for the paragraph on the side that corresponds to
    /// the end of the text, based on the current paragraph direction. If unset,
    /// the value is inherited from the parent.
    #[serde(rename="indentEnd")]
    pub indent_end: Option<Dimension>,
    /// The named style type of the paragraph.
    /// 
    /// Since updating the named style type affects other properties within
    /// ParagraphStyle, the named style type is applied before the other properties
    /// are updated.
    #[serde(rename="namedStyleType")]
    pub named_style_type: Option<String>,
    /// Whether all lines of the paragraph should be laid out on the same page or
    /// column if possible. If unset, the value is inherited from the parent.
    #[serde(rename="keepLinesTogether")]
    pub keep_lines_together: Option<bool>,
}

impl Part for ParagraphStyle {}


/// A StructuralElement representing a
/// section break. A section is a range of content which has the same
/// SectionStyle. A section break represents
/// the start of a new section, and the section style applies to the section
/// after the section break.
/// 
/// The document body always begins with a section break.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SectionBreak {
    /// The suggested deletion IDs. If empty, then there are no suggested deletions
    /// of this content.
    #[serde(rename="suggestedDeletionIds")]
    pub suggested_deletion_ids: Option<Vec<String>>,
    /// The style of the section after this section break.
    #[serde(rename="sectionStyle")]
    pub section_style: Option<SectionStyle>,
    /// The suggested insertion IDs. A SectionBreak may have multiple insertion IDs if it is
    /// a nested suggested change. If empty, then this is not a suggested
    /// insertion.
    #[serde(rename="suggestedInsertionIds")]
    pub suggested_insertion_ids: Option<Vec<String>>,
}

impl Part for SectionBreak {}


/// Describes the bullet of a paragraph.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Bullet {
    /// The nesting level of this paragraph in the list.
    #[serde(rename="nestingLevel")]
    pub nesting_level: Option<i32>,
    /// The ID of the list this paragraph belongs to.
    #[serde(rename="listId")]
    pub list_id: Option<String>,
    /// The paragraph specific text style applied to this bullet.
    #[serde(rename="textStyle")]
    pub text_style: Option<TextStyle>,
}

impl Part for Bullet {}


/// A document footer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Footer {
    /// The contents of the footer.
    /// 
    /// The indexes for a footer's content begin at zero.
    pub content: Option<Vec<StructuralElement>>,
    /// The ID of the footer.
    #[serde(rename="footerId")]
    pub footer_id: Option<String>,
}

impl Part for Footer {}


/// A mask that indicates which of the fields on the base NestingLevel have been changed in this suggestion. For
/// any field set to true, there is a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NestingLevelSuggestionState {
    /// A mask that indicates which of the fields in text style have been changed in this
    /// suggestion.
    #[serde(rename="textStyleSuggestionState")]
    pub text_style_suggestion_state: Option<TextStyleSuggestionState>,
    /// Indicates if there was a suggested change to
    /// glyph_format.
    #[serde(rename="glyphFormatSuggested")]
    pub glyph_format_suggested: Option<bool>,
    /// Indicates if there was a suggested change to
    /// indent_start.
    #[serde(rename="indentStartSuggested")]
    pub indent_start_suggested: Option<bool>,
    /// Indicates if there was a suggested change to
    /// start_number.
    #[serde(rename="startNumberSuggested")]
    pub start_number_suggested: Option<bool>,
    /// Indicates if there was a suggested change to
    /// glyph_type.
    #[serde(rename="glyphTypeSuggested")]
    pub glyph_type_suggested: Option<bool>,
    /// Indicates if there was a suggested change to
    /// indent_first_line.
    #[serde(rename="indentFirstLineSuggested")]
    pub indent_first_line_suggested: Option<bool>,
    /// Indicates if there was a suggested change to
    /// glyph_symbol.
    #[serde(rename="glyphSymbolSuggested")]
    pub glyph_symbol_suggested: Option<bool>,
    /// Indicates if there was a suggested change to
    /// bullet_alignment.
    #[serde(rename="bulletAlignmentSuggested")]
    pub bullet_alignment_suggested: Option<bool>,
}

impl Part for NestingLevelSuggestionState {}


/// A suggested change to a
/// ParagraphStyle.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SuggestedParagraphStyle {
    /// A mask that indicates which of the fields on the base ParagraphStyle have been changed in this suggestion.
    #[serde(rename="paragraphStyleSuggestionState")]
    pub paragraph_style_suggestion_state: Option<ParagraphStyleSuggestionState>,
    /// A ParagraphStyle that only includes
    /// the changes made in this suggestion. This can be used along with the
    /// paragraph_suggestion_state
    /// to see which fields have changed and their new values.
    #[serde(rename="paragraphStyle")]
    pub paragraph_style: Option<ParagraphStyle>,
}

impl Part for SuggestedParagraphStyle {}


/// An object that is tethered to a Paragraph
/// and positioned relative to the beginning of the paragraph. A PositionedObject
/// contains an EmbeddedObject such as an
/// image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PositionedObject {
    /// The properties of this positioned object.
    #[serde(rename="positionedObjectProperties")]
    pub positioned_object_properties: Option<PositionedObjectProperties>,
    /// The suggested deletion IDs. If empty, then there are no suggested deletions
    /// of this content.
    #[serde(rename="suggestedDeletionIds")]
    pub suggested_deletion_ids: Option<Vec<String>>,
    /// The ID of this positioned object.
    #[serde(rename="objectId")]
    pub object_id: Option<String>,
    /// The suggested changes to the positioned object properties, keyed by
    /// suggestion ID.
    #[serde(rename="suggestedPositionedObjectPropertiesChanges")]
    pub suggested_positioned_object_properties_changes: Option<HashMap<String, SuggestedPositionedObjectProperties>>,
    /// The suggested insertion ID. If empty, then this is not a suggested
    /// insertion.
    #[serde(rename="suggestedInsertionId")]
    pub suggested_insertion_id: Option<String>,
}

impl Part for PositionedObject {}


/// A named style. Paragraphs in the document can inherit their
/// TextStyle and
/// ParagraphStyle from this named style
/// when they have the same named style type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NamedStyle {
    /// The text style of this named style.
    #[serde(rename="textStyle")]
    pub text_style: Option<TextStyle>,
    /// The type of this named style.
    #[serde(rename="namedStyleType")]
    pub named_style_type: Option<String>,
    /// The paragraph style of this named style.
    #[serde(rename="paragraphStyle")]
    pub paragraph_style: Option<ParagraphStyle>,
}

impl Part for NamedStyle {}


/// A mask that indicates which of the fields on the base TableRowStyle have been changed in this suggestion.
/// For any field set to true, there is a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableRowStyleSuggestionState {
    /// Indicates if there was a suggested change to min_row_height.
    #[serde(rename="minRowHeightSuggested")]
    pub min_row_height_suggested: Option<bool>,
}

impl Part for TableRowStyleSuggestionState {}


/// A collection of Ranges with the same named range
/// ID.
/// 
/// Named ranges allow developers to associate parts of a document with an
/// arbitrary user-defined label so their contents can be programmatically read
/// or edited at a later time. A document can contain multiple named ranges with
/// the same name, but every named range has a unique ID.
/// 
/// A named range is created with a single Range,
/// and content inserted inside a named range generally expands that range.
/// However, certain document changes can cause the range to be split into
/// multiple ranges.
/// 
/// Named ranges are not private. All applications and collaborators that have
/// access to the document can see its named ranges.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NamedRange {
    /// The ranges that belong to this named range.
    pub ranges: Option<Vec<Range>>,
    /// The ID of the named range.
    #[serde(rename="namedRangeId")]
    pub named_range_id: Option<String>,
    /// The name of the named range.
    pub name: Option<String>,
}

impl Part for NamedRange {}


/// Location of a single cell within a table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableCellLocation {
    /// The zero-based row index. For example, the second row in the table has a
    /// row index of 1.
    #[serde(rename="rowIndex")]
    pub row_index: Option<i32>,
    /// The zero-based column index. For example, the second column in the table
    /// has a column index of 1.
    #[serde(rename="columnIndex")]
    pub column_index: Option<i32>,
    /// The location where the table starts in the document.
    #[serde(rename="tableStartLocation")]
    pub table_start_location: Option<Location>,
}

impl Part for TableCellLocation {}


/// A document footnote.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Footnote {
    /// The contents of the footnote.
    /// 
    /// The indexes for a footnote's content begin at zero.
    pub content: Option<Vec<StructuralElement>>,
    /// The ID of the footnote.
    #[serde(rename="footnoteId")]
    pub footnote_id: Option<String>,
}

impl Part for Footnote {}


/// Deletes bullets from all of the paragraphs that overlap with the given range.
/// 
/// The nesting level of each paragraph will be visually preserved by adding
/// indent to the start of the corresponding paragraph.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteParagraphBulletsRequest {
    /// The range to delete bullets from.
    pub range: Option<Range>,
}

impl Part for DeleteParagraphBulletsRequest {}


/// Properties of a PositionedObject.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PositionedObjectProperties {
    /// The positioning of this positioned object relative to the newline of the
    /// Paragraph that references this positioned
    /// object.
    pub positioning: Option<PositionedObjectPositioning>,
    /// The embedded object of this positioned object.
    #[serde(rename="embeddedObject")]
    pub embedded_object: Option<EmbeddedObject>,
}

impl Part for PositionedObjectProperties {}


/// A mask that indicates which of the fields on the base EmbeddedObjectBorder have been changed in this
/// suggestion. For any field set to true, there is a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EmbeddedObjectBorderSuggestionState {
    /// Indicates if there was a suggested change to color.
    #[serde(rename="colorSuggested")]
    pub color_suggested: Option<bool>,
    /// Indicates if there was a suggested change to dash_style.
    #[serde(rename="dashStyleSuggested")]
    pub dash_style_suggested: Option<bool>,
    /// Indicates if there was a suggested change to width.
    #[serde(rename="widthSuggested")]
    pub width_suggested: Option<bool>,
    /// Indicates if there was a suggested change to property_state.
    #[serde(rename="propertyStateSuggested")]
    pub property_state_suggested: Option<bool>,
}

impl Part for EmbeddedObjectBorderSuggestionState {}


/// A mask that indicates which of the fields on the base Background have been changed in this suggestion.
/// For any field set to true, the Backgound has a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BackgroundSuggestionState {
    /// Indicates whether the current background color has been modified in this
    /// suggestion.
    #[serde(rename="backgroundColorSuggested")]
    pub background_color_suggested: Option<bool>,
}

impl Part for BackgroundSuggestionState {}


/// A width and height.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Size {
    /// The width of the object.
    pub width: Option<Dimension>,
    /// The height of the object.
    pub height: Option<Dimension>,
}

impl Part for Size {}


/// Deletes a NamedRange.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteNamedRangeRequest {
    /// The ID of the named range to delete.
    #[serde(rename="namedRangeId")]
    pub named_range_id: Option<String>,
    /// The name of the range(s) to delete. All named ranges with the given
    /// name will be deleted.
    pub name: Option<String>,
}

impl Part for DeleteNamedRangeRequest {}


/// Creates a NamedRange referencing the given
/// range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateNamedRangeRequest {
    /// The range to apply the name to.
    pub range: Option<Range>,
    /// The name of the NamedRange. Names do not need to be unique.
    /// 
    /// Names must be at least 1 character and no more than 256 characters,
    /// measured in UTF-16 code units.
    pub name: Option<String>,
}

impl Part for CreateNamedRangeRequest {}


/// Properties that apply to a section's column.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SectionColumnProperties {
    /// The width of the column.
    pub width: Option<Dimension>,
    /// The padding at the end of the column.
    #[serde(rename="paddingEnd")]
    pub padding_end: Option<Dimension>,
}

impl Part for SectionColumnProperties {}


/// Inserts a page break followed by a newline at the specified location.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InsertPageBreakRequest {
    /// Inserts the page break at the end of the document body.
    /// 
    /// Page breaks cannot be inserted inside a footnote, header or footer.
    /// Since page breaks can only be inserted inside the body, the segment ID field must be
    /// empty.
    #[serde(rename="endOfSegmentLocation")]
    pub end_of_segment_location: Option<EndOfSegmentLocation>,
    /// Inserts the page break at a specific index in the document.
    /// 
    /// The page break must be inserted inside the bounds of an existing
    /// Paragraph. For instance, it cannot be
    /// inserted at a table's start index (i.e. between the table and its
    /// preceding paragraph).
    /// 
    /// Page breaks cannot be inserted inside a table, equation, footnote, header
    /// or footer. Since page breaks can only be inserted inside the body, the
    /// segment ID field must be
    /// empty.
    pub location: Option<Location>,
}

impl Part for InsertPageBreakRequest {}


/// The contents and style of a cell in a Table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableCell {
    /// The content of the cell.
    pub content: Option<Vec<StructuralElement>>,
    /// The zero-based end index of this cell, exclusive, in UTF-16 code units.
    #[serde(rename="endIndex")]
    pub end_index: Option<i32>,
    /// The zero-based start index of this cell, in UTF-16 code units.
    #[serde(rename="startIndex")]
    pub start_index: Option<i32>,
    /// The style of the cell.
    #[serde(rename="tableCellStyle")]
    pub table_cell_style: Option<TableCellStyle>,
    /// The suggested changes to the table cell style, keyed by suggestion ID.
    #[serde(rename="suggestedTableCellStyleChanges")]
    pub suggested_table_cell_style_changes: Option<HashMap<String, SuggestedTableCellStyle>>,
    /// The suggested deletion IDs. If empty, then there are no suggested deletions
    /// of this content.
    #[serde(rename="suggestedDeletionIds")]
    pub suggested_deletion_ids: Option<Vec<String>>,
    /// The suggested insertion IDs. A TableCell
    /// may have multiple insertion IDs if it is a nested suggested change. If
    /// empty, then this is not a suggested insertion.
    #[serde(rename="suggestedInsertionIds")]
    pub suggested_insertion_ids: Option<Vec<String>>,
}

impl Part for TableCell {}


/// A mask that indicates which of the fields on the base TextStyle have been changed in this suggestion.
/// For any field set to true, there is a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TextStyleSuggestionState {
    /// Indicates if there was a suggested change to font_size.
    #[serde(rename="fontSizeSuggested")]
    pub font_size_suggested: Option<bool>,
    /// Indicates if there was a suggested change to strikethrough.
    #[serde(rename="strikethroughSuggested")]
    pub strikethrough_suggested: Option<bool>,
    /// Indicates if there was a suggested change to weighted_font_family.
    #[serde(rename="weightedFontFamilySuggested")]
    pub weighted_font_family_suggested: Option<bool>,
    /// Indicates if there was a suggested change to underline.
    #[serde(rename="underlineSuggested")]
    pub underline_suggested: Option<bool>,
    /// Indicates if there was a suggested change to small_caps.
    #[serde(rename="smallCapsSuggested")]
    pub small_caps_suggested: Option<bool>,
    /// Indicates if there was a suggested change to link.
    #[serde(rename="linkSuggested")]
    pub link_suggested: Option<bool>,
    /// Indicates if there was a suggested change to italic.
    #[serde(rename="italicSuggested")]
    pub italic_suggested: Option<bool>,
    /// Indicates if there was a suggested change to bold.
    #[serde(rename="boldSuggested")]
    pub bold_suggested: Option<bool>,
    /// Indicates if there was a suggested change to baseline_offset.
    #[serde(rename="baselineOffsetSuggested")]
    pub baseline_offset_suggested: Option<bool>,
    /// Indicates if there was a suggested change to foreground_color.
    #[serde(rename="foregroundColorSuggested")]
    pub foreground_color_suggested: Option<bool>,
    /// Indicates if there was a suggested change to background_color.
    #[serde(rename="backgroundColorSuggested")]
    pub background_color_suggested: Option<bool>,
}

impl Part for TextStyleSuggestionState {}


/// A mask that indicates which of the fields on the base ParagraphStyle have been changed in this suggestion.
/// For any field set to true, there is a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ParagraphStyleSuggestionState {
    /// Indicates if there was a suggested change to avoid_widow_and_orphan.
    #[serde(rename="avoidWidowAndOrphanSuggested")]
    pub avoid_widow_and_orphan_suggested: Option<bool>,
    /// Indicates if there was a suggested change to border_bottom.
    #[serde(rename="borderBottomSuggested")]
    pub border_bottom_suggested: Option<bool>,
    /// Indicates if there was a suggested change to named_style_type.
    #[serde(rename="namedStyleTypeSuggested")]
    pub named_style_type_suggested: Option<bool>,
    /// Indicates if there was a suggested change to border_between.
    #[serde(rename="borderBetweenSuggested")]
    pub border_between_suggested: Option<bool>,
    /// Indicates if there was a suggested change to border_top.
    #[serde(rename="borderTopSuggested")]
    pub border_top_suggested: Option<bool>,
    /// Indicates if there was a suggested change to keep_with_next.
    #[serde(rename="keepWithNextSuggested")]
    pub keep_with_next_suggested: Option<bool>,
    /// Indicates if there was a suggested change to space_above.
    #[serde(rename="spaceAboveSuggested")]
    pub space_above_suggested: Option<bool>,
    /// Indicates if there was a suggested change to alignment.
    #[serde(rename="alignmentSuggested")]
    pub alignment_suggested: Option<bool>,
    /// Indicates if there was a suggested change to indent_end.
    #[serde(rename="indentEndSuggested")]
    pub indent_end_suggested: Option<bool>,
    /// Indicates if there was a suggested change to direction.
    #[serde(rename="directionSuggested")]
    pub direction_suggested: Option<bool>,
    /// Indicates if there was a suggested change to border_right.
    #[serde(rename="borderRightSuggested")]
    pub border_right_suggested: Option<bool>,
    /// A mask that indicates which of the fields in shading have been changed in
    /// this suggestion.
    #[serde(rename="shadingSuggestionState")]
    pub shading_suggestion_state: Option<ShadingSuggestionState>,
    /// Indicates if there was a suggested change to keep_lines_together.
    #[serde(rename="keepLinesTogetherSuggested")]
    pub keep_lines_together_suggested: Option<bool>,
    /// Indicates if there was a suggested change to line_spacing.
    #[serde(rename="lineSpacingSuggested")]
    pub line_spacing_suggested: Option<bool>,
    /// Indicates if there was a suggested change to spacing_mode.
    #[serde(rename="spacingModeSuggested")]
    pub spacing_mode_suggested: Option<bool>,
    /// Indicates if there was a suggested change to indent_start.
    #[serde(rename="indentStartSuggested")]
    pub indent_start_suggested: Option<bool>,
    /// Indicates if there was a suggested change to space_below.
    #[serde(rename="spaceBelowSuggested")]
    pub space_below_suggested: Option<bool>,
    /// Indicates if there was a suggested change to border_left.
    #[serde(rename="borderLeftSuggested")]
    pub border_left_suggested: Option<bool>,
    /// Indicates if there was a suggested change to indent_first_line.
    #[serde(rename="indentFirstLineSuggested")]
    pub indent_first_line_suggested: Option<bool>,
    /// Indicates if there was a suggested change to heading_id.
    #[serde(rename="headingIdSuggested")]
    pub heading_id_suggested: Option<bool>,
}

impl Part for ParagraphStyleSuggestionState {}


/// A tab stop within a paragraph.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TabStop {
    /// The alignment of this tab stop. If unset, the value defaults to START.
    pub alignment: Option<String>,
    /// The offset between this tab stop and the start margin.
    pub offset: Option<Dimension>,
}

impl Part for TabStop {}


/// Properties of an InlineObject.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InlineObjectProperties {
    /// The embedded object of this inline object.
    #[serde(rename="embeddedObject")]
    pub embedded_object: Option<EmbeddedObject>,
}

impl Part for InlineObjectProperties {}


/// Represents the background of a document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Background {
    /// The background color.
    pub color: Option<OptionalColor>,
}

impl Part for Background {}


/// A suggested change to a TextStyle.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SuggestedTextStyle {
    /// A TextStyle that only includes
    /// the changes made in this suggestion. This can be used along with the
    /// text_style_suggestion_state
    /// to see which fields have changed and their new values.
    #[serde(rename="textStyle")]
    pub text_style: Option<TextStyle>,
    /// A mask that indicates which of the fields on the base TextStyle have been changed in this suggestion.
    #[serde(rename="textStyleSuggestionState")]
    pub text_style_suggestion_state: Option<TextStyleSuggestionState>,
}

impl Part for SuggestedTextStyle {}


/// A mask that indicates which of the fields on the base
/// PositionedObjectProperties
/// have been changed in this suggestion. For any field set to true, there is a
/// new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PositionedObjectPropertiesSuggestionState {
    /// A mask that indicates which of the fields in positioning have been
    /// changed in this suggestion.
    #[serde(rename="positioningSuggestionState")]
    pub positioning_suggestion_state: Option<PositionedObjectPositioningSuggestionState>,
    /// A mask that indicates which of the fields in embedded_object have been
    /// changed in this suggestion.
    #[serde(rename="embeddedObjectSuggestionState")]
    pub embedded_object_suggestion_state: Option<EmbeddedObjectSuggestionState>,
}

impl Part for PositionedObjectPropertiesSuggestionState {}


/// A suggested change to PositionedObjectProperties.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SuggestedPositionedObjectProperties {
    /// A PositionedObjectProperties that only includes the
    /// changes made in this suggestion. This can be used along with the
    /// positioned_object_properties_suggestion_state
    /// to see which fields have changed and their new values.
    #[serde(rename="positionedObjectProperties")]
    pub positioned_object_properties: Option<PositionedObjectProperties>,
    /// A mask that indicates which of the fields on the base
    /// PositionedObjectProperties have been changed in this
    /// suggestion.
    #[serde(rename="positionedObjectPropertiesSuggestionState")]
    pub positioned_object_properties_suggestion_state: Option<PositionedObjectPropertiesSuggestionState>,
}

impl Part for SuggestedPositionedObjectProperties {}


/// A List represents the list attributes for a group of paragraphs that all
/// belong to the same list. A paragraph that is part of a list has a reference
/// to the list's ID in its bullet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct List {
    /// The properties of the list.
    #[serde(rename="listProperties")]
    pub list_properties: Option<ListProperties>,
    /// The suggested deletion IDs. If empty, then there are no suggested deletions
    /// of this list.
    #[serde(rename="suggestedDeletionIds")]
    pub suggested_deletion_ids: Option<Vec<String>>,
    /// The suggested changes to the list properties, keyed by suggestion
    /// ID.
    #[serde(rename="suggestedListPropertiesChanges")]
    pub suggested_list_properties_changes: Option<HashMap<String, SuggestedListProperties>>,
    /// The suggested insertion ID. If empty, then this is not a suggested
    /// insertion.
    #[serde(rename="suggestedInsertionId")]
    pub suggested_insertion_id: Option<String>,
}

impl Part for List {}


/// The properties of a list which describe the look
/// and feel of bullets belonging to paragraphs associated with a list.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListProperties {
    /// Describes the properties of the bullets at the associated level.
    /// 
    /// A list has at most nine levels of nesting with nesting level 0
    /// corresponding to the top-most level and nesting level 8 corresponding to
    /// the most nested level. The nesting levels are returned in ascending order
    /// with the least nested returned first.
    #[serde(rename="nestingLevels")]
    pub nesting_levels: Option<Vec<NestingLevel>>,
}

impl Part for ListProperties {}


/// A ParagraphElement representing a
/// page break. A page break makes the subsequent text start at the top of the
/// next page.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PageBreak {
    /// The text style of this PageBreak.
    /// 
    /// Similar to text content, like text runs and footnote references, the text
    /// style of a page break can affect content layout as well as the styling of
    /// text inserted adjacent to it.
    #[serde(rename="textStyle")]
    pub text_style: Option<TextStyle>,
    /// The suggested deletion IDs. If empty, then there are no suggested deletions
    /// of this content.
    #[serde(rename="suggestedDeletionIds")]
    pub suggested_deletion_ids: Option<Vec<String>>,
    /// The suggested text style changes to this PageBreak, keyed by suggestion ID.
    #[serde(rename="suggestedTextStyleChanges")]
    pub suggested_text_style_changes: Option<HashMap<String, SuggestedTextStyle>>,
    /// The suggested insertion IDs. A PageBreak
    /// may have multiple insertion IDs if it is a nested suggested change. If
    /// empty, then this is not a suggested insertion.
    #[serde(rename="suggestedInsertionIds")]
    pub suggested_insertion_ids: Option<Vec<String>>,
}

impl Part for PageBreak {}


/// An embedded object in the document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EmbeddedObject {
    /// The properties of an image.
    #[serde(rename="imageProperties")]
    pub image_properties: Option<ImageProperties>,
    /// The description of the embedded object. The `title` and `description` are
    /// both combined to display alt text.
    pub description: Option<String>,
    /// The title of the embedded object. The `title` and `description` are both
    /// combined to display alt text.
    pub title: Option<String>,
    /// The properties of an embedded drawing.
    #[serde(rename="embeddedDrawingProperties")]
    pub embedded_drawing_properties: Option<EmbeddedDrawingProperties>,
    /// The bottom margin of the embedded object.
    #[serde(rename="marginBottom")]
    pub margin_bottom: Option<Dimension>,
    /// The border of the embedded object.
    #[serde(rename="embeddedObjectBorder")]
    pub embedded_object_border: Option<EmbeddedObjectBorder>,
    /// A reference to the external linked source content. For example, it contains
    /// a reference to the source Sheets chart when the embedded object is a linked
    /// chart.
    /// 
    /// If unset, then the embedded object is not linked.
    #[serde(rename="linkedContentReference")]
    pub linked_content_reference: Option<LinkedContentReference>,
    /// The left margin of the embedded object.
    #[serde(rename="marginLeft")]
    pub margin_left: Option<Dimension>,
    /// The right margin of the embedded object.
    #[serde(rename="marginRight")]
    pub margin_right: Option<Dimension>,
    /// The top margin of the embedded object.
    #[serde(rename="marginTop")]
    pub margin_top: Option<Dimension>,
    /// The visible size of the image after cropping.
    pub size: Option<Size>,
}

impl Part for EmbeddedObject {}


/// A suggested change to the NamedStyles.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SuggestedNamedStyles {
    /// A mask that indicates which of the fields on the base NamedStyles have been changed in this suggestion.
    #[serde(rename="namedStylesSuggestionState")]
    pub named_styles_suggestion_state: Option<NamedStylesSuggestionState>,
    /// A NamedStyles that only includes the
    /// changes made in this suggestion. This can be used along with the
    /// named_styles_suggestion_state to
    /// see which fields have changed and their new values.
    #[serde(rename="namedStyles")]
    pub named_styles: Option<NamedStyles>,
}

impl Part for SuggestedNamedStyles {}


/// An object that appears inline with text. An InlineObject contains
/// an EmbeddedObject such as an image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InlineObject {
    /// The suggested deletion IDs. If empty, then there are no suggested deletions
    /// of this content.
    #[serde(rename="suggestedDeletionIds")]
    pub suggested_deletion_ids: Option<Vec<String>>,
    /// The properties of this inline object.
    #[serde(rename="inlineObjectProperties")]
    pub inline_object_properties: Option<InlineObjectProperties>,
    /// The suggested changes to the inline object properties, keyed by suggestion
    /// ID.
    #[serde(rename="suggestedInlineObjectPropertiesChanges")]
    pub suggested_inline_object_properties_changes: Option<HashMap<String, SuggestedInlineObjectProperties>>,
    /// The ID of this inline object.
    #[serde(rename="objectId")]
    pub object_id: Option<String>,
    /// The suggested insertion ID. If empty, then this is not a suggested
    /// insertion.
    #[serde(rename="suggestedInsertionId")]
    pub suggested_insertion_id: Option<String>,
}

impl Part for InlineObject {}


/// A suggested change to InlineObjectProperties.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SuggestedInlineObjectProperties {
    /// An InlineObjectProperties
    /// that only includes the changes made in this suggestion. This can be used
    /// along with the inline_object_properties_suggestion_state
    /// to see which fields have changed and their new values.
    #[serde(rename="inlineObjectProperties")]
    pub inline_object_properties: Option<InlineObjectProperties>,
    /// A mask that indicates which of the fields on the base
    /// InlineObjectProperties have
    /// been changed in this suggestion.
    #[serde(rename="inlineObjectPropertiesSuggestionState")]
    pub inline_object_properties_suggestion_state: Option<InlineObjectPropertiesSuggestionState>,
}

impl Part for SuggestedInlineObjectProperties {}


/// A ParagraphElement describes content within a
/// Paragraph.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ParagraphElement {
    /// The zero-base end index of this paragraph element, exclusive, in UTF-16
    /// code units.
    #[serde(rename="endIndex")]
    pub end_index: Option<i32>,
    /// An equation paragraph element.
    pub equation: Option<Equation>,
    /// A column break paragraph element.
    #[serde(rename="columnBreak")]
    pub column_break: Option<ColumnBreak>,
    /// The zero-based start index of this paragraph element, in UTF-16 code units.
    #[serde(rename="startIndex")]
    pub start_index: Option<i32>,
    /// A page break paragraph element.
    #[serde(rename="pageBreak")]
    pub page_break: Option<PageBreak>,
    /// A horizontal rule paragraph element.
    #[serde(rename="horizontalRule")]
    pub horizontal_rule: Option<HorizontalRule>,
    /// A text run paragraph element.
    #[serde(rename="textRun")]
    pub text_run: Option<TextRun>,
    /// An auto text paragraph element.
    #[serde(rename="autoText")]
    pub auto_text: Option<AutoText>,
    /// An inline object paragraph element.
    #[serde(rename="inlineObjectElement")]
    pub inline_object_element: Option<InlineObjectElement>,
    /// A footnote reference paragraph element.
    #[serde(rename="footnoteReference")]
    pub footnote_reference: Option<FootnoteReference>,
}

impl Part for ParagraphElement {}


/// A suggested change to the DocumentStyle.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SuggestedDocumentStyle {
    /// A DocumentStyle that only includes
    /// the changes made in this suggestion. This can be used along with the
    /// document_style_suggestion_state
    /// to see which fields have changed and their new values.
    #[serde(rename="documentStyle")]
    pub document_style: Option<DocumentStyle>,
    /// A mask that indicates which of the fields on the base DocumentStyle have been changed in this suggestion.
    #[serde(rename="documentStyleSuggestionState")]
    pub document_style_suggestion_state: Option<DocumentStyleSuggestionState>,
}

impl Part for SuggestedDocumentStyle {}


/// A solid color.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Color {
    /// The RGB color value.
    #[serde(rename="rgbColor")]
    pub rgb_color: Option<RgbColor>,
}

impl Part for Color {}


/// The suggestion state of a NamedStyles
/// message.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NamedStylesSuggestionState {
    /// A mask that indicates which of the fields on the corresponding NamedStyle in styles have been changed in this
    /// suggestion.
    /// 
    /// The order of these named style suggestion states match the order of the
    /// corresponding named style within the named styles suggestion.
    #[serde(rename="stylesSuggestionStates")]
    pub styles_suggestion_states: Option<Vec<NamedStyleSuggestionState>>,
}

impl Part for NamedStylesSuggestionState {}


/// An RGB color.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RgbColor {
    /// The blue component of the color, from 0.0 to 1.0.
    pub blue: Option<f32>,
    /// The green component of the color, from 0.0 to 1.0.
    pub green: Option<f32>,
    /// The red component of the color, from 0.0 to 1.0.
    pub red: Option<f32>,
}

impl Part for RgbColor {}


/// Update the styling of all paragraphs that overlap with the given range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateParagraphStyleRequest {
    /// The fields that should be updated.
    /// 
    /// At least one field must be specified. The root `paragraph_style` is implied
    /// and should not be specified.
    /// 
    /// For example, to update the paragraph style's alignment property, set
    /// `fields` to `"alignment"`.
    /// 
    /// To reset a property to its default value, include its field name in the
    /// field mask but leave the field itself unset.
    pub fields: Option<String>,
    /// The range overlapping the paragraphs to style.
    pub range: Option<Range>,
    /// The styles to set on the paragraphs.
    /// 
    /// Certain paragraph style changes may cause other changes in order to mirror
    /// the behavior of the Docs editor. See the documentation of ParagraphStyle for more information.
    #[serde(rename="paragraphStyle")]
    pub paragraph_style: Option<ParagraphStyle>,
}

impl Part for UpdateParagraphStyleRequest {}


/// The result of creating a named range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateNamedRangeResponse {
    /// The ID of the created named range.
    #[serde(rename="namedRangeId")]
    pub named_range_id: Option<String>,
}

impl Part for CreateNamedRangeResponse {}


/// A StructuralElement representing a
/// table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Table {
    /// Number of rows in the table.
    pub rows: Option<i32>,
    /// The suggested deletion IDs. If empty, then there are no suggested deletions
    /// of this content.
    #[serde(rename="suggestedDeletionIds")]
    pub suggested_deletion_ids: Option<Vec<String>>,
    /// The style of the table.
    #[serde(rename="tableStyle")]
    pub table_style: Option<TableStyle>,
    /// The contents and style of each row.
    #[serde(rename="tableRows")]
    pub table_rows: Option<Vec<TableRow>>,
    /// Number of columns in the table.
    /// 
    /// It is possible for a table to be non-rectangular, so some rows may have a
    /// different number of cells.
    pub columns: Option<i32>,
    /// The suggested insertion IDs. A Table may have
    /// multiple insertion IDs if it is a nested suggested change. If empty, then
    /// this is not a suggested insertion.
    #[serde(rename="suggestedInsertionIds")]
    pub suggested_insertion_ids: Option<Vec<String>>,
}

impl Part for Table {}


/// A collection of all the NamedRanges in the
/// document that share a given name.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NamedRanges {
    /// The NamedRanges that share the same name.
    #[serde(rename="namedRanges")]
    pub named_ranges: Option<Vec<NamedRange>>,
    /// The name that all the named ranges share.
    pub name: Option<String>,
}

impl Part for NamedRanges {}


/// Response message from a BatchUpdateDocument request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch update documents](struct.DocumentBatchUpdateCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchUpdateDocumentResponse {
    /// The updated write control after applying the request.
    #[serde(rename="writeControl")]
    pub write_control: Option<WriteControl>,
    /// The ID of the document to which the updates were applied to.
    #[serde(rename="documentId")]
    pub document_id: Option<String>,
    /// The reply of the updates. This maps 1:1 with the updates, although replies
    /// to some requests may be empty.
    pub replies: Option<Vec<Response>>,
}

impl ResponseResult for BatchUpdateDocumentResponse {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *document* resources.
/// It is not used directly, but through the `Docs` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_docs1 as docs1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use docs1::Docs;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Docs::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `batch_update(...)`, `create(...)` and `get(...)`
/// // to build up your call.
/// let rb = hub.documents();
/// # }
/// ```
pub struct DocumentMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Docs<C, A>,
}

impl<'a, C, A> MethodsBuilder for DocumentMethods<'a, C, A> {}

impl<'a, C, A> DocumentMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a blank document using the title given in the request. Other fields
    /// in the request, including any provided content, are ignored.
    /// 
    /// Returns the created document.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: Document) -> DocumentCreateCall<'a, C, A> {
        DocumentCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the latest version of the specified document.
    /// 
    /// # Arguments
    ///
    /// * `documentId` - The ID of the document to retrieve.
    pub fn get(&self, document_id: &str) -> DocumentGetCall<'a, C, A> {
        DocumentGetCall {
            hub: self.hub,
            _document_id: document_id.to_string(),
            _suggestions_view_mode: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Applies one or more updates to the document.
    /// 
    /// Each request is validated before
    /// being applied. If any request is not valid, then the entire request will
    /// fail and nothing will be applied.
    /// 
    /// Some requests have replies to
    /// give you some information about how they are applied. Other requests do
    /// not need to return information; these each return an empty reply.
    /// The order of replies matches that of the requests.
    /// 
    /// For example, suppose you call batchUpdate with four updates, and only the
    /// third one returns information. The response would have two empty replies,
    /// the reply to the third request, and another empty reply, in that order.
    /// 
    /// Because other users may be editing the document, the document
    /// might not exactly reflect your changes: your changes may
    /// be altered with respect to collaborator changes. If there are no
    /// collaborators, the document should reflect your changes. In any case,
    /// the updates in your request are guaranteed to be applied together
    /// atomically.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `documentId` - The ID of the document to update.
    pub fn batch_update(&self, request: BatchUpdateDocumentRequest, document_id: &str) -> DocumentBatchUpdateCall<'a, C, A> {
        DocumentBatchUpdateCall {
            hub: self.hub,
            _request: request,
            _document_id: document_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Creates a blank document using the title given in the request. Other fields
/// in the request, including any provided content, are ignored.
/// 
/// Returns the created document.
///
/// A builder for the *create* method supported by a *document* resource.
/// It is not used directly, but through a `DocumentMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_docs1 as docs1;
/// use docs1::Document;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use docs1::Docs;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Docs::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Document::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.documents().create(req)
///              .doit();
/// # }
/// ```
pub struct DocumentCreateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Docs<C, A>,
    _request: Document,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for DocumentCreateCall<'a, C, A> {}

impl<'a, C, A> DocumentCreateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Document)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "docs.documents.create",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/documents";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Document.as_ref().to_string(), ());
        }


        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
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
    pub fn request(mut self, new_value: Document) -> DocumentCreateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DocumentCreateCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> DocumentCreateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Document`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> DocumentCreateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets the latest version of the specified document.
///
/// A builder for the *get* method supported by a *document* resource.
/// It is not used directly, but through a `DocumentMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_docs1 as docs1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use docs1::Docs;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Docs::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.documents().get("documentId")
///              .suggestions_view_mode("accusam")
///              .doit();
/// # }
/// ```
pub struct DocumentGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Docs<C, A>,
    _document_id: String,
    _suggestions_view_mode: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for DocumentGetCall<'a, C, A> {}

impl<'a, C, A> DocumentGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Document)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "docs.documents.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("documentId", self._document_id.to_string()));
        if let Some(value) = self._suggestions_view_mode {
            params.push(("suggestionsViewMode", value.to_string()));
        }
        for &field in ["alt", "documentId", "suggestionsViewMode"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/documents/{documentId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DocumentReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{documentId}", "documentId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["documentId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the document to retrieve.
    ///
    /// Sets the *document id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn document_id(mut self, new_value: &str) -> DocumentGetCall<'a, C, A> {
        self._document_id = new_value.to_string();
        self
    }
    /// The suggestions view mode to apply to the document. This allows viewing the
    /// document with all suggestions inline, accepted or rejected. If one is not
    /// specified, DEFAULT_FOR_CURRENT_ACCESS is
    /// used.
    ///
    /// Sets the *suggestions view mode* query property to the given value.
    pub fn suggestions_view_mode(mut self, new_value: &str) -> DocumentGetCall<'a, C, A> {
        self._suggestions_view_mode = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DocumentGetCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> DocumentGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::DocumentReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> DocumentGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Applies one or more updates to the document.
/// 
/// Each request is validated before
/// being applied. If any request is not valid, then the entire request will
/// fail and nothing will be applied.
/// 
/// Some requests have replies to
/// give you some information about how they are applied. Other requests do
/// not need to return information; these each return an empty reply.
/// The order of replies matches that of the requests.
/// 
/// For example, suppose you call batchUpdate with four updates, and only the
/// third one returns information. The response would have two empty replies,
/// the reply to the third request, and another empty reply, in that order.
/// 
/// Because other users may be editing the document, the document
/// might not exactly reflect your changes: your changes may
/// be altered with respect to collaborator changes. If there are no
/// collaborators, the document should reflect your changes. In any case,
/// the updates in your request are guaranteed to be applied together
/// atomically.
///
/// A builder for the *batchUpdate* method supported by a *document* resource.
/// It is not used directly, but through a `DocumentMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_docs1 as docs1;
/// use docs1::BatchUpdateDocumentRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use docs1::Docs;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Docs::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BatchUpdateDocumentRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.documents().batch_update(req, "documentId")
///              .doit();
/// # }
/// ```
pub struct DocumentBatchUpdateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Docs<C, A>,
    _request: BatchUpdateDocumentRequest,
    _document_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for DocumentBatchUpdateCall<'a, C, A> {}

impl<'a, C, A> DocumentBatchUpdateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, BatchUpdateDocumentResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "docs.documents.batchUpdate",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("documentId", self._document_id.to_string()));
        for &field in ["alt", "documentId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/documents/{documentId}:batchUpdate";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Document.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{documentId}", "documentId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["documentId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
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
    pub fn request(mut self, new_value: BatchUpdateDocumentRequest) -> DocumentBatchUpdateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the document to update.
    ///
    /// Sets the *document id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn document_id(mut self, new_value: &str) -> DocumentBatchUpdateCall<'a, C, A> {
        self._document_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DocumentBatchUpdateCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> DocumentBatchUpdateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Document`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> DocumentBatchUpdateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


