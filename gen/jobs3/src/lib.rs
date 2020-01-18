// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Cloud Talent Solution* crate version *1.0.12+20190702*, where *20190702* is the exact revision of the *jobs:v3* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.12*.
//! 
//! Everything else about the *Cloud Talent Solution* *v3* API can be found at the
//! [official documentation site](https://cloud.google.com/talent-solution/job-search/docs/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/jobs3).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.CloudTalentSolution.html) ... 
//! 
//! * projects
//!  * [*client events create*](struct.ProjectClientEventCreateCall.html), [*companies create*](struct.ProjectCompanyCreateCall.html), [*companies delete*](struct.ProjectCompanyDeleteCall.html), [*companies get*](struct.ProjectCompanyGetCall.html), [*companies list*](struct.ProjectCompanyListCall.html), [*companies patch*](struct.ProjectCompanyPatchCall.html), [*complete*](struct.ProjectCompleteCall.html), [*jobs batch delete*](struct.ProjectJobBatchDeleteCall.html), [*jobs create*](struct.ProjectJobCreateCall.html), [*jobs delete*](struct.ProjectJobDeleteCall.html), [*jobs get*](struct.ProjectJobGetCall.html), [*jobs list*](struct.ProjectJobListCall.html), [*jobs patch*](struct.ProjectJobPatchCall.html), [*jobs search*](struct.ProjectJobSearchCall.html) and [*jobs search for alert*](struct.ProjectJobSearchForAlertCall.html)
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
//! * **[Hub](struct.CloudTalentSolution.html)**
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
//! let r = hub.projects().jobs_create(...).doit()
//! let r = hub.projects().jobs_patch(...).doit()
//! let r = hub.projects().jobs_get(...).doit()
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
//! google-jobs3 = "*"
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
//! extern crate google_jobs3 as jobs3;
//! use jobs3::CreateJobRequest;
//! use jobs3::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use jobs3::CloudTalentSolution;
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
//! let mut hub = CloudTalentSolution::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = CreateJobRequest::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.projects().jobs_create(req, "parent")
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
    /// Manage job postings
    Full,

    /// View and manage your data across Google Cloud Platform services
    CloudPlatform,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Full => "https://www.googleapis.com/auth/jobs",
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

/// Central instance to access all CloudTalentSolution related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_jobs3 as jobs3;
/// use jobs3::CreateJobRequest;
/// use jobs3::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use jobs3::CloudTalentSolution;
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
/// let mut hub = CloudTalentSolution::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = CreateJobRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().jobs_create(req, "parent")
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
pub struct CloudTalentSolution<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for CloudTalentSolution<C, A> {}

impl<'a, C, A> CloudTalentSolution<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> CloudTalentSolution<C, A> {
        CloudTalentSolution {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.12".to_string(),
            _base_url: "https://jobs.googleapis.com/".to_string(),
            _root_url: "https://jobs.googleapis.com/".to_string(),
        }
    }

    pub fn projects(&'a self) -> ProjectMethods<'a, C, A> {
        ProjectMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.12`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://jobs.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://jobs.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// Output only.
/// 
/// Spell check result.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpellingCorrection {
    /// Indicates if the query was corrected by the spell checker.
    pub corrected: Option<bool>,
    /// Correction output consisting of the corrected keyword string.
    #[serde(rename="correctedText")]
    pub corrected_text: Option<String>,
}

impl Part for SpellingCorrection {}


/// Input only.
/// 
/// Geographic region of the search.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocationFilter {
    /// Optional.
    /// 
    /// 
    /// The distance_in_miles is applied when the location being searched for is
    /// identified as a city or smaller. When the location being searched for is a
    /// state or larger, this field is ignored.
    #[serde(rename="distanceInMiles")]
    pub distance_in_miles: Option<f64>,
    /// Optional.
    /// 
    /// The latitude and longitude of the geographic center from which to
    /// search. This field's ignored if `address` is provided.
    #[serde(rename="latLng")]
    pub lat_lng: Option<LatLng>,
    /// Optional.
    /// 
    /// The address name, such as "Mountain View" or "Bay Area".
    pub address: Option<String>,
    /// Optional.
    /// 
    /// Allows the client to return jobs without a
    /// set location, specifically, telecommuting jobs (telecommuting is considered
    /// by the service as a special location.
    /// Job.posting_region indicates if a job permits telecommuting.
    /// If this field is set to TelecommutePreference.TELECOMMUTE_ALLOWED,
    /// telecommuting jobs are searched, and address and lat_lng are
    /// ignored. If not set or set to
    /// TelecommutePreference.TELECOMMUTE_EXCLUDED, telecommute job are not
    /// searched.
    /// 
    /// This filter can be used by itself to search exclusively for telecommuting
    /// jobs, or it can be combined with another location
    /// filter to search for a combination of job locations,
    /// such as "Mountain View" or "telecommuting" jobs. However, when used in
    /// combination with other location filters, telecommuting jobs can be
    /// treated as less relevant than other jobs in the search response.
    #[serde(rename="telecommutePreference")]
    pub telecommute_preference: Option<String>,
    /// Optional.
    /// 
    /// CLDR region code of the country/region of the address. This is used
    /// to address ambiguity of the user-input location, for example, "Liverpool"
    /// against "Liverpool, NY, US" or "Liverpool, UK".
    /// 
    /// Set this field if all the jobs to search against are from a same region,
    /// or jobs are world-wide, but the job seeker is from a specific region.
    /// 
    /// See http://cldr.unicode.org/ and
    /// http://www.unicode.org/cldr/charts/30/supplemental/territory_information.html
    /// for details. Example: "CH" for Switzerland.
    #[serde(rename="regionCode")]
    pub region_code: Option<String>,
}

impl Part for LocationFilter {}


/// Input only.
/// 
/// Create job request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [jobs create projects](struct.ProjectJobCreateCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateJobRequest {
    /// Required.
    /// 
    /// The Job to be created.
    pub job: Option<Job>,
}

impl RequestValue for CreateJobRequest {}


/// Compensation range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CompensationRange {
    /// Optional.
    /// 
    /// The minimum amount of compensation. If left empty, the value is set
    /// to zero and the currency code is set to match the
    /// currency code of max_compensation.
    #[serde(rename="minCompensation")]
    pub min_compensation: Option<Money>,
    /// Optional.
    /// 
    /// The maximum amount of compensation. If left empty, the value is set
    /// to a maximal compensation value and the currency code is set to
    /// match the currency code of
    /// min_compensation.
    #[serde(rename="maxCompensation")]
    pub max_compensation: Option<Money>,
}

impl Part for CompensationRange {}


/// Derived details about the company.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CompanyDerivedInfo {
    /// A structured headquarters location of the company, resolved from
    /// Company.hq_location if provided.
    #[serde(rename="headquartersLocation")]
    pub headquarters_location: Option<Location>,
}

impl Part for CompanyDerivedInfo {}


/// Output only.
/// 
/// Response for SearchJob method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [jobs search for alert projects](struct.ProjectJobSearchForAlertCall.html) (response)
/// * [jobs search projects](struct.ProjectJobSearchCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchJobsResponse {
    /// The token that specifies the starting position of the next page of results.
    /// This field is empty if there are no more results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// The spell checking result, and correction.
    #[serde(rename="spellCorrection")]
    pub spell_correction: Option<SpellingCorrection>,
    /// An estimation of the number of jobs that match the specified query.
    /// 
    /// This number is not guaranteed to be accurate. For accurate results,
    /// see enable_precise_result_size.
    #[serde(rename="estimatedTotalSize")]
    pub estimated_total_size: Option<i32>,
    /// The precise result count, which is available only if the client set
    /// enable_precise_result_size to `true`, or if the response
    /// is the last page of results. Otherwise, the value is `-1`.
    #[serde(rename="totalSize")]
    pub total_size: Option<i32>,
    /// The Job entities that match the specified SearchJobsRequest.
    #[serde(rename="matchingJobs")]
    pub matching_jobs: Option<Vec<MatchingJob>>,
    /// The histogram results that match specified
    /// SearchJobsRequest.histogram_facets.
    #[serde(rename="histogramResults")]
    pub histogram_results: Option<HistogramResults>,
    /// If query broadening is enabled, we may append additional results from the
    /// broadened query. This number indicates how many of the jobs returned in the
    /// jobs field are from the broadened query. These results are always at the
    /// end of the jobs list. In particular, a value of 0, or if the field isn't
    /// set, all the jobs in the jobs list are from the original
    /// (without broadening) query. If this field is non-zero, subsequent requests
    /// with offset after this result set should contain all broadened results.
    #[serde(rename="broadenedQueryJobsCount")]
    pub broadened_query_jobs_count: Option<i32>,
    /// The location filters that the service applied to the specified query. If
    /// any filters are lat-lng based, the JobLocation.location_type is
    /// JobLocation.LocationType#LOCATION_TYPE_UNSPECIFIED.
    #[serde(rename="locationFilters")]
    pub location_filters: Option<Vec<Location>>,
    /// Additional information for the API invocation, such as the request
    /// tracking id.
    pub metadata: Option<ResponseMetadata>,
}

impl ResponseResult for SearchJobsResponse {}


/// Output only.
/// 
/// A resource that represents a location with full geographic information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    /// The type of a location, which corresponds to the address lines field of
    /// PostalAddress. For example, "Downtown, Atlanta, GA, USA" has a type of
    /// LocationType#NEIGHBORHOOD, and "Kansas City, KS, USA" has a type of
    /// LocationType#LOCALITY.
    #[serde(rename="locationType")]
    pub location_type: Option<String>,
    /// Radius in miles of the job location. This value is derived from the
    /// location bounding box in which a circle with the specified radius
    /// centered from LatLng covers the area associated with the job location.
    /// For example, currently, "Mountain View, CA, USA" has a radius of
    /// 6.17 miles.
    #[serde(rename="radiusInMiles")]
    pub radius_in_miles: Option<f64>,
    /// An object representing a latitude/longitude pair.
    #[serde(rename="latLng")]
    pub lat_lng: Option<LatLng>,
    /// Postal address of the location that includes human readable information,
    /// such as postal delivery and payments addresses. Given a postal address,
    /// a postal service can deliver items to a premises, P.O. Box, or other
    /// delivery location.
    #[serde(rename="postalAddress")]
    pub postal_address: Option<PostalAddress>,
}

impl Part for Location {}


/// Output only.
/// 
/// Job entry with metadata inside SearchJobsResponse.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MatchingJob {
    /// Contains snippets of text from the Job.description and similar
    /// fields that most closely match a search query's keywords, if available.
    /// All HTML tags in the original fields are stripped when returned in this
    /// field, and matching query keywords are enclosed in HTML bold tags.
    #[serde(rename="searchTextSnippet")]
    pub search_text_snippet: Option<String>,
    /// Job resource that matches the specified SearchJobsRequest.
    pub job: Option<Job>,
    /// Commute information which is generated based on specified
    ///  CommuteFilter.
    #[serde(rename="commuteInfo")]
    pub commute_info: Option<CommuteInfo>,
    /// A summary of the job with core information that's displayed on the search
    /// results listing page.
    #[serde(rename="jobSummary")]
    pub job_summary: Option<String>,
    /// Contains snippets of text from the Job.job_title field most
    /// closely matching a search query's keywords, if available. The matching
    /// query keywords are enclosed in HTML bold tags.
    #[serde(rename="jobTitleSnippet")]
    pub job_title_snippet: Option<String>,
}

impl Part for MatchingJob {}


/// Output only.
/// 
/// The List companies response object.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [companies list projects](struct.ProjectCompanyListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCompaniesResponse {
    /// A token to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// Companies for the current client.
    pub companies: Option<Vec<Company>>,
    /// Additional information for the API invocation, such as the request
    /// tracking id.
    pub metadata: Option<ResponseMetadata>,
}

impl ResponseResult for ListCompaniesResponse {}


/// Represents a time of day. The date and time zone are either not significant
/// or are specified elsewhere. An API may choose to allow leap seconds. Related
/// types are google.type.Date and `google.protobuf.Timestamp`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeOfDay {
    /// Hours of day in 24 hour format. Should be from 0 to 23. An API may choose
    /// to allow the value "24:00:00" for scenarios like business closing time.
    pub hours: Option<i32>,
    /// Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999.
    pub nanos: Option<i32>,
    /// Minutes of hour of day. Must be from 0 to 59.
    pub minutes: Option<i32>,
    /// Seconds of minutes of the time. Must normally be from 0 to 59. An API may
    /// allow the value 60 if it allows leap-seconds.
    pub seconds: Option<i32>,
}

impl Part for TimeOfDay {}


/// Input only.
/// 
/// Batch delete jobs request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [jobs batch delete projects](struct.ProjectJobBatchDeleteCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchDeleteJobsRequest {
    /// Required.
    /// 
    /// The filter string specifies the jobs to be deleted.
    /// 
    /// Supported operator: =, AND
    /// 
    /// The fields eligible for filtering are:
    /// 
    /// * `companyName` (Required)
    /// * `requisitionId` (Required)
    /// 
    /// Sample Query: companyName = "projects/api-test-project/companies/123" AND
    /// requisitionId = "req-1"
    pub filter: Option<String>,
}

impl RequestValue for BatchDeleteJobsRequest {}


/// A generic empty message that you can re-use to avoid defining duplicated
/// empty messages in your APIs. A typical example is to use it as the request
/// or the response type of an API method. For instance:
/// 
/// ````text
/// service Foo {
///   rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty);
/// }
/// ````
/// 
/// The JSON representation for `Empty` is empty JSON object `{}`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [jobs delete projects](struct.ProjectJobDeleteCall.html) (response)
/// * [jobs batch delete projects](struct.ProjectJobBatchDeleteCall.html) (response)
/// * [companies delete projects](struct.ProjectCompanyDeleteCall.html) (response)
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl ResponseResult for Empty {}


/// Output only.
/// 
/// Commute details related to this job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommuteInfo {
    /// The number of seconds required to travel to the job location from the
    /// query location. A duration of 0 seconds indicates that the job is not
    /// reachable within the requested duration, but was returned as part of an
    /// expanded query.
    #[serde(rename="travelDuration")]
    pub travel_duration: Option<String>,
    /// Location used as the destination in the commute calculation.
    #[serde(rename="jobLocation")]
    pub job_location: Option<Location>,
}

impl Part for CommuteInfo {}


/// Input only.
/// 
/// Request for updating a specified company.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [companies patch projects](struct.ProjectCompanyPatchCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateCompanyRequest {
    /// Required.
    /// 
    /// The company resource to replace the current resource in the system.
    pub company: Option<Company>,
    /// Optional but strongly recommended for the best service
    /// experience.
    /// 
    /// If update_mask is provided, only the specified fields in
    /// company are updated. Otherwise all the fields are updated.
    /// 
    /// A field mask to specify the company fields to be updated. Only
    /// top level fields of Company are supported.
    #[serde(rename="updateMask")]
    pub update_mask: Option<String>,
}

impl RequestValue for UpdateCompanyRequest {}


/// Custom attributes histogram request. An error is thrown if neither
/// string_value_histogram or long_value_histogram_bucketing_option has
/// been defined.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomAttributeHistogramRequest {
    /// Optional. If set to true, the response includes the histogram value for
    /// each key as a string.
    #[serde(rename="stringValueHistogram")]
    pub string_value_histogram: Option<bool>,
    /// Optional.
    /// 
    /// Specifies buckets used to perform a range histogram on Job's
    /// filterable long custom field values, or min/max value requirements.
    #[serde(rename="longValueHistogramBucketingOption")]
    pub long_value_histogram_bucketing_option: Option<NumericBucketingOption>,
    /// Required.
    /// 
    /// Specifies the custom field key to perform a histogram on. If specified
    /// without `long_value_histogram_bucketing_option`, histogram on string values
    /// of the given `key` is triggered, otherwise histogram is performed on long
    /// values.
    pub key: Option<String>,
}

impl Part for CustomAttributeHistogramRequest {}


/// A Company resource represents a company in the service. A company is the
/// entity that owns job postings, that is, the hiring entity responsible for
/// employing applicants for the job position.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [companies patch projects](struct.ProjectCompanyPatchCall.html) (response)
/// * [companies get projects](struct.ProjectCompanyGetCall.html) (response)
/// * [companies create projects](struct.ProjectCompanyCreateCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Company {
    /// Required.
    /// 
    /// The display name of the company, for example, "Google, LLC".
    #[serde(rename="displayName")]
    pub display_name: Option<String>,
    /// Required during company update.
    /// 
    /// The resource name for a company. This is generated by the service when a
    /// company is created.
    /// 
    /// The format is "projects/{project_id}/companies/{company_id}", for example,
    /// "projects/api-test-project/companies/foo".
    pub name: Option<String>,
    /// Optional.
    /// 
    /// The URI to employer's career site or careers page on the employer's web
    /// site, for example, "https://careers.google.com".
    #[serde(rename="careerSiteUri")]
    pub career_site_uri: Option<String>,
    /// Output only. Derived details about the company.
    #[serde(rename="derivedInfo")]
    pub derived_info: Option<CompanyDerivedInfo>,
    /// Optional.
    /// 
    /// Equal Employment Opportunity legal disclaimer text to be
    /// associated with all jobs, and typically to be displayed in all
    /// roles.
    /// 
    /// The maximum number of allowed characters is 500.
    #[serde(rename="eeoText")]
    pub eeo_text: Option<String>,
    /// Optional.
    /// 
    /// A URI that hosts the employer's company logo.
    #[serde(rename="imageUri")]
    pub image_uri: Option<String>,
    /// Optional.
    /// 
    /// The street address of the company's main headquarters, which may be
    /// different from the job location. The service attempts
    /// to geolocate the provided address, and populates a more specific
    /// location wherever possible in DerivedInfo.headquarters_location.
    #[serde(rename="headquartersAddress")]
    pub headquarters_address: Option<String>,
    /// Required.
    /// 
    /// Client side company identifier, used to uniquely identify the
    /// company.
    /// 
    /// The maximum number of allowed characters is 255.
    #[serde(rename="externalId")]
    pub external_id: Option<String>,
    /// Optional.
    /// 
    /// The URI representing the company's primary web site or home page,
    /// for example, "https://www.google.com".
    /// 
    /// The maximum number of allowed characters is 255.
    #[serde(rename="websiteUri")]
    pub website_uri: Option<String>,
    /// Output only. Indicates whether a company is flagged to be suspended from
    /// public availability by the service when job content appears suspicious,
    /// abusive, or spammy.
    pub suspended: Option<bool>,
    /// Optional.
    /// 
    /// Set to true if it is the hiring agency that post jobs for other
    /// employers.
    /// 
    /// Defaults to false if not provided.
    #[serde(rename="hiringAgency")]
    pub hiring_agency: Option<bool>,
    /// Optional.
    /// 
    /// A list of keys of filterable Job.custom_attributes, whose
    /// corresponding `string_values` are used in keyword search. Jobs with
    /// `string_values` under these specified field keys are returned if any
    /// of the values matches the search keyword. Custom field values with
    /// parenthesis, brackets and special symbols won't be properly searchable,
    /// and those keyword queries need to be surrounded by quotes.
    #[serde(rename="keywordSearchableJobCustomAttributes")]
    pub keyword_searchable_job_custom_attributes: Option<Vec<String>>,
    /// Optional.
    /// 
    /// The employer's company size.
    pub size: Option<String>,
}

impl ResponseResult for Company {}


/// Output only.
/// 
/// Resource that represents completion results.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CompletionResult {
    /// The completion topic.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The suggestion for the query.
    pub suggestion: Option<String>,
    /// The URI of the company image for CompletionType.COMPANY_NAME.
    #[serde(rename="imageUri")]
    pub image_uri: Option<String>,
}

impl Part for CompletionResult {}


/// Input only.
/// 
/// The Request of the CreateCompany method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [companies create projects](struct.ProjectCompanyCreateCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateCompanyRequest {
    /// Required.
    /// 
    /// The company to be created.
    pub company: Option<Company>,
}

impl RequestValue for CreateCompanyRequest {}


/// A Job resource represents a job posting (also referred to as a "job listing"
/// or "job requisition"). A job belongs to a Company, which is the hiring
/// entity responsible for the job.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [jobs create projects](struct.ProjectJobCreateCall.html) (response)
/// * [jobs patch projects](struct.ProjectJobPatchCall.html) (response)
/// * [jobs get projects](struct.ProjectJobGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Job {
    /// Optional.
    /// 
    /// The language of the posting. This field is distinct from
    /// any requirements for fluency that are associated with the job.
    /// 
    /// Language codes must be in BCP-47 format, such as "en-US" or "sr-Latn".
    /// For more information, see
    /// [Tags for Identifying Languages](https://tools.ietf.org/html/bcp47){:
    /// class="external" target="_blank" }.
    /// 
    /// If this field is unspecified and Job.description is present, detected
    /// language code based on Job.description is assigned, otherwise
    /// defaults to 'en_US'.
    #[serde(rename="languageCode")]
    pub language_code: Option<String>,
    /// Optional.
    /// 
    /// Options for job processing.
    #[serde(rename="processingOptions")]
    pub processing_options: Option<ProcessingOptions>,
    /// Optional but strongly recommended for the best service experience.
    /// 
    /// Location(s) where the employer is looking to hire for this job posting.
    /// 
    /// Specifying the full street address(es) of the hiring location enables
    /// better API results, especially job searches by commute time.
    /// 
    /// At most 50 locations are allowed for best search performance. If a job has
    /// more locations, it is suggested to split it into multiple jobs with unique
    /// requisition_ids (e.g. 'ReqA' becomes 'ReqA-1', 'ReqA-2', etc.) as
    /// multiple jobs with the same company_name, language_code and
    /// requisition_id are not allowed. If the original requisition_id must
    /// be preserved, a custom field should be used for storage. It is also
    /// suggested to group the locations that close to each other in the same job
    /// for better search experience.
    /// 
    /// The maximum number of allowed characters is 500.
    pub addresses: Option<Vec<String>>,
    /// Output only. Display name of the company listing the job.
    #[serde(rename="companyDisplayName")]
    pub company_display_name: Option<String>,
    /// Required.
    /// 
    /// The resource name of the company listing the job, such as
    /// "projects/api-test-project/companies/foo".
    #[serde(rename="companyName")]
    pub company_name: Option<String>,
    /// Output only. Derived details about the job posting.
    #[serde(rename="derivedInfo")]
    pub derived_info: Option<JobDerivedInfo>,
    /// Optional.
    /// 
    /// A description of bonus, commission, and other compensation
    /// incentives associated with the job not including salary or pay.
    /// 
    /// The maximum number of allowed characters is 10,000.
    pub incentives: Option<String>,
    /// Optional.
    /// 
    /// The benefits included with the job.
    #[serde(rename="jobBenefits")]
    pub job_benefits: Option<Vec<String>>,
    /// Optional.
    /// 
    /// A description of the qualifications required to perform the
    /// job. The use of this field is recommended
    /// as an alternative to using the more general description field.
    /// 
    /// This field accepts and sanitizes HTML input, and also accepts
    /// bold, italic, ordered list, and unordered list markup tags.
    /// 
    /// The maximum number of allowed characters is 10,000.
    pub qualifications: Option<String>,
    /// Optional.
    /// 
    /// The employment type(s) of a job, for example,
    /// full time or
    /// part time.
    #[serde(rename="employmentTypes")]
    pub employment_types: Option<Vec<String>>,
    /// Output only. The timestamp when this job posting was created.
    #[serde(rename="postingCreateTime")]
    pub posting_create_time: Option<String>,
    /// Optional.
    /// 
    /// The visibility of the job.
    /// 
    /// Defaults to Visibility.ACCOUNT_ONLY if not specified.
    pub visibility: Option<String>,
    /// Optional.
    /// 
    /// The end timestamp of the job. Typically this field is used for contracting
    /// engagements. Invalid timestamps are ignored.
    #[serde(rename="jobEndTime")]
    pub job_end_time: Option<String>,
    /// Optional.
    /// 
    /// The job PostingRegion (for example, state, country) throughout which
    /// the job is available. If this field is set, a
    /// LocationFilter in a search query within the job region
    /// finds this job posting if an exact location match isn't specified.
    /// If this field is set to PostingRegion.NATION or
    /// PostingRegion.ADMINISTRATIVE_AREA, setting job Job.addresses
    /// to the same location level as this field is strongly recommended.
    #[serde(rename="postingRegion")]
    pub posting_region: Option<String>,
    /// Required. At least one field within ApplicationInfo must be specified.
    /// 
    /// Job application information.
    #[serde(rename="applicationInfo")]
    pub application_info: Option<ApplicationInfo>,
    /// Optional but strongly recommended for the best service
    /// experience.
    /// 
    /// The expiration timestamp of the job. After this timestamp, the
    /// job is marked as expired, and it no longer appears in search results. The
    /// expired job can't be deleted or listed by the DeleteJob and
    /// ListJobs APIs, but it can be retrieved with the GetJob API or
    /// updated with the UpdateJob API. An expired job can be updated and
    /// opened again by using a future expiration timestamp. Updating an expired
    /// job fails if there is another existing open job with same company_name,
    /// language_code and requisition_id.
    /// 
    /// The expired jobs are retained in our system for 90 days. However, the
    /// overall expired job count cannot exceed 3 times the maximum of open jobs
    /// count over the past week, otherwise jobs with earlier expire time are
    /// cleaned first. Expired jobs are no longer accessible after they are cleaned
    /// out.
    /// 
    /// Invalid timestamps are ignored, and treated as expire time not provided.
    /// 
    /// Timestamp before the instant request is made is considered valid, the job
    /// will be treated as expired immediately.
    /// 
    /// If this value is not provided at the time of job creation or is invalid,
    /// the job posting expires after 30 days from the job's creation time. For
    /// example, if the job was created on 2017/01/01 13:00AM UTC with an
    /// unspecified expiration date, the job expires after 2017/01/31 13:00AM UTC.
    /// 
    /// If this value is not provided on job update, it depends on the field masks
    /// set by UpdateJobRequest.update_mask. If the field masks include
    /// expiry_time, or the masks are empty meaning that every field is
    /// updated, the job posting expires after 30 days from the job's last
    /// update time. Otherwise the expiration date isn't updated.
    #[serde(rename="postingExpireTime")]
    pub posting_expire_time: Option<String>,
    /// Required during job update.
    /// 
    /// The resource name for the job. This is generated by the service when a
    /// job is created.
    /// 
    /// The format is "projects/{project_id}/jobs/{job_id}",
    /// for example, "projects/api-test-project/jobs/1234".
    /// 
    /// Use of this field in job queries and API calls is preferred over the use of
    /// requisition_id since this value is unique.
    pub name: Option<String>,
    /// Required.
    /// 
    /// The title of the job, such as "Software Engineer"
    /// 
    /// The maximum number of allowed characters is 500.
    pub title: Option<String>,
    /// Required.
    /// 
    /// The description of the job, which typically includes a multi-paragraph
    /// description of the company and related information. Separate fields are
    /// provided on the job object for responsibilities,
    /// qualifications, and other job characteristics. Use of
    /// these separate job fields is recommended.
    /// 
    /// This field accepts and sanitizes HTML input, and also accepts
    /// bold, italic, ordered list, and unordered list markup tags.
    /// 
    /// The maximum number of allowed characters is 100,000.
    pub description: Option<String>,
    /// Optional.
    /// 
    /// A promotion value of the job, as determined by the client.
    /// The value determines the sort order of the jobs returned when searching for
    /// jobs using the featured jobs search call, with higher promotional values
    /// being returned first and ties being resolved by relevance sort. Only the
    /// jobs with a promotionValue >0 are returned in a FEATURED_JOB_SEARCH.
    /// 
    /// Default value is 0, and negative values are treated as 0.
    #[serde(rename="promotionValue")]
    pub promotion_value: Option<i32>,
    /// Optional.
    /// 
    /// A map of fields to hold both filterable and non-filterable custom job
    /// attributes that are not covered by the provided structured fields.
    /// 
    /// The keys of the map are strings up to 64 bytes and must match the
    /// pattern: a-zA-Z*. For example, key0LikeThis or
    /// KEY_1_LIKE_THIS.
    /// 
    /// At most 100 filterable and at most 100 unfilterable keys are supported.
    /// For filterable `string_values`, across all keys at most 200 values are
    /// allowed, with each string no more than 255 characters. For unfilterable
    /// `string_values`, the maximum total size of `string_values` across all keys
    /// is 50KB.
    #[serde(rename="customAttributes")]
    pub custom_attributes: Option<HashMap<String, CustomAttribute>>,
    /// Optional.
    /// 
    /// The desired education degrees for the job, such as Bachelors, Masters.
    #[serde(rename="degreeTypes")]
    pub degree_types: Option<Vec<String>>,
    /// Optional.
    /// 
    /// A description of job responsibilities. The use of this field is
    /// recommended as an alternative to using the more general description
    /// field.
    /// 
    /// This field accepts and sanitizes HTML input, and also accepts
    /// bold, italic, ordered list, and unordered list markup tags.
    /// 
    /// The maximum number of allowed characters is 10,000.
    pub responsibilities: Option<String>,
    /// Optional.
    /// 
    /// The start timestamp of the job in UTC time zone. Typically this field
    /// is used for contracting engagements. Invalid timestamps are ignored.
    #[serde(rename="jobStartTime")]
    pub job_start_time: Option<String>,
    /// Optional.
    /// 
    /// Job compensation information.
    #[serde(rename="compensationInfo")]
    pub compensation_info: Option<CompensationInfo>,
    /// Optional.
    /// 
    /// The department or functional area within the company with the open
    /// position.
    /// 
    /// The maximum number of allowed characters is 255.
    pub department: Option<String>,
    /// Optional.
    /// 
    /// The experience level associated with the job, such as "Entry Level".
    #[serde(rename="jobLevel")]
    pub job_level: Option<String>,
    /// Output only. The timestamp when this job posting was last updated.
    #[serde(rename="postingUpdateTime")]
    pub posting_update_time: Option<String>,
    /// Required.
    /// 
    /// The requisition ID, also referred to as the posting ID, assigned by the
    /// client to identify a job. This field is intended to be used by clients
    /// for client identification and tracking of postings. A job is not allowed
    /// to be created if there is another job with the same [company_name],
    /// language_code and requisition_id.
    /// 
    /// The maximum number of allowed characters is 255.
    #[serde(rename="requisitionId")]
    pub requisition_id: Option<String>,
    /// Optional.
    /// 
    /// The timestamp this job posting was most recently published. The default
    /// value is the time the request arrives at the server. Invalid timestamps are
    /// ignored.
    #[serde(rename="postingPublishTime")]
    pub posting_publish_time: Option<String>,
}

impl ResponseResult for Job {}


/// Input only.
/// 
/// Meta information related to the job searcher or entity
/// conducting the job search. This information is used to improve the
/// performance of the service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RequestMetadata {
    /// Optional.
    /// 
    /// The type of device used by the job seeker at the time of the call to the
    /// service.
    #[serde(rename="deviceInfo")]
    pub device_info: Option<DeviceInfo>,
    /// Required.
    /// 
    /// The client-defined scope or source of the service call, which typically
    /// is the domain on
    /// which the service has been implemented and is currently being run.
    /// 
    /// For example, if the service is being run by client <em>Foo, Inc.</em>, on
    /// job board www.foo.com and career site www.bar.com, then this field is
    /// set to "foo.com" for use on the job board, and "bar.com" for use on the
    /// career site.
    /// 
    /// If this field isn't available for some reason, send "UNKNOWN".
    /// Any improvements to the model for a particular tenant site rely on this
    /// field being set correctly to a domain.
    /// 
    /// The maximum number of allowed characters is 255.
    pub domain: Option<String>,
    /// Required.
    /// 
    /// A unique user identification string, as determined by the client.
    /// To have the strongest positive impact on search quality
    /// make sure the client-level is unique.
    /// Obfuscate this field for privacy concerns before
    /// providing it to the service.
    /// 
    /// If this field is not available for some reason, send "UNKNOWN". Note
    /// that any improvements to the model for a particular tenant
    /// site, rely on this field being set correctly to a unique user_id.
    /// 
    /// The maximum number of allowed characters is 255.
    #[serde(rename="userId")]
    pub user_id: Option<String>,
    /// Required.
    /// 
    /// A unique session identification string. A session is defined as the
    /// duration of an end user's interaction with the service over a certain
    /// period.
    /// Obfuscate this field for privacy concerns before
    /// providing it to the service.
    /// 
    /// If this field is not available for some reason, send "UNKNOWN". Note
    /// that any improvements to the model for a particular tenant
    /// site, rely on this field being set correctly to some unique session_id.
    /// 
    /// The maximum number of allowed characters is 255.
    #[serde(rename="sessionId")]
    pub session_id: Option<String>,
}

impl Part for RequestMetadata {}


/// Custom attribute values that are either filterable or non-filterable.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomAttribute {
    /// Optional.
    /// 
    /// If the `filterable` flag is true, custom field values are searchable.
    /// If false, values are not searchable.
    /// 
    /// Default is false.
    pub filterable: Option<bool>,
    /// Optional but exactly one of string_values or long_values must
    /// be specified.
    /// 
    /// This field is used to perform a string match (`CASE_SENSITIVE_MATCH` or
    /// `CASE_INSENSITIVE_MATCH`) search.
    /// For filterable `string_value`s, a maximum total number of 200 values
    /// is allowed, with each `string_value` has a byte size of no more than
    /// 255B. For unfilterable `string_values`, the maximum total byte size of
    /// unfilterable `string_values` is 50KB.
    /// 
    /// Empty string is not allowed.
    #[serde(rename="stringValues")]
    pub string_values: Option<Vec<String>>,
    /// Optional but exactly one of string_values or long_values must
    /// be specified.
    /// 
    /// This field is used to perform number range search.
    /// (`EQ`, `GT`, `GE`, `LE`, `LT`) over filterable `long_value`.
    /// 
    /// Currently at most 1 long_values is supported.
    #[serde(rename="longValues")]
    pub long_values: Option<Vec<String>>,
}

impl Part for CustomAttribute {}


/// A compensation entry that represents one component of compensation, such
/// as base pay, bonus, or other compensation type.
/// 
/// Annualization: One compensation entry can be annualized if
/// - it contains valid amount or range.
/// - and its expected_units_per_year is set or can be derived.
/// Its annualized range is determined as (amount or range) times
/// expected_units_per_year.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CompensationEntry {
    /// Optional.
    /// 
    /// Compensation amount.
    pub amount: Option<Money>,
    /// Optional.
    /// 
    /// Compensation description.  For example, could
    /// indicate equity terms or provide additional context to an estimated
    /// bonus.
    pub description: Option<String>,
    /// Optional.
    /// 
    /// Compensation range.
    pub range: Option<CompensationRange>,
    /// Optional.
    /// 
    /// Compensation type.
    /// 
    /// Default is CompensationUnit.COMPENSATION_TYPE_UNSPECIFIED.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// Optional.
    /// 
    /// Expected number of units paid each year. If not specified, when
    /// Job.employment_types is FULLTIME, a default value is inferred
    /// based on unit. Default values:
    /// - HOURLY: 2080
    /// - DAILY: 260
    /// - WEEKLY: 52
    /// - MONTHLY: 12
    /// - ANNUAL: 1
    #[serde(rename="expectedUnitsPerYear")]
    pub expected_units_per_year: Option<f64>,
    /// Optional.
    /// 
    /// Frequency of the specified amount.
    /// 
    /// Default is CompensationUnit.COMPENSATION_UNIT_UNSPECIFIED.
    pub unit: Option<String>,
}

impl Part for CompensationEntry {}


/// Message representing a period of time between two timestamps.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimestampRange {
    /// End of the period.
    #[serde(rename="endTime")]
    pub end_time: Option<String>,
    /// Begin of the period.
    #[serde(rename="startTime")]
    pub start_time: Option<String>,
}

impl Part for TimestampRange {}


/// Input only.
/// 
/// Filter on job compensation type and amount.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CompensationFilter {
    /// Required.
    /// 
    /// Specify desired `base compensation entry's`
    /// CompensationInfo.CompensationUnit.
    pub units: Option<Vec<String>>,
    /// Optional.
    /// 
    /// Compensation range.
    pub range: Option<CompensationRange>,
    /// Required.
    /// 
    /// Type of filter.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// Optional.
    /// 
    /// If set to true, jobs with unspecified compensation range fields are
    /// included.
    #[serde(rename="includeJobsWithUnspecifiedCompensationRange")]
    pub include_jobs_with_unspecified_compensation_range: Option<bool>,
}

impl Part for CompensationFilter {}


/// Input only.
/// 
/// Histogram facets to be specified in SearchJobsRequest.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HistogramFacets {
    /// Optional.
    /// 
    /// Specifies the simple type of histogram facets, for example,
    /// `COMPANY_SIZE`, `EMPLOYMENT_TYPE` etc.
    #[serde(rename="simpleHistogramFacets")]
    pub simple_histogram_facets: Option<Vec<String>>,
    /// Optional.
    /// 
    /// Specifies the custom attributes histogram requests.
    /// Duplicate values of CustomAttributeHistogramRequest.key are not
    /// allowed.
    #[serde(rename="customAttributeHistogramFacets")]
    pub custom_attribute_histogram_facets: Option<Vec<CustomAttributeHistogramRequest>>,
    /// Optional.
    /// 
    /// Specifies compensation field-based histogram requests.
    /// Duplicate values of CompensationHistogramRequest.type are not allowed.
    #[serde(rename="compensationHistogramFacets")]
    pub compensation_histogram_facets: Option<Vec<CompensationHistogramRequest>>,
}

impl Part for HistogramFacets {}


/// An event issued when an end user interacts with the application that
/// implements Cloud Talent Solution. Providing this information improves the
/// quality of search and recommendation for the API clients, enabling the
/// service to perform optimally. The number of events sent must be consistent
/// with other calls, such as job searches, issued to the service by the client.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [client events create projects](struct.ProjectClientEventCreateCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClientEvent {
    /// Required.
    /// 
    /// A unique identifier, generated by the client application. This `event_id`
    /// is used to establish the relationship between different events
    /// (see parent_event_id).
    #[serde(rename="eventId")]
    pub event_id: Option<String>,
    /// A event issued when a job seeker interacts with the application that
    /// implements Cloud Talent Solution.
    #[serde(rename="jobEvent")]
    pub job_event: Option<JobEvent>,
    /// Required.
    /// 
    /// A unique ID generated in the API responses. It can be found in
    /// ResponseMetadata.request_id.
    #[serde(rename="requestId")]
    pub request_id: Option<String>,
    /// Optional.
    /// 
    /// The event_id of an event that resulted in the current event. For example, a
    /// Job view event usually follows a parent
    /// impression event: A job seeker first does a
    /// search where a list of jobs appears
    /// (impression). The job seeker then selects a
    /// result and views the description of a particular job (Job
    /// view).
    #[serde(rename="parentEventId")]
    pub parent_event_id: Option<String>,
    /// Optional.
    /// 
    /// Extra information about this event. Used for storing information with no
    /// matching field in event payload, for example, user application specific
    /// context or details.
    /// 
    /// At most 20 keys are supported. The maximum total size of all keys and
    /// values is 2 KB.
    #[serde(rename="extraInfo")]
    pub extra_info: Option<HashMap<String, String>>,
    /// Required.
    /// 
    /// The timestamp of the event.
    #[serde(rename="createTime")]
    pub create_time: Option<String>,
}

impl ResponseResult for ClientEvent {}


/// Output only.
/// 
/// Additional information returned to client, such as debugging information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResponseMetadata {
    /// A unique id associated with this call.
    /// This id is logged for tracking purposes.
    #[serde(rename="requestId")]
    pub request_id: Option<String>,
}

impl Part for ResponseMetadata {}


/// Output only.
/// 
/// Histogram results that match HistogramFacets specified in
/// SearchJobsRequest.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HistogramResults {
    /// Specifies histogram results for custom attributes that match
    /// HistogramFacets.custom_attribute_histogram_facets.
    #[serde(rename="customAttributeHistogramResults")]
    pub custom_attribute_histogram_results: Option<Vec<CustomAttributeHistogramResult>>,
    /// Specifies histogram results that matches
    /// HistogramFacets.simple_histogram_facets.
    #[serde(rename="simpleHistogramResults")]
    pub simple_histogram_results: Option<Vec<HistogramResult>>,
    /// Specifies compensation field-based histogram results that match
    /// HistogramFacets.compensation_histogram_requests.
    #[serde(rename="compensationHistogramResults")]
    pub compensation_histogram_results: Option<Vec<CompensationHistogramResult>>,
}

impl Part for HistogramResults {}


/// Input only.
/// 
/// Options for job processing.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProcessingOptions {
    /// Optional.
    /// 
    /// Option for job HTML content sanitization. Applied fields are:
    /// 
    /// * description
    /// * applicationInfo.instruction
    /// * incentives
    /// * qualifications
    /// * responsibilities
    /// 
    /// HTML tags in these fields may be stripped if sanitiazation is not
    /// disabled.
    /// 
    /// Defaults to HtmlSanitization.SIMPLE_FORMATTING_ONLY.
    #[serde(rename="htmlSanitization")]
    pub html_sanitization: Option<String>,
    /// Optional.
    /// 
    /// If set to `true`, the service does not attempt to resolve a
    /// more precise address for the job.
    #[serde(rename="disableStreetAddressResolution")]
    pub disable_street_address_resolution: Option<bool>,
}

impl Part for ProcessingOptions {}


/// Output only.
/// 
/// Response of auto-complete query.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [complete projects](struct.ProjectCompleteCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CompleteQueryResponse {
    /// Results of the matching job/company candidates.
    #[serde(rename="completionResults")]
    pub completion_results: Option<Vec<CompletionResult>>,
    /// Additional information for the API invocation, such as the request
    /// tracking id.
    pub metadata: Option<ResponseMetadata>,
}

impl ResponseResult for CompleteQueryResponse {}


/// Output only.
/// 
/// List jobs response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [jobs list projects](struct.ProjectJobListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListJobsResponse {
    /// A token to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// The Jobs for a given company.
    /// 
    /// The maximum number of items returned is based on the limit field
    /// provided in the request.
    pub jobs: Option<Vec<Job>>,
    /// Additional information for the API invocation, such as the request
    /// tracking id.
    pub metadata: Option<ResponseMetadata>,
}

impl ResponseResult for ListJobsResponse {}


/// Input only.
/// 
/// The Request body of the `SearchJobs` call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [jobs search for alert projects](struct.ProjectJobSearchForAlertCall.html) (request)
/// * [jobs search projects](struct.ProjectJobSearchCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchJobsRequest {
    /// Optional.
    /// 
    /// The criteria determining how search results are sorted. Default is
    /// "relevance desc".
    /// 
    /// Supported options are:
    /// 
    /// * `"relevance desc"`: By relevance descending, as determined by the API
    /// algorithms. Relevance thresholding of query results is only available
    /// with this ordering.
    /// * `"posting_publish_time desc"`: By Job.posting_publish_time
    /// descending.
    /// * `"posting_update_time desc"`: By Job.posting_update_time
    /// descending.
    /// * `"title"`: By Job.title ascending.
    /// * `"title desc"`: By Job.title descending.
    /// * `"annualized_base_compensation"`: By job's
    /// CompensationInfo.annualized_base_compensation_range ascending. Jobs
    /// whose annualized base compensation is unspecified are put at the end of
    /// search results.
    /// * `"annualized_base_compensation desc"`: By job's
    /// CompensationInfo.annualized_base_compensation_range descending. Jobs
    /// whose annualized base compensation is unspecified are put at the end of
    /// search results.
    /// * `"annualized_total_compensation"`: By job's
    /// CompensationInfo.annualized_total_compensation_range ascending. Jobs
    /// whose annualized base compensation is unspecified are put at the end of
    /// search results.
    /// * `"annualized_total_compensation desc"`: By job's
    /// CompensationInfo.annualized_total_compensation_range descending. Jobs
    /// whose annualized base compensation is unspecified are put at the end of
    /// search results.
    #[serde(rename="orderBy")]
    pub order_by: Option<String>,
    /// Optional.
    /// 
    /// Controls whether highly similar jobs are returned next to each other in
    /// the search results. Jobs are identified as highly similar based on
    /// their titles, job categories, and locations. Highly similar results are
    /// clustered so that only one representative job of the cluster is
    /// displayed to the job seeker higher up in the results, with the other jobs
    /// being displayed lower down in the results.
    /// 
    /// Defaults to DiversificationLevel.SIMPLE if no value
    /// is specified.
    #[serde(rename="diversificationLevel")]
    pub diversification_level: Option<String>,
    /// Optional.
    /// 
    /// Controls whether to broaden the search when it produces sparse results.
    /// Broadened queries append results to the end of the matching results
    /// list.
    /// 
    /// Defaults to false.
    #[serde(rename="enableBroadening")]
    pub enable_broadening: Option<bool>,
    /// Optional.
    /// 
    /// Query used to search against jobs, such as keyword, location filters, etc.
    #[serde(rename="jobQuery")]
    pub job_query: Option<JobQuery>,
    /// Optional.
    /// 
    /// A limit on the number of jobs returned in the search results.
    /// Increasing this value above the default value of 10 can increase search
    /// response time. The value can be between 1 and 100.
    #[serde(rename="pageSize")]
    pub page_size: Option<i32>,
    /// Optional.
    /// 
    /// Histogram requests for jobs matching JobQuery.
    #[serde(rename="histogramFacets")]
    pub histogram_facets: Option<HistogramFacets>,
    /// Optional.
    /// 
    /// Mode of a search.
    /// 
    /// Defaults to SearchMode.JOB_SEARCH.
    #[serde(rename="searchMode")]
    pub search_mode: Option<String>,
    /// Optional.
    /// 
    /// The token specifying the current offset within
    /// search results. See SearchJobsResponse.next_page_token for
    /// an explanation of how to obtain the next set of query results.
    #[serde(rename="pageToken")]
    pub page_token: Option<String>,
    /// Required.
    /// 
    /// The meta information collected about the job searcher, used to improve the
    /// search quality of the service. The identifiers (such as `user_id`) are
    /// provided by users, and must be unique and consistent.
    #[serde(rename="requestMetadata")]
    pub request_metadata: Option<RequestMetadata>,
    /// Optional.
    /// 
    /// Controls if the search job request requires the return of a precise
    /// count of the first 300 results. Setting this to `true` ensures
    /// consistency in the number of results per page. Best practice is to set this
    /// value to true if a client allows users to jump directly to a
    /// non-sequential search results page.
    /// 
    /// Enabling this flag may adversely impact performance.
    /// 
    /// Defaults to false.
    #[serde(rename="requirePreciseResultSize")]
    pub require_precise_result_size: Option<bool>,
    /// Optional.
    /// 
    /// The desired job attributes returned for jobs in the
    /// search response. Defaults to JobView.SMALL if no value is specified.
    #[serde(rename="jobView")]
    pub job_view: Option<String>,
    /// Optional.
    /// 
    /// An integer that specifies the current offset (that is, starting result
    /// location, amongst the jobs deemed by the API as relevant) in search
    /// results. This field is only considered if page_token is unset.
    /// 
    /// For example, 0 means to  return results starting from the first matching
    /// job, and 10 means to return from the 11th job. This can be used for
    /// pagination, (for example, pageSize = 10 and offset = 10 means to return
    /// from the second page).
    pub offset: Option<i32>,
    /// Optional.
    /// 
    /// Controls whether to disable exact keyword match on Job.job_title,
    /// Job.description, Job.company_display_name, Job.locations,
    /// Job.qualifications. When disable keyword match is turned off, a
    /// keyword match returns jobs that do not match given category filters when
    /// there are matching keywords. For example, the query "program manager," a
    /// result is returned even if the job posting has the title "software
    /// developer," which does not fall into "program manager" ontology, but does
    /// have "program manager" appearing in its description.
    /// 
    /// For queries like "cloud" that does not contain title or
    /// location specific ontology, jobs with "cloud" keyword matches are returned
    /// regardless of this flag's value.
    /// 
    /// Please use Company.keyword_searchable_custom_fields or
    /// Company.keyword_searchable_custom_attributes if company specific
    /// globally matched custom field/attribute string values is needed. Enabling
    /// keyword match improves recall of subsequent search requests.
    /// 
    /// Defaults to false.
    #[serde(rename="disableKeywordMatch")]
    pub disable_keyword_match: Option<bool>,
}

impl RequestValue for SearchJobsRequest {}


/// Output only.
/// 
/// Custom numeric bucketing result.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NumericBucketingResult {
    /// Count within each bucket. Its size is the length of
    /// NumericBucketingOption.bucket_bounds plus 1.
    pub counts: Option<Vec<BucketizedCount>>,
    /// Stores the maximum value of the numeric field. Is populated only if
    /// [NumericBucketingOption.requires_min_max] is set to true.
    #[serde(rename="maxValue")]
    pub max_value: Option<f64>,
    /// Stores the minimum value of the numeric field. Will be populated only if
    /// [NumericBucketingOption.requires_min_max] is set to true.
    #[serde(rename="minValue")]
    pub min_value: Option<f64>,
}

impl Part for NumericBucketingResult {}


/// Job compensation details.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CompensationInfo {
    /// Output only. Annualized total compensation range. Computed as
    /// all compensation entries' CompensationEntry.compensation times
    /// CompensationEntry.expected_units_per_year.
    /// 
    /// See CompensationEntry for explanation on compensation annualization.
    #[serde(rename="annualizedTotalCompensationRange")]
    pub annualized_total_compensation_range: Option<CompensationRange>,
    /// Output only. Annualized base compensation range. Computed as
    /// base compensation entry's CompensationEntry.compensation times
    /// CompensationEntry.expected_units_per_year.
    /// 
    /// See CompensationEntry for explanation on compensation annualization.
    #[serde(rename="annualizedBaseCompensationRange")]
    pub annualized_base_compensation_range: Option<CompensationRange>,
    /// Optional.
    /// 
    /// Job compensation information.
    /// 
    /// At most one entry can be of type
    /// CompensationInfo.CompensationType.BASE, which is
    /// referred as ** base compensation entry ** for the job.
    pub entries: Option<Vec<CompensationEntry>>,
}

impl Part for CompensationInfo {}


/// Input only.
/// 
/// Compensation based histogram request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CompensationHistogramRequest {
    /// Required.
    /// 
    /// Numeric histogram options, like buckets, whether include min or max value.
    #[serde(rename="bucketingOption")]
    pub bucketing_option: Option<NumericBucketingOption>,
    /// Required.
    /// 
    /// Type of the request, representing which field the histogramming should be
    /// performed over. A single request can only specify one histogram of each
    /// `CompensationHistogramRequestType`.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl Part for CompensationHistogramRequest {}


/// Output only.
/// 
/// Custom attribute histogram result.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomAttributeHistogramResult {
    /// Stores bucketed histogram counting result or min/max values for
    /// custom attribute long values associated with `key`.
    #[serde(rename="longValueHistogramResult")]
    pub long_value_histogram_result: Option<NumericBucketingResult>,
    /// Stores a map from the values of string custom field associated
    /// with `key` to the number of jobs with that value in this histogram result.
    #[serde(rename="stringValueHistogramResult")]
    pub string_value_histogram_result: Option<HashMap<String, i32>>,
    /// Stores the key of custom attribute the histogram is performed on.
    pub key: Option<String>,
}

impl Part for CustomAttributeHistogramResult {}


/// Represents a postal address, e.g. for postal delivery or payments addresses.
/// Given a postal address, a postal service can deliver items to a premise, P.O.
/// Box or similar.
/// It is not intended to model geographical locations (roads, towns,
/// mountains).
/// 
/// In typical usage an address would be created via user input or from importing
/// existing data, depending on the type of process.
/// 
/// Advice on address input / editing:
///  - Use an i18n-ready address widget such as
///    https://github.com/googlei18n/libaddressinput)
/// - Users should not be presented with UI elements for input or editing of
///   fields outside countries where that field is used.
/// 
/// For more guidance on how to use this schema, please see:
/// https://support.google.com/business/answer/6397478
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PostalAddress {
    /// Optional. BCP-47 language code of the contents of this address (if
    /// known). This is often the UI language of the input form or is expected
    /// to match one of the languages used in the address' country/region, or their
    /// transliterated equivalents.
    /// This can affect formatting in certain countries, but is not critical
    /// to the correctness of the data and will never affect any validation or
    /// other non-formatting related operations.
    /// 
    /// If this value is not known, it should be omitted (rather than specifying a
    /// possibly incorrect default).
    /// 
    /// Examples: "zh-Hant", "ja", "ja-Latn", "en".
    #[serde(rename="languageCode")]
    pub language_code: Option<String>,
    /// Optional. The recipient at the address.
    /// This field may, under certain circumstances, contain multiline information.
    /// For example, it might contain "care of" information.
    pub recipients: Option<Vec<String>>,
    /// Optional. Generally refers to the city/town portion of the address.
    /// Examples: US city, IT comune, UK post town.
    /// In regions of the world where localities are not well defined or do not fit
    /// into this structure well, leave locality empty and use address_lines.
    pub locality: Option<String>,
    /// Optional. Additional, country-specific, sorting code. This is not used
    /// in most regions. Where it is used, the value is either a string like
    /// "CEDEX", optionally followed by a number (e.g. "CEDEX 7"), or just a number
    /// alone, representing the "sector code" (Jamaica), "delivery area indicator"
    /// (Malawi) or "post office indicator" (e.g. Côte d'Ivoire).
    #[serde(rename="sortingCode")]
    pub sorting_code: Option<String>,
    /// Required. CLDR region code of the country/region of the address. This
    /// is never inferred and it is up to the user to ensure the value is
    /// correct. See http://cldr.unicode.org/ and
    /// http://www.unicode.org/cldr/charts/30/supplemental/territory_information.html
    /// for details. Example: "CH" for Switzerland.
    #[serde(rename="regionCode")]
    pub region_code: Option<String>,
    /// Optional. Highest administrative subdivision which is used for postal
    /// addresses of a country or region.
    /// For example, this can be a state, a province, an oblast, or a prefecture.
    /// Specifically, for Spain this is the province and not the autonomous
    /// community (e.g. "Barcelona" and not "Catalonia").
    /// Many countries don't use an administrative area in postal addresses. E.g.
    /// in Switzerland this should be left unpopulated.
    #[serde(rename="administrativeArea")]
    pub administrative_area: Option<String>,
    /// Unstructured address lines describing the lower levels of an address.
    /// 
    /// Because values in address_lines do not have type information and may
    /// sometimes contain multiple values in a single field (e.g.
    /// "Austin, TX"), it is important that the line order is clear. The order of
    /// address lines should be "envelope order" for the country/region of the
    /// address. In places where this can vary (e.g. Japan), address_language is
    /// used to make it explicit (e.g. "ja" for large-to-small ordering and
    /// "ja-Latn" or "en" for small-to-large). This way, the most specific line of
    /// an address can be selected based on the language.
    /// 
    /// The minimum permitted structural representation of an address consists
    /// of a region_code with all remaining information placed in the
    /// address_lines. It would be possible to format such an address very
    /// approximately without geocoding, but no semantic reasoning could be
    /// made about any of the address components until it was at least
    /// partially resolved.
    /// 
    /// Creating an address only containing a region_code and address_lines, and
    /// then geocoding is the recommended way to handle completely unstructured
    /// addresses (as opposed to guessing which parts of the address should be
    /// localities or administrative areas).
    #[serde(rename="addressLines")]
    pub address_lines: Option<Vec<String>>,
    /// Optional. Postal code of the address. Not all countries use or require
    /// postal codes to be present, but where they are used, they may trigger
    /// additional validation with other parts of the address (e.g. state/zip
    /// validation in the U.S.A.).
    #[serde(rename="postalCode")]
    pub postal_code: Option<String>,
    /// Optional. The name of the organization at the address.
    pub organization: Option<String>,
    /// Optional. Sublocality of the address.
    /// For example, this can be neighborhoods, boroughs, districts.
    pub sublocality: Option<String>,
    /// The schema revision of the `PostalAddress`. This must be set to 0, which is
    /// the latest revision.
    /// 
    /// All new revisions **must** be backward compatible with old revisions.
    pub revision: Option<i32>,
}

impl Part for PostalAddress {}


/// Represents starting and ending value of a range in double.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketRange {
    /// Ending value of the bucket range.
    pub to: Option<f64>,
    /// Starting value of the bucket range.
    pub from: Option<f64>,
}

impl Part for BucketRange {}


/// Input only.
/// 
/// The query required to perform a search query.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobQuery {
    /// Optional.
    /// 
    /// This flag controls the spell-check feature. If false, the
    /// service attempts to correct a misspelled query,
    /// for example, "enginee" is corrected to "engineer".
    /// 
    /// Defaults to false: a spell check is performed.
    #[serde(rename="disableSpellCheck")]
    pub disable_spell_check: Option<bool>,
    /// Optional.
    /// 
    /// This filter specifies a structured syntax to match against the
    /// Job.custom_attributes marked as `filterable`.
    /// 
    /// The syntax for this expression is a subset of SQL syntax.
    /// 
    /// Supported operators are: `=`, `!=`, `<`, `<=`, `>`, and `>=` where the
    /// left of the operator is a custom field key and the right of the operator
    /// is a number or a quoted string. You must escape backslash (\\) and
    /// quote (\") characters.
    /// 
    /// Supported functions are `LOWER([field_name])` to
    /// perform a case insensitive match and `EMPTY([field_name])` to filter on the
    /// existence of a key.
    /// 
    /// Boolean expressions (AND/OR/NOT) are supported up to 3 levels of
    /// nesting (for example, "((A AND B AND C) OR NOT D) AND E"), a maximum of 100
    /// comparisons or functions are allowed in the expression. The expression
    /// must be < 3000 bytes in length.
    /// 
    /// Sample Query:
    /// `(LOWER(driving_license)="class \"a\"" OR EMPTY(driving_license)) AND
    /// driving_years > 10`
    #[serde(rename="customAttributeFilter")]
    pub custom_attribute_filter: Option<String>,
    /// Optional.
    /// 
    /// The employment type filter specifies the employment type of jobs to
    /// search against, such as EmploymentType.FULL_TIME.
    /// 
    /// If a value is not specified, jobs in the search results includes any
    /// employment type.
    /// 
    /// If multiple values are specified, jobs in the search results include
    /// any of the specified employment types.
    #[serde(rename="employmentTypes")]
    pub employment_types: Option<Vec<String>>,
    /// Optional.
    /// 
    /// This filter specifies the locale of jobs to search against,
    /// for example, "en-US".
    /// 
    /// If a value isn't specified, the search results can contain jobs in any
    /// locale.
    /// 
    /// 
    /// Language codes should be in BCP-47 format, such as "en-US" or "sr-Latn".
    /// For more information, see
    /// [Tags for Identifying Languages](https://tools.ietf.org/html/bcp47).
    /// 
    /// At most 10 language code filters are allowed.
    #[serde(rename="languageCodes")]
    pub language_codes: Option<Vec<String>>,
    /// Optional.
    /// 
    /// Jobs published within a range specified by this filter are searched
    /// against.
    #[serde(rename="publishTimeRange")]
    pub publish_time_range: Option<TimestampRange>,
    /// Optional.
    /// 
    ///  Allows filtering jobs by commute time with different travel methods (for
    ///  example, driving or public transit). Note: This only works with COMMUTE
    ///  MODE. When specified, [JobQuery.location_filters] is
    ///  ignored.
    /// 
    ///  Currently we don't support sorting by commute time.
    #[serde(rename="commuteFilter")]
    pub commute_filter: Option<CommuteFilter>,
    /// Optional.
    /// 
    /// The category filter specifies the categories of jobs to search against.
    /// See Category for more information.
    /// 
    /// If a value is not specified, jobs from any category are searched against.
    /// 
    /// If multiple values are specified, jobs from any of the specified
    /// categories are searched against.
    #[serde(rename="jobCategories")]
    pub job_categories: Option<Vec<String>>,
    /// Optional.
    /// 
    /// The query string that matches against the job title, description, and
    /// location fields.
    /// 
    /// The maximum number of allowed characters is 255.
    pub query: Option<String>,
    /// Optional.
    /// 
    /// This filter specifies the company entities to search against.
    /// 
    /// If a value isn't specified, jobs are searched for against all
    /// companies.
    /// 
    /// If multiple values are specified, jobs are searched against the
    /// companies specified.
    /// 
    /// The format is "projects/{project_id}/companies/{company_id}", for example,
    /// "projects/api-test-project/companies/foo".
    /// 
    /// At most 20 company filters are allowed.
    #[serde(rename="companyNames")]
    pub company_names: Option<Vec<String>>,
    /// Optional.
    /// 
    /// This filter specifies the exact company display
    /// name of the jobs to search against.
    /// 
    /// If a value isn't specified, jobs within the search results are
    /// associated with any company.
    /// 
    /// If multiple values are specified, jobs within the search results may be
    /// associated with any of the specified companies.
    /// 
    /// At most 20 company display name filters are allowed.
    #[serde(rename="companyDisplayNames")]
    pub company_display_names: Option<Vec<String>>,
    /// Optional.
    /// 
    /// The location filter specifies geo-regions containing the jobs to
    /// search against. See LocationFilter for more information.
    /// 
    /// If a location value isn't specified, jobs fitting the other search
    /// criteria are retrieved regardless of where they're located.
    /// 
    /// If multiple values are specified, jobs are retrieved from any of the
    /// specified locations. If different values are specified for the
    /// LocationFilter.distance_in_miles parameter, the maximum provided
    /// distance is used for all locations.
    /// 
    /// At most 5 location filters are allowed.
    #[serde(rename="locationFilters")]
    pub location_filters: Option<Vec<LocationFilter>>,
    /// Optional.
    /// 
    /// This search filter is applied only to
    /// Job.compensation_info. For example, if the filter is specified
    /// as "Hourly job with per-hour compensation > $15", only jobs meeting
    /// these criteria are searched. If a filter isn't defined, all open jobs
    /// are searched.
    #[serde(rename="compensationFilter")]
    pub compensation_filter: Option<CompensationFilter>,
}

impl Part for JobQuery {}


/// Application related details of a job posting.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApplicationInfo {
    /// Optional but at least one of uris,
    /// emails or instruction must be
    /// specified.
    /// 
    /// Use this field to provide instructions, such as "Mail your application
    /// to ...", that a candidate can follow to apply for the job.
    /// 
    /// This field accepts and sanitizes HTML input, and also accepts
    /// bold, italic, ordered list, and unordered list markup tags.
    /// 
    /// The maximum number of allowed characters is 3,000.
    pub instruction: Option<String>,
    /// Optional but at least one of uris,
    /// emails or instruction must be
    /// specified.
    /// 
    /// Use this field to specify email address(es) to which resumes or
    /// applications can be sent.
    /// 
    /// The maximum number of allowed characters for each entry is 255.
    pub emails: Option<Vec<String>>,
    /// Optional but at least one of uris,
    /// emails or instruction must be
    /// specified.
    /// 
    /// Use this URI field to direct an applicant to a website, for example to
    /// link to an online application form.
    /// 
    /// The maximum number of allowed characters for each entry is 2,000.
    pub uris: Option<Vec<String>>,
}

impl Part for ApplicationInfo {}


/// Represents count of jobs within one bucket.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketizedCount {
    /// Number of jobs whose numeric field value fall into `range`.
    pub count: Option<i32>,
    /// Bucket range on which histogram was performed for the numeric field,
    /// that is, the count represents number of jobs in this range.
    pub range: Option<BucketRange>,
}

impl Part for BucketizedCount {}


/// Output only.
/// 
/// Compensation based histogram result.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CompensationHistogramResult {
    /// Type of the request, corresponding to
    /// CompensationHistogramRequest.type.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// Histogram result.
    pub result: Option<NumericBucketingResult>,
}

impl Part for CompensationHistogramResult {}


/// Represents an amount of money with its currency type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Money {
    /// Number of nano (10^-9) units of the amount.
    /// The value must be between -999,999,999 and +999,999,999 inclusive.
    /// If `units` is positive, `nanos` must be positive or zero.
    /// If `units` is zero, `nanos` can be positive, zero, or negative.
    /// If `units` is negative, `nanos` must be negative or zero.
    /// For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
    pub nanos: Option<i32>,
    /// The whole units of the amount.
    /// For example if `currencyCode` is `"USD"`, then 1 unit is one US dollar.
    pub units: Option<String>,
    /// The 3-letter currency code defined in ISO 4217.
    #[serde(rename="currencyCode")]
    pub currency_code: Option<String>,
}

impl Part for Money {}


/// An object representing a latitude/longitude pair. This is expressed as a pair
/// of doubles representing degrees latitude and degrees longitude. Unless
/// specified otherwise, this must conform to the
/// <a href="http://www.unoosa.org/pdf/icg/2012/template/WGS_84.pdf">WGS84
/// standard</a>. Values must be within normalized ranges.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LatLng {
    /// The latitude in degrees. It must be in the range [-90.0, +90.0].
    pub latitude: Option<f64>,
    /// The longitude in degrees. It must be in the range [-180.0, +180.0].
    pub longitude: Option<f64>,
}

impl Part for LatLng {}


/// Device information collected from the job seeker, candidate, or
/// other entity conducting the job search. Providing this information improves
/// the quality of the search results across devices.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// Optional.
    /// 
    /// Type of the device.
    #[serde(rename="deviceType")]
    pub device_type: Option<String>,
    /// Optional.
    /// 
    /// A device-specific ID. The ID must be a unique identifier that
    /// distinguishes the device from other devices.
    pub id: Option<String>,
}

impl Part for DeviceInfo {}


/// Input only.
/// 
/// Update job request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [jobs patch projects](struct.ProjectJobPatchCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateJobRequest {
    /// Required.
    /// 
    /// The Job to be updated.
    pub job: Option<Job>,
    /// Optional but strongly recommended to be provided for the best service
    /// experience.
    /// 
    /// If update_mask is provided, only the specified fields in
    /// job are updated. Otherwise all the fields are updated.
    /// 
    /// A field mask to restrict the fields that are updated. Only
    /// top level fields of Job are supported.
    #[serde(rename="updateMask")]
    pub update_mask: Option<String>,
}

impl RequestValue for UpdateJobRequest {}


/// Output only.
/// 
/// Derived details about the job posting.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobDerivedInfo {
    /// Job categories derived from Job.title and Job.description.
    #[serde(rename="jobCategories")]
    pub job_categories: Option<Vec<String>>,
    /// Structured locations of the job, resolved from Job.addresses.
    /// 
    /// locations are exactly matched to Job.addresses in the same
    /// order.
    pub locations: Option<Vec<Location>>,
}

impl Part for JobDerivedInfo {}


/// An event issued when a job seeker interacts with the application that
/// implements Cloud Talent Solution.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobEvent {
    /// Required.
    /// 
    /// The job name(s) associated with this event.
    /// For example, if this is an impression event,
    /// this field contains the identifiers of all jobs shown to the job seeker.
    /// If this was a view event, this field contains the
    /// identifier of the viewed job.
    pub jobs: Option<Vec<String>>,
    /// Required.
    /// 
    /// The type of the event (see JobEventType).
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl Part for JobEvent {}


/// Output only.
/// 
/// Result of a histogram call. The response contains the histogram map for the
/// search type specified by HistogramResult.field.
/// The response is a map of each filter value to the corresponding count of
/// jobs for that filter.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HistogramResult {
    /// A map from the values of field to the number of jobs with that value
    /// in this search result.
    /// 
    /// Key: search type (filter names, such as the companyName).
    /// 
    /// Values: the count of jobs that match the filter for this search.
    pub values: Option<HashMap<String, i32>>,
    /// The Histogram search filters.
    #[serde(rename="searchType")]
    pub search_type: Option<String>,
}

impl Part for HistogramResult {}


/// Input only.
/// 
/// Parameters needed for commute search.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommuteFilter {
    /// Required.
    /// 
    /// The latitude and longitude of the location from which to calculate the
    /// commute time.
    #[serde(rename="startCoordinates")]
    pub start_coordinates: Option<LatLng>,
    /// Optional.
    /// 
    /// The departure time used to calculate traffic impact, represented as
    /// google.type.TimeOfDay in local time zone.
    /// 
    /// Currently traffic model is restricted to hour level resolution.
    #[serde(rename="departureTime")]
    pub departure_time: Option<TimeOfDay>,
    /// Required.
    /// 
    /// The method of transportation for which to calculate the commute time.
    #[serde(rename="commuteMethod")]
    pub commute_method: Option<String>,
    /// Optional.
    /// If true, jobs without "precise" addresses (street level addresses or GPS
    /// coordinates) might also be returned. For city and coarser level addresses,
    /// text matching is used. If this field is set to false or is not specified,
    /// only jobs that include precise addresses are returned by Commute
    /// Search.
    /// 
    /// Note: If `allow_imprecise_addresses` is set to true, Commute Search is not
    /// able to calculate accurate commute times to jobs with city level and
    /// coarser address information. Jobs with imprecise addresses will return a
    /// `travel_duration` time of 0 regardless of distance from the job seeker.
    #[serde(rename="allowImpreciseAddresses")]
    pub allow_imprecise_addresses: Option<bool>,
    /// Required.
    /// 
    /// The maximum travel time in seconds. The maximum allowed value is `3600s`
    /// (one hour). Format is `123s`.
    #[serde(rename="travelDuration")]
    pub travel_duration: Option<String>,
    /// Optional.
    /// 
    /// Specifies the traffic density to use when calculating commute time.
    #[serde(rename="roadTraffic")]
    pub road_traffic: Option<String>,
}

impl Part for CommuteFilter {}


/// The report event request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [client events create projects](struct.ProjectClientEventCreateCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateClientEventRequest {
    /// Required.
    /// 
    /// Events issued when end user interacts with customer's application that
    /// uses Cloud Talent Solution.
    #[serde(rename="clientEvent")]
    pub client_event: Option<ClientEvent>,
}

impl RequestValue for CreateClientEventRequest {}


/// Input only.
/// 
/// Use this field to specify bucketing option for the histogram search response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NumericBucketingOption {
    /// Required.
    /// 
    /// Two adjacent values form a histogram bucket. Values should be in
    /// ascending order. For example, if [5, 10, 15] are provided, four buckets are
    /// created: (-inf, 5), 5, 10), [10, 15), [15, inf). At most 20
    /// [buckets_bound is supported.
    #[serde(rename="bucketBounds")]
    pub bucket_bounds: Option<Vec<f64>>,
    /// Optional.
    /// 
    /// If set to true, the histogram result includes minimum/maximum
    /// value of the numeric field.
    #[serde(rename="requiresMinMax")]
    pub requires_min_max: Option<bool>,
}

impl Part for NumericBucketingOption {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the `CloudTalentSolution` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_jobs3 as jobs3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use jobs3::CloudTalentSolution;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = CloudTalentSolution::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `client_events_create(...)`, `companies_create(...)`, `companies_delete(...)`, `companies_get(...)`, `companies_list(...)`, `companies_patch(...)`, `complete(...)`, `jobs_batch_delete(...)`, `jobs_create(...)`, `jobs_delete(...)`, `jobs_get(...)`, `jobs_list(...)`, `jobs_patch(...)`, `jobs_search(...)` and `jobs_search_for_alert(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudTalentSolution<C, A>,
}

impl<'a, C, A> MethodsBuilder for ProjectMethods<'a, C, A> {}

impl<'a, C, A> ProjectMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Report events issued when end user interacts with customer's application
    /// that uses Cloud Talent Solution. You may inspect the created events in
    /// [self service
    /// tools](https://console.cloud.google.com/talent-solution/overview).
    /// [Learn
    /// more](https://cloud.google.com/talent-solution/docs/management-tools)
    /// about self service tools.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Parent project name.
    pub fn client_events_create(&self, request: CreateClientEventRequest, parent: &str) -> ProjectClientEventCreateCall<'a, C, A> {
        ProjectClientEventCreateCall {
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
    /// Creates a new company entity.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required.
    ///              Resource name of the project under which the company is created.
    ///              The format is "projects/{project_id}", for example,
    ///              "projects/api-test-project".
    pub fn companies_create(&self, request: CreateCompanyRequest, parent: &str) -> ProjectCompanyCreateCall<'a, C, A> {
        ProjectCompanyCreateCall {
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
    /// Lists jobs by filter.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required.
    ///              The resource name of the project under which the job is created.
    ///              The format is "projects/{project_id}", for example,
    ///              "projects/api-test-project".
    pub fn jobs_list(&self, parent: &str) -> ProjectJobListCall<'a, C, A> {
        ProjectJobListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _job_view: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified job.
    /// 
    /// Typically, the job becomes unsearchable within 10 seconds, but it may take
    /// up to 5 minutes.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required.
    ///            The resource name of the job to be deleted.
    ///            The format is "projects/{project_id}/jobs/{job_id}",
    ///            for example, "projects/api-test-project/jobs/1234".
    pub fn jobs_delete(&self, name: &str) -> ProjectJobDeleteCall<'a, C, A> {
        ProjectJobDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves specified company.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required.
    ///            The resource name of the company to be retrieved.
    ///            The format is "projects/{project_id}/companies/{company_id}", for example,
    ///            "projects/api-test-project/companies/foo".
    pub fn companies_get(&self, name: &str) -> ProjectCompanyGetCall<'a, C, A> {
        ProjectCompanyGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Searches for jobs using the provided SearchJobsRequest.
    /// 
    /// This call constrains the visibility of jobs
    /// present in the database, and only returns jobs that the caller has
    /// permission to search against.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required.
    ///              The resource name of the project to search within.
    ///              The format is "projects/{project_id}", for example,
    ///              "projects/api-test-project".
    pub fn jobs_search(&self, request: SearchJobsRequest, parent: &str) -> ProjectJobSearchCall<'a, C, A> {
        ProjectJobSearchCall {
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
    /// Lists all companies associated with the service account.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required.
    ///              Resource name of the project under which the company is created.
    ///              The format is "projects/{project_id}", for example,
    ///              "projects/api-test-project".
    pub fn companies_list(&self, parent: &str) -> ProjectCompanyListCall<'a, C, A> {
        ProjectCompanyListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _require_open_jobs: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified job, whose status is OPEN or recently EXPIRED
    /// within the last 90 days.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required.
    ///            The resource name of the job to retrieve.
    ///            The format is "projects/{project_id}/jobs/{job_id}",
    ///            for example, "projects/api-test-project/jobs/1234".
    pub fn jobs_get(&self, name: &str) -> ProjectJobGetCall<'a, C, A> {
        ProjectJobGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Searches for jobs using the provided SearchJobsRequest.
    /// 
    /// This API call is intended for the use case of targeting passive job
    /// seekers (for example, job seekers who have signed up to receive email
    /// alerts about potential job opportunities), and has different algorithmic
    /// adjustments that are targeted to passive job seekers.
    /// 
    /// This call constrains the visibility of jobs
    /// present in the database, and only returns jobs the caller has
    /// permission to search against.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required.
    ///              The resource name of the project to search within.
    ///              The format is "projects/{project_id}", for example,
    ///              "projects/api-test-project".
    pub fn jobs_search_for_alert(&self, request: SearchJobsRequest, parent: &str) -> ProjectJobSearchForAlertCall<'a, C, A> {
        ProjectJobSearchForAlertCall {
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
    /// Creates a new job.
    /// 
    /// Typically, the job becomes searchable within 10 seconds, but it may take
    /// up to 5 minutes.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required.
    ///              The resource name of the project under which the job is created.
    ///              The format is "projects/{project_id}", for example,
    ///              "projects/api-test-project".
    pub fn jobs_create(&self, request: CreateJobRequest, parent: &str) -> ProjectJobCreateCall<'a, C, A> {
        ProjectJobCreateCall {
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
    /// Updates specified company. Company names can't be updated. To update a
    /// company name, delete the company and all jobs associated with it, and only
    /// then re-create them.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required during company update.
    ///            The resource name for a company. This is generated by the service when a
    ///            company is created.
    ///            The format is "projects/{project_id}/companies/{company_id}", for example,
    ///            "projects/api-test-project/companies/foo".
    pub fn companies_patch(&self, request: UpdateCompanyRequest, name: &str) -> ProjectCompanyPatchCall<'a, C, A> {
        ProjectCompanyPatchCall {
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
    /// Deletes a list of Jobs by filter.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required.
    ///              The resource name of the project under which the job is created.
    ///              The format is "projects/{project_id}", for example,
    ///              "projects/api-test-project".
    pub fn jobs_batch_delete(&self, request: BatchDeleteJobsRequest, parent: &str) -> ProjectJobBatchDeleteCall<'a, C, A> {
        ProjectJobBatchDeleteCall {
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
    /// Deletes specified company.
    /// Prerequisite: The company has no jobs associated with it.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required.
    ///            The resource name of the company to be deleted.
    ///            The format is "projects/{project_id}/companies/{company_id}", for example,
    ///            "projects/api-test-project/companies/foo".
    pub fn companies_delete(&self, name: &str) -> ProjectCompanyDeleteCall<'a, C, A> {
        ProjectCompanyDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates specified job.
    /// 
    /// Typically, updated contents become visible in search results within 10
    /// seconds, but it may take up to 5 minutes.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required during job update.
    ///            The resource name for the job. This is generated by the service when a
    ///            job is created.
    ///            The format is "projects/{project_id}/jobs/{job_id}",
    ///            for example, "projects/api-test-project/jobs/1234".
    ///            Use of this field in job queries and API calls is preferred over the use of
    ///            requisition_id since this value is unique.
    pub fn jobs_patch(&self, request: UpdateJobRequest, name: &str) -> ProjectJobPatchCall<'a, C, A> {
        ProjectJobPatchCall {
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
    /// Completes the specified prefix with keyword suggestions.
    /// Intended for use by a job search auto-complete search box.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required.
    ///            Resource name of project the completion is performed within.
    ///            The format is "projects/{project_id}", for example,
    ///            "projects/api-test-project".
    pub fn complete(&self, name: &str) -> ProjectCompleteCall<'a, C, A> {
        ProjectCompleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _type_: Default::default(),
            _scope: Default::default(),
            _query: Default::default(),
            _page_size: Default::default(),
            _language_codes: Default::default(),
            _language_code: Default::default(),
            _company_name: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Report events issued when end user interacts with customer's application
/// that uses Cloud Talent Solution. You may inspect the created events in
/// [self service
/// tools](https://console.cloud.google.com/talent-solution/overview).
/// [Learn
/// more](https://cloud.google.com/talent-solution/docs/management-tools)
/// about self service tools.
///
/// A builder for the *clientEvents.create* method supported by a *project* resource.
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
/// # extern crate google_jobs3 as jobs3;
/// use jobs3::CreateClientEventRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use jobs3::CloudTalentSolution;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudTalentSolution::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = CreateClientEventRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().client_events_create(req, "parent")
///              .doit();
/// # }
/// ```
pub struct ProjectClientEventCreateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudTalentSolution<C, A>,
    _request: CreateClientEventRequest,
    _parent: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectClientEventCreateCall<'a, C, A> {}

impl<'a, C, A> ProjectClientEventCreateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ClientEvent)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "jobs.projects.clientEvents.create",
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

        let mut url = self.hub._base_url.clone() + "v3/{+parent}/clientEvents";
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
    pub fn request(mut self, new_value: CreateClientEventRequest) -> ProjectClientEventCreateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Parent project name.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> ProjectClientEventCreateCall<'a, C, A> {
        self._parent = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ProjectClientEventCreateCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectClientEventCreateCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectClientEventCreateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a new company entity.
///
/// A builder for the *companies.create* method supported by a *project* resource.
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
/// # extern crate google_jobs3 as jobs3;
/// use jobs3::CreateCompanyRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use jobs3::CloudTalentSolution;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudTalentSolution::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = CreateCompanyRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().companies_create(req, "parent")
///              .doit();
/// # }
/// ```
pub struct ProjectCompanyCreateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudTalentSolution<C, A>,
    _request: CreateCompanyRequest,
    _parent: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectCompanyCreateCall<'a, C, A> {}

impl<'a, C, A> ProjectCompanyCreateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Company)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "jobs.projects.companies.create",
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

        let mut url = self.hub._base_url.clone() + "v3/{+parent}/companies";
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
    pub fn request(mut self, new_value: CreateCompanyRequest) -> ProjectCompanyCreateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Required.
    /// 
    /// Resource name of the project under which the company is created.
    /// 
    /// The format is "projects/{project_id}", for example,
    /// "projects/api-test-project".
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> ProjectCompanyCreateCall<'a, C, A> {
        self._parent = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ProjectCompanyCreateCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectCompanyCreateCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectCompanyCreateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists jobs by filter.
///
/// A builder for the *jobs.list* method supported by a *project* resource.
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
/// # extern crate google_jobs3 as jobs3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use jobs3::CloudTalentSolution;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudTalentSolution::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().jobs_list("parent")
///              .page_token("kasd")
///              .page_size(-22)
///              .job_view("takimata")
///              .filter("justo")
///              .doit();
/// # }
/// ```
pub struct ProjectJobListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudTalentSolution<C, A>,
    _parent: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _job_view: Option<String>,
    _filter: Option<String>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectJobListCall<'a, C, A> {}

impl<'a, C, A> ProjectJobListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListJobsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "jobs.projects.jobs.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("parent", self._parent.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._job_view {
            params.push(("jobView", value.to_string()));
        }
        if let Some(value) = self._filter {
            params.push(("filter", value.to_string()));
        }
        for &field in ["alt", "parent", "pageToken", "pageSize", "jobView", "filter"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v3/{+parent}/jobs";
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


    /// Required.
    /// 
    /// The resource name of the project under which the job is created.
    /// 
    /// The format is "projects/{project_id}", for example,
    /// "projects/api-test-project".
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> ProjectJobListCall<'a, C, A> {
        self._parent = new_value.to_string();
        self
    }
    /// Optional.
    /// 
    /// The starting point of a query result.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ProjectJobListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Optional.
    /// 
    /// The maximum number of jobs to be returned per page of results.
    /// 
    /// If job_view is set to JobView.JOB_VIEW_ID_ONLY, the maximum allowed
    /// page size is 1000. Otherwise, the maximum allowed page size is 100.
    /// 
    /// Default is 100 if empty or a number < 1 is specified.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> ProjectJobListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// Optional.
    /// 
    /// The desired job attributes returned for jobs in the
    /// search response. Defaults to JobView.JOB_VIEW_FULL if no value is
    /// specified.
    ///
    /// Sets the *job view* query property to the given value.
    pub fn job_view(mut self, new_value: &str) -> ProjectJobListCall<'a, C, A> {
        self._job_view = Some(new_value.to_string());
        self
    }
    /// Required.
    /// 
    /// The filter string specifies the jobs to be enumerated.
    /// 
    /// Supported operator: =, AND
    /// 
    /// The fields eligible for filtering are:
    /// 
    /// * `companyName` (Required)
    /// * `requisitionId` (Optional)
    /// 
    /// Sample Query:
    /// 
    /// * companyName = "projects/api-test-project/companies/123"
    /// * companyName = "projects/api-test-project/companies/123" AND requisitionId
    /// = "req-1"
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> ProjectJobListCall<'a, C, A> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ProjectJobListCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectJobListCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectJobListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deletes the specified job.
/// 
/// Typically, the job becomes unsearchable within 10 seconds, but it may take
/// up to 5 minutes.
///
/// A builder for the *jobs.delete* method supported by a *project* resource.
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
/// # extern crate google_jobs3 as jobs3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use jobs3::CloudTalentSolution;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudTalentSolution::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().jobs_delete("name")
///              .doit();
/// # }
/// ```
pub struct ProjectJobDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudTalentSolution<C, A>,
    _name: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectJobDeleteCall<'a, C, A> {}

impl<'a, C, A> ProjectJobDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Empty)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "jobs.projects.jobs.delete",
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

        let mut url = self.hub._base_url.clone() + "v3/{+name}";
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


    /// Required.
    /// 
    /// The resource name of the job to be deleted.
    /// 
    /// The format is "projects/{project_id}/jobs/{job_id}",
    /// for example, "projects/api-test-project/jobs/1234".
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> ProjectJobDeleteCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ProjectJobDeleteCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectJobDeleteCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectJobDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves specified company.
///
/// A builder for the *companies.get* method supported by a *project* resource.
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
/// # extern crate google_jobs3 as jobs3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use jobs3::CloudTalentSolution;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudTalentSolution::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().companies_get("name")
///              .doit();
/// # }
/// ```
pub struct ProjectCompanyGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudTalentSolution<C, A>,
    _name: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectCompanyGetCall<'a, C, A> {}

impl<'a, C, A> ProjectCompanyGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Company)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "jobs.projects.companies.get",
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

        let mut url = self.hub._base_url.clone() + "v3/{+name}";
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


    /// Required.
    /// 
    /// The resource name of the company to be retrieved.
    /// 
    /// The format is "projects/{project_id}/companies/{company_id}", for example,
    /// "projects/api-test-project/companies/foo".
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> ProjectCompanyGetCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ProjectCompanyGetCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectCompanyGetCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectCompanyGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Searches for jobs using the provided SearchJobsRequest.
/// 
/// This call constrains the visibility of jobs
/// present in the database, and only returns jobs that the caller has
/// permission to search against.
///
/// A builder for the *jobs.search* method supported by a *project* resource.
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
/// # extern crate google_jobs3 as jobs3;
/// use jobs3::SearchJobsRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use jobs3::CloudTalentSolution;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudTalentSolution::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = SearchJobsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().jobs_search(req, "parent")
///              .doit();
/// # }
/// ```
pub struct ProjectJobSearchCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudTalentSolution<C, A>,
    _request: SearchJobsRequest,
    _parent: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectJobSearchCall<'a, C, A> {}

impl<'a, C, A> ProjectJobSearchCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, SearchJobsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "jobs.projects.jobs.search",
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

        let mut url = self.hub._base_url.clone() + "v3/{+parent}/jobs:search";
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
    pub fn request(mut self, new_value: SearchJobsRequest) -> ProjectJobSearchCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Required.
    /// 
    /// The resource name of the project to search within.
    /// 
    /// The format is "projects/{project_id}", for example,
    /// "projects/api-test-project".
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> ProjectJobSearchCall<'a, C, A> {
        self._parent = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ProjectJobSearchCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectJobSearchCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectJobSearchCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists all companies associated with the service account.
///
/// A builder for the *companies.list* method supported by a *project* resource.
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
/// # extern crate google_jobs3 as jobs3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use jobs3::CloudTalentSolution;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudTalentSolution::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().companies_list("parent")
///              .require_open_jobs(false)
///              .page_token("dolores")
///              .page_size(-61)
///              .doit();
/// # }
/// ```
pub struct ProjectCompanyListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudTalentSolution<C, A>,
    _parent: String,
    _require_open_jobs: Option<bool>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectCompanyListCall<'a, C, A> {}

impl<'a, C, A> ProjectCompanyListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListCompaniesResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "jobs.projects.companies.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("parent", self._parent.to_string()));
        if let Some(value) = self._require_open_jobs {
            params.push(("requireOpenJobs", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        for &field in ["alt", "parent", "requireOpenJobs", "pageToken", "pageSize"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v3/{+parent}/companies";
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


    /// Required.
    /// 
    /// Resource name of the project under which the company is created.
    /// 
    /// The format is "projects/{project_id}", for example,
    /// "projects/api-test-project".
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> ProjectCompanyListCall<'a, C, A> {
        self._parent = new_value.to_string();
        self
    }
    /// Optional.
    /// 
    /// Set to true if the companies requested must have open jobs.
    /// 
    /// Defaults to false.
    /// 
    /// If true, at most page_size of companies are fetched, among which
    /// only those with open jobs are returned.
    ///
    /// Sets the *require open jobs* query property to the given value.
    pub fn require_open_jobs(mut self, new_value: bool) -> ProjectCompanyListCall<'a, C, A> {
        self._require_open_jobs = Some(new_value);
        self
    }
    /// Optional.
    /// 
    /// The starting indicator from which to return results.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ProjectCompanyListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Optional.
    /// 
    /// The maximum number of companies to be returned, at most 100.
    /// Default is 100 if a non-positive number is provided.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> ProjectCompanyListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ProjectCompanyListCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectCompanyListCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectCompanyListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves the specified job, whose status is OPEN or recently EXPIRED
/// within the last 90 days.
///
/// A builder for the *jobs.get* method supported by a *project* resource.
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
/// # extern crate google_jobs3 as jobs3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use jobs3::CloudTalentSolution;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudTalentSolution::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().jobs_get("name")
///              .doit();
/// # }
/// ```
pub struct ProjectJobGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudTalentSolution<C, A>,
    _name: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectJobGetCall<'a, C, A> {}

impl<'a, C, A> ProjectJobGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Job)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "jobs.projects.jobs.get",
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

        let mut url = self.hub._base_url.clone() + "v3/{+name}";
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


    /// Required.
    /// 
    /// The resource name of the job to retrieve.
    /// 
    /// The format is "projects/{project_id}/jobs/{job_id}",
    /// for example, "projects/api-test-project/jobs/1234".
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> ProjectJobGetCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ProjectJobGetCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectJobGetCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectJobGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Searches for jobs using the provided SearchJobsRequest.
/// 
/// This API call is intended for the use case of targeting passive job
/// seekers (for example, job seekers who have signed up to receive email
/// alerts about potential job opportunities), and has different algorithmic
/// adjustments that are targeted to passive job seekers.
/// 
/// This call constrains the visibility of jobs
/// present in the database, and only returns jobs the caller has
/// permission to search against.
///
/// A builder for the *jobs.searchForAlert* method supported by a *project* resource.
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
/// # extern crate google_jobs3 as jobs3;
/// use jobs3::SearchJobsRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use jobs3::CloudTalentSolution;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudTalentSolution::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = SearchJobsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().jobs_search_for_alert(req, "parent")
///              .doit();
/// # }
/// ```
pub struct ProjectJobSearchForAlertCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudTalentSolution<C, A>,
    _request: SearchJobsRequest,
    _parent: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectJobSearchForAlertCall<'a, C, A> {}

impl<'a, C, A> ProjectJobSearchForAlertCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, SearchJobsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "jobs.projects.jobs.searchForAlert",
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

        let mut url = self.hub._base_url.clone() + "v3/{+parent}/jobs:searchForAlert";
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
    pub fn request(mut self, new_value: SearchJobsRequest) -> ProjectJobSearchForAlertCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Required.
    /// 
    /// The resource name of the project to search within.
    /// 
    /// The format is "projects/{project_id}", for example,
    /// "projects/api-test-project".
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> ProjectJobSearchForAlertCall<'a, C, A> {
        self._parent = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ProjectJobSearchForAlertCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectJobSearchForAlertCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectJobSearchForAlertCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a new job.
/// 
/// Typically, the job becomes searchable within 10 seconds, but it may take
/// up to 5 minutes.
///
/// A builder for the *jobs.create* method supported by a *project* resource.
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
/// # extern crate google_jobs3 as jobs3;
/// use jobs3::CreateJobRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use jobs3::CloudTalentSolution;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudTalentSolution::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = CreateJobRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().jobs_create(req, "parent")
///              .doit();
/// # }
/// ```
pub struct ProjectJobCreateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudTalentSolution<C, A>,
    _request: CreateJobRequest,
    _parent: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectJobCreateCall<'a, C, A> {}

impl<'a, C, A> ProjectJobCreateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Job)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "jobs.projects.jobs.create",
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

        let mut url = self.hub._base_url.clone() + "v3/{+parent}/jobs";
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
    pub fn request(mut self, new_value: CreateJobRequest) -> ProjectJobCreateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Required.
    /// 
    /// The resource name of the project under which the job is created.
    /// 
    /// The format is "projects/{project_id}", for example,
    /// "projects/api-test-project".
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> ProjectJobCreateCall<'a, C, A> {
        self._parent = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ProjectJobCreateCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectJobCreateCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectJobCreateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates specified company. Company names can't be updated. To update a
/// company name, delete the company and all jobs associated with it, and only
/// then re-create them.
///
/// A builder for the *companies.patch* method supported by a *project* resource.
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
/// # extern crate google_jobs3 as jobs3;
/// use jobs3::UpdateCompanyRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use jobs3::CloudTalentSolution;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudTalentSolution::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = UpdateCompanyRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().companies_patch(req, "name")
///              .doit();
/// # }
/// ```
pub struct ProjectCompanyPatchCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudTalentSolution<C, A>,
    _request: UpdateCompanyRequest,
    _name: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectCompanyPatchCall<'a, C, A> {}

impl<'a, C, A> ProjectCompanyPatchCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Company)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "jobs.projects.companies.patch",
                               http_method: hyper::method::Method::Patch });
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

        let mut url = self.hub._base_url.clone() + "v3/{+name}";
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Patch, url.clone())
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
    pub fn request(mut self, new_value: UpdateCompanyRequest) -> ProjectCompanyPatchCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Required during company update.
    /// 
    /// The resource name for a company. This is generated by the service when a
    /// company is created.
    /// 
    /// The format is "projects/{project_id}/companies/{company_id}", for example,
    /// "projects/api-test-project/companies/foo".
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> ProjectCompanyPatchCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ProjectCompanyPatchCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectCompanyPatchCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectCompanyPatchCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deletes a list of Jobs by filter.
///
/// A builder for the *jobs.batchDelete* method supported by a *project* resource.
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
/// # extern crate google_jobs3 as jobs3;
/// use jobs3::BatchDeleteJobsRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use jobs3::CloudTalentSolution;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudTalentSolution::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BatchDeleteJobsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().jobs_batch_delete(req, "parent")
///              .doit();
/// # }
/// ```
pub struct ProjectJobBatchDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudTalentSolution<C, A>,
    _request: BatchDeleteJobsRequest,
    _parent: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectJobBatchDeleteCall<'a, C, A> {}

impl<'a, C, A> ProjectJobBatchDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Empty)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "jobs.projects.jobs.batchDelete",
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

        let mut url = self.hub._base_url.clone() + "v3/{+parent}/jobs:batchDelete";
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
    pub fn request(mut self, new_value: BatchDeleteJobsRequest) -> ProjectJobBatchDeleteCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Required.
    /// 
    /// The resource name of the project under which the job is created.
    /// 
    /// The format is "projects/{project_id}", for example,
    /// "projects/api-test-project".
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> ProjectJobBatchDeleteCall<'a, C, A> {
        self._parent = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ProjectJobBatchDeleteCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectJobBatchDeleteCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectJobBatchDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deletes specified company.
/// Prerequisite: The company has no jobs associated with it.
///
/// A builder for the *companies.delete* method supported by a *project* resource.
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
/// # extern crate google_jobs3 as jobs3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use jobs3::CloudTalentSolution;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudTalentSolution::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().companies_delete("name")
///              .doit();
/// # }
/// ```
pub struct ProjectCompanyDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudTalentSolution<C, A>,
    _name: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectCompanyDeleteCall<'a, C, A> {}

impl<'a, C, A> ProjectCompanyDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Empty)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "jobs.projects.companies.delete",
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

        let mut url = self.hub._base_url.clone() + "v3/{+name}";
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


    /// Required.
    /// 
    /// The resource name of the company to be deleted.
    /// 
    /// The format is "projects/{project_id}/companies/{company_id}", for example,
    /// "projects/api-test-project/companies/foo".
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> ProjectCompanyDeleteCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ProjectCompanyDeleteCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectCompanyDeleteCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectCompanyDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates specified job.
/// 
/// Typically, updated contents become visible in search results within 10
/// seconds, but it may take up to 5 minutes.
///
/// A builder for the *jobs.patch* method supported by a *project* resource.
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
/// # extern crate google_jobs3 as jobs3;
/// use jobs3::UpdateJobRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use jobs3::CloudTalentSolution;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudTalentSolution::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = UpdateJobRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().jobs_patch(req, "name")
///              .doit();
/// # }
/// ```
pub struct ProjectJobPatchCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudTalentSolution<C, A>,
    _request: UpdateJobRequest,
    _name: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectJobPatchCall<'a, C, A> {}

impl<'a, C, A> ProjectJobPatchCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Job)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "jobs.projects.jobs.patch",
                               http_method: hyper::method::Method::Patch });
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

        let mut url = self.hub._base_url.clone() + "v3/{+name}";
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Patch, url.clone())
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
    pub fn request(mut self, new_value: UpdateJobRequest) -> ProjectJobPatchCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Required during job update.
    /// 
    /// The resource name for the job. This is generated by the service when a
    /// job is created.
    /// 
    /// The format is "projects/{project_id}/jobs/{job_id}",
    /// for example, "projects/api-test-project/jobs/1234".
    /// 
    /// Use of this field in job queries and API calls is preferred over the use of
    /// requisition_id since this value is unique.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> ProjectJobPatchCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ProjectJobPatchCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectJobPatchCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectJobPatchCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Completes the specified prefix with keyword suggestions.
/// Intended for use by a job search auto-complete search box.
///
/// A builder for the *complete* method supported by a *project* resource.
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
/// # extern crate google_jobs3 as jobs3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use jobs3::CloudTalentSolution;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudTalentSolution::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().complete("name")
///              .type_("diam")
///              .scope("ipsum")
///              .query("Lorem")
///              .page_size(-21)
///              .add_language_codes("duo")
///              .language_code("aliquyam")
///              .company_name("sea")
///              .doit();
/// # }
/// ```
pub struct ProjectCompleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudTalentSolution<C, A>,
    _name: String,
    _type_: Option<String>,
    _scope: Option<String>,
    _query: Option<String>,
    _page_size: Option<i32>,
    _language_codes: Vec<String>,
    _language_code: Option<String>,
    _company_name: Option<String>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectCompleteCall<'a, C, A> {}

impl<'a, C, A> ProjectCompleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, CompleteQueryResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "jobs.projects.complete",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(10 + self._additional_params.len());
        params.push(("name", self._name.to_string()));
        if let Some(value) = self._type_ {
            params.push(("type", value.to_string()));
        }
        if let Some(value) = self._scope {
            params.push(("scope", value.to_string()));
        }
        if let Some(value) = self._query {
            params.push(("query", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if self._language_codes.len() > 0 {
            for f in self._language_codes.iter() {
                params.push(("languageCodes", f.to_string()));
            }
        }
        if let Some(value) = self._language_code {
            params.push(("languageCode", value.to_string()));
        }
        if let Some(value) = self._company_name {
            params.push(("companyName", value.to_string()));
        }
        for &field in ["alt", "name", "type", "scope", "query", "pageSize", "languageCodes", "languageCode", "companyName"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v3/{+name}:complete";
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


    /// Required.
    /// 
    /// Resource name of project the completion is performed within.
    /// 
    /// The format is "projects/{project_id}", for example,
    /// "projects/api-test-project".
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> ProjectCompleteCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// Optional.
    /// 
    /// The completion topic. The default is CompletionType.COMBINED.
    ///
    /// Sets the *type* query property to the given value.
    pub fn type_(mut self, new_value: &str) -> ProjectCompleteCall<'a, C, A> {
        self._type_ = Some(new_value.to_string());
        self
    }
    /// Optional.
    /// 
    /// The scope of the completion. The defaults is CompletionScope.PUBLIC.
    ///
    /// Sets the *scope* query property to the given value.
    pub fn scope(mut self, new_value: &str) -> ProjectCompleteCall<'a, C, A> {
        self._scope = Some(new_value.to_string());
        self
    }
    /// Required.
    /// 
    /// The query used to generate suggestions.
    /// 
    /// The maximum number of allowed characters is 255.
    ///
    /// Sets the *query* query property to the given value.
    pub fn query(mut self, new_value: &str) -> ProjectCompleteCall<'a, C, A> {
        self._query = Some(new_value.to_string());
        self
    }
    /// Required.
    /// 
    /// Completion result count.
    /// 
    /// The maximum allowed page size is 10.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> ProjectCompleteCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// Optional.
    /// 
    /// The list of languages of the query. This is
    /// the BCP-47 language code, such as "en-US" or "sr-Latn".
    /// For more information, see
    /// [Tags for Identifying Languages](https://tools.ietf.org/html/bcp47).
    /// 
    /// For CompletionType.JOB_TITLE type, only open jobs with the same
    /// language_codes are returned.
    /// 
    /// For CompletionType.COMPANY_NAME type,
    /// only companies having open jobs with the same language_codes are
    /// returned.
    /// 
    /// For CompletionType.COMBINED type, only open jobs with the same
    /// language_codes or companies having open jobs with the same
    /// language_codes are returned.
    /// 
    /// The maximum number of allowed characters is 255.
    ///
    /// Append the given value to the *language codes* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_language_codes(mut self, new_value: &str) -> ProjectCompleteCall<'a, C, A> {
        self._language_codes.push(new_value.to_string());
        self
    }
    /// Deprecated. Use language_codes instead.
    /// 
    /// Optional.
    /// 
    /// The language of the query. This is
    /// the BCP-47 language code, such as "en-US" or "sr-Latn".
    /// For more information, see
    /// [Tags for Identifying Languages](https://tools.ietf.org/html/bcp47).
    /// 
    /// For CompletionType.JOB_TITLE type, only open jobs with the same
    /// language_code are returned.
    /// 
    /// For CompletionType.COMPANY_NAME type,
    /// only companies having open jobs with the same language_code are
    /// returned.
    /// 
    /// For CompletionType.COMBINED type, only open jobs with the same
    /// language_code or companies having open jobs with the same
    /// language_code are returned.
    /// 
    /// The maximum number of allowed characters is 255.
    ///
    /// Sets the *language code* query property to the given value.
    pub fn language_code(mut self, new_value: &str) -> ProjectCompleteCall<'a, C, A> {
        self._language_code = Some(new_value.to_string());
        self
    }
    /// Optional.
    /// 
    /// If provided, restricts completion to specified company.
    /// 
    /// The format is "projects/{project_id}/companies/{company_id}", for example,
    /// "projects/api-test-project/companies/foo".
    ///
    /// Sets the *company name* query property to the given value.
    pub fn company_name(mut self, new_value: &str) -> ProjectCompleteCall<'a, C, A> {
        self._company_name = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ProjectCompleteCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectCompleteCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectCompleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


