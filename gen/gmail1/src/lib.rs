// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *gmail* crate version *0.1.14+20160316*, where *20160316* is the exact revision of the *gmail:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.14*.
//! 
//! Everything else about the *gmail* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/gmail/api/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/gmail1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Gmail.html) ... 
//! 
//! * users
//!  * [*drafts create*](struct.UserDraftCreateCall.html), [*drafts delete*](struct.UserDraftDeleteCall.html), [*drafts get*](struct.UserDraftGetCall.html), [*drafts list*](struct.UserDraftListCall.html), [*drafts send*](struct.UserDraftSendCall.html), [*drafts update*](struct.UserDraftUpdateCall.html), [*get profile*](struct.UserGetProfileCall.html), [*history list*](struct.UserHistoryListCall.html), [*labels create*](struct.UserLabelCreateCall.html), [*labels delete*](struct.UserLabelDeleteCall.html), [*labels get*](struct.UserLabelGetCall.html), [*labels list*](struct.UserLabelListCall.html), [*labels patch*](struct.UserLabelPatchCall.html), [*labels update*](struct.UserLabelUpdateCall.html), [*messages attachments get*](struct.UserMessageAttachmentGetCall.html), [*messages batch delete*](struct.UserMessageBatchDeleteCall.html), [*messages delete*](struct.UserMessageDeleteCall.html), [*messages get*](struct.UserMessageGetCall.html), [*messages import*](struct.UserMessageImportCall.html), [*messages insert*](struct.UserMessageInsertCall.html), [*messages list*](struct.UserMessageListCall.html), [*messages modify*](struct.UserMessageModifyCall.html), [*messages send*](struct.UserMessageSendCall.html), [*messages trash*](struct.UserMessageTrashCall.html), [*messages untrash*](struct.UserMessageUntrashCall.html), [*stop*](struct.UserStopCall.html), [*threads delete*](struct.UserThreadDeleteCall.html), [*threads get*](struct.UserThreadGetCall.html), [*threads list*](struct.UserThreadListCall.html), [*threads modify*](struct.UserThreadModifyCall.html), [*threads trash*](struct.UserThreadTrashCall.html), [*threads untrash*](struct.UserThreadUntrashCall.html) and [*watch*](struct.UserWatchCall.html)
//! 
//! 
//! Upload supported by ...
//! 
//! * [*messages import users*](struct.UserMessageImportCall.html)
//! * [*drafts create users*](struct.UserDraftCreateCall.html)
//! * [*drafts send users*](struct.UserDraftSendCall.html)
//! * [*drafts update users*](struct.UserDraftUpdateCall.html)
//! * [*messages send users*](struct.UserMessageSendCall.html)
//! * [*messages insert users*](struct.UserMessageInsertCall.html)
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](../index.html).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.Gmail.html)**
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
//! let r = hub.users().messages_untrash(...).doit()
//! let r = hub.users().messages_get(...).doit()
//! let r = hub.users().messages_modify(...).doit()
//! let r = hub.users().messages_import(...).doit()
//! let r = hub.users().messages_insert(...).doit()
//! let r = hub.users().messages_send(...).doit()
//! let r = hub.users().messages_trash(...).doit()
//! let r = hub.users().drafts_send(...).doit()
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
//! google-gmail1 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_gmail1 as gmail1;
//! use gmail1::Message;
//! use gmail1::{Result, Error};
//! use std::fs;
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use gmail1::Gmail;
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
//! let mut hub = Gmail::new(hyper::Client::new(), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = Message::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `upload(...)`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.users().messages_import(req, "userId")
//!              .process_for_calendar(true)
//!              .never_mark_spam(false)
//!              .internal_date_source("At")
//!              .deleted(false)
//!              .upload(fs::File::open("file.ext").unwrap(), "application/octet-stream".parse().unwrap());
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

#[cfg(feature = "with-serde-codegen")]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));