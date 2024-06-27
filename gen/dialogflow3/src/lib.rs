// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Dialogflow* crate version *5.0.5+20240614*, where *20240614* is the exact revision of the *dialogflow:v3* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.
//! 
//! Everything else about the *Dialogflow* *v3* API can be found at the
//! [official documentation site](https://cloud.google.com/dialogflow/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/dialogflow3).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](Dialogflow) ... 
//! 
//! * projects
//!  * [*locations agents changelogs get*](api::ProjectLocationAgentChangelogGetCall), [*locations agents changelogs list*](api::ProjectLocationAgentChangelogListCall), [*locations agents create*](api::ProjectLocationAgentCreateCall), [*locations agents delete*](api::ProjectLocationAgentDeleteCall), [*locations agents entity types create*](api::ProjectLocationAgentEntityTypeCreateCall), [*locations agents entity types delete*](api::ProjectLocationAgentEntityTypeDeleteCall), [*locations agents entity types export*](api::ProjectLocationAgentEntityTypeExportCall), [*locations agents entity types get*](api::ProjectLocationAgentEntityTypeGetCall), [*locations agents entity types import*](api::ProjectLocationAgentEntityTypeImportCall), [*locations agents entity types list*](api::ProjectLocationAgentEntityTypeListCall), [*locations agents entity types patch*](api::ProjectLocationAgentEntityTypePatchCall), [*locations agents environments continuous test results list*](api::ProjectLocationAgentEnvironmentContinuousTestResultListCall), [*locations agents environments create*](api::ProjectLocationAgentEnvironmentCreateCall), [*locations agents environments delete*](api::ProjectLocationAgentEnvironmentDeleteCall), [*locations agents environments deploy flow*](api::ProjectLocationAgentEnvironmentDeployFlowCall), [*locations agents environments deployments get*](api::ProjectLocationAgentEnvironmentDeploymentGetCall), [*locations agents environments deployments list*](api::ProjectLocationAgentEnvironmentDeploymentListCall), [*locations agents environments experiments create*](api::ProjectLocationAgentEnvironmentExperimentCreateCall), [*locations agents environments experiments delete*](api::ProjectLocationAgentEnvironmentExperimentDeleteCall), [*locations agents environments experiments get*](api::ProjectLocationAgentEnvironmentExperimentGetCall), [*locations agents environments experiments list*](api::ProjectLocationAgentEnvironmentExperimentListCall), [*locations agents environments experiments patch*](api::ProjectLocationAgentEnvironmentExperimentPatchCall), [*locations agents environments experiments start*](api::ProjectLocationAgentEnvironmentExperimentStartCall), [*locations agents environments experiments stop*](api::ProjectLocationAgentEnvironmentExperimentStopCall), [*locations agents environments get*](api::ProjectLocationAgentEnvironmentGetCall), [*locations agents environments list*](api::ProjectLocationAgentEnvironmentListCall), [*locations agents environments lookup environment history*](api::ProjectLocationAgentEnvironmentLookupEnvironmentHistoryCall), [*locations agents environments patch*](api::ProjectLocationAgentEnvironmentPatchCall), [*locations agents environments run continuous test*](api::ProjectLocationAgentEnvironmentRunContinuousTestCall), [*locations agents environments sessions detect intent*](api::ProjectLocationAgentEnvironmentSessionDetectIntentCall), [*locations agents environments sessions entity types create*](api::ProjectLocationAgentEnvironmentSessionEntityTypeCreateCall), [*locations agents environments sessions entity types delete*](api::ProjectLocationAgentEnvironmentSessionEntityTypeDeleteCall), [*locations agents environments sessions entity types get*](api::ProjectLocationAgentEnvironmentSessionEntityTypeGetCall), [*locations agents environments sessions entity types list*](api::ProjectLocationAgentEnvironmentSessionEntityTypeListCall), [*locations agents environments sessions entity types patch*](api::ProjectLocationAgentEnvironmentSessionEntityTypePatchCall), [*locations agents environments sessions fulfill intent*](api::ProjectLocationAgentEnvironmentSessionFulfillIntentCall), [*locations agents environments sessions match intent*](api::ProjectLocationAgentEnvironmentSessionMatchIntentCall), [*locations agents environments sessions server streaming detect intent*](api::ProjectLocationAgentEnvironmentSessionServerStreamingDetectIntentCall), [*locations agents export*](api::ProjectLocationAgentExportCall), [*locations agents flows create*](api::ProjectLocationAgentFlowCreateCall), [*locations agents flows delete*](api::ProjectLocationAgentFlowDeleteCall), [*locations agents flows export*](api::ProjectLocationAgentFlowExportCall), [*locations agents flows get*](api::ProjectLocationAgentFlowGetCall), [*locations agents flows get validation result*](api::ProjectLocationAgentFlowGetValidationResultCall), [*locations agents flows import*](api::ProjectLocationAgentFlowImportCall), [*locations agents flows list*](api::ProjectLocationAgentFlowListCall), [*locations agents flows pages create*](api::ProjectLocationAgentFlowPageCreateCall), [*locations agents flows pages delete*](api::ProjectLocationAgentFlowPageDeleteCall), [*locations agents flows pages get*](api::ProjectLocationAgentFlowPageGetCall), [*locations agents flows pages list*](api::ProjectLocationAgentFlowPageListCall), [*locations agents flows pages patch*](api::ProjectLocationAgentFlowPagePatchCall), [*locations agents flows patch*](api::ProjectLocationAgentFlowPatchCall), [*locations agents flows train*](api::ProjectLocationAgentFlowTrainCall), [*locations agents flows transition route groups create*](api::ProjectLocationAgentFlowTransitionRouteGroupCreateCall), [*locations agents flows transition route groups delete*](api::ProjectLocationAgentFlowTransitionRouteGroupDeleteCall), [*locations agents flows transition route groups get*](api::ProjectLocationAgentFlowTransitionRouteGroupGetCall), [*locations agents flows transition route groups list*](api::ProjectLocationAgentFlowTransitionRouteGroupListCall), [*locations agents flows transition route groups patch*](api::ProjectLocationAgentFlowTransitionRouteGroupPatchCall), [*locations agents flows validate*](api::ProjectLocationAgentFlowValidateCall), [*locations agents flows versions compare versions*](api::ProjectLocationAgentFlowVersionCompareVersionCall), [*locations agents flows versions create*](api::ProjectLocationAgentFlowVersionCreateCall), [*locations agents flows versions delete*](api::ProjectLocationAgentFlowVersionDeleteCall), [*locations agents flows versions get*](api::ProjectLocationAgentFlowVersionGetCall), [*locations agents flows versions list*](api::ProjectLocationAgentFlowVersionListCall), [*locations agents flows versions load*](api::ProjectLocationAgentFlowVersionLoadCall), [*locations agents flows versions patch*](api::ProjectLocationAgentFlowVersionPatchCall), [*locations agents generators create*](api::ProjectLocationAgentGeneratorCreateCall), [*locations agents generators delete*](api::ProjectLocationAgentGeneratorDeleteCall), [*locations agents generators get*](api::ProjectLocationAgentGeneratorGetCall), [*locations agents generators list*](api::ProjectLocationAgentGeneratorListCall), [*locations agents generators patch*](api::ProjectLocationAgentGeneratorPatchCall), [*locations agents get*](api::ProjectLocationAgentGetCall), [*locations agents get generative settings*](api::ProjectLocationAgentGetGenerativeSettingCall), [*locations agents get validation result*](api::ProjectLocationAgentGetValidationResultCall), [*locations agents intents create*](api::ProjectLocationAgentIntentCreateCall), [*locations agents intents delete*](api::ProjectLocationAgentIntentDeleteCall), [*locations agents intents export*](api::ProjectLocationAgentIntentExportCall), [*locations agents intents get*](api::ProjectLocationAgentIntentGetCall), [*locations agents intents import*](api::ProjectLocationAgentIntentImportCall), [*locations agents intents list*](api::ProjectLocationAgentIntentListCall), [*locations agents intents patch*](api::ProjectLocationAgentIntentPatchCall), [*locations agents list*](api::ProjectLocationAgentListCall), [*locations agents patch*](api::ProjectLocationAgentPatchCall), [*locations agents restore*](api::ProjectLocationAgentRestoreCall), [*locations agents sessions detect intent*](api::ProjectLocationAgentSessionDetectIntentCall), [*locations agents sessions entity types create*](api::ProjectLocationAgentSessionEntityTypeCreateCall), [*locations agents sessions entity types delete*](api::ProjectLocationAgentSessionEntityTypeDeleteCall), [*locations agents sessions entity types get*](api::ProjectLocationAgentSessionEntityTypeGetCall), [*locations agents sessions entity types list*](api::ProjectLocationAgentSessionEntityTypeListCall), [*locations agents sessions entity types patch*](api::ProjectLocationAgentSessionEntityTypePatchCall), [*locations agents sessions fulfill intent*](api::ProjectLocationAgentSessionFulfillIntentCall), [*locations agents sessions match intent*](api::ProjectLocationAgentSessionMatchIntentCall), [*locations agents sessions server streaming detect intent*](api::ProjectLocationAgentSessionServerStreamingDetectIntentCall), [*locations agents sessions submit answer feedback*](api::ProjectLocationAgentSessionSubmitAnswerFeedbackCall), [*locations agents test cases batch delete*](api::ProjectLocationAgentTestCaseBatchDeleteCall), [*locations agents test cases batch run*](api::ProjectLocationAgentTestCaseBatchRunCall), [*locations agents test cases calculate coverage*](api::ProjectLocationAgentTestCaseCalculateCoverageCall), [*locations agents test cases create*](api::ProjectLocationAgentTestCaseCreateCall), [*locations agents test cases export*](api::ProjectLocationAgentTestCaseExportCall), [*locations agents test cases get*](api::ProjectLocationAgentTestCaseGetCall), [*locations agents test cases import*](api::ProjectLocationAgentTestCaseImportCall), [*locations agents test cases list*](api::ProjectLocationAgentTestCaseListCall), [*locations agents test cases patch*](api::ProjectLocationAgentTestCasePatchCall), [*locations agents test cases results get*](api::ProjectLocationAgentTestCaseResultGetCall), [*locations agents test cases results list*](api::ProjectLocationAgentTestCaseResultListCall), [*locations agents test cases run*](api::ProjectLocationAgentTestCaseRunCall), [*locations agents transition route groups create*](api::ProjectLocationAgentTransitionRouteGroupCreateCall), [*locations agents transition route groups delete*](api::ProjectLocationAgentTransitionRouteGroupDeleteCall), [*locations agents transition route groups get*](api::ProjectLocationAgentTransitionRouteGroupGetCall), [*locations agents transition route groups list*](api::ProjectLocationAgentTransitionRouteGroupListCall), [*locations agents transition route groups patch*](api::ProjectLocationAgentTransitionRouteGroupPatchCall), [*locations agents update generative settings*](api::ProjectLocationAgentUpdateGenerativeSettingCall), [*locations agents validate*](api::ProjectLocationAgentValidateCall), [*locations agents webhooks create*](api::ProjectLocationAgentWebhookCreateCall), [*locations agents webhooks delete*](api::ProjectLocationAgentWebhookDeleteCall), [*locations agents webhooks get*](api::ProjectLocationAgentWebhookGetCall), [*locations agents webhooks list*](api::ProjectLocationAgentWebhookListCall), [*locations agents webhooks patch*](api::ProjectLocationAgentWebhookPatchCall), [*locations get*](api::ProjectLocationGetCall), [*locations list*](api::ProjectLocationListCall), [*locations operations cancel*](api::ProjectLocationOperationCancelCall), [*locations operations get*](api::ProjectLocationOperationGetCall), [*locations operations list*](api::ProjectLocationOperationListCall), [*locations security settings create*](api::ProjectLocationSecuritySettingCreateCall), [*locations security settings delete*](api::ProjectLocationSecuritySettingDeleteCall), [*locations security settings get*](api::ProjectLocationSecuritySettingGetCall), [*locations security settings list*](api::ProjectLocationSecuritySettingListCall), [*locations security settings patch*](api::ProjectLocationSecuritySettingPatchCall), [*operations cancel*](api::ProjectOperationCancelCall), [*operations get*](api::ProjectOperationGetCall) and [*operations list*](api::ProjectOperationListCall)
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
//! * **[Hub](Dialogflow)**
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
//! let r = hub.projects().locations_agents_entity_types_export(...).doit().await
//! let r = hub.projects().locations_agents_entity_types_import(...).doit().await
//! let r = hub.projects().locations_agents_environments_create(...).doit().await
//! let r = hub.projects().locations_agents_environments_deploy_flow(...).doit().await
//! let r = hub.projects().locations_agents_environments_patch(...).doit().await
//! let r = hub.projects().locations_agents_environments_run_continuous_test(...).doit().await
//! let r = hub.projects().locations_agents_flows_versions_create(...).doit().await
//! let r = hub.projects().locations_agents_flows_versions_load(...).doit().await
//! let r = hub.projects().locations_agents_flows_export(...).doit().await
//! let r = hub.projects().locations_agents_flows_import(...).doit().await
//! let r = hub.projects().locations_agents_flows_train(...).doit().await
//! let r = hub.projects().locations_agents_intents_export(...).doit().await
//! let r = hub.projects().locations_agents_intents_import(...).doit().await
//! let r = hub.projects().locations_agents_test_cases_batch_run(...).doit().await
//! let r = hub.projects().locations_agents_test_cases_export(...).doit().await
//! let r = hub.projects().locations_agents_test_cases_import(...).doit().await
//! let r = hub.projects().locations_agents_test_cases_run(...).doit().await
//! let r = hub.projects().locations_agents_export(...).doit().await
//! let r = hub.projects().locations_agents_restore(...).doit().await
//! let r = hub.projects().locations_operations_get(...).doit().await
//! let r = hub.projects().operations_get(...).doit().await
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
//! google-dialogflow3 = "*"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate google_dialogflow3 as dialogflow3;
//! use dialogflow3::api::GoogleCloudDialogflowCxV3Environment;
//! use dialogflow3::{Result, Error};
//! # async fn dox() {
//! use std::default::Default;
//! use dialogflow3::{Dialogflow, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
//! let mut hub = Dialogflow::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = GoogleCloudDialogflowCxV3Environment::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.projects().locations_agents_environments_patch(req, "name")
//!              .update_mask(FieldMask::new::<&str>(&[]))
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
pub use api::Dialogflow;
pub use client::{Result, Error, Delegate, FieldMask};

// Re-export the yup_oauth2 crate, that is required to call some methods of the hub and the client
#[cfg(feature = "yup-oauth2")]
pub use client::oauth2;