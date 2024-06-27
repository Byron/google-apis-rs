// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Android Enterprise* crate version *5.0.5+20240625*, where *20240625* is the exact revision of the *androidenterprise:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.
//! 
//! Everything else about the *Android Enterprise* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/android/work/play/emm-api).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/androidenterprise1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](AndroidEnterprise) ... 
//! 
//! * [devices](api::Device)
//!  * [*force report upload*](api::DeviceForceReportUploadCall), [*get*](api::DeviceGetCall), [*get state*](api::DeviceGetStateCall), [*list*](api::DeviceListCall), [*set state*](api::DeviceSetStateCall) and [*update*](api::DeviceUpdateCall)
//! * [enterprises](api::Enterprise)
//!  * [*acknowledge notification set*](api::EnterpriseAcknowledgeNotificationSetCall), [*complete signup*](api::EnterpriseCompleteSignupCall), [*create enrollment token*](api::EnterpriseCreateEnrollmentTokenCall), [*create web token*](api::EnterpriseCreateWebTokenCall), [*enroll*](api::EnterpriseEnrollCall), [*generate signup url*](api::EnterpriseGenerateSignupUrlCall), [*get*](api::EnterpriseGetCall), [*get service account*](api::EnterpriseGetServiceAccountCall), [*get store layout*](api::EnterpriseGetStoreLayoutCall), [*list*](api::EnterpriseListCall), [*pull notification set*](api::EnterprisePullNotificationSetCall), [*send test push notification*](api::EnterpriseSendTestPushNotificationCall), [*set account*](api::EnterpriseSetAccountCall), [*set store layout*](api::EnterpriseSetStoreLayoutCall) and [*unenroll*](api::EnterpriseUnenrollCall)
//! * [entitlements](api::Entitlement)
//!  * [*delete*](api::EntitlementDeleteCall), [*get*](api::EntitlementGetCall), [*list*](api::EntitlementListCall) and [*update*](api::EntitlementUpdateCall)
//! * grouplicenses
//!  * [*get*](api::GrouplicenseGetCall) and [*list*](api::GrouplicenseListCall)
//! * grouplicenseusers
//!  * [*list*](api::GrouplicenseuserListCall)
//! * [installs](api::Install)
//!  * [*delete*](api::InstallDeleteCall), [*get*](api::InstallGetCall), [*list*](api::InstallListCall) and [*update*](api::InstallUpdateCall)
//! * managedconfigurationsfordevice
//!  * [*delete*](api::ManagedconfigurationsfordeviceDeleteCall), [*get*](api::ManagedconfigurationsfordeviceGetCall), [*list*](api::ManagedconfigurationsfordeviceListCall) and [*update*](api::ManagedconfigurationsfordeviceUpdateCall)
//! * managedconfigurationsforuser
//!  * [*delete*](api::ManagedconfigurationsforuserDeleteCall), [*get*](api::ManagedconfigurationsforuserGetCall), [*list*](api::ManagedconfigurationsforuserListCall) and [*update*](api::ManagedconfigurationsforuserUpdateCall)
//! * managedconfigurationssettings
//!  * [*list*](api::ManagedconfigurationssettingListCall)
//! * [permissions](api::Permission)
//!  * [*get*](api::PermissionGetCall)
//! * [products](api::Product)
//!  * [*approve*](api::ProductApproveCall), [*generate approval url*](api::ProductGenerateApprovalUrlCall), [*get*](api::ProductGetCall), [*get app restrictions schema*](api::ProductGetAppRestrictionsSchemaCall), [*get permissions*](api::ProductGetPermissionCall), [*list*](api::ProductListCall) and [*unapprove*](api::ProductUnapproveCall)
//! * serviceaccountkeys
//!  * [*delete*](api::ServiceaccountkeyDeleteCall), [*insert*](api::ServiceaccountkeyInsertCall) and [*list*](api::ServiceaccountkeyListCall)
//! * storelayoutclusters
//!  * [*delete*](api::StorelayoutclusterDeleteCall), [*get*](api::StorelayoutclusterGetCall), [*insert*](api::StorelayoutclusterInsertCall), [*list*](api::StorelayoutclusterListCall) and [*update*](api::StorelayoutclusterUpdateCall)
//! * storelayoutpages
//!  * [*delete*](api::StorelayoutpageDeleteCall), [*get*](api::StorelayoutpageGetCall), [*insert*](api::StorelayoutpageInsertCall), [*list*](api::StorelayoutpageListCall) and [*update*](api::StorelayoutpageUpdateCall)
//! * [users](api::User)
//!  * [*delete*](api::UserDeleteCall), [*generate authentication token*](api::UserGenerateAuthenticationTokenCall), [*get*](api::UserGetCall), [*get available product set*](api::UserGetAvailableProductSetCall), [*insert*](api::UserInsertCall), [*list*](api::UserListCall), [*revoke device access*](api::UserRevokeDeviceAccesCall), [*set available product set*](api::UserSetAvailableProductSetCall) and [*update*](api::UserUpdateCall)
//! * webapps
//!  * [*delete*](api::WebappDeleteCall), [*get*](api::WebappGetCall), [*insert*](api::WebappInsertCall), [*list*](api::WebappListCall) and [*update*](api::WebappUpdateCall)
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
//! * **[Hub](AndroidEnterprise)**
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
//! let r = hub.enterprises().acknowledge_notification_set(...).doit().await
//! let r = hub.enterprises().complete_signup(...).doit().await
//! let r = hub.enterprises().create_enrollment_token(...).doit().await
//! let r = hub.enterprises().create_web_token(...).doit().await
//! let r = hub.enterprises().enroll(...).doit().await
//! let r = hub.enterprises().generate_signup_url(...).doit().await
//! let r = hub.enterprises().get(...).doit().await
//! let r = hub.enterprises().get_service_account(...).doit().await
//! let r = hub.enterprises().get_store_layout(...).doit().await
//! let r = hub.enterprises().list(...).doit().await
//! let r = hub.enterprises().pull_notification_set(...).doit().await
//! let r = hub.enterprises().send_test_push_notification(...).doit().await
//! let r = hub.enterprises().set_account(...).doit().await
//! let r = hub.enterprises().set_store_layout(...).doit().await
//! let r = hub.enterprises().unenroll(...).doit().await
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
//! google-androidenterprise1 = "*"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate google_androidenterprise1 as androidenterprise1;
//! use androidenterprise1::{Result, Error};
//! # async fn dox() {
//! use std::default::Default;
//! use androidenterprise1::{AndroidEnterprise, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
//! let mut hub = AndroidEnterprise::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.enterprises().complete_signup()
//!              .enterprise_token("no")
//!              .completion_token("ipsum")
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
pub use api::AndroidEnterprise;
pub use client::{Result, Error, Delegate, FieldMask};

// Re-export the yup_oauth2 crate, that is required to call some methods of the hub and the client
#[cfg(feature = "yup-oauth2")]
pub use client::oauth2;