// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *digitalassetlinks* crate version *1.0.14+20200702*, where *20200702* is the exact revision of the *digitalassetlinks:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.14*.
//! 
//! Everything else about the *digitalassetlinks* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/digital-asset-links/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/digitalassetlinks1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Digitalassetlinks.html) ... 
//! 
//! * assetlinks
//!  * [*check*](struct.AssetlinkCheckCall.html)
//! * [statements](struct.Statement.html)
//!  * [*list*](struct.StatementListCall.html)
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
//! * **[Hub](struct.Digitalassetlinks.html)**
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
//! let r = hub.assetlinks().check(...).doit()
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
//! google-digitalassetlinks1 = "*"
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
//! extern crate google_digitalassetlinks1 as digitalassetlinks1;
//! use digitalassetlinks1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use digitalassetlinks1::Digitalassetlinks;
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
//! let mut hub = Digitalassetlinks::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.assetlinks().check()
//!              .target_web_site("accusam")
//!              .target_android_app_package_name("takimata")
//!              .target_android_app_certificate_sha256_fingerprint("justo")
//!              .source_web_site("amet.")
//!              .source_android_app_package_name("erat")
//!              .source_android_app_certificate_sha256_fingerprint("labore")
//!              .relation("sea")
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

/// Central instance to access all Digitalassetlinks related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_digitalassetlinks1 as digitalassetlinks1;
/// use digitalassetlinks1::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use digitalassetlinks1::Digitalassetlinks;
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
/// let mut hub = Digitalassetlinks::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.assetlinks().check()
///              .target_web_site("nonumy")
///              .target_android_app_package_name("dolores")
///              .target_android_app_certificate_sha256_fingerprint("gubergren")
///              .source_web_site("sadipscing")
///              .source_android_app_package_name("aliquyam")
///              .source_android_app_certificate_sha256_fingerprint("ea")
///              .relation("no")
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
pub struct Digitalassetlinks<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for Digitalassetlinks<C, A> {}

impl<'a, C, A> Digitalassetlinks<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Digitalassetlinks<C, A> {
        Digitalassetlinks {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.14".to_string(),
            _base_url: "https://digitalassetlinks.googleapis.com/".to_string(),
            _root_url: "https://digitalassetlinks.googleapis.com/".to_string(),
        }
    }

    pub fn assetlinks(&'a self) -> AssetlinkMethods<'a, C, A> {
        AssetlinkMethods { hub: &self }
    }
    pub fn statements(&'a self) -> StatementMethods<'a, C, A> {
        StatementMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.14`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://digitalassetlinks.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://digitalassetlinks.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// Response message for the CheckAssetLinks call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [check assetlinks](struct.AssetlinkCheckCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CheckResponse {
    /// Error codes that describe the result of the Check operation.
    #[serde(rename="errorCode")]
    pub error_code: Option<Vec<String>>,
    /// From serving time, how much longer the response should be considered valid
    /// barring further updates.
    /// REQUIRED
    #[serde(rename="maxAge")]
    pub max_age: Option<String>,
    /// Human-readable message containing information intended to help end users
    /// understand, reproduce and debug the result.
    /// 
    /// 
    /// The message will be in English and we are currently not planning to offer
    /// any translations.
    /// 
    /// Please note that no guarantees are made about the contents or format of
    /// this string.  Any aspect of it may be subject to change without notice.
    /// You should not attempt to programmatically parse this data.  For
    /// programmatic access, use the error_code field below.
    #[serde(rename="debugString")]
    pub debug_string: Option<String>,
    /// Set to true if the assets specified in the request are linked by the
    /// relation specified in the request.
    pub linked: Option<bool>,
}

impl ResponseResult for CheckResponse {}


/// Response message for the List call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list statements](struct.StatementListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListResponse {
    /// Error codes that describe the result of the List operation.
    #[serde(rename="errorCode")]
    pub error_code: Option<Vec<String>>,
    /// A list of all the matching statements that have been found.
    pub statements: Option<Vec<Statement>>,
    /// Human-readable message containing information intended to help end users
    /// understand, reproduce and debug the result.
    /// 
    /// 
    /// The message will be in English and we are currently not planning to offer
    /// any translations.
    /// 
    /// Please note that no guarantees are made about the contents or format of
    /// this string.  Any aspect of it may be subject to change without notice.
    /// You should not attempt to programmatically parse this data.  For
    /// programmatic access, use the error_code field below.
    #[serde(rename="debugString")]
    pub debug_string: Option<String>,
    /// From serving time, how much longer the response should be considered valid
    /// barring further updates.
    /// REQUIRED
    #[serde(rename="maxAge")]
    pub max_age: Option<String>,
}

impl ResponseResult for ListResponse {}


/// Describes an X509 certificate.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CertificateInfo {
    /// The uppercase SHA-265 fingerprint of the certificate.  From the PEM
    /// certificate, it can be acquired like this:
    /// 
    /// ````text
    /// $ keytool -printcert -file $CERTFILE | grep SHA256:
    /// SHA256: 14:6D:E9:83:C5:73:06:50:D8:EE:B9:95:2F:34:FC:64:16:A0:83: \
    ///     42:E6:1D:BE:A8:8A:04:96:B2:3F:CF:44:E5
    /// ````
    /// 
    /// or like this:
    /// 
    /// ````text
    /// $ openssl x509 -in $CERTFILE -noout -fingerprint -sha256
    /// SHA256 Fingerprint=14:6D:E9:83:C5:73:06:50:D8:EE:B9:95:2F:34:FC:64: \
    ///     16:A0:83:42:E6:1D:BE:A8:8A:04:96:B2:3F:CF:44:E5
    /// ````
    /// 
    /// In this example, the contents of this field would be `14:6D:E9:83:C5:73: 06:50:D8:EE:B9:95:2F:34:FC:64:16:A0:83:42:E6:1D:BE:A8:8A:04:96:B2:3F:CF: 44:E5`.
    /// 
    /// If these tools are not available to you, you can convert the PEM
    /// certificate into the DER format, compute the SHA-256 hash of that string
    /// and represent the result as a hexstring (that is, uppercase hexadecimal
    /// representations of each octet, separated by colons).
    #[serde(rename="sha256Fingerprint")]
    pub sha256_fingerprint: Option<String>,
}

impl Part for CertificateInfo {}


/// Uniquely identifies an asset.
/// 
/// A digital asset is an identifiable and addressable online entity that
/// typically provides some service or content.  Examples of assets are websites,
/// Android apps, Twitter feeds, and Plus Pages.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Asset {
    /// Set if this is a web asset.
    pub web: Option<WebAsset>,
    /// Set if this is an Android App asset.
    #[serde(rename="androidApp")]
    pub android_app: Option<AndroidAppAsset>,
}

impl Part for Asset {}


/// Describes a reliable statement that has been made about the relationship
/// between a source asset and a target asset.
/// 
/// Statements are always made by the source asset, either directly or by
/// delegating to a statement list that is stored elsewhere.
/// 
/// For more detailed definitions of statements and assets, please refer
/// to our [API documentation landing
/// page](/digital-asset-links/v1/getting-started).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list statements](struct.StatementListCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Statement {
    /// Every statement has a source asset.
    /// REQUIRED
    pub source: Option<Asset>,
    /// The relation identifies the use of the statement as intended by the source
    /// asset's owner (that is, the person or entity who issued the statement).
    /// Every complete statement has a relation.
    /// 
    /// We identify relations with strings of the format `<kind>/<detail>`, where
    /// `<kind>` must be one of a set of pre-defined purpose categories, and
    /// `<detail>` is a free-form lowercase alphanumeric string that describes the
    /// specific use case of the statement.
    /// 
    /// Refer to [our API documentation](/digital-asset-links/v1/relation-strings)
    /// for the current list of supported relations.
    /// 
    /// Example: `delegate_permission/common.handle_all_urls`
    /// REQUIRED
    pub relation: Option<String>,
    /// Every statement has a target asset.
    /// REQUIRED
    pub target: Option<Asset>,
}

impl Resource for Statement {}


/// Describes an android app asset.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AndroidAppAsset {
    /// Android App assets are naturally identified by their Java package name.
    /// For example, the Google Maps app uses the package name
    /// `com.google.android.apps.maps`.
    /// REQUIRED
    #[serde(rename="packageName")]
    pub package_name: Option<String>,
    /// Because there is no global enforcement of package name uniqueness, we also
    /// require a signing certificate, which in combination with the package name
    /// uniquely identifies an app.
    /// 
    /// Some apps' signing keys are rotated, so they may be signed by different
    /// keys over time.  We treat these as distinct assets, since we use (package
    /// name, cert) as the unique ID.  This should not normally pose any problems
    /// as both versions of the app will make the same or similar statements.
    /// Other assets making statements about the app will have to be updated when a
    /// key is rotated, however.
    /// 
    /// (Note that the syntaxes for publishing and querying for statements contain
    /// syntactic sugar to easily let you specify apps that are known by multiple
    /// certificates.)
    /// REQUIRED
    pub certificate: Option<CertificateInfo>,
}

impl Part for AndroidAppAsset {}


/// Describes a web asset.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WebAsset {
    /// Web assets are identified by a URL that contains only the scheme, hostname
    /// and port parts.  The format is
    /// 
    /// ````text
    /// http[s]://<hostname>[:<port>]
    /// ````
    /// 
    /// Hostnames must be fully qualified: they must end in a single period
    /// ("`.`").
    /// 
    /// Only the schemes "http" and "https" are currently allowed.
    /// 
    /// Port numbers are given as a decimal number, and they must be omitted if the
    /// standard port numbers are used: 80 for http and 443 for https.
    /// 
    /// We call this limited URL the "site".  All URLs that share the same scheme,
    /// hostname and port are considered to be a part of the site and thus belong
    /// to the web asset.
    /// 
    /// Example: the asset with the site `https://www.google.com` contains all
    /// these URLs:
    /// 
    /// * `https://www.google.com/`
    /// * `https://www.google.com:443/`
    /// * `https://www.google.com/foo`
    /// * `https://www.google.com/foo?bar`
    /// * `https://www.google.com/foo#bar`
    /// * `https://user@password:www.google.com/`
    /// 
    /// But it does not contain these URLs:
    /// 
    /// * `http://www.google.com/`       (wrong scheme)
    /// * `https://google.com/`          (hostname does not match)
    /// * `https://www.google.com:444/`  (port does not match)
    ///   REQUIRED
    pub site: Option<String>,
}

impl Part for WebAsset {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *assetlink* resources.
/// It is not used directly, but through the `Digitalassetlinks` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_digitalassetlinks1 as digitalassetlinks1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use digitalassetlinks1::Digitalassetlinks;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Digitalassetlinks::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `check(...)`
/// // to build up your call.
/// let rb = hub.assetlinks();
/// # }
/// ```
pub struct AssetlinkMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Digitalassetlinks<C, A>,
}

impl<'a, C, A> MethodsBuilder for AssetlinkMethods<'a, C, A> {}

impl<'a, C, A> AssetlinkMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Determines whether the specified (directional) relationship exists between
    /// the specified source and target assets.
    /// 
    /// The relation describes the intent of the link between the two assets as
    /// claimed by the source asset.  An example for such relationships is the
    /// delegation of privileges or permissions.
    /// 
    /// This command is most often used by infrastructure systems to check
    /// preconditions for an action.  For example, a client may want to know if it
    /// is OK to send a web URL to a particular mobile app instead. The client can
    /// check for the relevant asset link from the website to the mobile app to
    /// decide if the operation should be allowed.
    /// 
    /// A note about security: if you specify a secure asset as the source, such as
    /// an HTTPS website or an Android app, the API will ensure that any
    /// statements used to generate the response have been made in a secure way by
    /// the owner of that asset.  Conversely, if the source asset is an insecure
    /// HTTP website (that is, the URL starts with `http://` instead of
    /// `https://`), the API cannot verify its statements securely, and it is not
    /// possible to ensure that the website's statements have not been altered by a
    /// third party.  For more information, see the [Digital Asset Links technical
    /// design
    /// specification](https://github.com/google/digitalassetlinks/blob/master/well-known/details.md).
    pub fn check(&self) -> AssetlinkCheckCall<'a, C, A> {
        AssetlinkCheckCall {
            hub: self.hub,
            _target_web_site: Default::default(),
            _target_android_app_package_name: Default::default(),
            _target_android_app_certificate_sha256_fingerprint: Default::default(),
            _source_web_site: Default::default(),
            _source_android_app_package_name: Default::default(),
            _source_android_app_certificate_sha256_fingerprint: Default::default(),
            _relation: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *statement* resources.
/// It is not used directly, but through the `Digitalassetlinks` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_digitalassetlinks1 as digitalassetlinks1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use digitalassetlinks1::Digitalassetlinks;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Digitalassetlinks::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.statements();
/// # }
/// ```
pub struct StatementMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Digitalassetlinks<C, A>,
}

impl<'a, C, A> MethodsBuilder for StatementMethods<'a, C, A> {}

impl<'a, C, A> StatementMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of all statements from a given source that match the
    /// specified target and statement string.
    /// 
    /// The API guarantees that all statements with secure source assets, such as
    /// HTTPS websites or Android apps, have been made in a secure way by the owner
    /// of those assets, as described in the [Digital Asset Links technical design
    /// specification](https://github.com/google/digitalassetlinks/blob/master/well-known/details.md).
    /// Specifically, you should consider that for insecure websites (that is,
    /// where the URL starts with `http://` instead of `https://`), this guarantee
    /// cannot be made.
    /// 
    /// The `List` command is most useful in cases where the API client wants to
    /// know all the ways in which two assets are related, or enumerate all the
    /// relationships from a particular source asset.  Example: a feature that
    /// helps users navigate to related items.  When a mobile app is running on a
    /// device, the feature would make it easy to navigate to the corresponding web
    /// site or Google+ profile.
    pub fn list(&self) -> StatementListCall<'a, C, A> {
        StatementListCall {
            hub: self.hub,
            _source_web_site: Default::default(),
            _source_android_app_package_name: Default::default(),
            _source_android_app_certificate_sha256_fingerprint: Default::default(),
            _relation: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Determines whether the specified (directional) relationship exists between
/// the specified source and target assets.
/// 
/// The relation describes the intent of the link between the two assets as
/// claimed by the source asset.  An example for such relationships is the
/// delegation of privileges or permissions.
/// 
/// This command is most often used by infrastructure systems to check
/// preconditions for an action.  For example, a client may want to know if it
/// is OK to send a web URL to a particular mobile app instead. The client can
/// check for the relevant asset link from the website to the mobile app to
/// decide if the operation should be allowed.
/// 
/// A note about security: if you specify a secure asset as the source, such as
/// an HTTPS website or an Android app, the API will ensure that any
/// statements used to generate the response have been made in a secure way by
/// the owner of that asset.  Conversely, if the source asset is an insecure
/// HTTP website (that is, the URL starts with `http://` instead of
/// `https://`), the API cannot verify its statements securely, and it is not
/// possible to ensure that the website's statements have not been altered by a
/// third party.  For more information, see the [Digital Asset Links technical
/// design
/// specification](https://github.com/google/digitalassetlinks/blob/master/well-known/details.md).
///
/// A builder for the *check* method supported by a *assetlink* resource.
/// It is not used directly, but through a `AssetlinkMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_digitalassetlinks1 as digitalassetlinks1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use digitalassetlinks1::Digitalassetlinks;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Digitalassetlinks::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.assetlinks().check()
///              .target_web_site("justo")
///              .target_android_app_package_name("justo")
///              .target_android_app_certificate_sha256_fingerprint("et")
///              .source_web_site("et")
///              .source_android_app_package_name("diam")
///              .source_android_app_certificate_sha256_fingerprint("ipsum")
///              .relation("Lorem")
///              .doit();
/// # }
/// ```
pub struct AssetlinkCheckCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Digitalassetlinks<C, A>,
    _target_web_site: Option<String>,
    _target_android_app_package_name: Option<String>,
    _target_android_app_certificate_sha256_fingerprint: Option<String>,
    _source_web_site: Option<String>,
    _source_android_app_package_name: Option<String>,
    _source_android_app_certificate_sha256_fingerprint: Option<String>,
    _relation: Option<String>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for AssetlinkCheckCall<'a, C, A> {}

impl<'a, C, A> AssetlinkCheckCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, CheckResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "digitalassetlinks.assetlinks.check",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(9 + self._additional_params.len());
        if let Some(value) = self._target_web_site {
            params.push(("target.web.site", value.to_string()));
        }
        if let Some(value) = self._target_android_app_package_name {
            params.push(("target.androidApp.packageName", value.to_string()));
        }
        if let Some(value) = self._target_android_app_certificate_sha256_fingerprint {
            params.push(("target.androidApp.certificate.sha256Fingerprint", value.to_string()));
        }
        if let Some(value) = self._source_web_site {
            params.push(("source.web.site", value.to_string()));
        }
        if let Some(value) = self._source_android_app_package_name {
            params.push(("source.androidApp.packageName", value.to_string()));
        }
        if let Some(value) = self._source_android_app_certificate_sha256_fingerprint {
            params.push(("source.androidApp.certificate.sha256Fingerprint", value.to_string()));
        }
        if let Some(value) = self._relation {
            params.push(("relation", value.to_string()));
        }
        for &field in ["alt", "target.web.site", "target.androidApp.packageName", "target.androidApp.certificate.sha256Fingerprint", "source.web.site", "source.androidApp.packageName", "source.androidApp.certificate.sha256Fingerprint", "relation"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/assetlinks:check";
        
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



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
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


    /// Web assets are identified by a URL that contains only the scheme, hostname
    /// and port parts.  The format is
    /// 
    /// ````text
    /// http[s]://<hostname>[:<port>]
    /// ````
    /// 
    /// Hostnames must be fully qualified: they must end in a single period
    /// ("`.`").
    /// 
    /// Only the schemes "http" and "https" are currently allowed.
    /// 
    /// Port numbers are given as a decimal number, and they must be omitted if the
    /// standard port numbers are used: 80 for http and 443 for https.
    /// 
    /// We call this limited URL the "site".  All URLs that share the same scheme,
    /// hostname and port are considered to be a part of the site and thus belong
    /// to the web asset.
    /// 
    /// Example: the asset with the site `https://www.google.com` contains all
    /// these URLs:
    /// 
    /// * `https://www.google.com/`
    /// * `https://www.google.com:443/`
    /// * `https://www.google.com/foo`
    /// * `https://www.google.com/foo?bar`
    /// * `https://www.google.com/foo#bar`
    /// * `https://user@password:www.google.com/`
    /// 
    /// But it does not contain these URLs:
    /// 
    /// * `http://www.google.com/`       (wrong scheme)
    /// * `https://google.com/`          (hostname does not match)
    /// * `https://www.google.com:444/`  (port does not match)
    ///   REQUIRED
    ///
    /// Sets the *target.web.site* query property to the given value.
    pub fn target_web_site(mut self, new_value: &str) -> AssetlinkCheckCall<'a, C, A> {
        self._target_web_site = Some(new_value.to_string());
        self
    }
    /// Android App assets are naturally identified by their Java package name.
    /// For example, the Google Maps app uses the package name
    /// `com.google.android.apps.maps`.
    /// REQUIRED
    ///
    /// Sets the *target.android app.package name* query property to the given value.
    pub fn target_android_app_package_name(mut self, new_value: &str) -> AssetlinkCheckCall<'a, C, A> {
        self._target_android_app_package_name = Some(new_value.to_string());
        self
    }
    /// The uppercase SHA-265 fingerprint of the certificate.  From the PEM
    /// certificate, it can be acquired like this:
    /// 
    /// ````text
    /// $ keytool -printcert -file $CERTFILE | grep SHA256:
    /// SHA256: 14:6D:E9:83:C5:73:06:50:D8:EE:B9:95:2F:34:FC:64:16:A0:83: \
    ///     42:E6:1D:BE:A8:8A:04:96:B2:3F:CF:44:E5
    /// ````
    /// 
    /// or like this:
    /// 
    /// ````text
    /// $ openssl x509 -in $CERTFILE -noout -fingerprint -sha256
    /// SHA256 Fingerprint=14:6D:E9:83:C5:73:06:50:D8:EE:B9:95:2F:34:FC:64: \
    ///     16:A0:83:42:E6:1D:BE:A8:8A:04:96:B2:3F:CF:44:E5
    /// ````
    /// 
    /// In this example, the contents of this field would be `14:6D:E9:83:C5:73: 06:50:D8:EE:B9:95:2F:34:FC:64:16:A0:83:42:E6:1D:BE:A8:8A:04:96:B2:3F:CF: 44:E5`.
    /// 
    /// If these tools are not available to you, you can convert the PEM
    /// certificate into the DER format, compute the SHA-256 hash of that string
    /// and represent the result as a hexstring (that is, uppercase hexadecimal
    /// representations of each octet, separated by colons).
    ///
    /// Sets the *target.android app.certificate.sha256 fingerprint* query property to the given value.
    pub fn target_android_app_certificate_sha256_fingerprint(mut self, new_value: &str) -> AssetlinkCheckCall<'a, C, A> {
        self._target_android_app_certificate_sha256_fingerprint = Some(new_value.to_string());
        self
    }
    /// Web assets are identified by a URL that contains only the scheme, hostname
    /// and port parts.  The format is
    /// 
    /// ````text
    /// http[s]://<hostname>[:<port>]
    /// ````
    /// 
    /// Hostnames must be fully qualified: they must end in a single period
    /// ("`.`").
    /// 
    /// Only the schemes "http" and "https" are currently allowed.
    /// 
    /// Port numbers are given as a decimal number, and they must be omitted if the
    /// standard port numbers are used: 80 for http and 443 for https.
    /// 
    /// We call this limited URL the "site".  All URLs that share the same scheme,
    /// hostname and port are considered to be a part of the site and thus belong
    /// to the web asset.
    /// 
    /// Example: the asset with the site `https://www.google.com` contains all
    /// these URLs:
    /// 
    /// * `https://www.google.com/`
    /// * `https://www.google.com:443/`
    /// * `https://www.google.com/foo`
    /// * `https://www.google.com/foo?bar`
    /// * `https://www.google.com/foo#bar`
    /// * `https://user@password:www.google.com/`
    /// 
    /// But it does not contain these URLs:
    /// 
    /// * `http://www.google.com/`       (wrong scheme)
    /// * `https://google.com/`          (hostname does not match)
    /// * `https://www.google.com:444/`  (port does not match)
    ///   REQUIRED
    ///
    /// Sets the *source.web.site* query property to the given value.
    pub fn source_web_site(mut self, new_value: &str) -> AssetlinkCheckCall<'a, C, A> {
        self._source_web_site = Some(new_value.to_string());
        self
    }
    /// Android App assets are naturally identified by their Java package name.
    /// For example, the Google Maps app uses the package name
    /// `com.google.android.apps.maps`.
    /// REQUIRED
    ///
    /// Sets the *source.android app.package name* query property to the given value.
    pub fn source_android_app_package_name(mut self, new_value: &str) -> AssetlinkCheckCall<'a, C, A> {
        self._source_android_app_package_name = Some(new_value.to_string());
        self
    }
    /// The uppercase SHA-265 fingerprint of the certificate.  From the PEM
    /// certificate, it can be acquired like this:
    /// 
    /// ````text
    /// $ keytool -printcert -file $CERTFILE | grep SHA256:
    /// SHA256: 14:6D:E9:83:C5:73:06:50:D8:EE:B9:95:2F:34:FC:64:16:A0:83: \
    ///     42:E6:1D:BE:A8:8A:04:96:B2:3F:CF:44:E5
    /// ````
    /// 
    /// or like this:
    /// 
    /// ````text
    /// $ openssl x509 -in $CERTFILE -noout -fingerprint -sha256
    /// SHA256 Fingerprint=14:6D:E9:83:C5:73:06:50:D8:EE:B9:95:2F:34:FC:64: \
    ///     16:A0:83:42:E6:1D:BE:A8:8A:04:96:B2:3F:CF:44:E5
    /// ````
    /// 
    /// In this example, the contents of this field would be `14:6D:E9:83:C5:73: 06:50:D8:EE:B9:95:2F:34:FC:64:16:A0:83:42:E6:1D:BE:A8:8A:04:96:B2:3F:CF: 44:E5`.
    /// 
    /// If these tools are not available to you, you can convert the PEM
    /// certificate into the DER format, compute the SHA-256 hash of that string
    /// and represent the result as a hexstring (that is, uppercase hexadecimal
    /// representations of each octet, separated by colons).
    ///
    /// Sets the *source.android app.certificate.sha256 fingerprint* query property to the given value.
    pub fn source_android_app_certificate_sha256_fingerprint(mut self, new_value: &str) -> AssetlinkCheckCall<'a, C, A> {
        self._source_android_app_certificate_sha256_fingerprint = Some(new_value.to_string());
        self
    }
    /// Query string for the relation.
    /// 
    /// We identify relations with strings of the format `<kind>/<detail>`, where
    /// `<kind>` must be one of a set of pre-defined purpose categories, and
    /// `<detail>` is a free-form lowercase alphanumeric string that describes the
    /// specific use case of the statement.
    /// 
    /// Refer to [our API documentation](/digital-asset-links/v1/relation-strings)
    /// for the current list of supported relations.
    /// 
    /// For a query to match an asset link, both the query's and the asset link's
    /// relation strings must match exactly.
    /// 
    /// Example: A query with relation `delegate_permission/common.handle_all_urls`
    /// matches an asset link with relation
    /// `delegate_permission/common.handle_all_urls`.
    ///
    /// Sets the *relation* query property to the given value.
    pub fn relation(mut self, new_value: &str) -> AssetlinkCheckCall<'a, C, A> {
        self._relation = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> AssetlinkCheckCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AssetlinkCheckCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Retrieves a list of all statements from a given source that match the
/// specified target and statement string.
/// 
/// The API guarantees that all statements with secure source assets, such as
/// HTTPS websites or Android apps, have been made in a secure way by the owner
/// of those assets, as described in the [Digital Asset Links technical design
/// specification](https://github.com/google/digitalassetlinks/blob/master/well-known/details.md).
/// Specifically, you should consider that for insecure websites (that is,
/// where the URL starts with `http://` instead of `https://`), this guarantee
/// cannot be made.
/// 
/// The `List` command is most useful in cases where the API client wants to
/// know all the ways in which two assets are related, or enumerate all the
/// relationships from a particular source asset.  Example: a feature that
/// helps users navigate to related items.  When a mobile app is running on a
/// device, the feature would make it easy to navigate to the corresponding web
/// site or Google+ profile.
///
/// A builder for the *list* method supported by a *statement* resource.
/// It is not used directly, but through a `StatementMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_digitalassetlinks1 as digitalassetlinks1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use digitalassetlinks1::Digitalassetlinks;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Digitalassetlinks::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.statements().list()
///              .source_web_site("et")
///              .source_android_app_package_name("duo")
///              .source_android_app_certificate_sha256_fingerprint("aliquyam")
///              .relation("sea")
///              .doit();
/// # }
/// ```
pub struct StatementListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Digitalassetlinks<C, A>,
    _source_web_site: Option<String>,
    _source_android_app_package_name: Option<String>,
    _source_android_app_certificate_sha256_fingerprint: Option<String>,
    _relation: Option<String>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for StatementListCall<'a, C, A> {}

impl<'a, C, A> StatementListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "digitalassetlinks.statements.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        if let Some(value) = self._source_web_site {
            params.push(("source.web.site", value.to_string()));
        }
        if let Some(value) = self._source_android_app_package_name {
            params.push(("source.androidApp.packageName", value.to_string()));
        }
        if let Some(value) = self._source_android_app_certificate_sha256_fingerprint {
            params.push(("source.androidApp.certificate.sha256Fingerprint", value.to_string()));
        }
        if let Some(value) = self._relation {
            params.push(("relation", value.to_string()));
        }
        for &field in ["alt", "source.web.site", "source.androidApp.packageName", "source.androidApp.certificate.sha256Fingerprint", "relation"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/statements:list";
        
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



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
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


    /// Web assets are identified by a URL that contains only the scheme, hostname
    /// and port parts.  The format is
    /// 
    /// ````text
    /// http[s]://<hostname>[:<port>]
    /// ````
    /// 
    /// Hostnames must be fully qualified: they must end in a single period
    /// ("`.`").
    /// 
    /// Only the schemes "http" and "https" are currently allowed.
    /// 
    /// Port numbers are given as a decimal number, and they must be omitted if the
    /// standard port numbers are used: 80 for http and 443 for https.
    /// 
    /// We call this limited URL the "site".  All URLs that share the same scheme,
    /// hostname and port are considered to be a part of the site and thus belong
    /// to the web asset.
    /// 
    /// Example: the asset with the site `https://www.google.com` contains all
    /// these URLs:
    /// 
    /// * `https://www.google.com/`
    /// * `https://www.google.com:443/`
    /// * `https://www.google.com/foo`
    /// * `https://www.google.com/foo?bar`
    /// * `https://www.google.com/foo#bar`
    /// * `https://user@password:www.google.com/`
    /// 
    /// But it does not contain these URLs:
    /// 
    /// * `http://www.google.com/`       (wrong scheme)
    /// * `https://google.com/`          (hostname does not match)
    /// * `https://www.google.com:444/`  (port does not match)
    ///   REQUIRED
    ///
    /// Sets the *source.web.site* query property to the given value.
    pub fn source_web_site(mut self, new_value: &str) -> StatementListCall<'a, C, A> {
        self._source_web_site = Some(new_value.to_string());
        self
    }
    /// Android App assets are naturally identified by their Java package name.
    /// For example, the Google Maps app uses the package name
    /// `com.google.android.apps.maps`.
    /// REQUIRED
    ///
    /// Sets the *source.android app.package name* query property to the given value.
    pub fn source_android_app_package_name(mut self, new_value: &str) -> StatementListCall<'a, C, A> {
        self._source_android_app_package_name = Some(new_value.to_string());
        self
    }
    /// The uppercase SHA-265 fingerprint of the certificate.  From the PEM
    /// certificate, it can be acquired like this:
    /// 
    /// ````text
    /// $ keytool -printcert -file $CERTFILE | grep SHA256:
    /// SHA256: 14:6D:E9:83:C5:73:06:50:D8:EE:B9:95:2F:34:FC:64:16:A0:83: \
    ///     42:E6:1D:BE:A8:8A:04:96:B2:3F:CF:44:E5
    /// ````
    /// 
    /// or like this:
    /// 
    /// ````text
    /// $ openssl x509 -in $CERTFILE -noout -fingerprint -sha256
    /// SHA256 Fingerprint=14:6D:E9:83:C5:73:06:50:D8:EE:B9:95:2F:34:FC:64: \
    ///     16:A0:83:42:E6:1D:BE:A8:8A:04:96:B2:3F:CF:44:E5
    /// ````
    /// 
    /// In this example, the contents of this field would be `14:6D:E9:83:C5:73: 06:50:D8:EE:B9:95:2F:34:FC:64:16:A0:83:42:E6:1D:BE:A8:8A:04:96:B2:3F:CF: 44:E5`.
    /// 
    /// If these tools are not available to you, you can convert the PEM
    /// certificate into the DER format, compute the SHA-256 hash of that string
    /// and represent the result as a hexstring (that is, uppercase hexadecimal
    /// representations of each octet, separated by colons).
    ///
    /// Sets the *source.android app.certificate.sha256 fingerprint* query property to the given value.
    pub fn source_android_app_certificate_sha256_fingerprint(mut self, new_value: &str) -> StatementListCall<'a, C, A> {
        self._source_android_app_certificate_sha256_fingerprint = Some(new_value.to_string());
        self
    }
    /// Use only associations that match the specified relation.
    /// 
    /// See the [`Statement`](#Statement) message for a detailed definition of
    /// relation strings.
    /// 
    /// For a query to match a statement, one of the following must be true:
    /// 
    /// * both the query's and the statement's relation strings match exactly,
    ///   or
    /// * the query's relation string is empty or missing.
    /// 
    /// Example: A query with relation `delegate_permission/common.handle_all_urls`
    /// matches an asset link with relation
    /// `delegate_permission/common.handle_all_urls`.
    ///
    /// Sets the *relation* query property to the given value.
    pub fn relation(mut self, new_value: &str) -> StatementListCall<'a, C, A> {
        self._relation = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> StatementListCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> StatementListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


