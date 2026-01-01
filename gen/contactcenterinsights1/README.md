<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-contactcenterinsights1` library allows access to all features of the *Google Contactcenterinsights* service.

This documentation was generated from *Contactcenterinsights* crate version *7.0.0+20251222*, where *20251222* is the exact revision of the *contactcenterinsights:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v7.0.0*.

Everything else about the *Contactcenterinsights* *v1* API can be found at the
[official documentation site](https://cloud.google.com/contact-center/insights/docs).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/Contactcenterinsights) ...

* projects
 * [*locations analysis rules create*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAnalysisRuleCreateCall), [*locations analysis rules delete*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAnalysisRuleDeleteCall), [*locations analysis rules get*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAnalysisRuleGetCall), [*locations analysis rules list*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAnalysisRuleListCall), [*locations analysis rules patch*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAnalysisRulePatchCall), [*locations assessment rules create*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAssessmentRuleCreateCall), [*locations assessment rules delete*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAssessmentRuleDeleteCall), [*locations assessment rules get*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAssessmentRuleGetCall), [*locations assessment rules list*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAssessmentRuleListCall), [*locations assessment rules patch*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAssessmentRulePatchCall), [*locations authorized view sets authorized views conversations assessments appeal*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationAssessmentAppealCall), [*locations authorized view sets authorized views conversations assessments create*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationAssessmentCreateCall), [*locations authorized view sets authorized views conversations assessments delete*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationAssessmentDeleteCall), [*locations authorized view sets authorized views conversations assessments finalize*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationAssessmentFinalizeCall), [*locations authorized view sets authorized views conversations assessments get*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationAssessmentGetCall), [*locations authorized view sets authorized views conversations assessments list*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationAssessmentListCall), [*locations authorized view sets authorized views conversations assessments notes create*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationAssessmentNoteCreateCall), [*locations authorized view sets authorized views conversations assessments notes delete*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationAssessmentNoteDeleteCall), [*locations authorized view sets authorized views conversations assessments notes list*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationAssessmentNoteListCall), [*locations authorized view sets authorized views conversations assessments notes patch*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationAssessmentNotePatchCall), [*locations authorized view sets authorized views conversations assessments publish*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationAssessmentPublishCall), [*locations authorized view sets authorized views conversations calculate stats*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationCalculateStatCall), [*locations authorized view sets authorized views conversations delete*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationDeleteCall), [*locations authorized view sets authorized views conversations feedback labels create*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationFeedbackLabelCreateCall), [*locations authorized view sets authorized views conversations feedback labels delete*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationFeedbackLabelDeleteCall), [*locations authorized view sets authorized views conversations feedback labels get*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationFeedbackLabelGetCall), [*locations authorized view sets authorized views conversations feedback labels list*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationFeedbackLabelListCall), [*locations authorized view sets authorized views conversations feedback labels patch*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationFeedbackLabelPatchCall), [*locations authorized view sets authorized views conversations generate signed audio*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationGenerateSignedAudioCall), [*locations authorized view sets authorized views conversations get*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationGetCall), [*locations authorized view sets authorized views conversations list*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetAuthorizedViewConversationListCall), [*locations authorized view sets authorized views create*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetAuthorizedViewCreateCall), [*locations authorized view sets authorized views delete*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetAuthorizedViewDeleteCall), [*locations authorized view sets authorized views get*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetAuthorizedViewGetCall), [*locations authorized view sets authorized views get iam policy*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetAuthorizedViewGetIamPolicyCall), [*locations authorized view sets authorized views list*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetAuthorizedViewListCall), [*locations authorized view sets authorized views operations cancel*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetAuthorizedViewOperationCancelCall), [*locations authorized view sets authorized views operations get*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetAuthorizedViewOperationGetCall), [*locations authorized view sets authorized views operations list*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetAuthorizedViewOperationListCall), [*locations authorized view sets authorized views patch*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetAuthorizedViewPatchCall), [*locations authorized view sets authorized views query metrics*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetAuthorizedViewQueryMetricCall), [*locations authorized view sets authorized views query performance overview*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetAuthorizedViewQueryPerformanceOverviewCall), [*locations authorized view sets authorized views search*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetAuthorizedViewSearchCall), [*locations authorized view sets authorized views set iam policy*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetAuthorizedViewSetIamPolicyCall), [*locations authorized view sets authorized views test iam permissions*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetAuthorizedViewTestIamPermissionCall), [*locations authorized view sets create*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetCreateCall), [*locations authorized view sets delete*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetDeleteCall), [*locations authorized view sets get*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetGetCall), [*locations authorized view sets list*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetListCall), [*locations authorized view sets patch*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationAuthorizedViewSetPatchCall), [*locations bulk delete feedback labels*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationBulkDeleteFeedbackLabelCall), [*locations bulk download feedback labels*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationBulkDownloadFeedbackLabelCall), [*locations bulk upload feedback labels*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationBulkUploadFeedbackLabelCall), [*locations conversations analyses create*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationConversationAnalysisCreateCall), [*locations conversations analyses delete*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationConversationAnalysisDeleteCall), [*locations conversations analyses get*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationConversationAnalysisGetCall), [*locations conversations analyses list*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationConversationAnalysisListCall), [*locations conversations assessments appeal*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationConversationAssessmentAppealCall), [*locations conversations assessments create*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationConversationAssessmentCreateCall), [*locations conversations assessments delete*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationConversationAssessmentDeleteCall), [*locations conversations assessments finalize*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationConversationAssessmentFinalizeCall), [*locations conversations assessments get*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationConversationAssessmentGetCall), [*locations conversations assessments list*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationConversationAssessmentListCall), [*locations conversations assessments notes create*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationConversationAssessmentNoteCreateCall), [*locations conversations assessments notes delete*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationConversationAssessmentNoteDeleteCall), [*locations conversations assessments notes list*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationConversationAssessmentNoteListCall), [*locations conversations assessments notes patch*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationConversationAssessmentNotePatchCall), [*locations conversations assessments publish*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationConversationAssessmentPublishCall), [*locations conversations bulk analyze*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationConversationBulkAnalyzeCall), [*locations conversations bulk delete*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationConversationBulkDeleteCall), [*locations conversations calculate stats*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationConversationCalculateStatCall), [*locations conversations create*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationConversationCreateCall), [*locations conversations delete*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationConversationDeleteCall), [*locations conversations feedback labels create*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationConversationFeedbackLabelCreateCall), [*locations conversations feedback labels delete*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationConversationFeedbackLabelDeleteCall), [*locations conversations feedback labels get*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationConversationFeedbackLabelGetCall), [*locations conversations feedback labels list*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationConversationFeedbackLabelListCall), [*locations conversations feedback labels patch*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationConversationFeedbackLabelPatchCall), [*locations conversations generate signed audio*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationConversationGenerateSignedAudioCall), [*locations conversations get*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationConversationGetCall), [*locations conversations ingest*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationConversationIngestCall), [*locations conversations list*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationConversationListCall), [*locations conversations patch*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationConversationPatchCall), [*locations conversations sample*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationConversationSampleCall), [*locations conversations segments bulk analyze*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationConversationSegmentBulkAnalyzeCall), [*locations conversations upload*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationConversationUploadCall), [*locations datasets bulk delete feedback labels*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationDatasetBulkDeleteFeedbackLabelCall), [*locations datasets bulk download feedback labels*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationDatasetBulkDownloadFeedbackLabelCall), [*locations datasets bulk upload feedback labels*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationDatasetBulkUploadFeedbackLabelCall), [*locations datasets conversations bulk delete*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationDatasetConversationBulkDeleteCall), [*locations datasets conversations calculate stats*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationDatasetConversationCalculateStatCall), [*locations datasets conversations delete*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationDatasetConversationDeleteCall), [*locations datasets conversations feedback labels create*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationDatasetConversationFeedbackLabelCreateCall), [*locations datasets conversations feedback labels delete*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationDatasetConversationFeedbackLabelDeleteCall), [*locations datasets conversations feedback labels get*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationDatasetConversationFeedbackLabelGetCall), [*locations datasets conversations feedback labels list*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationDatasetConversationFeedbackLabelListCall), [*locations datasets conversations feedback labels patch*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationDatasetConversationFeedbackLabelPatchCall), [*locations datasets conversations generate signed audio*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationDatasetConversationGenerateSignedAudioCall), [*locations datasets conversations get*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationDatasetConversationGetCall), [*locations datasets conversations ingest*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationDatasetConversationIngestCall), [*locations datasets conversations list*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationDatasetConversationListCall), [*locations datasets conversations sample*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationDatasetConversationSampleCall), [*locations datasets create*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationDatasetCreateCall), [*locations datasets delete*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationDatasetDeleteCall), [*locations datasets get*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationDatasetGetCall), [*locations datasets insightsdata export*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationDatasetInsightsdataExportCall), [*locations datasets list*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationDatasetListCall), [*locations datasets list all feedback labels*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationDatasetListAllFeedbackLabelCall), [*locations datasets patch*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationDatasetPatchCall), [*locations encryption spec initialize*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationEncryptionSpecInitializeCall), [*locations get encryption spec*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationGetEncryptionSpecCall), [*locations get settings*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationGetSettingCall), [*locations insightsdata export*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationInsightsdataExportCall), [*locations issue models calculate issue model stats*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationIssueModelCalculateIssueModelStatCall), [*locations issue models create*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationIssueModelCreateCall), [*locations issue models delete*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationIssueModelDeleteCall), [*locations issue models deploy*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationIssueModelDeployCall), [*locations issue models export*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationIssueModelExportCall), [*locations issue models get*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationIssueModelGetCall), [*locations issue models import*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationIssueModelImportCall), [*locations issue models issues create*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationIssueModelIssueCreateCall), [*locations issue models issues delete*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationIssueModelIssueDeleteCall), [*locations issue models issues get*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationIssueModelIssueGetCall), [*locations issue models issues list*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationIssueModelIssueListCall), [*locations issue models issues patch*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationIssueModelIssuePatchCall), [*locations issue models list*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationIssueModelListCall), [*locations issue models patch*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationIssueModelPatchCall), [*locations issue models undeploy*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationIssueModelUndeployCall), [*locations list all feedback labels*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationListAllFeedbackLabelCall), [*locations operations cancel*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationOperationCancelCall), [*locations operations get*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationOperationGetCall), [*locations operations list*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationOperationListCall), [*locations phrase matchers create*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationPhraseMatcherCreateCall), [*locations phrase matchers delete*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationPhraseMatcherDeleteCall), [*locations phrase matchers get*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationPhraseMatcherGetCall), [*locations phrase matchers list*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationPhraseMatcherListCall), [*locations phrase matchers patch*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationPhraseMatcherPatchCall), [*locations qa question tags create*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationQaQuestionTagCreateCall), [*locations qa question tags delete*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationQaQuestionTagDeleteCall), [*locations qa question tags get*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationQaQuestionTagGetCall), [*locations qa question tags list*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationQaQuestionTagListCall), [*locations qa question tags patch*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationQaQuestionTagPatchCall), [*locations qa scorecards create*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationQaScorecardCreateCall), [*locations qa scorecards delete*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationQaScorecardDeleteCall), [*locations qa scorecards get*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationQaScorecardGetCall), [*locations qa scorecards list*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationQaScorecardListCall), [*locations qa scorecards patch*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationQaScorecardPatchCall), [*locations qa scorecards revisions create*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationQaScorecardRevisionCreateCall), [*locations qa scorecards revisions delete*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationQaScorecardRevisionDeleteCall), [*locations qa scorecards revisions deploy*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationQaScorecardRevisionDeployCall), [*locations qa scorecards revisions get*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationQaScorecardRevisionGetCall), [*locations qa scorecards revisions list*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationQaScorecardRevisionListCall), [*locations qa scorecards revisions qa questions create*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationQaScorecardRevisionQaQuestionCreateCall), [*locations qa scorecards revisions qa questions delete*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationQaScorecardRevisionQaQuestionDeleteCall), [*locations qa scorecards revisions qa questions get*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationQaScorecardRevisionQaQuestionGetCall), [*locations qa scorecards revisions qa questions list*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationQaScorecardRevisionQaQuestionListCall), [*locations qa scorecards revisions qa questions patch*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationQaScorecardRevisionQaQuestionPatchCall), [*locations qa scorecards revisions tune qa scorecard revision*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationQaScorecardRevisionTuneQaScorecardRevisionCall), [*locations qa scorecards revisions undeploy*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationQaScorecardRevisionUndeployCall), [*locations query metrics*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationQueryMetricCall), [*locations query performance overview*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationQueryPerformanceOverviewCall), [*locations update settings*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationUpdateSettingCall), [*locations views create*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationViewCreateCall), [*locations views delete*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationViewDeleteCall), [*locations views get*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationViewGetCall), [*locations views list*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationViewListCall) and [*locations views patch*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/api::ProjectLocationViewPatchCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/Contactcenterinsights)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/common::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/common::CallBuilder)
* **[Resources](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/common::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/common::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/common::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.projects().locations_authorized_view_sets_authorized_views_operations_get(...).doit().await
let r = hub.projects().locations_authorized_view_sets_authorized_views_query_metrics(...).doit().await
let r = hub.projects().locations_authorized_view_sets_authorized_views_query_performance_overview(...).doit().await
let r = hub.projects().locations_conversations_analyses_create(...).doit().await
let r = hub.projects().locations_conversations_segments_bulk_analyze(...).doit().await
let r = hub.projects().locations_conversations_bulk_analyze(...).doit().await
let r = hub.projects().locations_conversations_bulk_delete(...).doit().await
let r = hub.projects().locations_conversations_ingest(...).doit().await
let r = hub.projects().locations_conversations_sample(...).doit().await
let r = hub.projects().locations_conversations_upload(...).doit().await
let r = hub.projects().locations_datasets_conversations_bulk_delete(...).doit().await
let r = hub.projects().locations_datasets_conversations_ingest(...).doit().await
let r = hub.projects().locations_datasets_conversations_sample(...).doit().await
let r = hub.projects().locations_datasets_insightsdata_export(...).doit().await
let r = hub.projects().locations_datasets_bulk_delete_feedback_labels(...).doit().await
let r = hub.projects().locations_datasets_bulk_download_feedback_labels(...).doit().await
let r = hub.projects().locations_datasets_bulk_upload_feedback_labels(...).doit().await
let r = hub.projects().locations_datasets_delete(...).doit().await
let r = hub.projects().locations_encryption_spec_initialize(...).doit().await
let r = hub.projects().locations_insightsdata_export(...).doit().await
let r = hub.projects().locations_issue_models_issues_create(...).doit().await
let r = hub.projects().locations_issue_models_create(...).doit().await
let r = hub.projects().locations_issue_models_delete(...).doit().await
let r = hub.projects().locations_issue_models_deploy(...).doit().await
let r = hub.projects().locations_issue_models_export(...).doit().await
let r = hub.projects().locations_issue_models_import(...).doit().await
let r = hub.projects().locations_issue_models_undeploy(...).doit().await
let r = hub.projects().locations_operations_get(...).doit().await
let r = hub.projects().locations_qa_question_tags_delete(...).doit().await
let r = hub.projects().locations_qa_question_tags_patch(...).doit().await
let r = hub.projects().locations_qa_scorecards_revisions_tune_qa_scorecard_revision(...).doit().await
let r = hub.projects().locations_bulk_delete_feedback_labels(...).doit().await
let r = hub.projects().locations_bulk_download_feedback_labels(...).doit().await
let r = hub.projects().locations_bulk_upload_feedback_labels(...).doit().await
let r = hub.projects().locations_query_metrics(...).doit().await
let r = hub.projects().locations_query_performance_overview(...).doit().await
```

The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities`
supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be
specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
The `doit()` method performs the actual communication with the server and returns the respective result.

# Usage

## Setting up your Project

To use this library, you would put the following lines into your `Cargo.toml` file:

```toml
[dependencies]
google-contactcenterinsights1 = "*"
serde = "1"
serde_json = "1"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_contactcenterinsights1 as contactcenterinsights1;
use contactcenterinsights1::api::GoogleCloudContactcenterinsightsV1QaQuestionTag;
use contactcenterinsights1::{Result, Error};
use contactcenterinsights1::{Contactcenterinsights, FieldMask, hyper_rustls, hyper_util, yup_oauth2};

// Get an ApplicationSecret instance by some means. It contains the `client_id` and
// `client_secret`, among other things.
let secret: yup_oauth2::ApplicationSecret = Default::default();
// Instantiate the authenticator. It will choose a suitable authentication flow for you,
// unless you replace  `None` with the desired Flow.
// Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about
// what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
// retrieve them from storage.
let connector = hyper_rustls::HttpsConnectorBuilder::new()
    .with_native_roots()
    .unwrap()
    .https_only()
    .enable_http2()
    .build();

let executor = hyper_util::rt::TokioExecutor::new();
let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
    secret,
    yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    yup_oauth2::client::CustomHyperClientBuilder::from(
        hyper_util::client::legacy::Client::builder(executor).build(connector),
    ),
).build().await.unwrap();

let client = hyper_util::client::legacy::Client::builder(
    hyper_util::rt::TokioExecutor::new()
)
.build(
    hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .unwrap()
        .https_or_http()
        .enable_http2()
        .build()
);
let mut hub = Contactcenterinsights::new(client, auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = GoogleCloudContactcenterinsightsV1QaQuestionTag::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.projects().locations_qa_question_tags_patch(req, "name")
             .update_mask(FieldMask::new::<&str>(&[]))
             .doit().await;

match result {
    Err(e) => match e {
        // The Error enum provides details about what exactly happened.
        // You can also just use its `Debug`, `Display` or `Error` traits
         Error::HttpError(_)
        |Error::Io(_)
        |Error::MissingAPIKey
        |Error::MissingToken(_)
        |Error::Cancelled
        |Error::UploadSizeLimitExceeded(_, _)
        |Error::Failure(_)
        |Error::BadRequest(_)
        |Error::FieldClash(_)
        |Error::JsonDecodeError(_, _) => println!("{}", e),
    },
    Ok(res) => println!("Success: {:?}", res),
}

```
## Handling Errors

All errors produced by the system are provided either as [Result](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/common::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the
[Hub Delegate](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/common::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/common::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/common::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols:
*simple* and *resumable*. The distinctiveness of each is represented by customized
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/common::Delegate) to the
[Method Builder](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/common::CallBuilder) before making the final `doit()` call.
Respective methods will be called to provide progress information, as well as determine whether the system should
retry on failure.

The [delegate trait](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/common::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/common::RequestValue) and
[decodable](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/common::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/common::Part) which are identifiable by name, which will be sent to
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/common::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-contactcenterinsights1/7.0.0+20251222/google_contactcenterinsights1/common::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **contactcenterinsights1** library was generated by Sebastian Thiel, and is placed
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

