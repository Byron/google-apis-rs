// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *books* crate version *1.0.9+20181212*, where *20181212* is the exact revision of the *books:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.9*.
//! 
//! Everything else about the *books* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/books/docs/v1/getting_started).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/books1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Books.html) ... 
//! 
//! * bookshelves
//!  * [*get*](struct.BookshelveGetCall.html), [*list*](struct.BookshelveListCall.html) and [*volumes list*](struct.BookshelveVolumeListCall.html)
//! * cloudloading
//!  * [*add book*](struct.CloudloadingAddBookCall.html), [*delete book*](struct.CloudloadingDeleteBookCall.html) and [*update book*](struct.CloudloadingUpdateBookCall.html)
//! * dictionary
//!  * [*list offline metadata*](struct.DictionaryListOfflineMetadataCall.html)
//! * familysharing
//!  * [*get family info*](struct.FamilysharingGetFamilyInfoCall.html), [*share*](struct.FamilysharingShareCall.html) and [*unshare*](struct.FamilysharingUnshareCall.html)
//! * layers
//!  * [*annotation data get*](struct.LayerAnnotationDataGetCall.html), [*annotation data list*](struct.LayerAnnotationDataListCall.html), [*get*](struct.LayerGetCall.html), [*list*](struct.LayerListCall.html), [*volume annotations get*](struct.LayerVolumeAnnotationGetCall.html) and [*volume annotations list*](struct.LayerVolumeAnnotationListCall.html)
//! * myconfig
//!  * [*get user settings*](struct.MyconfigGetUserSettingCall.html), [*release download access*](struct.MyconfigReleaseDownloadAccesCall.html), [*request access*](struct.MyconfigRequestAccesCall.html), [*sync volume licenses*](struct.MyconfigSyncVolumeLicenseCall.html) and [*update user settings*](struct.MyconfigUpdateUserSettingCall.html)
//! * mylibrary
//!  * [*annotations delete*](struct.MylibraryAnnotationDeleteCall.html), [*annotations insert*](struct.MylibraryAnnotationInsertCall.html), [*annotations list*](struct.MylibraryAnnotationListCall.html), [*annotations summary*](struct.MylibraryAnnotationSummaryCall.html), [*annotations update*](struct.MylibraryAnnotationUpdateCall.html), [*bookshelves add volume*](struct.MylibraryBookshelveAddVolumeCall.html), [*bookshelves clear volumes*](struct.MylibraryBookshelveClearVolumeCall.html), [*bookshelves get*](struct.MylibraryBookshelveGetCall.html), [*bookshelves list*](struct.MylibraryBookshelveListCall.html), [*bookshelves move volume*](struct.MylibraryBookshelveMoveVolumeCall.html), [*bookshelves remove volume*](struct.MylibraryBookshelveRemoveVolumeCall.html), [*bookshelves volumes list*](struct.MylibraryBookshelveVolumeListCall.html), [*readingpositions get*](struct.MylibraryReadingpositionGetCall.html) and [*readingpositions set position*](struct.MylibraryReadingpositionSetPositionCall.html)
//! * [notification](struct.Notification.html)
//!  * [*get*](struct.NotificationGetCall.html)
//! * onboarding
//!  * [*list categories*](struct.OnboardingListCategoryCall.html) and [*list category volumes*](struct.OnboardingListCategoryVolumeCall.html)
//! * personalizedstream
//!  * [*get*](struct.PersonalizedstreamGetCall.html)
//! * promooffer
//!  * [*accept*](struct.PromoofferAcceptCall.html), [*dismiss*](struct.PromoofferDismisCall.html) and [*get*](struct.PromoofferGetCall.html)
//! * series
//!  * [*get*](struct.SeryGetCall.html) and [*membership get*](struct.SeryMembershipGetCall.html)
//! * [volumes](struct.Volume.html)
//!  * [*associated list*](struct.VolumeAssociatedListCall.html), [*get*](struct.VolumeGetCall.html), [*list*](struct.VolumeListCall.html), [*mybooks list*](struct.VolumeMybookListCall.html), [*recommended list*](struct.VolumeRecommendedListCall.html), [*recommended rate*](struct.VolumeRecommendedRateCall.html) and [*useruploaded list*](struct.VolumeUseruploadedListCall.html)
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
//! * **[Hub](struct.Books.html)**
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
//! let r = hub.volumes().useruploaded_list(...).doit()
//! let r = hub.myconfig().sync_volume_licenses(...).doit()
//! let r = hub.volumes().list(...).doit()
//! let r = hub.volumes().associated_list(...).doit()
//! let r = hub.bookshelves().volumes_list(...).doit()
//! let r = hub.volumes().recommended_list(...).doit()
//! let r = hub.mylibrary().bookshelves_volumes_list(...).doit()
//! let r = hub.volumes().mybooks_list(...).doit()
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
//! google-books1 = "*"
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
//! extern crate google_books1 as books1;
//! use books1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use books1::Books;
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
//! let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.volumes().list("q")
//!              .start_index(82)
//!              .source("gubergren")
//!              .show_preorders(false)
//!              .projection("aliquyam")
//!              .print_type("ea")
//!              .partner("no")
//!              .order_by("justo")
//!              .max_results(80)
//!              .max_allowed_maturity_rating("et")
//!              .library_restrict("et")
//!              .lang_restrict("diam")
//!              .filter("ipsum")
//!              .download("Lorem")
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
    /// Manage your books
    Full,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Full => "https://www.googleapis.com/auth/books",
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

/// Central instance to access all Books related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_books1 as books1;
/// use books1::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use books1::Books;
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
/// let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.volumes().list("q")
///              .start_index(31)
///              .source("aliquyam")
///              .show_preorders(true)
///              .projection("Lorem")
///              .print_type("eos")
///              .partner("erat")
///              .order_by("sadipscing")
///              .max_results(53)
///              .max_allowed_maturity_rating("eirmod")
///              .library_restrict("elitr")
///              .lang_restrict("amet")
///              .filter("no")
///              .download("labore")
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
pub struct Books<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for Books<C, A> {}

impl<'a, C, A> Books<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Books<C, A> {
        Books {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.9".to_string(),
            _base_url: "https://www.googleapis.com/books/v1/".to_string(),
            _root_url: "https://www.googleapis.com/".to_string(),
        }
    }

    pub fn bookshelves(&'a self) -> BookshelveMethods<'a, C, A> {
        BookshelveMethods { hub: &self }
    }
    pub fn cloudloading(&'a self) -> CloudloadingMethods<'a, C, A> {
        CloudloadingMethods { hub: &self }
    }
    pub fn dictionary(&'a self) -> DictionaryMethods<'a, C, A> {
        DictionaryMethods { hub: &self }
    }
    pub fn familysharing(&'a self) -> FamilysharingMethods<'a, C, A> {
        FamilysharingMethods { hub: &self }
    }
    pub fn layers(&'a self) -> LayerMethods<'a, C, A> {
        LayerMethods { hub: &self }
    }
    pub fn myconfig(&'a self) -> MyconfigMethods<'a, C, A> {
        MyconfigMethods { hub: &self }
    }
    pub fn mylibrary(&'a self) -> MylibraryMethods<'a, C, A> {
        MylibraryMethods { hub: &self }
    }
    pub fn notification(&'a self) -> NotificationMethods<'a, C, A> {
        NotificationMethods { hub: &self }
    }
    pub fn onboarding(&'a self) -> OnboardingMethods<'a, C, A> {
        OnboardingMethods { hub: &self }
    }
    pub fn personalizedstream(&'a self) -> PersonalizedstreamMethods<'a, C, A> {
        PersonalizedstreamMethods { hub: &self }
    }
    pub fn promooffer(&'a self) -> PromoofferMethods<'a, C, A> {
        PromoofferMethods { hub: &self }
    }
    pub fn series(&'a self) -> SeryMethods<'a, C, A> {
        SeryMethods { hub: &self }
    }
    pub fn volumes(&'a self) -> VolumeMethods<'a, C, A> {
        VolumeMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.9`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/books/v1/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Volumeseriesinfo {
    /// The display number string. This should be used only for display purposes and the actual sequence should be inferred from the below orderNumber.
    #[serde(rename="bookDisplayNumber")]
    pub book_display_number: Option<String>,
    /// Resource type.
    pub kind: Option<String>,
    /// Short book title in the context of the series.
    #[serde(rename="shortSeriesBookTitle")]
    pub short_series_book_title: Option<String>,
    /// no description provided
    #[serde(rename="volumeSeries")]
    pub volume_series: Option<Vec<VolumeseriesinfoVolumeSeries>>,
}

impl Part for Volumeseriesinfo {}


/// Author of this review.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReviewAuthor {
    /// Name of this person.
    #[serde(rename="displayName")]
    pub display_name: Option<String>,
}

impl NestedType for ReviewAuthor {}
impl Part for ReviewAuthor {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConcurrentAccessRestriction {
    /// Client nonce for verification. Download access and client-validation only.
    pub nonce: Option<String>,
    /// Resource type.
    pub kind: Option<String>,
    /// Whether this volume has any concurrent access restrictions.
    pub restricted: Option<bool>,
    /// Identifies the volume for which this entry applies.
    #[serde(rename="volumeId")]
    pub volume_id: Option<String>,
    /// The maximum number of concurrent access licenses for this volume.
    #[serde(rename="maxConcurrentDevices")]
    pub max_concurrent_devices: Option<i32>,
    /// Whether access is granted for this (user, device, volume).
    #[serde(rename="deviceAllowed")]
    pub device_allowed: Option<bool>,
    /// Client app identifier for verification. Download access and client-validation only.
    pub source: Option<String>,
    /// Time in seconds for license auto-expiration.
    #[serde(rename="timeWindowSeconds")]
    pub time_window_seconds: Option<i32>,
    /// Response signature.
    pub signature: Option<String>,
    /// Error/warning reason code.
    #[serde(rename="reasonCode")]
    pub reason_code: Option<String>,
    /// Error/warning message.
    pub message: Option<String>,
}

impl Part for ConcurrentAccessRestriction {}


/// Selection ranges sent from the client.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnnotationClientVersionRanges {
    /// Range in image CFI format for this annotation sent by client.
    #[serde(rename="imageCfiRange")]
    pub image_cfi_range: Option<BooksAnnotationsRange>,
    /// Range in GB text format for this annotation sent by client.
    #[serde(rename="gbTextRange")]
    pub gb_text_range: Option<BooksAnnotationsRange>,
    /// Content version the client sent in.
    #[serde(rename="contentVersion")]
    pub content_version: Option<String>,
    /// Range in CFI format for this annotation sent by client.
    #[serde(rename="cfiRange")]
    pub cfi_range: Option<BooksAnnotationsRange>,
    /// Range in GB image format for this annotation sent by client.
    #[serde(rename="gbImageRange")]
    pub gb_image_range: Option<BooksAnnotationsRange>,
}

impl NestedType for AnnotationClientVersionRanges {}
impl Part for AnnotationClientVersionRanges {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [request access myconfig](struct.MyconfigRequestAccesCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RequestAccess {
    /// A download access response.
    #[serde(rename="downloadAccess")]
    pub download_access: Option<DownloadAccessRestriction>,
    /// Resource type.
    pub kind: Option<String>,
    /// A concurrent access response.
    #[serde(rename="concurrentAccess")]
    pub concurrent_access: Option<ConcurrentAccessRestriction>,
}

impl ResponseResult for RequestAccess {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [volume annotations list layers](struct.LayerVolumeAnnotationListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Volumeannotations {
    /// Token to pass in for pagination for the next page. This will not be present if this request does not have more results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// A list of volume annotations.
    pub items: Option<Vec<Volumeannotation>>,
    /// Resource type
    pub kind: Option<String>,
    /// The version string for all of the volume annotations in this layer (not just the ones in this response). Note: the version string doesn't apply to the annotation data, just the information in this response (e.g. the location of annotations in the book).
    pub version: Option<String>,
    /// The total number of volume annotations found.
    #[serde(rename="totalItems")]
    pub total_items: Option<i32>,
}

impl ResponseResult for Volumeannotations {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [annotations list mylibrary](struct.MylibraryAnnotationListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Annotations {
    /// Token to pass in for pagination for the next page. This will not be present if this request does not have more results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// A list of annotations.
    pub items: Option<Vec<Annotation>>,
    /// Resource type.
    pub kind: Option<String>,
    /// Total number of annotations found. This may be greater than the number of notes returned in this response if results have been paginated.
    #[serde(rename="totalItems")]
    pub total_items: Option<i32>,
}

impl ResponseResult for Annotations {}


/// Offers available for this volume (sales and rentals).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeSaleInfoOffers {
    /// The rental duration (for rental offers only).
    #[serde(rename="rentalDuration")]
    pub rental_duration: Option<VolumeSaleInfoOffersRentalDuration>,
    /// Offer retail (=discounted) price in Micros
    #[serde(rename="retailPrice")]
    pub retail_price: Option<VolumeSaleInfoOffersRetailPrice>,
    /// Indicates whether the offer is giftable.
    pub giftable: Option<bool>,
    /// Offer list (=undiscounted) price in Micros.
    #[serde(rename="listPrice")]
    pub list_price: Option<VolumeSaleInfoOffersListPrice>,
    /// The finsky offer type (e.g., PURCHASE=0 RENTAL=3)
    #[serde(rename="finskyOfferType")]
    pub finsky_offer_type: Option<i32>,
}

impl NestedType for VolumeSaleInfoOffers {}
impl Part for VolumeSaleInfoOffers {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list offline metadata dictionary](struct.DictionaryListOfflineMetadataCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    /// A list of offline dictionary metadata.
    pub items: Option<Vec<MetadataItems>>,
    /// Resource type.
    pub kind: Option<String>,
}

impl ResponseResult for Metadata {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get personalizedstream](struct.PersonalizedstreamGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Discoveryclusters {
    /// no description provided
    pub clusters: Option<Vec<DiscoveryclustersClusters>>,
    /// Resorce type.
    pub kind: Option<String>,
    /// no description provided
    #[serde(rename="totalClusters")]
    pub total_clusters: Option<i32>,
}

impl ResponseResult for Discoveryclusters {}


/// Information on the ability to share with the family.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeUserInfoFamilySharing {
    /// Whether or not sharing this volume is temporarily disabled due to issues with the Family Wallet.
    #[serde(rename="isSharingDisabledByFop")]
    pub is_sharing_disabled_by_fop: Option<bool>,
    /// The role of the user in the family.
    #[serde(rename="familyRole")]
    pub family_role: Option<String>,
    /// Whether or not this volume can be shared with the family by the user. This includes sharing eligibility of both the volume and the user. If the value is true, the user can initiate a family sharing action.
    #[serde(rename="isSharingAllowed")]
    pub is_sharing_allowed: Option<bool>,
}

impl NestedType for VolumeUserInfoFamilySharing {}
impl Part for VolumeUserInfoFamilySharing {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeseriesinfoVolumeSeries {
    /// The book order number in the series.
    #[serde(rename="orderNumber")]
    pub order_number: Option<i32>,
    /// The series id.
    #[serde(rename="seriesId")]
    pub series_id: Option<String>,
    /// List of issues. Applicable only for Collection Edition and Omnibus.
    pub issue: Option<Vec<VolumeseriesinfoVolumeSeriesIssue>>,
    /// The book type in the context of series. Examples - Single Issue, Collection Edition, etc.
    #[serde(rename="seriesBookType")]
    pub series_book_type: Option<String>,
}

impl NestedType for VolumeseriesinfoVolumeSeries {}
impl Part for VolumeseriesinfoVolumeSeries {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get notification](struct.NotificationGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Notification {
    /// no description provided
    pub body: Option<String>,
    /// The list of crm experiment ids.
    #[serde(rename="crmExperimentIds")]
    pub crm_experiment_ids: Option<Vec<String>>,
    /// no description provided
    pub dont_show_notification: Option<bool>,
    /// no description provided
    pub show_notification_settings_action: Option<bool>,
    /// no description provided
    #[serde(rename="iconUrl")]
    pub icon_url: Option<String>,
    /// no description provided
    pub reason: Option<String>,
    /// no description provided
    #[serde(rename="notificationGroup")]
    pub notification_group: Option<String>,
    /// no description provided
    pub notification_type: Option<String>,
    /// no description provided
    pub pcampaign_id: Option<String>,
    /// no description provided
    pub doc_type: Option<String>,
    /// Resource type.
    pub kind: Option<String>,
    /// no description provided
    pub title: Option<String>,
    /// no description provided
    pub is_document_mature: Option<bool>,
    /// no description provided
    #[serde(rename="timeToExpireMs")]
    pub time_to_expire_ms: Option<String>,
    /// no description provided
    #[serde(rename="targetUrl")]
    pub target_url: Option<String>,
    /// no description provided
    pub doc_id: Option<String>,
}

impl ResponseResult for Notification {}


/// User settings in sub-objects, each for different purposes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UsersettingsNotesExport {
    /// no description provided
    #[serde(rename="isEnabled")]
    pub is_enabled: Option<bool>,
    /// no description provided
    #[serde(rename="folderName")]
    pub folder_name: Option<String>,
}

impl NestedType for UsersettingsNotesExport {}
impl Part for UsersettingsNotesExport {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [annotations insert mylibrary](struct.MylibraryAnnotationInsertCall.html) (request|response)
/// * [annotations update mylibrary](struct.MylibraryAnnotationUpdateCall.html) (request|response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Annotation {
    /// Timestamp for the last time this annotation was modified.
    pub updated: Option<String>,
    /// Indicates that this annotation is deleted.
    pub deleted: Option<bool>,
    /// Selection ranges for the most recent content version.
    #[serde(rename="currentVersionRanges")]
    pub current_version_ranges: Option<AnnotationCurrentVersionRanges>,
    /// Anchor text after excerpt. For requests, if the user bookmarked a screen that has no flowing text on it, then this field should be empty.
    #[serde(rename="afterSelectedText")]
    pub after_selected_text: Option<String>,
    /// The volume that this annotation belongs to.
    #[serde(rename="volumeId")]
    pub volume_id: Option<String>,
    /// Excerpt from the volume.
    #[serde(rename="selectedText")]
    pub selected_text: Option<String>,
    /// User-created data for this annotation.
    pub data: Option<String>,
    /// Id of this annotation, in the form of a GUID.
    pub id: Option<String>,
    /// Resource type.
    pub kind: Option<String>,
    /// Timestamp for the created time of this annotation.
    pub created: Option<String>,
    /// Anchor text before excerpt. For requests, if the user bookmarked a screen that has no flowing text on it, then this field should be empty.
    #[serde(rename="beforeSelectedText")]
    pub before_selected_text: Option<String>,
    /// Selection ranges sent from the client.
    #[serde(rename="clientVersionRanges")]
    pub client_version_ranges: Option<AnnotationClientVersionRanges>,
    /// Pages that this annotation spans.
    #[serde(rename="pageIds")]
    pub page_ids: Option<Vec<String>>,
    /// The layer this annotation is for.
    #[serde(rename="layerId")]
    pub layer_id: Option<String>,
    /// The highlight style for this annotation.
    #[serde(rename="highlightStyle")]
    pub highlight_style: Option<String>,
    /// no description provided
    #[serde(rename="layerSummary")]
    pub layer_summary: Option<AnnotationLayerSummary>,
    /// URL to this resource.
    #[serde(rename="selfLink")]
    pub self_link: Option<String>,
}

impl RequestValue for Annotation {}
impl ResponseResult for Annotation {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [readingpositions get mylibrary](struct.MylibraryReadingpositionGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReadingPosition {
    /// Position in a PDF file.
    #[serde(rename="pdfPosition")]
    pub pdf_position: Option<String>,
    /// Resource type for a reading position.
    pub kind: Option<String>,
    /// Position in a volume for image-based content.
    #[serde(rename="gbImagePosition")]
    pub gb_image_position: Option<String>,
    /// Position in a volume for text-based content.
    #[serde(rename="gbTextPosition")]
    pub gb_text_position: Option<String>,
    /// Position in an EPUB as a CFI.
    #[serde(rename="epubCfiPosition")]
    pub epub_cfi_position: Option<String>,
    /// Timestamp when this reading position was last updated (formatted UTC timestamp with millisecond resolution).
    pub updated: Option<String>,
    /// Volume id associated with this reading position.
    #[serde(rename="volumeId")]
    pub volume_id: Option<String>,
}

impl ResponseResult for ReadingPosition {}


/// Offer retail (=discounted) price in Micros
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeSaleInfoOffersRetailPrice {
    /// no description provided
    #[serde(rename="currencyCode")]
    pub currency_code: Option<String>,
    /// no description provided
    #[serde(rename="amountInMicros")]
    pub amount_in_micros: Option<f64>,
}

impl NestedType for VolumeSaleInfoOffersRetailPrice {}
impl Part for VolumeSaleInfoOffersRetailPrice {}


/// Search result information related to this volume.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeSearchInfo {
    /// A text snippet containing the search query.
    #[serde(rename="textSnippet")]
    pub text_snippet: Option<String>,
}

impl NestedType for VolumeSearchInfo {}
impl Part for VolumeSearchInfo {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [annotation data get layers](struct.LayerAnnotationDataGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Annotationdata {
    /// The type of annotation this data is for.
    #[serde(rename="annotationType")]
    pub annotation_type: Option<String>,
    /// Resource Type
    pub kind: Option<String>,
    /// Timestamp for the last time this data was updated. (RFC 3339 UTC date-time format).
    pub updated: Option<String>,
    /// The volume id for this data. *
    #[serde(rename="volumeId")]
    pub volume_id: Option<String>,
    /// Base64 encoded data for this annotation data.
    pub encoded_data: Option<String>,
    /// The Layer id for this data. *
    #[serde(rename="layerId")]
    pub layer_id: Option<String>,
    /// no description provided
    pub data: Option<String>,
    /// Unique id for this annotation data.
    pub id: Option<String>,
    /// URL for this resource. *
    #[serde(rename="selfLink")]
    pub self_link: Option<String>,
}

impl ResponseResult for Annotationdata {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [membership get series](struct.SeryMembershipGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Seriesmembership {
    /// no description provided
    pub member: Option<Vec<Volume>>,
    /// no description provided
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// Resorce type.
    pub kind: Option<String>,
}

impl ResponseResult for Seriesmembership {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [annotation data list layers](struct.LayerAnnotationDataListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Annotationsdata {
    /// Token to pass in for pagination for the next page. This will not be present if this request does not have more results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// A list of Annotation Data.
    pub items: Option<Vec<Annotationdata>>,
    /// Resource type
    pub kind: Option<String>,
    /// The total number of volume annotations found.
    #[serde(rename="totalItems")]
    pub total_items: Option<i32>,
}

impl ResponseResult for Annotationsdata {}


/// A list of image links for all the sizes that are available. (In LITE projection.)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeVolumeInfoImageLinks {
    /// Image link for large size (width of ~800 pixels). (In LITE projection)
    pub large: Option<String>,
    /// Image link for extra large size (width of ~1280 pixels). (In LITE projection)
    #[serde(rename="extraLarge")]
    pub extra_large: Option<String>,
    /// Image link for medium size (width of ~575 pixels). (In LITE projection)
    pub medium: Option<String>,
    /// Image link for small thumbnail size (width of ~80 pixels). (In LITE projection)
    #[serde(rename="smallThumbnail")]
    pub small_thumbnail: Option<String>,
    /// Image link for small size (width of ~300 pixels). (In LITE projection)
    pub small: Option<String>,
    /// Image link for thumbnail size (width of ~128 pixels). (In LITE projection)
    pub thumbnail: Option<String>,
}

impl NestedType for VolumeVolumeInfoImageLinks {}
impl Part for VolumeVolumeInfoImageLinks {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get promooffer](struct.PromoofferGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Offers {
    /// A list of offers.
    pub items: Option<Vec<OffersItems>>,
    /// Resource type.
    pub kind: Option<String>,
}

impl ResponseResult for Offers {}


/// Information about epub content. (In LITE projection.)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeAccessInfoEpub {
    /// Is a flowing text epub available either as public domain or for purchase. (In LITE projection.)
    #[serde(rename="isAvailable")]
    pub is_available: Option<bool>,
    /// URL to download epub. (In LITE projection.)
    #[serde(rename="downloadLink")]
    pub download_link: Option<String>,
    /// URL to retrieve ACS token for epub download. (In LITE projection.)
    #[serde(rename="acsTokenLink")]
    pub acs_token_link: Option<String>,
}

impl NestedType for VolumeAccessInfoEpub {}
impl Part for VolumeAccessInfoEpub {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DiscoveryclustersClusters {
    /// no description provided
    #[serde(rename="totalVolumes")]
    pub total_volumes: Option<i32>,
    /// no description provided
    #[serde(rename="subTitle")]
    pub sub_title: Option<String>,
    /// no description provided
    pub uid: Option<String>,
    /// no description provided
    pub banner_with_content_container: Option<DiscoveryclustersClustersBannerWithContentContainer>,
    /// no description provided
    pub title: Option<String>,
    /// no description provided
    pub volumes: Option<Vec<Volume>>,
}

impl NestedType for DiscoveryclustersClusters {}
impl Part for DiscoveryclustersClusters {}


/// Information about pdf content. (In LITE projection.)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeAccessInfoPdf {
    /// Is a scanned image pdf available either as public domain or for purchase. (In LITE projection.)
    #[serde(rename="isAvailable")]
    pub is_available: Option<bool>,
    /// URL to download pdf. (In LITE projection.)
    #[serde(rename="downloadLink")]
    pub download_link: Option<String>,
    /// URL to retrieve ACS token for pdf download. (In LITE projection.)
    #[serde(rename="acsTokenLink")]
    pub acs_token_link: Option<String>,
}

impl NestedType for VolumeAccessInfoPdf {}
impl Part for VolumeAccessInfoPdf {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [annotations summary mylibrary](struct.MylibraryAnnotationSummaryCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnnotationsSummary {
    /// no description provided
    pub layers: Option<Vec<AnnotationsSummaryLayers>>,
    /// no description provided
    pub kind: Option<String>,
}

impl ResponseResult for AnnotationsSummary {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [bookshelves list mylibrary](struct.MylibraryBookshelveListCall.html) (response)
/// * [list bookshelves](struct.BookshelveListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Bookshelves {
    /// A list of bookshelves.
    pub items: Option<Vec<Bookshelf>>,
    /// Resource type.
    pub kind: Option<String>,
}

impl ResponseResult for Bookshelves {}


/// Physical dimensions of this volume.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeVolumeInfoDimensions {
    /// Width of this volume (in cm).
    pub width: Option<String>,
    /// Height or length of this volume (in cm).
    pub height: Option<String>,
    /// Thickness of this volume (in cm).
    pub thickness: Option<String>,
}

impl NestedType for VolumeVolumeInfoDimensions {}
impl Part for VolumeVolumeInfoDimensions {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list layers](struct.LayerListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Layersummaries {
    /// The total number of layer summaries found.
    #[serde(rename="totalItems")]
    pub total_items: Option<i32>,
    /// A list of layer summary items.
    pub items: Option<Vec<Layersummary>>,
    /// Resource type.
    pub kind: Option<String>,
}

impl ResponseResult for Layersummaries {}


/// A list of offers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OffersItems {
    /// no description provided
    #[serde(rename="gservicesKey")]
    pub gservices_key: Option<String>,
    /// no description provided
    pub items: Option<Vec<OffersItemsItems>>,
    /// no description provided
    #[serde(rename="artUrl")]
    pub art_url: Option<String>,
    /// no description provided
    pub id: Option<String>,
}

impl NestedType for OffersItems {}
impl Part for OffersItems {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DiscoveryclustersClustersBannerWithContentContainer {
    /// no description provided
    #[serde(rename="maskColorArgb")]
    pub mask_color_argb: Option<String>,
    /// no description provided
    #[serde(rename="moreButtonText")]
    pub more_button_text: Option<String>,
    /// no description provided
    #[serde(rename="moreButtonUrl")]
    pub more_button_url: Option<String>,
    /// no description provided
    #[serde(rename="textColorArgb")]
    pub text_color_argb: Option<String>,
    /// no description provided
    #[serde(rename="imageUrl")]
    pub image_url: Option<String>,
    /// no description provided
    #[serde(rename="fillColorArgb")]
    pub fill_color_argb: Option<String>,
}

impl NestedType for DiscoveryclustersClustersBannerWithContentContainer {}
impl Part for DiscoveryclustersClustersBannerWithContentContainer {}


/// Copy/Paste accounting information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeUserInfoCopy {
    /// no description provided
    #[serde(rename="limitType")]
    pub limit_type: Option<String>,
    /// no description provided
    #[serde(rename="remainingCharacterCount")]
    pub remaining_character_count: Option<i32>,
    /// no description provided
    pub updated: Option<String>,
    /// no description provided
    #[serde(rename="allowedCharacterCount")]
    pub allowed_character_count: Option<i32>,
}

impl NestedType for VolumeUserInfoCopy {}
impl Part for VolumeUserInfoCopy {}


/// Any information about a volume related to reading or obtaining that volume text. This information can depend on country (books may be public domain in one country but not in another, e.g.).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeAccessInfo {
    /// URL to read this volume on the Google Books site. Link will not allow users to read non-viewable volumes.
    #[serde(rename="webReaderLink")]
    pub web_reader_link: Option<String>,
    /// Whether or not this book is public domain in the country listed above.
    #[serde(rename="publicDomain")]
    pub public_domain: Option<bool>,
    /// Whether this volume requires that the client explicitly request offline download license rather than have it done automatically when loading the content, if the client supports it.
    #[serde(rename="explicitOfflineLicenseManagement")]
    pub explicit_offline_license_management: Option<bool>,
    /// Whether this volume can be embedded in a viewport using the Embedded Viewer API.
    pub embeddable: Option<bool>,
    /// Information about a volume's download license access restrictions.
    #[serde(rename="downloadAccess")]
    pub download_access: Option<DownloadAccessRestriction>,
    /// The two-letter ISO_3166-1 country code for which this access information is valid. (In LITE projection.)
    pub country: Option<String>,
    /// For ordered but not yet processed orders, we give a URL that can be used to go to the appropriate Google Wallet page.
    #[serde(rename="viewOrderUrl")]
    pub view_order_url: Option<String>,
    /// Whether text-to-speech is permitted for this volume. Values can be ALLOWED, ALLOWED_FOR_ACCESSIBILITY, or NOT_ALLOWED.
    #[serde(rename="textToSpeechPermission")]
    pub text_to_speech_permission: Option<String>,
    /// URL to the Google Drive viewer if this volume is uploaded by the user by selecting the file from Google Drive.
    #[serde(rename="driveImportedContentLink")]
    pub drive_imported_content_link: Option<String>,
    /// Information about pdf content. (In LITE projection.)
    pub pdf: Option<VolumeAccessInfoPdf>,
    /// Whether quote sharing is allowed for this volume.
    #[serde(rename="quoteSharingAllowed")]
    pub quote_sharing_allowed: Option<bool>,
    /// The read access of a volume. Possible values are PARTIAL, ALL_PAGES, NO_PAGES or UNKNOWN. This value depends on the country listed above. A value of PARTIAL means that the publisher has allowed some portion of the volume to be viewed publicly, without purchase. This can apply to eBooks as well as non-eBooks. Public domain books will always have a value of ALL_PAGES.
    pub viewability: Option<String>,
    /// Information about epub content. (In LITE projection.)
    pub epub: Option<VolumeAccessInfoEpub>,
    /// Combines the access and viewability of this volume into a single status field for this user. Values can be FULL_PURCHASED, FULL_PUBLIC_DOMAIN, SAMPLE or NONE. (In LITE projection.)
    #[serde(rename="accessViewStatus")]
    pub access_view_status: Option<String>,
}

impl NestedType for VolumeAccessInfo {}
impl Part for VolumeAccessInfo {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [update book cloudloading](struct.CloudloadingUpdateBookCall.html) (request|response)
/// * [add book cloudloading](struct.CloudloadingAddBookCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BooksCloudloadingResource {
    /// no description provided
    pub author: Option<String>,
    /// no description provided
    #[serde(rename="processingState")]
    pub processing_state: Option<String>,
    /// no description provided
    #[serde(rename="volumeId")]
    pub volume_id: Option<String>,
    /// no description provided
    pub title: Option<String>,
}

impl RequestValue for BooksCloudloadingResource {}
impl ResponseResult for BooksCloudloadingResource {}


/// Suggested retail price. (In LITE projection.)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeSaleInfoListPrice {
    /// Amount in the currency listed below. (In LITE projection.)
    pub amount: Option<f64>,
    /// An ISO 4217, three-letter currency code. (In LITE projection.)
    #[serde(rename="currencyCode")]
    pub currency_code: Option<String>,
}

impl NestedType for VolumeSaleInfoListPrice {}
impl Part for VolumeSaleInfoListPrice {}


/// Selection ranges for the most recent content version.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnnotationCurrentVersionRanges {
    /// Range in image CFI format for this annotation for version above.
    #[serde(rename="imageCfiRange")]
    pub image_cfi_range: Option<BooksAnnotationsRange>,
    /// Range in GB text format for this annotation for version above.
    #[serde(rename="gbTextRange")]
    pub gb_text_range: Option<BooksAnnotationsRange>,
    /// Content version applicable to ranges below.
    #[serde(rename="contentVersion")]
    pub content_version: Option<String>,
    /// Range in CFI format for this annotation for version above.
    #[serde(rename="cfiRange")]
    pub cfi_range: Option<BooksAnnotationsRange>,
    /// Range in GB image format for this annotation for version above.
    #[serde(rename="gbImageRange")]
    pub gb_image_range: Option<BooksAnnotationsRange>,
}

impl NestedType for AnnotationCurrentVersionRanges {}
impl Part for AnnotationCurrentVersionRanges {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [useruploaded list volumes](struct.VolumeUseruploadedListCall.html) (response)
/// * [sync volume licenses myconfig](struct.MyconfigSyncVolumeLicenseCall.html) (response)
/// * [list volumes](struct.VolumeListCall.html) (response)
/// * [associated list volumes](struct.VolumeAssociatedListCall.html) (response)
/// * [volumes list bookshelves](struct.BookshelveVolumeListCall.html) (response)
/// * [recommended list volumes](struct.VolumeRecommendedListCall.html) (response)
/// * [bookshelves volumes list mylibrary](struct.MylibraryBookshelveVolumeListCall.html) (response)
/// * [mybooks list volumes](struct.VolumeMybookListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Volumes {
    /// Total number of volumes found. This might be greater than the number of volumes returned in this response if results have been paginated.
    #[serde(rename="totalItems")]
    pub total_items: Option<i32>,
    /// A list of volumes.
    pub items: Option<Vec<Volume>>,
    /// Resource type.
    pub kind: Option<String>,
}

impl ResponseResult for Volumes {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UsersettingsNotificationMoreFromAuthors {
    /// no description provided
    pub opted_state: Option<String>,
}

impl NestedType for UsersettingsNotificationMoreFromAuthors {}
impl Part for UsersettingsNotificationMoreFromAuthors {}


/// Period during this book is/was a valid rental.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeUserInfoRentalPeriod {
    /// no description provided
    #[serde(rename="startUtcSec")]
    pub start_utc_sec: Option<String>,
    /// no description provided
    #[serde(rename="endUtcSec")]
    pub end_utc_sec: Option<String>,
}

impl NestedType for VolumeUserInfoRentalPeriod {}
impl Part for VolumeUserInfoRentalPeriod {}


/// What layers exist in this volume and high level information about them.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeLayerInfo {
    /// A layer should appear here if and only if the layer exists for this book.
    pub layers: Option<Vec<VolumeLayerInfoLayers>>,
}

impl NestedType for VolumeLayerInfo {}
impl Part for VolumeLayerInfo {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get family info familysharing](struct.FamilysharingGetFamilyInfoCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FamilyInfo {
    /// Resource type.
    pub kind: Option<String>,
    /// Family membership info of the user that made the request.
    pub membership: Option<FamilyInfoMembership>,
}

impl ResponseResult for FamilyInfo {}


/// A layer should appear here if and only if the layer exists for this book.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeLayerInfoLayers {
    /// The current version of this layer's volume annotations. Note that this version applies only to the data in the books.layers.volumeAnnotations.* responses. The actual annotation data is versioned separately.
    #[serde(rename="volumeAnnotationsVersion")]
    pub volume_annotations_version: Option<String>,
    /// The layer id of this layer (e.g. "geo").
    #[serde(rename="layerId")]
    pub layer_id: Option<String>,
}

impl NestedType for VolumeLayerInfoLayers {}
impl Part for VolumeLayerInfoLayers {}


/// The rental duration (for rental offers only).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeSaleInfoOffersRentalDuration {
    /// no description provided
    pub count: Option<f64>,
    /// no description provided
    pub unit: Option<String>,
}

impl NestedType for VolumeSaleInfoOffersRentalDuration {}
impl Part for VolumeSaleInfoOffersRentalDuration {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list categories onboarding](struct.OnboardingListCategoryCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Category {
    /// A list of onboarding categories.
    pub items: Option<Vec<CategoryItems>>,
    /// Resource type.
    pub kind: Option<String>,
}

impl ResponseResult for Category {}


/// A list of onboarding categories.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CategoryItems {
    /// no description provided
    #[serde(rename="badgeUrl")]
    pub badge_url: Option<String>,
    /// no description provided
    #[serde(rename="categoryId")]
    pub category_id: Option<String>,
    /// no description provided
    pub name: Option<String>,
}

impl NestedType for CategoryItems {}
impl Part for CategoryItems {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Review {
    /// Star rating for this review. Possible values are ONE, TWO, THREE, FOUR, FIVE or NOT_RATED.
    pub rating: Option<String>,
    /// Resource type for a review.
    pub kind: Option<String>,
    /// Title for this review.
    pub title: Option<String>,
    /// Author of this review.
    pub author: Option<ReviewAuthor>,
    /// Volume that this review is for.
    #[serde(rename="volumeId")]
    pub volume_id: Option<String>,
    /// Review text.
    pub content: Option<String>,
    /// Information regarding the source of this review, when the review is not from a Google Books user.
    pub source: Option<ReviewSource>,
    /// Date of this review.
    pub date: Option<String>,
    /// Source type for this review. Possible values are EDITORIAL, WEB_USER or GOOGLE_USER.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// URL for the full review text, for reviews gathered from the web.
    #[serde(rename="fullTextUrl")]
    pub full_text_url: Option<String>,
}

impl Part for Review {}


/// Recommendation related information for this volume.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeRecommendedInfo {
    /// A text explaining why this volume is recommended.
    pub explanation: Option<String>,
}

impl NestedType for VolumeRecommendedInfo {}
impl Part for VolumeRecommendedInfo {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OffersItemsItems {
    /// no description provided
    pub description: Option<String>,
    /// no description provided
    pub author: Option<String>,
    /// no description provided
    pub title: Option<String>,
    /// no description provided
    #[serde(rename="coverUrl")]
    pub cover_url: Option<String>,
    /// no description provided
    #[serde(rename="volumeId")]
    pub volume_id: Option<String>,
    /// no description provided
    #[serde(rename="canonicalVolumeLink")]
    pub canonical_volume_link: Option<String>,
}

impl NestedType for OffersItemsItems {}
impl Part for OffersItemsItems {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get user settings myconfig](struct.MyconfigGetUserSettingCall.html) (response)
/// * [update user settings myconfig](struct.MyconfigUpdateUserSettingCall.html) (request|response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Usersettings {
    /// no description provided
    pub notification: Option<UsersettingsNotification>,
    /// Resource type.
    pub kind: Option<String>,
    /// User settings in sub-objects, each for different purposes.
    #[serde(rename="notesExport")]
    pub notes_export: Option<UsersettingsNotesExport>,
}

impl RequestValue for Usersettings {}
impl ResponseResult for Usersettings {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UsersettingsNotificationMatchMyInterests {
    /// no description provided
    pub opted_state: Option<String>,
}

impl NestedType for UsersettingsNotificationMatchMyInterests {}
impl Part for UsersettingsNotificationMatchMyInterests {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UsersettingsNotificationMoreFromSeries {
    /// no description provided
    pub opted_state: Option<String>,
}

impl NestedType for UsersettingsNotificationMoreFromSeries {}
impl Part for UsersettingsNotificationMoreFromSeries {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get layers](struct.LayerGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Layersummary {
    /// Resource Type
    pub kind: Option<String>,
    /// The number of annotations for this layer.
    #[serde(rename="annotationCount")]
    pub annotation_count: Option<i32>,
    /// The number of data items for this layer.
    #[serde(rename="dataCount")]
    pub data_count: Option<i32>,
    /// The list of annotation types contained for this layer.
    #[serde(rename="annotationTypes")]
    pub annotation_types: Option<Vec<String>>,
    /// Timestamp for the last time an item in this layer was updated. (RFC 3339 UTC date-time format).
    pub updated: Option<String>,
    /// The volume id this resource is for.
    #[serde(rename="volumeId")]
    pub volume_id: Option<String>,
    /// Link to get data for this annotation.
    #[serde(rename="annotationsDataLink")]
    pub annotations_data_link: Option<String>,
    /// The link to get the annotations for this layer.
    #[serde(rename="annotationsLink")]
    pub annotations_link: Option<String>,
    /// The content version this resource is for.
    #[serde(rename="contentVersion")]
    pub content_version: Option<String>,
    /// The layer id for this summary.
    #[serde(rename="layerId")]
    pub layer_id: Option<String>,
    /// The current version of this layer's volume annotations. Note that this version applies only to the data in the books.layers.volumeAnnotations.* responses. The actual annotation data is versioned separately.
    #[serde(rename="volumeAnnotationsVersion")]
    pub volume_annotations_version: Option<String>,
    /// Unique id of this layer summary.
    pub id: Option<String>,
    /// URL to this resource.
    #[serde(rename="selfLink")]
    pub self_link: Option<String>,
}

impl ResponseResult for Layersummary {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DownloadAccessRestriction {
    /// Client nonce for verification. Download access and client-validation only.
    pub nonce: Option<String>,
    /// Resource type.
    pub kind: Option<String>,
    /// If deviceAllowed, whether access was just acquired with this request.
    #[serde(rename="justAcquired")]
    pub just_acquired: Option<bool>,
    /// If restricted, the maximum number of content download licenses for this volume.
    #[serde(rename="maxDownloadDevices")]
    pub max_download_devices: Option<i32>,
    /// If restricted, the number of content download licenses already acquired (including the requesting client, if licensed).
    #[serde(rename="downloadsAcquired")]
    pub downloads_acquired: Option<i32>,
    /// Identifies the volume for which this entry applies.
    #[serde(rename="volumeId")]
    pub volume_id: Option<String>,
    /// If restricted, whether access is granted for this (user, device, volume).
    #[serde(rename="deviceAllowed")]
    pub device_allowed: Option<bool>,
    /// Client app identifier for verification. Download access and client-validation only.
    pub source: Option<String>,
    /// Response signature.
    pub signature: Option<String>,
    /// Error/warning reason code. Additional codes may be added in the future. 0 OK 100 ACCESS_DENIED_PUBLISHER_LIMIT 101 ACCESS_DENIED_LIMIT 200 WARNING_USED_LAST_ACCESS
    #[serde(rename="reasonCode")]
    pub reason_code: Option<String>,
    /// Error/warning message.
    pub message: Option<String>,
    /// Whether this volume has any download access restrictions.
    pub restricted: Option<bool>,
}

impl Part for DownloadAccessRestriction {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [release download access myconfig](struct.MyconfigReleaseDownloadAccesCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DownloadAccesses {
    /// A list of download access responses.
    #[serde(rename="downloadAccessList")]
    pub download_access_list: Option<Vec<DownloadAccessRestriction>>,
    /// Resource type.
    pub kind: Option<String>,
}

impl ResponseResult for DownloadAccesses {}


/// Information regarding the source of this review, when the review is not from a Google Books user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReviewSource {
    /// Extra text about the source of the review.
    #[serde(rename="extraDescription")]
    pub extra_description: Option<String>,
    /// URL of the source of the review.
    pub url: Option<String>,
    /// Name of the source.
    pub description: Option<String>,
}

impl NestedType for ReviewSource {}
impl Part for ReviewSource {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get bookshelves](struct.BookshelveGetCall.html) (response)
/// * [bookshelves get mylibrary](struct.MylibraryBookshelveGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Bookshelf {
    /// Resource type for bookshelf metadata.
    pub kind: Option<String>,
    /// Description of this bookshelf.
    pub description: Option<String>,
    /// Title of this bookshelf.
    pub title: Option<String>,
    /// Number of volumes in this bookshelf.
    #[serde(rename="volumeCount")]
    pub volume_count: Option<i32>,
    /// Created time for this bookshelf (formatted UTC timestamp with millisecond resolution).
    pub created: Option<String>,
    /// Last modified time of this bookshelf (formatted UTC timestamp with millisecond resolution).
    pub updated: Option<String>,
    /// Whether this bookshelf is PUBLIC or PRIVATE.
    pub access: Option<String>,
    /// Last time a volume was added or removed from this bookshelf (formatted UTC timestamp with millisecond resolution).
    #[serde(rename="volumesLastUpdated")]
    pub volumes_last_updated: Option<String>,
    /// Id of this bookshelf, only unique by user.
    pub id: Option<i32>,
    /// URL to this resource.
    #[serde(rename="selfLink")]
    pub self_link: Option<String>,
}

impl ResponseResult for Bookshelf {}


/// General volume information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeVolumeInfo {
    /// Volume subtitle. (In LITE projection.)
    pub subtitle: Option<String>,
    /// no description provided
    #[serde(rename="seriesInfo")]
    pub series_info: Option<Volumeseriesinfo>,
    /// Total number of pages as per publisher metadata.
    #[serde(rename="pageCount")]
    pub page_count: Option<i32>,
    /// A list of image links for all the sizes that are available. (In LITE projection.)
    #[serde(rename="imageLinks")]
    pub image_links: Option<VolumeVolumeInfoImageLinks>,
    /// Whether the volume has comics content.
    #[serde(rename="comicsContent")]
    pub comics_content: Option<bool>,
    /// The main category to which this volume belongs. It will be the category from the categories list returned below that has the highest weight.
    #[serde(rename="mainCategory")]
    pub main_category: Option<String>,
    /// A top-level summary of the panelization info in this volume.
    #[serde(rename="panelizationSummary")]
    pub panelization_summary: Option<VolumeVolumeInfoPanelizationSummary>,
    /// The names of the authors and/or editors for this volume. (In LITE projection)
    pub authors: Option<Vec<String>>,
    /// The mean review rating for this volume. (min = 1.0, max = 5.0)
    #[serde(rename="averageRating")]
    pub average_rating: Option<f64>,
    /// A list of subject categories, such as "Fiction", "Suspense", etc.
    pub categories: Option<Vec<String>>,
    /// A synopsis of the volume. The text of the description is formatted in HTML and includes simple formatting elements, such as b, i, and br tags. (In LITE projection.)
    pub description: Option<String>,
    /// Publisher of this volume. (In LITE projection.)
    pub publisher: Option<String>,
    /// Physical dimensions of this volume.
    pub dimensions: Option<VolumeVolumeInfoDimensions>,
    /// Best language for this volume (based on content). It is the two-letter ISO 639-1 code such as 'fr', 'en', etc.
    pub language: Option<String>,
    /// URL to preview this volume on the Google Books site.
    #[serde(rename="previewLink")]
    pub preview_link: Option<String>,
    /// no description provided
    #[serde(rename="maturityRating")]
    pub maturity_rating: Option<String>,
    /// URL to view information about this volume on the Google Books site. (In LITE projection)
    #[serde(rename="infoLink")]
    pub info_link: Option<String>,
    /// Date of publication. (In LITE projection.)
    #[serde(rename="publishedDate")]
    pub published_date: Option<String>,
    /// Type of publication of this volume. Possible values are BOOK or MAGAZINE.
    #[serde(rename="printType")]
    pub print_type: Option<String>,
    /// Total number of sample pages as per publisher metadata.
    #[serde(rename="samplePageCount")]
    pub sample_page_count: Option<i32>,
    /// The reading modes available for this volume.
    #[serde(rename="readingModes")]
    pub reading_modes: Option<String>,
    /// An identifier for the version of the volume content (text & images). (In LITE projection)
    #[serde(rename="contentVersion")]
    pub content_version: Option<String>,
    /// Total number of printed pages in generated pdf representation.
    #[serde(rename="printedPageCount")]
    pub printed_page_count: Option<i32>,
    /// Industry standard identifiers for this volume.
    #[serde(rename="industryIdentifiers")]
    pub industry_identifiers: Option<Vec<VolumeVolumeInfoIndustryIdentifiers>>,
    /// Volume title. (In LITE projection.)
    pub title: Option<String>,
    /// The number of review ratings for this volume.
    #[serde(rename="ratingsCount")]
    pub ratings_count: Option<i32>,
    /// Whether anonymous logging should be allowed.
    #[serde(rename="allowAnonLogging")]
    pub allow_anon_logging: Option<bool>,
    /// Canonical URL for a volume. (In LITE projection.)
    #[serde(rename="canonicalVolumeLink")]
    pub canonical_volume_link: Option<String>,
}

impl NestedType for VolumeVolumeInfo {}
impl Part for VolumeVolumeInfo {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnnotationLayerSummary {
    /// Type of limitation on this layer. "limited" or "unlimited" for the "copy" layer.
    #[serde(rename="limitType")]
    pub limit_type: Option<String>,
    /// Remaining allowed characters on this layer, especially for the "copy" layer.
    #[serde(rename="remainingCharacterCount")]
    pub remaining_character_count: Option<i32>,
    /// Maximum allowed characters on this layer, especially for the "copy" layer.
    #[serde(rename="allowedCharacterCount")]
    pub allowed_character_count: Option<i32>,
}

impl NestedType for AnnotationLayerSummary {}
impl Part for AnnotationLayerSummary {}


/// The actual selling price of the book. This is the same as the suggested retail or list price unless there are offers or discounts on this volume. (In LITE projection.)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeSaleInfoRetailPrice {
    /// Amount in the currency listed below. (In LITE projection.)
    pub amount: Option<f64>,
    /// An ISO 4217, three-letter currency code. (In LITE projection.)
    #[serde(rename="currencyCode")]
    pub currency_code: Option<String>,
}

impl NestedType for VolumeSaleInfoRetailPrice {}
impl Part for VolumeSaleInfoRetailPrice {}


/// Offer list (=undiscounted) price in Micros.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeSaleInfoOffersListPrice {
    /// no description provided
    #[serde(rename="currencyCode")]
    pub currency_code: Option<String>,
    /// no description provided
    #[serde(rename="amountInMicros")]
    pub amount_in_micros: Option<f64>,
}

impl NestedType for VolumeSaleInfoOffersListPrice {}
impl Part for VolumeSaleInfoOffersListPrice {}


/// Family membership info of the user that made the request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FamilyInfoMembership {
    /// Restrictions on user buying and acquiring content.
    #[serde(rename="acquirePermission")]
    pub acquire_permission: Option<String>,
    /// The maximum allowed maturity rating for the user.
    #[serde(rename="allowedMaturityRating")]
    pub allowed_maturity_rating: Option<String>,
    /// The role of the user in the family.
    pub role: Option<String>,
    /// The age group of the user.
    #[serde(rename="ageGroup")]
    pub age_group: Option<String>,
    /// no description provided
    #[serde(rename="isInFamily")]
    pub is_in_family: Option<bool>,
}

impl NestedType for FamilyInfoMembership {}
impl Part for FamilyInfoMembership {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [volume annotations get layers](struct.LayerVolumeAnnotationGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Volumeannotation {
    /// The type of annotation this is.
    #[serde(rename="annotationType")]
    pub annotation_type: Option<String>,
    /// Resource Type
    pub kind: Option<String>,
    /// Indicates that this annotation is deleted.
    pub deleted: Option<bool>,
    /// The content ranges to identify the selected text.
    #[serde(rename="contentRanges")]
    pub content_ranges: Option<VolumeannotationContentRanges>,
    /// Timestamp for the last time this anntoation was updated. (RFC 3339 UTC date-time format).
    pub updated: Option<String>,
    /// The Volume this annotation is for.
    #[serde(rename="volumeId")]
    pub volume_id: Option<String>,
    /// The annotation data id for this volume annotation.
    #[serde(rename="annotationDataId")]
    pub annotation_data_id: Option<String>,
    /// Link to get data for this annotation.
    #[serde(rename="annotationDataLink")]
    pub annotation_data_link: Option<String>,
    /// Pages the annotation spans.
    #[serde(rename="pageIds")]
    pub page_ids: Option<Vec<String>>,
    /// The Layer this annotation is for.
    #[serde(rename="layerId")]
    pub layer_id: Option<String>,
    /// Excerpt from the volume.
    #[serde(rename="selectedText")]
    pub selected_text: Option<String>,
    /// Data for this annotation.
    pub data: Option<String>,
    /// Unique id of this volume annotation.
    pub id: Option<String>,
    /// URL to this resource.
    #[serde(rename="selfLink")]
    pub self_link: Option<String>,
}

impl ResponseResult for Volumeannotation {}


/// A list of offline dictionary metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetadataItems {
    /// no description provided
    pub encrypted_key: Option<String>,
    /// no description provided
    pub version: Option<String>,
    /// no description provided
    pub download_url: Option<String>,
    /// no description provided
    pub language: Option<String>,
    /// no description provided
    pub size: Option<String>,
}

impl NestedType for MetadataItems {}
impl Part for MetadataItems {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list category volumes onboarding](struct.OnboardingListCategoryVolumeCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Volume2 {
    /// no description provided
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// A list of volumes.
    pub items: Option<Vec<Volume>>,
    /// Resource type.
    pub kind: Option<String>,
}

impl ResponseResult for Volume2 {}


/// List of issues. Applicable only for Collection Edition and Omnibus.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeseriesinfoVolumeSeriesIssue {
    /// no description provided
    #[serde(rename="issueOrderNumber")]
    pub issue_order_number: Option<i32>,
    /// no description provided
    #[serde(rename="issueDisplayNumber")]
    pub issue_display_number: Option<String>,
}

impl NestedType for VolumeseriesinfoVolumeSeriesIssue {}
impl Part for VolumeseriesinfoVolumeSeriesIssue {}


/// Industry standard identifiers for this volume.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeVolumeInfoIndustryIdentifiers {
    /// Industry specific volume identifier.
    pub identifier: Option<String>,
    /// Identifier type. Possible values are ISBN_10, ISBN_13, ISSN and OTHER.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl NestedType for VolumeVolumeInfoIndustryIdentifiers {}
impl Part for VolumeVolumeInfoIndustryIdentifiers {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UsersettingsNotification {
    /// no description provided
    #[serde(rename="matchMyInterests")]
    pub match_my_interests: Option<UsersettingsNotificationMatchMyInterests>,
    /// no description provided
    #[serde(rename="rewardExpirations")]
    pub reward_expirations: Option<UsersettingsNotificationRewardExpirations>,
    /// no description provided
    #[serde(rename="moreFromSeries")]
    pub more_from_series: Option<UsersettingsNotificationMoreFromSeries>,
    /// no description provided
    #[serde(rename="moreFromAuthors")]
    pub more_from_authors: Option<UsersettingsNotificationMoreFromAuthors>,
    /// no description provided
    #[serde(rename="priceDrop")]
    pub price_drop: Option<UsersettingsNotificationPriceDrop>,
}

impl NestedType for UsersettingsNotification {}
impl Part for UsersettingsNotification {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [useruploaded list volumes](struct.VolumeUseruploadedListCall.html) (none)
/// * [recommended rate volumes](struct.VolumeRecommendedRateCall.html) (none)
/// * [list volumes](struct.VolumeListCall.html) (none)
/// * [associated list volumes](struct.VolumeAssociatedListCall.html) (none)
/// * [get volumes](struct.VolumeGetCall.html) (response)
/// * [recommended list volumes](struct.VolumeRecommendedListCall.html) (none)
/// * [mybooks list volumes](struct.VolumeMybookListCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Volume {
    /// Resource type for a volume. (In LITE projection.)
    pub kind: Option<String>,
    /// Any information about a volume related to reading or obtaining that volume text. This information can depend on country (books may be public domain in one country but not in another, e.g.).
    #[serde(rename="accessInfo")]
    pub access_info: Option<VolumeAccessInfo>,
    /// Search result information related to this volume.
    #[serde(rename="searchInfo")]
    pub search_info: Option<VolumeSearchInfo>,
    /// Any information about a volume related to the eBookstore and/or purchaseability. This information can depend on the country where the request originates from (i.e. books may not be for sale in certain countries).
    #[serde(rename="saleInfo")]
    pub sale_info: Option<VolumeSaleInfo>,
    /// Opaque identifier for a specific version of a volume resource. (In LITE projection)
    pub etag: Option<String>,
    /// What layers exist in this volume and high level information about them.
    #[serde(rename="layerInfo")]
    pub layer_info: Option<VolumeLayerInfo>,
    /// General volume information.
    #[serde(rename="volumeInfo")]
    pub volume_info: Option<VolumeVolumeInfo>,
    /// Recommendation related information for this volume.
    #[serde(rename="recommendedInfo")]
    pub recommended_info: Option<VolumeRecommendedInfo>,
    /// Unique identifier for a volume. (In LITE projection.)
    pub id: Option<String>,
    /// URL to this resource. (In LITE projection.)
    #[serde(rename="selfLink")]
    pub self_link: Option<String>,
    /// User specific information related to this volume. (e.g. page this user last read or whether they purchased this book)
    #[serde(rename="userInfo")]
    pub user_info: Option<VolumeUserInfo>,
}

impl Resource for Volume {}
impl ResponseResult for Volume {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [recommended rate volumes](struct.VolumeRecommendedRateCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BooksVolumesRecommendedRateResponse {
    /// no description provided
    pub consistency_token: Option<String>,
}

impl ResponseResult for BooksVolumesRecommendedRateResponse {}


/// User specific information related to this volume. (e.g. page this user last read or whether they purchased this book)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeUserInfo {
    /// Whether or not the user shared this volume with the family.
    #[serde(rename="isFamilySharedFromUser")]
    pub is_family_shared_from_user: Option<bool>,
    /// Period during this book is/was a valid rental.
    #[serde(rename="rentalPeriod")]
    pub rental_period: Option<VolumeUserInfoRentalPeriod>,
    /// How this volume was acquired.
    #[serde(rename="acquisitionType")]
    pub acquisition_type: Option<i32>,
    /// Whether or not this volume was purchased by the authenticated user making the request. (In LITE projection.)
    #[serde(rename="isPurchased")]
    pub is_purchased: Option<bool>,
    /// Timestamp when this volume was last modified by a user action, such as a reading position update, volume purchase or writing a review. (RFC 3339 UTC date-time format).
    pub updated: Option<String>,
    /// Whether or not this volume was pre-ordered by the authenticated user making the request. (In LITE projection.)
    #[serde(rename="isPreordered")]
    pub is_preordered: Option<bool>,
    /// Copy/Paste accounting information.
    pub copy: Option<VolumeUserInfoCopy>,
    /// Deprecated: Replaced by familySharing.
    #[serde(rename="isFamilySharingDisabledByFop")]
    pub is_family_sharing_disabled_by_fop: Option<bool>,
    /// Whether or not this volume was user uploaded.
    #[serde(rename="isUploaded")]
    pub is_uploaded: Option<bool>,
    /// Timestamp when this volume was acquired by the user. (RFC 3339 UTC date-time format) Acquiring includes purchase, user upload, receiving family sharing, etc.
    #[serde(rename="acquiredTime")]
    pub acquired_time: Option<String>,
    /// Whether or not this volume is currently in "my books."
    #[serde(rename="isInMyBooks")]
    pub is_in_my_books: Option<bool>,
    /// no description provided
    #[serde(rename="userUploadedVolumeInfo")]
    pub user_uploaded_volume_info: Option<VolumeUserInfoUserUploadedVolumeInfo>,
    /// Information on the ability to share with the family.
    #[serde(rename="familySharing")]
    pub family_sharing: Option<VolumeUserInfoFamilySharing>,
    /// Whether this book is an active or an expired rental.
    #[serde(rename="rentalState")]
    pub rental_state: Option<String>,
    /// Deprecated: Replaced by familySharing.
    #[serde(rename="isFamilySharingAllowed")]
    pub is_family_sharing_allowed: Option<bool>,
    /// The user's current reading position in the volume, if one is available. (In LITE projection.)
    #[serde(rename="readingPosition")]
    pub reading_position: Option<ReadingPosition>,
    /// Whether or not the user received this volume through family sharing.
    #[serde(rename="isFamilySharedToUser")]
    pub is_family_shared_to_user: Option<bool>,
    /// Whether this volume is purchased, sample, pd download etc.
    #[serde(rename="entitlementType")]
    pub entitlement_type: Option<i32>,
    /// This user's review of this volume, if one exists.
    pub review: Option<Review>,
}

impl NestedType for VolumeUserInfo {}
impl Part for VolumeUserInfo {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UsersettingsNotificationRewardExpirations {
    /// no description provided
    pub opted_state: Option<String>,
}

impl NestedType for UsersettingsNotificationRewardExpirations {}
impl Part for UsersettingsNotificationRewardExpirations {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SeriesSeries {
    /// no description provided
    #[serde(rename="seriesId")]
    pub series_id: Option<String>,
    /// no description provided
    #[serde(rename="bannerImageUrl")]
    pub banner_image_url: Option<String>,
    /// no description provided
    #[serde(rename="imageUrl")]
    pub image_url: Option<String>,
    /// no description provided
    #[serde(rename="seriesType")]
    pub series_type: Option<String>,
    /// no description provided
    pub title: Option<String>,
}

impl NestedType for SeriesSeries {}
impl Part for SeriesSeries {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeUserInfoUserUploadedVolumeInfo {
    /// no description provided
    #[serde(rename="processingState")]
    pub processing_state: Option<String>,
}

impl NestedType for VolumeUserInfoUserUploadedVolumeInfo {}
impl Part for VolumeUserInfoUserUploadedVolumeInfo {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BooksAnnotationsRange {
    /// The starting position for the range.
    #[serde(rename="startPosition")]
    pub start_position: Option<String>,
    /// The ending position for the range.
    #[serde(rename="endPosition")]
    pub end_position: Option<String>,
    /// The offset from the starting position.
    #[serde(rename="startOffset")]
    pub start_offset: Option<String>,
    /// The offset from the ending position.
    #[serde(rename="endOffset")]
    pub end_offset: Option<String>,
}

impl Part for BooksAnnotationsRange {}


/// Any information about a volume related to the eBookstore and/or purchaseability. This information can depend on the country where the request originates from (i.e. books may not be for sale in certain countries).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeSaleInfo {
    /// The two-letter ISO_3166-1 country code for which this sale information is valid. (In LITE projection.)
    pub country: Option<String>,
    /// The actual selling price of the book. This is the same as the suggested retail or list price unless there are offers or discounts on this volume. (In LITE projection.)
    #[serde(rename="retailPrice")]
    pub retail_price: Option<VolumeSaleInfoRetailPrice>,
    /// Whether or not this volume is an eBook (can be added to the My eBooks shelf).
    #[serde(rename="isEbook")]
    pub is_ebook: Option<bool>,
    /// Offers available for this volume (sales and rentals).
    pub offers: Option<Vec<VolumeSaleInfoOffers>>,
    /// Whether or not this book is available for sale or offered for free in the Google eBookstore for the country listed above. Possible values are FOR_SALE, FOR_RENTAL_ONLY, FOR_SALE_AND_RENTAL, FREE, NOT_FOR_SALE, or FOR_PREORDER.
    pub saleability: Option<String>,
    /// URL to purchase this volume on the Google Books site. (In LITE projection)
    #[serde(rename="buyLink")]
    pub buy_link: Option<String>,
    /// The date on which this book is available for sale.
    #[serde(rename="onSaleDate")]
    pub on_sale_date: Option<String>,
    /// Suggested retail price. (In LITE projection.)
    #[serde(rename="listPrice")]
    pub list_price: Option<VolumeSaleInfoListPrice>,
}

impl NestedType for VolumeSaleInfo {}
impl Part for VolumeSaleInfo {}


/// The content ranges to identify the selected text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeannotationContentRanges {
    /// Range in GB image format for this annotation for version above.
    #[serde(rename="gbImageRange")]
    pub gb_image_range: Option<BooksAnnotationsRange>,
    /// Range in GB text format for this annotation for version above.
    #[serde(rename="gbTextRange")]
    pub gb_text_range: Option<BooksAnnotationsRange>,
    /// Content version applicable to ranges below.
    #[serde(rename="contentVersion")]
    pub content_version: Option<String>,
    /// Range in CFI format for this annotation for version above.
    #[serde(rename="cfiRange")]
    pub cfi_range: Option<BooksAnnotationsRange>,
}

impl NestedType for VolumeannotationContentRanges {}
impl Part for VolumeannotationContentRanges {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UsersettingsNotificationPriceDrop {
    /// no description provided
    pub opted_state: Option<String>,
}

impl NestedType for UsersettingsNotificationPriceDrop {}
impl Part for UsersettingsNotificationPriceDrop {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get series](struct.SeryGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Series {
    /// no description provided
    pub series: Option<Vec<SeriesSeries>>,
    /// Resource type.
    pub kind: Option<String>,
}

impl ResponseResult for Series {}


/// A top-level summary of the panelization info in this volume.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeVolumeInfoPanelizationSummary {
    /// no description provided
    #[serde(rename="containsEpubBubbles")]
    pub contains_epub_bubbles: Option<bool>,
    /// no description provided
    #[serde(rename="containsImageBubbles")]
    pub contains_image_bubbles: Option<bool>,
    /// no description provided
    #[serde(rename="epubBubbleVersion")]
    pub epub_bubble_version: Option<String>,
    /// no description provided
    #[serde(rename="imageBubbleVersion")]
    pub image_bubble_version: Option<String>,
}

impl NestedType for VolumeVolumeInfoPanelizationSummary {}
impl Part for VolumeVolumeInfoPanelizationSummary {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnnotationsSummaryLayers {
    /// no description provided
    #[serde(rename="limitType")]
    pub limit_type: Option<String>,
    /// no description provided
    #[serde(rename="remainingCharacterCount")]
    pub remaining_character_count: Option<i32>,
    /// no description provided
    pub updated: Option<String>,
    /// no description provided
    #[serde(rename="allowedCharacterCount")]
    pub allowed_character_count: Option<i32>,
    /// no description provided
    #[serde(rename="layerId")]
    pub layer_id: Option<String>,
}

impl NestedType for AnnotationsSummaryLayers {}
impl Part for AnnotationsSummaryLayers {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *layer* resources.
/// It is not used directly, but through the `Books` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_books1 as books1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use books1::Books;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `annotation_data_get(...)`, `annotation_data_list(...)`, `get(...)`, `list(...)`, `volume_annotations_get(...)` and `volume_annotations_list(...)`
/// // to build up your call.
/// let rb = hub.layers();
/// # }
/// ```
pub struct LayerMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
}

impl<'a, C, A> MethodsBuilder for LayerMethods<'a, C, A> {}

impl<'a, C, A> LayerMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the annotation data.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - The volume to retrieve annotations for.
    /// * `layerId` - The ID for the layer to get the annotations.
    /// * `annotationDataId` - The ID of the annotation data to retrieve.
    /// * `contentVersion` - The content version for the volume you are trying to retrieve.
    pub fn annotation_data_get(&self, volume_id: &str, layer_id: &str, annotation_data_id: &str, content_version: &str) -> LayerAnnotationDataGetCall<'a, C, A> {
        LayerAnnotationDataGetCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _layer_id: layer_id.to_string(),
            _annotation_data_id: annotation_data_id.to_string(),
            _content_version: content_version.to_string(),
            _w: Default::default(),
            _source: Default::default(),
            _scale: Default::default(),
            _locale: Default::default(),
            _h: Default::default(),
            _allow_web_definitions: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the volume annotation.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - The volume to retrieve annotations for.
    /// * `layerId` - The ID for the layer to get the annotations.
    /// * `annotationId` - The ID of the volume annotation to retrieve.
    pub fn volume_annotations_get(&self, volume_id: &str, layer_id: &str, annotation_id: &str) -> LayerVolumeAnnotationGetCall<'a, C, A> {
        LayerVolumeAnnotationGetCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _layer_id: layer_id.to_string(),
            _annotation_id: annotation_id.to_string(),
            _source: Default::default(),
            _locale: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List the layer summaries for a volume.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - The volume to retrieve layers for.
    pub fn list(&self, volume_id: &str) -> LayerListCall<'a, C, A> {
        LayerListCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _source: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _content_version: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the layer summary for a volume.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - The volume to retrieve layers for.
    /// * `summaryId` - The ID for the layer to get the summary for.
    pub fn get(&self, volume_id: &str, summary_id: &str) -> LayerGetCall<'a, C, A> {
        LayerGetCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _summary_id: summary_id.to_string(),
            _source: Default::default(),
            _content_version: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the volume annotations for a volume and layer.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - The volume to retrieve annotations for.
    /// * `layerId` - The ID for the layer to get the annotations.
    /// * `contentVersion` - The content version for the requested volume.
    pub fn volume_annotations_list(&self, volume_id: &str, layer_id: &str, content_version: &str) -> LayerVolumeAnnotationListCall<'a, C, A> {
        LayerVolumeAnnotationListCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _layer_id: layer_id.to_string(),
            _content_version: content_version.to_string(),
            _volume_annotations_version: Default::default(),
            _updated_min: Default::default(),
            _updated_max: Default::default(),
            _start_position: Default::default(),
            _start_offset: Default::default(),
            _source: Default::default(),
            _show_deleted: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _locale: Default::default(),
            _end_position: Default::default(),
            _end_offset: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the annotation data for a volume and layer.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - The volume to retrieve annotation data for.
    /// * `layerId` - The ID for the layer to get the annotation data.
    /// * `contentVersion` - The content version for the requested volume.
    pub fn annotation_data_list(&self, volume_id: &str, layer_id: &str, content_version: &str) -> LayerAnnotationDataListCall<'a, C, A> {
        LayerAnnotationDataListCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _layer_id: layer_id.to_string(),
            _content_version: content_version.to_string(),
            _w: Default::default(),
            _updated_min: Default::default(),
            _updated_max: Default::default(),
            _source: Default::default(),
            _scale: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _locale: Default::default(),
            _h: Default::default(),
            _annotation_data_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *volume* resources.
/// It is not used directly, but through the `Books` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_books1 as books1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use books1::Books;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `associated_list(...)`, `get(...)`, `list(...)`, `mybooks_list(...)`, `recommended_list(...)`, `recommended_rate(...)` and `useruploaded_list(...)`
/// // to build up your call.
/// let rb = hub.volumes();
/// # }
/// ```
pub struct VolumeMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
}

impl<'a, C, A> MethodsBuilder for VolumeMethods<'a, C, A> {}

impl<'a, C, A> VolumeMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Rate a recommended book for the current user.
    /// 
    /// # Arguments
    ///
    /// * `rating` - Rating to be given to the volume.
    /// * `volumeId` - ID of the source volume.
    pub fn recommended_rate(&self, rating: &str, volume_id: &str) -> VolumeRecommendedRateCall<'a, C, A> {
        VolumeRecommendedRateCall {
            hub: self.hub,
            _rating: rating.to_string(),
            _volume_id: volume_id.to_string(),
            _source: Default::default(),
            _locale: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Return a list of books uploaded by the current user.
    pub fn useruploaded_list(&self) -> VolumeUseruploadedListCall<'a, C, A> {
        VolumeUseruploadedListCall {
            hub: self.hub,
            _volume_id: Default::default(),
            _start_index: Default::default(),
            _source: Default::default(),
            _processing_state: Default::default(),
            _max_results: Default::default(),
            _locale: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Return a list of books in My Library.
    pub fn mybooks_list(&self) -> VolumeMybookListCall<'a, C, A> {
        VolumeMybookListCall {
            hub: self.hub,
            _start_index: Default::default(),
            _source: Default::default(),
            _processing_state: Default::default(),
            _max_results: Default::default(),
            _locale: Default::default(),
            _country: Default::default(),
            _acquire_method: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Performs a book search.
    /// 
    /// # Arguments
    ///
    /// * `q` - Full-text search query string.
    pub fn list(&self, q: &str) -> VolumeListCall<'a, C, A> {
        VolumeListCall {
            hub: self.hub,
            _q: q.to_string(),
            _start_index: Default::default(),
            _source: Default::default(),
            _show_preorders: Default::default(),
            _projection: Default::default(),
            _print_type: Default::default(),
            _partner: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _max_allowed_maturity_rating: Default::default(),
            _library_restrict: Default::default(),
            _lang_restrict: Default::default(),
            _filter: Default::default(),
            _download: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Return a list of associated books.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - ID of the source volume.
    pub fn associated_list(&self, volume_id: &str) -> VolumeAssociatedListCall<'a, C, A> {
        VolumeAssociatedListCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _source: Default::default(),
            _max_allowed_maturity_rating: Default::default(),
            _locale: Default::default(),
            _association: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets volume information for a single volume.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - ID of volume to retrieve.
    pub fn get(&self, volume_id: &str) -> VolumeGetCall<'a, C, A> {
        VolumeGetCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _user_library_consistent_read: Default::default(),
            _source: Default::default(),
            _projection: Default::default(),
            _partner: Default::default(),
            _include_non_comics_series: Default::default(),
            _country: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Return a list of recommended books for the current user.
    pub fn recommended_list(&self) -> VolumeRecommendedListCall<'a, C, A> {
        VolumeRecommendedListCall {
            hub: self.hub,
            _source: Default::default(),
            _max_allowed_maturity_rating: Default::default(),
            _locale: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *dictionary* resources.
/// It is not used directly, but through the `Books` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_books1 as books1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use books1::Books;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list_offline_metadata(...)`
/// // to build up your call.
/// let rb = hub.dictionary();
/// # }
/// ```
pub struct DictionaryMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
}

impl<'a, C, A> MethodsBuilder for DictionaryMethods<'a, C, A> {}

impl<'a, C, A> DictionaryMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of offline dictionary metadata available
    /// 
    /// # Arguments
    ///
    /// * `cpksver` - The device/version ID from which to request the data.
    pub fn list_offline_metadata(&self, cpksver: &str) -> DictionaryListOfflineMetadataCall<'a, C, A> {
        DictionaryListOfflineMetadataCall {
            hub: self.hub,
            _cpksver: cpksver.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *familysharing* resources.
/// It is not used directly, but through the `Books` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_books1 as books1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use books1::Books;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_family_info(...)`, `share(...)` and `unshare(...)`
/// // to build up your call.
/// let rb = hub.familysharing();
/// # }
/// ```
pub struct FamilysharingMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
}

impl<'a, C, A> MethodsBuilder for FamilysharingMethods<'a, C, A> {}

impl<'a, C, A> FamilysharingMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Initiates revoking content that has already been shared with the user's family. Empty response indicates success.
    pub fn unshare(&self) -> FamilysharingUnshareCall<'a, C, A> {
        FamilysharingUnshareCall {
            hub: self.hub,
            _volume_id: Default::default(),
            _source: Default::default(),
            _doc_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information regarding the family that the user is part of.
    pub fn get_family_info(&self) -> FamilysharingGetFamilyInfoCall<'a, C, A> {
        FamilysharingGetFamilyInfoCall {
            hub: self.hub,
            _source: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Initiates sharing of the content with the user's family. Empty response indicates success.
    pub fn share(&self) -> FamilysharingShareCall<'a, C, A> {
        FamilysharingShareCall {
            hub: self.hub,
            _volume_id: Default::default(),
            _source: Default::default(),
            _doc_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *bookshelve* resources.
/// It is not used directly, but through the `Books` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_books1 as books1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use books1::Books;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `list(...)` and `volumes_list(...)`
/// // to build up your call.
/// let rb = hub.bookshelves();
/// # }
/// ```
pub struct BookshelveMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
}

impl<'a, C, A> MethodsBuilder for BookshelveMethods<'a, C, A> {}

impl<'a, C, A> BookshelveMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves volumes in a specific bookshelf for the specified user.
    /// 
    /// # Arguments
    ///
    /// * `userId` - ID of user for whom to retrieve bookshelf volumes.
    /// * `shelf` - ID of bookshelf to retrieve volumes.
    pub fn volumes_list(&self, user_id: &str, shelf: &str) -> BookshelveVolumeListCall<'a, C, A> {
        BookshelveVolumeListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _shelf: shelf.to_string(),
            _start_index: Default::default(),
            _source: Default::default(),
            _show_preorders: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of public bookshelves for the specified user.
    /// 
    /// # Arguments
    ///
    /// * `userId` - ID of user for whom to retrieve bookshelves.
    pub fn list(&self, user_id: &str) -> BookshelveListCall<'a, C, A> {
        BookshelveListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _source: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves metadata for a specific bookshelf for the specified user.
    /// 
    /// # Arguments
    ///
    /// * `userId` - ID of user for whom to retrieve bookshelves.
    /// * `shelf` - ID of bookshelf to retrieve.
    pub fn get(&self, user_id: &str, shelf: &str) -> BookshelveGetCall<'a, C, A> {
        BookshelveGetCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _shelf: shelf.to_string(),
            _source: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *promooffer* resources.
/// It is not used directly, but through the `Books` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_books1 as books1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use books1::Books;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `accept(...)`, `dismiss(...)` and `get(...)`
/// // to build up your call.
/// let rb = hub.promooffer();
/// # }
/// ```
pub struct PromoofferMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
}

impl<'a, C, A> MethodsBuilder for PromoofferMethods<'a, C, A> {}

impl<'a, C, A> PromoofferMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// 
    pub fn accept(&self) -> PromoofferAcceptCall<'a, C, A> {
        PromoofferAcceptCall {
            hub: self.hub,
            _volume_id: Default::default(),
            _serial: Default::default(),
            _product: Default::default(),
            _offer_id: Default::default(),
            _model: Default::default(),
            _manufacturer: Default::default(),
            _device: Default::default(),
            _android_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// 
    pub fn dismiss(&self) -> PromoofferDismisCall<'a, C, A> {
        PromoofferDismisCall {
            hub: self.hub,
            _serial: Default::default(),
            _product: Default::default(),
            _offer_id: Default::default(),
            _model: Default::default(),
            _manufacturer: Default::default(),
            _device: Default::default(),
            _android_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of promo offers available to the user
    pub fn get(&self) -> PromoofferGetCall<'a, C, A> {
        PromoofferGetCall {
            hub: self.hub,
            _serial: Default::default(),
            _product: Default::default(),
            _model: Default::default(),
            _manufacturer: Default::default(),
            _device: Default::default(),
            _android_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *personalizedstream* resources.
/// It is not used directly, but through the `Books` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_books1 as books1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use books1::Books;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.personalizedstream();
/// # }
/// ```
pub struct PersonalizedstreamMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
}

impl<'a, C, A> MethodsBuilder for PersonalizedstreamMethods<'a, C, A> {}

impl<'a, C, A> PersonalizedstreamMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a stream of personalized book clusters
    pub fn get(&self) -> PersonalizedstreamGetCall<'a, C, A> {
        PersonalizedstreamGetCall {
            hub: self.hub,
            _source: Default::default(),
            _max_allowed_maturity_rating: Default::default(),
            _locale: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *onboarding* resources.
/// It is not used directly, but through the `Books` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_books1 as books1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use books1::Books;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list_categories(...)` and `list_category_volumes(...)`
/// // to build up your call.
/// let rb = hub.onboarding();
/// # }
/// ```
pub struct OnboardingMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
}

impl<'a, C, A> MethodsBuilder for OnboardingMethods<'a, C, A> {}

impl<'a, C, A> OnboardingMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// List categories for onboarding experience.
    pub fn list_categories(&self) -> OnboardingListCategoryCall<'a, C, A> {
        OnboardingListCategoryCall {
            hub: self.hub,
            _locale: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List available volumes under categories for onboarding experience.
    pub fn list_category_volumes(&self) -> OnboardingListCategoryVolumeCall<'a, C, A> {
        OnboardingListCategoryVolumeCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _max_allowed_maturity_rating: Default::default(),
            _locale: Default::default(),
            _category_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *myconfig* resources.
/// It is not used directly, but through the `Books` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_books1 as books1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use books1::Books;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_user_settings(...)`, `release_download_access(...)`, `request_access(...)`, `sync_volume_licenses(...)` and `update_user_settings(...)`
/// // to build up your call.
/// let rb = hub.myconfig();
/// # }
/// ```
pub struct MyconfigMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
}

impl<'a, C, A> MethodsBuilder for MyconfigMethods<'a, C, A> {}

impl<'a, C, A> MyconfigMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Request concurrent and download access restrictions.
    /// 
    /// # Arguments
    ///
    /// * `source` - String to identify the originator of this request.
    /// * `volumeId` - The volume to request concurrent/download restrictions for.
    /// * `nonce` - The client nonce value.
    /// * `cpksver` - The device/version ID from which to request the restrictions.
    pub fn request_access(&self, source: &str, volume_id: &str, nonce: &str, cpksver: &str) -> MyconfigRequestAccesCall<'a, C, A> {
        MyconfigRequestAccesCall {
            hub: self.hub,
            _source: source.to_string(),
            _volume_id: volume_id.to_string(),
            _nonce: nonce.to_string(),
            _cpksver: cpksver.to_string(),
            _locale: Default::default(),
            _license_types: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Release downloaded content access restriction.
    /// 
    /// # Arguments
    ///
    /// * `volumeIds` - The volume(s) to release restrictions for.
    /// * `cpksver` - The device/version ID from which to release the restriction.
    pub fn release_download_access(&self, volume_ids: &Vec<String>, cpksver: &str) -> MyconfigReleaseDownloadAccesCall<'a, C, A> {
        MyconfigReleaseDownloadAccesCall {
            hub: self.hub,
            _volume_ids: volume_ids.clone(),
            _cpksver: cpksver.to_string(),
            _source: Default::default(),
            _locale: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Request downloaded content access for specified volumes on the My eBooks shelf.
    /// 
    /// # Arguments
    ///
    /// * `source` - String to identify the originator of this request.
    /// * `nonce` - The client nonce value.
    /// * `cpksver` - The device/version ID from which to release the restriction.
    pub fn sync_volume_licenses(&self, source: &str, nonce: &str, cpksver: &str) -> MyconfigSyncVolumeLicenseCall<'a, C, A> {
        MyconfigSyncVolumeLicenseCall {
            hub: self.hub,
            _source: source.to_string(),
            _nonce: nonce.to_string(),
            _cpksver: cpksver.to_string(),
            _volume_ids: Default::default(),
            _show_preorders: Default::default(),
            _locale: Default::default(),
            _include_non_comics_series: Default::default(),
            _features: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the current settings for the user.
    pub fn get_user_settings(&self) -> MyconfigGetUserSettingCall<'a, C, A> {
        MyconfigGetUserSettingCall {
            hub: self.hub,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the settings for the user. If a sub-object is specified, it will overwrite the existing sub-object stored in the server. Unspecified sub-objects will retain the existing value.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn update_user_settings(&self, request: Usersettings) -> MyconfigUpdateUserSettingCall<'a, C, A> {
        MyconfigUpdateUserSettingCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *mylibrary* resources.
/// It is not used directly, but through the `Books` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_books1 as books1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use books1::Books;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `annotations_delete(...)`, `annotations_insert(...)`, `annotations_list(...)`, `annotations_summary(...)`, `annotations_update(...)`, `bookshelves_add_volume(...)`, `bookshelves_clear_volumes(...)`, `bookshelves_get(...)`, `bookshelves_list(...)`, `bookshelves_move_volume(...)`, `bookshelves_remove_volume(...)`, `bookshelves_volumes_list(...)`, `readingpositions_get(...)` and `readingpositions_set_position(...)`
/// // to build up your call.
/// let rb = hub.mylibrary();
/// # }
/// ```
pub struct MylibraryMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
}

impl<'a, C, A> MethodsBuilder for MylibraryMethods<'a, C, A> {}

impl<'a, C, A> MylibraryMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Clears all volumes from a bookshelf.
    /// 
    /// # Arguments
    ///
    /// * `shelf` - ID of bookshelf from which to remove a volume.
    pub fn bookshelves_clear_volumes(&self, shelf: &str) -> MylibraryBookshelveClearVolumeCall<'a, C, A> {
        MylibraryBookshelveClearVolumeCall {
            hub: self.hub,
            _shelf: shelf.to_string(),
            _source: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Moves a volume within a bookshelf.
    /// 
    /// # Arguments
    ///
    /// * `shelf` - ID of bookshelf with the volume.
    /// * `volumeId` - ID of volume to move.
    /// * `volumePosition` - Position on shelf to move the item (0 puts the item before the current first item, 1 puts it between the first and the second and so on.)
    pub fn bookshelves_move_volume(&self, shelf: &str, volume_id: &str, volume_position: i32) -> MylibraryBookshelveMoveVolumeCall<'a, C, A> {
        MylibraryBookshelveMoveVolumeCall {
            hub: self.hub,
            _shelf: shelf.to_string(),
            _volume_id: volume_id.to_string(),
            _volume_position: volume_position,
            _source: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets volume information for volumes on a bookshelf.
    /// 
    /// # Arguments
    ///
    /// * `shelf` - The bookshelf ID or name retrieve volumes for.
    pub fn bookshelves_volumes_list(&self, shelf: &str) -> MylibraryBookshelveVolumeListCall<'a, C, A> {
        MylibraryBookshelveVolumeListCall {
            hub: self.hub,
            _shelf: shelf.to_string(),
            _start_index: Default::default(),
            _source: Default::default(),
            _show_preorders: Default::default(),
            _q: Default::default(),
            _projection: Default::default(),
            _max_results: Default::default(),
            _country: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the summary of specified layers.
    /// 
    /// # Arguments
    ///
    /// * `layerIds` - Array of layer IDs to get the summary for.
    /// * `volumeId` - Volume id to get the summary for.
    pub fn annotations_summary(&self, layer_ids: &Vec<String>, volume_id: &str) -> MylibraryAnnotationSummaryCall<'a, C, A> {
        MylibraryAnnotationSummaryCall {
            hub: self.hub,
            _layer_ids: layer_ids.clone(),
            _volume_id: volume_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an annotation.
    /// 
    /// # Arguments
    ///
    /// * `annotationId` - The ID for the annotation to delete.
    pub fn annotations_delete(&self, annotation_id: &str) -> MylibraryAnnotationDeleteCall<'a, C, A> {
        MylibraryAnnotationDeleteCall {
            hub: self.hub,
            _annotation_id: annotation_id.to_string(),
            _source: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a volume to a bookshelf.
    /// 
    /// # Arguments
    ///
    /// * `shelf` - ID of bookshelf to which to add a volume.
    /// * `volumeId` - ID of volume to add.
    pub fn bookshelves_add_volume(&self, shelf: &str, volume_id: &str) -> MylibraryBookshelveAddVolumeCall<'a, C, A> {
        MylibraryBookshelveAddVolumeCall {
            hub: self.hub,
            _shelf: shelf.to_string(),
            _volume_id: volume_id.to_string(),
            _source: Default::default(),
            _reason: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new annotation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn annotations_insert(&self, request: Annotation) -> MylibraryAnnotationInsertCall<'a, C, A> {
        MylibraryAnnotationInsertCall {
            hub: self.hub,
            _request: request,
            _source: Default::default(),
            _show_only_summary_in_response: Default::default(),
            _country: Default::default(),
            _annotation_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes a volume from a bookshelf.
    /// 
    /// # Arguments
    ///
    /// * `shelf` - ID of bookshelf from which to remove a volume.
    /// * `volumeId` - ID of volume to remove.
    pub fn bookshelves_remove_volume(&self, shelf: &str, volume_id: &str) -> MylibraryBookshelveRemoveVolumeCall<'a, C, A> {
        MylibraryBookshelveRemoveVolumeCall {
            hub: self.hub,
            _shelf: shelf.to_string(),
            _volume_id: volume_id.to_string(),
            _source: Default::default(),
            _reason: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of annotations, possibly filtered.
    pub fn annotations_list(&self) -> MylibraryAnnotationListCall<'a, C, A> {
        MylibraryAnnotationListCall {
            hub: self.hub,
            _volume_id: Default::default(),
            _updated_min: Default::default(),
            _updated_max: Default::default(),
            _source: Default::default(),
            _show_deleted: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _layer_ids: Default::default(),
            _layer_id: Default::default(),
            _content_version: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing annotation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `annotationId` - The ID for the annotation to update.
    pub fn annotations_update(&self, request: Annotation, annotation_id: &str) -> MylibraryAnnotationUpdateCall<'a, C, A> {
        MylibraryAnnotationUpdateCall {
            hub: self.hub,
            _request: request,
            _annotation_id: annotation_id.to_string(),
            _source: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets my reading position information for a volume.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - ID of volume for which to update the reading position.
    /// * `timestamp` - RFC 3339 UTC format timestamp associated with this reading position.
    /// * `position` - Position string for the new volume reading position.
    pub fn readingpositions_set_position(&self, volume_id: &str, timestamp: &str, position: &str) -> MylibraryReadingpositionSetPositionCall<'a, C, A> {
        MylibraryReadingpositionSetPositionCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _timestamp: timestamp.to_string(),
            _position: position.to_string(),
            _source: Default::default(),
            _device_cookie: Default::default(),
            _content_version: Default::default(),
            _action: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves metadata for a specific bookshelf belonging to the authenticated user.
    /// 
    /// # Arguments
    ///
    /// * `shelf` - ID of bookshelf to retrieve.
    pub fn bookshelves_get(&self, shelf: &str) -> MylibraryBookshelveGetCall<'a, C, A> {
        MylibraryBookshelveGetCall {
            hub: self.hub,
            _shelf: shelf.to_string(),
            _source: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of bookshelves belonging to the authenticated user.
    pub fn bookshelves_list(&self) -> MylibraryBookshelveListCall<'a, C, A> {
        MylibraryBookshelveListCall {
            hub: self.hub,
            _source: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves my reading position information for a volume.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - ID of volume for which to retrieve a reading position.
    pub fn readingpositions_get(&self, volume_id: &str) -> MylibraryReadingpositionGetCall<'a, C, A> {
        MylibraryReadingpositionGetCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _source: Default::default(),
            _content_version: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *sery* resources.
/// It is not used directly, but through the `Books` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_books1 as books1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use books1::Books;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `membership_get(...)`
/// // to build up your call.
/// let rb = hub.series();
/// # }
/// ```
pub struct SeryMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
}

impl<'a, C, A> MethodsBuilder for SeryMethods<'a, C, A> {}

impl<'a, C, A> SeryMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns Series metadata for the given series ids.
    /// 
    /// # Arguments
    ///
    /// * `series_id` - String that identifies the series
    pub fn get(&self, series_id: &Vec<String>) -> SeryGetCall<'a, C, A> {
        SeryGetCall {
            hub: self.hub,
            _series_id: series_id.clone(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns Series membership data given the series id.
    /// 
    /// # Arguments
    ///
    /// * `series_id` - String that identifies the series
    pub fn membership_get(&self, series_id: &str) -> SeryMembershipGetCall<'a, C, A> {
        SeryMembershipGetCall {
            hub: self.hub,
            _series_id: series_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *cloudloading* resources.
/// It is not used directly, but through the `Books` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_books1 as books1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use books1::Books;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `add_book(...)`, `delete_book(...)` and `update_book(...)`
/// // to build up your call.
/// let rb = hub.cloudloading();
/// # }
/// ```
pub struct CloudloadingMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
}

impl<'a, C, A> MethodsBuilder for CloudloadingMethods<'a, C, A> {}

impl<'a, C, A> CloudloadingMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// 
    pub fn add_book(&self) -> CloudloadingAddBookCall<'a, C, A> {
        CloudloadingAddBookCall {
            hub: self.hub,
            _upload_client_token: Default::default(),
            _name: Default::default(),
            _mime_type: Default::default(),
            _drive_document_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// 
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn update_book(&self, request: BooksCloudloadingResource) -> CloudloadingUpdateBookCall<'a, C, A> {
        CloudloadingUpdateBookCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Remove the book and its contents
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - The id of the book to be removed.
    pub fn delete_book(&self, volume_id: &str) -> CloudloadingDeleteBookCall<'a, C, A> {
        CloudloadingDeleteBookCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *notification* resources.
/// It is not used directly, but through the `Books` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_books1 as books1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use books1::Books;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.notification();
/// # }
/// ```
pub struct NotificationMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
}

impl<'a, C, A> MethodsBuilder for NotificationMethods<'a, C, A> {}

impl<'a, C, A> NotificationMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns notification details for a given notification id.
    /// 
    /// # Arguments
    ///
    /// * `notification_id` - String to identify the notification.
    pub fn get(&self, notification_id: &str) -> NotificationGetCall<'a, C, A> {
        NotificationGetCall {
            hub: self.hub,
            _notification_id: notification_id.to_string(),
            _source: Default::default(),
            _locale: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Gets the annotation data.
///
/// A builder for the *annotationData.get* method supported by a *layer* resource.
/// It is not used directly, but through a `LayerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.layers().annotation_data_get("volumeId", "layerId", "annotationDataId", "contentVersion")
///              .w(-73)
///              .source("Lorem")
///              .scale(-9)
///              .locale("et")
///              .h(-70)
///              .allow_web_definitions(true)
///              .doit();
/// # }
/// ```
pub struct LayerAnnotationDataGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _volume_id: String,
    _layer_id: String,
    _annotation_data_id: String,
    _content_version: String,
    _w: Option<i32>,
    _source: Option<String>,
    _scale: Option<i32>,
    _locale: Option<String>,
    _h: Option<i32>,
    _allow_web_definitions: Option<bool>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for LayerAnnotationDataGetCall<'a, C, A> {}

impl<'a, C, A> LayerAnnotationDataGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Annotationdata)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.layers.annotationData.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(12 + self._additional_params.len());
        params.push(("volumeId", self._volume_id.to_string()));
        params.push(("layerId", self._layer_id.to_string()));
        params.push(("annotationDataId", self._annotation_data_id.to_string()));
        params.push(("contentVersion", self._content_version.to_string()));
        if let Some(value) = self._w {
            params.push(("w", value.to_string()));
        }
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._scale {
            params.push(("scale", value.to_string()));
        }
        if let Some(value) = self._locale {
            params.push(("locale", value.to_string()));
        }
        if let Some(value) = self._h {
            params.push(("h", value.to_string()));
        }
        if let Some(value) = self._allow_web_definitions {
            params.push(("allowWebDefinitions", value.to_string()));
        }
        for &field in ["alt", "volumeId", "layerId", "annotationDataId", "contentVersion", "w", "source", "scale", "locale", "h", "allowWebDefinitions"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "volumes/{volumeId}/layers/{layerId}/data/{annotationDataId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{volumeId}", "volumeId"), ("{layerId}", "layerId"), ("{annotationDataId}", "annotationDataId")].iter() {
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
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["annotationDataId", "layerId", "volumeId"].iter() {
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


    /// The volume to retrieve annotations for.
    ///
    /// Sets the *volume id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn volume_id(mut self, new_value: &str) -> LayerAnnotationDataGetCall<'a, C, A> {
        self._volume_id = new_value.to_string();
        self
    }
    /// The ID for the layer to get the annotations.
    ///
    /// Sets the *layer id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn layer_id(mut self, new_value: &str) -> LayerAnnotationDataGetCall<'a, C, A> {
        self._layer_id = new_value.to_string();
        self
    }
    /// The ID of the annotation data to retrieve.
    ///
    /// Sets the *annotation data id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn annotation_data_id(mut self, new_value: &str) -> LayerAnnotationDataGetCall<'a, C, A> {
        self._annotation_data_id = new_value.to_string();
        self
    }
    /// The content version for the volume you are trying to retrieve.
    ///
    /// Sets the *content version* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn content_version(mut self, new_value: &str) -> LayerAnnotationDataGetCall<'a, C, A> {
        self._content_version = new_value.to_string();
        self
    }
    /// The requested pixel width for any images. If width is provided height must also be provided.
    ///
    /// Sets the *w* query property to the given value.
    pub fn w(mut self, new_value: i32) -> LayerAnnotationDataGetCall<'a, C, A> {
        self._w = Some(new_value);
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> LayerAnnotationDataGetCall<'a, C, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The requested scale for the image.
    ///
    /// Sets the *scale* query property to the given value.
    pub fn scale(mut self, new_value: i32) -> LayerAnnotationDataGetCall<'a, C, A> {
        self._scale = Some(new_value);
        self
    }
    /// The locale information for the data. ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'.
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> LayerAnnotationDataGetCall<'a, C, A> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// The requested pixel height for any images. If height is provided width must also be provided.
    ///
    /// Sets the *h* query property to the given value.
    pub fn h(mut self, new_value: i32) -> LayerAnnotationDataGetCall<'a, C, A> {
        self._h = Some(new_value);
        self
    }
    /// For the dictionary layer. Whether or not to allow web definitions.
    ///
    /// Sets the *allow web definitions* query property to the given value.
    pub fn allow_web_definitions(mut self, new_value: bool) -> LayerAnnotationDataGetCall<'a, C, A> {
        self._allow_web_definitions = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LayerAnnotationDataGetCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> LayerAnnotationDataGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> LayerAnnotationDataGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets the volume annotation.
///
/// A builder for the *volumeAnnotations.get* method supported by a *layer* resource.
/// It is not used directly, but through a `LayerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.layers().volume_annotations_get("volumeId", "layerId", "annotationId")
///              .source("amet")
///              .locale("et")
///              .doit();
/// # }
/// ```
pub struct LayerVolumeAnnotationGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _volume_id: String,
    _layer_id: String,
    _annotation_id: String,
    _source: Option<String>,
    _locale: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for LayerVolumeAnnotationGetCall<'a, C, A> {}

impl<'a, C, A> LayerVolumeAnnotationGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Volumeannotation)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.layers.volumeAnnotations.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("volumeId", self._volume_id.to_string()));
        params.push(("layerId", self._layer_id.to_string()));
        params.push(("annotationId", self._annotation_id.to_string()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._locale {
            params.push(("locale", value.to_string()));
        }
        for &field in ["alt", "volumeId", "layerId", "annotationId", "source", "locale"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "volumes/{volumeId}/layers/{layerId}/annotations/{annotationId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{volumeId}", "volumeId"), ("{layerId}", "layerId"), ("{annotationId}", "annotationId")].iter() {
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
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["annotationId", "layerId", "volumeId"].iter() {
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


    /// The volume to retrieve annotations for.
    ///
    /// Sets the *volume id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn volume_id(mut self, new_value: &str) -> LayerVolumeAnnotationGetCall<'a, C, A> {
        self._volume_id = new_value.to_string();
        self
    }
    /// The ID for the layer to get the annotations.
    ///
    /// Sets the *layer id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn layer_id(mut self, new_value: &str) -> LayerVolumeAnnotationGetCall<'a, C, A> {
        self._layer_id = new_value.to_string();
        self
    }
    /// The ID of the volume annotation to retrieve.
    ///
    /// Sets the *annotation id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn annotation_id(mut self, new_value: &str) -> LayerVolumeAnnotationGetCall<'a, C, A> {
        self._annotation_id = new_value.to_string();
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> LayerVolumeAnnotationGetCall<'a, C, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The locale information for the data. ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'.
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> LayerVolumeAnnotationGetCall<'a, C, A> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LayerVolumeAnnotationGetCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> LayerVolumeAnnotationGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> LayerVolumeAnnotationGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// List the layer summaries for a volume.
///
/// A builder for the *list* method supported by a *layer* resource.
/// It is not used directly, but through a `LayerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.layers().list("volumeId")
///              .source("ut")
///              .page_token("ea")
///              .max_results(21)
///              .content_version("dolor")
///              .doit();
/// # }
/// ```
pub struct LayerListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _volume_id: String,
    _source: Option<String>,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _content_version: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for LayerListCall<'a, C, A> {}

impl<'a, C, A> LayerListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Layersummaries)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.layers.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("volumeId", self._volume_id.to_string()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._content_version {
            params.push(("contentVersion", value.to_string()));
        }
        for &field in ["alt", "volumeId", "source", "pageToken", "maxResults", "contentVersion"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "volumes/{volumeId}/layersummary";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{volumeId}", "volumeId")].iter() {
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
            for param_name in ["volumeId"].iter() {
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


    /// The volume to retrieve layers for.
    ///
    /// Sets the *volume id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn volume_id(mut self, new_value: &str) -> LayerListCall<'a, C, A> {
        self._volume_id = new_value.to_string();
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> LayerListCall<'a, C, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The value of the nextToken from the previous page.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> LayerListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of results to return
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> LayerListCall<'a, C, A> {
        self._max_results = Some(new_value);
        self
    }
    /// The content version for the requested volume.
    ///
    /// Sets the *content version* query property to the given value.
    pub fn content_version(mut self, new_value: &str) -> LayerListCall<'a, C, A> {
        self._content_version = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LayerListCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> LayerListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> LayerListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets the layer summary for a volume.
///
/// A builder for the *get* method supported by a *layer* resource.
/// It is not used directly, but through a `LayerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.layers().get("volumeId", "summaryId")
///              .source("et")
///              .content_version("consetetur")
///              .doit();
/// # }
/// ```
pub struct LayerGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _volume_id: String,
    _summary_id: String,
    _source: Option<String>,
    _content_version: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for LayerGetCall<'a, C, A> {}

impl<'a, C, A> LayerGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Layersummary)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.layers.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("volumeId", self._volume_id.to_string()));
        params.push(("summaryId", self._summary_id.to_string()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._content_version {
            params.push(("contentVersion", value.to_string()));
        }
        for &field in ["alt", "volumeId", "summaryId", "source", "contentVersion"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "volumes/{volumeId}/layersummary/{summaryId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{volumeId}", "volumeId"), ("{summaryId}", "summaryId")].iter() {
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
            for param_name in ["summaryId", "volumeId"].iter() {
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


    /// The volume to retrieve layers for.
    ///
    /// Sets the *volume id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn volume_id(mut self, new_value: &str) -> LayerGetCall<'a, C, A> {
        self._volume_id = new_value.to_string();
        self
    }
    /// The ID for the layer to get the summary for.
    ///
    /// Sets the *summary id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn summary_id(mut self, new_value: &str) -> LayerGetCall<'a, C, A> {
        self._summary_id = new_value.to_string();
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> LayerGetCall<'a, C, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The content version for the requested volume.
    ///
    /// Sets the *content version* query property to the given value.
    pub fn content_version(mut self, new_value: &str) -> LayerGetCall<'a, C, A> {
        self._content_version = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LayerGetCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> LayerGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> LayerGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets the volume annotations for a volume and layer.
///
/// A builder for the *volumeAnnotations.list* method supported by a *layer* resource.
/// It is not used directly, but through a `LayerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.layers().volume_annotations_list("volumeId", "layerId", "contentVersion")
///              .volume_annotations_version("gubergren")
///              .updated_min("justo")
///              .updated_max("sit")
///              .start_position("vero")
///              .start_offset("diam")
///              .source("rebum.")
///              .show_deleted(true)
///              .page_token("sadipscing")
///              .max_results(25)
///              .locale("sadipscing")
///              .end_position("invidunt")
///              .end_offset("consetetur")
///              .doit();
/// # }
/// ```
pub struct LayerVolumeAnnotationListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _volume_id: String,
    _layer_id: String,
    _content_version: String,
    _volume_annotations_version: Option<String>,
    _updated_min: Option<String>,
    _updated_max: Option<String>,
    _start_position: Option<String>,
    _start_offset: Option<String>,
    _source: Option<String>,
    _show_deleted: Option<bool>,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _locale: Option<String>,
    _end_position: Option<String>,
    _end_offset: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for LayerVolumeAnnotationListCall<'a, C, A> {}

impl<'a, C, A> LayerVolumeAnnotationListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Volumeannotations)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.layers.volumeAnnotations.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(17 + self._additional_params.len());
        params.push(("volumeId", self._volume_id.to_string()));
        params.push(("layerId", self._layer_id.to_string()));
        params.push(("contentVersion", self._content_version.to_string()));
        if let Some(value) = self._volume_annotations_version {
            params.push(("volumeAnnotationsVersion", value.to_string()));
        }
        if let Some(value) = self._updated_min {
            params.push(("updatedMin", value.to_string()));
        }
        if let Some(value) = self._updated_max {
            params.push(("updatedMax", value.to_string()));
        }
        if let Some(value) = self._start_position {
            params.push(("startPosition", value.to_string()));
        }
        if let Some(value) = self._start_offset {
            params.push(("startOffset", value.to_string()));
        }
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._show_deleted {
            params.push(("showDeleted", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._locale {
            params.push(("locale", value.to_string()));
        }
        if let Some(value) = self._end_position {
            params.push(("endPosition", value.to_string()));
        }
        if let Some(value) = self._end_offset {
            params.push(("endOffset", value.to_string()));
        }
        for &field in ["alt", "volumeId", "layerId", "contentVersion", "volumeAnnotationsVersion", "updatedMin", "updatedMax", "startPosition", "startOffset", "source", "showDeleted", "pageToken", "maxResults", "locale", "endPosition", "endOffset"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "volumes/{volumeId}/layers/{layerId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{volumeId}", "volumeId"), ("{layerId}", "layerId")].iter() {
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
            for param_name in ["layerId", "volumeId"].iter() {
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


    /// The volume to retrieve annotations for.
    ///
    /// Sets the *volume id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn volume_id(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, C, A> {
        self._volume_id = new_value.to_string();
        self
    }
    /// The ID for the layer to get the annotations.
    ///
    /// Sets the *layer id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn layer_id(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, C, A> {
        self._layer_id = new_value.to_string();
        self
    }
    /// The content version for the requested volume.
    ///
    /// Sets the *content version* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn content_version(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, C, A> {
        self._content_version = new_value.to_string();
        self
    }
    /// The version of the volume annotations that you are requesting.
    ///
    /// Sets the *volume annotations version* query property to the given value.
    pub fn volume_annotations_version(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, C, A> {
        self._volume_annotations_version = Some(new_value.to_string());
        self
    }
    /// RFC 3339 timestamp to restrict to items updated since this timestamp (inclusive).
    ///
    /// Sets the *updated min* query property to the given value.
    pub fn updated_min(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, C, A> {
        self._updated_min = Some(new_value.to_string());
        self
    }
    /// RFC 3339 timestamp to restrict to items updated prior to this timestamp (exclusive).
    ///
    /// Sets the *updated max* query property to the given value.
    pub fn updated_max(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, C, A> {
        self._updated_max = Some(new_value.to_string());
        self
    }
    /// The start position to start retrieving data from.
    ///
    /// Sets the *start position* query property to the given value.
    pub fn start_position(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, C, A> {
        self._start_position = Some(new_value.to_string());
        self
    }
    /// The start offset to start retrieving data from.
    ///
    /// Sets the *start offset* query property to the given value.
    pub fn start_offset(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, C, A> {
        self._start_offset = Some(new_value.to_string());
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, C, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Set to true to return deleted annotations. updatedMin must be in the request to use this. Defaults to false.
    ///
    /// Sets the *show deleted* query property to the given value.
    pub fn show_deleted(mut self, new_value: bool) -> LayerVolumeAnnotationListCall<'a, C, A> {
        self._show_deleted = Some(new_value);
        self
    }
    /// The value of the nextToken from the previous page.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of results to return
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> LayerVolumeAnnotationListCall<'a, C, A> {
        self._max_results = Some(new_value);
        self
    }
    /// The locale information for the data. ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'.
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, C, A> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// The end position to end retrieving data from.
    ///
    /// Sets the *end position* query property to the given value.
    pub fn end_position(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, C, A> {
        self._end_position = Some(new_value.to_string());
        self
    }
    /// The end offset to end retrieving data from.
    ///
    /// Sets the *end offset* query property to the given value.
    pub fn end_offset(mut self, new_value: &str) -> LayerVolumeAnnotationListCall<'a, C, A> {
        self._end_offset = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LayerVolumeAnnotationListCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> LayerVolumeAnnotationListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> LayerVolumeAnnotationListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets the annotation data for a volume and layer.
///
/// A builder for the *annotationData.list* method supported by a *layer* resource.
/// It is not used directly, but through a `LayerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.layers().annotation_data_list("volumeId", "layerId", "contentVersion")
///              .w(-5)
///              .updated_min("et")
///              .updated_max("clita")
///              .source("consetetur")
///              .scale(-58)
///              .page_token("nonumy")
///              .max_results(88)
///              .locale("sanctus")
///              .h(-58)
///              .add_annotation_data_id("At")
///              .doit();
/// # }
/// ```
pub struct LayerAnnotationDataListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _volume_id: String,
    _layer_id: String,
    _content_version: String,
    _w: Option<i32>,
    _updated_min: Option<String>,
    _updated_max: Option<String>,
    _source: Option<String>,
    _scale: Option<i32>,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _locale: Option<String>,
    _h: Option<i32>,
    _annotation_data_id: Vec<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for LayerAnnotationDataListCall<'a, C, A> {}

impl<'a, C, A> LayerAnnotationDataListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Annotationsdata)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.layers.annotationData.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(15 + self._additional_params.len());
        params.push(("volumeId", self._volume_id.to_string()));
        params.push(("layerId", self._layer_id.to_string()));
        params.push(("contentVersion", self._content_version.to_string()));
        if let Some(value) = self._w {
            params.push(("w", value.to_string()));
        }
        if let Some(value) = self._updated_min {
            params.push(("updatedMin", value.to_string()));
        }
        if let Some(value) = self._updated_max {
            params.push(("updatedMax", value.to_string()));
        }
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._scale {
            params.push(("scale", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._locale {
            params.push(("locale", value.to_string()));
        }
        if let Some(value) = self._h {
            params.push(("h", value.to_string()));
        }
        if self._annotation_data_id.len() > 0 {
            for f in self._annotation_data_id.iter() {
                params.push(("annotationDataId", f.to_string()));
            }
        }
        for &field in ["alt", "volumeId", "layerId", "contentVersion", "w", "updatedMin", "updatedMax", "source", "scale", "pageToken", "maxResults", "locale", "h", "annotationDataId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "volumes/{volumeId}/layers/{layerId}/data";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{volumeId}", "volumeId"), ("{layerId}", "layerId")].iter() {
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
            for param_name in ["layerId", "volumeId"].iter() {
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


    /// The volume to retrieve annotation data for.
    ///
    /// Sets the *volume id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn volume_id(mut self, new_value: &str) -> LayerAnnotationDataListCall<'a, C, A> {
        self._volume_id = new_value.to_string();
        self
    }
    /// The ID for the layer to get the annotation data.
    ///
    /// Sets the *layer id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn layer_id(mut self, new_value: &str) -> LayerAnnotationDataListCall<'a, C, A> {
        self._layer_id = new_value.to_string();
        self
    }
    /// The content version for the requested volume.
    ///
    /// Sets the *content version* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn content_version(mut self, new_value: &str) -> LayerAnnotationDataListCall<'a, C, A> {
        self._content_version = new_value.to_string();
        self
    }
    /// The requested pixel width for any images. If width is provided height must also be provided.
    ///
    /// Sets the *w* query property to the given value.
    pub fn w(mut self, new_value: i32) -> LayerAnnotationDataListCall<'a, C, A> {
        self._w = Some(new_value);
        self
    }
    /// RFC 3339 timestamp to restrict to items updated since this timestamp (inclusive).
    ///
    /// Sets the *updated min* query property to the given value.
    pub fn updated_min(mut self, new_value: &str) -> LayerAnnotationDataListCall<'a, C, A> {
        self._updated_min = Some(new_value.to_string());
        self
    }
    /// RFC 3339 timestamp to restrict to items updated prior to this timestamp (exclusive).
    ///
    /// Sets the *updated max* query property to the given value.
    pub fn updated_max(mut self, new_value: &str) -> LayerAnnotationDataListCall<'a, C, A> {
        self._updated_max = Some(new_value.to_string());
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> LayerAnnotationDataListCall<'a, C, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The requested scale for the image.
    ///
    /// Sets the *scale* query property to the given value.
    pub fn scale(mut self, new_value: i32) -> LayerAnnotationDataListCall<'a, C, A> {
        self._scale = Some(new_value);
        self
    }
    /// The value of the nextToken from the previous page.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> LayerAnnotationDataListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of results to return
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> LayerAnnotationDataListCall<'a, C, A> {
        self._max_results = Some(new_value);
        self
    }
    /// The locale information for the data. ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'.
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> LayerAnnotationDataListCall<'a, C, A> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// The requested pixel height for any images. If height is provided width must also be provided.
    ///
    /// Sets the *h* query property to the given value.
    pub fn h(mut self, new_value: i32) -> LayerAnnotationDataListCall<'a, C, A> {
        self._h = Some(new_value);
        self
    }
    /// The list of Annotation Data Ids to retrieve. Pagination is ignored if this is set.
    ///
    /// Append the given value to the *annotation data id* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_annotation_data_id(mut self, new_value: &str) -> LayerAnnotationDataListCall<'a, C, A> {
        self._annotation_data_id.push(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LayerAnnotationDataListCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> LayerAnnotationDataListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> LayerAnnotationDataListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Rate a recommended book for the current user.
///
/// A builder for the *recommended.rate* method supported by a *volume* resource.
/// It is not used directly, but through a `VolumeMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.volumes().recommended_rate("rating", "volumeId")
///              .source("ea")
///              .locale("sadipscing")
///              .doit();
/// # }
/// ```
pub struct VolumeRecommendedRateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _rating: String,
    _volume_id: String,
    _source: Option<String>,
    _locale: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for VolumeRecommendedRateCall<'a, C, A> {}

impl<'a, C, A> VolumeRecommendedRateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, BooksVolumesRecommendedRateResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.volumes.recommended.rate",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("rating", self._rating.to_string()));
        params.push(("volumeId", self._volume_id.to_string()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._locale {
            params.push(("locale", value.to_string()));
        }
        for &field in ["alt", "rating", "volumeId", "source", "locale"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "volumes/recommended/rate";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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


    /// Rating to be given to the volume.
    ///
    /// Sets the *rating* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn rating(mut self, new_value: &str) -> VolumeRecommendedRateCall<'a, C, A> {
        self._rating = new_value.to_string();
        self
    }
    /// ID of the source volume.
    ///
    /// Sets the *volume id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn volume_id(mut self, new_value: &str) -> VolumeRecommendedRateCall<'a, C, A> {
        self._volume_id = new_value.to_string();
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> VolumeRecommendedRateCall<'a, C, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'. Used for generating recommendations.
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> VolumeRecommendedRateCall<'a, C, A> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> VolumeRecommendedRateCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> VolumeRecommendedRateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> VolumeRecommendedRateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Return a list of books uploaded by the current user.
///
/// A builder for the *useruploaded.list* method supported by a *volume* resource.
/// It is not used directly, but through a `VolumeMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.volumes().useruploaded_list()
///              .add_volume_id("rebum.")
///              .start_index(68)
///              .source("nonumy")
///              .add_processing_state("sed")
///              .max_results(19)
///              .locale("sit")
///              .doit();
/// # }
/// ```
pub struct VolumeUseruploadedListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _volume_id: Vec<String>,
    _start_index: Option<u32>,
    _source: Option<String>,
    _processing_state: Vec<String>,
    _max_results: Option<u32>,
    _locale: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for VolumeUseruploadedListCall<'a, C, A> {}

impl<'a, C, A> VolumeUseruploadedListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Volumes)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.volumes.useruploaded.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(8 + self._additional_params.len());
        if self._volume_id.len() > 0 {
            for f in self._volume_id.iter() {
                params.push(("volumeId", f.to_string()));
            }
        }
        if let Some(value) = self._start_index {
            params.push(("startIndex", value.to_string()));
        }
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if self._processing_state.len() > 0 {
            for f in self._processing_state.iter() {
                params.push(("processingState", f.to_string()));
            }
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._locale {
            params.push(("locale", value.to_string()));
        }
        for &field in ["alt", "volumeId", "startIndex", "source", "processingState", "maxResults", "locale"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "volumes/useruploaded";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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


    /// The ids of the volumes to be returned. If not specified all that match the processingState are returned.
    ///
    /// Append the given value to the *volume id* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_volume_id(mut self, new_value: &str) -> VolumeUseruploadedListCall<'a, C, A> {
        self._volume_id.push(new_value.to_string());
        self
    }
    /// Index of the first result to return (starts at 0)
    ///
    /// Sets the *start index* query property to the given value.
    pub fn start_index(mut self, new_value: u32) -> VolumeUseruploadedListCall<'a, C, A> {
        self._start_index = Some(new_value);
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> VolumeUseruploadedListCall<'a, C, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The processing state of the user uploaded volumes to be returned.
    ///
    /// Append the given value to the *processing state* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_processing_state(mut self, new_value: &str) -> VolumeUseruploadedListCall<'a, C, A> {
        self._processing_state.push(new_value.to_string());
        self
    }
    /// Maximum number of results to return.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> VolumeUseruploadedListCall<'a, C, A> {
        self._max_results = Some(new_value);
        self
    }
    /// ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'. Used for generating recommendations.
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> VolumeUseruploadedListCall<'a, C, A> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> VolumeUseruploadedListCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> VolumeUseruploadedListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> VolumeUseruploadedListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Return a list of books in My Library.
///
/// A builder for the *mybooks.list* method supported by a *volume* resource.
/// It is not used directly, but through a `VolumeMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.volumes().mybooks_list()
///              .start_index(61)
///              .source("consetetur")
///              .add_processing_state("labore")
///              .max_results(71)
///              .locale("ea")
///              .country("gubergren")
///              .add_acquire_method("aliquyam")
///              .doit();
/// # }
/// ```
pub struct VolumeMybookListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _start_index: Option<u32>,
    _source: Option<String>,
    _processing_state: Vec<String>,
    _max_results: Option<u32>,
    _locale: Option<String>,
    _country: Option<String>,
    _acquire_method: Vec<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for VolumeMybookListCall<'a, C, A> {}

impl<'a, C, A> VolumeMybookListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Volumes)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.volumes.mybooks.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(9 + self._additional_params.len());
        if let Some(value) = self._start_index {
            params.push(("startIndex", value.to_string()));
        }
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if self._processing_state.len() > 0 {
            for f in self._processing_state.iter() {
                params.push(("processingState", f.to_string()));
            }
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._locale {
            params.push(("locale", value.to_string()));
        }
        if let Some(value) = self._country {
            params.push(("country", value.to_string()));
        }
        if self._acquire_method.len() > 0 {
            for f in self._acquire_method.iter() {
                params.push(("acquireMethod", f.to_string()));
            }
        }
        for &field in ["alt", "startIndex", "source", "processingState", "maxResults", "locale", "country", "acquireMethod"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "volumes/mybooks";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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


    /// Index of the first result to return (starts at 0)
    ///
    /// Sets the *start index* query property to the given value.
    pub fn start_index(mut self, new_value: u32) -> VolumeMybookListCall<'a, C, A> {
        self._start_index = Some(new_value);
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> VolumeMybookListCall<'a, C, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The processing state of the user uploaded volumes to be returned. Applicable only if the UPLOADED is specified in the acquireMethod.
    ///
    /// Append the given value to the *processing state* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_processing_state(mut self, new_value: &str) -> VolumeMybookListCall<'a, C, A> {
        self._processing_state.push(new_value.to_string());
        self
    }
    /// Maximum number of results to return.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> VolumeMybookListCall<'a, C, A> {
        self._max_results = Some(new_value);
        self
    }
    /// ISO-639-1 language and ISO-3166-1 country code. Ex:'en_US'. Used for generating recommendations.
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> VolumeMybookListCall<'a, C, A> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// ISO-3166-1 code to override the IP-based location.
    ///
    /// Sets the *country* query property to the given value.
    pub fn country(mut self, new_value: &str) -> VolumeMybookListCall<'a, C, A> {
        self._country = Some(new_value.to_string());
        self
    }
    /// How the book was acquired
    ///
    /// Append the given value to the *acquire method* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_acquire_method(mut self, new_value: &str) -> VolumeMybookListCall<'a, C, A> {
        self._acquire_method.push(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> VolumeMybookListCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> VolumeMybookListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> VolumeMybookListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Performs a book search.
///
/// A builder for the *list* method supported by a *volume* resource.
/// It is not used directly, but through a `VolumeMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.volumes().list("q")
///              .start_index(63)
///              .source("sea")
///              .show_preorders(false)
///              .projection("ipsum")
///              .print_type("aliquyam")
///              .partner("dolores")
///              .order_by("sit")
///              .max_results(60)
///              .max_allowed_maturity_rating("ut")
///              .library_restrict("justo")
///              .lang_restrict("est")
///              .filter("amet")
///              .download("accusam")
///              .doit();
/// # }
/// ```
pub struct VolumeListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _q: String,
    _start_index: Option<u32>,
    _source: Option<String>,
    _show_preorders: Option<bool>,
    _projection: Option<String>,
    _print_type: Option<String>,
    _partner: Option<String>,
    _order_by: Option<String>,
    _max_results: Option<u32>,
    _max_allowed_maturity_rating: Option<String>,
    _library_restrict: Option<String>,
    _lang_restrict: Option<String>,
    _filter: Option<String>,
    _download: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for VolumeListCall<'a, C, A> {}

impl<'a, C, A> VolumeListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Volumes)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.volumes.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(16 + self._additional_params.len());
        params.push(("q", self._q.to_string()));
        if let Some(value) = self._start_index {
            params.push(("startIndex", value.to_string()));
        }
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._show_preorders {
            params.push(("showPreorders", value.to_string()));
        }
        if let Some(value) = self._projection {
            params.push(("projection", value.to_string()));
        }
        if let Some(value) = self._print_type {
            params.push(("printType", value.to_string()));
        }
        if let Some(value) = self._partner {
            params.push(("partner", value.to_string()));
        }
        if let Some(value) = self._order_by {
            params.push(("orderBy", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._max_allowed_maturity_rating {
            params.push(("maxAllowedMaturityRating", value.to_string()));
        }
        if let Some(value) = self._library_restrict {
            params.push(("libraryRestrict", value.to_string()));
        }
        if let Some(value) = self._lang_restrict {
            params.push(("langRestrict", value.to_string()));
        }
        if let Some(value) = self._filter {
            params.push(("filter", value.to_string()));
        }
        if let Some(value) = self._download {
            params.push(("download", value.to_string()));
        }
        for &field in ["alt", "q", "startIndex", "source", "showPreorders", "projection", "printType", "partner", "orderBy", "maxResults", "maxAllowedMaturityRating", "libraryRestrict", "langRestrict", "filter", "download"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "volumes";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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


    /// Full-text search query string.
    ///
    /// Sets the *q* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn q(mut self, new_value: &str) -> VolumeListCall<'a, C, A> {
        self._q = new_value.to_string();
        self
    }
    /// Index of the first result to return (starts at 0)
    ///
    /// Sets the *start index* query property to the given value.
    pub fn start_index(mut self, new_value: u32) -> VolumeListCall<'a, C, A> {
        self._start_index = Some(new_value);
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> VolumeListCall<'a, C, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Set to true to show books available for preorder. Defaults to false.
    ///
    /// Sets the *show preorders* query property to the given value.
    pub fn show_preorders(mut self, new_value: bool) -> VolumeListCall<'a, C, A> {
        self._show_preorders = Some(new_value);
        self
    }
    /// Restrict information returned to a set of selected fields.
    ///
    /// Sets the *projection* query property to the given value.
    pub fn projection(mut self, new_value: &str) -> VolumeListCall<'a, C, A> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Restrict to books or magazines.
    ///
    /// Sets the *print type* query property to the given value.
    pub fn print_type(mut self, new_value: &str) -> VolumeListCall<'a, C, A> {
        self._print_type = Some(new_value.to_string());
        self
    }
    /// Restrict and brand results for partner ID.
    ///
    /// Sets the *partner* query property to the given value.
    pub fn partner(mut self, new_value: &str) -> VolumeListCall<'a, C, A> {
        self._partner = Some(new_value.to_string());
        self
    }
    /// Sort search results.
    ///
    /// Sets the *order by* query property to the given value.
    pub fn order_by(mut self, new_value: &str) -> VolumeListCall<'a, C, A> {
        self._order_by = Some(new_value.to_string());
        self
    }
    /// Maximum number of results to return.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> VolumeListCall<'a, C, A> {
        self._max_results = Some(new_value);
        self
    }
    /// The maximum allowed maturity rating of returned recommendations. Books with a higher maturity rating are filtered out.
    ///
    /// Sets the *max allowed maturity rating* query property to the given value.
    pub fn max_allowed_maturity_rating(mut self, new_value: &str) -> VolumeListCall<'a, C, A> {
        self._max_allowed_maturity_rating = Some(new_value.to_string());
        self
    }
    /// Restrict search to this user's library.
    ///
    /// Sets the *library restrict* query property to the given value.
    pub fn library_restrict(mut self, new_value: &str) -> VolumeListCall<'a, C, A> {
        self._library_restrict = Some(new_value.to_string());
        self
    }
    /// Restrict results to books with this language code.
    ///
    /// Sets the *lang restrict* query property to the given value.
    pub fn lang_restrict(mut self, new_value: &str) -> VolumeListCall<'a, C, A> {
        self._lang_restrict = Some(new_value.to_string());
        self
    }
    /// Filter search results.
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> VolumeListCall<'a, C, A> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// Restrict to volumes by download availability.
    ///
    /// Sets the *download* query property to the given value.
    pub fn download(mut self, new_value: &str) -> VolumeListCall<'a, C, A> {
        self._download = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> VolumeListCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> VolumeListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> VolumeListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Return a list of associated books.
///
/// A builder for the *associated.list* method supported by a *volume* resource.
/// It is not used directly, but through a `VolumeMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.volumes().associated_list("volumeId")
///              .source("diam")
///              .max_allowed_maturity_rating("justo")
///              .locale("est")
///              .association("clita")
///              .doit();
/// # }
/// ```
pub struct VolumeAssociatedListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _volume_id: String,
    _source: Option<String>,
    _max_allowed_maturity_rating: Option<String>,
    _locale: Option<String>,
    _association: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for VolumeAssociatedListCall<'a, C, A> {}

impl<'a, C, A> VolumeAssociatedListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Volumes)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.volumes.associated.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("volumeId", self._volume_id.to_string()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._max_allowed_maturity_rating {
            params.push(("maxAllowedMaturityRating", value.to_string()));
        }
        if let Some(value) = self._locale {
            params.push(("locale", value.to_string()));
        }
        if let Some(value) = self._association {
            params.push(("association", value.to_string()));
        }
        for &field in ["alt", "volumeId", "source", "maxAllowedMaturityRating", "locale", "association"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "volumes/{volumeId}/associated";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{volumeId}", "volumeId")].iter() {
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
            for param_name in ["volumeId"].iter() {
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


    /// ID of the source volume.
    ///
    /// Sets the *volume id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn volume_id(mut self, new_value: &str) -> VolumeAssociatedListCall<'a, C, A> {
        self._volume_id = new_value.to_string();
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> VolumeAssociatedListCall<'a, C, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The maximum allowed maturity rating of returned recommendations. Books with a higher maturity rating are filtered out.
    ///
    /// Sets the *max allowed maturity rating* query property to the given value.
    pub fn max_allowed_maturity_rating(mut self, new_value: &str) -> VolumeAssociatedListCall<'a, C, A> {
        self._max_allowed_maturity_rating = Some(new_value.to_string());
        self
    }
    /// ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'. Used for generating recommendations.
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> VolumeAssociatedListCall<'a, C, A> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// Association type.
    ///
    /// Sets the *association* query property to the given value.
    pub fn association(mut self, new_value: &str) -> VolumeAssociatedListCall<'a, C, A> {
        self._association = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> VolumeAssociatedListCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> VolumeAssociatedListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> VolumeAssociatedListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets volume information for a single volume.
///
/// A builder for the *get* method supported by a *volume* resource.
/// It is not used directly, but through a `VolumeMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.volumes().get("volumeId")
///              .user_library_consistent_read(false)
///              .source("dolores")
///              .projection("eos")
///              .partner("voluptua.")
///              .include_non_comics_series(true)
///              .country("sed")
///              .doit();
/// # }
/// ```
pub struct VolumeGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _volume_id: String,
    _user_library_consistent_read: Option<bool>,
    _source: Option<String>,
    _projection: Option<String>,
    _partner: Option<String>,
    _include_non_comics_series: Option<bool>,
    _country: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for VolumeGetCall<'a, C, A> {}

impl<'a, C, A> VolumeGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Volume)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.volumes.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(9 + self._additional_params.len());
        params.push(("volumeId", self._volume_id.to_string()));
        if let Some(value) = self._user_library_consistent_read {
            params.push(("user_library_consistent_read", value.to_string()));
        }
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._projection {
            params.push(("projection", value.to_string()));
        }
        if let Some(value) = self._partner {
            params.push(("partner", value.to_string()));
        }
        if let Some(value) = self._include_non_comics_series {
            params.push(("includeNonComicsSeries", value.to_string()));
        }
        if let Some(value) = self._country {
            params.push(("country", value.to_string()));
        }
        for &field in ["alt", "volumeId", "user_library_consistent_read", "source", "projection", "partner", "includeNonComicsSeries", "country"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "volumes/{volumeId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{volumeId}", "volumeId")].iter() {
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
            for param_name in ["volumeId"].iter() {
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


    /// ID of volume to retrieve.
    ///
    /// Sets the *volume id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn volume_id(mut self, new_value: &str) -> VolumeGetCall<'a, C, A> {
        self._volume_id = new_value.to_string();
        self
    }
    ///
    /// Sets the *user_library_consistent_read* query property to the given value.
    pub fn user_library_consistent_read(mut self, new_value: bool) -> VolumeGetCall<'a, C, A> {
        self._user_library_consistent_read = Some(new_value);
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> VolumeGetCall<'a, C, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Restrict information returned to a set of selected fields.
    ///
    /// Sets the *projection* query property to the given value.
    pub fn projection(mut self, new_value: &str) -> VolumeGetCall<'a, C, A> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Brand results for partner ID.
    ///
    /// Sets the *partner* query property to the given value.
    pub fn partner(mut self, new_value: &str) -> VolumeGetCall<'a, C, A> {
        self._partner = Some(new_value.to_string());
        self
    }
    /// Set to true to include non-comics series. Defaults to false.
    ///
    /// Sets the *include non comics series* query property to the given value.
    pub fn include_non_comics_series(mut self, new_value: bool) -> VolumeGetCall<'a, C, A> {
        self._include_non_comics_series = Some(new_value);
        self
    }
    /// ISO-3166-1 code to override the IP-based location.
    ///
    /// Sets the *country* query property to the given value.
    pub fn country(mut self, new_value: &str) -> VolumeGetCall<'a, C, A> {
        self._country = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> VolumeGetCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> VolumeGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> VolumeGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Return a list of recommended books for the current user.
///
/// A builder for the *recommended.list* method supported by a *volume* resource.
/// It is not used directly, but through a `VolumeMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.volumes().recommended_list()
///              .source("aliquyam")
///              .max_allowed_maturity_rating("ea")
///              .locale("ea")
///              .doit();
/// # }
/// ```
pub struct VolumeRecommendedListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _source: Option<String>,
    _max_allowed_maturity_rating: Option<String>,
    _locale: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for VolumeRecommendedListCall<'a, C, A> {}

impl<'a, C, A> VolumeRecommendedListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Volumes)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.volumes.recommended.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._max_allowed_maturity_rating {
            params.push(("maxAllowedMaturityRating", value.to_string()));
        }
        if let Some(value) = self._locale {
            params.push(("locale", value.to_string()));
        }
        for &field in ["alt", "source", "maxAllowedMaturityRating", "locale"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "volumes/recommended";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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


    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> VolumeRecommendedListCall<'a, C, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The maximum allowed maturity rating of returned recommendations. Books with a higher maturity rating are filtered out.
    ///
    /// Sets the *max allowed maturity rating* query property to the given value.
    pub fn max_allowed_maturity_rating(mut self, new_value: &str) -> VolumeRecommendedListCall<'a, C, A> {
        self._max_allowed_maturity_rating = Some(new_value.to_string());
        self
    }
    /// ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'. Used for generating recommendations.
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> VolumeRecommendedListCall<'a, C, A> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> VolumeRecommendedListCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> VolumeRecommendedListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> VolumeRecommendedListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns a list of offline dictionary metadata available
///
/// A builder for the *listOfflineMetadata* method supported by a *dictionary* resource.
/// It is not used directly, but through a `DictionaryMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.dictionary().list_offline_metadata("cpksver")
///              .doit();
/// # }
/// ```
pub struct DictionaryListOfflineMetadataCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _cpksver: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for DictionaryListOfflineMetadataCall<'a, C, A> {}

impl<'a, C, A> DictionaryListOfflineMetadataCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Metadata)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.dictionary.listOfflineMetadata",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        params.push(("cpksver", self._cpksver.to_string()));
        for &field in ["alt", "cpksver"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "dictionary/listOfflineMetadata";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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


    /// The device/version ID from which to request the data.
    ///
    /// Sets the *cpksver* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn cpksver(mut self, new_value: &str) -> DictionaryListOfflineMetadataCall<'a, C, A> {
        self._cpksver = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DictionaryListOfflineMetadataCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> DictionaryListOfflineMetadataCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> DictionaryListOfflineMetadataCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Initiates revoking content that has already been shared with the user's family. Empty response indicates success.
///
/// A builder for the *unshare* method supported by a *familysharing* resource.
/// It is not used directly, but through a `FamilysharingMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.familysharing().unshare()
///              .volume_id("dolor")
///              .source("diam")
///              .doc_id("kasd")
///              .doit();
/// # }
/// ```
pub struct FamilysharingUnshareCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _volume_id: Option<String>,
    _source: Option<String>,
    _doc_id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for FamilysharingUnshareCall<'a, C, A> {}

impl<'a, C, A> FamilysharingUnshareCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.familysharing.unshare",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        if let Some(value) = self._volume_id {
            params.push(("volumeId", value.to_string()));
        }
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._doc_id {
            params.push(("docId", value.to_string()));
        }
        for &field in ["volumeId", "source", "docId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "familysharing/unshare";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The volume to unshare.
    ///
    /// Sets the *volume id* query property to the given value.
    pub fn volume_id(mut self, new_value: &str) -> FamilysharingUnshareCall<'a, C, A> {
        self._volume_id = Some(new_value.to_string());
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> FamilysharingUnshareCall<'a, C, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The docid to unshare.
    ///
    /// Sets the *doc id* query property to the given value.
    pub fn doc_id(mut self, new_value: &str) -> FamilysharingUnshareCall<'a, C, A> {
        self._doc_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> FamilysharingUnshareCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> FamilysharingUnshareCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> FamilysharingUnshareCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets information regarding the family that the user is part of.
///
/// A builder for the *getFamilyInfo* method supported by a *familysharing* resource.
/// It is not used directly, but through a `FamilysharingMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.familysharing().get_family_info()
///              .source("invidunt")
///              .doit();
/// # }
/// ```
pub struct FamilysharingGetFamilyInfoCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _source: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for FamilysharingGetFamilyInfoCall<'a, C, A> {}

impl<'a, C, A> FamilysharingGetFamilyInfoCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, FamilyInfo)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.familysharing.getFamilyInfo",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        for &field in ["alt", "source"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "familysharing/getFamilyInfo";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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


    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> FamilysharingGetFamilyInfoCall<'a, C, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> FamilysharingGetFamilyInfoCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> FamilysharingGetFamilyInfoCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> FamilysharingGetFamilyInfoCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Initiates sharing of the content with the user's family. Empty response indicates success.
///
/// A builder for the *share* method supported by a *familysharing* resource.
/// It is not used directly, but through a `FamilysharingMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.familysharing().share()
///              .volume_id("rebum.")
///              .source("Lorem")
///              .doc_id("clita")
///              .doit();
/// # }
/// ```
pub struct FamilysharingShareCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _volume_id: Option<String>,
    _source: Option<String>,
    _doc_id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for FamilysharingShareCall<'a, C, A> {}

impl<'a, C, A> FamilysharingShareCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.familysharing.share",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        if let Some(value) = self._volume_id {
            params.push(("volumeId", value.to_string()));
        }
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._doc_id {
            params.push(("docId", value.to_string()));
        }
        for &field in ["volumeId", "source", "docId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "familysharing/share";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The volume to share.
    ///
    /// Sets the *volume id* query property to the given value.
    pub fn volume_id(mut self, new_value: &str) -> FamilysharingShareCall<'a, C, A> {
        self._volume_id = Some(new_value.to_string());
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> FamilysharingShareCall<'a, C, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The docid to share.
    ///
    /// Sets the *doc id* query property to the given value.
    pub fn doc_id(mut self, new_value: &str) -> FamilysharingShareCall<'a, C, A> {
        self._doc_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> FamilysharingShareCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> FamilysharingShareCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> FamilysharingShareCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves volumes in a specific bookshelf for the specified user.
///
/// A builder for the *volumes.list* method supported by a *bookshelve* resource.
/// It is not used directly, but through a `BookshelveMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.bookshelves().volumes_list("userId", "shelf")
///              .start_index(24)
///              .source("consetetur")
///              .show_preorders(true)
///              .max_results(21)
///              .doit();
/// # }
/// ```
pub struct BookshelveVolumeListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _user_id: String,
    _shelf: String,
    _start_index: Option<u32>,
    _source: Option<String>,
    _show_preorders: Option<bool>,
    _max_results: Option<u32>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for BookshelveVolumeListCall<'a, C, A> {}

impl<'a, C, A> BookshelveVolumeListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Volumes)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.bookshelves.volumes.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(8 + self._additional_params.len());
        params.push(("userId", self._user_id.to_string()));
        params.push(("shelf", self._shelf.to_string()));
        if let Some(value) = self._start_index {
            params.push(("startIndex", value.to_string()));
        }
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._show_preorders {
            params.push(("showPreorders", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        for &field in ["alt", "userId", "shelf", "startIndex", "source", "showPreorders", "maxResults"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "users/{userId}/bookshelves/{shelf}/volumes";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{userId}", "userId"), ("{shelf}", "shelf")].iter() {
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
            for param_name in ["shelf", "userId"].iter() {
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


    /// ID of user for whom to retrieve bookshelf volumes.
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> BookshelveVolumeListCall<'a, C, A> {
        self._user_id = new_value.to_string();
        self
    }
    /// ID of bookshelf to retrieve volumes.
    ///
    /// Sets the *shelf* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn shelf(mut self, new_value: &str) -> BookshelveVolumeListCall<'a, C, A> {
        self._shelf = new_value.to_string();
        self
    }
    /// Index of the first element to return (starts at 0)
    ///
    /// Sets the *start index* query property to the given value.
    pub fn start_index(mut self, new_value: u32) -> BookshelveVolumeListCall<'a, C, A> {
        self._start_index = Some(new_value);
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> BookshelveVolumeListCall<'a, C, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Set to true to show pre-ordered books. Defaults to false.
    ///
    /// Sets the *show preorders* query property to the given value.
    pub fn show_preorders(mut self, new_value: bool) -> BookshelveVolumeListCall<'a, C, A> {
        self._show_preorders = Some(new_value);
        self
    }
    /// Maximum number of results to return
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> BookshelveVolumeListCall<'a, C, A> {
        self._max_results = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> BookshelveVolumeListCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> BookshelveVolumeListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> BookshelveVolumeListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves a list of public bookshelves for the specified user.
///
/// A builder for the *list* method supported by a *bookshelve* resource.
/// It is not used directly, but through a `BookshelveMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.bookshelves().list("userId")
///              .source("takimata")
///              .doit();
/// # }
/// ```
pub struct BookshelveListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _user_id: String,
    _source: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for BookshelveListCall<'a, C, A> {}

impl<'a, C, A> BookshelveListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Bookshelves)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.bookshelves.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("userId", self._user_id.to_string()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        for &field in ["alt", "userId", "source"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "users/{userId}/bookshelves";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{userId}", "userId")].iter() {
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
            for param_name in ["userId"].iter() {
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


    /// ID of user for whom to retrieve bookshelves.
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> BookshelveListCall<'a, C, A> {
        self._user_id = new_value.to_string();
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> BookshelveListCall<'a, C, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> BookshelveListCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> BookshelveListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> BookshelveListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves metadata for a specific bookshelf for the specified user.
///
/// A builder for the *get* method supported by a *bookshelve* resource.
/// It is not used directly, but through a `BookshelveMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.bookshelves().get("userId", "shelf")
///              .source("rebum.")
///              .doit();
/// # }
/// ```
pub struct BookshelveGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _user_id: String,
    _shelf: String,
    _source: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for BookshelveGetCall<'a, C, A> {}

impl<'a, C, A> BookshelveGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Bookshelf)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.bookshelves.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("userId", self._user_id.to_string()));
        params.push(("shelf", self._shelf.to_string()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        for &field in ["alt", "userId", "shelf", "source"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "users/{userId}/bookshelves/{shelf}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{userId}", "userId"), ("{shelf}", "shelf")].iter() {
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
            for param_name in ["shelf", "userId"].iter() {
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


    /// ID of user for whom to retrieve bookshelves.
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> BookshelveGetCall<'a, C, A> {
        self._user_id = new_value.to_string();
        self
    }
    /// ID of bookshelf to retrieve.
    ///
    /// Sets the *shelf* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn shelf(mut self, new_value: &str) -> BookshelveGetCall<'a, C, A> {
        self._shelf = new_value.to_string();
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> BookshelveGetCall<'a, C, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> BookshelveGetCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> BookshelveGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> BookshelveGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// 
///
/// A builder for the *accept* method supported by a *promooffer* resource.
/// It is not used directly, but through a `PromoofferMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.promooffer().accept()
///              .volume_id("Lorem")
///              .serial("Lorem")
///              .product("diam")
///              .offer_id("ut")
///              .model("ut")
///              .manufacturer("amet.")
///              .device("ipsum")
///              .android_id("ut")
///              .doit();
/// # }
/// ```
pub struct PromoofferAcceptCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _volume_id: Option<String>,
    _serial: Option<String>,
    _product: Option<String>,
    _offer_id: Option<String>,
    _model: Option<String>,
    _manufacturer: Option<String>,
    _device: Option<String>,
    _android_id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for PromoofferAcceptCall<'a, C, A> {}

impl<'a, C, A> PromoofferAcceptCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.promooffer.accept",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(9 + self._additional_params.len());
        if let Some(value) = self._volume_id {
            params.push(("volumeId", value.to_string()));
        }
        if let Some(value) = self._serial {
            params.push(("serial", value.to_string()));
        }
        if let Some(value) = self._product {
            params.push(("product", value.to_string()));
        }
        if let Some(value) = self._offer_id {
            params.push(("offerId", value.to_string()));
        }
        if let Some(value) = self._model {
            params.push(("model", value.to_string()));
        }
        if let Some(value) = self._manufacturer {
            params.push(("manufacturer", value.to_string()));
        }
        if let Some(value) = self._device {
            params.push(("device", value.to_string()));
        }
        if let Some(value) = self._android_id {
            params.push(("androidId", value.to_string()));
        }
        for &field in ["volumeId", "serial", "product", "offerId", "model", "manufacturer", "device", "androidId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "promooffer/accept";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Volume id to exercise the offer
    ///
    /// Sets the *volume id* query property to the given value.
    pub fn volume_id(mut self, new_value: &str) -> PromoofferAcceptCall<'a, C, A> {
        self._volume_id = Some(new_value.to_string());
        self
    }
    /// device serial
    ///
    /// Sets the *serial* query property to the given value.
    pub fn serial(mut self, new_value: &str) -> PromoofferAcceptCall<'a, C, A> {
        self._serial = Some(new_value.to_string());
        self
    }
    /// device product
    ///
    /// Sets the *product* query property to the given value.
    pub fn product(mut self, new_value: &str) -> PromoofferAcceptCall<'a, C, A> {
        self._product = Some(new_value.to_string());
        self
    }
    ///
    /// Sets the *offer id* query property to the given value.
    pub fn offer_id(mut self, new_value: &str) -> PromoofferAcceptCall<'a, C, A> {
        self._offer_id = Some(new_value.to_string());
        self
    }
    /// device model
    ///
    /// Sets the *model* query property to the given value.
    pub fn model(mut self, new_value: &str) -> PromoofferAcceptCall<'a, C, A> {
        self._model = Some(new_value.to_string());
        self
    }
    /// device manufacturer
    ///
    /// Sets the *manufacturer* query property to the given value.
    pub fn manufacturer(mut self, new_value: &str) -> PromoofferAcceptCall<'a, C, A> {
        self._manufacturer = Some(new_value.to_string());
        self
    }
    /// device device
    ///
    /// Sets the *device* query property to the given value.
    pub fn device(mut self, new_value: &str) -> PromoofferAcceptCall<'a, C, A> {
        self._device = Some(new_value.to_string());
        self
    }
    /// device android_id
    ///
    /// Sets the *android id* query property to the given value.
    pub fn android_id(mut self, new_value: &str) -> PromoofferAcceptCall<'a, C, A> {
        self._android_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PromoofferAcceptCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> PromoofferAcceptCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> PromoofferAcceptCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// 
///
/// A builder for the *dismiss* method supported by a *promooffer* resource.
/// It is not used directly, but through a `PromoofferMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.promooffer().dismiss()
///              .serial("dolor")
///              .product("sea")
///              .offer_id("ut")
///              .model("eirmod")
///              .manufacturer("sanctus")
///              .device("voluptua.")
///              .android_id("dolor")
///              .doit();
/// # }
/// ```
pub struct PromoofferDismisCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _serial: Option<String>,
    _product: Option<String>,
    _offer_id: Option<String>,
    _model: Option<String>,
    _manufacturer: Option<String>,
    _device: Option<String>,
    _android_id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for PromoofferDismisCall<'a, C, A> {}

impl<'a, C, A> PromoofferDismisCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.promooffer.dismiss",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(8 + self._additional_params.len());
        if let Some(value) = self._serial {
            params.push(("serial", value.to_string()));
        }
        if let Some(value) = self._product {
            params.push(("product", value.to_string()));
        }
        if let Some(value) = self._offer_id {
            params.push(("offerId", value.to_string()));
        }
        if let Some(value) = self._model {
            params.push(("model", value.to_string()));
        }
        if let Some(value) = self._manufacturer {
            params.push(("manufacturer", value.to_string()));
        }
        if let Some(value) = self._device {
            params.push(("device", value.to_string()));
        }
        if let Some(value) = self._android_id {
            params.push(("androidId", value.to_string()));
        }
        for &field in ["serial", "product", "offerId", "model", "manufacturer", "device", "androidId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "promooffer/dismiss";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// device serial
    ///
    /// Sets the *serial* query property to the given value.
    pub fn serial(mut self, new_value: &str) -> PromoofferDismisCall<'a, C, A> {
        self._serial = Some(new_value.to_string());
        self
    }
    /// device product
    ///
    /// Sets the *product* query property to the given value.
    pub fn product(mut self, new_value: &str) -> PromoofferDismisCall<'a, C, A> {
        self._product = Some(new_value.to_string());
        self
    }
    /// Offer to dimiss
    ///
    /// Sets the *offer id* query property to the given value.
    pub fn offer_id(mut self, new_value: &str) -> PromoofferDismisCall<'a, C, A> {
        self._offer_id = Some(new_value.to_string());
        self
    }
    /// device model
    ///
    /// Sets the *model* query property to the given value.
    pub fn model(mut self, new_value: &str) -> PromoofferDismisCall<'a, C, A> {
        self._model = Some(new_value.to_string());
        self
    }
    /// device manufacturer
    ///
    /// Sets the *manufacturer* query property to the given value.
    pub fn manufacturer(mut self, new_value: &str) -> PromoofferDismisCall<'a, C, A> {
        self._manufacturer = Some(new_value.to_string());
        self
    }
    /// device device
    ///
    /// Sets the *device* query property to the given value.
    pub fn device(mut self, new_value: &str) -> PromoofferDismisCall<'a, C, A> {
        self._device = Some(new_value.to_string());
        self
    }
    /// device android_id
    ///
    /// Sets the *android id* query property to the given value.
    pub fn android_id(mut self, new_value: &str) -> PromoofferDismisCall<'a, C, A> {
        self._android_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PromoofferDismisCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> PromoofferDismisCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> PromoofferDismisCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns a list of promo offers available to the user
///
/// A builder for the *get* method supported by a *promooffer* resource.
/// It is not used directly, but through a `PromoofferMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.promooffer().get()
///              .serial("et")
///              .product("et")
///              .model("vero")
///              .manufacturer("ut")
///              .device("sed")
///              .android_id("et")
///              .doit();
/// # }
/// ```
pub struct PromoofferGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _serial: Option<String>,
    _product: Option<String>,
    _model: Option<String>,
    _manufacturer: Option<String>,
    _device: Option<String>,
    _android_id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for PromoofferGetCall<'a, C, A> {}

impl<'a, C, A> PromoofferGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Offers)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.promooffer.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(8 + self._additional_params.len());
        if let Some(value) = self._serial {
            params.push(("serial", value.to_string()));
        }
        if let Some(value) = self._product {
            params.push(("product", value.to_string()));
        }
        if let Some(value) = self._model {
            params.push(("model", value.to_string()));
        }
        if let Some(value) = self._manufacturer {
            params.push(("manufacturer", value.to_string()));
        }
        if let Some(value) = self._device {
            params.push(("device", value.to_string()));
        }
        if let Some(value) = self._android_id {
            params.push(("androidId", value.to_string()));
        }
        for &field in ["alt", "serial", "product", "model", "manufacturer", "device", "androidId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "promooffer/get";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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


    /// device serial
    ///
    /// Sets the *serial* query property to the given value.
    pub fn serial(mut self, new_value: &str) -> PromoofferGetCall<'a, C, A> {
        self._serial = Some(new_value.to_string());
        self
    }
    /// device product
    ///
    /// Sets the *product* query property to the given value.
    pub fn product(mut self, new_value: &str) -> PromoofferGetCall<'a, C, A> {
        self._product = Some(new_value.to_string());
        self
    }
    /// device model
    ///
    /// Sets the *model* query property to the given value.
    pub fn model(mut self, new_value: &str) -> PromoofferGetCall<'a, C, A> {
        self._model = Some(new_value.to_string());
        self
    }
    /// device manufacturer
    ///
    /// Sets the *manufacturer* query property to the given value.
    pub fn manufacturer(mut self, new_value: &str) -> PromoofferGetCall<'a, C, A> {
        self._manufacturer = Some(new_value.to_string());
        self
    }
    /// device device
    ///
    /// Sets the *device* query property to the given value.
    pub fn device(mut self, new_value: &str) -> PromoofferGetCall<'a, C, A> {
        self._device = Some(new_value.to_string());
        self
    }
    /// device android_id
    ///
    /// Sets the *android id* query property to the given value.
    pub fn android_id(mut self, new_value: &str) -> PromoofferGetCall<'a, C, A> {
        self._android_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PromoofferGetCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> PromoofferGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> PromoofferGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns a stream of personalized book clusters
///
/// A builder for the *get* method supported by a *personalizedstream* resource.
/// It is not used directly, but through a `PersonalizedstreamMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.personalizedstream().get()
///              .source("ipsum")
///              .max_allowed_maturity_rating("justo")
///              .locale("dolore")
///              .doit();
/// # }
/// ```
pub struct PersonalizedstreamGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _source: Option<String>,
    _max_allowed_maturity_rating: Option<String>,
    _locale: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for PersonalizedstreamGetCall<'a, C, A> {}

impl<'a, C, A> PersonalizedstreamGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Discoveryclusters)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.personalizedstream.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._max_allowed_maturity_rating {
            params.push(("maxAllowedMaturityRating", value.to_string()));
        }
        if let Some(value) = self._locale {
            params.push(("locale", value.to_string()));
        }
        for &field in ["alt", "source", "maxAllowedMaturityRating", "locale"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "personalizedstream/get";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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


    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> PersonalizedstreamGetCall<'a, C, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The maximum allowed maturity rating of returned recommendations. Books with a higher maturity rating are filtered out.
    ///
    /// Sets the *max allowed maturity rating* query property to the given value.
    pub fn max_allowed_maturity_rating(mut self, new_value: &str) -> PersonalizedstreamGetCall<'a, C, A> {
        self._max_allowed_maturity_rating = Some(new_value.to_string());
        self
    }
    /// ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'. Used for generating recommendations.
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> PersonalizedstreamGetCall<'a, C, A> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PersonalizedstreamGetCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> PersonalizedstreamGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> PersonalizedstreamGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// List categories for onboarding experience.
///
/// A builder for the *listCategories* method supported by a *onboarding* resource.
/// It is not used directly, but through a `OnboardingMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.onboarding().list_categories()
///              .locale("vero")
///              .doit();
/// # }
/// ```
pub struct OnboardingListCategoryCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _locale: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OnboardingListCategoryCall<'a, C, A> {}

impl<'a, C, A> OnboardingListCategoryCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Category)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.onboarding.listCategories",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        if let Some(value) = self._locale {
            params.push(("locale", value.to_string()));
        }
        for &field in ["alt", "locale"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "onboarding/listCategories";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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


    /// ISO-639-1 language and ISO-3166-1 country code. Default is en-US if unset.
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> OnboardingListCategoryCall<'a, C, A> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OnboardingListCategoryCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> OnboardingListCategoryCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> OnboardingListCategoryCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// List available volumes under categories for onboarding experience.
///
/// A builder for the *listCategoryVolumes* method supported by a *onboarding* resource.
/// It is not used directly, but through a `OnboardingMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.onboarding().list_category_volumes()
///              .page_token("dolor")
///              .page_size(43)
///              .max_allowed_maturity_rating("et")
///              .locale("nonumy")
///              .add_category_id("et")
///              .doit();
/// # }
/// ```
pub struct OnboardingListCategoryVolumeCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _page_token: Option<String>,
    _page_size: Option<u32>,
    _max_allowed_maturity_rating: Option<String>,
    _locale: Option<String>,
    _category_id: Vec<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OnboardingListCategoryVolumeCall<'a, C, A> {}

impl<'a, C, A> OnboardingListCategoryVolumeCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Volume2)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.onboarding.listCategoryVolumes",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._max_allowed_maturity_rating {
            params.push(("maxAllowedMaturityRating", value.to_string()));
        }
        if let Some(value) = self._locale {
            params.push(("locale", value.to_string()));
        }
        if self._category_id.len() > 0 {
            for f in self._category_id.iter() {
                params.push(("categoryId", f.to_string()));
            }
        }
        for &field in ["alt", "pageToken", "pageSize", "maxAllowedMaturityRating", "locale", "categoryId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "onboarding/listCategoryVolumes";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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


    /// The value of the nextToken from the previous page.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> OnboardingListCategoryVolumeCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Number of maximum results per page to be included in the response.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: u32) -> OnboardingListCategoryVolumeCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// The maximum allowed maturity rating of returned volumes. Books with a higher maturity rating are filtered out.
    ///
    /// Sets the *max allowed maturity rating* query property to the given value.
    pub fn max_allowed_maturity_rating(mut self, new_value: &str) -> OnboardingListCategoryVolumeCall<'a, C, A> {
        self._max_allowed_maturity_rating = Some(new_value.to_string());
        self
    }
    /// ISO-639-1 language and ISO-3166-1 country code. Default is en-US if unset.
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> OnboardingListCategoryVolumeCall<'a, C, A> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// List of category ids requested.
    ///
    /// Append the given value to the *category id* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_category_id(mut self, new_value: &str) -> OnboardingListCategoryVolumeCall<'a, C, A> {
        self._category_id.push(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OnboardingListCategoryVolumeCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> OnboardingListCategoryVolumeCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> OnboardingListCategoryVolumeCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Request concurrent and download access restrictions.
///
/// A builder for the *requestAccess* method supported by a *myconfig* resource.
/// It is not used directly, but through a `MyconfigMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.myconfig().request_access("source", "volumeId", "nonce", "cpksver")
///              .locale("labore")
///              .license_types("aliquyam")
///              .doit();
/// # }
/// ```
pub struct MyconfigRequestAccesCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _source: String,
    _volume_id: String,
    _nonce: String,
    _cpksver: String,
    _locale: Option<String>,
    _license_types: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for MyconfigRequestAccesCall<'a, C, A> {}

impl<'a, C, A> MyconfigRequestAccesCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, RequestAccess)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.myconfig.requestAccess",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(8 + self._additional_params.len());
        params.push(("source", self._source.to_string()));
        params.push(("volumeId", self._volume_id.to_string()));
        params.push(("nonce", self._nonce.to_string()));
        params.push(("cpksver", self._cpksver.to_string()));
        if let Some(value) = self._locale {
            params.push(("locale", value.to_string()));
        }
        if let Some(value) = self._license_types {
            params.push(("licenseTypes", value.to_string()));
        }
        for &field in ["alt", "source", "volumeId", "nonce", "cpksver", "locale", "licenseTypes"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "myconfig/requestAccess";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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


    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn source(mut self, new_value: &str) -> MyconfigRequestAccesCall<'a, C, A> {
        self._source = new_value.to_string();
        self
    }
    /// The volume to request concurrent/download restrictions for.
    ///
    /// Sets the *volume id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn volume_id(mut self, new_value: &str) -> MyconfigRequestAccesCall<'a, C, A> {
        self._volume_id = new_value.to_string();
        self
    }
    /// The client nonce value.
    ///
    /// Sets the *nonce* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn nonce(mut self, new_value: &str) -> MyconfigRequestAccesCall<'a, C, A> {
        self._nonce = new_value.to_string();
        self
    }
    /// The device/version ID from which to request the restrictions.
    ///
    /// Sets the *cpksver* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn cpksver(mut self, new_value: &str) -> MyconfigRequestAccesCall<'a, C, A> {
        self._cpksver = new_value.to_string();
        self
    }
    /// ISO-639-1, ISO-3166-1 codes for message localization, i.e. en_US.
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> MyconfigRequestAccesCall<'a, C, A> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// The type of access license to request. If not specified, the default is BOTH.
    ///
    /// Sets the *license types* query property to the given value.
    pub fn license_types(mut self, new_value: &str) -> MyconfigRequestAccesCall<'a, C, A> {
        self._license_types = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MyconfigRequestAccesCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MyconfigRequestAccesCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> MyconfigRequestAccesCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Release downloaded content access restriction.
///
/// A builder for the *releaseDownloadAccess* method supported by a *myconfig* resource.
/// It is not used directly, but through a `MyconfigMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.myconfig().release_download_access("volumeIds", "cpksver")
///              .source("sea")
///              .locale("elitr")
///              .doit();
/// # }
/// ```
pub struct MyconfigReleaseDownloadAccesCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _volume_ids: Vec<String>,
    _cpksver: String,
    _source: Option<String>,
    _locale: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for MyconfigReleaseDownloadAccesCall<'a, C, A> {}

impl<'a, C, A> MyconfigReleaseDownloadAccesCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, DownloadAccesses)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.myconfig.releaseDownloadAccess",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        if self._volume_ids.len() > 0 {
            for f in self._volume_ids.iter() {
                params.push(("volumeIds", f.to_string()));
            }
        }
        params.push(("cpksver", self._cpksver.to_string()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._locale {
            params.push(("locale", value.to_string()));
        }
        for &field in ["alt", "volumeIds", "cpksver", "source", "locale"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "myconfig/releaseDownloadAccess";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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


    /// The volume(s) to release restrictions for.
    ///
    /// Append the given value to the *volume ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn add_volume_ids(mut self, new_value: &str) -> MyconfigReleaseDownloadAccesCall<'a, C, A> {
        self._volume_ids.push(new_value.to_string());
        self
    }
    /// The device/version ID from which to release the restriction.
    ///
    /// Sets the *cpksver* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn cpksver(mut self, new_value: &str) -> MyconfigReleaseDownloadAccesCall<'a, C, A> {
        self._cpksver = new_value.to_string();
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> MyconfigReleaseDownloadAccesCall<'a, C, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// ISO-639-1, ISO-3166-1 codes for message localization, i.e. en_US.
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> MyconfigReleaseDownloadAccesCall<'a, C, A> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MyconfigReleaseDownloadAccesCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MyconfigReleaseDownloadAccesCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> MyconfigReleaseDownloadAccesCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Request downloaded content access for specified volumes on the My eBooks shelf.
///
/// A builder for the *syncVolumeLicenses* method supported by a *myconfig* resource.
/// It is not used directly, but through a `MyconfigMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.myconfig().sync_volume_licenses("source", "nonce", "cpksver")
///              .add_volume_ids("diam")
///              .show_preorders(true)
///              .locale("dolores")
///              .include_non_comics_series(true)
///              .add_features("dolor")
///              .doit();
/// # }
/// ```
pub struct MyconfigSyncVolumeLicenseCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _source: String,
    _nonce: String,
    _cpksver: String,
    _volume_ids: Vec<String>,
    _show_preorders: Option<bool>,
    _locale: Option<String>,
    _include_non_comics_series: Option<bool>,
    _features: Vec<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for MyconfigSyncVolumeLicenseCall<'a, C, A> {}

impl<'a, C, A> MyconfigSyncVolumeLicenseCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Volumes)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.myconfig.syncVolumeLicenses",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(10 + self._additional_params.len());
        params.push(("source", self._source.to_string()));
        params.push(("nonce", self._nonce.to_string()));
        params.push(("cpksver", self._cpksver.to_string()));
        if self._volume_ids.len() > 0 {
            for f in self._volume_ids.iter() {
                params.push(("volumeIds", f.to_string()));
            }
        }
        if let Some(value) = self._show_preorders {
            params.push(("showPreorders", value.to_string()));
        }
        if let Some(value) = self._locale {
            params.push(("locale", value.to_string()));
        }
        if let Some(value) = self._include_non_comics_series {
            params.push(("includeNonComicsSeries", value.to_string()));
        }
        if self._features.len() > 0 {
            for f in self._features.iter() {
                params.push(("features", f.to_string()));
            }
        }
        for &field in ["alt", "source", "nonce", "cpksver", "volumeIds", "showPreorders", "locale", "includeNonComicsSeries", "features"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "myconfig/syncVolumeLicenses";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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


    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn source(mut self, new_value: &str) -> MyconfigSyncVolumeLicenseCall<'a, C, A> {
        self._source = new_value.to_string();
        self
    }
    /// The client nonce value.
    ///
    /// Sets the *nonce* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn nonce(mut self, new_value: &str) -> MyconfigSyncVolumeLicenseCall<'a, C, A> {
        self._nonce = new_value.to_string();
        self
    }
    /// The device/version ID from which to release the restriction.
    ///
    /// Sets the *cpksver* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn cpksver(mut self, new_value: &str) -> MyconfigSyncVolumeLicenseCall<'a, C, A> {
        self._cpksver = new_value.to_string();
        self
    }
    /// The volume(s) to request download restrictions for.
    ///
    /// Append the given value to the *volume ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_volume_ids(mut self, new_value: &str) -> MyconfigSyncVolumeLicenseCall<'a, C, A> {
        self._volume_ids.push(new_value.to_string());
        self
    }
    /// Set to true to show pre-ordered books. Defaults to false.
    ///
    /// Sets the *show preorders* query property to the given value.
    pub fn show_preorders(mut self, new_value: bool) -> MyconfigSyncVolumeLicenseCall<'a, C, A> {
        self._show_preorders = Some(new_value);
        self
    }
    /// ISO-639-1, ISO-3166-1 codes for message localization, i.e. en_US.
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> MyconfigSyncVolumeLicenseCall<'a, C, A> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// Set to true to include non-comics series. Defaults to false.
    ///
    /// Sets the *include non comics series* query property to the given value.
    pub fn include_non_comics_series(mut self, new_value: bool) -> MyconfigSyncVolumeLicenseCall<'a, C, A> {
        self._include_non_comics_series = Some(new_value);
        self
    }
    /// List of features supported by the client, i.e., 'RENTALS'
    ///
    /// Append the given value to the *features* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_features(mut self, new_value: &str) -> MyconfigSyncVolumeLicenseCall<'a, C, A> {
        self._features.push(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MyconfigSyncVolumeLicenseCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MyconfigSyncVolumeLicenseCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> MyconfigSyncVolumeLicenseCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets the current settings for the user.
///
/// A builder for the *getUserSettings* method supported by a *myconfig* resource.
/// It is not used directly, but through a `MyconfigMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.myconfig().get_user_settings()
///              .doit();
/// # }
/// ```
pub struct MyconfigGetUserSettingCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for MyconfigGetUserSettingCall<'a, C, A> {}

impl<'a, C, A> MyconfigGetUserSettingCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Usersettings)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.myconfig.getUserSettings",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(2 + self._additional_params.len());
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

        let mut url = self.hub._base_url.clone() + "myconfig/getUserSettings";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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


    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MyconfigGetUserSettingCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MyconfigGetUserSettingCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> MyconfigGetUserSettingCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Sets the settings for the user. If a sub-object is specified, it will overwrite the existing sub-object stored in the server. Unspecified sub-objects will retain the existing value.
///
/// A builder for the *updateUserSettings* method supported by a *myconfig* resource.
/// It is not used directly, but through a `MyconfigMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// use books1::Usersettings;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Usersettings::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.myconfig().update_user_settings(req)
///              .doit();
/// # }
/// ```
pub struct MyconfigUpdateUserSettingCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _request: Usersettings,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for MyconfigUpdateUserSettingCall<'a, C, A> {}

impl<'a, C, A> MyconfigUpdateUserSettingCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Usersettings)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.myconfig.updateUserSettings",
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

        let mut url = self.hub._base_url.clone() + "myconfig/updateUserSettings";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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
    pub fn request(mut self, new_value: Usersettings) -> MyconfigUpdateUserSettingCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MyconfigUpdateUserSettingCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MyconfigUpdateUserSettingCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> MyconfigUpdateUserSettingCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Clears all volumes from a bookshelf.
///
/// A builder for the *bookshelves.clearVolumes* method supported by a *mylibrary* resource.
/// It is not used directly, but through a `MylibraryMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().bookshelves_clear_volumes("shelf")
///              .source("elitr")
///              .doit();
/// # }
/// ```
pub struct MylibraryBookshelveClearVolumeCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _shelf: String,
    _source: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for MylibraryBookshelveClearVolumeCall<'a, C, A> {}

impl<'a, C, A> MylibraryBookshelveClearVolumeCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.mylibrary.bookshelves.clearVolumes",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        params.push(("shelf", self._shelf.to_string()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        for &field in ["shelf", "source"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "mylibrary/bookshelves/{shelf}/clearVolumes";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{shelf}", "shelf")].iter() {
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
            for param_name in ["shelf"].iter() {
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
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// ID of bookshelf from which to remove a volume.
    ///
    /// Sets the *shelf* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn shelf(mut self, new_value: &str) -> MylibraryBookshelveClearVolumeCall<'a, C, A> {
        self._shelf = new_value.to_string();
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> MylibraryBookshelveClearVolumeCall<'a, C, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MylibraryBookshelveClearVolumeCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryBookshelveClearVolumeCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> MylibraryBookshelveClearVolumeCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Moves a volume within a bookshelf.
///
/// A builder for the *bookshelves.moveVolume* method supported by a *mylibrary* resource.
/// It is not used directly, but through a `MylibraryMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().bookshelves_move_volume("shelf", "volumeId", -65)
///              .source("sed")
///              .doit();
/// # }
/// ```
pub struct MylibraryBookshelveMoveVolumeCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _shelf: String,
    _volume_id: String,
    _volume_position: i32,
    _source: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for MylibraryBookshelveMoveVolumeCall<'a, C, A> {}

impl<'a, C, A> MylibraryBookshelveMoveVolumeCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.mylibrary.bookshelves.moveVolume",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("shelf", self._shelf.to_string()));
        params.push(("volumeId", self._volume_id.to_string()));
        params.push(("volumePosition", self._volume_position.to_string()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        for &field in ["shelf", "volumeId", "volumePosition", "source"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "mylibrary/bookshelves/{shelf}/moveVolume";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{shelf}", "shelf")].iter() {
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
            for param_name in ["shelf"].iter() {
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
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// ID of bookshelf with the volume.
    ///
    /// Sets the *shelf* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn shelf(mut self, new_value: &str) -> MylibraryBookshelveMoveVolumeCall<'a, C, A> {
        self._shelf = new_value.to_string();
        self
    }
    /// ID of volume to move.
    ///
    /// Sets the *volume id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn volume_id(mut self, new_value: &str) -> MylibraryBookshelveMoveVolumeCall<'a, C, A> {
        self._volume_id = new_value.to_string();
        self
    }
    /// Position on shelf to move the item (0 puts the item before the current first item, 1 puts it between the first and the second and so on.)
    ///
    /// Sets the *volume position* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn volume_position(mut self, new_value: i32) -> MylibraryBookshelveMoveVolumeCall<'a, C, A> {
        self._volume_position = new_value;
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> MylibraryBookshelveMoveVolumeCall<'a, C, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MylibraryBookshelveMoveVolumeCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryBookshelveMoveVolumeCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> MylibraryBookshelveMoveVolumeCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets volume information for volumes on a bookshelf.
///
/// A builder for the *bookshelves.volumes.list* method supported by a *mylibrary* resource.
/// It is not used directly, but through a `MylibraryMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().bookshelves_volumes_list("shelf")
///              .start_index(94)
///              .source("dolore")
///              .show_preorders(true)
///              .q("consetetur")
///              .projection("consetetur")
///              .max_results(11)
///              .country("labore")
///              .doit();
/// # }
/// ```
pub struct MylibraryBookshelveVolumeListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _shelf: String,
    _start_index: Option<u32>,
    _source: Option<String>,
    _show_preorders: Option<bool>,
    _q: Option<String>,
    _projection: Option<String>,
    _max_results: Option<u32>,
    _country: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for MylibraryBookshelveVolumeListCall<'a, C, A> {}

impl<'a, C, A> MylibraryBookshelveVolumeListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Volumes)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.mylibrary.bookshelves.volumes.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(10 + self._additional_params.len());
        params.push(("shelf", self._shelf.to_string()));
        if let Some(value) = self._start_index {
            params.push(("startIndex", value.to_string()));
        }
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._show_preorders {
            params.push(("showPreorders", value.to_string()));
        }
        if let Some(value) = self._q {
            params.push(("q", value.to_string()));
        }
        if let Some(value) = self._projection {
            params.push(("projection", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._country {
            params.push(("country", value.to_string()));
        }
        for &field in ["alt", "shelf", "startIndex", "source", "showPreorders", "q", "projection", "maxResults", "country"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "mylibrary/bookshelves/{shelf}/volumes";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{shelf}", "shelf")].iter() {
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
            for param_name in ["shelf"].iter() {
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


    /// The bookshelf ID or name retrieve volumes for.
    ///
    /// Sets the *shelf* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn shelf(mut self, new_value: &str) -> MylibraryBookshelveVolumeListCall<'a, C, A> {
        self._shelf = new_value.to_string();
        self
    }
    /// Index of the first element to return (starts at 0)
    ///
    /// Sets the *start index* query property to the given value.
    pub fn start_index(mut self, new_value: u32) -> MylibraryBookshelveVolumeListCall<'a, C, A> {
        self._start_index = Some(new_value);
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> MylibraryBookshelveVolumeListCall<'a, C, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Set to true to show pre-ordered books. Defaults to false.
    ///
    /// Sets the *show preorders* query property to the given value.
    pub fn show_preorders(mut self, new_value: bool) -> MylibraryBookshelveVolumeListCall<'a, C, A> {
        self._show_preorders = Some(new_value);
        self
    }
    /// Full-text search query string in this bookshelf.
    ///
    /// Sets the *q* query property to the given value.
    pub fn q(mut self, new_value: &str) -> MylibraryBookshelveVolumeListCall<'a, C, A> {
        self._q = Some(new_value.to_string());
        self
    }
    /// Restrict information returned to a set of selected fields.
    ///
    /// Sets the *projection* query property to the given value.
    pub fn projection(mut self, new_value: &str) -> MylibraryBookshelveVolumeListCall<'a, C, A> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Maximum number of results to return
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> MylibraryBookshelveVolumeListCall<'a, C, A> {
        self._max_results = Some(new_value);
        self
    }
    /// ISO-3166-1 code to override the IP-based location.
    ///
    /// Sets the *country* query property to the given value.
    pub fn country(mut self, new_value: &str) -> MylibraryBookshelveVolumeListCall<'a, C, A> {
        self._country = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MylibraryBookshelveVolumeListCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryBookshelveVolumeListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> MylibraryBookshelveVolumeListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets the summary of specified layers.
///
/// A builder for the *annotations.summary* method supported by a *mylibrary* resource.
/// It is not used directly, but through a `MylibraryMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().annotations_summary("layerIds", "volumeId")
///              .doit();
/// # }
/// ```
pub struct MylibraryAnnotationSummaryCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _layer_ids: Vec<String>,
    _volume_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for MylibraryAnnotationSummaryCall<'a, C, A> {}

impl<'a, C, A> MylibraryAnnotationSummaryCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, AnnotationsSummary)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.mylibrary.annotations.summary",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        if self._layer_ids.len() > 0 {
            for f in self._layer_ids.iter() {
                params.push(("layerIds", f.to_string()));
            }
        }
        params.push(("volumeId", self._volume_id.to_string()));
        for &field in ["alt", "layerIds", "volumeId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "mylibrary/annotations/summary";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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


    /// Array of layer IDs to get the summary for.
    ///
    /// Append the given value to the *layer ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn add_layer_ids(mut self, new_value: &str) -> MylibraryAnnotationSummaryCall<'a, C, A> {
        self._layer_ids.push(new_value.to_string());
        self
    }
    /// Volume id to get the summary for.
    ///
    /// Sets the *volume id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn volume_id(mut self, new_value: &str) -> MylibraryAnnotationSummaryCall<'a, C, A> {
        self._volume_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MylibraryAnnotationSummaryCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryAnnotationSummaryCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> MylibraryAnnotationSummaryCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deletes an annotation.
///
/// A builder for the *annotations.delete* method supported by a *mylibrary* resource.
/// It is not used directly, but through a `MylibraryMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().annotations_delete("annotationId")
///              .source("accusam")
///              .doit();
/// # }
/// ```
pub struct MylibraryAnnotationDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _annotation_id: String,
    _source: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for MylibraryAnnotationDeleteCall<'a, C, A> {}

impl<'a, C, A> MylibraryAnnotationDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.mylibrary.annotations.delete",
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        params.push(("annotationId", self._annotation_id.to_string()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        for &field in ["annotationId", "source"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "mylibrary/annotations/{annotationId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{annotationId}", "annotationId")].iter() {
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
            for param_name in ["annotationId"].iter() {
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
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID for the annotation to delete.
    ///
    /// Sets the *annotation id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn annotation_id(mut self, new_value: &str) -> MylibraryAnnotationDeleteCall<'a, C, A> {
        self._annotation_id = new_value.to_string();
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> MylibraryAnnotationDeleteCall<'a, C, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MylibraryAnnotationDeleteCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryAnnotationDeleteCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> MylibraryAnnotationDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Adds a volume to a bookshelf.
///
/// A builder for the *bookshelves.addVolume* method supported by a *mylibrary* resource.
/// It is not used directly, but through a `MylibraryMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().bookshelves_add_volume("shelf", "volumeId")
///              .source("rebum.")
///              .reason("et")
///              .doit();
/// # }
/// ```
pub struct MylibraryBookshelveAddVolumeCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _shelf: String,
    _volume_id: String,
    _source: Option<String>,
    _reason: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for MylibraryBookshelveAddVolumeCall<'a, C, A> {}

impl<'a, C, A> MylibraryBookshelveAddVolumeCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.mylibrary.bookshelves.addVolume",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("shelf", self._shelf.to_string()));
        params.push(("volumeId", self._volume_id.to_string()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._reason {
            params.push(("reason", value.to_string()));
        }
        for &field in ["shelf", "volumeId", "source", "reason"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "mylibrary/bookshelves/{shelf}/addVolume";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{shelf}", "shelf")].iter() {
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
            for param_name in ["shelf"].iter() {
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
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// ID of bookshelf to which to add a volume.
    ///
    /// Sets the *shelf* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn shelf(mut self, new_value: &str) -> MylibraryBookshelveAddVolumeCall<'a, C, A> {
        self._shelf = new_value.to_string();
        self
    }
    /// ID of volume to add.
    ///
    /// Sets the *volume id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn volume_id(mut self, new_value: &str) -> MylibraryBookshelveAddVolumeCall<'a, C, A> {
        self._volume_id = new_value.to_string();
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> MylibraryBookshelveAddVolumeCall<'a, C, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The reason for which the book is added to the library.
    ///
    /// Sets the *reason* query property to the given value.
    pub fn reason(mut self, new_value: &str) -> MylibraryBookshelveAddVolumeCall<'a, C, A> {
        self._reason = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MylibraryBookshelveAddVolumeCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryBookshelveAddVolumeCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> MylibraryBookshelveAddVolumeCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Inserts a new annotation.
///
/// A builder for the *annotations.insert* method supported by a *mylibrary* resource.
/// It is not used directly, but through a `MylibraryMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// use books1::Annotation;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Annotation::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().annotations_insert(req)
///              .source("clita")
///              .show_only_summary_in_response(true)
///              .country("dolores")
///              .annotation_id("vero")
///              .doit();
/// # }
/// ```
pub struct MylibraryAnnotationInsertCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _request: Annotation,
    _source: Option<String>,
    _show_only_summary_in_response: Option<bool>,
    _country: Option<String>,
    _annotation_id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for MylibraryAnnotationInsertCall<'a, C, A> {}

impl<'a, C, A> MylibraryAnnotationInsertCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Annotation)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.mylibrary.annotations.insert",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._show_only_summary_in_response {
            params.push(("showOnlySummaryInResponse", value.to_string()));
        }
        if let Some(value) = self._country {
            params.push(("country", value.to_string()));
        }
        if let Some(value) = self._annotation_id {
            params.push(("annotationId", value.to_string()));
        }
        for &field in ["alt", "source", "showOnlySummaryInResponse", "country", "annotationId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "mylibrary/annotations";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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
    pub fn request(mut self, new_value: Annotation) -> MylibraryAnnotationInsertCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> MylibraryAnnotationInsertCall<'a, C, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Requests that only the summary of the specified layer be provided in the response.
    ///
    /// Sets the *show only summary in response* query property to the given value.
    pub fn show_only_summary_in_response(mut self, new_value: bool) -> MylibraryAnnotationInsertCall<'a, C, A> {
        self._show_only_summary_in_response = Some(new_value);
        self
    }
    /// ISO-3166-1 code to override the IP-based location.
    ///
    /// Sets the *country* query property to the given value.
    pub fn country(mut self, new_value: &str) -> MylibraryAnnotationInsertCall<'a, C, A> {
        self._country = Some(new_value.to_string());
        self
    }
    /// The ID for the annotation to insert.
    ///
    /// Sets the *annotation id* query property to the given value.
    pub fn annotation_id(mut self, new_value: &str) -> MylibraryAnnotationInsertCall<'a, C, A> {
        self._annotation_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MylibraryAnnotationInsertCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryAnnotationInsertCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> MylibraryAnnotationInsertCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Removes a volume from a bookshelf.
///
/// A builder for the *bookshelves.removeVolume* method supported by a *mylibrary* resource.
/// It is not used directly, but through a `MylibraryMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().bookshelves_remove_volume("shelf", "volumeId")
///              .source("consetetur")
///              .reason("eos")
///              .doit();
/// # }
/// ```
pub struct MylibraryBookshelveRemoveVolumeCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _shelf: String,
    _volume_id: String,
    _source: Option<String>,
    _reason: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for MylibraryBookshelveRemoveVolumeCall<'a, C, A> {}

impl<'a, C, A> MylibraryBookshelveRemoveVolumeCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.mylibrary.bookshelves.removeVolume",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("shelf", self._shelf.to_string()));
        params.push(("volumeId", self._volume_id.to_string()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._reason {
            params.push(("reason", value.to_string()));
        }
        for &field in ["shelf", "volumeId", "source", "reason"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "mylibrary/bookshelves/{shelf}/removeVolume";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{shelf}", "shelf")].iter() {
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
            for param_name in ["shelf"].iter() {
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
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// ID of bookshelf from which to remove a volume.
    ///
    /// Sets the *shelf* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn shelf(mut self, new_value: &str) -> MylibraryBookshelveRemoveVolumeCall<'a, C, A> {
        self._shelf = new_value.to_string();
        self
    }
    /// ID of volume to remove.
    ///
    /// Sets the *volume id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn volume_id(mut self, new_value: &str) -> MylibraryBookshelveRemoveVolumeCall<'a, C, A> {
        self._volume_id = new_value.to_string();
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> MylibraryBookshelveRemoveVolumeCall<'a, C, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The reason for which the book is removed from the library.
    ///
    /// Sets the *reason* query property to the given value.
    pub fn reason(mut self, new_value: &str) -> MylibraryBookshelveRemoveVolumeCall<'a, C, A> {
        self._reason = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MylibraryBookshelveRemoveVolumeCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryBookshelveRemoveVolumeCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> MylibraryBookshelveRemoveVolumeCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves a list of annotations, possibly filtered.
///
/// A builder for the *annotations.list* method supported by a *mylibrary* resource.
/// It is not used directly, but through a `MylibraryMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().annotations_list()
///              .volume_id("justo")
///              .updated_min("tempor")
///              .updated_max("gubergren")
///              .source("dolore")
///              .show_deleted(false)
///              .page_token("dolore")
///              .max_results(18)
///              .add_layer_ids("elitr")
///              .layer_id("magna")
///              .content_version("ipsum")
///              .doit();
/// # }
/// ```
pub struct MylibraryAnnotationListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _volume_id: Option<String>,
    _updated_min: Option<String>,
    _updated_max: Option<String>,
    _source: Option<String>,
    _show_deleted: Option<bool>,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _layer_ids: Vec<String>,
    _layer_id: Option<String>,
    _content_version: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for MylibraryAnnotationListCall<'a, C, A> {}

impl<'a, C, A> MylibraryAnnotationListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Annotations)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.mylibrary.annotations.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(12 + self._additional_params.len());
        if let Some(value) = self._volume_id {
            params.push(("volumeId", value.to_string()));
        }
        if let Some(value) = self._updated_min {
            params.push(("updatedMin", value.to_string()));
        }
        if let Some(value) = self._updated_max {
            params.push(("updatedMax", value.to_string()));
        }
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._show_deleted {
            params.push(("showDeleted", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if self._layer_ids.len() > 0 {
            for f in self._layer_ids.iter() {
                params.push(("layerIds", f.to_string()));
            }
        }
        if let Some(value) = self._layer_id {
            params.push(("layerId", value.to_string()));
        }
        if let Some(value) = self._content_version {
            params.push(("contentVersion", value.to_string()));
        }
        for &field in ["alt", "volumeId", "updatedMin", "updatedMax", "source", "showDeleted", "pageToken", "maxResults", "layerIds", "layerId", "contentVersion"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "mylibrary/annotations";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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


    /// The volume to restrict annotations to.
    ///
    /// Sets the *volume id* query property to the given value.
    pub fn volume_id(mut self, new_value: &str) -> MylibraryAnnotationListCall<'a, C, A> {
        self._volume_id = Some(new_value.to_string());
        self
    }
    /// RFC 3339 timestamp to restrict to items updated since this timestamp (inclusive).
    ///
    /// Sets the *updated min* query property to the given value.
    pub fn updated_min(mut self, new_value: &str) -> MylibraryAnnotationListCall<'a, C, A> {
        self._updated_min = Some(new_value.to_string());
        self
    }
    /// RFC 3339 timestamp to restrict to items updated prior to this timestamp (exclusive).
    ///
    /// Sets the *updated max* query property to the given value.
    pub fn updated_max(mut self, new_value: &str) -> MylibraryAnnotationListCall<'a, C, A> {
        self._updated_max = Some(new_value.to_string());
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> MylibraryAnnotationListCall<'a, C, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Set to true to return deleted annotations. updatedMin must be in the request to use this. Defaults to false.
    ///
    /// Sets the *show deleted* query property to the given value.
    pub fn show_deleted(mut self, new_value: bool) -> MylibraryAnnotationListCall<'a, C, A> {
        self._show_deleted = Some(new_value);
        self
    }
    /// The value of the nextToken from the previous page.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> MylibraryAnnotationListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of results to return
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> MylibraryAnnotationListCall<'a, C, A> {
        self._max_results = Some(new_value);
        self
    }
    /// The layer ID(s) to limit annotation by.
    ///
    /// Append the given value to the *layer ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_layer_ids(mut self, new_value: &str) -> MylibraryAnnotationListCall<'a, C, A> {
        self._layer_ids.push(new_value.to_string());
        self
    }
    /// The layer ID to limit annotation by.
    ///
    /// Sets the *layer id* query property to the given value.
    pub fn layer_id(mut self, new_value: &str) -> MylibraryAnnotationListCall<'a, C, A> {
        self._layer_id = Some(new_value.to_string());
        self
    }
    /// The content version for the requested volume.
    ///
    /// Sets the *content version* query property to the given value.
    pub fn content_version(mut self, new_value: &str) -> MylibraryAnnotationListCall<'a, C, A> {
        self._content_version = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MylibraryAnnotationListCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryAnnotationListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> MylibraryAnnotationListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates an existing annotation.
///
/// A builder for the *annotations.update* method supported by a *mylibrary* resource.
/// It is not used directly, but through a `MylibraryMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// use books1::Annotation;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Annotation::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().annotations_update(req, "annotationId")
///              .source("accusam")
///              .doit();
/// # }
/// ```
pub struct MylibraryAnnotationUpdateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _request: Annotation,
    _annotation_id: String,
    _source: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for MylibraryAnnotationUpdateCall<'a, C, A> {}

impl<'a, C, A> MylibraryAnnotationUpdateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Annotation)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.mylibrary.annotations.update",
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("annotationId", self._annotation_id.to_string()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        for &field in ["alt", "annotationId", "source"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "mylibrary/annotations/{annotationId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{annotationId}", "annotationId")].iter() {
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
            for param_name in ["annotationId"].iter() {
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, url.clone())
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
    pub fn request(mut self, new_value: Annotation) -> MylibraryAnnotationUpdateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID for the annotation to update.
    ///
    /// Sets the *annotation id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn annotation_id(mut self, new_value: &str) -> MylibraryAnnotationUpdateCall<'a, C, A> {
        self._annotation_id = new_value.to_string();
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> MylibraryAnnotationUpdateCall<'a, C, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MylibraryAnnotationUpdateCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryAnnotationUpdateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> MylibraryAnnotationUpdateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Sets my reading position information for a volume.
///
/// A builder for the *readingpositions.setPosition* method supported by a *mylibrary* resource.
/// It is not used directly, but through a `MylibraryMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().readingpositions_set_position("volumeId", "timestamp", "position")
///              .source("sed")
///              .device_cookie("diam")
///              .content_version("magna")
///              .action("dolor")
///              .doit();
/// # }
/// ```
pub struct MylibraryReadingpositionSetPositionCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _volume_id: String,
    _timestamp: String,
    _position: String,
    _source: Option<String>,
    _device_cookie: Option<String>,
    _content_version: Option<String>,
    _action: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for MylibraryReadingpositionSetPositionCall<'a, C, A> {}

impl<'a, C, A> MylibraryReadingpositionSetPositionCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.mylibrary.readingpositions.setPosition",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(8 + self._additional_params.len());
        params.push(("volumeId", self._volume_id.to_string()));
        params.push(("timestamp", self._timestamp.to_string()));
        params.push(("position", self._position.to_string()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._device_cookie {
            params.push(("deviceCookie", value.to_string()));
        }
        if let Some(value) = self._content_version {
            params.push(("contentVersion", value.to_string()));
        }
        if let Some(value) = self._action {
            params.push(("action", value.to_string()));
        }
        for &field in ["volumeId", "timestamp", "position", "source", "deviceCookie", "contentVersion", "action"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "mylibrary/readingpositions/{volumeId}/setPosition";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{volumeId}", "volumeId")].iter() {
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
            for param_name in ["volumeId"].iter() {
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
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// ID of volume for which to update the reading position.
    ///
    /// Sets the *volume id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn volume_id(mut self, new_value: &str) -> MylibraryReadingpositionSetPositionCall<'a, C, A> {
        self._volume_id = new_value.to_string();
        self
    }
    /// RFC 3339 UTC format timestamp associated with this reading position.
    ///
    /// Sets the *timestamp* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn timestamp(mut self, new_value: &str) -> MylibraryReadingpositionSetPositionCall<'a, C, A> {
        self._timestamp = new_value.to_string();
        self
    }
    /// Position string for the new volume reading position.
    ///
    /// Sets the *position* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn position(mut self, new_value: &str) -> MylibraryReadingpositionSetPositionCall<'a, C, A> {
        self._position = new_value.to_string();
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> MylibraryReadingpositionSetPositionCall<'a, C, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Random persistent device cookie optional on set position.
    ///
    /// Sets the *device cookie* query property to the given value.
    pub fn device_cookie(mut self, new_value: &str) -> MylibraryReadingpositionSetPositionCall<'a, C, A> {
        self._device_cookie = Some(new_value.to_string());
        self
    }
    /// Volume content version for which this reading position applies.
    ///
    /// Sets the *content version* query property to the given value.
    pub fn content_version(mut self, new_value: &str) -> MylibraryReadingpositionSetPositionCall<'a, C, A> {
        self._content_version = Some(new_value.to_string());
        self
    }
    /// Action that caused this reading position to be set.
    ///
    /// Sets the *action* query property to the given value.
    pub fn action(mut self, new_value: &str) -> MylibraryReadingpositionSetPositionCall<'a, C, A> {
        self._action = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MylibraryReadingpositionSetPositionCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryReadingpositionSetPositionCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> MylibraryReadingpositionSetPositionCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves metadata for a specific bookshelf belonging to the authenticated user.
///
/// A builder for the *bookshelves.get* method supported by a *mylibrary* resource.
/// It is not used directly, but through a `MylibraryMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().bookshelves_get("shelf")
///              .source("dolor")
///              .doit();
/// # }
/// ```
pub struct MylibraryBookshelveGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _shelf: String,
    _source: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for MylibraryBookshelveGetCall<'a, C, A> {}

impl<'a, C, A> MylibraryBookshelveGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Bookshelf)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.mylibrary.bookshelves.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("shelf", self._shelf.to_string()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        for &field in ["alt", "shelf", "source"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "mylibrary/bookshelves/{shelf}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{shelf}", "shelf")].iter() {
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
            for param_name in ["shelf"].iter() {
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


    /// ID of bookshelf to retrieve.
    ///
    /// Sets the *shelf* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn shelf(mut self, new_value: &str) -> MylibraryBookshelveGetCall<'a, C, A> {
        self._shelf = new_value.to_string();
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> MylibraryBookshelveGetCall<'a, C, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MylibraryBookshelveGetCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryBookshelveGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> MylibraryBookshelveGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves a list of bookshelves belonging to the authenticated user.
///
/// A builder for the *bookshelves.list* method supported by a *mylibrary* resource.
/// It is not used directly, but through a `MylibraryMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().bookshelves_list()
///              .source("vero")
///              .doit();
/// # }
/// ```
pub struct MylibraryBookshelveListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _source: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for MylibraryBookshelveListCall<'a, C, A> {}

impl<'a, C, A> MylibraryBookshelveListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Bookshelves)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.mylibrary.bookshelves.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        for &field in ["alt", "source"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "mylibrary/bookshelves";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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


    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> MylibraryBookshelveListCall<'a, C, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MylibraryBookshelveListCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryBookshelveListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> MylibraryBookshelveListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves my reading position information for a volume.
///
/// A builder for the *readingpositions.get* method supported by a *mylibrary* resource.
/// It is not used directly, but through a `MylibraryMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.mylibrary().readingpositions_get("volumeId")
///              .source("takimata")
///              .content_version("dolores")
///              .doit();
/// # }
/// ```
pub struct MylibraryReadingpositionGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _volume_id: String,
    _source: Option<String>,
    _content_version: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for MylibraryReadingpositionGetCall<'a, C, A> {}

impl<'a, C, A> MylibraryReadingpositionGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ReadingPosition)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.mylibrary.readingpositions.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("volumeId", self._volume_id.to_string()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._content_version {
            params.push(("contentVersion", value.to_string()));
        }
        for &field in ["alt", "volumeId", "source", "contentVersion"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "mylibrary/readingpositions/{volumeId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{volumeId}", "volumeId")].iter() {
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
            for param_name in ["volumeId"].iter() {
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


    /// ID of volume for which to retrieve a reading position.
    ///
    /// Sets the *volume id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn volume_id(mut self, new_value: &str) -> MylibraryReadingpositionGetCall<'a, C, A> {
        self._volume_id = new_value.to_string();
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> MylibraryReadingpositionGetCall<'a, C, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// Volume content version for which this reading position is requested.
    ///
    /// Sets the *content version* query property to the given value.
    pub fn content_version(mut self, new_value: &str) -> MylibraryReadingpositionGetCall<'a, C, A> {
        self._content_version = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MylibraryReadingpositionGetCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MylibraryReadingpositionGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> MylibraryReadingpositionGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns Series metadata for the given series ids.
///
/// A builder for the *get* method supported by a *sery* resource.
/// It is not used directly, but through a `SeryMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.series().get("series_id")
///              .doit();
/// # }
/// ```
pub struct SeryGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _series_id: Vec<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for SeryGetCall<'a, C, A> {}

impl<'a, C, A> SeryGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Series)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.series.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        if self._series_id.len() > 0 {
            for f in self._series_id.iter() {
                params.push(("series_id", f.to_string()));
            }
        }
        for &field in ["alt", "series_id"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "series/get";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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


    /// String that identifies the series
    ///
    /// Append the given value to the *series_id* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn add_series_id(mut self, new_value: &str) -> SeryGetCall<'a, C, A> {
        self._series_id.push(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> SeryGetCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> SeryGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> SeryGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns Series membership data given the series id.
///
/// A builder for the *membership.get* method supported by a *sery* resource.
/// It is not used directly, but through a `SeryMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.series().membership_get("series_id")
///              .page_token("amet.")
///              .page_size(83)
///              .doit();
/// # }
/// ```
pub struct SeryMembershipGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _series_id: String,
    _page_token: Option<String>,
    _page_size: Option<u32>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for SeryMembershipGetCall<'a, C, A> {}

impl<'a, C, A> SeryMembershipGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Seriesmembership)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.series.membership.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("series_id", self._series_id.to_string()));
        if let Some(value) = self._page_token {
            params.push(("page_token", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("page_size", value.to_string()));
        }
        for &field in ["alt", "series_id", "page_token", "page_size"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "series/membership/get";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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


    /// String that identifies the series
    ///
    /// Sets the *series_id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn series_id(mut self, new_value: &str) -> SeryMembershipGetCall<'a, C, A> {
        self._series_id = new_value.to_string();
        self
    }
    /// The value of the nextToken from the previous page.
    ///
    /// Sets the *page_token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> SeryMembershipGetCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Number of maximum results per page to be included in the response.
    ///
    /// Sets the *page_size* query property to the given value.
    pub fn page_size(mut self, new_value: u32) -> SeryMembershipGetCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> SeryMembershipGetCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> SeryMembershipGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> SeryMembershipGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// 
///
/// A builder for the *addBook* method supported by a *cloudloading* resource.
/// It is not used directly, but through a `CloudloadingMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.cloudloading().add_book()
///              .upload_client_token("dolores")
///              .name("et")
///              .mime_type("sed")
///              .drive_document_id("et")
///              .doit();
/// # }
/// ```
pub struct CloudloadingAddBookCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _upload_client_token: Option<String>,
    _name: Option<String>,
    _mime_type: Option<String>,
    _drive_document_id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CloudloadingAddBookCall<'a, C, A> {}

impl<'a, C, A> CloudloadingAddBookCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, BooksCloudloadingResource)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.cloudloading.addBook",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        if let Some(value) = self._upload_client_token {
            params.push(("upload_client_token", value.to_string()));
        }
        if let Some(value) = self._name {
            params.push(("name", value.to_string()));
        }
        if let Some(value) = self._mime_type {
            params.push(("mime_type", value.to_string()));
        }
        if let Some(value) = self._drive_document_id {
            params.push(("drive_document_id", value.to_string()));
        }
        for &field in ["alt", "upload_client_token", "name", "mime_type", "drive_document_id"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "cloudloading/addBook";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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


    ///
    /// Sets the *upload_client_token* query property to the given value.
    pub fn upload_client_token(mut self, new_value: &str) -> CloudloadingAddBookCall<'a, C, A> {
        self._upload_client_token = Some(new_value.to_string());
        self
    }
    /// The document name. It can be set only if the drive_document_id is set.
    ///
    /// Sets the *name* query property to the given value.
    pub fn name(mut self, new_value: &str) -> CloudloadingAddBookCall<'a, C, A> {
        self._name = Some(new_value.to_string());
        self
    }
    /// The document MIME type. It can be set only if the drive_document_id is set.
    ///
    /// Sets the *mime_type* query property to the given value.
    pub fn mime_type(mut self, new_value: &str) -> CloudloadingAddBookCall<'a, C, A> {
        self._mime_type = Some(new_value.to_string());
        self
    }
    /// A drive document id. The upload_client_token must not be set.
    ///
    /// Sets the *drive_document_id* query property to the given value.
    pub fn drive_document_id(mut self, new_value: &str) -> CloudloadingAddBookCall<'a, C, A> {
        self._drive_document_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CloudloadingAddBookCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> CloudloadingAddBookCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> CloudloadingAddBookCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// 
///
/// A builder for the *updateBook* method supported by a *cloudloading* resource.
/// It is not used directly, but through a `CloudloadingMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// use books1::BooksCloudloadingResource;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BooksCloudloadingResource::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.cloudloading().update_book(req)
///              .doit();
/// # }
/// ```
pub struct CloudloadingUpdateBookCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _request: BooksCloudloadingResource,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CloudloadingUpdateBookCall<'a, C, A> {}

impl<'a, C, A> CloudloadingUpdateBookCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, BooksCloudloadingResource)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.cloudloading.updateBook",
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

        let mut url = self.hub._base_url.clone() + "cloudloading/updateBook";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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
    pub fn request(mut self, new_value: BooksCloudloadingResource) -> CloudloadingUpdateBookCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CloudloadingUpdateBookCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> CloudloadingUpdateBookCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> CloudloadingUpdateBookCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Remove the book and its contents
///
/// A builder for the *deleteBook* method supported by a *cloudloading* resource.
/// It is not used directly, but through a `CloudloadingMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.cloudloading().delete_book("volumeId")
///              .doit();
/// # }
/// ```
pub struct CloudloadingDeleteBookCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _volume_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CloudloadingDeleteBookCall<'a, C, A> {}

impl<'a, C, A> CloudloadingDeleteBookCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.cloudloading.deleteBook",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(2 + self._additional_params.len());
        params.push(("volumeId", self._volume_id.to_string()));
        for &field in ["volumeId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "cloudloading/deleteBook";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The id of the book to be removed.
    ///
    /// Sets the *volume id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn volume_id(mut self, new_value: &str) -> CloudloadingDeleteBookCall<'a, C, A> {
        self._volume_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CloudloadingDeleteBookCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> CloudloadingDeleteBookCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> CloudloadingDeleteBookCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns notification details for a given notification id.
///
/// A builder for the *get* method supported by a *notification* resource.
/// It is not used directly, but through a `NotificationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_books1 as books1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use books1::Books;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Books::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.notification().get("notification_id")
///              .source("sit")
///              .locale("aliquyam")
///              .doit();
/// # }
/// ```
pub struct NotificationGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Books<C, A>,
    _notification_id: String,
    _source: Option<String>,
    _locale: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for NotificationGetCall<'a, C, A> {}

impl<'a, C, A> NotificationGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Notification)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "books.notification.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("notification_id", self._notification_id.to_string()));
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._locale {
            params.push(("locale", value.to_string()));
        }
        for &field in ["alt", "notification_id", "source", "locale"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "notification/get";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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


    /// String to identify the notification.
    ///
    /// Sets the *notification_id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn notification_id(mut self, new_value: &str) -> NotificationGetCall<'a, C, A> {
        self._notification_id = new_value.to_string();
        self
    }
    /// String to identify the originator of this request.
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> NotificationGetCall<'a, C, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'. Used for generating notification title and body.
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> NotificationGetCall<'a, C, A> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> NotificationGetCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> NotificationGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> NotificationGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


