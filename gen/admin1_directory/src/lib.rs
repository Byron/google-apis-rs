// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *directory* crate version *0.1.13+20160323*, where *20160323* is the exact revision of the *admin:directory_v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.13*.
//! 
//! Everything else about the *directory* *v1_directory* API can be found at the
//! [official documentation site](https://developers.google.com/admin-sdk/directory/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/admin1_directory).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Directory.html) ... 
//! 
//! * [asps](struct.Asp.html)
//!  * [*delete*](struct.AspDeleteCall.html), [*get*](struct.AspGetCall.html) and [*list*](struct.AspListCall.html)
//! * [channels](struct.Channel.html)
//!  * [*stop*](struct.ChannelStopCall.html)
//! * chromeosdevices
//!  * [*get*](struct.ChromeosdeviceGetCall.html), [*list*](struct.ChromeosdeviceListCall.html), [*patch*](struct.ChromeosdevicePatchCall.html) and [*update*](struct.ChromeosdeviceUpdateCall.html)
//! * [customers](struct.Customer.html)
//!  * [*get*](struct.CustomerGetCall.html), [*patch*](struct.CustomerPatchCall.html) and [*update*](struct.CustomerUpdateCall.html)
//! * domain aliases
//!  * [*delete*](struct.DomainAliaseDeleteCall.html), [*get*](struct.DomainAliaseGetCall.html), [*insert*](struct.DomainAliaseInsertCall.html) and [*list*](struct.DomainAliaseListCall.html)
//! * domains
//!  * [*delete*](struct.DomainDeleteCall.html), [*get*](struct.DomainGetCall.html), [*insert*](struct.DomainInsertCall.html) and [*list*](struct.DomainListCall.html)
//! * [groups](struct.Group.html)
//!  * [*aliases delete*](struct.GroupAliaseDeleteCall.html), [*aliases insert*](struct.GroupAliaseInsertCall.html), [*aliases list*](struct.GroupAliaseListCall.html), [*delete*](struct.GroupDeleteCall.html), [*get*](struct.GroupGetCall.html), [*insert*](struct.GroupInsertCall.html), [*list*](struct.GroupListCall.html), [*patch*](struct.GroupPatchCall.html) and [*update*](struct.GroupUpdateCall.html)
//! * [members](struct.Member.html)
//!  * [*delete*](struct.MemberDeleteCall.html), [*get*](struct.MemberGetCall.html), [*insert*](struct.MemberInsertCall.html), [*list*](struct.MemberListCall.html), [*patch*](struct.MemberPatchCall.html) and [*update*](struct.MemberUpdateCall.html)
//! * mobiledevices
//!  * [*action*](struct.MobiledeviceActionCall.html), [*delete*](struct.MobiledeviceDeleteCall.html), [*get*](struct.MobiledeviceGetCall.html) and [*list*](struct.MobiledeviceListCall.html)
//! * [notifications](struct.Notification.html)
//!  * [*delete*](struct.NotificationDeleteCall.html), [*get*](struct.NotificationGetCall.html), [*list*](struct.NotificationListCall.html), [*patch*](struct.NotificationPatchCall.html) and [*update*](struct.NotificationUpdateCall.html)
//! * orgunits
//!  * [*delete*](struct.OrgunitDeleteCall.html), [*get*](struct.OrgunitGetCall.html), [*insert*](struct.OrgunitInsertCall.html), [*list*](struct.OrgunitListCall.html), [*patch*](struct.OrgunitPatchCall.html) and [*update*](struct.OrgunitUpdateCall.html)
//! * [privileges](struct.Privilege.html)
//!  * [*list*](struct.PrivilegeListCall.html)
//! * resources
//!  * [*calendars delete*](struct.ResourceCalendarDeleteCall.html), [*calendars get*](struct.ResourceCalendarGetCall.html), [*calendars insert*](struct.ResourceCalendarInsertCall.html), [*calendars list*](struct.ResourceCalendarListCall.html), [*calendars patch*](struct.ResourceCalendarPatchCall.html) and [*calendars update*](struct.ResourceCalendarUpdateCall.html)
//! * [role assignments](struct.RoleAssignment.html)
//!  * [*delete*](struct.RoleAssignmentDeleteCall.html), [*get*](struct.RoleAssignmentGetCall.html), [*insert*](struct.RoleAssignmentInsertCall.html) and [*list*](struct.RoleAssignmentListCall.html)
//! * [roles](struct.Role.html)
//!  * [*delete*](struct.RoleDeleteCall.html), [*get*](struct.RoleGetCall.html), [*insert*](struct.RoleInsertCall.html), [*list*](struct.RoleListCall.html), [*patch*](struct.RolePatchCall.html) and [*update*](struct.RoleUpdateCall.html)
//! * [schemas](struct.Schema.html)
//!  * [*delete*](struct.SchemaDeleteCall.html), [*get*](struct.SchemaGetCall.html), [*insert*](struct.SchemaInsertCall.html), [*list*](struct.SchemaListCall.html), [*patch*](struct.SchemaPatchCall.html) and [*update*](struct.SchemaUpdateCall.html)
//! * [tokens](struct.Token.html)
//!  * [*delete*](struct.TokenDeleteCall.html), [*get*](struct.TokenGetCall.html) and [*list*](struct.TokenListCall.html)
//! * [users](struct.User.html)
//!  * [*aliases delete*](struct.UserAliaseDeleteCall.html), [*aliases insert*](struct.UserAliaseInsertCall.html), [*aliases list*](struct.UserAliaseListCall.html), [*aliases watch*](struct.UserAliaseWatchCall.html), [*delete*](struct.UserDeleteCall.html), [*get*](struct.UserGetCall.html), [*insert*](struct.UserInsertCall.html), [*list*](struct.UserListCall.html), [*make admin*](struct.UserMakeAdminCall.html), [*patch*](struct.UserPatchCall.html), [*photos delete*](struct.UserPhotoDeleteCall.html), [*photos get*](struct.UserPhotoGetCall.html), [*photos patch*](struct.UserPhotoPatchCall.html), [*photos update*](struct.UserPhotoUpdateCall.html), [*undelete*](struct.UserUndeleteCall.html), [*update*](struct.UserUpdateCall.html) and [*watch*](struct.UserWatchCall.html)
//! * [verification codes](struct.VerificationCode.html)
//!  * [*generate*](struct.VerificationCodeGenerateCall.html), [*invalidate*](struct.VerificationCodeInvalidateCall.html) and [*list*](struct.VerificationCodeListCall.html)
//! 
//! 
//! Subscription supported by ...
//! 
//! * [*list users*](struct.UserListCall.html)
//! * [*watch users*](struct.UserWatchCall.html)
//! * [*aliases watch users*](struct.UserAliaseWatchCall.html)
//! * [*aliases list users*](struct.UserAliaseListCall.html)
//! * [*aliases list groups*](struct.GroupAliaseListCall.html)
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](../index.html).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.Directory.html)**
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
//! let r = hub.users().photos_patch(...).doit()
//! let r = hub.users().aliases_delete(...).doit()
//! let r = hub.users().undelete(...).doit()
//! let r = hub.users().photos_get(...).doit()
//! let r = hub.users().update(...).doit()
//! let r = hub.users().aliases_watch(...).doit()
//! let r = hub.users().insert(...).doit()
//! let r = hub.users().photos_delete(...).doit()
//! let r = hub.users().patch(...).doit()
//! let r = hub.users().photos_update(...).doit()
//! let r = hub.users().watch(...).doit()
//! let r = hub.users().get(...).doit()
//! let r = hub.users().aliases_insert(...).doit()
//! let r = hub.users().make_admin(...).doit()
//! let r = hub.users().aliases_list(...).doit()
//! let r = hub.users().list(...).doit()
//! let r = hub.users().delete(...).doit()
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
//! google-admin1_directory = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_admin1_directory as admin1_directory;
//! use admin1_directory::Channel;
//! use admin1_directory::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use admin1_directory::Directory;
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
//! let mut hub = Directory::new(hyper::Client::new(), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = Channel::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.users().watch(req)
//!              .view_type("et")
//!              .sort_order("sadipscing")
//!              .show_deleted("accusam")
//!              .query("magna")
//!              .projection("Lorem")
//!              .page_token("rebum.")
//!              .order_by("et")
//!              .max_results(-64)
//!              .event("eos")
//!              .domain("dolores")
//!              .customer("vero")
//!              .custom_field_mask("consetetur")
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
#![cfg_attr(feature = "nightly", feature(custom_derive, custom_attribute, plugin))]
#![cfg_attr(feature = "nightly", plugin(serde_macros))]
#![allow(unused_imports, unused_mut, dead_code)]

#[cfg(feature = "nightly")]
include!("lib.rs.in");

#[cfg(feature = "with-syntex")]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));