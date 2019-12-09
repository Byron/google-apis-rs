// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *testing* crate version *1.0.12+20190627*, where *20190627* is the exact revision of the *testing:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.12*.
//! 
//! Everything else about the *testing* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/cloud-test-lab/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/testing1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Testing.html) ... 
//! 
//! * application detail service
//!  * [*get apk details*](struct.ApplicationDetailServiceGetApkDetailCall.html)
//! * projects
//!  * [*test matrices cancel*](struct.ProjectTestMatriceCancelCall.html), [*test matrices create*](struct.ProjectTestMatriceCreateCall.html) and [*test matrices get*](struct.ProjectTestMatriceGetCall.html)
//! * [test environment catalog](struct.TestEnvironmentCatalog.html)
//!  * [*get*](struct.TestEnvironmentCatalogGetCall.html)
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
//! * **[Hub](struct.Testing.html)**
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
//! let r = hub.projects().test_matrices_create(...).doit()
//! let r = hub.projects().test_matrices_get(...).doit()
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
//! google-testing1 = "*"
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
//! extern crate google_testing1 as testing1;
//! use testing1::TestMatrix;
//! use testing1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use testing1::Testing;
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
//! let mut hub = Testing::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = TestMatrix::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.projects().test_matrices_create(req, "projectId")
//!              .request_id("sed")
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
    /// View and manage your data across Google Cloud Platform services
    CloudPlatform,

    /// View your data across Google Cloud Platform services
    CloudPlatformReadOnly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            Scope::CloudPlatformReadOnly => "https://www.googleapis.com/auth/cloud-platform.read-only",
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

/// Central instance to access all Testing related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_testing1 as testing1;
/// use testing1::TestMatrix;
/// use testing1::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use testing1::Testing;
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
/// let mut hub = Testing::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = TestMatrix::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().test_matrices_create(req, "projectId")
///              .request_id("dolores")
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
pub struct Testing<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for Testing<C, A> {}

impl<'a, C, A> Testing<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Testing<C, A> {
        Testing {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.12".to_string(),
            _base_url: "https://testing.googleapis.com/".to_string(),
            _root_url: "https://testing.googleapis.com/".to_string(),
        }
    }

    pub fn application_detail_service(&'a self) -> ApplicationDetailServiceMethods<'a, C, A> {
        ApplicationDetailServiceMethods { hub: &self }
    }
    pub fn projects(&'a self) -> ProjectMethods<'a, C, A> {
        ProjectMethods { hub: &self }
    }
    pub fn test_environment_catalog(&'a self) -> TestEnvironmentCatalogMethods<'a, C, A> {
        TestEnvironmentCatalogMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.12`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://testing.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://testing.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// A single Android device.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AndroidDevice {
    /// Required. The locale the test device used for testing.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    pub locale: Option<String>,
    /// Required. The id of the Android OS version to be used.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    #[serde(rename="androidVersionId")]
    pub android_version_id: Option<String>,
    /// Required. The id of the Android device to be used.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    #[serde(rename="androidModelId")]
    pub android_model_id: Option<String>,
    /// Required. How the device is oriented during the test.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    pub orientation: Option<String>,
}

impl Part for AndroidDevice {}


/// Locations where the results of running the test are stored.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResultStorage {
    /// The tool results history that contains the tool results execution that
    /// results are written to.
    /// 
    /// If not provided, the service will choose an appropriate value.
    #[serde(rename="toolResultsHistory")]
    pub tool_results_history: Option<ToolResultsHistory>,
    /// Required.
    #[serde(rename="googleCloudStorage")]
    pub google_cloud_storage: Option<GoogleCloudStorage>,
    /// Output only. The tool results execution that results are written to.
    #[serde(rename="toolResultsExecution")]
    pub tool_results_execution: Option<ToolResultsExecution>,
    /// Output only. URL to the results in the Firebase Web Console.
    #[serde(rename="resultsUrl")]
    pub results_url: Option<String>,
}

impl Part for ResultStorage {}


/// Represents a tool results execution resource.
/// 
/// This has the results of a TestMatrix.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ToolResultsExecution {
    /// Output only. The cloud project that owns the tool results execution.
    #[serde(rename="projectId")]
    pub project_id: Option<String>,
    /// Output only. A tool results execution ID.
    #[serde(rename="executionId")]
    pub execution_id: Option<String>,
    /// Output only. A tool results history ID.
    #[serde(rename="historyId")]
    pub history_id: Option<String>,
}

impl Part for ToolResultsExecution {}


/// A single iOS device.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IosDevice {
    /// Required. The locale the test device used for testing.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    pub locale: Option<String>,
    /// Required. The id of the iOS major software version to be used.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    #[serde(rename="iosVersionId")]
    pub ios_version_id: Option<String>,
    /// Required. How the device is oriented during the test.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    pub orientation: Option<String>,
    /// Required. The id of the iOS device to be used.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    #[serde(rename="iosModelId")]
    pub ios_model_id: Option<String>,
}

impl Part for IosDevice {}


/// The <intent-filter> section of an <activity> tag.
/// https://developer.android.com/guide/topics/manifest/intent-filter-element.html
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IntentFilter {
    /// The android:mimeType value of the <data> tag.
    #[serde(rename="mimeType")]
    pub mime_type: Option<String>,
    /// The android:name value of the <action> tag.
    #[serde(rename="actionNames")]
    pub action_names: Option<Vec<String>>,
    /// The android:name value of the <category> tag.
    #[serde(rename="categoryNames")]
    pub category_names: Option<Vec<String>>,
}

impl Part for IntentFilter {}


/// The environment in which the test is run.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Environment {
    /// An iOS device which must be used with an iOS test.
    #[serde(rename="iosDevice")]
    pub ios_device: Option<IosDevice>,
    /// An Android device which must be used with an Android test.
    #[serde(rename="androidDevice")]
    pub android_device: Option<AndroidDevice>,
}

impl Part for Environment {}


/// A set of Android device configuration permutations is defined by the
/// the cross-product of the given axes. Internally, the given AndroidMatrix
/// will be expanded into a set of AndroidDevices.
/// 
/// Only supported permutations will be instantiated.  Invalid permutations
/// (e.g., incompatible models/versions) are ignored.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AndroidMatrix {
    /// Required. The ids of the set of Android OS version to be used.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    #[serde(rename="androidVersionIds")]
    pub android_version_ids: Option<Vec<String>>,
    /// Required. The ids of the set of Android device to be used.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    #[serde(rename="androidModelIds")]
    pub android_model_ids: Option<Vec<String>>,
    /// Required. The set of locales the test device will enable for testing.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    pub locales: Option<Vec<String>>,
    /// Required. The set of orientations to test with.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    pub orientations: Option<Vec<String>>,
}

impl Part for AndroidMatrix {}


/// Response containing the current state of the specified test matrix.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [test matrices cancel projects](struct.ProjectTestMatriceCancelCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelTestMatrixResponse {
    /// The current rolled-up state of the test matrix.
    /// If this state is already final, then the cancelation request will
    /// have no effect.
    #[serde(rename="testState")]
    pub test_state: Option<String>,
}

impl ResponseResult for CancelTestMatrixResponse {}


/// A starting intent specified by an action, uri, and categories.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StartActivityIntent {
    /// Action name.
    /// Required for START_ACTIVITY.
    pub action: Option<String>,
    /// URI for the action.
    pub uri: Option<String>,
    /// Intent categories to set on the intent.
    pub categories: Option<Vec<String>>,
}

impl Part for StartActivityIntent {}


/// An Xcode version that an iOS version is compatible with.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct XcodeVersion {
    /// The id for this version.
    /// Example: "9.2".
    pub version: Option<String>,
    /// Tags for this Xcode version.
    /// Example: "default".
    pub tags: Option<Vec<String>>,
}

impl Part for XcodeVersion {}


/// Represents a tool results step resource.
/// 
/// This has the results of a TestExecution.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ToolResultsStep {
    /// Output only. The cloud project that owns the tool results step.
    #[serde(rename="projectId")]
    pub project_id: Option<String>,
    /// Output only. A tool results execution ID.
    #[serde(rename="executionId")]
    pub execution_id: Option<String>,
    /// Output only. A tool results step ID.
    #[serde(rename="stepId")]
    pub step_id: Option<String>,
    /// Output only. A tool results history ID.
    #[serde(rename="historyId")]
    pub history_id: Option<String>,
}

impl Part for ToolResultsStep {}


/// Data about the relative number of devices running a
/// given configuration of the Android platform.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Distribution {
    /// Output only. The time this distribution was measured.
    #[serde(rename="measurementTime")]
    pub measurement_time: Option<String>,
    /// Output only. The estimated fraction (0-1) of the total market with this
    /// configuration.
    #[serde(rename="marketShare")]
    pub market_share: Option<f64>,
}

impl Part for Distribution {}


/// Network emulation parameters.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TrafficRule {
    /// Packet delay, must be >= 0.
    pub delay: Option<String>,
    /// Packet loss ratio (0.0 - 1.0).
    #[serde(rename="packetLossRatio")]
    pub packet_loss_ratio: Option<f32>,
    /// Bandwidth in kbits/second.
    pub bandwidth: Option<f32>,
    /// Burst size in kbits.
    pub burst: Option<f32>,
    /// Packet duplication ratio (0.0 - 1.0).
    #[serde(rename="packetDuplicationRatio")]
    pub packet_duplication_ratio: Option<f32>,
}

impl Part for TrafficRule {}


/// A description of an Android device tests may be run on.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AndroidModel {
    /// Whether this device is a phone, tablet, wearable, etc.
    #[serde(rename="formFactor")]
    pub form_factor: Option<String>,
    /// The human-readable marketing name for this device model.
    /// Examples: "Nexus 5", "Galaxy S5".
    pub name: Option<String>,
    /// Whether this device is virtual or physical.
    pub form: Option<String>,
    /// Tags for this dimension.
    /// Examples: "default", "preview", "deprecated".
    pub tags: Option<Vec<String>>,
    /// True if and only if tests with this model are recorded by stitching
    /// together screenshots. See use_low_spec_video_recording in device config.
    #[serde(rename="lowFpsVideoRecording")]
    pub low_fps_video_recording: Option<bool>,
    /// The company that this device is branded with.
    /// Example: "Google", "Samsung".
    pub brand: Option<String>,
    /// The unique opaque id for this model.
    /// Use this for invoking the TestExecutionService.
    pub id: Option<String>,
    /// The set of Android versions this device supports.
    #[serde(rename="supportedVersionIds")]
    pub supported_version_ids: Option<Vec<String>>,
    /// Screen density in DPI.
    /// This corresponds to ro.sf.lcd_density
    #[serde(rename="screenDensity")]
    pub screen_density: Option<i32>,
    /// The list of supported ABIs for this device.
    /// This corresponds to either android.os.Build.SUPPORTED_ABIS (for API level
    /// 21 and above) or android.os.Build.CPU_ABI/CPU_ABI2.
    /// The most preferred ABI is the first element in the list.
    /// 
    /// Elements are optionally prefixed by "version_id:" (where version_id is
    /// the id of an AndroidVersion), denoting an ABI that is supported only on
    /// a particular version.
    #[serde(rename="supportedAbis")]
    pub supported_abis: Option<Vec<String>>,
    /// Screen size in the horizontal (X) dimension measured in pixels.
    #[serde(rename="screenX")]
    pub screen_x: Option<i32>,
    /// Screen size in the vertical (Y) dimension measured in pixels.
    #[serde(rename="screenY")]
    pub screen_y: Option<i32>,
    /// The manufacturer of this device.
    pub manufacturer: Option<String>,
    /// The name of the industrial design.
    /// This corresponds to android.os.Build.DEVICE.
    pub codename: Option<String>,
}

impl Part for AndroidModel {}


/// A list of Android device configurations in which the test is to be executed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AndroidDeviceList {
    /// Required. A list of Android devices.
    #[serde(rename="androidDevices")]
    pub android_devices: Option<Vec<AndroidDevice>>,
}

impl Part for AndroidDeviceList {}


/// Information about the client which invoked the test.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClientInfo {
    /// The list of detailed information about client.
    #[serde(rename="clientInfoDetails")]
    pub client_info_details: Option<Vec<ClientInfoDetail>>,
    /// Required. Client name, such as gcloud.
    pub name: Option<String>,
}

impl Part for ClientInfo {}


/// A description of how to set up an iOS device prior to running the test.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IosTestSetup {
    /// The network traffic profile used for running the test.
    /// Available network profiles can be queried by using the
    /// NETWORK_CONFIGURATION environment type when calling
    /// TestEnvironmentDiscoveryService.GetTestEnvironmentCatalog.
    #[serde(rename="networkProfile")]
    pub network_profile: Option<String>,
}

impl Part for IosTestSetup {}


/// Response containing the details of the specified Android application APK.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get apk details application detail service](struct.ApplicationDetailServiceGetApkDetailCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetApkDetailsResponse {
    /// Details of the Android APK.
    #[serde(rename="apkDetail")]
    pub apk_detail: Option<ApkDetail>,
}

impl ResponseResult for GetApkDetailsResponse {}


/// A test of an iOS application that uses the XCTest framework.
/// Xcode supports the option to "build for testing", which generates an
/// .xctestrun file that contains a test specification (arguments, test methods,
/// etc). This test type accepts a zip file containing the .xctestrun file and
/// the corresponding contents of the Build/Products directory that contains all
/// the binaries needed to run the tests.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IosXcTest {
    /// Output only. The bundle id for the application under test.
    #[serde(rename="appBundleId")]
    pub app_bundle_id: Option<String>,
    /// An .xctestrun file that will override the .xctestrun file in the
    /// tests zip. Because the .xctestrun file contains environment variables along
    /// with test methods to run and/or ignore, this can be useful for sharding
    /// tests. Default is taken from the tests zip.
    pub xctestrun: Option<FileReference>,
    /// Required. The .zip containing the .xctestrun file and the contents of the
    /// DerivedData/Build/Products directory.
    /// The .xctestrun file in this zip is ignored if the xctestrun field is
    /// specified.
    #[serde(rename="testsZip")]
    pub tests_zip: Option<FileReference>,
    /// The Xcode version that should be used for the test.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    /// Defaults to the latest Xcode version Firebase Test Lab supports.
    #[serde(rename="xcodeVersion")]
    pub xcode_version: Option<String>,
}

impl Part for IosXcTest {}


/// Represents a tool results history resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ToolResultsHistory {
    /// Required. The cloud project that owns the tool results history.
    #[serde(rename="projectId")]
    pub project_id: Option<String>,
    /// Required. A tool results history ID.
    #[serde(rename="historyId")]
    pub history_id: Option<String>,
}

impl Part for ToolResultsHistory {}


/// A single test executed in a single environment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestExecution {
    /// Output only. The cloud project that owns the test execution.
    #[serde(rename="projectId")]
    pub project_id: Option<String>,
    /// Output only. Id of the containing TestMatrix.
    #[serde(rename="matrixId")]
    pub matrix_id: Option<String>,
    /// Output only. Indicates the current progress of the test execution
    /// (e.g., FINISHED).
    pub state: Option<String>,
    /// Output only. Unique id set by the service.
    pub id: Option<String>,
    /// Output only. How the host machine(s) are configured.
    pub environment: Option<Environment>,
    /// Output only. How to run the test.
    #[serde(rename="testSpecification")]
    pub test_specification: Option<TestSpecification>,
    /// Output only. The time this test execution was initially created.
    pub timestamp: Option<String>,
    /// Output only. Where the results for this execution are written.
    #[serde(rename="toolResultsStep")]
    pub tool_results_step: Option<ToolResultsStep>,
    /// Output only. Additional details about the running test.
    #[serde(rename="testDetails")]
    pub test_details: Option<TestDetails>,
}

impl Part for TestExecution {}


/// An Android app manifest. See
/// http://developer.android.com/guide/topics/manifest/manifest-intro.html
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApkManifest {
    /// Maximum API level on which the application is designed to run.
    #[serde(rename="maxSdkVersion")]
    pub max_sdk_version: Option<i32>,
    /// Minimum API level required for the application to run.
    #[serde(rename="minSdkVersion")]
    pub min_sdk_version: Option<i32>,
    /// Full Java-style package name for this application, e.g.
    /// "com.example.foo".
    #[serde(rename="packageName")]
    pub package_name: Option<String>,
    /// Specifies the API Level on which the application is designed to run.
    #[serde(rename="targetSdkVersion")]
    pub target_sdk_version: Option<i32>,
    /// User-readable name for the application.
    #[serde(rename="applicationLabel")]
    pub application_label: Option<String>,
    /// no description provided
    #[serde(rename="intentFilters")]
    pub intent_filters: Option<Vec<IntentFilter>>,
}

impl Part for ApkManifest {}


/// A description of how to set up the Android device prior to running the test.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestSetup {
    /// The device will be logged in on this account for the duration of the test.
    pub account: Option<Account>,
    /// Environment variables to set for the test (only applicable for
    /// instrumentation tests).
    #[serde(rename="environmentVariables")]
    pub environment_variables: Option<Vec<EnvironmentVariable>>,
    /// The network traffic profile used for running the test.
    /// Available network profiles can be queried by using the
    /// NETWORK_CONFIGURATION environment type when calling
    /// TestEnvironmentDiscoveryService.GetTestEnvironmentCatalog.
    #[serde(rename="networkProfile")]
    pub network_profile: Option<String>,
    /// APKs to install in addition to those being directly tested.
    /// Currently capped at 100.
    #[serde(rename="additionalApks")]
    pub additional_apks: Option<Vec<Apk>>,
    /// List of directories on the device to upload to GCS at the end of the test;
    /// they must be absolute paths under /sdcard or /data/local/tmp.
    /// Path names are restricted to characters a-z A-Z 0-9 _ - . + and /
    /// 
    /// Note: The paths /sdcard and /data will be made available and treated as
    /// implicit path substitutions. E.g. if /sdcard on a particular device does
    /// not map to external storage, the system will replace it with the external
    /// storage path prefix for that device.
    #[serde(rename="directoriesToPull")]
    pub directories_to_pull: Option<Vec<String>>,
    /// List of files to push to the device before starting the test.
    #[serde(rename="filesToPush")]
    pub files_to_push: Option<Vec<DeviceFile>>,
}

impl Part for TestSetup {}


/// The currently supported Android devices.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AndroidDeviceCatalog {
    /// The set of supported Android device models.
    pub models: Option<Vec<AndroidModel>>,
    /// The set of supported runtime configurations.
    #[serde(rename="runtimeConfiguration")]
    pub runtime_configuration: Option<AndroidRuntimeConfiguration>,
    /// The set of supported Android OS versions.
    pub versions: Option<Vec<AndroidVersion>>,
}

impl Part for AndroidDeviceCatalog {}


/// An opaque binary blob file to install on the device before the test starts.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ObbFile {
    /// Required. OBB file name which must conform to the format as specified by
    /// Android
    /// e.g. [main|patch].0300110.com.example.android.obb
    /// which will be installed into
    ///   \<shared-storage\>/Android/obb/\<package-name\>/
    /// on the device.
    #[serde(rename="obbFileName")]
    pub obb_file_name: Option<String>,
    /// Required. Opaque Binary Blob (OBB) file(s) to install on the device.
    pub obb: Option<FileReference>,
}

impl Part for ObbFile {}


/// Represents a whole or partial calendar date, e.g. a birthday. The time of day
/// and time zone are either specified elsewhere or are not significant. The date
/// is relative to the Proleptic Gregorian Calendar. This can represent:
/// 
/// * A full date, with non-zero year, month and day values
/// * A month and day value, with a zero year, e.g. an anniversary
/// * A year on its own, with zero month and day values
/// * A year and month value, with a zero day, e.g. a credit card expiration date
/// 
/// Related types are google.type.TimeOfDay and `google.protobuf.Timestamp`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Date {
    /// Month of year. Must be from 1 to 12, or 0 if specifying a year without a
    /// month and day.
    pub month: Option<i32>,
    /// Day of month. Must be from 1 to 31 and valid for the year and month, or 0
    /// if specifying a year by itself or a year and month where the day is not
    /// significant.
    pub day: Option<i32>,
    /// Year of date. Must be from 1 to 9999, or 0 if specifying a date without
    /// a year.
    pub year: Option<i32>,
}

impl Part for Date {}


/// A test of an Android Application with a Test Loop.
/// The intent \<intent-name\> will be implicitly added, since Games is the only
/// user of this api, for the time being.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AndroidTestLoop {
    /// A multi-apk app bundle for the application under test.
    #[serde(rename="appBundle")]
    pub app_bundle: Option<AppBundle>,
    /// The java package for the application under test.
    /// The default is determined by examining the application's manifest.
    #[serde(rename="appPackageId")]
    pub app_package_id: Option<String>,
    /// The APK for the application under test.
    #[serde(rename="appApk")]
    pub app_apk: Option<FileReference>,
    /// The list of scenario labels that should be run during the test.
    /// The scenario labels should map to labels defined in the application's
    /// manifest. For example, player_experience and
    /// com.google.test.loops.player_experience add all of the loops labeled in the
    /// manifest with the com.google.test.loops.player_experience name to the
    /// execution.
    /// Scenarios can also be specified in the scenarios field.
    #[serde(rename="scenarioLabels")]
    pub scenario_labels: Option<Vec<String>>,
    /// The list of scenarios that should be run during the test.
    /// The default is all test loops, derived from the application's
    /// manifest.
    pub scenarios: Option<Vec<i32>>,
}

impl Part for AndroidTestLoop {}


/// A test of an Android application that can control an Android component
/// independently of its normal lifecycle.
/// Android instrumentation tests run an application APK and test APK inside the
/// same process on a virtual or physical AndroidDevice.  They also specify
/// a test runner class, such as com.google.GoogleTestRunner, which can vary
/// on the specific instrumentation framework chosen.
/// 
/// See <http://developer.android.com/tools/testing/testing_android.html> for
/// more information on types of Android tests.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AndroidInstrumentationTest {
    /// A multi-apk app bundle for the application under test.
    #[serde(rename="appBundle")]
    pub app_bundle: Option<AppBundle>,
    /// Required. The APK containing the test code to be executed.
    #[serde(rename="testApk")]
    pub test_apk: Option<FileReference>,
    /// The InstrumentationTestRunner class.
    /// The default value is determined by examining the application's manifest.
    #[serde(rename="testRunnerClass")]
    pub test_runner_class: Option<String>,
    /// The java package for the test to be executed.
    /// The default value is determined by examining the application's manifest.
    #[serde(rename="testPackageId")]
    pub test_package_id: Option<String>,
    /// The APK for the application under test.
    #[serde(rename="appApk")]
    pub app_apk: Option<FileReference>,
    /// The java package for the application under test.
    /// The default value is determined by examining the application's manifest.
    #[serde(rename="appPackageId")]
    pub app_package_id: Option<String>,
    /// The option of whether running each test within its own invocation of
    /// instrumentation with Android Test Orchestrator or not.
    /// ** Orchestrator is only compatible with AndroidJUnitRunner version 1.0 or
    /// higher! **
    /// Orchestrator offers the following benefits:
    ///  - No shared state
    ///  - Crashes are isolated
    ///  - Logs are scoped per test
    /// 
    /// See
    /// <https://developer.android.com/training/testing/junit-runner.html#using-android-test-orchestrator>
    /// for more information about Android Test Orchestrator.
    /// 
    /// If not set, the test will be run without the orchestrator.
    #[serde(rename="orchestratorOption")]
    pub orchestrator_option: Option<String>,
    /// Each target must be fully qualified with the package name or class name,
    /// in one of these formats:
    ///  - "package package_name"
    ///  - "class package_name.class_name"
    ///  - "class package_name.class_name#method_name"
    /// 
    /// If empty, all targets in the module will be run.
    #[serde(rename="testTargets")]
    pub test_targets: Option<Vec<String>>,
}

impl Part for AndroidInstrumentationTest {}


/// A list of iOS device configurations in which the test is to be executed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IosDeviceList {
    /// Required. A list of iOS devices.
    #[serde(rename="iosDevices")]
    pub ios_devices: Option<Vec<IosDevice>>,
}

impl Part for IosDeviceList {}


/// Additional details about the progress of the running test.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestDetails {
    /// Output only. Human-readable, detailed descriptions of the test's progress.
    /// For example: "Provisioning a device", "Starting Test".
    /// 
    /// During the course of execution new data may be appended
    /// to the end of progress_messages.
    #[serde(rename="progressMessages")]
    pub progress_messages: Option<Vec<String>>,
    /// Output only. If the TestState is ERROR, then this string will contain
    /// human-readable details about the error.
    #[serde(rename="errorMessage")]
    pub error_message: Option<String>,
}

impl Part for TestDetails {}


/// Key-value pair of detailed information about the client which invoked the
/// test. Examples: {'Version', '1.0'}, {'Release Track', 'BETA'}.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClientInfoDetail {
    /// Required. The key of detailed client information.
    pub key: Option<String>,
    /// Required. The value of detailed client information.
    pub value: Option<String>,
}

impl Part for ClientInfoDetail {}


/// A storage location within Google cloud storage (GCS).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudStorage {
    /// Required. The path to a directory in GCS that will
    /// eventually contain the results for this test.
    /// The requesting user must have write access on the bucket in the supplied
    /// path.
    #[serde(rename="gcsPath")]
    pub gcs_path: Option<String>,
}

impl Part for GoogleCloudStorage {}


/// An Android App Bundle file format, containing a BundleConfig.pb file,
/// a base module directory, zero or more dynamic feature module directories.
/// <p>See https://developer.android.com/guide/app-bundle/build for guidance on
/// building App Bundles.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppBundle {
    /// .aab file representing the app bundle under test.
    #[serde(rename="bundleLocation")]
    pub bundle_location: Option<FileReference>,
}

impl Part for AppBundle {}


/// Identifies an account and how to log into it.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    /// An automatic google login account.
    #[serde(rename="googleAuto")]
    pub google_auto: Option<GoogleAuto>,
}

impl Part for Account {}


/// A description of an iOS device tests may be run on.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IosModel {
    /// Whether this device is a phone, tablet, wearable, etc.
    #[serde(rename="formFactor")]
    pub form_factor: Option<String>,
    /// The human-readable name for this device model.
    /// Examples: "iPhone 4s", "iPad Mini 2".
    pub name: Option<String>,
    /// Tags for this dimension.
    /// Examples: "default", "preview", "deprecated".
    pub tags: Option<Vec<String>>,
    /// The set of iOS major software versions this device supports.
    #[serde(rename="supportedVersionIds")]
    pub supported_version_ids: Option<Vec<String>>,
    /// The unique opaque id for this model.
    /// Use this for invoking the TestExecutionService.
    pub id: Option<String>,
    /// Device capabilities.
    /// Copied from
    /// https://developer.apple.com/library/archive/documentation/DeviceInformation/Reference/iOSDeviceCompatibility/DeviceCompatibilityMatrix/DeviceCompatibilityMatrix.html
    #[serde(rename="deviceCapabilities")]
    pub device_capabilities: Option<Vec<String>>,
}

impl Part for IosModel {}


/// The currently provided software environment on the devices under test.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProvidedSoftwareCatalog {
    /// A string representing the current version of Android Test
    /// Orchestrator that is provided by TestExecutionService.
    /// Example: "1.0.2 beta".
    #[serde(rename="orchestratorVersion")]
    pub orchestrator_version: Option<String>,
}

impl Part for ProvidedSoftwareCatalog {}


/// A description of how to run the test.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestSpecification {
    /// Test setup requirements for iOS.
    #[serde(rename="iosTestSetup")]
    pub ios_test_setup: Option<IosTestSetup>,
    /// An iOS XCTest, via an .xctestrun file.
    #[serde(rename="iosXcTest")]
    pub ios_xc_test: Option<IosXcTest>,
    /// Max time a test execution is allowed to run before it is
    /// automatically cancelled.
    /// The default value is 5 min.
    #[serde(rename="testTimeout")]
    pub test_timeout: Option<String>,
    /// Test setup requirements for Android e.g. files to install, bootstrap
    /// scripts.
    #[serde(rename="testSetup")]
    pub test_setup: Option<TestSetup>,
    /// Disables video recording. May reduce test latency.
    #[serde(rename="disableVideoRecording")]
    pub disable_video_recording: Option<bool>,
    /// Disables performance metrics recording. May reduce test latency.
    #[serde(rename="disablePerformanceMetrics")]
    pub disable_performance_metrics: Option<bool>,
    /// An Android Application with a Test Loop.
    #[serde(rename="androidTestLoop")]
    pub android_test_loop: Option<AndroidTestLoop>,
    /// An Android robo test.
    #[serde(rename="androidRoboTest")]
    pub android_robo_test: Option<AndroidRoboTest>,
    /// An Android instrumentation test.
    #[serde(rename="androidInstrumentationTest")]
    pub android_instrumentation_test: Option<AndroidInstrumentationTest>,
}

impl Part for TestSpecification {}


/// The matrix of environments in which the test is to be executed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnvironmentMatrix {
    /// A list of Android devices; the test will be run only on the specified
    /// devices.
    #[serde(rename="androidDeviceList")]
    pub android_device_list: Option<AndroidDeviceList>,
    /// A matrix of Android devices.
    #[serde(rename="androidMatrix")]
    pub android_matrix: Option<AndroidMatrix>,
    /// A list of iOS devices.
    #[serde(rename="iosDeviceList")]
    pub ios_device_list: Option<IosDeviceList>,
}

impl Part for EnvironmentMatrix {}


/// Message for specifying the start activities to crawl.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RoboStartingIntent {
    /// An intent that starts an activity with specific details.
    #[serde(rename="startActivity")]
    pub start_activity: Option<StartActivityIntent>,
    /// Timeout in seconds for each intent.
    pub timeout: Option<String>,
    /// An intent that starts the main launcher activity.
    #[serde(rename="launcherActivity")]
    pub launcher_activity: Option<LauncherActivityIntent>,
}

impl Part for RoboStartingIntent {}


/// A test of an android application that explores the application on a virtual
/// or physical Android Device, finding culprits and crashes as it goes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AndroidRoboTest {
    /// A multi-apk app bundle for the application under test.
    #[serde(rename="appBundle")]
    pub app_bundle: Option<AppBundle>,
    /// A set of directives Robo should apply during the crawl.
    /// This allows users to customize the crawl. For example, the username and
    /// password for a test account can be provided.
    #[serde(rename="roboDirectives")]
    pub robo_directives: Option<Vec<RoboDirective>>,
    /// A JSON file with a sequence of actions Robo should perform as a prologue
    /// for the crawl.
    #[serde(rename="roboScript")]
    pub robo_script: Option<FileReference>,
    /// The max number of steps Robo can execute.
    /// Default is no limit.
    #[serde(rename="maxSteps")]
    pub max_steps: Option<i32>,
    /// The max depth of the traversal stack Robo can explore. Needs to be at least
    /// 2 to make Robo explore the app beyond the first activity.
    /// Default is 50.
    #[serde(rename="maxDepth")]
    pub max_depth: Option<i32>,
    /// The intents used to launch the app for the crawl.
    /// If none are provided, then the main launcher activity is launched.
    /// If some are provided, then only those provided are launched (the main
    /// launcher activity must be provided explicitly).
    #[serde(rename="startingIntents")]
    pub starting_intents: Option<Vec<RoboStartingIntent>>,
    /// The APK for the application under test.
    #[serde(rename="appApk")]
    pub app_apk: Option<FileReference>,
    /// The java package for the application under test.
    /// The default value is determined by examining the application's manifest.
    #[serde(rename="appPackageId")]
    pub app_package_id: Option<String>,
    /// The initial activity that should be used to start the app.
    #[serde(rename="appInitialActivity")]
    pub app_initial_activity: Option<String>,
}

impl Part for AndroidRoboTest {}


/// A reference to a file, used for user inputs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get apk details application detail service](struct.ApplicationDetailServiceGetApkDetailCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileReference {
    /// A path to a file in Google Cloud Storage.
    /// Example: gs://build-app-1414623860166/app-debug-unaligned.apk
    #[serde(rename="gcsPath")]
    pub gcs_path: Option<String>,
}

impl RequestValue for FileReference {}


/// A version of the Android OS.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AndroidVersion {
    /// The date this Android version became available in the market.
    #[serde(rename="releaseDate")]
    pub release_date: Option<Date>,
    /// A string representing this version of the Android OS.
    /// Examples: "4.3", "4.4".
    #[serde(rename="versionString")]
    pub version_string: Option<String>,
    /// Tags for this dimension.
    /// Examples: "default", "preview", "deprecated".
    pub tags: Option<Vec<String>>,
    /// Market share for this version.
    pub distribution: Option<Distribution>,
    /// The API level for this Android version.
    /// Examples: 18, 19.
    #[serde(rename="apiLevel")]
    pub api_level: Option<i32>,
    /// The code name for this Android version.
    /// Examples: "JellyBean", "KitKat".
    #[serde(rename="codeName")]
    pub code_name: Option<String>,
    /// An opaque id for this Android version.
    /// Use this id to invoke the TestExecutionService.
    pub id: Option<String>,
}

impl Part for AndroidVersion {}


/// Directs Robo to interact with a specific UI element if it is encountered
/// during the crawl. Currently, Robo can perform text entry or element click.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RoboDirective {
    /// The text that Robo is directed to set. If left empty, the directive will be
    /// treated as a CLICK on the element matching the resource_name.
    #[serde(rename="inputText")]
    pub input_text: Option<String>,
    /// Required. The android resource name of the target UI element.
    /// For example,
    ///    in Java: R.string.foo
    ///    in xml: @string/foo
    /// Only the "foo" part is needed.
    /// Reference doc:
    /// https://developer.android.com/guide/topics/resources/accessing-resources.html
    #[serde(rename="resourceName")]
    pub resource_name: Option<String>,
    /// Required. The type of action that Robo should perform on the specified
    /// element.
    #[serde(rename="actionType")]
    pub action_type: Option<String>,
}

impl Part for RoboDirective {}


/// An iOS version.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IosVersion {
    /// An integer representing the major iOS version.
    /// Examples: "8", "9".
    #[serde(rename="majorVersion")]
    pub major_version: Option<i32>,
    /// Tags for this dimension.
    /// Examples: "default", "preview", "deprecated".
    pub tags: Option<Vec<String>>,
    /// An integer representing the minor iOS version.
    /// Examples: "1", "2".
    #[serde(rename="minorVersion")]
    pub minor_version: Option<i32>,
    /// An opaque id for this iOS version.
    /// Use this id to invoke the TestExecutionService.
    pub id: Option<String>,
    /// The available Xcode versions for this version.
    #[serde(rename="supportedXcodeVersionIds")]
    pub supported_xcode_version_ids: Option<Vec<String>>,
}

impl Part for IosVersion {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkConfiguration {
    /// The unique opaque id for this network traffic configuration.
    pub id: Option<String>,
    /// The emulation rule applying to the download traffic.
    #[serde(rename="downRule")]
    pub down_rule: Option<TrafficRule>,
    /// The emulation rule applying to the upload traffic.
    #[serde(rename="upRule")]
    pub up_rule: Option<TrafficRule>,
}

impl Part for NetworkConfiguration {}


/// Screen orientation of the device.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Orientation {
    /// The id for this orientation.
    /// Example: "portrait".
    pub id: Option<String>,
    /// A human-friendly name for this orientation.
    /// Example: "portrait".
    pub name: Option<String>,
    /// Tags for this dimension.
    /// Example: "default".
    pub tags: Option<Vec<String>>,
}

impl Part for Orientation {}


/// A location/region designation for language.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Locale {
    /// A human-friendly string representing the region for this
    /// locale. Example: "United States". Not present for every locale.
    pub region: Option<String>,
    /// The id for this locale.
    /// Example: "en_US".
    pub id: Option<String>,
    /// A human-friendly name for this language/locale.
    /// Example: "English".
    pub name: Option<String>,
    /// Tags for this dimension.
    /// Example: "default".
    pub tags: Option<Vec<String>>,
}

impl Part for Locale {}


/// Android configuration that can be selected at the time a test is run.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AndroidRuntimeConfiguration {
    /// The set of available orientations.
    pub orientations: Option<Vec<Orientation>>,
    /// The set of available locales.
    pub locales: Option<Vec<Locale>>,
}

impl Part for AndroidRuntimeConfiguration {}


/// A description of a test environment.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get test environment catalog](struct.TestEnvironmentCatalogGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestEnvironmentCatalog {
    /// Supported iOS devices.
    #[serde(rename="iosDeviceCatalog")]
    pub ios_device_catalog: Option<IosDeviceCatalog>,
    /// Supported Android devices.
    #[serde(rename="androidDeviceCatalog")]
    pub android_device_catalog: Option<AndroidDeviceCatalog>,
    /// Supported network configurations.
    #[serde(rename="networkConfigurationCatalog")]
    pub network_configuration_catalog: Option<NetworkConfigurationCatalog>,
    /// The software test environment provided by TestExecutionService.
    #[serde(rename="softwareCatalog")]
    pub software_catalog: Option<ProvidedSoftwareCatalog>,
}

impl ResponseResult for TestEnvironmentCatalog {}


/// A key-value pair passed as an environment variable to the test.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnvironmentVariable {
    /// Key for the environment variable.
    pub key: Option<String>,
    /// Value for the environment variable.
    pub value: Option<String>,
}

impl Part for EnvironmentVariable {}


/// An Android package file to install.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Apk {
    /// The java package for the APK to be installed.
    /// Value is determined by examining the application's manifest.
    #[serde(rename="packageName")]
    pub package_name: Option<String>,
    /// The path to an APK to be installed on the device before the test begins.
    pub location: Option<FileReference>,
}

impl Part for Apk {}


/// The currently supported iOS devices.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IosDeviceCatalog {
    /// The set of supported iOS device models.
    pub models: Option<Vec<IosModel>>,
    /// The set of supported Xcode versions.
    #[serde(rename="xcodeVersions")]
    pub xcode_versions: Option<Vec<XcodeVersion>>,
    /// The set of supported runtime configurations.
    #[serde(rename="runtimeConfiguration")]
    pub runtime_configuration: Option<IosRuntimeConfiguration>,
    /// The set of supported iOS software versions.
    pub versions: Option<Vec<IosVersion>>,
}

impl Part for IosDeviceCatalog {}


/// TestMatrix captures all details about a test. It contains the environment
/// configuration, test specification, test executions and overall state and
/// outcome.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [test matrices create projects](struct.ProjectTestMatriceCreateCall.html) (request|response)
/// * [test matrices get projects](struct.ProjectTestMatriceGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestMatrix {
    /// Information about the client which invoked the test.
    #[serde(rename="clientInfo")]
    pub client_info: Option<ClientInfo>,
    /// The cloud project that owns the test matrix.
    #[serde(rename="projectId")]
    pub project_id: Option<String>,
    /// Required. Where the results for the matrix are written.
    #[serde(rename="resultStorage")]
    pub result_storage: Option<ResultStorage>,
    /// The number of times a TestExecution should be re-attempted if one or more
    /// of its test cases fail for any reason.
    /// The maximum number of reruns allowed is 10.
    /// 
    /// Default is 0, which implies no reruns.
    #[serde(rename="flakyTestAttempts")]
    pub flaky_test_attempts: Option<i32>,
    /// Output only. Indicates the current progress of the test matrix.
    pub state: Option<String>,
    /// Output only. The list of test executions that the service creates for
    /// this matrix.
    #[serde(rename="testExecutions")]
    pub test_executions: Option<Vec<TestExecution>>,
    /// Required. How to run the test.
    #[serde(rename="testSpecification")]
    pub test_specification: Option<TestSpecification>,
    /// Output only. Unique id set by the service.
    #[serde(rename="testMatrixId")]
    pub test_matrix_id: Option<String>,
    /// Output only. The time this test matrix was initially created.
    pub timestamp: Option<String>,
    /// Output only. Describes why the matrix is considered invalid.
    /// Only useful for matrices in the INVALID state.
    #[serde(rename="invalidMatrixDetails")]
    pub invalid_matrix_details: Option<String>,
    /// Required. The devices the tests are being executed on.
    #[serde(rename="environmentMatrix")]
    pub environment_matrix: Option<EnvironmentMatrix>,
    /// Output Only. The overall outcome of the test.
    /// Only set when the test matrix state is FINISHED.
    #[serde(rename="outcomeSummary")]
    pub outcome_summary: Option<String>,
}

impl RequestValue for TestMatrix {}
impl ResponseResult for TestMatrix {}


/// Specifies an intent that starts the main launcher activity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LauncherActivityIntent { _never_set: Option<bool> }

impl Part for LauncherActivityIntent {}


/// A single device file description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceFile {
    /// A reference to a regular file.
    #[serde(rename="regularFile")]
    pub regular_file: Option<RegularFile>,
    /// A reference to an opaque binary blob file.
    #[serde(rename="obbFile")]
    pub obb_file: Option<ObbFile>,
}

impl Part for DeviceFile {}


/// Android application details based on application manifest and apk archive
/// contents.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApkDetail {
    /// no description provided
    #[serde(rename="apkManifest")]
    pub apk_manifest: Option<ApkManifest>,
}

impl Part for ApkDetail {}


/// iOS configuration that can be selected at the time a test is run.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IosRuntimeConfiguration {
    /// The set of available orientations.
    pub orientations: Option<Vec<Orientation>>,
    /// The set of available locales.
    pub locales: Option<Vec<Locale>>,
}

impl Part for IosRuntimeConfiguration {}


/// A file or directory to install on the device before the test starts.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RegularFile {
    /// Required. The source file.
    pub content: Option<FileReference>,
    /// Required. Where to put the content on the device. Must be an absolute,
    /// whitelisted path. If the file exists, it will be replaced.
    /// The following device-side directories and any of their subdirectories are
    /// whitelisted:
    /// <p>${EXTERNAL_STORAGE}, or /sdcard</p>
    /// <p>${ANDROID_DATA}/local/tmp, or /data/local/tmp</p>
    /// <p>Specifying a path outside of these directory trees is invalid.
    /// 
    /// <p> The paths /sdcard and /data will be made available and treated as
    /// implicit path substitutions. E.g. if /sdcard on a particular device does
    /// not map to external storage, the system will replace it with the external
    /// storage path prefix for that device and copy the file there.
    /// 
    /// <p> It is strongly advised to use the <a href=
    /// "http://developer.android.com/reference/android/os/Environment.html">
    /// Environment API</a> in app and test code to access files on the device in a
    /// portable way.
    #[serde(rename="devicePath")]
    pub device_path: Option<String>,
}

impl Part for RegularFile {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkConfigurationCatalog {
    /// no description provided
    pub configurations: Option<Vec<NetworkConfiguration>>,
}

impl Part for NetworkConfigurationCatalog {}


/// Enables automatic Google account login.
/// If set, the service automatically generates a Google test account and adds
/// it to the device, before executing the test. Note that test accounts might be
/// reused.
/// Many applications show their full set of functionalities when an account is
/// present on the device. Logging into the device with these generated accounts
/// allows testing more functionalities.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAuto { _never_set: Option<bool> }

impl Part for GoogleAuto {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *testEnvironmentCatalog* resources.
/// It is not used directly, but through the `Testing` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_testing1 as testing1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use testing1::Testing;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Testing::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.test_environment_catalog();
/// # }
/// ```
pub struct TestEnvironmentCatalogMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Testing<C, A>,
}

impl<'a, C, A> MethodsBuilder for TestEnvironmentCatalogMethods<'a, C, A> {}

impl<'a, C, A> TestEnvironmentCatalogMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the catalog of supported test environments.
    /// 
    /// May return any of the following canonical error codes:
    /// 
    /// - INVALID_ARGUMENT - if the request is malformed
    /// - NOT_FOUND - if the environment type does not exist
    /// - INTERNAL - if an internal error occurred
    /// 
    /// # Arguments
    ///
    /// * `environmentType` - Required. The type of environment that should be listed.
    pub fn get(&self, environment_type: &str) -> TestEnvironmentCatalogGetCall<'a, C, A> {
        TestEnvironmentCatalogGetCall {
            hub: self.hub,
            _environment_type: environment_type.to_string(),
            _project_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *applicationDetailService* resources.
/// It is not used directly, but through the `Testing` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_testing1 as testing1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use testing1::Testing;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Testing::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_apk_details(...)`
/// // to build up your call.
/// let rb = hub.application_detail_service();
/// # }
/// ```
pub struct ApplicationDetailServiceMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Testing<C, A>,
}

impl<'a, C, A> MethodsBuilder for ApplicationDetailServiceMethods<'a, C, A> {}

impl<'a, C, A> ApplicationDetailServiceMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the details of an Android application APK.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn get_apk_details(&self, request: FileReference) -> ApplicationDetailServiceGetApkDetailCall<'a, C, A> {
        ApplicationDetailServiceGetApkDetailCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the `Testing` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_testing1 as testing1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use testing1::Testing;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Testing::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `test_matrices_cancel(...)`, `test_matrices_create(...)` and `test_matrices_get(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Testing<C, A>,
}

impl<'a, C, A> MethodsBuilder for ProjectMethods<'a, C, A> {}

impl<'a, C, A> ProjectMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates and runs a matrix of tests according to the given specifications.
    /// Unsupported environments will be returned in the state UNSUPPORTED.
    /// Matrices are limited to at most 200 supported executions.
    /// 
    /// May return any of the following canonical error codes:
    /// 
    /// * PERMISSION_DENIED - if the user is not authorized to write to project
    /// * INVALID_ARGUMENT - if the request is malformed or if the matrix expands
    ///   to more than 200 supported executions
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - The GCE project under which this job will run.
    pub fn test_matrices_create(&self, request: TestMatrix, project_id: &str) -> ProjectTestMatriceCreateCall<'a, C, A> {
        ProjectTestMatriceCreateCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Cancels unfinished test executions in a test matrix.
    /// This call returns immediately and cancellation proceeds asychronously.
    /// If the matrix is already final, this operation will have no effect.
    /// 
    /// May return any of the following canonical error codes:
    /// 
    /// - PERMISSION_DENIED - if the user is not authorized to read project
    /// - INVALID_ARGUMENT - if the request is malformed
    /// - NOT_FOUND - if the Test Matrix does not exist
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Cloud project that owns the test.
    /// * `testMatrixId` - Test matrix that will be canceled.
    pub fn test_matrices_cancel(&self, project_id: &str, test_matrix_id: &str) -> ProjectTestMatriceCancelCall<'a, C, A> {
        ProjectTestMatriceCancelCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _test_matrix_id: test_matrix_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Checks the status of a test matrix.
    /// 
    /// May return any of the following canonical error codes:
    /// 
    /// - PERMISSION_DENIED - if the user is not authorized to read project
    /// - INVALID_ARGUMENT - if the request is malformed
    /// - NOT_FOUND - if the Test Matrix does not exist
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Cloud project that owns the test matrix.
    /// * `testMatrixId` - Unique test matrix id which was assigned by the service.
    pub fn test_matrices_get(&self, project_id: &str, test_matrix_id: &str) -> ProjectTestMatriceGetCall<'a, C, A> {
        ProjectTestMatriceGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _test_matrix_id: test_matrix_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Gets the catalog of supported test environments.
/// 
/// May return any of the following canonical error codes:
/// 
/// - INVALID_ARGUMENT - if the request is malformed
/// - NOT_FOUND - if the environment type does not exist
/// - INTERNAL - if an internal error occurred
///
/// A builder for the *get* method supported by a *testEnvironmentCatalog* resource.
/// It is not used directly, but through a `TestEnvironmentCatalogMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_testing1 as testing1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use testing1::Testing;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Testing::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.test_environment_catalog().get("environmentType")
///              .project_id("accusam")
///              .doit();
/// # }
/// ```
pub struct TestEnvironmentCatalogGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Testing<C, A>,
    _environment_type: String,
    _project_id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for TestEnvironmentCatalogGetCall<'a, C, A> {}

impl<'a, C, A> TestEnvironmentCatalogGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, TestEnvironmentCatalog)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "testing.testEnvironmentCatalog.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("environmentType", self._environment_type.to_string()));
        if let Some(value) = self._project_id {
            params.push(("projectId", value.to_string()));
        }
        for &field in ["alt", "environmentType", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/testEnvironmentCatalog/{environmentType}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{environmentType}", "environmentType")].iter() {
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
            for param_name in ["environmentType"].iter() {
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


    /// Required. The type of environment that should be listed.
    ///
    /// Sets the *environment type* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn environment_type(mut self, new_value: &str) -> TestEnvironmentCatalogGetCall<'a, C, A> {
        self._environment_type = new_value.to_string();
        self
    }
    /// For authorization, the cloud project requesting the TestEnvironmentCatalog.
    ///
    /// Sets the *project id* query property to the given value.
    pub fn project_id(mut self, new_value: &str) -> TestEnvironmentCatalogGetCall<'a, C, A> {
        self._project_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> TestEnvironmentCatalogGetCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> TestEnvironmentCatalogGetCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> TestEnvironmentCatalogGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets the details of an Android application APK.
///
/// A builder for the *getApkDetails* method supported by a *applicationDetailService* resource.
/// It is not used directly, but through a `ApplicationDetailServiceMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_testing1 as testing1;
/// use testing1::FileReference;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use testing1::Testing;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Testing::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = FileReference::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.application_detail_service().get_apk_details(req)
///              .doit();
/// # }
/// ```
pub struct ApplicationDetailServiceGetApkDetailCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Testing<C, A>,
    _request: FileReference,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ApplicationDetailServiceGetApkDetailCall<'a, C, A> {}

impl<'a, C, A> ApplicationDetailServiceGetApkDetailCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GetApkDetailsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "testing.applicationDetailService.getApkDetails",
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

        let mut url = self.hub._base_url.clone() + "v1/applicationDetailService/getApkDetails";
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
    pub fn request(mut self, new_value: FileReference) -> ApplicationDetailServiceGetApkDetailCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ApplicationDetailServiceGetApkDetailCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ApplicationDetailServiceGetApkDetailCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ApplicationDetailServiceGetApkDetailCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates and runs a matrix of tests according to the given specifications.
/// Unsupported environments will be returned in the state UNSUPPORTED.
/// Matrices are limited to at most 200 supported executions.
/// 
/// May return any of the following canonical error codes:
/// 
/// * PERMISSION_DENIED - if the user is not authorized to write to project
/// * INVALID_ARGUMENT - if the request is malformed or if the matrix expands
///   to more than 200 supported executions
///
/// A builder for the *testMatrices.create* method supported by a *project* resource.
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
/// # extern crate google_testing1 as testing1;
/// use testing1::TestMatrix;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use testing1::Testing;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Testing::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = TestMatrix::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().test_matrices_create(req, "projectId")
///              .request_id("justo")
///              .doit();
/// # }
/// ```
pub struct ProjectTestMatriceCreateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Testing<C, A>,
    _request: TestMatrix,
    _project_id: String,
    _request_id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectTestMatriceCreateCall<'a, C, A> {}

impl<'a, C, A> ProjectTestMatriceCreateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, TestMatrix)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "testing.projects.testMatrices.create",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        if let Some(value) = self._request_id {
            params.push(("requestId", value.to_string()));
        }
        for &field in ["alt", "projectId", "requestId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/projects/{projectId}/testMatrices";
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
    pub fn request(mut self, new_value: TestMatrix) -> ProjectTestMatriceCreateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The GCE project under which this job will run.
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> ProjectTestMatriceCreateCall<'a, C, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// A string id used to detect duplicated requests.
    /// Ids are automatically scoped to a project, so
    /// users should ensure the ID is unique per-project.
    /// A UUID is recommended.
    /// 
    /// Optional, but strongly recommended.
    ///
    /// Sets the *request id* query property to the given value.
    pub fn request_id(mut self, new_value: &str) -> ProjectTestMatriceCreateCall<'a, C, A> {
        self._request_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectTestMatriceCreateCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectTestMatriceCreateCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectTestMatriceCreateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Cancels unfinished test executions in a test matrix.
/// This call returns immediately and cancellation proceeds asychronously.
/// If the matrix is already final, this operation will have no effect.
/// 
/// May return any of the following canonical error codes:
/// 
/// - PERMISSION_DENIED - if the user is not authorized to read project
/// - INVALID_ARGUMENT - if the request is malformed
/// - NOT_FOUND - if the Test Matrix does not exist
///
/// A builder for the *testMatrices.cancel* method supported by a *project* resource.
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
/// # extern crate google_testing1 as testing1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use testing1::Testing;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Testing::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().test_matrices_cancel("projectId", "testMatrixId")
///              .doit();
/// # }
/// ```
pub struct ProjectTestMatriceCancelCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Testing<C, A>,
    _project_id: String,
    _test_matrix_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectTestMatriceCancelCall<'a, C, A> {}

impl<'a, C, A> ProjectTestMatriceCancelCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, CancelTestMatrixResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "testing.projects.testMatrices.cancel",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        params.push(("testMatrixId", self._test_matrix_id.to_string()));
        for &field in ["alt", "projectId", "testMatrixId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/projects/{projectId}/testMatrices/{testMatrixId}:cancel";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId"), ("{testMatrixId}", "testMatrixId")].iter() {
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
            for param_name in ["testMatrixId", "projectId"].iter() {
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
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


    /// Cloud project that owns the test.
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> ProjectTestMatriceCancelCall<'a, C, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// Test matrix that will be canceled.
    ///
    /// Sets the *test matrix id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn test_matrix_id(mut self, new_value: &str) -> ProjectTestMatriceCancelCall<'a, C, A> {
        self._test_matrix_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectTestMatriceCancelCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectTestMatriceCancelCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectTestMatriceCancelCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Checks the status of a test matrix.
/// 
/// May return any of the following canonical error codes:
/// 
/// - PERMISSION_DENIED - if the user is not authorized to read project
/// - INVALID_ARGUMENT - if the request is malformed
/// - NOT_FOUND - if the Test Matrix does not exist
///
/// A builder for the *testMatrices.get* method supported by a *project* resource.
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
/// # extern crate google_testing1 as testing1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use testing1::Testing;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Testing::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().test_matrices_get("projectId", "testMatrixId")
///              .doit();
/// # }
/// ```
pub struct ProjectTestMatriceGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Testing<C, A>,
    _project_id: String,
    _test_matrix_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectTestMatriceGetCall<'a, C, A> {}

impl<'a, C, A> ProjectTestMatriceGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, TestMatrix)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "testing.projects.testMatrices.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        params.push(("testMatrixId", self._test_matrix_id.to_string()));
        for &field in ["alt", "projectId", "testMatrixId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/projects/{projectId}/testMatrices/{testMatrixId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId"), ("{testMatrixId}", "testMatrixId")].iter() {
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
            for param_name in ["testMatrixId", "projectId"].iter() {
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


    /// Cloud project that owns the test matrix.
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> ProjectTestMatriceGetCall<'a, C, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// Unique test matrix id which was assigned by the service.
    ///
    /// Sets the *test matrix id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn test_matrix_id(mut self, new_value: &str) -> ProjectTestMatriceGetCall<'a, C, A> {
        self._test_matrix_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectTestMatriceGetCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectTestMatriceGetCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectTestMatriceGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


