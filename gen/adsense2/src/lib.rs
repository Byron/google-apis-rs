// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *adsense* crate version *5.0.5+20240626*, where *20240626* is the exact revision of the *adsense:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.
//! 
//! Everything else about the *adsense* *v2* API can be found at the
//! [official documentation site](https://developers.google.com/adsense/management/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/adsense2).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](Adsense) ... 
//! 
//! * [accounts](api::Account)
//!  * [*adclients adunits create*](api::AccountAdclientAdunitCreateCall), [*adclients adunits get*](api::AccountAdclientAdunitGetCall), [*adclients adunits get adcode*](api::AccountAdclientAdunitGetAdcodeCall), [*adclients adunits list*](api::AccountAdclientAdunitListCall), [*adclients adunits list linked custom channels*](api::AccountAdclientAdunitListLinkedCustomChannelCall), [*adclients adunits patch*](api::AccountAdclientAdunitPatchCall), [*adclients customchannels create*](api::AccountAdclientCustomchannelCreateCall), [*adclients customchannels delete*](api::AccountAdclientCustomchannelDeleteCall), [*adclients customchannels get*](api::AccountAdclientCustomchannelGetCall), [*adclients customchannels list*](api::AccountAdclientCustomchannelListCall), [*adclients customchannels list linked ad units*](api::AccountAdclientCustomchannelListLinkedAdUnitCall), [*adclients customchannels patch*](api::AccountAdclientCustomchannelPatchCall), [*adclients get*](api::AccountAdclientGetCall), [*adclients get adcode*](api::AccountAdclientGetAdcodeCall), [*adclients list*](api::AccountAdclientListCall), [*adclients urlchannels get*](api::AccountAdclientUrlchannelGetCall), [*adclients urlchannels list*](api::AccountAdclientUrlchannelListCall), [*alerts list*](api::AccountAlertListCall), [*get*](api::AccountGetCall), [*get ad blocking recovery tag*](api::AccountGetAdBlockingRecoveryTagCall), [*list*](api::AccountListCall), [*list child accounts*](api::AccountListChildAccountCall), [*payments list*](api::AccountPaymentListCall), [*policy issues get*](api::AccountPolicyIssueGetCall), [*policy issues list*](api::AccountPolicyIssueListCall), [*reports generate*](api::AccountReportGenerateCall), [*reports generate csv*](api::AccountReportGenerateCsvCall), [*reports get saved*](api::AccountReportGetSavedCall), [*reports saved generate*](api::AccountReportSavedGenerateCall), [*reports saved generate csv*](api::AccountReportSavedGenerateCsvCall), [*reports saved list*](api::AccountReportSavedListCall), [*sites get*](api::AccountSiteGetCall) and [*sites list*](api::AccountSiteListCall)
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
//! * **[Hub](Adsense)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](client::MethodsBuilder) which in turn
//!       allow access to individual [*Call Builders*](client::CallBuilder)
//! * **[Resources](client::Resource)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](client::Part)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](client::CallBuilder)**
//!     * operations to apply to *Resources*
//! 
//! All *structures* are marked with applicable traits to further categorize them and ease browsing.
//! 
//! Generally speaking, you can invoke *Activities* like this:
//! 
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit().await
//! ```
//! 
//! Or specifically ...
//! 
//! ```ignore
//! let r = hub.accounts().adclients_adunits_create(...).doit().await
//! let r = hub.accounts().adclients_adunits_get(...).doit().await
//! let r = hub.accounts().adclients_adunits_get_adcode(...).doit().await
//! let r = hub.accounts().adclients_adunits_list(...).doit().await
//! let r = hub.accounts().adclients_adunits_list_linked_custom_channels(...).doit().await
//! let r = hub.accounts().adclients_adunits_patch(...).doit().await
//! let r = hub.accounts().adclients_customchannels_create(...).doit().await
//! let r = hub.accounts().adclients_customchannels_delete(...).doit().await
//! let r = hub.accounts().adclients_customchannels_get(...).doit().await
//! let r = hub.accounts().adclients_customchannels_list(...).doit().await
//! let r = hub.accounts().adclients_customchannels_list_linked_ad_units(...).doit().await
//! let r = hub.accounts().adclients_customchannels_patch(...).doit().await
//! let r = hub.accounts().adclients_urlchannels_get(...).doit().await
//! let r = hub.accounts().adclients_urlchannels_list(...).doit().await
//! let r = hub.accounts().adclients_get(...).doit().await
//! let r = hub.accounts().adclients_get_adcode(...).doit().await
//! let r = hub.accounts().adclients_list(...).doit().await
//! let r = hub.accounts().alerts_list(...).doit().await
//! let r = hub.accounts().payments_list(...).doit().await
//! let r = hub.accounts().policy_issues_get(...).doit().await
//! let r = hub.accounts().policy_issues_list(...).doit().await
//! let r = hub.accounts().reports_saved_generate(...).doit().await
//! let r = hub.accounts().reports_saved_generate_csv(...).doit().await
//! let r = hub.accounts().reports_saved_list(...).doit().await
//! let r = hub.accounts().reports_generate(...).doit().await
//! let r = hub.accounts().reports_generate_csv(...).doit().await
//! let r = hub.accounts().reports_get_saved(...).doit().await
//! let r = hub.accounts().sites_get(...).doit().await
//! let r = hub.accounts().sites_list(...).doit().await
//! let r = hub.accounts().get(...).doit().await
//! let r = hub.accounts().get_ad_blocking_recovery_tag(...).doit().await
//! let r = hub.accounts().list(...).doit().await
//! let r = hub.accounts().list_child_accounts(...).doit().await
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
//! google-adsense2 = "*"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate google_adsense2 as adsense2;
//! use adsense2::{Result, Error};
//! # async fn dox() {
//! use std::default::Default;
//! use adsense2::{Adsense, oauth2, hyper, hyper_rustls, chrono, FieldMask};
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: oauth2::ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = oauth2::InstalledFlowAuthenticator::builder(
//!         secret,
//!         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
//!     ).build().await.unwrap();
//! let mut hub = Adsense::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.accounts().reports_generate("account")
//!              .start_date_year(-4)
//!              .start_date_month(-17)
//!              .start_date_day(-55)
//!              .reporting_time_zone("invidunt")
//!              .add_order_by("amet")
//!              .add_metrics("duo")
//!              .limit(-50)
//!              .language_code("sed")
//!              .add_filters("ut")
//!              .end_date_year(-12)
//!              .end_date_month(-16)
//!              .end_date_day(-57)
//!              .add_dimensions("ipsum")
//!              .date_range("ipsum")
//!              .currency_code("est")
//!              .doit().await;
//! 
//! match result {
//!     Err(e) => match e {
//!         // The Error enum provides details about what exactly happened.
//!         // You can also just use its `Debug`, `Display` or `Error` traits
//!          Error::HttpError(_)
//!         |Error::Io(_)
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
//! All errors produced by the system are provided either as [Result](client::Result) enumeration as return value of
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](client::Result), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](client::ResponseResult), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//! 
//! Methods supporting uploads can do so using up to 2 different protocols: 
//! *simple* and *resumable*. The distinctiveness of each is represented by customized 
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//! 
//! ## Customization and Callbacks
//! 
//! You may alter the way an `doit()` method is called by providing a [delegate](client::Delegate) to the 
//! [Method Builder](client::CallBuilder) before making the final `doit()` call. 
//! Respective methods will be called to provide progress information, as well as determine whether the system should 
//! retry on failure.
//! 
//! The [delegate trait](client::Delegate) is default-implemented, allowing you to customize it with minimal effort.
//! 
//! ## Optional Parts in Server-Requests
//! 
//! All structures provided by this library are made to be [encodable](client::RequestValue) and 
//! [decodable](client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
//! are valid.
//! Most optionals are are considered [Parts](client::Part) which are identifiable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](client::RequestValue) are moved
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! ## Cargo Features
//! 
//! * `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
//! the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
//! generated `openapi` spec would be invalid.
//! 
//! 
//! 

// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]

// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

// Re-export the hyper and hyper_rustls crate, they are required to build the hub
pub use hyper;
pub use hyper_rustls;
pub extern crate google_apis_common as client;
pub use client::chrono;
pub mod api;

// Re-export the hub type and some basic client structs
pub use api::Adsense;
pub use client::{Result, Error, Delegate, FieldMask};

// Re-export the yup_oauth2 crate, that is required to call some methods of the hub and the client
#[cfg(feature = "yup-oauth2")]
pub use client::oauth2;