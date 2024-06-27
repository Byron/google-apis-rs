// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Cloud Healthcare* crate version *5.0.5+20240605*, where *20240605* is the exact revision of the *healthcare:v1beta1* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.
//! 
//! Everything else about the *Cloud Healthcare* *v1_beta1* API can be found at the
//! [official documentation site](https://cloud.google.com/healthcare).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/healthcare1_beta1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](CloudHealthcare) ... 
//! 
//! * projects
//!  * [*locations datasets annotation stores annotations create*](api::ProjectLocationDatasetAnnotationStoreAnnotationCreateCall), [*locations datasets annotation stores annotations delete*](api::ProjectLocationDatasetAnnotationStoreAnnotationDeleteCall), [*locations datasets annotation stores annotations get*](api::ProjectLocationDatasetAnnotationStoreAnnotationGetCall), [*locations datasets annotation stores annotations list*](api::ProjectLocationDatasetAnnotationStoreAnnotationListCall), [*locations datasets annotation stores annotations patch*](api::ProjectLocationDatasetAnnotationStoreAnnotationPatchCall), [*locations datasets annotation stores create*](api::ProjectLocationDatasetAnnotationStoreCreateCall), [*locations datasets annotation stores delete*](api::ProjectLocationDatasetAnnotationStoreDeleteCall), [*locations datasets annotation stores evaluate*](api::ProjectLocationDatasetAnnotationStoreEvaluateCall), [*locations datasets annotation stores export*](api::ProjectLocationDatasetAnnotationStoreExportCall), [*locations datasets annotation stores get*](api::ProjectLocationDatasetAnnotationStoreGetCall), [*locations datasets annotation stores get iam policy*](api::ProjectLocationDatasetAnnotationStoreGetIamPolicyCall), [*locations datasets annotation stores import*](api::ProjectLocationDatasetAnnotationStoreImportCall), [*locations datasets annotation stores list*](api::ProjectLocationDatasetAnnotationStoreListCall), [*locations datasets annotation stores patch*](api::ProjectLocationDatasetAnnotationStorePatchCall), [*locations datasets annotation stores set iam policy*](api::ProjectLocationDatasetAnnotationStoreSetIamPolicyCall), [*locations datasets annotation stores test iam permissions*](api::ProjectLocationDatasetAnnotationStoreTestIamPermissionCall), [*locations datasets consent stores attribute definitions create*](api::ProjectLocationDatasetConsentStoreAttributeDefinitionCreateCall), [*locations datasets consent stores attribute definitions delete*](api::ProjectLocationDatasetConsentStoreAttributeDefinitionDeleteCall), [*locations datasets consent stores attribute definitions get*](api::ProjectLocationDatasetConsentStoreAttributeDefinitionGetCall), [*locations datasets consent stores attribute definitions list*](api::ProjectLocationDatasetConsentStoreAttributeDefinitionListCall), [*locations datasets consent stores attribute definitions patch*](api::ProjectLocationDatasetConsentStoreAttributeDefinitionPatchCall), [*locations datasets consent stores check data access*](api::ProjectLocationDatasetConsentStoreCheckDataAccesCall), [*locations datasets consent stores consent artifacts create*](api::ProjectLocationDatasetConsentStoreConsentArtifactCreateCall), [*locations datasets consent stores consent artifacts delete*](api::ProjectLocationDatasetConsentStoreConsentArtifactDeleteCall), [*locations datasets consent stores consent artifacts get*](api::ProjectLocationDatasetConsentStoreConsentArtifactGetCall), [*locations datasets consent stores consent artifacts list*](api::ProjectLocationDatasetConsentStoreConsentArtifactListCall), [*locations datasets consent stores consents activate*](api::ProjectLocationDatasetConsentStoreConsentActivateCall), [*locations datasets consent stores consents create*](api::ProjectLocationDatasetConsentStoreConsentCreateCall), [*locations datasets consent stores consents delete*](api::ProjectLocationDatasetConsentStoreConsentDeleteCall), [*locations datasets consent stores consents delete revision*](api::ProjectLocationDatasetConsentStoreConsentDeleteRevisionCall), [*locations datasets consent stores consents get*](api::ProjectLocationDatasetConsentStoreConsentGetCall), [*locations datasets consent stores consents list*](api::ProjectLocationDatasetConsentStoreConsentListCall), [*locations datasets consent stores consents list revisions*](api::ProjectLocationDatasetConsentStoreConsentListRevisionCall), [*locations datasets consent stores consents patch*](api::ProjectLocationDatasetConsentStoreConsentPatchCall), [*locations datasets consent stores consents reject*](api::ProjectLocationDatasetConsentStoreConsentRejectCall), [*locations datasets consent stores consents revoke*](api::ProjectLocationDatasetConsentStoreConsentRevokeCall), [*locations datasets consent stores create*](api::ProjectLocationDatasetConsentStoreCreateCall), [*locations datasets consent stores delete*](api::ProjectLocationDatasetConsentStoreDeleteCall), [*locations datasets consent stores evaluate user consents*](api::ProjectLocationDatasetConsentStoreEvaluateUserConsentCall), [*locations datasets consent stores get*](api::ProjectLocationDatasetConsentStoreGetCall), [*locations datasets consent stores get iam policy*](api::ProjectLocationDatasetConsentStoreGetIamPolicyCall), [*locations datasets consent stores list*](api::ProjectLocationDatasetConsentStoreListCall), [*locations datasets consent stores patch*](api::ProjectLocationDatasetConsentStorePatchCall), [*locations datasets consent stores query accessible data*](api::ProjectLocationDatasetConsentStoreQueryAccessibleDataCall), [*locations datasets consent stores set iam policy*](api::ProjectLocationDatasetConsentStoreSetIamPolicyCall), [*locations datasets consent stores test iam permissions*](api::ProjectLocationDatasetConsentStoreTestIamPermissionCall), [*locations datasets consent stores user data mappings archive*](api::ProjectLocationDatasetConsentStoreUserDataMappingArchiveCall), [*locations datasets consent stores user data mappings create*](api::ProjectLocationDatasetConsentStoreUserDataMappingCreateCall), [*locations datasets consent stores user data mappings delete*](api::ProjectLocationDatasetConsentStoreUserDataMappingDeleteCall), [*locations datasets consent stores user data mappings get*](api::ProjectLocationDatasetConsentStoreUserDataMappingGetCall), [*locations datasets consent stores user data mappings list*](api::ProjectLocationDatasetConsentStoreUserDataMappingListCall), [*locations datasets consent stores user data mappings patch*](api::ProjectLocationDatasetConsentStoreUserDataMappingPatchCall), [*locations datasets create*](api::ProjectLocationDatasetCreateCall), [*locations datasets data mapper workspaces get iam policy*](api::ProjectLocationDatasetDataMapperWorkspaceGetIamPolicyCall), [*locations datasets data mapper workspaces set iam policy*](api::ProjectLocationDatasetDataMapperWorkspaceSetIamPolicyCall), [*locations datasets data mapper workspaces test iam permissions*](api::ProjectLocationDatasetDataMapperWorkspaceTestIamPermissionCall), [*locations datasets deidentify*](api::ProjectLocationDatasetDeidentifyCall), [*locations datasets delete*](api::ProjectLocationDatasetDeleteCall), [*locations datasets dicom stores create*](api::ProjectLocationDatasetDicomStoreCreateCall), [*locations datasets dicom stores deidentify*](api::ProjectLocationDatasetDicomStoreDeidentifyCall), [*locations datasets dicom stores delete*](api::ProjectLocationDatasetDicomStoreDeleteCall), [*locations datasets dicom stores dicom web studies get study metrics*](api::ProjectLocationDatasetDicomStoreDicomWebStudyGetStudyMetricCall), [*locations datasets dicom stores dicom web studies series get series metrics*](api::ProjectLocationDatasetDicomStoreDicomWebStudySeriesGetSeriesMetricCall), [*locations datasets dicom stores dicom web studies series instances get storage info*](api::ProjectLocationDatasetDicomStoreDicomWebStudySeriesInstanceGetStorageInfoCall), [*locations datasets dicom stores dicom web studies set blob storage settings*](api::ProjectLocationDatasetDicomStoreDicomWebStudySetBlobStorageSettingCall), [*locations datasets dicom stores export*](api::ProjectLocationDatasetDicomStoreExportCall), [*locations datasets dicom stores get*](api::ProjectLocationDatasetDicomStoreGetCall), [*locations datasets dicom stores get dicom store metrics*](api::ProjectLocationDatasetDicomStoreGetDICOMStoreMetricCall), [*locations datasets dicom stores get iam policy*](api::ProjectLocationDatasetDicomStoreGetIamPolicyCall), [*locations datasets dicom stores import*](api::ProjectLocationDatasetDicomStoreImportCall), [*locations datasets dicom stores list*](api::ProjectLocationDatasetDicomStoreListCall), [*locations datasets dicom stores patch*](api::ProjectLocationDatasetDicomStorePatchCall), [*locations datasets dicom stores search for instances*](api::ProjectLocationDatasetDicomStoreSearchForInstanceCall), [*locations datasets dicom stores search for series*](api::ProjectLocationDatasetDicomStoreSearchForSeryCall), [*locations datasets dicom stores search for studies*](api::ProjectLocationDatasetDicomStoreSearchForStudyCall), [*locations datasets dicom stores set blob storage settings*](api::ProjectLocationDatasetDicomStoreSetBlobStorageSettingCall), [*locations datasets dicom stores set iam policy*](api::ProjectLocationDatasetDicomStoreSetIamPolicyCall), [*locations datasets dicom stores store instances*](api::ProjectLocationDatasetDicomStoreStoreInstanceCall), [*locations datasets dicom stores studies delete*](api::ProjectLocationDatasetDicomStoreStudyDeleteCall), [*locations datasets dicom stores studies retrieve metadata*](api::ProjectLocationDatasetDicomStoreStudyRetrieveMetadataCall), [*locations datasets dicom stores studies retrieve study*](api::ProjectLocationDatasetDicomStoreStudyRetrieveStudyCall), [*locations datasets dicom stores studies search for instances*](api::ProjectLocationDatasetDicomStoreStudySearchForInstanceCall), [*locations datasets dicom stores studies search for series*](api::ProjectLocationDatasetDicomStoreStudySearchForSeryCall), [*locations datasets dicom stores studies series delete*](api::ProjectLocationDatasetDicomStoreStudySeriesDeleteCall), [*locations datasets dicom stores studies series instances bulkdata retrieve bulkdata*](api::ProjectLocationDatasetDicomStoreStudySeriesInstanceBulkdataRetrieveBulkdataCall), [*locations datasets dicom stores studies series instances delete*](api::ProjectLocationDatasetDicomStoreStudySeriesInstanceDeleteCall), [*locations datasets dicom stores studies series instances frames retrieve frames*](api::ProjectLocationDatasetDicomStoreStudySeriesInstanceFrameRetrieveFrameCall), [*locations datasets dicom stores studies series instances frames retrieve rendered*](api::ProjectLocationDatasetDicomStoreStudySeriesInstanceFrameRetrieveRenderedCall), [*locations datasets dicom stores studies series instances retrieve instance*](api::ProjectLocationDatasetDicomStoreStudySeriesInstanceRetrieveInstanceCall), [*locations datasets dicom stores studies series instances retrieve metadata*](api::ProjectLocationDatasetDicomStoreStudySeriesInstanceRetrieveMetadataCall), [*locations datasets dicom stores studies series instances retrieve rendered*](api::ProjectLocationDatasetDicomStoreStudySeriesInstanceRetrieveRenderedCall), [*locations datasets dicom stores studies series retrieve metadata*](api::ProjectLocationDatasetDicomStoreStudySeriesRetrieveMetadataCall), [*locations datasets dicom stores studies series retrieve series*](api::ProjectLocationDatasetDicomStoreStudySeriesRetrieveSeryCall), [*locations datasets dicom stores studies series search for instances*](api::ProjectLocationDatasetDicomStoreStudySeriesSearchForInstanceCall), [*locations datasets dicom stores studies store instances*](api::ProjectLocationDatasetDicomStoreStudyStoreInstanceCall), [*locations datasets dicom stores test iam permissions*](api::ProjectLocationDatasetDicomStoreTestIamPermissionCall), [*locations datasets fhir stores apply admin consents*](api::ProjectLocationDatasetFhirStoreApplyAdminConsentCall), [*locations datasets fhir stores apply consents*](api::ProjectLocationDatasetFhirStoreApplyConsentCall), [*locations datasets fhir stores configure search*](api::ProjectLocationDatasetFhirStoreConfigureSearchCall), [*locations datasets fhir stores create*](api::ProjectLocationDatasetFhirStoreCreateCall), [*locations datasets fhir stores deidentify*](api::ProjectLocationDatasetFhirStoreDeidentifyCall), [*locations datasets fhir stores delete*](api::ProjectLocationDatasetFhirStoreDeleteCall), [*locations datasets fhir stores explain data access*](api::ProjectLocationDatasetFhirStoreExplainDataAccesCall), [*locations datasets fhir stores export*](api::ProjectLocationDatasetFhirStoreExportCall), [*locations datasets fhir stores export history*](api::ProjectLocationDatasetFhirStoreExportHistoryCall), [*locations datasets fhir stores fhir  concept map-search-translate*](api::ProjectLocationDatasetFhirStoreFhirConceptMapSearchTranslateCall), [*locations datasets fhir stores fhir  concept map-translate*](api::ProjectLocationDatasetFhirStoreFhirConceptMapTranslateCall), [*locations datasets fhir stores fhir  consent-enforcement-status*](api::ProjectLocationDatasetFhirStoreFhirConsentEnforcementStatuCall), [*locations datasets fhir stores fhir  observation-lastn*](api::ProjectLocationDatasetFhirStoreFhirObservationLastnCall), [*locations datasets fhir stores fhir  patient-consent-enforcement-status*](api::ProjectLocationDatasetFhirStoreFhirPatientConsentEnforcementStatuCall), [*locations datasets fhir stores fhir  patient-everything*](api::ProjectLocationDatasetFhirStoreFhirPatientEverythingCall), [*locations datasets fhir stores fhir  resource-incoming-references*](api::ProjectLocationDatasetFhirStoreFhirResourceIncomingReferenceCall), [*locations datasets fhir stores fhir  resource-purge*](api::ProjectLocationDatasetFhirStoreFhirResourcePurgeCall), [*locations datasets fhir stores fhir  resource-validate*](api::ProjectLocationDatasetFhirStoreFhirResourceValidateCall), [*locations datasets fhir stores fhir capabilities*](api::ProjectLocationDatasetFhirStoreFhirCapabilityCall), [*locations datasets fhir stores fhir conditional delete*](api::ProjectLocationDatasetFhirStoreFhirConditionalDeleteCall), [*locations datasets fhir stores fhir conditional patch*](api::ProjectLocationDatasetFhirStoreFhirConditionalPatchCall), [*locations datasets fhir stores fhir conditional update*](api::ProjectLocationDatasetFhirStoreFhirConditionalUpdateCall), [*locations datasets fhir stores fhir create*](api::ProjectLocationDatasetFhirStoreFhirCreateCall), [*locations datasets fhir stores fhir delete*](api::ProjectLocationDatasetFhirStoreFhirDeleteCall), [*locations datasets fhir stores fhir execute bundle*](api::ProjectLocationDatasetFhirStoreFhirExecuteBundleCall), [*locations datasets fhir stores fhir history*](api::ProjectLocationDatasetFhirStoreFhirHistoryCall), [*locations datasets fhir stores fhir patch*](api::ProjectLocationDatasetFhirStoreFhirPatchCall), [*locations datasets fhir stores fhir read*](api::ProjectLocationDatasetFhirStoreFhirReadCall), [*locations datasets fhir stores fhir search*](api::ProjectLocationDatasetFhirStoreFhirSearchCall), [*locations datasets fhir stores fhir search-type*](api::ProjectLocationDatasetFhirStoreFhirSearchTypeCall), [*locations datasets fhir stores fhir update*](api::ProjectLocationDatasetFhirStoreFhirUpdateCall), [*locations datasets fhir stores fhir vread*](api::ProjectLocationDatasetFhirStoreFhirVreadCall), [*locations datasets fhir stores get*](api::ProjectLocationDatasetFhirStoreGetCall), [*locations datasets fhir stores get fhir store metrics*](api::ProjectLocationDatasetFhirStoreGetFHIRStoreMetricCall), [*locations datasets fhir stores get iam policy*](api::ProjectLocationDatasetFhirStoreGetIamPolicyCall), [*locations datasets fhir stores import*](api::ProjectLocationDatasetFhirStoreImportCall), [*locations datasets fhir stores import history*](api::ProjectLocationDatasetFhirStoreImportHistoryCall), [*locations datasets fhir stores list*](api::ProjectLocationDatasetFhirStoreListCall), [*locations datasets fhir stores patch*](api::ProjectLocationDatasetFhirStorePatchCall), [*locations datasets fhir stores rollback*](api::ProjectLocationDatasetFhirStoreRollbackCall), [*locations datasets fhir stores set iam policy*](api::ProjectLocationDatasetFhirStoreSetIamPolicyCall), [*locations datasets fhir stores test iam permissions*](api::ProjectLocationDatasetFhirStoreTestIamPermissionCall), [*locations datasets get*](api::ProjectLocationDatasetGetCall), [*locations datasets get iam policy*](api::ProjectLocationDatasetGetIamPolicyCall), [*locations datasets hl7 v2 stores create*](api::ProjectLocationDatasetHl7V2StoreCreateCall), [*locations datasets hl7 v2 stores delete*](api::ProjectLocationDatasetHl7V2StoreDeleteCall), [*locations datasets hl7 v2 stores export*](api::ProjectLocationDatasetHl7V2StoreExportCall), [*locations datasets hl7 v2 stores get*](api::ProjectLocationDatasetHl7V2StoreGetCall), [*locations datasets hl7 v2 stores get hl7v2 store metrics*](api::ProjectLocationDatasetHl7V2StoreGetHL7v2StoreMetricCall), [*locations datasets hl7 v2 stores get iam policy*](api::ProjectLocationDatasetHl7V2StoreGetIamPolicyCall), [*locations datasets hl7 v2 stores import*](api::ProjectLocationDatasetHl7V2StoreImportCall), [*locations datasets hl7 v2 stores list*](api::ProjectLocationDatasetHl7V2StoreListCall), [*locations datasets hl7 v2 stores messages batch get*](api::ProjectLocationDatasetHl7V2StoreMessageBatchGetCall), [*locations datasets hl7 v2 stores messages create*](api::ProjectLocationDatasetHl7V2StoreMessageCreateCall), [*locations datasets hl7 v2 stores messages delete*](api::ProjectLocationDatasetHl7V2StoreMessageDeleteCall), [*locations datasets hl7 v2 stores messages get*](api::ProjectLocationDatasetHl7V2StoreMessageGetCall), [*locations datasets hl7 v2 stores messages ingest*](api::ProjectLocationDatasetHl7V2StoreMessageIngestCall), [*locations datasets hl7 v2 stores messages list*](api::ProjectLocationDatasetHl7V2StoreMessageListCall), [*locations datasets hl7 v2 stores messages patch*](api::ProjectLocationDatasetHl7V2StoreMessagePatchCall), [*locations datasets hl7 v2 stores patch*](api::ProjectLocationDatasetHl7V2StorePatchCall), [*locations datasets hl7 v2 stores set iam policy*](api::ProjectLocationDatasetHl7V2StoreSetIamPolicyCall), [*locations datasets hl7 v2 stores test iam permissions*](api::ProjectLocationDatasetHl7V2StoreTestIamPermissionCall), [*locations datasets list*](api::ProjectLocationDatasetListCall), [*locations datasets operations cancel*](api::ProjectLocationDatasetOperationCancelCall), [*locations datasets operations get*](api::ProjectLocationDatasetOperationGetCall), [*locations datasets operations list*](api::ProjectLocationDatasetOperationListCall), [*locations datasets patch*](api::ProjectLocationDatasetPatchCall), [*locations datasets set iam policy*](api::ProjectLocationDatasetSetIamPolicyCall), [*locations datasets test iam permissions*](api::ProjectLocationDatasetTestIamPermissionCall), [*locations get*](api::ProjectLocationGetCall), [*locations list*](api::ProjectLocationListCall) and [*locations services nlp analyze entities*](api::ProjectLocationServiceNlpAnalyzeEntityCall)
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
//! * **[Hub](CloudHealthcare)**
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
//! let r = hub.projects().locations_datasets_dicom_stores_studies_series_instances_bulkdata_retrieve_bulkdata(...).doit().await
//! let r = hub.projects().locations_datasets_dicom_stores_studies_series_instances_frames_retrieve_frames(...).doit().await
//! let r = hub.projects().locations_datasets_dicom_stores_studies_series_instances_frames_retrieve_rendered(...).doit().await
//! let r = hub.projects().locations_datasets_dicom_stores_studies_series_instances_retrieve_instance(...).doit().await
//! let r = hub.projects().locations_datasets_dicom_stores_studies_series_instances_retrieve_metadata(...).doit().await
//! let r = hub.projects().locations_datasets_dicom_stores_studies_series_instances_retrieve_rendered(...).doit().await
//! let r = hub.projects().locations_datasets_dicom_stores_studies_series_retrieve_metadata(...).doit().await
//! let r = hub.projects().locations_datasets_dicom_stores_studies_series_retrieve_series(...).doit().await
//! let r = hub.projects().locations_datasets_dicom_stores_studies_series_search_for_instances(...).doit().await
//! let r = hub.projects().locations_datasets_dicom_stores_studies_retrieve_metadata(...).doit().await
//! let r = hub.projects().locations_datasets_dicom_stores_studies_retrieve_study(...).doit().await
//! let r = hub.projects().locations_datasets_dicom_stores_studies_search_for_instances(...).doit().await
//! let r = hub.projects().locations_datasets_dicom_stores_studies_search_for_series(...).doit().await
//! let r = hub.projects().locations_datasets_dicom_stores_studies_store_instances(...).doit().await
//! let r = hub.projects().locations_datasets_dicom_stores_search_for_instances(...).doit().await
//! let r = hub.projects().locations_datasets_dicom_stores_search_for_series(...).doit().await
//! let r = hub.projects().locations_datasets_dicom_stores_search_for_studies(...).doit().await
//! let r = hub.projects().locations_datasets_dicom_stores_store_instances(...).doit().await
//! let r = hub.projects().locations_datasets_fhir_stores_fhir__concept_map_search_translate(...).doit().await
//! let r = hub.projects().locations_datasets_fhir_stores_fhir__concept_map_translate(...).doit().await
//! let r = hub.projects().locations_datasets_fhir_stores_fhir__consent_enforcement_status(...).doit().await
//! let r = hub.projects().locations_datasets_fhir_stores_fhir__observation_lastn(...).doit().await
//! let r = hub.projects().locations_datasets_fhir_stores_fhir__patient_consent_enforcement_status(...).doit().await
//! let r = hub.projects().locations_datasets_fhir_stores_fhir__patient_everything(...).doit().await
//! let r = hub.projects().locations_datasets_fhir_stores_fhir__resource_incoming_references(...).doit().await
//! let r = hub.projects().locations_datasets_fhir_stores_fhir__resource_validate(...).doit().await
//! let r = hub.projects().locations_datasets_fhir_stores_fhir_capabilities(...).doit().await
//! let r = hub.projects().locations_datasets_fhir_stores_fhir_conditional_patch(...).doit().await
//! let r = hub.projects().locations_datasets_fhir_stores_fhir_conditional_update(...).doit().await
//! let r = hub.projects().locations_datasets_fhir_stores_fhir_create(...).doit().await
//! let r = hub.projects().locations_datasets_fhir_stores_fhir_delete(...).doit().await
//! let r = hub.projects().locations_datasets_fhir_stores_fhir_execute_bundle(...).doit().await
//! let r = hub.projects().locations_datasets_fhir_stores_fhir_history(...).doit().await
//! let r = hub.projects().locations_datasets_fhir_stores_fhir_patch(...).doit().await
//! let r = hub.projects().locations_datasets_fhir_stores_fhir_read(...).doit().await
//! let r = hub.projects().locations_datasets_fhir_stores_fhir_search(...).doit().await
//! let r = hub.projects().locations_datasets_fhir_stores_fhir_search_type(...).doit().await
//! let r = hub.projects().locations_datasets_fhir_stores_fhir_update(...).doit().await
//! let r = hub.projects().locations_datasets_fhir_stores_fhir_vread(...).doit().await
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
//! google-healthcare1_beta1 = "*"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate google_healthcare1_beta1 as healthcare1_beta1;
//! use healthcare1_beta1::{Result, Error};
//! # async fn dox() {
//! use std::default::Default;
//! use healthcare1_beta1::{CloudHealthcare, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
//! let mut hub = CloudHealthcare::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.projects().locations_datasets_fhir_stores_fhir__concept_map_search_translate("parent")
//!              .url("amet.")
//!              .target("takimata")
//!              .system("amet.")
//!              .source("duo")
//!              .concept_map_version("ipsum")
//!              .code("gubergren")
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
pub use api::CloudHealthcare;
pub use client::{Result, Error, Delegate, FieldMask};

// Re-export the yup_oauth2 crate, that is required to call some methods of the hub and the client
#[cfg(feature = "yup-oauth2")]
pub use client::oauth2;