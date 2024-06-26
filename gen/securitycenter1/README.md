<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-securitycenter1` library allows access to all features of the *Google Security Command Center* service.

This documentation was generated from *Security Command Center* crate version *5.0.5+20240622*, where *20240622* is the exact revision of the *securitycenter:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *Security Command Center* *v1* API can be found at the
[official documentation site](https://cloud.google.com/security-command-center).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/SecurityCommandCenter) ... 

* [folders](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::Folder)
 * [*assets group*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderAssetGroupCall), [*assets list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderAssetListCall), [*assets update security marks*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderAssetUpdateSecurityMarkCall), [*big query exports create*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderBigQueryExportCreateCall), [*big query exports delete*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderBigQueryExportDeleteCall), [*big query exports get*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderBigQueryExportGetCall), [*big query exports list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderBigQueryExportListCall), [*big query exports patch*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderBigQueryExportPatchCall), [*event threat detection settings custom modules create*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderEventThreatDetectionSettingCustomModuleCreateCall), [*event threat detection settings custom modules delete*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderEventThreatDetectionSettingCustomModuleDeleteCall), [*event threat detection settings custom modules get*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderEventThreatDetectionSettingCustomModuleGetCall), [*event threat detection settings custom modules list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderEventThreatDetectionSettingCustomModuleListCall), [*event threat detection settings custom modules list descendant*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderEventThreatDetectionSettingCustomModuleListDescendantCall), [*event threat detection settings custom modules patch*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderEventThreatDetectionSettingCustomModulePatchCall), [*event threat detection settings effective custom modules get*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderEventThreatDetectionSettingEffectiveCustomModuleGetCall), [*event threat detection settings effective custom modules list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderEventThreatDetectionSettingEffectiveCustomModuleListCall), [*event threat detection settings validate custom module*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderEventThreatDetectionSettingValidateCustomModuleCall), [*findings bulk mute*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderFindingBulkMuteCall), [*locations mute configs create*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderLocationMuteConfigCreateCall), [*locations mute configs delete*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderLocationMuteConfigDeleteCall), [*locations mute configs get*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderLocationMuteConfigGetCall), [*locations mute configs list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderLocationMuteConfigListCall), [*locations mute configs patch*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderLocationMuteConfigPatchCall), [*mute configs create*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderMuteConfigCreateCall), [*mute configs delete*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderMuteConfigDeleteCall), [*mute configs get*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderMuteConfigGetCall), [*mute configs list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderMuteConfigListCall), [*mute configs patch*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderMuteConfigPatchCall), [*notification configs create*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderNotificationConfigCreateCall), [*notification configs delete*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderNotificationConfigDeleteCall), [*notification configs get*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderNotificationConfigGetCall), [*notification configs list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderNotificationConfigListCall), [*notification configs patch*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderNotificationConfigPatchCall), [*security health analytics settings custom modules create*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderSecurityHealthAnalyticsSettingCustomModuleCreateCall), [*security health analytics settings custom modules delete*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderSecurityHealthAnalyticsSettingCustomModuleDeleteCall), [*security health analytics settings custom modules get*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderSecurityHealthAnalyticsSettingCustomModuleGetCall), [*security health analytics settings custom modules list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderSecurityHealthAnalyticsSettingCustomModuleListCall), [*security health analytics settings custom modules list descendant*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderSecurityHealthAnalyticsSettingCustomModuleListDescendantCall), [*security health analytics settings custom modules patch*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderSecurityHealthAnalyticsSettingCustomModulePatchCall), [*security health analytics settings custom modules simulate*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderSecurityHealthAnalyticsSettingCustomModuleSimulateCall), [*security health analytics settings effective custom modules get*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderSecurityHealthAnalyticsSettingEffectiveCustomModuleGetCall), [*security health analytics settings effective custom modules list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderSecurityHealthAnalyticsSettingEffectiveCustomModuleListCall), [*sources findings external systems patch*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderSourceFindingExternalSystemPatchCall), [*sources findings group*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderSourceFindingGroupCall), [*sources findings list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderSourceFindingListCall), [*sources findings patch*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderSourceFindingPatchCall), [*sources findings set mute*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderSourceFindingSetMuteCall), [*sources findings set state*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderSourceFindingSetStateCall), [*sources findings update security marks*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderSourceFindingUpdateSecurityMarkCall) and [*sources list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::FolderSourceListCall)
* organizations
 * [*assets group*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationAssetGroupCall), [*assets list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationAssetListCall), [*assets run discovery*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationAssetRunDiscoveryCall), [*assets update security marks*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationAssetUpdateSecurityMarkCall), [*big query exports create*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationBigQueryExportCreateCall), [*big query exports delete*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationBigQueryExportDeleteCall), [*big query exports get*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationBigQueryExportGetCall), [*big query exports list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationBigQueryExportListCall), [*big query exports patch*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationBigQueryExportPatchCall), [*event threat detection settings custom modules create*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationEventThreatDetectionSettingCustomModuleCreateCall), [*event threat detection settings custom modules delete*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationEventThreatDetectionSettingCustomModuleDeleteCall), [*event threat detection settings custom modules get*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationEventThreatDetectionSettingCustomModuleGetCall), [*event threat detection settings custom modules list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationEventThreatDetectionSettingCustomModuleListCall), [*event threat detection settings custom modules list descendant*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationEventThreatDetectionSettingCustomModuleListDescendantCall), [*event threat detection settings custom modules patch*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationEventThreatDetectionSettingCustomModulePatchCall), [*event threat detection settings effective custom modules get*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationEventThreatDetectionSettingEffectiveCustomModuleGetCall), [*event threat detection settings effective custom modules list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationEventThreatDetectionSettingEffectiveCustomModuleListCall), [*event threat detection settings validate custom module*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationEventThreatDetectionSettingValidateCustomModuleCall), [*findings bulk mute*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationFindingBulkMuteCall), [*get organization settings*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationGetOrganizationSettingCall), [*locations mute configs create*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationLocationMuteConfigCreateCall), [*locations mute configs delete*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationLocationMuteConfigDeleteCall), [*locations mute configs get*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationLocationMuteConfigGetCall), [*locations mute configs list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationLocationMuteConfigListCall), [*locations mute configs patch*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationLocationMuteConfigPatchCall), [*mute configs create*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationMuteConfigCreateCall), [*mute configs delete*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationMuteConfigDeleteCall), [*mute configs get*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationMuteConfigGetCall), [*mute configs list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationMuteConfigListCall), [*mute configs patch*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationMuteConfigPatchCall), [*notification configs create*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationNotificationConfigCreateCall), [*notification configs delete*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationNotificationConfigDeleteCall), [*notification configs get*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationNotificationConfigGetCall), [*notification configs list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationNotificationConfigListCall), [*notification configs patch*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationNotificationConfigPatchCall), [*operations cancel*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationOperationCancelCall), [*operations delete*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationOperationDeleteCall), [*operations get*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationOperationGetCall), [*operations list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationOperationListCall), [*resource value configs batch create*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationResourceValueConfigBatchCreateCall), [*resource value configs delete*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationResourceValueConfigDeleteCall), [*resource value configs get*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationResourceValueConfigGetCall), [*resource value configs list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationResourceValueConfigListCall), [*resource value configs patch*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationResourceValueConfigPatchCall), [*security health analytics settings custom modules create*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationSecurityHealthAnalyticsSettingCustomModuleCreateCall), [*security health analytics settings custom modules delete*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationSecurityHealthAnalyticsSettingCustomModuleDeleteCall), [*security health analytics settings custom modules get*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationSecurityHealthAnalyticsSettingCustomModuleGetCall), [*security health analytics settings custom modules list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationSecurityHealthAnalyticsSettingCustomModuleListCall), [*security health analytics settings custom modules list descendant*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationSecurityHealthAnalyticsSettingCustomModuleListDescendantCall), [*security health analytics settings custom modules patch*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationSecurityHealthAnalyticsSettingCustomModulePatchCall), [*security health analytics settings custom modules simulate*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationSecurityHealthAnalyticsSettingCustomModuleSimulateCall), [*security health analytics settings effective custom modules get*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationSecurityHealthAnalyticsSettingEffectiveCustomModuleGetCall), [*security health analytics settings effective custom modules list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationSecurityHealthAnalyticsSettingEffectiveCustomModuleListCall), [*simulations attack exposure results attack paths list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationSimulationAttackExposureResultAttackPathListCall), [*simulations attack exposure results valued resources list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationSimulationAttackExposureResultValuedResourceListCall), [*simulations attack paths list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationSimulationAttackPathListCall), [*simulations get*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationSimulationGetCall), [*simulations valued resources attack paths list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationSimulationValuedResourceAttackPathListCall), [*simulations valued resources get*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationSimulationValuedResourceGetCall), [*simulations valued resources list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationSimulationValuedResourceListCall), [*sources create*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationSourceCreateCall), [*sources findings create*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationSourceFindingCreateCall), [*sources findings external systems patch*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationSourceFindingExternalSystemPatchCall), [*sources findings group*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationSourceFindingGroupCall), [*sources findings list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationSourceFindingListCall), [*sources findings patch*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationSourceFindingPatchCall), [*sources findings set mute*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationSourceFindingSetMuteCall), [*sources findings set state*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationSourceFindingSetStateCall), [*sources findings update security marks*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationSourceFindingUpdateSecurityMarkCall), [*sources get*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationSourceGetCall), [*sources get iam policy*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationSourceGetIamPolicyCall), [*sources list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationSourceListCall), [*sources patch*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationSourcePatchCall), [*sources set iam policy*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationSourceSetIamPolicyCall), [*sources test iam permissions*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationSourceTestIamPermissionCall) and [*update organization settings*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::OrganizationUpdateOrganizationSettingCall)
* projects
 * [*assets group*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectAssetGroupCall), [*assets list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectAssetListCall), [*assets update security marks*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectAssetUpdateSecurityMarkCall), [*big query exports create*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectBigQueryExportCreateCall), [*big query exports delete*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectBigQueryExportDeleteCall), [*big query exports get*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectBigQueryExportGetCall), [*big query exports list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectBigQueryExportListCall), [*big query exports patch*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectBigQueryExportPatchCall), [*event threat detection settings custom modules create*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectEventThreatDetectionSettingCustomModuleCreateCall), [*event threat detection settings custom modules delete*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectEventThreatDetectionSettingCustomModuleDeleteCall), [*event threat detection settings custom modules get*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectEventThreatDetectionSettingCustomModuleGetCall), [*event threat detection settings custom modules list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectEventThreatDetectionSettingCustomModuleListCall), [*event threat detection settings custom modules list descendant*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectEventThreatDetectionSettingCustomModuleListDescendantCall), [*event threat detection settings custom modules patch*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectEventThreatDetectionSettingCustomModulePatchCall), [*event threat detection settings effective custom modules get*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectEventThreatDetectionSettingEffectiveCustomModuleGetCall), [*event threat detection settings effective custom modules list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectEventThreatDetectionSettingEffectiveCustomModuleListCall), [*event threat detection settings validate custom module*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectEventThreatDetectionSettingValidateCustomModuleCall), [*findings bulk mute*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectFindingBulkMuteCall), [*locations mute configs create*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectLocationMuteConfigCreateCall), [*locations mute configs delete*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectLocationMuteConfigDeleteCall), [*locations mute configs get*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectLocationMuteConfigGetCall), [*locations mute configs list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectLocationMuteConfigListCall), [*locations mute configs patch*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectLocationMuteConfigPatchCall), [*mute configs create*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectMuteConfigCreateCall), [*mute configs delete*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectMuteConfigDeleteCall), [*mute configs get*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectMuteConfigGetCall), [*mute configs list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectMuteConfigListCall), [*mute configs patch*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectMuteConfigPatchCall), [*notification configs create*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectNotificationConfigCreateCall), [*notification configs delete*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectNotificationConfigDeleteCall), [*notification configs get*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectNotificationConfigGetCall), [*notification configs list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectNotificationConfigListCall), [*notification configs patch*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectNotificationConfigPatchCall), [*security health analytics settings custom modules create*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectSecurityHealthAnalyticsSettingCustomModuleCreateCall), [*security health analytics settings custom modules delete*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectSecurityHealthAnalyticsSettingCustomModuleDeleteCall), [*security health analytics settings custom modules get*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectSecurityHealthAnalyticsSettingCustomModuleGetCall), [*security health analytics settings custom modules list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectSecurityHealthAnalyticsSettingCustomModuleListCall), [*security health analytics settings custom modules list descendant*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectSecurityHealthAnalyticsSettingCustomModuleListDescendantCall), [*security health analytics settings custom modules patch*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectSecurityHealthAnalyticsSettingCustomModulePatchCall), [*security health analytics settings custom modules simulate*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectSecurityHealthAnalyticsSettingCustomModuleSimulateCall), [*security health analytics settings effective custom modules get*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectSecurityHealthAnalyticsSettingEffectiveCustomModuleGetCall), [*security health analytics settings effective custom modules list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectSecurityHealthAnalyticsSettingEffectiveCustomModuleListCall), [*sources findings external systems patch*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectSourceFindingExternalSystemPatchCall), [*sources findings group*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectSourceFindingGroupCall), [*sources findings list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectSourceFindingListCall), [*sources findings patch*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectSourceFindingPatchCall), [*sources findings set mute*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectSourceFindingSetMuteCall), [*sources findings set state*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectSourceFindingSetStateCall), [*sources findings update security marks*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectSourceFindingUpdateSecurityMarkCall) and [*sources list*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/api::ProjectSourceListCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/SecurityCommandCenter)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/client::CallBuilder)
* **[Resources](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.folders().assets_group(...).doit().await
let r = hub.folders().assets_list(...).doit().await
let r = hub.folders().assets_update_security_marks(...).doit().await
let r = hub.folders().big_query_exports_create(...).doit().await
let r = hub.folders().big_query_exports_delete(...).doit().await
let r = hub.folders().big_query_exports_get(...).doit().await
let r = hub.folders().big_query_exports_list(...).doit().await
let r = hub.folders().big_query_exports_patch(...).doit().await
let r = hub.folders().event_threat_detection_settings_custom_modules_create(...).doit().await
let r = hub.folders().event_threat_detection_settings_custom_modules_delete(...).doit().await
let r = hub.folders().event_threat_detection_settings_custom_modules_get(...).doit().await
let r = hub.folders().event_threat_detection_settings_custom_modules_list(...).doit().await
let r = hub.folders().event_threat_detection_settings_custom_modules_list_descendant(...).doit().await
let r = hub.folders().event_threat_detection_settings_custom_modules_patch(...).doit().await
let r = hub.folders().event_threat_detection_settings_effective_custom_modules_get(...).doit().await
let r = hub.folders().event_threat_detection_settings_effective_custom_modules_list(...).doit().await
let r = hub.folders().event_threat_detection_settings_validate_custom_module(...).doit().await
let r = hub.folders().findings_bulk_mute(...).doit().await
let r = hub.folders().locations_mute_configs_create(...).doit().await
let r = hub.folders().locations_mute_configs_delete(...).doit().await
let r = hub.folders().locations_mute_configs_get(...).doit().await
let r = hub.folders().locations_mute_configs_list(...).doit().await
let r = hub.folders().locations_mute_configs_patch(...).doit().await
let r = hub.folders().mute_configs_create(...).doit().await
let r = hub.folders().mute_configs_delete(...).doit().await
let r = hub.folders().mute_configs_get(...).doit().await
let r = hub.folders().mute_configs_list(...).doit().await
let r = hub.folders().mute_configs_patch(...).doit().await
let r = hub.folders().notification_configs_create(...).doit().await
let r = hub.folders().notification_configs_delete(...).doit().await
let r = hub.folders().notification_configs_get(...).doit().await
let r = hub.folders().notification_configs_list(...).doit().await
let r = hub.folders().notification_configs_patch(...).doit().await
let r = hub.folders().security_health_analytics_settings_custom_modules_create(...).doit().await
let r = hub.folders().security_health_analytics_settings_custom_modules_delete(...).doit().await
let r = hub.folders().security_health_analytics_settings_custom_modules_get(...).doit().await
let r = hub.folders().security_health_analytics_settings_custom_modules_list(...).doit().await
let r = hub.folders().security_health_analytics_settings_custom_modules_list_descendant(...).doit().await
let r = hub.folders().security_health_analytics_settings_custom_modules_patch(...).doit().await
let r = hub.folders().security_health_analytics_settings_custom_modules_simulate(...).doit().await
let r = hub.folders().security_health_analytics_settings_effective_custom_modules_get(...).doit().await
let r = hub.folders().security_health_analytics_settings_effective_custom_modules_list(...).doit().await
let r = hub.folders().sources_findings_external_systems_patch(...).doit().await
let r = hub.folders().sources_findings_group(...).doit().await
let r = hub.folders().sources_findings_list(...).doit().await
let r = hub.folders().sources_findings_patch(...).doit().await
let r = hub.folders().sources_findings_set_mute(...).doit().await
let r = hub.folders().sources_findings_set_state(...).doit().await
let r = hub.folders().sources_findings_update_security_marks(...).doit().await
let r = hub.folders().sources_list(...).doit().await
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
google-securitycenter1 = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_securitycenter1 as securitycenter1;
use securitycenter1::{Result, Error};
use std::default::Default;
use securitycenter1::{SecurityCommandCenter, oauth2, hyper, hyper_rustls, chrono, FieldMask};

// Get an ApplicationSecret instance by some means. It contains the `client_id` and 
// `client_secret`, among other things.
let secret: oauth2::ApplicationSecret = Default::default();
// Instantiate the authenticator. It will choose a suitable authentication flow for you, 
// unless you replace  `None` with the desired Flow.
// Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
// what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
// retrieve them from storage.
let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    ).build().await.unwrap();
let mut hub = SecurityCommandCenter::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.folders().assets_list("parent")
             .read_time(chrono::Utc::now())
             .page_token("magna")
             .page_size(-11)
             .order_by("ipsum")
             .filter("voluptua.")
             .field_mask(FieldMask::new::<&str>(&[]))
             .compare_duration(chrono::Duration::seconds(9827880))
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/client::Delegate) to the 
[Method Builder](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/client::RequestValue) and 
[decodable](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-securitycenter1/5.0.5+20240622/google_securitycenter1/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **securitycenter1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

