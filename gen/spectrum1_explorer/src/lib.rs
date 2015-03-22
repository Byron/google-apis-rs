// DO NOT EDIT !
// This file was generated automatically from 'src/mako/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *spectrum* crate version *0.1.1+20150112*, where *20150112* is the exact revision of the *spectrum:v1explorer* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.1*.
//! 
//! Everything else about the *spectrum* *v1_explorer* API can be found at the
//! [official documentation site](http://developers.google.com/spectrum).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/spectrum1_explorer).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Spectrum.html) ... 
//! 
//! * paws
//!  * [*get spectrum*](struct.PawGetSpectrumCall.html), [*get spectrum batch*](struct.PawGetSpectrumBatchCall.html), [*init*](struct.PawInitCall.html), [*notify spectrum use*](struct.PawNotifySpectrumUseCall.html), [*register*](struct.PawRegisterCall.html) and [*verify device*](struct.PawVerifyDeviceCall.html)
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
//! * **[Hub](struct.Spectrum.html)**
//!     * a central object to maintain state and allow accessing all *Activities*
//! * **[Resources](trait.Resource.html)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](trait.Part.html)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](trait.CallBuilder.html)**
//!     * operations to apply to *Resources*
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
//! let r = hub.paws().get_spectrum_batch(...).doit()
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
//! google-spectrum1_explorer = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate "yup-oauth2" as oauth2;
//! extern crate "google-spectrum1_explorer" as spectrum1_explorer;
//! use spectrum1_explorer::PawsGetSpectrumBatchRequest;
//! use spectrum1_explorer::Result;
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use spectrum1_explorer::Spectrum;
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
//! let mut hub = Spectrum::new(hyper::Client::new(), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req: PawsGetSpectrumBatchRequest = Default::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.paws().get_spectrum_batch(&req)
//!              .doit();
//! 
//! match result {
//!     Result::HttpError(err) => println!("HTTPERROR: {:?}", err),
//!     Result::MissingAPIKey => println!("Auth: Missing API Key - used if there are no scopes"),
//!     Result::MissingToken => println!("OAuth2: Missing Token"),
//!     Result::Cancelled => println!("Operation cancelled by user"),
//!     Result::UploadSizeLimitExceeded(size, max_size) => println!("Upload size too big: {} of {}", size, max_size),
//!     Result::Failure(_) => println!("General Failure (hyper::client::Response doesn't print)"),
//!     Result::FieldClash(clashed_field) => println!("You added custom parameter which is part of builder: {:?}", clashed_field),
//!     Result::JsonDecodeError(err) => println!("Couldn't understand server reply - maybe API needs update: {:?}", err),
//!     Result::Success(_) => println!("Success (value doesn't print)"),
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
//! ## Uploads and Downlods
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
//! [decodable](trait.ResponseResult.html) via json. Optionals are used to indicate that partial requests are responses are valid.
//! Most optionals are are considered [Parts](trait.Part.html) which are identifyable by name, which will be sent to 
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
#![feature(core,io,thread_sleep)]
// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any 
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]
// Required for serde annotations
#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]

#[macro_use]
extern crate hyper;
extern crate serde;
extern crate "yup-oauth2" as oauth2;
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
use std::thread::sleep;

pub use cmn::{MultiPartReader, ToParts, MethodInfo, Result, CallBuilder, Hub, ReadSeek, Part, ResponseResult, RequestValue, NestedType, Delegate, DefaultDelegate, ResourceMethodsBuilder, Resource, JsonServerError};


// ##############
// UTILITIES ###
// ############




// ########
// HUB ###
// ######

/// Central instance to access all Spectrum related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-spectrum1_explorer" as spectrum1_explorer;
/// use spectrum1_explorer::PawsGetSpectrumBatchRequest;
/// use spectrum1_explorer::Result;
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use spectrum1_explorer::Spectrum;
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
/// let mut hub = Spectrum::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: PawsGetSpectrumBatchRequest = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.paws().get_spectrum_batch(&req)
///              .doit();
/// 
/// match result {
///     Result::HttpError(err) => println!("HTTPERROR: {:?}", err),
///     Result::MissingAPIKey => println!("Auth: Missing API Key - used if there are no scopes"),
///     Result::MissingToken => println!("OAuth2: Missing Token"),
///     Result::Cancelled => println!("Operation cancelled by user"),
///     Result::UploadSizeLimitExceeded(size, max_size) => println!("Upload size too big: {} of {}", size, max_size),
///     Result::Failure(_) => println!("General Failure (hyper::client::Response doesn't print)"),
///     Result::FieldClash(clashed_field) => println!("You added custom parameter which is part of builder: {:?}", clashed_field),
///     Result::JsonDecodeError(err) => println!("Couldn't understand server reply - maybe API needs update: {:?}", err),
///     Result::Success(_) => println!("Success (value doesn't print)"),
/// }
/// # }
/// ```
pub struct Spectrum<C, NC, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,

    _m: PhantomData<NC>
}

impl<'a, C, NC, A> Hub for Spectrum<C, NC, A> {}

impl<'a, C, NC, A> Spectrum<C, NC, A>
    where  NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Spectrum<C, NC, A> {
        Spectrum {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/0.1.1".to_string(),
            _m: PhantomData
        }
    }

    pub fn paws(&'a self) -> PawMethods<'a, C, NC, A> {
        PawMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/0.1.1`.
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
/// This parameter is used to specify the geolocation of the device.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GeoLocation {
    /// The location confidence level, as an integer percentage, may be required, depending on the regulatory domain. When the parameter is optional and not provided, its value is assumed to be 95. Valid values range from 0 to 99, since, in practice, 100-percent confidence is not achievable. The confidence value is meaningful only when geolocation refers to a point with uncertainty.    
    pub confidence: i32,
    /// If present, indicates that the geolocation represents a region. Database support for regions is optional.    
    pub region: GeoLocationPolygon,
    /// If present, indicates that the geolocation represents a point. Paradoxically, a point is parameterized using an ellipse, where the center represents the location of the point and the distances along the major and minor axes represent the uncertainty. The uncertainty values may be required, depending on the regulatory domain.    
    pub point: GeoLocationEllipse,
}

impl Part for GeoLocation {}


/// A "point" with uncertainty is represented using the Ellipse shape.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GeoLocationEllipse {
    /// A required geo-spatial point representing the center of the ellipse.    
    pub center: GeoLocationPoint,
    /// A floating-point number that expresses the location uncertainty along the minor axis of the ellipse. May be required by the regulatory domain. When the uncertainty is optional, the default value is 0.    
    #[serde(alias="semiMinorAxis")]
    pub semi_minor_axis: f64,
    /// A floating-point number that expresses the orientation of the ellipse, representing the rotation, in degrees, of the semi-major axis from North towards the East. For example, when the uncertainty is greatest along the North-South direction, orientation is 0 degrees; conversely, if the uncertainty is greatest along the East-West direction, orientation is 90 degrees. When orientation is not present, the orientation is assumed to be 0.    
    pub orientation: f64,
    /// A floating-point number that expresses the location uncertainty along the major axis of the ellipse. May be required by the regulatory domain. When the uncertainty is optional, the default value is 0.    
    #[serde(alias="semiMajorAxis")]
    pub semi_major_axis: f64,
}

impl Part for GeoLocationEllipse {}


/// The structure used to represent an organization and an email address.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct VcardTypedText {
    /// The text string associated with this item. For example, for an org field: ACME, inc. For an email field: smith@example.com.    
    pub text: String,
}

impl Part for VcardTypedText {}


/// The spectrum-use notification message which must contain the geolocation of the Device and parameters required by the regulatory domain.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notify spectrum use paws](struct.PawNotifySpectrumUseCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct PawsNotifySpectrumUseRequest {
    /// Device descriptor information is required in the spectrum-use notification message.    
    #[serde(alias="deviceDesc")]
    pub device_desc: Option<DeviceDescriptor>,
    /// The message type (e.g., INIT_REQ, AVAIL_SPECTRUM_REQ, ...).
    /// 
    /// Required field.
    #[serde(alias="type")]
    pub type_: Option<String>,
    /// The geolocation of the master device (the device that is sending the spectrum-use notification) to the database is required in the spectrum-use notification message.    
    pub location: Option<GeoLocation>,
    /// The PAWS version. Must be exactly 1.0.
    /// 
    /// Required field.
    pub version: Option<String>,
    /// A spectrum list is required in the spectrum-use notification. The list specifies the spectrum that the device expects to use, which includes frequency ranges and maximum power levels. The list may be empty if the device decides not to use any of spectrum. For consistency, the psdBandwidthHz value should match that from one of the spectrum elements in the corresponding available spectrum response previously sent to the device by the database. Note that maximum power levels in the spectrum element must be expressed as power spectral density over the specified psdBandwidthHz value. The actual bandwidth to be used (as computed from the start and stop frequencies) may be different from the psdBandwidthHz value. As an example, when regulatory rules express maximum power spectral density in terms of maximum power over any 100 kHz band, then the psdBandwidthHz value should be set to 100 kHz, even though the actual bandwidth used can be 20 kHz.    
    pub spectra: Option<Vec<SpectrumMessage>>,
}

impl RequestValue for PawsNotifySpectrumUseRequest {}


/// This parameter contains device-owner information required as part of device registration. The regulatory domains may require additional parameters.
/// 
/// All contact information must be expressed using the structure defined by the vCard format specification. Only the contact fields of vCard are supported:  
/// - fn: Full name of an individual 
/// - org: Name of the organization 
/// - adr: Address fields 
/// - tel: Telephone numbers 
/// - email: Email addresses  
/// 
/// Note that the vCard specification defines maximum lengths for each field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct DeviceOwner {
    /// The vCard contact information for the device operator is optional, but may be required by specific regulatory domains.    
    pub operator: Vcard,
    /// The vCard contact information for the individual or business that owns the device is required.    
    pub owner: Vcard,
}

impl Part for DeviceOwner {}


/// The start and stop times of an event. This is used to indicate the time period for which a spectrum profile is valid.
/// 
/// Both times are expressed using the format, YYYY-MM-DDThh:mm:ssZ, as defined in RFC3339. The times must be expressed using UTC.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct EventTime {
    /// The exclusive end of the event. It will be present.    
    #[serde(alias="stopTime")]
    pub stop_time: String,
    /// The inclusive start of the event. It will be present.    
    #[serde(alias="startTime")]
    pub start_time: String,
}

impl Part for EventTime {}


/// A specific range of frequencies together with the associated maximum power level and channel identifier.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FrequencyRange {
    /// The required inclusive start of the frequency range (in Hertz).    
    #[serde(alias="startHz")]
    pub start_hz: f64,
    /// The database may include a channel identifier, when applicable. When it is included, the device should treat it as informative. The length of the identifier should not exceed 16 characters.    
    #[serde(alias="channelId")]
    pub channel_id: String,
    /// The required exclusive end of the frequency range (in Hertz).    
    #[serde(alias="stopHz")]
    pub stop_hz: f64,
    /// The maximum total power level (EIRP)—computed over the corresponding operating bandwidth—that is permitted within the frequency range. Depending on the context in which the frequency-range element appears, this value may be required. For example, it is required in the available-spectrum response, available-spectrum-batch response, and spectrum-use notification message, but it should not be present (it is not applicable) when the frequency range appears inside a device-capabilities message.    
    #[serde(alias="maxPowerDBm")]
    pub max_power_d_bm: f64,
}

impl Part for FrequencyRange {}


/// The response message for the batch available spectrum query contains a schedule of available spectrum for the device at multiple locations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get spectrum batch paws](struct.PawGetSpectrumBatchCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct PawsGetSpectrumBatchResponse {
    /// The available spectrum batch response must contain a geo-spectrum schedule list, The list may be empty if spectrum is not available. The database may return more than one geo-spectrum schedule to represent future changes to the available spectrum. How far in advance a schedule may be provided depends upon the applicable regulatory domain. The database may return available spectrum for fewer geolocations than requested. The device must not make assumptions about the order of the entries in the list, and must use the geolocation value in each geo-spectrum schedule entry to match available spectrum to a location.    
    #[serde(alias="geoSpectrumSchedules")]
    pub geo_spectrum_schedules: Vec<GeoSpectrumSchedule>,
    /// Identifies what kind of resource this is. Value: the fixed string "spectrum#pawsGetSpectrumBatchResponse".    
    pub kind: String,
    /// The database must return in its available spectrum response the device descriptor information it received in the master device's available spectrum batch request.    
    #[serde(alias="deviceDesc")]
    pub device_desc: DeviceDescriptor,
    /// A database may include the databaseChange parameter to notify a device of a change to its database URI, providing one or more alternate database URIs. The device should use this information to update its list of pre-configured databases by (only) replacing its entry for the responding database with the list of alternate URIs.    
    #[serde(alias="databaseChange")]
    pub database_change: DbUpdateSpec,
    /// The database may return a constraint on the allowed maximum total bandwidth (in Hertz), which does not need to be contiguous. A regulatory domain may require the database to return this parameter. When this parameter is present in the available spectrum batch response, the device must apply this constraint to its spectrum-selection logic to ensure that total bandwidth does not exceed this value.    
    #[serde(alias="maxTotalBwHz")]
    pub max_total_bw_hz: f64,
    /// The database may return a constraint on the allowed maximum contiguous bandwidth (in Hertz). A regulatory domain may require the database to return this parameter. When this parameter is present in the response, the device must apply this constraint to its spectrum-selection logic to ensure that no single block of spectrum has bandwidth that exceeds this value.    
    #[serde(alias="maxContiguousBwHz")]
    pub max_contiguous_bw_hz: f64,
    /// The PAWS version. Must be exactly 1.0.
    /// 
    /// Required field.
    pub version: String,
    /// The database includes a timestamp of the form, YYYY-MM-DDThh:mm:ssZ (Internet timestamp format per RFC3339), in its available spectrum batch response. The timestamp should be used by the device as a reference for the start and stop times specified in the response spectrum schedules.    
    pub timestamp: String,
    /// The database should return ruleset information, which identifies the applicable regulatory authority and ruleset for the available spectrum batch response. If included, the device must use the corresponding ruleset to interpret the response. Values provided in the returned ruleset information, such as maxLocationChange, take precedence over any conflicting values provided in the ruleset information returned in a prior initialization response sent by the database to the device.    
    #[serde(alias="rulesetInfo")]
    pub ruleset_info: RulesetInfo,
    /// For regulatory domains that require a spectrum-usage report from devices, the database must return true for this parameter if the geo-spectrum schedules list is not empty; otherwise, the database should either return false or omit this parameter. If this parameter is present and its value is true, the device must send a spectrum use notify message to the database; otherwise, the device should not send the notification.    
    #[serde(alias="needsSpectrumReport")]
    pub needs_spectrum_report: bool,
    /// The message type (e.g., INIT_REQ, AVAIL_SPECTRUM_REQ, ...).
    /// 
    /// Required field.
    #[serde(alias="type")]
    pub type_: String,
}

impl ResponseResult for PawsGetSpectrumBatchResponse {}


/// The structure used to represent a telephone number.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct VcardTelephone {
    /// A nested telephone URI of the form: tel:+1-123-456-7890.    
    pub uri: String,
}

impl Part for VcardTelephone {}


/// The device validity element describes whether a particular device is valid to operate in the regulatory domain.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct DeviceValidity {
    /// The validity status: true if the device is valid for operation, false otherwise. It will always be present.    
    #[serde(alias="isValid")]
    pub is_valid: bool,
    /// If the device identifier is not valid, the database may include a reason. The reason may be in any language. The length of the value should not exceed 128 characters.    
    pub reason: String,
    /// The descriptor of the device for which the validity check was requested. It will always be present.    
    #[serde(alias="deviceDesc")]
    pub device_desc: DeviceDescriptor,
}

impl Part for DeviceValidity {}


/// The initialization response message communicates database parameters to the requesting device.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [init paws](struct.PawInitCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct PawsInitResponse {
    /// A database may include the databaseChange parameter to notify a device of a change to its database URI, providing one or more alternate database URIs. The device should use this information to update its list of pre-configured databases by (only) replacing its entry for the responding database with the list of alternate URIs.    
    #[serde(alias="databaseChange")]
    pub database_change: DbUpdateSpec,
    /// Identifies what kind of resource this is. Value: the fixed string "spectrum#pawsInitResponse".    
    pub kind: String,
    /// The PAWS version. Must be exactly 1.0.
    /// 
    /// Required field.
    pub version: String,
    /// The message type (e.g., INIT_REQ, AVAIL_SPECTRUM_REQ, ...).
    /// 
    /// Required field.
    #[serde(alias="type")]
    pub type_: String,
    /// The rulesetInfo parameter must be included in the response. This parameter specifies the regulatory domain and parameters applicable to that domain. The database must include the authority field, which defines the regulatory domain for the location specified in the INIT_REQ message.    
    #[serde(alias="rulesetInfo")]
    pub ruleset_info: RulesetInfo,
}

impl ResponseResult for PawsInitResponse {}


/// An empty response to the notification.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notify spectrum use paws](struct.PawNotifySpectrumUseCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct PawsNotifySpectrumUseResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "spectrum#pawsNotifySpectrumUseResponse".    
    pub kind: String,
    /// The PAWS version. Must be exactly 1.0.
    /// 
    /// Required field.
    pub version: String,
    /// The message type (e.g., INIT_REQ, AVAIL_SPECTRUM_REQ, ...).
    /// 
    /// Required field.
    #[serde(alias="type")]
    pub type_: String,
}

impl ResponseResult for PawsNotifySpectrumUseResponse {}


/// The device validation response message.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [verify device paws](struct.PawVerifyDeviceCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct PawsVerifyDeviceResponse {
    /// A database may include the databaseChange parameter to notify a device of a change to its database URI, providing one or more alternate database URIs. The device should use this information to update its list of pre-configured databases by (only) replacing its entry for the responding database with the list of alternate URIs.    
    #[serde(alias="databaseChange")]
    pub database_change: DbUpdateSpec,
    /// Identifies what kind of resource this is. Value: the fixed string "spectrum#pawsVerifyDeviceResponse".    
    pub kind: String,
    /// The PAWS version. Must be exactly 1.0.
    /// 
    /// Required field.
    pub version: String,
    /// A device validities list is required in the device validation response to report whether each slave device listed in a previous device validation request is valid. The number of entries must match the number of device descriptors listed in the previous device validation request.    
    #[serde(alias="deviceValidities")]
    pub device_validities: Vec<DeviceValidity>,
    /// The message type (e.g., INIT_REQ, AVAIL_SPECTRUM_REQ, ...).
    /// 
    /// Required field.
    #[serde(alias="type")]
    pub type_: String,
}

impl ResponseResult for PawsVerifyDeviceResponse {}


/// Antenna characteristics provide additional information, such as the antenna height, antenna type, etc. Whether antenna characteristics must be provided in a request depends on the device type and regulatory domain.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct AntennaCharacteristics {
    /// If the height is required, then the height type (AGL for above ground level or AMSL for above mean sea level) is also required. The default is AGL.    
    #[serde(alias="heightType")]
    pub height_type: String,
    /// The height uncertainty in meters. Whether this is required depends on the regulatory domain.    
    #[serde(alias="heightUncertainty")]
    pub height_uncertainty: f64,
    /// The antenna height in meters. Whether the antenna height is required depends on the device type and the regulatory domain. Note that the height may be negative.    
    pub height: f64,
}

impl Part for AntennaCharacteristics {}


/// A single geolocation on the globe.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GeoLocationPoint {
    /// A required floating-point number that expresses the latitude in degrees using the WGS84 datum. For details on this encoding, see the National Imagery and Mapping Agency's Technical Report TR8350.2.    
    pub latitude: f64,
    /// A required floating-point number that expresses the longitude in degrees using the WGS84 datum. For details on this encoding, see the National Imagery and Mapping Agency's Technical Report TR8350.2.    
    pub longitude: f64,
}

impl Part for GeoLocationPoint {}


/// The registration response message simply acknowledges receipt of the request and is otherwise empty.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [register paws](struct.PawRegisterCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct PawsRegisterResponse {
    /// A database may include the databaseChange parameter to notify a device of a change to its database URI, providing one or more alternate database URIs. The device should use this information to update its list of pre-configured databases by (only) replacing its entry for the responding database with the list of alternate URIs.    
    #[serde(alias="databaseChange")]
    pub database_change: DbUpdateSpec,
    /// Identifies what kind of resource this is. Value: the fixed string "spectrum#pawsRegisterResponse".    
    pub kind: String,
    /// The PAWS version. Must be exactly 1.0.
    /// 
    /// Required field.
    pub version: String,
    /// The message type (e.g., INIT_REQ, AVAIL_SPECTRUM_REQ, ...).
    /// 
    /// Required field.
    #[serde(alias="type")]
    pub type_: String,
}

impl ResponseResult for PawsRegisterResponse {}


/// A vCard-in-JSON message that contains only the fields needed for PAWS:  
/// - fn: Full name of an individual 
/// - org: Name of the organization 
/// - adr: Address fields 
/// - tel: Telephone numbers 
/// - email: Email addresses
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct Vcard {
    /// The organization associated with the registering entity.    
    pub org: VcardTypedText,
    /// A telephone number that can be used to call the contact.    
    pub tel: VcardTelephone,
    /// An email address that can be used to reach the contact.    
    pub email: VcardTypedText,
    /// The full name of the contact person. For example: John A. Smith.    
    #[serde(alias="fn")]
    pub fn_: String,
    /// The street address of the entity.    
    pub adr: VcardAddress,
}

impl Part for Vcard {}


/// The request message for a batch available spectrum query protocol.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get spectrum batch paws](struct.PawGetSpectrumBatchCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct PawsGetSpectrumBatchRequest {
    /// When the available spectrum request is made on behalf of a specific device (a master or slave device), device descriptor information for the device on whose behalf the request is made is required (in such cases, the requestType parameter must be empty). When a requestType value is specified, device descriptor information may be optional or required according to the rules of the applicable regulatory domain.    
    #[serde(alias="deviceDesc")]
    pub device_desc: Option<DeviceDescriptor>,
    /// Depending on device type and regulatory domain, antenna characteristics may be required.    
    pub antenna: Option<AntennaCharacteristics>,
    /// The request type parameter is an optional parameter that can be used to modify an available spectrum batch request, but its use depends on applicable regulatory rules. For example, It may be used to request generic slave device parameters without having to specify the device descriptor for a specific device. When the requestType parameter is missing, the request is for a specific device (master or slave), and the device descriptor parameter for the device on whose behalf the batch request is made is required.    
    #[serde(alias="requestType")]
    pub request_type: Option<String>,
    /// When an available spectrum batch request is made by the master device (a device with geolocation capability) on behalf of a slave device (a device without geolocation capability), the rules of the applicable regulatory domain may require the master device to provide its own device descriptor information (in addition to device descriptor information for the slave device in a separate parameter).    
    #[serde(alias="masterDeviceDesc")]
    pub master_device_desc: Option<DeviceDescriptor>,
    /// A geolocation list is required. This allows a device to specify its current location plus additional anticipated locations when allowed by the regulatory domain. At least one location must be included. Geolocation must be given as the location of the radiation center of the device's antenna. If a location specifies a region, rather than a point, the database may return an UNIMPLEMENTED error if it does not support query by region.
    /// 
    /// There is no upper limit on the number of locations included in a available spectrum batch request, but the database may restrict the number of locations it supports by returning a response with fewer locations than specified in the batch request. Note that geolocations must be those of the master device (a device with geolocation capability that makes an available spectrum batch request), whether the master device is making the request on its own behalf or on behalf of a slave device (one without geolocation capability).
    pub locations: Option<Vec<GeoLocation>>,
    /// The master device may include its device capabilities to limit the available-spectrum batch response to the spectrum that is compatible with its capabilities. The database should not return spectrum that is incompatible with the specified capabilities.    
    pub capabilities: Option<DeviceCapabilities>,
    /// The PAWS version. Must be exactly 1.0.
    /// 
    /// Required field.
    pub version: Option<String>,
    /// Depending on device type and regulatory domain, device owner information may be included in an available spectrum batch request. This allows the device to register and get spectrum-availability information in a single request.    
    pub owner: Option<DeviceOwner>,
    /// The message type (e.g., INIT_REQ, AVAIL_SPECTRUM_REQ, ...).
    /// 
    /// Required field.
    #[serde(alias="type")]
    pub type_: Option<String>,
}

impl RequestValue for PawsGetSpectrumBatchRequest {}


/// This message is provided by the database to notify devices of an upcoming change to the database URI.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct DbUpdateSpec {
    /// A required list of one or more databases. A device should update its preconfigured list of databases to replace (only) the database that provided the response with the specified entries.    
    pub databases: Vec<DatabaseSpec>,
}

impl Part for DbUpdateSpec {}


/// A region is represented using the polygonal shape.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GeoLocationPolygon {
    /// When the geolocation describes a region, the exterior field refers to a list of latitude/longitude points that represent the vertices of a polygon. The first and last points must be the same. Thus, a minimum of four points is required. The following polygon restrictions from RFC5491 apply:  
    /// - A connecting line shall not cross another connecting line of the same polygon. 
    /// - The vertices must be defined in a counterclockwise order. 
    /// - The edges of a polygon are defined by the shortest path between two points in space (not a geodesic curve). Consequently, the length between two adjacent vertices should be restricted to a maximum of 130 km. 
    /// - All vertices are assumed to be at the same altitude. 
    /// - Polygon shapes should be restricted to a maximum of 15 vertices (16 points that include the repeated vertex).
    pub exterior: Vec<GeoLocationPoint>,
}

impl Part for GeoLocationPolygon {}


/// The schedule of spectrum profiles available at a particular geolocation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct GeoSpectrumSchedule {
    /// A list of available spectrum profiles and associated times. It will always be present, and at least one schedule must be included (though it may be empty if there is no available spectrum). More than one schedule may be included to represent future changes to the available spectrum.    
    #[serde(alias="spectrumSchedules")]
    pub spectrum_schedules: Vec<SpectrumSchedule>,
    /// The geolocation identifies the location at which the spectrum schedule applies. It will always be present.    
    pub location: GeoLocation,
}

impl Part for GeoSpectrumSchedule {}


/// The initialization request message allows the master device to initiate exchange of capabilities with the database.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [init paws](struct.PawInitCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct PawsInitRequest {
    /// The DeviceDescriptor parameter is required. If the database does not support the device or any of the rulesets specified in the device descriptor, it must return an UNSUPPORTED error code in the error response.    
    #[serde(alias="deviceDesc")]
    pub device_desc: Option<DeviceDescriptor>,
    /// The message type (e.g., INIT_REQ, AVAIL_SPECTRUM_REQ, ...).
    /// 
    /// Required field.
    #[serde(alias="type")]
    pub type_: Option<String>,
    /// A device's geolocation is required.    
    pub location: Option<GeoLocation>,
    /// The PAWS version. Must be exactly 1.0.
    /// 
    /// Required field.
    pub version: Option<String>,
}

impl RequestValue for PawsInitRequest {}


/// The structure used to represent a street address.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct VcardAddress {
    /// The postal code associated with the address. For example: 94423.    
    pub code: String,
    /// The street number and name. For example: 123 Any St.    
    pub street: String,
    /// The city or local equivalent portion of the address. For example: San Jose.    
    pub locality: String,
    /// The country name. For example: US.    
    pub country: String,
    /// The state or local equivalent portion of the address. For example: CA.    
    pub region: String,
    /// An optional post office box number.    
    pub pobox: String,
}

impl Part for VcardAddress {}


/// The device descriptor contains parameters that identify the specific device, such as its manufacturer serial number, regulatory-specific identifier (e.g., FCC ID), and any other device characteristics required by regulatory domains.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceDescriptor {
    /// Specifies the ETSI white space device type. Valid values are single-letter strings, such as A, B, etc. Consult the ETSI documentation for details about the device types.    
    #[serde(alias="etsiEnDeviceType")]
    pub etsi_en_device_type: String,
    /// Specifies the device's FCC certification identifier. The value is an identifier string whose length should not exceed 32 characters. Note that, in practice, a valid FCC ID may be limited to 19 characters.    
    #[serde(alias="fccId")]
    pub fcc_id: String,
    /// The manufacturer's device serial number; required by the applicable regulatory domain. The length of the value must not exceed 64 characters.    
    #[serde(alias="serialNumber")]
    pub serial_number: String,
    /// Specifies the ETSI white space device technology identifier. The string value must not exceed 64 characters in length. Consult the ETSI documentation for details about the device types.    
    #[serde(alias="etsiEnTechnologyId")]
    pub etsi_en_technology_id: String,
    /// Specifies the TV Band White Space device type, as defined by the FCC. Valid values are FIXED, MODE_1, MODE_2.    
    #[serde(alias="fccTvbdDeviceType")]
    pub fcc_tvbd_device_type: String,
    /// Specifies the ETSI white space device category. Valid values are the strings master and slave. This field is case-insensitive. Consult the ETSI documentation for details about the device types.    
    #[serde(alias="etsiEnDeviceCategory")]
    pub etsi_en_device_category: String,
    /// The list of identifiers for rulesets supported by the device. A database may require that the device provide this list before servicing the device requests. If the database does not support any of the rulesets specified in the list, the database may refuse to service the device requests. If present, the list must contain at least one entry.
    /// 
    /// For information about the valid requests, see section 9.2 of the PAWS specification. Currently, FccTvBandWhiteSpace-2010 is the only supported ruleset.
    #[serde(alias="rulesetIds")]
    pub ruleset_ids: Vec<String>,
    /// Specifies the ETSI white space device emissions class. The values are represented by numeric strings, such as 1, 2, etc. Consult the ETSI documentation for details about the device types.    
    #[serde(alias="etsiEnDeviceEmissionsClass")]
    pub etsi_en_device_emissions_class: String,
    /// The manufacturer's ID may be required by the regulatory domain. This should represent the name of the device manufacturer, should be consistent across all devices from the same manufacturer, and should be distinct from that of other manufacturers. The string value must not exceed 64 characters in length.    
    #[serde(alias="manufacturerId")]
    pub manufacturer_id: String,
    /// The device's model ID may be required by the regulatory domain. The string value must not exceed 64 characters in length.    
    #[serde(alias="modelId")]
    pub model_id: String,
}

impl Part for DeviceDescriptor {}


/// This contains parameters for the ruleset of a regulatory domain that is communicated using the initialization and available-spectrum processes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct RulesetInfo {
    /// The identifiers of the rulesets supported for the device's location. The database should include at least one applicable ruleset in the initialization response. The device may use the ruleset identifiers to determine parameters to include in subsequent requests. Within the context of the available-spectrum responses, the database should include the identifier of the ruleset that it used to determine the available-spectrum response. If included, the device must use the specified ruleset to interpret the response. If the device does not support the indicated ruleset, it must not operate in the spectrum governed by the ruleset.    
    #[serde(alias="rulesetIds")]
    pub ruleset_ids: Vec<String>,
    /// The maximum duration, in seconds, between requests for available spectrum. It is required in the initialization response, but optional otherwise. The device must contact the database to get available spectrum no less frequently than this duration. If the new spectrum information indicates that the device is using spectrum that is no longer available, it must immediately cease use of those frequencies under rules for database-managed spectrum. If this value is provided within the context of an available-spectrum response, it takes precedence over the value within the initialization response.    
    #[serde(alias="maxPollingSecs")]
    pub max_polling_secs: i32,
    /// The regulatory domain to which the ruleset belongs is required. It must be a 2-letter country code. The device should use this to determine additional device behavior required by the associated regulatory domain.    
    pub authority: String,
    /// The maximum location change in meters is required in the initialization response, but optional otherwise. When the device changes location by more than this specified distance, it must contact the database to get the available spectrum for the new location. If the device is using spectrum that is no longer available, it must immediately cease use of the spectrum under rules for database-managed spectrum. If this value is provided within the context of an available-spectrum response, it takes precedence over the value within the initialization response.    
    #[serde(alias="maxLocationChange")]
    pub max_location_change: f64,
}

impl Part for RulesetInfo {}


/// This message contains the name and URI of a database.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct DatabaseSpec {
    /// The display name for a database.    
    pub name: String,
    /// The corresponding URI of the database.    
    pub uri: String,
}

impl Part for DatabaseSpec {}


/// The registration request message contains the required registration parameters.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [register paws](struct.PawRegisterCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct PawsRegisterRequest {
    /// A DeviceDescriptor is required.    
    #[serde(alias="deviceDesc")]
    pub device_desc: Option<DeviceDescriptor>,
    /// The PAWS version. Must be exactly 1.0.
    /// 
    /// Required field.
    pub version: Option<String>,
    /// Antenna characteristics, including its height and height type.    
    pub antenna: Option<AntennaCharacteristics>,
    /// The message type (e.g., INIT_REQ, AVAIL_SPECTRUM_REQ, ...).
    /// 
    /// Required field.
    #[serde(alias="type")]
    pub type_: Option<String>,
    /// Device owner information is required.    
    #[serde(alias="deviceOwner")]
    pub device_owner: Option<DeviceOwner>,
    /// A device's geolocation is required.    
    pub location: Option<GeoLocation>,
}

impl RequestValue for PawsRegisterRequest {}


/// Available spectrum can be logically characterized by a list of frequency ranges and permissible power levels for each range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpectrumMessage {
    /// The bandwidth (in Hertz) for which permissible power levels are specified. For example, FCC regulation would require only one spectrum specification at 6MHz bandwidth, but Ofcom regulation would require two specifications, at 0.1MHz and 8MHz. This parameter may be empty if there is no available spectrum. It will be present otherwise.    
    pub bandwidth: f64,
    /// The list of frequency ranges and permissible power levels. The list may be empty if there is no available spectrum, otherwise it will be present.    
    #[serde(alias="frequencyRanges")]
    pub frequency_ranges: Vec<FrequencyRange>,
}

impl Part for SpectrumMessage {}


/// The response message for the available spectrum query which contains a schedule of available spectrum for the device.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get spectrum paws](struct.PawGetSpectrumCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct PawsGetSpectrumResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "spectrum#pawsGetSpectrumResponse".    
    pub kind: String,
    /// The database must return, in its available spectrum response, the device descriptor information it received in the master device's available spectrum request.    
    #[serde(alias="deviceDesc")]
    pub device_desc: DeviceDescriptor,
    /// A database may include the databaseChange parameter to notify a device of a change to its database URI, providing one or more alternate database URIs. The device should use this information to update its list of pre-configured databases by (only) replacing its entry for the responding database with the list of alternate URIs.    
    #[serde(alias="databaseChange")]
    pub database_change: DbUpdateSpec,
    /// The database may return a constraint on the allowed maximum total bandwidth (in Hertz), which need not be contiguous. A regulatory domain may require the database to return this parameter. When this parameter is present in the available spectrum response, the device must apply this constraint to its spectrum-selection logic to ensure that total bandwidth does not exceed this value.    
    #[serde(alias="maxTotalBwHz")]
    pub max_total_bw_hz: f64,
    /// The available spectrum response must contain a spectrum schedule list. The list may be empty if spectrum is not available. The database may return more than one spectrum schedule to represent future changes to the available spectrum. How far in advance a schedule may be provided depends on the applicable regulatory domain.    
    #[serde(alias="spectrumSchedules")]
    pub spectrum_schedules: Vec<SpectrumSchedule>,
    /// The database may return a constraint on the allowed maximum contiguous bandwidth (in Hertz). A regulatory domain may require the database to return this parameter. When this parameter is present in the response, the device must apply this constraint to its spectrum-selection logic to ensure that no single block of spectrum has bandwidth that exceeds this value.    
    #[serde(alias="maxContiguousBwHz")]
    pub max_contiguous_bw_hz: f64,
    /// The PAWS version. Must be exactly 1.0.
    /// 
    /// Required field.
    pub version: String,
    /// The database includes a timestamp of the form YYYY-MM-DDThh:mm:ssZ (Internet timestamp format per RFC3339) in its available spectrum response. The timestamp should be used by the device as a reference for the start and stop times specified in the response spectrum schedules.    
    pub timestamp: String,
    /// The database should return ruleset information, which identifies the applicable regulatory authority and ruleset for the available spectrum response. If included, the device must use the corresponding ruleset to interpret the response. Values provided in the returned ruleset information, such as maxLocationChange, take precedence over any conflicting values provided in the ruleset information returned in a prior initialization response sent by the database to the device.    
    #[serde(alias="rulesetInfo")]
    pub ruleset_info: RulesetInfo,
    /// For regulatory domains that require a spectrum-usage report from devices, the database must return true for this parameter if the spectrum schedule list is not empty; otherwise, the database will either return false or omit this parameter. If this parameter is present and its value is true, the device must send a spectrum use notify message to the database; otherwise, the device must not send the notification.    
    #[serde(alias="needsSpectrumReport")]
    pub needs_spectrum_report: bool,
    /// The message type (e.g., INIT_REQ, AVAIL_SPECTRUM_REQ, ...).
    /// 
    /// Required field.
    #[serde(alias="type")]
    pub type_: String,
}

impl ResponseResult for PawsGetSpectrumResponse {}


/// Device capabilities provide additional information that may be used by a device to provide additional information to the database that may help it to determine available spectrum. If the database does not support device capabilities it will ignore the parameter altogether.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct DeviceCapabilities {
    /// An optional list of frequency ranges supported by the device. Each element must contain start and stop frequencies in which the device can operate. Channel identifiers are optional. When specified, the database should not return available spectrum that falls outside these ranges or channel IDs.    
    #[serde(alias="frequencyRanges")]
    pub frequency_ranges: Vec<FrequencyRange>,
}

impl Part for DeviceCapabilities {}


/// The spectrum schedule element combines an event time with spectrum profile to define a time period in which the profile is valid.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct SpectrumSchedule {
    /// The event time expresses when the spectrum profile is valid. It will always be present.    
    #[serde(alias="eventTime")]
    pub event_time: EventTime,
    /// A list of spectrum messages representing the usable profile. It will always be present, but may be empty when there is no available spectrum.    
    pub spectra: Vec<SpectrumMessage>,
}

impl Part for SpectrumSchedule {}


/// The device validation request message.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [verify device paws](struct.PawVerifyDeviceCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct PawsVerifyDeviceRequest {
    /// A list of device descriptors, which specifies the slave devices to be validated, is required.    
    #[serde(alias="deviceDescs")]
    pub device_descs: Option<Vec<DeviceDescriptor>>,
    /// The message type (e.g., INIT_REQ, AVAIL_SPECTRUM_REQ, ...).
    /// 
    /// Required field.
    #[serde(alias="type")]
    pub type_: Option<String>,
    /// The PAWS version. Must be exactly 1.0.
    /// 
    /// Required field.
    pub version: Option<String>,
}

impl RequestValue for PawsVerifyDeviceRequest {}


/// The request message for the available spectrum query protocol which must include the device's geolocation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get spectrum paws](struct.PawGetSpectrumCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct PawsGetSpectrumRequest {
    /// When the available spectrum request is made on behalf of a specific device (a master or slave device), device descriptor information for that device is required (in such cases, the requestType parameter must be empty). When a requestType value is specified, device descriptor information may be optional or required according to the rules of the applicable regulatory domain.    
    #[serde(alias="deviceDesc")]
    pub device_desc: Option<DeviceDescriptor>,
    /// The PAWS version. Must be exactly 1.0.
    /// 
    /// Required field.
    pub version: Option<String>,
    /// Depending on device type and regulatory domain, the characteristics of the antenna may be required.    
    pub antenna: Option<AntennaCharacteristics>,
    /// The request type parameter is an optional parameter that can be used to modify an available spectrum request, but its use depends on applicable regulatory rules. It may be used, for example, to request generic slave device parameters without having to specify the device descriptor for a specific device. When the requestType parameter is missing, the request is for a specific device (master or slave), and the deviceDesc parameter for the device on whose behalf the request is made is required.    
    #[serde(alias="requestType")]
    pub request_type: Option<String>,
    /// The master device may include its device capabilities to limit the available-spectrum response to the spectrum that is compatible with its capabilities. The database should not return spectrum that is incompatible with the specified capabilities.    
    pub capabilities: Option<DeviceCapabilities>,
    /// When an available spectrum request is made by the master device (a device with geolocation capability) on behalf of a slave device (a device without geolocation capability), the rules of the applicable regulatory domain may require the master device to provide its own device descriptor information (in addition to device descriptor information for the slave device, which is provided in a separate parameter).    
    #[serde(alias="masterDeviceDesc")]
    pub master_device_desc: Option<DeviceDescriptor>,
    /// The geolocation of the master device (a device with geolocation capability that makes an available spectrum request) is required whether the master device is making the request on its own behalf or on behalf of a slave device (one without geolocation capability). The location must be the location of the radiation center of the master device's antenna. To support mobile devices, a regulatory domain may allow the anticipated position of the master device to be given instead. If the location specifies a region, rather than a point, the database may return an UNIMPLEMENTED error code if it does not support query by region.    
    pub location: Option<GeoLocation>,
    /// Depending on device type and regulatory domain, device owner information may be included in an available spectrum request. This allows the device to register and get spectrum-availability information in a single request.    
    pub owner: Option<DeviceOwner>,
    /// The message type (e.g., INIT_REQ, AVAIL_SPECTRUM_REQ, ...).
    /// 
    /// Required field.
    #[serde(alias="type")]
    pub type_: Option<String>,
}

impl RequestValue for PawsGetSpectrumRequest {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *paw* resources.
/// It is not used directly, but through the `Spectrum` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-spectrum1_explorer" as spectrum1_explorer;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use spectrum1_explorer::Spectrum;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Spectrum::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_spectrum(...)`, `get_spectrum_batch(...)`, `init(...)`, `notify_spectrum_use(...)`, `register(...)` and `verify_device(...)`
/// // to build up your call.
/// let rb = hub.paws();
/// # }
/// ```
pub struct PawMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Spectrum<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for PawMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> PawMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Notifies the database that the device has selected certain frequency ranges for transmission. Only to be invoked when required by the regulator. The Google Spectrum Database does not operate in domains that require notification, so this always yields an UNIMPLEMENTED error.    
    pub fn notify_spectrum_use(&self, request: &PawsNotifySpectrumUseRequest) -> PawNotifySpectrumUseCall<'a, C, NC, A> {
        PawNotifySpectrumUseCall {
            hub: self.hub,
            _request: request.clone(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// The Google Spectrum Database implements registration in the getSpectrum method. As such this always returns an UNIMPLEMENTED error.    
    pub fn register(&self, request: &PawsRegisterRequest) -> PawRegisterCall<'a, C, NC, A> {
        PawRegisterCall {
            hub: self.hub,
            _request: request.clone(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Requests information about the available spectrum for a device at a location. Requests from a fixed-mode device must include owner information so the device can be registered with the database.    
    pub fn get_spectrum(&self, request: &PawsGetSpectrumRequest) -> PawGetSpectrumCall<'a, C, NC, A> {
        PawGetSpectrumCall {
            hub: self.hub,
            _request: request.clone(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Initializes the connection between a white space device and the database.    
    pub fn init(&self, request: &PawsInitRequest) -> PawInitCall<'a, C, NC, A> {
        PawInitCall {
            hub: self.hub,
            _request: request.clone(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// The Google Spectrum Database does not support batch requests, so this method always yields an UNIMPLEMENTED error.    
    pub fn get_spectrum_batch(&self, request: &PawsGetSpectrumBatchRequest) -> PawGetSpectrumBatchCall<'a, C, NC, A> {
        PawGetSpectrumBatchCall {
            hub: self.hub,
            _request: request.clone(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Validates a device for white space use in accordance with regulatory rules. The Google Spectrum Database does not support master/slave configurations, so this always yields an UNIMPLEMENTED error.    
    pub fn verify_device(&self, request: &PawsVerifyDeviceRequest) -> PawVerifyDeviceCall<'a, C, NC, A> {
        PawVerifyDeviceCall {
            hub: self.hub,
            _request: request.clone(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Notifies the database that the device has selected certain frequency ranges for transmission. Only to be invoked when required by the regulator. The Google Spectrum Database does not operate in domains that require notification, so this always yields an UNIMPLEMENTED error.
///
/// A builder for the *notifySpectrumUse* method supported by a *paw* resource.
/// It is not used directly, but through a `PawMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-spectrum1_explorer" as spectrum1_explorer;
/// use spectrum1_explorer::PawsNotifySpectrumUseRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use spectrum1_explorer::Spectrum;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Spectrum::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: PawsNotifySpectrumUseRequest = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.paws().notify_spectrum_use(&req)
///              .doit();
/// # }
/// ```
pub struct PawNotifySpectrumUseCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Spectrum<C, NC, A>,
    _request: PawsNotifySpectrumUseRequest,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, NC, A> CallBuilder for PawNotifySpectrumUseCall<'a, C, NC, A> {}

impl<'a, C, NC, A> PawNotifySpectrumUseCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, PawsNotifySpectrumUseResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "spectrum.paws.notifySpectrumUse", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/spectrum/v1explorer/paws/notifySpectrumUse".to_string();
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Result::MissingAPIKey
            }
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
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
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &PawsNotifySpectrumUseRequest) -> PawNotifySpectrumUseCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PawNotifySpectrumUseCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PawNotifySpectrumUseCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

}


/// The Google Spectrum Database implements registration in the getSpectrum method. As such this always returns an UNIMPLEMENTED error.
///
/// A builder for the *register* method supported by a *paw* resource.
/// It is not used directly, but through a `PawMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-spectrum1_explorer" as spectrum1_explorer;
/// use spectrum1_explorer::PawsRegisterRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use spectrum1_explorer::Spectrum;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Spectrum::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: PawsRegisterRequest = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.paws().register(&req)
///              .doit();
/// # }
/// ```
pub struct PawRegisterCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Spectrum<C, NC, A>,
    _request: PawsRegisterRequest,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, NC, A> CallBuilder for PawRegisterCall<'a, C, NC, A> {}

impl<'a, C, NC, A> PawRegisterCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, PawsRegisterResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "spectrum.paws.register", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/spectrum/v1explorer/paws/register".to_string();
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Result::MissingAPIKey
            }
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
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
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &PawsRegisterRequest) -> PawRegisterCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PawRegisterCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PawRegisterCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

}


/// Requests information about the available spectrum for a device at a location. Requests from a fixed-mode device must include owner information so the device can be registered with the database.
///
/// A builder for the *getSpectrum* method supported by a *paw* resource.
/// It is not used directly, but through a `PawMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-spectrum1_explorer" as spectrum1_explorer;
/// use spectrum1_explorer::PawsGetSpectrumRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use spectrum1_explorer::Spectrum;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Spectrum::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: PawsGetSpectrumRequest = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.paws().get_spectrum(&req)
///              .doit();
/// # }
/// ```
pub struct PawGetSpectrumCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Spectrum<C, NC, A>,
    _request: PawsGetSpectrumRequest,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, NC, A> CallBuilder for PawGetSpectrumCall<'a, C, NC, A> {}

impl<'a, C, NC, A> PawGetSpectrumCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, PawsGetSpectrumResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "spectrum.paws.getSpectrum", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/spectrum/v1explorer/paws/getSpectrum".to_string();
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Result::MissingAPIKey
            }
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
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
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &PawsGetSpectrumRequest) -> PawGetSpectrumCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PawGetSpectrumCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PawGetSpectrumCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

}


/// Initializes the connection between a white space device and the database.
///
/// A builder for the *init* method supported by a *paw* resource.
/// It is not used directly, but through a `PawMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-spectrum1_explorer" as spectrum1_explorer;
/// use spectrum1_explorer::PawsInitRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use spectrum1_explorer::Spectrum;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Spectrum::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: PawsInitRequest = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.paws().init(&req)
///              .doit();
/// # }
/// ```
pub struct PawInitCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Spectrum<C, NC, A>,
    _request: PawsInitRequest,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, NC, A> CallBuilder for PawInitCall<'a, C, NC, A> {}

impl<'a, C, NC, A> PawInitCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, PawsInitResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "spectrum.paws.init", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/spectrum/v1explorer/paws/init".to_string();
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Result::MissingAPIKey
            }
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
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
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &PawsInitRequest) -> PawInitCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PawInitCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PawInitCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

}


/// The Google Spectrum Database does not support batch requests, so this method always yields an UNIMPLEMENTED error.
///
/// A builder for the *getSpectrumBatch* method supported by a *paw* resource.
/// It is not used directly, but through a `PawMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-spectrum1_explorer" as spectrum1_explorer;
/// use spectrum1_explorer::PawsGetSpectrumBatchRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use spectrum1_explorer::Spectrum;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Spectrum::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: PawsGetSpectrumBatchRequest = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.paws().get_spectrum_batch(&req)
///              .doit();
/// # }
/// ```
pub struct PawGetSpectrumBatchCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Spectrum<C, NC, A>,
    _request: PawsGetSpectrumBatchRequest,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, NC, A> CallBuilder for PawGetSpectrumBatchCall<'a, C, NC, A> {}

impl<'a, C, NC, A> PawGetSpectrumBatchCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, PawsGetSpectrumBatchResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "spectrum.paws.getSpectrumBatch", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/spectrum/v1explorer/paws/getSpectrumBatch".to_string();
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Result::MissingAPIKey
            }
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
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
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &PawsGetSpectrumBatchRequest) -> PawGetSpectrumBatchCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PawGetSpectrumBatchCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PawGetSpectrumBatchCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

}


/// Validates a device for white space use in accordance with regulatory rules. The Google Spectrum Database does not support master/slave configurations, so this always yields an UNIMPLEMENTED error.
///
/// A builder for the *verifyDevice* method supported by a *paw* resource.
/// It is not used directly, but through a `PawMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-spectrum1_explorer" as spectrum1_explorer;
/// use spectrum1_explorer::PawsVerifyDeviceRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use spectrum1_explorer::Spectrum;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Spectrum::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: PawsVerifyDeviceRequest = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.paws().verify_device(&req)
///              .doit();
/// # }
/// ```
pub struct PawVerifyDeviceCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Spectrum<C, NC, A>,
    _request: PawsVerifyDeviceRequest,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, NC, A> CallBuilder for PawVerifyDeviceCall<'a, C, NC, A> {}

impl<'a, C, NC, A> PawVerifyDeviceCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, PawsVerifyDeviceResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "spectrum.paws.verifyDevice", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/spectrum/v1explorer/paws/verifyDevice".to_string();
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Result::MissingAPIKey
            }
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
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
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &PawsVerifyDeviceRequest) -> PawVerifyDeviceCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PawVerifyDeviceCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PawVerifyDeviceCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

}


