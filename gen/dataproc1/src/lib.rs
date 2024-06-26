// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Dataproc* crate version *5.0.5+20240617*, where *20240617* is the exact revision of the *dataproc:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.
//! 
//! Everything else about the *Dataproc* *v1* API can be found at the
//! [official documentation site](https://cloud.google.com/dataproc/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/dataproc1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](Dataproc) ... 
//! 
//! * projects
//!  * [*locations autoscaling policies create*](api::ProjectLocationAutoscalingPolicyCreateCall), [*locations autoscaling policies delete*](api::ProjectLocationAutoscalingPolicyDeleteCall), [*locations autoscaling policies get*](api::ProjectLocationAutoscalingPolicyGetCall), [*locations autoscaling policies get iam policy*](api::ProjectLocationAutoscalingPolicyGetIamPolicyCall), [*locations autoscaling policies list*](api::ProjectLocationAutoscalingPolicyListCall), [*locations autoscaling policies set iam policy*](api::ProjectLocationAutoscalingPolicySetIamPolicyCall), [*locations autoscaling policies test iam permissions*](api::ProjectLocationAutoscalingPolicyTestIamPermissionCall), [*locations autoscaling policies update*](api::ProjectLocationAutoscalingPolicyUpdateCall), [*locations batches analyze*](api::ProjectLocationBatchAnalyzeCall), [*locations batches create*](api::ProjectLocationBatchCreateCall), [*locations batches delete*](api::ProjectLocationBatchDeleteCall), [*locations batches get*](api::ProjectLocationBatchGetCall), [*locations batches list*](api::ProjectLocationBatchListCall), [*locations operations cancel*](api::ProjectLocationOperationCancelCall), [*locations operations delete*](api::ProjectLocationOperationDeleteCall), [*locations operations get*](api::ProjectLocationOperationGetCall), [*locations operations list*](api::ProjectLocationOperationListCall), [*locations session templates create*](api::ProjectLocationSessionTemplateCreateCall), [*locations session templates delete*](api::ProjectLocationSessionTemplateDeleteCall), [*locations session templates get*](api::ProjectLocationSessionTemplateGetCall), [*locations session templates list*](api::ProjectLocationSessionTemplateListCall), [*locations session templates patch*](api::ProjectLocationSessionTemplatePatchCall), [*locations sessions create*](api::ProjectLocationSessionCreateCall), [*locations sessions delete*](api::ProjectLocationSessionDeleteCall), [*locations sessions get*](api::ProjectLocationSessionGetCall), [*locations sessions list*](api::ProjectLocationSessionListCall), [*locations sessions terminate*](api::ProjectLocationSessionTerminateCall), [*locations workflow templates create*](api::ProjectLocationWorkflowTemplateCreateCall), [*locations workflow templates delete*](api::ProjectLocationWorkflowTemplateDeleteCall), [*locations workflow templates get*](api::ProjectLocationWorkflowTemplateGetCall), [*locations workflow templates get iam policy*](api::ProjectLocationWorkflowTemplateGetIamPolicyCall), [*locations workflow templates instantiate*](api::ProjectLocationWorkflowTemplateInstantiateCall), [*locations workflow templates instantiate inline*](api::ProjectLocationWorkflowTemplateInstantiateInlineCall), [*locations workflow templates list*](api::ProjectLocationWorkflowTemplateListCall), [*locations workflow templates set iam policy*](api::ProjectLocationWorkflowTemplateSetIamPolicyCall), [*locations workflow templates test iam permissions*](api::ProjectLocationWorkflowTemplateTestIamPermissionCall), [*locations workflow templates update*](api::ProjectLocationWorkflowTemplateUpdateCall), [*regions autoscaling policies create*](api::ProjectRegionAutoscalingPolicyCreateCall), [*regions autoscaling policies delete*](api::ProjectRegionAutoscalingPolicyDeleteCall), [*regions autoscaling policies get*](api::ProjectRegionAutoscalingPolicyGetCall), [*regions autoscaling policies get iam policy*](api::ProjectRegionAutoscalingPolicyGetIamPolicyCall), [*regions autoscaling policies list*](api::ProjectRegionAutoscalingPolicyListCall), [*regions autoscaling policies set iam policy*](api::ProjectRegionAutoscalingPolicySetIamPolicyCall), [*regions autoscaling policies test iam permissions*](api::ProjectRegionAutoscalingPolicyTestIamPermissionCall), [*regions autoscaling policies update*](api::ProjectRegionAutoscalingPolicyUpdateCall), [*regions clusters create*](api::ProjectRegionClusterCreateCall), [*regions clusters delete*](api::ProjectRegionClusterDeleteCall), [*regions clusters diagnose*](api::ProjectRegionClusterDiagnoseCall), [*regions clusters get*](api::ProjectRegionClusterGetCall), [*regions clusters get iam policy*](api::ProjectRegionClusterGetIamPolicyCall), [*regions clusters inject credentials*](api::ProjectRegionClusterInjectCredentialCall), [*regions clusters list*](api::ProjectRegionClusterListCall), [*regions clusters node groups create*](api::ProjectRegionClusterNodeGroupCreateCall), [*regions clusters node groups get*](api::ProjectRegionClusterNodeGroupGetCall), [*regions clusters node groups repair*](api::ProjectRegionClusterNodeGroupRepairCall), [*regions clusters node groups resize*](api::ProjectRegionClusterNodeGroupResizeCall), [*regions clusters patch*](api::ProjectRegionClusterPatchCall), [*regions clusters repair*](api::ProjectRegionClusterRepairCall), [*regions clusters set iam policy*](api::ProjectRegionClusterSetIamPolicyCall), [*regions clusters start*](api::ProjectRegionClusterStartCall), [*regions clusters stop*](api::ProjectRegionClusterStopCall), [*regions clusters test iam permissions*](api::ProjectRegionClusterTestIamPermissionCall), [*regions jobs cancel*](api::ProjectRegionJobCancelCall), [*regions jobs delete*](api::ProjectRegionJobDeleteCall), [*regions jobs get*](api::ProjectRegionJobGetCall), [*regions jobs get iam policy*](api::ProjectRegionJobGetIamPolicyCall), [*regions jobs list*](api::ProjectRegionJobListCall), [*regions jobs patch*](api::ProjectRegionJobPatchCall), [*regions jobs set iam policy*](api::ProjectRegionJobSetIamPolicyCall), [*regions jobs submit*](api::ProjectRegionJobSubmitCall), [*regions jobs submit as operation*](api::ProjectRegionJobSubmitAsOperationCall), [*regions jobs test iam permissions*](api::ProjectRegionJobTestIamPermissionCall), [*regions operations cancel*](api::ProjectRegionOperationCancelCall), [*regions operations delete*](api::ProjectRegionOperationDeleteCall), [*regions operations get*](api::ProjectRegionOperationGetCall), [*regions operations get iam policy*](api::ProjectRegionOperationGetIamPolicyCall), [*regions operations list*](api::ProjectRegionOperationListCall), [*regions operations set iam policy*](api::ProjectRegionOperationSetIamPolicyCall), [*regions operations test iam permissions*](api::ProjectRegionOperationTestIamPermissionCall), [*regions workflow templates create*](api::ProjectRegionWorkflowTemplateCreateCall), [*regions workflow templates delete*](api::ProjectRegionWorkflowTemplateDeleteCall), [*regions workflow templates get*](api::ProjectRegionWorkflowTemplateGetCall), [*regions workflow templates get iam policy*](api::ProjectRegionWorkflowTemplateGetIamPolicyCall), [*regions workflow templates instantiate*](api::ProjectRegionWorkflowTemplateInstantiateCall), [*regions workflow templates instantiate inline*](api::ProjectRegionWorkflowTemplateInstantiateInlineCall), [*regions workflow templates list*](api::ProjectRegionWorkflowTemplateListCall), [*regions workflow templates set iam policy*](api::ProjectRegionWorkflowTemplateSetIamPolicyCall), [*regions workflow templates test iam permissions*](api::ProjectRegionWorkflowTemplateTestIamPermissionCall) and [*regions workflow templates update*](api::ProjectRegionWorkflowTemplateUpdateCall)
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
//! * **[Hub](Dataproc)**
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
//! let r = hub.projects().locations_batches_analyze(...).doit().await
//! let r = hub.projects().locations_batches_create(...).doit().await
//! let r = hub.projects().locations_operations_get(...).doit().await
//! let r = hub.projects().locations_sessions_create(...).doit().await
//! let r = hub.projects().locations_sessions_delete(...).doit().await
//! let r = hub.projects().locations_sessions_terminate(...).doit().await
//! let r = hub.projects().locations_workflow_templates_instantiate(...).doit().await
//! let r = hub.projects().locations_workflow_templates_instantiate_inline(...).doit().await
//! let r = hub.projects().regions_clusters_node_groups_create(...).doit().await
//! let r = hub.projects().regions_clusters_node_groups_repair(...).doit().await
//! let r = hub.projects().regions_clusters_node_groups_resize(...).doit().await
//! let r = hub.projects().regions_clusters_create(...).doit().await
//! let r = hub.projects().regions_clusters_delete(...).doit().await
//! let r = hub.projects().regions_clusters_diagnose(...).doit().await
//! let r = hub.projects().regions_clusters_inject_credentials(...).doit().await
//! let r = hub.projects().regions_clusters_patch(...).doit().await
//! let r = hub.projects().regions_clusters_repair(...).doit().await
//! let r = hub.projects().regions_clusters_start(...).doit().await
//! let r = hub.projects().regions_clusters_stop(...).doit().await
//! let r = hub.projects().regions_jobs_submit_as_operation(...).doit().await
//! let r = hub.projects().regions_operations_get(...).doit().await
//! let r = hub.projects().regions_workflow_templates_instantiate(...).doit().await
//! let r = hub.projects().regions_workflow_templates_instantiate_inline(...).doit().await
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
//! google-dataproc1 = "*"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate google_dataproc1 as dataproc1;
//! use dataproc1::api::Cluster;
//! use dataproc1::{Result, Error};
//! # async fn dox() {
//! use std::default::Default;
//! use dataproc1::{Dataproc, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
//! let mut hub = Dataproc::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = Cluster::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.projects().regions_clusters_patch(req, "projectId", "region", "clusterName")
//!              .update_mask(FieldMask::new::<&str>(&[]))
//!              .request_id("amet.")
//!              .graceful_decommission_timeout(chrono::Duration::seconds(5583278))
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
pub use api::Dataproc;
pub use client::{Result, Error, Delegate, FieldMask};

// Re-export the yup_oauth2 crate, that is required to call some methods of the hub and the client
#[cfg(feature = "yup-oauth2")]
pub use client::oauth2;