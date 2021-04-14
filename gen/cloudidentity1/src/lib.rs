// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Cloud Identity* crate version *2.0.3+20210310*, where *20210310* is the exact revision of the *cloudidentity:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v2.0.3*.
//! 
//! Everything else about the *Cloud Identity* *v1* API can be found at the
//! [official documentation site](https://cloud.google.com/identity/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/cloudidentity1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](CloudIdentity) ... 
//! 
//! * devices
//!  * [*cancel wipe*](api::DeviceCancelWipeCall), [*create*](api::DeviceCreateCall), [*delete*](api::DeviceDeleteCall), [*device users approve*](api::DeviceDeviceUserApproveCall), [*device users block*](api::DeviceDeviceUserBlockCall), [*device users cancel wipe*](api::DeviceDeviceUserCancelWipeCall), [*device users client states get*](api::DeviceDeviceUserClientStateGetCall), [*device users client states list*](api::DeviceDeviceUserClientStateListCall), [*device users client states patch*](api::DeviceDeviceUserClientStatePatchCall), [*device users delete*](api::DeviceDeviceUserDeleteCall), [*device users get*](api::DeviceDeviceUserGetCall), [*device users list*](api::DeviceDeviceUserListCall), [*device users lookup*](api::DeviceDeviceUserLookupCall), [*device users wipe*](api::DeviceDeviceUserWipeCall), [*get*](api::DeviceGetCall), [*list*](api::DeviceListCall) and [*wipe*](api::DeviceWipeCall)
//! * [groups](api::Group)
//!  * [*create*](api::GroupCreateCall), [*delete*](api::GroupDeleteCall), [*get*](api::GroupGetCall), [*list*](api::GroupListCall), [*lookup*](api::GroupLookupCall), [*memberships check transitive membership*](api::GroupMembershipCheckTransitiveMembershipCall), [*memberships create*](api::GroupMembershipCreateCall), [*memberships delete*](api::GroupMembershipDeleteCall), [*memberships get*](api::GroupMembershipGetCall), [*memberships get membership graph*](api::GroupMembershipGetMembershipGraphCall), [*memberships list*](api::GroupMembershipListCall), [*memberships lookup*](api::GroupMembershipLookupCall), [*memberships modify membership roles*](api::GroupMembershipModifyMembershipRoleCall), [*memberships search transitive groups*](api::GroupMembershipSearchTransitiveGroupCall), [*memberships search transitive memberships*](api::GroupMembershipSearchTransitiveMembershipCall), [*patch*](api::GroupPatchCall) and [*search*](api::GroupSearchCall)
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
//! * **[Hub](CloudIdentity)**
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
//! let r = hub.groups().memberships_check_transitive_membership(...).doit().await
//! let r = hub.groups().memberships_create(...).doit().await
//! let r = hub.groups().memberships_delete(...).doit().await
//! let r = hub.groups().memberships_get(...).doit().await
//! let r = hub.groups().memberships_get_membership_graph(...).doit().await
//! let r = hub.groups().memberships_list(...).doit().await
//! let r = hub.groups().memberships_lookup(...).doit().await
//! let r = hub.groups().memberships_modify_membership_roles(...).doit().await
//! let r = hub.groups().memberships_search_transitive_groups(...).doit().await
//! let r = hub.groups().memberships_search_transitive_memberships(...).doit().await
//! let r = hub.groups().create(...).doit().await
//! let r = hub.groups().delete(...).doit().await
//! let r = hub.groups().get(...).doit().await
//! let r = hub.groups().list(...).doit().await
//! let r = hub.groups().lookup(...).doit().await
//! let r = hub.groups().patch(...).doit().await
//! let r = hub.groups().search(...).doit().await
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
//! google-cloudidentity1 = "*"
//! # This project intentionally uses an old version of Hyper. See
//! # https://github.com/Byron/google-apis-rs/issues/173 for more
//! # information.
//! hyper = "^0.14"
//! hyper-rustls = "^0.22"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! yup-oauth2 = "^5.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_cloudidentity1 as cloudidentity1;
//! use cloudidentity1::{Result, Error};
//! # async fn dox() {
//! use std::default::Default;
//! use oauth2;
//! use cloudidentity1::CloudIdentity;
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: oauth2::ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
//!         secret,
//!         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
//!     ).build().await.unwrap();
//! let mut hub = CloudIdentity::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.groups().memberships_list("parent")
//!              .view("At")
//!              .page_token("sanctus")
//!              .page_size(-80)
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
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate yup_oauth2 as oauth2;
extern crate mime;
extern crate url;

pub mod api;
pub mod client;

// Re-export the hub type and some basic client structs
pub use api::CloudIdentity;
pub use client::{Result, Error, Delegate};
