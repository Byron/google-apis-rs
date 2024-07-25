// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Aiplatform* crate version *5.0.5+20240715*, where *20240715* is the exact revision of the *aiplatform:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.
//! 
//! Everything else about the *Aiplatform* *v1* API can be found at the
//! [official documentation site](https://cloud.google.com/vertex-ai/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/aiplatform1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](Aiplatform) ... 
//! 
//! * projects
//!  * [*locations batch prediction jobs cancel*](api::ProjectLocationBatchPredictionJobCancelCall), [*locations batch prediction jobs create*](api::ProjectLocationBatchPredictionJobCreateCall), [*locations batch prediction jobs delete*](api::ProjectLocationBatchPredictionJobDeleteCall), [*locations batch prediction jobs get*](api::ProjectLocationBatchPredictionJobGetCall), [*locations batch prediction jobs list*](api::ProjectLocationBatchPredictionJobListCall), [*locations custom jobs cancel*](api::ProjectLocationCustomJobCancelCall), [*locations custom jobs create*](api::ProjectLocationCustomJobCreateCall), [*locations custom jobs delete*](api::ProjectLocationCustomJobDeleteCall), [*locations custom jobs get*](api::ProjectLocationCustomJobGetCall), [*locations custom jobs list*](api::ProjectLocationCustomJobListCall), [*locations custom jobs operations cancel*](api::ProjectLocationCustomJobOperationCancelCall), [*locations custom jobs operations delete*](api::ProjectLocationCustomJobOperationDeleteCall), [*locations custom jobs operations get*](api::ProjectLocationCustomJobOperationGetCall), [*locations custom jobs operations list*](api::ProjectLocationCustomJobOperationListCall), [*locations custom jobs operations wait*](api::ProjectLocationCustomJobOperationWaitCall), [*locations data labeling jobs cancel*](api::ProjectLocationDataLabelingJobCancelCall), [*locations data labeling jobs create*](api::ProjectLocationDataLabelingJobCreateCall), [*locations data labeling jobs delete*](api::ProjectLocationDataLabelingJobDeleteCall), [*locations data labeling jobs get*](api::ProjectLocationDataLabelingJobGetCall), [*locations data labeling jobs list*](api::ProjectLocationDataLabelingJobListCall), [*locations data labeling jobs operations cancel*](api::ProjectLocationDataLabelingJobOperationCancelCall), [*locations data labeling jobs operations delete*](api::ProjectLocationDataLabelingJobOperationDeleteCall), [*locations data labeling jobs operations get*](api::ProjectLocationDataLabelingJobOperationGetCall), [*locations data labeling jobs operations list*](api::ProjectLocationDataLabelingJobOperationListCall), [*locations data labeling jobs operations wait*](api::ProjectLocationDataLabelingJobOperationWaitCall), [*locations datasets annotation specs get*](api::ProjectLocationDatasetAnnotationSpecGetCall), [*locations datasets annotation specs operations cancel*](api::ProjectLocationDatasetAnnotationSpecOperationCancelCall), [*locations datasets annotation specs operations delete*](api::ProjectLocationDatasetAnnotationSpecOperationDeleteCall), [*locations datasets annotation specs operations get*](api::ProjectLocationDatasetAnnotationSpecOperationGetCall), [*locations datasets annotation specs operations list*](api::ProjectLocationDatasetAnnotationSpecOperationListCall), [*locations datasets annotation specs operations wait*](api::ProjectLocationDatasetAnnotationSpecOperationWaitCall), [*locations datasets create*](api::ProjectLocationDatasetCreateCall), [*locations datasets data items annotations list*](api::ProjectLocationDatasetDataItemAnnotationListCall), [*locations datasets data items annotations operations cancel*](api::ProjectLocationDatasetDataItemAnnotationOperationCancelCall), [*locations datasets data items annotations operations delete*](api::ProjectLocationDatasetDataItemAnnotationOperationDeleteCall), [*locations datasets data items annotations operations get*](api::ProjectLocationDatasetDataItemAnnotationOperationGetCall), [*locations datasets data items annotations operations list*](api::ProjectLocationDatasetDataItemAnnotationOperationListCall), [*locations datasets data items annotations operations wait*](api::ProjectLocationDatasetDataItemAnnotationOperationWaitCall), [*locations datasets data items list*](api::ProjectLocationDatasetDataItemListCall), [*locations datasets data items operations cancel*](api::ProjectLocationDatasetDataItemOperationCancelCall), [*locations datasets data items operations delete*](api::ProjectLocationDatasetDataItemOperationDeleteCall), [*locations datasets data items operations get*](api::ProjectLocationDatasetDataItemOperationGetCall), [*locations datasets data items operations list*](api::ProjectLocationDatasetDataItemOperationListCall), [*locations datasets data items operations wait*](api::ProjectLocationDatasetDataItemOperationWaitCall), [*locations datasets dataset versions create*](api::ProjectLocationDatasetDatasetVersionCreateCall), [*locations datasets dataset versions delete*](api::ProjectLocationDatasetDatasetVersionDeleteCall), [*locations datasets dataset versions get*](api::ProjectLocationDatasetDatasetVersionGetCall), [*locations datasets dataset versions list*](api::ProjectLocationDatasetDatasetVersionListCall), [*locations datasets dataset versions patch*](api::ProjectLocationDatasetDatasetVersionPatchCall), [*locations datasets dataset versions restore*](api::ProjectLocationDatasetDatasetVersionRestoreCall), [*locations datasets delete*](api::ProjectLocationDatasetDeleteCall), [*locations datasets export*](api::ProjectLocationDatasetExportCall), [*locations datasets get*](api::ProjectLocationDatasetGetCall), [*locations datasets import*](api::ProjectLocationDatasetImportCall), [*locations datasets list*](api::ProjectLocationDatasetListCall), [*locations datasets operations cancel*](api::ProjectLocationDatasetOperationCancelCall), [*locations datasets operations delete*](api::ProjectLocationDatasetOperationDeleteCall), [*locations datasets operations get*](api::ProjectLocationDatasetOperationGetCall), [*locations datasets operations list*](api::ProjectLocationDatasetOperationListCall), [*locations datasets operations wait*](api::ProjectLocationDatasetOperationWaitCall), [*locations datasets patch*](api::ProjectLocationDatasetPatchCall), [*locations datasets saved queries delete*](api::ProjectLocationDatasetSavedQueryDeleteCall), [*locations datasets saved queries list*](api::ProjectLocationDatasetSavedQueryListCall), [*locations datasets saved queries operations cancel*](api::ProjectLocationDatasetSavedQueryOperationCancelCall), [*locations datasets saved queries operations delete*](api::ProjectLocationDatasetSavedQueryOperationDeleteCall), [*locations datasets saved queries operations get*](api::ProjectLocationDatasetSavedQueryOperationGetCall), [*locations datasets saved queries operations list*](api::ProjectLocationDatasetSavedQueryOperationListCall), [*locations datasets saved queries operations wait*](api::ProjectLocationDatasetSavedQueryOperationWaitCall), [*locations datasets search data items*](api::ProjectLocationDatasetSearchDataItemCall), [*locations deployment resource pools create*](api::ProjectLocationDeploymentResourcePoolCreateCall), [*locations deployment resource pools delete*](api::ProjectLocationDeploymentResourcePoolDeleteCall), [*locations deployment resource pools get*](api::ProjectLocationDeploymentResourcePoolGetCall), [*locations deployment resource pools list*](api::ProjectLocationDeploymentResourcePoolListCall), [*locations deployment resource pools operations cancel*](api::ProjectLocationDeploymentResourcePoolOperationCancelCall), [*locations deployment resource pools operations delete*](api::ProjectLocationDeploymentResourcePoolOperationDeleteCall), [*locations deployment resource pools operations get*](api::ProjectLocationDeploymentResourcePoolOperationGetCall), [*locations deployment resource pools operations list*](api::ProjectLocationDeploymentResourcePoolOperationListCall), [*locations deployment resource pools operations wait*](api::ProjectLocationDeploymentResourcePoolOperationWaitCall), [*locations deployment resource pools patch*](api::ProjectLocationDeploymentResourcePoolPatchCall), [*locations deployment resource pools query deployed models*](api::ProjectLocationDeploymentResourcePoolQueryDeployedModelCall), [*locations endpoints compute tokens*](api::ProjectLocationEndpointComputeTokenCall), [*locations endpoints count tokens*](api::ProjectLocationEndpointCountTokenCall), [*locations endpoints create*](api::ProjectLocationEndpointCreateCall), [*locations endpoints delete*](api::ProjectLocationEndpointDeleteCall), [*locations endpoints deploy model*](api::ProjectLocationEndpointDeployModelCall), [*locations endpoints direct predict*](api::ProjectLocationEndpointDirectPredictCall), [*locations endpoints direct raw predict*](api::ProjectLocationEndpointDirectRawPredictCall), [*locations endpoints explain*](api::ProjectLocationEndpointExplainCall), [*locations endpoints generate content*](api::ProjectLocationEndpointGenerateContentCall), [*locations endpoints get*](api::ProjectLocationEndpointGetCall), [*locations endpoints list*](api::ProjectLocationEndpointListCall), [*locations endpoints mutate deployed model*](api::ProjectLocationEndpointMutateDeployedModelCall), [*locations endpoints operations cancel*](api::ProjectLocationEndpointOperationCancelCall), [*locations endpoints operations delete*](api::ProjectLocationEndpointOperationDeleteCall), [*locations endpoints operations get*](api::ProjectLocationEndpointOperationGetCall), [*locations endpoints operations list*](api::ProjectLocationEndpointOperationListCall), [*locations endpoints operations wait*](api::ProjectLocationEndpointOperationWaitCall), [*locations endpoints patch*](api::ProjectLocationEndpointPatchCall), [*locations endpoints predict*](api::ProjectLocationEndpointPredictCall), [*locations endpoints raw predict*](api::ProjectLocationEndpointRawPredictCall), [*locations endpoints server streaming predict*](api::ProjectLocationEndpointServerStreamingPredictCall), [*locations endpoints stream generate content*](api::ProjectLocationEndpointStreamGenerateContentCall), [*locations endpoints stream raw predict*](api::ProjectLocationEndpointStreamRawPredictCall), [*locations endpoints undeploy model*](api::ProjectLocationEndpointUndeployModelCall), [*locations evaluate instances*](api::ProjectLocationEvaluateInstanceCall), [*locations feature groups create*](api::ProjectLocationFeatureGroupCreateCall), [*locations feature groups delete*](api::ProjectLocationFeatureGroupDeleteCall), [*locations feature groups features create*](api::ProjectLocationFeatureGroupFeatureCreateCall), [*locations feature groups features delete*](api::ProjectLocationFeatureGroupFeatureDeleteCall), [*locations feature groups features get*](api::ProjectLocationFeatureGroupFeatureGetCall), [*locations feature groups features list*](api::ProjectLocationFeatureGroupFeatureListCall), [*locations feature groups features operations delete*](api::ProjectLocationFeatureGroupFeatureOperationDeleteCall), [*locations feature groups features operations get*](api::ProjectLocationFeatureGroupFeatureOperationGetCall), [*locations feature groups features operations list wait*](api::ProjectLocationFeatureGroupFeatureOperationListWaitCall), [*locations feature groups features operations wait*](api::ProjectLocationFeatureGroupFeatureOperationWaitCall), [*locations feature groups features patch*](api::ProjectLocationFeatureGroupFeaturePatchCall), [*locations feature groups get*](api::ProjectLocationFeatureGroupGetCall), [*locations feature groups list*](api::ProjectLocationFeatureGroupListCall), [*locations feature groups operations delete*](api::ProjectLocationFeatureGroupOperationDeleteCall), [*locations feature groups operations get*](api::ProjectLocationFeatureGroupOperationGetCall), [*locations feature groups operations list wait*](api::ProjectLocationFeatureGroupOperationListWaitCall), [*locations feature groups operations wait*](api::ProjectLocationFeatureGroupOperationWaitCall), [*locations feature groups patch*](api::ProjectLocationFeatureGroupPatchCall), [*locations feature online stores create*](api::ProjectLocationFeatureOnlineStoreCreateCall), [*locations feature online stores delete*](api::ProjectLocationFeatureOnlineStoreDeleteCall), [*locations feature online stores feature views create*](api::ProjectLocationFeatureOnlineStoreFeatureViewCreateCall), [*locations feature online stores feature views delete*](api::ProjectLocationFeatureOnlineStoreFeatureViewDeleteCall), [*locations feature online stores feature views feature view syncs get*](api::ProjectLocationFeatureOnlineStoreFeatureViewFeatureViewSyncGetCall), [*locations feature online stores feature views feature view syncs list*](api::ProjectLocationFeatureOnlineStoreFeatureViewFeatureViewSyncListCall), [*locations feature online stores feature views fetch feature values*](api::ProjectLocationFeatureOnlineStoreFeatureViewFetchFeatureValueCall), [*locations feature online stores feature views get*](api::ProjectLocationFeatureOnlineStoreFeatureViewGetCall), [*locations feature online stores feature views list*](api::ProjectLocationFeatureOnlineStoreFeatureViewListCall), [*locations feature online stores feature views operations delete*](api::ProjectLocationFeatureOnlineStoreFeatureViewOperationDeleteCall), [*locations feature online stores feature views operations get*](api::ProjectLocationFeatureOnlineStoreFeatureViewOperationGetCall), [*locations feature online stores feature views operations list wait*](api::ProjectLocationFeatureOnlineStoreFeatureViewOperationListWaitCall), [*locations feature online stores feature views operations wait*](api::ProjectLocationFeatureOnlineStoreFeatureViewOperationWaitCall), [*locations feature online stores feature views patch*](api::ProjectLocationFeatureOnlineStoreFeatureViewPatchCall), [*locations feature online stores feature views search nearest entities*](api::ProjectLocationFeatureOnlineStoreFeatureViewSearchNearestEntityCall), [*locations feature online stores feature views sync*](api::ProjectLocationFeatureOnlineStoreFeatureViewSyncCall), [*locations feature online stores get*](api::ProjectLocationFeatureOnlineStoreGetCall), [*locations feature online stores list*](api::ProjectLocationFeatureOnlineStoreListCall), [*locations feature online stores operations delete*](api::ProjectLocationFeatureOnlineStoreOperationDeleteCall), [*locations feature online stores operations get*](api::ProjectLocationFeatureOnlineStoreOperationGetCall), [*locations feature online stores operations list wait*](api::ProjectLocationFeatureOnlineStoreOperationListWaitCall), [*locations feature online stores operations wait*](api::ProjectLocationFeatureOnlineStoreOperationWaitCall), [*locations feature online stores patch*](api::ProjectLocationFeatureOnlineStorePatchCall), [*locations featurestores batch read feature values*](api::ProjectLocationFeaturestoreBatchReadFeatureValueCall), [*locations featurestores create*](api::ProjectLocationFeaturestoreCreateCall), [*locations featurestores delete*](api::ProjectLocationFeaturestoreDeleteCall), [*locations featurestores entity types create*](api::ProjectLocationFeaturestoreEntityTypeCreateCall), [*locations featurestores entity types delete*](api::ProjectLocationFeaturestoreEntityTypeDeleteCall), [*locations featurestores entity types delete feature values*](api::ProjectLocationFeaturestoreEntityTypeDeleteFeatureValueCall), [*locations featurestores entity types export feature values*](api::ProjectLocationFeaturestoreEntityTypeExportFeatureValueCall), [*locations featurestores entity types features batch create*](api::ProjectLocationFeaturestoreEntityTypeFeatureBatchCreateCall), [*locations featurestores entity types features create*](api::ProjectLocationFeaturestoreEntityTypeFeatureCreateCall), [*locations featurestores entity types features delete*](api::ProjectLocationFeaturestoreEntityTypeFeatureDeleteCall), [*locations featurestores entity types features get*](api::ProjectLocationFeaturestoreEntityTypeFeatureGetCall), [*locations featurestores entity types features list*](api::ProjectLocationFeaturestoreEntityTypeFeatureListCall), [*locations featurestores entity types features operations cancel*](api::ProjectLocationFeaturestoreEntityTypeFeatureOperationCancelCall), [*locations featurestores entity types features operations delete*](api::ProjectLocationFeaturestoreEntityTypeFeatureOperationDeleteCall), [*locations featurestores entity types features operations get*](api::ProjectLocationFeaturestoreEntityTypeFeatureOperationGetCall), [*locations featurestores entity types features operations list*](api::ProjectLocationFeaturestoreEntityTypeFeatureOperationListCall), [*locations featurestores entity types features operations wait*](api::ProjectLocationFeaturestoreEntityTypeFeatureOperationWaitCall), [*locations featurestores entity types features patch*](api::ProjectLocationFeaturestoreEntityTypeFeaturePatchCall), [*locations featurestores entity types get*](api::ProjectLocationFeaturestoreEntityTypeGetCall), [*locations featurestores entity types get iam policy*](api::ProjectLocationFeaturestoreEntityTypeGetIamPolicyCall), [*locations featurestores entity types import feature values*](api::ProjectLocationFeaturestoreEntityTypeImportFeatureValueCall), [*locations featurestores entity types list*](api::ProjectLocationFeaturestoreEntityTypeListCall), [*locations featurestores entity types operations cancel*](api::ProjectLocationFeaturestoreEntityTypeOperationCancelCall), [*locations featurestores entity types operations delete*](api::ProjectLocationFeaturestoreEntityTypeOperationDeleteCall), [*locations featurestores entity types operations get*](api::ProjectLocationFeaturestoreEntityTypeOperationGetCall), [*locations featurestores entity types operations list*](api::ProjectLocationFeaturestoreEntityTypeOperationListCall), [*locations featurestores entity types operations wait*](api::ProjectLocationFeaturestoreEntityTypeOperationWaitCall), [*locations featurestores entity types patch*](api::ProjectLocationFeaturestoreEntityTypePatchCall), [*locations featurestores entity types read feature values*](api::ProjectLocationFeaturestoreEntityTypeReadFeatureValueCall), [*locations featurestores entity types set iam policy*](api::ProjectLocationFeaturestoreEntityTypeSetIamPolicyCall), [*locations featurestores entity types streaming read feature values*](api::ProjectLocationFeaturestoreEntityTypeStreamingReadFeatureValueCall), [*locations featurestores entity types test iam permissions*](api::ProjectLocationFeaturestoreEntityTypeTestIamPermissionCall), [*locations featurestores entity types write feature values*](api::ProjectLocationFeaturestoreEntityTypeWriteFeatureValueCall), [*locations featurestores get*](api::ProjectLocationFeaturestoreGetCall), [*locations featurestores get iam policy*](api::ProjectLocationFeaturestoreGetIamPolicyCall), [*locations featurestores list*](api::ProjectLocationFeaturestoreListCall), [*locations featurestores operations cancel*](api::ProjectLocationFeaturestoreOperationCancelCall), [*locations featurestores operations delete*](api::ProjectLocationFeaturestoreOperationDeleteCall), [*locations featurestores operations get*](api::ProjectLocationFeaturestoreOperationGetCall), [*locations featurestores operations list*](api::ProjectLocationFeaturestoreOperationListCall), [*locations featurestores operations wait*](api::ProjectLocationFeaturestoreOperationWaitCall), [*locations featurestores patch*](api::ProjectLocationFeaturestorePatchCall), [*locations featurestores search features*](api::ProjectLocationFeaturestoreSearchFeatureCall), [*locations featurestores set iam policy*](api::ProjectLocationFeaturestoreSetIamPolicyCall), [*locations featurestores test iam permissions*](api::ProjectLocationFeaturestoreTestIamPermissionCall), [*locations get*](api::ProjectLocationGetCall), [*locations hyperparameter tuning jobs cancel*](api::ProjectLocationHyperparameterTuningJobCancelCall), [*locations hyperparameter tuning jobs create*](api::ProjectLocationHyperparameterTuningJobCreateCall), [*locations hyperparameter tuning jobs delete*](api::ProjectLocationHyperparameterTuningJobDeleteCall), [*locations hyperparameter tuning jobs get*](api::ProjectLocationHyperparameterTuningJobGetCall), [*locations hyperparameter tuning jobs list*](api::ProjectLocationHyperparameterTuningJobListCall), [*locations hyperparameter tuning jobs operations cancel*](api::ProjectLocationHyperparameterTuningJobOperationCancelCall), [*locations hyperparameter tuning jobs operations delete*](api::ProjectLocationHyperparameterTuningJobOperationDeleteCall), [*locations hyperparameter tuning jobs operations get*](api::ProjectLocationHyperparameterTuningJobOperationGetCall), [*locations hyperparameter tuning jobs operations list*](api::ProjectLocationHyperparameterTuningJobOperationListCall), [*locations hyperparameter tuning jobs operations wait*](api::ProjectLocationHyperparameterTuningJobOperationWaitCall), [*locations index endpoints create*](api::ProjectLocationIndexEndpointCreateCall), [*locations index endpoints delete*](api::ProjectLocationIndexEndpointDeleteCall), [*locations index endpoints deploy index*](api::ProjectLocationIndexEndpointDeployIndexCall), [*locations index endpoints find neighbors*](api::ProjectLocationIndexEndpointFindNeighborCall), [*locations index endpoints get*](api::ProjectLocationIndexEndpointGetCall), [*locations index endpoints list*](api::ProjectLocationIndexEndpointListCall), [*locations index endpoints mutate deployed index*](api::ProjectLocationIndexEndpointMutateDeployedIndexCall), [*locations index endpoints operations cancel*](api::ProjectLocationIndexEndpointOperationCancelCall), [*locations index endpoints operations delete*](api::ProjectLocationIndexEndpointOperationDeleteCall), [*locations index endpoints operations get*](api::ProjectLocationIndexEndpointOperationGetCall), [*locations index endpoints operations list*](api::ProjectLocationIndexEndpointOperationListCall), [*locations index endpoints operations wait*](api::ProjectLocationIndexEndpointOperationWaitCall), [*locations index endpoints patch*](api::ProjectLocationIndexEndpointPatchCall), [*locations index endpoints read index datapoints*](api::ProjectLocationIndexEndpointReadIndexDatapointCall), [*locations index endpoints undeploy index*](api::ProjectLocationIndexEndpointUndeployIndexCall), [*locations indexes create*](api::ProjectLocationIndexCreateCall), [*locations indexes delete*](api::ProjectLocationIndexDeleteCall), [*locations indexes get*](api::ProjectLocationIndexGetCall), [*locations indexes list*](api::ProjectLocationIndexListCall), [*locations indexes operations cancel*](api::ProjectLocationIndexOperationCancelCall), [*locations indexes operations delete*](api::ProjectLocationIndexOperationDeleteCall), [*locations indexes operations get*](api::ProjectLocationIndexOperationGetCall), [*locations indexes operations list*](api::ProjectLocationIndexOperationListCall), [*locations indexes operations wait*](api::ProjectLocationIndexOperationWaitCall), [*locations indexes patch*](api::ProjectLocationIndexPatchCall), [*locations indexes remove datapoints*](api::ProjectLocationIndexRemoveDatapointCall), [*locations indexes upsert datapoints*](api::ProjectLocationIndexUpsertDatapointCall), [*locations list*](api::ProjectLocationListCall), [*locations metadata stores artifacts create*](api::ProjectLocationMetadataStoreArtifactCreateCall), [*locations metadata stores artifacts delete*](api::ProjectLocationMetadataStoreArtifactDeleteCall), [*locations metadata stores artifacts get*](api::ProjectLocationMetadataStoreArtifactGetCall), [*locations metadata stores artifacts list*](api::ProjectLocationMetadataStoreArtifactListCall), [*locations metadata stores artifacts operations cancel*](api::ProjectLocationMetadataStoreArtifactOperationCancelCall), [*locations metadata stores artifacts operations delete*](api::ProjectLocationMetadataStoreArtifactOperationDeleteCall), [*locations metadata stores artifacts operations get*](api::ProjectLocationMetadataStoreArtifactOperationGetCall), [*locations metadata stores artifacts operations list*](api::ProjectLocationMetadataStoreArtifactOperationListCall), [*locations metadata stores artifacts operations wait*](api::ProjectLocationMetadataStoreArtifactOperationWaitCall), [*locations metadata stores artifacts patch*](api::ProjectLocationMetadataStoreArtifactPatchCall), [*locations metadata stores artifacts purge*](api::ProjectLocationMetadataStoreArtifactPurgeCall), [*locations metadata stores artifacts query artifact lineage subgraph*](api::ProjectLocationMetadataStoreArtifactQueryArtifactLineageSubgraphCall), [*locations metadata stores contexts add context artifacts and executions*](api::ProjectLocationMetadataStoreContextAddContextArtifactsAndExecutionCall), [*locations metadata stores contexts add context children*](api::ProjectLocationMetadataStoreContextAddContextChildrenCall), [*locations metadata stores contexts create*](api::ProjectLocationMetadataStoreContextCreateCall), [*locations metadata stores contexts delete*](api::ProjectLocationMetadataStoreContextDeleteCall), [*locations metadata stores contexts get*](api::ProjectLocationMetadataStoreContextGetCall), [*locations metadata stores contexts list*](api::ProjectLocationMetadataStoreContextListCall), [*locations metadata stores contexts operations cancel*](api::ProjectLocationMetadataStoreContextOperationCancelCall), [*locations metadata stores contexts operations delete*](api::ProjectLocationMetadataStoreContextOperationDeleteCall), [*locations metadata stores contexts operations get*](api::ProjectLocationMetadataStoreContextOperationGetCall), [*locations metadata stores contexts operations list*](api::ProjectLocationMetadataStoreContextOperationListCall), [*locations metadata stores contexts operations wait*](api::ProjectLocationMetadataStoreContextOperationWaitCall), [*locations metadata stores contexts patch*](api::ProjectLocationMetadataStoreContextPatchCall), [*locations metadata stores contexts purge*](api::ProjectLocationMetadataStoreContextPurgeCall), [*locations metadata stores contexts query context lineage subgraph*](api::ProjectLocationMetadataStoreContextQueryContextLineageSubgraphCall), [*locations metadata stores contexts remove context children*](api::ProjectLocationMetadataStoreContextRemoveContextChildrenCall), [*locations metadata stores create*](api::ProjectLocationMetadataStoreCreateCall), [*locations metadata stores delete*](api::ProjectLocationMetadataStoreDeleteCall), [*locations metadata stores executions add execution events*](api::ProjectLocationMetadataStoreExecutionAddExecutionEventCall), [*locations metadata stores executions create*](api::ProjectLocationMetadataStoreExecutionCreateCall), [*locations metadata stores executions delete*](api::ProjectLocationMetadataStoreExecutionDeleteCall), [*locations metadata stores executions get*](api::ProjectLocationMetadataStoreExecutionGetCall), [*locations metadata stores executions list*](api::ProjectLocationMetadataStoreExecutionListCall), [*locations metadata stores executions operations cancel*](api::ProjectLocationMetadataStoreExecutionOperationCancelCall), [*locations metadata stores executions operations delete*](api::ProjectLocationMetadataStoreExecutionOperationDeleteCall), [*locations metadata stores executions operations get*](api::ProjectLocationMetadataStoreExecutionOperationGetCall), [*locations metadata stores executions operations list*](api::ProjectLocationMetadataStoreExecutionOperationListCall), [*locations metadata stores executions operations wait*](api::ProjectLocationMetadataStoreExecutionOperationWaitCall), [*locations metadata stores executions patch*](api::ProjectLocationMetadataStoreExecutionPatchCall), [*locations metadata stores executions purge*](api::ProjectLocationMetadataStoreExecutionPurgeCall), [*locations metadata stores executions query execution inputs and outputs*](api::ProjectLocationMetadataStoreExecutionQueryExecutionInputsAndOutputCall), [*locations metadata stores get*](api::ProjectLocationMetadataStoreGetCall), [*locations metadata stores list*](api::ProjectLocationMetadataStoreListCall), [*locations metadata stores metadata schemas create*](api::ProjectLocationMetadataStoreMetadataSchemaCreateCall), [*locations metadata stores metadata schemas get*](api::ProjectLocationMetadataStoreMetadataSchemaGetCall), [*locations metadata stores metadata schemas list*](api::ProjectLocationMetadataStoreMetadataSchemaListCall), [*locations metadata stores operations cancel*](api::ProjectLocationMetadataStoreOperationCancelCall), [*locations metadata stores operations delete*](api::ProjectLocationMetadataStoreOperationDeleteCall), [*locations metadata stores operations get*](api::ProjectLocationMetadataStoreOperationGetCall), [*locations metadata stores operations list*](api::ProjectLocationMetadataStoreOperationListCall), [*locations metadata stores operations wait*](api::ProjectLocationMetadataStoreOperationWaitCall), [*locations migratable resources batch migrate*](api::ProjectLocationMigratableResourceBatchMigrateCall), [*locations migratable resources operations cancel*](api::ProjectLocationMigratableResourceOperationCancelCall), [*locations migratable resources operations delete*](api::ProjectLocationMigratableResourceOperationDeleteCall), [*locations migratable resources operations get*](api::ProjectLocationMigratableResourceOperationGetCall), [*locations migratable resources operations list*](api::ProjectLocationMigratableResourceOperationListCall), [*locations migratable resources operations wait*](api::ProjectLocationMigratableResourceOperationWaitCall), [*locations migratable resources search*](api::ProjectLocationMigratableResourceSearchCall), [*locations model deployment monitoring jobs create*](api::ProjectLocationModelDeploymentMonitoringJobCreateCall), [*locations model deployment monitoring jobs delete*](api::ProjectLocationModelDeploymentMonitoringJobDeleteCall), [*locations model deployment monitoring jobs get*](api::ProjectLocationModelDeploymentMonitoringJobGetCall), [*locations model deployment monitoring jobs list*](api::ProjectLocationModelDeploymentMonitoringJobListCall), [*locations model deployment monitoring jobs operations cancel*](api::ProjectLocationModelDeploymentMonitoringJobOperationCancelCall), [*locations model deployment monitoring jobs operations delete*](api::ProjectLocationModelDeploymentMonitoringJobOperationDeleteCall), [*locations model deployment monitoring jobs operations get*](api::ProjectLocationModelDeploymentMonitoringJobOperationGetCall), [*locations model deployment monitoring jobs operations list*](api::ProjectLocationModelDeploymentMonitoringJobOperationListCall), [*locations model deployment monitoring jobs operations wait*](api::ProjectLocationModelDeploymentMonitoringJobOperationWaitCall), [*locations model deployment monitoring jobs patch*](api::ProjectLocationModelDeploymentMonitoringJobPatchCall), [*locations model deployment monitoring jobs pause*](api::ProjectLocationModelDeploymentMonitoringJobPauseCall), [*locations model deployment monitoring jobs resume*](api::ProjectLocationModelDeploymentMonitoringJobResumeCall), [*locations model deployment monitoring jobs search model deployment monitoring stats anomalies*](api::ProjectLocationModelDeploymentMonitoringJobSearchModelDeploymentMonitoringStatsAnomalyCall), [*locations models copy*](api::ProjectLocationModelCopyCall), [*locations models delete*](api::ProjectLocationModelDeleteCall), [*locations models delete version*](api::ProjectLocationModelDeleteVersionCall), [*locations models evaluations get*](api::ProjectLocationModelEvaluationGetCall), [*locations models evaluations import*](api::ProjectLocationModelEvaluationImportCall), [*locations models evaluations list*](api::ProjectLocationModelEvaluationListCall), [*locations models evaluations operations cancel*](api::ProjectLocationModelEvaluationOperationCancelCall), [*locations models evaluations operations delete*](api::ProjectLocationModelEvaluationOperationDeleteCall), [*locations models evaluations operations get*](api::ProjectLocationModelEvaluationOperationGetCall), [*locations models evaluations operations list*](api::ProjectLocationModelEvaluationOperationListCall), [*locations models evaluations operations wait*](api::ProjectLocationModelEvaluationOperationWaitCall), [*locations models evaluations slices batch import*](api::ProjectLocationModelEvaluationSliceBatchImportCall), [*locations models evaluations slices get*](api::ProjectLocationModelEvaluationSliceGetCall), [*locations models evaluations slices list*](api::ProjectLocationModelEvaluationSliceListCall), [*locations models export*](api::ProjectLocationModelExportCall), [*locations models get*](api::ProjectLocationModelGetCall), [*locations models get iam policy*](api::ProjectLocationModelGetIamPolicyCall), [*locations models list*](api::ProjectLocationModelListCall), [*locations models list versions*](api::ProjectLocationModelListVersionCall), [*locations models merge version aliases*](api::ProjectLocationModelMergeVersionAliaseCall), [*locations models operations cancel*](api::ProjectLocationModelOperationCancelCall), [*locations models operations delete*](api::ProjectLocationModelOperationDeleteCall), [*locations models operations get*](api::ProjectLocationModelOperationGetCall), [*locations models operations list*](api::ProjectLocationModelOperationListCall), [*locations models operations wait*](api::ProjectLocationModelOperationWaitCall), [*locations models patch*](api::ProjectLocationModelPatchCall), [*locations models set iam policy*](api::ProjectLocationModelSetIamPolicyCall), [*locations models test iam permissions*](api::ProjectLocationModelTestIamPermissionCall), [*locations models update explanation dataset*](api::ProjectLocationModelUpdateExplanationDatasetCall), [*locations models upload*](api::ProjectLocationModelUploadCall), [*locations nas jobs cancel*](api::ProjectLocationNasJobCancelCall), [*locations nas jobs create*](api::ProjectLocationNasJobCreateCall), [*locations nas jobs delete*](api::ProjectLocationNasJobDeleteCall), [*locations nas jobs get*](api::ProjectLocationNasJobGetCall), [*locations nas jobs list*](api::ProjectLocationNasJobListCall), [*locations nas jobs nas trial details get*](api::ProjectLocationNasJobNasTrialDetailGetCall), [*locations nas jobs nas trial details list*](api::ProjectLocationNasJobNasTrialDetailListCall), [*locations notebook execution jobs create*](api::ProjectLocationNotebookExecutionJobCreateCall), [*locations notebook execution jobs delete*](api::ProjectLocationNotebookExecutionJobDeleteCall), [*locations notebook execution jobs get*](api::ProjectLocationNotebookExecutionJobGetCall), [*locations notebook execution jobs list*](api::ProjectLocationNotebookExecutionJobListCall), [*locations notebook execution jobs operations cancel*](api::ProjectLocationNotebookExecutionJobOperationCancelCall), [*locations notebook execution jobs operations delete*](api::ProjectLocationNotebookExecutionJobOperationDeleteCall), [*locations notebook execution jobs operations get*](api::ProjectLocationNotebookExecutionJobOperationGetCall), [*locations notebook execution jobs operations list*](api::ProjectLocationNotebookExecutionJobOperationListCall), [*locations notebook execution jobs operations wait*](api::ProjectLocationNotebookExecutionJobOperationWaitCall), [*locations notebook runtime templates create*](api::ProjectLocationNotebookRuntimeTemplateCreateCall), [*locations notebook runtime templates delete*](api::ProjectLocationNotebookRuntimeTemplateDeleteCall), [*locations notebook runtime templates get*](api::ProjectLocationNotebookRuntimeTemplateGetCall), [*locations notebook runtime templates get iam policy*](api::ProjectLocationNotebookRuntimeTemplateGetIamPolicyCall), [*locations notebook runtime templates list*](api::ProjectLocationNotebookRuntimeTemplateListCall), [*locations notebook runtime templates operations cancel*](api::ProjectLocationNotebookRuntimeTemplateOperationCancelCall), [*locations notebook runtime templates operations delete*](api::ProjectLocationNotebookRuntimeTemplateOperationDeleteCall), [*locations notebook runtime templates operations get*](api::ProjectLocationNotebookRuntimeTemplateOperationGetCall), [*locations notebook runtime templates operations list*](api::ProjectLocationNotebookRuntimeTemplateOperationListCall), [*locations notebook runtime templates operations wait*](api::ProjectLocationNotebookRuntimeTemplateOperationWaitCall), [*locations notebook runtime templates patch*](api::ProjectLocationNotebookRuntimeTemplatePatchCall), [*locations notebook runtime templates set iam policy*](api::ProjectLocationNotebookRuntimeTemplateSetIamPolicyCall), [*locations notebook runtime templates test iam permissions*](api::ProjectLocationNotebookRuntimeTemplateTestIamPermissionCall), [*locations notebook runtimes assign*](api::ProjectLocationNotebookRuntimeAssignCall), [*locations notebook runtimes delete*](api::ProjectLocationNotebookRuntimeDeleteCall), [*locations notebook runtimes get*](api::ProjectLocationNotebookRuntimeGetCall), [*locations notebook runtimes list*](api::ProjectLocationNotebookRuntimeListCall), [*locations notebook runtimes operations cancel*](api::ProjectLocationNotebookRuntimeOperationCancelCall), [*locations notebook runtimes operations delete*](api::ProjectLocationNotebookRuntimeOperationDeleteCall), [*locations notebook runtimes operations get*](api::ProjectLocationNotebookRuntimeOperationGetCall), [*locations notebook runtimes operations list*](api::ProjectLocationNotebookRuntimeOperationListCall), [*locations notebook runtimes operations wait*](api::ProjectLocationNotebookRuntimeOperationWaitCall), [*locations notebook runtimes start*](api::ProjectLocationNotebookRuntimeStartCall), [*locations notebook runtimes upgrade*](api::ProjectLocationNotebookRuntimeUpgradeCall), [*locations operations cancel*](api::ProjectLocationOperationCancelCall), [*locations operations delete*](api::ProjectLocationOperationDeleteCall), [*locations operations get*](api::ProjectLocationOperationGetCall), [*locations operations list*](api::ProjectLocationOperationListCall), [*locations operations wait*](api::ProjectLocationOperationWaitCall), [*locations persistent resources create*](api::ProjectLocationPersistentResourceCreateCall), [*locations persistent resources delete*](api::ProjectLocationPersistentResourceDeleteCall), [*locations persistent resources get*](api::ProjectLocationPersistentResourceGetCall), [*locations persistent resources list*](api::ProjectLocationPersistentResourceListCall), [*locations persistent resources operations cancel*](api::ProjectLocationPersistentResourceOperationCancelCall), [*locations persistent resources operations delete*](api::ProjectLocationPersistentResourceOperationDeleteCall), [*locations persistent resources operations get*](api::ProjectLocationPersistentResourceOperationGetCall), [*locations persistent resources operations list*](api::ProjectLocationPersistentResourceOperationListCall), [*locations persistent resources operations wait*](api::ProjectLocationPersistentResourceOperationWaitCall), [*locations persistent resources patch*](api::ProjectLocationPersistentResourcePatchCall), [*locations persistent resources reboot*](api::ProjectLocationPersistentResourceRebootCall), [*locations pipeline jobs batch cancel*](api::ProjectLocationPipelineJobBatchCancelCall), [*locations pipeline jobs batch delete*](api::ProjectLocationPipelineJobBatchDeleteCall), [*locations pipeline jobs cancel*](api::ProjectLocationPipelineJobCancelCall), [*locations pipeline jobs create*](api::ProjectLocationPipelineJobCreateCall), [*locations pipeline jobs delete*](api::ProjectLocationPipelineJobDeleteCall), [*locations pipeline jobs get*](api::ProjectLocationPipelineJobGetCall), [*locations pipeline jobs list*](api::ProjectLocationPipelineJobListCall), [*locations pipeline jobs operations cancel*](api::ProjectLocationPipelineJobOperationCancelCall), [*locations pipeline jobs operations delete*](api::ProjectLocationPipelineJobOperationDeleteCall), [*locations pipeline jobs operations get*](api::ProjectLocationPipelineJobOperationGetCall), [*locations pipeline jobs operations list*](api::ProjectLocationPipelineJobOperationListCall), [*locations pipeline jobs operations wait*](api::ProjectLocationPipelineJobOperationWaitCall), [*locations publishers models compute tokens*](api::ProjectLocationPublisherModelComputeTokenCall), [*locations publishers models count tokens*](api::ProjectLocationPublisherModelCountTokenCall), [*locations publishers models generate content*](api::ProjectLocationPublisherModelGenerateContentCall), [*locations publishers models predict*](api::ProjectLocationPublisherModelPredictCall), [*locations publishers models raw predict*](api::ProjectLocationPublisherModelRawPredictCall), [*locations publishers models server streaming predict*](api::ProjectLocationPublisherModelServerStreamingPredictCall), [*locations publishers models stream generate content*](api::ProjectLocationPublisherModelStreamGenerateContentCall), [*locations publishers models stream raw predict*](api::ProjectLocationPublisherModelStreamRawPredictCall), [*locations schedules create*](api::ProjectLocationScheduleCreateCall), [*locations schedules delete*](api::ProjectLocationScheduleDeleteCall), [*locations schedules get*](api::ProjectLocationScheduleGetCall), [*locations schedules list*](api::ProjectLocationScheduleListCall), [*locations schedules operations cancel*](api::ProjectLocationScheduleOperationCancelCall), [*locations schedules operations delete*](api::ProjectLocationScheduleOperationDeleteCall), [*locations schedules operations get*](api::ProjectLocationScheduleOperationGetCall), [*locations schedules operations list*](api::ProjectLocationScheduleOperationListCall), [*locations schedules operations wait*](api::ProjectLocationScheduleOperationWaitCall), [*locations schedules patch*](api::ProjectLocationSchedulePatchCall), [*locations schedules pause*](api::ProjectLocationSchedulePauseCall), [*locations schedules resume*](api::ProjectLocationScheduleResumeCall), [*locations specialist pools create*](api::ProjectLocationSpecialistPoolCreateCall), [*locations specialist pools delete*](api::ProjectLocationSpecialistPoolDeleteCall), [*locations specialist pools get*](api::ProjectLocationSpecialistPoolGetCall), [*locations specialist pools list*](api::ProjectLocationSpecialistPoolListCall), [*locations specialist pools operations cancel*](api::ProjectLocationSpecialistPoolOperationCancelCall), [*locations specialist pools operations delete*](api::ProjectLocationSpecialistPoolOperationDeleteCall), [*locations specialist pools operations get*](api::ProjectLocationSpecialistPoolOperationGetCall), [*locations specialist pools operations list*](api::ProjectLocationSpecialistPoolOperationListCall), [*locations specialist pools operations wait*](api::ProjectLocationSpecialistPoolOperationWaitCall), [*locations specialist pools patch*](api::ProjectLocationSpecialistPoolPatchCall), [*locations studies create*](api::ProjectLocationStudyCreateCall), [*locations studies delete*](api::ProjectLocationStudyDeleteCall), [*locations studies get*](api::ProjectLocationStudyGetCall), [*locations studies list*](api::ProjectLocationStudyListCall), [*locations studies lookup*](api::ProjectLocationStudyLookupCall), [*locations studies operations cancel*](api::ProjectLocationStudyOperationCancelCall), [*locations studies operations delete*](api::ProjectLocationStudyOperationDeleteCall), [*locations studies operations get*](api::ProjectLocationStudyOperationGetCall), [*locations studies operations list*](api::ProjectLocationStudyOperationListCall), [*locations studies operations wait*](api::ProjectLocationStudyOperationWaitCall), [*locations studies trials add trial measurement*](api::ProjectLocationStudyTrialAddTrialMeasurementCall), [*locations studies trials check trial early stopping state*](api::ProjectLocationStudyTrialCheckTrialEarlyStoppingStateCall), [*locations studies trials complete*](api::ProjectLocationStudyTrialCompleteCall), [*locations studies trials create*](api::ProjectLocationStudyTrialCreateCall), [*locations studies trials delete*](api::ProjectLocationStudyTrialDeleteCall), [*locations studies trials get*](api::ProjectLocationStudyTrialGetCall), [*locations studies trials list*](api::ProjectLocationStudyTrialListCall), [*locations studies trials list optimal trials*](api::ProjectLocationStudyTrialListOptimalTrialCall), [*locations studies trials operations cancel*](api::ProjectLocationStudyTrialOperationCancelCall), [*locations studies trials operations delete*](api::ProjectLocationStudyTrialOperationDeleteCall), [*locations studies trials operations get*](api::ProjectLocationStudyTrialOperationGetCall), [*locations studies trials operations list*](api::ProjectLocationStudyTrialOperationListCall), [*locations studies trials operations wait*](api::ProjectLocationStudyTrialOperationWaitCall), [*locations studies trials stop*](api::ProjectLocationStudyTrialStopCall), [*locations studies trials suggest*](api::ProjectLocationStudyTrialSuggestCall), [*locations tensorboards batch read*](api::ProjectLocationTensorboardBatchReadCall), [*locations tensorboards create*](api::ProjectLocationTensorboardCreateCall), [*locations tensorboards delete*](api::ProjectLocationTensorboardDeleteCall), [*locations tensorboards experiments batch create*](api::ProjectLocationTensorboardExperimentBatchCreateCall), [*locations tensorboards experiments create*](api::ProjectLocationTensorboardExperimentCreateCall), [*locations tensorboards experiments delete*](api::ProjectLocationTensorboardExperimentDeleteCall), [*locations tensorboards experiments get*](api::ProjectLocationTensorboardExperimentGetCall), [*locations tensorboards experiments list*](api::ProjectLocationTensorboardExperimentListCall), [*locations tensorboards experiments operations cancel*](api::ProjectLocationTensorboardExperimentOperationCancelCall), [*locations tensorboards experiments operations delete*](api::ProjectLocationTensorboardExperimentOperationDeleteCall), [*locations tensorboards experiments operations get*](api::ProjectLocationTensorboardExperimentOperationGetCall), [*locations tensorboards experiments operations list*](api::ProjectLocationTensorboardExperimentOperationListCall), [*locations tensorboards experiments operations wait*](api::ProjectLocationTensorboardExperimentOperationWaitCall), [*locations tensorboards experiments patch*](api::ProjectLocationTensorboardExperimentPatchCall), [*locations tensorboards experiments runs batch create*](api::ProjectLocationTensorboardExperimentRunBatchCreateCall), [*locations tensorboards experiments runs create*](api::ProjectLocationTensorboardExperimentRunCreateCall), [*locations tensorboards experiments runs delete*](api::ProjectLocationTensorboardExperimentRunDeleteCall), [*locations tensorboards experiments runs get*](api::ProjectLocationTensorboardExperimentRunGetCall), [*locations tensorboards experiments runs list*](api::ProjectLocationTensorboardExperimentRunListCall), [*locations tensorboards experiments runs operations cancel*](api::ProjectLocationTensorboardExperimentRunOperationCancelCall), [*locations tensorboards experiments runs operations delete*](api::ProjectLocationTensorboardExperimentRunOperationDeleteCall), [*locations tensorboards experiments runs operations get*](api::ProjectLocationTensorboardExperimentRunOperationGetCall), [*locations tensorboards experiments runs operations list*](api::ProjectLocationTensorboardExperimentRunOperationListCall), [*locations tensorboards experiments runs operations wait*](api::ProjectLocationTensorboardExperimentRunOperationWaitCall), [*locations tensorboards experiments runs patch*](api::ProjectLocationTensorboardExperimentRunPatchCall), [*locations tensorboards experiments runs time series create*](api::ProjectLocationTensorboardExperimentRunTimeSeryCreateCall), [*locations tensorboards experiments runs time series delete*](api::ProjectLocationTensorboardExperimentRunTimeSeryDeleteCall), [*locations tensorboards experiments runs time series export tensorboard time series*](api::ProjectLocationTensorboardExperimentRunTimeSeryExportTensorboardTimeSeryCall), [*locations tensorboards experiments runs time series get*](api::ProjectLocationTensorboardExperimentRunTimeSeryGetCall), [*locations tensorboards experiments runs time series list*](api::ProjectLocationTensorboardExperimentRunTimeSeryListCall), [*locations tensorboards experiments runs time series operations cancel*](api::ProjectLocationTensorboardExperimentRunTimeSeryOperationCancelCall), [*locations tensorboards experiments runs time series operations delete*](api::ProjectLocationTensorboardExperimentRunTimeSeryOperationDeleteCall), [*locations tensorboards experiments runs time series operations get*](api::ProjectLocationTensorboardExperimentRunTimeSeryOperationGetCall), [*locations tensorboards experiments runs time series operations list*](api::ProjectLocationTensorboardExperimentRunTimeSeryOperationListCall), [*locations tensorboards experiments runs time series operations wait*](api::ProjectLocationTensorboardExperimentRunTimeSeryOperationWaitCall), [*locations tensorboards experiments runs time series patch*](api::ProjectLocationTensorboardExperimentRunTimeSeryPatchCall), [*locations tensorboards experiments runs time series read*](api::ProjectLocationTensorboardExperimentRunTimeSeryReadCall), [*locations tensorboards experiments runs time series read blob data*](api::ProjectLocationTensorboardExperimentRunTimeSeryReadBlobDataCall), [*locations tensorboards experiments runs write*](api::ProjectLocationTensorboardExperimentRunWriteCall), [*locations tensorboards experiments write*](api::ProjectLocationTensorboardExperimentWriteCall), [*locations tensorboards get*](api::ProjectLocationTensorboardGetCall), [*locations tensorboards list*](api::ProjectLocationTensorboardListCall), [*locations tensorboards operations cancel*](api::ProjectLocationTensorboardOperationCancelCall), [*locations tensorboards operations delete*](api::ProjectLocationTensorboardOperationDeleteCall), [*locations tensorboards operations get*](api::ProjectLocationTensorboardOperationGetCall), [*locations tensorboards operations list*](api::ProjectLocationTensorboardOperationListCall), [*locations tensorboards operations wait*](api::ProjectLocationTensorboardOperationWaitCall), [*locations tensorboards patch*](api::ProjectLocationTensorboardPatchCall), [*locations tensorboards read size*](api::ProjectLocationTensorboardReadSizeCall), [*locations tensorboards read usage*](api::ProjectLocationTensorboardReadUsageCall), [*locations training pipelines cancel*](api::ProjectLocationTrainingPipelineCancelCall), [*locations training pipelines create*](api::ProjectLocationTrainingPipelineCreateCall), [*locations training pipelines delete*](api::ProjectLocationTrainingPipelineDeleteCall), [*locations training pipelines get*](api::ProjectLocationTrainingPipelineGetCall), [*locations training pipelines list*](api::ProjectLocationTrainingPipelineListCall), [*locations training pipelines operations cancel*](api::ProjectLocationTrainingPipelineOperationCancelCall), [*locations training pipelines operations delete*](api::ProjectLocationTrainingPipelineOperationDeleteCall), [*locations training pipelines operations get*](api::ProjectLocationTrainingPipelineOperationGetCall), [*locations training pipelines operations list*](api::ProjectLocationTrainingPipelineOperationListCall), [*locations training pipelines operations wait*](api::ProjectLocationTrainingPipelineOperationWaitCall), [*locations tuning jobs cancel*](api::ProjectLocationTuningJobCancelCall), [*locations tuning jobs create*](api::ProjectLocationTuningJobCreateCall), [*locations tuning jobs get*](api::ProjectLocationTuningJobGetCall), [*locations tuning jobs list*](api::ProjectLocationTuningJobListCall), [*locations tuning jobs operations cancel*](api::ProjectLocationTuningJobOperationCancelCall), [*locations tuning jobs operations get*](api::ProjectLocationTuningJobOperationGetCall) and [*locations tuning jobs operations list*](api::ProjectLocationTuningJobOperationListCall)
//! * publishers
//!  * [*models get*](api::PublisherModelGetCall)
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
//! * **[Hub](Aiplatform)**
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
//! let r = hub.projects().locations_data_labeling_jobs_operations_wait(...).doit().await
//! let r = hub.projects().locations_data_labeling_jobs_operations_get(...).doit().await
//! let r = hub.projects().locations_data_labeling_jobs_delete(...).doit().await
//! let r = hub.projects().locations_notebook_execution_jobs_operations_wait(...).doit().await
//! let r = hub.projects().locations_notebook_execution_jobs_operations_get(...).doit().await
//! let r = hub.projects().locations_notebook_execution_jobs_create(...).doit().await
//! let r = hub.projects().locations_notebook_execution_jobs_delete(...).doit().await
//! let r = hub.projects().locations_training_pipelines_operations_wait(...).doit().await
//! let r = hub.projects().locations_training_pipelines_operations_get(...).doit().await
//! let r = hub.projects().locations_training_pipelines_delete(...).doit().await
//! let r = hub.projects().locations_indexes_operations_get(...).doit().await
//! let r = hub.projects().locations_indexes_operations_wait(...).doit().await
//! let r = hub.projects().locations_indexes_delete(...).doit().await
//! let r = hub.projects().locations_indexes_patch(...).doit().await
//! let r = hub.projects().locations_indexes_create(...).doit().await
//! let r = hub.projects().locations_studies_trials_operations_get(...).doit().await
//! let r = hub.projects().locations_studies_trials_operations_wait(...).doit().await
//! let r = hub.projects().locations_studies_trials_check_trial_early_stopping_state(...).doit().await
//! let r = hub.projects().locations_studies_trials_suggest(...).doit().await
//! let r = hub.projects().locations_studies_operations_wait(...).doit().await
//! let r = hub.projects().locations_studies_operations_get(...).doit().await
//! let r = hub.projects().locations_migratable_resources_operations_wait(...).doit().await
//! let r = hub.projects().locations_migratable_resources_operations_get(...).doit().await
//! let r = hub.projects().locations_migratable_resources_batch_migrate(...).doit().await
//! let r = hub.projects().locations_nas_jobs_delete(...).doit().await
//! let r = hub.projects().locations_schedules_operations_get(...).doit().await
//! let r = hub.projects().locations_schedules_operations_wait(...).doit().await
//! let r = hub.projects().locations_schedules_delete(...).doit().await
//! let r = hub.projects().locations_tensorboards_operations_wait(...).doit().await
//! let r = hub.projects().locations_tensorboards_operations_get(...).doit().await
//! let r = hub.projects().locations_tensorboards_experiments_runs_time_series_operations_wait(...).doit().await
//! let r = hub.projects().locations_tensorboards_experiments_runs_time_series_operations_get(...).doit().await
//! let r = hub.projects().locations_tensorboards_experiments_runs_time_series_delete(...).doit().await
//! let r = hub.projects().locations_tensorboards_experiments_runs_operations_get(...).doit().await
//! let r = hub.projects().locations_tensorboards_experiments_runs_operations_wait(...).doit().await
//! let r = hub.projects().locations_tensorboards_experiments_runs_delete(...).doit().await
//! let r = hub.projects().locations_tensorboards_experiments_operations_get(...).doit().await
//! let r = hub.projects().locations_tensorboards_experiments_operations_wait(...).doit().await
//! let r = hub.projects().locations_tensorboards_experiments_delete(...).doit().await
//! let r = hub.projects().locations_tensorboards_delete(...).doit().await
//! let r = hub.projects().locations_tensorboards_create(...).doit().await
//! let r = hub.projects().locations_tensorboards_patch(...).doit().await
//! let r = hub.projects().locations_feature_groups_operations_wait(...).doit().await
//! let r = hub.projects().locations_feature_groups_operations_get(...).doit().await
//! let r = hub.projects().locations_feature_groups_features_operations_wait(...).doit().await
//! let r = hub.projects().locations_feature_groups_features_operations_get(...).doit().await
//! let r = hub.projects().locations_feature_groups_features_patch(...).doit().await
//! let r = hub.projects().locations_feature_groups_features_create(...).doit().await
//! let r = hub.projects().locations_feature_groups_features_delete(...).doit().await
//! let r = hub.projects().locations_feature_groups_delete(...).doit().await
//! let r = hub.projects().locations_feature_groups_patch(...).doit().await
//! let r = hub.projects().locations_feature_groups_create(...).doit().await
//! let r = hub.projects().locations_feature_online_stores_feature_views_operations_get(...).doit().await
//! let r = hub.projects().locations_feature_online_stores_feature_views_operations_wait(...).doit().await
//! let r = hub.projects().locations_feature_online_stores_feature_views_patch(...).doit().await
//! let r = hub.projects().locations_feature_online_stores_feature_views_create(...).doit().await
//! let r = hub.projects().locations_feature_online_stores_feature_views_delete(...).doit().await
//! let r = hub.projects().locations_feature_online_stores_operations_get(...).doit().await
//! let r = hub.projects().locations_feature_online_stores_operations_wait(...).doit().await
//! let r = hub.projects().locations_feature_online_stores_patch(...).doit().await
//! let r = hub.projects().locations_feature_online_stores_create(...).doit().await
//! let r = hub.projects().locations_feature_online_stores_delete(...).doit().await
//! let r = hub.projects().locations_index_endpoints_operations_get(...).doit().await
//! let r = hub.projects().locations_index_endpoints_operations_wait(...).doit().await
//! let r = hub.projects().locations_index_endpoints_mutate_deployed_index(...).doit().await
//! let r = hub.projects().locations_index_endpoints_undeploy_index(...).doit().await
//! let r = hub.projects().locations_index_endpoints_deploy_index(...).doit().await
//! let r = hub.projects().locations_index_endpoints_delete(...).doit().await
//! let r = hub.projects().locations_index_endpoints_create(...).doit().await
//! let r = hub.projects().locations_endpoints_operations_get(...).doit().await
//! let r = hub.projects().locations_endpoints_operations_wait(...).doit().await
//! let r = hub.projects().locations_endpoints_undeploy_model(...).doit().await
//! let r = hub.projects().locations_endpoints_deploy_model(...).doit().await
//! let r = hub.projects().locations_endpoints_delete(...).doit().await
//! let r = hub.projects().locations_endpoints_mutate_deployed_model(...).doit().await
//! let r = hub.projects().locations_endpoints_create(...).doit().await
//! let r = hub.projects().locations_specialist_pools_operations_get(...).doit().await
//! let r = hub.projects().locations_specialist_pools_operations_wait(...).doit().await
//! let r = hub.projects().locations_specialist_pools_patch(...).doit().await
//! let r = hub.projects().locations_specialist_pools_delete(...).doit().await
//! let r = hub.projects().locations_specialist_pools_create(...).doit().await
//! let r = hub.projects().locations_custom_jobs_operations_get(...).doit().await
//! let r = hub.projects().locations_custom_jobs_operations_wait(...).doit().await
//! let r = hub.projects().locations_custom_jobs_delete(...).doit().await
//! let r = hub.projects().locations_model_deployment_monitoring_jobs_operations_get(...).doit().await
//! let r = hub.projects().locations_model_deployment_monitoring_jobs_operations_wait(...).doit().await
//! let r = hub.projects().locations_model_deployment_monitoring_jobs_patch(...).doit().await
//! let r = hub.projects().locations_model_deployment_monitoring_jobs_delete(...).doit().await
//! let r = hub.projects().locations_datasets_data_items_operations_wait(...).doit().await
//! let r = hub.projects().locations_datasets_data_items_operations_get(...).doit().await
//! let r = hub.projects().locations_datasets_data_items_annotations_operations_get(...).doit().await
//! let r = hub.projects().locations_datasets_data_items_annotations_operations_wait(...).doit().await
//! let r = hub.projects().locations_datasets_annotation_specs_operations_wait(...).doit().await
//! let r = hub.projects().locations_datasets_annotation_specs_operations_get(...).doit().await
//! let r = hub.projects().locations_datasets_dataset_versions_delete(...).doit().await
//! let r = hub.projects().locations_datasets_dataset_versions_create(...).doit().await
//! let r = hub.projects().locations_datasets_dataset_versions_restore(...).doit().await
//! let r = hub.projects().locations_datasets_saved_queries_operations_wait(...).doit().await
//! let r = hub.projects().locations_datasets_saved_queries_operations_get(...).doit().await
//! let r = hub.projects().locations_datasets_saved_queries_delete(...).doit().await
//! let r = hub.projects().locations_datasets_operations_wait(...).doit().await
//! let r = hub.projects().locations_datasets_operations_get(...).doit().await
//! let r = hub.projects().locations_datasets_delete(...).doit().await
//! let r = hub.projects().locations_datasets_export(...).doit().await
//! let r = hub.projects().locations_datasets_import(...).doit().await
//! let r = hub.projects().locations_datasets_create(...).doit().await
//! let r = hub.projects().locations_featurestores_operations_wait(...).doit().await
//! let r = hub.projects().locations_featurestores_operations_get(...).doit().await
//! let r = hub.projects().locations_featurestores_entity_types_operations_get(...).doit().await
//! let r = hub.projects().locations_featurestores_entity_types_operations_wait(...).doit().await
//! let r = hub.projects().locations_featurestores_entity_types_features_operations_get(...).doit().await
//! let r = hub.projects().locations_featurestores_entity_types_features_operations_wait(...).doit().await
//! let r = hub.projects().locations_featurestores_entity_types_features_delete(...).doit().await
//! let r = hub.projects().locations_featurestores_entity_types_features_create(...).doit().await
//! let r = hub.projects().locations_featurestores_entity_types_features_batch_create(...).doit().await
//! let r = hub.projects().locations_featurestores_entity_types_delete(...).doit().await
//! let r = hub.projects().locations_featurestores_entity_types_export_feature_values(...).doit().await
//! let r = hub.projects().locations_featurestores_entity_types_create(...).doit().await
//! let r = hub.projects().locations_featurestores_entity_types_delete_feature_values(...).doit().await
//! let r = hub.projects().locations_featurestores_entity_types_import_feature_values(...).doit().await
//! let r = hub.projects().locations_featurestores_patch(...).doit().await
//! let r = hub.projects().locations_featurestores_create(...).doit().await
//! let r = hub.projects().locations_featurestores_delete(...).doit().await
//! let r = hub.projects().locations_featurestores_batch_read_feature_values(...).doit().await
//! let r = hub.projects().locations_notebook_runtimes_operations_wait(...).doit().await
//! let r = hub.projects().locations_notebook_runtimes_operations_get(...).doit().await
//! let r = hub.projects().locations_notebook_runtimes_upgrade(...).doit().await
//! let r = hub.projects().locations_notebook_runtimes_delete(...).doit().await
//! let r = hub.projects().locations_notebook_runtimes_start(...).doit().await
//! let r = hub.projects().locations_notebook_runtimes_assign(...).doit().await
//! let r = hub.projects().locations_metadata_stores_executions_operations_wait(...).doit().await
//! let r = hub.projects().locations_metadata_stores_executions_operations_get(...).doit().await
//! let r = hub.projects().locations_metadata_stores_executions_purge(...).doit().await
//! let r = hub.projects().locations_metadata_stores_executions_delete(...).doit().await
//! let r = hub.projects().locations_metadata_stores_contexts_operations_get(...).doit().await
//! let r = hub.projects().locations_metadata_stores_contexts_operations_wait(...).doit().await
//! let r = hub.projects().locations_metadata_stores_contexts_delete(...).doit().await
//! let r = hub.projects().locations_metadata_stores_contexts_purge(...).doit().await
//! let r = hub.projects().locations_metadata_stores_artifacts_operations_wait(...).doit().await
//! let r = hub.projects().locations_metadata_stores_artifacts_operations_get(...).doit().await
//! let r = hub.projects().locations_metadata_stores_artifacts_delete(...).doit().await
//! let r = hub.projects().locations_metadata_stores_artifacts_purge(...).doit().await
//! let r = hub.projects().locations_metadata_stores_operations_get(...).doit().await
//! let r = hub.projects().locations_metadata_stores_operations_wait(...).doit().await
//! let r = hub.projects().locations_metadata_stores_delete(...).doit().await
//! let r = hub.projects().locations_metadata_stores_create(...).doit().await
//! let r = hub.projects().locations_operations_wait(...).doit().await
//! let r = hub.projects().locations_operations_get(...).doit().await
//! let r = hub.projects().locations_hyperparameter_tuning_jobs_operations_get(...).doit().await
//! let r = hub.projects().locations_hyperparameter_tuning_jobs_operations_wait(...).doit().await
//! let r = hub.projects().locations_hyperparameter_tuning_jobs_delete(...).doit().await
//! let r = hub.projects().locations_persistent_resources_operations_get(...).doit().await
//! let r = hub.projects().locations_persistent_resources_operations_wait(...).doit().await
//! let r = hub.projects().locations_persistent_resources_patch(...).doit().await
//! let r = hub.projects().locations_persistent_resources_reboot(...).doit().await
//! let r = hub.projects().locations_persistent_resources_delete(...).doit().await
//! let r = hub.projects().locations_persistent_resources_create(...).doit().await
//! let r = hub.projects().locations_models_evaluations_operations_wait(...).doit().await
//! let r = hub.projects().locations_models_evaluations_operations_get(...).doit().await
//! let r = hub.projects().locations_models_operations_wait(...).doit().await
//! let r = hub.projects().locations_models_operations_get(...).doit().await
//! let r = hub.projects().locations_models_delete(...).doit().await
//! let r = hub.projects().locations_models_update_explanation_dataset(...).doit().await
//! let r = hub.projects().locations_models_copy(...).doit().await
//! let r = hub.projects().locations_models_upload(...).doit().await
//! let r = hub.projects().locations_models_export(...).doit().await
//! let r = hub.projects().locations_models_delete_version(...).doit().await
//! let r = hub.projects().locations_notebook_runtime_templates_operations_get(...).doit().await
//! let r = hub.projects().locations_notebook_runtime_templates_operations_wait(...).doit().await
//! let r = hub.projects().locations_notebook_runtime_templates_create(...).doit().await
//! let r = hub.projects().locations_notebook_runtime_templates_delete(...).doit().await
//! let r = hub.projects().locations_batch_prediction_jobs_delete(...).doit().await
//! let r = hub.projects().locations_tuning_jobs_operations_get(...).doit().await
//! let r = hub.projects().locations_deployment_resource_pools_operations_get(...).doit().await
//! let r = hub.projects().locations_deployment_resource_pools_operations_wait(...).doit().await
//! let r = hub.projects().locations_deployment_resource_pools_patch(...).doit().await
//! let r = hub.projects().locations_deployment_resource_pools_create(...).doit().await
//! let r = hub.projects().locations_deployment_resource_pools_delete(...).doit().await
//! let r = hub.projects().locations_pipeline_jobs_operations_get(...).doit().await
//! let r = hub.projects().locations_pipeline_jobs_operations_wait(...).doit().await
//! let r = hub.projects().locations_pipeline_jobs_delete(...).doit().await
//! let r = hub.projects().locations_pipeline_jobs_batch_delete(...).doit().await
//! let r = hub.projects().locations_pipeline_jobs_batch_cancel(...).doit().await
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
//! google-aiplatform1 = "*"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate google_aiplatform1 as aiplatform1;
//! use aiplatform1::api::GoogleCloudAiplatformV1FeatureView;
//! use aiplatform1::{Result, Error};
//! # async fn dox() {
//! use std::default::Default;
//! use aiplatform1::{Aiplatform, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
//! let mut hub = Aiplatform::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = GoogleCloudAiplatformV1FeatureView::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.projects().locations_feature_online_stores_feature_views_create(req, "parent")
//!              .run_sync_immediately(false)
//!              .feature_view_id("amet.")
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
pub use api::Aiplatform;
pub use client::{Result, Error, Delegate, FieldMask};

// Re-export the yup_oauth2 crate, that is required to call some methods of the hub and the client
#[cfg(feature = "yup-oauth2")]
pub use client::oauth2;