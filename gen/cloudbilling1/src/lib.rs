// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Cloudbilling* crate version *6.0.0+20240614*, where *20240614* is the exact revision of the *cloudbilling:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v6.0.0*.
//!
//! Everything else about the *Cloudbilling* *v1* API can be found at the
//! [official documentation site](https://cloud.google.com/billing/docs/apis).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/cloudbilling1).
//! # Features
//!
//! Handle the following *Resources* with ease from the central [hub](Cloudbilling) ...
//!
//! * [billing accounts](api::BillingAccount)
//!  * [*create*](api::BillingAccountCreateCall), [*get*](api::BillingAccountGetCall), [*get iam policy*](api::BillingAccountGetIamPolicyCall), [*list*](api::BillingAccountListCall), [*move*](api::BillingAccountMoveCall), [*patch*](api::BillingAccountPatchCall), [*projects list*](api::BillingAccountProjectListCall), [*set iam policy*](api::BillingAccountSetIamPolicyCall), [*sub accounts create*](api::BillingAccountSubAccountCreateCall), [*sub accounts list*](api::BillingAccountSubAccountListCall) and [*test iam permissions*](api::BillingAccountTestIamPermissionCall)
//! * organizations
//!  * [*billing accounts create*](api::OrganizationBillingAccountCreateCall), [*billing accounts list*](api::OrganizationBillingAccountListCall) and [*billing accounts move*](api::OrganizationBillingAccountMoveCall)
//! * projects
//!  * [*get billing info*](api::ProjectGetBillingInfoCall) and [*update billing info*](api::ProjectUpdateBillingInfoCall)
//! * [services](api::Service)
//!  * [*list*](api::ServiceListCall) and [*skus list*](api::ServiceSkuListCall)
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
//! * **[Hub](Cloudbilling)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](common::MethodsBuilder) which in turn
//!       allow access to individual [*Call Builders*](common::CallBuilder)
//! * **[Resources](common::Resource)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](common::Part)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](common::CallBuilder)**
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
//! let r = hub.billing_accounts().projects_list(...).doit().await
//! let r = hub.billing_accounts().sub_accounts_create(...).doit().await
//! let r = hub.billing_accounts().sub_accounts_list(...).doit().await
//! let r = hub.billing_accounts().create(...).doit().await
//! let r = hub.billing_accounts().get(...).doit().await
//! let r = hub.billing_accounts().get_iam_policy(...).doit().await
//! let r = hub.billing_accounts().list(...).doit().await
//! let r = hub.billing_accounts().move_(...).doit().await
//! let r = hub.billing_accounts().patch(...).doit().await
//! let r = hub.billing_accounts().set_iam_policy(...).doit().await
//! let r = hub.billing_accounts().test_iam_permissions(...).doit().await
//! let r = hub.organizations().billing_accounts_create(...).doit().await
//! let r = hub.organizations().billing_accounts_move(...).doit().await
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
//! google-cloudbilling1 = "*"
//! serde = "1"
//! serde_json = "1"
//! ```
//!
//! ## A complete example
//!
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate google_cloudbilling1 as cloudbilling1;
//! use cloudbilling1::{Result, Error};
//! # async fn dox() {
//! use cloudbilling1::{Cloudbilling, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
//!
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and
//! // `client_secret`, among other things.
//! let secret: yup_oauth2::ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you,
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
//!     secret,
//!     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
//! ).build().await.unwrap();
//!
//! let client = hyper_util::client::legacy::Client::builder(
//!     hyper_util::rt::TokioExecutor::new()
//! )
//! .build(
//!     hyper_rustls::HttpsConnectorBuilder::new()
//!         .with_native_roots()
//!         .unwrap()
//!         .https_or_http()
//!         .enable_http1()
//!         .build()
//! );
//! let mut hub = Cloudbilling::new(client, auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.billing_accounts().sub_accounts_list("parent")
//!              .page_token("At")
//!              .page_size(-8)
//!              .filter("sed")
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
//! All errors produced by the system are provided either as [Result](common::Result) enumeration as return value of
//! the doit() methods, or handed as possibly intermediate results to either the
//! [Hub Delegate](common::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).
//!
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This
//! makes the system potentially resilient to all kinds of errors.
//!
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](common::Result), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](common::ResponseResult), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//!
//! Methods supporting uploads can do so using up to 2 different protocols:
//! *simple* and *resumable*. The distinctiveness of each is represented by customized
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//!
//! ## Customization and Callbacks
//!
//! You may alter the way an `doit()` method is called by providing a [delegate](common::Delegate) to the
//! [Method Builder](common::CallBuilder) before making the final `doit()` call.
//! Respective methods will be called to provide progress information, as well as determine whether the system should
//! retry on failure.
//!
//! The [delegate trait](common::Delegate) is default-implemented, allowing you to customize it with minimal effort.
//!
//! ## Optional Parts in Server-Requests
//!
//! All structures provided by this library are made to be [encodable](common::RequestValue) and
//! [decodable](common::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses
//! are valid.
//! Most optionals are are considered [Parts](common::Part) which are identifiable by name, which will be sent to
//! the server to indicate either the set parts of the request or the desired parts in the response.
//!
//! ## Builder Arguments
//!
//! Using [method builders](common::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//!
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](common::RequestValue) are moved
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

// Unused attributes happen thanks to defined, but unused structures We don't
// warn about this, as depending on the API, some data structures or facilities
// are never used. Instead of pre-determining this, we just disable the lint.
// It's manually tuned to not have any unused imports in fully featured APIs.
// Same with unused_mut.
#![allow(unused_imports, unused_mut, dead_code)]

// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

pub extern crate hyper;
pub extern crate hyper_rustls;
pub extern crate hyper_util;
#[cfg(feature = "yup-oauth2")]
pub extern crate yup_oauth2;

pub extern crate google_apis_common as common;
pub use common::{Delegate, Error, FieldMask, Result};

pub mod api;
pub use api::Cloudbilling;
