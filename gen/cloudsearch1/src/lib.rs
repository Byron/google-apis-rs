// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *cloudsearch* crate version *0.1.3+20150309*, where *20150309* is the exact revision of the *cloudsearch:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.3*.
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/cloudsearch1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Cloudsearch.html) ... 
//! 
//! * projects
//!  * [*indexes documents create*](struct.ProjectIndexeDocumentCreateCall.html), [*indexes documents delete*](struct.ProjectIndexeDocumentDeleteCall.html), [*indexes documents get*](struct.ProjectIndexeDocumentGetCall.html), [*indexes documents list*](struct.ProjectIndexeDocumentListCall.html), [*indexes list*](struct.ProjectIndexeListCall.html) and [*indexes search*](struct.ProjectIndexeSearchCall.html)
//! 
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](../index.html).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.Cloudsearch.html)**
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
//! let r = hub.projects().indexes_documents_get(...).doit()
//! let r = hub.projects().indexes_documents_create(...).doit()
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
//! google-cloudsearch1 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_cloudsearch1 as cloudsearch1;
//! use cloudsearch1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use cloudsearch1::Cloudsearch;
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
//!                               hyper::Client::new(),
//!                               <MemoryStorage as Default>::default(), None);
//! let mut hub = Cloudsearch::new(hyper::Client::new(), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.projects().indexes_documents_get("projectId", "indexId", "docId")
//!              .doit();
//! 
//! match result {
//!     Err(e) => match e {
//!         Error::HttpError(err) => println!("HTTPERROR: {:?}", err),
//!         Error::MissingAPIKey => println!("Auth: Missing API Key - used if there are no scopes"),
//!         Error::MissingToken => println!("OAuth2: Missing Token"),
//!         Error::Cancelled => println!("Operation canceled by user"),
//!         Error::UploadSizeLimitExceeded(size, max_size) => println!("Upload size too big: {} of {}", size, max_size),
//!         Error::Failure(_) => println!("General Failure (hyper::client::Response doesn't print)"),
//!         Error::FieldClash(clashed_field) => println!("You added custom parameter which is part of builder: {:?}", clashed_field),
//!         Error::JsonDecodeError(err) => println!("Couldn't understand server reply - maybe API needs update: {:?}", err),
//!     },
//!     Ok(_) => println!("Success (value doesn't print)"),
//! }
//! # }
//! ```
//! ## Handling Errors
//! 
//! All errors produced by the system are provided either as [Result](enum.Result.html) enumeration as return value of 
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](trait.Delegate.html), or the [Authenticator Delegate](../yup-oauth2/trait.AuthenticatorDelegate.html).
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
//! * [request values](trait.RequestValue.html) are borrowed
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 
#![feature(std_misc)]
// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any 
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]
// Required for serde annotations
#![feature(custom_derive, custom_attribute, plugin, slice_patterns)]
#![plugin(serde_macros)]

#[macro_use]
extern crate hyper;
extern crate serde;
extern crate yup_oauth2 as oauth2;
extern crate mime;
extern crate url;

mod cmn;

use std::collections::HashMap;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::default::Default;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use serde::json;
use std::io;
use std::fs;
use std::thread::sleep_ms;

pub use cmn::{MultiPartReader, ToParts, MethodInfo, Result, Error, CallBuilder, Hub, ReadSeek, Part, ResponseResult, RequestValue, NestedType, Delegate, DefaultDelegate, MethodsBuilder, Resource, JsonServerError};


// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// FOR TESTING ONLY
    Full,

    /// View your email address
    UserinfoEmail,

    /// View and manage your data across Google Cloud Platform services
    CloudPlatform,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Full => "https://www.googleapis.com/auth/cloudsearch",
            Scope::UserinfoEmail => "https://www.googleapis.com/auth/userinfo.email",
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Full
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all Cloudsearch related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_cloudsearch1 as cloudsearch1;
/// use cloudsearch1::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use cloudsearch1::Cloudsearch;
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
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Cloudsearch::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().indexes_documents_get("projectId", "indexId", "docId")
///              .doit();
/// 
/// match result {
///     Err(e) => match e {
///         Error::HttpError(err) => println!("HTTPERROR: {:?}", err),
///         Error::MissingAPIKey => println!("Auth: Missing API Key - used if there are no scopes"),
///         Error::MissingToken => println!("OAuth2: Missing Token"),
///         Error::Cancelled => println!("Operation canceled by user"),
///         Error::UploadSizeLimitExceeded(size, max_size) => println!("Upload size too big: {} of {}", size, max_size),
///         Error::Failure(_) => println!("General Failure (hyper::client::Response doesn't print)"),
///         Error::FieldClash(clashed_field) => println!("You added custom parameter which is part of builder: {:?}", clashed_field),
///         Error::JsonDecodeError(err) => println!("Couldn't understand server reply - maybe API needs update: {:?}", err),
///     },
///     Ok(_) => println!("Success (value doesn't print)"),
/// }
/// # }
/// ```
pub struct Cloudsearch<C, NC, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,

    _m: PhantomData<NC>
}

impl<'a, C, NC, A> Hub for Cloudsearch<C, NC, A> {}

impl<'a, C, NC, A> Cloudsearch<C, NC, A>
    where  NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Cloudsearch<C, NC, A> {
        Cloudsearch {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/0.1.3".to_string(),
            _m: PhantomData
        }
    }

    pub fn projects(&'a self) -> ProjectMethods<'a, C, NC, A> {
        ProjectMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/0.1.3`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        let prev = self._user_agent.clone();
        self._user_agent = agent_name;
        prev
    }
}


// ############
// SCHEMAS ###
// ##########
/// A response returned from a listing documents request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [indexes documents list projects](struct.ProjectIndexeDocumentListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ListDocumentsResponse {
    /// If there are more results, retrieve them by invoking list documents call with the same arguments and this `nextPageToken`. If there are no more results, this field is not set.
    #[serde(alias="nextPageToken")]
    pub next_page_token: String,
    /// The list of documents.
    pub documents: Vec<Document>,
}

impl ResponseResult for ListDocumentsResponse {}


/// Information about an index.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct IndexInfo {
    /// The project associated with the index. It cannot be the empty string.
    #[serde(alias="projectId")]
    pub project_id: String,
    /// The index identifier. It cannot be the empty string. It must contain only visible, printable ASCII characters (ASCII codes 33 through 126 inclusive) and be no longer than 100 characters. It cannot begin with an exclamation point ('!'), and it can't begin and end with double underscores ("__").
    #[serde(alias="indexId")]
    pub index_id: String,
    /// Names of indexed fields.
    #[serde(alias="indexedField")]
    pub indexed_field: FieldNames,
}

impl Part for IndexInfo {}


/// A document returned in a SearchResponse.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct SearchResult {
    /// If there are more results, retrieve them by invoking search call with the same arguments and this `nextPageToken`. If there are no more results, this field is not set.
    #[serde(alias="nextPageToken")]
    pub next_page_token: String,
    /// The list of fields in the result. Each field is either from the stored document, the built-in fields (`_rank`, the document rank, and `_score` if scoring is enabled), or computed from any extra `fieldExpressions` defined in the request. For example, if a request contains a `fieldExpressions` named `"TotalPrice"` and expressed as `"Price + Tax"`, the result will have a field whose name is `"TotalPrice"` and whose value is set to the computed sum of the value of field `"Price"` and the value of field `"Tax"`. If a request contains a `fieldExpressions` named `"snippet"` and expressed as `"snippet(\"good times\", content)"`, the result will have a field whose name is `"snippet"` and whose value contains a snippet of text from field `"content"` matching the query "good times".
    pub fields: HashMap<String, FieldValueList>,
    /// The unique identifier of the document.
    #[serde(alias="docId")]
    pub doc_id: String,
}

impl Part for SearchResult {}


/// Names of indexed fields.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct FieldNames {
    /// The names of fields in which TEXT values are stored.
    #[serde(alias="textFields")]
    pub text_fields: Vec<String>,
    /// The names of fields in which DATE values are stored.
    #[serde(alias="dateFields")]
    pub date_fields: Vec<String>,
    /// The names of fields in which NUMBER values are stored.
    #[serde(alias="numberFields")]
    pub number_fields: Vec<String>,
    /// The names of fields in which GEO values are stored.
    #[serde(alias="geoFields")]
    pub geo_fields: Vec<String>,
    /// The names of fields in which ATOM values are stored.
    #[serde(alias="atomFields")]
    pub atom_fields: Vec<String>,
    /// The names of fields in which HTML values are stored.
    #[serde(alias="htmlFields")]
    pub html_fields: Vec<String>,
}

impl Part for FieldNames {}


/// The values of a document field under the same field name.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FieldValueList {
    /// The list of typed values.
    pub values: Vec<FieldValue>,
}

impl Part for FieldValueList {}


/// A response returned from a listing indexes request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [indexes list projects](struct.ProjectIndexeListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ListIndexesResponse {
    /// If there are more results, retrieve them by invoking list indexes call with the same arguments and this `nextPageToken`. If there are no more results, this field is not set.
    #[serde(alias="nextPageToken")]
    pub next_page_token: String,
    /// The information about available indexes.
    pub indexes: Vec<IndexInfo>,
}

impl ResponseResult for ListIndexesResponse {}


/// A response returned from a search request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [indexes search projects](struct.ProjectIndexeSearchCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct SearchResponse {
    /// The number of documents that match the query. It is greater than or equal to the number of documents actually returned. This is an approximation and not an exact count unless it is less than or equal to `matchedCountAccuracy` in search parameter.
    #[serde(alias="matchedCount")]
    pub matched_count: i64,
    /// The list of documents that match the search query.
    pub results: Vec<SearchResult>,
}

impl ResponseResult for SearchResponse {}


/// The message representing a document resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [indexes documents get projects](struct.ProjectIndexeDocumentGetCall.html) (response)
/// * [indexes documents create projects](struct.ProjectIndexeDocumentCreateCall.html) (request|response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Document {
    /// The list of fields in the document. It cannot be the empty list. Each field has a name and a list of values. The field name is unique to a document and is case sensitive. The name can only contain ASCII characters. It must start with a letter and can contain letters, digits, or underscore. It cannot be longer than 500 characters and cannot be the empty string. A field can have multiple values with same or different types, however, it cannot have multiple Timestamp or number values.
    pub fields: Option<HashMap<String, FieldValueList>>,
    /// The unique identifier of the document. It must contain only visible, printable ASCII characters (ASCII codes 33 through 126 inclusive) and be no longer than 500 characters. It cannot begin with an exclamation point ('!'), and it can't begin and end with double underscores ("__"). If missing, it is automatically assigned for the document.
    #[serde(alias="docId")]
    pub doc_id: Option<String>,
    /// A positive integer which determines the default ordering of documents returned from a search. The rank can be set explicitly when the document is created. It is a bad idea to assign the same rank to many documents, and the same rank should never be assigned to more than 10,000 documents. By default (when it is not specified or set to 0), it is set at the time the document is created to the number of seconds since January 1, 2011. The rank can be used in field_expressions, order_by or return_fields in a search request, where it is referenced as `_rank`.
    pub rank: Option<i32>,
}

impl RequestValue for Document {}
impl ResponseResult for Document {}


/// The value of a document field and associated metadata. Exactly one of the value fields may be set.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FieldValue {
    /// The language of a string value. If given, the language must be a valid `ISO 639-1` code.
    pub lang: String,
    /// The value of a number-valued field.
    #[serde(alias="numberValue")]
    pub number_value: f64,
    /// The value of a timestamp-valued field.
    #[serde(alias="timestampValue")]
    pub timestamp_value: String,
    /// The value of a string-valued field.
    #[serde(alias="stringValue")]
    pub string_value: String,
    /// The format of a string value. By default, the string format is `DEFAULT`, where a format will be automatically detected.
    #[serde(alias="stringFormat")]
    pub string_format: String,
    /// The value of a GEO-valued field, represented in string with any of the listed [ways of writing coordinates](http://en.wikipedia.org/wiki/Geographic_coordinate_conversion#Ways_of_writing_coordinates)
    #[serde(alias="geoValue")]
    pub geo_value: String,
}

impl Part for FieldValue {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [indexes documents delete projects](struct.ProjectIndexeDocumentDeleteCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Empty;

impl ResponseResult for Empty {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the `Cloudsearch` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_cloudsearch1 as cloudsearch1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use cloudsearch1::Cloudsearch;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Cloudsearch::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `indexes_documents_create(...)`, `indexes_documents_delete(...)`, `indexes_documents_get(...)`, `indexes_documents_list(...)`, `indexes_list(...)` and `indexes_search(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Cloudsearch<C, NC, A>,
}

impl<'a, C, NC, A> MethodsBuilder for ProjectMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> ProjectMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists search indexes belonging to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - The project from which to retrieve indexes. It cannot be the empty string.
    /// * `indexNamePrefix` - The prefix of the index name. It is used to list all indexes with names that have this prefix.
    /// * `pageSize` - The maximum number of indexes to return per page. If not specified, 100 indexes are returned per page.
    /// * `pageToken` - A `nextPageToken` returned from previous list indexes call as the starting point for this call. If not specified, list indexes from the beginning.
    /// * `view` - Specifies which parts of the IndexInfo resource is returned in the response. If not specified, `ID_ONLY` is used.
    pub fn indexes_list(&self, project_id: &str, index_name_prefix: &str, page_size: i32, page_token: &str, view: &str) -> ProjectIndexeListCall<'a, C, NC, A> {
        ProjectIndexeListCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _index_name_prefix: index_name_prefix.to_string(),
            _page_size: page_size,
            _page_token: page_token.to_string(),
            _view: view.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a document from an index.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - The project associated with the index for retrieving the document. It cannot be the empty string.
    /// * `indexId` - The index from which to retrieve the document. It cannot be the empty string.
    /// * `docId` - The identifier of the document to retrieve. It cannot be the empty string.
    pub fn indexes_documents_get(&self, project_id: &str, index_id: &str, doc_id: &str) -> ProjectIndexeDocumentGetCall<'a, C, NC, A> {
        ProjectIndexeDocumentGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _index_id: index_id.to_string(),
            _doc_id: doc_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the documents in the named index that match the query.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - The project associated with the index for searching document. It cannot be the empty string.
    /// * `indexId` - The index to search. It cannot be the empty string.
    /// * `query` - The query string in search query syntax. If the query is missing or empty, all documents are returned.
    /// * `fieldExpressions` - Customized expressions used in `orderBy` or `returnFields`. The expression can contain fields in `Document`, the built-in fields ( `_rank`, the document rank, and `_score` if scoring is enabled) and fields defined in `fieldExpressions`. Each field expression is represented in a json object with the following fields: * `name`: the name of the field expression in string. * `expression`: the expression to be computed. It can be a combination of supported functions encoded in string. Expressions involving number fields can use the arithmetical operators (`+`, `-`, `*`, `/`) and the built-in numeric functions (`max`, `min`, `pow`, `count`, `log`, `abs`). Expressions involving geopoint fields can use the `geopoint` and `distance` functions. Expressions for text and html fields can use the `snippet` function. For example: ``` fieldExpressions={name: "TotalPrice", expression: "(Price+Tax)"} ``` ``` fieldExpressions={name: "snippet", expression: "snippet('good times', content)"} ``` The field expression names can be used in `orderBy` and `returnFields` after they are defined in `fieldExpressions`.
    /// * `pageSize` - The maximum number of search results to return per page. Searches perform best when the `pageSize` is kept as small as possible. If not specified, 10 results are returned per page.
    /// * `pageToken` - A `nextPageToken` returned from previous Search call as the starting point for this call. Pagination tokens provide better performance and consistency than offsets, and they cannot be used in combination with offsets.
    /// * `offset` - Offset is used to move to an arbitrary result, independent of the previous results. Offsets are inefficient when compared to `pageToken`. `pageToken` and `offset` cannot be both set. The default value of `offset` is 0.
    /// * `matchedCountAccuracy` - Minimum accuracy requirement for `matchedCount` in search response. If specified, `matchedCount` will be accurate up to at least that number. For example, when set to 100, any `matchedCount <= 100` is accurate. This option may add considerable latency/expense. By default (when it is not specified or set to 0), the accuracy is the same as `pageSize`.
    /// * `orderBy` - Comma-separated list of fields for sorting on the search result, including fields from `Document`, the built-in fields (`_rank` and `_score`), and fields defined in `fieldExpressions`. For example: `orderBy="foo,bar"`. The default sorting order is ascending. To specify descending order for a field, a suffix `" desc"` should be appended to the field name. For example: `orderBy="foo desc,bar"`. The default value for text sort is the empty string, and the default value for numeric sort is 0. If not specified, the search results are automatically sorted by descending `_rank`. Sorting by ascending `_rank` is not allowed.
    /// * `scorer` - The scoring function to invoke on a search result for this query. If `scorer` is not set, scoring is disabled and `_score` is 0 for all documents in the search result. To enable document relevancy score based on term frequency, set `"scorer=generic"`.
    /// * `scorerSize` - Maximum number of top retrieved results to score. It is valid only when `scorer` is set. If not specified, 100 retrieved results are scored.
    /// * `returnFields` - List of fields to return in `SearchResult` objects. It can be fields from `Document`, the built-in fields `_rank` and `_score`, and fields defined in `fieldExpressions`. Use `"*"` to return all fields from `Document`.
    pub fn indexes_search(&self, project_id: &str, index_id: &str, query: &str, field_expressions: &Vec<String>, page_size: i32, page_token: &str, offset: i32, matched_count_accuracy: i32, order_by: &str, scorer: &str, scorer_size: i32, return_fields: &Vec<String>) -> ProjectIndexeSearchCall<'a, C, NC, A> {
        ProjectIndexeSearchCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _index_id: index_id.to_string(),
            _query: query.to_string(),
            _field_expressions: field_expressions.clone(),
            _page_size: page_size,
            _page_token: page_token.to_string(),
            _offset: offset,
            _matched_count_accuracy: matched_count_accuracy,
            _order_by: order_by.to_string(),
            _scorer: scorer.to_string(),
            _scorer_size: scorer_size,
            _return_fields: return_fields.clone(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists documents in the specified search index. Intended for batch processing.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - The project associated with the index for listing documents. It cannot be the empty string.
    /// * `indexId` - The index from which to list the documents. It cannot be the empty string.
    /// * `pageSize` - The maximum number of documents to return per page. If not specified, 100 documents are returned per page.
    /// * `pageToken` - A `nextPageToken` returned from previous list documents call as the starting point for this call. If not specified, list documents from the beginning.
    /// * `view` - Specifies which part of the document resource is returned in the response. If not specified, `ID_ONLY` is used.
    pub fn indexes_documents_list(&self, project_id: &str, index_id: &str, page_size: i32, page_token: &str, view: &str) -> ProjectIndexeDocumentListCall<'a, C, NC, A> {
        ProjectIndexeDocumentListCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _index_id: index_id.to_string(),
            _page_size: page_size,
            _page_token: page_token.to_string(),
            _view: view.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a document for indexing or updates an indexed document. The returned document contains only the ID of the new document. When `docId` is absent from the document, it is provided by the server.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - The project associated with the index for adding document. It cannot be the empty string.
    /// * `indexId` - The index to add document to. It cannot be the empty string.
    pub fn indexes_documents_create(&self, request: &Document, project_id: &str, index_id: &str) -> ProjectIndexeDocumentCreateCall<'a, C, NC, A> {
        ProjectIndexeDocumentCreateCall {
            hub: self.hub,
            _request: request.clone(),
            _project_id: project_id.to_string(),
            _index_id: index_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a document from an index.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - The project associated with the index for deleting document. It cannot be the empty string.
    /// * `indexId` - The index from which to delete the document. It cannot be the empty string.
    /// * `docId` - The document to be deleted. It cannot be the empty string.
    pub fn indexes_documents_delete(&self, project_id: &str, index_id: &str, doc_id: &str) -> ProjectIndexeDocumentDeleteCall<'a, C, NC, A> {
        ProjectIndexeDocumentDeleteCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _index_id: index_id.to_string(),
            _doc_id: doc_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Lists search indexes belonging to the specified project.
///
/// A builder for the *indexes.list* method supported by a *project* resource.
/// It is not used directly, but through a `ProjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudsearch1 as cloudsearch1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudsearch1::Cloudsearch;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Cloudsearch::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().indexes_list("projectId", "indexNamePrefix", -81, "pageToken", "view")
///              .doit();
/// # }
/// ```
pub struct ProjectIndexeListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Cloudsearch<C, NC, A>,
    _project_id: String,
    _index_name_prefix: String,
    _page_size: i32,
    _page_token: String,
    _view: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ProjectIndexeListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ProjectIndexeListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListIndexesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudsearch.projects.indexes.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((7 + self._additional_params.len()));
        params.push(("projectId", self._project_id.to_string()));
        params.push(("indexNamePrefix", self._index_name_prefix.to_string()));
        params.push(("pageSize", self._page_size.to_string()));
        params.push(("pageToken", self._page_token.to_string()));
        params.push(("view", self._view.to_string()));
        for &field in ["alt", "projectId", "indexNamePrefix", "pageSize", "pageToken", "view"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://cloudsearch.googleapis.com/v1/projects/{projectId}/indexes".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId")].iter() {
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
            for param_name in ["projectId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The project from which to retrieve indexes. It cannot be the empty string.
    pub fn project_id(mut self, new_value: &str) -> ProjectIndexeListCall<'a, C, NC, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// Sets the *index name prefix* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The prefix of the index name. It is used to list all indexes with names that have this prefix.
    pub fn index_name_prefix(mut self, new_value: &str) -> ProjectIndexeListCall<'a, C, NC, A> {
        self._index_name_prefix = new_value.to_string();
        self
    }
    /// Sets the *page size* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The maximum number of indexes to return per page. If not specified, 100 indexes are returned per page.
    pub fn page_size(mut self, new_value: i32) -> ProjectIndexeListCall<'a, C, NC, A> {
        self._page_size = new_value;
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// A `nextPageToken` returned from previous list indexes call as the starting point for this call. If not specified, list indexes from the beginning.
    pub fn page_token(mut self, new_value: &str) -> ProjectIndexeListCall<'a, C, NC, A> {
        self._page_token = new_value.to_string();
        self
    }
    /// Sets the *view* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Specifies which parts of the IndexInfo resource is returned in the response. If not specified, `ID_ONLY` is used.
    pub fn view(mut self, new_value: &str) -> ProjectIndexeListCall<'a, C, NC, A> {
        self._view = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectIndexeListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ProjectIndexeListCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> ProjectIndexeListCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Retrieves a document from an index.
///
/// A builder for the *indexes.documents.get* method supported by a *project* resource.
/// It is not used directly, but through a `ProjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudsearch1 as cloudsearch1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudsearch1::Cloudsearch;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Cloudsearch::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().indexes_documents_get("projectId", "indexId", "docId")
///              .doit();
/// # }
/// ```
pub struct ProjectIndexeDocumentGetCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Cloudsearch<C, NC, A>,
    _project_id: String,
    _index_id: String,
    _doc_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ProjectIndexeDocumentGetCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ProjectIndexeDocumentGetCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Document)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudsearch.projects.indexes.documents.get", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("projectId", self._project_id.to_string()));
        params.push(("indexId", self._index_id.to_string()));
        params.push(("docId", self._doc_id.to_string()));
        for &field in ["alt", "projectId", "indexId", "docId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://cloudsearch.googleapis.com/v1/projects/{projectId}/indexes/{indexId}/documents/{docId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId"), ("{indexId}", "indexId"), ("{docId}", "docId")].iter() {
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
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["projectId", "indexId", "docId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The project associated with the index for retrieving the document. It cannot be the empty string.
    pub fn project_id(mut self, new_value: &str) -> ProjectIndexeDocumentGetCall<'a, C, NC, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// Sets the *index id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The index from which to retrieve the document. It cannot be the empty string.
    pub fn index_id(mut self, new_value: &str) -> ProjectIndexeDocumentGetCall<'a, C, NC, A> {
        self._index_id = new_value.to_string();
        self
    }
    /// Sets the *doc id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The identifier of the document to retrieve. It cannot be the empty string.
    pub fn doc_id(mut self, new_value: &str) -> ProjectIndexeDocumentGetCall<'a, C, NC, A> {
        self._doc_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectIndexeDocumentGetCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ProjectIndexeDocumentGetCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> ProjectIndexeDocumentGetCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Lists the documents in the named index that match the query.
///
/// A builder for the *indexes.search* method supported by a *project* resource.
/// It is not used directly, but through a `ProjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudsearch1 as cloudsearch1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudsearch1::Cloudsearch;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Cloudsearch::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().indexes_search("projectId", "indexId", "query", "fieldExpressions", -21, "pageToken", -34, -17, "orderBy", "scorer", -5, "returnFields")
///              .doit();
/// # }
/// ```
pub struct ProjectIndexeSearchCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Cloudsearch<C, NC, A>,
    _project_id: String,
    _index_id: String,
    _query: String,
    _field_expressions: Vec<String>,
    _page_size: i32,
    _page_token: String,
    _offset: i32,
    _matched_count_accuracy: i32,
    _order_by: String,
    _scorer: String,
    _scorer_size: i32,
    _return_fields: Vec<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ProjectIndexeSearchCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ProjectIndexeSearchCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, SearchResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudsearch.projects.indexes.search", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((14 + self._additional_params.len()));
        params.push(("projectId", self._project_id.to_string()));
        params.push(("indexId", self._index_id.to_string()));
        params.push(("query", self._query.to_string()));
        if self._field_expressions.len() > 0 {
            let mut s = String::new();
            for f in self._field_expressions.iter() {
                s.push_str(&("/".to_string() + &f.to_string()));
            }
            params.push(("fieldExpressions", s));
        }
        params.push(("pageSize", self._page_size.to_string()));
        params.push(("pageToken", self._page_token.to_string()));
        params.push(("offset", self._offset.to_string()));
        params.push(("matchedCountAccuracy", self._matched_count_accuracy.to_string()));
        params.push(("orderBy", self._order_by.to_string()));
        params.push(("scorer", self._scorer.to_string()));
        params.push(("scorerSize", self._scorer_size.to_string()));
        if self._return_fields.len() > 0 {
            let mut s = String::new();
            for f in self._return_fields.iter() {
                s.push_str(&("/".to_string() + &f.to_string()));
            }
            params.push(("returnFields", s));
        }
        for &field in ["alt", "projectId", "indexId", "query", "fieldExpressions", "pageSize", "pageToken", "offset", "matchedCountAccuracy", "orderBy", "scorer", "scorerSize", "returnFields"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://cloudsearch.googleapis.com/v1/projects/{projectId}/indexes/{indexId}/search".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId"), ("{indexId}", "indexId")].iter() {
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
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["projectId", "indexId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The project associated with the index for searching document. It cannot be the empty string.
    pub fn project_id(mut self, new_value: &str) -> ProjectIndexeSearchCall<'a, C, NC, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// Sets the *index id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The index to search. It cannot be the empty string.
    pub fn index_id(mut self, new_value: &str) -> ProjectIndexeSearchCall<'a, C, NC, A> {
        self._index_id = new_value.to_string();
        self
    }
    /// Sets the *query* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The query string in search query syntax. If the query is missing or empty, all documents are returned.
    pub fn query(mut self, new_value: &str) -> ProjectIndexeSearchCall<'a, C, NC, A> {
        self._query = new_value.to_string();
        self
    }
    /// Append the given value to the *field expressions* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Customized expressions used in `orderBy` or `returnFields`. The expression can contain fields in `Document`, the built-in fields ( `_rank`, the document rank, and `_score` if scoring is enabled) and fields defined in `fieldExpressions`. Each field expression is represented in a json object with the following fields: * `name`: the name of the field expression in string. * `expression`: the expression to be computed. It can be a combination of supported functions encoded in string. Expressions involving number fields can use the arithmetical operators (`+`, `-`, `*`, `/`) and the built-in numeric functions (`max`, `min`, `pow`, `count`, `log`, `abs`). Expressions involving geopoint fields can use the `geopoint` and `distance` functions. Expressions for text and html fields can use the `snippet` function. For example: ``` fieldExpressions={name: "TotalPrice", expression: "(Price+Tax)"} ``` ``` fieldExpressions={name: "snippet", expression: "snippet('good times', content)"} ``` The field expression names can be used in `orderBy` and `returnFields` after they are defined in `fieldExpressions`.
    pub fn add_field_expressions(mut self, new_value: &str) -> ProjectIndexeSearchCall<'a, C, NC, A> {
        self._field_expressions.push(new_value.to_string());
        self
    }
    /// Sets the *page size* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The maximum number of search results to return per page. Searches perform best when the `pageSize` is kept as small as possible. If not specified, 10 results are returned per page.
    pub fn page_size(mut self, new_value: i32) -> ProjectIndexeSearchCall<'a, C, NC, A> {
        self._page_size = new_value;
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// A `nextPageToken` returned from previous Search call as the starting point for this call. Pagination tokens provide better performance and consistency than offsets, and they cannot be used in combination with offsets.
    pub fn page_token(mut self, new_value: &str) -> ProjectIndexeSearchCall<'a, C, NC, A> {
        self._page_token = new_value.to_string();
        self
    }
    /// Sets the *offset* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Offset is used to move to an arbitrary result, independent of the previous results. Offsets are inefficient when compared to `pageToken`. `pageToken` and `offset` cannot be both set. The default value of `offset` is 0.
    pub fn offset(mut self, new_value: i32) -> ProjectIndexeSearchCall<'a, C, NC, A> {
        self._offset = new_value;
        self
    }
    /// Sets the *matched count accuracy* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Minimum accuracy requirement for `matchedCount` in search response. If specified, `matchedCount` will be accurate up to at least that number. For example, when set to 100, any `matchedCount <= 100` is accurate. This option may add considerable latency/expense. By default (when it is not specified or set to 0), the accuracy is the same as `pageSize`.
    pub fn matched_count_accuracy(mut self, new_value: i32) -> ProjectIndexeSearchCall<'a, C, NC, A> {
        self._matched_count_accuracy = new_value;
        self
    }
    /// Sets the *order by* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Comma-separated list of fields for sorting on the search result, including fields from `Document`, the built-in fields (`_rank` and `_score`), and fields defined in `fieldExpressions`. For example: `orderBy="foo,bar"`. The default sorting order is ascending. To specify descending order for a field, a suffix `" desc"` should be appended to the field name. For example: `orderBy="foo desc,bar"`. The default value for text sort is the empty string, and the default value for numeric sort is 0. If not specified, the search results are automatically sorted by descending `_rank`. Sorting by ascending `_rank` is not allowed.
    pub fn order_by(mut self, new_value: &str) -> ProjectIndexeSearchCall<'a, C, NC, A> {
        self._order_by = new_value.to_string();
        self
    }
    /// Sets the *scorer* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The scoring function to invoke on a search result for this query. If `scorer` is not set, scoring is disabled and `_score` is 0 for all documents in the search result. To enable document relevancy score based on term frequency, set `"scorer=generic"`.
    pub fn scorer(mut self, new_value: &str) -> ProjectIndexeSearchCall<'a, C, NC, A> {
        self._scorer = new_value.to_string();
        self
    }
    /// Sets the *scorer size* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Maximum number of top retrieved results to score. It is valid only when `scorer` is set. If not specified, 100 retrieved results are scored.
    pub fn scorer_size(mut self, new_value: i32) -> ProjectIndexeSearchCall<'a, C, NC, A> {
        self._scorer_size = new_value;
        self
    }
    /// Append the given value to the *return fields* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// List of fields to return in `SearchResult` objects. It can be fields from `Document`, the built-in fields `_rank` and `_score`, and fields defined in `fieldExpressions`. Use `"*"` to return all fields from `Document`.
    pub fn add_return_fields(mut self, new_value: &str) -> ProjectIndexeSearchCall<'a, C, NC, A> {
        self._return_fields.push(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectIndexeSearchCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ProjectIndexeSearchCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> ProjectIndexeSearchCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Lists documents in the specified search index. Intended for batch processing.
///
/// A builder for the *indexes.documents.list* method supported by a *project* resource.
/// It is not used directly, but through a `ProjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudsearch1 as cloudsearch1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudsearch1::Cloudsearch;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Cloudsearch::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().indexes_documents_list("projectId", "indexId", -9, "pageToken", "view")
///              .doit();
/// # }
/// ```
pub struct ProjectIndexeDocumentListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Cloudsearch<C, NC, A>,
    _project_id: String,
    _index_id: String,
    _page_size: i32,
    _page_token: String,
    _view: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ProjectIndexeDocumentListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ProjectIndexeDocumentListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListDocumentsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudsearch.projects.indexes.documents.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((7 + self._additional_params.len()));
        params.push(("projectId", self._project_id.to_string()));
        params.push(("indexId", self._index_id.to_string()));
        params.push(("pageSize", self._page_size.to_string()));
        params.push(("pageToken", self._page_token.to_string()));
        params.push(("view", self._view.to_string()));
        for &field in ["alt", "projectId", "indexId", "pageSize", "pageToken", "view"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://cloudsearch.googleapis.com/v1/projects/{projectId}/indexes/{indexId}/documents".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId"), ("{indexId}", "indexId")].iter() {
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
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["projectId", "indexId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The project associated with the index for listing documents. It cannot be the empty string.
    pub fn project_id(mut self, new_value: &str) -> ProjectIndexeDocumentListCall<'a, C, NC, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// Sets the *index id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The index from which to list the documents. It cannot be the empty string.
    pub fn index_id(mut self, new_value: &str) -> ProjectIndexeDocumentListCall<'a, C, NC, A> {
        self._index_id = new_value.to_string();
        self
    }
    /// Sets the *page size* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The maximum number of documents to return per page. If not specified, 100 documents are returned per page.
    pub fn page_size(mut self, new_value: i32) -> ProjectIndexeDocumentListCall<'a, C, NC, A> {
        self._page_size = new_value;
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// A `nextPageToken` returned from previous list documents call as the starting point for this call. If not specified, list documents from the beginning.
    pub fn page_token(mut self, new_value: &str) -> ProjectIndexeDocumentListCall<'a, C, NC, A> {
        self._page_token = new_value.to_string();
        self
    }
    /// Sets the *view* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Specifies which part of the document resource is returned in the response. If not specified, `ID_ONLY` is used.
    pub fn view(mut self, new_value: &str) -> ProjectIndexeDocumentListCall<'a, C, NC, A> {
        self._view = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectIndexeDocumentListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ProjectIndexeDocumentListCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> ProjectIndexeDocumentListCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Inserts a document for indexing or updates an indexed document. The returned document contains only the ID of the new document. When `docId` is absent from the document, it is provided by the server.
///
/// A builder for the *indexes.documents.create* method supported by a *project* resource.
/// It is not used directly, but through a `ProjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudsearch1 as cloudsearch1;
/// use cloudsearch1::Document;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudsearch1::Cloudsearch;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Cloudsearch::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: Document = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().indexes_documents_create(&req, "projectId", "indexId")
///              .doit();
/// # }
/// ```
pub struct ProjectIndexeDocumentCreateCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Cloudsearch<C, NC, A>,
    _request: Document,
    _project_id: String,
    _index_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ProjectIndexeDocumentCreateCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ProjectIndexeDocumentCreateCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Document)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudsearch.projects.indexes.documents.create", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("projectId", self._project_id.to_string()));
        params.push(("indexId", self._index_id.to_string()));
        for &field in ["alt", "projectId", "indexId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://cloudsearch.googleapis.com/v1/projects/{projectId}/indexes/{indexId}/documents".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId"), ("{indexId}", "indexId")].iter() {
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
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["projectId", "indexId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_ref())
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
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &Document) -> ProjectIndexeDocumentCreateCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The project associated with the index for adding document. It cannot be the empty string.
    pub fn project_id(mut self, new_value: &str) -> ProjectIndexeDocumentCreateCall<'a, C, NC, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// Sets the *index id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The index to add document to. It cannot be the empty string.
    pub fn index_id(mut self, new_value: &str) -> ProjectIndexeDocumentCreateCall<'a, C, NC, A> {
        self._index_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectIndexeDocumentCreateCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ProjectIndexeDocumentCreateCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> ProjectIndexeDocumentCreateCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Deletes a document from an index.
///
/// A builder for the *indexes.documents.delete* method supported by a *project* resource.
/// It is not used directly, but through a `ProjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudsearch1 as cloudsearch1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudsearch1::Cloudsearch;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Cloudsearch::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().indexes_documents_delete("projectId", "indexId", "docId")
///              .doit();
/// # }
/// ```
pub struct ProjectIndexeDocumentDeleteCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Cloudsearch<C, NC, A>,
    _project_id: String,
    _index_id: String,
    _doc_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ProjectIndexeDocumentDeleteCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ProjectIndexeDocumentDeleteCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Empty)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudsearch.projects.indexes.documents.delete", 
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("projectId", self._project_id.to_string()));
        params.push(("indexId", self._index_id.to_string()));
        params.push(("docId", self._doc_id.to_string()));
        for &field in ["alt", "projectId", "indexId", "docId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://cloudsearch.googleapis.com/v1/projects/{projectId}/indexes/{indexId}/documents/{docId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId"), ("{indexId}", "indexId"), ("{docId}", "docId")].iter() {
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
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["projectId", "indexId", "docId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Delete, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The project associated with the index for deleting document. It cannot be the empty string.
    pub fn project_id(mut self, new_value: &str) -> ProjectIndexeDocumentDeleteCall<'a, C, NC, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// Sets the *index id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The index from which to delete the document. It cannot be the empty string.
    pub fn index_id(mut self, new_value: &str) -> ProjectIndexeDocumentDeleteCall<'a, C, NC, A> {
        self._index_id = new_value.to_string();
        self
    }
    /// Sets the *doc id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The document to be deleted. It cannot be the empty string.
    pub fn doc_id(mut self, new_value: &str) -> ProjectIndexeDocumentDeleteCall<'a, C, NC, A> {
        self._doc_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectIndexeDocumentDeleteCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ProjectIndexeDocumentDeleteCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> ProjectIndexeDocumentDeleteCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


