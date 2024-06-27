// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Android Management* crate version *5.0.5+20240626*, where *20240626* is the exact revision of the *androidmanagement:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.
//! 
//! Everything else about the *Android Management* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/android/management).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/androidmanagement1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](AndroidManagement) ... 
//! 
//! * [enterprises](api::Enterprise)
//!  * [*applications get*](api::EnterpriseApplicationGetCall), [*create*](api::EnterpriseCreateCall), [*delete*](api::EnterpriseDeleteCall), [*devices delete*](api::EnterpriseDeviceDeleteCall), [*devices get*](api::EnterpriseDeviceGetCall), [*devices issue command*](api::EnterpriseDeviceIssueCommandCall), [*devices list*](api::EnterpriseDeviceListCall), [*devices operations cancel*](api::EnterpriseDeviceOperationCancelCall), [*devices operations get*](api::EnterpriseDeviceOperationGetCall), [*devices operations list*](api::EnterpriseDeviceOperationListCall), [*devices patch*](api::EnterpriseDevicePatchCall), [*enrollment tokens create*](api::EnterpriseEnrollmentTokenCreateCall), [*enrollment tokens delete*](api::EnterpriseEnrollmentTokenDeleteCall), [*enrollment tokens get*](api::EnterpriseEnrollmentTokenGetCall), [*enrollment tokens list*](api::EnterpriseEnrollmentTokenListCall), [*get*](api::EnterpriseGetCall), [*list*](api::EnterpriseListCall), [*migration tokens create*](api::EnterpriseMigrationTokenCreateCall), [*migration tokens get*](api::EnterpriseMigrationTokenGetCall), [*migration tokens list*](api::EnterpriseMigrationTokenListCall), [*patch*](api::EnterprisePatchCall), [*policies delete*](api::EnterprisePolicyDeleteCall), [*policies get*](api::EnterprisePolicyGetCall), [*policies list*](api::EnterprisePolicyListCall), [*policies patch*](api::EnterprisePolicyPatchCall), [*web apps create*](api::EnterpriseWebAppCreateCall), [*web apps delete*](api::EnterpriseWebAppDeleteCall), [*web apps get*](api::EnterpriseWebAppGetCall), [*web apps list*](api::EnterpriseWebAppListCall), [*web apps patch*](api::EnterpriseWebAppPatchCall) and [*web tokens create*](api::EnterpriseWebTokenCreateCall)
//! * [provisioning info](api::ProvisioningInfo)
//!  * [*get*](api::ProvisioningInfoGetCall)
//! * [signup urls](api::SignupUrl)
//!  * [*create*](api::SignupUrlCreateCall)
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
//! * **[Hub](AndroidManagement)**
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
//! let r = hub.enterprises().applications_get(...).doit().await
//! let r = hub.enterprises().devices_operations_cancel(...).doit().await
//! let r = hub.enterprises().devices_operations_get(...).doit().await
//! let r = hub.enterprises().devices_operations_list(...).doit().await
//! let r = hub.enterprises().devices_delete(...).doit().await
//! let r = hub.enterprises().devices_get(...).doit().await
//! let r = hub.enterprises().devices_issue_command(...).doit().await
//! let r = hub.enterprises().devices_list(...).doit().await
//! let r = hub.enterprises().devices_patch(...).doit().await
//! let r = hub.enterprises().enrollment_tokens_create(...).doit().await
//! let r = hub.enterprises().enrollment_tokens_delete(...).doit().await
//! let r = hub.enterprises().enrollment_tokens_get(...).doit().await
//! let r = hub.enterprises().enrollment_tokens_list(...).doit().await
//! let r = hub.enterprises().migration_tokens_create(...).doit().await
//! let r = hub.enterprises().migration_tokens_get(...).doit().await
//! let r = hub.enterprises().migration_tokens_list(...).doit().await
//! let r = hub.enterprises().policies_delete(...).doit().await
//! let r = hub.enterprises().policies_get(...).doit().await
//! let r = hub.enterprises().policies_list(...).doit().await
//! let r = hub.enterprises().policies_patch(...).doit().await
//! let r = hub.enterprises().web_apps_create(...).doit().await
//! let r = hub.enterprises().web_apps_delete(...).doit().await
//! let r = hub.enterprises().web_apps_get(...).doit().await
//! let r = hub.enterprises().web_apps_list(...).doit().await
//! let r = hub.enterprises().web_apps_patch(...).doit().await
//! let r = hub.enterprises().web_tokens_create(...).doit().await
//! let r = hub.enterprises().create(...).doit().await
//! let r = hub.enterprises().delete(...).doit().await
//! let r = hub.enterprises().get(...).doit().await
//! let r = hub.enterprises().list(...).doit().await
//! let r = hub.enterprises().patch(...).doit().await
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
//! google-androidmanagement1 = "*"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate google_androidmanagement1 as androidmanagement1;
//! use androidmanagement1::api::Enterprise;
//! use androidmanagement1::{Result, Error};
//! # async fn dox() {
//! use std::default::Default;
//! use androidmanagement1::{AndroidManagement, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
//! let mut hub = AndroidManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = Enterprise::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.enterprises().create(req)
//!              .signup_url_name("voluptua.")
//!              .project_id("At")
//!              .enterprise_token("sanctus")
//!              .agreement_accepted(false)
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
pub use api::AndroidManagement;
pub use client::{Result, Error, Delegate, FieldMask};

// Re-export the yup_oauth2 crate, that is required to call some methods of the hub and the client
#[cfg(feature = "yup-oauth2")]
pub use client::oauth2;