// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *calendar* crate version *1.0.3+20161211*, where *20161211* is the exact revision of the *calendar:v3* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.3*.
//! 
//! Everything else about the *calendar* *v3* API can be found at the
//! [official documentation site](https://developers.google.com/google-apps/calendar/firstapp).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/calendar3).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.CalendarHub.html) ... 
//! 
//! * [acl](struct.Acl.html)
//!  * [*delete*](struct.AclDeleteCall.html), [*get*](struct.AclGetCall.html), [*insert*](struct.AclInsertCall.html), [*list*](struct.AclListCall.html), [*patch*](struct.AclPatchCall.html), [*update*](struct.AclUpdateCall.html) and [*watch*](struct.AclWatchCall.html)
//! * [calendar list](struct.CalendarList.html)
//!  * [*delete*](struct.CalendarListDeleteCall.html), [*get*](struct.CalendarListGetCall.html), [*insert*](struct.CalendarListInsertCall.html), [*list*](struct.CalendarListListCall.html), [*patch*](struct.CalendarListPatchCall.html), [*update*](struct.CalendarListUpdateCall.html) and [*watch*](struct.CalendarListWatchCall.html)
//! * [calendars](struct.Calendar.html)
//!  * [*clear*](struct.CalendarClearCall.html), [*delete*](struct.CalendarDeleteCall.html), [*get*](struct.CalendarGetCall.html), [*insert*](struct.CalendarInsertCall.html), [*patch*](struct.CalendarPatchCall.html) and [*update*](struct.CalendarUpdateCall.html)
//! * [channels](struct.Channel.html)
//!  * [*stop*](struct.ChannelStopCall.html)
//! * colors
//!  * [*get*](struct.ColorGetCall.html)
//! * [events](struct.Event.html)
//!  * [*delete*](struct.EventDeleteCall.html), [*get*](struct.EventGetCall.html), [*import*](struct.EventImportCall.html), [*insert*](struct.EventInsertCall.html), [*instances*](struct.EventInstanceCall.html), [*list*](struct.EventListCall.html), [*move*](struct.EventMoveCall.html), [*patch*](struct.EventPatchCall.html), [*quick add*](struct.EventQuickAddCall.html), [*update*](struct.EventUpdateCall.html) and [*watch*](struct.EventWatchCall.html)
//! * freebusy
//!  * [*query*](struct.FreebusyQueryCall.html)
//! * [settings](struct.Setting.html)
//!  * [*get*](struct.SettingGetCall.html), [*list*](struct.SettingListCall.html) and [*watch*](struct.SettingWatchCall.html)
//! 
//! 
//! Subscription supported by ...
//! 
//! * [*list settings*](struct.SettingListCall.html)
//! * [*list events*](struct.EventListCall.html)
//! * [*list calendar list*](struct.CalendarListListCall.html)
//! * [*watch events*](struct.EventWatchCall.html)
//! * [*instances events*](struct.EventInstanceCall.html)
//! * [*watch settings*](struct.SettingWatchCall.html)
//! * [*watch acl*](struct.AclWatchCall.html)
//! * [*list acl*](struct.AclListCall.html)
//! * [*watch calendar list*](struct.CalendarListWatchCall.html)
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.CalendarHub.html)**
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
//! let r = hub.events().delete(...).doit()
//! let r = hub.events().insert(...).doit()
//! let r = hub.events().instances(...).doit()
//! let r = hub.events().quick_add(...).doit()
//! let r = hub.events().patch(...).doit()
//! let r = hub.events().import(...).doit()
//! let r = hub.events().move_(...).doit()
//! let r = hub.events().update(...).doit()
//! let r = hub.events().watch(...).doit()
//! let r = hub.events().get(...).doit()
//! let r = hub.events().list(...).doit()
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
//! google-calendar3 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_calendar3 as calendar3;
//! use calendar3::Channel;
//! use calendar3::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use calendar3::CalendarHub;
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
//! let mut hub = CalendarHub::new(hyper::Client::new(), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = Channel::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.events().watch(req, "calendarId")
//!              .updated_min("dolor")
//!              .time_zone("sea")
//!              .time_min("ut")
//!              .time_max("eirmod")
//!              .sync_token("sanctus")
//!              .single_events(true)
//!              .show_hidden_invitations(false)
//!              .show_deleted(true)
//!              .add_shared_extended_property("et")
//!              .q("vero")
//!              .add_private_extended_property("ut")
//!              .page_token("sed")
//!              .order_by("et")
//!              .max_results(-55)
//!              .max_attendees(-20)
//!              .i_cal_uid("dolore")
//!              .always_include_email(true)
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
//! [Hub Delegate](trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).
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