// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Tag Manager* crate version *0.1.12+20160209*, where *20160209* is the exact revision of the *tagmanager:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.12*.
//! 
//! Everything else about the *Tag Manager* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/tag-manager/api/v1/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/tagmanager1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.TagManager.html) ... 
//! 
//! * [accounts](struct.Account.html)
//!  * [*containers create*](struct.AccountContainerCreateCall.html), [*containers delete*](struct.AccountContainerDeleteCall.html), [*containers environments create*](struct.AccountContainerEnvironmentCreateCall.html), [*containers environments delete*](struct.AccountContainerEnvironmentDeleteCall.html), [*containers environments get*](struct.AccountContainerEnvironmentGetCall.html), [*containers environments list*](struct.AccountContainerEnvironmentListCall.html), [*containers environments patch*](struct.AccountContainerEnvironmentPatchCall.html), [*containers environments update*](struct.AccountContainerEnvironmentUpdateCall.html), [*containers folders create*](struct.AccountContainerFolderCreateCall.html), [*containers folders delete*](struct.AccountContainerFolderDeleteCall.html), [*containers folders entities list*](struct.AccountContainerFolderEntityListCall.html), [*containers folders get*](struct.AccountContainerFolderGetCall.html), [*containers folders list*](struct.AccountContainerFolderListCall.html), [*containers folders update*](struct.AccountContainerFolderUpdateCall.html), [*containers get*](struct.AccountContainerGetCall.html), [*containers list*](struct.AccountContainerListCall.html), [*containers move_folders update*](struct.AccountContainerMoveFolderUpdateCall.html), [*containers reauthorize_environments update*](struct.AccountContainerReauthorizeEnvironmentUpdateCall.html), [*containers tags create*](struct.AccountContainerTagCreateCall.html), [*containers tags delete*](struct.AccountContainerTagDeleteCall.html), [*containers tags get*](struct.AccountContainerTagGetCall.html), [*containers tags list*](struct.AccountContainerTagListCall.html), [*containers tags update*](struct.AccountContainerTagUpdateCall.html), [*containers triggers create*](struct.AccountContainerTriggerCreateCall.html), [*containers triggers delete*](struct.AccountContainerTriggerDeleteCall.html), [*containers triggers get*](struct.AccountContainerTriggerGetCall.html), [*containers triggers list*](struct.AccountContainerTriggerListCall.html), [*containers triggers update*](struct.AccountContainerTriggerUpdateCall.html), [*containers update*](struct.AccountContainerUpdateCall.html), [*containers variables create*](struct.AccountContainerVariableCreateCall.html), [*containers variables delete*](struct.AccountContainerVariableDeleteCall.html), [*containers variables get*](struct.AccountContainerVariableGetCall.html), [*containers variables list*](struct.AccountContainerVariableListCall.html), [*containers variables update*](struct.AccountContainerVariableUpdateCall.html), [*containers versions create*](struct.AccountContainerVersionCreateCall.html), [*containers versions delete*](struct.AccountContainerVersionDeleteCall.html), [*containers versions get*](struct.AccountContainerVersionGetCall.html), [*containers versions list*](struct.AccountContainerVersionListCall.html), [*containers versions publish*](struct.AccountContainerVersionPublishCall.html), [*containers versions restore*](struct.AccountContainerVersionRestoreCall.html), [*containers versions undelete*](struct.AccountContainerVersionUndeleteCall.html), [*containers versions update*](struct.AccountContainerVersionUpdateCall.html), [*get*](struct.AccountGetCall.html), [*list*](struct.AccountListCall.html), [*permissions create*](struct.AccountPermissionCreateCall.html), [*permissions delete*](struct.AccountPermissionDeleteCall.html), [*permissions get*](struct.AccountPermissionGetCall.html), [*permissions list*](struct.AccountPermissionListCall.html), [*permissions update*](struct.AccountPermissionUpdateCall.html) and [*update*](struct.AccountUpdateCall.html)
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
//! * **[Hub](struct.TagManager.html)**
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
//! let r = hub.accounts().containers_list(...).doit()
//! let r = hub.accounts().permissions_list(...).doit()
//! let r = hub.accounts().containers_environments_create(...).doit()
//! let r = hub.accounts().permissions_create(...).doit()
//! let r = hub.accounts().containers_environments_delete(...).doit()
//! let r = hub.accounts().permissions_delete(...).doit()
//! let r = hub.accounts().containers_get(...).doit()
//! let r = hub.accounts().containers_environments_patch(...).doit()
//! let r = hub.accounts().containers_versions_list(...).doit()
//! let r = hub.accounts().containers_triggers_update(...).doit()
//! let r = hub.accounts().containers_triggers_get(...).doit()
//! let r = hub.accounts().containers_delete(...).doit()
//! let r = hub.accounts().containers_folders_delete(...).doit()
//! let r = hub.accounts().containers_create(...).doit()
//! let r = hub.accounts().containers_tags_delete(...).doit()
//! let r = hub.accounts().containers_folders_entities_list(...).doit()
//! let r = hub.accounts().containers_versions_undelete(...).doit()
//! let r = hub.accounts().containers_environments_list(...).doit()
//! let r = hub.accounts().containers_tags_list(...).doit()
//! let r = hub.accounts().containers_versions_publish(...).doit()
//! let r = hub.accounts().containers_folders_get(...).doit()
//! let r = hub.accounts().containers_environments_get(...).doit()
//! let r = hub.accounts().containers_tags_create(...).doit()
//! let r = hub.accounts().containers_triggers_list(...).doit()
//! let r = hub.accounts().containers_versions_delete(...).doit()
//! let r = hub.accounts().update(...).doit()
//! let r = hub.accounts().containers_versions_create(...).doit()
//! let r = hub.accounts().permissions_get(...).doit()
//! let r = hub.accounts().containers_move_folders_update(...).doit()
//! let r = hub.accounts().containers_versions_restore(...).doit()
//! let r = hub.accounts().containers_variables_create(...).doit()
//! let r = hub.accounts().containers_variables_list(...).doit()
//! let r = hub.accounts().containers_tags_get(...).doit()
//! let r = hub.accounts().containers_variables_get(...).doit()
//! let r = hub.accounts().containers_triggers_delete(...).doit()
//! let r = hub.accounts().containers_triggers_create(...).doit()
//! let r = hub.accounts().containers_folders_update(...).doit()
//! let r = hub.accounts().list(...).doit()
//! let r = hub.accounts().permissions_update(...).doit()
//! let r = hub.accounts().containers_variables_delete(...).doit()
//! let r = hub.accounts().containers_reauthorize_environments_update(...).doit()
//! let r = hub.accounts().get(...).doit()
//! let r = hub.accounts().containers_update(...).doit()
//! let r = hub.accounts().containers_tags_update(...).doit()
//! let r = hub.accounts().containers_environments_update(...).doit()
//! let r = hub.accounts().containers_folders_create(...).doit()
//! let r = hub.accounts().containers_folders_list(...).doit()
//! let r = hub.accounts().containers_versions_update(...).doit()
//! let r = hub.accounts().containers_variables_update(...).doit()
//! let r = hub.accounts().containers_versions_get(...).doit()
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
//! google-tagmanager1 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_tagmanager1 as tagmanager1;
//! use tagmanager1::Folder;
//! use tagmanager1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use tagmanager1::TagManager;
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
//! let mut hub = TagManager::new(hyper::Client::new(), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = Folder::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.accounts().containers_move_folders_update(req, "accountId", "containerId", "folderId")
//!              .add_variable_id("clita")
//!              .add_trigger_id("invidunt")
//!              .add_tag_id("eirmod")
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