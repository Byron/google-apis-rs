// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Logging* crate version *5.0.5+20240531*, where *20240531* is the exact revision of the *logging:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.
//! 
//! Everything else about the *Logging* *v2* API can be found at the
//! [official documentation site](https://cloud.google.com/logging/docs/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/logging2).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](Logging) ... 
//! 
//! * billing accounts
//!  * [*exclusions create*](api::BillingAccountExclusionCreateCall), [*exclusions delete*](api::BillingAccountExclusionDeleteCall), [*exclusions get*](api::BillingAccountExclusionGetCall), [*exclusions list*](api::BillingAccountExclusionListCall), [*exclusions patch*](api::BillingAccountExclusionPatchCall), [*get cmek settings*](api::BillingAccountGetCmekSettingCall), [*get settings*](api::BillingAccountGetSettingCall), [*locations buckets create*](api::BillingAccountLocationBucketCreateCall), [*locations buckets create async*](api::BillingAccountLocationBucketCreateAsyncCall), [*locations buckets delete*](api::BillingAccountLocationBucketDeleteCall), [*locations buckets get*](api::BillingAccountLocationBucketGetCall), [*locations buckets links create*](api::BillingAccountLocationBucketLinkCreateCall), [*locations buckets links delete*](api::BillingAccountLocationBucketLinkDeleteCall), [*locations buckets links get*](api::BillingAccountLocationBucketLinkGetCall), [*locations buckets links list*](api::BillingAccountLocationBucketLinkListCall), [*locations buckets list*](api::BillingAccountLocationBucketListCall), [*locations buckets patch*](api::BillingAccountLocationBucketPatchCall), [*locations buckets undelete*](api::BillingAccountLocationBucketUndeleteCall), [*locations buckets update async*](api::BillingAccountLocationBucketUpdateAsyncCall), [*locations buckets views create*](api::BillingAccountLocationBucketViewCreateCall), [*locations buckets views delete*](api::BillingAccountLocationBucketViewDeleteCall), [*locations buckets views get*](api::BillingAccountLocationBucketViewGetCall), [*locations buckets views list*](api::BillingAccountLocationBucketViewListCall), [*locations buckets views logs list*](api::BillingAccountLocationBucketViewLogListCall), [*locations buckets views patch*](api::BillingAccountLocationBucketViewPatchCall), [*locations get*](api::BillingAccountLocationGetCall), [*locations list*](api::BillingAccountLocationListCall), [*locations operations cancel*](api::BillingAccountLocationOperationCancelCall), [*locations operations get*](api::BillingAccountLocationOperationGetCall), [*locations operations list*](api::BillingAccountLocationOperationListCall), [*locations recent queries list*](api::BillingAccountLocationRecentQueryListCall), [*locations saved queries create*](api::BillingAccountLocationSavedQueryCreateCall), [*locations saved queries delete*](api::BillingAccountLocationSavedQueryDeleteCall), [*locations saved queries list*](api::BillingAccountLocationSavedQueryListCall), [*logs delete*](api::BillingAccountLogDeleteCall), [*logs list*](api::BillingAccountLogListCall), [*sinks create*](api::BillingAccountSinkCreateCall), [*sinks delete*](api::BillingAccountSinkDeleteCall), [*sinks get*](api::BillingAccountSinkGetCall), [*sinks list*](api::BillingAccountSinkListCall), [*sinks patch*](api::BillingAccountSinkPatchCall) and [*sinks update*](api::BillingAccountSinkUpdateCall)
//! * entries
//!  * [*copy*](api::EntryCopyCall), [*list*](api::EntryListCall), [*tail*](api::EntryTailCall) and [*write*](api::EntryWriteCall)
//! * exclusions
//!  * [*create*](api::ExclusionCreateCall), [*delete*](api::ExclusionDeleteCall), [*get*](api::ExclusionGetCall), [*list*](api::ExclusionListCall) and [*patch*](api::ExclusionPatchCall)
//! * folders
//!  * [*exclusions create*](api::FolderExclusionCreateCall), [*exclusions delete*](api::FolderExclusionDeleteCall), [*exclusions get*](api::FolderExclusionGetCall), [*exclusions list*](api::FolderExclusionListCall), [*exclusions patch*](api::FolderExclusionPatchCall), [*get cmek settings*](api::FolderGetCmekSettingCall), [*get settings*](api::FolderGetSettingCall), [*locations buckets create*](api::FolderLocationBucketCreateCall), [*locations buckets create async*](api::FolderLocationBucketCreateAsyncCall), [*locations buckets delete*](api::FolderLocationBucketDeleteCall), [*locations buckets get*](api::FolderLocationBucketGetCall), [*locations buckets links create*](api::FolderLocationBucketLinkCreateCall), [*locations buckets links delete*](api::FolderLocationBucketLinkDeleteCall), [*locations buckets links get*](api::FolderLocationBucketLinkGetCall), [*locations buckets links list*](api::FolderLocationBucketLinkListCall), [*locations buckets list*](api::FolderLocationBucketListCall), [*locations buckets patch*](api::FolderLocationBucketPatchCall), [*locations buckets undelete*](api::FolderLocationBucketUndeleteCall), [*locations buckets update async*](api::FolderLocationBucketUpdateAsyncCall), [*locations buckets views create*](api::FolderLocationBucketViewCreateCall), [*locations buckets views delete*](api::FolderLocationBucketViewDeleteCall), [*locations buckets views get*](api::FolderLocationBucketViewGetCall), [*locations buckets views get iam policy*](api::FolderLocationBucketViewGetIamPolicyCall), [*locations buckets views list*](api::FolderLocationBucketViewListCall), [*locations buckets views logs list*](api::FolderLocationBucketViewLogListCall), [*locations buckets views patch*](api::FolderLocationBucketViewPatchCall), [*locations buckets views set iam policy*](api::FolderLocationBucketViewSetIamPolicyCall), [*locations buckets views test iam permissions*](api::FolderLocationBucketViewTestIamPermissionCall), [*locations get*](api::FolderLocationGetCall), [*locations list*](api::FolderLocationListCall), [*locations operations cancel*](api::FolderLocationOperationCancelCall), [*locations operations get*](api::FolderLocationOperationGetCall), [*locations operations list*](api::FolderLocationOperationListCall), [*locations recent queries list*](api::FolderLocationRecentQueryListCall), [*locations saved queries create*](api::FolderLocationSavedQueryCreateCall), [*locations saved queries delete*](api::FolderLocationSavedQueryDeleteCall), [*locations saved queries list*](api::FolderLocationSavedQueryListCall), [*logs delete*](api::FolderLogDeleteCall), [*logs list*](api::FolderLogListCall), [*sinks create*](api::FolderSinkCreateCall), [*sinks delete*](api::FolderSinkDeleteCall), [*sinks get*](api::FolderSinkGetCall), [*sinks list*](api::FolderSinkListCall), [*sinks patch*](api::FolderSinkPatchCall), [*sinks update*](api::FolderSinkUpdateCall) and [*update settings*](api::FolderUpdateSettingCall)
//! * [locations](api::Location)
//!  * [*buckets create*](api::LocationBucketCreateCall), [*buckets create async*](api::LocationBucketCreateAsyncCall), [*buckets delete*](api::LocationBucketDeleteCall), [*buckets get*](api::LocationBucketGetCall), [*buckets links create*](api::LocationBucketLinkCreateCall), [*buckets links delete*](api::LocationBucketLinkDeleteCall), [*buckets links get*](api::LocationBucketLinkGetCall), [*buckets links list*](api::LocationBucketLinkListCall), [*buckets list*](api::LocationBucketListCall), [*buckets patch*](api::LocationBucketPatchCall), [*buckets undelete*](api::LocationBucketUndeleteCall), [*buckets update async*](api::LocationBucketUpdateAsyncCall), [*buckets views create*](api::LocationBucketViewCreateCall), [*buckets views delete*](api::LocationBucketViewDeleteCall), [*buckets views get*](api::LocationBucketViewGetCall), [*buckets views get iam policy*](api::LocationBucketViewGetIamPolicyCall), [*buckets views list*](api::LocationBucketViewListCall), [*buckets views patch*](api::LocationBucketViewPatchCall), [*buckets views set iam policy*](api::LocationBucketViewSetIamPolicyCall), [*buckets views test iam permissions*](api::LocationBucketViewTestIamPermissionCall), [*get*](api::LocationGetCall), [*list*](api::LocationListCall), [*operations cancel*](api::LocationOperationCancelCall), [*operations get*](api::LocationOperationGetCall) and [*operations list*](api::LocationOperationListCall)
//! * logs
//!  * [*delete*](api::LogDeleteCall) and [*list*](api::LogListCall)
//! * [monitored resource descriptors](api::MonitoredResourceDescriptor)
//!  * [*list*](api::MonitoredResourceDescriptorListCall)
//! * organizations
//!  * [*exclusions create*](api::OrganizationExclusionCreateCall), [*exclusions delete*](api::OrganizationExclusionDeleteCall), [*exclusions get*](api::OrganizationExclusionGetCall), [*exclusions list*](api::OrganizationExclusionListCall), [*exclusions patch*](api::OrganizationExclusionPatchCall), [*get cmek settings*](api::OrganizationGetCmekSettingCall), [*get settings*](api::OrganizationGetSettingCall), [*locations buckets create*](api::OrganizationLocationBucketCreateCall), [*locations buckets create async*](api::OrganizationLocationBucketCreateAsyncCall), [*locations buckets delete*](api::OrganizationLocationBucketDeleteCall), [*locations buckets get*](api::OrganizationLocationBucketGetCall), [*locations buckets links create*](api::OrganizationLocationBucketLinkCreateCall), [*locations buckets links delete*](api::OrganizationLocationBucketLinkDeleteCall), [*locations buckets links get*](api::OrganizationLocationBucketLinkGetCall), [*locations buckets links list*](api::OrganizationLocationBucketLinkListCall), [*locations buckets list*](api::OrganizationLocationBucketListCall), [*locations buckets patch*](api::OrganizationLocationBucketPatchCall), [*locations buckets undelete*](api::OrganizationLocationBucketUndeleteCall), [*locations buckets update async*](api::OrganizationLocationBucketUpdateAsyncCall), [*locations buckets views create*](api::OrganizationLocationBucketViewCreateCall), [*locations buckets views delete*](api::OrganizationLocationBucketViewDeleteCall), [*locations buckets views get*](api::OrganizationLocationBucketViewGetCall), [*locations buckets views get iam policy*](api::OrganizationLocationBucketViewGetIamPolicyCall), [*locations buckets views list*](api::OrganizationLocationBucketViewListCall), [*locations buckets views logs list*](api::OrganizationLocationBucketViewLogListCall), [*locations buckets views patch*](api::OrganizationLocationBucketViewPatchCall), [*locations buckets views set iam policy*](api::OrganizationLocationBucketViewSetIamPolicyCall), [*locations buckets views test iam permissions*](api::OrganizationLocationBucketViewTestIamPermissionCall), [*locations get*](api::OrganizationLocationGetCall), [*locations list*](api::OrganizationLocationListCall), [*locations operations cancel*](api::OrganizationLocationOperationCancelCall), [*locations operations get*](api::OrganizationLocationOperationGetCall), [*locations operations list*](api::OrganizationLocationOperationListCall), [*locations recent queries list*](api::OrganizationLocationRecentQueryListCall), [*locations saved queries create*](api::OrganizationLocationSavedQueryCreateCall), [*locations saved queries delete*](api::OrganizationLocationSavedQueryDeleteCall), [*locations saved queries list*](api::OrganizationLocationSavedQueryListCall), [*logs delete*](api::OrganizationLogDeleteCall), [*logs list*](api::OrganizationLogListCall), [*sinks create*](api::OrganizationSinkCreateCall), [*sinks delete*](api::OrganizationSinkDeleteCall), [*sinks get*](api::OrganizationSinkGetCall), [*sinks list*](api::OrganizationSinkListCall), [*sinks patch*](api::OrganizationSinkPatchCall), [*sinks update*](api::OrganizationSinkUpdateCall), [*update cmek settings*](api::OrganizationUpdateCmekSettingCall) and [*update settings*](api::OrganizationUpdateSettingCall)
//! * projects
//!  * [*exclusions create*](api::ProjectExclusionCreateCall), [*exclusions delete*](api::ProjectExclusionDeleteCall), [*exclusions get*](api::ProjectExclusionGetCall), [*exclusions list*](api::ProjectExclusionListCall), [*exclusions patch*](api::ProjectExclusionPatchCall), [*get cmek settings*](api::ProjectGetCmekSettingCall), [*get settings*](api::ProjectGetSettingCall), [*locations buckets create*](api::ProjectLocationBucketCreateCall), [*locations buckets create async*](api::ProjectLocationBucketCreateAsyncCall), [*locations buckets delete*](api::ProjectLocationBucketDeleteCall), [*locations buckets get*](api::ProjectLocationBucketGetCall), [*locations buckets links create*](api::ProjectLocationBucketLinkCreateCall), [*locations buckets links delete*](api::ProjectLocationBucketLinkDeleteCall), [*locations buckets links get*](api::ProjectLocationBucketLinkGetCall), [*locations buckets links list*](api::ProjectLocationBucketLinkListCall), [*locations buckets list*](api::ProjectLocationBucketListCall), [*locations buckets patch*](api::ProjectLocationBucketPatchCall), [*locations buckets undelete*](api::ProjectLocationBucketUndeleteCall), [*locations buckets update async*](api::ProjectLocationBucketUpdateAsyncCall), [*locations buckets views create*](api::ProjectLocationBucketViewCreateCall), [*locations buckets views delete*](api::ProjectLocationBucketViewDeleteCall), [*locations buckets views get*](api::ProjectLocationBucketViewGetCall), [*locations buckets views get iam policy*](api::ProjectLocationBucketViewGetIamPolicyCall), [*locations buckets views list*](api::ProjectLocationBucketViewListCall), [*locations buckets views logs list*](api::ProjectLocationBucketViewLogListCall), [*locations buckets views patch*](api::ProjectLocationBucketViewPatchCall), [*locations buckets views set iam policy*](api::ProjectLocationBucketViewSetIamPolicyCall), [*locations buckets views test iam permissions*](api::ProjectLocationBucketViewTestIamPermissionCall), [*locations get*](api::ProjectLocationGetCall), [*locations list*](api::ProjectLocationListCall), [*locations operations cancel*](api::ProjectLocationOperationCancelCall), [*locations operations get*](api::ProjectLocationOperationGetCall), [*locations operations list*](api::ProjectLocationOperationListCall), [*locations recent queries list*](api::ProjectLocationRecentQueryListCall), [*locations saved queries create*](api::ProjectLocationSavedQueryCreateCall), [*locations saved queries delete*](api::ProjectLocationSavedQueryDeleteCall), [*locations saved queries list*](api::ProjectLocationSavedQueryListCall), [*logs delete*](api::ProjectLogDeleteCall), [*logs list*](api::ProjectLogListCall), [*metrics create*](api::ProjectMetricCreateCall), [*metrics delete*](api::ProjectMetricDeleteCall), [*metrics get*](api::ProjectMetricGetCall), [*metrics list*](api::ProjectMetricListCall), [*metrics update*](api::ProjectMetricUpdateCall), [*sinks create*](api::ProjectSinkCreateCall), [*sinks delete*](api::ProjectSinkDeleteCall), [*sinks get*](api::ProjectSinkGetCall), [*sinks list*](api::ProjectSinkListCall), [*sinks patch*](api::ProjectSinkPatchCall) and [*sinks update*](api::ProjectSinkUpdateCall)
//! * sinks
//!  * [*create*](api::SinkCreateCall), [*delete*](api::SinkDeleteCall), [*get*](api::SinkGetCall), [*list*](api::SinkListCall) and [*update*](api::SinkUpdateCall)
//! 
//! Other activities are ...
//! 
//! * [get cmek settings](api::MethodGetCmekSettingCall)
//! * [get settings](api::MethodGetSettingCall)
//! * [update cmek settings](api::MethodUpdateCmekSettingCall)
//! * [update settings](api::MethodUpdateSettingCall)
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](Logging)**
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
//! let r = hub.billing_accounts().exclusions_delete(...).doit().await
//! let r = hub.billing_accounts().locations_buckets_views_delete(...).doit().await
//! let r = hub.billing_accounts().locations_buckets_delete(...).doit().await
//! let r = hub.billing_accounts().locations_buckets_undelete(...).doit().await
//! let r = hub.billing_accounts().locations_operations_cancel(...).doit().await
//! let r = hub.billing_accounts().locations_saved_queries_delete(...).doit().await
//! let r = hub.billing_accounts().logs_delete(...).doit().await
//! let r = hub.billing_accounts().sinks_delete(...).doit().await
//! let r = hub.exclusions().delete(...).doit().await
//! let r = hub.folders().exclusions_delete(...).doit().await
//! let r = hub.folders().locations_buckets_views_delete(...).doit().await
//! let r = hub.folders().locations_buckets_delete(...).doit().await
//! let r = hub.folders().locations_buckets_undelete(...).doit().await
//! let r = hub.folders().locations_operations_cancel(...).doit().await
//! let r = hub.folders().locations_saved_queries_delete(...).doit().await
//! let r = hub.folders().logs_delete(...).doit().await
//! let r = hub.folders().sinks_delete(...).doit().await
//! let r = hub.locations().buckets_views_delete(...).doit().await
//! let r = hub.locations().buckets_delete(...).doit().await
//! let r = hub.locations().buckets_undelete(...).doit().await
//! let r = hub.locations().operations_cancel(...).doit().await
//! let r = hub.logs().delete(...).doit().await
//! let r = hub.organizations().exclusions_delete(...).doit().await
//! let r = hub.organizations().locations_buckets_views_delete(...).doit().await
//! let r = hub.organizations().locations_buckets_delete(...).doit().await
//! let r = hub.organizations().locations_buckets_undelete(...).doit().await
//! let r = hub.organizations().locations_operations_cancel(...).doit().await
//! let r = hub.organizations().locations_saved_queries_delete(...).doit().await
//! let r = hub.organizations().logs_delete(...).doit().await
//! let r = hub.organizations().sinks_delete(...).doit().await
//! let r = hub.projects().exclusions_delete(...).doit().await
//! let r = hub.projects().locations_buckets_views_delete(...).doit().await
//! let r = hub.projects().locations_buckets_delete(...).doit().await
//! let r = hub.projects().locations_buckets_undelete(...).doit().await
//! let r = hub.projects().locations_operations_cancel(...).doit().await
//! let r = hub.projects().locations_saved_queries_delete(...).doit().await
//! let r = hub.projects().logs_delete(...).doit().await
//! let r = hub.projects().metrics_delete(...).doit().await
//! let r = hub.projects().sinks_delete(...).doit().await
//! let r = hub.sinks().delete(...).doit().await
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
//! google-logging2 = "*"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate google_logging2 as logging2;
//! use logging2::api::UndeleteBucketRequest;
//! use logging2::{Result, Error};
//! # async fn dox() {
//! use std::default::Default;
//! use logging2::{Logging, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
//! let mut hub = Logging::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = UndeleteBucketRequest::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.billing_accounts().locations_buckets_undelete(req, "name")
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
pub use api::Logging;
pub use client::{Result, Error, Delegate, FieldMask};

// Re-export the yup_oauth2 crate, that is required to call some methods of the hub and the client
#[cfg(feature = "yup-oauth2")]
pub use client::oauth2;