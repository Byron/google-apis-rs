// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Playable Locations* crate version *1.0.14+20200707*, where *20200707* is the exact revision of the *playablelocations:v3* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.14*.
//! 
//! Everything else about the *Playable Locations* *v3* API can be found at the
//! [official documentation site](https://developers.google.com/maps/contact-sales/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/playablelocations3).
//! # Features
//! 
//! Use the following functionality with ease from the central [hub](struct.PlayableLocations.html) ... 
//! 
//! 
//! * [log impressions](struct.MethodLogImpressionCall.html)
//! * [log player reports](struct.MethodLogPlayerReportCall.html)
//! * [sample playable locations](struct.MethodSamplePlayableLocationCall.html)
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.PlayableLocations.html)**
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
//! let r = hub.methods().log_impressions(...).doit()
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
//! google-playablelocations3 = "*"
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
//! extern crate google_playablelocations3 as playablelocations3;
//! use playablelocations3::GoogleMapsPlayablelocationsV3LogImpressionsRequest;
//! use playablelocations3::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use playablelocations3::PlayableLocations;
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
//! let mut hub = PlayableLocations::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = GoogleMapsPlayablelocationsV3LogImpressionsRequest::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.methods().log_impressions(req)
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




// ########
// HUB ###
// ######

/// Central instance to access all PlayableLocations related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_playablelocations3 as playablelocations3;
/// use playablelocations3::GoogleMapsPlayablelocationsV3LogImpressionsRequest;
/// use playablelocations3::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use playablelocations3::PlayableLocations;
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
/// let mut hub = PlayableLocations::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleMapsPlayablelocationsV3LogImpressionsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.methods().log_impressions(req)
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
pub struct PlayableLocations<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for PlayableLocations<C, A> {}

impl<'a, C, A> PlayableLocations<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> PlayableLocations<C, A> {
        PlayableLocations {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.14".to_string(),
            _base_url: "https://playablelocations.googleapis.com/".to_string(),
            _root_url: "https://playablelocations.googleapis.com/".to_string(),
        }
    }

    pub fn methods(&'a self) -> MethodMethods<'a, C, A> {
        MethodMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.14`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://playablelocations.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://playablelocations.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// A request for logging impressions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [log impressions](struct.MethodLogImpressionCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleMapsPlayablelocationsV3LogImpressionsRequest {
    /// Required. Impression event details. The maximum number of impression reports that you
    /// can log at once is 50.
    pub impressions: Option<Vec<GoogleMapsPlayablelocationsV3Impression>>,
    /// Required. Information about the client device. For example, device model and
    /// operating system.
    #[serde(rename="clientInfo")]
    pub client_info: Option<GoogleMapsUnityClientInfo>,
    /// Required. A string that uniquely identifies the log impressions request. This allows
    /// you to detect duplicate requests. We recommend that you use UUIDs for this
    /// value. The value must not exceed 50 characters.
    /// 
    /// You should reuse the `request_id` only when retrying a request in case of
    /// failure. In this case, the request must be identical to the one that
    /// failed.
    #[serde(rename="requestId")]
    pub request_id: Option<String>,
}

impl RequestValue for GoogleMapsPlayablelocationsV3LogImpressionsRequest {}


/// 
/// Life of a query:
/// 
/// - When a game starts in a new location, your game server issues a
/// SamplePlayableLocations
/// request. The request specifies the S2 cell, and contains one or more
/// "criteria" for filtering:
/// 
/// - Criterion 0: i locations for long-lived bases, or level 0 monsters, or...
/// - Criterion 1: j locations for short-lived bases, or level 1 monsters, ...
/// - Criterion 2: k locations for random objects.
/// - etc (up to 5 criterion may be specified).
/// 
/// `PlayableLocationList` will then contain mutually
/// exclusive lists of `PlayableLocation` objects that satisfy each of
/// the criteria. Think of it as a collection of real-world locations that you
/// can then associate with your game state.
/// 
/// Note: These points are impermanent in nature. E.g, parks can close, and
/// places can be removed.
/// 
/// The response specifies how long you can expect the playable locations to
/// last. Once they expire, you should query the `samplePlayableLocations` API
/// again to get a fresh view of the real world.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sample playable locations](struct.MethodSamplePlayableLocationCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleMapsPlayablelocationsV3SamplePlayableLocationsRequest {
    /// Required. Specifies the area to search within for playable locations.
    #[serde(rename="areaFilter")]
    pub area_filter: Option<GoogleMapsPlayablelocationsV3SampleAreaFilter>,
    /// Required. Specifies one or more (up to 5) criteria for filtering the
    /// returned playable locations.
    pub criteria: Option<Vec<GoogleMapsPlayablelocationsV3SampleCriterion>>,
}

impl RequestValue for GoogleMapsPlayablelocationsV3SamplePlayableLocationsRequest {}


/// A response for the LogImpressions method.
/// This method returns no data upon success.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [log impressions](struct.MethodLogImpressionCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleMapsPlayablelocationsV3LogImpressionsResponse { _never_set: Option<bool> }

impl ResponseResult for GoogleMapsPlayablelocationsV3LogImpressionsResponse {}


/// Specifies the area to search for playable locations.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleMapsPlayablelocationsV3SampleAreaFilter {
    /// Required. The S2 cell ID of the area you want. This must be between cell level 11 and
    /// 14 (inclusive).
    /// 
    /// S2 cells are 64-bit integers that identify areas on the Earth. They are
    /// hierarchical, and can therefore be used for spatial indexing.
    /// 
    /// The S2 geometry library is available in a number of languages:
    /// 
    ///   * [C++](https://github.com/google/s2geometry)
    ///   * [Java](https://github.com/google/s2-geometry-library-java)
    ///   * [Go](https://github.com/golang/geo)
    ///   * [Python](https://github.com/google/s2geometry/tree/master/src/python)
    #[serde(rename="s2CellId")]
    pub s2_cell_id: Option<String>,
}

impl Part for GoogleMapsPlayablelocationsV3SampleAreaFilter {}


/// Specifies the filters to use when searching for playable locations.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleMapsPlayablelocationsV3SampleFilter {
    /// Specifies the maximum number of playable locations to return. This value
    /// must not be greater than 1000. The default value is 100.
    /// 
    /// Only the top-ranking playable locations are returned.
    #[serde(rename="maxLocationCount")]
    pub max_location_count: Option<i32>,
    /// A set of options that control the spacing between playable locations. By
    /// default the minimum distance between locations is 200m.
    pub spacing: Option<GoogleMapsPlayablelocationsV3SampleSpacingOptions>,
    /// Restricts the set of playable locations to just the
    /// [types](/maps/documentation/gaming/tt/types) that you want.
    #[serde(rename="includedTypes")]
    pub included_types: Option<Vec<String>>,
}

impl Part for GoogleMapsPlayablelocationsV3SampleFilter {}


/// 
/// Response for the
/// SamplePlayableLocations
/// method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sample playable locations](struct.MethodSamplePlayableLocationCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleMapsPlayablelocationsV3SamplePlayableLocationsResponse {
    /// Each PlayableLocation object corresponds to a game_object_type specified
    /// in the request.
    #[serde(rename="locationsPerGameObjectType")]
    pub locations_per_game_object_type: Option<HashMap<String, GoogleMapsPlayablelocationsV3SamplePlayableLocationList>>,
    /// Required. Specifies the "time-to-live" for the set of playable locations. You can use
    /// this value to determine how long to cache the set of playable locations.
    /// After this length of time, your back-end game server should issue a new
    /// SamplePlayableLocations
    /// request to get a fresh set of playable locations (because for example, they
    /// might have been removed, a park might have closed for the day, a
    /// business might have closed permanently).
    pub ttl: Option<String>,
}

impl ResponseResult for GoogleMapsPlayablelocationsV3SamplePlayableLocationsResponse {}


/// A response for the LogPlayerReports
/// method.
/// 
/// This method returns no data upon success.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [log player reports](struct.MethodLogPlayerReportCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleMapsPlayablelocationsV3LogPlayerReportsResponse { _never_set: Option<bool> }

impl ResponseResult for GoogleMapsPlayablelocationsV3LogPlayerReportsResponse {}


/// A geographical point suitable for placing game objects in location-based
/// games.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleMapsPlayablelocationsV3SamplePlayableLocation {
    /// Required. The name of this playable location.
    pub name: Option<String>,
    /// A [plus code] (http://openlocationcode.com)
    #[serde(rename="plusCode")]
    pub plus_code: Option<String>,
    /// Required. The latitude and longitude associated with the center of the playable
    /// location.
    /// 
    /// By default, the set of playable locations returned from
    /// SamplePlayableLocations use
    /// center-point coordinates.
    #[serde(rename="centerPoint")]
    pub center_point: Option<GoogleTypeLatLng>,
    /// A [place ID] (https://developers.google.com/places/place-id)
    #[serde(rename="placeId")]
    pub place_id: Option<String>,
    /// A collection of [Playable Location
    /// Types](/maps/documentation/gaming/tt/types) for this playable location. The
    /// first type in the collection is the primary type.
    /// 
    /// Type information might not be available for all playable locations.
    pub types: Option<Vec<String>>,
    /// The playable location's coordinates, snapped to the sidewalk of the
    /// nearest road, if a nearby road exists.
    #[serde(rename="snappedPoint")]
    pub snapped_point: Option<GoogleTypeLatLng>,
}

impl Part for GoogleMapsPlayablelocationsV3SamplePlayableLocation {}


/// Encapsulates impression event details.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleMapsPlayablelocationsV3Impression {
    /// Required. The name of the playable location.
    #[serde(rename="locationName")]
    pub location_name: Option<String>,
    /// An arbitrary, developer-defined type identifier for each type of game
    /// object used in your game.
    /// 
    /// Since players interact with differ types of game objects in different ways,
    /// this field allows you to segregate impression data by type for analysis.
    /// 
    /// You should assign a unique `game_object_type` ID to represent a distinct
    /// type of game object in your game.
    /// 
    /// For example, 1=monster location, 2=powerup location.
    #[serde(rename="gameObjectType")]
    pub game_object_type: Option<i32>,
    /// Required. The type of impression event.
    #[serde(rename="impressionType")]
    pub impression_type: Option<String>,
}

impl Part for GoogleMapsPlayablelocationsV3Impression {}


/// Client information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleMapsUnityClientInfo {
    /// Language code (in BCP-47 format) indicating the UI language of the client.
    /// Examples are "en", "en-US" or "ja-Latn". For more information, see
    /// http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.
    #[serde(rename="languageCode")]
    pub language_code: Option<String>,
    /// Application version number, such as "1.2.3". The exact format is
    /// application-dependent.
    #[serde(rename="applicationVersion")]
    pub application_version: Option<String>,
    /// Device model as reported by the device. The exact format is
    /// platform-dependent.
    #[serde(rename="deviceModel")]
    pub device_model: Option<String>,
    /// API client name and version. For example, the SDK calling the API. The
    /// exact format is up to the client.
    #[serde(rename="apiClient")]
    pub api_client: Option<String>,
    /// Platform where the application is running.
    pub platform: Option<String>,
    /// Application ID, such as the package name on Android and the bundle
    /// identifier on iOS platforms.
    #[serde(rename="applicationId")]
    pub application_id: Option<String>,
    /// Operating system name and version as reported by the OS. For example,
    /// "Mac OS X 10.10.4". The exact format is platform-dependent.
    #[serde(rename="operatingSystem")]
    pub operating_system: Option<String>,
    /// Build number/version of the operating system. e.g., the contents of
    /// android.os.Build.ID in Android, or the contents of sysctl "kern.osversion"
    /// in iOS.
    #[serde(rename="operatingSystemBuild")]
    pub operating_system_build: Option<String>,
}

impl Part for GoogleMapsUnityClientInfo {}


/// A list of PlayableLocation objects that satisfies a single Criterion.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleMapsPlayablelocationsV3SamplePlayableLocationList {
    /// A list of playable locations for this game object type.
    pub locations: Option<Vec<GoogleMapsPlayablelocationsV3SamplePlayableLocation>>,
}

impl Part for GoogleMapsPlayablelocationsV3SamplePlayableLocationList {}


/// An object representing a latitude/longitude pair. This is expressed as a pair
/// of doubles representing degrees latitude and degrees longitude. Unless
/// specified otherwise, this must conform to the
/// <a href="http://www.unoosa.org/pdf/icg/2012/template/WGS_84.pdf">WGS84
/// standard</a>. Values must be within normalized ranges.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeLatLng {
    /// The latitude in degrees. It must be in the range [-90.0, +90.0].
    pub latitude: Option<f64>,
    /// The longitude in degrees. It must be in the range [-180.0, +180.0].
    pub longitude: Option<f64>,
}

impl Part for GoogleTypeLatLng {}


/// A set of options that specifies the separation between playable locations.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleMapsPlayablelocationsV3SampleSpacingOptions {
    /// Required. The minimum spacing between any two playable locations, measured in meters.
    /// The minimum value is 30.
    /// The maximum value is 1000.
    /// 
    /// Inputs will be rounded up to the next 10 meter interval.
    /// 
    /// The default value is 200m.
    /// 
    /// Set this field to remove tight clusters of playable locations.
    /// 
    /// Note:
    /// 
    /// The spacing is a greedy algorithm. It optimizes for selecting the highest
    /// ranking locations first, not to maximize the number of locations selected.
    /// Consider the following scenario:
    /// 
    ///   * Rank: A: 2, B: 1, C: 3.
    ///   * Distance: A--200m--B--200m--C
    /// 
    /// If spacing=250, it will pick the highest ranked location [B], not [A, C].
    /// 
    /// 
    /// Note:
    /// 
    /// Spacing works within the game object type itself, as well as the previous
    /// ones.
    /// Suppose three game object types, each with the following spacing:
    /// 
    ///   * X: 400m, Y: undefined, Z: 200m.
    /// 
    /// 1. Add locations for X, within 400m of each other.
    /// 2. Add locations for Y, without any spacing.
    /// 3. Finally, add locations for Z within 200m of each other as well X and Y.
    /// 
    /// The distance diagram between those locations end up as:
    /// 
    ///   * From->To.
    ///   * X->X: 400m
    ///   * Y->X, Y->Y: unspecified.
    ///   * Z->X, Z->Y, Z->Z: 200m.
    #[serde(rename="minSpacingMeters")]
    pub min_spacing_meters: Option<f64>,
    /// Specifies whether the minimum spacing constraint applies to the
    /// center-point or to the snapped point of playable locations. The default
    /// value is `CENTER_POINT`.
    /// 
    /// If a snapped point is not available for a playable location, its
    /// center-point is used instead.
    /// 
    /// Set this to the point type used in your game.
    #[serde(rename="pointType")]
    pub point_type: Option<String>,
}

impl Part for GoogleMapsPlayablelocationsV3SampleSpacingOptions {}


/// Encapsulates a filter criterion for searching for a set of playable
/// locations.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleMapsPlayablelocationsV3SampleCriterion {
    /// Specifies which `PlayableLocation` fields are returned.
    /// 
    /// `name` (which is used for logging impressions), `center_point` and
    /// `place_id` (or `plus_code`) are always returned.
    /// 
    /// The following fields are omitted unless you specify them here:
    /// 
    ///   * snapped_point
    ///   * types
    /// 
    /// Note: The more fields you include, the more expensive in terms of data and
    /// associated latency your query will be.
    #[serde(rename="fieldsToReturn")]
    pub fields_to_return: Option<String>,
    /// Specifies filtering options, and specifies what will be included in the
    /// result set.
    pub filter: Option<GoogleMapsPlayablelocationsV3SampleFilter>,
    /// Required. An arbitrary, developer-defined identifier of the type of game object that
    /// the playable location is used for. This field allows you to specify
    /// criteria per game object type when searching for playable locations.
    /// 
    /// You should assign a unique `game_object_type` ID across all
    /// `request_criteria` to represent a distinct type of game object. For
    /// example, 1=monster location, 2=powerup location.
    /// 
    /// The response contains a map<game_object_type, Response>.
    #[serde(rename="gameObjectType")]
    pub game_object_type: Option<i32>,
}

impl Part for GoogleMapsPlayablelocationsV3SampleCriterion {}


/// A request for logging your player's bad location reports.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [log player reports](struct.MethodLogPlayerReportCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleMapsPlayablelocationsV3LogPlayerReportsRequest {
    /// Required. Information about the client device (for example, device model and
    /// operating system).
    #[serde(rename="clientInfo")]
    pub client_info: Option<GoogleMapsUnityClientInfo>,
    /// Required. Player reports. The maximum number of player reports that you can log at
    /// once is 50.
    #[serde(rename="playerReports")]
    pub player_reports: Option<Vec<GoogleMapsPlayablelocationsV3PlayerReport>>,
    /// Required. A string that uniquely identifies the log player reports request. This
    /// allows you to detect duplicate requests. We recommend that you use UUIDs
    /// for this value. The value must not exceed 50 characters.
    /// 
    /// You should reuse the `request_id` only when retrying a request in the case
    /// of a failure. In that case, the request must be identical to the one that
    /// failed.
    #[serde(rename="requestId")]
    pub request_id: Option<String>,
}

impl RequestValue for GoogleMapsPlayablelocationsV3LogPlayerReportsRequest {}


/// A report submitted by a player about a playable location that is considered
/// inappropriate for use in the game.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleMapsPlayablelocationsV3PlayerReport {
    /// Required. The name of the playable location.
    #[serde(rename="locationName")]
    pub location_name: Option<String>,
    /// Language code (in BCP-47 format) indicating the language of the freeform
    /// description provided in `reason_details`. Examples are "en", "en-US" or
    /// "ja-Latn". For more information, see
    /// http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.
    #[serde(rename="languageCode")]
    pub language_code: Option<String>,
    /// Required. A free-form description detailing why the playable location is
    /// considered bad.
    #[serde(rename="reasonDetails")]
    pub reason_details: Option<String>,
    /// Required. One or more reasons why this playable location is considered bad.
    pub reasons: Option<Vec<String>>,
}

impl Part for GoogleMapsPlayablelocationsV3PlayerReport {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all free methods, which are not associated with a particular resource.
/// It is not used directly, but through the `PlayableLocations` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_playablelocations3 as playablelocations3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use playablelocations3::PlayableLocations;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = PlayableLocations::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `log_impressions(...)`, `log_player_reports(...)` and `sample_playable_locations(...)`
/// // to build up your call.
/// let rb = hub.methods();
/// # }
/// ```
pub struct MethodMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PlayableLocations<C, A>,
}

impl<'a, C, A> MethodsBuilder for MethodMethods<'a, C, A> {}

impl<'a, C, A> MethodMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Logs new events when playable locations are displayed, and when they are
    /// interacted with.
    /// 
    /// Impressions are not partially saved; either all impressions are saved and
    /// this request succeeds, or no impressions are saved, and this request fails.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn log_impressions(&self, request: GoogleMapsPlayablelocationsV3LogImpressionsRequest) -> MethodLogImpressionCall<'a, C, A> {
        MethodLogImpressionCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Logs bad playable location reports submitted by players.
    /// 
    /// Reports are not partially saved; either all reports are saved and this
    /// request succeeds, or no reports are saved, and this request fails.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn log_player_reports(&self, request: GoogleMapsPlayablelocationsV3LogPlayerReportsRequest) -> MethodLogPlayerReportCall<'a, C, A> {
        MethodLogPlayerReportCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a set of playable locations that lie within a specified area,
    /// that satisfy optional filter criteria.
    /// 
    /// Note: Identical `SamplePlayableLocations` requests can return different
    /// results as the state of the world changes over time.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn sample_playable_locations(&self, request: GoogleMapsPlayablelocationsV3SamplePlayableLocationsRequest) -> MethodSamplePlayableLocationCall<'a, C, A> {
        MethodSamplePlayableLocationCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Logs new events when playable locations are displayed, and when they are
/// interacted with.
/// 
/// Impressions are not partially saved; either all impressions are saved and
/// this request succeeds, or no impressions are saved, and this request fails.
///
/// A builder for the *logImpressions* method.
/// It is not used directly, but through a `MethodMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_playablelocations3 as playablelocations3;
/// use playablelocations3::GoogleMapsPlayablelocationsV3LogImpressionsRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use playablelocations3::PlayableLocations;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = PlayableLocations::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleMapsPlayablelocationsV3LogImpressionsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.methods().log_impressions(req)
///              .doit();
/// # }
/// ```
pub struct MethodLogImpressionCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PlayableLocations<C, A>,
    _request: GoogleMapsPlayablelocationsV3LogImpressionsRequest,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for MethodLogImpressionCall<'a, C, A> {}

impl<'a, C, A> MethodLogImpressionCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GoogleMapsPlayablelocationsV3LogImpressionsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "playablelocations.logImpressions",
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

        let mut url = self.hub._base_url.clone() + "v3:logImpressions";
        
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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
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
    pub fn request(mut self, new_value: GoogleMapsPlayablelocationsV3LogImpressionsRequest) -> MethodLogImpressionCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> MethodLogImpressionCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> MethodLogImpressionCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Logs bad playable location reports submitted by players.
/// 
/// Reports are not partially saved; either all reports are saved and this
/// request succeeds, or no reports are saved, and this request fails.
///
/// A builder for the *logPlayerReports* method.
/// It is not used directly, but through a `MethodMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_playablelocations3 as playablelocations3;
/// use playablelocations3::GoogleMapsPlayablelocationsV3LogPlayerReportsRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use playablelocations3::PlayableLocations;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = PlayableLocations::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleMapsPlayablelocationsV3LogPlayerReportsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.methods().log_player_reports(req)
///              .doit();
/// # }
/// ```
pub struct MethodLogPlayerReportCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PlayableLocations<C, A>,
    _request: GoogleMapsPlayablelocationsV3LogPlayerReportsRequest,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for MethodLogPlayerReportCall<'a, C, A> {}

impl<'a, C, A> MethodLogPlayerReportCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GoogleMapsPlayablelocationsV3LogPlayerReportsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "playablelocations.logPlayerReports",
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

        let mut url = self.hub._base_url.clone() + "v3:logPlayerReports";
        
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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
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
    pub fn request(mut self, new_value: GoogleMapsPlayablelocationsV3LogPlayerReportsRequest) -> MethodLogPlayerReportCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> MethodLogPlayerReportCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> MethodLogPlayerReportCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Returns a set of playable locations that lie within a specified area,
/// that satisfy optional filter criteria.
/// 
/// Note: Identical `SamplePlayableLocations` requests can return different
/// results as the state of the world changes over time.
///
/// A builder for the *samplePlayableLocations* method.
/// It is not used directly, but through a `MethodMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_playablelocations3 as playablelocations3;
/// use playablelocations3::GoogleMapsPlayablelocationsV3SamplePlayableLocationsRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use playablelocations3::PlayableLocations;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = PlayableLocations::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleMapsPlayablelocationsV3SamplePlayableLocationsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.methods().sample_playable_locations(req)
///              .doit();
/// # }
/// ```
pub struct MethodSamplePlayableLocationCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PlayableLocations<C, A>,
    _request: GoogleMapsPlayablelocationsV3SamplePlayableLocationsRequest,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for MethodSamplePlayableLocationCall<'a, C, A> {}

impl<'a, C, A> MethodSamplePlayableLocationCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GoogleMapsPlayablelocationsV3SamplePlayableLocationsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "playablelocations.samplePlayableLocations",
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

        let mut url = self.hub._base_url.clone() + "v3:samplePlayableLocations";
        
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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
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
    pub fn request(mut self, new_value: GoogleMapsPlayablelocationsV3SamplePlayableLocationsRequest) -> MethodSamplePlayableLocationCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> MethodSamplePlayableLocationCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> MethodSamplePlayableLocationCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


