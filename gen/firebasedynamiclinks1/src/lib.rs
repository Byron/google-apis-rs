// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Firebase Dynamic Links* crate version *1.0.10+20190628*, where *20190628* is the exact revision of the *firebasedynamiclinks:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.10*.
//! 
//! Everything else about the *Firebase Dynamic Links* *v1* API can be found at the
//! [official documentation site](https://firebase.google.com/docs/dynamic-links/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/firebasedynamiclinks1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.FirebaseDynamicLinks.html) ... 
//! 
//! * [managed short links](struct.ManagedShortLink.html)
//!  * [*create*](struct.ManagedShortLinkCreateCall.html)
//! * short links
//!  * [*create*](struct.ShortLinkCreateCall.html)
//! 
//! Other activities are ...
//! 
//! * [get link stats](struct.MethodGetLinkStatCall.html)
//! * [install attribution](struct.MethodInstallAttributionCall.html)
//! * [reopen attribution](struct.MethodReopenAttributionCall.html)
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.FirebaseDynamicLinks.html)**
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
//! let r = hub.methods().install_attribution(...).doit()
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
//! google-firebasedynamiclinks1 = "*"
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
//! extern crate google_firebasedynamiclinks1 as firebasedynamiclinks1;
//! use firebasedynamiclinks1::GetIosPostInstallAttributionRequest;
//! use firebasedynamiclinks1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use firebasedynamiclinks1::FirebaseDynamicLinks;
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
//! let mut hub = FirebaseDynamicLinks::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = GetIosPostInstallAttributionRequest::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.methods().install_attribution(req)
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
    /// View and administer all your Firebase data and settings
    Firebase,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Firebase => "https://www.googleapis.com/auth/firebase",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Firebase
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all FirebaseDynamicLinks related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_firebasedynamiclinks1 as firebasedynamiclinks1;
/// use firebasedynamiclinks1::GetIosPostInstallAttributionRequest;
/// use firebasedynamiclinks1::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use firebasedynamiclinks1::FirebaseDynamicLinks;
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
/// let mut hub = FirebaseDynamicLinks::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GetIosPostInstallAttributionRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.methods().install_attribution(req)
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
pub struct FirebaseDynamicLinks<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for FirebaseDynamicLinks<C, A> {}

impl<'a, C, A> FirebaseDynamicLinks<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> FirebaseDynamicLinks<C, A> {
        FirebaseDynamicLinks {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.10".to_string(),
            _base_url: "https://firebasedynamiclinks.googleapis.com/".to_string(),
            _root_url: "https://firebasedynamiclinks.googleapis.com/".to_string(),
        }
    }

    pub fn managed_short_links(&'a self) -> ManagedShortLinkMethods<'a, C, A> {
        ManagedShortLinkMethods { hub: &self }
    }
    pub fn methods(&'a self) -> MethodMethods<'a, C, A> {
        MethodMethods { hub: &self }
    }
    pub fn short_links(&'a self) -> ShortLinkMethods<'a, C, A> {
        ShortLinkMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.10`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://firebasedynamiclinks.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://firebasedynamiclinks.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// Information of navigation behavior.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NavigationInfo {
    /// If this option is on, FDL click will be forced to redirect rather than
    /// show an interstitial page.
    #[serde(rename="enableForcedRedirect")]
    pub enable_forced_redirect: Option<bool>,
}

impl Part for NavigationInfo {}


/// Response for iSDK to execute strong match flow for post-install attribution.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [install attribution](struct.MethodInstallAttributionCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetIosPostInstallAttributionResponse {
    /// Describes why match failed, ie: "discarded due to low confidence".
    /// This message will be publicly visible.
    #[serde(rename="matchMessage")]
    pub match_message: Option<String>,
    /// The confidence of the returned attribution.
    #[serde(rename="attributionConfidence")]
    pub attribution_confidence: Option<String>,
    /// Instruction for iSDK to attemmpt to perform strong match. For instance,
    /// if browser does not support/allow cookie or outside of support browsers,
    /// this will be false.
    #[serde(rename="isStrongMatchExecutable")]
    pub is_strong_match_executable: Option<bool>,
    /// Which IP version the request was made from.
    #[serde(rename="requestIpVersion")]
    pub request_ip_version: Option<String>,
    /// Scion term value to be propagated by iSDK to Scion at app-reopen.
    #[serde(rename="utmTerm")]
    pub utm_term: Option<String>,
    /// Invitation ID attributed post-install via one of several techniques
    /// (fingerprint, copy unique).
    #[serde(rename="invitationId")]
    pub invitation_id: Option<String>,
    /// The entire FDL, expanded from a short link. It is the same as the
    /// requested_link, if it is long. Parameters from this should not be
    /// used directly (ie: server can default utm_[campaign|medium|source]
    /// to a value when requested_link lack them, server determine the best
    /// fallback_link when requested_link specifies >1 fallback links).
    #[serde(rename="resolvedLink")]
    pub resolved_link: Option<String>,
    /// Entire FDL (short or long) attributed post-install via one of several
    /// techniques (fingerprint, copy unique).
    #[serde(rename="requestedLink")]
    pub requested_link: Option<String>,
    /// Scion medium value to be propagated by iSDK to Scion at post-install.
    #[serde(rename="utmMedium")]
    pub utm_medium: Option<String>,
    /// Scion source value to be propagated by iSDK to Scion at post-install.
    #[serde(rename="utmSource")]
    pub utm_source: Option<String>,
    /// The minimum version for app, specified by dev through ?imv= parameter.
    /// Return to iSDK to allow app to evaluate if current version meets this.
    #[serde(rename="appMinimumVersion")]
    pub app_minimum_version: Option<String>,
    /// User-agent specific custom-scheme URIs for iSDK to open. This will be set
    /// according to the user-agent tha the click was originally made in. There is
    /// no Safari-equivalent custom-scheme open URLs.
    /// ie: googlechrome://www.example.com
    /// ie: firefox://open-url?url=http://www.example.com
    /// ie: opera-http://example.com
    #[serde(rename="externalBrowserDestinationLink")]
    pub external_browser_destination_link: Option<String>,
    /// Scion content value to be propagated by iSDK to Scion at app-reopen.
    #[serde(rename="utmContent")]
    pub utm_content: Option<String>,
    /// The link to navigate to update the app if min version is not met.
    /// This is either (in order): 1) fallback link (from ?ifl= parameter, if
    /// specified by developer) or 2) AppStore URL (from ?isi= parameter, if
    /// specified), or 3) the payload link (from required link= parameter).
    #[serde(rename="fallbackLink")]
    pub fallback_link: Option<String>,
    /// Scion campaign value to be propagated by iSDK to Scion at post-install.
    #[serde(rename="utmCampaign")]
    pub utm_campaign: Option<String>,
    /// The deep-link attributed post-install via one of several techniques
    /// (fingerprint, copy unique).
    #[serde(rename="deepLink")]
    pub deep_link: Option<String>,
}

impl ResponseResult for GetIosPostInstallAttributionResponse {}


/// Android related attributes to the Dynamic Link.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AndroidInfo {
    /// Link to open on Android if the app is not installed.
    #[serde(rename="androidFallbackLink")]
    pub android_fallback_link: Option<String>,
    /// Android package name of the app.
    #[serde(rename="androidPackageName")]
    pub android_package_name: Option<String>,
    /// If specified, this overrides the ‘link’ parameter on Android.
    #[serde(rename="androidLink")]
    pub android_link: Option<String>,
    /// Minimum version code for the Android app. If the installed app’s version
    /// code is lower, then the user is taken to the Play Store.
    #[serde(rename="androidMinPackageVersionCode")]
    pub android_min_package_version_code: Option<String>,
}

impl Part for AndroidInfo {}


/// Request to create a short Dynamic Link.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create short links](struct.ShortLinkCreateCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateShortDynamicLinkRequest {
    /// Information about the Dynamic Link to be shortened.
    /// [Learn
    /// more](https://firebase.google.com/docs/reference/dynamic-links/link-shortener).
    #[serde(rename="dynamicLinkInfo")]
    pub dynamic_link_info: Option<DynamicLinkInfo>,
    /// Full long Dynamic Link URL with desired query parameters specified.
    /// For example,
    /// "https://sample.app.goo.gl/?link=http://www.google.com&apn=com.sample",
    /// [Learn
    /// more](https://firebase.google.com/docs/reference/dynamic-links/link-shortener).
    #[serde(rename="longDynamicLink")]
    pub long_dynamic_link: Option<String>,
    /// Google SDK version. Version takes the form "$major.$minor.$patch"
    #[serde(rename="sdkVersion")]
    pub sdk_version: Option<String>,
    /// Short Dynamic Link suffix. Optional.
    pub suffix: Option<Suffix>,
}

impl RequestValue for CreateShortDynamicLinkRequest {}


/// Dynamic Link event stat.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DynamicLinkEventStat {
    /// The number of times this event occurred.
    pub count: Option<String>,
    /// Requested platform.
    pub platform: Option<String>,
    /// Link event.
    pub event: Option<String>,
}

impl Part for DynamicLinkEventStat {}


/// Request for iSDK to execute strong match flow for post-install attribution.
/// This is meant for iOS requests only. Requests from other platforms will
/// not be honored.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [install attribution](struct.MethodInstallAttributionCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetIosPostInstallAttributionRequest {
    /// Strong match page information. Disambiguates between default UI and
    /// custom page to present when strong match succeeds/fails to find cookie.
    #[serde(rename="visualStyle")]
    pub visual_style: Option<String>,
    /// Google SDK version. Version takes the form "$major.$minor.$patch"
    #[serde(rename="sdkVersion")]
    pub sdk_version: Option<String>,
    /// App installation epoch time (https://en.wikipedia.org/wiki/Unix_time).
    /// This is a client signal for a more accurate weak match.
    #[serde(rename="appInstallationTime")]
    pub app_installation_time: Option<String>,
    /// Possible unique matched link that server need to check before performing
    /// fingerprint match. If passed link is short server need to expand the link.
    /// If link is long server need to vslidate the link.
    #[serde(rename="uniqueMatchLinkToCheck")]
    pub unique_match_link_to_check: Option<String>,
    /// Device information.
    pub device: Option<DeviceInfo>,
    /// App post install attribution retrieval information. Disambiguates
    /// mechanism (iSDK or developer invoked) to retrieve payload from
    /// clicked link.
    #[serde(rename="retrievalMethod")]
    pub retrieval_method: Option<String>,
    /// iOS version, ie: 9.3.5.
    /// Consider adding "build".
    #[serde(rename="iosVersion")]
    pub ios_version: Option<String>,
    /// APP bundle ID.
    #[serde(rename="bundleId")]
    pub bundle_id: Option<String>,
}

impl RequestValue for GetIosPostInstallAttributionRequest {}


/// Information about a Dynamic Link.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DynamicLinkInfo {
    /// Information of navigation behavior of a Firebase Dynamic Links.
    #[serde(rename="navigationInfo")]
    pub navigation_info: Option<NavigationInfo>,
    /// Parameters used for tracking. See all tracking parameters in the
    /// [documentation](https://firebase.google.com/docs/dynamic-links/create-manually).
    #[serde(rename="analyticsInfo")]
    pub analytics_info: Option<AnalyticsInfo>,
    /// iOS related information. See iOS related parameters in the
    /// [documentation](https://firebase.google.com/docs/dynamic-links/create-manually).
    #[serde(rename="iosInfo")]
    pub ios_info: Option<IosInfo>,
    /// Android related information. See Android related parameters in the
    /// [documentation](https://firebase.google.com/docs/dynamic-links/create-manually).
    #[serde(rename="androidInfo")]
    pub android_info: Option<AndroidInfo>,
    /// Parameters for social meta tag params.
    /// Used to set meta tag data for link previews on social sites.
    #[serde(rename="socialMetaTagInfo")]
    pub social_meta_tag_info: Option<SocialMetaTagInfo>,
    /// E.g. https://maps.app.goo.gl, https://maps.page.link, https://g.co/maps
    /// More examples can be found in description of getNormalizedUriPrefix in
    /// j/c/g/firebase/dynamiclinks/uri/DdlDomain.java
    /// 
    /// Will fallback to dynamic_link_domain is this field is missing
    #[serde(rename="domainUriPrefix")]
    pub domain_uri_prefix: Option<String>,
    /// The link your app will open, You can specify any URL your app can handle.
    /// This link must be a well-formatted URL, be properly URL-encoded, and use
    /// the HTTP or HTTPS scheme. See 'link' parameters in the
    /// [documentation](https://firebase.google.com/docs/dynamic-links/create-manually).
    /// 
    /// Required.
    pub link: Option<String>,
    /// Desktop related information. See desktop related parameters in the
    /// [documentation](https://firebase.google.com/docs/dynamic-links/create-manually).
    #[serde(rename="desktopInfo")]
    pub desktop_info: Option<DesktopInfo>,
    /// Dynamic Links domain that the project owns, e.g. abcd.app.goo.gl
    /// [Learn
    /// more](https://firebase.google.com/docs/dynamic-links/android/receive) on
    /// how to set up Dynamic Link domain associated with your Firebase project.
    /// 
    /// Required if missing domain_uri_prefix.
    #[serde(rename="dynamicLinkDomain")]
    pub dynamic_link_domain: Option<String>,
}

impl Part for DynamicLinkInfo {}


/// Response to create a short Dynamic Link.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create managed short links](struct.ManagedShortLinkCreateCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateManagedShortLinkResponse {
    /// Information about potential warnings on link creation.
    pub warning: Option<Vec<DynamicLinkWarning>>,
    /// Short Dynamic Link value. e.g. https://abcd.app.goo.gl/wxyz
    #[serde(rename="managedShortLink")]
    pub managed_short_link: Option<ManagedShortLink>,
    /// Preview link to show the link flow chart. (debug info.)
    #[serde(rename="previewLink")]
    pub preview_link: Option<String>,
}

impl ResponseResult for CreateManagedShortLinkResponse {}


/// Tracking parameters supported by Dynamic Link.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyticsInfo {
    /// Google Play Campaign Measurements.
    #[serde(rename="googlePlayAnalytics")]
    pub google_play_analytics: Option<GooglePlayAnalytics>,
    /// iTunes Connect App Analytics.
    #[serde(rename="itunesConnectAnalytics")]
    pub itunes_connect_analytics: Option<ITunesConnectAnalytics>,
}

impl Part for AnalyticsInfo {}


/// Analytics stats of a Dynamic Link for a given timeframe.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get link stats](struct.MethodGetLinkStatCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DynamicLinkStats {
    /// Dynamic Link event stats.
    #[serde(rename="linkEventStats")]
    pub link_event_stats: Option<Vec<DynamicLinkEventStat>>,
}

impl ResponseResult for DynamicLinkStats {}


/// Short Dynamic Link suffix.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Suffix {
    /// Suffix option.
    pub option: Option<String>,
    /// Only applies to Option.CUSTOM.
    #[serde(rename="customSuffix")]
    pub custom_suffix: Option<String>,
}

impl Part for Suffix {}


/// Dynamic Links warning messages.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DynamicLinkWarning {
    /// The warning code.
    #[serde(rename="warningCode")]
    pub warning_code: Option<String>,
    /// The document describing the warning, and helps resolve.
    #[serde(rename="warningDocumentLink")]
    pub warning_document_link: Option<String>,
    /// The warning message to help developers improve their requests.
    #[serde(rename="warningMessage")]
    pub warning_message: Option<String>,
}

impl Part for DynamicLinkWarning {}


/// iOS related attributes to the Dynamic Link..
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IosInfo {
    /// Custom (destination) scheme to use for iOS. By default, we’ll use the
    /// bundle ID as the custom scheme. Developer can override this behavior using
    /// this param.
    #[serde(rename="iosCustomScheme")]
    pub ios_custom_scheme: Option<String>,
    /// Link to open on iOS if the app is not installed.
    #[serde(rename="iosFallbackLink")]
    pub ios_fallback_link: Option<String>,
    /// iOS bundle ID of the app.
    #[serde(rename="iosBundleId")]
    pub ios_bundle_id: Option<String>,
    /// If specified, this overrides the ios_fallback_link value on iPads.
    #[serde(rename="iosIpadFallbackLink")]
    pub ios_ipad_fallback_link: Option<String>,
    /// iOS minimum version.
    #[serde(rename="iosMinimumVersion")]
    pub ios_minimum_version: Option<String>,
    /// iPad bundle ID of the app.
    #[serde(rename="iosIpadBundleId")]
    pub ios_ipad_bundle_id: Option<String>,
    /// iOS App Store ID.
    #[serde(rename="iosAppStoreId")]
    pub ios_app_store_id: Option<String>,
}

impl Part for IosInfo {}


/// Parameters for social meta tag params.
/// Used to set meta tag data for link previews on social sites.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SocialMetaTagInfo {
    /// Title to be displayed. Optional.
    #[serde(rename="socialTitle")]
    pub social_title: Option<String>,
    /// A short description of the link. Optional.
    #[serde(rename="socialDescription")]
    pub social_description: Option<String>,
    /// An image url string. Optional.
    #[serde(rename="socialImageLink")]
    pub social_image_link: Option<String>,
}

impl Part for SocialMetaTagInfo {}


/// Signals associated with the device making the request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// Device language code setting.
    #[serde(rename="languageCode")]
    pub language_code: Option<String>,
    /// Device language code raw setting.
    /// iOS does returns language code in different format than iOS WebView.
    /// For example WebView returns en_US, but iOS returns en-US.
    /// Field below will return raw value returned by iOS.
    #[serde(rename="languageCodeRaw")]
    pub language_code_raw: Option<String>,
    /// Device model name.
    #[serde(rename="deviceModelName")]
    pub device_model_name: Option<String>,
    /// Device language code setting obtained by executing JavaScript code in
    /// WebView.
    #[serde(rename="languageCodeFromWebview")]
    pub language_code_from_webview: Option<String>,
    /// Device display resolution height.
    #[serde(rename="screenResolutionHeight")]
    pub screen_resolution_height: Option<String>,
    /// Device timezone setting.
    pub timezone: Option<String>,
    /// Device display resolution width.
    #[serde(rename="screenResolutionWidth")]
    pub screen_resolution_width: Option<String>,
}

impl Part for DeviceInfo {}


/// Parameters for iTunes Connect App Analytics.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ITunesConnectAnalytics {
    /// iTune media types, including music, podcasts, audiobooks and so on.
    pub mt: Option<String>,
    /// Affiliate token used to create affiliate-coded links.
    pub at: Option<String>,
    /// Provider token that enables analytics for Dynamic Links from within iTunes
    /// Connect.
    pub pt: Option<String>,
    /// Campaign text that developers can optionally add to any link in order to
    /// track sales from a specific marketing campaign.
    pub ct: Option<String>,
}

impl Part for ITunesConnectAnalytics {}


/// Request to create a managed Short Dynamic Link.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create managed short links](struct.ManagedShortLinkCreateCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateManagedShortLinkRequest {
    /// Information about the Dynamic Link to be shortened.
    /// [Learn
    /// more](https://firebase.google.com/docs/reference/dynamic-links/link-shortener).
    #[serde(rename="dynamicLinkInfo")]
    pub dynamic_link_info: Option<DynamicLinkInfo>,
    /// Full long Dynamic Link URL with desired query parameters specified.
    /// For example,
    /// "https://sample.app.goo.gl/?link=http://www.google.com&apn=com.sample",
    /// [Learn
    /// more](https://firebase.google.com/docs/reference/dynamic-links/link-shortener).
    #[serde(rename="longDynamicLink")]
    pub long_dynamic_link: Option<String>,
    /// Google SDK version. Version takes the form "$major.$minor.$patch"
    #[serde(rename="sdkVersion")]
    pub sdk_version: Option<String>,
    /// Link name to associate with the link. It's used for marketer to identify
    /// manually-created links in the Firebase console
    /// (https://console.firebase.google.com/).
    /// Links must be named to be tracked.
    pub name: Option<String>,
    /// Short Dynamic Link suffix. Optional.
    pub suffix: Option<Suffix>,
}

impl RequestValue for CreateManagedShortLinkRequest {}


/// Response for iSDK to get reopen attribution for app universal link open
/// deeplinking. This endpoint is meant for only iOS requests.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reopen attribution](struct.MethodReopenAttributionCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetIosReopenAttributionResponse {
    /// Scion medium value to be propagated by iSDK to Scion at app-reopen.
    #[serde(rename="utmMedium")]
    pub utm_medium: Option<String>,
    /// Scion source value to be propagated by iSDK to Scion at app-reopen.
    #[serde(rename="utmSource")]
    pub utm_source: Option<String>,
    /// FDL input value of the "&imv=" parameter, minimum app version to be
    /// returned to Google Firebase SDK running on iOS-9.
    #[serde(rename="iosMinAppVersion")]
    pub ios_min_app_version: Option<String>,
    /// Scion term value to be propagated by iSDK to Scion at app-reopen.
    #[serde(rename="utmTerm")]
    pub utm_term: Option<String>,
    /// Scion content value to be propagated by iSDK to Scion at app-reopen.
    #[serde(rename="utmContent")]
    pub utm_content: Option<String>,
    /// Optional invitation ID, for only invite typed requested FDL links.
    #[serde(rename="invitationId")]
    pub invitation_id: Option<String>,
    /// The entire FDL, expanded from a short link. It is the same as the
    /// requested_link, if it is long.
    #[serde(rename="resolvedLink")]
    pub resolved_link: Option<String>,
    /// Scion campaign value to be propagated by iSDK to Scion at app-reopen.
    #[serde(rename="utmCampaign")]
    pub utm_campaign: Option<String>,
    /// The deep-link attributed the app universal link open. For both regular
    /// FDL links and invite FDL links.
    #[serde(rename="deepLink")]
    pub deep_link: Option<String>,
}

impl ResponseResult for GetIosReopenAttributionResponse {}


/// Parameters for Google Play Campaign Measurements.
/// [Learn
/// more](https://developers.google.com/analytics/devguides/collection/android/v4/campaigns#campaign-params)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePlayAnalytics {
    /// Campaign medium; used to identify a medium such as email or cost-per-click.
    #[serde(rename="utmMedium")]
    pub utm_medium: Option<String>,
    /// Campaign content; used for A/B testing and content-targeted ads to
    /// differentiate ads or links that point to the same URL.
    #[serde(rename="utmContent")]
    pub utm_content: Option<String>,
    /// Campaign source; used to identify a search engine, newsletter, or other
    /// source.
    #[serde(rename="utmSource")]
    pub utm_source: Option<String>,
    /// [AdWords autotagging
    /// parameter](https://support.google.com/analytics/answer/1033981?hl=en); used
    /// to measure Google AdWords ads. This value is generated dynamically and
    /// should never be modified.
    pub gclid: Option<String>,
    /// Campaign name; used for keyword analysis to identify a specific product
    /// promotion or strategic campaign.
    #[serde(rename="utmCampaign")]
    pub utm_campaign: Option<String>,
    /// Campaign term; used with paid search to supply the keywords for ads.
    #[serde(rename="utmTerm")]
    pub utm_term: Option<String>,
}

impl Part for GooglePlayAnalytics {}


/// Managed Short Link.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create managed short links](struct.ManagedShortLinkCreateCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManagedShortLink {
    /// Full Dyamic Link info
    pub info: Option<DynamicLinkInfo>,
    /// Short durable link url, for example, "https://sample.app.goo.gl/xyz123".
    /// 
    /// Required.
    pub link: Option<String>,
    /// Attributes that have been flagged about this short url.
    #[serde(rename="flaggedAttribute")]
    pub flagged_attribute: Option<Vec<String>>,
    /// Link name defined by the creator.
    /// 
    /// Required.
    #[serde(rename="linkName")]
    pub link_name: Option<String>,
    /// Creation timestamp of the short link.
    #[serde(rename="creationTime")]
    pub creation_time: Option<String>,
    /// Visibility status of link.
    pub visibility: Option<String>,
}

impl Resource for ManagedShortLink {}


/// Desktop related attributes to the Dynamic Link.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DesktopInfo {
    /// Link to open on desktop.
    #[serde(rename="desktopFallbackLink")]
    pub desktop_fallback_link: Option<String>,
}

impl Part for DesktopInfo {}


/// Response to create a short Dynamic Link.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create short links](struct.ShortLinkCreateCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateShortDynamicLinkResponse {
    /// Preview link to show the link flow chart. (debug info.)
    #[serde(rename="previewLink")]
    pub preview_link: Option<String>,
    /// Information about potential warnings on link creation.
    pub warning: Option<Vec<DynamicLinkWarning>>,
    /// Short Dynamic Link value. e.g. https://abcd.app.goo.gl/wxyz
    #[serde(rename="shortLink")]
    pub short_link: Option<String>,
}

impl ResponseResult for CreateShortDynamicLinkResponse {}


/// Request for iSDK to get reopen attribution for app universal link open
/// deeplinking. This endpoint is meant for only iOS requests.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reopen attribution](struct.MethodReopenAttributionCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetIosReopenAttributionRequest {
    /// Google SDK version. Version takes the form "$major.$minor.$patch"
    #[serde(rename="sdkVersion")]
    pub sdk_version: Option<String>,
    /// FDL link to be verified from an app universal link open.
    /// The FDL link can be one of:
    /// 1) short FDL.
    /// e.g. <app_code>.page.link/<ddl_id>, or
    /// 2) long FDL.
    /// e.g. <app_code>.page.link/?{query params}, or
    /// 3) Invite FDL.
    /// e.g. <app_code>.page.link/i/<invite_id_or_alias>
    #[serde(rename="requestedLink")]
    pub requested_link: Option<String>,
    /// APP bundle ID.
    #[serde(rename="bundleId")]
    pub bundle_id: Option<String>,
}

impl RequestValue for GetIosReopenAttributionRequest {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *shortLink* resources.
/// It is not used directly, but through the `FirebaseDynamicLinks` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_firebasedynamiclinks1 as firebasedynamiclinks1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use firebasedynamiclinks1::FirebaseDynamicLinks;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = FirebaseDynamicLinks::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`
/// // to build up your call.
/// let rb = hub.short_links();
/// # }
/// ```
pub struct ShortLinkMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a FirebaseDynamicLinks<C, A>,
}

impl<'a, C, A> MethodsBuilder for ShortLinkMethods<'a, C, A> {}

impl<'a, C, A> ShortLinkMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a short Dynamic Link given either a valid long Dynamic Link or
    /// details such as Dynamic Link domain, Android and iOS app information.
    /// The created short Dynamic Link will not expire.
    /// 
    /// Repeated calls with the same long Dynamic Link or Dynamic Link information
    /// will produce the same short Dynamic Link.
    /// 
    /// The Dynamic Link domain in the request must be owned by requester's
    /// Firebase project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: CreateShortDynamicLinkRequest) -> ShortLinkCreateCall<'a, C, A> {
        ShortLinkCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all free methods, which are not associated with a particular resource.
/// It is not used directly, but through the `FirebaseDynamicLinks` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_firebasedynamiclinks1 as firebasedynamiclinks1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use firebasedynamiclinks1::FirebaseDynamicLinks;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = FirebaseDynamicLinks::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_link_stats(...)`, `install_attribution(...)` and `reopen_attribution(...)`
/// // to build up your call.
/// let rb = hub.methods();
/// # }
/// ```
pub struct MethodMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a FirebaseDynamicLinks<C, A>,
}

impl<'a, C, A> MethodsBuilder for MethodMethods<'a, C, A> {}

impl<'a, C, A> MethodMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Fetches analytics stats of a short Dynamic Link for a given
    /// duration. Metrics include number of clicks, redirects, installs,
    /// app first opens, and app reopens.
    /// 
    /// # Arguments
    ///
    /// * `dynamicLink` - Dynamic Link URL. e.g. https://abcd.app.goo.gl/wxyz
    pub fn get_link_stats(&self, dynamic_link: &str) -> MethodGetLinkStatCall<'a, C, A> {
        MethodGetLinkStatCall {
            hub: self.hub,
            _dynamic_link: dynamic_link.to_string(),
            _sdk_version: Default::default(),
            _duration_days: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get iOS strong/weak-match info for post-install attribution.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn install_attribution(&self, request: GetIosPostInstallAttributionRequest) -> MethodInstallAttributionCall<'a, C, A> {
        MethodInstallAttributionCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get iOS reopen attribution for app universal link open deeplinking.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn reopen_attribution(&self, request: GetIosReopenAttributionRequest) -> MethodReopenAttributionCall<'a, C, A> {
        MethodReopenAttributionCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *managedShortLink* resources.
/// It is not used directly, but through the `FirebaseDynamicLinks` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_firebasedynamiclinks1 as firebasedynamiclinks1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use firebasedynamiclinks1::FirebaseDynamicLinks;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = FirebaseDynamicLinks::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`
/// // to build up your call.
/// let rb = hub.managed_short_links();
/// # }
/// ```
pub struct ManagedShortLinkMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a FirebaseDynamicLinks<C, A>,
}

impl<'a, C, A> MethodsBuilder for ManagedShortLinkMethods<'a, C, A> {}

impl<'a, C, A> ManagedShortLinkMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a managed short Dynamic Link given either a valid long Dynamic Link
    /// or details such as Dynamic Link domain, Android and iOS app information.
    /// The created short Dynamic Link will not expire.
    /// 
    /// This differs from CreateShortDynamicLink in the following ways:
    /// 
    /// * The request will also contain a name for the link (non unique name
    ///   for the front end).
    /// * The response must be authenticated with an auth token (generated with
    ///   the admin service account).
    /// * The link will appear in the FDL list of links in the console front end.
    /// 
    /// The Dynamic Link domain in the request must be owned by requester's
    /// Firebase project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: CreateManagedShortLinkRequest) -> ManagedShortLinkCreateCall<'a, C, A> {
        ManagedShortLinkCreateCall {
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

/// Creates a short Dynamic Link given either a valid long Dynamic Link or
/// details such as Dynamic Link domain, Android and iOS app information.
/// The created short Dynamic Link will not expire.
/// 
/// Repeated calls with the same long Dynamic Link or Dynamic Link information
/// will produce the same short Dynamic Link.
/// 
/// The Dynamic Link domain in the request must be owned by requester's
/// Firebase project.
///
/// A builder for the *create* method supported by a *shortLink* resource.
/// It is not used directly, but through a `ShortLinkMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_firebasedynamiclinks1 as firebasedynamiclinks1;
/// use firebasedynamiclinks1::CreateShortDynamicLinkRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use firebasedynamiclinks1::FirebaseDynamicLinks;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = FirebaseDynamicLinks::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = CreateShortDynamicLinkRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.short_links().create(req)
///              .doit();
/// # }
/// ```
pub struct ShortLinkCreateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a FirebaseDynamicLinks<C, A>,
    _request: CreateShortDynamicLinkRequest,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ShortLinkCreateCall<'a, C, A> {}

impl<'a, C, A> ShortLinkCreateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, CreateShortDynamicLinkResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "firebasedynamiclinks.shortLinks.create",
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

        let mut url = self.hub._base_url.clone() + "v1/shortLinks";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Firebase.as_ref().to_string(), ());
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
    pub fn request(mut self, new_value: CreateShortDynamicLinkRequest) -> ShortLinkCreateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ShortLinkCreateCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ShortLinkCreateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Firebase`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ShortLinkCreateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Fetches analytics stats of a short Dynamic Link for a given
/// duration. Metrics include number of clicks, redirects, installs,
/// app first opens, and app reopens.
///
/// A builder for the *getLinkStats* method.
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
/// # extern crate google_firebasedynamiclinks1 as firebasedynamiclinks1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use firebasedynamiclinks1::FirebaseDynamicLinks;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = FirebaseDynamicLinks::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.methods().get_link_stats("dynamicLink")
///              .sdk_version("sit")
///              .duration_days("Stet")
///              .doit();
/// # }
/// ```
pub struct MethodGetLinkStatCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a FirebaseDynamicLinks<C, A>,
    _dynamic_link: String,
    _sdk_version: Option<String>,
    _duration_days: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for MethodGetLinkStatCall<'a, C, A> {}

impl<'a, C, A> MethodGetLinkStatCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, DynamicLinkStats)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "firebasedynamiclinks.getLinkStats",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("dynamicLink", self._dynamic_link.to_string()));
        if let Some(value) = self._sdk_version {
            params.push(("sdkVersion", value.to_string()));
        }
        if let Some(value) = self._duration_days {
            params.push(("durationDays", value.to_string()));
        }
        for &field in ["alt", "dynamicLink", "sdkVersion", "durationDays"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{dynamicLink}/linkStats";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Firebase.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{dynamicLink}", "dynamicLink")].iter() {
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
            for param_name in ["dynamicLink"].iter() {
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


    /// Dynamic Link URL. e.g. https://abcd.app.goo.gl/wxyz
    ///
    /// Sets the *dynamic link* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dynamic_link(mut self, new_value: &str) -> MethodGetLinkStatCall<'a, C, A> {
        self._dynamic_link = new_value.to_string();
        self
    }
    /// Google SDK version. Version takes the form "$major.$minor.$patch"
    ///
    /// Sets the *sdk version* query property to the given value.
    pub fn sdk_version(mut self, new_value: &str) -> MethodGetLinkStatCall<'a, C, A> {
        self._sdk_version = Some(new_value.to_string());
        self
    }
    /// The span of time requested in days.
    ///
    /// Sets the *duration days* query property to the given value.
    pub fn duration_days(mut self, new_value: &str) -> MethodGetLinkStatCall<'a, C, A> {
        self._duration_days = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MethodGetLinkStatCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> MethodGetLinkStatCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Firebase`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> MethodGetLinkStatCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Get iOS strong/weak-match info for post-install attribution.
///
/// A builder for the *installAttribution* method.
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
/// # extern crate google_firebasedynamiclinks1 as firebasedynamiclinks1;
/// use firebasedynamiclinks1::GetIosPostInstallAttributionRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use firebasedynamiclinks1::FirebaseDynamicLinks;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = FirebaseDynamicLinks::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GetIosPostInstallAttributionRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.methods().install_attribution(req)
///              .doit();
/// # }
/// ```
pub struct MethodInstallAttributionCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a FirebaseDynamicLinks<C, A>,
    _request: GetIosPostInstallAttributionRequest,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for MethodInstallAttributionCall<'a, C, A> {}

impl<'a, C, A> MethodInstallAttributionCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GetIosPostInstallAttributionResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "firebasedynamiclinks.installAttribution",
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

        let mut url = self.hub._base_url.clone() + "v1/installAttribution";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Firebase.as_ref().to_string(), ());
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
    pub fn request(mut self, new_value: GetIosPostInstallAttributionRequest) -> MethodInstallAttributionCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MethodInstallAttributionCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> MethodInstallAttributionCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Firebase`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> MethodInstallAttributionCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Get iOS reopen attribution for app universal link open deeplinking.
///
/// A builder for the *reopenAttribution* method.
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
/// # extern crate google_firebasedynamiclinks1 as firebasedynamiclinks1;
/// use firebasedynamiclinks1::GetIosReopenAttributionRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use firebasedynamiclinks1::FirebaseDynamicLinks;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = FirebaseDynamicLinks::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GetIosReopenAttributionRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.methods().reopen_attribution(req)
///              .doit();
/// # }
/// ```
pub struct MethodReopenAttributionCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a FirebaseDynamicLinks<C, A>,
    _request: GetIosReopenAttributionRequest,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for MethodReopenAttributionCall<'a, C, A> {}

impl<'a, C, A> MethodReopenAttributionCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GetIosReopenAttributionResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "firebasedynamiclinks.reopenAttribution",
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

        let mut url = self.hub._base_url.clone() + "v1/reopenAttribution";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Firebase.as_ref().to_string(), ());
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
    pub fn request(mut self, new_value: GetIosReopenAttributionRequest) -> MethodReopenAttributionCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MethodReopenAttributionCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> MethodReopenAttributionCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Firebase`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> MethodReopenAttributionCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a managed short Dynamic Link given either a valid long Dynamic Link
/// or details such as Dynamic Link domain, Android and iOS app information.
/// The created short Dynamic Link will not expire.
/// 
/// This differs from CreateShortDynamicLink in the following ways:
/// 
/// * The request will also contain a name for the link (non unique name
///   for the front end).
/// * The response must be authenticated with an auth token (generated with
///   the admin service account).
/// * The link will appear in the FDL list of links in the console front end.
/// 
/// The Dynamic Link domain in the request must be owned by requester's
/// Firebase project.
///
/// A builder for the *create* method supported by a *managedShortLink* resource.
/// It is not used directly, but through a `ManagedShortLinkMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_firebasedynamiclinks1 as firebasedynamiclinks1;
/// use firebasedynamiclinks1::CreateManagedShortLinkRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use firebasedynamiclinks1::FirebaseDynamicLinks;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = FirebaseDynamicLinks::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = CreateManagedShortLinkRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.managed_short_links().create(req)
///              .doit();
/// # }
/// ```
pub struct ManagedShortLinkCreateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a FirebaseDynamicLinks<C, A>,
    _request: CreateManagedShortLinkRequest,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ManagedShortLinkCreateCall<'a, C, A> {}

impl<'a, C, A> ManagedShortLinkCreateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, CreateManagedShortLinkResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "firebasedynamiclinks.managedShortLinks.create",
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

        let mut url = self.hub._base_url.clone() + "v1/managedShortLinks:create";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Firebase.as_ref().to_string(), ());
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
    pub fn request(mut self, new_value: CreateManagedShortLinkRequest) -> ManagedShortLinkCreateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ManagedShortLinkCreateCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ManagedShortLinkCreateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Firebase`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ManagedShortLinkCreateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


