// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Dns* crate version *5.0.5+20240616*, where *20240616* is the exact revision of the *dns:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.
//! 
//! Everything else about the *Dns* *v2* API can be found at the
//! [official documentation site](https://cloud.google.com/dns/docs).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/dns2).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](Dns) ... 
//! 
//! * [changes](api::Change)
//!  * [*create*](api::ChangeCreateCall), [*get*](api::ChangeGetCall) and [*list*](api::ChangeListCall)
//! * [dns keys](api::DnsKey)
//!  * [*get*](api::DnsKeyGetCall) and [*list*](api::DnsKeyListCall)
//! * managed zone operations
//!  * [*get*](api::ManagedZoneOperationGetCall) and [*list*](api::ManagedZoneOperationListCall)
//! * [managed zones](api::ManagedZone)
//!  * [*create*](api::ManagedZoneCreateCall), [*delete*](api::ManagedZoneDeleteCall), [*get*](api::ManagedZoneGetCall), [*get iam policy*](api::ManagedZoneGetIamPolicyCall), [*list*](api::ManagedZoneListCall), [*patch*](api::ManagedZonePatchCall), [*set iam policy*](api::ManagedZoneSetIamPolicyCall), [*test iam permissions*](api::ManagedZoneTestIamPermissionCall) and [*update*](api::ManagedZoneUpdateCall)
//! * [policies](api::Policy)
//!  * [*create*](api::PolicyCreateCall), [*delete*](api::PolicyDeleteCall), [*get*](api::PolicyGetCall), [*list*](api::PolicyListCall), [*patch*](api::PolicyPatchCall) and [*update*](api::PolicyUpdateCall)
//! * [projects](api::Project)
//!  * [*get*](api::ProjectGetCall)
//! * [resource record sets](api::ResourceRecordSet)
//!  * [*create*](api::ResourceRecordSetCreateCall), [*delete*](api::ResourceRecordSetDeleteCall), [*get*](api::ResourceRecordSetGetCall), [*list*](api::ResourceRecordSetListCall) and [*patch*](api::ResourceRecordSetPatchCall)
//! * [response policies](api::ResponsePolicy)
//!  * [*create*](api::ResponsePolicyCreateCall), [*delete*](api::ResponsePolicyDeleteCall), [*get*](api::ResponsePolicyGetCall), [*list*](api::ResponsePolicyListCall), [*patch*](api::ResponsePolicyPatchCall) and [*update*](api::ResponsePolicyUpdateCall)
//! * [response policy rules](api::ResponsePolicyRule)
//!  * [*create*](api::ResponsePolicyRuleCreateCall), [*delete*](api::ResponsePolicyRuleDeleteCall), [*get*](api::ResponsePolicyRuleGetCall), [*list*](api::ResponsePolicyRuleListCall), [*patch*](api::ResponsePolicyRulePatchCall) and [*update*](api::ResponsePolicyRuleUpdateCall)
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
//! * **[Hub](Dns)**
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
//! let r = hub.managed_zones().create(...).doit().await
//! let r = hub.managed_zones().delete(...).doit().await
//! let r = hub.managed_zones().get(...).doit().await
//! let r = hub.managed_zones().get_iam_policy(...).doit().await
//! let r = hub.managed_zones().list(...).doit().await
//! let r = hub.managed_zones().patch(...).doit().await
//! let r = hub.managed_zones().set_iam_policy(...).doit().await
//! let r = hub.managed_zones().test_iam_permissions(...).doit().await
//! let r = hub.managed_zones().update(...).doit().await
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
//! google-dns2 = "*"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate google_dns2 as dns2;
//! use dns2::{Result, Error};
//! # async fn dox() {
//! use std::default::Default;
//! use dns2::{Dns, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
//! let mut hub = Dns::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.managed_zones().list("project", "location")
//!              .page_token("sed")
//!              .max_results(-2)
//!              .dns_name("takimata")
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
pub use api::Dns;
pub use client::{Result, Error, Delegate, FieldMask};

// Re-export the yup_oauth2 crate, that is required to call some methods of the hub and the client
#[cfg(feature = "yup-oauth2")]
pub use client::oauth2;