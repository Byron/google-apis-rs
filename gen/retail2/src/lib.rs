// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Cloud Retail* crate version *5.0.5+20240614*, where *20240614* is the exact revision of the *retail:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.
//! 
//! Everything else about the *Cloud Retail* *v2* API can be found at the
//! [official documentation site](https://cloud.google.com/recommendations).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/retail2).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](CloudRetail) ... 
//! 
//! * projects
//!  * [*locations catalogs attributes config add catalog attribute*](api::ProjectLocationCatalogAttributesConfigAddCatalogAttributeCall), [*locations catalogs attributes config remove catalog attribute*](api::ProjectLocationCatalogAttributesConfigRemoveCatalogAttributeCall), [*locations catalogs attributes config replace catalog attribute*](api::ProjectLocationCatalogAttributesConfigReplaceCatalogAttributeCall), [*locations catalogs branches operations get*](api::ProjectLocationCatalogBranchOperationGetCall), [*locations catalogs branches products add fulfillment places*](api::ProjectLocationCatalogBranchProductAddFulfillmentPlaceCall), [*locations catalogs branches products add local inventories*](api::ProjectLocationCatalogBranchProductAddLocalInventoryCall), [*locations catalogs branches products create*](api::ProjectLocationCatalogBranchProductCreateCall), [*locations catalogs branches products delete*](api::ProjectLocationCatalogBranchProductDeleteCall), [*locations catalogs branches products get*](api::ProjectLocationCatalogBranchProductGetCall), [*locations catalogs branches products import*](api::ProjectLocationCatalogBranchProductImportCall), [*locations catalogs branches products list*](api::ProjectLocationCatalogBranchProductListCall), [*locations catalogs branches products patch*](api::ProjectLocationCatalogBranchProductPatchCall), [*locations catalogs branches products purge*](api::ProjectLocationCatalogBranchProductPurgeCall), [*locations catalogs branches products remove fulfillment places*](api::ProjectLocationCatalogBranchProductRemoveFulfillmentPlaceCall), [*locations catalogs branches products remove local inventories*](api::ProjectLocationCatalogBranchProductRemoveLocalInventoryCall), [*locations catalogs branches products set inventory*](api::ProjectLocationCatalogBranchProductSetInventoryCall), [*locations catalogs complete query*](api::ProjectLocationCatalogCompleteQueryCall), [*locations catalogs completion data import*](api::ProjectLocationCatalogCompletionDataImportCall), [*locations catalogs controls create*](api::ProjectLocationCatalogControlCreateCall), [*locations catalogs controls delete*](api::ProjectLocationCatalogControlDeleteCall), [*locations catalogs controls get*](api::ProjectLocationCatalogControlGetCall), [*locations catalogs controls list*](api::ProjectLocationCatalogControlListCall), [*locations catalogs controls patch*](api::ProjectLocationCatalogControlPatchCall), [*locations catalogs export analytics metrics*](api::ProjectLocationCatalogExportAnalyticsMetricCall), [*locations catalogs get attributes config*](api::ProjectLocationCatalogGetAttributesConfigCall), [*locations catalogs get completion config*](api::ProjectLocationCatalogGetCompletionConfigCall), [*locations catalogs get default branch*](api::ProjectLocationCatalogGetDefaultBranchCall), [*locations catalogs list*](api::ProjectLocationCatalogListCall), [*locations catalogs models create*](api::ProjectLocationCatalogModelCreateCall), [*locations catalogs models delete*](api::ProjectLocationCatalogModelDeleteCall), [*locations catalogs models get*](api::ProjectLocationCatalogModelGetCall), [*locations catalogs models list*](api::ProjectLocationCatalogModelListCall), [*locations catalogs models patch*](api::ProjectLocationCatalogModelPatchCall), [*locations catalogs models pause*](api::ProjectLocationCatalogModelPauseCall), [*locations catalogs models resume*](api::ProjectLocationCatalogModelResumeCall), [*locations catalogs models tune*](api::ProjectLocationCatalogModelTuneCall), [*locations catalogs operations get*](api::ProjectLocationCatalogOperationGetCall), [*locations catalogs operations list*](api::ProjectLocationCatalogOperationListCall), [*locations catalogs patch*](api::ProjectLocationCatalogPatchCall), [*locations catalogs placements predict*](api::ProjectLocationCatalogPlacementPredictCall), [*locations catalogs placements search*](api::ProjectLocationCatalogPlacementSearchCall), [*locations catalogs serving configs add control*](api::ProjectLocationCatalogServingConfigAddControlCall), [*locations catalogs serving configs create*](api::ProjectLocationCatalogServingConfigCreateCall), [*locations catalogs serving configs delete*](api::ProjectLocationCatalogServingConfigDeleteCall), [*locations catalogs serving configs get*](api::ProjectLocationCatalogServingConfigGetCall), [*locations catalogs serving configs list*](api::ProjectLocationCatalogServingConfigListCall), [*locations catalogs serving configs patch*](api::ProjectLocationCatalogServingConfigPatchCall), [*locations catalogs serving configs predict*](api::ProjectLocationCatalogServingConfigPredictCall), [*locations catalogs serving configs remove control*](api::ProjectLocationCatalogServingConfigRemoveControlCall), [*locations catalogs serving configs search*](api::ProjectLocationCatalogServingConfigSearchCall), [*locations catalogs set default branch*](api::ProjectLocationCatalogSetDefaultBranchCall), [*locations catalogs update attributes config*](api::ProjectLocationCatalogUpdateAttributesConfigCall), [*locations catalogs update completion config*](api::ProjectLocationCatalogUpdateCompletionConfigCall), [*locations catalogs user events collect*](api::ProjectLocationCatalogUserEventCollectCall), [*locations catalogs user events import*](api::ProjectLocationCatalogUserEventImportCall), [*locations catalogs user events purge*](api::ProjectLocationCatalogUserEventPurgeCall), [*locations catalogs user events rejoin*](api::ProjectLocationCatalogUserEventRejoinCall), [*locations catalogs user events write*](api::ProjectLocationCatalogUserEventWriteCall), [*locations operations get*](api::ProjectLocationOperationGetCall), [*locations operations list*](api::ProjectLocationOperationListCall), [*operations get*](api::ProjectOperationGetCall) and [*operations list*](api::ProjectOperationListCall)
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
//! * **[Hub](CloudRetail)**
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
//! let r = hub.projects().locations_catalogs_branches_operations_get(...).doit().await
//! let r = hub.projects().locations_catalogs_branches_products_add_fulfillment_places(...).doit().await
//! let r = hub.projects().locations_catalogs_branches_products_add_local_inventories(...).doit().await
//! let r = hub.projects().locations_catalogs_branches_products_import(...).doit().await
//! let r = hub.projects().locations_catalogs_branches_products_purge(...).doit().await
//! let r = hub.projects().locations_catalogs_branches_products_remove_fulfillment_places(...).doit().await
//! let r = hub.projects().locations_catalogs_branches_products_remove_local_inventories(...).doit().await
//! let r = hub.projects().locations_catalogs_branches_products_set_inventory(...).doit().await
//! let r = hub.projects().locations_catalogs_completion_data_import(...).doit().await
//! let r = hub.projects().locations_catalogs_models_create(...).doit().await
//! let r = hub.projects().locations_catalogs_models_tune(...).doit().await
//! let r = hub.projects().locations_catalogs_operations_get(...).doit().await
//! let r = hub.projects().locations_catalogs_user_events_import(...).doit().await
//! let r = hub.projects().locations_catalogs_user_events_purge(...).doit().await
//! let r = hub.projects().locations_catalogs_user_events_rejoin(...).doit().await
//! let r = hub.projects().locations_catalogs_export_analytics_metrics(...).doit().await
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
//! google-retail2 = "*"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate google_retail2 as retail2;
//! use retail2::api::GoogleCloudRetailV2Model;
//! use retail2::{Result, Error};
//! # async fn dox() {
//! use std::default::Default;
//! use retail2::{CloudRetail, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
//! let mut hub = CloudRetail::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = GoogleCloudRetailV2Model::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.projects().locations_catalogs_models_create(req, "parent")
//!              .dry_run(false)
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
pub use api::CloudRetail;
pub use client::{Result, Error, Delegate, FieldMask};

// Re-export the yup_oauth2 crate, that is required to call some methods of the hub and the client
#[cfg(feature = "yup-oauth2")]
pub use client::oauth2;