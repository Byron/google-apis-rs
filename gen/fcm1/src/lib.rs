// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Firebase Cloud Messaging* crate version *1.0.13+20200406*, where *20200406* is the exact revision of the *fcm:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.13*.
//! 
//! Everything else about the *Firebase Cloud Messaging* *v1* API can be found at the
//! [official documentation site](https://firebase.google.com/docs/cloud-messaging).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/fcm1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.FirebaseCloudMessaging.html) ... 
//! 
//! * projects
//!  * [*messages send*](struct.ProjectMessageSendCall.html)
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
//! * **[Hub](struct.FirebaseCloudMessaging.html)**
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
//! let r = hub.projects().messages_send(...).doit()
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
//! google-fcm1 = "*"
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
//! extern crate google_fcm1 as fcm1;
//! use fcm1::SendMessageRequest;
//! use fcm1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use fcm1::FirebaseCloudMessaging;
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
//! let mut hub = FirebaseCloudMessaging::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = SendMessageRequest::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.projects().messages_send(req, "parent")
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

/// Central instance to access all FirebaseCloudMessaging related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_fcm1 as fcm1;
/// use fcm1::SendMessageRequest;
/// use fcm1::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use fcm1::FirebaseCloudMessaging;
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
/// let mut hub = FirebaseCloudMessaging::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = SendMessageRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().messages_send(req, "parent")
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
pub struct FirebaseCloudMessaging<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for FirebaseCloudMessaging<C, A> {}

impl<'a, C, A> FirebaseCloudMessaging<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> FirebaseCloudMessaging<C, A> {
        FirebaseCloudMessaging {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.13".to_string(),
            _base_url: "https://fcm.googleapis.com/".to_string(),
            _root_url: "https://fcm.googleapis.com/".to_string(),
        }
    }

    pub fn projects(&'a self) -> ProjectMethods<'a, C, A> {
        ProjectMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.13`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://fcm.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://fcm.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// Platform independent options for features provided by the FCM SDKs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FcmOptions {
    /// Label associated with the message's analytics data.
    #[serde(rename="analyticsLabel")]
    pub analytics_label: Option<String>,
}

impl Part for FcmOptions {}


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
pub struct Color {
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

impl Part for Color {}


/// Basic notification template to use across all platforms.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Notification {
    /// The notification's body text.
    pub body: Option<String>,
    /// Contains the URL of an image that is going to be downloaded on the device
    /// and displayed in a notification.
    /// JPEG, PNG, BMP have full support across platforms. Animated GIF and video
    /// only work on iOS. WebP and HEIF have varying levels of support across
    /// platforms and platform versions.
    /// Android has 1MB image size limit.
    /// Quota usage and implications/costs for hosting image on Firebase Storage:
    /// https://firebase.google.com/pricing
    pub image: Option<String>,
    /// The notification's title.
    pub title: Option<String>,
}

impl Part for Notification {}


/// Android specific options for messages sent through
/// [FCM connection server](https://goo.gl/4GLdUl).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AndroidConfig {
    /// Options for features provided by the FCM SDK for Android.
    #[serde(rename="fcmOptions")]
    pub fcm_options: Option<AndroidFcmOptions>,
    /// Notification to send to android devices.
    pub notification: Option<AndroidNotification>,
    /// Message priority. Can take "normal" and "high" values.
    /// For more information, see [Setting the priority of a
    /// message](https://goo.gl/GjONJv).
    pub priority: Option<String>,
    /// An identifier of a group of messages that can be collapsed, so that only
    /// the last message gets sent when delivery can be resumed. A maximum of 4
    /// different collapse keys is allowed at any given time.
    #[serde(rename="collapseKey")]
    pub collapse_key: Option<String>,
    /// If set to true, messages will be allowed to be delivered to the app while
    /// the device is in direct boot mode. See [Support Direct Boot
    /// mode](https://developer.android.com/training/articles/direct-boot).
    #[serde(rename="directBootOk")]
    pub direct_boot_ok: Option<bool>,
    /// How long (in seconds) the message should be kept in FCM storage if the
    /// device is offline. The maximum time to live supported is 4 weeks, and the
    /// default value is 4 weeks if not set. Set it to 0 if want to send the
    /// message immediately.
    /// In JSON format, the Duration type is encoded as a string rather than an
    /// object, where the string ends in the suffix "s" (indicating seconds) and
    /// is preceded by the number of seconds, with nanoseconds expressed as
    /// fractional seconds. For example, 3 seconds with 0 nanoseconds should be
    /// encoded in JSON format as "3s", while 3 seconds and 1 nanosecond should
    /// be expressed in JSON format as "3.000000001s". The ttl will be rounded down
    /// to the nearest second.
    pub ttl: Option<String>,
    /// Package name of the application where the registration token must match in
    /// order to receive the message.
    #[serde(rename="restrictedPackageName")]
    pub restricted_package_name: Option<String>,
    /// Arbitrary key/value payload. If present, it will override
    /// google.firebase.fcm.v1.Message.data.
    pub data: Option<HashMap<String, String>>,
}

impl Part for AndroidConfig {}


/// Settings to control notification LED.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LightSettings {
    /// Required. Set `color` of the LED with
    /// [google.type.Color](https://github.com/googleapis/googleapis/blob/master/google/type/color.proto).
    pub color: Option<Color>,
    /// Required. Along with `light_off_duration`, define the blink rate of LED
    /// flashes. Resolution defined by
    /// [proto.Duration](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Duration)
    #[serde(rename="lightOnDuration")]
    pub light_on_duration: Option<String>,
    /// Required. Along with `light_on_duration `, define the blink rate of LED
    /// flashes. Resolution defined by
    /// [proto.Duration](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Duration)
    #[serde(rename="lightOffDuration")]
    pub light_off_duration: Option<String>,
}

impl Part for LightSettings {}


/// [Apple Push Notification Service](https://goo.gl/MXRTPa) specific options.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApnsConfig {
    /// HTTP request headers defined in Apple Push Notification Service. Refer to
    /// [APNs request
    /// headers](https://developer.apple.com/documentation/usernotifications/setting_up_a_remote_notification_server/sending_notification_requests_to_apns)
    /// for supported headers, e.g. "apns-priority": "10".
    pub headers: Option<HashMap<String, String>>,
    /// APNs payload as a JSON object, including both `aps` dictionary and custom
    /// payload. See [Payload Key
    /// Reference](https://developer.apple.com/documentation/usernotifications/setting_up_a_remote_notification_server/generating_a_remote_notification).
    /// If present, it overrides google.firebase.fcm.v1.Notification.title
    /// and google.firebase.fcm.v1.Notification.body.
    pub payload: Option<HashMap<String, String>>,
    /// Options for features provided by the FCM SDK for iOS.
    #[serde(rename="fcmOptions")]
    pub fcm_options: Option<ApnsFcmOptions>,
}

impl Part for ApnsConfig {}


/// Options for features provided by the FCM SDK for Android.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AndroidFcmOptions {
    /// Label associated with the message's analytics data.
    #[serde(rename="analyticsLabel")]
    pub analytics_label: Option<String>,
}

impl Part for AndroidFcmOptions {}


/// Options for features provided by the FCM SDK for iOS.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApnsFcmOptions {
    /// Contains the URL of an image that is going to be displayed in a
    /// notification. If present, it will override
    /// google.firebase.fcm.v1.Notification.image.
    pub image: Option<String>,
    /// Label associated with the message's analytics data.
    #[serde(rename="analyticsLabel")]
    pub analytics_label: Option<String>,
}

impl Part for ApnsFcmOptions {}


/// [Webpush protocol](https://tools.ietf.org/html/rfc8030) options.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WebpushConfig {
    /// HTTP headers defined in webpush protocol. Refer to
    /// [Webpush protocol](https://tools.ietf.org/html/rfc8030#section-5) for
    /// supported headers, e.g. "TTL": "15".
    pub headers: Option<HashMap<String, String>>,
    /// Arbitrary key/value payload. If present, it will override
    /// google.firebase.fcm.v1.Message.data.
    pub data: Option<HashMap<String, String>>,
    /// Options for features provided by the FCM SDK for Web.
    #[serde(rename="fcmOptions")]
    pub fcm_options: Option<WebpushFcmOptions>,
    /// Web Notification options as a JSON object. Supports Notification instance
    /// properties as defined in [Web Notification
    /// API](https://developer.mozilla.org/en-US/docs/Web/API/Notification). If
    /// present, "title" and "body" fields override
    /// [google.firebase.fcm.v1.Notification.title] and
    /// [google.firebase.fcm.v1.Notification.body].
    pub notification: Option<HashMap<String, String>>,
}

impl Part for WebpushConfig {}


/// Message to send by Firebase Cloud Messaging Service.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [messages send projects](struct.ProjectMessageSendCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    /// Output Only. The identifier of the message sent, in the format of
    /// `projects/*/messages/{message_id}`.
    pub name: Option<String>,
    /// Input only. Template for FCM SDK feature options to use across all
    /// platforms.
    #[serde(rename="fcmOptions")]
    pub fcm_options: Option<FcmOptions>,
    /// Input only. Basic notification template to use across all platforms.
    pub notification: Option<Notification>,
    /// Input only. [Webpush protocol](https://tools.ietf.org/html/rfc8030)
    /// options.
    pub webpush: Option<WebpushConfig>,
    /// Topic name to send a message to, e.g. "weather".
    /// Note: "/topics/" prefix should not be provided.
    pub topic: Option<String>,
    /// Registration token to send a message to.
    pub token: Option<String>,
    /// Input only. Android specific options for messages sent through
    /// [FCM connection server](https://goo.gl/4GLdUl).
    pub android: Option<AndroidConfig>,
    /// Input only. Arbitrary key/value payload. The key should not be a reserved
    /// word ("from", "message_type", or any word starting with "google" or "gcm").
    pub data: Option<HashMap<String, String>>,
    /// Input only. [Apple Push Notification Service](https://goo.gl/MXRTPa)
    /// specific options.
    pub apns: Option<ApnsConfig>,
    /// Condition to send a message to,
    /// e.g. "'foo' in topics && 'bar' in topics".
    pub condition: Option<String>,
}

impl ResponseResult for Message {}


/// Notification to send to android devices.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AndroidNotification {
    /// The notification's body text. If present, it will override
    /// google.firebase.fcm.v1.Notification.body.
    pub body: Option<String>,
    /// The key to the body string in the app's string resources to use to localize
    /// the body text to the user's current localization.
    /// See [String Resources](https://goo.gl/NdFZGI) for more information.
    #[serde(rename="bodyLocKey")]
    pub body_loc_key: Option<String>,
    /// The notification's icon color, expressed in #rrggbb format.
    pub color: Option<String>,
    /// Contains the URL of an image that is going to be displayed in a
    /// notification. If present, it will override
    /// google.firebase.fcm.v1.Notification.image.
    pub image: Option<String>,
    /// The [notification's channel
    /// id](https://developer.android.com/guide/topics/ui/notifiers/notifications#ManageChannels)
    /// (new in Android O). The app must create a channel with this channel ID
    /// before any notification with this channel ID is received. If you don't send
    /// this channel ID in the request, or if the channel ID provided has not yet
    /// been created by the app, FCM uses the channel ID specified in the app
    /// manifest.
    #[serde(rename="channelId")]
    pub channel_id: Option<String>,
    /// Variable string values to be used in place of the format specifiers in
    /// body_loc_key to use to localize the body text to the user's current
    /// localization.
    /// See [Formatting and Styling](https://goo.gl/MalYE3) for more information.
    #[serde(rename="bodyLocArgs")]
    pub body_loc_args: Option<Vec<String>>,
    /// Settings to control the notification's LED blinking rate and color if LED
    /// is available on the device. The total blinking time is controlled by the
    /// OS.
    #[serde(rename="lightSettings")]
    pub light_settings: Option<LightSettings>,
    /// When set to false or unset, the notification is automatically
    /// dismissed when the user clicks it in the panel. When set to true, the
    /// notification persists even when the user clicks it.
    pub sticky: Option<bool>,
    /// Identifier used to replace existing notifications in the notification
    /// drawer.
    /// If not specified, each request creates a new notification.
    /// If specified and a notification with the same tag is already being shown,
    /// the new notification replaces the existing one in the notification drawer.
    pub tag: Option<String>,
    /// Set the
    /// [Notification.visibility](https://developer.android.com/reference/android/app/Notification.html#visibility)
    /// of the notification.
    pub visibility: Option<String>,
    /// The key to the title string in the app's string resources to use to
    /// localize the title text to the user's current localization.
    /// See [String Resources](https://goo.gl/NdFZGI) for more information.
    #[serde(rename="titleLocKey")]
    pub title_loc_key: Option<String>,
    /// If set to true, use the Android framework's default sound for the
    /// notification. Default values are specified in
    /// [config.xml](https://android.googlesource.com/platform/frameworks/base/+/master/core/res/res/values/config.xml).
    #[serde(rename="defaultSound")]
    pub default_sound: Option<bool>,
    /// If set to true, use the Android framework's default LED light settings for
    /// the notification. Default values are specified in
    /// [config.xml](https://android.googlesource.com/platform/frameworks/base/+/master/core/res/res/values/config.xml).
    /// If `default_light_settings` is set to true and `light_settings` is also
    /// set, the user-specified `light_settings` is used instead of the
    /// default value.
    #[serde(rename="defaultLightSettings")]
    pub default_light_settings: Option<bool>,
    /// Sets the "ticker" text, which is sent to accessibility services.
    /// Prior to API level 21 (`Lollipop`), sets the text that is displayed in the
    /// status bar when the notification first arrives.
    pub ticker: Option<String>,
    /// The notification's icon.
    /// Sets the notification icon to myicon for drawable resource myicon.
    /// If you don't send this key in the request, FCM displays the launcher icon
    /// specified in your app manifest.
    pub icon: Option<String>,
    /// The sound to play when the device receives the notification.
    /// Supports "default" or the filename of a sound resource bundled in the app.
    /// Sound files must reside in /res/raw/.
    pub sound: Option<String>,
    /// Set the time that the event in the notification occurred. Notifications in
    /// the panel are sorted by this time. A point in time is represented using
    /// [protobuf.Timestamp](https://developers.google.com/protocol-buffers/docs/reference/java/com/google/protobuf/Timestamp).
    #[serde(rename="eventTime")]
    pub event_time: Option<String>,
    /// The notification's title. If present, it will override
    /// google.firebase.fcm.v1.Notification.title.
    pub title: Option<String>,
    /// Set whether or not this notification is relevant only to the current
    /// device. Some notifications can be bridged to other devices for remote
    /// display, such as a Wear OS watch. This hint can be set to recommend this
    /// notification not be bridged. See [Wear OS
    /// guides](https://developer.android.com/training/wearables/notifications/bridger#existing-method-of-preventing-bridging)
    #[serde(rename="localOnly")]
    pub local_only: Option<bool>,
    /// The action associated with a user click on the notification.
    /// If specified, an activity with a matching intent filter is launched when
    /// a user clicks on the notification.
    #[serde(rename="clickAction")]
    pub click_action: Option<String>,
    /// Sets the number of items this notification represents. May be displayed as
    /// a badge count for launchers that support badging.See [Notification
    /// Badge](https://developer.android.com/training/notify-user/badges).
    /// For example, this might be useful if you're using just one notification to
    /// represent multiple new messages but you want the count here to represent
    /// the number of total new messages.
    /// If zero or unspecified, systems that support badging use the default, which
    /// is to increment a number displayed on the long-press menu each time a new
    /// notification arrives.
    #[serde(rename="notificationCount")]
    pub notification_count: Option<i32>,
    /// Set the vibration pattern to use. Pass in an array of
    /// [protobuf.Duration](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Duration)
    /// to turn on or off the vibrator. The first value indicates the `Duration` to
    /// wait before turning the vibrator on. The next value indicates the
    /// `Duration` to keep the vibrator on. Subsequent values alternate between
    /// `Duration` to turn the vibrator off and to turn the vibrator on.
    /// If `vibrate_timings` is set and `default_vibrate_timings` is set to `true`,
    /// the default value is used instead of the user-specified `vibrate_timings`.
    #[serde(rename="vibrateTimings")]
    pub vibrate_timings: Option<Vec<String>>,
    /// Set the relative priority for this notification. Priority is an indication
    /// of how much of the user's attention should be consumed by this
    /// notification. Low-priority notifications may be hidden from the user in
    /// certain situations, while the user might be interrupted for a
    /// higher-priority notification. The effect of setting the same priorities may
    /// differ slightly on different platforms. Note this priority differs from
    /// `AndroidMessagePriority`. This priority is processed by the client after
    /// the message has been delivered, whereas
    /// [AndroidMessagePriority](https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#androidmessagepriority)
    /// is an FCM concept that controls when the message is delivered.
    #[serde(rename="notificationPriority")]
    pub notification_priority: Option<String>,
    /// If set to true, use the Android framework's default vibrate pattern for the
    /// notification. Default values are specified in
    /// [config.xml](https://android.googlesource.com/platform/frameworks/base/+/master/core/res/res/values/config.xml).
    /// If `default_vibrate_timings` is set to true and `vibrate_timings` is also
    /// set, the default value is used instead of the user-specified
    /// `vibrate_timings`.
    #[serde(rename="defaultVibrateTimings")]
    pub default_vibrate_timings: Option<bool>,
    /// Variable string values to be used in place of the format specifiers in
    /// title_loc_key to use to localize the title text to the user's current
    /// localization.
    /// See [Formatting and Styling](https://goo.gl/MalYE3) for more information.
    #[serde(rename="titleLocArgs")]
    pub title_loc_args: Option<Vec<String>>,
}

impl Part for AndroidNotification {}


/// Request to send a message to specified target.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [messages send projects](struct.ProjectMessageSendCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SendMessageRequest {
    /// Required. Message to send.
    pub message: Option<Message>,
    /// Flag for testing the request without actually delivering the message.
    #[serde(rename="validateOnly")]
    pub validate_only: Option<bool>,
}

impl RequestValue for SendMessageRequest {}


/// Options for features provided by the FCM SDK for Web.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WebpushFcmOptions {
    /// Label associated with the message's analytics data.
    #[serde(rename="analyticsLabel")]
    pub analytics_label: Option<String>,
    /// The link to open when the user clicks on the notification.
    /// For all URL values, HTTPS is required.
    pub link: Option<String>,
}

impl Part for WebpushFcmOptions {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the `FirebaseCloudMessaging` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_fcm1 as fcm1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use fcm1::FirebaseCloudMessaging;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = FirebaseCloudMessaging::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `messages_send(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a FirebaseCloudMessaging<C, A>,
}

impl<'a, C, A> MethodsBuilder for ProjectMethods<'a, C, A> {}

impl<'a, C, A> ProjectMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Send a message to specified target (a registration token, topic
    /// or condition).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. It contains the Firebase project id (i.e. the unique identifier
    ///              for your Firebase project), in the format of `projects/{project_id}`.
    ///              For legacy support, the numeric project number with no padding is also
    ///              supported in the format of `projects/{project_number}`.
    pub fn messages_send(&self, request: SendMessageRequest, parent: &str) -> ProjectMessageSendCall<'a, C, A> {
        ProjectMessageSendCall {
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

/// Send a message to specified target (a registration token, topic
/// or condition).
///
/// A builder for the *messages.send* method supported by a *project* resource.
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
/// # extern crate google_fcm1 as fcm1;
/// use fcm1::SendMessageRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use fcm1::FirebaseCloudMessaging;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = FirebaseCloudMessaging::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = SendMessageRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().messages_send(req, "parent")
///              .doit();
/// # }
/// ```
pub struct ProjectMessageSendCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a FirebaseCloudMessaging<C, A>,
    _request: SendMessageRequest,
    _parent: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectMessageSendCall<'a, C, A> {}

impl<'a, C, A> ProjectMessageSendCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Message)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "fcm.projects.messages.send",
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

        let mut url = self.hub._base_url.clone() + "v1/{+parent}/messages:send";
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
    pub fn request(mut self, new_value: SendMessageRequest) -> ProjectMessageSendCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Required. It contains the Firebase project id (i.e. the unique identifier
    /// for your Firebase project), in the format of `projects/{project_id}`.
    /// For legacy support, the numeric project number with no padding is also
    /// supported in the format of `projects/{project_number}`.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> ProjectMessageSendCall<'a, C, A> {
        self._parent = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ProjectMessageSendCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectMessageSendCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectMessageSendCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


