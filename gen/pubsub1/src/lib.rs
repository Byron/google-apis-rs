// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Pubsub* crate version *6.0.0+20240618*, where *20240618* is the exact revision of the *pubsub:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v6.0.0*.
//!
//! Everything else about the *Pubsub* *v1* API can be found at the
//! [official documentation site](https://cloud.google.com/pubsub/docs).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/pubsub1).
//! # Features
//!
//! Handle the following *Resources* with ease from the central [hub](Pubsub) ...
//!
//! * projects
//!  * [*schemas commit*](api::ProjectSchemaCommitCall), [*schemas create*](api::ProjectSchemaCreateCall), [*schemas delete*](api::ProjectSchemaDeleteCall), [*schemas delete revision*](api::ProjectSchemaDeleteRevisionCall), [*schemas get*](api::ProjectSchemaGetCall), [*schemas get iam policy*](api::ProjectSchemaGetIamPolicyCall), [*schemas list*](api::ProjectSchemaListCall), [*schemas list revisions*](api::ProjectSchemaListRevisionCall), [*schemas rollback*](api::ProjectSchemaRollbackCall), [*schemas set iam policy*](api::ProjectSchemaSetIamPolicyCall), [*schemas test iam permissions*](api::ProjectSchemaTestIamPermissionCall), [*schemas validate*](api::ProjectSchemaValidateCall), [*schemas validate message*](api::ProjectSchemaValidateMessageCall), [*snapshots create*](api::ProjectSnapshotCreateCall), [*snapshots delete*](api::ProjectSnapshotDeleteCall), [*snapshots get*](api::ProjectSnapshotGetCall), [*snapshots get iam policy*](api::ProjectSnapshotGetIamPolicyCall), [*snapshots list*](api::ProjectSnapshotListCall), [*snapshots patch*](api::ProjectSnapshotPatchCall), [*snapshots set iam policy*](api::ProjectSnapshotSetIamPolicyCall), [*snapshots test iam permissions*](api::ProjectSnapshotTestIamPermissionCall), [*subscriptions acknowledge*](api::ProjectSubscriptionAcknowledgeCall), [*subscriptions create*](api::ProjectSubscriptionCreateCall), [*subscriptions delete*](api::ProjectSubscriptionDeleteCall), [*subscriptions detach*](api::ProjectSubscriptionDetachCall), [*subscriptions get*](api::ProjectSubscriptionGetCall), [*subscriptions get iam policy*](api::ProjectSubscriptionGetIamPolicyCall), [*subscriptions list*](api::ProjectSubscriptionListCall), [*subscriptions modify ack deadline*](api::ProjectSubscriptionModifyAckDeadlineCall), [*subscriptions modify push config*](api::ProjectSubscriptionModifyPushConfigCall), [*subscriptions patch*](api::ProjectSubscriptionPatchCall), [*subscriptions pull*](api::ProjectSubscriptionPullCall), [*subscriptions seek*](api::ProjectSubscriptionSeekCall), [*subscriptions set iam policy*](api::ProjectSubscriptionSetIamPolicyCall), [*subscriptions test iam permissions*](api::ProjectSubscriptionTestIamPermissionCall), [*topics create*](api::ProjectTopicCreateCall), [*topics delete*](api::ProjectTopicDeleteCall), [*topics get*](api::ProjectTopicGetCall), [*topics get iam policy*](api::ProjectTopicGetIamPolicyCall), [*topics list*](api::ProjectTopicListCall), [*topics patch*](api::ProjectTopicPatchCall), [*topics publish*](api::ProjectTopicPublishCall), [*topics set iam policy*](api::ProjectTopicSetIamPolicyCall), [*topics snapshots list*](api::ProjectTopicSnapshotListCall), [*topics subscriptions list*](api::ProjectTopicSubscriptionListCall) and [*topics test iam permissions*](api::ProjectTopicTestIamPermissionCall)
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
//! * **[Hub](Pubsub)**
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
//! let r = hub.projects().schemas_get_iam_policy(...).doit().await
//! let r = hub.projects().schemas_set_iam_policy(...).doit().await
//! let r = hub.projects().snapshots_get_iam_policy(...).doit().await
//! let r = hub.projects().snapshots_set_iam_policy(...).doit().await
//! let r = hub.projects().subscriptions_get_iam_policy(...).doit().await
//! let r = hub.projects().subscriptions_set_iam_policy(...).doit().await
//! let r = hub.projects().topics_get_iam_policy(...).doit().await
//! let r = hub.projects().topics_set_iam_policy(...).doit().await
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
//! google-pubsub1 = "*"
//! serde = "1"
//! serde_json = "1"
//! ```
//!
//! ## A complete example
//!
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate google_pubsub1 as pubsub1;
//! use pubsub1::{Result, Error};
//! # async fn dox() {
//! use pubsub1::{Pubsub, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
//! let mut hub = Pubsub::new(client, auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.projects().schemas_get_iam_policy("resource")
//!              .options_requested_policy_version(-55)
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
pub use api::Pubsub;
