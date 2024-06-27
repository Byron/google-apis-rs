// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Tag Manager* crate version *5.0.5+20240619*, where *20240619* is the exact revision of the *tagmanager:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.
//! 
//! Everything else about the *Tag Manager* *v2* API can be found at the
//! [official documentation site](https://developers.google.com/tag-manager).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/tagmanager2).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](TagManager) ... 
//! 
//! * [accounts](api::Account)
//!  * [*containers combine*](api::AccountContainerCombineCall), [*containers create*](api::AccountContainerCreateCall), [*containers delete*](api::AccountContainerDeleteCall), [*containers destinations get*](api::AccountContainerDestinationGetCall), [*containers destinations link*](api::AccountContainerDestinationLinkCall), [*containers destinations list*](api::AccountContainerDestinationListCall), [*containers environments create*](api::AccountContainerEnvironmentCreateCall), [*containers environments delete*](api::AccountContainerEnvironmentDeleteCall), [*containers environments get*](api::AccountContainerEnvironmentGetCall), [*containers environments list*](api::AccountContainerEnvironmentListCall), [*containers environments reauthorize*](api::AccountContainerEnvironmentReauthorizeCall), [*containers environments update*](api::AccountContainerEnvironmentUpdateCall), [*containers get*](api::AccountContainerGetCall), [*containers list*](api::AccountContainerListCall), [*containers lookup*](api::AccountContainerLookupCall), [*containers move_tag_id*](api::AccountContainerMoveTagIdCall), [*containers snippet*](api::AccountContainerSnippetCall), [*containers update*](api::AccountContainerUpdateCall), [*containers version_headers latest*](api::AccountContainerVersionHeaderLatestCall), [*containers version_headers list*](api::AccountContainerVersionHeaderListCall), [*containers versions delete*](api::AccountContainerVersionDeleteCall), [*containers versions get*](api::AccountContainerVersionGetCall), [*containers versions live*](api::AccountContainerVersionLiveCall), [*containers versions publish*](api::AccountContainerVersionPublishCall), [*containers versions set_latest*](api::AccountContainerVersionSetLatestCall), [*containers versions undelete*](api::AccountContainerVersionUndeleteCall), [*containers versions update*](api::AccountContainerVersionUpdateCall), [*containers workspaces built_in_variables create*](api::AccountContainerWorkspaceBuiltInVariableCreateCall), [*containers workspaces built_in_variables delete*](api::AccountContainerWorkspaceBuiltInVariableDeleteCall), [*containers workspaces built_in_variables list*](api::AccountContainerWorkspaceBuiltInVariableListCall), [*containers workspaces built_in_variables revert*](api::AccountContainerWorkspaceBuiltInVariableRevertCall), [*containers workspaces clients create*](api::AccountContainerWorkspaceClientCreateCall), [*containers workspaces clients delete*](api::AccountContainerWorkspaceClientDeleteCall), [*containers workspaces clients get*](api::AccountContainerWorkspaceClientGetCall), [*containers workspaces clients list*](api::AccountContainerWorkspaceClientListCall), [*containers workspaces clients revert*](api::AccountContainerWorkspaceClientRevertCall), [*containers workspaces clients update*](api::AccountContainerWorkspaceClientUpdateCall), [*containers workspaces create*](api::AccountContainerWorkspaceCreateCall), [*containers workspaces create_version*](api::AccountContainerWorkspaceCreateVersionCall), [*containers workspaces delete*](api::AccountContainerWorkspaceDeleteCall), [*containers workspaces folders create*](api::AccountContainerWorkspaceFolderCreateCall), [*containers workspaces folders delete*](api::AccountContainerWorkspaceFolderDeleteCall), [*containers workspaces folders entities*](api::AccountContainerWorkspaceFolderEntityCall), [*containers workspaces folders get*](api::AccountContainerWorkspaceFolderGetCall), [*containers workspaces folders list*](api::AccountContainerWorkspaceFolderListCall), [*containers workspaces folders move_entities_to_folder*](api::AccountContainerWorkspaceFolderMoveEntitiesToFolderCall), [*containers workspaces folders revert*](api::AccountContainerWorkspaceFolderRevertCall), [*containers workspaces folders update*](api::AccountContainerWorkspaceFolderUpdateCall), [*containers workspaces get*](api::AccountContainerWorkspaceGetCall), [*containers workspaces get status*](api::AccountContainerWorkspaceGetStatuCall), [*containers workspaces gtag_config create*](api::AccountContainerWorkspaceGtagConfigCreateCall), [*containers workspaces gtag_config delete*](api::AccountContainerWorkspaceGtagConfigDeleteCall), [*containers workspaces gtag_config get*](api::AccountContainerWorkspaceGtagConfigGetCall), [*containers workspaces gtag_config list*](api::AccountContainerWorkspaceGtagConfigListCall), [*containers workspaces gtag_config update*](api::AccountContainerWorkspaceGtagConfigUpdateCall), [*containers workspaces list*](api::AccountContainerWorkspaceListCall), [*containers workspaces quick_preview*](api::AccountContainerWorkspaceQuickPreviewCall), [*containers workspaces resolve_conflict*](api::AccountContainerWorkspaceResolveConflictCall), [*containers workspaces sync*](api::AccountContainerWorkspaceSyncCall), [*containers workspaces tags create*](api::AccountContainerWorkspaceTagCreateCall), [*containers workspaces tags delete*](api::AccountContainerWorkspaceTagDeleteCall), [*containers workspaces tags get*](api::AccountContainerWorkspaceTagGetCall), [*containers workspaces tags list*](api::AccountContainerWorkspaceTagListCall), [*containers workspaces tags revert*](api::AccountContainerWorkspaceTagRevertCall), [*containers workspaces tags update*](api::AccountContainerWorkspaceTagUpdateCall), [*containers workspaces templates create*](api::AccountContainerWorkspaceTemplateCreateCall), [*containers workspaces templates delete*](api::AccountContainerWorkspaceTemplateDeleteCall), [*containers workspaces templates get*](api::AccountContainerWorkspaceTemplateGetCall), [*containers workspaces templates list*](api::AccountContainerWorkspaceTemplateListCall), [*containers workspaces templates revert*](api::AccountContainerWorkspaceTemplateRevertCall), [*containers workspaces templates update*](api::AccountContainerWorkspaceTemplateUpdateCall), [*containers workspaces transformations create*](api::AccountContainerWorkspaceTransformationCreateCall), [*containers workspaces transformations delete*](api::AccountContainerWorkspaceTransformationDeleteCall), [*containers workspaces transformations get*](api::AccountContainerWorkspaceTransformationGetCall), [*containers workspaces transformations list*](api::AccountContainerWorkspaceTransformationListCall), [*containers workspaces transformations revert*](api::AccountContainerWorkspaceTransformationRevertCall), [*containers workspaces transformations update*](api::AccountContainerWorkspaceTransformationUpdateCall), [*containers workspaces triggers create*](api::AccountContainerWorkspaceTriggerCreateCall), [*containers workspaces triggers delete*](api::AccountContainerWorkspaceTriggerDeleteCall), [*containers workspaces triggers get*](api::AccountContainerWorkspaceTriggerGetCall), [*containers workspaces triggers list*](api::AccountContainerWorkspaceTriggerListCall), [*containers workspaces triggers revert*](api::AccountContainerWorkspaceTriggerRevertCall), [*containers workspaces triggers update*](api::AccountContainerWorkspaceTriggerUpdateCall), [*containers workspaces update*](api::AccountContainerWorkspaceUpdateCall), [*containers workspaces variables create*](api::AccountContainerWorkspaceVariableCreateCall), [*containers workspaces variables delete*](api::AccountContainerWorkspaceVariableDeleteCall), [*containers workspaces variables get*](api::AccountContainerWorkspaceVariableGetCall), [*containers workspaces variables list*](api::AccountContainerWorkspaceVariableListCall), [*containers workspaces variables revert*](api::AccountContainerWorkspaceVariableRevertCall), [*containers workspaces variables update*](api::AccountContainerWorkspaceVariableUpdateCall), [*containers workspaces zones create*](api::AccountContainerWorkspaceZoneCreateCall), [*containers workspaces zones delete*](api::AccountContainerWorkspaceZoneDeleteCall), [*containers workspaces zones get*](api::AccountContainerWorkspaceZoneGetCall), [*containers workspaces zones list*](api::AccountContainerWorkspaceZoneListCall), [*containers workspaces zones revert*](api::AccountContainerWorkspaceZoneRevertCall), [*containers workspaces zones update*](api::AccountContainerWorkspaceZoneUpdateCall), [*get*](api::AccountGetCall), [*list*](api::AccountListCall), [*update*](api::AccountUpdateCall), [*user_permissions create*](api::AccountUserPermissionCreateCall), [*user_permissions delete*](api::AccountUserPermissionDeleteCall), [*user_permissions get*](api::AccountUserPermissionGetCall), [*user_permissions list*](api::AccountUserPermissionListCall) and [*user_permissions update*](api::AccountUserPermissionUpdateCall)
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
//! * **[Hub](TagManager)**
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
//! let r = hub.accounts().containers_destinations_get(...).doit().await
//! let r = hub.accounts().containers_destinations_link(...).doit().await
//! let r = hub.accounts().containers_destinations_list(...).doit().await
//! let r = hub.accounts().containers_environments_create(...).doit().await
//! let r = hub.accounts().containers_environments_delete(...).doit().await
//! let r = hub.accounts().containers_environments_get(...).doit().await
//! let r = hub.accounts().containers_environments_list(...).doit().await
//! let r = hub.accounts().containers_environments_reauthorize(...).doit().await
//! let r = hub.accounts().containers_environments_update(...).doit().await
//! let r = hub.accounts().containers_version_headers_latest(...).doit().await
//! let r = hub.accounts().containers_version_headers_list(...).doit().await
//! let r = hub.accounts().containers_versions_delete(...).doit().await
//! let r = hub.accounts().containers_versions_get(...).doit().await
//! let r = hub.accounts().containers_versions_live(...).doit().await
//! let r = hub.accounts().containers_versions_publish(...).doit().await
//! let r = hub.accounts().containers_versions_set_latest(...).doit().await
//! let r = hub.accounts().containers_versions_undelete(...).doit().await
//! let r = hub.accounts().containers_versions_update(...).doit().await
//! let r = hub.accounts().containers_workspaces_built_in_variables_create(...).doit().await
//! let r = hub.accounts().containers_workspaces_built_in_variables_delete(...).doit().await
//! let r = hub.accounts().containers_workspaces_built_in_variables_list(...).doit().await
//! let r = hub.accounts().containers_workspaces_built_in_variables_revert(...).doit().await
//! let r = hub.accounts().containers_workspaces_clients_create(...).doit().await
//! let r = hub.accounts().containers_workspaces_clients_delete(...).doit().await
//! let r = hub.accounts().containers_workspaces_clients_get(...).doit().await
//! let r = hub.accounts().containers_workspaces_clients_list(...).doit().await
//! let r = hub.accounts().containers_workspaces_clients_revert(...).doit().await
//! let r = hub.accounts().containers_workspaces_clients_update(...).doit().await
//! let r = hub.accounts().containers_workspaces_folders_create(...).doit().await
//! let r = hub.accounts().containers_workspaces_folders_delete(...).doit().await
//! let r = hub.accounts().containers_workspaces_folders_entities(...).doit().await
//! let r = hub.accounts().containers_workspaces_folders_get(...).doit().await
//! let r = hub.accounts().containers_workspaces_folders_list(...).doit().await
//! let r = hub.accounts().containers_workspaces_folders_move_entities_to_folder(...).doit().await
//! let r = hub.accounts().containers_workspaces_folders_revert(...).doit().await
//! let r = hub.accounts().containers_workspaces_folders_update(...).doit().await
//! let r = hub.accounts().containers_workspaces_gtag_config_create(...).doit().await
//! let r = hub.accounts().containers_workspaces_gtag_config_delete(...).doit().await
//! let r = hub.accounts().containers_workspaces_gtag_config_get(...).doit().await
//! let r = hub.accounts().containers_workspaces_gtag_config_list(...).doit().await
//! let r = hub.accounts().containers_workspaces_gtag_config_update(...).doit().await
//! let r = hub.accounts().containers_workspaces_tags_create(...).doit().await
//! let r = hub.accounts().containers_workspaces_tags_delete(...).doit().await
//! let r = hub.accounts().containers_workspaces_tags_get(...).doit().await
//! let r = hub.accounts().containers_workspaces_tags_list(...).doit().await
//! let r = hub.accounts().containers_workspaces_tags_revert(...).doit().await
//! let r = hub.accounts().containers_workspaces_tags_update(...).doit().await
//! let r = hub.accounts().containers_workspaces_templates_create(...).doit().await
//! let r = hub.accounts().containers_workspaces_templates_delete(...).doit().await
//! let r = hub.accounts().containers_workspaces_templates_get(...).doit().await
//! let r = hub.accounts().containers_workspaces_templates_list(...).doit().await
//! let r = hub.accounts().containers_workspaces_templates_revert(...).doit().await
//! let r = hub.accounts().containers_workspaces_templates_update(...).doit().await
//! let r = hub.accounts().containers_workspaces_transformations_create(...).doit().await
//! let r = hub.accounts().containers_workspaces_transformations_delete(...).doit().await
//! let r = hub.accounts().containers_workspaces_transformations_get(...).doit().await
//! let r = hub.accounts().containers_workspaces_transformations_list(...).doit().await
//! let r = hub.accounts().containers_workspaces_transformations_revert(...).doit().await
//! let r = hub.accounts().containers_workspaces_transformations_update(...).doit().await
//! let r = hub.accounts().containers_workspaces_triggers_create(...).doit().await
//! let r = hub.accounts().containers_workspaces_triggers_delete(...).doit().await
//! let r = hub.accounts().containers_workspaces_triggers_get(...).doit().await
//! let r = hub.accounts().containers_workspaces_triggers_list(...).doit().await
//! let r = hub.accounts().containers_workspaces_triggers_revert(...).doit().await
//! let r = hub.accounts().containers_workspaces_triggers_update(...).doit().await
//! let r = hub.accounts().containers_workspaces_variables_create(...).doit().await
//! let r = hub.accounts().containers_workspaces_variables_delete(...).doit().await
//! let r = hub.accounts().containers_workspaces_variables_get(...).doit().await
//! let r = hub.accounts().containers_workspaces_variables_list(...).doit().await
//! let r = hub.accounts().containers_workspaces_variables_revert(...).doit().await
//! let r = hub.accounts().containers_workspaces_variables_update(...).doit().await
//! let r = hub.accounts().containers_workspaces_zones_create(...).doit().await
//! let r = hub.accounts().containers_workspaces_zones_delete(...).doit().await
//! let r = hub.accounts().containers_workspaces_zones_get(...).doit().await
//! let r = hub.accounts().containers_workspaces_zones_list(...).doit().await
//! let r = hub.accounts().containers_workspaces_zones_revert(...).doit().await
//! let r = hub.accounts().containers_workspaces_zones_update(...).doit().await
//! let r = hub.accounts().containers_workspaces_create(...).doit().await
//! let r = hub.accounts().containers_workspaces_create_version(...).doit().await
//! let r = hub.accounts().containers_workspaces_delete(...).doit().await
//! let r = hub.accounts().containers_workspaces_get(...).doit().await
//! let r = hub.accounts().containers_workspaces_get_status(...).doit().await
//! let r = hub.accounts().containers_workspaces_list(...).doit().await
//! let r = hub.accounts().containers_workspaces_quick_preview(...).doit().await
//! let r = hub.accounts().containers_workspaces_resolve_conflict(...).doit().await
//! let r = hub.accounts().containers_workspaces_sync(...).doit().await
//! let r = hub.accounts().containers_workspaces_update(...).doit().await
//! let r = hub.accounts().containers_combine(...).doit().await
//! let r = hub.accounts().containers_create(...).doit().await
//! let r = hub.accounts().containers_delete(...).doit().await
//! let r = hub.accounts().containers_get(...).doit().await
//! let r = hub.accounts().containers_list(...).doit().await
//! let r = hub.accounts().containers_lookup(...).doit().await
//! let r = hub.accounts().containers_move_tag_id(...).doit().await
//! let r = hub.accounts().containers_snippet(...).doit().await
//! let r = hub.accounts().containers_update(...).doit().await
//! let r = hub.accounts().user_permissions_create(...).doit().await
//! let r = hub.accounts().user_permissions_delete(...).doit().await
//! let r = hub.accounts().user_permissions_get(...).doit().await
//! let r = hub.accounts().user_permissions_list(...).doit().await
//! let r = hub.accounts().user_permissions_update(...).doit().await
//! let r = hub.accounts().get(...).doit().await
//! let r = hub.accounts().list(...).doit().await
//! let r = hub.accounts().update(...).doit().await
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
//! google-tagmanager2 = "*"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate google_tagmanager2 as tagmanager2;
//! use tagmanager2::{Result, Error};
//! # async fn dox() {
//! use std::default::Default;
//! use tagmanager2::{TagManager, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
//! let mut hub = TagManager::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.accounts().containers_move_tag_id("path")
//!              .tag_name("ipsum")
//!              .tag_id("gubergren")
//!              .copy_users(true)
//!              .copy_terms_of_service(false)
//!              .copy_settings(true)
//!              .allow_user_permission_feature_update(false)
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
pub use api::TagManager;
pub use client::{Result, Error, Delegate, FieldMask};

// Re-export the yup_oauth2 crate, that is required to call some methods of the hub and the client
#[cfg(feature = "yup-oauth2")]
pub use client::oauth2;