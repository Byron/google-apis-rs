// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *analytics* crate version *1.0.0+20160805*, where *20160805* is the exact revision of the *analytics:v3* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.0*.
//! 
//! Everything else about the *analytics* *v3* API can be found at the
//! [official documentation site](https://developers.google.com/analytics/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/analytics3).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Analytics.html) ... 
//! 
//! * data
//!  * [*ga get*](struct.DataGaGetCall.html), [*mcf get*](struct.DataMcfGetCall.html) and [*realtime get*](struct.DataRealtimeGetCall.html)
//! * management
//!  * [*account summaries list*](struct.ManagementAccountSummaryListCall.html), [*account user links delete*](struct.ManagementAccountUserLinkDeleteCall.html), [*account user links insert*](struct.ManagementAccountUserLinkInsertCall.html), [*account user links list*](struct.ManagementAccountUserLinkListCall.html), [*account user links update*](struct.ManagementAccountUserLinkUpdateCall.html), [*accounts list*](struct.ManagementAccountListCall.html), [*custom data sources list*](struct.ManagementCustomDataSourceListCall.html), [*custom dimensions get*](struct.ManagementCustomDimensionGetCall.html), [*custom dimensions insert*](struct.ManagementCustomDimensionInsertCall.html), [*custom dimensions list*](struct.ManagementCustomDimensionListCall.html), [*custom dimensions patch*](struct.ManagementCustomDimensionPatchCall.html), [*custom dimensions update*](struct.ManagementCustomDimensionUpdateCall.html), [*custom metrics get*](struct.ManagementCustomMetricGetCall.html), [*custom metrics insert*](struct.ManagementCustomMetricInsertCall.html), [*custom metrics list*](struct.ManagementCustomMetricListCall.html), [*custom metrics patch*](struct.ManagementCustomMetricPatchCall.html), [*custom metrics update*](struct.ManagementCustomMetricUpdateCall.html), [*experiments delete*](struct.ManagementExperimentDeleteCall.html), [*experiments get*](struct.ManagementExperimentGetCall.html), [*experiments insert*](struct.ManagementExperimentInsertCall.html), [*experiments list*](struct.ManagementExperimentListCall.html), [*experiments patch*](struct.ManagementExperimentPatchCall.html), [*experiments update*](struct.ManagementExperimentUpdateCall.html), [*filters delete*](struct.ManagementFilterDeleteCall.html), [*filters get*](struct.ManagementFilterGetCall.html), [*filters insert*](struct.ManagementFilterInsertCall.html), [*filters list*](struct.ManagementFilterListCall.html), [*filters patch*](struct.ManagementFilterPatchCall.html), [*filters update*](struct.ManagementFilterUpdateCall.html), [*goals get*](struct.ManagementGoalGetCall.html), [*goals insert*](struct.ManagementGoalInsertCall.html), [*goals list*](struct.ManagementGoalListCall.html), [*goals patch*](struct.ManagementGoalPatchCall.html), [*goals update*](struct.ManagementGoalUpdateCall.html), [*profile filter links delete*](struct.ManagementProfileFilterLinkDeleteCall.html), [*profile filter links get*](struct.ManagementProfileFilterLinkGetCall.html), [*profile filter links insert*](struct.ManagementProfileFilterLinkInsertCall.html), [*profile filter links list*](struct.ManagementProfileFilterLinkListCall.html), [*profile filter links patch*](struct.ManagementProfileFilterLinkPatchCall.html), [*profile filter links update*](struct.ManagementProfileFilterLinkUpdateCall.html), [*profile user links delete*](struct.ManagementProfileUserLinkDeleteCall.html), [*profile user links insert*](struct.ManagementProfileUserLinkInsertCall.html), [*profile user links list*](struct.ManagementProfileUserLinkListCall.html), [*profile user links update*](struct.ManagementProfileUserLinkUpdateCall.html), [*profiles delete*](struct.ManagementProfileDeleteCall.html), [*profiles get*](struct.ManagementProfileGetCall.html), [*profiles insert*](struct.ManagementProfileInsertCall.html), [*profiles list*](struct.ManagementProfileListCall.html), [*profiles patch*](struct.ManagementProfilePatchCall.html), [*profiles update*](struct.ManagementProfileUpdateCall.html), [*remarketing audience get*](struct.ManagementRemarketingAudienceGetCall.html), [*remarketing audience insert*](struct.ManagementRemarketingAudienceInsertCall.html), [*remarketing audience list*](struct.ManagementRemarketingAudienceListCall.html), [*remarketing audience patch*](struct.ManagementRemarketingAudiencePatchCall.html), [*remarketing audience update*](struct.ManagementRemarketingAudienceUpdateCall.html), [*segments list*](struct.ManagementSegmentListCall.html), [*unsampled reports delete*](struct.ManagementUnsampledReportDeleteCall.html), [*unsampled reports get*](struct.ManagementUnsampledReportGetCall.html), [*unsampled reports insert*](struct.ManagementUnsampledReportInsertCall.html), [*unsampled reports list*](struct.ManagementUnsampledReportListCall.html), [*uploads delete upload data*](struct.ManagementUploadDeleteUploadDataCall.html), [*uploads get*](struct.ManagementUploadGetCall.html), [*uploads list*](struct.ManagementUploadListCall.html), [*uploads upload data*](struct.ManagementUploadUploadDataCall.html), [*web property ad words links delete*](struct.ManagementWebPropertyAdWordsLinkDeleteCall.html), [*web property ad words links get*](struct.ManagementWebPropertyAdWordsLinkGetCall.html), [*web property ad words links insert*](struct.ManagementWebPropertyAdWordsLinkInsertCall.html), [*web property ad words links list*](struct.ManagementWebPropertyAdWordsLinkListCall.html), [*web property ad words links patch*](struct.ManagementWebPropertyAdWordsLinkPatchCall.html), [*web property ad words links update*](struct.ManagementWebPropertyAdWordsLinkUpdateCall.html), [*webproperties get*](struct.ManagementWebpropertyGetCall.html), [*webproperties insert*](struct.ManagementWebpropertyInsertCall.html), [*webproperties list*](struct.ManagementWebpropertyListCall.html), [*webproperties patch*](struct.ManagementWebpropertyPatchCall.html), [*webproperties update*](struct.ManagementWebpropertyUpdateCall.html), [*webproperty user links delete*](struct.ManagementWebpropertyUserLinkDeleteCall.html), [*webproperty user links insert*](struct.ManagementWebpropertyUserLinkInsertCall.html), [*webproperty user links list*](struct.ManagementWebpropertyUserLinkListCall.html) and [*webproperty user links update*](struct.ManagementWebpropertyUserLinkUpdateCall.html)
//! * metadata
//!  * [*columns list*](struct.MetadataColumnListCall.html)
//! * provisioning
//!  * [*create account ticket*](struct.ProvisioningCreateAccountTicketCall.html)
//! 
//! 
//! Upload supported by ...
//! 
//! * [*uploads upload data management*](struct.ManagementUploadUploadDataCall.html)
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](../index.html).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.Analytics.html)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](trait.MethodsBuilder.html) which in turn
//!       allow access to individual [*Call Builders*](trait.CallBuilder.html)
//! * **[Resources](trait.Resource.html)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](trait.Part.html)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](trait.CallBuilder.html)**
//!     * operations to apply to *Resources*
//! 
//! All *structures* are marked with applicable traits to further categorize them and ease browsing.
//! 
//! Generally speaking, you can invoke *Activities* like this:
//! 
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit()
//! ```
//! 
//! Or specifically ...
//! 
//! ```ignore
//! let r = hub.management().webproperty_user_links_insert(...).doit()
//! let r = hub.management().profile_user_links_insert(...).doit()
//! let r = hub.management().profile_user_links_update(...).doit()
//! let r = hub.management().account_user_links_update(...).doit()
//! let r = hub.management().webproperty_user_links_update(...).doit()
//! let r = hub.management().account_user_links_insert(...).doit()
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
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_analytics3 as analytics3;
//! use analytics3::EntityUserLink;
//! use analytics3::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use analytics3::Analytics;
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
//!                               hyper::Client::new(),
//!                               <MemoryStorage as Default>::default(), None);
//! let mut hub = Analytics::new(hyper::Client::new(), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = EntityUserLink::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.management().profile_user_links_update(req, "accountId", "webPropertyId", "profileId", "linkId")
//!              .doit();
//! 
//! match result {
//!     Err(e) => match e {
//!         // The Error enum provides details about what exactly happened.
//!         // You can also just use its `Debug`, `Display` or `Error` traits
//!          Error::HttpError(_)
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
//! All errors produced by the system are provided either as [Result](enum.Result.html) enumeration as return value of 
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](trait.Delegate.html), or the [Authenticator Delegate](../yup-oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](enum.Result.html), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](trait.ResponseResult.html), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//! 
//! Methods supporting uploads can do so using up to 2 different protocols: 
//! *simple* and *resumable*. The distinctiveness of each is represented by customized 
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//! 
//! ## Customization and Callbacks
//! 
//! You may alter the way an `doit()` method is called by providing a [delegate](trait.Delegate.html) to the 
//! [Method Builder](trait.CallBuilder.html) before making the final `doit()` call. 
//! Respective methods will be called to provide progress information, as well as determine whether the system should 
//! retry on failure.
//! 
//! The [delegate trait](trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.
//! 
//! ## Optional Parts in Server-Requests
//! 
//! All structures provided by this library are made to be [enocodable](trait.RequestValue.html) and 
//! [decodable](trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
//! are valid.
//! Most optionals are are considered [Parts](trait.Part.html) which are identifiable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](trait.RequestValue.html) are moved
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
#![cfg_attr(feature = "nightly", feature(proc_macro))]
#![allow(unused_imports, unused_mut, dead_code)]

#[cfg(feature = "nightly")]
include!("lib.rs.in");

#[cfg(feature = "with-serde-codegen")]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));