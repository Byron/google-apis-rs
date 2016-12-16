// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *SQL Admin* crate version *1.0.0+20161213*, where *20161213* is the exact revision of the *sqladmin:v1beta4* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.0*.
//! 
//! Everything else about the *SQL Admin* *v1_beta4* API can be found at the
//! [official documentation site](https://cloud.google.com/sql/docs/reference/latest).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/sqladmin1_beta4).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.SQLAdmin.html) ... 
//! 
//! * [backup runs](struct.BackupRun.html)
//!  * [*delete*](struct.BackupRunDeleteCall.html), [*get*](struct.BackupRunGetCall.html), [*insert*](struct.BackupRunInsertCall.html) and [*list*](struct.BackupRunListCall.html)
//! * [databases](struct.Database.html)
//!  * [*delete*](struct.DatabaseDeleteCall.html), [*get*](struct.DatabaseGetCall.html), [*insert*](struct.DatabaseInsertCall.html), [*list*](struct.DatabaseListCall.html), [*patch*](struct.DatabasePatchCall.html) and [*update*](struct.DatabaseUpdateCall.html)
//! * [flags](struct.Flag.html)
//!  * [*list*](struct.FlagListCall.html)
//! * instances
//!  * [*clone*](struct.InstanceCloneCall.html), [*delete*](struct.InstanceDeleteCall.html), [*export*](struct.InstanceExportCall.html), [*failover*](struct.InstanceFailoverCall.html), [*get*](struct.InstanceGetCall.html), [*import*](struct.InstanceImportCall.html), [*insert*](struct.InstanceInsertCall.html), [*list*](struct.InstanceListCall.html), [*patch*](struct.InstancePatchCall.html), [*promote replica*](struct.InstancePromoteReplicaCall.html), [*reset ssl config*](struct.InstanceResetSslConfigCall.html), [*restart*](struct.InstanceRestartCall.html), [*restore backup*](struct.InstanceRestoreBackupCall.html), [*start replica*](struct.InstanceStartReplicaCall.html), [*stop replica*](struct.InstanceStopReplicaCall.html), [*truncate log*](struct.InstanceTruncateLogCall.html) and [*update*](struct.InstanceUpdateCall.html)
//! * [operations](struct.Operation.html)
//!  * [*get*](struct.OperationGetCall.html) and [*list*](struct.OperationListCall.html)
//! * [ssl certs](struct.SslCert.html)
//!  * [*create ephemeral*](struct.SslCertCreateEphemeralCall.html), [*delete*](struct.SslCertDeleteCall.html), [*get*](struct.SslCertGetCall.html), [*insert*](struct.SslCertInsertCall.html) and [*list*](struct.SslCertListCall.html)
//! * [tiers](struct.Tier.html)
//!  * [*list*](struct.TierListCall.html)
//! * [users](struct.User.html)
//!  * [*delete*](struct.UserDeleteCall.html), [*insert*](struct.UserInsertCall.html), [*list*](struct.UserListCall.html) and [*update*](struct.UserUpdateCall.html)
//! 
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](../index.html).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.SQLAdmin.html)**
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
//! let r = hub.ssl_certs().delete(...).doit()
//! let r = hub.instances().truncate_log(...).doit()
//! let r = hub.users().delete(...).doit()
//! let r = hub.databases().delete(...).doit()
//! let r = hub.instances().failover(...).doit()
//! let r = hub.databases().patch(...).doit()
//! let r = hub.instances().reset_ssl_config(...).doit()
//! let r = hub.instances().promote_replica(...).doit()
//! let r = hub.databases().update(...).doit()
//! let r = hub.operations().list(...).doit()
//! let r = hub.users().update(...).doit()
//! let r = hub.databases().insert(...).doit()
//! let r = hub.backup_runs().delete(...).doit()
//! let r = hub.instances().patch(...).doit()
//! let r = hub.instances().clone(...).doit()
//! let r = hub.instances().delete(...).doit()
//! let r = hub.operations().get(...).doit()
//! let r = hub.instances().stop_replica(...).doit()
//! let r = hub.instances().start_replica(...).doit()
//! let r = hub.users().insert(...).doit()
//! let r = hub.instances().insert(...).doit()
//! let r = hub.instances().import(...).doit()
//! let r = hub.backup_runs().insert(...).doit()
//! let r = hub.instances().update(...).doit()
//! let r = hub.instances().restart(...).doit()
//! let r = hub.instances().export(...).doit()
//! let r = hub.instances().restore_backup(...).doit()
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
//! google-sqladmin1_beta4 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_sqladmin1_beta4 as sqladmin1_beta4;
//! use sqladmin1_beta4::User;
//! use sqladmin1_beta4::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use sqladmin1_beta4::SQLAdmin;
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
//! let mut hub = SQLAdmin::new(hyper::Client::new(), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = User::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.users().update(req, "project", "instance", "host", "name")
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