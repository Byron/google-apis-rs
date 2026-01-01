// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Contactcenterinsights* crate version *7.0.0+20251222*, where *20251222* is the exact revision of the *contactcenterinsights:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v7.0.0*.
//!
//! Everything else about the *Contactcenterinsights* *v1* API can be found at the
//! [official documentation site](https://cloud.google.com/contact-center/insights/docs).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/contactcenterinsights1).
//! # Features
//!
//! Handle the following *Resources* with ease from the central [hub](Contactcenterinsights) ...
//!
//! * projects
//!  * [*locations analysis rules create*](api::ProjectLocationAnalysisRuleCreateCall), [*locations analysis rules delete*](api::ProjectLocationAnalysisRuleDeleteCall), [*locations analysis rules get*](api::ProjectLocationAnalysisRuleGetCall), [*locations analysis rules list*](api::ProjectLocationAnalysisRuleListCall), [*locations analysis rules patch*](api::ProjectLocationAnalysisRulePatchCall), [*locations assessment rules create*](api::ProjectLocationAssessmentRuleCreateCall), [*locations assessment rules delete*](api::ProjectLocationAssessmentRuleDeleteCall), [*locations assessment rules get*](api::ProjectLocationAssessmentRuleGetCall), [*locations assessment rules list*](api::ProjectLocationAssessmentRuleListCall), [*locations assessment rules patch*](api::ProjectLocationAssessmentRulePatchCall), [*locations authorized view sets authorized views conversations assessments appeal*](api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationAssessmentAppealCall), [*locations authorized view sets authorized views conversations assessments create*](api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationAssessmentCreateCall), [*locations authorized view sets authorized views conversations assessments delete*](api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationAssessmentDeleteCall), [*locations authorized view sets authorized views conversations assessments finalize*](api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationAssessmentFinalizeCall), [*locations authorized view sets authorized views conversations assessments get*](api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationAssessmentGetCall), [*locations authorized view sets authorized views conversations assessments list*](api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationAssessmentListCall), [*locations authorized view sets authorized views conversations assessments notes create*](api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationAssessmentNoteCreateCall), [*locations authorized view sets authorized views conversations assessments notes delete*](api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationAssessmentNoteDeleteCall), [*locations authorized view sets authorized views conversations assessments notes list*](api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationAssessmentNoteListCall), [*locations authorized view sets authorized views conversations assessments notes patch*](api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationAssessmentNotePatchCall), [*locations authorized view sets authorized views conversations assessments publish*](api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationAssessmentPublishCall), [*locations authorized view sets authorized views conversations calculate stats*](api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationCalculateStatCall), [*locations authorized view sets authorized views conversations delete*](api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationDeleteCall), [*locations authorized view sets authorized views conversations feedback labels create*](api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationFeedbackLabelCreateCall), [*locations authorized view sets authorized views conversations feedback labels delete*](api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationFeedbackLabelDeleteCall), [*locations authorized view sets authorized views conversations feedback labels get*](api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationFeedbackLabelGetCall), [*locations authorized view sets authorized views conversations feedback labels list*](api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationFeedbackLabelListCall), [*locations authorized view sets authorized views conversations feedback labels patch*](api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationFeedbackLabelPatchCall), [*locations authorized view sets authorized views conversations generate signed audio*](api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationGenerateSignedAudioCall), [*locations authorized view sets authorized views conversations get*](api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationGetCall), [*locations authorized view sets authorized views conversations list*](api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationListCall), [*locations authorized view sets authorized views create*](api::ProjectLocationAuthorizedViewSetAuthorizedViewCreateCall), [*locations authorized view sets authorized views delete*](api::ProjectLocationAuthorizedViewSetAuthorizedViewDeleteCall), [*locations authorized view sets authorized views get*](api::ProjectLocationAuthorizedViewSetAuthorizedViewGetCall), [*locations authorized view sets authorized views get iam policy*](api::ProjectLocationAuthorizedViewSetAuthorizedViewGetIamPolicyCall), [*locations authorized view sets authorized views list*](api::ProjectLocationAuthorizedViewSetAuthorizedViewListCall), [*locations authorized view sets authorized views operations cancel*](api::ProjectLocationAuthorizedViewSetAuthorizedViewOperationCancelCall), [*locations authorized view sets authorized views operations get*](api::ProjectLocationAuthorizedViewSetAuthorizedViewOperationGetCall), [*locations authorized view sets authorized views operations list*](api::ProjectLocationAuthorizedViewSetAuthorizedViewOperationListCall), [*locations authorized view sets authorized views patch*](api::ProjectLocationAuthorizedViewSetAuthorizedViewPatchCall), [*locations authorized view sets authorized views query metrics*](api::ProjectLocationAuthorizedViewSetAuthorizedViewQueryMetricCall), [*locations authorized view sets authorized views query performance overview*](api::ProjectLocationAuthorizedViewSetAuthorizedViewQueryPerformanceOverviewCall), [*locations authorized view sets authorized views search*](api::ProjectLocationAuthorizedViewSetAuthorizedViewSearchCall), [*locations authorized view sets authorized views set iam policy*](api::ProjectLocationAuthorizedViewSetAuthorizedViewSetIamPolicyCall), [*locations authorized view sets authorized views test iam permissions*](api::ProjectLocationAuthorizedViewSetAuthorizedViewTestIamPermissionCall), [*locations authorized view sets create*](api::ProjectLocationAuthorizedViewSetCreateCall), [*locations authorized view sets delete*](api::ProjectLocationAuthorizedViewSetDeleteCall), [*locations authorized view sets get*](api::ProjectLocationAuthorizedViewSetGetCall), [*locations authorized view sets list*](api::ProjectLocationAuthorizedViewSetListCall), [*locations authorized view sets patch*](api::ProjectLocationAuthorizedViewSetPatchCall), [*locations bulk delete feedback labels*](api::ProjectLocationBulkDeleteFeedbackLabelCall), [*locations bulk download feedback labels*](api::ProjectLocationBulkDownloadFeedbackLabelCall), [*locations bulk upload feedback labels*](api::ProjectLocationBulkUploadFeedbackLabelCall), [*locations conversations analyses create*](api::ProjectLocationConversationAnalysisCreateCall), [*locations conversations analyses delete*](api::ProjectLocationConversationAnalysisDeleteCall), [*locations conversations analyses get*](api::ProjectLocationConversationAnalysisGetCall), [*locations conversations analyses list*](api::ProjectLocationConversationAnalysisListCall), [*locations conversations assessments appeal*](api::ProjectLocationConversationAssessmentAppealCall), [*locations conversations assessments create*](api::ProjectLocationConversationAssessmentCreateCall), [*locations conversations assessments delete*](api::ProjectLocationConversationAssessmentDeleteCall), [*locations conversations assessments finalize*](api::ProjectLocationConversationAssessmentFinalizeCall), [*locations conversations assessments get*](api::ProjectLocationConversationAssessmentGetCall), [*locations conversations assessments list*](api::ProjectLocationConversationAssessmentListCall), [*locations conversations assessments notes create*](api::ProjectLocationConversationAssessmentNoteCreateCall), [*locations conversations assessments notes delete*](api::ProjectLocationConversationAssessmentNoteDeleteCall), [*locations conversations assessments notes list*](api::ProjectLocationConversationAssessmentNoteListCall), [*locations conversations assessments notes patch*](api::ProjectLocationConversationAssessmentNotePatchCall), [*locations conversations assessments publish*](api::ProjectLocationConversationAssessmentPublishCall), [*locations conversations bulk analyze*](api::ProjectLocationConversationBulkAnalyzeCall), [*locations conversations bulk delete*](api::ProjectLocationConversationBulkDeleteCall), [*locations conversations calculate stats*](api::ProjectLocationConversationCalculateStatCall), [*locations conversations create*](api::ProjectLocationConversationCreateCall), [*locations conversations delete*](api::ProjectLocationConversationDeleteCall), [*locations conversations feedback labels create*](api::ProjectLocationConversationFeedbackLabelCreateCall), [*locations conversations feedback labels delete*](api::ProjectLocationConversationFeedbackLabelDeleteCall), [*locations conversations feedback labels get*](api::ProjectLocationConversationFeedbackLabelGetCall), [*locations conversations feedback labels list*](api::ProjectLocationConversationFeedbackLabelListCall), [*locations conversations feedback labels patch*](api::ProjectLocationConversationFeedbackLabelPatchCall), [*locations conversations generate signed audio*](api::ProjectLocationConversationGenerateSignedAudioCall), [*locations conversations get*](api::ProjectLocationConversationGetCall), [*locations conversations ingest*](api::ProjectLocationConversationIngestCall), [*locations conversations list*](api::ProjectLocationConversationListCall), [*locations conversations patch*](api::ProjectLocationConversationPatchCall), [*locations conversations sample*](api::ProjectLocationConversationSampleCall), [*locations conversations segments bulk analyze*](api::ProjectLocationConversationSegmentBulkAnalyzeCall), [*locations conversations upload*](api::ProjectLocationConversationUploadCall), [*locations datasets bulk delete feedback labels*](api::ProjectLocationDatasetBulkDeleteFeedbackLabelCall), [*locations datasets bulk download feedback labels*](api::ProjectLocationDatasetBulkDownloadFeedbackLabelCall), [*locations datasets bulk upload feedback labels*](api::ProjectLocationDatasetBulkUploadFeedbackLabelCall), [*locations datasets conversations bulk delete*](api::ProjectLocationDatasetConversationBulkDeleteCall), [*locations datasets conversations calculate stats*](api::ProjectLocationDatasetConversationCalculateStatCall), [*locations datasets conversations delete*](api::ProjectLocationDatasetConversationDeleteCall), [*locations datasets conversations feedback labels create*](api::ProjectLocationDatasetConversationFeedbackLabelCreateCall), [*locations datasets conversations feedback labels delete*](api::ProjectLocationDatasetConversationFeedbackLabelDeleteCall), [*locations datasets conversations feedback labels get*](api::ProjectLocationDatasetConversationFeedbackLabelGetCall), [*locations datasets conversations feedback labels list*](api::ProjectLocationDatasetConversationFeedbackLabelListCall), [*locations datasets conversations feedback labels patch*](api::ProjectLocationDatasetConversationFeedbackLabelPatchCall), [*locations datasets conversations generate signed audio*](api::ProjectLocationDatasetConversationGenerateSignedAudioCall), [*locations datasets conversations get*](api::ProjectLocationDatasetConversationGetCall), [*locations datasets conversations ingest*](api::ProjectLocationDatasetConversationIngestCall), [*locations datasets conversations list*](api::ProjectLocationDatasetConversationListCall), [*locations datasets conversations sample*](api::ProjectLocationDatasetConversationSampleCall), [*locations datasets create*](api::ProjectLocationDatasetCreateCall), [*locations datasets delete*](api::ProjectLocationDatasetDeleteCall), [*locations datasets get*](api::ProjectLocationDatasetGetCall), [*locations datasets insightsdata export*](api::ProjectLocationDatasetInsightsdataExportCall), [*locations datasets list*](api::ProjectLocationDatasetListCall), [*locations datasets list all feedback labels*](api::ProjectLocationDatasetListAllFeedbackLabelCall), [*locations datasets patch*](api::ProjectLocationDatasetPatchCall), [*locations encryption spec initialize*](api::ProjectLocationEncryptionSpecInitializeCall), [*locations get encryption spec*](api::ProjectLocationGetEncryptionSpecCall), [*locations get settings*](api::ProjectLocationGetSettingCall), [*locations insightsdata export*](api::ProjectLocationInsightsdataExportCall), [*locations issue models calculate issue model stats*](api::ProjectLocationIssueModelCalculateIssueModelStatCall), [*locations issue models create*](api::ProjectLocationIssueModelCreateCall), [*locations issue models delete*](api::ProjectLocationIssueModelDeleteCall), [*locations issue models deploy*](api::ProjectLocationIssueModelDeployCall), [*locations issue models export*](api::ProjectLocationIssueModelExportCall), [*locations issue models get*](api::ProjectLocationIssueModelGetCall), [*locations issue models import*](api::ProjectLocationIssueModelImportCall), [*locations issue models issues create*](api::ProjectLocationIssueModelIssueCreateCall), [*locations issue models issues delete*](api::ProjectLocationIssueModelIssueDeleteCall), [*locations issue models issues get*](api::ProjectLocationIssueModelIssueGetCall), [*locations issue models issues list*](api::ProjectLocationIssueModelIssueListCall), [*locations issue models issues patch*](api::ProjectLocationIssueModelIssuePatchCall), [*locations issue models list*](api::ProjectLocationIssueModelListCall), [*locations issue models patch*](api::ProjectLocationIssueModelPatchCall), [*locations issue models undeploy*](api::ProjectLocationIssueModelUndeployCall), [*locations list all feedback labels*](api::ProjectLocationListAllFeedbackLabelCall), [*locations operations cancel*](api::ProjectLocationOperationCancelCall), [*locations operations get*](api::ProjectLocationOperationGetCall), [*locations operations list*](api::ProjectLocationOperationListCall), [*locations phrase matchers create*](api::ProjectLocationPhraseMatcherCreateCall), [*locations phrase matchers delete*](api::ProjectLocationPhraseMatcherDeleteCall), [*locations phrase matchers get*](api::ProjectLocationPhraseMatcherGetCall), [*locations phrase matchers list*](api::ProjectLocationPhraseMatcherListCall), [*locations phrase matchers patch*](api::ProjectLocationPhraseMatcherPatchCall), [*locations qa question tags create*](api::ProjectLocationQaQuestionTagCreateCall), [*locations qa question tags delete*](api::ProjectLocationQaQuestionTagDeleteCall), [*locations qa question tags get*](api::ProjectLocationQaQuestionTagGetCall), [*locations qa question tags list*](api::ProjectLocationQaQuestionTagListCall), [*locations qa question tags patch*](api::ProjectLocationQaQuestionTagPatchCall), [*locations qa scorecards create*](api::ProjectLocationQaScorecardCreateCall), [*locations qa scorecards delete*](api::ProjectLocationQaScorecardDeleteCall), [*locations qa scorecards get*](api::ProjectLocationQaScorecardGetCall), [*locations qa scorecards list*](api::ProjectLocationQaScorecardListCall), [*locations qa scorecards patch*](api::ProjectLocationQaScorecardPatchCall), [*locations qa scorecards revisions create*](api::ProjectLocationQaScorecardRevisionCreateCall), [*locations qa scorecards revisions delete*](api::ProjectLocationQaScorecardRevisionDeleteCall), [*locations qa scorecards revisions deploy*](api::ProjectLocationQaScorecardRevisionDeployCall), [*locations qa scorecards revisions get*](api::ProjectLocationQaScorecardRevisionGetCall), [*locations qa scorecards revisions list*](api::ProjectLocationQaScorecardRevisionListCall), [*locations qa scorecards revisions qa questions create*](api::ProjectLocationQaScorecardRevisionQaQuestionCreateCall), [*locations qa scorecards revisions qa questions delete*](api::ProjectLocationQaScorecardRevisionQaQuestionDeleteCall), [*locations qa scorecards revisions qa questions get*](api::ProjectLocationQaScorecardRevisionQaQuestionGetCall), [*locations qa scorecards revisions qa questions list*](api::ProjectLocationQaScorecardRevisionQaQuestionListCall), [*locations qa scorecards revisions qa questions patch*](api::ProjectLocationQaScorecardRevisionQaQuestionPatchCall), [*locations qa scorecards revisions tune qa scorecard revision*](api::ProjectLocationQaScorecardRevisionTuneQaScorecardRevisionCall), [*locations qa scorecards revisions undeploy*](api::ProjectLocationQaScorecardRevisionUndeployCall), [*locations query metrics*](api::ProjectLocationQueryMetricCall), [*locations query performance overview*](api::ProjectLocationQueryPerformanceOverviewCall), [*locations update settings*](api::ProjectLocationUpdateSettingCall), [*locations views create*](api::ProjectLocationViewCreateCall), [*locations views delete*](api::ProjectLocationViewDeleteCall), [*locations views get*](api::ProjectLocationViewGetCall), [*locations views list*](api::ProjectLocationViewListCall) and [*locations views patch*](api::ProjectLocationViewPatchCall)
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
//! * **[Hub](Contactcenterinsights)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](common::MethodsBuilder) which in turn
//!       allow access to individual [*Call Builders*](common::CallBuilder)
//! * **[Resources](common::Resource)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](common::Part)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](common::CallBuilder)**
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
//! let r = hub.projects().locations_authorized_view_sets_authorized_views_operations_get(...).doit().await
//! let r = hub.projects().locations_authorized_view_sets_authorized_views_query_metrics(...).doit().await
//! let r = hub.projects().locations_authorized_view_sets_authorized_views_query_performance_overview(...).doit().await
//! let r = hub.projects().locations_conversations_analyses_create(...).doit().await
//! let r = hub.projects().locations_conversations_segments_bulk_analyze(...).doit().await
//! let r = hub.projects().locations_conversations_bulk_analyze(...).doit().await
//! let r = hub.projects().locations_conversations_bulk_delete(...).doit().await
//! let r = hub.projects().locations_conversations_ingest(...).doit().await
//! let r = hub.projects().locations_conversations_sample(...).doit().await
//! let r = hub.projects().locations_conversations_upload(...).doit().await
//! let r = hub.projects().locations_datasets_conversations_bulk_delete(...).doit().await
//! let r = hub.projects().locations_datasets_conversations_ingest(...).doit().await
//! let r = hub.projects().locations_datasets_conversations_sample(...).doit().await
//! let r = hub.projects().locations_datasets_insightsdata_export(...).doit().await
//! let r = hub.projects().locations_datasets_bulk_delete_feedback_labels(...).doit().await
//! let r = hub.projects().locations_datasets_bulk_download_feedback_labels(...).doit().await
//! let r = hub.projects().locations_datasets_bulk_upload_feedback_labels(...).doit().await
//! let r = hub.projects().locations_datasets_delete(...).doit().await
//! let r = hub.projects().locations_encryption_spec_initialize(...).doit().await
//! let r = hub.projects().locations_insightsdata_export(...).doit().await
//! let r = hub.projects().locations_issue_models_issues_create(...).doit().await
//! let r = hub.projects().locations_issue_models_create(...).doit().await
//! let r = hub.projects().locations_issue_models_delete(...).doit().await
//! let r = hub.projects().locations_issue_models_deploy(...).doit().await
//! let r = hub.projects().locations_issue_models_export(...).doit().await
//! let r = hub.projects().locations_issue_models_import(...).doit().await
//! let r = hub.projects().locations_issue_models_undeploy(...).doit().await
//! let r = hub.projects().locations_operations_get(...).doit().await
//! let r = hub.projects().locations_qa_question_tags_delete(...).doit().await
//! let r = hub.projects().locations_qa_question_tags_patch(...).doit().await
//! let r = hub.projects().locations_qa_scorecards_revisions_tune_qa_scorecard_revision(...).doit().await
//! let r = hub.projects().locations_bulk_delete_feedback_labels(...).doit().await
//! let r = hub.projects().locations_bulk_download_feedback_labels(...).doit().await
//! let r = hub.projects().locations_bulk_upload_feedback_labels(...).doit().await
//! let r = hub.projects().locations_query_metrics(...).doit().await
//! let r = hub.projects().locations_query_performance_overview(...).doit().await
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
//! google-contactcenterinsights1 = "*"
//! serde = "1"
//! serde_json = "1"
//! ```
//!
//! ## A complete example
//!
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate google_contactcenterinsights1 as contactcenterinsights1;
//! use contactcenterinsights1::api::GoogleCloudContactcenterinsightsV1QaQuestionTag;
//! use contactcenterinsights1::{Result, Error};
//! # async fn dox() {
//! use contactcenterinsights1::{Contactcenterinsights, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
//!
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and
//! // `client_secret`, among other things.
//! let secret: yup_oauth2::ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you,
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let connector = hyper_rustls::HttpsConnectorBuilder::new()
//!     .with_native_roots()
//!     .unwrap()
//!     .https_only()
//!     .enable_http2()
//!     .build();
//!
//! let executor = hyper_util::rt::TokioExecutor::new();
//! let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
//!     secret,
//!     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
//!     yup_oauth2::client::CustomHyperClientBuilder::from(
//!         hyper_util::client::legacy::Client::builder(executor).build(connector),
//!     ),
//! ).build().await.unwrap();
//!
//! let client = hyper_util::client::legacy::Client::builder(
//!     hyper_util::rt::TokioExecutor::new()
//! )
//! .build(
//!     hyper_rustls::HttpsConnectorBuilder::new()
//!         .with_native_roots()
//!         .unwrap()
//!         .https_or_http()
//!         .enable_http2()
//!         .build()
//! );
//! let mut hub = Contactcenterinsights::new(client, auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = GoogleCloudContactcenterinsightsV1QaQuestionTag::default();
//!
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.projects().locations_qa_question_tags_patch(req, "name")
//!              .update_mask(FieldMask::new::<&str>(&[]))
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
//! All errors produced by the system are provided either as [Result](common::Result) enumeration as return value of
//! the doit() methods, or handed as possibly intermediate results to either the
//! [Hub Delegate](common::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).
//!
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This
//! makes the system potentially resilient to all kinds of errors.
//!
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](common::Result), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](common::ResponseResult), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//!
//! Methods supporting uploads can do so using up to 2 different protocols:
//! *simple* and *resumable*. The distinctiveness of each is represented by customized
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//!
//! ## Customization and Callbacks
//!
//! You may alter the way an `doit()` method is called by providing a [delegate](common::Delegate) to the
//! [Method Builder](common::CallBuilder) before making the final `doit()` call.
//! Respective methods will be called to provide progress information, as well as determine whether the system should
//! retry on failure.
//!
//! The [delegate trait](common::Delegate) is default-implemented, allowing you to customize it with minimal effort.
//!
//! ## Optional Parts in Server-Requests
//!
//! All structures provided by this library are made to be [encodable](common::RequestValue) and
//! [decodable](common::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses
//! are valid.
//! Most optionals are are considered [Parts](common::Part) which are identifiable by name, which will be sent to
//! the server to indicate either the set parts of the request or the desired parts in the response.
//!
//! ## Builder Arguments
//!
//! Using [method builders](common::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//!
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](common::RequestValue) are moved
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

// Unused attributes happen thanks to defined, but unused structures We don't
// warn about this, as depending on the API, some data structures or facilities
// are never used. Instead of pre-determining this, we just disable the lint.
// It's manually tuned to not have any unused imports in fully featured APIs.
// Same with unused_mut.
#![allow(unused_imports, unused_mut, dead_code)]

// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

pub extern crate hyper;
pub extern crate hyper_rustls;
pub extern crate hyper_util;
#[cfg(feature = "yup-oauth2")]
pub extern crate yup_oauth2;

pub extern crate google_apis_common as common;
pub use common::{Delegate, Error, FieldMask, Result};

pub mod api;
pub use api::Contactcenterinsights;
