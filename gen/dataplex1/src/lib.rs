// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Cloud Dataplex* crate version *5.0.5+20240611*, where *20240611* is the exact revision of the *dataplex:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.
//! 
//! Everything else about the *Cloud Dataplex* *v1* API can be found at the
//! [official documentation site](https://cloud.google.com/dataplex/docs).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/dataplex1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](CloudDataplex) ... 
//! 
//! * projects
//!  * [*locations aspect types create*](api::ProjectLocationAspectTypeCreateCall), [*locations aspect types delete*](api::ProjectLocationAspectTypeDeleteCall), [*locations aspect types get*](api::ProjectLocationAspectTypeGetCall), [*locations aspect types get iam policy*](api::ProjectLocationAspectTypeGetIamPolicyCall), [*locations aspect types list*](api::ProjectLocationAspectTypeListCall), [*locations aspect types patch*](api::ProjectLocationAspectTypePatchCall), [*locations aspect types set iam policy*](api::ProjectLocationAspectTypeSetIamPolicyCall), [*locations aspect types test iam permissions*](api::ProjectLocationAspectTypeTestIamPermissionCall), [*locations data attribute bindings create*](api::ProjectLocationDataAttributeBindingCreateCall), [*locations data attribute bindings delete*](api::ProjectLocationDataAttributeBindingDeleteCall), [*locations data attribute bindings get*](api::ProjectLocationDataAttributeBindingGetCall), [*locations data attribute bindings get iam policy*](api::ProjectLocationDataAttributeBindingGetIamPolicyCall), [*locations data attribute bindings list*](api::ProjectLocationDataAttributeBindingListCall), [*locations data attribute bindings patch*](api::ProjectLocationDataAttributeBindingPatchCall), [*locations data attribute bindings set iam policy*](api::ProjectLocationDataAttributeBindingSetIamPolicyCall), [*locations data attribute bindings test iam permissions*](api::ProjectLocationDataAttributeBindingTestIamPermissionCall), [*locations data scans create*](api::ProjectLocationDataScanCreateCall), [*locations data scans delete*](api::ProjectLocationDataScanDeleteCall), [*locations data scans generate data quality rules*](api::ProjectLocationDataScanGenerateDataQualityRuleCall), [*locations data scans get*](api::ProjectLocationDataScanGetCall), [*locations data scans get iam policy*](api::ProjectLocationDataScanGetIamPolicyCall), [*locations data scans jobs generate data quality rules*](api::ProjectLocationDataScanJobGenerateDataQualityRuleCall), [*locations data scans jobs get*](api::ProjectLocationDataScanJobGetCall), [*locations data scans jobs list*](api::ProjectLocationDataScanJobListCall), [*locations data scans list*](api::ProjectLocationDataScanListCall), [*locations data scans patch*](api::ProjectLocationDataScanPatchCall), [*locations data scans run*](api::ProjectLocationDataScanRunCall), [*locations data scans set iam policy*](api::ProjectLocationDataScanSetIamPolicyCall), [*locations data scans test iam permissions*](api::ProjectLocationDataScanTestIamPermissionCall), [*locations data taxonomies attributes create*](api::ProjectLocationDataTaxonomyAttributeCreateCall), [*locations data taxonomies attributes delete*](api::ProjectLocationDataTaxonomyAttributeDeleteCall), [*locations data taxonomies attributes get*](api::ProjectLocationDataTaxonomyAttributeGetCall), [*locations data taxonomies attributes get iam policy*](api::ProjectLocationDataTaxonomyAttributeGetIamPolicyCall), [*locations data taxonomies attributes list*](api::ProjectLocationDataTaxonomyAttributeListCall), [*locations data taxonomies attributes patch*](api::ProjectLocationDataTaxonomyAttributePatchCall), [*locations data taxonomies attributes set iam policy*](api::ProjectLocationDataTaxonomyAttributeSetIamPolicyCall), [*locations data taxonomies attributes test iam permissions*](api::ProjectLocationDataTaxonomyAttributeTestIamPermissionCall), [*locations data taxonomies create*](api::ProjectLocationDataTaxonomyCreateCall), [*locations data taxonomies delete*](api::ProjectLocationDataTaxonomyDeleteCall), [*locations data taxonomies get*](api::ProjectLocationDataTaxonomyGetCall), [*locations data taxonomies get iam policy*](api::ProjectLocationDataTaxonomyGetIamPolicyCall), [*locations data taxonomies list*](api::ProjectLocationDataTaxonomyListCall), [*locations data taxonomies patch*](api::ProjectLocationDataTaxonomyPatchCall), [*locations data taxonomies set iam policy*](api::ProjectLocationDataTaxonomySetIamPolicyCall), [*locations data taxonomies test iam permissions*](api::ProjectLocationDataTaxonomyTestIamPermissionCall), [*locations entry groups create*](api::ProjectLocationEntryGroupCreateCall), [*locations entry groups delete*](api::ProjectLocationEntryGroupDeleteCall), [*locations entry groups entries create*](api::ProjectLocationEntryGroupEntryCreateCall), [*locations entry groups entries delete*](api::ProjectLocationEntryGroupEntryDeleteCall), [*locations entry groups entries get*](api::ProjectLocationEntryGroupEntryGetCall), [*locations entry groups entries list*](api::ProjectLocationEntryGroupEntryListCall), [*locations entry groups entries patch*](api::ProjectLocationEntryGroupEntryPatchCall), [*locations entry groups get*](api::ProjectLocationEntryGroupGetCall), [*locations entry groups get iam policy*](api::ProjectLocationEntryGroupGetIamPolicyCall), [*locations entry groups list*](api::ProjectLocationEntryGroupListCall), [*locations entry groups patch*](api::ProjectLocationEntryGroupPatchCall), [*locations entry groups set iam policy*](api::ProjectLocationEntryGroupSetIamPolicyCall), [*locations entry groups test iam permissions*](api::ProjectLocationEntryGroupTestIamPermissionCall), [*locations entry types create*](api::ProjectLocationEntryTypeCreateCall), [*locations entry types delete*](api::ProjectLocationEntryTypeDeleteCall), [*locations entry types get*](api::ProjectLocationEntryTypeGetCall), [*locations entry types get iam policy*](api::ProjectLocationEntryTypeGetIamPolicyCall), [*locations entry types list*](api::ProjectLocationEntryTypeListCall), [*locations entry types patch*](api::ProjectLocationEntryTypePatchCall), [*locations entry types set iam policy*](api::ProjectLocationEntryTypeSetIamPolicyCall), [*locations entry types test iam permissions*](api::ProjectLocationEntryTypeTestIamPermissionCall), [*locations get*](api::ProjectLocationGetCall), [*locations governance rules get iam policy*](api::ProjectLocationGovernanceRuleGetIamPolicyCall), [*locations governance rules set iam policy*](api::ProjectLocationGovernanceRuleSetIamPolicyCall), [*locations governance rules test iam permissions*](api::ProjectLocationGovernanceRuleTestIamPermissionCall), [*locations lakes actions list*](api::ProjectLocationLakeActionListCall), [*locations lakes content create*](api::ProjectLocationLakeContentCreateCall), [*locations lakes content delete*](api::ProjectLocationLakeContentDeleteCall), [*locations lakes content get*](api::ProjectLocationLakeContentGetCall), [*locations lakes content get iam policy*](api::ProjectLocationLakeContentGetIamPolicyCall), [*locations lakes content list*](api::ProjectLocationLakeContentListCall), [*locations lakes content patch*](api::ProjectLocationLakeContentPatchCall), [*locations lakes content set iam policy*](api::ProjectLocationLakeContentSetIamPolicyCall), [*locations lakes content test iam permissions*](api::ProjectLocationLakeContentTestIamPermissionCall), [*locations lakes contentitems create*](api::ProjectLocationLakeContentitemCreateCall), [*locations lakes contentitems delete*](api::ProjectLocationLakeContentitemDeleteCall), [*locations lakes contentitems get*](api::ProjectLocationLakeContentitemGetCall), [*locations lakes contentitems get iam policy*](api::ProjectLocationLakeContentitemGetIamPolicyCall), [*locations lakes contentitems list*](api::ProjectLocationLakeContentitemListCall), [*locations lakes contentitems patch*](api::ProjectLocationLakeContentitemPatchCall), [*locations lakes contentitems set iam policy*](api::ProjectLocationLakeContentitemSetIamPolicyCall), [*locations lakes contentitems test iam permissions*](api::ProjectLocationLakeContentitemTestIamPermissionCall), [*locations lakes create*](api::ProjectLocationLakeCreateCall), [*locations lakes delete*](api::ProjectLocationLakeDeleteCall), [*locations lakes environments create*](api::ProjectLocationLakeEnvironmentCreateCall), [*locations lakes environments delete*](api::ProjectLocationLakeEnvironmentDeleteCall), [*locations lakes environments get*](api::ProjectLocationLakeEnvironmentGetCall), [*locations lakes environments get iam policy*](api::ProjectLocationLakeEnvironmentGetIamPolicyCall), [*locations lakes environments list*](api::ProjectLocationLakeEnvironmentListCall), [*locations lakes environments patch*](api::ProjectLocationLakeEnvironmentPatchCall), [*locations lakes environments sessions list*](api::ProjectLocationLakeEnvironmentSessionListCall), [*locations lakes environments set iam policy*](api::ProjectLocationLakeEnvironmentSetIamPolicyCall), [*locations lakes environments test iam permissions*](api::ProjectLocationLakeEnvironmentTestIamPermissionCall), [*locations lakes get*](api::ProjectLocationLakeGetCall), [*locations lakes get iam policy*](api::ProjectLocationLakeGetIamPolicyCall), [*locations lakes list*](api::ProjectLocationLakeListCall), [*locations lakes patch*](api::ProjectLocationLakePatchCall), [*locations lakes set iam policy*](api::ProjectLocationLakeSetIamPolicyCall), [*locations lakes tasks create*](api::ProjectLocationLakeTaskCreateCall), [*locations lakes tasks delete*](api::ProjectLocationLakeTaskDeleteCall), [*locations lakes tasks get*](api::ProjectLocationLakeTaskGetCall), [*locations lakes tasks get iam policy*](api::ProjectLocationLakeTaskGetIamPolicyCall), [*locations lakes tasks jobs cancel*](api::ProjectLocationLakeTaskJobCancelCall), [*locations lakes tasks jobs get*](api::ProjectLocationLakeTaskJobGetCall), [*locations lakes tasks jobs list*](api::ProjectLocationLakeTaskJobListCall), [*locations lakes tasks list*](api::ProjectLocationLakeTaskListCall), [*locations lakes tasks patch*](api::ProjectLocationLakeTaskPatchCall), [*locations lakes tasks run*](api::ProjectLocationLakeTaskRunCall), [*locations lakes tasks set iam policy*](api::ProjectLocationLakeTaskSetIamPolicyCall), [*locations lakes tasks test iam permissions*](api::ProjectLocationLakeTaskTestIamPermissionCall), [*locations lakes test iam permissions*](api::ProjectLocationLakeTestIamPermissionCall), [*locations lakes zones actions list*](api::ProjectLocationLakeZoneActionListCall), [*locations lakes zones assets actions list*](api::ProjectLocationLakeZoneAssetActionListCall), [*locations lakes zones assets create*](api::ProjectLocationLakeZoneAssetCreateCall), [*locations lakes zones assets delete*](api::ProjectLocationLakeZoneAssetDeleteCall), [*locations lakes zones assets get*](api::ProjectLocationLakeZoneAssetGetCall), [*locations lakes zones assets get iam policy*](api::ProjectLocationLakeZoneAssetGetIamPolicyCall), [*locations lakes zones assets list*](api::ProjectLocationLakeZoneAssetListCall), [*locations lakes zones assets patch*](api::ProjectLocationLakeZoneAssetPatchCall), [*locations lakes zones assets set iam policy*](api::ProjectLocationLakeZoneAssetSetIamPolicyCall), [*locations lakes zones assets test iam permissions*](api::ProjectLocationLakeZoneAssetTestIamPermissionCall), [*locations lakes zones create*](api::ProjectLocationLakeZoneCreateCall), [*locations lakes zones delete*](api::ProjectLocationLakeZoneDeleteCall), [*locations lakes zones entities create*](api::ProjectLocationLakeZoneEntityCreateCall), [*locations lakes zones entities delete*](api::ProjectLocationLakeZoneEntityDeleteCall), [*locations lakes zones entities get*](api::ProjectLocationLakeZoneEntityGetCall), [*locations lakes zones entities list*](api::ProjectLocationLakeZoneEntityListCall), [*locations lakes zones entities partitions create*](api::ProjectLocationLakeZoneEntityPartitionCreateCall), [*locations lakes zones entities partitions delete*](api::ProjectLocationLakeZoneEntityPartitionDeleteCall), [*locations lakes zones entities partitions get*](api::ProjectLocationLakeZoneEntityPartitionGetCall), [*locations lakes zones entities partitions list*](api::ProjectLocationLakeZoneEntityPartitionListCall), [*locations lakes zones entities update*](api::ProjectLocationLakeZoneEntityUpdateCall), [*locations lakes zones get*](api::ProjectLocationLakeZoneGetCall), [*locations lakes zones get iam policy*](api::ProjectLocationLakeZoneGetIamPolicyCall), [*locations lakes zones list*](api::ProjectLocationLakeZoneListCall), [*locations lakes zones patch*](api::ProjectLocationLakeZonePatchCall), [*locations lakes zones set iam policy*](api::ProjectLocationLakeZoneSetIamPolicyCall), [*locations lakes zones test iam permissions*](api::ProjectLocationLakeZoneTestIamPermissionCall), [*locations list*](api::ProjectLocationListCall), [*locations lookup entry*](api::ProjectLocationLookupEntryCall), [*locations operations cancel*](api::ProjectLocationOperationCancelCall), [*locations operations delete*](api::ProjectLocationOperationDeleteCall), [*locations operations get*](api::ProjectLocationOperationGetCall), [*locations operations list*](api::ProjectLocationOperationListCall) and [*locations search entries*](api::ProjectLocationSearchEntryCall)
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
//! * **[Hub](CloudDataplex)**
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
//! let r = hub.projects().locations_aspect_types_create(...).doit().await
//! let r = hub.projects().locations_aspect_types_delete(...).doit().await
//! let r = hub.projects().locations_aspect_types_patch(...).doit().await
//! let r = hub.projects().locations_data_attribute_bindings_create(...).doit().await
//! let r = hub.projects().locations_data_attribute_bindings_delete(...).doit().await
//! let r = hub.projects().locations_data_attribute_bindings_patch(...).doit().await
//! let r = hub.projects().locations_data_scans_create(...).doit().await
//! let r = hub.projects().locations_data_scans_delete(...).doit().await
//! let r = hub.projects().locations_data_scans_patch(...).doit().await
//! let r = hub.projects().locations_data_taxonomies_attributes_create(...).doit().await
//! let r = hub.projects().locations_data_taxonomies_attributes_delete(...).doit().await
//! let r = hub.projects().locations_data_taxonomies_attributes_patch(...).doit().await
//! let r = hub.projects().locations_data_taxonomies_create(...).doit().await
//! let r = hub.projects().locations_data_taxonomies_delete(...).doit().await
//! let r = hub.projects().locations_data_taxonomies_patch(...).doit().await
//! let r = hub.projects().locations_entry_groups_create(...).doit().await
//! let r = hub.projects().locations_entry_groups_delete(...).doit().await
//! let r = hub.projects().locations_entry_groups_patch(...).doit().await
//! let r = hub.projects().locations_entry_types_create(...).doit().await
//! let r = hub.projects().locations_entry_types_delete(...).doit().await
//! let r = hub.projects().locations_entry_types_patch(...).doit().await
//! let r = hub.projects().locations_lakes_environments_create(...).doit().await
//! let r = hub.projects().locations_lakes_environments_delete(...).doit().await
//! let r = hub.projects().locations_lakes_environments_patch(...).doit().await
//! let r = hub.projects().locations_lakes_tasks_create(...).doit().await
//! let r = hub.projects().locations_lakes_tasks_delete(...).doit().await
//! let r = hub.projects().locations_lakes_tasks_patch(...).doit().await
//! let r = hub.projects().locations_lakes_zones_assets_create(...).doit().await
//! let r = hub.projects().locations_lakes_zones_assets_delete(...).doit().await
//! let r = hub.projects().locations_lakes_zones_assets_patch(...).doit().await
//! let r = hub.projects().locations_lakes_zones_create(...).doit().await
//! let r = hub.projects().locations_lakes_zones_delete(...).doit().await
//! let r = hub.projects().locations_lakes_zones_patch(...).doit().await
//! let r = hub.projects().locations_lakes_create(...).doit().await
//! let r = hub.projects().locations_lakes_delete(...).doit().await
//! let r = hub.projects().locations_lakes_patch(...).doit().await
//! let r = hub.projects().locations_operations_get(...).doit().await
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
//! google-dataplex1 = "*"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate google_dataplex1 as dataplex1;
//! use dataplex1::api::GoogleCloudDataplexV1AspectType;
//! use dataplex1::{Result, Error};
//! # async fn dox() {
//! use std::default::Default;
//! use dataplex1::{CloudDataplex, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
//! let mut hub = CloudDataplex::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = GoogleCloudDataplexV1AspectType::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.projects().locations_aspect_types_create(req, "parent")
//!              .validate_only(false)
//!              .aspect_type_id("amet.")
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
pub use api::CloudDataplex;
pub use client::{Result, Error, Delegate, FieldMask};

// Re-export the yup_oauth2 crate, that is required to call some methods of the hub and the client
#[cfg(feature = "yup-oauth2")]
pub use client::oauth2;