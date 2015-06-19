// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *storage* crate version *0.1.8+20150326*, where *20150326* is the exact revision of the *storage:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.8*.
//! 
//! Everything else about the *storage* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/storage/docs/json_api/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/storage1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Storage.html) ... 
//! 
//! * [bucket access controls](struct.BucketAccessControl.html)
//!  * [*delete*](struct.BucketAccessControlDeleteCall.html), [*get*](struct.BucketAccessControlGetCall.html), [*insert*](struct.BucketAccessControlInsertCall.html), [*list*](struct.BucketAccessControlListCall.html), [*patch*](struct.BucketAccessControlPatchCall.html) and [*update*](struct.BucketAccessControlUpdateCall.html)
//! * [buckets](struct.Bucket.html)
//!  * [*delete*](struct.BucketDeleteCall.html), [*get*](struct.BucketGetCall.html), [*insert*](struct.BucketInsertCall.html), [*list*](struct.BucketListCall.html), [*patch*](struct.BucketPatchCall.html) and [*update*](struct.BucketUpdateCall.html)
//! * [channels](struct.Channel.html)
//!  * [*stop*](struct.ChannelStopCall.html)
//! * default object access controls
//!  * [*delete*](struct.DefaultObjectAccessControlDeleteCall.html), [*get*](struct.DefaultObjectAccessControlGetCall.html), [*insert*](struct.DefaultObjectAccessControlInsertCall.html), [*list*](struct.DefaultObjectAccessControlListCall.html), [*patch*](struct.DefaultObjectAccessControlPatchCall.html) and [*update*](struct.DefaultObjectAccessControlUpdateCall.html)
//! * [object access controls](struct.ObjectAccessControl.html)
//!  * [*delete*](struct.ObjectAccessControlDeleteCall.html), [*get*](struct.ObjectAccessControlGetCall.html), [*insert*](struct.ObjectAccessControlInsertCall.html), [*list*](struct.ObjectAccessControlListCall.html), [*patch*](struct.ObjectAccessControlPatchCall.html) and [*update*](struct.ObjectAccessControlUpdateCall.html)
//! * [objects](struct.Object.html)
//!  * [*compose*](struct.ObjectComposeCall.html), [*copy*](struct.ObjectCopyCall.html), [*delete*](struct.ObjectDeleteCall.html), [*get*](struct.ObjectGetCall.html), [*insert*](struct.ObjectInsertCall.html), [*list*](struct.ObjectListCall.html), [*patch*](struct.ObjectPatchCall.html), [*rewrite*](struct.ObjectRewriteCall.html), [*update*](struct.ObjectUpdateCall.html) and [*watch all*](struct.ObjectWatchAllCall.html)
//! 
//! 
//! Upload supported by ...
//! 
//! * [*insert objects*](struct.ObjectInsertCall.html)
//! 
//! Download supported by ...
//! 
//! * [*get objects*](struct.ObjectGetCall.html)
//! * [*update objects*](struct.ObjectUpdateCall.html)
//! * [*insert objects*](struct.ObjectInsertCall.html)
//! * [*compose objects*](struct.ObjectComposeCall.html)
//! * [*copy objects*](struct.ObjectCopyCall.html)
//! 
//! Subscription supported by ...
//! 
//! * [*watch all objects*](struct.ObjectWatchAllCall.html)
//! * [*list objects*](struct.ObjectListCall.html)
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](../index.html).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.Storage.html)**
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
//! let r = hub.objects().list(...).doit()
//! let r = hub.objects().rewrite(...).doit()
//! let r = hub.objects().copy(...).doit()
//! let r = hub.objects().watch_all(...).doit()
//! let r = hub.objects().get(...).doit()
//! let r = hub.objects().insert(...).doit()
//! let r = hub.objects().compose(...).doit()
//! let r = hub.objects().update(...).doit()
//! let r = hub.objects().delete(...).doit()
//! let r = hub.objects().patch(...).doit()
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
//! google-storage1 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_storage1 as storage1;
//! use storage1::Object;
//! use storage1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use storage1::Storage;
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
//! let mut hub = Storage::new(hyper::Client::new(), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = Object::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.objects().rewrite(req, "sourceBucket", "sourceObject", "destinationBucket", "destinationObject")
//!              .source_generation("accusam")
//!              .rewrite_token("dolores")
//!              .projection("consetetur")
//!              .max_bytes_rewritten_per_call("dolor")
//!              .if_source_metageneration_not_match("aliquyam")
//!              .if_source_metageneration_match("elitr")
//!              .if_source_generation_not_match("ea")
//!              .if_source_generation_match("et")
//!              .if_metageneration_not_match("Stet")
//!              .if_metageneration_match("sed")
//!              .if_generation_not_match("dolor")
//!              .if_generation_match("sanctus")
//!              .destination_predefined_acl("dolore")
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
#![allow(unused_imports, unused_mut, dead_code)]

include!(concat!(env!("OUT_DIR"), "/lib.rs"));