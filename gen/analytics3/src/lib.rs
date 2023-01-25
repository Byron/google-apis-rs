// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *analytics* crate version *5.0.2-beta-1+20190807*, where *20190807* is the exact revision of the *analytics:v3* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.2-beta-1*.
//! 
//! Everything else about the *analytics* *v3* API can be found at the
//! [official documentation site](https://developers.google.com/analytics/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/analytics3).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](Analytics) ... 
//! 
//! * data
//!  * [*ga get*](api::DataGaGetCall), [*mcf get*](api::DataMcfGetCall) and [*realtime get*](api::DataRealtimeGetCall)
//! * management
//!  * [*account summaries list*](api::ManagementAccountSummaryListCall), [*account user links delete*](api::ManagementAccountUserLinkDeleteCall), [*account user links insert*](api::ManagementAccountUserLinkInsertCall), [*account user links list*](api::ManagementAccountUserLinkListCall), [*account user links update*](api::ManagementAccountUserLinkUpdateCall), [*accounts list*](api::ManagementAccountListCall), [*client id hash client id*](api::ManagementClientIdHashClientIdCall), [*custom data sources list*](api::ManagementCustomDataSourceListCall), [*custom dimensions get*](api::ManagementCustomDimensionGetCall), [*custom dimensions insert*](api::ManagementCustomDimensionInsertCall), [*custom dimensions list*](api::ManagementCustomDimensionListCall), [*custom dimensions patch*](api::ManagementCustomDimensionPatchCall), [*custom dimensions update*](api::ManagementCustomDimensionUpdateCall), [*custom metrics get*](api::ManagementCustomMetricGetCall), [*custom metrics insert*](api::ManagementCustomMetricInsertCall), [*custom metrics list*](api::ManagementCustomMetricListCall), [*custom metrics patch*](api::ManagementCustomMetricPatchCall), [*custom metrics update*](api::ManagementCustomMetricUpdateCall), [*experiments delete*](api::ManagementExperimentDeleteCall), [*experiments get*](api::ManagementExperimentGetCall), [*experiments insert*](api::ManagementExperimentInsertCall), [*experiments list*](api::ManagementExperimentListCall), [*experiments patch*](api::ManagementExperimentPatchCall), [*experiments update*](api::ManagementExperimentUpdateCall), [*filters delete*](api::ManagementFilterDeleteCall), [*filters get*](api::ManagementFilterGetCall), [*filters insert*](api::ManagementFilterInsertCall), [*filters list*](api::ManagementFilterListCall), [*filters patch*](api::ManagementFilterPatchCall), [*filters update*](api::ManagementFilterUpdateCall), [*goals get*](api::ManagementGoalGetCall), [*goals insert*](api::ManagementGoalInsertCall), [*goals list*](api::ManagementGoalListCall), [*goals patch*](api::ManagementGoalPatchCall), [*goals update*](api::ManagementGoalUpdateCall), [*profile filter links delete*](api::ManagementProfileFilterLinkDeleteCall), [*profile filter links get*](api::ManagementProfileFilterLinkGetCall), [*profile filter links insert*](api::ManagementProfileFilterLinkInsertCall), [*profile filter links list*](api::ManagementProfileFilterLinkListCall), [*profile filter links patch*](api::ManagementProfileFilterLinkPatchCall), [*profile filter links update*](api::ManagementProfileFilterLinkUpdateCall), [*profile user links delete*](api::ManagementProfileUserLinkDeleteCall), [*profile user links insert*](api::ManagementProfileUserLinkInsertCall), [*profile user links list*](api::ManagementProfileUserLinkListCall), [*profile user links update*](api::ManagementProfileUserLinkUpdateCall), [*profiles delete*](api::ManagementProfileDeleteCall), [*profiles get*](api::ManagementProfileGetCall), [*profiles insert*](api::ManagementProfileInsertCall), [*profiles list*](api::ManagementProfileListCall), [*profiles patch*](api::ManagementProfilePatchCall), [*profiles update*](api::ManagementProfileUpdateCall), [*remarketing audience delete*](api::ManagementRemarketingAudienceDeleteCall), [*remarketing audience get*](api::ManagementRemarketingAudienceGetCall), [*remarketing audience insert*](api::ManagementRemarketingAudienceInsertCall), [*remarketing audience list*](api::ManagementRemarketingAudienceListCall), [*remarketing audience patch*](api::ManagementRemarketingAudiencePatchCall), [*remarketing audience update*](api::ManagementRemarketingAudienceUpdateCall), [*segments list*](api::ManagementSegmentListCall), [*unsampled reports delete*](api::ManagementUnsampledReportDeleteCall), [*unsampled reports get*](api::ManagementUnsampledReportGetCall), [*unsampled reports insert*](api::ManagementUnsampledReportInsertCall), [*unsampled reports list*](api::ManagementUnsampledReportListCall), [*uploads delete upload data*](api::ManagementUploadDeleteUploadDataCall), [*uploads get*](api::ManagementUploadGetCall), [*uploads list*](api::ManagementUploadListCall), [*uploads upload data*](api::ManagementUploadUploadDataCall), [*web property ad words links delete*](api::ManagementWebPropertyAdWordsLinkDeleteCall), [*web property ad words links get*](api::ManagementWebPropertyAdWordsLinkGetCall), [*web property ad words links insert*](api::ManagementWebPropertyAdWordsLinkInsertCall), [*web property ad words links list*](api::ManagementWebPropertyAdWordsLinkListCall), [*web property ad words links patch*](api::ManagementWebPropertyAdWordsLinkPatchCall), [*web property ad words links update*](api::ManagementWebPropertyAdWordsLinkUpdateCall), [*webproperties get*](api::ManagementWebpropertyGetCall), [*webproperties insert*](api::ManagementWebpropertyInsertCall), [*webproperties list*](api::ManagementWebpropertyListCall), [*webproperties patch*](api::ManagementWebpropertyPatchCall), [*webproperties update*](api::ManagementWebpropertyUpdateCall), [*webproperty user links delete*](api::ManagementWebpropertyUserLinkDeleteCall), [*webproperty user links insert*](api::ManagementWebpropertyUserLinkInsertCall), [*webproperty user links list*](api::ManagementWebpropertyUserLinkListCall) and [*webproperty user links update*](api::ManagementWebpropertyUserLinkUpdateCall)
//! * metadata
//!  * [*columns list*](api::MetadataColumnListCall)
//! * provisioning
//!  * [*create account ticket*](api::ProvisioningCreateAccountTicketCall) and [*create account tree*](api::ProvisioningCreateAccountTreeCall)
//! * user deletion
//!  * [*user deletion request upsert*](api::UserDeletionUserDeletionRequestUpsertCall)
//! 
//! 
//! Upload supported by ...
//! 
//! * [*uploads upload data management*](api::ManagementUploadUploadDataCall)
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](Analytics)**
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
//! let r = hub.management().account_user_links_insert(...).doit().await
//! let r = hub.management().account_user_links_update(...).doit().await
//! let r = hub.management().profile_user_links_insert(...).doit().await
//! let r = hub.management().profile_user_links_update(...).doit().await
//! let r = hub.management().webproperty_user_links_insert(...).doit().await
//! let r = hub.management().webproperty_user_links_update(...).doit().await
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
//! google-analytics3 = "*"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate google_analytics3 as analytics3;
//! use analytics3::api::EntityUserLink;
//! use analytics3::{Result, Error};
//! # async fn dox() {
//! use std::default::Default;
//! use analytics3::{Analytics, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
//! let mut hub = Analytics::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = EntityUserLink::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.management().profile_user_links_update(req, "accountId", "webPropertyId", "profileId", "linkId")
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
pub use api::Analytics;
pub use client::{Result, Error, Delegate, FieldMask};

// Re-export the yup_oauth2 crate, that is required to call some methods of the hub and the client
#[cfg(feature = "yup-oauth2")]
pub use client::oauth2;