// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Firebase Dynamic Links* crate version *1.0.5+20170517*, where *20170517* is the exact revision of the *firebasedynamiclinks:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.5*.
//! 
//! Everything else about the *Firebase Dynamic Links* *v1* API can be found at the
//! [official documentation site](https://firebase.google.com/docs/dynamic-links/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/firebasedynamiclinks1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.FirebaseDynamicLinks.html) ... 
//! 
//! * short links
//!  * [*create*](struct.ShortLinkCreateCall.html)
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
//! let r = hub.short_links().create(...).doit()
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
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_firebasedynamiclinks1 as firebasedynamiclinks1;
//! use firebasedynamiclinks1::CreateShortDynamicLinkRequest;
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
//! let mut req = CreateShortDynamicLinkRequest::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.short_links().create(req)
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
/// use firebasedynamiclinks1::CreateShortDynamicLinkRequest;
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
/// let mut req = CreateShortDynamicLinkRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.short_links().create(req)
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
            _user_agent: "google-api-rust-client/1.0.5".to_string(),
            _base_url: "https://firebasedynamiclinks.googleapis.com/".to_string(),
            _root_url: "https://firebasedynamiclinks.googleapis.com/".to_string(),
        }
    }

    pub fn short_links(&'a self) -> ShortLinkMethods<'a, C, A> {
        ShortLinkMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.5`.
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


/// Short Dynamic Link suffix.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Suffix {
    /// Suffix option.
    pub option: Option<String>,
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
    /// iPad bundle ID of the app.
    #[serde(rename="iosIpadBundleId")]
    pub ios_ipad_bundle_id: Option<String>,
    /// iOS App Store ID.
    #[serde(rename="iosAppStoreId")]
    pub ios_app_store_id: Option<String>,
}

impl Part for IosInfo {}


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
    /// [Learn more](https://firebase.google.com/docs/dynamic-links/android#create-a-dynamic-link-programmatically).
    #[serde(rename="dynamicLinkInfo")]
    pub dynamic_link_info: Option<DynamicLinkInfo>,
    /// Full long Dynamic Link URL with desired query parameters specified.
    /// For example,
    /// "https://sample.app.goo.gl/?link=http://www.google.com&apn=com.sample",
    /// [Learn more](https://firebase.google.com/docs/dynamic-links/android#create-a-dynamic-link-programmatically).
    #[serde(rename="longDynamicLink")]
    pub long_dynamic_link: Option<String>,
    /// Short Dynamic Link suffix. Optional.
    pub suffix: Option<Suffix>,
}

impl RequestValue for CreateShortDynamicLinkRequest {}


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


/// Parameters for Google Play Campaign Measurements.
/// [Learn more](https://developers.google.com/analytics/devguides/collection/android/v4/campaigns#campaign-params)
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
    /// [AdWords autotagging parameter](https://support.google.com/analytics/answer/1033981?hl=en);
    /// used to measure Google AdWords ads. This value is generated dynamically
    /// and should never be modified.
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
    /// The link your app will open, You can specify any URL your app can handle.
    /// This link must be a well-formatted URL, be properly URL-encoded, and use
    /// the HTTP or HTTPS scheme. See 'link' parameters in the
    /// [documentation](https://firebase.google.com/docs/dynamic-links/create-manually).
    /// 
    /// Required.
    pub link: Option<String>,
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
    /// Dynamic Links domain that the project owns, e.g. abcd.app.goo.gl
    /// [Learn more](https://firebase.google.com/docs/dynamic-links/android/receive)
    /// on how to set up Dynamic Link domain associated with your Firebase project.
    /// 
    /// Required.
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
/// * [create short links](struct.ShortLinkCreateCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateShortDynamicLinkResponse {
    /// Short Dynamic Link value. e.g. https://abcd.app.goo.gl/wxyz
    #[serde(rename="shortLink")]
    pub short_link: Option<String>,
    /// Information about potential warnings on link creation.
    pub warning: Option<Vec<DynamicLinkWarning>>,
    /// Preivew link to show the link flow chart.
    #[serde(rename="previewLink")]
    pub preview_link: Option<String>,
}

impl ResponseResult for CreateShortDynamicLinkResponse {}



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
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
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


        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }

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
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, &url)
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
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
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
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> ShortLinkCreateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


