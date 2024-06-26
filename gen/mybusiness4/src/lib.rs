// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *My Business* crate version *5.0.5+0*, where *0* is the exact revision of the *mybusiness:v4* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.
//! 
//! Everything else about the *My Business* *v4* API can be found at the
//! [official documentation site](https://developers.google.com/my-business/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/mybusiness4).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](MyBusiness) ... 
//! 
//! * [accounts](api::Account)
//!  * [*admins create*](api::AccountAdminCreateCall), [*admins delete*](api::AccountAdminDeleteCall), [*admins list*](api::AccountAdminListCall), [*admins patch*](api::AccountAdminPatchCall), [*create*](api::AccountCreateCall), [*delete notifications*](api::AccountDeleteNotificationCall), [*generate account number*](api::AccountGenerateAccountNumberCall), [*get*](api::AccountGetCall), [*get notifications*](api::AccountGetNotificationCall), [*invitations accept*](api::AccountInvitationAcceptCall), [*invitations decline*](api::AccountInvitationDeclineCall), [*invitations list*](api::AccountInvitationListCall), [*list*](api::AccountListCall), [*list recommend google locations*](api::AccountListRecommendGoogleLocationCall), [*locations admins create*](api::AccountLocationAdminCreateCall), [*locations admins delete*](api::AccountLocationAdminDeleteCall), [*locations admins list*](api::AccountLocationAdminListCall), [*locations admins patch*](api::AccountLocationAdminPatchCall), [*locations associate*](api::AccountLocationAssociateCall), [*locations batch get*](api::AccountLocationBatchGetCall), [*locations batch get reviews*](api::AccountLocationBatchGetReviewCall), [*locations clear association*](api::AccountLocationClearAssociationCall), [*locations create*](api::AccountLocationCreateCall), [*locations delete*](api::AccountLocationDeleteCall), [*locations fetch verification options*](api::AccountLocationFetchVerificationOptionCall), [*locations find matches*](api::AccountLocationFindMatchCall), [*locations followers get metadata*](api::AccountLocationFollowerGetMetadataCall), [*locations get*](api::AccountLocationGetCall), [*locations get google updated*](api::AccountLocationGetGoogleUpdatedCall), [*locations list*](api::AccountLocationListCall), [*locations local posts create*](api::AccountLocationLocalPostCreateCall), [*locations local posts delete*](api::AccountLocationLocalPostDeleteCall), [*locations local posts get*](api::AccountLocationLocalPostGetCall), [*locations local posts list*](api::AccountLocationLocalPostListCall), [*locations local posts patch*](api::AccountLocationLocalPostPatchCall), [*locations local posts report insights*](api::AccountLocationLocalPostReportInsightCall), [*locations media create*](api::AccountLocationMediaCreateCall), [*locations media customers get*](api::AccountLocationMediaCustomerGetCall), [*locations media customers list*](api::AccountLocationMediaCustomerListCall), [*locations media delete*](api::AccountLocationMediaDeleteCall), [*locations media get*](api::AccountLocationMediaGetCall), [*locations media list*](api::AccountLocationMediaListCall), [*locations media patch*](api::AccountLocationMediaPatchCall), [*locations media start upload*](api::AccountLocationMediaStartUploadCall), [*locations patch*](api::AccountLocationPatchCall), [*locations questions answers delete*](api::AccountLocationQuestionAnswerDeleteCall), [*locations questions answers list*](api::AccountLocationQuestionAnswerListCall), [*locations questions answers upsert*](api::AccountLocationQuestionAnswerUpsertCall), [*locations questions create*](api::AccountLocationQuestionCreateCall), [*locations questions delete*](api::AccountLocationQuestionDeleteCall), [*locations questions list*](api::AccountLocationQuestionListCall), [*locations questions patch*](api::AccountLocationQuestionPatchCall), [*locations report insights*](api::AccountLocationReportInsightCall), [*locations reviews delete reply*](api::AccountLocationReviewDeleteReplyCall), [*locations reviews get*](api::AccountLocationReviewGetCall), [*locations reviews list*](api::AccountLocationReviewListCall), [*locations reviews update reply*](api::AccountLocationReviewUpdateReplyCall), [*locations transfer*](api::AccountLocationTransferCall), [*locations verifications complete*](api::AccountLocationVerificationCompleteCall), [*locations verifications list*](api::AccountLocationVerificationListCall), [*locations verify*](api::AccountLocationVerifyCall), [*update*](api::AccountUpdateCall) and [*update notifications*](api::AccountUpdateNotificationCall)
//! * [attributes](api::Attribute)
//!  * [*list*](api::AttributeListCall)
//! * [categories](api::Category)
//!  * [*list*](api::CategoryListCall)
//! * [chains](api::Chain)
//!  * [*get*](api::ChainGetCall) and [*search*](api::ChainSearchCall)
//! * [google locations](api::GoogleLocation)
//!  * [*report*](api::GoogleLocationReportCall) and [*search*](api::GoogleLocationSearchCall)
//! * [verification tokens](api::VerificationToken)
//!  * [*generate*](api::VerificationTokenGenerateCall)
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
//! * **[Hub](MyBusiness)**
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
//! let r = hub.accounts().admins_create(...).doit().await
//! let r = hub.accounts().admins_delete(...).doit().await
//! let r = hub.accounts().admins_list(...).doit().await
//! let r = hub.accounts().admins_patch(...).doit().await
//! let r = hub.accounts().invitations_accept(...).doit().await
//! let r = hub.accounts().invitations_decline(...).doit().await
//! let r = hub.accounts().invitations_list(...).doit().await
//! let r = hub.accounts().locations_admins_create(...).doit().await
//! let r = hub.accounts().locations_admins_delete(...).doit().await
//! let r = hub.accounts().locations_admins_list(...).doit().await
//! let r = hub.accounts().locations_admins_patch(...).doit().await
//! let r = hub.accounts().locations_followers_get_metadata(...).doit().await
//! let r = hub.accounts().locations_local_posts_create(...).doit().await
//! let r = hub.accounts().locations_local_posts_delete(...).doit().await
//! let r = hub.accounts().locations_local_posts_get(...).doit().await
//! let r = hub.accounts().locations_local_posts_list(...).doit().await
//! let r = hub.accounts().locations_local_posts_patch(...).doit().await
//! let r = hub.accounts().locations_local_posts_report_insights(...).doit().await
//! let r = hub.accounts().locations_media_customers_get(...).doit().await
//! let r = hub.accounts().locations_media_customers_list(...).doit().await
//! let r = hub.accounts().locations_media_create(...).doit().await
//! let r = hub.accounts().locations_media_delete(...).doit().await
//! let r = hub.accounts().locations_media_get(...).doit().await
//! let r = hub.accounts().locations_media_list(...).doit().await
//! let r = hub.accounts().locations_media_patch(...).doit().await
//! let r = hub.accounts().locations_media_start_upload(...).doit().await
//! let r = hub.accounts().locations_questions_answers_delete(...).doit().await
//! let r = hub.accounts().locations_questions_answers_list(...).doit().await
//! let r = hub.accounts().locations_questions_answers_upsert(...).doit().await
//! let r = hub.accounts().locations_questions_create(...).doit().await
//! let r = hub.accounts().locations_questions_delete(...).doit().await
//! let r = hub.accounts().locations_questions_list(...).doit().await
//! let r = hub.accounts().locations_questions_patch(...).doit().await
//! let r = hub.accounts().locations_reviews_delete_reply(...).doit().await
//! let r = hub.accounts().locations_reviews_get(...).doit().await
//! let r = hub.accounts().locations_reviews_list(...).doit().await
//! let r = hub.accounts().locations_reviews_update_reply(...).doit().await
//! let r = hub.accounts().locations_verifications_complete(...).doit().await
//! let r = hub.accounts().locations_verifications_list(...).doit().await
//! let r = hub.accounts().locations_associate(...).doit().await
//! let r = hub.accounts().locations_batch_get(...).doit().await
//! let r = hub.accounts().locations_batch_get_reviews(...).doit().await
//! let r = hub.accounts().locations_clear_association(...).doit().await
//! let r = hub.accounts().locations_create(...).doit().await
//! let r = hub.accounts().locations_delete(...).doit().await
//! let r = hub.accounts().locations_fetch_verification_options(...).doit().await
//! let r = hub.accounts().locations_find_matches(...).doit().await
//! let r = hub.accounts().locations_get(...).doit().await
//! let r = hub.accounts().locations_get_google_updated(...).doit().await
//! let r = hub.accounts().locations_list(...).doit().await
//! let r = hub.accounts().locations_patch(...).doit().await
//! let r = hub.accounts().locations_report_insights(...).doit().await
//! let r = hub.accounts().locations_transfer(...).doit().await
//! let r = hub.accounts().locations_verify(...).doit().await
//! let r = hub.accounts().create(...).doit().await
//! let r = hub.accounts().delete_notifications(...).doit().await
//! let r = hub.accounts().generate_account_number(...).doit().await
//! let r = hub.accounts().get(...).doit().await
//! let r = hub.accounts().get_notifications(...).doit().await
//! let r = hub.accounts().list(...).doit().await
//! let r = hub.accounts().list_recommend_google_locations(...).doit().await
//! let r = hub.accounts().update(...).doit().await
//! let r = hub.accounts().update_notifications(...).doit().await
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
//! google-mybusiness4 = "*"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate google_mybusiness4 as mybusiness4;
//! use mybusiness4::{Result, Error};
//! # async fn dox() {
//! use std::default::Default;
//! use mybusiness4::{MyBusiness, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
//! let mut hub = MyBusiness::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.accounts().locations_questions_list("parent")
//!              .page_token("sed")
//!              .page_size(-2)
//!              .order_by("takimata")
//!              .filter("amet.")
//!              .answers_per_question(-20)
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
pub use api::MyBusiness;
pub use client::{Result, Error, Delegate, FieldMask};

// Re-export the yup_oauth2 crate, that is required to call some methods of the hub and the client
#[cfg(feature = "yup-oauth2")]
pub use client::oauth2;