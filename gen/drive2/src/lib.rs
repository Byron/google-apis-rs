// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *drive* crate version *0.1.15+20160901*, where *20160901* is the exact revision of the *drive:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.15*.
//! 
//! Everything else about the *drive* *v2* API can be found at the
//! [official documentation site](https://developers.google.com/drive/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/drive2).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Drive.html) ... 
//! 
//! * [about](struct.About.html)
//!  * [*get*](struct.AboutGetCall.html)
//! * [apps](struct.App.html)
//!  * [*get*](struct.AppGetCall.html) and [*list*](struct.AppListCall.html)
//! * [changes](struct.Change.html)
//!  * [*get*](struct.ChangeGetCall.html), [*list*](struct.ChangeListCall.html) and [*watch*](struct.ChangeWatchCall.html)
//! * [channels](struct.Channel.html)
//!  * [*stop*](struct.ChannelStopCall.html)
//! * children
//!  * [*delete*](struct.ChildrenDeleteCall.html), [*get*](struct.ChildrenGetCall.html), [*insert*](struct.ChildrenInsertCall.html) and [*list*](struct.ChildrenListCall.html)
//! * [comments](struct.Comment.html)
//!  * [*delete*](struct.CommentDeleteCall.html), [*get*](struct.CommentGetCall.html), [*insert*](struct.CommentInsertCall.html), [*list*](struct.CommentListCall.html), [*patch*](struct.CommentPatchCall.html) and [*update*](struct.CommentUpdateCall.html)
//! * [files](struct.File.html)
//!  * [*copy*](struct.FileCopyCall.html), [*delete*](struct.FileDeleteCall.html), [*empty trash*](struct.FileEmptyTrashCall.html), [*export*](struct.FileExportCall.html), [*generate ids*](struct.FileGenerateIdCall.html), [*get*](struct.FileGetCall.html), [*insert*](struct.FileInsertCall.html), [*list*](struct.FileListCall.html), [*patch*](struct.FilePatchCall.html), [*touch*](struct.FileTouchCall.html), [*trash*](struct.FileTrashCall.html), [*untrash*](struct.FileUntrashCall.html), [*update*](struct.FileUpdateCall.html) and [*watch*](struct.FileWatchCall.html)
//! * parents
//!  * [*delete*](struct.ParentDeleteCall.html), [*get*](struct.ParentGetCall.html), [*insert*](struct.ParentInsertCall.html) and [*list*](struct.ParentListCall.html)
//! * [permissions](struct.Permission.html)
//!  * [*delete*](struct.PermissionDeleteCall.html), [*get*](struct.PermissionGetCall.html), [*get id for email*](struct.PermissionGetIdForEmailCall.html), [*insert*](struct.PermissionInsertCall.html), [*list*](struct.PermissionListCall.html), [*patch*](struct.PermissionPatchCall.html) and [*update*](struct.PermissionUpdateCall.html)
//! * [properties](struct.Property.html)
//!  * [*delete*](struct.PropertyDeleteCall.html), [*get*](struct.PropertyGetCall.html), [*insert*](struct.PropertyInsertCall.html), [*list*](struct.PropertyListCall.html), [*patch*](struct.PropertyPatchCall.html) and [*update*](struct.PropertyUpdateCall.html)
//! * realtime
//!  * [*get*](struct.RealtimeGetCall.html) and [*update*](struct.RealtimeUpdateCall.html)
//! * replies
//!  * [*delete*](struct.ReplyDeleteCall.html), [*get*](struct.ReplyGetCall.html), [*insert*](struct.ReplyInsertCall.html), [*list*](struct.ReplyListCall.html), [*patch*](struct.ReplyPatchCall.html) and [*update*](struct.ReplyUpdateCall.html)
//! * [revisions](struct.Revision.html)
//!  * [*delete*](struct.RevisionDeleteCall.html), [*get*](struct.RevisionGetCall.html), [*list*](struct.RevisionListCall.html), [*patch*](struct.RevisionPatchCall.html) and [*update*](struct.RevisionUpdateCall.html)
//! 
//! 
//! Upload supported by ...
//! 
//! * [*insert files*](struct.FileInsertCall.html)
//! * [*update realtime*](struct.RealtimeUpdateCall.html)
//! * [*update files*](struct.FileUpdateCall.html)
//! 
//! Download supported by ...
//! 
//! * [*watch files*](struct.FileWatchCall.html)
//! * [*get realtime*](struct.RealtimeGetCall.html)
//! * [*export files*](struct.FileExportCall.html)
//! * [*get files*](struct.FileGetCall.html)
//! 
//! Subscription supported by ...
//! 
//! * [*watch files*](struct.FileWatchCall.html)
//! * [*watch changes*](struct.ChangeWatchCall.html)
//! * [*insert files*](struct.FileInsertCall.html)
//! * [*list changes*](struct.ChangeListCall.html)
//! * [*get files*](struct.FileGetCall.html)
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](../index.html).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.Drive.html)**
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
//! let r = hub.files().watch(...).doit()
//! let r = hub.files().empty_trash(...).doit()
//! let r = hub.files().generate_ids(...).doit()
//! let r = hub.files().copy(...).doit()
//! let r = hub.files().list(...).doit()
//! let r = hub.files().delete(...).doit()
//! let r = hub.files().patch(...).doit()
//! let r = hub.files().update(...).doit()
//! let r = hub.files().insert(...).doit()
//! let r = hub.files().untrash(...).doit()
//! let r = hub.files().trash(...).doit()
//! let r = hub.files().touch(...).doit()
//! let r = hub.files().get(...).doit()
//! let r = hub.files().export(...).doit()
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
//! google-drive2 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_drive2 as drive2;
//! use drive2::File;
//! use drive2::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use drive2::Drive;
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
//! let mut hub = Drive::new(hyper::Client::new(), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = File::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.files().patch(req, "fileId")
//!              .use_content_as_indexable_text(false)
//!              .update_viewed_date(true)
//!              .timed_text_track_name("dolore")
//!              .timed_text_language("Lorem")
//!              .set_modified_date(false)
//!              .remove_parents("consetetur")
//!              .pinned(false)
//!              .ocr_language("labore")
//!              .ocr(false)
//!              .new_revision(false)
//!              .modified_date_behavior("sadipscing")
//!              .convert(false)
//!              .add_parents("magna")
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

#[cfg(feature = "with-serde-codegen")]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));