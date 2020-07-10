// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Document* crate version *1.0.14+20200619*, where *20200619* is the exact revision of the *documentai:v1beta2* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.14*.
//! 
//! Everything else about the *Document* *v1_beta2* API can be found at the
//! [official documentation site](https://cloud.google.com/document-ai/docs/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/documentai1_beta2).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Document.html) ... 
//! 
//! * projects
//!  * [*documents batch process*](struct.ProjectDocumentBatchProcesCall.html), [*documents process*](struct.ProjectDocumentProcesCall.html), [*locations documents batch process*](struct.ProjectLocationDocumentBatchProcesCall.html), [*locations documents process*](struct.ProjectLocationDocumentProcesCall.html), [*locations operations get*](struct.ProjectLocationOperationGetCall.html) and [*operations get*](struct.ProjectOperationGetCall.html)
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
//! * **[Hub](struct.Document.html)**
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
//! let r = hub.projects().operations_get(...).doit()
//! let r = hub.projects().documents_batch_process(...).doit()
//! let r = hub.projects().locations_operations_get(...).doit()
//! let r = hub.projects().locations_documents_batch_process(...).doit()
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
//! google-documentai1_beta2 = "*"
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
//! extern crate google_documentai1_beta2 as documentai1_beta2;
//! use documentai1_beta2::GoogleCloudDocumentaiV1beta2BatchProcessDocumentsRequest;
//! use documentai1_beta2::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use documentai1_beta2::Document;
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
//! let mut hub = Document::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = GoogleCloudDocumentaiV1beta2BatchProcessDocumentsRequest::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.projects().documents_batch_process(req, "parent")
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
//! All structures provided by this library are made to be [encodable](trait.RequestValue.html) and 
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
    /// View and manage your data across Google Cloud Platform services
    CloudPlatform,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::CloudPlatform
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all Document related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_documentai1_beta2 as documentai1_beta2;
/// use documentai1_beta2::GoogleCloudDocumentaiV1beta2BatchProcessDocumentsRequest;
/// use documentai1_beta2::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use documentai1_beta2::Document;
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
/// let mut hub = Document::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudDocumentaiV1beta2BatchProcessDocumentsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().documents_batch_process(req, "parent")
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
pub struct Document<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for Document<C, A> {}

impl<'a, C, A> Document<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Document<C, A> {
        Document {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.14".to_string(),
            _base_url: "https://documentai.googleapis.com/".to_string(),
            _root_url: "https://documentai.googleapis.com/".to_string(),
        }
    }

    pub fn projects(&'a self) -> ProjectMethods<'a, C, A> {
        ProjectMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.14`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://documentai.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://documentai.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// Request to batch process documents as an asynchronous operation. The output
/// is written to Cloud Storage as JSON in the [Document] format.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [documents batch process projects](struct.ProjectDocumentBatchProcesCall.html) (request)
/// * [locations documents batch process projects](struct.ProjectLocationDocumentBatchProcesCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2BatchProcessDocumentsRequest {
    /// Required. Individual requests for each document.
    pub requests: Option<Vec<GoogleCloudDocumentaiV1beta2ProcessDocumentRequest>>,
}

impl RequestValue for GoogleCloudDocumentaiV1beta2BatchProcessDocumentsRequest {}


/// Annotation for common text style attributes. This adheres to CSS
/// conventions as much as possible.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentStyle {
    /// Text anchor indexing into the Document.text.
    #[serde(rename="textAnchor")]
    pub text_anchor: Option<GoogleCloudDocumentaiV1beta2DocumentTextAnchor>,
    /// Text style. Possible values are normal, italic, and oblique.
    /// https://www.w3schools.com/cssref/pr_font_font-style.asp
    #[serde(rename="textStyle")]
    pub text_style: Option<String>,
    /// Font size.
    #[serde(rename="fontSize")]
    pub font_size: Option<GoogleCloudDocumentaiV1beta2DocumentStyleFontSize>,
    /// Text decoration. Follows CSS standard.
    /// <text-decoration-line> <text-decoration-color> <text-decoration-style>
    /// https://www.w3schools.com/cssref/pr_text_text-decoration.asp
    #[serde(rename="textDecoration")]
    pub text_decoration: Option<String>,
    /// Text background color.
    #[serde(rename="backgroundColor")]
    pub background_color: Option<GoogleTypeColor>,
    /// Font weight. Possible values are normal, bold, bolder, and lighter.
    /// https://www.w3schools.com/cssref/pr_font_weight.asp
    #[serde(rename="fontWeight")]
    pub font_weight: Option<String>,
    /// Text color.
    pub color: Option<GoogleTypeColor>,
}

impl Part for GoogleCloudDocumentaiV1beta2DocumentStyle {}


/// A text segment in the Document.text. The indices may be out of bounds
/// which indicate that the text extends into another document shard for
/// large sharded documents. See ShardInfo.text_offset
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentTextAnchorTextSegment {
    /// TextSegment half open end UTF-8 char index in the
    /// Document.text.
    #[serde(rename="endIndex")]
    pub end_index: Option<String>,
    /// TextSegment start UTF-8 char index in the Document.text.
    #[serde(rename="startIndex")]
    pub start_index: Option<String>,
}

impl Part for GoogleCloudDocumentaiV1beta2DocumentTextAnchorTextSegment {}


/// A block has a set of lines (collected into paragraphs) that have a
/// common line-spacing and orientation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentPageBlock {
    /// Layout for Block.
    pub layout: Option<GoogleCloudDocumentaiV1beta2DocumentPageLayout>,
    /// A list of detected languages together with confidence.
    #[serde(rename="detectedLanguages")]
    pub detected_languages: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguage>>,
}

impl Part for GoogleCloudDocumentaiV1beta2DocumentPageBlock {}


/// Label attaches schema information and/or other metadata to segments within
/// a Document. Multiple Labels on a single field can denote either
/// different labels, different instances of the same label created at
/// different times, or some combination of both.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentLabel {
    /// Label is generated AutoML model. This field stores the full resource
    /// name of the AutoML model.
    /// 
    /// Format:
    /// `projects/{project-id}/locations/{location-id}/models/{model-id}`
    #[serde(rename="automlModel")]
    pub automl_model: Option<String>,
    /// Confidence score between 0 and 1 for label assignment.
    pub confidence: Option<f32>,
    /// Name of the label.
    /// 
    /// When the label is generated from AutoML Text Classification model, this
    /// field represents the name of the category.
    pub name: Option<String>,
}

impl Part for GoogleCloudDocumentaiV1beta2DocumentLabel {}


/// For a large document, sharding may be performed to produce several
/// document shards. Each document shard contains this field to detail which
/// shard it is.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentShardInfo {
    /// The 0-based index of this shard.
    #[serde(rename="shardIndex")]
    pub shard_index: Option<String>,
    /// The index of the first character in Document.text in the overall
    /// document global text.
    #[serde(rename="textOffset")]
    pub text_offset: Option<String>,
    /// Total number of shards.
    #[serde(rename="shardCount")]
    pub shard_count: Option<String>,
}

impl Part for GoogleCloudDocumentaiV1beta2DocumentShardInfo {}


/// Detected language for a structural component.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguage {
    /// The BCP-47 language code, such as "en-US" or "sr-Latn". For more
    /// information, see
    /// http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.
    #[serde(rename="languageCode")]
    pub language_code: Option<String>,
    /// Confidence of detected language. Range [0, 1].
    pub confidence: Option<f32>,
}

impl Part for GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguage {}


/// A hint for a table bounding box on the page for table parsing.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2TableBoundHint {
    /// Bounding box hint for a table on this page. The coordinates must be
    /// normalized to [0,1] and the bounding box must be an axis-aligned rectangle.
    #[serde(rename="boundingBox")]
    pub bounding_box: Option<GoogleCloudDocumentaiV1beta2BoundingPoly>,
    /// Optional. Page number for multi-paged inputs this hint applies to. If not
    /// provided, this hint will apply to all pages by default. This value is
    /// 1-based.
    #[serde(rename="pageNumber")]
    pub page_number: Option<i32>,
}

impl Part for GoogleCloudDocumentaiV1beta2TableBoundHint {}


/// Detected break at the end of a Token.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentPageTokenDetectedBreak {
    /// Detected break type.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl Part for GoogleCloudDocumentaiV1beta2DocumentPageTokenDetectedBreak {}


/// A collection of lines that a human would perceive as a paragraph.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentPageParagraph {
    /// Layout for Paragraph.
    pub layout: Option<GoogleCloudDocumentaiV1beta2DocumentPageLayout>,
    /// A list of detected languages together with confidence.
    #[serde(rename="detectedLanguages")]
    pub detected_languages: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguage>>,
}

impl Part for GoogleCloudDocumentaiV1beta2DocumentPageParagraph {}


/// A row of table cells.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentPageTableTableRow {
    /// Cells that make up this row.
    pub cells: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageTableTableCell>>,
}

impl Part for GoogleCloudDocumentaiV1beta2DocumentPageTableTableRow {}


/// A phrase in the text that is a known entity type, such as a person, an
/// organization, or location.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentEntity {
    /// Text value in the document e.g. `1600 Amphitheatre Pkwy`.
    #[serde(rename="mentionText")]
    pub mention_text: Option<String>,
    /// Provenance of the entity.
    /// Text anchor indexing into the Document.text.
    #[serde(rename="textAnchor")]
    pub text_anchor: Option<GoogleCloudDocumentaiV1beta2DocumentTextAnchor>,
    /// Optional. Confidence of detected Schema entity. Range [0, 1].
    pub confidence: Option<f32>,
    /// Deprecated.  Use `id` field instead.
    #[serde(rename="mentionId")]
    pub mention_id: Option<String>,
    /// Entity type from a schema e.g. `Address`.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl Part for GoogleCloudDocumentaiV1beta2DocumentEntity {}


/// Parameters to control form extraction behavior.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2FormExtractionParams {
    /// Model version of the form extraction system. Default is
    /// "builtin/stable". Specify "builtin/latest" for the latest model.
    /// For custom form models, specify: “custom/{model_name}". Model name
    /// format is "bucket_name/path/to/modeldir" corresponding to
    /// "gs://bucket_name/path/to/modeldir" where annotated examples are stored.
    #[serde(rename="modelVersion")]
    pub model_version: Option<String>,
    /// Reserved for future use.
    #[serde(rename="keyValuePairHints")]
    pub key_value_pair_hints: Option<Vec<GoogleCloudDocumentaiV1beta2KeyValuePairHint>>,
    /// Whether to enable form extraction.
    pub enabled: Option<bool>,
}

impl Part for GoogleCloudDocumentaiV1beta2FormExtractionParams {}


/// Parameters to control Optical Character Recognition (OCR) behavior.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2OcrParams {
    /// List of languages to use for OCR. In most cases, an empty value
    /// yields the best results since it enables automatic language detection. For
    /// languages based on the Latin alphabet, setting `language_hints` is not
    /// needed. In rare cases, when the language of the text in the image is known,
    /// setting a hint will help get better results (although it will be a
    /// significant hindrance if the hint is wrong). Document processing returns an
    /// error if one or more of the specified languages is not one of the
    /// supported languages.
    #[serde(rename="languageHints")]
    pub language_hints: Option<Vec<String>>,
}

impl Part for GoogleCloudDocumentaiV1beta2OcrParams {}


/// The Google Cloud Storage location where the output file will be written to.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2GcsDestination {
    /// no description provided
    pub uri: Option<String>,
}

impl Part for GoogleCloudDocumentaiV1beta2GcsDestination {}


/// A detected token.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentPageToken {
    /// Detected break at the end of a Token.
    #[serde(rename="detectedBreak")]
    pub detected_break: Option<GoogleCloudDocumentaiV1beta2DocumentPageTokenDetectedBreak>,
    /// Layout for Token.
    pub layout: Option<GoogleCloudDocumentaiV1beta2DocumentPageLayout>,
    /// A list of detected languages together with confidence.
    #[serde(rename="detectedLanguages")]
    pub detected_languages: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguage>>,
}

impl Part for GoogleCloudDocumentaiV1beta2DocumentPageToken {}


/// Reserved for future use.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2KeyValuePairHint {
    /// Type of the value. This is case-insensitive, and could be one of:
    /// ADDRESS, LOCATION, ORGANIZATION, PERSON, PHONE_NUMBER,
    /// ID, NUMBER, EMAIL, PRICE, TERMS, DATE, NAME. Types not in this list will
    /// be ignored.
    #[serde(rename="valueTypes")]
    pub value_types: Option<Vec<String>>,
    /// The key text for the hint.
    pub key: Option<String>,
}

impl Part for GoogleCloudDocumentaiV1beta2KeyValuePairHint {}


/// Visual element describing a layout unit on a page.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentPageLayout {
    /// Text anchor indexing into the Document.text.
    #[serde(rename="textAnchor")]
    pub text_anchor: Option<GoogleCloudDocumentaiV1beta2DocumentTextAnchor>,
    /// Confidence of the current Layout within context of the object this
    /// layout is for. e.g. confidence can be for a single token, a table,
    /// a visual element, etc. depending on context. Range [0, 1].
    pub confidence: Option<f32>,
    /// Detected orientation for the Layout.
    pub orientation: Option<String>,
    /// The bounding polygon for the Layout.
    #[serde(rename="boundingPoly")]
    pub bounding_poly: Option<GoogleCloudDocumentaiV1beta2BoundingPoly>,
}

impl Part for GoogleCloudDocumentaiV1beta2DocumentPageLayout {}


/// A form field detected on the page.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentPageFormField {
    /// Layout for the FormField name. e.g. `Address`, `Email`,
    /// `Grand total`, `Phone number`, etc.
    #[serde(rename="fieldName")]
    pub field_name: Option<GoogleCloudDocumentaiV1beta2DocumentPageLayout>,
    /// If the value is non-textual, this field represents the type. Current
    /// valid values are:
    /// - blank (this indicates the field_value is normal text)
    /// - "unfilled_checkbox"
    /// - "filled_checkbox"
    #[serde(rename="valueType")]
    pub value_type: Option<String>,
    /// A list of detected languages for value together with confidence.
    #[serde(rename="valueDetectedLanguages")]
    pub value_detected_languages: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguage>>,
    /// Layout for the FormField value.
    #[serde(rename="fieldValue")]
    pub field_value: Option<GoogleCloudDocumentaiV1beta2DocumentPageLayout>,
    /// A list of detected languages for name together with confidence.
    #[serde(rename="nameDetectedLanguages")]
    pub name_detected_languages: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguage>>,
}

impl Part for GoogleCloudDocumentaiV1beta2DocumentPageFormField {}


/// Represents a color in the RGBA color space. This representation is designed
/// for simplicity of conversion to/from color representations in various
/// languages over compactness; for example, the fields of this representation
/// can be trivially provided to the constructor of "java.awt.Color" in Java; it
/// can also be trivially provided to UIColor's "+colorWithRed:green:blue:alpha"
/// method in iOS; and, with just a little work, it can be easily formatted into
/// a CSS "rgba()" string in JavaScript, as well.
/// 
/// Note: this proto does not carry information about the absolute color space
/// that should be used to interpret the RGB value (e.g. sRGB, Adobe RGB,
/// DCI-P3, BT.2020, etc.). By default, applications SHOULD assume the sRGB color
/// space.
/// 
/// Note: when color equality needs to be decided, implementations, unless
/// documented otherwise, will treat two colors to be equal if all their red,
/// green, blue and alpha values each differ by at most 1e-5.
/// 
/// Example (Java):
/// 
/// ````text
///  import com.google.type.Color;
/// 
///  // ...
///  public static java.awt.Color fromProto(Color protocolor) {
///    float alpha = protocolor.hasAlpha()
///        ? protocolor.getAlpha().getValue()
///        : 1.0;
/// 
///    return new java.awt.Color(
///        protocolor.getRed(),
///        protocolor.getGreen(),
///        protocolor.getBlue(),
///        alpha);
///  }
/// 
///  public static Color toProto(java.awt.Color color) {
///    float red = (float) color.getRed();
///    float green = (float) color.getGreen();
///    float blue = (float) color.getBlue();
///    float denominator = 255.0;
///    Color.Builder resultBuilder =
///        Color
///            .newBuilder()
///            .setRed(red / denominator)
///            .setGreen(green / denominator)
///            .setBlue(blue / denominator);
///    int alpha = color.getAlpha();
///    if (alpha != 255) {
///      result.setAlpha(
///          FloatValue
///              .newBuilder()
///              .setValue(((float) alpha) / denominator)
///              .build());
///    }
///    return resultBuilder.build();
///  }
///  // ...
/// ````
/// 
/// Example (iOS / Obj-C):
/// 
/// ````text
///  // ...
///  static UIColor* fromProto(Color* protocolor) {
///     float red = [protocolor red];
///     float green = [protocolor green];
///     float blue = [protocolor blue];
///     FloatValue* alpha_wrapper = [protocolor alpha];
///     float alpha = 1.0;
///     if (alpha_wrapper != nil) {
///       alpha = [alpha_wrapper value];
///     }
///     return [UIColor colorWithRed:red green:green blue:blue alpha:alpha];
///  }
/// 
///  static Color* toProto(UIColor* color) {
///      CGFloat red, green, blue, alpha;
///      if (![color getRed:&red green:&green blue:&blue alpha:&alpha]) {
///        return nil;
///      }
///      Color* result = [[Color alloc] init];
///      [result setRed:red];
///      [result setGreen:green];
///      [result setBlue:blue];
///      if (alpha <= 0.9999) {
///        [result setAlpha:floatWrapperWithValue(alpha)];
///      }
///      [result autorelease];
///      return result;
/// }
/// // ...
/// ````
/// 
/// Example (JavaScript):
/// 
/// ````text
/// // ...
/// 
/// var protoToCssColor = function(rgb_color) {
///    var redFrac = rgb_color.red || 0.0;
///    var greenFrac = rgb_color.green || 0.0;
///    var blueFrac = rgb_color.blue || 0.0;
///    var red = Math.floor(redFrac * 255);
///    var green = Math.floor(greenFrac * 255);
///    var blue = Math.floor(blueFrac * 255);
/// 
///    if (!('alpha' in rgb_color)) {
///       return rgbToCssColor_(red, green, blue);
///    }
/// 
///    var alphaFrac = rgb_color.alpha.value || 0.0;
///    var rgbParams = [red, green, blue].join(',');
///    return ['rgba(', rgbParams, ',', alphaFrac, ')'].join('');
/// };
/// 
/// var rgbToCssColor_ = function(red, green, blue) {
///   var rgbNumber = new Number((red << 16) | (green << 8) | blue);
///   var hexString = rgbNumber.toString(16);
///   var missingZeros = 6 - hexString.length;
///   var resultBuilder = ['#'];
///   for (var i = 0; i < missingZeros; i++) {
///      resultBuilder.push('0');
///   }
///   resultBuilder.push(hexString);
///   return resultBuilder.join('');
/// };
/// 
/// // ...
/// ````
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeColor {
    /// The amount of blue in the color as a value in the interval [0, 1].
    pub blue: Option<f32>,
    /// The fraction of this color that should be applied to the pixel. That is,
    /// the final pixel color is defined by the equation:
    /// 
    ///   pixel color = alpha * (this color) + (1.0 - alpha) * (background color)
    /// 
    /// This means that a value of 1.0 corresponds to a solid color, whereas
    /// a value of 0.0 corresponds to a completely transparent color. This
    /// uses a wrapper message rather than a simple float scalar so that it is
    /// possible to distinguish between a default value and the value being unset.
    /// If omitted, this color object is to be rendered as a solid color
    /// (as if the alpha value had been explicitly given with a value of 1.0).
    pub alpha: Option<f32>,
    /// The amount of green in the color as a value in the interval [0, 1].
    pub green: Option<f32>,
    /// The amount of red in the color as a value in the interval [0, 1].
    pub red: Option<f32>,
}

impl Part for GoogleTypeColor {}


/// A vertex represents a 2D point in the image.
/// NOTE: the normalized vertex coordinates are relative to the original image
/// and range from 0 to 1.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2NormalizedVertex {
    /// Y coordinate.
    pub y: Option<f32>,
    /// X coordinate.
    pub x: Option<f32>,
}

impl Part for GoogleCloudDocumentaiV1beta2NormalizedVertex {}


/// A collection of tokens that a human would perceive as a line.
/// Does not cross column boundaries, can be horizontal, vertical, etc.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentPageLine {
    /// Layout for Line.
    pub layout: Option<GoogleCloudDocumentaiV1beta2DocumentPageLayout>,
    /// A list of detected languages together with confidence.
    #[serde(rename="detectedLanguages")]
    pub detected_languages: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguage>>,
}

impl Part for GoogleCloudDocumentaiV1beta2DocumentPageLine {}


/// Request to process one document.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [documents process projects](struct.ProjectDocumentProcesCall.html) (request)
/// * [locations documents process projects](struct.ProjectLocationDocumentProcesCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2ProcessDocumentRequest {
    /// Target project and location to make a call.
    /// 
    /// Format: `projects/{project-id}/locations/{location-id}`.
    /// 
    /// If no location is specified, a region will be chosen automatically.
    /// This field is only populated when used in ProcessDocument method.
    pub parent: Option<String>,
    /// The desired output location. This field is only needed in
    /// BatchProcessDocumentsRequest.
    #[serde(rename="outputConfig")]
    pub output_config: Option<GoogleCloudDocumentaiV1beta2OutputConfig>,
    /// Controls entity extraction behavior. If not specified, the system will
    /// decide reasonable defaults.
    #[serde(rename="entityExtractionParams")]
    pub entity_extraction_params: Option<GoogleCloudDocumentaiV1beta2EntityExtractionParams>,
    /// Controls table extraction behavior. If not specified, the system will
    /// decide reasonable defaults.
    #[serde(rename="tableExtractionParams")]
    pub table_extraction_params: Option<GoogleCloudDocumentaiV1beta2TableExtractionParams>,
    /// Required. Information about the input file.
    #[serde(rename="inputConfig")]
    pub input_config: Option<GoogleCloudDocumentaiV1beta2InputConfig>,
    /// Controls OCR behavior. If not specified, the system will decide reasonable
    /// defaults.
    #[serde(rename="ocrParams")]
    pub ocr_params: Option<GoogleCloudDocumentaiV1beta2OcrParams>,
    /// Specifies a known document type for deeper structure detection. Valid
    /// values are currently "general" and "invoice". If not provided, "general"\
    /// is used as default. If any other value is given, the request is rejected.
    #[serde(rename="documentType")]
    pub document_type: Option<String>,
    /// Controls AutoML model prediction behavior. AutoMlParams cannot be used
    /// together with other Params.
    #[serde(rename="automlParams")]
    pub automl_params: Option<GoogleCloudDocumentaiV1beta2AutoMlParams>,
    /// Controls form extraction behavior. If not specified, the system will
    /// decide reasonable defaults.
    #[serde(rename="formExtractionParams")]
    pub form_extraction_params: Option<GoogleCloudDocumentaiV1beta2FormExtractionParams>,
}

impl RequestValue for GoogleCloudDocumentaiV1beta2ProcessDocumentRequest {}


/// The desired output location and metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2OutputConfig {
    /// The Google Cloud Storage location to write the output to.
    #[serde(rename="gcsDestination")]
    pub gcs_destination: Option<GoogleCloudDocumentaiV1beta2GcsDestination>,
    /// The max number of pages to include into each output Document shard JSON on
    /// Google Cloud Storage.
    /// 
    /// The valid range is [1, 100]. If not specified, the default value is 20.
    /// 
    /// For example, for one pdf file with 100 pages, 100 parsed pages will be
    /// produced. If `pages_per_shard` = 20, then 5 Document shard JSON files each
    /// containing 20 parsed pages will be written under the prefix
    /// OutputConfig.gcs_destination.uri and suffix pages-x-to-y.json where
    /// x and y are 1-indexed page numbers.
    /// 
    /// Example GCS outputs with 157 pages and pages_per_shard = 50:
    /// 
    /// <prefix>pages-001-to-050.json
    /// <prefix>pages-051-to-100.json
    /// <prefix>pages-101-to-150.json
    /// <prefix>pages-151-to-157.json
    #[serde(rename="pagesPerShard")]
    pub pages_per_shard: Option<i32>,
}

impl Part for GoogleCloudDocumentaiV1beta2OutputConfig {}


/// A page in a Document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentPage {
    /// A list of visually detected tables on the page.
    pub tables: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageTable>>,
    /// Layout for the page.
    pub layout: Option<GoogleCloudDocumentaiV1beta2DocumentPageLayout>,
    /// A list of detected non-text visual elements e.g. checkbox,
    /// signature etc. on the page.
    #[serde(rename="visualElements")]
    pub visual_elements: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageVisualElement>>,
    /// A list of visually detected form fields on the page.
    #[serde(rename="formFields")]
    pub form_fields: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageFormField>>,
    /// A list of visually detected text lines on the page.
    /// A collection of tokens that a human would perceive as a line.
    pub lines: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageLine>>,
    /// A list of detected languages together with confidence.
    #[serde(rename="detectedLanguages")]
    pub detected_languages: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguage>>,
    /// A list of visually detected tokens on the page.
    pub tokens: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageToken>>,
    /// 1-based index for current Page in a parent Document.
    /// Useful when a page is taken out of a Document for individual
    /// processing.
    #[serde(rename="pageNumber")]
    pub page_number: Option<i32>,
    /// A list of visually detected text paragraphs on the page.
    /// A collection of lines that a human would perceive as a paragraph.
    pub paragraphs: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageParagraph>>,
    /// A list of visually detected text blocks on the page.
    /// A block has a set of lines (collected into paragraphs) that have a common
    /// line-spacing and orientation.
    pub blocks: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageBlock>>,
    /// Physical dimension of the page.
    pub dimension: Option<GoogleCloudDocumentaiV1beta2DocumentPageDimension>,
}

impl Part for GoogleCloudDocumentaiV1beta2DocumentPage {}


/// Relationship between Entities.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentEntityRelation {
    /// Relationship description.
    pub relation: Option<String>,
    /// Object entity id.
    #[serde(rename="objectId")]
    pub object_id: Option<String>,
    /// Subject entity id.
    #[serde(rename="subjectId")]
    pub subject_id: Option<String>,
}

impl Part for GoogleCloudDocumentaiV1beta2DocumentEntityRelation {}


/// A bounding polygon for the detected image annotation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2BoundingPoly {
    /// The bounding polygon normalized vertices.
    #[serde(rename="normalizedVertices")]
    pub normalized_vertices: Option<Vec<GoogleCloudDocumentaiV1beta2NormalizedVertex>>,
    /// The bounding polygon vertices.
    pub vertices: Option<Vec<GoogleCloudDocumentaiV1beta2Vertex>>,
}

impl Part for GoogleCloudDocumentaiV1beta2BoundingPoly {}


/// The desired input location and metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2InputConfig {
    /// Required. Mimetype of the input. Current supported mimetypes are application/pdf,
    /// image/tiff, and image/gif.
    /// In addition, application/json type is supported for requests with
    /// ProcessDocumentRequest.automl_params field set. The JSON file needs to
    /// be in Document format.
    #[serde(rename="mimeType")]
    pub mime_type: Option<String>,
    /// The Google Cloud Storage location to read the input from. This must be a
    /// single file.
    #[serde(rename="gcsSource")]
    pub gcs_source: Option<GoogleCloudDocumentaiV1beta2GcsSource>,
    /// Content in bytes, represented as a stream of bytes.
    /// Note: As with all `bytes` fields, proto buffer messages use a pure binary
    /// representation, whereas JSON representations use base64.
    /// 
    /// This field only works for synchronous ProcessDocument method.
    pub contents: Option<String>,
}

impl Part for GoogleCloudDocumentaiV1beta2InputConfig {}


/// A translation of the text segment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentTranslation {
    /// Provenance of the translation.
    /// Text anchor indexing into the Document.text.
    #[serde(rename="textAnchor")]
    pub text_anchor: Option<GoogleCloudDocumentaiV1beta2DocumentTextAnchor>,
    /// The BCP-47 language code, such as "en-US" or "sr-Latn". For more
    /// information, see
    /// http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.
    #[serde(rename="languageCode")]
    pub language_code: Option<String>,
    /// Text translated into the target language.
    #[serde(rename="translatedText")]
    pub translated_text: Option<String>,
}

impl Part for GoogleCloudDocumentaiV1beta2DocumentTranslation {}


/// Parameters to control table extraction behavior.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2TableExtractionParams {
    /// Optional. Reserved for future use.
    #[serde(rename="headerHints")]
    pub header_hints: Option<Vec<String>>,
    /// Model version of the table extraction system. Default is "builtin/stable".
    /// Specify "builtin/latest" for the latest model.
    #[serde(rename="modelVersion")]
    pub model_version: Option<String>,
    /// Whether to enable table extraction.
    pub enabled: Option<bool>,
    /// Optional. Table bounding box hints that can be provided to complex cases
    /// which our algorithm cannot locate the table(s) in.
    #[serde(rename="tableBoundHints")]
    pub table_bound_hints: Option<Vec<GoogleCloudDocumentaiV1beta2TableBoundHint>>,
}

impl Part for GoogleCloudDocumentaiV1beta2TableExtractionParams {}


/// Document represents the canonical document resource in Document Understanding
/// AI.
/// It is an interchange format that provides insights into documents and allows
/// for collaboration between users and Document Understanding AI to iterate and
/// optimize for quality.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [documents process projects](struct.ProjectDocumentProcesCall.html) (response)
/// * [locations documents process projects](struct.ProjectLocationDocumentProcesCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2Document {
    /// An IANA published MIME type (also referred to as media type). For more
    /// information, see
    /// https://www.iana.org/assignments/media-types/media-types.xhtml.
    #[serde(rename="mimeType")]
    pub mime_type: Option<String>,
    /// Styles for the Document.text.
    #[serde(rename="textStyles")]
    pub text_styles: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentStyle>>,
    /// UTF-8 encoded text in reading order from the document.
    pub text: Option<String>,
    /// Labels for this document.
    pub labels: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentLabel>>,
    /// A list of translations on Document.text. For document shards,
    /// translations in this list may cross shard boundaries.
    pub translations: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentTranslation>>,
    /// Currently supports Google Cloud Storage URI of the form
    ///    `gs://bucket_name/object_name`. Object versioning is not supported.
    ///    See [Google Cloud Storage Request
    ///    URIs](https://cloud.google.com/storage/docs/reference-uris) for more
    ///    info.
    pub uri: Option<String>,
    /// Relationship among Document.entities.
    #[serde(rename="entityRelations")]
    pub entity_relations: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentEntityRelation>>,
    /// Inline document content, represented as a stream of bytes.
    /// Note: As with all `bytes` fields, protobuffers use a pure binary
    /// representation, whereas JSON representations use base64.
    pub content: Option<String>,
    /// A list of entities detected on Document.text. For document shards,
    /// entities in this list may cross shard boundaries.
    pub entities: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentEntity>>,
    /// Information about the sharding if this document is sharded part of a larger
    /// document. If the document is not sharded, this message is not specified.
    #[serde(rename="shardInfo")]
    pub shard_info: Option<GoogleCloudDocumentaiV1beta2DocumentShardInfo>,
    /// Any error that occurred while processing this document.
    pub error: Option<GoogleRpcStatus>,
    /// Visual page layout for the Document.
    pub pages: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPage>>,
}

impl ResponseResult for GoogleCloudDocumentaiV1beta2Document {}


/// A table representation similar to HTML table structure.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentPageTable {
    /// Header rows of the table.
    #[serde(rename="headerRows")]
    pub header_rows: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageTableTableRow>>,
    /// Body rows of the table.
    #[serde(rename="bodyRows")]
    pub body_rows: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageTableTableRow>>,
    /// Layout for Table.
    pub layout: Option<GoogleCloudDocumentaiV1beta2DocumentPageLayout>,
    /// A list of detected languages together with confidence.
    #[serde(rename="detectedLanguages")]
    pub detected_languages: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguage>>,
}

impl Part for GoogleCloudDocumentaiV1beta2DocumentPageTable {}


/// The `Status` type defines a logical error model that is suitable for
/// different programming environments, including REST APIs and RPC APIs. It is
/// used by [gRPC](https://github.com/grpc). Each `Status` message contains
/// three pieces of data: error code, error message, and error details.
/// 
/// You can find out more about this error model and how to work with it in the
/// [API Design Guide](https://cloud.google.com/apis/design/errors).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleRpcStatus {
    /// A developer-facing error message, which should be in English. Any
    /// user-facing error message should be localized and sent in the
    /// google.rpc.Status.details field, or localized by the client.
    pub message: Option<String>,
    /// The status code, which should be an enum value of google.rpc.Code.
    pub code: Option<i32>,
    /// A list of messages that carry the error details.  There is a common set of
    /// message types for APIs to use.
    pub details: Option<Vec<HashMap<String, String>>>,
}

impl Part for GoogleRpcStatus {}


/// Text reference indexing into the Document.text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentTextAnchor {
    /// The text segments from the Document.text.
    #[serde(rename="textSegments")]
    pub text_segments: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentTextAnchorTextSegment>>,
}

impl Part for GoogleCloudDocumentaiV1beta2DocumentTextAnchor {}


/// A vertex represents a 2D point in the image.
/// NOTE: the vertex coordinates are in the same scale as the original image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2Vertex {
    /// Y coordinate.
    pub y: Option<i32>,
    /// X coordinate.
    pub x: Option<i32>,
}

impl Part for GoogleCloudDocumentaiV1beta2Vertex {}


/// Font size with unit.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentStyleFontSize {
    /// Unit for the font size. Follows CSS naming (in, px, pt, etc.).
    pub unit: Option<String>,
    /// Font size for the text.
    pub size: Option<f32>,
}

impl Part for GoogleCloudDocumentaiV1beta2DocumentStyleFontSize {}


/// Parameters to control entity extraction behavior.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2EntityExtractionParams {
    /// Model version of the entity extraction. Default is
    /// "builtin/stable". Specify "builtin/latest" for the latest model.
    #[serde(rename="modelVersion")]
    pub model_version: Option<String>,
    /// Whether to enable entity extraction.
    pub enabled: Option<bool>,
}

impl Part for GoogleCloudDocumentaiV1beta2EntityExtractionParams {}


/// A cell representation inside the table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentPageTableTableCell {
    /// How many columns this cell spans.
    #[serde(rename="colSpan")]
    pub col_span: Option<i32>,
    /// How many rows this cell spans.
    #[serde(rename="rowSpan")]
    pub row_span: Option<i32>,
    /// Layout for TableCell.
    pub layout: Option<GoogleCloudDocumentaiV1beta2DocumentPageLayout>,
    /// A list of detected languages together with confidence.
    #[serde(rename="detectedLanguages")]
    pub detected_languages: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguage>>,
}

impl Part for GoogleCloudDocumentaiV1beta2DocumentPageTableTableCell {}


/// This resource represents a long-running operation that is the result of a
/// network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [operations get projects](struct.ProjectOperationGetCall.html) (response)
/// * [documents batch process projects](struct.ProjectDocumentBatchProcesCall.html) (response)
/// * [locations operations get projects](struct.ProjectLocationOperationGetCall.html) (response)
/// * [locations documents batch process projects](struct.ProjectLocationDocumentBatchProcesCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleLongrunningOperation {
    /// The error result of the operation in case of failure or cancellation.
    pub error: Option<GoogleRpcStatus>,
    /// If the value is `false`, it means the operation is still in progress.
    /// If `true`, the operation is completed, and either `error` or `response` is
    /// available.
    pub done: Option<bool>,
    /// The normal response of the operation in case of success.  If the original
    /// method returns no data on success, such as `Delete`, the response is
    /// `google.protobuf.Empty`.  If the original method is standard
    /// `Get`/`Create`/`Update`, the response should be the resource.  For other
    /// methods, the response should have the type `XxxResponse`, where `Xxx`
    /// is the original method name.  For example, if the original method name
    /// is `TakeSnapshot()`, the inferred response type is
    /// `TakeSnapshotResponse`.
    pub response: Option<HashMap<String, String>>,
    /// The server-assigned name, which is only unique within the same service that
    /// originally returns it. If you use the default HTTP mapping, the
    /// `name` should be a resource name ending with `operations/{unique_id}`.
    pub name: Option<String>,
    /// Service-specific metadata associated with the operation.  It typically
    /// contains progress information and common metadata such as create time.
    /// Some services might not provide such metadata.  Any method that returns a
    /// long-running operation should document the metadata type, if any.
    pub metadata: Option<HashMap<String, String>>,
}

impl ResponseResult for GoogleLongrunningOperation {}


/// Detected non-text visual elements e.g. checkbox, signature etc. on the
/// page.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentPageVisualElement {
    /// Layout for VisualElement.
    pub layout: Option<GoogleCloudDocumentaiV1beta2DocumentPageLayout>,
    /// Type of the VisualElement.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// A list of detected languages together with confidence.
    #[serde(rename="detectedLanguages")]
    pub detected_languages: Option<Vec<GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguage>>,
}

impl Part for GoogleCloudDocumentaiV1beta2DocumentPageVisualElement {}


/// Dimension for the page.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2DocumentPageDimension {
    /// Page width.
    pub width: Option<f32>,
    /// Dimension unit.
    pub unit: Option<String>,
    /// Page height.
    pub height: Option<f32>,
}

impl Part for GoogleCloudDocumentaiV1beta2DocumentPageDimension {}


/// The Google Cloud Storage location where the input file will be read from.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2GcsSource {
    /// no description provided
    pub uri: Option<String>,
}

impl Part for GoogleCloudDocumentaiV1beta2GcsSource {}


/// Parameters to control AutoML model prediction behavior.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDocumentaiV1beta2AutoMlParams {
    /// Resource name of the AutoML model.
    /// 
    /// Format: `projects/{project-id}/locations/{location-id}/models/{model-id}`.
    pub model: Option<String>,
}

impl Part for GoogleCloudDocumentaiV1beta2AutoMlParams {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the `Document` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_documentai1_beta2 as documentai1_beta2;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use documentai1_beta2::Document;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Document::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `documents_batch_process(...)`, `documents_process(...)`, `locations_documents_batch_process(...)`, `locations_documents_process(...)`, `locations_operations_get(...)` and `operations_get(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Document<C, A>,
}

impl<'a, C, A> MethodsBuilder for ProjectMethods<'a, C, A> {}

impl<'a, C, A> ProjectMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// LRO endpoint to batch process many documents. The output is written
    /// to Cloud Storage as JSON in the [Document] format.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Target project and location to make a call.
    ///              Format: `projects/{project-id}/locations/{location-id}`.
    ///              If no location is specified, a region will be chosen automatically.
    pub fn documents_batch_process(&self, request: GoogleCloudDocumentaiV1beta2BatchProcessDocumentsRequest, parent: &str) -> ProjectDocumentBatchProcesCall<'a, C, A> {
        ProjectDocumentBatchProcesCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Processes a single document.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Target project and location to make a call.
    ///              Format: `projects/{project-id}/locations/{location-id}`.
    ///              If no location is specified, a region will be chosen automatically.
    ///              This field is only populated when used in ProcessDocument method.
    pub fn documents_process(&self, request: GoogleCloudDocumentaiV1beta2ProcessDocumentRequest, parent: &str) -> ProjectDocumentProcesCall<'a, C, A> {
        ProjectDocumentProcesCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the latest state of a long-running operation.  Clients can use this
    /// method to poll the operation result at intervals as recommended by the API
    /// service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn locations_operations_get(&self, name: &str) -> ProjectLocationOperationGetCall<'a, C, A> {
        ProjectLocationOperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the latest state of a long-running operation.  Clients can use this
    /// method to poll the operation result at intervals as recommended by the API
    /// service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn operations_get(&self, name: &str) -> ProjectOperationGetCall<'a, C, A> {
        ProjectOperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Processes a single document.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Target project and location to make a call.
    ///              Format: `projects/{project-id}/locations/{location-id}`.
    ///              If no location is specified, a region will be chosen automatically.
    ///              This field is only populated when used in ProcessDocument method.
    pub fn locations_documents_process(&self, request: GoogleCloudDocumentaiV1beta2ProcessDocumentRequest, parent: &str) -> ProjectLocationDocumentProcesCall<'a, C, A> {
        ProjectLocationDocumentProcesCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// LRO endpoint to batch process many documents. The output is written
    /// to Cloud Storage as JSON in the [Document] format.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Target project and location to make a call.
    ///              Format: `projects/{project-id}/locations/{location-id}`.
    ///              If no location is specified, a region will be chosen automatically.
    pub fn locations_documents_batch_process(&self, request: GoogleCloudDocumentaiV1beta2BatchProcessDocumentsRequest, parent: &str) -> ProjectLocationDocumentBatchProcesCall<'a, C, A> {
        ProjectLocationDocumentBatchProcesCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// LRO endpoint to batch process many documents. The output is written
/// to Cloud Storage as JSON in the [Document] format.
///
/// A builder for the *documents.batchProcess* method supported by a *project* resource.
/// It is not used directly, but through a `ProjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_documentai1_beta2 as documentai1_beta2;
/// use documentai1_beta2::GoogleCloudDocumentaiV1beta2BatchProcessDocumentsRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use documentai1_beta2::Document;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Document::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudDocumentaiV1beta2BatchProcessDocumentsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().documents_batch_process(req, "parent")
///              .doit();
/// # }
/// ```
pub struct ProjectDocumentBatchProcesCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Document<C, A>,
    _request: GoogleCloudDocumentaiV1beta2BatchProcessDocumentsRequest,
    _parent: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectDocumentBatchProcesCall<'a, C, A> {}

impl<'a, C, A> ProjectDocumentBatchProcesCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GoogleLongrunningOperation)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "documentai.projects.documents.batchProcess",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("parent", self._parent.to_string()));
        for &field in ["alt", "parent"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta2/{+parent}/documents:batchProcess";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+parent}", "parent")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["parent"].iter() {
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

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
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
    pub fn request(mut self, new_value: GoogleCloudDocumentaiV1beta2BatchProcessDocumentsRequest) -> ProjectDocumentBatchProcesCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Target project and location to make a call.
    /// 
    /// Format: `projects/{project-id}/locations/{location-id}`.
    /// 
    /// If no location is specified, a region will be chosen automatically.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> ProjectDocumentBatchProcesCall<'a, C, A> {
        self._parent = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ProjectDocumentBatchProcesCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectDocumentBatchProcesCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectDocumentBatchProcesCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Processes a single document.
///
/// A builder for the *documents.process* method supported by a *project* resource.
/// It is not used directly, but through a `ProjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_documentai1_beta2 as documentai1_beta2;
/// use documentai1_beta2::GoogleCloudDocumentaiV1beta2ProcessDocumentRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use documentai1_beta2::Document;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Document::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudDocumentaiV1beta2ProcessDocumentRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().documents_process(req, "parent")
///              .doit();
/// # }
/// ```
pub struct ProjectDocumentProcesCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Document<C, A>,
    _request: GoogleCloudDocumentaiV1beta2ProcessDocumentRequest,
    _parent: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectDocumentProcesCall<'a, C, A> {}

impl<'a, C, A> ProjectDocumentProcesCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GoogleCloudDocumentaiV1beta2Document)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "documentai.projects.documents.process",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("parent", self._parent.to_string()));
        for &field in ["alt", "parent"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta2/{+parent}/documents:process";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+parent}", "parent")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["parent"].iter() {
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

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
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
    pub fn request(mut self, new_value: GoogleCloudDocumentaiV1beta2ProcessDocumentRequest) -> ProjectDocumentProcesCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Target project and location to make a call.
    /// 
    /// Format: `projects/{project-id}/locations/{location-id}`.
    /// 
    /// If no location is specified, a region will be chosen automatically.
    /// This field is only populated when used in ProcessDocument method.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> ProjectDocumentProcesCall<'a, C, A> {
        self._parent = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ProjectDocumentProcesCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectDocumentProcesCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectDocumentProcesCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets the latest state of a long-running operation.  Clients can use this
/// method to poll the operation result at intervals as recommended by the API
/// service.
///
/// A builder for the *locations.operations.get* method supported by a *project* resource.
/// It is not used directly, but through a `ProjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_documentai1_beta2 as documentai1_beta2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use documentai1_beta2::Document;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Document::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().locations_operations_get("name")
///              .doit();
/// # }
/// ```
pub struct ProjectLocationOperationGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Document<C, A>,
    _name: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectLocationOperationGetCall<'a, C, A> {}

impl<'a, C, A> ProjectLocationOperationGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GoogleLongrunningOperation)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "documentai.projects.locations.operations.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        params.push(("name", self._name.to_string()));
        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta2/{+name}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["name"].iter() {
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

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
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


    /// The name of the operation resource.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> ProjectLocationOperationGetCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ProjectLocationOperationGetCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectLocationOperationGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectLocationOperationGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets the latest state of a long-running operation.  Clients can use this
/// method to poll the operation result at intervals as recommended by the API
/// service.
///
/// A builder for the *operations.get* method supported by a *project* resource.
/// It is not used directly, but through a `ProjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_documentai1_beta2 as documentai1_beta2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use documentai1_beta2::Document;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Document::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().operations_get("name")
///              .doit();
/// # }
/// ```
pub struct ProjectOperationGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Document<C, A>,
    _name: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectOperationGetCall<'a, C, A> {}

impl<'a, C, A> ProjectOperationGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GoogleLongrunningOperation)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "documentai.projects.operations.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        params.push(("name", self._name.to_string()));
        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta2/{+name}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["name"].iter() {
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

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
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


    /// The name of the operation resource.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> ProjectOperationGetCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ProjectOperationGetCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectOperationGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectOperationGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Processes a single document.
///
/// A builder for the *locations.documents.process* method supported by a *project* resource.
/// It is not used directly, but through a `ProjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_documentai1_beta2 as documentai1_beta2;
/// use documentai1_beta2::GoogleCloudDocumentaiV1beta2ProcessDocumentRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use documentai1_beta2::Document;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Document::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudDocumentaiV1beta2ProcessDocumentRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().locations_documents_process(req, "parent")
///              .doit();
/// # }
/// ```
pub struct ProjectLocationDocumentProcesCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Document<C, A>,
    _request: GoogleCloudDocumentaiV1beta2ProcessDocumentRequest,
    _parent: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectLocationDocumentProcesCall<'a, C, A> {}

impl<'a, C, A> ProjectLocationDocumentProcesCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GoogleCloudDocumentaiV1beta2Document)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "documentai.projects.locations.documents.process",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("parent", self._parent.to_string()));
        for &field in ["alt", "parent"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta2/{+parent}/documents:process";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+parent}", "parent")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["parent"].iter() {
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

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
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
    pub fn request(mut self, new_value: GoogleCloudDocumentaiV1beta2ProcessDocumentRequest) -> ProjectLocationDocumentProcesCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Target project and location to make a call.
    /// 
    /// Format: `projects/{project-id}/locations/{location-id}`.
    /// 
    /// If no location is specified, a region will be chosen automatically.
    /// This field is only populated when used in ProcessDocument method.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> ProjectLocationDocumentProcesCall<'a, C, A> {
        self._parent = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ProjectLocationDocumentProcesCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectLocationDocumentProcesCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectLocationDocumentProcesCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// LRO endpoint to batch process many documents. The output is written
/// to Cloud Storage as JSON in the [Document] format.
///
/// A builder for the *locations.documents.batchProcess* method supported by a *project* resource.
/// It is not used directly, but through a `ProjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_documentai1_beta2 as documentai1_beta2;
/// use documentai1_beta2::GoogleCloudDocumentaiV1beta2BatchProcessDocumentsRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use documentai1_beta2::Document;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Document::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudDocumentaiV1beta2BatchProcessDocumentsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().locations_documents_batch_process(req, "parent")
///              .doit();
/// # }
/// ```
pub struct ProjectLocationDocumentBatchProcesCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Document<C, A>,
    _request: GoogleCloudDocumentaiV1beta2BatchProcessDocumentsRequest,
    _parent: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectLocationDocumentBatchProcesCall<'a, C, A> {}

impl<'a, C, A> ProjectLocationDocumentBatchProcesCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GoogleLongrunningOperation)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "documentai.projects.locations.documents.batchProcess",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("parent", self._parent.to_string()));
        for &field in ["alt", "parent"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta2/{+parent}/documents:batchProcess";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+parent}", "parent")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["parent"].iter() {
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

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
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
    pub fn request(mut self, new_value: GoogleCloudDocumentaiV1beta2BatchProcessDocumentsRequest) -> ProjectLocationDocumentBatchProcesCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Target project and location to make a call.
    /// 
    /// Format: `projects/{project-id}/locations/{location-id}`.
    /// 
    /// If no location is specified, a region will be chosen automatically.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> ProjectLocationDocumentBatchProcesCall<'a, C, A> {
        self._parent = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ProjectLocationDocumentBatchProcesCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectLocationDocumentBatchProcesCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectLocationDocumentBatchProcesCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


