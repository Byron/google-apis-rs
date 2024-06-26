// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Security Command Center* crate version *5.0.5+20240622*, where *20240622* is the exact revision of the *securitycenter:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.
//! 
//! Everything else about the *Security Command Center* *v1* API can be found at the
//! [official documentation site](https://cloud.google.com/security-command-center).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/securitycenter1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](SecurityCommandCenter) ... 
//! 
//! * [folders](api::Folder)
//!  * [*assets group*](api::FolderAssetGroupCall), [*assets list*](api::FolderAssetListCall), [*assets update security marks*](api::FolderAssetUpdateSecurityMarkCall), [*big query exports create*](api::FolderBigQueryExportCreateCall), [*big query exports delete*](api::FolderBigQueryExportDeleteCall), [*big query exports get*](api::FolderBigQueryExportGetCall), [*big query exports list*](api::FolderBigQueryExportListCall), [*big query exports patch*](api::FolderBigQueryExportPatchCall), [*event threat detection settings custom modules create*](api::FolderEventThreatDetectionSettingCustomModuleCreateCall), [*event threat detection settings custom modules delete*](api::FolderEventThreatDetectionSettingCustomModuleDeleteCall), [*event threat detection settings custom modules get*](api::FolderEventThreatDetectionSettingCustomModuleGetCall), [*event threat detection settings custom modules list*](api::FolderEventThreatDetectionSettingCustomModuleListCall), [*event threat detection settings custom modules list descendant*](api::FolderEventThreatDetectionSettingCustomModuleListDescendantCall), [*event threat detection settings custom modules patch*](api::FolderEventThreatDetectionSettingCustomModulePatchCall), [*event threat detection settings effective custom modules get*](api::FolderEventThreatDetectionSettingEffectiveCustomModuleGetCall), [*event threat detection settings effective custom modules list*](api::FolderEventThreatDetectionSettingEffectiveCustomModuleListCall), [*event threat detection settings validate custom module*](api::FolderEventThreatDetectionSettingValidateCustomModuleCall), [*findings bulk mute*](api::FolderFindingBulkMuteCall), [*locations mute configs create*](api::FolderLocationMuteConfigCreateCall), [*locations mute configs delete*](api::FolderLocationMuteConfigDeleteCall), [*locations mute configs get*](api::FolderLocationMuteConfigGetCall), [*locations mute configs list*](api::FolderLocationMuteConfigListCall), [*locations mute configs patch*](api::FolderLocationMuteConfigPatchCall), [*mute configs create*](api::FolderMuteConfigCreateCall), [*mute configs delete*](api::FolderMuteConfigDeleteCall), [*mute configs get*](api::FolderMuteConfigGetCall), [*mute configs list*](api::FolderMuteConfigListCall), [*mute configs patch*](api::FolderMuteConfigPatchCall), [*notification configs create*](api::FolderNotificationConfigCreateCall), [*notification configs delete*](api::FolderNotificationConfigDeleteCall), [*notification configs get*](api::FolderNotificationConfigGetCall), [*notification configs list*](api::FolderNotificationConfigListCall), [*notification configs patch*](api::FolderNotificationConfigPatchCall), [*security health analytics settings custom modules create*](api::FolderSecurityHealthAnalyticsSettingCustomModuleCreateCall), [*security health analytics settings custom modules delete*](api::FolderSecurityHealthAnalyticsSettingCustomModuleDeleteCall), [*security health analytics settings custom modules get*](api::FolderSecurityHealthAnalyticsSettingCustomModuleGetCall), [*security health analytics settings custom modules list*](api::FolderSecurityHealthAnalyticsSettingCustomModuleListCall), [*security health analytics settings custom modules list descendant*](api::FolderSecurityHealthAnalyticsSettingCustomModuleListDescendantCall), [*security health analytics settings custom modules patch*](api::FolderSecurityHealthAnalyticsSettingCustomModulePatchCall), [*security health analytics settings custom modules simulate*](api::FolderSecurityHealthAnalyticsSettingCustomModuleSimulateCall), [*security health analytics settings effective custom modules get*](api::FolderSecurityHealthAnalyticsSettingEffectiveCustomModuleGetCall), [*security health analytics settings effective custom modules list*](api::FolderSecurityHealthAnalyticsSettingEffectiveCustomModuleListCall), [*sources findings external systems patch*](api::FolderSourceFindingExternalSystemPatchCall), [*sources findings group*](api::FolderSourceFindingGroupCall), [*sources findings list*](api::FolderSourceFindingListCall), [*sources findings patch*](api::FolderSourceFindingPatchCall), [*sources findings set mute*](api::FolderSourceFindingSetMuteCall), [*sources findings set state*](api::FolderSourceFindingSetStateCall), [*sources findings update security marks*](api::FolderSourceFindingUpdateSecurityMarkCall) and [*sources list*](api::FolderSourceListCall)
//! * organizations
//!  * [*assets group*](api::OrganizationAssetGroupCall), [*assets list*](api::OrganizationAssetListCall), [*assets run discovery*](api::OrganizationAssetRunDiscoveryCall), [*assets update security marks*](api::OrganizationAssetUpdateSecurityMarkCall), [*big query exports create*](api::OrganizationBigQueryExportCreateCall), [*big query exports delete*](api::OrganizationBigQueryExportDeleteCall), [*big query exports get*](api::OrganizationBigQueryExportGetCall), [*big query exports list*](api::OrganizationBigQueryExportListCall), [*big query exports patch*](api::OrganizationBigQueryExportPatchCall), [*event threat detection settings custom modules create*](api::OrganizationEventThreatDetectionSettingCustomModuleCreateCall), [*event threat detection settings custom modules delete*](api::OrganizationEventThreatDetectionSettingCustomModuleDeleteCall), [*event threat detection settings custom modules get*](api::OrganizationEventThreatDetectionSettingCustomModuleGetCall), [*event threat detection settings custom modules list*](api::OrganizationEventThreatDetectionSettingCustomModuleListCall), [*event threat detection settings custom modules list descendant*](api::OrganizationEventThreatDetectionSettingCustomModuleListDescendantCall), [*event threat detection settings custom modules patch*](api::OrganizationEventThreatDetectionSettingCustomModulePatchCall), [*event threat detection settings effective custom modules get*](api::OrganizationEventThreatDetectionSettingEffectiveCustomModuleGetCall), [*event threat detection settings effective custom modules list*](api::OrganizationEventThreatDetectionSettingEffectiveCustomModuleListCall), [*event threat detection settings validate custom module*](api::OrganizationEventThreatDetectionSettingValidateCustomModuleCall), [*findings bulk mute*](api::OrganizationFindingBulkMuteCall), [*get organization settings*](api::OrganizationGetOrganizationSettingCall), [*locations mute configs create*](api::OrganizationLocationMuteConfigCreateCall), [*locations mute configs delete*](api::OrganizationLocationMuteConfigDeleteCall), [*locations mute configs get*](api::OrganizationLocationMuteConfigGetCall), [*locations mute configs list*](api::OrganizationLocationMuteConfigListCall), [*locations mute configs patch*](api::OrganizationLocationMuteConfigPatchCall), [*mute configs create*](api::OrganizationMuteConfigCreateCall), [*mute configs delete*](api::OrganizationMuteConfigDeleteCall), [*mute configs get*](api::OrganizationMuteConfigGetCall), [*mute configs list*](api::OrganizationMuteConfigListCall), [*mute configs patch*](api::OrganizationMuteConfigPatchCall), [*notification configs create*](api::OrganizationNotificationConfigCreateCall), [*notification configs delete*](api::OrganizationNotificationConfigDeleteCall), [*notification configs get*](api::OrganizationNotificationConfigGetCall), [*notification configs list*](api::OrganizationNotificationConfigListCall), [*notification configs patch*](api::OrganizationNotificationConfigPatchCall), [*operations cancel*](api::OrganizationOperationCancelCall), [*operations delete*](api::OrganizationOperationDeleteCall), [*operations get*](api::OrganizationOperationGetCall), [*operations list*](api::OrganizationOperationListCall), [*resource value configs batch create*](api::OrganizationResourceValueConfigBatchCreateCall), [*resource value configs delete*](api::OrganizationResourceValueConfigDeleteCall), [*resource value configs get*](api::OrganizationResourceValueConfigGetCall), [*resource value configs list*](api::OrganizationResourceValueConfigListCall), [*resource value configs patch*](api::OrganizationResourceValueConfigPatchCall), [*security health analytics settings custom modules create*](api::OrganizationSecurityHealthAnalyticsSettingCustomModuleCreateCall), [*security health analytics settings custom modules delete*](api::OrganizationSecurityHealthAnalyticsSettingCustomModuleDeleteCall), [*security health analytics settings custom modules get*](api::OrganizationSecurityHealthAnalyticsSettingCustomModuleGetCall), [*security health analytics settings custom modules list*](api::OrganizationSecurityHealthAnalyticsSettingCustomModuleListCall), [*security health analytics settings custom modules list descendant*](api::OrganizationSecurityHealthAnalyticsSettingCustomModuleListDescendantCall), [*security health analytics settings custom modules patch*](api::OrganizationSecurityHealthAnalyticsSettingCustomModulePatchCall), [*security health analytics settings custom modules simulate*](api::OrganizationSecurityHealthAnalyticsSettingCustomModuleSimulateCall), [*security health analytics settings effective custom modules get*](api::OrganizationSecurityHealthAnalyticsSettingEffectiveCustomModuleGetCall), [*security health analytics settings effective custom modules list*](api::OrganizationSecurityHealthAnalyticsSettingEffectiveCustomModuleListCall), [*simulations attack exposure results attack paths list*](api::OrganizationSimulationAttackExposureResultAttackPathListCall), [*simulations attack exposure results valued resources list*](api::OrganizationSimulationAttackExposureResultValuedResourceListCall), [*simulations attack paths list*](api::OrganizationSimulationAttackPathListCall), [*simulations get*](api::OrganizationSimulationGetCall), [*simulations valued resources attack paths list*](api::OrganizationSimulationValuedResourceAttackPathListCall), [*simulations valued resources get*](api::OrganizationSimulationValuedResourceGetCall), [*simulations valued resources list*](api::OrganizationSimulationValuedResourceListCall), [*sources create*](api::OrganizationSourceCreateCall), [*sources findings create*](api::OrganizationSourceFindingCreateCall), [*sources findings external systems patch*](api::OrganizationSourceFindingExternalSystemPatchCall), [*sources findings group*](api::OrganizationSourceFindingGroupCall), [*sources findings list*](api::OrganizationSourceFindingListCall), [*sources findings patch*](api::OrganizationSourceFindingPatchCall), [*sources findings set mute*](api::OrganizationSourceFindingSetMuteCall), [*sources findings set state*](api::OrganizationSourceFindingSetStateCall), [*sources findings update security marks*](api::OrganizationSourceFindingUpdateSecurityMarkCall), [*sources get*](api::OrganizationSourceGetCall), [*sources get iam policy*](api::OrganizationSourceGetIamPolicyCall), [*sources list*](api::OrganizationSourceListCall), [*sources patch*](api::OrganizationSourcePatchCall), [*sources set iam policy*](api::OrganizationSourceSetIamPolicyCall), [*sources test iam permissions*](api::OrganizationSourceTestIamPermissionCall) and [*update organization settings*](api::OrganizationUpdateOrganizationSettingCall)
//! * projects
//!  * [*assets group*](api::ProjectAssetGroupCall), [*assets list*](api::ProjectAssetListCall), [*assets update security marks*](api::ProjectAssetUpdateSecurityMarkCall), [*big query exports create*](api::ProjectBigQueryExportCreateCall), [*big query exports delete*](api::ProjectBigQueryExportDeleteCall), [*big query exports get*](api::ProjectBigQueryExportGetCall), [*big query exports list*](api::ProjectBigQueryExportListCall), [*big query exports patch*](api::ProjectBigQueryExportPatchCall), [*event threat detection settings custom modules create*](api::ProjectEventThreatDetectionSettingCustomModuleCreateCall), [*event threat detection settings custom modules delete*](api::ProjectEventThreatDetectionSettingCustomModuleDeleteCall), [*event threat detection settings custom modules get*](api::ProjectEventThreatDetectionSettingCustomModuleGetCall), [*event threat detection settings custom modules list*](api::ProjectEventThreatDetectionSettingCustomModuleListCall), [*event threat detection settings custom modules list descendant*](api::ProjectEventThreatDetectionSettingCustomModuleListDescendantCall), [*event threat detection settings custom modules patch*](api::ProjectEventThreatDetectionSettingCustomModulePatchCall), [*event threat detection settings effective custom modules get*](api::ProjectEventThreatDetectionSettingEffectiveCustomModuleGetCall), [*event threat detection settings effective custom modules list*](api::ProjectEventThreatDetectionSettingEffectiveCustomModuleListCall), [*event threat detection settings validate custom module*](api::ProjectEventThreatDetectionSettingValidateCustomModuleCall), [*findings bulk mute*](api::ProjectFindingBulkMuteCall), [*locations mute configs create*](api::ProjectLocationMuteConfigCreateCall), [*locations mute configs delete*](api::ProjectLocationMuteConfigDeleteCall), [*locations mute configs get*](api::ProjectLocationMuteConfigGetCall), [*locations mute configs list*](api::ProjectLocationMuteConfigListCall), [*locations mute configs patch*](api::ProjectLocationMuteConfigPatchCall), [*mute configs create*](api::ProjectMuteConfigCreateCall), [*mute configs delete*](api::ProjectMuteConfigDeleteCall), [*mute configs get*](api::ProjectMuteConfigGetCall), [*mute configs list*](api::ProjectMuteConfigListCall), [*mute configs patch*](api::ProjectMuteConfigPatchCall), [*notification configs create*](api::ProjectNotificationConfigCreateCall), [*notification configs delete*](api::ProjectNotificationConfigDeleteCall), [*notification configs get*](api::ProjectNotificationConfigGetCall), [*notification configs list*](api::ProjectNotificationConfigListCall), [*notification configs patch*](api::ProjectNotificationConfigPatchCall), [*security health analytics settings custom modules create*](api::ProjectSecurityHealthAnalyticsSettingCustomModuleCreateCall), [*security health analytics settings custom modules delete*](api::ProjectSecurityHealthAnalyticsSettingCustomModuleDeleteCall), [*security health analytics settings custom modules get*](api::ProjectSecurityHealthAnalyticsSettingCustomModuleGetCall), [*security health analytics settings custom modules list*](api::ProjectSecurityHealthAnalyticsSettingCustomModuleListCall), [*security health analytics settings custom modules list descendant*](api::ProjectSecurityHealthAnalyticsSettingCustomModuleListDescendantCall), [*security health analytics settings custom modules patch*](api::ProjectSecurityHealthAnalyticsSettingCustomModulePatchCall), [*security health analytics settings custom modules simulate*](api::ProjectSecurityHealthAnalyticsSettingCustomModuleSimulateCall), [*security health analytics settings effective custom modules get*](api::ProjectSecurityHealthAnalyticsSettingEffectiveCustomModuleGetCall), [*security health analytics settings effective custom modules list*](api::ProjectSecurityHealthAnalyticsSettingEffectiveCustomModuleListCall), [*sources findings external systems patch*](api::ProjectSourceFindingExternalSystemPatchCall), [*sources findings group*](api::ProjectSourceFindingGroupCall), [*sources findings list*](api::ProjectSourceFindingListCall), [*sources findings patch*](api::ProjectSourceFindingPatchCall), [*sources findings set mute*](api::ProjectSourceFindingSetMuteCall), [*sources findings set state*](api::ProjectSourceFindingSetStateCall), [*sources findings update security marks*](api::ProjectSourceFindingUpdateSecurityMarkCall) and [*sources list*](api::ProjectSourceListCall)
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
//! * **[Hub](SecurityCommandCenter)**
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
//! let r = hub.folders().assets_group(...).doit().await
//! let r = hub.folders().assets_list(...).doit().await
//! let r = hub.folders().assets_update_security_marks(...).doit().await
//! let r = hub.folders().big_query_exports_create(...).doit().await
//! let r = hub.folders().big_query_exports_delete(...).doit().await
//! let r = hub.folders().big_query_exports_get(...).doit().await
//! let r = hub.folders().big_query_exports_list(...).doit().await
//! let r = hub.folders().big_query_exports_patch(...).doit().await
//! let r = hub.folders().event_threat_detection_settings_custom_modules_create(...).doit().await
//! let r = hub.folders().event_threat_detection_settings_custom_modules_delete(...).doit().await
//! let r = hub.folders().event_threat_detection_settings_custom_modules_get(...).doit().await
//! let r = hub.folders().event_threat_detection_settings_custom_modules_list(...).doit().await
//! let r = hub.folders().event_threat_detection_settings_custom_modules_list_descendant(...).doit().await
//! let r = hub.folders().event_threat_detection_settings_custom_modules_patch(...).doit().await
//! let r = hub.folders().event_threat_detection_settings_effective_custom_modules_get(...).doit().await
//! let r = hub.folders().event_threat_detection_settings_effective_custom_modules_list(...).doit().await
//! let r = hub.folders().event_threat_detection_settings_validate_custom_module(...).doit().await
//! let r = hub.folders().findings_bulk_mute(...).doit().await
//! let r = hub.folders().locations_mute_configs_create(...).doit().await
//! let r = hub.folders().locations_mute_configs_delete(...).doit().await
//! let r = hub.folders().locations_mute_configs_get(...).doit().await
//! let r = hub.folders().locations_mute_configs_list(...).doit().await
//! let r = hub.folders().locations_mute_configs_patch(...).doit().await
//! let r = hub.folders().mute_configs_create(...).doit().await
//! let r = hub.folders().mute_configs_delete(...).doit().await
//! let r = hub.folders().mute_configs_get(...).doit().await
//! let r = hub.folders().mute_configs_list(...).doit().await
//! let r = hub.folders().mute_configs_patch(...).doit().await
//! let r = hub.folders().notification_configs_create(...).doit().await
//! let r = hub.folders().notification_configs_delete(...).doit().await
//! let r = hub.folders().notification_configs_get(...).doit().await
//! let r = hub.folders().notification_configs_list(...).doit().await
//! let r = hub.folders().notification_configs_patch(...).doit().await
//! let r = hub.folders().security_health_analytics_settings_custom_modules_create(...).doit().await
//! let r = hub.folders().security_health_analytics_settings_custom_modules_delete(...).doit().await
//! let r = hub.folders().security_health_analytics_settings_custom_modules_get(...).doit().await
//! let r = hub.folders().security_health_analytics_settings_custom_modules_list(...).doit().await
//! let r = hub.folders().security_health_analytics_settings_custom_modules_list_descendant(...).doit().await
//! let r = hub.folders().security_health_analytics_settings_custom_modules_patch(...).doit().await
//! let r = hub.folders().security_health_analytics_settings_custom_modules_simulate(...).doit().await
//! let r = hub.folders().security_health_analytics_settings_effective_custom_modules_get(...).doit().await
//! let r = hub.folders().security_health_analytics_settings_effective_custom_modules_list(...).doit().await
//! let r = hub.folders().sources_findings_external_systems_patch(...).doit().await
//! let r = hub.folders().sources_findings_group(...).doit().await
//! let r = hub.folders().sources_findings_list(...).doit().await
//! let r = hub.folders().sources_findings_patch(...).doit().await
//! let r = hub.folders().sources_findings_set_mute(...).doit().await
//! let r = hub.folders().sources_findings_set_state(...).doit().await
//! let r = hub.folders().sources_findings_update_security_marks(...).doit().await
//! let r = hub.folders().sources_list(...).doit().await
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
//! google-securitycenter1 = "*"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate google_securitycenter1 as securitycenter1;
//! use securitycenter1::{Result, Error};
//! # async fn dox() {
//! use std::default::Default;
//! use securitycenter1::{SecurityCommandCenter, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
//! let mut hub = SecurityCommandCenter::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.folders().assets_list("parent")
//!              .read_time(chrono::Utc::now())
//!              .page_token("sed")
//!              .page_size(-2)
//!              .order_by("takimata")
//!              .filter("amet.")
//!              .field_mask(FieldMask::new::<&str>(&[]))
//!              .compare_duration(chrono::Duration::seconds(6037203))
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
pub use api::SecurityCommandCenter;
pub use client::{Result, Error, Delegate, FieldMask};

// Re-export the yup_oauth2 crate, that is required to call some methods of the hub and the client
#[cfg(feature = "yup-oauth2")]
pub use client::oauth2;