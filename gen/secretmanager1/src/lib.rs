// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Secret Manager* crate version *5.0.5+20240621*, where *20240621* is the exact revision of the *secretmanager:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.
//! 
//! Everything else about the *Secret Manager* *v1* API can be found at the
//! [official documentation site](https://cloud.google.com/secret-manager/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/secretmanager1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](SecretManager) ... 
//! 
//! * projects
//!  * [*locations get*](api::ProjectLocationGetCall), [*locations list*](api::ProjectLocationListCall), [*locations secrets add version*](api::ProjectLocationSecretAddVersionCall), [*locations secrets create*](api::ProjectLocationSecretCreateCall), [*locations secrets delete*](api::ProjectLocationSecretDeleteCall), [*locations secrets get*](api::ProjectLocationSecretGetCall), [*locations secrets get iam policy*](api::ProjectLocationSecretGetIamPolicyCall), [*locations secrets list*](api::ProjectLocationSecretListCall), [*locations secrets patch*](api::ProjectLocationSecretPatchCall), [*locations secrets set iam policy*](api::ProjectLocationSecretSetIamPolicyCall), [*locations secrets test iam permissions*](api::ProjectLocationSecretTestIamPermissionCall), [*locations secrets versions access*](api::ProjectLocationSecretVersionAccesCall), [*locations secrets versions destroy*](api::ProjectLocationSecretVersionDestroyCall), [*locations secrets versions disable*](api::ProjectLocationSecretVersionDisableCall), [*locations secrets versions enable*](api::ProjectLocationSecretVersionEnableCall), [*locations secrets versions get*](api::ProjectLocationSecretVersionGetCall), [*locations secrets versions list*](api::ProjectLocationSecretVersionListCall), [*secrets add version*](api::ProjectSecretAddVersionCall), [*secrets create*](api::ProjectSecretCreateCall), [*secrets delete*](api::ProjectSecretDeleteCall), [*secrets get*](api::ProjectSecretGetCall), [*secrets get iam policy*](api::ProjectSecretGetIamPolicyCall), [*secrets list*](api::ProjectSecretListCall), [*secrets patch*](api::ProjectSecretPatchCall), [*secrets set iam policy*](api::ProjectSecretSetIamPolicyCall), [*secrets test iam permissions*](api::ProjectSecretTestIamPermissionCall), [*secrets versions access*](api::ProjectSecretVersionAccesCall), [*secrets versions destroy*](api::ProjectSecretVersionDestroyCall), [*secrets versions disable*](api::ProjectSecretVersionDisableCall), [*secrets versions enable*](api::ProjectSecretVersionEnableCall), [*secrets versions get*](api::ProjectSecretVersionGetCall) and [*secrets versions list*](api::ProjectSecretVersionListCall)
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
//! * **[Hub](SecretManager)**
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
//! let r = hub.projects().locations_secrets_versions_destroy(...).doit().await
//! let r = hub.projects().locations_secrets_versions_disable(...).doit().await
//! let r = hub.projects().locations_secrets_versions_enable(...).doit().await
//! let r = hub.projects().locations_secrets_versions_get(...).doit().await
//! let r = hub.projects().locations_secrets_add_version(...).doit().await
//! let r = hub.projects().secrets_versions_destroy(...).doit().await
//! let r = hub.projects().secrets_versions_disable(...).doit().await
//! let r = hub.projects().secrets_versions_enable(...).doit().await
//! let r = hub.projects().secrets_versions_get(...).doit().await
//! let r = hub.projects().secrets_add_version(...).doit().await
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
//! google-secretmanager1 = "*"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate google_secretmanager1 as secretmanager1;
//! use secretmanager1::api::DestroySecretVersionRequest;
//! use secretmanager1::{Result, Error};
//! # async fn dox() {
//! use std::default::Default;
//! use secretmanager1::{SecretManager, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
//! let mut hub = SecretManager::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = DestroySecretVersionRequest::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.projects().locations_secrets_versions_destroy(req, "name")
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
pub use api::SecretManager;
pub use client::{Result, Error, Delegate, FieldMask};

// Re-export the yup_oauth2 crate, that is required to call some methods of the hub and the client
#[cfg(feature = "yup-oauth2")]
pub use client::oauth2;