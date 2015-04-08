// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *discovery* crate version *0.1.5+00000000*, where *00000000* is the exact revision of the *discovery:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.5*.
//! 
//! Everything else about the *discovery* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/discovery/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/discovery1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Discovery.html) ... 
//! 
//! * apis
//!  * [*get rest*](struct.ApiGetRestCall.html) and [*list*](struct.ApiListCall.html)
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
//! * **[Hub](struct.Discovery.html)**
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
//! let r = hub.apis().get_rest(...).doit()
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
//! google-discovery1 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_discovery1 as discovery1;
//! use discovery1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use discovery1::Discovery;
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
//! let mut hub = Discovery::new(hyper::Client::new(), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.apis().get_rest("api", "version")
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
use serde::json;
use std::io;
use std::fs;
use std::thread::sleep_ms;

pub use cmn::{MultiPartReader, ToParts, MethodInfo, Result, Error, CallBuilder, Hub, ReadSeek, Part, ResponseResult, RequestValue, NestedType, Delegate, DefaultDelegate, MethodsBuilder, Resource, JsonServerError};


// ##############
// UTILITIES ###
// ############




// ########
// HUB ###
// ######

/// Central instance to access all Discovery related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_discovery1 as discovery1;
/// use discovery1::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use discovery1::Discovery;
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
/// let mut hub = Discovery::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.apis().get_rest("api", "version")
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
pub struct Discovery<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
}

impl<'a, C, A> Hub for Discovery<C, A> {}

impl<'a, C, A> Discovery<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Discovery<C, A> {
        Discovery {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/0.1.5".to_string(),
        }
    }

    pub fn apis(&'a self) -> ApiMethods<'a, C, A> {
        ApiMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/0.1.5`.
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
/// OAuth 2.0 authentication information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct RestDescriptionAuthOauth2 {
    /// Available OAuth 2.0 scopes.
    pub scopes: HashMap<String, RestDescriptionAuthOauth2Scopes>,
}

impl NestedType for RestDescriptionAuthOauth2 {}
impl Part for RestDescriptionAuthOauth2 {}


/// The schema for the response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct RestMethodResponse {
    /// Schema ID for the response schema.
    #[serde(rename="$ref")]
    pub ref_: String,
}

impl NestedType for RestMethodResponse {}
impl Part for RestMethodResponse {}


/// In a variant data type, the value of one property is used to determine how to interpret the entire entity. Its value must exist in a map of descriminant values to schema names.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct JsonSchemaVariant {
    /// The map of discriminant value to schema to use for parsing..
    pub map: Vec<JsonSchemaVariantMap>,
    /// The name of the type discriminant property.
    pub discriminant: String,
}

impl NestedType for JsonSchemaVariant {}
impl Part for JsonSchemaVariant {}


/// Supported upload protocols.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct RestMethodMediaUploadProtocols {
    /// Supports uploading as a single HTTP request.
    pub simple: RestMethodMediaUploadProtocolsSimple,
    /// Supports the Resumable Media Upload protocol.
    pub resumable: RestMethodMediaUploadProtocolsResumable,
}

impl NestedType for RestMethodMediaUploadProtocols {}
impl Part for RestMethodMediaUploadProtocols {}


/// Supports the Resumable Media Upload protocol.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct RestMethodMediaUploadProtocolsResumable {
    /// The URI path to be used for upload. Should be used in conjunction with the basePath property at the api-level.
    pub path: String,
    /// True if this endpoint supports uploading multipart media.
    pub multipart: bool,
}

impl NestedType for RestMethodMediaUploadProtocolsResumable {}
impl Part for RestMethodMediaUploadProtocolsResumable {}


/// Additional information about this property.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct JsonSchemaAnnotations {
    /// A list of methods for which this property is required on requests.
    pub required: Vec<String>,
}

impl NestedType for JsonSchemaAnnotations {}
impl Part for JsonSchemaAnnotations {}


/// The map of discriminant value to schema to use for parsing..
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct JsonSchemaVariantMap {
    /// no description provided
    pub type_value: String,
    /// no description provided
    #[serde(rename="$ref")]
    pub ref_: String,
}

impl NestedType for JsonSchemaVariantMap {}
impl Part for JsonSchemaVariantMap {}


/// Links to 16x16 and 32x32 icons representing the API.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct RestDescriptionIcons {
    /// The URL of the 32x32 icon.
    pub x32: String,
    /// The URL of the 16x16 icon.
    pub x16: String,
}

impl NestedType for RestDescriptionIcons {}
impl Part for RestDescriptionIcons {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct RestMethod {
    /// OAuth 2.0 scopes applicable to this method.
    pub scopes: Vec<String>,
    /// Description of this method.
    pub description: String,
    /// Details for all parameters in this method.
    pub parameters: HashMap<String, JsonSchema>,
    /// Whether this method supports media uploads.
    #[serde(rename="supportsMediaUpload")]
    pub supports_media_upload: bool,
    /// Whether this method requires an ETag to be specified. The ETag is sent as an HTTP If-Match or If-None-Match header.
    #[serde(rename="etagRequired")]
    pub etag_required: bool,
    /// Media upload parameters.
    #[serde(rename="mediaUpload")]
    pub media_upload: RestMethodMediaUpload,
    /// The schema for the request.
    pub request: RestMethodRequest,
    /// Indicates that downloads from this method should use the download service URL (i.e. "/download"). Only applies if the method supports media download.
    #[serde(rename="useMediaDownloadService")]
    pub use_media_download_service: bool,
    /// HTTP method used by this method.
    #[serde(rename="httpMethod")]
    pub http_method: String,
    /// Whether this method supports subscriptions.
    #[serde(rename="supportsSubscription")]
    pub supports_subscription: bool,
    /// Ordered list of required parameters, serves as a hint to clients on how to structure their method signatures. The array is ordered such that the "most-significant" parameter appears first.
    #[serde(rename="parameterOrder")]
    pub parameter_order: Vec<String>,
    /// A unique ID for this method. This property can be used to match methods between different versions of Discovery.
    pub id: String,
    /// The URI path of this REST method. Should be used in conjunction with the basePath property at the api-level.
    pub path: String,
    /// The schema for the response.
    pub response: RestMethodResponse,
    /// Whether this method supports media downloads.
    #[serde(rename="supportsMediaDownload")]
    pub supports_media_download: bool,
}

impl Part for RestMethod {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get rest apis](struct.ApiGetRestCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct RestDescription {
    /// The protocol described by this document.
    pub protocol: String,
    /// API-level methods for this API.
    pub methods: HashMap<String, RestMethod>,
    /// Labels for the status of this API, such as labs or deprecated.
    pub labels: Vec<String>,
    /// The kind for this response.
    pub kind: String,
    /// Indicates how the API name should be capitalized and split into various parts. Useful for generating pretty class names.
    #[serde(rename="canonicalName")]
    pub canonical_name: String,
    /// The name of the owner of this API. See ownerDomain.
    #[serde(rename="ownerName")]
    pub owner_name: String,
    /// A link to human readable documentation for the API.
    #[serde(rename="documentationLink")]
    pub documentation_link: String,
    /// Authentication information.
    pub auth: RestDescriptionAuth,
    /// The package of the owner of this API. See ownerDomain.
    #[serde(rename="packagePath")]
    pub package_path: String,
    /// The path for REST batch requests.
    #[serde(rename="batchPath")]
    pub batch_path: String,
    /// The ID of this API.
    pub id: String,
    /// A list of supported features for this API.
    pub features: Vec<String>,
    /// The domain of the owner of this API. Together with the ownerName and a packagePath values, this can be used to generate a library for this API which would have a unique fully qualified name.
    #[serde(rename="ownerDomain")]
    pub owner_domain: String,
    /// The root URL under which all API services live.
    #[serde(rename="rootUrl")]
    pub root_url: String,
    /// The name of this API.
    pub name: String,
    /// Common parameters that apply across all apis.
    pub parameters: HashMap<String, JsonSchema>,
    /// Links to 16x16 and 32x32 icons representing the API.
    pub icons: RestDescriptionIcons,
    /// The description of this API.
    pub description: String,
    /// The title of this API.
    pub title: String,
    /// [DEPRECATED] The base URL for REST requests.
    #[serde(rename="baseUrl")]
    pub base_url: String,
    /// The ETag for this response.
    pub etag: String,
    /// The version of this API.
    pub version: String,
    /// The base path for all REST requests.
    #[serde(rename="servicePath")]
    pub service_path: String,
    /// Indicate the version of the Discovery API used to generate this doc.
    #[serde(rename="discoveryVersion")]
    pub discovery_version: String,
    /// The schemas for this API.
    pub schemas: HashMap<String, JsonSchema>,
    /// [DEPRECATED] The base path for REST requests.
    #[serde(rename="basePath")]
    pub base_path: String,
    /// The resources in this API.
    pub resources: HashMap<String, RestResource>,
    /// The version of this API.
    pub revision: String,
}

impl ResponseResult for RestDescription {}


/// Media upload parameters.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct RestMethodMediaUpload {
    /// Maximum size of a media upload, such as "1MB", "2GB" or "3TB".
    #[serde(rename="maxSize")]
    pub max_size: String,
    /// MIME Media Ranges for acceptable media uploads to this method.
    pub accept: Vec<String>,
    /// Supported upload protocols.
    pub protocols: RestMethodMediaUploadProtocols,
}

impl NestedType for RestMethodMediaUpload {}
impl Part for RestMethodMediaUpload {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list apis](struct.ApiListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct DirectoryList {
    /// The individual directory entries. One entry per api/version pair.
    pub items: Vec<DirectoryListItems>,
    /// Indicate the version of the Discovery API used to generate this doc.
    #[serde(rename="discoveryVersion")]
    pub discovery_version: String,
    /// The kind for this response.
    pub kind: String,
}

impl ResponseResult for DirectoryList {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct JsonSchema {
    /// A description of this object.
    pub description: String,
    /// An additional regular expression or key that helps constrain the value. For more details see: http://tools.ietf.org/html/draft-zyp-json-schema-03#section-5.23
    pub format: String,
    /// Values this parameter may take (if it is an enum).
    #[serde(rename="enum")]
    pub enum_: Vec<String>,
    /// In a variant data type, the value of one property is used to determine how to interpret the entire entity. Its value must exist in a map of descriminant values to schema names.
    pub variant: JsonSchemaVariant,
    /// The descriptions for the enums. Each position maps to the corresponding value in the "enum" array.
    #[serde(rename="enumDescriptions")]
    pub enum_descriptions: Vec<String>,
    /// The value is read-only, generated by the service. The value cannot be modified by the client. If the value is included in a POST, PUT, or PATCH request, it is ignored by the service.
    #[serde(rename="readOnly")]
    pub read_only: bool,
    /// The minimum value of this parameter.
    pub minimum: String,
    /// Whether this parameter may appear multiple times.
    pub repeated: bool,
    /// Unique identifier for this schema.
    pub id: String,
    /// A reference to another schema. The value of this property is the "id" of another schema.
    #[serde(rename="$ref")]
    pub ref_: String,
    /// The default value of this property (if one exists).
    pub default: String,
    /// If this is a schema for an array, this property is the schema for each element in the array.
    pub items: Option<Box<JsonSchema>>,
    /// Whether the parameter is required.
    pub required: bool,
    /// The maximum value of this parameter.
    pub maximum: String,
    /// If this is a schema for an object, list the schema for each property of this object.
    pub properties: HashMap<String, JsonSchema>,
    /// Whether this parameter goes in the query or the path for REST requests.
    pub location: String,
    /// The regular expression this parameter must conform to. Uses Java 6 regex format: http://docs.oracle.com/javase/6/docs/api/java/util/regex/Pattern.html
    pub pattern: String,
    /// If this is a schema for an object, this property is the schema for any additional properties with dynamic keys on this object.
    #[serde(rename="additionalProperties")]
    pub additional_properties: Option<Box<JsonSchema>>,
    /// The value type for this schema. A list of values can be found here: http://tools.ietf.org/html/draft-zyp-json-schema-03#section-5.1
    #[serde(rename="type")]
    pub type_: String,
    /// Additional information about this property.
    pub annotations: JsonSchemaAnnotations,
}

impl Part for JsonSchema {}


/// The individual directory entries. One entry per api/version pair.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct DirectoryListItems {
    /// The kind for this response.
    pub kind: String,
    /// The URL for the discovery REST document.
    #[serde(rename="discoveryRestUrl")]
    pub discovery_rest_url: String,
    /// The description of this API.
    pub description: String,
    /// Links to 16x16 and 32x32 icons representing the API.
    pub icons: DirectoryListItemsIcons,
    /// Labels for the status of this API, such as labs or deprecated.
    pub labels: Vec<String>,
    /// True if this version is the preferred version to use.
    pub preferred: bool,
    /// A link to the discovery document.
    #[serde(rename="discoveryLink")]
    pub discovery_link: String,
    /// The version of the API.
    pub version: String,
    /// The title of this API.
    pub title: String,
    /// A link to human readable documentation for the API.
    #[serde(rename="documentationLink")]
    pub documentation_link: String,
    /// The id of this API.
    pub id: String,
    /// The name of the API.
    pub name: String,
}

impl NestedType for DirectoryListItems {}
impl Part for DirectoryListItems {}


/// Authentication information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct RestDescriptionAuth {
    /// OAuth 2.0 authentication information.
    pub oauth2: RestDescriptionAuthOauth2,
}

impl NestedType for RestDescriptionAuth {}
impl Part for RestDescriptionAuth {}


/// The scope value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct RestDescriptionAuthOauth2Scopes {
    /// Description of scope.
    pub description: String,
}

impl NestedType for RestDescriptionAuthOauth2Scopes {}
impl Part for RestDescriptionAuthOauth2Scopes {}


/// Links to 16x16 and 32x32 icons representing the API.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct DirectoryListItemsIcons {
    /// The URL of the 32x32 icon.
    pub x32: String,
    /// The URL of the 16x16 icon.
    pub x16: String,
}

impl NestedType for DirectoryListItemsIcons {}
impl Part for DirectoryListItemsIcons {}


/// The schema for the request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct RestMethodRequest {
    /// parameter name.
    #[serde(rename="parameterName")]
    pub parameter_name: String,
    /// Schema ID for the request schema.
    #[serde(rename="$ref")]
    pub ref_: String,
}

impl NestedType for RestMethodRequest {}
impl Part for RestMethodRequest {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct RestResource {
    /// Methods on this resource.
    pub methods: HashMap<String, RestMethod>,
    /// Sub-resources on this resource.
    pub resources: HashMap<String, RestResource>,
}

impl Part for RestResource {}


/// Supports uploading as a single HTTP request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct RestMethodMediaUploadProtocolsSimple {
    /// The URI path to be used for upload. Should be used in conjunction with the basePath property at the api-level.
    pub path: String,
    /// True if this endpoint supports upload multipart media.
    pub multipart: bool,
}

impl NestedType for RestMethodMediaUploadProtocolsSimple {}
impl Part for RestMethodMediaUploadProtocolsSimple {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *api* resources.
/// It is not used directly, but through the `Discovery` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_discovery1 as discovery1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use discovery1::Discovery;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Discovery::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_rest(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.apis();
/// # }
/// ```
pub struct ApiMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Discovery<C, A>,
}

impl<'a, C, A> MethodsBuilder for ApiMethods<'a, C, A> {}

impl<'a, C, A> ApiMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieve the description of a particular version of an api.
    /// 
    /// # Arguments
    ///
    /// * `api` - The name of the API.
    /// * `version` - The version of the API.
    pub fn get_rest(&self, api: &str, version: &str) -> ApiGetRestCall<'a, C, A> {
        ApiGetRestCall {
            hub: self.hub,
            _api: api.to_string(),
            _version: version.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieve the list of APIs supported at this endpoint.
    pub fn list(&self) -> ApiListCall<'a, C, A> {
        ApiListCall {
            hub: self.hub,
            _preferred: Default::default(),
            _name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Retrieve the description of a particular version of an api.
///
/// A builder for the *getRest* method supported by a *api* resource.
/// It is not used directly, but through a `ApiMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_discovery1 as discovery1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use discovery1::Discovery;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Discovery::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.apis().get_rest("api", "version")
///              .doit();
/// # }
/// ```
pub struct ApiGetRestCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Discovery<C, A>,
    _api: String,
    _version: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for ApiGetRestCall<'a, C, A> {}

impl<'a, C, A> ApiGetRestCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, RestDescription)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "discovery.apis.getRest", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("api", self._api.to_string()));
        params.push(("version", self._version.to_string()));
        for &field in ["alt", "api", "version"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/discovery/v1/apis/{api}/{version}/rest".to_string();
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{api}", "api"), ("{version}", "version")].iter() {
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
            for param_name in ["api", "version"].iter() {
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
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()));

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


    /// Sets the *api* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The name of the API.
    pub fn api(mut self, new_value: &str) -> ApiGetRestCall<'a, C, A> {
        self._api = new_value.to_string();
        self
    }
    /// Sets the *version* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The version of the API.
    pub fn version(mut self, new_value: &str) -> ApiGetRestCall<'a, C, A> {
        self._version = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ApiGetRestCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> ApiGetRestCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Retrieve the list of APIs supported at this endpoint.
///
/// A builder for the *list* method supported by a *api* resource.
/// It is not used directly, but through a `ApiMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_discovery1 as discovery1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use discovery1::Discovery;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Discovery::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.apis().list()
///              .preferred(true)
///              .name("justo")
///              .doit();
/// # }
/// ```
pub struct ApiListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Discovery<C, A>,
    _preferred: Option<bool>,
    _name: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for ApiListCall<'a, C, A> {}

impl<'a, C, A> ApiListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, DirectoryList)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "discovery.apis.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        if let Some(value) = self._preferred {
            params.push(("preferred", value.to_string()));
        }
        if let Some(value) = self._name {
            params.push(("name", value.to_string()));
        }
        for &field in ["alt", "preferred", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/discovery/v1/apis".to_string();
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(Error::MissingAPIKey)
            }
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()));

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


    /// Sets the *preferred* query property to the given value.
    ///
    /// 
    /// Return only the preferred version of an API.
    pub fn preferred(mut self, new_value: bool) -> ApiListCall<'a, C, A> {
        self._preferred = Some(new_value);
        self
    }
    /// Sets the *name* query property to the given value.
    ///
    /// 
    /// Only include APIs with the given name.
    pub fn name(mut self, new_value: &str) -> ApiListCall<'a, C, A> {
        self._name = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ApiListCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> ApiListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


