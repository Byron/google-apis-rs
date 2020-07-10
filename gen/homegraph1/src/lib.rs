// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Home Graph Service* crate version *1.0.14+20200703*, where *20200703* is the exact revision of the *homegraph:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.14*.
//! 
//! Everything else about the *Home Graph Service* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/actions/smarthome/create-app#request-sync).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/homegraph1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.HomeGraphService.html) ... 
//! 
//! * agent users
//!  * [*delete*](struct.AgentUserDeleteCall.html)
//! * [devices](struct.Device.html)
//!  * [*query*](struct.DeviceQueryCall.html), [*report state and notification*](struct.DeviceReportStateAndNotificationCall.html), [*request sync*](struct.DeviceRequestSyncCall.html) and [*sync*](struct.DeviceSyncCall.html)
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
//! * **[Hub](struct.HomeGraphService.html)**
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
//! let r = hub.devices().query(...).doit()
//! let r = hub.devices().request_sync(...).doit()
//! let r = hub.devices().sync(...).doit()
//! let r = hub.devices().report_state_and_notification(...).doit()
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
//! google-homegraph1 = "*"
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
//! extern crate google_homegraph1 as homegraph1;
//! use homegraph1::QueryRequest;
//! use homegraph1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use homegraph1::HomeGraphService;
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
//! let mut hub = HomeGraphService::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = QueryRequest::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.devices().query(req)
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

/// Central instance to access all HomeGraphService related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_homegraph1 as homegraph1;
/// use homegraph1::QueryRequest;
/// use homegraph1::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use homegraph1::HomeGraphService;
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
/// let mut hub = HomeGraphService::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = QueryRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.devices().query(req)
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
pub struct HomeGraphService<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for HomeGraphService<C, A> {}

impl<'a, C, A> HomeGraphService<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> HomeGraphService<C, A> {
        HomeGraphService {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.14".to_string(),
            _base_url: "https://homegraph.googleapis.com/".to_string(),
            _root_url: "https://homegraph.googleapis.com/".to_string(),
        }
    }

    pub fn agent_users(&'a self) -> AgentUserMethods<'a, C, A> {
        AgentUserMethods { hub: &self }
    }
    pub fn devices(&'a self) -> DeviceMethods<'a, C, A> {
        DeviceMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.14`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://homegraph.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://homegraph.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// Payload containing device information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SyncResponsePayload {
    /// Third-party user ID
    #[serde(rename="agentUserId")]
    pub agent_user_id: Option<String>,
    /// Devices associated with the third-party user.
    pub devices: Option<Vec<Device>>,
}

impl Part for SyncResponsePayload {}


/// Request type for the
/// [`Query`](#google.home.graph.v1.HomeGraphApiService.Query) call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [query devices](struct.DeviceQueryCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryRequest {
    /// Required. Inputs containing third-party device IDs for which to
    /// get the device states.
    pub inputs: Option<Vec<QueryRequestInput>>,
    /// Required. Third-party user ID.
    #[serde(rename="agentUserId")]
    pub agent_user_id: Option<String>,
    /// Request ID used for debugging.
    #[serde(rename="requestId")]
    pub request_id: Option<String>,
}

impl RequestValue for QueryRequest {}


/// Device information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// Device model.
    pub model: Option<String>,
    /// Device software version.
    #[serde(rename="swVersion")]
    pub sw_version: Option<String>,
    /// Device hardware version.
    #[serde(rename="hwVersion")]
    pub hw_version: Option<String>,
    /// Device manufacturer.
    pub manufacturer: Option<String>,
}

impl Part for DeviceInfo {}


/// Response type for the
/// [`Sync`](#google.home.graph.v1.HomeGraphApiService.Sync) call.
/// This should follow the same format as the Google smart home
/// `action.devices.SYNC`
/// [response](https://developers.google.com/assistant/smarthome/reference/intent/sync).
/// 
/// # Example
/// 
/// ````textjson
/// {
///   "requestId": "ff36a3cc-ec34-11e6-b1a0-64510650abcf",
///   "payload": {
///     "agentUserId": "1836.15267389",
///     "devices": [{
///       "id": "123",
///       "type": "action.devices.types.OUTLET",
///       "traits": [
///         "action.devices.traits.OnOff"
///       ],
///       "name": {
///         "defaultNames": ["My Outlet 1234"],
///         "name": "Night light",
///         "nicknames": ["wall plug"]
///       },
///       "willReportState": false,
///       "deviceInfo": {
///         "manufacturer": "lights-out-inc",
///         "model": "hs1234",
///         "hwVersion": "3.2",
///         "swVersion": "11.4"
///       },
///       "customData": {
///         "fooValue": 74,
///         "barValue": true,
///         "bazValue": "foo"
///       }
///     }]
///   }
/// }
/// ````
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sync devices](struct.DeviceSyncCall.html) (response)
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SyncResponse {
    /// Devices associated with the third-party user.
    pub payload: Option<SyncResponsePayload>,
    /// Request ID used for debugging. Copied from the request.
    #[serde(rename="requestId")]
    pub request_id: Option<String>,
}

impl ResponseResult for SyncResponse {}


/// Payload containing the state and notification information for devices.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StateAndNotificationPayload {
    /// The devices for updating state and sending notifications.
    pub devices: Option<ReportStateAndNotificationDevice>,
}

impl Part for StateAndNotificationPayload {}


/// Request type for the
/// [`ReportStateAndNotification`](#google.home.graph.v1.HomeGraphApiService.ReportStateAndNotification)
/// call. It may include states, notifications, or both. States and notifications
/// are defined per `device_id` (for example, "123" and "456" in the following
/// example).
/// 
/// # Example
/// 
/// ````textjson
/// {
///   "requestId": "ff36a3cc-ec34-11e6-b1a0-64510650abcf",
///   "agentUserId": "1234",
///   "payload": {
///     "devices": {
///       "states": {
///         "123": {
///           "on": true
///         },
///         "456": {
///           "on": true,
///           "brightness": 10
///         }
///       },
///     }
///   }
/// }
/// ````
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [report state and notification devices](struct.DeviceReportStateAndNotificationCall.html) (request)
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportStateAndNotificationRequest {
    /// Unique identifier per event (for example, a doorbell press).
    #[serde(rename="eventId")]
    pub event_id: Option<String>,
    /// Required. Third-party user ID.
    #[serde(rename="agentUserId")]
    pub agent_user_id: Option<String>,
    /// Token to maintain state in the follow up notification response.
    /// Deprecated. See the [notifications
    /// guide](https://developers.google.com/assistant/smarthome/develop/notifications)
    /// for details on implementing follow up notifications.
    #[serde(rename="followUpToken")]
    pub follow_up_token: Option<String>,
    /// Request ID used for debugging.
    #[serde(rename="requestId")]
    pub request_id: Option<String>,
    /// Required. State of devices to update and notification metadata for devices.
    pub payload: Option<StateAndNotificationPayload>,
}

impl RequestValue for ReportStateAndNotificationRequest {}


/// Response type for the
/// [`RequestSyncDevices`](#google.home.graph.v1.HomeGraphApiService.RequestSyncDevices)
/// call.
/// 
/// Intentionally empty upon success. An HTTP response code is returned
/// with more details upon failure.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [request sync devices](struct.DeviceRequestSyncCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RequestSyncDevicesResponse { _never_set: Option<bool> }

impl ResponseResult for RequestSyncDevicesResponse {}


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
/// * [delete agent users](struct.AgentUserDeleteCall.html) (response)
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl ResponseResult for Empty {}


/// The states and notifications specific to a device.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportStateAndNotificationDevice {
    /// States of devices to update. See the **Device STATES** section
    /// of the individual trait [reference
    /// guides](https://developers.google.com/assistant/smarthome/traits).
    pub states: Option<HashMap<String, String>>,
    /// Notifications metadata for devices. See the **Device NOTIFICATIONS**
    /// section of the individual trait [reference
    /// guides](https://developers.google.com/assistant/smarthome/traits).
    pub notifications: Option<HashMap<String, String>>,
}

impl Part for ReportStateAndNotificationDevice {}


/// Device ID inputs to QueryRequest.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryRequestInput {
    /// Payload containing third-party device IDs.
    pub payload: Option<QueryRequestPayload>,
}

impl Part for QueryRequestInput {}


/// Payload containing device states information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryResponsePayload {
    /// States of the devices. Map of third-party device ID to struct of device
    /// states.
    pub devices: Option<HashMap<String, HashMap<String, String>>>,
}

impl Part for QueryResponsePayload {}


/// Third-party device ID for one device.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AgentDeviceId {
    /// Third-party device ID.
    pub id: Option<String>,
}

impl Part for AgentDeviceId {}


/// Payload containing device IDs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryRequestPayload {
    /// Third-party device IDs for which to get the device states.
    pub devices: Option<Vec<AgentDeviceId>>,
}

impl Part for QueryRequestPayload {}


/// Request type for the
/// [`RequestSyncDevices`](#google.home.graph.v1.HomeGraphApiService.RequestSyncDevices)
/// call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [request sync devices](struct.DeviceRequestSyncCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RequestSyncDevicesRequest {
    /// Optional. If set, the request will be added to a queue and a response will
    /// be returned immediately. This enables concurrent requests for the given
    /// `agent_user_id`, but the caller will not receive any error responses.
    pub async: Option<bool>,
    /// Required. Third-party user ID.
    #[serde(rename="agentUserId")]
    pub agent_user_id: Option<String>,
}

impl RequestValue for RequestSyncDevicesRequest {}


/// Alternate third-party device ID.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AgentOtherDeviceId {
    /// Project ID for your smart home Action.
    #[serde(rename="agentId")]
    pub agent_id: Option<String>,
    /// Unique third-party device ID.
    #[serde(rename="deviceId")]
    pub device_id: Option<String>,
}

impl Part for AgentOtherDeviceId {}


/// Response type for the
/// [`ReportStateAndNotification`](#google.home.graph.v1.HomeGraphApiService.ReportStateAndNotification)
/// call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [report state and notification devices](struct.DeviceReportStateAndNotificationCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportStateAndNotificationResponse {
    /// Request ID copied from ReportStateAndNotificationRequest.
    #[serde(rename="requestId")]
    pub request_id: Option<String>,
}

impl ResponseResult for ReportStateAndNotificationResponse {}


/// Request type for the [`Sync`](#google.home.graph.v1.HomeGraphApiService.Sync)
/// call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sync devices](struct.DeviceSyncCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SyncRequest {
    /// Required. Third-party user ID.
    #[serde(rename="agentUserId")]
    pub agent_user_id: Option<String>,
    /// Request ID used for debugging.
    #[serde(rename="requestId")]
    pub request_id: Option<String>,
}

impl RequestValue for SyncRequest {}


/// Third-party device definition.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [query devices](struct.DeviceQueryCall.html) (none)
/// * [request sync devices](struct.DeviceRequestSyncCall.html) (none)
/// * [sync devices](struct.DeviceSyncCall.html) (none)
/// * [report state and notification devices](struct.DeviceReportStateAndNotificationCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Device {
    /// Alternate IDs associated with this device.
    /// This is used to identify cloud synced devices enabled for [local
    /// fulfillment](https://developers.google.com/assistant/smarthome/concepts/local).
    #[serde(rename="otherDeviceIds")]
    pub other_device_ids: Option<Vec<AgentOtherDeviceId>>,
    /// Names given to this device by your smart home Action.
    pub name: Option<DeviceNames>,
    /// Custom device attributes stored in Home Graph and provided to your
    /// smart home Action in each
    /// [QUERY](https://developers.google.com/assistant/smarthome/reference/intent/query)
    /// and
    /// [EXECUTE](https://developers.google.com/assistant/smarthome/reference/intent/execute)
    /// intent.
    #[serde(rename="customData")]
    pub custom_data: Option<HashMap<String, String>>,
    /// Device manufacturer, model, hardware version, and software version.
    #[serde(rename="deviceInfo")]
    pub device_info: Option<DeviceInfo>,
    /// Indicates whether your smart home Action will report notifications
    /// to Google for this device via
    /// ReportStateAndNotification.
    /// 
    /// If your smart home Action enables users to control device notifications,
    /// you should update this field and call
    /// RequestSyncDevices.
    #[serde(rename="notificationSupportedByAgent")]
    pub notification_supported_by_agent: Option<bool>,
    /// Indicates whether your smart home Action will report state of this device
    /// to Google via
    /// ReportStateAndNotification.
    #[serde(rename="willReportState")]
    pub will_report_state: Option<bool>,
    /// Traits supported by the device.
    /// See [device
    /// traits](https://developers.google.com/assistant/smarthome/traits).
    pub traits: Option<Vec<String>>,
    /// Suggested name for the structure where this device is installed.
    /// Google attempts to use this value during user setup.
    #[serde(rename="structureHint")]
    pub structure_hint: Option<String>,
    /// Attributes for the traits supported by the device.
    pub attributes: Option<HashMap<String, String>>,
    /// Hardware type of the device.
    /// See [device
    /// types](https://developers.google.com/assistant/smarthome/guides).
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// Third-party device ID.
    pub id: Option<String>,
    /// Suggested name for the room where this device is installed.
    /// Google attempts to use this value during user setup.
    #[serde(rename="roomHint")]
    pub room_hint: Option<String>,
}

impl Resource for Device {}


/// Identifiers used to describe the device.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceNames {
    /// List of names provided by the manufacturer rather than the user, such as
    /// serial numbers, SKUs, etc.
    #[serde(rename="defaultNames")]
    pub default_names: Option<Vec<String>>,
    /// Primary name of the device, generally provided by the user.
    pub name: Option<String>,
    /// Additional names provided by the user for the device.
    pub nicknames: Option<Vec<String>>,
}

impl Part for DeviceNames {}


/// Response type for the
/// [`Query`](#google.home.graph.v1.HomeGraphApiService.Query) call.
/// This should follow the same format as the Google smart home
/// `action.devices.QUERY`
/// [response](https://developers.google.com/assistant/smarthome/reference/intent/query).
/// 
/// # Example
/// 
/// ````textjson
/// {
///   "requestId": "ff36a3cc-ec34-11e6-b1a0-64510650abcf",
///   "payload": {
///     "devices": {
///       "123": {
///         "on": true,
///         "online": true
///       },
///       "456": {
///         "on": true,
///         "online": true,
///         "brightness": 80,
///         "color": {
///           "name": "cerulean",
///           "spectrumRGB": 31655
///         }
///       }
///     }
///   }
/// }
/// ````
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [query devices](struct.DeviceQueryCall.html) (response)
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryResponse {
    /// Device states for the devices given in the request.
    pub payload: Option<QueryResponsePayload>,
    /// Request ID used for debugging. Copied from the request.
    #[serde(rename="requestId")]
    pub request_id: Option<String>,
}

impl ResponseResult for QueryResponse {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *device* resources.
/// It is not used directly, but through the `HomeGraphService` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_homegraph1 as homegraph1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use homegraph1::HomeGraphService;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = HomeGraphService::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `query(...)`, `report_state_and_notification(...)`, `request_sync(...)` and `sync(...)`
/// // to build up your call.
/// let rb = hub.devices();
/// # }
/// ```
pub struct DeviceMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a HomeGraphService<C, A>,
}

impl<'a, C, A> MethodsBuilder for DeviceMethods<'a, C, A> {}

impl<'a, C, A> DeviceMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the current states in Home Graph for the given set of the third-party
    /// user's devices.
    /// 
    /// The third-party user's identity is passed in via the `agent_user_id`
    /// (see QueryRequest).
    /// This request must be authorized using service account credentials from your
    /// Actions console project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn query(&self, request: QueryRequest) -> DeviceQueryCall<'a, C, A> {
        DeviceQueryCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Requests Google to send an `action.devices.SYNC`
    /// [intent](https://developers.google.com/assistant/smarthome/reference/intent/sync)
    /// to your smart home Action to update device metadata for the given user.
    /// 
    /// 
    /// The third-party user's identity is passed via the `agent_user_id`
    /// (see RequestSyncDevicesRequest).
    /// This request must be authorized using service account credentials from your
    /// Actions console project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn request_sync(&self, request: RequestSyncDevicesRequest) -> DeviceRequestSyncCall<'a, C, A> {
        DeviceRequestSyncCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets all the devices associated with the given third-party user.
    /// 
    /// The third-party user's identity is passed in via the `agent_user_id`
    /// (see SyncRequest).
    /// This request must be authorized using service account credentials from your
    /// Actions console project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn sync(&self, request: SyncRequest) -> DeviceSyncCall<'a, C, A> {
        DeviceSyncCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Reports device state and optionally sends device notifications.
    /// Called by your smart home Action when the state of a third-party device
    /// changes or you need to send a notification about the device.
    /// See [Implement Report
    /// State](https://developers.google.com/assistant/smarthome/develop/report-state)
    /// for more information.
    /// 
    /// This method updates the device state according to its declared
    /// [traits](https://developers.google.com/assistant/smarthome/concepts/devices-traits).
    /// Publishing a new state value outside of these traits will result in an
    /// `INVALID_ARGUMENT` error response.
    /// 
    /// The third-party user's identity is passed in via the `agent_user_id`
    /// (see ReportStateAndNotificationRequest).
    /// This request must be authorized using service account credentials from your
    /// Actions console project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn report_state_and_notification(&self, request: ReportStateAndNotificationRequest) -> DeviceReportStateAndNotificationCall<'a, C, A> {
        DeviceReportStateAndNotificationCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *agentUser* resources.
/// It is not used directly, but through the `HomeGraphService` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_homegraph1 as homegraph1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use homegraph1::HomeGraphService;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = HomeGraphService::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`
/// // to build up your call.
/// let rb = hub.agent_users();
/// # }
/// ```
pub struct AgentUserMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a HomeGraphService<C, A>,
}

impl<'a, C, A> MethodsBuilder for AgentUserMethods<'a, C, A> {}

impl<'a, C, A> AgentUserMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Unlinks the given third-party user from your smart home Action.
    /// All data related to this user will be deleted.
    /// 
    /// For more details on how users link their accounts, see
    /// [fulfillment and
    /// authentication](https://developers.google.com/assistant/smarthome/concepts/fulfillment-authentication).
    /// 
    /// The third-party user's identity is passed in via the `agent_user_id`
    /// (see DeleteAgentUserRequest).
    /// This request must be authorized using service account credentials from your
    /// Actions console project.
    /// 
    /// # Arguments
    ///
    /// * `agentUserId` - Required. Third-party user ID.
    pub fn delete(&self, agent_user_id: &str) -> AgentUserDeleteCall<'a, C, A> {
        AgentUserDeleteCall {
            hub: self.hub,
            _agent_user_id: agent_user_id.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Gets the current states in Home Graph for the given set of the third-party
/// user's devices.
/// 
/// The third-party user's identity is passed in via the `agent_user_id`
/// (see QueryRequest).
/// This request must be authorized using service account credentials from your
/// Actions console project.
///
/// A builder for the *query* method supported by a *device* resource.
/// It is not used directly, but through a `DeviceMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_homegraph1 as homegraph1;
/// use homegraph1::QueryRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use homegraph1::HomeGraphService;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = HomeGraphService::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = QueryRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.devices().query(req)
///              .doit();
/// # }
/// ```
pub struct DeviceQueryCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a HomeGraphService<C, A>,
    _request: QueryRequest,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for DeviceQueryCall<'a, C, A> {}

impl<'a, C, A> DeviceQueryCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, QueryResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "homegraph.devices.query",
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

        let mut url = self.hub._base_url.clone() + "v1/devices:query";
        
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
    pub fn request(mut self, new_value: QueryRequest) -> DeviceQueryCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> DeviceQueryCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DeviceQueryCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Requests Google to send an `action.devices.SYNC`
/// [intent](https://developers.google.com/assistant/smarthome/reference/intent/sync)
/// to your smart home Action to update device metadata for the given user.
/// 
/// 
/// The third-party user's identity is passed via the `agent_user_id`
/// (see RequestSyncDevicesRequest).
/// This request must be authorized using service account credentials from your
/// Actions console project.
///
/// A builder for the *requestSync* method supported by a *device* resource.
/// It is not used directly, but through a `DeviceMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_homegraph1 as homegraph1;
/// use homegraph1::RequestSyncDevicesRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use homegraph1::HomeGraphService;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = HomeGraphService::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = RequestSyncDevicesRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.devices().request_sync(req)
///              .doit();
/// # }
/// ```
pub struct DeviceRequestSyncCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a HomeGraphService<C, A>,
    _request: RequestSyncDevicesRequest,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for DeviceRequestSyncCall<'a, C, A> {}

impl<'a, C, A> DeviceRequestSyncCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, RequestSyncDevicesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "homegraph.devices.requestSync",
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

        let mut url = self.hub._base_url.clone() + "v1/devices:requestSync";
        
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
    pub fn request(mut self, new_value: RequestSyncDevicesRequest) -> DeviceRequestSyncCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> DeviceRequestSyncCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DeviceRequestSyncCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Gets all the devices associated with the given third-party user.
/// 
/// The third-party user's identity is passed in via the `agent_user_id`
/// (see SyncRequest).
/// This request must be authorized using service account credentials from your
/// Actions console project.
///
/// A builder for the *sync* method supported by a *device* resource.
/// It is not used directly, but through a `DeviceMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_homegraph1 as homegraph1;
/// use homegraph1::SyncRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use homegraph1::HomeGraphService;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = HomeGraphService::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = SyncRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.devices().sync(req)
///              .doit();
/// # }
/// ```
pub struct DeviceSyncCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a HomeGraphService<C, A>,
    _request: SyncRequest,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for DeviceSyncCall<'a, C, A> {}

impl<'a, C, A> DeviceSyncCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, SyncResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "homegraph.devices.sync",
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

        let mut url = self.hub._base_url.clone() + "v1/devices:sync";
        
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
    pub fn request(mut self, new_value: SyncRequest) -> DeviceSyncCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> DeviceSyncCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DeviceSyncCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Reports device state and optionally sends device notifications.
/// Called by your smart home Action when the state of a third-party device
/// changes or you need to send a notification about the device.
/// See [Implement Report
/// State](https://developers.google.com/assistant/smarthome/develop/report-state)
/// for more information.
/// 
/// This method updates the device state according to its declared
/// [traits](https://developers.google.com/assistant/smarthome/concepts/devices-traits).
/// Publishing a new state value outside of these traits will result in an
/// `INVALID_ARGUMENT` error response.
/// 
/// The third-party user's identity is passed in via the `agent_user_id`
/// (see ReportStateAndNotificationRequest).
/// This request must be authorized using service account credentials from your
/// Actions console project.
///
/// A builder for the *reportStateAndNotification* method supported by a *device* resource.
/// It is not used directly, but through a `DeviceMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_homegraph1 as homegraph1;
/// use homegraph1::ReportStateAndNotificationRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use homegraph1::HomeGraphService;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = HomeGraphService::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ReportStateAndNotificationRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.devices().report_state_and_notification(req)
///              .doit();
/// # }
/// ```
pub struct DeviceReportStateAndNotificationCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a HomeGraphService<C, A>,
    _request: ReportStateAndNotificationRequest,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for DeviceReportStateAndNotificationCall<'a, C, A> {}

impl<'a, C, A> DeviceReportStateAndNotificationCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ReportStateAndNotificationResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "homegraph.devices.reportStateAndNotification",
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

        let mut url = self.hub._base_url.clone() + "v1/devices:reportStateAndNotification";
        
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
    pub fn request(mut self, new_value: ReportStateAndNotificationRequest) -> DeviceReportStateAndNotificationCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> DeviceReportStateAndNotificationCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DeviceReportStateAndNotificationCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Unlinks the given third-party user from your smart home Action.
/// All data related to this user will be deleted.
/// 
/// For more details on how users link their accounts, see
/// [fulfillment and
/// authentication](https://developers.google.com/assistant/smarthome/concepts/fulfillment-authentication).
/// 
/// The third-party user's identity is passed in via the `agent_user_id`
/// (see DeleteAgentUserRequest).
/// This request must be authorized using service account credentials from your
/// Actions console project.
///
/// A builder for the *delete* method supported by a *agentUser* resource.
/// It is not used directly, but through a `AgentUserMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_homegraph1 as homegraph1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use homegraph1::HomeGraphService;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = HomeGraphService::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.agent_users().delete("agentUserId")
///              .request_id("sit")
///              .doit();
/// # }
/// ```
pub struct AgentUserDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a HomeGraphService<C, A>,
    _agent_user_id: String,
    _request_id: Option<String>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for AgentUserDeleteCall<'a, C, A> {}

impl<'a, C, A> AgentUserDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


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
        dlg.begin(MethodInfo { id: "homegraph.agentUsers.delete",
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("agentUserId", self._agent_user_id.to_string()));
        if let Some(value) = self._request_id {
            params.push(("requestId", value.to_string()));
        }
        for &field in ["alt", "agentUserId", "requestId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+agentUserId}";
        
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

        for &(find_this, param_name) in [("{+agentUserId}", "agentUserId")].iter() {
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
            for param_name in ["agentUserId"].iter() {
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
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Delete, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()));

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


    /// Required. Third-party user ID.
    ///
    /// Sets the *agent user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn agent_user_id(mut self, new_value: &str) -> AgentUserDeleteCall<'a, C, A> {
        self._agent_user_id = new_value.to_string();
        self
    }
    /// Request ID used for debugging.
    ///
    /// Sets the *request id* query property to the given value.
    pub fn request_id(mut self, new_value: &str) -> AgentUserDeleteCall<'a, C, A> {
        self._request_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> AgentUserDeleteCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AgentUserDeleteCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


