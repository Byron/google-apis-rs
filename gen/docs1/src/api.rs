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
    /// See, edit, create, and delete all your Google Docs documents
    Document,

    /// See all your Google Docs documents
    DocumentReadonly,

    /// See, edit, create, and delete all of your Google Drive files
    Drive,

    /// See, edit, create, and delete only the specific Google Drive files you use with this app
    DriveFile,

    /// See and download all your Google Drive files
    DriveReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Document => "https://www.googleapis.com/auth/documents",
            Scope::DocumentReadonly => "https://www.googleapis.com/auth/documents.readonly",
            Scope::Drive => "https://www.googleapis.com/auth/drive",
            Scope::DriveFile => "https://www.googleapis.com/auth/drive.file",
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
/// extern crate google_docs1 as docs1;
/// use docs1::api::BatchUpdateDocumentRequest;
/// use docs1::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use docs1::{Docs, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
/// let mut hub = Docs::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BatchUpdateDocumentRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.documents().batch_update(req, "documentId")
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
pub struct Docs<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: Box<dyn client::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, S> client::Hub for Docs<S> {}

impl<'a, S> Docs<S> {

    pub fn new<A: 'static + client::GetToken>(client: hyper::Client<S, hyper::body::Body>, auth: A) -> Docs<S> {
        Docs {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/5.0.5".to_string(),
            _base_url: "https://docs.googleapis.com/".to_string(),
            _root_url: "https://docs.googleapis.com/".to_string(),
        }
    }

    pub fn documents(&'a self) -> DocumentMethods<'a, S> {
        DocumentMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/5.0.5`.
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
/// A ParagraphElement representing a spot in the text that's dynamically replaced with content that can change over time, like a page number.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AutoText {
    /// The suggested deletion IDs. If empty, then there are no suggested deletions of this content.
    #[serde(rename="suggestedDeletionIds")]
    
    pub suggested_deletion_ids: Option<Vec<String>>,
    /// The suggested insertion IDs. An AutoText may have multiple insertion IDs if it's a nested suggested change. If empty, then this is not a suggested insertion.
    #[serde(rename="suggestedInsertionIds")]
    
    pub suggested_insertion_ids: Option<Vec<String>>,
    /// The suggested text style changes to this AutoText, keyed by suggestion ID.
    #[serde(rename="suggestedTextStyleChanges")]
    
    pub suggested_text_style_changes: Option<HashMap<String, SuggestedTextStyle>>,
    /// The text style of this AutoText.
    #[serde(rename="textStyle")]
    
    pub text_style: Option<TextStyle>,
    /// The type of this auto text.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for AutoText {}


/// Represents the background of a document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Background {
    /// The background color.
    
    pub color: Option<OptionalColor>,
}

impl client::Part for Background {}


/// A mask that indicates which of the fields on the base Background have been changed in this suggestion. For any field set to true, the Backgound has a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BackgroundSuggestionState {
    /// Indicates whether the current background color has been modified in this suggestion.
    #[serde(rename="backgroundColorSuggested")]
    
    pub background_color_suggested: Option<bool>,
}

impl client::Part for BackgroundSuggestionState {}


/// Request message for BatchUpdateDocument.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch update documents](DocumentBatchUpdateCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchUpdateDocumentRequest {
    /// A list of updates to apply to the document.
    
    pub requests: Option<Vec<Request>>,
    /// Provides control over how write requests are executed.
    #[serde(rename="writeControl")]
    
    pub write_control: Option<WriteControl>,
}

impl client::RequestValue for BatchUpdateDocumentRequest {}


/// Response message from a BatchUpdateDocument request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch update documents](DocumentBatchUpdateCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchUpdateDocumentResponse {
    /// The ID of the document to which the updates were applied to.
    #[serde(rename="documentId")]
    
    pub document_id: Option<String>,
    /// The reply of the updates. This maps 1:1 with the updates, although replies to some requests may be empty.
    
    pub replies: Option<Vec<Response>>,
    /// The updated write control after applying the request.
    #[serde(rename="writeControl")]
    
    pub write_control: Option<WriteControl>,
}

impl client::ResponseResult for BatchUpdateDocumentResponse {}


/// The document body. The body typically contains the full document contents except for headers, footers, and footnotes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Body {
    /// The contents of the body. The indexes for the body's content begin at zero.
    
    pub content: Option<Vec<StructuralElement>>,
}

impl client::Part for Body {}


/// Describes the bullet of a paragraph.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Bullet {
    /// The ID of the list this paragraph belongs to.
    #[serde(rename="listId")]
    
    pub list_id: Option<String>,
    /// The nesting level of this paragraph in the list.
    #[serde(rename="nestingLevel")]
    
    pub nesting_level: Option<i32>,
    /// The paragraph-specific text style applied to this bullet.
    #[serde(rename="textStyle")]
    
    pub text_style: Option<TextStyle>,
}

impl client::Part for Bullet {}


/// A mask that indicates which of the fields on the base Bullet have been changed in this suggestion. For any field set to true, there's a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BulletSuggestionState {
    /// Indicates if there was a suggested change to the list_id.
    #[serde(rename="listIdSuggested")]
    
    pub list_id_suggested: Option<bool>,
    /// Indicates if there was a suggested change to the nesting_level.
    #[serde(rename="nestingLevelSuggested")]
    
    pub nesting_level_suggested: Option<bool>,
    /// A mask that indicates which of the fields in text style have been changed in this suggestion.
    #[serde(rename="textStyleSuggestionState")]
    
    pub text_style_suggestion_state: Option<TextStyleSuggestionState>,
}

impl client::Part for BulletSuggestionState {}


/// A solid color.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Color {
    /// The RGB color value.
    #[serde(rename="rgbColor")]
    
    pub rgb_color: Option<RgbColor>,
}

impl client::Part for Color {}


/// A ParagraphElement representing a column break. A column break makes the subsequent text start at the top of the next column.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ColumnBreak {
    /// The suggested deletion IDs. If empty, then there are no suggested deletions of this content.
    #[serde(rename="suggestedDeletionIds")]
    
    pub suggested_deletion_ids: Option<Vec<String>>,
    /// The suggested insertion IDs. A ColumnBreak may have multiple insertion IDs if it's a nested suggested change. If empty, then this is not a suggested insertion.
    #[serde(rename="suggestedInsertionIds")]
    
    pub suggested_insertion_ids: Option<Vec<String>>,
    /// The suggested text style changes to this ColumnBreak, keyed by suggestion ID.
    #[serde(rename="suggestedTextStyleChanges")]
    
    pub suggested_text_style_changes: Option<HashMap<String, SuggestedTextStyle>>,
    /// The text style of this ColumnBreak. Similar to text content, like text runs and footnote references, the text style of a column break can affect content layout as well as the styling of text inserted next to it.
    #[serde(rename="textStyle")]
    
    pub text_style: Option<TextStyle>,
}

impl client::Part for ColumnBreak {}


/// Creates a Footer. The new footer is applied to the SectionStyle at the location of the SectionBreak if specified, otherwise it is applied to the DocumentStyle. If a footer of the specified type already exists, a 400 bad request error is returned.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateFooterRequest {
    /// The location of the SectionBreak immediately preceding the section whose SectionStyle this footer should belong to. If this is unset or refers to the first section break in the document, the footer applies to the document style.
    #[serde(rename="sectionBreakLocation")]
    
    pub section_break_location: Option<Location>,
    /// The type of footer to create.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for CreateFooterRequest {}


/// The result of creating a footer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateFooterResponse {
    /// The ID of the created footer.
    #[serde(rename="footerId")]
    
    pub footer_id: Option<String>,
}

impl client::Part for CreateFooterResponse {}


/// Creates a Footnote segment and inserts a new FootnoteReference to it at the given location. The new Footnote segment will contain a space followed by a newline character.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateFootnoteRequest {
    /// Inserts the footnote reference at the end of the document body. Footnote references cannot be inserted inside a header, footer or footnote. Since footnote references can only be inserted in the body, the segment ID field must be empty.
    #[serde(rename="endOfSegmentLocation")]
    
    pub end_of_segment_location: Option<EndOfSegmentLocation>,
    /// Inserts the footnote reference at a specific index in the document. The footnote reference must be inserted inside the bounds of an existing Paragraph. For instance, it cannot be inserted at a table's start index (i.e. between the table and its preceding paragraph). Footnote references cannot be inserted inside an equation, header, footer or footnote. Since footnote references can only be inserted in the body, the segment ID field must be empty.
    
    pub location: Option<Location>,
}

impl client::Part for CreateFootnoteRequest {}


/// The result of creating a footnote.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateFootnoteResponse {
    /// The ID of the created footnote.
    #[serde(rename="footnoteId")]
    
    pub footnote_id: Option<String>,
}

impl client::Part for CreateFootnoteResponse {}


/// Creates a Header. The new header is applied to the SectionStyle at the location of the SectionBreak if specified, otherwise it is applied to the DocumentStyle. If a header of the specified type already exists, a 400 bad request error is returned.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateHeaderRequest {
    /// The location of the SectionBreak which begins the section this header should belong to. If `section_break_location' is unset or if it refers to the first section break in the document body, the header applies to the DocumentStyle
    #[serde(rename="sectionBreakLocation")]
    
    pub section_break_location: Option<Location>,
    /// The type of header to create.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for CreateHeaderRequest {}


/// The result of creating a header.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateHeaderResponse {
    /// The ID of the created header.
    #[serde(rename="headerId")]
    
    pub header_id: Option<String>,
}

impl client::Part for CreateHeaderResponse {}


/// Creates a NamedRange referencing the given range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateNamedRangeRequest {
    /// The name of the NamedRange. Names do not need to be unique. Names must be at least 1 character and no more than 256 characters, measured in UTF-16 code units.
    
    pub name: Option<String>,
    /// The range to apply the name to.
    
    pub range: Option<Range>,
}

impl client::Part for CreateNamedRangeRequest {}


/// The result of creating a named range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateNamedRangeResponse {
    /// The ID of the created named range.
    #[serde(rename="namedRangeId")]
    
    pub named_range_id: Option<String>,
}

impl client::Part for CreateNamedRangeResponse {}


/// Creates bullets for all of the paragraphs that overlap with the given range. The nesting level of each paragraph will be determined by counting leading tabs in front of each paragraph. To avoid excess space between the bullet and the corresponding paragraph, these leading tabs are removed by this request. This may change the indices of parts of the text. If the paragraph immediately before paragraphs being updated is in a list with a matching preset, the paragraphs being updated are added to that preceding list.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateParagraphBulletsRequest {
    /// The kinds of bullet glyphs to be used.
    #[serde(rename="bulletPreset")]
    
    pub bullet_preset: Option<String>,
    /// The range to apply the bullet preset to.
    
    pub range: Option<Range>,
}

impl client::Part for CreateParagraphBulletsRequest {}


/// The crop properties of an image. The crop rectangle is represented using fractional offsets from the original content's 4 edges. - If the offset is in the interval (0, 1), the corresponding edge of crop rectangle is positioned inside of the image's original bounding rectangle. - If the offset is negative or greater than 1, the corresponding edge of crop rectangle is positioned outside of the image's original bounding rectangle. - If all offsets and rotation angle are 0, the image is not cropped.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CropProperties {
    /// The clockwise rotation angle of the crop rectangle around its center, in radians. Rotation is applied after the offsets.
    
    pub angle: Option<f32>,
    /// The offset specifies how far inwards the bottom edge of the crop rectangle is from the bottom edge of the original content as a fraction of the original content's height.
    #[serde(rename="offsetBottom")]
    
    pub offset_bottom: Option<f32>,
    /// The offset specifies how far inwards the left edge of the crop rectangle is from the left edge of the original content as a fraction of the original content's width.
    #[serde(rename="offsetLeft")]
    
    pub offset_left: Option<f32>,
    /// The offset specifies how far inwards the right edge of the crop rectangle is from the right edge of the original content as a fraction of the original content's width.
    #[serde(rename="offsetRight")]
    
    pub offset_right: Option<f32>,
    /// The offset specifies how far inwards the top edge of the crop rectangle is from the top edge of the original content as a fraction of the original content's height.
    #[serde(rename="offsetTop")]
    
    pub offset_top: Option<f32>,
}

impl client::Part for CropProperties {}


/// A mask that indicates which of the fields on the base CropProperties have been changed in this suggestion. For any field set to true, there's a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CropPropertiesSuggestionState {
    /// Indicates if there was a suggested change to angle.
    #[serde(rename="angleSuggested")]
    
    pub angle_suggested: Option<bool>,
    /// Indicates if there was a suggested change to offset_bottom.
    #[serde(rename="offsetBottomSuggested")]
    
    pub offset_bottom_suggested: Option<bool>,
    /// Indicates if there was a suggested change to offset_left.
    #[serde(rename="offsetLeftSuggested")]
    
    pub offset_left_suggested: Option<bool>,
    /// Indicates if there was a suggested change to offset_right.
    #[serde(rename="offsetRightSuggested")]
    
    pub offset_right_suggested: Option<bool>,
    /// Indicates if there was a suggested change to offset_top.
    #[serde(rename="offsetTopSuggested")]
    
    pub offset_top_suggested: Option<bool>,
}

impl client::Part for CropPropertiesSuggestionState {}


/// Deletes content from the document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteContentRangeRequest {
    /// The range of content to delete. Deleting text that crosses a paragraph boundary may result in changes to paragraph styles, lists, positioned objects and bookmarks as the two paragraphs are merged. Attempting to delete certain ranges can result in an invalid document structure in which case a 400 bad request error is returned. Some examples of invalid delete requests include: * Deleting one code unit of a surrogate pair. * Deleting the last newline character of a Body, Header, Footer, Footnote, TableCell or TableOfContents. * Deleting the start or end of a Table, TableOfContents or Equation without deleting the entire element. * Deleting the newline character before a Table, TableOfContents or SectionBreak without deleting the element. * Deleting individual rows or cells of a table. Deleting the content within a table cell is allowed.
    
    pub range: Option<Range>,
}

impl client::Part for DeleteContentRangeRequest {}


/// Deletes a Footer from the document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteFooterRequest {
    /// The id of the footer to delete. If this footer is defined on DocumentStyle, the reference to this footer is removed, resulting in no footer of that type for the first section of the document. If this footer is defined on a SectionStyle, the reference to this footer is removed and the footer of that type is now continued from the previous section.
    #[serde(rename="footerId")]
    
    pub footer_id: Option<String>,
}

impl client::Part for DeleteFooterRequest {}


/// Deletes a Header from the document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteHeaderRequest {
    /// The id of the header to delete. If this header is defined on DocumentStyle, the reference to this header is removed, resulting in no header of that type for the first section of the document. If this header is defined on a SectionStyle, the reference to this header is removed and the header of that type is now continued from the previous section.
    #[serde(rename="headerId")]
    
    pub header_id: Option<String>,
}

impl client::Part for DeleteHeaderRequest {}


/// Deletes a NamedRange.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteNamedRangeRequest {
    /// The name of the range(s) to delete. All named ranges with the given name will be deleted.
    
    pub name: Option<String>,
    /// The ID of the named range to delete.
    #[serde(rename="namedRangeId")]
    
    pub named_range_id: Option<String>,
}

impl client::Part for DeleteNamedRangeRequest {}


/// Deletes bullets from all of the paragraphs that overlap with the given range. The nesting level of each paragraph will be visually preserved by adding indent to the start of the corresponding paragraph.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteParagraphBulletsRequest {
    /// The range to delete bullets from.
    
    pub range: Option<Range>,
}

impl client::Part for DeleteParagraphBulletsRequest {}


/// Deletes a PositionedObject from the document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeletePositionedObjectRequest {
    /// The ID of the positioned object to delete.
    #[serde(rename="objectId")]
    
    pub object_id: Option<String>,
}

impl client::Part for DeletePositionedObjectRequest {}


/// Deletes a column from a table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteTableColumnRequest {
    /// The reference table cell location from which the column will be deleted. The column this cell spans will be deleted. If this is a merged cell that spans multiple columns, all columns that the cell spans will be deleted. If no columns remain in the table after this deletion, the whole table is deleted.
    #[serde(rename="tableCellLocation")]
    
    pub table_cell_location: Option<TableCellLocation>,
}

impl client::Part for DeleteTableColumnRequest {}


/// Deletes a row from a table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteTableRowRequest {
    /// The reference table cell location from which the row will be deleted. The row this cell spans will be deleted. If this is a merged cell that spans multiple rows, all rows that the cell spans will be deleted. If no rows remain in the table after this deletion, the whole table is deleted.
    #[serde(rename="tableCellLocation")]
    
    pub table_cell_location: Option<TableCellLocation>,
}

impl client::Part for DeleteTableRowRequest {}


/// A magnitude in a single direction in the specified units.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Dimension {
    /// The magnitude.
    
    pub magnitude: Option<f64>,
    /// The units for magnitude.
    
    pub unit: Option<String>,
}

impl client::Part for Dimension {}


/// A Google Docs document.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch update documents](DocumentBatchUpdateCall) (none)
/// * [create documents](DocumentCreateCall) (request|response)
/// * [get documents](DocumentGetCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Document {
    /// Output only. The main body of the document.
    
    pub body: Option<Body>,
    /// Output only. The ID of the document.
    #[serde(rename="documentId")]
    
    pub document_id: Option<String>,
    /// Output only. The style of the document.
    #[serde(rename="documentStyle")]
    
    pub document_style: Option<DocumentStyle>,
    /// Output only. The footers in the document, keyed by footer ID.
    
    pub footers: Option<HashMap<String, Footer>>,
    /// Output only. The footnotes in the document, keyed by footnote ID.
    
    pub footnotes: Option<HashMap<String, Footnote>>,
    /// Output only. The headers in the document, keyed by header ID.
    
    pub headers: Option<HashMap<String, Header>>,
    /// Output only. The inline objects in the document, keyed by object ID.
    #[serde(rename="inlineObjects")]
    
    pub inline_objects: Option<HashMap<String, InlineObject>>,
    /// Output only. The lists in the document, keyed by list ID.
    
    pub lists: Option<HashMap<String, List>>,
    /// Output only. The named ranges in the document, keyed by name.
    #[serde(rename="namedRanges")]
    
    pub named_ranges: Option<HashMap<String, NamedRanges>>,
    /// Output only. The named styles of the document.
    #[serde(rename="namedStyles")]
    
    pub named_styles: Option<NamedStyles>,
    /// Output only. The positioned objects in the document, keyed by object ID.
    #[serde(rename="positionedObjects")]
    
    pub positioned_objects: Option<HashMap<String, PositionedObject>>,
    /// Output only. The revision ID of the document. Can be used in update requests to specify which revision of a document to apply updates to and how the request should behave if the document has been edited since that revision. Only populated if the user has edit access to the document. The revision ID is not a sequential number but an opaque string. The format of the revision ID might change over time. A returned revision ID is only guaranteed to be valid for 24 hours after it has been returned and cannot be shared across users. If the revision ID is unchanged between calls, then the document has not changed. Conversely, a changed ID (for the same document and user) usually means the document has been updated. However, a changed ID can also be due to internal factors such as ID format changes.
    #[serde(rename="revisionId")]
    
    pub revision_id: Option<String>,
    /// Output only. The suggested changes to the style of the document, keyed by suggestion ID.
    #[serde(rename="suggestedDocumentStyleChanges")]
    
    pub suggested_document_style_changes: Option<HashMap<String, SuggestedDocumentStyle>>,
    /// Output only. The suggested changes to the named styles of the document, keyed by suggestion ID.
    #[serde(rename="suggestedNamedStylesChanges")]
    
    pub suggested_named_styles_changes: Option<HashMap<String, SuggestedNamedStyles>>,
    /// Output only. The suggestions view mode applied to the document. Note: When editing a document, changes must be based on a document with SUGGESTIONS_INLINE.
    #[serde(rename="suggestionsViewMode")]
    
    pub suggestions_view_mode: Option<String>,
    /// The title of the document.
    
    pub title: Option<String>,
}

impl client::RequestValue for Document {}
impl client::Resource for Document {}
impl client::ResponseResult for Document {}


/// The style of the document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DocumentStyle {
    /// The background of the document. Documents cannot have a transparent background color.
    
    pub background: Option<Background>,
    /// The ID of the default footer. If not set, there's no default footer. This property is read-only.
    #[serde(rename="defaultFooterId")]
    
    pub default_footer_id: Option<String>,
    /// The ID of the default header. If not set, there's no default header. This property is read-only.
    #[serde(rename="defaultHeaderId")]
    
    pub default_header_id: Option<String>,
    /// The ID of the footer used only for even pages. The value of use_even_page_header_footer determines whether to use the default_footer_id or this value for the footer on even pages. If not set, there's no even page footer. This property is read-only.
    #[serde(rename="evenPageFooterId")]
    
    pub even_page_footer_id: Option<String>,
    /// The ID of the header used only for even pages. The value of use_even_page_header_footer determines whether to use the default_header_id or this value for the header on even pages. If not set, there's no even page header. This property is read-only.
    #[serde(rename="evenPageHeaderId")]
    
    pub even_page_header_id: Option<String>,
    /// The ID of the footer used only for the first page. If not set then a unique footer for the first page does not exist. The value of use_first_page_header_footer determines whether to use the default_footer_id or this value for the footer on the first page. If not set, there's no first page footer. This property is read-only.
    #[serde(rename="firstPageFooterId")]
    
    pub first_page_footer_id: Option<String>,
    /// The ID of the header used only for the first page. If not set then a unique header for the first page does not exist. The value of use_first_page_header_footer determines whether to use the default_header_id or this value for the header on the first page. If not set, there's no first page header. This property is read-only.
    #[serde(rename="firstPageHeaderId")]
    
    pub first_page_header_id: Option<String>,
    /// Optional. Indicates whether to flip the dimensions of the page_size, which allows changing the page orientation between portrait and landscape.
    #[serde(rename="flipPageOrientation")]
    
    pub flip_page_orientation: Option<bool>,
    /// The bottom page margin. Updating the bottom page margin on the document style clears the bottom page margin on all section styles.
    #[serde(rename="marginBottom")]
    
    pub margin_bottom: Option<Dimension>,
    /// The amount of space between the bottom of the page and the contents of the footer.
    #[serde(rename="marginFooter")]
    
    pub margin_footer: Option<Dimension>,
    /// The amount of space between the top of the page and the contents of the header.
    #[serde(rename="marginHeader")]
    
    pub margin_header: Option<Dimension>,
    /// The left page margin. Updating the left page margin on the document style clears the left page margin on all section styles. It may also cause columns to resize in all sections.
    #[serde(rename="marginLeft")]
    
    pub margin_left: Option<Dimension>,
    /// The right page margin. Updating the right page margin on the document style clears the right page margin on all section styles. It may also cause columns to resize in all sections.
    #[serde(rename="marginRight")]
    
    pub margin_right: Option<Dimension>,
    /// The top page margin. Updating the top page margin on the document style clears the top page margin on all section styles.
    #[serde(rename="marginTop")]
    
    pub margin_top: Option<Dimension>,
    /// The page number from which to start counting the number of pages.
    #[serde(rename="pageNumberStart")]
    
    pub page_number_start: Option<i32>,
    /// The size of a page in the document.
    #[serde(rename="pageSize")]
    
    pub page_size: Option<Size>,
    /// Indicates whether DocumentStyle margin_header, SectionStyle margin_header and DocumentStyle margin_footer, SectionStyle margin_footer are respected. When false, the default values in the Docs editor for header and footer margin are used. This property is read-only.
    #[serde(rename="useCustomHeaderFooterMargins")]
    
    pub use_custom_header_footer_margins: Option<bool>,
    /// Indicates whether to use the even page header / footer IDs for the even pages.
    #[serde(rename="useEvenPageHeaderFooter")]
    
    pub use_even_page_header_footer: Option<bool>,
    /// Indicates whether to use the first page header / footer IDs for the first page.
    #[serde(rename="useFirstPageHeaderFooter")]
    
    pub use_first_page_header_footer: Option<bool>,
}

impl client::Part for DocumentStyle {}


/// A mask that indicates which of the fields on the base DocumentStyle have been changed in this suggestion. For any field set to true, there's a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DocumentStyleSuggestionState {
    /// A mask that indicates which of the fields in background have been changed in this suggestion.
    #[serde(rename="backgroundSuggestionState")]
    
    pub background_suggestion_state: Option<BackgroundSuggestionState>,
    /// Indicates if there was a suggested change to default_footer_id.
    #[serde(rename="defaultFooterIdSuggested")]
    
    pub default_footer_id_suggested: Option<bool>,
    /// Indicates if there was a suggested change to default_header_id.
    #[serde(rename="defaultHeaderIdSuggested")]
    
    pub default_header_id_suggested: Option<bool>,
    /// Indicates if there was a suggested change to even_page_footer_id.
    #[serde(rename="evenPageFooterIdSuggested")]
    
    pub even_page_footer_id_suggested: Option<bool>,
    /// Indicates if there was a suggested change to even_page_header_id.
    #[serde(rename="evenPageHeaderIdSuggested")]
    
    pub even_page_header_id_suggested: Option<bool>,
    /// Indicates if there was a suggested change to first_page_footer_id.
    #[serde(rename="firstPageFooterIdSuggested")]
    
    pub first_page_footer_id_suggested: Option<bool>,
    /// Indicates if there was a suggested change to first_page_header_id.
    #[serde(rename="firstPageHeaderIdSuggested")]
    
    pub first_page_header_id_suggested: Option<bool>,
    /// Optional. Indicates if there was a suggested change to flip_page_orientation.
    #[serde(rename="flipPageOrientationSuggested")]
    
    pub flip_page_orientation_suggested: Option<bool>,
    /// Indicates if there was a suggested change to margin_bottom.
    #[serde(rename="marginBottomSuggested")]
    
    pub margin_bottom_suggested: Option<bool>,
    /// Indicates if there was a suggested change to margin_footer.
    #[serde(rename="marginFooterSuggested")]
    
    pub margin_footer_suggested: Option<bool>,
    /// Indicates if there was a suggested change to margin_header.
    #[serde(rename="marginHeaderSuggested")]
    
    pub margin_header_suggested: Option<bool>,
    /// Indicates if there was a suggested change to margin_left.
    #[serde(rename="marginLeftSuggested")]
    
    pub margin_left_suggested: Option<bool>,
    /// Indicates if there was a suggested change to margin_right.
    #[serde(rename="marginRightSuggested")]
    
    pub margin_right_suggested: Option<bool>,
    /// Indicates if there was a suggested change to margin_top.
    #[serde(rename="marginTopSuggested")]
    
    pub margin_top_suggested: Option<bool>,
    /// Indicates if there was a suggested change to page_number_start.
    #[serde(rename="pageNumberStartSuggested")]
    
    pub page_number_start_suggested: Option<bool>,
    /// A mask that indicates which of the fields in size have been changed in this suggestion.
    #[serde(rename="pageSizeSuggestionState")]
    
    pub page_size_suggestion_state: Option<SizeSuggestionState>,
    /// Indicates if there was a suggested change to use_custom_header_footer_margins.
    #[serde(rename="useCustomHeaderFooterMarginsSuggested")]
    
    pub use_custom_header_footer_margins_suggested: Option<bool>,
    /// Indicates if there was a suggested change to use_even_page_header_footer.
    #[serde(rename="useEvenPageHeaderFooterSuggested")]
    
    pub use_even_page_header_footer_suggested: Option<bool>,
    /// Indicates if there was a suggested change to use_first_page_header_footer.
    #[serde(rename="useFirstPageHeaderFooterSuggested")]
    
    pub use_first_page_header_footer_suggested: Option<bool>,
}

impl client::Part for DocumentStyleSuggestionState {}


/// The properties of an embedded drawing and used to differentiate the object type. An embedded drawing is one that's created and edited within a document. Note that extensive details are not supported.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EmbeddedDrawingProperties { _never_set: Option<bool> }

impl client::Part for EmbeddedDrawingProperties {}


/// A mask that indicates which of the fields on the base EmbeddedDrawingProperties have been changed in this suggestion. For any field set to true, there's a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EmbeddedDrawingPropertiesSuggestionState { _never_set: Option<bool> }

impl client::Part for EmbeddedDrawingPropertiesSuggestionState {}


/// An embedded object in the document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EmbeddedObject {
    /// The description of the embedded object. The `title` and `description` are both combined to display alt text.
    
    pub description: Option<String>,
    /// The properties of an embedded drawing.
    #[serde(rename="embeddedDrawingProperties")]
    
    pub embedded_drawing_properties: Option<EmbeddedDrawingProperties>,
    /// The border of the embedded object.
    #[serde(rename="embeddedObjectBorder")]
    
    pub embedded_object_border: Option<EmbeddedObjectBorder>,
    /// The properties of an image.
    #[serde(rename="imageProperties")]
    
    pub image_properties: Option<ImageProperties>,
    /// A reference to the external linked source content. For example, it contains a reference to the source Google Sheets chart when the embedded object is a linked chart. If unset, then the embedded object is not linked.
    #[serde(rename="linkedContentReference")]
    
    pub linked_content_reference: Option<LinkedContentReference>,
    /// The bottom margin of the embedded object.
    #[serde(rename="marginBottom")]
    
    pub margin_bottom: Option<Dimension>,
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
    /// The title of the embedded object. The `title` and `description` are both combined to display alt text.
    
    pub title: Option<String>,
}

impl client::Part for EmbeddedObject {}


/// A border around an EmbeddedObject.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EmbeddedObjectBorder {
    /// The color of the border.
    
    pub color: Option<OptionalColor>,
    /// The dash style of the border.
    #[serde(rename="dashStyle")]
    
    pub dash_style: Option<String>,
    /// The property state of the border property.
    #[serde(rename="propertyState")]
    
    pub property_state: Option<String>,
    /// The width of the border.
    
    pub width: Option<Dimension>,
}

impl client::Part for EmbeddedObjectBorder {}


/// A mask that indicates which of the fields on the base EmbeddedObjectBorder have been changed in this suggestion. For any field set to true, there's a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EmbeddedObjectBorderSuggestionState {
    /// Indicates if there was a suggested change to color.
    #[serde(rename="colorSuggested")]
    
    pub color_suggested: Option<bool>,
    /// Indicates if there was a suggested change to dash_style.
    #[serde(rename="dashStyleSuggested")]
    
    pub dash_style_suggested: Option<bool>,
    /// Indicates if there was a suggested change to property_state.
    #[serde(rename="propertyStateSuggested")]
    
    pub property_state_suggested: Option<bool>,
    /// Indicates if there was a suggested change to width.
    #[serde(rename="widthSuggested")]
    
    pub width_suggested: Option<bool>,
}

impl client::Part for EmbeddedObjectBorderSuggestionState {}


/// A mask that indicates which of the fields on the base EmbeddedObject have been changed in this suggestion. For any field set to true, there's a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EmbeddedObjectSuggestionState {
    /// Indicates if there was a suggested change to description.
    #[serde(rename="descriptionSuggested")]
    
    pub description_suggested: Option<bool>,
    /// A mask that indicates which of the fields in embedded_drawing_properties have been changed in this suggestion.
    #[serde(rename="embeddedDrawingPropertiesSuggestionState")]
    
    pub embedded_drawing_properties_suggestion_state: Option<EmbeddedDrawingPropertiesSuggestionState>,
    /// A mask that indicates which of the fields in embedded_object_border have been changed in this suggestion.
    #[serde(rename="embeddedObjectBorderSuggestionState")]
    
    pub embedded_object_border_suggestion_state: Option<EmbeddedObjectBorderSuggestionState>,
    /// A mask that indicates which of the fields in image_properties have been changed in this suggestion.
    #[serde(rename="imagePropertiesSuggestionState")]
    
    pub image_properties_suggestion_state: Option<ImagePropertiesSuggestionState>,
    /// A mask that indicates which of the fields in linked_content_reference have been changed in this suggestion.
    #[serde(rename="linkedContentReferenceSuggestionState")]
    
    pub linked_content_reference_suggestion_state: Option<LinkedContentReferenceSuggestionState>,
    /// Indicates if there was a suggested change to margin_bottom.
    #[serde(rename="marginBottomSuggested")]
    
    pub margin_bottom_suggested: Option<bool>,
    /// Indicates if there was a suggested change to margin_left.
    #[serde(rename="marginLeftSuggested")]
    
    pub margin_left_suggested: Option<bool>,
    /// Indicates if there was a suggested change to margin_right.
    #[serde(rename="marginRightSuggested")]
    
    pub margin_right_suggested: Option<bool>,
    /// Indicates if there was a suggested change to margin_top.
    #[serde(rename="marginTopSuggested")]
    
    pub margin_top_suggested: Option<bool>,
    /// A mask that indicates which of the fields in size have been changed in this suggestion.
    #[serde(rename="sizeSuggestionState")]
    
    pub size_suggestion_state: Option<SizeSuggestionState>,
    /// Indicates if there was a suggested change to title.
    #[serde(rename="titleSuggested")]
    
    pub title_suggested: Option<bool>,
}

impl client::Part for EmbeddedObjectSuggestionState {}


/// Location at the end of a body, header, footer or footnote. The location is immediately before the last newline in the document segment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EndOfSegmentLocation {
    /// The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body.
    #[serde(rename="segmentId")]
    
    pub segment_id: Option<String>,
}

impl client::Part for EndOfSegmentLocation {}


/// A ParagraphElement representing an equation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Equation {
    /// The suggested deletion IDs. If empty, then there are no suggested deletions of this content.
    #[serde(rename="suggestedDeletionIds")]
    
    pub suggested_deletion_ids: Option<Vec<String>>,
    /// The suggested insertion IDs. An Equation may have multiple insertion IDs if it's a nested suggested change. If empty, then this is not a suggested insertion.
    #[serde(rename="suggestedInsertionIds")]
    
    pub suggested_insertion_ids: Option<Vec<String>>,
}

impl client::Part for Equation {}


/// A document footer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Footer {
    /// The contents of the footer. The indexes for a footer's content begin at zero.
    
    pub content: Option<Vec<StructuralElement>>,
    /// The ID of the footer.
    #[serde(rename="footerId")]
    
    pub footer_id: Option<String>,
}

impl client::Part for Footer {}


/// A document footnote.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Footnote {
    /// The contents of the footnote. The indexes for a footnote's content begin at zero.
    
    pub content: Option<Vec<StructuralElement>>,
    /// The ID of the footnote.
    #[serde(rename="footnoteId")]
    
    pub footnote_id: Option<String>,
}

impl client::Part for Footnote {}


/// A ParagraphElement representing a footnote reference. A footnote reference is the inline content rendered with a number and is used to identify the footnote.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FootnoteReference {
    /// The ID of the footnote that contains the content of this footnote reference.
    #[serde(rename="footnoteId")]
    
    pub footnote_id: Option<String>,
    /// The rendered number of this footnote.
    #[serde(rename="footnoteNumber")]
    
    pub footnote_number: Option<String>,
    /// The suggested deletion IDs. If empty, then there are no suggested deletions of this content.
    #[serde(rename="suggestedDeletionIds")]
    
    pub suggested_deletion_ids: Option<Vec<String>>,
    /// The suggested insertion IDs. A FootnoteReference may have multiple insertion IDs if it's a nested suggested change. If empty, then this is not a suggested insertion.
    #[serde(rename="suggestedInsertionIds")]
    
    pub suggested_insertion_ids: Option<Vec<String>>,
    /// The suggested text style changes to this FootnoteReference, keyed by suggestion ID.
    #[serde(rename="suggestedTextStyleChanges")]
    
    pub suggested_text_style_changes: Option<HashMap<String, SuggestedTextStyle>>,
    /// The text style of this FootnoteReference.
    #[serde(rename="textStyle")]
    
    pub text_style: Option<TextStyle>,
}

impl client::Part for FootnoteReference {}


/// A document header.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Header {
    /// The contents of the header. The indexes for a header's content begin at zero.
    
    pub content: Option<Vec<StructuralElement>>,
    /// The ID of the header.
    #[serde(rename="headerId")]
    
    pub header_id: Option<String>,
}

impl client::Part for Header {}


/// A ParagraphElement representing a horizontal line.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HorizontalRule {
    /// The suggested deletion IDs. If empty, then there are no suggested deletions of this content.
    #[serde(rename="suggestedDeletionIds")]
    
    pub suggested_deletion_ids: Option<Vec<String>>,
    /// The suggested insertion IDs. A HorizontalRule may have multiple insertion IDs if it is a nested suggested change. If empty, then this is not a suggested insertion.
    #[serde(rename="suggestedInsertionIds")]
    
    pub suggested_insertion_ids: Option<Vec<String>>,
    /// The suggested text style changes to this HorizontalRule, keyed by suggestion ID.
    #[serde(rename="suggestedTextStyleChanges")]
    
    pub suggested_text_style_changes: Option<HashMap<String, SuggestedTextStyle>>,
    /// The text style of this HorizontalRule. Similar to text content, like text runs and footnote references, the text style of a horizontal rule can affect content layout as well as the styling of text inserted next to it.
    #[serde(rename="textStyle")]
    
    pub text_style: Option<TextStyle>,
}

impl client::Part for HorizontalRule {}


/// The properties of an image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImageProperties {
    /// The clockwise rotation angle of the image, in radians.
    
    pub angle: Option<f32>,
    /// The brightness effect of the image. The value should be in the interval [-1.0, 1.0], where 0 means no effect.
    
    pub brightness: Option<f32>,
    /// A URI to the image with a default lifetime of 30 minutes. This URI is tagged with the account of the requester. Anyone with the URI effectively accesses the image as the original requester. Access to the image may be lost if the document's sharing settings change.
    #[serde(rename="contentUri")]
    
    pub content_uri: Option<String>,
    /// The contrast effect of the image. The value should be in the interval [-1.0, 1.0], where 0 means no effect.
    
    pub contrast: Option<f32>,
    /// The crop properties of the image.
    #[serde(rename="cropProperties")]
    
    pub crop_properties: Option<CropProperties>,
    /// The source URI is the URI used to insert the image. The source URI can be empty.
    #[serde(rename="sourceUri")]
    
    pub source_uri: Option<String>,
    /// The transparency effect of the image. The value should be in the interval [0.0, 1.0], where 0 means no effect and 1 means transparent.
    
    pub transparency: Option<f32>,
}

impl client::Part for ImageProperties {}


/// A mask that indicates which of the fields on the base ImageProperties have been changed in this suggestion. For any field set to true, there's a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImagePropertiesSuggestionState {
    /// Indicates if there was a suggested change to angle.
    #[serde(rename="angleSuggested")]
    
    pub angle_suggested: Option<bool>,
    /// Indicates if there was a suggested change to brightness.
    #[serde(rename="brightnessSuggested")]
    
    pub brightness_suggested: Option<bool>,
    /// Indicates if there was a suggested change to content_uri.
    #[serde(rename="contentUriSuggested")]
    
    pub content_uri_suggested: Option<bool>,
    /// Indicates if there was a suggested change to contrast.
    #[serde(rename="contrastSuggested")]
    
    pub contrast_suggested: Option<bool>,
    /// A mask that indicates which of the fields in crop_properties have been changed in this suggestion.
    #[serde(rename="cropPropertiesSuggestionState")]
    
    pub crop_properties_suggestion_state: Option<CropPropertiesSuggestionState>,
    /// Indicates if there was a suggested change to source_uri.
    #[serde(rename="sourceUriSuggested")]
    
    pub source_uri_suggested: Option<bool>,
    /// Indicates if there was a suggested change to transparency.
    #[serde(rename="transparencySuggested")]
    
    pub transparency_suggested: Option<bool>,
}

impl client::Part for ImagePropertiesSuggestionState {}


/// An object that appears inline with text. An InlineObject contains an EmbeddedObject such as an image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InlineObject {
    /// The properties of this inline object.
    #[serde(rename="inlineObjectProperties")]
    
    pub inline_object_properties: Option<InlineObjectProperties>,
    /// The ID of this inline object. Can be used to update an object’s properties.
    #[serde(rename="objectId")]
    
    pub object_id: Option<String>,
    /// The suggested deletion IDs. If empty, then there are no suggested deletions of this content.
    #[serde(rename="suggestedDeletionIds")]
    
    pub suggested_deletion_ids: Option<Vec<String>>,
    /// The suggested changes to the inline object properties, keyed by suggestion ID.
    #[serde(rename="suggestedInlineObjectPropertiesChanges")]
    
    pub suggested_inline_object_properties_changes: Option<HashMap<String, SuggestedInlineObjectProperties>>,
    /// The suggested insertion ID. If empty, then this is not a suggested insertion.
    #[serde(rename="suggestedInsertionId")]
    
    pub suggested_insertion_id: Option<String>,
}

impl client::Part for InlineObject {}


/// A ParagraphElement that contains an InlineObject.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InlineObjectElement {
    /// The ID of the InlineObject this element contains.
    #[serde(rename="inlineObjectId")]
    
    pub inline_object_id: Option<String>,
    /// The suggested deletion IDs. If empty, then there are no suggested deletions of this content.
    #[serde(rename="suggestedDeletionIds")]
    
    pub suggested_deletion_ids: Option<Vec<String>>,
    /// The suggested insertion IDs. An InlineObjectElement may have multiple insertion IDs if it's a nested suggested change. If empty, then this is not a suggested insertion.
    #[serde(rename="suggestedInsertionIds")]
    
    pub suggested_insertion_ids: Option<Vec<String>>,
    /// The suggested text style changes to this InlineObject, keyed by suggestion ID.
    #[serde(rename="suggestedTextStyleChanges")]
    
    pub suggested_text_style_changes: Option<HashMap<String, SuggestedTextStyle>>,
    /// The text style of this InlineObjectElement. Similar to text content, like text runs and footnote references, the text style of an inline object element can affect content layout as well as the styling of text inserted next to it.
    #[serde(rename="textStyle")]
    
    pub text_style: Option<TextStyle>,
}

impl client::Part for InlineObjectElement {}


/// Properties of an InlineObject.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InlineObjectProperties {
    /// The embedded object of this inline object.
    #[serde(rename="embeddedObject")]
    
    pub embedded_object: Option<EmbeddedObject>,
}

impl client::Part for InlineObjectProperties {}


/// A mask that indicates which of the fields on the base InlineObjectProperties have been changed in this suggestion. For any field set to true, there's a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InlineObjectPropertiesSuggestionState {
    /// A mask that indicates which of the fields in embedded_object have been changed in this suggestion.
    #[serde(rename="embeddedObjectSuggestionState")]
    
    pub embedded_object_suggestion_state: Option<EmbeddedObjectSuggestionState>,
}

impl client::Part for InlineObjectPropertiesSuggestionState {}


/// Inserts an InlineObject containing an image at the given location.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InsertInlineImageRequest {
    /// Inserts the text at the end of a header, footer or the document body. Inline images cannot be inserted inside a footnote.
    #[serde(rename="endOfSegmentLocation")]
    
    pub end_of_segment_location: Option<EndOfSegmentLocation>,
    /// Inserts the image at a specific index in the document. The image must be inserted inside the bounds of an existing Paragraph. For instance, it cannot be inserted at a table's start index (i.e. between the table and its preceding paragraph). Inline images cannot be inserted inside a footnote or equation.
    
    pub location: Option<Location>,
    /// The size that the image should appear as in the document. This property is optional and the final size of the image in the document is determined by the following rules: * If neither width nor height is specified, then a default size of the image is calculated based on its resolution. * If one dimension is specified then the other dimension is calculated to preserve the aspect ratio of the image. * If both width and height are specified, the image is scaled to fit within the provided dimensions while maintaining its aspect ratio.
    #[serde(rename="objectSize")]
    
    pub object_size: Option<Size>,
    /// The image URI. The image is fetched once at insertion time and a copy is stored for display inside the document. Images must be less than 50MB in size, cannot exceed 25 megapixels, and must be in one of PNG, JPEG, or GIF format. The provided URI must be publicly accessible and at most 2 kB in length. The URI itself is saved with the image, and exposed via the ImageProperties.content_uri field.
    
    pub uri: Option<String>,
}

impl client::Part for InsertInlineImageRequest {}


/// The result of inserting an inline image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InsertInlineImageResponse {
    /// The ID of the created InlineObject.
    #[serde(rename="objectId")]
    
    pub object_id: Option<String>,
}

impl client::Part for InsertInlineImageResponse {}


/// The result of inserting an embedded Google Sheets chart.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InsertInlineSheetsChartResponse {
    /// The object ID of the inserted chart.
    #[serde(rename="objectId")]
    
    pub object_id: Option<String>,
}

impl client::Part for InsertInlineSheetsChartResponse {}


/// Inserts a page break followed by a newline at the specified location.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InsertPageBreakRequest {
    /// Inserts the page break at the end of the document body. Page breaks cannot be inserted inside a footnote, header or footer. Since page breaks can only be inserted inside the body, the segment ID field must be empty.
    #[serde(rename="endOfSegmentLocation")]
    
    pub end_of_segment_location: Option<EndOfSegmentLocation>,
    /// Inserts the page break at a specific index in the document. The page break must be inserted inside the bounds of an existing Paragraph. For instance, it cannot be inserted at a table's start index (i.e. between the table and its preceding paragraph). Page breaks cannot be inserted inside a table, equation, footnote, header or footer. Since page breaks can only be inserted inside the body, the segment ID field must be empty.
    
    pub location: Option<Location>,
}

impl client::Part for InsertPageBreakRequest {}


/// Inserts a section break at the given location. A newline character will be inserted before the section break.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InsertSectionBreakRequest {
    /// Inserts a newline and a section break at the end of the document body. Section breaks cannot be inserted inside a footnote, header or footer. Because section breaks can only be inserted inside the body, the segment ID field must be empty.
    #[serde(rename="endOfSegmentLocation")]
    
    pub end_of_segment_location: Option<EndOfSegmentLocation>,
    /// Inserts a newline and a section break at a specific index in the document. The section break must be inserted inside the bounds of an existing Paragraph. For instance, it cannot be inserted at a table's start index (i.e. between the table and its preceding paragraph). Section breaks cannot be inserted inside a table, equation, footnote, header, or footer. Since section breaks can only be inserted inside the body, the segment ID field must be empty.
    
    pub location: Option<Location>,
    /// The type of section to insert.
    #[serde(rename="sectionType")]
    
    pub section_type: Option<String>,
}

impl client::Part for InsertSectionBreakRequest {}


/// Inserts an empty column into a table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InsertTableColumnRequest {
    /// Whether to insert new column to the right of the reference cell location. - `True`: insert to the right. - `False`: insert to the left.
    #[serde(rename="insertRight")]
    
    pub insert_right: Option<bool>,
    /// The reference table cell location from which columns will be inserted. A new column will be inserted to the left (or right) of the column where the reference cell is. If the reference cell is a merged cell, a new column will be inserted to the left (or right) of the merged cell.
    #[serde(rename="tableCellLocation")]
    
    pub table_cell_location: Option<TableCellLocation>,
}

impl client::Part for InsertTableColumnRequest {}


/// Inserts a table at the specified location. A newline character will be inserted before the inserted table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InsertTableRequest {
    /// The number of columns in the table.
    
    pub columns: Option<i32>,
    /// Inserts the table at the end of the given header, footer or document body. A newline character will be inserted before the inserted table. Tables cannot be inserted inside a footnote.
    #[serde(rename="endOfSegmentLocation")]
    
    pub end_of_segment_location: Option<EndOfSegmentLocation>,
    /// Inserts the table at a specific model index. A newline character will be inserted before the inserted table, therefore the table start index will be at the specified location index + 1. The table must be inserted inside the bounds of an existing Paragraph. For instance, it cannot be inserted at a table's start index (i.e. between an existing table and its preceding paragraph). Tables cannot be inserted inside a footnote or equation.
    
    pub location: Option<Location>,
    /// The number of rows in the table.
    
    pub rows: Option<i32>,
}

impl client::Part for InsertTableRequest {}


/// Inserts an empty row into a table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InsertTableRowRequest {
    /// Whether to insert new row below the reference cell location. - `True`: insert below the cell. - `False`: insert above the cell.
    #[serde(rename="insertBelow")]
    
    pub insert_below: Option<bool>,
    /// The reference table cell location from which rows will be inserted. A new row will be inserted above (or below) the row where the reference cell is. If the reference cell is a merged cell, a new row will be inserted above (or below) the merged cell.
    #[serde(rename="tableCellLocation")]
    
    pub table_cell_location: Option<TableCellLocation>,
}

impl client::Part for InsertTableRowRequest {}


/// Inserts text at the specified location.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InsertTextRequest {
    /// Inserts the text at the end of a header, footer, footnote or the document body.
    #[serde(rename="endOfSegmentLocation")]
    
    pub end_of_segment_location: Option<EndOfSegmentLocation>,
    /// Inserts the text at a specific index in the document. Text must be inserted inside the bounds of an existing Paragraph. For instance, text cannot be inserted at a table's start index (i.e. between the table and its preceding paragraph). The text must be inserted in the preceding paragraph.
    
    pub location: Option<Location>,
    /// The text to be inserted. Inserting a newline character will implicitly create a new Paragraph at that index. The paragraph style of the new paragraph will be copied from the paragraph at the current insertion index, including lists and bullets. Text styles for inserted text will be determined automatically, generally preserving the styling of neighboring text. In most cases, the text style for the inserted text will match the text immediately before the insertion index. Some control characters (U+0000-U+0008, U+000C-U+001F) and characters from the Unicode Basic Multilingual Plane Private Use Area (U+E000-U+F8FF) will be stripped out of the inserted text.
    
    pub text: Option<String>,
}

impl client::Part for InsertTextRequest {}


/// A reference to another portion of a document or an external URL resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Link {
    /// The ID of a bookmark in this document.
    #[serde(rename="bookmarkId")]
    
    pub bookmark_id: Option<String>,
    /// The ID of a heading in this document.
    #[serde(rename="headingId")]
    
    pub heading_id: Option<String>,
    /// An external URL.
    
    pub url: Option<String>,
}

impl client::Part for Link {}


/// A reference to the external linked source content.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LinkedContentReference {
    /// A reference to the linked chart.
    #[serde(rename="sheetsChartReference")]
    
    pub sheets_chart_reference: Option<SheetsChartReference>,
}

impl client::Part for LinkedContentReference {}


/// A mask that indicates which of the fields on the base LinkedContentReference have been changed in this suggestion. For any field set to true, there's a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LinkedContentReferenceSuggestionState {
    /// A mask that indicates which of the fields in sheets_chart_reference have been changed in this suggestion.
    #[serde(rename="sheetsChartReferenceSuggestionState")]
    
    pub sheets_chart_reference_suggestion_state: Option<SheetsChartReferenceSuggestionState>,
}

impl client::Part for LinkedContentReferenceSuggestionState {}


/// A List represents the list attributes for a group of paragraphs that all belong to the same list. A paragraph that's part of a list has a reference to the list's ID in its bullet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct List {
    /// The properties of the list.
    #[serde(rename="listProperties")]
    
    pub list_properties: Option<ListProperties>,
    /// The suggested deletion IDs. If empty, then there are no suggested deletions of this list.
    #[serde(rename="suggestedDeletionIds")]
    
    pub suggested_deletion_ids: Option<Vec<String>>,
    /// The suggested insertion ID. If empty, then this is not a suggested insertion.
    #[serde(rename="suggestedInsertionId")]
    
    pub suggested_insertion_id: Option<String>,
    /// The suggested changes to the list properties, keyed by suggestion ID.
    #[serde(rename="suggestedListPropertiesChanges")]
    
    pub suggested_list_properties_changes: Option<HashMap<String, SuggestedListProperties>>,
}

impl client::Part for List {}


/// The properties of a list that describe the look and feel of bullets belonging to paragraphs associated with a list.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListProperties {
    /// Describes the properties of the bullets at the associated level. A list has at most 9 levels of nesting with nesting level 0 corresponding to the top-most level and nesting level 8 corresponding to the most nested level. The nesting levels are returned in ascending order with the least nested returned first.
    #[serde(rename="nestingLevels")]
    
    pub nesting_levels: Option<Vec<NestingLevel>>,
}

impl client::Part for ListProperties {}


/// A mask that indicates which of the fields on the base ListProperties have been changed in this suggestion. For any field set to true, there's a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListPropertiesSuggestionState {
    /// A mask that indicates which of the fields on the corresponding NestingLevel in nesting_levels have been changed in this suggestion. The nesting level suggestion states are returned in ascending order of the nesting level with the least nested returned first.
    #[serde(rename="nestingLevelsSuggestionStates")]
    
    pub nesting_levels_suggestion_states: Option<Vec<NestingLevelSuggestionState>>,
}

impl client::Part for ListPropertiesSuggestionState {}


/// A particular location in the document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    /// The zero-based index, in UTF-16 code units. The index is relative to the beginning of the segment specified by segment_id.
    
    pub index: Option<i32>,
    /// The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body.
    #[serde(rename="segmentId")]
    
    pub segment_id: Option<String>,
}

impl client::Part for Location {}


/// Merges cells in a Table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MergeTableCellsRequest {
    /// The table range specifying which cells of the table to merge. Any text in the cells being merged will be concatenated and stored in the "head" cell of the range. This is the upper-left cell of the range when the content direction is left to right, and the upper-right cell of the range otherwise. If the range is non-rectangular (which can occur in some cases where the range covers cells that are already merged or where the table is non-rectangular), a 400 bad request error is returned.
    #[serde(rename="tableRange")]
    
    pub table_range: Option<TableRange>,
}

impl client::Part for MergeTableCellsRequest {}


/// A collection of Ranges with the same named range ID. Named ranges allow developers to associate parts of a document with an arbitrary user-defined label so their contents can be programmatically read or edited later. A document can contain multiple named ranges with the same name, but every named range has a unique ID. A named range is created with a single Range, and content inserted inside a named range generally expands that range. However, certain document changes can cause the range to be split into multiple ranges. Named ranges are not private. All applications and collaborators that have access to the document can see its named ranges.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NamedRange {
    /// The name of the named range.
    
    pub name: Option<String>,
    /// The ID of the named range.
    #[serde(rename="namedRangeId")]
    
    pub named_range_id: Option<String>,
    /// The ranges that belong to this named range.
    
    pub ranges: Option<Vec<Range>>,
}

impl client::Part for NamedRange {}


/// A collection of all the NamedRanges in the document that share a given name.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NamedRanges {
    /// The name that all the named ranges share.
    
    pub name: Option<String>,
    /// The NamedRanges that share the same name.
    #[serde(rename="namedRanges")]
    
    pub named_ranges: Option<Vec<NamedRange>>,
}

impl client::Part for NamedRanges {}


/// A named style. Paragraphs in the document can inherit their TextStyle and ParagraphStyle from this named style when they have the same named style type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NamedStyle {
    /// The type of this named style.
    #[serde(rename="namedStyleType")]
    
    pub named_style_type: Option<String>,
    /// The paragraph style of this named style.
    #[serde(rename="paragraphStyle")]
    
    pub paragraph_style: Option<ParagraphStyle>,
    /// The text style of this named style.
    #[serde(rename="textStyle")]
    
    pub text_style: Option<TextStyle>,
}

impl client::Part for NamedStyle {}


/// A suggestion state of a NamedStyle message.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NamedStyleSuggestionState {
    /// The named style type that this suggestion state corresponds to. This field is provided as a convenience for matching the NamedStyleSuggestionState with its corresponding NamedStyle.
    #[serde(rename="namedStyleType")]
    
    pub named_style_type: Option<String>,
    /// A mask that indicates which of the fields in paragraph style have been changed in this suggestion.
    #[serde(rename="paragraphStyleSuggestionState")]
    
    pub paragraph_style_suggestion_state: Option<ParagraphStyleSuggestionState>,
    /// A mask that indicates which of the fields in text style have been changed in this suggestion.
    #[serde(rename="textStyleSuggestionState")]
    
    pub text_style_suggestion_state: Option<TextStyleSuggestionState>,
}

impl client::Part for NamedStyleSuggestionState {}


/// The named styles. Paragraphs in the document can inherit their TextStyle and ParagraphStyle from these named styles.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NamedStyles {
    /// The named styles. There's an entry for each of the possible named style types.
    
    pub styles: Option<Vec<NamedStyle>>,
}

impl client::Part for NamedStyles {}


/// The suggestion state of a NamedStyles message.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NamedStylesSuggestionState {
    /// A mask that indicates which of the fields on the corresponding NamedStyle in styles have been changed in this suggestion. The order of these named style suggestion states matches the order of the corresponding named style within the named styles suggestion.
    #[serde(rename="stylesSuggestionStates")]
    
    pub styles_suggestion_states: Option<Vec<NamedStyleSuggestionState>>,
}

impl client::Part for NamedStylesSuggestionState {}


/// Contains properties describing the look and feel of a list bullet at a given level of nesting.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NestingLevel {
    /// The alignment of the bullet within the space allotted for rendering the bullet.
    #[serde(rename="bulletAlignment")]
    
    pub bullet_alignment: Option<String>,
    /// The format string used by bullets at this level of nesting. The glyph format contains one or more placeholders, and these placeholders are replaced with the appropriate values depending on the glyph_type or glyph_symbol. The placeholders follow the pattern `%[nesting_level]`. Furthermore, placeholders can have prefixes and suffixes. Thus, the glyph format follows the pattern `%[nesting_level]`. Note that the prefix and suffix are optional and can be arbitrary strings. For example, the glyph format `%0.` indicates that the rendered glyph will replace the placeholder with the corresponding glyph for nesting level 0 followed by a period as the suffix. So a list with a glyph type of UPPER_ALPHA and glyph format `%0.` at nesting level 0 will result in a list with rendered glyphs `A.` `B.` `C.` The glyph format can contain placeholders for the current nesting level as well as placeholders for parent nesting levels. For example, a list can have a glyph format of `%0.` at nesting level 0 and a glyph format of `%0.%1.` at nesting level 1. Assuming both nesting levels have DECIMAL glyph types, this would result in a list with rendered glyphs `1.` `2.` ` 2.1.` ` 2.2.` `3.` For nesting levels that are ordered, the string that replaces a placeholder in the glyph format for a particular paragraph depends on the paragraph's order within the list.
    #[serde(rename="glyphFormat")]
    
    pub glyph_format: Option<String>,
    /// A custom glyph symbol used by bullets when paragraphs at this level of nesting are unordered. The glyph symbol replaces placeholders within the glyph_format. For example, if the glyph_symbol is the solid circle corresponding to Unicode U+25cf code point and the glyph_format is `%0`, the rendered glyph would be the solid circle.
    #[serde(rename="glyphSymbol")]
    
    pub glyph_symbol: Option<String>,
    /// The type of glyph used by bullets when paragraphs at this level of nesting are ordered. The glyph type determines the type of glyph used to replace placeholders within the glyph_format when paragraphs at this level of nesting are ordered. For example, if the nesting level is 0, the glyph_format is `%0.` and the glyph type is DECIMAL, then the rendered glyph would replace the placeholder `%0` in the glyph format with a number corresponding to list item's order within the list.
    #[serde(rename="glyphType")]
    
    pub glyph_type: Option<String>,
    /// The amount of indentation for the first line of paragraphs at this level of nesting.
    #[serde(rename="indentFirstLine")]
    
    pub indent_first_line: Option<Dimension>,
    /// The amount of indentation for paragraphs at this level of nesting. Applied to the side that corresponds to the start of the text, based on the paragraph's content direction.
    #[serde(rename="indentStart")]
    
    pub indent_start: Option<Dimension>,
    /// The number of the first list item at this nesting level. A value of 0 is treated as a value of 1 for lettered lists and Roman numeral lists. For values of both 0 and 1, lettered and Roman numeral lists will begin at `a` and `i` respectively. This value is ignored for nesting levels with unordered glyphs.
    #[serde(rename="startNumber")]
    
    pub start_number: Option<i32>,
    /// The text style of bullets at this level of nesting.
    #[serde(rename="textStyle")]
    
    pub text_style: Option<TextStyle>,
}

impl client::Part for NestingLevel {}


/// A mask that indicates which of the fields on the base NestingLevel have been changed in this suggestion. For any field set to true, there's a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NestingLevelSuggestionState {
    /// Indicates if there was a suggested change to bullet_alignment.
    #[serde(rename="bulletAlignmentSuggested")]
    
    pub bullet_alignment_suggested: Option<bool>,
    /// Indicates if there was a suggested change to glyph_format.
    #[serde(rename="glyphFormatSuggested")]
    
    pub glyph_format_suggested: Option<bool>,
    /// Indicates if there was a suggested change to glyph_symbol.
    #[serde(rename="glyphSymbolSuggested")]
    
    pub glyph_symbol_suggested: Option<bool>,
    /// Indicates if there was a suggested change to glyph_type.
    #[serde(rename="glyphTypeSuggested")]
    
    pub glyph_type_suggested: Option<bool>,
    /// Indicates if there was a suggested change to indent_first_line.
    #[serde(rename="indentFirstLineSuggested")]
    
    pub indent_first_line_suggested: Option<bool>,
    /// Indicates if there was a suggested change to indent_start.
    #[serde(rename="indentStartSuggested")]
    
    pub indent_start_suggested: Option<bool>,
    /// Indicates if there was a suggested change to start_number.
    #[serde(rename="startNumberSuggested")]
    
    pub start_number_suggested: Option<bool>,
    /// A mask that indicates which of the fields in text style have been changed in this suggestion.
    #[serde(rename="textStyleSuggestionState")]
    
    pub text_style_suggestion_state: Option<TextStyleSuggestionState>,
}

impl client::Part for NestingLevelSuggestionState {}


/// A collection of object IDs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ObjectReferences {
    /// The object IDs.
    #[serde(rename="objectIds")]
    
    pub object_ids: Option<Vec<String>>,
}

impl client::Part for ObjectReferences {}


/// A color that can either be fully opaque or fully transparent.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OptionalColor {
    /// If set, this will be used as an opaque color. If unset, this represents a transparent color.
    
    pub color: Option<Color>,
}

impl client::Part for OptionalColor {}


/// A ParagraphElement representing a page break. A page break makes the subsequent text start at the top of the next page.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PageBreak {
    /// The suggested deletion IDs. If empty, then there are no suggested deletions of this content.
    #[serde(rename="suggestedDeletionIds")]
    
    pub suggested_deletion_ids: Option<Vec<String>>,
    /// The suggested insertion IDs. A PageBreak may have multiple insertion IDs if it's a nested suggested change. If empty, then this is not a suggested insertion.
    #[serde(rename="suggestedInsertionIds")]
    
    pub suggested_insertion_ids: Option<Vec<String>>,
    /// The suggested text style changes to this PageBreak, keyed by suggestion ID.
    #[serde(rename="suggestedTextStyleChanges")]
    
    pub suggested_text_style_changes: Option<HashMap<String, SuggestedTextStyle>>,
    /// The text style of this PageBreak. Similar to text content, like text runs and footnote references, the text style of a page break can affect content layout as well as the styling of text inserted next to it.
    #[serde(rename="textStyle")]
    
    pub text_style: Option<TextStyle>,
}

impl client::Part for PageBreak {}


/// A StructuralElement representing a paragraph. A paragraph is a range of content that's terminated with a newline character.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Paragraph {
    /// The bullet for this paragraph. If not present, the paragraph does not belong to a list.
    
    pub bullet: Option<Bullet>,
    /// The content of the paragraph, broken down into its component parts.
    
    pub elements: Option<Vec<ParagraphElement>>,
    /// The style of this paragraph.
    #[serde(rename="paragraphStyle")]
    
    pub paragraph_style: Option<ParagraphStyle>,
    /// The IDs of the positioned objects tethered to this paragraph.
    #[serde(rename="positionedObjectIds")]
    
    pub positioned_object_ids: Option<Vec<String>>,
    /// The suggested changes to this paragraph's bullet.
    #[serde(rename="suggestedBulletChanges")]
    
    pub suggested_bullet_changes: Option<HashMap<String, SuggestedBullet>>,
    /// The suggested paragraph style changes to this paragraph, keyed by suggestion ID.
    #[serde(rename="suggestedParagraphStyleChanges")]
    
    pub suggested_paragraph_style_changes: Option<HashMap<String, SuggestedParagraphStyle>>,
    /// The IDs of the positioned objects suggested to be attached to this paragraph, keyed by suggestion ID.
    #[serde(rename="suggestedPositionedObjectIds")]
    
    pub suggested_positioned_object_ids: Option<HashMap<String, ObjectReferences>>,
}

impl client::Part for Paragraph {}


/// A border around a paragraph.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ParagraphBorder {
    /// The color of the border.
    
    pub color: Option<OptionalColor>,
    /// The dash style of the border.
    #[serde(rename="dashStyle")]
    
    pub dash_style: Option<String>,
    /// The padding of the border.
    
    pub padding: Option<Dimension>,
    /// The width of the border.
    
    pub width: Option<Dimension>,
}

impl client::Part for ParagraphBorder {}


/// A ParagraphElement describes content within a Paragraph.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ParagraphElement {
    /// An auto text paragraph element.
    #[serde(rename="autoText")]
    
    pub auto_text: Option<AutoText>,
    /// A column break paragraph element.
    #[serde(rename="columnBreak")]
    
    pub column_break: Option<ColumnBreak>,
    /// The zero-base end index of this paragraph element, exclusive, in UTF-16 code units.
    #[serde(rename="endIndex")]
    
    pub end_index: Option<i32>,
    /// An equation paragraph element.
    
    pub equation: Option<Equation>,
    /// A footnote reference paragraph element.
    #[serde(rename="footnoteReference")]
    
    pub footnote_reference: Option<FootnoteReference>,
    /// A horizontal rule paragraph element.
    #[serde(rename="horizontalRule")]
    
    pub horizontal_rule: Option<HorizontalRule>,
    /// An inline object paragraph element.
    #[serde(rename="inlineObjectElement")]
    
    pub inline_object_element: Option<InlineObjectElement>,
    /// A page break paragraph element.
    #[serde(rename="pageBreak")]
    
    pub page_break: Option<PageBreak>,
    /// A paragraph element that links to a person or email address.
    
    pub person: Option<Person>,
    /// A paragraph element that links to a Google resource (such as a file in Google Drive, a YouTube video, or a Calendar event.)
    #[serde(rename="richLink")]
    
    pub rich_link: Option<RichLink>,
    /// The zero-based start index of this paragraph element, in UTF-16 code units.
    #[serde(rename="startIndex")]
    
    pub start_index: Option<i32>,
    /// A text run paragraph element.
    #[serde(rename="textRun")]
    
    pub text_run: Option<TextRun>,
}

impl client::Part for ParagraphElement {}


/// Styles that apply to a whole paragraph. Inherited paragraph styles are represented as unset fields in this message. A paragraph style's parent depends on where the paragraph style is defined: * The ParagraphStyle on a Paragraph inherits from the paragraph's corresponding named style type. * The ParagraphStyle on a named style inherits from the normal text named style. * The ParagraphStyle of the normal text named style inherits from the default paragraph style in the Docs editor. * The ParagraphStyle on a Paragraph element that's contained in a table may inherit its paragraph style from the table style. If the paragraph style does not inherit from a parent, unsetting fields will revert the style to a value matching the defaults in the Docs editor.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ParagraphStyle {
    /// The text alignment for this paragraph.
    
    pub alignment: Option<String>,
    /// Whether to avoid widows and orphans for the paragraph. If unset, the value is inherited from the parent.
    #[serde(rename="avoidWidowAndOrphan")]
    
    pub avoid_widow_and_orphan: Option<bool>,
    /// The border between this paragraph and the next and previous paragraphs. If unset, the value is inherited from the parent. The between border is rendered when the adjacent paragraph has the same border and indent properties. Paragraph borders cannot be partially updated. When changing a paragraph border, the new border must be specified in its entirety.
    #[serde(rename="borderBetween")]
    
    pub border_between: Option<ParagraphBorder>,
    /// The border at the bottom of this paragraph. If unset, the value is inherited from the parent. The bottom border is rendered when the paragraph below has different border and indent properties. Paragraph borders cannot be partially updated. When changing a paragraph border, the new border must be specified in its entirety.
    #[serde(rename="borderBottom")]
    
    pub border_bottom: Option<ParagraphBorder>,
    /// The border to the left of this paragraph. If unset, the value is inherited from the parent. Paragraph borders cannot be partially updated. When changing a paragraph border, the new border must be specified in its entirety.
    #[serde(rename="borderLeft")]
    
    pub border_left: Option<ParagraphBorder>,
    /// The border to the right of this paragraph. If unset, the value is inherited from the parent. Paragraph borders cannot be partially updated. When changing a paragraph border, the new border must be specified in its entirety.
    #[serde(rename="borderRight")]
    
    pub border_right: Option<ParagraphBorder>,
    /// The border at the top of this paragraph. If unset, the value is inherited from the parent. The top border is rendered when the paragraph above has different border and indent properties. Paragraph borders cannot be partially updated. When changing a paragraph border, the new border must be specified in its entirety.
    #[serde(rename="borderTop")]
    
    pub border_top: Option<ParagraphBorder>,
    /// The text direction of this paragraph. If unset, the value defaults to LEFT_TO_RIGHT since paragraph direction is not inherited.
    
    pub direction: Option<String>,
    /// The heading ID of the paragraph. If empty, then this paragraph is not a heading. This property is read-only.
    #[serde(rename="headingId")]
    
    pub heading_id: Option<String>,
    /// The amount of indentation for the paragraph on the side that corresponds to the end of the text, based on the current paragraph direction. If unset, the value is inherited from the parent.
    #[serde(rename="indentEnd")]
    
    pub indent_end: Option<Dimension>,
    /// The amount of indentation for the first line of the paragraph. If unset, the value is inherited from the parent.
    #[serde(rename="indentFirstLine")]
    
    pub indent_first_line: Option<Dimension>,
    /// The amount of indentation for the paragraph on the side that corresponds to the start of the text, based on the current paragraph direction. If unset, the value is inherited from the parent.
    #[serde(rename="indentStart")]
    
    pub indent_start: Option<Dimension>,
    /// Whether all lines of the paragraph should be laid out on the same page or column if possible. If unset, the value is inherited from the parent.
    #[serde(rename="keepLinesTogether")]
    
    pub keep_lines_together: Option<bool>,
    /// Whether at least a part of this paragraph should be laid out on the same page or column as the next paragraph if possible. If unset, the value is inherited from the parent.
    #[serde(rename="keepWithNext")]
    
    pub keep_with_next: Option<bool>,
    /// The amount of space between lines, as a percentage of normal, where normal is represented as 100.0. If unset, the value is inherited from the parent.
    #[serde(rename="lineSpacing")]
    
    pub line_spacing: Option<f32>,
    /// The named style type of the paragraph. Since updating the named style type affects other properties within ParagraphStyle, the named style type is applied before the other properties are updated.
    #[serde(rename="namedStyleType")]
    
    pub named_style_type: Option<String>,
    /// Whether the current paragraph should always start at the beginning of a page. If unset, the value is inherited from the parent. Attempting to update page_break_before for paragraphs in unsupported regions, including Table, Header, Footer and Footnote, can result in an invalid document state that returns a 400 bad request error.
    #[serde(rename="pageBreakBefore")]
    
    pub page_break_before: Option<bool>,
    /// The shading of the paragraph. If unset, the value is inherited from the parent.
    
    pub shading: Option<Shading>,
    /// The amount of extra space above the paragraph. If unset, the value is inherited from the parent.
    #[serde(rename="spaceAbove")]
    
    pub space_above: Option<Dimension>,
    /// The amount of extra space below the paragraph. If unset, the value is inherited from the parent.
    #[serde(rename="spaceBelow")]
    
    pub space_below: Option<Dimension>,
    /// The spacing mode for the paragraph.
    #[serde(rename="spacingMode")]
    
    pub spacing_mode: Option<String>,
    /// A list of the tab stops for this paragraph. The list of tab stops is not inherited. This property is read-only.
    #[serde(rename="tabStops")]
    
    pub tab_stops: Option<Vec<TabStop>>,
}

impl client::Part for ParagraphStyle {}


/// A mask that indicates which of the fields on the base ParagraphStyle have been changed in this suggestion. For any field set to true, there's a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ParagraphStyleSuggestionState {
    /// Indicates if there was a suggested change to alignment.
    #[serde(rename="alignmentSuggested")]
    
    pub alignment_suggested: Option<bool>,
    /// Indicates if there was a suggested change to avoid_widow_and_orphan.
    #[serde(rename="avoidWidowAndOrphanSuggested")]
    
    pub avoid_widow_and_orphan_suggested: Option<bool>,
    /// Indicates if there was a suggested change to border_between.
    #[serde(rename="borderBetweenSuggested")]
    
    pub border_between_suggested: Option<bool>,
    /// Indicates if there was a suggested change to border_bottom.
    #[serde(rename="borderBottomSuggested")]
    
    pub border_bottom_suggested: Option<bool>,
    /// Indicates if there was a suggested change to border_left.
    #[serde(rename="borderLeftSuggested")]
    
    pub border_left_suggested: Option<bool>,
    /// Indicates if there was a suggested change to border_right.
    #[serde(rename="borderRightSuggested")]
    
    pub border_right_suggested: Option<bool>,
    /// Indicates if there was a suggested change to border_top.
    #[serde(rename="borderTopSuggested")]
    
    pub border_top_suggested: Option<bool>,
    /// Indicates if there was a suggested change to direction.
    #[serde(rename="directionSuggested")]
    
    pub direction_suggested: Option<bool>,
    /// Indicates if there was a suggested change to heading_id.
    #[serde(rename="headingIdSuggested")]
    
    pub heading_id_suggested: Option<bool>,
    /// Indicates if there was a suggested change to indent_end.
    #[serde(rename="indentEndSuggested")]
    
    pub indent_end_suggested: Option<bool>,
    /// Indicates if there was a suggested change to indent_first_line.
    #[serde(rename="indentFirstLineSuggested")]
    
    pub indent_first_line_suggested: Option<bool>,
    /// Indicates if there was a suggested change to indent_start.
    #[serde(rename="indentStartSuggested")]
    
    pub indent_start_suggested: Option<bool>,
    /// Indicates if there was a suggested change to keep_lines_together.
    #[serde(rename="keepLinesTogetherSuggested")]
    
    pub keep_lines_together_suggested: Option<bool>,
    /// Indicates if there was a suggested change to keep_with_next.
    #[serde(rename="keepWithNextSuggested")]
    
    pub keep_with_next_suggested: Option<bool>,
    /// Indicates if there was a suggested change to line_spacing.
    #[serde(rename="lineSpacingSuggested")]
    
    pub line_spacing_suggested: Option<bool>,
    /// Indicates if there was a suggested change to named_style_type.
    #[serde(rename="namedStyleTypeSuggested")]
    
    pub named_style_type_suggested: Option<bool>,
    /// Indicates if there was a suggested change to page_break_before.
    #[serde(rename="pageBreakBeforeSuggested")]
    
    pub page_break_before_suggested: Option<bool>,
    /// A mask that indicates which of the fields in shading have been changed in this suggestion.
    #[serde(rename="shadingSuggestionState")]
    
    pub shading_suggestion_state: Option<ShadingSuggestionState>,
    /// Indicates if there was a suggested change to space_above.
    #[serde(rename="spaceAboveSuggested")]
    
    pub space_above_suggested: Option<bool>,
    /// Indicates if there was a suggested change to space_below.
    #[serde(rename="spaceBelowSuggested")]
    
    pub space_below_suggested: Option<bool>,
    /// Indicates if there was a suggested change to spacing_mode.
    #[serde(rename="spacingModeSuggested")]
    
    pub spacing_mode_suggested: Option<bool>,
}

impl client::Part for ParagraphStyleSuggestionState {}


/// A person or email address mentioned in a document. These mentions behave as a single, immutable element containing the person's name or email address.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Person {
    /// Output only. The unique ID of this link.
    #[serde(rename="personId")]
    
    pub person_id: Option<String>,
    /// Output only. The properties of this Person. This field is always present.
    #[serde(rename="personProperties")]
    
    pub person_properties: Option<PersonProperties>,
    /// IDs for suggestions that remove this person link from the document. A Person might have multiple deletion IDs if, for example, multiple users suggest deleting it. If empty, then this person link isn't suggested for deletion.
    #[serde(rename="suggestedDeletionIds")]
    
    pub suggested_deletion_ids: Option<Vec<String>>,
    /// IDs for suggestions that insert this person link into the document. A Person might have multiple insertion IDs if it's a nested suggested change (a suggestion within a suggestion made by a different user, for example). If empty, then this person link isn't a suggested insertion.
    #[serde(rename="suggestedInsertionIds")]
    
    pub suggested_insertion_ids: Option<Vec<String>>,
    /// The suggested text style changes to this Person, keyed by suggestion ID.
    #[serde(rename="suggestedTextStyleChanges")]
    
    pub suggested_text_style_changes: Option<HashMap<String, SuggestedTextStyle>>,
    /// The text style of this Person.
    #[serde(rename="textStyle")]
    
    pub text_style: Option<TextStyle>,
}

impl client::Part for Person {}


/// Properties specific to a linked Person.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PersonProperties {
    /// Output only. The email address linked to this Person. This field is always present.
    
    pub email: Option<String>,
    /// Output only. The name of the person if it's displayed in the link text instead of the person's email address.
    
    pub name: Option<String>,
}

impl client::Part for PersonProperties {}


/// Updates the number of pinned table header rows in a table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PinTableHeaderRowsRequest {
    /// The number of table rows to pin, where 0 implies that all rows are unpinned.
    #[serde(rename="pinnedHeaderRowsCount")]
    
    pub pinned_header_rows_count: Option<i32>,
    /// The location where the table starts in the document.
    #[serde(rename="tableStartLocation")]
    
    pub table_start_location: Option<Location>,
}

impl client::Part for PinTableHeaderRowsRequest {}


/// An object that's tethered to a Paragraph and positioned relative to the beginning of the paragraph. A PositionedObject contains an EmbeddedObject such as an image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PositionedObject {
    /// The ID of this positioned object.
    #[serde(rename="objectId")]
    
    pub object_id: Option<String>,
    /// The properties of this positioned object.
    #[serde(rename="positionedObjectProperties")]
    
    pub positioned_object_properties: Option<PositionedObjectProperties>,
    /// The suggested deletion IDs. If empty, then there are no suggested deletions of this content.
    #[serde(rename="suggestedDeletionIds")]
    
    pub suggested_deletion_ids: Option<Vec<String>>,
    /// The suggested insertion ID. If empty, then this is not a suggested insertion.
    #[serde(rename="suggestedInsertionId")]
    
    pub suggested_insertion_id: Option<String>,
    /// The suggested changes to the positioned object properties, keyed by suggestion ID.
    #[serde(rename="suggestedPositionedObjectPropertiesChanges")]
    
    pub suggested_positioned_object_properties_changes: Option<HashMap<String, SuggestedPositionedObjectProperties>>,
}

impl client::Part for PositionedObject {}


/// The positioning of a PositionedObject. The positioned object is positioned relative to the beginning of the Paragraph it's tethered to.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PositionedObjectPositioning {
    /// The layout of this positioned object.
    
    pub layout: Option<String>,
    /// The offset of the left edge of the positioned object relative to the beginning of the Paragraph it's tethered to. The exact positioning of the object can depend on other content in the document and the document's styling.
    #[serde(rename="leftOffset")]
    
    pub left_offset: Option<Dimension>,
    /// The offset of the top edge of the positioned object relative to the beginning of the Paragraph it's tethered to. The exact positioning of the object can depend on other content in the document and the document's styling.
    #[serde(rename="topOffset")]
    
    pub top_offset: Option<Dimension>,
}

impl client::Part for PositionedObjectPositioning {}


/// A mask that indicates which of the fields on the base PositionedObjectPositioning have been changed in this suggestion. For any field set to true, there's a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PositionedObjectPositioningSuggestionState {
    /// Indicates if there was a suggested change to layout.
    #[serde(rename="layoutSuggested")]
    
    pub layout_suggested: Option<bool>,
    /// Indicates if there was a suggested change to left_offset.
    #[serde(rename="leftOffsetSuggested")]
    
    pub left_offset_suggested: Option<bool>,
    /// Indicates if there was a suggested change to top_offset.
    #[serde(rename="topOffsetSuggested")]
    
    pub top_offset_suggested: Option<bool>,
}

impl client::Part for PositionedObjectPositioningSuggestionState {}


/// Properties of a PositionedObject.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PositionedObjectProperties {
    /// The embedded object of this positioned object.
    #[serde(rename="embeddedObject")]
    
    pub embedded_object: Option<EmbeddedObject>,
    /// The positioning of this positioned object relative to the newline of the Paragraph that references this positioned object.
    
    pub positioning: Option<PositionedObjectPositioning>,
}

impl client::Part for PositionedObjectProperties {}


/// A mask that indicates which of the fields on the base PositionedObjectProperties have been changed in this suggestion. For any field set to true, there's a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PositionedObjectPropertiesSuggestionState {
    /// A mask that indicates which of the fields in embedded_object have been changed in this suggestion.
    #[serde(rename="embeddedObjectSuggestionState")]
    
    pub embedded_object_suggestion_state: Option<EmbeddedObjectSuggestionState>,
    /// A mask that indicates which of the fields in positioning have been changed in this suggestion.
    #[serde(rename="positioningSuggestionState")]
    
    pub positioning_suggestion_state: Option<PositionedObjectPositioningSuggestionState>,
}

impl client::Part for PositionedObjectPropertiesSuggestionState {}


/// Specifies a contiguous range of text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Range {
    /// The zero-based end index of this range, exclusive, in UTF-16 code units. In all current uses, an end index must be provided. This field is an Int32Value in order to accommodate future use cases with open-ended ranges.
    #[serde(rename="endIndex")]
    
    pub end_index: Option<i32>,
    /// The ID of the header, footer, or footnote that this range is contained in. An empty segment ID signifies the document's body.
    #[serde(rename="segmentId")]
    
    pub segment_id: Option<String>,
    /// The zero-based start index of this range, in UTF-16 code units. In all current uses, a start index must be provided. This field is an Int32Value in order to accommodate future use cases with open-ended ranges.
    #[serde(rename="startIndex")]
    
    pub start_index: Option<i32>,
}

impl client::Part for Range {}


/// Replaces all instances of text matching a criteria with replace text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReplaceAllTextRequest {
    /// Finds text in the document matching this substring.
    #[serde(rename="containsText")]
    
    pub contains_text: Option<SubstringMatchCriteria>,
    /// The text that will replace the matched text.
    #[serde(rename="replaceText")]
    
    pub replace_text: Option<String>,
}

impl client::Part for ReplaceAllTextRequest {}


/// The result of replacing text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReplaceAllTextResponse {
    /// The number of occurrences changed by replacing all text.
    #[serde(rename="occurrencesChanged")]
    
    pub occurrences_changed: Option<i32>,
}

impl client::Part for ReplaceAllTextResponse {}


/// Replaces an existing image with a new image. Replacing an image removes some image effects from the existing image in order to mirror the behavior of the Docs editor.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReplaceImageRequest {
    /// The ID of the existing image that will be replaced. The ID can be retrieved from the response of a get request.
    #[serde(rename="imageObjectId")]
    
    pub image_object_id: Option<String>,
    /// The replacement method.
    #[serde(rename="imageReplaceMethod")]
    
    pub image_replace_method: Option<String>,
    /// The URI of the new image. The image is fetched once at insertion time and a copy is stored for display inside the document. Images must be less than 50MB, cannot exceed 25 megapixels, and must be in PNG, JPEG, or GIF format. The provided URI can't surpass 2 KB in length. The URI is saved with the image, and exposed through the ImageProperties.source_uri field.
    
    pub uri: Option<String>,
}

impl client::Part for ReplaceImageRequest {}


/// Replaces the contents of the specified NamedRange or NamedRanges with the given replacement content. Note that an individual NamedRange may consist of multiple discontinuous ranges. In this case, only the content in the first range will be replaced. The other ranges and their content will be deleted. In cases where replacing or deleting any ranges would result in an invalid document structure, a 400 bad request error is returned.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReplaceNamedRangeContentRequest {
    /// The ID of the named range whose content will be replaced. If there is no named range with the given ID a 400 bad request error is returned.
    #[serde(rename="namedRangeId")]
    
    pub named_range_id: Option<String>,
    /// The name of the NamedRanges whose content will be replaced. If there are multiple named ranges with the given name, then the content of each one will be replaced. If there are no named ranges with the given name, then the request will be a no-op.
    #[serde(rename="namedRangeName")]
    
    pub named_range_name: Option<String>,
    /// Replaces the content of the specified named range(s) with the given text.
    
    pub text: Option<String>,
}

impl client::Part for ReplaceNamedRangeContentRequest {}


/// A single update to apply to a document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Request {
    /// Creates a footer.
    #[serde(rename="createFooter")]
    
    pub create_footer: Option<CreateFooterRequest>,
    /// Creates a footnote.
    #[serde(rename="createFootnote")]
    
    pub create_footnote: Option<CreateFootnoteRequest>,
    /// Creates a header.
    #[serde(rename="createHeader")]
    
    pub create_header: Option<CreateHeaderRequest>,
    /// Creates a named range.
    #[serde(rename="createNamedRange")]
    
    pub create_named_range: Option<CreateNamedRangeRequest>,
    /// Creates bullets for paragraphs.
    #[serde(rename="createParagraphBullets")]
    
    pub create_paragraph_bullets: Option<CreateParagraphBulletsRequest>,
    /// Deletes content from the document.
    #[serde(rename="deleteContentRange")]
    
    pub delete_content_range: Option<DeleteContentRangeRequest>,
    /// Deletes a footer from the document.
    #[serde(rename="deleteFooter")]
    
    pub delete_footer: Option<DeleteFooterRequest>,
    /// Deletes a header from the document.
    #[serde(rename="deleteHeader")]
    
    pub delete_header: Option<DeleteHeaderRequest>,
    /// Deletes a named range.
    #[serde(rename="deleteNamedRange")]
    
    pub delete_named_range: Option<DeleteNamedRangeRequest>,
    /// Deletes bullets from paragraphs.
    #[serde(rename="deleteParagraphBullets")]
    
    pub delete_paragraph_bullets: Option<DeleteParagraphBulletsRequest>,
    /// Deletes a positioned object from the document.
    #[serde(rename="deletePositionedObject")]
    
    pub delete_positioned_object: Option<DeletePositionedObjectRequest>,
    /// Deletes a column from a table.
    #[serde(rename="deleteTableColumn")]
    
    pub delete_table_column: Option<DeleteTableColumnRequest>,
    /// Deletes a row from a table.
    #[serde(rename="deleteTableRow")]
    
    pub delete_table_row: Option<DeleteTableRowRequest>,
    /// Inserts an inline image at the specified location.
    #[serde(rename="insertInlineImage")]
    
    pub insert_inline_image: Option<InsertInlineImageRequest>,
    /// Inserts a page break at the specified location.
    #[serde(rename="insertPageBreak")]
    
    pub insert_page_break: Option<InsertPageBreakRequest>,
    /// Inserts a section break at the specified location.
    #[serde(rename="insertSectionBreak")]
    
    pub insert_section_break: Option<InsertSectionBreakRequest>,
    /// Inserts a table at the specified location.
    #[serde(rename="insertTable")]
    
    pub insert_table: Option<InsertTableRequest>,
    /// Inserts an empty column into a table.
    #[serde(rename="insertTableColumn")]
    
    pub insert_table_column: Option<InsertTableColumnRequest>,
    /// Inserts an empty row into a table.
    #[serde(rename="insertTableRow")]
    
    pub insert_table_row: Option<InsertTableRowRequest>,
    /// Inserts text at the specified location.
    #[serde(rename="insertText")]
    
    pub insert_text: Option<InsertTextRequest>,
    /// Merges cells in a table.
    #[serde(rename="mergeTableCells")]
    
    pub merge_table_cells: Option<MergeTableCellsRequest>,
    /// Updates the number of pinned header rows in a table.
    #[serde(rename="pinTableHeaderRows")]
    
    pub pin_table_header_rows: Option<PinTableHeaderRowsRequest>,
    /// Replaces all instances of the specified text.
    #[serde(rename="replaceAllText")]
    
    pub replace_all_text: Option<ReplaceAllTextRequest>,
    /// Replaces an image in the document.
    #[serde(rename="replaceImage")]
    
    pub replace_image: Option<ReplaceImageRequest>,
    /// Replaces the content in a named range.
    #[serde(rename="replaceNamedRangeContent")]
    
    pub replace_named_range_content: Option<ReplaceNamedRangeContentRequest>,
    /// Unmerges cells in a table.
    #[serde(rename="unmergeTableCells")]
    
    pub unmerge_table_cells: Option<UnmergeTableCellsRequest>,
    /// Updates the style of the document.
    #[serde(rename="updateDocumentStyle")]
    
    pub update_document_style: Option<UpdateDocumentStyleRequest>,
    /// Updates the paragraph style at the specified range.
    #[serde(rename="updateParagraphStyle")]
    
    pub update_paragraph_style: Option<UpdateParagraphStyleRequest>,
    /// Updates the section style of the specified range.
    #[serde(rename="updateSectionStyle")]
    
    pub update_section_style: Option<UpdateSectionStyleRequest>,
    /// Updates the style of table cells.
    #[serde(rename="updateTableCellStyle")]
    
    pub update_table_cell_style: Option<UpdateTableCellStyleRequest>,
    /// Updates the properties of columns in a table.
    #[serde(rename="updateTableColumnProperties")]
    
    pub update_table_column_properties: Option<UpdateTableColumnPropertiesRequest>,
    /// Updates the row style in a table.
    #[serde(rename="updateTableRowStyle")]
    
    pub update_table_row_style: Option<UpdateTableRowStyleRequest>,
    /// Updates the text style at the specified range.
    #[serde(rename="updateTextStyle")]
    
    pub update_text_style: Option<UpdateTextStyleRequest>,
}

impl client::Part for Request {}


/// A single response from an update.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Response {
    /// The result of creating a footer.
    #[serde(rename="createFooter")]
    
    pub create_footer: Option<CreateFooterResponse>,
    /// The result of creating a footnote.
    #[serde(rename="createFootnote")]
    
    pub create_footnote: Option<CreateFootnoteResponse>,
    /// The result of creating a header.
    #[serde(rename="createHeader")]
    
    pub create_header: Option<CreateHeaderResponse>,
    /// The result of creating a named range.
    #[serde(rename="createNamedRange")]
    
    pub create_named_range: Option<CreateNamedRangeResponse>,
    /// The result of inserting an inline image.
    #[serde(rename="insertInlineImage")]
    
    pub insert_inline_image: Option<InsertInlineImageResponse>,
    /// The result of inserting an inline Google Sheets chart.
    #[serde(rename="insertInlineSheetsChart")]
    
    pub insert_inline_sheets_chart: Option<InsertInlineSheetsChartResponse>,
    /// The result of replacing text.
    #[serde(rename="replaceAllText")]
    
    pub replace_all_text: Option<ReplaceAllTextResponse>,
}

impl client::Part for Response {}


/// An RGB color.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RgbColor {
    /// The blue component of the color, from 0.0 to 1.0.
    
    pub blue: Option<f32>,
    /// The green component of the color, from 0.0 to 1.0.
    
    pub green: Option<f32>,
    /// The red component of the color, from 0.0 to 1.0.
    
    pub red: Option<f32>,
}

impl client::Part for RgbColor {}


/// A link to a Google resource (such as a file in Drive, a YouTube video, or a Calendar event).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RichLink {
    /// Output only. The ID of this link.
    #[serde(rename="richLinkId")]
    
    pub rich_link_id: Option<String>,
    /// Output only. The properties of this RichLink. This field is always present.
    #[serde(rename="richLinkProperties")]
    
    pub rich_link_properties: Option<RichLinkProperties>,
    /// IDs for suggestions that remove this link from the document. A RichLink might have multiple deletion IDs if, for example, multiple users suggest deleting it. If empty, then this person link isn't suggested for deletion.
    #[serde(rename="suggestedDeletionIds")]
    
    pub suggested_deletion_ids: Option<Vec<String>>,
    /// IDs for suggestions that insert this link into the document. A RichLink might have multiple insertion IDs if it's a nested suggested change (a suggestion within a suggestion made by a different user, for example). If empty, then this person link isn't a suggested insertion.
    #[serde(rename="suggestedInsertionIds")]
    
    pub suggested_insertion_ids: Option<Vec<String>>,
    /// The suggested text style changes to this RichLink, keyed by suggestion ID.
    #[serde(rename="suggestedTextStyleChanges")]
    
    pub suggested_text_style_changes: Option<HashMap<String, SuggestedTextStyle>>,
    /// The text style of this RichLink.
    #[serde(rename="textStyle")]
    
    pub text_style: Option<TextStyle>,
}

impl client::Part for RichLink {}


/// Properties specific to a RichLink.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RichLinkProperties {
    /// Output only. The [MIME type](https://developers.google.com/drive/api/v3/mime-types) of the RichLink, if there's one (for example, when it's a file in Drive).
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
    /// Output only. The title of the RichLink as displayed in the link. This title matches the title of the linked resource at the time of the insertion or last update of the link. This field is always present.
    
    pub title: Option<String>,
    /// Output only. The URI to the RichLink. This is always present.
    
    pub uri: Option<String>,
}

impl client::Part for RichLinkProperties {}


/// A StructuralElement representing a section break. A section is a range of content that has the same SectionStyle. A section break represents the start of a new section, and the section style applies to the section after the section break. The document body always begins with a section break.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SectionBreak {
    /// The style of the section after this section break.
    #[serde(rename="sectionStyle")]
    
    pub section_style: Option<SectionStyle>,
    /// The suggested deletion IDs. If empty, then there are no suggested deletions of this content.
    #[serde(rename="suggestedDeletionIds")]
    
    pub suggested_deletion_ids: Option<Vec<String>>,
    /// The suggested insertion IDs. A SectionBreak may have multiple insertion IDs if it's a nested suggested change. If empty, then this is not a suggested insertion.
    #[serde(rename="suggestedInsertionIds")]
    
    pub suggested_insertion_ids: Option<Vec<String>>,
}

impl client::Part for SectionBreak {}


/// Properties that apply to a section's column.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SectionColumnProperties {
    /// The padding at the end of the column.
    #[serde(rename="paddingEnd")]
    
    pub padding_end: Option<Dimension>,
    /// Output only. The width of the column.
    
    pub width: Option<Dimension>,
}

impl client::Part for SectionColumnProperties {}


/// The styling that applies to a section.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SectionStyle {
    /// The section's columns properties. If empty, the section contains one column with the default properties in the Docs editor. A section can be updated to have no more than 3 columns. When updating this property, setting a concrete value is required. Unsetting this property will result in a 400 bad request error.
    #[serde(rename="columnProperties")]
    
    pub column_properties: Option<Vec<SectionColumnProperties>>,
    /// The style of column separators. This style can be set even when there's one column in the section. When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error.
    #[serde(rename="columnSeparatorStyle")]
    
    pub column_separator_style: Option<String>,
    /// The content direction of this section. If unset, the value defaults to LEFT_TO_RIGHT. When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error.
    #[serde(rename="contentDirection")]
    
    pub content_direction: Option<String>,
    /// The ID of the default footer. If unset, the value inherits from the previous SectionBreak's SectionStyle. If the value is unset in the first SectionBreak, it inherits from DocumentStyle's default_footer_id. This property is read-only.
    #[serde(rename="defaultFooterId")]
    
    pub default_footer_id: Option<String>,
    /// The ID of the default header. If unset, the value inherits from the previous SectionBreak's SectionStyle. If the value is unset in the first SectionBreak, it inherits from DocumentStyle's default_header_id. This property is read-only.
    #[serde(rename="defaultHeaderId")]
    
    pub default_header_id: Option<String>,
    /// The ID of the footer used only for even pages. If the value of DocumentStyle's use_even_page_header_footer is true, this value is used for the footers on even pages in the section. If it is false, the footers on even pages use the default_footer_id. If unset, the value inherits from the previous SectionBreak's SectionStyle. If the value is unset in the first SectionBreak, it inherits from DocumentStyle's even_page_footer_id. This property is read-only.
    #[serde(rename="evenPageFooterId")]
    
    pub even_page_footer_id: Option<String>,
    /// The ID of the header used only for even pages. If the value of DocumentStyle's use_even_page_header_footer is true, this value is used for the headers on even pages in the section. If it is false, the headers on even pages use the default_header_id. If unset, the value inherits from the previous SectionBreak's SectionStyle. If the value is unset in the first SectionBreak, it inherits from DocumentStyle's even_page_header_id. This property is read-only.
    #[serde(rename="evenPageHeaderId")]
    
    pub even_page_header_id: Option<String>,
    /// The ID of the footer used only for the first page of the section. If use_first_page_header_footer is true, this value is used for the footer on the first page of the section. If it's false, the footer on the first page of the section uses the default_footer_id. If unset, the value inherits from the previous SectionBreak's SectionStyle. If the value is unset in the first SectionBreak, it inherits from DocumentStyle's first_page_footer_id. This property is read-only.
    #[serde(rename="firstPageFooterId")]
    
    pub first_page_footer_id: Option<String>,
    /// The ID of the header used only for the first page of the section. If use_first_page_header_footer is true, this value is used for the header on the first page of the section. If it's false, the header on the first page of the section uses the default_header_id. If unset, the value inherits from the previous SectionBreak's SectionStyle. If the value is unset in the first SectionBreak, it inherits from DocumentStyle's first_page_header_id. This property is read-only.
    #[serde(rename="firstPageHeaderId")]
    
    pub first_page_header_id: Option<String>,
    /// Optional. Indicates whether to flip the dimensions of DocumentStyle's page_size for this section, which allows changing the page orientation between portrait and landscape. If unset, the value inherits from DocumentStyle's flip_page_orientation. When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error.
    #[serde(rename="flipPageOrientation")]
    
    pub flip_page_orientation: Option<bool>,
    /// The bottom page margin of the section. If unset, the value defaults to margin_bottom from DocumentStyle. When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error.
    #[serde(rename="marginBottom")]
    
    pub margin_bottom: Option<Dimension>,
    /// The footer margin of the section. If unset, the value defaults to margin_footer from DocumentStyle. If updated, use_custom_header_footer_margins is set to true on DocumentStyle. The value of use_custom_header_footer_margins on DocumentStyle indicates if a footer margin is being respected for this section When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error.
    #[serde(rename="marginFooter")]
    
    pub margin_footer: Option<Dimension>,
    /// The header margin of the section. If unset, the value defaults to margin_header from DocumentStyle. If updated, use_custom_header_footer_margins is set to true on DocumentStyle. The value of use_custom_header_footer_margins on DocumentStyle indicates if a header margin is being respected for this section. When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error.
    #[serde(rename="marginHeader")]
    
    pub margin_header: Option<Dimension>,
    /// The left page margin of the section. If unset, the value defaults to margin_left from DocumentStyle. Updating the left margin causes columns in this section to resize. Since the margin affects column width, it's applied before column properties. When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error.
    #[serde(rename="marginLeft")]
    
    pub margin_left: Option<Dimension>,
    /// The right page margin of the section. If unset, the value defaults to margin_right from DocumentStyle. Updating the right margin causes columns in this section to resize. Since the margin affects column width, it's applied before column properties. When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error.
    #[serde(rename="marginRight")]
    
    pub margin_right: Option<Dimension>,
    /// The top page margin of the section. If unset, the value defaults to margin_top from DocumentStyle. When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error.
    #[serde(rename="marginTop")]
    
    pub margin_top: Option<Dimension>,
    /// The page number from which to start counting the number of pages for this section. If unset, page numbering continues from the previous section. If the value is unset in the first SectionBreak, refer to DocumentStyle's page_number_start. When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error.
    #[serde(rename="pageNumberStart")]
    
    pub page_number_start: Option<i32>,
    /// Output only. The type of section.
    #[serde(rename="sectionType")]
    
    pub section_type: Option<String>,
    /// Indicates whether to use the first page header / footer IDs for the first page of the section. If unset, it inherits from DocumentStyle's use_first_page_header_footer for the first section. If the value is unset for subsequent sectors, it should be interpreted as false. When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error.
    #[serde(rename="useFirstPageHeaderFooter")]
    
    pub use_first_page_header_footer: Option<bool>,
}

impl client::Part for SectionStyle {}


/// The shading of a paragraph.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Shading {
    /// The background color of this paragraph shading.
    #[serde(rename="backgroundColor")]
    
    pub background_color: Option<OptionalColor>,
}

impl client::Part for Shading {}


/// A mask that indicates which of the fields on the base Shading have been changed in this suggested change. For any field set to true, there's a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ShadingSuggestionState {
    /// Indicates if there was a suggested change to the Shading.
    #[serde(rename="backgroundColorSuggested")]
    
    pub background_color_suggested: Option<bool>,
}

impl client::Part for ShadingSuggestionState {}


/// A reference to a linked chart embedded from Google Sheets.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SheetsChartReference {
    /// The ID of the specific chart in the Google Sheets spreadsheet that's embedded.
    #[serde(rename="chartId")]
    
    pub chart_id: Option<i32>,
    /// The ID of the Google Sheets spreadsheet that contains the source chart.
    #[serde(rename="spreadsheetId")]
    
    pub spreadsheet_id: Option<String>,
}

impl client::Part for SheetsChartReference {}


/// A mask that indicates which of the fields on the base SheetsChartReference have been changed in this suggestion. For any field set to true, there's a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SheetsChartReferenceSuggestionState {
    /// Indicates if there was a suggested change to chart_id.
    #[serde(rename="chartIdSuggested")]
    
    pub chart_id_suggested: Option<bool>,
    /// Indicates if there was a suggested change to spreadsheet_id.
    #[serde(rename="spreadsheetIdSuggested")]
    
    pub spreadsheet_id_suggested: Option<bool>,
}

impl client::Part for SheetsChartReferenceSuggestionState {}


/// A width and height.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Size {
    /// The height of the object.
    
    pub height: Option<Dimension>,
    /// The width of the object.
    
    pub width: Option<Dimension>,
}

impl client::Part for Size {}


/// A mask that indicates which of the fields on the base Size have been changed in this suggestion. For any field set to true, the Size has a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SizeSuggestionState {
    /// Indicates if there was a suggested change to height.
    #[serde(rename="heightSuggested")]
    
    pub height_suggested: Option<bool>,
    /// Indicates if there was a suggested change to width.
    #[serde(rename="widthSuggested")]
    
    pub width_suggested: Option<bool>,
}

impl client::Part for SizeSuggestionState {}


/// A StructuralElement describes content that provides structure to the document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StructuralElement {
    /// The zero-based end index of this structural element, exclusive, in UTF-16 code units.
    #[serde(rename="endIndex")]
    
    pub end_index: Option<i32>,
    /// A paragraph type of structural element.
    
    pub paragraph: Option<Paragraph>,
    /// A section break type of structural element.
    #[serde(rename="sectionBreak")]
    
    pub section_break: Option<SectionBreak>,
    /// The zero-based start index of this structural element, in UTF-16 code units.
    #[serde(rename="startIndex")]
    
    pub start_index: Option<i32>,
    /// A table type of structural element.
    
    pub table: Option<Table>,
    /// A table of contents type of structural element.
    #[serde(rename="tableOfContents")]
    
    pub table_of_contents: Option<TableOfContents>,
}

impl client::Part for StructuralElement {}


/// A criteria that matches a specific string of text in the document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubstringMatchCriteria {
    /// Indicates whether the search should respect case: - `True`: the search is case sensitive. - `False`: the search is case insensitive.
    #[serde(rename="matchCase")]
    
    pub match_case: Option<bool>,
    /// The text to search for in the document.
    
    pub text: Option<String>,
}

impl client::Part for SubstringMatchCriteria {}


/// A suggested change to a Bullet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SuggestedBullet {
    /// A Bullet that only includes the changes made in this suggestion. This can be used along with the bullet_suggestion_state to see which fields have changed and their new values.
    
    pub bullet: Option<Bullet>,
    /// A mask that indicates which of the fields on the base Bullet have been changed in this suggestion.
    #[serde(rename="bulletSuggestionState")]
    
    pub bullet_suggestion_state: Option<BulletSuggestionState>,
}

impl client::Part for SuggestedBullet {}


/// A suggested change to the DocumentStyle.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SuggestedDocumentStyle {
    /// A DocumentStyle that only includes the changes made in this suggestion. This can be used along with the document_style_suggestion_state to see which fields have changed and their new values.
    #[serde(rename="documentStyle")]
    
    pub document_style: Option<DocumentStyle>,
    /// A mask that indicates which of the fields on the base DocumentStyle have been changed in this suggestion.
    #[serde(rename="documentStyleSuggestionState")]
    
    pub document_style_suggestion_state: Option<DocumentStyleSuggestionState>,
}

impl client::Part for SuggestedDocumentStyle {}


/// A suggested change to InlineObjectProperties.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SuggestedInlineObjectProperties {
    /// An InlineObjectProperties that only includes the changes made in this suggestion. This can be used along with the inline_object_properties_suggestion_state to see which fields have changed and their new values.
    #[serde(rename="inlineObjectProperties")]
    
    pub inline_object_properties: Option<InlineObjectProperties>,
    /// A mask that indicates which of the fields on the base InlineObjectProperties have been changed in this suggestion.
    #[serde(rename="inlineObjectPropertiesSuggestionState")]
    
    pub inline_object_properties_suggestion_state: Option<InlineObjectPropertiesSuggestionState>,
}

impl client::Part for SuggestedInlineObjectProperties {}


/// A suggested change to ListProperties.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SuggestedListProperties {
    /// A ListProperties that only includes the changes made in this suggestion. This can be used along with the list_properties_suggestion_state to see which fields have changed and their new values.
    #[serde(rename="listProperties")]
    
    pub list_properties: Option<ListProperties>,
    /// A mask that indicates which of the fields on the base ListProperties have been changed in this suggestion.
    #[serde(rename="listPropertiesSuggestionState")]
    
    pub list_properties_suggestion_state: Option<ListPropertiesSuggestionState>,
}

impl client::Part for SuggestedListProperties {}


/// A suggested change to the NamedStyles.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SuggestedNamedStyles {
    /// A NamedStyles that only includes the changes made in this suggestion. This can be used along with the named_styles_suggestion_state to see which fields have changed and their new values.
    #[serde(rename="namedStyles")]
    
    pub named_styles: Option<NamedStyles>,
    /// A mask that indicates which of the fields on the base NamedStyles have been changed in this suggestion.
    #[serde(rename="namedStylesSuggestionState")]
    
    pub named_styles_suggestion_state: Option<NamedStylesSuggestionState>,
}

impl client::Part for SuggestedNamedStyles {}


/// A suggested change to a ParagraphStyle.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SuggestedParagraphStyle {
    /// A ParagraphStyle that only includes the changes made in this suggestion. This can be used along with the paragraph_style_suggestion_state to see which fields have changed and their new values.
    #[serde(rename="paragraphStyle")]
    
    pub paragraph_style: Option<ParagraphStyle>,
    /// A mask that indicates which of the fields on the base ParagraphStyle have been changed in this suggestion.
    #[serde(rename="paragraphStyleSuggestionState")]
    
    pub paragraph_style_suggestion_state: Option<ParagraphStyleSuggestionState>,
}

impl client::Part for SuggestedParagraphStyle {}


/// A suggested change to PositionedObjectProperties.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SuggestedPositionedObjectProperties {
    /// A PositionedObjectProperties that only includes the changes made in this suggestion. This can be used along with the positioned_object_properties_suggestion_state to see which fields have changed and their new values.
    #[serde(rename="positionedObjectProperties")]
    
    pub positioned_object_properties: Option<PositionedObjectProperties>,
    /// A mask that indicates which of the fields on the base PositionedObjectProperties have been changed in this suggestion.
    #[serde(rename="positionedObjectPropertiesSuggestionState")]
    
    pub positioned_object_properties_suggestion_state: Option<PositionedObjectPropertiesSuggestionState>,
}

impl client::Part for SuggestedPositionedObjectProperties {}


/// A suggested change to a TableCellStyle.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SuggestedTableCellStyle {
    /// A TableCellStyle that only includes the changes made in this suggestion. This can be used along with the table_cell_style_suggestion_state to see which fields have changed and their new values.
    #[serde(rename="tableCellStyle")]
    
    pub table_cell_style: Option<TableCellStyle>,
    /// A mask that indicates which of the fields on the base TableCellStyle have been changed in this suggestion.
    #[serde(rename="tableCellStyleSuggestionState")]
    
    pub table_cell_style_suggestion_state: Option<TableCellStyleSuggestionState>,
}

impl client::Part for SuggestedTableCellStyle {}


/// A suggested change to a TableRowStyle.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SuggestedTableRowStyle {
    /// A TableRowStyle that only includes the changes made in this suggestion. This can be used along with the table_row_style_suggestion_state to see which fields have changed and their new values.
    #[serde(rename="tableRowStyle")]
    
    pub table_row_style: Option<TableRowStyle>,
    /// A mask that indicates which of the fields on the base TableRowStyle have been changed in this suggestion.
    #[serde(rename="tableRowStyleSuggestionState")]
    
    pub table_row_style_suggestion_state: Option<TableRowStyleSuggestionState>,
}

impl client::Part for SuggestedTableRowStyle {}


/// A suggested change to a TextStyle.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SuggestedTextStyle {
    /// A TextStyle that only includes the changes made in this suggestion. This can be used along with the text_style_suggestion_state to see which fields have changed and their new values.
    #[serde(rename="textStyle")]
    
    pub text_style: Option<TextStyle>,
    /// A mask that indicates which of the fields on the base TextStyle have been changed in this suggestion.
    #[serde(rename="textStyleSuggestionState")]
    
    pub text_style_suggestion_state: Option<TextStyleSuggestionState>,
}

impl client::Part for SuggestedTextStyle {}


/// A tab stop within a paragraph.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TabStop {
    /// The alignment of this tab stop. If unset, the value defaults to START.
    
    pub alignment: Option<String>,
    /// The offset between this tab stop and the start margin.
    
    pub offset: Option<Dimension>,
}

impl client::Part for TabStop {}


/// A StructuralElement representing a table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Table {
    /// Number of columns in the table. It's possible for a table to be non-rectangular, so some rows may have a different number of cells.
    
    pub columns: Option<i32>,
    /// Number of rows in the table.
    
    pub rows: Option<i32>,
    /// The suggested deletion IDs. If empty, then there are no suggested deletions of this content.
    #[serde(rename="suggestedDeletionIds")]
    
    pub suggested_deletion_ids: Option<Vec<String>>,
    /// The suggested insertion IDs. A Table may have multiple insertion IDs if it's a nested suggested change. If empty, then this is not a suggested insertion.
    #[serde(rename="suggestedInsertionIds")]
    
    pub suggested_insertion_ids: Option<Vec<String>>,
    /// The contents and style of each row.
    #[serde(rename="tableRows")]
    
    pub table_rows: Option<Vec<TableRow>>,
    /// The style of the table.
    #[serde(rename="tableStyle")]
    
    pub table_style: Option<TableStyle>,
}

impl client::Part for Table {}


/// The contents and style of a cell in a Table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
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
    /// The suggested deletion IDs. If empty, then there are no suggested deletions of this content.
    #[serde(rename="suggestedDeletionIds")]
    
    pub suggested_deletion_ids: Option<Vec<String>>,
    /// The suggested insertion IDs. A TableCell may have multiple insertion IDs if it's a nested suggested change. If empty, then this is not a suggested insertion.
    #[serde(rename="suggestedInsertionIds")]
    
    pub suggested_insertion_ids: Option<Vec<String>>,
    /// The suggested changes to the table cell style, keyed by suggestion ID.
    #[serde(rename="suggestedTableCellStyleChanges")]
    
    pub suggested_table_cell_style_changes: Option<HashMap<String, SuggestedTableCellStyle>>,
    /// The style of the cell.
    #[serde(rename="tableCellStyle")]
    
    pub table_cell_style: Option<TableCellStyle>,
}

impl client::Part for TableCell {}


/// A border around a table cell. Table cell borders cannot be transparent. To hide a table cell border, make its width 0.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableCellBorder {
    /// The color of the border. This color cannot be transparent.
    
    pub color: Option<OptionalColor>,
    /// The dash style of the border.
    #[serde(rename="dashStyle")]
    
    pub dash_style: Option<String>,
    /// The width of the border.
    
    pub width: Option<Dimension>,
}

impl client::Part for TableCellBorder {}


/// Location of a single cell within a table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableCellLocation {
    /// The zero-based column index. For example, the second column in the table has a column index of 1.
    #[serde(rename="columnIndex")]
    
    pub column_index: Option<i32>,
    /// The zero-based row index. For example, the second row in the table has a row index of 1.
    #[serde(rename="rowIndex")]
    
    pub row_index: Option<i32>,
    /// The location where the table starts in the document.
    #[serde(rename="tableStartLocation")]
    
    pub table_start_location: Option<Location>,
}

impl client::Part for TableCellLocation {}


/// The style of a TableCell. Inherited table cell styles are represented as unset fields in this message. A table cell style can inherit from the table's style.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableCellStyle {
    /// The background color of the cell.
    #[serde(rename="backgroundColor")]
    
    pub background_color: Option<OptionalColor>,
    /// The bottom border of the cell.
    #[serde(rename="borderBottom")]
    
    pub border_bottom: Option<TableCellBorder>,
    /// The left border of the cell.
    #[serde(rename="borderLeft")]
    
    pub border_left: Option<TableCellBorder>,
    /// The right border of the cell.
    #[serde(rename="borderRight")]
    
    pub border_right: Option<TableCellBorder>,
    /// The top border of the cell.
    #[serde(rename="borderTop")]
    
    pub border_top: Option<TableCellBorder>,
    /// The column span of the cell. This property is read-only.
    #[serde(rename="columnSpan")]
    
    pub column_span: Option<i32>,
    /// The alignment of the content in the table cell. The default alignment matches the alignment for newly created table cells in the Docs editor.
    #[serde(rename="contentAlignment")]
    
    pub content_alignment: Option<String>,
    /// The bottom padding of the cell.
    #[serde(rename="paddingBottom")]
    
    pub padding_bottom: Option<Dimension>,
    /// The left padding of the cell.
    #[serde(rename="paddingLeft")]
    
    pub padding_left: Option<Dimension>,
    /// The right padding of the cell.
    #[serde(rename="paddingRight")]
    
    pub padding_right: Option<Dimension>,
    /// The top padding of the cell.
    #[serde(rename="paddingTop")]
    
    pub padding_top: Option<Dimension>,
    /// The row span of the cell. This property is read-only.
    #[serde(rename="rowSpan")]
    
    pub row_span: Option<i32>,
}

impl client::Part for TableCellStyle {}


/// A mask that indicates which of the fields on the base TableCellStyle have been changed in this suggestion. For any field set to true, there's a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableCellStyleSuggestionState {
    /// Indicates if there was a suggested change to background_color.
    #[serde(rename="backgroundColorSuggested")]
    
    pub background_color_suggested: Option<bool>,
    /// Indicates if there was a suggested change to border_bottom.
    #[serde(rename="borderBottomSuggested")]
    
    pub border_bottom_suggested: Option<bool>,
    /// Indicates if there was a suggested change to border_left.
    #[serde(rename="borderLeftSuggested")]
    
    pub border_left_suggested: Option<bool>,
    /// Indicates if there was a suggested change to border_right.
    #[serde(rename="borderRightSuggested")]
    
    pub border_right_suggested: Option<bool>,
    /// Indicates if there was a suggested change to border_top.
    #[serde(rename="borderTopSuggested")]
    
    pub border_top_suggested: Option<bool>,
    /// Indicates if there was a suggested change to column_span.
    #[serde(rename="columnSpanSuggested")]
    
    pub column_span_suggested: Option<bool>,
    /// Indicates if there was a suggested change to content_alignment.
    #[serde(rename="contentAlignmentSuggested")]
    
    pub content_alignment_suggested: Option<bool>,
    /// Indicates if there was a suggested change to padding_bottom.
    #[serde(rename="paddingBottomSuggested")]
    
    pub padding_bottom_suggested: Option<bool>,
    /// Indicates if there was a suggested change to padding_left.
    #[serde(rename="paddingLeftSuggested")]
    
    pub padding_left_suggested: Option<bool>,
    /// Indicates if there was a suggested change to padding_right.
    #[serde(rename="paddingRightSuggested")]
    
    pub padding_right_suggested: Option<bool>,
    /// Indicates if there was a suggested change to padding_top.
    #[serde(rename="paddingTopSuggested")]
    
    pub padding_top_suggested: Option<bool>,
    /// Indicates if there was a suggested change to row_span.
    #[serde(rename="rowSpanSuggested")]
    
    pub row_span_suggested: Option<bool>,
}

impl client::Part for TableCellStyleSuggestionState {}


/// The properties of a column in a table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableColumnProperties {
    /// The width of the column. Set when the column's `width_type` is FIXED_WIDTH.
    
    pub width: Option<Dimension>,
    /// The width type of the column.
    #[serde(rename="widthType")]
    
    pub width_type: Option<String>,
}

impl client::Part for TableColumnProperties {}


/// A StructuralElement representing a table of contents.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableOfContents {
    /// The content of the table of contents.
    
    pub content: Option<Vec<StructuralElement>>,
    /// The suggested deletion IDs. If empty, then there are no suggested deletions of this content.
    #[serde(rename="suggestedDeletionIds")]
    
    pub suggested_deletion_ids: Option<Vec<String>>,
    /// The suggested insertion IDs. A TableOfContents may have multiple insertion IDs if it is a nested suggested change. If empty, then this is not a suggested insertion.
    #[serde(rename="suggestedInsertionIds")]
    
    pub suggested_insertion_ids: Option<Vec<String>>,
}

impl client::Part for TableOfContents {}


/// A table range represents a reference to a subset of a table. It's important to note that the cells specified by a table range do not necessarily form a rectangle. For example, let's say we have a 3 x 3 table where all the cells of the last row are merged together. The table looks like this: [ ] A table range with table cell location = (table_start_location, row = 0, column = 0), row span = 3 and column span = 2 specifies the following cells: x x [ x x x ]
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableRange {
    /// The column span of the table range.
    #[serde(rename="columnSpan")]
    
    pub column_span: Option<i32>,
    /// The row span of the table range.
    #[serde(rename="rowSpan")]
    
    pub row_span: Option<i32>,
    /// The cell location where the table range starts.
    #[serde(rename="tableCellLocation")]
    
    pub table_cell_location: Option<TableCellLocation>,
}

impl client::Part for TableRange {}


/// The contents and style of a row in a Table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableRow {
    /// The zero-based end index of this row, exclusive, in UTF-16 code units.
    #[serde(rename="endIndex")]
    
    pub end_index: Option<i32>,
    /// The zero-based start index of this row, in UTF-16 code units.
    #[serde(rename="startIndex")]
    
    pub start_index: Option<i32>,
    /// The suggested deletion IDs. If empty, then there are no suggested deletions of this content.
    #[serde(rename="suggestedDeletionIds")]
    
    pub suggested_deletion_ids: Option<Vec<String>>,
    /// The suggested insertion IDs. A TableRow may have multiple insertion IDs if it's a nested suggested change. If empty, then this is not a suggested insertion.
    #[serde(rename="suggestedInsertionIds")]
    
    pub suggested_insertion_ids: Option<Vec<String>>,
    /// The suggested style changes to this row, keyed by suggestion ID.
    #[serde(rename="suggestedTableRowStyleChanges")]
    
    pub suggested_table_row_style_changes: Option<HashMap<String, SuggestedTableRowStyle>>,
    /// The contents and style of each cell in this row. It's possible for a table to be non-rectangular, so some rows may have a different number of cells than other rows in the same table.
    #[serde(rename="tableCells")]
    
    pub table_cells: Option<Vec<TableCell>>,
    /// The style of the table row.
    #[serde(rename="tableRowStyle")]
    
    pub table_row_style: Option<TableRowStyle>,
}

impl client::Part for TableRow {}


/// Styles that apply to a table row.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableRowStyle {
    /// The minimum height of the row. The row will be rendered in the Docs editor at a height equal to or greater than this value in order to show all the content in the row's cells.
    #[serde(rename="minRowHeight")]
    
    pub min_row_height: Option<Dimension>,
    /// Whether the row cannot overflow across page or column boundaries.
    #[serde(rename="preventOverflow")]
    
    pub prevent_overflow: Option<bool>,
    /// Whether the row is a table header.
    #[serde(rename="tableHeader")]
    
    pub table_header: Option<bool>,
}

impl client::Part for TableRowStyle {}


/// A mask that indicates which of the fields on the base TableRowStyle have been changed in this suggestion. For any field set to true, there's a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableRowStyleSuggestionState {
    /// Indicates if there was a suggested change to min_row_height.
    #[serde(rename="minRowHeightSuggested")]
    
    pub min_row_height_suggested: Option<bool>,
}

impl client::Part for TableRowStyleSuggestionState {}


/// Styles that apply to a table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableStyle {
    /// The properties of each column. Note that in Docs, tables contain rows and rows contain cells, similar to HTML. So the properties for a row can be found on the row's table_row_style.
    #[serde(rename="tableColumnProperties")]
    
    pub table_column_properties: Option<Vec<TableColumnProperties>>,
}

impl client::Part for TableStyle {}


/// A ParagraphElement that represents a run of text that all has the same styling.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TextRun {
    /// The text of this run. Any non-text elements in the run are replaced with the Unicode character U+E907.
    
    pub content: Option<String>,
    /// The suggested deletion IDs. If empty, then there are no suggested deletions of this content.
    #[serde(rename="suggestedDeletionIds")]
    
    pub suggested_deletion_ids: Option<Vec<String>>,
    /// The suggested insertion IDs. A TextRun may have multiple insertion IDs if it's a nested suggested change. If empty, then this is not a suggested insertion.
    #[serde(rename="suggestedInsertionIds")]
    
    pub suggested_insertion_ids: Option<Vec<String>>,
    /// The suggested text style changes to this run, keyed by suggestion ID.
    #[serde(rename="suggestedTextStyleChanges")]
    
    pub suggested_text_style_changes: Option<HashMap<String, SuggestedTextStyle>>,
    /// The text style of this run.
    #[serde(rename="textStyle")]
    
    pub text_style: Option<TextStyle>,
}

impl client::Part for TextRun {}


/// Represents the styling that can be applied to text. Inherited text styles are represented as unset fields in this message. A text style's parent depends on where the text style is defined: * The TextStyle of text in a Paragraph inherits from the paragraph's corresponding named style type. * The TextStyle on a named style inherits from the normal text named style. * The TextStyle of the normal text named style inherits from the default text style in the Docs editor. * The TextStyle on a Paragraph element that's contained in a table may inherit its text style from the table style. If the text style does not inherit from a parent, unsetting fields will revert the style to a value matching the defaults in the Docs editor.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TextStyle {
    /// The background color of the text. If set, the color is either an RGB color or transparent, depending on the `color` field.
    #[serde(rename="backgroundColor")]
    
    pub background_color: Option<OptionalColor>,
    /// The text's vertical offset from its normal position. Text with `SUPERSCRIPT` or `SUBSCRIPT` baseline offsets is automatically rendered in a smaller font size, computed based on the `font_size` field. Changes in this field don't affect the `font_size`.
    #[serde(rename="baselineOffset")]
    
    pub baseline_offset: Option<String>,
    /// Whether or not the text is rendered as bold.
    
    pub bold: Option<bool>,
    /// The size of the text's font.
    #[serde(rename="fontSize")]
    
    pub font_size: Option<Dimension>,
    /// The foreground color of the text. If set, the color is either an RGB color or transparent, depending on the `color` field.
    #[serde(rename="foregroundColor")]
    
    pub foreground_color: Option<OptionalColor>,
    /// Whether or not the text is italicized.
    
    pub italic: Option<bool>,
    /// The hyperlink destination of the text. If unset, there's no link. Links are not inherited from parent text. Changing the link in an update request causes some other changes to the text style of the range: * When setting a link, the text foreground color will be updated to the default link color and the text will be underlined. If these fields are modified in the same request, those values will be used instead of the link defaults. * Setting a link on a text range that overlaps with an existing link will also update the existing link to point to the new URL. * Links are not settable on newline characters. As a result, setting a link on a text range that crosses a paragraph boundary, such as `"ABC\n123"`, will separate the newline character(s) into their own text runs. The link will be applied separately to the runs before and after the newline. * Removing a link will update the text style of the range to match the style of the preceding text (or the default text styles if the preceding text is another link) unless different styles are being set in the same request.
    
    pub link: Option<Link>,
    /// Whether or not the text is in small capital letters.
    #[serde(rename="smallCaps")]
    
    pub small_caps: Option<bool>,
    /// Whether or not the text is struck through.
    
    pub strikethrough: Option<bool>,
    /// Whether or not the text is underlined.
    
    pub underline: Option<bool>,
    /// The font family and rendered weight of the text. If an update request specifies values for both `weighted_font_family` and `bold`, the `weighted_font_family` is applied first, then `bold`. If `weighted_font_family#weight` is not set, it defaults to `400`. If `weighted_font_family` is set, then `weighted_font_family#font_family` must also be set with a non-empty value. Otherwise, a 400 bad request error is returned.
    #[serde(rename="weightedFontFamily")]
    
    pub weighted_font_family: Option<WeightedFontFamily>,
}

impl client::Part for TextStyle {}


/// A mask that indicates which of the fields on the base TextStyle have been changed in this suggestion. For any field set to true, there's a new suggested value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TextStyleSuggestionState {
    /// Indicates if there was a suggested change to background_color.
    #[serde(rename="backgroundColorSuggested")]
    
    pub background_color_suggested: Option<bool>,
    /// Indicates if there was a suggested change to baseline_offset.
    #[serde(rename="baselineOffsetSuggested")]
    
    pub baseline_offset_suggested: Option<bool>,
    /// Indicates if there was a suggested change to bold.
    #[serde(rename="boldSuggested")]
    
    pub bold_suggested: Option<bool>,
    /// Indicates if there was a suggested change to font_size.
    #[serde(rename="fontSizeSuggested")]
    
    pub font_size_suggested: Option<bool>,
    /// Indicates if there was a suggested change to foreground_color.
    #[serde(rename="foregroundColorSuggested")]
    
    pub foreground_color_suggested: Option<bool>,
    /// Indicates if there was a suggested change to italic.
    #[serde(rename="italicSuggested")]
    
    pub italic_suggested: Option<bool>,
    /// Indicates if there was a suggested change to link.
    #[serde(rename="linkSuggested")]
    
    pub link_suggested: Option<bool>,
    /// Indicates if there was a suggested change to small_caps.
    #[serde(rename="smallCapsSuggested")]
    
    pub small_caps_suggested: Option<bool>,
    /// Indicates if there was a suggested change to strikethrough.
    #[serde(rename="strikethroughSuggested")]
    
    pub strikethrough_suggested: Option<bool>,
    /// Indicates if there was a suggested change to underline.
    #[serde(rename="underlineSuggested")]
    
    pub underline_suggested: Option<bool>,
    /// Indicates if there was a suggested change to weighted_font_family.
    #[serde(rename="weightedFontFamilySuggested")]
    
    pub weighted_font_family_suggested: Option<bool>,
}

impl client::Part for TextStyleSuggestionState {}


/// Unmerges cells in a Table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UnmergeTableCellsRequest {
    /// The table range specifying which cells of the table to unmerge. All merged cells in this range will be unmerged, and cells that are already unmerged will not be affected. If the range has no merged cells, the request will do nothing. If there is text in any of the merged cells, the text will remain in the "head" cell of the resulting block of unmerged cells. The "head" cell is the upper-left cell when the content direction is from left to right, and the upper-right otherwise.
    #[serde(rename="tableRange")]
    
    pub table_range: Option<TableRange>,
}

impl client::Part for UnmergeTableCellsRequest {}


/// Updates the DocumentStyle.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateDocumentStyleRequest {
    /// The styles to set on the document. Certain document style changes may cause other changes in order to mirror the behavior of the Docs editor. See the documentation of DocumentStyle for more information.
    #[serde(rename="documentStyle")]
    
    pub document_style: Option<DocumentStyle>,
    /// The fields that should be updated. At least one field must be specified. The root `document_style` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field. For example to update the background, set `fields` to `"background"`.
    
    pub fields: Option<client::FieldMask>,
}

impl client::Part for UpdateDocumentStyleRequest {}


/// Update the styling of all paragraphs that overlap with the given range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateParagraphStyleRequest {
    /// The fields that should be updated. At least one field must be specified. The root `paragraph_style` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field. For example, to update the paragraph style's alignment property, set `fields` to `"alignment"`. To reset a property to its default value, include its field name in the field mask but leave the field itself unset.
    
    pub fields: Option<client::FieldMask>,
    /// The styles to set on the paragraphs. Certain paragraph style changes may cause other changes in order to mirror the behavior of the Docs editor. See the documentation of ParagraphStyle for more information.
    #[serde(rename="paragraphStyle")]
    
    pub paragraph_style: Option<ParagraphStyle>,
    /// The range overlapping the paragraphs to style.
    
    pub range: Option<Range>,
}

impl client::Part for UpdateParagraphStyleRequest {}


/// Updates the SectionStyle.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateSectionStyleRequest {
    /// The fields that should be updated. At least one field must be specified. The root `section_style` is implied and must not be specified. A single `"*"` can be used as short-hand for listing every field. For example to update the left margin, set `fields` to `"margin_left"`.
    
    pub fields: Option<client::FieldMask>,
    /// The range overlapping the sections to style. Because section breaks can only be inserted inside the body, the segment ID field must be empty.
    
    pub range: Option<Range>,
    /// The styles to be set on the section. Certain section style changes may cause other changes in order to mirror the behavior of the Docs editor. See the documentation of SectionStyle for more information.
    #[serde(rename="sectionStyle")]
    
    pub section_style: Option<SectionStyle>,
}

impl client::Part for UpdateSectionStyleRequest {}


/// Updates the style of a range of table cells.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateTableCellStyleRequest {
    /// The fields that should be updated. At least one field must be specified. The root `tableCellStyle` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field. For example to update the table cell background color, set `fields` to `"backgroundColor"`. To reset a property to its default value, include its field name in the field mask but leave the field itself unset.
    
    pub fields: Option<client::FieldMask>,
    /// The style to set on the table cells. When updating borders, if a cell shares a border with an adjacent cell, the corresponding border property of the adjacent cell is updated as well. Borders that are merged and invisible are not updated. Since updating a border shared by adjacent cells in the same request can cause conflicting border updates, border updates are applied in the following order: - `border_right` - `border_left` - `border_bottom` - `border_top`
    #[serde(rename="tableCellStyle")]
    
    pub table_cell_style: Option<TableCellStyle>,
    /// The table range representing the subset of the table to which the updates are applied.
    #[serde(rename="tableRange")]
    
    pub table_range: Option<TableRange>,
    /// The location where the table starts in the document. When specified, the updates are applied to all the cells in the table.
    #[serde(rename="tableStartLocation")]
    
    pub table_start_location: Option<Location>,
}

impl client::Part for UpdateTableCellStyleRequest {}


/// Updates the TableColumnProperties of columns in a table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateTableColumnPropertiesRequest {
    /// The list of zero-based column indices whose property should be updated. If no indices are specified, all columns will be updated.
    #[serde(rename="columnIndices")]
    
    pub column_indices: Option<Vec<i32>>,
    /// The fields that should be updated. At least one field must be specified. The root `tableColumnProperties` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field. For example to update the column width, set `fields` to `"width"`.
    
    pub fields: Option<client::FieldMask>,
    /// The table column properties to update. If the value of `table_column_properties#width` is less than 5 points (5/72 inch), a 400 bad request error is returned.
    #[serde(rename="tableColumnProperties")]
    
    pub table_column_properties: Option<TableColumnProperties>,
    /// The location where the table starts in the document.
    #[serde(rename="tableStartLocation")]
    
    pub table_start_location: Option<Location>,
}

impl client::Part for UpdateTableColumnPropertiesRequest {}


/// Updates the TableRowStyle of rows in a table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateTableRowStyleRequest {
    /// The fields that should be updated. At least one field must be specified. The root `tableRowStyle` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field. For example to update the minimum row height, set `fields` to `"min_row_height"`.
    
    pub fields: Option<client::FieldMask>,
    /// The list of zero-based row indices whose style should be updated. If no indices are specified, all rows will be updated.
    #[serde(rename="rowIndices")]
    
    pub row_indices: Option<Vec<i32>>,
    /// The styles to be set on the rows.
    #[serde(rename="tableRowStyle")]
    
    pub table_row_style: Option<TableRowStyle>,
    /// The location where the table starts in the document.
    #[serde(rename="tableStartLocation")]
    
    pub table_start_location: Option<Location>,
}

impl client::Part for UpdateTableRowStyleRequest {}


/// Update the styling of text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateTextStyleRequest {
    /// The fields that should be updated. At least one field must be specified. The root `text_style` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field. For example, to update the text style to bold, set `fields` to `"bold"`. To reset a property to its default value, include its field name in the field mask but leave the field itself unset.
    
    pub fields: Option<client::FieldMask>,
    /// The range of text to style. The range may be extended to include adjacent newlines. If the range fully contains a paragraph belonging to a list, the paragraph's bullet is also updated with the matching text style. Ranges cannot be inserted inside a relative UpdateTextStyleRequest.
    
    pub range: Option<Range>,
    /// The styles to set on the text. If the value for a particular style matches that of the parent, that style will be set to inherit. Certain text style changes may cause other changes in order to to mirror the behavior of the Docs editor. See the documentation of TextStyle for more information.
    #[serde(rename="textStyle")]
    
    pub text_style: Option<TextStyle>,
}

impl client::Part for UpdateTextStyleRequest {}


/// Represents a font family and weight of text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WeightedFontFamily {
    /// The font family of the text. The font family can be any font from the Font menu in Docs or from [Google Fonts] (https://fonts.google.com/). If the font name is unrecognized, the text is rendered in `Arial`.
    #[serde(rename="fontFamily")]
    
    pub font_family: Option<String>,
    /// The weight of the font. This field can have any value that's a multiple of `100` between `100` and `900`, inclusive. This range corresponds to the numerical values described in the CSS 2.1 Specification, [section 15.6](https://www.w3.org/TR/CSS21/fonts.html#font-boldness), with non-numerical values disallowed. The default value is `400` ("normal"). The font weight makes up just one component of the rendered font weight. A combination of the `weight` and the text style's resolved `bold` value determine the rendered weight, after accounting for inheritance: * If the text is bold and the weight is less than `400`, the rendered weight is 400. * If the text is bold and the weight is greater than or equal to `400` but is less than `700`, the rendered weight is `700`. * If the weight is greater than or equal to `700`, the rendered weight is equal to the weight. * If the text is not bold, the rendered weight is equal to the weight.
    
    pub weight: Option<i32>,
}

impl client::Part for WeightedFontFamily {}


/// Provides control over how write requests are executed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WriteControl {
    /// The optional revision ID of the document the write request is applied to. If this is not the latest revision of the document, the request is not processed and returns a 400 bad request error. When a required revision ID is returned in a response, it indicates the revision ID of the document after the request was applied.
    #[serde(rename="requiredRevisionId")]
    
    pub required_revision_id: Option<String>,
    /// The optional target revision ID of the document the write request is applied to. If collaborator changes have occurred after the document was read using the API, the changes produced by this write request are applied against the collaborator changes. This results in a new revision of the document that incorporates both the collaborator changes and the changes in the request, with the Docs server resolving conflicting changes. When using target revision ID, the API client can be thought of as another collaborator of the document. The target revision ID can only be used to write to recent versions of a document. If the target revision is too far behind the latest revision, the request is not processed and returns a 400 bad request error. The request should be tried again after retrieving the latest version of the document. Usually a revision ID remains valid for use as a target revision for several minutes after it's read, but for frequently edited documents this window might be shorter.
    #[serde(rename="targetRevisionId")]
    
    pub target_revision_id: Option<String>,
}

impl client::Part for WriteControl {}



// ###################
// MethodBuilders ###
// #################

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
/// let mut hub = Docs::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `batch_update(...)`, `create(...)` and `get(...)`
/// // to build up your call.
/// let rb = hub.documents();
/// # }
/// ```
pub struct DocumentMethods<'a, S>
    where S: 'a {

    hub: &'a Docs<S>,
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





// ###################
// CallBuilders   ###
// #################

/// Applies one or more updates to the document. Each request is validated before being applied. If any request is not valid, then the entire request will fail and nothing will be applied. Some requests have replies to give you some information about how they are applied. Other requests do not need to return information; these each return an empty reply. The order of replies matches that of the requests. For example, suppose you call batchUpdate with four updates, and only the third one returns information. The response would have two empty replies, the reply to the third request, and another empty reply, in that order. Because other users may be editing the document, the document might not exactly reflect your changes: your changes may be altered with respect to collaborator changes. If there are no collaborators, the document should reflect your changes. In any case, the updates in your request are guaranteed to be applied together atomically.
///
/// A builder for the *batchUpdate* method supported by a *document* resource.
/// It is not used directly, but through a [`DocumentMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_docs1 as docs1;
/// use docs1::api::BatchUpdateDocumentRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use docs1::{Docs, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Docs::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BatchUpdateDocumentRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.documents().batch_update(req, "documentId")
///              .doit().await;
/// # }
/// ```
pub struct DocumentBatchUpdateCall<'a, S>
    where S: 'a {

    hub: &'a Docs<S>,
    _request: BatchUpdateDocumentRequest,
    _document_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for DocumentBatchUpdateCall<'a, S> {}

impl<'a, S> DocumentBatchUpdateCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, BatchUpdateDocumentResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "docs.documents.batchUpdate",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "documentId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("documentId", self._document_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/documents/{documentId}:batchUpdate";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Document.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{documentId}", "documentId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["documentId"];
            params.remove_params(&to_remove);
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
    pub fn request(mut self, new_value: BatchUpdateDocumentRequest) -> DocumentBatchUpdateCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the document to update.
    ///
    /// Sets the *document id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn document_id(mut self, new_value: &str) -> DocumentBatchUpdateCall<'a, S> {
        self._document_id = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> DocumentBatchUpdateCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DocumentBatchUpdateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Document`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> DocumentBatchUpdateCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> DocumentBatchUpdateCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> DocumentBatchUpdateCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Creates a blank document using the title given in the request. Other fields in the request, including any provided content, are ignored. Returns the created document.
///
/// A builder for the *create* method supported by a *document* resource.
/// It is not used directly, but through a [`DocumentMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_docs1 as docs1;
/// use docs1::api::Document;
/// # async fn dox() {
/// # use std::default::Default;
/// # use docs1::{Docs, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Docs::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Document::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.documents().create(req)
///              .doit().await;
/// # }
/// ```
pub struct DocumentCreateCall<'a, S>
    where S: 'a {

    hub: &'a Docs<S>,
    _request: Document,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for DocumentCreateCall<'a, S> {}

impl<'a, S> DocumentCreateCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Document)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "docs.documents.create",
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
        let mut url = self.hub._base_url.clone() + "v1/documents";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Document.as_ref().to_string());
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
    pub fn request(mut self, new_value: Document) -> DocumentCreateCall<'a, S> {
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> DocumentCreateCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DocumentCreateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Document`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> DocumentCreateCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> DocumentCreateCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> DocumentCreateCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Gets the latest version of the specified document.
///
/// A builder for the *get* method supported by a *document* resource.
/// It is not used directly, but through a [`DocumentMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_docs1 as docs1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use docs1::{Docs, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Docs::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.documents().get("documentId")
///              .suggestions_view_mode("At")
///              .doit().await;
/// # }
/// ```
pub struct DocumentGetCall<'a, S>
    where S: 'a {

    hub: &'a Docs<S>,
    _document_id: String,
    _suggestions_view_mode: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for DocumentGetCall<'a, S> {}

impl<'a, S> DocumentGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Document)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "docs.documents.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "documentId", "suggestionsViewMode"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("documentId", self._document_id);
        if let Some(value) = self._suggestions_view_mode.as_ref() {
            params.push("suggestionsViewMode", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/documents/{documentId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::DocumentReadonly.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{documentId}", "documentId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["documentId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



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
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

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


    /// The ID of the document to retrieve.
    ///
    /// Sets the *document id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn document_id(mut self, new_value: &str) -> DocumentGetCall<'a, S> {
        self._document_id = new_value.to_string();
        self
    }
    /// The suggestions view mode to apply to the document. This allows viewing the document with all suggestions inline, accepted or rejected. If one is not specified, DEFAULT_FOR_CURRENT_ACCESS is used.
    ///
    /// Sets the *suggestions view mode* query property to the given value.
    pub fn suggestions_view_mode(mut self, new_value: &str) -> DocumentGetCall<'a, S> {
        self._suggestions_view_mode = Some(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> DocumentGetCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DocumentGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::DocumentReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> DocumentGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> DocumentGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> DocumentGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


