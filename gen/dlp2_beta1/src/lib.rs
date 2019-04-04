// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *DLP* crate version *1.0.8+20171205*, where *20171205* is the exact revision of the *dlp:v2beta1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.8*.
//! 
//! Everything else about the *DLP* *v2_beta1* API can be found at the
//! [official documentation site](https://cloud.google.com/dlp/docs/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/dlp2_beta1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.DLP.html) ... 
//! 
//! * content
//!  * [*deidentify*](struct.ContentDeidentifyCall.html), [*inspect*](struct.ContentInspectCall.html) and [*redact*](struct.ContentRedactCall.html)
//! * data source
//!  * [*analyze*](struct.DataSourceAnalyzeCall.html)
//! * inspect
//!  * [*operations cancel*](struct.InspectOperationCancelCall.html), [*operations create*](struct.InspectOperationCreateCall.html), [*operations delete*](struct.InspectOperationDeleteCall.html), [*operations get*](struct.InspectOperationGetCall.html), [*operations list*](struct.InspectOperationListCall.html) and [*results findings list*](struct.InspectResultFindingListCall.html)
//! * risk analysis
//!  * [*operations cancel*](struct.RiskAnalysiOperationCancelCall.html), [*operations delete*](struct.RiskAnalysiOperationDeleteCall.html), [*operations get*](struct.RiskAnalysiOperationGetCall.html) and [*operations list*](struct.RiskAnalysiOperationListCall.html)
//! * root categories
//!  * [*info types list*](struct.RootCategoryInfoTypeListCall.html) and [*list*](struct.RootCategoryListCall.html)
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
//! * **[Hub](struct.DLP.html)**
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
//! let r = hub.data_source().analyze(...).doit()
//! let r = hub.risk_analysis().operations_get(...).doit()
//! let r = hub.inspect().operations_get(...).doit()
//! let r = hub.inspect().operations_create(...).doit()
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
//! google-dlp2_beta1 = "*"
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
//! extern crate google_dlp2_beta1 as dlp2_beta1;
//! use dlp2_beta1::GooglePrivacyDlpV2beta1AnalyzeDataSourceRiskRequest;
//! use dlp2_beta1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use dlp2_beta1::DLP;
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
//! let mut hub = DLP::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = GooglePrivacyDlpV2beta1AnalyzeDataSourceRiskRequest::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.data_source().analyze(req)
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

pub use cmn::{MultiPartReader, ToParts, MethodInfo, Result, Error, CallBuilder, Hub, ReadSeek, Part,
              ResponseResult, RequestValue, NestedType, Delegate, DefaultDelegate, MethodsBuilder,
              Resource, ErrorResponse, remove_json_null_values};


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

/// Central instance to access all DLP related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_dlp2_beta1 as dlp2_beta1;
/// use dlp2_beta1::GooglePrivacyDlpV2beta1AnalyzeDataSourceRiskRequest;
/// use dlp2_beta1::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use dlp2_beta1::DLP;
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
/// let mut hub = DLP::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GooglePrivacyDlpV2beta1AnalyzeDataSourceRiskRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.data_source().analyze(req)
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
pub struct DLP<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for DLP<C, A> {}

impl<'a, C, A> DLP<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> DLP<C, A> {
        DLP {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.8".to_string(),
            _base_url: "https://dlp.googleapis.com/".to_string(),
            _root_url: "https://dlp.googleapis.com/".to_string(),
        }
    }

    pub fn content(&'a self) -> ContentMethods<'a, C, A> {
        ContentMethods { hub: &self }
    }
    pub fn data_source(&'a self) -> DataSourceMethods<'a, C, A> {
        DataSourceMethods { hub: &self }
    }
    pub fn inspect(&'a self) -> InspectMethods<'a, C, A> {
        InspectMethods { hub: &self }
    }
    pub fn risk_analysis(&'a self) -> RiskAnalysiMethods<'a, C, A> {
        RiskAnalysiMethods { hub: &self }
    }
    pub fn root_categories(&'a self) -> RootCategoryMethods<'a, C, A> {
        RootCategoryMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.8`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://dlp.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://dlp.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// Location of a finding within a `ContentItem.Table`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1TableLocation {
    /// The zero-based index of the row where the finding is located.
    #[serde(rename="rowIndex")]
    pub row_index: Option<String>,
}

impl Part for GooglePrivacyDlpV2beta1TableLocation {}


/// Represents a time of day. The date and time zone are either not significant
/// or are specified elsewhere. An API may choose to allow leap seconds. Related
/// types are google.type.Date and `google.protobuf.Timestamp`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeTimeOfDay {
    /// Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999.
    pub nanos: Option<i32>,
    /// Seconds of minutes of the time. Must normally be from 0 to 59. An API may
    /// allow the value 60 if it allows leap-seconds.
    pub seconds: Option<i32>,
    /// Hours of day in 24 hour format. Should be from 0 to 23. An API may choose
    /// to allow the value "24:00:00" for scenarios like business closing time.
    pub hours: Option<i32>,
    /// Minutes of hour of day. Must be from 0 to 59.
    pub minutes: Option<i32>,
}

impl Part for GoogleTypeTimeOfDay {}


/// Info Type Category description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1CategoryDescription {
    /// Human readable form of the category name.
    #[serde(rename="displayName")]
    pub display_name: Option<String>,
    /// Internal name of the category.
    pub name: Option<String>,
}

impl Part for GooglePrivacyDlpV2beta1CategoryDescription {}


/// A collection that informs the user the number of times a particular
/// `TransformationResultCode` and error details occurred.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1SummaryResult {
    /// no description provided
    pub count: Option<String>,
    /// no description provided
    pub code: Option<String>,
    /// A place for warnings or errors to show up if a transformation didn't
    /// work as expected.
    pub details: Option<String>,
}

impl Part for GooglePrivacyDlpV2beta1SummaryResult {}


/// This is a data encryption key (DEK) (as opposed to
/// a key encryption key (KEK) stored by KMS).
/// When using KMS to wrap/unwrap DEKs, be sure to set an appropriate
/// IAM policy on the KMS CryptoKey (KEK) to ensure an attacker cannot
/// unwrap the data crypto key.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1CryptoKey {
    /// no description provided
    #[serde(rename="kmsWrapped")]
    pub kms_wrapped: Option<GooglePrivacyDlpV2beta1KmsWrappedCryptoKey>,
    /// no description provided
    pub unwrapped: Option<GooglePrivacyDlpV2beta1UnwrappedCryptoKey>,
    /// no description provided
    pub transient: Option<GooglePrivacyDlpV2beta1TransientCryptoKey>,
}

impl Part for GooglePrivacyDlpV2beta1CryptoKey {}


/// A column with a semantic tag attached.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1TaggedField {
    /// Identifies the column. [required]
    pub field: Option<GooglePrivacyDlpV2beta1FieldId>,
    /// A column can be tagged with a custom tag. In this case, the user must
    /// indicate an auxiliary table that contains statistical information on
    /// the possible values of this column (below).
    #[serde(rename="customTag")]
    pub custom_tag: Option<String>,
    /// A column can be tagged with a InfoType to use the relevant public
    /// dataset as a statistical model of population, if available. We
    /// currently support US ZIP codes, region codes, ages and genders.
    #[serde(rename="infoType")]
    pub info_type: Option<GooglePrivacyDlpV2beta1InfoType>,
    /// If no semantic tag is indicated, we infer the statistical model from
    /// the distribution of values in the input data
    pub inferred: Option<GoogleProtobufEmpty>,
}

impl Part for GooglePrivacyDlpV2beta1TaggedField {}


/// Set of files to scan.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1FileSet {
    /// The url, in the format `gs://<bucket>/<path>`. Trailing wildcard in the
    /// path is allowed.
    pub url: Option<String>,
}

impl Part for GooglePrivacyDlpV2beta1FileSet {}


/// Buckets values based on fixed size ranges. The
/// Bucketing transformation can provide all of this functionality,
/// but requires more configuration. This message is provided as a convenience to
/// the user for simple bucketing strategies.
/// The resulting value will be a hyphenated string of
/// lower_bound-upper_bound.
/// This can be used on data of type: double, long.
/// If the bound Value type differs from the type of data
/// being transformed, we will first attempt converting the type of the data to
/// be transformed to match the type of the bound before comparing.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1FixedSizeBucketingConfig {
    /// Lower bound value of buckets. All values less than `lower_bound` are
    /// grouped together into a single bucket; for example if `lower_bound` = 10,
    /// then all values less than 10 are replaced with the value “-10”. [Required].
    #[serde(rename="lowerBound")]
    pub lower_bound: Option<GooglePrivacyDlpV2beta1Value>,
    /// Upper bound value of buckets. All values greater than upper_bound are
    /// grouped together into a single bucket; for example if `upper_bound` = 89,
    /// then all values greater than 89 are replaced with the value “89+”.
    /// [Required].
    #[serde(rename="upperBound")]
    pub upper_bound: Option<GooglePrivacyDlpV2beta1Value>,
    /// Size of each bucket (except for minimum and maximum buckets). So if
    /// `lower_bound` = 10, `upper_bound` = 89, and `bucket_size` = 10, then the
    /// following buckets would be used: -10, 10-20, 20-30, 30-40, 40-50, 50-60,
    /// 60-70, 70-80, 80-89, 89+. Precision up to 2 decimals works. [Required].
    #[serde(rename="bucketSize")]
    pub bucket_size: Option<f64>,
}

impl Part for GooglePrivacyDlpV2beta1FixedSizeBucketingConfig {}


/// Request to de-identify a list of items.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [deidentify content](struct.ContentDeidentifyCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1DeidentifyContentRequest {
    /// The list of items to inspect. Up to 100 are allowed per request.
    /// All items will be treated as text/*.
    pub items: Option<Vec<GooglePrivacyDlpV2beta1ContentItem>>,
    /// Configuration for the inspector.
    #[serde(rename="inspectConfig")]
    pub inspect_config: Option<GooglePrivacyDlpV2beta1InspectConfig>,
    /// Configuration for the de-identification of the list of content items.
    #[serde(rename="deidentifyConfig")]
    pub deidentify_config: Option<GooglePrivacyDlpV2beta1DeidentifyConfig>,
}

impl RequestValue for GooglePrivacyDlpV2beta1DeidentifyContentRequest {}


/// Results of redacting a list of items.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [redact content](struct.ContentRedactCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1RedactContentResponse {
    /// The redacted content.
    pub items: Option<Vec<GooglePrivacyDlpV2beta1ContentItem>>,
}

impl ResponseResult for GooglePrivacyDlpV2beta1RedactContentResponse {}


/// Reidentifiability metric. This corresponds to a risk model similar to what
/// is called "journalist risk" in the literature, except the attack dataset is
/// statistically modeled instead of being perfectly known. This can be done
/// using publicly available data (like the US Census), or using a custom
/// statistical model (indicated as one or several BigQuery tables), or by
/// extrapolating from the distribution of values in the input dataset.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1KMapEstimationConfig {
    /// ISO 3166-1 alpha-2 region code to use in the statistical modeling.
    /// Required if no column is tagged with a region-specific InfoType (like
    /// US_ZIP_5) or a region code.
    #[serde(rename="regionCode")]
    pub region_code: Option<String>,
    /// Fields considered to be quasi-identifiers. No two columns can have the
    /// same tag. [required]
    #[serde(rename="quasiIds")]
    pub quasi_ids: Option<Vec<GooglePrivacyDlpV2beta1TaggedField>>,
    /// Several auxiliary tables can be used in the analysis. Each custom_tag
    /// used to tag a quasi-identifiers column must appear in exactly one column
    /// of one auxiliary table.
    #[serde(rename="auxiliaryTables")]
    pub auxiliary_tables: Option<Vec<GooglePrivacyDlpV2beta1AuxiliaryTable>>,
}

impl Part for GooglePrivacyDlpV2beta1KMapEstimationConfig {}


/// The transformation to apply to the field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1FieldTransformation {
    /// Treat the contents of the field as free text, and selectively
    /// transform content that matches an `InfoType`.
    #[serde(rename="infoTypeTransformations")]
    pub info_type_transformations: Option<GooglePrivacyDlpV2beta1InfoTypeTransformations>,
    /// Apply the transformation to the entire field.
    #[serde(rename="primitiveTransformation")]
    pub primitive_transformation: Option<GooglePrivacyDlpV2beta1PrimitiveTransformation>,
    /// Only apply the transformation if the condition evaluates to true for the
    /// given `RecordCondition`. The conditions are allowed to reference fields
    /// that are not used in the actual transformation. [optional]
    /// 
    /// Example Use Cases:
    /// 
    /// - Apply a different bucket transformation to an age column if the zip code
    /// column for the same record is within a specific range.
    /// - Redact a field if the date of birth field is greater than 85.
    pub condition: Option<GooglePrivacyDlpV2beta1RecordCondition>,
    /// Input field(s) to apply the transformation to. [required]
    pub fields: Option<Vec<GooglePrivacyDlpV2beta1FieldId>>,
}

impl Part for GooglePrivacyDlpV2beta1FieldTransformation {}


/// Summary of a single tranformation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1TransformationSummary {
    /// Set if the transformation was limited to a specific FieldId.
    pub field: Option<GooglePrivacyDlpV2beta1FieldId>,
    /// The field transformation that was applied. This list will contain
    /// multiple only in the case of errors.
    #[serde(rename="fieldTransformations")]
    pub field_transformations: Option<Vec<GooglePrivacyDlpV2beta1FieldTransformation>>,
    /// Set if the transformation was limited to a specific info_type.
    #[serde(rename="infoType")]
    pub info_type: Option<GooglePrivacyDlpV2beta1InfoType>,
    /// The specific suppression option these stats apply to.
    #[serde(rename="recordSuppress")]
    pub record_suppress: Option<GooglePrivacyDlpV2beta1RecordSuppression>,
    /// no description provided
    pub results: Option<Vec<GooglePrivacyDlpV2beta1SummaryResult>>,
    /// The specific transformation these stats apply to.
    pub transformation: Option<GooglePrivacyDlpV2beta1PrimitiveTransformation>,
}

impl Part for GooglePrivacyDlpV2beta1TransformationSummary {}


/// A collection of expressions
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1Expressions {
    /// no description provided
    pub conditions: Option<GooglePrivacyDlpV2beta1Conditions>,
    /// The operator to apply to the result of conditions. Default and currently
    /// only supported value is `AND`.
    #[serde(rename="logicalOperator")]
    pub logical_operator: Option<String>,
}

impl Part for GooglePrivacyDlpV2beta1Expressions {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1ReplaceConfig {
    /// Content replacing sensitive information of given type. Max 256 chars.
    #[serde(rename="replaceWith")]
    pub replace_with: Option<String>,
    /// Type of information to replace. Only one ReplaceConfig per info_type
    /// should be provided. If ReplaceConfig does not have an info_type, the DLP
    /// API matches it against all info_types that are found but not specified in
    /// another ReplaceConfig.
    #[serde(rename="infoType")]
    pub info_type: Option<GooglePrivacyDlpV2beta1InfoType>,
}

impl Part for GooglePrivacyDlpV2beta1ReplaceConfig {}


/// Configuration for determining how redaction of images should occur.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1ImageRedactionConfig {
    /// If true, all text found in the image, regardless whether it matches an
    /// info_type, is redacted.
    #[serde(rename="redactAllText")]
    pub redact_all_text: Option<bool>,
    /// Only one per info_type should be provided per request. If not
    /// specified, and redact_all_text is false, the DLP API will redact all
    /// text that it matches against all info_types that are found, but not
    /// specified in another ImageRedactionConfig.
    #[serde(rename="infoType")]
    pub info_type: Option<GooglePrivacyDlpV2beta1InfoType>,
    /// The color to use when redacting content from an image. If not specified,
    /// the default is black.
    #[serde(rename="redactionColor")]
    pub redaction_color: Option<GooglePrivacyDlpV2beta1Color>,
}

impl Part for GooglePrivacyDlpV2beta1ImageRedactionConfig {}


/// The `Status` type defines a logical error model that is suitable for different
/// programming environments, including REST APIs and RPC APIs. It is used by
/// [gRPC](https://github.com/grpc). The error model is designed to be:
/// 
/// - Simple to use and understand for most users
/// - Flexible enough to meet unexpected needs
/// 
/// # Overview
/// 
/// The `Status` message contains three pieces of data: error code, error message,
/// and error details. The error code should be an enum value of
/// google.rpc.Code, but it may accept additional error codes if needed.  The
/// error message should be a developer-facing English message that helps
/// developers *understand* and *resolve* the error. If a localized user-facing
/// error message is needed, put the localized message in the error details or
/// localize it in the client. The optional error details may contain arbitrary
/// information about the error. There is a predefined set of error detail types
/// in the package `google.rpc` that can be used for common error conditions.
/// 
/// # Language mapping
/// 
/// The `Status` message is the logical representation of the error model, but it
/// is not necessarily the actual wire format. When the `Status` message is
/// exposed in different client libraries and different wire protocols, it can be
/// mapped differently. For example, it will likely be mapped to some exceptions
/// in Java, but more likely mapped to some error codes in C.
/// 
/// # Other uses
/// 
/// The error model and the `Status` message can be used in a variety of
/// environments, either with or without APIs, to provide a
/// consistent developer experience across different environments.
/// 
/// Example uses of this error model include:
/// 
/// - Partial errors. If a service needs to return partial errors to the client,
///     it may embed the `Status` in the normal response to indicate the partial
///     errors.
/// 
/// - Workflow errors. A typical workflow has multiple steps. Each step may
///     have a `Status` message for error reporting.
/// 
/// - Batch operations. If a client uses batch request and batch response, the
///     `Status` message should be used directly inside batch response, one for
///     each error sub-response.
/// 
/// - Asynchronous operations. If an API call embeds asynchronous operation
///     results in its response, the status of those operations should be
///     represented directly using the `Status` message.
/// 
/// - Logging. If some API errors are stored in logs, the message `Status` could
///     be used directly after any stripping needed for security/privacy reasons.
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


/// Message defining the location of a BigQuery table. A table is uniquely
/// identified  by its project_id, dataset_id, and table_name. Within a query
/// a table is often referenced with a string in the format of:
/// `<project_id>:<dataset_id>.<table_id>` or
/// `<project_id>.<dataset_id>.<table_id>`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1BigQueryTable {
    /// The Google Cloud Platform project ID of the project containing the table.
    /// If omitted, project ID is inferred from the API call.
    #[serde(rename="projectId")]
    pub project_id: Option<String>,
    /// Name of the table.
    #[serde(rename="tableId")]
    pub table_id: Option<String>,
    /// Dataset ID of the table.
    #[serde(rename="datasetId")]
    pub dataset_id: Option<String>,
}

impl Part for GooglePrivacyDlpV2beta1BigQueryTable {}


/// Buckets represented as ranges, along with replacement values. Ranges must
/// be non-overlapping.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1Bucket {
    /// Upper bound of the range, exclusive; type must match min.
    pub max: Option<GooglePrivacyDlpV2beta1Value>,
    /// Replacement value for this bucket. If not provided
    /// the default behavior will be to hyphenate the min-max range.
    #[serde(rename="replacementValue")]
    pub replacement_value: Option<GooglePrivacyDlpV2beta1Value>,
    /// Lower bound of the range, inclusive. Type should be the same as max if
    /// used.
    pub min: Option<GooglePrivacyDlpV2beta1Value>,
}

impl Part for GooglePrivacyDlpV2beta1Bucket {}


/// k-anonymity metric, used for analysis of reidentification risk.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1KAnonymityConfig {
    /// Optional message indicating that each distinct entity_id should not
    /// contribute to the k-anonymity count more than once per equivalence class.
    /// If an entity_id appears on several rows with different quasi-identifier
    /// tuples, it will contribute to each count exactly once.
    /// 
    /// This can lead to unexpected results. Consider a table where ID 1 is
    /// associated to quasi-identifier "foo", ID 2 to "bar", and ID 3 to *both*
    /// quasi-identifiers "foo" and "bar" (on separate rows), and where this ID
    /// is used as entity_id. Then, the anonymity value associated to ID 3 will
    /// be 2, even if it is the only ID to be associated to both values "foo" and
    /// "bar".
    #[serde(rename="entityId")]
    pub entity_id: Option<GooglePrivacyDlpV2beta1EntityId>,
    /// Set of fields to compute k-anonymity over. When multiple fields are
    /// specified, they are considered a single composite key. Structs and
    /// repeated data types are not supported; however, nested fields are
    /// supported so long as they are not structs themselves or nested within
    /// a repeated field.
    #[serde(rename="quasiIds")]
    pub quasi_ids: Option<Vec<GooglePrivacyDlpV2beta1FieldId>>,
}

impl Part for GooglePrivacyDlpV2beta1KAnonymityConfig {}


/// Configuration description of the scanning process.
/// When used with redactContent only info_types and min_likelihood are currently
/// used.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1InspectConfig {
    /// When true, excludes type information of the findings.
    #[serde(rename="excludeTypes")]
    pub exclude_types: Option<bool>,
    /// Only returns findings equal or above this threshold.
    #[serde(rename="minLikelihood")]
    pub min_likelihood: Option<String>,
    /// Configuration of findings limit given for specified info types.
    #[serde(rename="infoTypeLimits")]
    pub info_type_limits: Option<Vec<GooglePrivacyDlpV2beta1InfoTypeLimit>>,
    /// Custom info types provided by the user.
    #[serde(rename="customInfoTypes")]
    pub custom_info_types: Option<Vec<GooglePrivacyDlpV2beta1CustomInfoType>>,
    /// When true, a contextual quote from the data that triggered a finding is
    /// included in the response; see Finding.quote.
    #[serde(rename="includeQuote")]
    pub include_quote: Option<bool>,
    /// Restricts what info_types to look for. The values must correspond to
    /// InfoType values returned by ListInfoTypes or found in documentation.
    /// Empty info_types runs all enabled detectors.
    #[serde(rename="infoTypes")]
    pub info_types: Option<Vec<GooglePrivacyDlpV2beta1InfoType>>,
    /// Limits the number of findings per content item or long running operation.
    #[serde(rename="maxFindings")]
    pub max_findings: Option<i32>,
}

impl Part for GooglePrivacyDlpV2beta1InspectConfig {}


/// Datastore partition ID.
/// A partition ID identifies a grouping of entities. The grouping is always
/// by project and namespace, however the namespace ID may be empty.
/// 
/// A partition ID contains several dimensions:
/// project ID and namespace ID.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1PartitionId {
    /// The ID of the project to which the entities belong.
    #[serde(rename="projectId")]
    pub project_id: Option<String>,
    /// If not empty, the ID of the namespace to which the entities belong.
    #[serde(rename="namespaceId")]
    pub namespace_id: Option<String>,
}

impl Part for GooglePrivacyDlpV2beta1PartitionId {}


/// Request to search for potentially sensitive info in a list of items
/// and replace it with a default or provided content.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [redact content](struct.ContentRedactCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1RedactContentRequest {
    /// The list of items to inspect. Up to 100 are allowed per request.
    pub items: Option<Vec<GooglePrivacyDlpV2beta1ContentItem>>,
    /// The strings to replace findings text findings with. Must specify at least
    /// one of these or one ImageRedactionConfig if redacting images.
    #[serde(rename="replaceConfigs")]
    pub replace_configs: Option<Vec<GooglePrivacyDlpV2beta1ReplaceConfig>>,
    /// Configuration for the inspector.
    #[serde(rename="inspectConfig")]
    pub inspect_config: Option<GooglePrivacyDlpV2beta1InspectConfig>,
    /// The configuration for specifying what content to redact from images.
    #[serde(rename="imageRedactionConfigs")]
    pub image_redaction_configs: Option<Vec<GooglePrivacyDlpV2beta1ImageRedactionConfig>>,
}

impl RequestValue for GooglePrivacyDlpV2beta1RedactContentRequest {}


/// The request message for Operations.CancelOperation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [operations cancel inspect](struct.InspectOperationCancelCall.html) (request)
/// * [operations cancel risk analysis](struct.RiskAnalysiOperationCancelCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleLongrunningCancelOperationRequest { _never_set: Option<bool> }

impl RequestValue for GoogleLongrunningCancelOperationRequest {}


/// Bounding box encompassing detected text within an image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1ImageLocation {
    /// Width of the bounding box in pixels.
    pub width: Option<i32>,
    /// Top coordinate of the bounding box. (0,0) is upper left.
    pub top: Option<i32>,
    /// Height of the bounding box in pixels.
    pub height: Option<i32>,
    /// Left coordinate of the bounding box. (0,0) is upper left.
    pub left: Option<i32>,
}

impl Part for GooglePrivacyDlpV2beta1ImageLocation {}


/// Custom information type provided by the user. Used to find domain-specific
/// sensitive information configurable to the data in question.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1CustomInfoType {
    /// Surrogate info type.
    #[serde(rename="surrogateType")]
    pub surrogate_type: Option<GooglePrivacyDlpV2beta1SurrogateType>,
    /// Info type configuration. All custom info types must have configurations
    /// that do not conflict with built-in info types or other custom info types.
    #[serde(rename="infoType")]
    pub info_type: Option<GooglePrivacyDlpV2beta1InfoType>,
    /// Dictionary-based custom info type.
    pub dictionary: Option<GooglePrivacyDlpV2beta1Dictionary>,
}

impl Part for GooglePrivacyDlpV2beta1CustomInfoType {}


/// Compute numerical stats over an individual column, including
/// min, max, and quantiles.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1NumericalStatsConfig {
    /// Field to compute numerical stats on. Supported types are
    /// integer, float, date, datetime, timestamp, time.
    pub field: Option<GooglePrivacyDlpV2beta1FieldId>,
}

impl Part for GooglePrivacyDlpV2beta1NumericalStatsConfig {}


/// Using raw keys is prone to security risks due to accidentally
/// leaking the key. Choose another type of key if possible.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1UnwrappedCryptoKey {
    /// The AES 128/192/256 bit key. [required]
    pub key: Option<String>,
}

impl Part for GooglePrivacyDlpV2beta1UnwrappedCryptoKey {}


/// Additional configuration for inspect long running operations.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1OperationConfig {
    /// Max number of findings per file, Datastore entity, or database row.
    #[serde(rename="maxItemFindings")]
    pub max_item_findings: Option<String>,
}

impl Part for GooglePrivacyDlpV2beta1OperationConfig {}


/// A rule for transforming a value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1PrimitiveTransformation {
    /// no description provided
    #[serde(rename="characterMaskConfig")]
    pub character_mask_config: Option<GooglePrivacyDlpV2beta1CharacterMaskConfig>,
    /// no description provided
    #[serde(rename="redactConfig")]
    pub redact_config: Option<GooglePrivacyDlpV2beta1RedactConfig>,
    /// no description provided
    #[serde(rename="replaceWithInfoTypeConfig")]
    pub replace_with_info_type_config: Option<GooglePrivacyDlpV2beta1ReplaceWithInfoTypeConfig>,
    /// no description provided
    #[serde(rename="fixedSizeBucketingConfig")]
    pub fixed_size_bucketing_config: Option<GooglePrivacyDlpV2beta1FixedSizeBucketingConfig>,
    /// no description provided
    #[serde(rename="timePartConfig")]
    pub time_part_config: Option<GooglePrivacyDlpV2beta1TimePartConfig>,
    /// no description provided
    #[serde(rename="cryptoHashConfig")]
    pub crypto_hash_config: Option<GooglePrivacyDlpV2beta1CryptoHashConfig>,
    /// no description provided
    #[serde(rename="bucketingConfig")]
    pub bucketing_config: Option<GooglePrivacyDlpV2beta1BucketingConfig>,
    /// no description provided
    #[serde(rename="cryptoReplaceFfxFpeConfig")]
    pub crypto_replace_ffx_fpe_config: Option<GooglePrivacyDlpV2beta1CryptoReplaceFfxFpeConfig>,
    /// no description provided
    #[serde(rename="replaceConfig")]
    pub replace_config: Option<GooglePrivacyDlpV2beta1ReplaceValueConfig>,
}

impl Part for GooglePrivacyDlpV2beta1PrimitiveTransformation {}


/// Custom information type based on a dictionary of words or phrases. This can
/// be used to match sensitive information specific to the data, such as a list
/// of employee IDs or job titles.
/// 
/// Dictionary words are case-insensitive and all characters other than letters
/// and digits in the unicode [Basic Multilingual
/// Plane](https://en.wikipedia.org/wiki/Plane_%28Unicode%29#Basic_Multilingual_Plane)
/// will be replaced with whitespace when scanning for matches, so the
/// dictionary phrase "Sam Johnson" will match all three phrases "sam johnson",
/// "Sam, Johnson", and "Sam (Johnson)". Additionally, the characters
/// surrounding any match must be of a different type than the adjacent
/// characters within the word, so letters must be next to non-letters and
/// digits next to non-digits. For example, the dictionary word "jen" will
/// match the first three letters of the text "jen123" but will return no
/// matches for "jennifer".
/// 
/// Dictionary words containing a large number of characters that are not
/// letters or digits may result in unexpected findings because such characters
/// are treated as whitespace.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1Dictionary {
    /// List of words or phrases to search for.
    #[serde(rename="wordList")]
    pub word_list: Option<GooglePrivacyDlpV2beta1WordList>,
}

impl Part for GooglePrivacyDlpV2beta1Dictionary {}


/// Results of de-identifying a list of items.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [deidentify content](struct.ContentDeidentifyCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1DeidentifyContentResponse {
    /// no description provided
    pub items: Option<Vec<GooglePrivacyDlpV2beta1ContentItem>>,
    /// A review of the transformations that took place for each item.
    pub summaries: Option<Vec<GooglePrivacyDlpV2beta1DeidentificationSummary>>,
}

impl ResponseResult for GooglePrivacyDlpV2beta1DeidentifyContentResponse {}


/// Type of information detected by the API.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1InfoType {
    /// Name of the information type.
    pub name: Option<String>,
}

impl Part for GooglePrivacyDlpV2beta1InfoType {}


/// Shared message indicating Cloud storage type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1StorageConfig {
    /// BigQuery options specification.
    #[serde(rename="bigQueryOptions")]
    pub big_query_options: Option<GooglePrivacyDlpV2beta1BigQueryOptions>,
    /// Google Cloud Datastore options specification.
    #[serde(rename="datastoreOptions")]
    pub datastore_options: Option<GooglePrivacyDlpV2beta1DatastoreOptions>,
    /// Google Cloud Storage options specification.
    #[serde(rename="cloudStorageOptions")]
    pub cloud_storage_options: Option<GooglePrivacyDlpV2beta1CloudStorageOptions>,
}

impl Part for GooglePrivacyDlpV2beta1StorageConfig {}


/// A representation of a Datastore property in a projection.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1Projection {
    /// The property to project.
    pub property: Option<GooglePrivacyDlpV2beta1PropertyReference>,
}

impl Part for GooglePrivacyDlpV2beta1Projection {}


/// A unique identifier for a Datastore entity.
/// If a key's partition ID or any of its path kinds or names are
/// reserved/read-only, the key is reserved/read-only.
/// A reserved/read-only key is forbidden in certain documented contexts.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1Key {
    /// The entity path.
    /// An entity path consists of one or more elements composed of a kind and a
    /// string or numerical identifier, which identify entities. The first
    /// element identifies a _root entity_, the second element identifies
    /// a _child_ of the root entity, the third element identifies a child of the
    /// second entity, and so forth. The entities identified by all prefixes of
    /// the path are called the element's _ancestors_.
    /// 
    /// A path can never be empty, and a path can have at most 100 elements.
    pub path: Option<Vec<GooglePrivacyDlpV2beta1PathElement>>,
    /// Entities are partitioned into subsets, currently identified by a project
    /// ID and namespace ID.
    /// Queries are scoped to a single partition.
    #[serde(rename="partitionId")]
    pub partition_id: Option<GooglePrivacyDlpV2beta1PartitionId>,
}

impl Part for GooglePrivacyDlpV2beta1Key {}


/// A type of transformation that will scan unstructured text and
/// apply various `PrimitiveTransformation`s to each finding, where the
/// transformation is applied to only values that were identified as a specific
/// info_type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1InfoTypeTransformations {
    /// Transformation for each info type. Cannot specify more than one
    /// for a given info type. [required]
    pub transformations: Option<Vec<GooglePrivacyDlpV2beta1InfoTypeTransformation>>,
}

impl Part for GooglePrivacyDlpV2beta1InfoTypeTransformations {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1Row {
    /// no description provided
    pub values: Option<Vec<GooglePrivacyDlpV2beta1Value>>,
}

impl Part for GooglePrivacyDlpV2beta1Row {}


/// High level summary of deidentification.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1DeidentificationSummary {
    /// Transformations applied to the dataset.
    #[serde(rename="transformationSummaries")]
    pub transformation_summaries: Option<Vec<GooglePrivacyDlpV2beta1TransformationSummary>>,
    /// Total size in bytes that were transformed in some way.
    #[serde(rename="transformedBytes")]
    pub transformed_bytes: Option<String>,
}

impl Part for GooglePrivacyDlpV2beta1DeidentificationSummary {}


/// Message for detecting output from deidentification transformations
/// such as
/// [`CryptoReplaceFfxFpeConfig`](/dlp/docs/reference/rest/v2beta1/content/deidentify#CryptoReplaceFfxFpeConfig).
/// These types of transformations are
/// those that perform pseudonymization, thereby producing a "surrogate" as
/// output. This should be used in conjunction with a field on the
/// transformation such as `surrogate_info_type`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1SurrogateType { _never_set: Option<bool> }

impl Part for GooglePrivacyDlpV2beta1SurrogateType {}


/// A location in Cloud Storage.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1CloudStoragePath {
    /// The url, in the format of `gs://bucket/<path>`.
    pub path: Option<String>,
}

impl Part for GooglePrivacyDlpV2beta1CloudStoragePath {}


/// For use with `Date`, `Timestamp`, and `TimeOfDay`, extract or preserve a
/// portion of the value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1TimePartConfig {
    /// no description provided
    #[serde(rename="partToExtract")]
    pub part_to_extract: Option<String>,
}

impl Part for GooglePrivacyDlpV2beta1TimePartConfig {}


/// Response for ListRootCategories request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list root categories](struct.RootCategoryListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1ListRootCategoriesResponse {
    /// List of all into type categories supported by the API.
    pub categories: Option<Vec<GooglePrivacyDlpV2beta1CategoryDescription>>,
}

impl ResponseResult for GooglePrivacyDlpV2beta1ListRootCategoriesResponse {}


/// Container structure describing a single finding within a string or image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1Finding {
    /// The specific string that may be potentially sensitive info.
    pub quote: Option<String>,
    /// Estimate of how likely it is that the info_type is correct.
    pub likelihood: Option<String>,
    /// The specific type of info the string might be.
    #[serde(rename="infoType")]
    pub info_type: Option<GooglePrivacyDlpV2beta1InfoType>,
    /// Location of the info found.
    pub location: Option<GooglePrivacyDlpV2beta1Location>,
    /// Timestamp when finding was detected.
    #[serde(rename="createTime")]
    pub create_time: Option<String>,
}

impl Part for GooglePrivacyDlpV2beta1Finding {}


/// A reference to a property relative to the Datastore kind expressions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1PropertyReference {
    /// The name of the property.
    /// If name includes "."s, it may be interpreted as a property name path.
    pub name: Option<String>,
}

impl Part for GooglePrivacyDlpV2beta1PropertyReference {}


/// Message for a unique key indicating a record that contains a finding.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1RecordKey {
    /// no description provided
    #[serde(rename="cloudStorageKey")]
    pub cloud_storage_key: Option<GooglePrivacyDlpV2beta1CloudStorageKey>,
    /// no description provided
    #[serde(rename="datastoreKey")]
    pub datastore_key: Option<GooglePrivacyDlpV2beta1DatastoreKey>,
}

impl Part for GooglePrivacyDlpV2beta1RecordKey {}


/// General identifier of a data field in a storage service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1FieldId {
    /// Name describing the field.
    #[serde(rename="columnName")]
    pub column_name: Option<String>,
}

impl Part for GooglePrivacyDlpV2beta1FieldId {}


/// Partially mask a string by replacing a given number of characters with a
/// fixed character. Masking can start from the beginning or end of the string.
/// This can be used on data of any type (numbers, longs, and so on) and when
/// de-identifying structured data we'll attempt to preserve the original data's
/// type. (This allows you to take a long like 123 and modify it to a string like
/// **3.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1CharacterMaskConfig {
    /// When masking a string, items in this list will be skipped when replacing.
    /// For example, if your string is 555-555-5555 and you ask us to skip `-` and
    /// mask 5 chars with * we would produce ***-*55-5555.
    #[serde(rename="charactersToIgnore")]
    pub characters_to_ignore: Option<Vec<GooglePrivacyDlpV2beta1CharsToIgnore>>,
    /// Number of characters to mask. If not set, all matching chars will be
    /// masked. Skipped characters do not count towards this tally.
    #[serde(rename="numberToMask")]
    pub number_to_mask: Option<i32>,
    /// Character to mask the sensitive values&mdash;for example, "*" for an
    /// alphabetic string such as name, or "0" for a numeric string such as ZIP
    /// code or credit card number. String must have length 1. If not supplied, we
    /// will default to "*" for strings, 0 for digits.
    #[serde(rename="maskingCharacter")]
    pub masking_character: Option<String>,
    /// Mask characters in reverse order. For example, if `masking_character` is
    /// '0', number_to_mask is 14, and `reverse_order` is false, then
    /// 1234-5678-9012-3456 -> 00000000000000-3456
    /// If `masking_character` is '*', `number_to_mask` is 3, and `reverse_order`
    /// is true, then 12345 -> 12***
    #[serde(rename="reverseOrder")]
    pub reverse_order: Option<bool>,
}

impl Part for GooglePrivacyDlpV2beta1CharacterMaskConfig {}


/// Redact a given value. For example, if used with an `InfoTypeTransformation`
/// transforming PHONE_NUMBER, and input 'My phone number is 206-555-0123', the
/// output would be 'My phone number is '.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1RedactConfig { _never_set: Option<bool> }

impl Part for GooglePrivacyDlpV2beta1RedactConfig {}


/// Message defining a list of words or phrases to search for in the data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1WordList {
    /// Words or phrases defining the dictionary. The dictionary must contain
    /// at least one phrase and every phrase must contain at least 2 characters
    /// that are letters or digits. [required]
    pub words: Option<Vec<String>>,
}

impl Part for GooglePrivacyDlpV2beta1WordList {}


/// Replace each input value with a given `Value`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1ReplaceValueConfig {
    /// Value to replace it with.
    #[serde(rename="newValue")]
    pub new_value: Option<GooglePrivacyDlpV2beta1Value>,
}

impl Part for GooglePrivacyDlpV2beta1ReplaceValueConfig {}


/// A (kind, ID/name) pair used to construct a key path.
/// 
/// If either name or ID is set, the element is complete.
/// If neither is set, the element is incomplete.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1PathElement {
    /// The kind of the entity.
    /// A kind matching regex `__.*__` is reserved/read-only.
    /// A kind must not contain more than 1500 bytes when UTF-8 encoded.
    /// Cannot be `""`.
    pub kind: Option<String>,
    /// The auto-allocated ID of the entity.
    /// Never equal to zero. Values less than zero are discouraged and may not
    /// be supported in the future.
    pub id: Option<String>,
    /// The name of the entity.
    /// A name matching regex `__.*__` is reserved/read-only.
    /// A name must not be more than 1500 bytes when UTF-8 encoded.
    /// Cannot be `""`.
    pub name: Option<String>,
}

impl Part for GooglePrivacyDlpV2beta1PathElement {}


/// Represents a whole calendar date, e.g. date of birth. The time of day and
/// time zone are either specified elsewhere or are not significant. The date
/// is relative to the Proleptic Gregorian Calendar. The day may be 0 to
/// represent a year and month where the day is not significant, e.g. credit card
/// expiration date. The year may be 0 to represent a month and day independent
/// of year, e.g. anniversary date. Related types are google.type.TimeOfDay
/// and `google.protobuf.Timestamp`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeDate {
    /// Month of year. Must be from 1 to 12.
    pub month: Option<i32>,
    /// Day of month. Must be from 1 to 31 and valid for the year and month, or 0
    /// if specifying a year/month where the day is not significant.
    pub day: Option<i32>,
    /// Year of date. Must be from 1 to 9999, or 0 if specifying a date without
    /// a year.
    pub year: Option<i32>,
}

impl Part for GoogleTypeDate {}


/// Options defining a file or a set of files (path ending with *) within
/// a Google Cloud Storage bucket.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1CloudStorageOptions {
    /// no description provided
    #[serde(rename="fileSet")]
    pub file_set: Option<GooglePrivacyDlpV2beta1FileSet>,
}

impl Part for GooglePrivacyDlpV2beta1CloudStorageOptions {}


/// All the findings for a single scanned item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1InspectResult {
    /// If true, then this item might have more findings than were returned,
    /// and the findings returned are an arbitrary subset of all findings.
    /// The findings list might be truncated because the input items were too
    /// large, or because the server reached the maximum amount of resources
    /// allowed for a single API call. For best results, divide the input into
    /// smaller batches.
    #[serde(rename="findingsTruncated")]
    pub findings_truncated: Option<bool>,
    /// List of findings for an item.
    pub findings: Option<Vec<GooglePrivacyDlpV2beta1Finding>>,
}

impl Part for GooglePrivacyDlpV2beta1InspectResult {}


/// Replaces an identifier with a surrogate using FPE with the FFX
/// mode of operation.
/// The identifier must be representable by the US-ASCII character set.
/// For a given crypto key and context, the same identifier will be
/// replaced with the same surrogate.
/// Identifiers must be at least two characters long.
/// In the case that the identifier is the empty string, it will be skipped.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1CryptoReplaceFfxFpeConfig {
    /// The key used by the encryption algorithm. [required]
    #[serde(rename="cryptoKey")]
    pub crypto_key: Option<GooglePrivacyDlpV2beta1CryptoKey>,
    /// This is supported by mapping these to the alphanumeric characters
    /// that the FFX mode natively supports. This happens before/after
    /// encryption/decryption.
    /// Each character listed must appear only once.
    /// Number of characters must be in the range [2, 62].
    /// This must be encoded as ASCII.
    /// The order of characters does not matter.
    #[serde(rename="customAlphabet")]
    pub custom_alphabet: Option<String>,
    /// The native way to select the alphabet. Must be in the range [2, 62].
    pub radix: Option<i32>,
    /// A context may be used for higher security since the same
    /// identifier in two different contexts likely will be given a distinct
    /// surrogate. The principle is that the likeliness is inversely related
    /// to the ratio of the number of distinct identifiers per context over the
    /// number of possible surrogates: As long as this ratio is small, the
    /// likehood is large.
    /// 
    /// If the context is not set, a default tweak will be used.
    /// If the context is set but:
    /// 
    /// 1. there is no record present when transforming a given value or
    /// 1. the field is not present when transforming a given value,
    /// 
    /// a default tweak will be used.
    /// 
    /// Note that case (1) is expected when an `InfoTypeTransformation` is
    /// applied to both structured and non-structured `ContentItem`s.
    /// Currently, the referenced field may be of value type integer or string.
    /// 
    /// The tweak is constructed as a sequence of bytes in big endian byte order
    /// such that:
    /// 
    /// - a 64 bit integer is encoded followed by a single byte of value 1
    /// - a string is encoded in UTF-8 format followed by a single byte of value 2
    /// 
    /// This is also known as the 'tweak', as in tweakable encryption.
    pub context: Option<GooglePrivacyDlpV2beta1FieldId>,
    /// The custom info type to annotate the surrogate with.
    /// This annotation will be applied to the surrogate by prefixing it with
    /// the name of the custom info type followed by the number of
    /// characters comprising the surrogate. The following scheme defines the
    /// format: info_type_name(surrogate_character_count):surrogate
    /// 
    /// For example, if the name of custom info type is 'MY_TOKEN_INFO_TYPE' and
    /// the surrogate is 'abc', the full replacement value
    /// will be: 'MY_TOKEN_INFO_TYPE(3):abc'
    /// 
    /// This annotation identifies the surrogate when inspecting content using the
    /// custom info type
    /// [`SurrogateType`](/dlp/docs/reference/rest/v2beta1/InspectConfig#surrogatetype).
    /// This facilitates reversal of the surrogate when it occurs in free text.
    /// 
    /// In order for inspection to work properly, the name of this info type must
    /// not occur naturally anywhere in your data; otherwise, inspection may
    /// find a surrogate that does not correspond to an actual identifier.
    /// Therefore, choose your custom info type name carefully after considering
    /// what your data looks like. One way to select a name that has a high chance
    /// of yielding reliable detection is to include one or more unicode characters
    /// that are highly improbable to exist in your data.
    /// For example, assuming your data is entered from a regular ASCII keyboard,
    /// the symbol with the hex code point 29DD might be used like so:
    /// ⧝MY_TOKEN_TYPE
    #[serde(rename="surrogateInfoType")]
    pub surrogate_info_type: Option<GooglePrivacyDlpV2beta1InfoType>,
    /// no description provided
    #[serde(rename="commonAlphabet")]
    pub common_alphabet: Option<String>,
}

impl Part for GooglePrivacyDlpV2beta1CryptoReplaceFfxFpeConfig {}


/// Record key for a finding in a Cloud Storage file.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1CloudStorageKey {
    /// Byte offset of the referenced data in the file.
    #[serde(rename="startOffset")]
    pub start_offset: Option<String>,
    /// Path to the file.
    #[serde(rename="filePath")]
    pub file_path: Option<String>,
}

impl Part for GooglePrivacyDlpV2beta1CloudStorageKey {}


/// Results of inspecting a list of items.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [inspect content](struct.ContentInspectCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1InspectContentResponse {
    /// Each content_item from the request has a result in this list, in the
    /// same order as the request.
    pub results: Option<Vec<GooglePrivacyDlpV2beta1InspectResult>>,
}

impl ResponseResult for GooglePrivacyDlpV2beta1InspectContentResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1Conditions {
    /// no description provided
    pub conditions: Option<Vec<GooglePrivacyDlpV2beta1Condition>>,
}

impl Part for GooglePrivacyDlpV2beta1Conditions {}


/// Represents a color in the RGB color space.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1Color {
    /// The amount of blue in the color as a value in the interval [0, 1].
    pub blue: Option<f32>,
    /// The amount of green in the color as a value in the interval [0, 1].
    pub green: Option<f32>,
    /// The amount of red in the color as a value in the interval [0, 1].
    pub red: Option<f32>,
}

impl Part for GooglePrivacyDlpV2beta1Color {}


/// Generalization function that buckets values based on ranges. The ranges and
/// replacement values are dynamically provided by the user for custom behavior,
/// such as 1-30 -> LOW 31-65 -> MEDIUM 66-100 -> HIGH
/// This can be used on
/// data of type: number, long, string, timestamp.
/// If the bound `Value` type differs from the type of data being transformed, we
/// will first attempt converting the type of the data to be transformed to match
/// the type of the bound before comparing.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1BucketingConfig {
    /// no description provided
    pub buckets: Option<Vec<GooglePrivacyDlpV2beta1Bucket>>,
}

impl Part for GooglePrivacyDlpV2beta1BucketingConfig {}


/// The configuration that controls how the data will change.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1DeidentifyConfig {
    /// Treat the dataset as free-form text and apply the same free text
    /// transformation everywhere.
    #[serde(rename="infoTypeTransformations")]
    pub info_type_transformations: Option<GooglePrivacyDlpV2beta1InfoTypeTransformations>,
    /// Treat the dataset as structured. Transformations can be applied to
    /// specific locations within structured datasets, such as transforming
    /// a column within a table.
    #[serde(rename="recordTransformations")]
    pub record_transformations: Option<GooglePrivacyDlpV2beta1RecordTransformations>,
}

impl Part for GooglePrivacyDlpV2beta1DeidentifyConfig {}


/// Use this to have a random data crypto key generated.
/// It will be discarded after the operation/request finishes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1TransientCryptoKey {
    /// Name of the key. [required]
    /// This is an arbitrary string used to differentiate different keys.
    /// A unique key is generated per name: two separate `TransientCryptoKey`
    /// protos share the same generated key if their names are the same.
    /// When the data crypto key is generated, this name is not used in any way
    /// (repeating the api call will result in a different key being generated).
    pub name: Option<String>,
}

impl Part for GooglePrivacyDlpV2beta1TransientCryptoKey {}


/// Cloud repository for storing output.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1OutputStorageConfig {
    /// Store findings in a new table in the dataset.
    pub table: Option<GooglePrivacyDlpV2beta1BigQueryTable>,
    /// The path to a Google Cloud Storage location to store output.
    /// The bucket must already exist and
    /// the Google APIs service account for DLP must have write permission to
    /// write to the given bucket.
    /// Results are split over multiple csv files with each file name matching
    /// the pattern "[operation_id]_[count].csv", for example
    /// `3094877188788974909_1.csv`. The `operation_id` matches the
    /// identifier for the Operation, and the `count` is a counter used for
    /// tracking the number of files written.
    /// 
    /// The CSV file(s) contain the following columns regardless of storage type
    /// scanned:
    /// - id
    /// - info_type
    /// - likelihood
    /// - byte size of finding
    /// - quote
    /// - timestamp
    /// 
    /// For Cloud Storage the next columns are:
    /// 
    /// - file_path
    /// - start_offset
    /// 
    /// For Cloud Datastore the next columns are:
    /// 
    /// - project_id
    /// - namespace_id
    /// - path
    /// - column_name
    /// - offset
    /// 
    /// For BigQuery the next columns are:
    /// 
    /// - row_number
    /// - project_id
    /// - dataset_id
    /// - table_id
    #[serde(rename="storagePath")]
    pub storage_path: Option<GooglePrivacyDlpV2beta1CloudStoragePath>,
}

impl Part for GooglePrivacyDlpV2beta1OutputStorageConfig {}


/// A quasi-identifier column has a custom_tag, used to know which column
/// in the data corresponds to which column in the statistical model.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1QuasiIdField {
    /// no description provided
    pub field: Option<GooglePrivacyDlpV2beta1FieldId>,
    /// no description provided
    #[serde(rename="customTag")]
    pub custom_tag: Option<String>,
}

impl Part for GooglePrivacyDlpV2beta1QuasiIdField {}


/// A generic empty message that you can re-use to avoid defining duplicated
/// empty messages in your APIs. A typical example is to use it as the request
/// or the response type of an API method. For instance:
/// 
///     service Foo {
///       rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty);
///     }
/// 
/// The JSON representation for `Empty` is empty JSON object `{}`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [operations cancel inspect](struct.InspectOperationCancelCall.html) (response)
/// * [operations delete inspect](struct.InspectOperationDeleteCall.html) (response)
/// * [operations cancel risk analysis](struct.RiskAnalysiOperationCancelCall.html) (response)
/// * [operations delete risk analysis](struct.RiskAnalysiOperationDeleteCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleProtobufEmpty { _never_set: Option<bool> }

impl ResponseResult for GoogleProtobufEmpty {}


/// Pseudonymization method that generates surrogates via cryptographic hashing.
/// Uses SHA-256.
/// Outputs a 32 byte digest as an uppercase hex string
/// (for example, 41D1567F7F99F1DC2A5FAB886DEE5BEE).
/// Currently, only string and integer values can be hashed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1CryptoHashConfig {
    /// The key used by the hash function.
    #[serde(rename="cryptoKey")]
    pub crypto_key: Option<GooglePrivacyDlpV2beta1CryptoKey>,
}

impl Part for GooglePrivacyDlpV2beta1CryptoHashConfig {}


/// l-diversity metric, used for analysis of reidentification risk.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1LDiversityConfig {
    /// Sensitive field for computing the l-value.
    #[serde(rename="sensitiveAttribute")]
    pub sensitive_attribute: Option<GooglePrivacyDlpV2beta1FieldId>,
    /// Set of quasi-identifiers indicating how equivalence classes are
    /// defined for the l-diversity computation. When multiple fields are
    /// specified, they are considered a single composite key.
    #[serde(rename="quasiIds")]
    pub quasi_ids: Option<Vec<GooglePrivacyDlpV2beta1FieldId>>,
}

impl Part for GooglePrivacyDlpV2beta1LDiversityConfig {}


/// Request for scheduling a scan of a data subset from a Google Platform data
/// repository.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [operations create inspect](struct.InspectOperationCreateCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1CreateInspectOperationRequest {
    /// Optional location to store findings.
    #[serde(rename="outputConfig")]
    pub output_config: Option<GooglePrivacyDlpV2beta1OutputStorageConfig>,
    /// Configuration for the inspector.
    #[serde(rename="inspectConfig")]
    pub inspect_config: Option<GooglePrivacyDlpV2beta1InspectConfig>,
    /// Specification of the data set to process.
    #[serde(rename="storageConfig")]
    pub storage_config: Option<GooglePrivacyDlpV2beta1StorageConfig>,
    /// Additional configuration settings for long running operations.
    #[serde(rename="operationConfig")]
    pub operation_config: Option<GooglePrivacyDlpV2beta1OperationConfig>,
}

impl RequestValue for GooglePrivacyDlpV2beta1CreateInspectOperationRequest {}


/// Max findings configuration per info type, per content item or long running
/// operation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1InfoTypeLimit {
    /// Type of information the findings limit applies to. Only one limit per
    /// info_type should be provided. If InfoTypeLimit does not have an
    /// info_type, the DLP API applies the limit against all info_types that are
    /// found but not specified in another InfoTypeLimit.
    #[serde(rename="infoType")]
    pub info_type: Option<GooglePrivacyDlpV2beta1InfoType>,
    /// Max findings limit for the given infoType.
    #[serde(rename="maxFindings")]
    pub max_findings: Option<i32>,
}

impl Part for GooglePrivacyDlpV2beta1InfoTypeLimit {}


/// An auxiliary table contains statistical information on the relative
/// frequency of different quasi-identifiers values. It has one or several
/// quasi-identifiers columns, and one column that indicates the relative
/// frequency of each quasi-identifier tuple.
/// If a tuple is present in the data but not in the auxiliary table, the
/// corresponding relative frequency is assumed to be zero (and thus, the
/// tuple is highly reidentifiable).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1AuxiliaryTable {
    /// Auxiliary table location. [required]
    pub table: Option<GooglePrivacyDlpV2beta1BigQueryTable>,
    /// Quasi-identifier columns. [required]
    #[serde(rename="quasiIds")]
    pub quasi_ids: Option<Vec<GooglePrivacyDlpV2beta1QuasiIdField>>,
    /// The relative frequency column must contain a floating-point number
    /// between 0 and 1 (inclusive). Null values are assumed to be zero.
    /// [required]
    #[serde(rename="relativeFrequency")]
    pub relative_frequency: Option<GooglePrivacyDlpV2beta1FieldId>,
}

impl Part for GooglePrivacyDlpV2beta1AuxiliaryTable {}


/// The field type of `value` and `field` do not need to match to be
/// considered equal, but not all comparisons are possible.
/// 
/// A `value` of type:
/// 
/// - `string` can be compared against all other types
/// - `boolean` can only be compared against other booleans
/// - `integer` can be compared against doubles or a string if the string value
/// can be parsed as an integer.
/// - `double` can be compared against integers or a string if the string can
/// be parsed as a double.
/// - `Timestamp` can be compared against strings in RFC 3339 date string
/// format.
/// - `TimeOfDay` can be compared against timestamps and strings in the format
/// of 'HH:mm:ss'.
/// 
/// If we fail to compare do to type mismatch, a warning will be given and
/// the condition will evaluate to false.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1Condition {
    /// Operator used to compare the field or info type to the value. [required]
    pub operator: Option<String>,
    /// Field within the record this condition is evaluated against. [required]
    pub field: Option<GooglePrivacyDlpV2beta1FieldId>,
    /// Value to compare against. [Required, except for `EXISTS` tests.]
    pub value: Option<GooglePrivacyDlpV2beta1Value>,
}

impl Part for GooglePrivacyDlpV2beta1Condition {}


/// Description of the information type (infoType).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1InfoTypeDescription {
    /// Human readable form of the infoType name.
    #[serde(rename="displayName")]
    pub display_name: Option<String>,
    /// Internal name of the infoType.
    pub name: Option<String>,
    /// List of categories this infoType belongs to.
    pub categories: Option<Vec<GooglePrivacyDlpV2beta1CategoryDescription>>,
}

impl Part for GooglePrivacyDlpV2beta1InfoTypeDescription {}


/// A representation of a Datastore kind.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1KindExpression {
    /// The name of the kind.
    pub name: Option<String>,
}

impl Part for GooglePrivacyDlpV2beta1KindExpression {}


/// Replace each matching finding with the name of the info_type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1ReplaceWithInfoTypeConfig { _never_set: Option<bool> }

impl Part for GooglePrivacyDlpV2beta1ReplaceWithInfoTypeConfig {}


/// Request to search for potentially sensitive info in a list of items.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [inspect content](struct.ContentInspectCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1InspectContentRequest {
    /// The list of items to inspect. Items in a single request are
    /// considered "related" unless inspect_config.independent_inputs is true.
    /// Up to 100 are allowed per request.
    pub items: Option<Vec<GooglePrivacyDlpV2beta1ContentItem>>,
    /// Configuration for the inspector.
    #[serde(rename="inspectConfig")]
    pub inspect_config: Option<GooglePrivacyDlpV2beta1InspectConfig>,
}

impl RequestValue for GooglePrivacyDlpV2beta1InspectContentRequest {}


/// Response to the ListInspectFindings request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [results findings list inspect](struct.InspectResultFindingListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1ListInspectFindingsResponse {
    /// If not empty, indicates that there may be more results that match the
    /// request; this value should be passed in a new `ListInspectFindingsRequest`.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// The results.
    pub result: Option<GooglePrivacyDlpV2beta1InspectResult>,
}

impl ResponseResult for GooglePrivacyDlpV2beta1ListInspectFindingsResponse {}


/// A condition for determining whether a transformation should be applied to
/// a field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1RecordCondition {
    /// no description provided
    pub expressions: Option<GooglePrivacyDlpV2beta1Expressions>,
}

impl Part for GooglePrivacyDlpV2beta1RecordCondition {}


/// Set of primitive values supported by the system.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1Value {
    /// no description provided
    #[serde(rename="booleanValue")]
    pub boolean_value: Option<bool>,
    /// no description provided
    #[serde(rename="timeValue")]
    pub time_value: Option<GoogleTypeTimeOfDay>,
    /// no description provided
    #[serde(rename="timestampValue")]
    pub timestamp_value: Option<String>,
    /// no description provided
    #[serde(rename="dateValue")]
    pub date_value: Option<GoogleTypeDate>,
    /// no description provided
    #[serde(rename="stringValue")]
    pub string_value: Option<String>,
    /// no description provided
    #[serde(rename="floatValue")]
    pub float_value: Option<f64>,
    /// no description provided
    #[serde(rename="integerValue")]
    pub integer_value: Option<String>,
}

impl Part for GooglePrivacyDlpV2beta1Value {}


/// Privacy metric to compute for reidentification risk analysis.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1PrivacyMetric {
    /// no description provided
    #[serde(rename="numericalStatsConfig")]
    pub numerical_stats_config: Option<GooglePrivacyDlpV2beta1NumericalStatsConfig>,
    /// no description provided
    #[serde(rename="lDiversityConfig")]
    pub l_diversity_config: Option<GooglePrivacyDlpV2beta1LDiversityConfig>,
    /// no description provided
    #[serde(rename="kMapEstimationConfig")]
    pub k_map_estimation_config: Option<GooglePrivacyDlpV2beta1KMapEstimationConfig>,
    /// no description provided
    #[serde(rename="kAnonymityConfig")]
    pub k_anonymity_config: Option<GooglePrivacyDlpV2beta1KAnonymityConfig>,
    /// no description provided
    #[serde(rename="categoricalStatsConfig")]
    pub categorical_stats_config: Option<GooglePrivacyDlpV2beta1CategoricalStatsConfig>,
}

impl Part for GooglePrivacyDlpV2beta1PrivacyMetric {}


/// Request for creating a risk analysis operation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [analyze data source](struct.DataSourceAnalyzeCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1AnalyzeDataSourceRiskRequest {
    /// Privacy metric to compute.
    #[serde(rename="privacyMetric")]
    pub privacy_metric: Option<GooglePrivacyDlpV2beta1PrivacyMetric>,
    /// Input dataset to compute metrics over.
    #[serde(rename="sourceTable")]
    pub source_table: Option<GooglePrivacyDlpV2beta1BigQueryTable>,
}

impl RequestValue for GooglePrivacyDlpV2beta1AnalyzeDataSourceRiskRequest {}


/// Options defining a data set within Google Cloud Datastore.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1DatastoreOptions {
    /// A partition ID identifies a grouping of entities. The grouping is always
    /// by project and namespace, however the namespace ID may be empty.
    #[serde(rename="partitionId")]
    pub partition_id: Option<GooglePrivacyDlpV2beta1PartitionId>,
    /// The kind to process.
    pub kind: Option<GooglePrivacyDlpV2beta1KindExpression>,
    /// Properties to scan. If none are specified, all properties will be scanned
    /// by default.
    pub projection: Option<Vec<GooglePrivacyDlpV2beta1Projection>>,
}

impl Part for GooglePrivacyDlpV2beta1DatastoreOptions {}


/// An entity in a dataset is a field or set of fields that correspond to a
/// single person. For example, in medical records the `EntityId` might be
/// a patient identifier, or for financial records it might be an account
/// identifier. This message is used when generalizations or analysis must be
/// consistent across multiple rows pertaining to the same entity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1EntityId {
    /// Composite key indicating which field contains the entity identifier.
    pub field: Option<GooglePrivacyDlpV2beta1FieldId>,
}

impl Part for GooglePrivacyDlpV2beta1EntityId {}


/// Characters to skip when doing deidentification of a value. These will be left
/// alone and skipped.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1CharsToIgnore {
    /// no description provided
    #[serde(rename="commonCharactersToIgnore")]
    pub common_characters_to_ignore: Option<String>,
    /// no description provided
    #[serde(rename="charactersToSkip")]
    pub characters_to_skip: Option<String>,
}

impl Part for GooglePrivacyDlpV2beta1CharsToIgnore {}


/// Container structure for the content to inspect.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1ContentItem {
    /// Structured content for inspection.
    pub table: Option<GooglePrivacyDlpV2beta1Table>,
    /// Content data to inspect or redact.
    pub data: Option<String>,
    /// String data to inspect or redact.
    pub value: Option<String>,
    /// Type of the content, as defined in Content-Type HTTP header.
    /// Supported types are: all "text" types, octet streams, PNG images,
    /// JPEG images.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl Part for GooglePrivacyDlpV2beta1ContentItem {}


/// Specifies the location of a finding within its source item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1Location {
    /// Location within a `ContentItem.Table`.
    #[serde(rename="tableLocation")]
    pub table_location: Option<GooglePrivacyDlpV2beta1TableLocation>,
    /// Location within an image's pixels.
    #[serde(rename="imageBoxes")]
    pub image_boxes: Option<Vec<GooglePrivacyDlpV2beta1ImageLocation>>,
    /// Zero-based byte offsets within a content item.
    #[serde(rename="byteRange")]
    pub byte_range: Option<GooglePrivacyDlpV2beta1Range>,
    /// Character offsets within a content item, included when content type
    /// is a text. Default charset assumed to be UTF-8.
    #[serde(rename="codepointRange")]
    pub codepoint_range: Option<GooglePrivacyDlpV2beta1Range>,
    /// Field id of the field containing the finding.
    #[serde(rename="fieldId")]
    pub field_id: Option<GooglePrivacyDlpV2beta1FieldId>,
    /// Key of the finding.
    #[serde(rename="recordKey")]
    pub record_key: Option<GooglePrivacyDlpV2beta1RecordKey>,
}

impl Part for GooglePrivacyDlpV2beta1Location {}


/// Record key for a finding in Cloud Datastore.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1DatastoreKey {
    /// Datastore entity key.
    #[serde(rename="entityKey")]
    pub entity_key: Option<GooglePrivacyDlpV2beta1Key>,
}

impl Part for GooglePrivacyDlpV2beta1DatastoreKey {}


/// Response to the ListInfoTypes request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [info types list root categories](struct.RootCategoryInfoTypeListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1ListInfoTypesResponse {
    /// Set of sensitive info types belonging to a category.
    #[serde(rename="infoTypes")]
    pub info_types: Option<Vec<GooglePrivacyDlpV2beta1InfoTypeDescription>>,
}

impl ResponseResult for GooglePrivacyDlpV2beta1ListInfoTypesResponse {}


/// A type of transformation that is applied over structured data such as a
/// table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1RecordTransformations {
    /// Configuration defining which records get suppressed entirely. Records that
    /// match any suppression rule are omitted from the output [optional].
    #[serde(rename="recordSuppressions")]
    pub record_suppressions: Option<Vec<GooglePrivacyDlpV2beta1RecordSuppression>>,
    /// Transform the record by applying various field transformations.
    #[serde(rename="fieldTransformations")]
    pub field_transformations: Option<Vec<GooglePrivacyDlpV2beta1FieldTransformation>>,
}

impl Part for GooglePrivacyDlpV2beta1RecordTransformations {}


/// Structured content to inspect. Up to 50,000 `Value`s per request allowed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1Table {
    /// no description provided
    pub headers: Option<Vec<GooglePrivacyDlpV2beta1FieldId>>,
    /// no description provided
    pub rows: Option<Vec<GooglePrivacyDlpV2beta1Row>>,
}

impl Part for GooglePrivacyDlpV2beta1Table {}


/// Options defining BigQuery table and row identifiers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1BigQueryOptions {
    /// References to fields uniquely identifying rows within the table.
    /// Nested fields in the format, like `person.birthdate.year`, are allowed.
    #[serde(rename="identifyingFields")]
    pub identifying_fields: Option<Vec<GooglePrivacyDlpV2beta1FieldId>>,
    /// Complete BigQuery table reference.
    #[serde(rename="tableReference")]
    pub table_reference: Option<GooglePrivacyDlpV2beta1BigQueryTable>,
}

impl Part for GooglePrivacyDlpV2beta1BigQueryOptions {}


/// Include to use an existing data crypto key wrapped by KMS.
/// Authorization requires the following IAM permissions when sending a request
/// to perform a crypto transformation using a kms-wrapped crypto key:
/// dlp.kms.encrypt
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1KmsWrappedCryptoKey {
    /// The resource name of the KMS CryptoKey to use for unwrapping. [required]
    #[serde(rename="cryptoKeyName")]
    pub crypto_key_name: Option<String>,
    /// The wrapped data crypto key. [required]
    #[serde(rename="wrappedKey")]
    pub wrapped_key: Option<String>,
}

impl Part for GooglePrivacyDlpV2beta1KmsWrappedCryptoKey {}


/// This resource represents a long-running operation that is the result of a
/// network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [analyze data source](struct.DataSourceAnalyzeCall.html) (response)
/// * [operations get risk analysis](struct.RiskAnalysiOperationGetCall.html) (response)
/// * [operations get inspect](struct.InspectOperationGetCall.html) (response)
/// * [operations create inspect](struct.InspectOperationCreateCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleLongrunningOperation {
    /// The error result of the operation in case of failure or cancellation.
    pub error: Option<GoogleRpcStatus>,
    /// If the value is `false`, it means the operation is still in progress.
    /// If `true`, the operation is completed, and either `error` or `response` is
    /// available.
    pub done: Option<bool>,
    /// This field will contain an InspectOperationResult object for `inspect.operations.create` or a RiskAnalysisOperationResult object for `dataSource.analyze`.
    pub response: Option<HashMap<String, String>>,
    /// The server-assigned name. The `name` should have the format of `inspect/operations/<identifier>`.
    pub name: Option<String>,
    /// This field will contain an InspectOperationMetadata object for `inspect.operations.create` or a RiskAnalysisOperationMetadata object for `dataSource.analyze`.  This will always be returned with the Operation.
    pub metadata: Option<HashMap<String, String>>,
}

impl ResponseResult for GoogleLongrunningOperation {}


/// Generic half-open interval [start, end)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1Range {
    /// Index of the first character of the range (inclusive).
    pub start: Option<String>,
    /// Index of the last character of the range (exclusive).
    pub end: Option<String>,
}

impl Part for GooglePrivacyDlpV2beta1Range {}


/// A transformation to apply to text that is identified as a specific
/// info_type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1InfoTypeTransformation {
    /// Primitive transformation to apply to the info type. [required]
    #[serde(rename="primitiveTransformation")]
    pub primitive_transformation: Option<GooglePrivacyDlpV2beta1PrimitiveTransformation>,
    /// Info types to apply the transformation to. Empty list will match all
    /// available info types for this transformation.
    #[serde(rename="infoTypes")]
    pub info_types: Option<Vec<GooglePrivacyDlpV2beta1InfoType>>,
}

impl Part for GooglePrivacyDlpV2beta1InfoTypeTransformation {}


/// The response message for Operations.ListOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [operations list inspect](struct.InspectOperationListCall.html) (response)
/// * [operations list risk analysis](struct.RiskAnalysiOperationListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleLongrunningListOperationsResponse {
    /// A list of operations that matches the specified filter in the request.
    pub operations: Option<Vec<GoogleLongrunningOperation>>,
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
}

impl ResponseResult for GoogleLongrunningListOperationsResponse {}


/// Configuration to suppress records whose suppression conditions evaluate to
/// true.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1RecordSuppression {
    /// no description provided
    pub condition: Option<GooglePrivacyDlpV2beta1RecordCondition>,
}

impl Part for GooglePrivacyDlpV2beta1RecordSuppression {}


/// Compute numerical stats over an individual column, including
/// number of distinct values and value count distribution.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1CategoricalStatsConfig {
    /// Field to compute categorical stats on. All column types are
    /// supported except for arrays and structs. However, it may be more
    /// informative to use NumericalStats when the field type is supported,
    /// depending on the data.
    pub field: Option<GooglePrivacyDlpV2beta1FieldId>,
}

impl Part for GooglePrivacyDlpV2beta1CategoricalStatsConfig {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *content* resources.
/// It is not used directly, but through the `DLP` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_dlp2_beta1 as dlp2_beta1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use dlp2_beta1::DLP;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = DLP::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `deidentify(...)`, `inspect(...)` and `redact(...)`
/// // to build up your call.
/// let rb = hub.content();
/// # }
/// ```
pub struct ContentMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DLP<C, A>,
}

impl<'a, C, A> MethodsBuilder for ContentMethods<'a, C, A> {}

impl<'a, C, A> ContentMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Redacts potentially sensitive info from a list of strings.
    /// This method has limits on input size, processing time, and output size.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn redact(&self, request: GooglePrivacyDlpV2beta1RedactContentRequest) -> ContentRedactCall<'a, C, A> {
        ContentRedactCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// De-identifies potentially sensitive info from a list of strings.
    /// This method has limits on input size and output size.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn deidentify(&self, request: GooglePrivacyDlpV2beta1DeidentifyContentRequest) -> ContentDeidentifyCall<'a, C, A> {
        ContentDeidentifyCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Finds potentially sensitive info in a list of strings.
    /// This method has limits on input size, processing time, and output size.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn inspect(&self, request: GooglePrivacyDlpV2beta1InspectContentRequest) -> ContentInspectCall<'a, C, A> {
        ContentInspectCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *rootCategory* resources.
/// It is not used directly, but through the `DLP` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_dlp2_beta1 as dlp2_beta1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use dlp2_beta1::DLP;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = DLP::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `info_types_list(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.root_categories();
/// # }
/// ```
pub struct RootCategoryMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DLP<C, A>,
}

impl<'a, C, A> MethodsBuilder for RootCategoryMethods<'a, C, A> {}

impl<'a, C, A> RootCategoryMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns sensitive information types for given category.
    /// 
    /// # Arguments
    ///
    /// * `category` - Category name as returned by ListRootCategories.
    pub fn info_types_list(&self, category: &str) -> RootCategoryInfoTypeListCall<'a, C, A> {
        RootCategoryInfoTypeListCall {
            hub: self.hub,
            _category: category.to_string(),
            _language_code: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of root categories of sensitive information.
    pub fn list(&self) -> RootCategoryListCall<'a, C, A> {
        RootCategoryListCall {
            hub: self.hub,
            _language_code: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *inspect* resources.
/// It is not used directly, but through the `DLP` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_dlp2_beta1 as dlp2_beta1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use dlp2_beta1::DLP;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = DLP::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `operations_cancel(...)`, `operations_create(...)`, `operations_delete(...)`, `operations_get(...)`, `operations_list(...)` and `results_findings_list(...)`
/// // to build up your call.
/// let rb = hub.inspect();
/// # }
/// ```
pub struct InspectMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DLP<C, A>,
}

impl<'a, C, A> MethodsBuilder for InspectMethods<'a, C, A> {}

impl<'a, C, A> InspectMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Schedules a job scanning content in a Google Cloud Platform data
    /// repository.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn operations_create(&self, request: GooglePrivacyDlpV2beta1CreateInspectOperationRequest) -> InspectOperationCreateCall<'a, C, A> {
        InspectOperationCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Fetches the list of long running operations.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation's parent resource.
    pub fn operations_list(&self, name: &str) -> InspectOperationListCall<'a, C, A> {
        InspectOperationListCall {
            hub: self.hub,
            _name: name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
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
    pub fn operations_get(&self, name: &str) -> InspectOperationGetCall<'a, C, A> {
        InspectOperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Cancels an operation. Use the `inspect.operations.get` to check whether the cancellation succeeded or the operation completed despite cancellation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn operations_cancel(&self, request: GoogleLongrunningCancelOperationRequest, name: &str) -> InspectOperationCancelCall<'a, C, A> {
        InspectOperationCancelCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// This method is not supported and the server returns `UNIMPLEMENTED`.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource to be deleted.
    pub fn operations_delete(&self, name: &str) -> InspectOperationDeleteCall<'a, C, A> {
        InspectOperationDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns list of results for given inspect operation result set id.
    /// 
    /// # Arguments
    ///
    /// * `name` - Identifier of the results set returned as metadata of
    ///            the longrunning operation created by a call to InspectDataSource.
    ///            Should be in the format of `inspect/results/{id}`.
    pub fn results_findings_list(&self, name: &str) -> InspectResultFindingListCall<'a, C, A> {
        InspectResultFindingListCall {
            hub: self.hub,
            _name: name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *riskAnalysi* resources.
/// It is not used directly, but through the `DLP` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_dlp2_beta1 as dlp2_beta1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use dlp2_beta1::DLP;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = DLP::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `operations_cancel(...)`, `operations_delete(...)`, `operations_get(...)` and `operations_list(...)`
/// // to build up your call.
/// let rb = hub.risk_analysis();
/// # }
/// ```
pub struct RiskAnalysiMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DLP<C, A>,
}

impl<'a, C, A> MethodsBuilder for RiskAnalysiMethods<'a, C, A> {}

impl<'a, C, A> RiskAnalysiMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// This method is not supported and the server returns `UNIMPLEMENTED`.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource to be deleted.
    pub fn operations_delete(&self, name: &str) -> RiskAnalysiOperationDeleteCall<'a, C, A> {
        RiskAnalysiOperationDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Fetches the list of long running operations.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation's parent resource.
    pub fn operations_list(&self, name: &str) -> RiskAnalysiOperationListCall<'a, C, A> {
        RiskAnalysiOperationListCall {
            hub: self.hub,
            _name: name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
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
    pub fn operations_get(&self, name: &str) -> RiskAnalysiOperationGetCall<'a, C, A> {
        RiskAnalysiOperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Cancels an operation. Use the `inspect.operations.get` to check whether the cancellation succeeded or the operation completed despite cancellation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn operations_cancel(&self, request: GoogleLongrunningCancelOperationRequest, name: &str) -> RiskAnalysiOperationCancelCall<'a, C, A> {
        RiskAnalysiOperationCancelCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *dataSource* resources.
/// It is not used directly, but through the `DLP` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_dlp2_beta1 as dlp2_beta1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use dlp2_beta1::DLP;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = DLP::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `analyze(...)`
/// // to build up your call.
/// let rb = hub.data_source();
/// # }
/// ```
pub struct DataSourceMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DLP<C, A>,
}

impl<'a, C, A> MethodsBuilder for DataSourceMethods<'a, C, A> {}

impl<'a, C, A> DataSourceMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Schedules a job to compute risk analysis metrics over content in a Google
    /// Cloud Platform repository.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn analyze(&self, request: GooglePrivacyDlpV2beta1AnalyzeDataSourceRiskRequest) -> DataSourceAnalyzeCall<'a, C, A> {
        DataSourceAnalyzeCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Redacts potentially sensitive info from a list of strings.
/// This method has limits on input size, processing time, and output size.
///
/// A builder for the *redact* method supported by a *content* resource.
/// It is not used directly, but through a `ContentMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_dlp2_beta1 as dlp2_beta1;
/// use dlp2_beta1::GooglePrivacyDlpV2beta1RedactContentRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use dlp2_beta1::DLP;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DLP::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GooglePrivacyDlpV2beta1RedactContentRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.content().redact(req)
///              .doit();
/// # }
/// ```
pub struct ContentRedactCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DLP<C, A>,
    _request: GooglePrivacyDlpV2beta1RedactContentRequest,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ContentRedactCall<'a, C, A> {}

impl<'a, C, A> ContentRedactCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GooglePrivacyDlpV2beta1RedactContentResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "dlp.content.redact",
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

        let mut url = self.hub._base_url.clone() + "v2beta1/content:redact";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
    pub fn request(mut self, new_value: GooglePrivacyDlpV2beta1RedactContentRequest) -> ContentRedactCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ContentRedactCall<'a, C, A> {
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
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ContentRedactCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ContentRedactCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// De-identifies potentially sensitive info from a list of strings.
/// This method has limits on input size and output size.
///
/// A builder for the *deidentify* method supported by a *content* resource.
/// It is not used directly, but through a `ContentMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_dlp2_beta1 as dlp2_beta1;
/// use dlp2_beta1::GooglePrivacyDlpV2beta1DeidentifyContentRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use dlp2_beta1::DLP;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DLP::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GooglePrivacyDlpV2beta1DeidentifyContentRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.content().deidentify(req)
///              .doit();
/// # }
/// ```
pub struct ContentDeidentifyCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DLP<C, A>,
    _request: GooglePrivacyDlpV2beta1DeidentifyContentRequest,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ContentDeidentifyCall<'a, C, A> {}

impl<'a, C, A> ContentDeidentifyCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GooglePrivacyDlpV2beta1DeidentifyContentResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "dlp.content.deidentify",
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

        let mut url = self.hub._base_url.clone() + "v2beta1/content:deidentify";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
    pub fn request(mut self, new_value: GooglePrivacyDlpV2beta1DeidentifyContentRequest) -> ContentDeidentifyCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ContentDeidentifyCall<'a, C, A> {
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
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ContentDeidentifyCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ContentDeidentifyCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Finds potentially sensitive info in a list of strings.
/// This method has limits on input size, processing time, and output size.
///
/// A builder for the *inspect* method supported by a *content* resource.
/// It is not used directly, but through a `ContentMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_dlp2_beta1 as dlp2_beta1;
/// use dlp2_beta1::GooglePrivacyDlpV2beta1InspectContentRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use dlp2_beta1::DLP;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DLP::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GooglePrivacyDlpV2beta1InspectContentRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.content().inspect(req)
///              .doit();
/// # }
/// ```
pub struct ContentInspectCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DLP<C, A>,
    _request: GooglePrivacyDlpV2beta1InspectContentRequest,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ContentInspectCall<'a, C, A> {}

impl<'a, C, A> ContentInspectCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GooglePrivacyDlpV2beta1InspectContentResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "dlp.content.inspect",
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

        let mut url = self.hub._base_url.clone() + "v2beta1/content:inspect";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
    pub fn request(mut self, new_value: GooglePrivacyDlpV2beta1InspectContentRequest) -> ContentInspectCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ContentInspectCall<'a, C, A> {
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
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ContentInspectCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ContentInspectCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns sensitive information types for given category.
///
/// A builder for the *infoTypes.list* method supported by a *rootCategory* resource.
/// It is not used directly, but through a `RootCategoryMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_dlp2_beta1 as dlp2_beta1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use dlp2_beta1::DLP;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DLP::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.root_categories().info_types_list("category")
///              .language_code("sit")
///              .doit();
/// # }
/// ```
pub struct RootCategoryInfoTypeListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DLP<C, A>,
    _category: String,
    _language_code: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for RootCategoryInfoTypeListCall<'a, C, A> {}

impl<'a, C, A> RootCategoryInfoTypeListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GooglePrivacyDlpV2beta1ListInfoTypesResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "dlp.rootCategories.infoTypes.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("category", self._category.to_string()));
        if let Some(value) = self._language_code {
            params.push(("languageCode", value.to_string()));
        }
        for &field in ["alt", "category", "languageCode"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2beta1/rootCategories/{+category}/infoTypes";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+category}", "category")].iter() {
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
            for param_name in ["category"].iter() {
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


    /// Category name as returned by ListRootCategories.
    ///
    /// Sets the *category* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn category(mut self, new_value: &str) -> RootCategoryInfoTypeListCall<'a, C, A> {
        self._category = new_value.to_string();
        self
    }
    /// Optional BCP-47 language code for localized info type friendly
    /// names. If omitted, or if localized strings are not available,
    /// en-US strings will be returned.
    ///
    /// Sets the *language code* query property to the given value.
    pub fn language_code(mut self, new_value: &str) -> RootCategoryInfoTypeListCall<'a, C, A> {
        self._language_code = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> RootCategoryInfoTypeListCall<'a, C, A> {
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
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> RootCategoryInfoTypeListCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> RootCategoryInfoTypeListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns the list of root categories of sensitive information.
///
/// A builder for the *list* method supported by a *rootCategory* resource.
/// It is not used directly, but through a `RootCategoryMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_dlp2_beta1 as dlp2_beta1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use dlp2_beta1::DLP;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DLP::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.root_categories().list()
///              .language_code("Stet")
///              .doit();
/// # }
/// ```
pub struct RootCategoryListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DLP<C, A>,
    _language_code: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for RootCategoryListCall<'a, C, A> {}

impl<'a, C, A> RootCategoryListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GooglePrivacyDlpV2beta1ListRootCategoriesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "dlp.rootCategories.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        if let Some(value) = self._language_code {
            params.push(("languageCode", value.to_string()));
        }
        for &field in ["alt", "languageCode"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2beta1/rootCategories";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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


    /// Optional language code for localized friendly category names.
    /// If omitted or if localized strings are not available,
    /// en-US strings will be returned.
    ///
    /// Sets the *language code* query property to the given value.
    pub fn language_code(mut self, new_value: &str) -> RootCategoryListCall<'a, C, A> {
        self._language_code = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> RootCategoryListCall<'a, C, A> {
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
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> RootCategoryListCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> RootCategoryListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Schedules a job scanning content in a Google Cloud Platform data
/// repository.
///
/// A builder for the *operations.create* method supported by a *inspect* resource.
/// It is not used directly, but through a `InspectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_dlp2_beta1 as dlp2_beta1;
/// use dlp2_beta1::GooglePrivacyDlpV2beta1CreateInspectOperationRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use dlp2_beta1::DLP;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DLP::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GooglePrivacyDlpV2beta1CreateInspectOperationRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.inspect().operations_create(req)
///              .doit();
/// # }
/// ```
pub struct InspectOperationCreateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DLP<C, A>,
    _request: GooglePrivacyDlpV2beta1CreateInspectOperationRequest,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for InspectOperationCreateCall<'a, C, A> {}

impl<'a, C, A> InspectOperationCreateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GoogleLongrunningOperation)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "dlp.inspect.operations.create",
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

        let mut url = self.hub._base_url.clone() + "v2beta1/inspect/operations";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
    pub fn request(mut self, new_value: GooglePrivacyDlpV2beta1CreateInspectOperationRequest) -> InspectOperationCreateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> InspectOperationCreateCall<'a, C, A> {
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
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> InspectOperationCreateCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> InspectOperationCreateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Fetches the list of long running operations.
///
/// A builder for the *operations.list* method supported by a *inspect* resource.
/// It is not used directly, but through a `InspectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_dlp2_beta1 as dlp2_beta1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use dlp2_beta1::DLP;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DLP::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.inspect().operations_list("name")
///              .page_token("et")
///              .page_size(-18)
///              .filter("kasd")
///              .doit();
/// # }
/// ```
pub struct InspectOperationListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DLP<C, A>,
    _name: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _filter: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for InspectOperationListCall<'a, C, A> {}

impl<'a, C, A> InspectOperationListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GoogleLongrunningListOperationsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "dlp.inspect.operations.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("name", self._name.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._filter {
            params.push(("filter", value.to_string()));
        }
        for &field in ["alt", "name", "pageToken", "pageSize", "filter"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2beta1/{+name}";
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


    /// The name of the operation's parent resource.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> InspectOperationListCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The standard list page token.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> InspectOperationListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The list page size. The maximum allowed value is 256 and the default is 100.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> InspectOperationListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// Filters by `done`. That is, `done=true` or `done=false`.
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> InspectOperationListCall<'a, C, A> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> InspectOperationListCall<'a, C, A> {
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
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> InspectOperationListCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> InspectOperationListCall<'a, C, A>
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
/// A builder for the *operations.get* method supported by a *inspect* resource.
/// It is not used directly, but through a `InspectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_dlp2_beta1 as dlp2_beta1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use dlp2_beta1::DLP;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DLP::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.inspect().operations_get("name")
///              .doit();
/// # }
/// ```
pub struct InspectOperationGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DLP<C, A>,
    _name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for InspectOperationGetCall<'a, C, A> {}

impl<'a, C, A> InspectOperationGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GoogleLongrunningOperation)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "dlp.inspect.operations.get",
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

        let mut url = self.hub._base_url.clone() + "v2beta1/{+name}";
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


    /// The name of the operation resource.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> InspectOperationGetCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> InspectOperationGetCall<'a, C, A> {
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
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> InspectOperationGetCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> InspectOperationGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Cancels an operation. Use the `inspect.operations.get` to check whether the cancellation succeeded or the operation completed despite cancellation.
///
/// A builder for the *operations.cancel* method supported by a *inspect* resource.
/// It is not used directly, but through a `InspectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_dlp2_beta1 as dlp2_beta1;
/// use dlp2_beta1::GoogleLongrunningCancelOperationRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use dlp2_beta1::DLP;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DLP::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleLongrunningCancelOperationRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.inspect().operations_cancel(req, "name")
///              .doit();
/// # }
/// ```
pub struct InspectOperationCancelCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DLP<C, A>,
    _request: GoogleLongrunningCancelOperationRequest,
    _name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for InspectOperationCancelCall<'a, C, A> {}

impl<'a, C, A> InspectOperationCancelCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GoogleProtobufEmpty)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "dlp.inspect.operations.cancel",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
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

        let mut url = self.hub._base_url.clone() + "v2beta1/{+name}:cancel";
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
    pub fn request(mut self, new_value: GoogleLongrunningCancelOperationRequest) -> InspectOperationCancelCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The name of the operation resource to be cancelled.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> InspectOperationCancelCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> InspectOperationCancelCall<'a, C, A> {
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
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> InspectOperationCancelCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> InspectOperationCancelCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// This method is not supported and the server returns `UNIMPLEMENTED`.
///
/// A builder for the *operations.delete* method supported by a *inspect* resource.
/// It is not used directly, but through a `InspectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_dlp2_beta1 as dlp2_beta1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use dlp2_beta1::DLP;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DLP::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.inspect().operations_delete("name")
///              .doit();
/// # }
/// ```
pub struct InspectOperationDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DLP<C, A>,
    _name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for InspectOperationDeleteCall<'a, C, A> {}

impl<'a, C, A> InspectOperationDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GoogleProtobufEmpty)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "dlp.inspect.operations.delete",
                               http_method: hyper::method::Method::Delete });
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

        let mut url = self.hub._base_url.clone() + "v2beta1/{+name}";
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Delete, url.clone())
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


    /// The name of the operation resource to be deleted.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> InspectOperationDeleteCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> InspectOperationDeleteCall<'a, C, A> {
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
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> InspectOperationDeleteCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> InspectOperationDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns list of results for given inspect operation result set id.
///
/// A builder for the *results.findings.list* method supported by a *inspect* resource.
/// It is not used directly, but through a `InspectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_dlp2_beta1 as dlp2_beta1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use dlp2_beta1::DLP;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DLP::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.inspect().results_findings_list("name")
///              .page_token("erat")
///              .page_size(-35)
///              .filter("sea")
///              .doit();
/// # }
/// ```
pub struct InspectResultFindingListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DLP<C, A>,
    _name: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _filter: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for InspectResultFindingListCall<'a, C, A> {}

impl<'a, C, A> InspectResultFindingListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GooglePrivacyDlpV2beta1ListInspectFindingsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "dlp.inspect.results.findings.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("name", self._name.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._filter {
            params.push(("filter", value.to_string()));
        }
        for &field in ["alt", "name", "pageToken", "pageSize", "filter"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2beta1/{+name}/findings";
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


    /// Identifier of the results set returned as metadata of
    /// the longrunning operation created by a call to InspectDataSource.
    /// Should be in the format of `inspect/results/{id}`.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> InspectResultFindingListCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The value returned by the last `ListInspectFindingsResponse`; indicates
    /// that this is a continuation of a prior `ListInspectFindings` call, and that
    /// the system should return the next page of data.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> InspectResultFindingListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of results to return.
    /// If 0, the implementation selects a reasonable value.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> InspectResultFindingListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// Restricts findings to items that match. Supports info_type and likelihood.
    /// 
    /// Examples:
    /// 
    /// - info_type=EMAIL_ADDRESS
    /// - info_type=PHONE_NUMBER,EMAIL_ADDRESS
    /// - likelihood=VERY_LIKELY
    /// - likelihood=VERY_LIKELY,LIKELY
    /// - info_type=EMAIL_ADDRESS,likelihood=VERY_LIKELY,LIKELY
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> InspectResultFindingListCall<'a, C, A> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> InspectResultFindingListCall<'a, C, A> {
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
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> InspectResultFindingListCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> InspectResultFindingListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// This method is not supported and the server returns `UNIMPLEMENTED`.
///
/// A builder for the *operations.delete* method supported by a *riskAnalysi* resource.
/// It is not used directly, but through a `RiskAnalysiMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_dlp2_beta1 as dlp2_beta1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use dlp2_beta1::DLP;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DLP::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.risk_analysis().operations_delete("name")
///              .doit();
/// # }
/// ```
pub struct RiskAnalysiOperationDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DLP<C, A>,
    _name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for RiskAnalysiOperationDeleteCall<'a, C, A> {}

impl<'a, C, A> RiskAnalysiOperationDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GoogleProtobufEmpty)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "dlp.riskAnalysis.operations.delete",
                               http_method: hyper::method::Method::Delete });
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

        let mut url = self.hub._base_url.clone() + "v2beta1/{+name}";
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Delete, url.clone())
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


    /// The name of the operation resource to be deleted.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> RiskAnalysiOperationDeleteCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> RiskAnalysiOperationDeleteCall<'a, C, A> {
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
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> RiskAnalysiOperationDeleteCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> RiskAnalysiOperationDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Fetches the list of long running operations.
///
/// A builder for the *operations.list* method supported by a *riskAnalysi* resource.
/// It is not used directly, but through a `RiskAnalysiMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_dlp2_beta1 as dlp2_beta1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use dlp2_beta1::DLP;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DLP::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.risk_analysis().operations_list("name")
///              .page_token("gubergren")
///              .page_size(-95)
///              .filter("aliquyam")
///              .doit();
/// # }
/// ```
pub struct RiskAnalysiOperationListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DLP<C, A>,
    _name: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _filter: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for RiskAnalysiOperationListCall<'a, C, A> {}

impl<'a, C, A> RiskAnalysiOperationListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GoogleLongrunningListOperationsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "dlp.riskAnalysis.operations.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("name", self._name.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._filter {
            params.push(("filter", value.to_string()));
        }
        for &field in ["alt", "name", "pageToken", "pageSize", "filter"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2beta1/{+name}";
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


    /// The name of the operation's parent resource.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> RiskAnalysiOperationListCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The standard list page token.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> RiskAnalysiOperationListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The list page size. The maximum allowed value is 256 and the default is 100.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> RiskAnalysiOperationListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// Filters by `done`. That is, `done=true` or `done=false`.
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> RiskAnalysiOperationListCall<'a, C, A> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> RiskAnalysiOperationListCall<'a, C, A> {
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
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> RiskAnalysiOperationListCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> RiskAnalysiOperationListCall<'a, C, A>
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
/// A builder for the *operations.get* method supported by a *riskAnalysi* resource.
/// It is not used directly, but through a `RiskAnalysiMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_dlp2_beta1 as dlp2_beta1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use dlp2_beta1::DLP;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DLP::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.risk_analysis().operations_get("name")
///              .doit();
/// # }
/// ```
pub struct RiskAnalysiOperationGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DLP<C, A>,
    _name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for RiskAnalysiOperationGetCall<'a, C, A> {}

impl<'a, C, A> RiskAnalysiOperationGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GoogleLongrunningOperation)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "dlp.riskAnalysis.operations.get",
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

        let mut url = self.hub._base_url.clone() + "v2beta1/{+name}";
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


    /// The name of the operation resource.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> RiskAnalysiOperationGetCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> RiskAnalysiOperationGetCall<'a, C, A> {
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
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> RiskAnalysiOperationGetCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> RiskAnalysiOperationGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Cancels an operation. Use the `inspect.operations.get` to check whether the cancellation succeeded or the operation completed despite cancellation.
///
/// A builder for the *operations.cancel* method supported by a *riskAnalysi* resource.
/// It is not used directly, but through a `RiskAnalysiMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_dlp2_beta1 as dlp2_beta1;
/// use dlp2_beta1::GoogleLongrunningCancelOperationRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use dlp2_beta1::DLP;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DLP::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleLongrunningCancelOperationRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.risk_analysis().operations_cancel(req, "name")
///              .doit();
/// # }
/// ```
pub struct RiskAnalysiOperationCancelCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DLP<C, A>,
    _request: GoogleLongrunningCancelOperationRequest,
    _name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for RiskAnalysiOperationCancelCall<'a, C, A> {}

impl<'a, C, A> RiskAnalysiOperationCancelCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GoogleProtobufEmpty)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "dlp.riskAnalysis.operations.cancel",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
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

        let mut url = self.hub._base_url.clone() + "v2beta1/{+name}:cancel";
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
    pub fn request(mut self, new_value: GoogleLongrunningCancelOperationRequest) -> RiskAnalysiOperationCancelCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The name of the operation resource to be cancelled.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> RiskAnalysiOperationCancelCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> RiskAnalysiOperationCancelCall<'a, C, A> {
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
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> RiskAnalysiOperationCancelCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> RiskAnalysiOperationCancelCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Schedules a job to compute risk analysis metrics over content in a Google
/// Cloud Platform repository.
///
/// A builder for the *analyze* method supported by a *dataSource* resource.
/// It is not used directly, but through a `DataSourceMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_dlp2_beta1 as dlp2_beta1;
/// use dlp2_beta1::GooglePrivacyDlpV2beta1AnalyzeDataSourceRiskRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use dlp2_beta1::DLP;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DLP::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GooglePrivacyDlpV2beta1AnalyzeDataSourceRiskRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.data_source().analyze(req)
///              .doit();
/// # }
/// ```
pub struct DataSourceAnalyzeCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DLP<C, A>,
    _request: GooglePrivacyDlpV2beta1AnalyzeDataSourceRiskRequest,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for DataSourceAnalyzeCall<'a, C, A> {}

impl<'a, C, A> DataSourceAnalyzeCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GoogleLongrunningOperation)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "dlp.dataSource.analyze",
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

        let mut url = self.hub._base_url.clone() + "v2beta1/dataSource:analyze";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
    pub fn request(mut self, new_value: GooglePrivacyDlpV2beta1AnalyzeDataSourceRiskRequest) -> DataSourceAnalyzeCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DataSourceAnalyzeCall<'a, C, A> {
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
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> DataSourceAnalyzeCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> DataSourceAnalyzeCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


