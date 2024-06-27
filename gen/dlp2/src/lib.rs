// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *DLP* crate version *5.0.5+20240616*, where *20240616* is the exact revision of the *dlp:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.
//! 
//! Everything else about the *DLP* *v2* API can be found at the
//! [official documentation site](https://cloud.google.com/sensitive-data-protection/docs/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/dlp2).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](DLP) ... 
//! 
//! * info types
//!  * [*list*](api::InfoTypeListCall)
//! * locations
//!  * [*info types list*](api::LocationInfoTypeListCall)
//! * organizations
//!  * [*deidentify templates create*](api::OrganizationDeidentifyTemplateCreateCall), [*deidentify templates delete*](api::OrganizationDeidentifyTemplateDeleteCall), [*deidentify templates get*](api::OrganizationDeidentifyTemplateGetCall), [*deidentify templates list*](api::OrganizationDeidentifyTemplateListCall), [*deidentify templates patch*](api::OrganizationDeidentifyTemplatePatchCall), [*inspect templates create*](api::OrganizationInspectTemplateCreateCall), [*inspect templates delete*](api::OrganizationInspectTemplateDeleteCall), [*inspect templates get*](api::OrganizationInspectTemplateGetCall), [*inspect templates list*](api::OrganizationInspectTemplateListCall), [*inspect templates patch*](api::OrganizationInspectTemplatePatchCall), [*locations column data profiles get*](api::OrganizationLocationColumnDataProfileGetCall), [*locations column data profiles list*](api::OrganizationLocationColumnDataProfileListCall), [*locations connections create*](api::OrganizationLocationConnectionCreateCall), [*locations connections delete*](api::OrganizationLocationConnectionDeleteCall), [*locations connections get*](api::OrganizationLocationConnectionGetCall), [*locations connections patch*](api::OrganizationLocationConnectionPatchCall), [*locations connections search*](api::OrganizationLocationConnectionSearchCall), [*locations deidentify templates create*](api::OrganizationLocationDeidentifyTemplateCreateCall), [*locations deidentify templates delete*](api::OrganizationLocationDeidentifyTemplateDeleteCall), [*locations deidentify templates get*](api::OrganizationLocationDeidentifyTemplateGetCall), [*locations deidentify templates list*](api::OrganizationLocationDeidentifyTemplateListCall), [*locations deidentify templates patch*](api::OrganizationLocationDeidentifyTemplatePatchCall), [*locations discovery configs create*](api::OrganizationLocationDiscoveryConfigCreateCall), [*locations discovery configs delete*](api::OrganizationLocationDiscoveryConfigDeleteCall), [*locations discovery configs get*](api::OrganizationLocationDiscoveryConfigGetCall), [*locations discovery configs list*](api::OrganizationLocationDiscoveryConfigListCall), [*locations discovery configs patch*](api::OrganizationLocationDiscoveryConfigPatchCall), [*locations dlp jobs list*](api::OrganizationLocationDlpJobListCall), [*locations inspect templates create*](api::OrganizationLocationInspectTemplateCreateCall), [*locations inspect templates delete*](api::OrganizationLocationInspectTemplateDeleteCall), [*locations inspect templates get*](api::OrganizationLocationInspectTemplateGetCall), [*locations inspect templates list*](api::OrganizationLocationInspectTemplateListCall), [*locations inspect templates patch*](api::OrganizationLocationInspectTemplatePatchCall), [*locations job triggers create*](api::OrganizationLocationJobTriggerCreateCall), [*locations job triggers delete*](api::OrganizationLocationJobTriggerDeleteCall), [*locations job triggers get*](api::OrganizationLocationJobTriggerGetCall), [*locations job triggers list*](api::OrganizationLocationJobTriggerListCall), [*locations job triggers patch*](api::OrganizationLocationJobTriggerPatchCall), [*locations project data profiles get*](api::OrganizationLocationProjectDataProfileGetCall), [*locations project data profiles list*](api::OrganizationLocationProjectDataProfileListCall), [*locations stored info types create*](api::OrganizationLocationStoredInfoTypeCreateCall), [*locations stored info types delete*](api::OrganizationLocationStoredInfoTypeDeleteCall), [*locations stored info types get*](api::OrganizationLocationStoredInfoTypeGetCall), [*locations stored info types list*](api::OrganizationLocationStoredInfoTypeListCall), [*locations stored info types patch*](api::OrganizationLocationStoredInfoTypePatchCall), [*locations table data profiles delete*](api::OrganizationLocationTableDataProfileDeleteCall), [*locations table data profiles get*](api::OrganizationLocationTableDataProfileGetCall), [*locations table data profiles list*](api::OrganizationLocationTableDataProfileListCall), [*stored info types create*](api::OrganizationStoredInfoTypeCreateCall), [*stored info types delete*](api::OrganizationStoredInfoTypeDeleteCall), [*stored info types get*](api::OrganizationStoredInfoTypeGetCall), [*stored info types list*](api::OrganizationStoredInfoTypeListCall) and [*stored info types patch*](api::OrganizationStoredInfoTypePatchCall)
//! * projects
//!  * [*content deidentify*](api::ProjectContentDeidentifyCall), [*content inspect*](api::ProjectContentInspectCall), [*content reidentify*](api::ProjectContentReidentifyCall), [*deidentify templates create*](api::ProjectDeidentifyTemplateCreateCall), [*deidentify templates delete*](api::ProjectDeidentifyTemplateDeleteCall), [*deidentify templates get*](api::ProjectDeidentifyTemplateGetCall), [*deidentify templates list*](api::ProjectDeidentifyTemplateListCall), [*deidentify templates patch*](api::ProjectDeidentifyTemplatePatchCall), [*dlp jobs cancel*](api::ProjectDlpJobCancelCall), [*dlp jobs create*](api::ProjectDlpJobCreateCall), [*dlp jobs delete*](api::ProjectDlpJobDeleteCall), [*dlp jobs get*](api::ProjectDlpJobGetCall), [*dlp jobs list*](api::ProjectDlpJobListCall), [*image redact*](api::ProjectImageRedactCall), [*inspect templates create*](api::ProjectInspectTemplateCreateCall), [*inspect templates delete*](api::ProjectInspectTemplateDeleteCall), [*inspect templates get*](api::ProjectInspectTemplateGetCall), [*inspect templates list*](api::ProjectInspectTemplateListCall), [*inspect templates patch*](api::ProjectInspectTemplatePatchCall), [*job triggers activate*](api::ProjectJobTriggerActivateCall), [*job triggers create*](api::ProjectJobTriggerCreateCall), [*job triggers delete*](api::ProjectJobTriggerDeleteCall), [*job triggers get*](api::ProjectJobTriggerGetCall), [*job triggers list*](api::ProjectJobTriggerListCall), [*job triggers patch*](api::ProjectJobTriggerPatchCall), [*locations column data profiles get*](api::ProjectLocationColumnDataProfileGetCall), [*locations column data profiles list*](api::ProjectLocationColumnDataProfileListCall), [*locations connections create*](api::ProjectLocationConnectionCreateCall), [*locations connections delete*](api::ProjectLocationConnectionDeleteCall), [*locations connections get*](api::ProjectLocationConnectionGetCall), [*locations connections list*](api::ProjectLocationConnectionListCall), [*locations connections patch*](api::ProjectLocationConnectionPatchCall), [*locations connections search*](api::ProjectLocationConnectionSearchCall), [*locations content deidentify*](api::ProjectLocationContentDeidentifyCall), [*locations content inspect*](api::ProjectLocationContentInspectCall), [*locations content reidentify*](api::ProjectLocationContentReidentifyCall), [*locations deidentify templates create*](api::ProjectLocationDeidentifyTemplateCreateCall), [*locations deidentify templates delete*](api::ProjectLocationDeidentifyTemplateDeleteCall), [*locations deidentify templates get*](api::ProjectLocationDeidentifyTemplateGetCall), [*locations deidentify templates list*](api::ProjectLocationDeidentifyTemplateListCall), [*locations deidentify templates patch*](api::ProjectLocationDeidentifyTemplatePatchCall), [*locations discovery configs create*](api::ProjectLocationDiscoveryConfigCreateCall), [*locations discovery configs delete*](api::ProjectLocationDiscoveryConfigDeleteCall), [*locations discovery configs get*](api::ProjectLocationDiscoveryConfigGetCall), [*locations discovery configs list*](api::ProjectLocationDiscoveryConfigListCall), [*locations discovery configs patch*](api::ProjectLocationDiscoveryConfigPatchCall), [*locations dlp jobs cancel*](api::ProjectLocationDlpJobCancelCall), [*locations dlp jobs create*](api::ProjectLocationDlpJobCreateCall), [*locations dlp jobs delete*](api::ProjectLocationDlpJobDeleteCall), [*locations dlp jobs finish*](api::ProjectLocationDlpJobFinishCall), [*locations dlp jobs get*](api::ProjectLocationDlpJobGetCall), [*locations dlp jobs hybrid inspect*](api::ProjectLocationDlpJobHybridInspectCall), [*locations dlp jobs list*](api::ProjectLocationDlpJobListCall), [*locations image redact*](api::ProjectLocationImageRedactCall), [*locations inspect templates create*](api::ProjectLocationInspectTemplateCreateCall), [*locations inspect templates delete*](api::ProjectLocationInspectTemplateDeleteCall), [*locations inspect templates get*](api::ProjectLocationInspectTemplateGetCall), [*locations inspect templates list*](api::ProjectLocationInspectTemplateListCall), [*locations inspect templates patch*](api::ProjectLocationInspectTemplatePatchCall), [*locations job triggers activate*](api::ProjectLocationJobTriggerActivateCall), [*locations job triggers create*](api::ProjectLocationJobTriggerCreateCall), [*locations job triggers delete*](api::ProjectLocationJobTriggerDeleteCall), [*locations job triggers get*](api::ProjectLocationJobTriggerGetCall), [*locations job triggers hybrid inspect*](api::ProjectLocationJobTriggerHybridInspectCall), [*locations job triggers list*](api::ProjectLocationJobTriggerListCall), [*locations job triggers patch*](api::ProjectLocationJobTriggerPatchCall), [*locations project data profiles get*](api::ProjectLocationProjectDataProfileGetCall), [*locations project data profiles list*](api::ProjectLocationProjectDataProfileListCall), [*locations stored info types create*](api::ProjectLocationStoredInfoTypeCreateCall), [*locations stored info types delete*](api::ProjectLocationStoredInfoTypeDeleteCall), [*locations stored info types get*](api::ProjectLocationStoredInfoTypeGetCall), [*locations stored info types list*](api::ProjectLocationStoredInfoTypeListCall), [*locations stored info types patch*](api::ProjectLocationStoredInfoTypePatchCall), [*locations table data profiles delete*](api::ProjectLocationTableDataProfileDeleteCall), [*locations table data profiles get*](api::ProjectLocationTableDataProfileGetCall), [*locations table data profiles list*](api::ProjectLocationTableDataProfileListCall), [*stored info types create*](api::ProjectStoredInfoTypeCreateCall), [*stored info types delete*](api::ProjectStoredInfoTypeDeleteCall), [*stored info types get*](api::ProjectStoredInfoTypeGetCall), [*stored info types list*](api::ProjectStoredInfoTypeListCall) and [*stored info types patch*](api::ProjectStoredInfoTypePatchCall)
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
//! * **[Hub](DLP)**
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
//! let r = hub.organizations().deidentify_templates_delete(...).doit().await
//! let r = hub.organizations().inspect_templates_delete(...).doit().await
//! let r = hub.organizations().locations_connections_delete(...).doit().await
//! let r = hub.organizations().locations_deidentify_templates_delete(...).doit().await
//! let r = hub.organizations().locations_discovery_configs_delete(...).doit().await
//! let r = hub.organizations().locations_inspect_templates_delete(...).doit().await
//! let r = hub.organizations().locations_job_triggers_delete(...).doit().await
//! let r = hub.organizations().locations_stored_info_types_delete(...).doit().await
//! let r = hub.organizations().locations_table_data_profiles_delete(...).doit().await
//! let r = hub.organizations().stored_info_types_delete(...).doit().await
//! let r = hub.projects().deidentify_templates_delete(...).doit().await
//! let r = hub.projects().dlp_jobs_cancel(...).doit().await
//! let r = hub.projects().dlp_jobs_delete(...).doit().await
//! let r = hub.projects().inspect_templates_delete(...).doit().await
//! let r = hub.projects().job_triggers_delete(...).doit().await
//! let r = hub.projects().locations_connections_delete(...).doit().await
//! let r = hub.projects().locations_deidentify_templates_delete(...).doit().await
//! let r = hub.projects().locations_discovery_configs_delete(...).doit().await
//! let r = hub.projects().locations_dlp_jobs_cancel(...).doit().await
//! let r = hub.projects().locations_dlp_jobs_delete(...).doit().await
//! let r = hub.projects().locations_dlp_jobs_finish(...).doit().await
//! let r = hub.projects().locations_inspect_templates_delete(...).doit().await
//! let r = hub.projects().locations_job_triggers_delete(...).doit().await
//! let r = hub.projects().locations_stored_info_types_delete(...).doit().await
//! let r = hub.projects().locations_table_data_profiles_delete(...).doit().await
//! let r = hub.projects().stored_info_types_delete(...).doit().await
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
//! google-dlp2 = "*"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate google_dlp2 as dlp2;
//! use dlp2::api::GooglePrivacyDlpV2CancelDlpJobRequest;
//! use dlp2::{Result, Error};
//! # async fn dox() {
//! use std::default::Default;
//! use dlp2::{DLP, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
//! let mut hub = DLP::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = GooglePrivacyDlpV2CancelDlpJobRequest::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.projects().dlp_jobs_cancel(req, "name")
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
pub use api::DLP;
pub use client::{Result, Error, Delegate, FieldMask};

// Re-export the yup_oauth2 crate, that is required to call some methods of the hub and the client
#[cfg(feature = "yup-oauth2")]
pub use client::oauth2;