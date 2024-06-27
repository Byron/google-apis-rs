// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *AI Platform Notebooks* crate version *5.0.5+20240613*, where *20240613* is the exact revision of the *notebooks:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.
//! 
//! Everything else about the *AI Platform Notebooks* *v1* API can be found at the
//! [official documentation site](https://cloud.google.com/notebooks/docs/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/notebooks1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](AIPlatformNotebooks) ... 
//! 
//! * projects
//!  * [*locations environments create*](api::ProjectLocationEnvironmentCreateCall), [*locations environments delete*](api::ProjectLocationEnvironmentDeleteCall), [*locations environments get*](api::ProjectLocationEnvironmentGetCall), [*locations environments list*](api::ProjectLocationEnvironmentListCall), [*locations executions create*](api::ProjectLocationExecutionCreateCall), [*locations executions delete*](api::ProjectLocationExecutionDeleteCall), [*locations executions get*](api::ProjectLocationExecutionGetCall), [*locations executions list*](api::ProjectLocationExecutionListCall), [*locations get*](api::ProjectLocationGetCall), [*locations instances create*](api::ProjectLocationInstanceCreateCall), [*locations instances delete*](api::ProjectLocationInstanceDeleteCall), [*locations instances diagnose*](api::ProjectLocationInstanceDiagnoseCall), [*locations instances get*](api::ProjectLocationInstanceGetCall), [*locations instances get iam policy*](api::ProjectLocationInstanceGetIamPolicyCall), [*locations instances get instance health*](api::ProjectLocationInstanceGetInstanceHealthCall), [*locations instances is upgradeable*](api::ProjectLocationInstanceIsUpgradeableCall), [*locations instances list*](api::ProjectLocationInstanceListCall), [*locations instances migrate*](api::ProjectLocationInstanceMigrateCall), [*locations instances register*](api::ProjectLocationInstanceRegisterCall), [*locations instances report*](api::ProjectLocationInstanceReportCall), [*locations instances report event*](api::ProjectLocationInstanceReportEventCall), [*locations instances reset*](api::ProjectLocationInstanceResetCall), [*locations instances rollback*](api::ProjectLocationInstanceRollbackCall), [*locations instances set accelerator*](api::ProjectLocationInstanceSetAcceleratorCall), [*locations instances set iam policy*](api::ProjectLocationInstanceSetIamPolicyCall), [*locations instances set labels*](api::ProjectLocationInstanceSetLabelCall), [*locations instances set machine type*](api::ProjectLocationInstanceSetMachineTypeCall), [*locations instances start*](api::ProjectLocationInstanceStartCall), [*locations instances stop*](api::ProjectLocationInstanceStopCall), [*locations instances test iam permissions*](api::ProjectLocationInstanceTestIamPermissionCall), [*locations instances update config*](api::ProjectLocationInstanceUpdateConfigCall), [*locations instances update metadata items*](api::ProjectLocationInstanceUpdateMetadataItemCall), [*locations instances update shielded instance config*](api::ProjectLocationInstanceUpdateShieldedInstanceConfigCall), [*locations instances upgrade*](api::ProjectLocationInstanceUpgradeCall), [*locations instances upgrade internal*](api::ProjectLocationInstanceUpgradeInternalCall), [*locations list*](api::ProjectLocationListCall), [*locations operations cancel*](api::ProjectLocationOperationCancelCall), [*locations operations delete*](api::ProjectLocationOperationDeleteCall), [*locations operations get*](api::ProjectLocationOperationGetCall), [*locations operations list*](api::ProjectLocationOperationListCall), [*locations runtimes create*](api::ProjectLocationRuntimeCreateCall), [*locations runtimes delete*](api::ProjectLocationRuntimeDeleteCall), [*locations runtimes diagnose*](api::ProjectLocationRuntimeDiagnoseCall), [*locations runtimes get*](api::ProjectLocationRuntimeGetCall), [*locations runtimes get iam policy*](api::ProjectLocationRuntimeGetIamPolicyCall), [*locations runtimes list*](api::ProjectLocationRuntimeListCall), [*locations runtimes migrate*](api::ProjectLocationRuntimeMigrateCall), [*locations runtimes patch*](api::ProjectLocationRuntimePatchCall), [*locations runtimes refresh runtime token internal*](api::ProjectLocationRuntimeRefreshRuntimeTokenInternalCall), [*locations runtimes report event*](api::ProjectLocationRuntimeReportEventCall), [*locations runtimes reset*](api::ProjectLocationRuntimeResetCall), [*locations runtimes set iam policy*](api::ProjectLocationRuntimeSetIamPolicyCall), [*locations runtimes start*](api::ProjectLocationRuntimeStartCall), [*locations runtimes stop*](api::ProjectLocationRuntimeStopCall), [*locations runtimes switch*](api::ProjectLocationRuntimeSwitchCall), [*locations runtimes test iam permissions*](api::ProjectLocationRuntimeTestIamPermissionCall), [*locations runtimes upgrade*](api::ProjectLocationRuntimeUpgradeCall), [*locations schedules create*](api::ProjectLocationScheduleCreateCall), [*locations schedules delete*](api::ProjectLocationScheduleDeleteCall), [*locations schedules get*](api::ProjectLocationScheduleGetCall), [*locations schedules list*](api::ProjectLocationScheduleListCall) and [*locations schedules trigger*](api::ProjectLocationScheduleTriggerCall)
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
//! * **[Hub](AIPlatformNotebooks)**
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
//! let r = hub.projects().locations_environments_create(...).doit().await
//! let r = hub.projects().locations_environments_delete(...).doit().await
//! let r = hub.projects().locations_executions_create(...).doit().await
//! let r = hub.projects().locations_executions_delete(...).doit().await
//! let r = hub.projects().locations_instances_create(...).doit().await
//! let r = hub.projects().locations_instances_delete(...).doit().await
//! let r = hub.projects().locations_instances_diagnose(...).doit().await
//! let r = hub.projects().locations_instances_migrate(...).doit().await
//! let r = hub.projects().locations_instances_register(...).doit().await
//! let r = hub.projects().locations_instances_report(...).doit().await
//! let r = hub.projects().locations_instances_report_event(...).doit().await
//! let r = hub.projects().locations_instances_reset(...).doit().await
//! let r = hub.projects().locations_instances_rollback(...).doit().await
//! let r = hub.projects().locations_instances_set_accelerator(...).doit().await
//! let r = hub.projects().locations_instances_set_labels(...).doit().await
//! let r = hub.projects().locations_instances_set_machine_type(...).doit().await
//! let r = hub.projects().locations_instances_start(...).doit().await
//! let r = hub.projects().locations_instances_stop(...).doit().await
//! let r = hub.projects().locations_instances_update_config(...).doit().await
//! let r = hub.projects().locations_instances_update_shielded_instance_config(...).doit().await
//! let r = hub.projects().locations_instances_upgrade(...).doit().await
//! let r = hub.projects().locations_instances_upgrade_internal(...).doit().await
//! let r = hub.projects().locations_operations_get(...).doit().await
//! let r = hub.projects().locations_runtimes_create(...).doit().await
//! let r = hub.projects().locations_runtimes_delete(...).doit().await
//! let r = hub.projects().locations_runtimes_diagnose(...).doit().await
//! let r = hub.projects().locations_runtimes_migrate(...).doit().await
//! let r = hub.projects().locations_runtimes_patch(...).doit().await
//! let r = hub.projects().locations_runtimes_report_event(...).doit().await
//! let r = hub.projects().locations_runtimes_reset(...).doit().await
//! let r = hub.projects().locations_runtimes_start(...).doit().await
//! let r = hub.projects().locations_runtimes_stop(...).doit().await
//! let r = hub.projects().locations_runtimes_switch(...).doit().await
//! let r = hub.projects().locations_runtimes_upgrade(...).doit().await
//! let r = hub.projects().locations_schedules_create(...).doit().await
//! let r = hub.projects().locations_schedules_delete(...).doit().await
//! let r = hub.projects().locations_schedules_trigger(...).doit().await
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
//! google-notebooks1 = "*"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate google_notebooks1 as notebooks1;
//! use notebooks1::api::Runtime;
//! use notebooks1::{Result, Error};
//! # async fn dox() {
//! use std::default::Default;
//! use notebooks1::{AIPlatformNotebooks, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
//! let mut hub = AIPlatformNotebooks::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = Runtime::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.projects().locations_runtimes_create(req, "parent")
//!              .runtime_id("voluptua.")
//!              .request_id("At")
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
pub use api::AIPlatformNotebooks;
pub use client::{Result, Error, Delegate, FieldMask};

// Re-export the yup_oauth2 crate, that is required to call some methods of the hub and the client
#[cfg(feature = "yup-oauth2")]
pub use client::oauth2;