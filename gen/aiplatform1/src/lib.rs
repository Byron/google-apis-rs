// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Aiplatform* crate version *8.0.0+20251028*, where *20251028* is the exact revision of the *aiplatform:v1beta1* schema built by the [mako](http://www.makotemplates.org/) code generator *v8.0.0*.
//!
//! Everything else about the *Aiplatform* *v1_beta1* API can be found at the
//! [official documentation site](https://cloud.google.com/vertex-ai/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/aiplatform1_beta1).
//! # Features
//!
//! Handle the following *Resources* with ease from the central [hub](Aiplatform) ...
//!
//! * agents
//!  * [*operations cancel*](api::AgentOperationCancelCall), [*operations delete*](api::AgentOperationDeleteCall), [*operations get*](api::AgentOperationGetCall), [*operations list*](api::AgentOperationListCall) and [*operations wait*](api::AgentOperationWaitCall)
//! * apps
//!  * [*operations cancel*](api::AppOperationCancelCall), [*operations delete*](api::AppOperationDeleteCall), [*operations get*](api::AppOperationGetCall), [*operations list*](api::AppOperationListCall) and [*operations wait*](api::AppOperationWaitCall)
//! * batch prediction jobs
//!  * [*create*](api::BatchPredictionJobCreateCall), [*get*](api::BatchPredictionJobGetCall) and [*list*](api::BatchPredictionJobListCall)
//! * custom jobs
//!  * [*operations cancel*](api::CustomJobOperationCancelCall), [*operations delete*](api::CustomJobOperationDeleteCall), [*operations get*](api::CustomJobOperationGetCall), [*operations list*](api::CustomJobOperationListCall) and [*operations wait*](api::CustomJobOperationWaitCall)
//! * data labeling jobs
//!  * [*operations cancel*](api::DataLabelingJobOperationCancelCall), [*operations delete*](api::DataLabelingJobOperationDeleteCall), [*operations get*](api::DataLabelingJobOperationGetCall), [*operations list*](api::DataLabelingJobOperationListCall) and [*operations wait*](api::DataLabelingJobOperationWaitCall)
//! * datasets
//!  * [*annotation specs operations cancel*](api::DatasetAnnotationSpecOperationCancelCall), [*annotation specs operations delete*](api::DatasetAnnotationSpecOperationDeleteCall), [*annotation specs operations get*](api::DatasetAnnotationSpecOperationGetCall), [*annotation specs operations list*](api::DatasetAnnotationSpecOperationListCall), [*annotation specs operations wait*](api::DatasetAnnotationSpecOperationWaitCall), [*create*](api::DatasetCreateCall), [*data items annotations operations cancel*](api::DatasetDataItemAnnotationOperationCancelCall), [*data items annotations operations delete*](api::DatasetDataItemAnnotationOperationDeleteCall), [*data items annotations operations get*](api::DatasetDataItemAnnotationOperationGetCall), [*data items annotations operations list*](api::DatasetDataItemAnnotationOperationListCall), [*data items annotations operations wait*](api::DatasetDataItemAnnotationOperationWaitCall), [*data items operations cancel*](api::DatasetDataItemOperationCancelCall), [*data items operations delete*](api::DatasetDataItemOperationDeleteCall), [*data items operations get*](api::DatasetDataItemOperationGetCall), [*data items operations list*](api::DatasetDataItemOperationListCall), [*data items operations wait*](api::DatasetDataItemOperationWaitCall), [*dataset versions create*](api::DatasetDatasetVersionCreateCall), [*dataset versions delete*](api::DatasetDatasetVersionDeleteCall), [*dataset versions get*](api::DatasetDatasetVersionGetCall), [*dataset versions list*](api::DatasetDatasetVersionListCall), [*dataset versions patch*](api::DatasetDatasetVersionPatchCall), [*dataset versions restore*](api::DatasetDatasetVersionRestoreCall), [*delete*](api::DatasetDeleteCall), [*get*](api::DatasetGetCall), [*list*](api::DatasetListCall), [*operations cancel*](api::DatasetOperationCancelCall), [*operations delete*](api::DatasetOperationDeleteCall), [*operations get*](api::DatasetOperationGetCall), [*operations list*](api::DatasetOperationListCall), [*operations wait*](api::DatasetOperationWaitCall), [*patch*](api::DatasetPatchCall), [*saved queries operations cancel*](api::DatasetSavedQueryOperationCancelCall), [*saved queries operations delete*](api::DatasetSavedQueryOperationDeleteCall), [*saved queries operations get*](api::DatasetSavedQueryOperationGetCall), [*saved queries operations list*](api::DatasetSavedQueryOperationListCall) and [*saved queries operations wait*](api::DatasetSavedQueryOperationWaitCall)
//! * deployment resource pools
//!  * [*operations cancel*](api::DeploymentResourcePoolOperationCancelCall), [*operations delete*](api::DeploymentResourcePoolOperationDeleteCall), [*operations get*](api::DeploymentResourcePoolOperationGetCall), [*operations list*](api::DeploymentResourcePoolOperationListCall) and [*operations wait*](api::DeploymentResourcePoolOperationWaitCall)
//! * edge devices
//!  * [*operations cancel*](api::EdgeDeviceOperationCancelCall), [*operations delete*](api::EdgeDeviceOperationDeleteCall), [*operations get*](api::EdgeDeviceOperationGetCall), [*operations list*](api::EdgeDeviceOperationListCall) and [*operations wait*](api::EdgeDeviceOperationWaitCall)
//! * endpoints
//!  * [*chat completions*](api::EndpointChatCompletionCall), [*compute tokens*](api::EndpointComputeTokenCall), [*count tokens*](api::EndpointCountTokenCall), [*fetch predict operation*](api::EndpointFetchPredictOperationCall), [*generate content*](api::EndpointGenerateContentCall), [*operations cancel*](api::EndpointOperationCancelCall), [*operations delete*](api::EndpointOperationDeleteCall), [*operations get*](api::EndpointOperationGetCall), [*operations list*](api::EndpointOperationListCall), [*operations wait*](api::EndpointOperationWaitCall), [*predict*](api::EndpointPredictCall), [*predict long running*](api::EndpointPredictLongRunningCall) and [*stream generate content*](api::EndpointStreamGenerateContentCall)
//! * evaluation items
//!  * [*operations delete*](api::EvaluationItemOperationDeleteCall), [*operations get*](api::EvaluationItemOperationGetCall), [*operations list*](api::EvaluationItemOperationListCall) and [*operations wait*](api::EvaluationItemOperationWaitCall)
//! * evaluation runs
//!  * [*operations delete*](api::EvaluationRunOperationDeleteCall), [*operations get*](api::EvaluationRunOperationGetCall), [*operations list*](api::EvaluationRunOperationListCall) and [*operations wait*](api::EvaluationRunOperationWaitCall)
//! * evaluation sets
//!  * [*operations delete*](api::EvaluationSetOperationDeleteCall), [*operations get*](api::EvaluationSetOperationGetCall), [*operations list*](api::EvaluationSetOperationListCall) and [*operations wait*](api::EvaluationSetOperationWaitCall)
//! * evaluation tasks
//!  * [*operations delete*](api::EvaluationTaskOperationDeleteCall), [*operations get*](api::EvaluationTaskOperationGetCall), [*operations list*](api::EvaluationTaskOperationListCall) and [*operations wait*](api::EvaluationTaskOperationWaitCall)
//! * example stores
//!  * [*operations cancel*](api::ExampleStoreOperationCancelCall), [*operations delete*](api::ExampleStoreOperationDeleteCall), [*operations get*](api::ExampleStoreOperationGetCall), [*operations list*](api::ExampleStoreOperationListCall) and [*operations wait*](api::ExampleStoreOperationWaitCall)
//! * extension controllers
//!  * [*operations cancel*](api::ExtensionControllerOperationCancelCall), [*operations delete*](api::ExtensionControllerOperationDeleteCall), [*operations get*](api::ExtensionControllerOperationGetCall), [*operations list*](api::ExtensionControllerOperationListCall) and [*operations wait*](api::ExtensionControllerOperationWaitCall)
//! * extensions
//!  * [*operations cancel*](api::ExtensionOperationCancelCall), [*operations delete*](api::ExtensionOperationDeleteCall), [*operations get*](api::ExtensionOperationGetCall), [*operations list*](api::ExtensionOperationListCall) and [*operations wait*](api::ExtensionOperationWaitCall)
//! * feature groups
//!  * [*feature monitors operations delete*](api::FeatureGroupFeatureMonitorOperationDeleteCall), [*feature monitors operations get*](api::FeatureGroupFeatureMonitorOperationGetCall), [*feature monitors operations list*](api::FeatureGroupFeatureMonitorOperationListCall), [*feature monitors operations wait*](api::FeatureGroupFeatureMonitorOperationWaitCall), [*features operations delete*](api::FeatureGroupFeatureOperationDeleteCall), [*features operations get*](api::FeatureGroupFeatureOperationGetCall), [*features operations list*](api::FeatureGroupFeatureOperationListCall), [*features operations wait*](api::FeatureGroupFeatureOperationWaitCall), [*operations delete*](api::FeatureGroupOperationDeleteCall), [*operations get*](api::FeatureGroupOperationGetCall), [*operations list*](api::FeatureGroupOperationListCall) and [*operations wait*](api::FeatureGroupOperationWaitCall)
//! * feature online stores
//!  * [*feature views operations delete*](api::FeatureOnlineStoreFeatureViewOperationDeleteCall), [*feature views operations get*](api::FeatureOnlineStoreFeatureViewOperationGetCall), [*feature views operations list*](api::FeatureOnlineStoreFeatureViewOperationListCall), [*feature views operations wait*](api::FeatureOnlineStoreFeatureViewOperationWaitCall), [*operations delete*](api::FeatureOnlineStoreOperationDeleteCall), [*operations get*](api::FeatureOnlineStoreOperationGetCall), [*operations list*](api::FeatureOnlineStoreOperationListCall) and [*operations wait*](api::FeatureOnlineStoreOperationWaitCall)
//! * featurestores
//!  * [*entity types features operations cancel*](api::FeaturestoreEntityTypeFeatureOperationCancelCall), [*entity types features operations delete*](api::FeaturestoreEntityTypeFeatureOperationDeleteCall), [*entity types features operations get*](api::FeaturestoreEntityTypeFeatureOperationGetCall), [*entity types features operations list*](api::FeaturestoreEntityTypeFeatureOperationListCall), [*entity types features operations wait*](api::FeaturestoreEntityTypeFeatureOperationWaitCall), [*entity types operations cancel*](api::FeaturestoreEntityTypeOperationCancelCall), [*entity types operations delete*](api::FeaturestoreEntityTypeOperationDeleteCall), [*entity types operations get*](api::FeaturestoreEntityTypeOperationGetCall), [*entity types operations list*](api::FeaturestoreEntityTypeOperationListCall), [*entity types operations wait*](api::FeaturestoreEntityTypeOperationWaitCall), [*operations cancel*](api::FeaturestoreOperationCancelCall), [*operations delete*](api::FeaturestoreOperationDeleteCall), [*operations get*](api::FeaturestoreOperationGetCall), [*operations list*](api::FeaturestoreOperationListCall) and [*operations wait*](api::FeaturestoreOperationWaitCall)
//! * hyperparameter tuning jobs
//!  * [*operations cancel*](api::HyperparameterTuningJobOperationCancelCall), [*operations delete*](api::HyperparameterTuningJobOperationDeleteCall), [*operations get*](api::HyperparameterTuningJobOperationGetCall), [*operations list*](api::HyperparameterTuningJobOperationListCall) and [*operations wait*](api::HyperparameterTuningJobOperationWaitCall)
//! * index endpoints
//!  * [*operations cancel*](api::IndexEndpointOperationCancelCall), [*operations delete*](api::IndexEndpointOperationDeleteCall), [*operations get*](api::IndexEndpointOperationGetCall), [*operations list*](api::IndexEndpointOperationListCall) and [*operations wait*](api::IndexEndpointOperationWaitCall)
//! * indexes
//!  * [*operations cancel*](api::IndexOperationCancelCall), [*operations delete*](api::IndexOperationDeleteCall), [*operations get*](api::IndexOperationGetCall), [*operations list*](api::IndexOperationListCall) and [*operations wait*](api::IndexOperationWaitCall)
//! * media
//!  * [*upload*](api::MediaUploadCall)
//! * metadata stores
//!  * [*artifacts operations cancel*](api::MetadataStoreArtifactOperationCancelCall), [*artifacts operations delete*](api::MetadataStoreArtifactOperationDeleteCall), [*artifacts operations get*](api::MetadataStoreArtifactOperationGetCall), [*artifacts operations list*](api::MetadataStoreArtifactOperationListCall), [*artifacts operations wait*](api::MetadataStoreArtifactOperationWaitCall), [*contexts operations cancel*](api::MetadataStoreContextOperationCancelCall), [*contexts operations delete*](api::MetadataStoreContextOperationDeleteCall), [*contexts operations get*](api::MetadataStoreContextOperationGetCall), [*contexts operations list*](api::MetadataStoreContextOperationListCall), [*contexts operations wait*](api::MetadataStoreContextOperationWaitCall), [*executions operations cancel*](api::MetadataStoreExecutionOperationCancelCall), [*executions operations delete*](api::MetadataStoreExecutionOperationDeleteCall), [*executions operations get*](api::MetadataStoreExecutionOperationGetCall), [*executions operations list*](api::MetadataStoreExecutionOperationListCall), [*executions operations wait*](api::MetadataStoreExecutionOperationWaitCall), [*operations cancel*](api::MetadataStoreOperationCancelCall), [*operations delete*](api::MetadataStoreOperationDeleteCall), [*operations get*](api::MetadataStoreOperationGetCall), [*operations list*](api::MetadataStoreOperationListCall) and [*operations wait*](api::MetadataStoreOperationWaitCall)
//! * migratable resources
//!  * [*operations cancel*](api::MigratableResourceOperationCancelCall), [*operations delete*](api::MigratableResourceOperationDeleteCall), [*operations get*](api::MigratableResourceOperationGetCall), [*operations list*](api::MigratableResourceOperationListCall) and [*operations wait*](api::MigratableResourceOperationWaitCall)
//! * model deployment monitoring jobs
//!  * [*operations cancel*](api::ModelDeploymentMonitoringJobOperationCancelCall), [*operations delete*](api::ModelDeploymentMonitoringJobOperationDeleteCall), [*operations get*](api::ModelDeploymentMonitoringJobOperationGetCall), [*operations list*](api::ModelDeploymentMonitoringJobOperationListCall) and [*operations wait*](api::ModelDeploymentMonitoringJobOperationWaitCall)
//! * model monitors
//!  * [*operations cancel*](api::ModelMonitorOperationCancelCall), [*operations delete*](api::ModelMonitorOperationDeleteCall), [*operations get*](api::ModelMonitorOperationGetCall), [*operations list*](api::ModelMonitorOperationListCall) and [*operations wait*](api::ModelMonitorOperationWaitCall)
//! * models
//!  * [*evaluations operations cancel*](api::ModelEvaluationOperationCancelCall), [*evaluations operations delete*](api::ModelEvaluationOperationDeleteCall), [*evaluations operations get*](api::ModelEvaluationOperationGetCall), [*evaluations operations list*](api::ModelEvaluationOperationListCall), [*evaluations operations wait*](api::ModelEvaluationOperationWaitCall), [*operations cancel*](api::ModelOperationCancelCall), [*operations delete*](api::ModelOperationDeleteCall), [*operations get*](api::ModelOperationGetCall), [*operations list*](api::ModelOperationListCall) and [*operations wait*](api::ModelOperationWaitCall)
//! * notebook execution jobs
//!  * [*operations cancel*](api::NotebookExecutionJobOperationCancelCall), [*operations delete*](api::NotebookExecutionJobOperationDeleteCall), [*operations get*](api::NotebookExecutionJobOperationGetCall), [*operations list*](api::NotebookExecutionJobOperationListCall) and [*operations wait*](api::NotebookExecutionJobOperationWaitCall)
//! * notebook runtime templates
//!  * [*operations cancel*](api::NotebookRuntimeTemplateOperationCancelCall), [*operations delete*](api::NotebookRuntimeTemplateOperationDeleteCall), [*operations get*](api::NotebookRuntimeTemplateOperationGetCall), [*operations list*](api::NotebookRuntimeTemplateOperationListCall) and [*operations wait*](api::NotebookRuntimeTemplateOperationWaitCall)
//! * notebook runtimes
//!  * [*operations cancel*](api::NotebookRuntimeOperationCancelCall), [*operations delete*](api::NotebookRuntimeOperationDeleteCall), [*operations get*](api::NotebookRuntimeOperationGetCall), [*operations list*](api::NotebookRuntimeOperationListCall) and [*operations wait*](api::NotebookRuntimeOperationWaitCall)
//! * operations
//!  * [*cancel*](api::OperationCancelCall), [*delete*](api::OperationDeleteCall), [*get*](api::OperationGetCall), [*list*](api::OperationListCall) and [*wait*](api::OperationWaitCall)
//! * persistent resources
//!  * [*operations cancel*](api::PersistentResourceOperationCancelCall), [*operations delete*](api::PersistentResourceOperationDeleteCall), [*operations get*](api::PersistentResourceOperationGetCall), [*operations list*](api::PersistentResourceOperationListCall) and [*operations wait*](api::PersistentResourceOperationWaitCall)
//! * pipeline jobs
//!  * [*operations cancel*](api::PipelineJobOperationCancelCall), [*operations delete*](api::PipelineJobOperationDeleteCall), [*operations get*](api::PipelineJobOperationGetCall), [*operations list*](api::PipelineJobOperationListCall) and [*operations wait*](api::PipelineJobOperationWaitCall)
//! * projects
//!  * [*fetch publisher model config*](api::ProjectFetchPublisherModelConfigCall), [*get cache config*](api::ProjectGetCacheConfigCall), [*locations agents operations cancel*](api::ProjectLocationAgentOperationCancelCall), [*locations agents operations delete*](api::ProjectLocationAgentOperationDeleteCall), [*locations agents operations get*](api::ProjectLocationAgentOperationGetCall), [*locations agents operations list*](api::ProjectLocationAgentOperationListCall), [*locations agents operations wait*](api::ProjectLocationAgentOperationWaitCall), [*locations apps operations cancel*](api::ProjectLocationAppOperationCancelCall), [*locations apps operations delete*](api::ProjectLocationAppOperationDeleteCall), [*locations apps operations get*](api::ProjectLocationAppOperationGetCall), [*locations apps operations list*](api::ProjectLocationAppOperationListCall), [*locations apps operations wait*](api::ProjectLocationAppOperationWaitCall), [*locations augment prompt*](api::ProjectLocationAugmentPromptCall), [*locations batch prediction jobs cancel*](api::ProjectLocationBatchPredictionJobCancelCall), [*locations batch prediction jobs create*](api::ProjectLocationBatchPredictionJobCreateCall), [*locations batch prediction jobs delete*](api::ProjectLocationBatchPredictionJobDeleteCall), [*locations batch prediction jobs get*](api::ProjectLocationBatchPredictionJobGetCall), [*locations batch prediction jobs list*](api::ProjectLocationBatchPredictionJobListCall), [*locations cached contents create*](api::ProjectLocationCachedContentCreateCall), [*locations cached contents delete*](api::ProjectLocationCachedContentDeleteCall), [*locations cached contents get*](api::ProjectLocationCachedContentGetCall), [*locations cached contents list*](api::ProjectLocationCachedContentListCall), [*locations cached contents patch*](api::ProjectLocationCachedContentPatchCall), [*locations corroborate content*](api::ProjectLocationCorroborateContentCall), [*locations custom jobs cancel*](api::ProjectLocationCustomJobCancelCall), [*locations custom jobs create*](api::ProjectLocationCustomJobCreateCall), [*locations custom jobs delete*](api::ProjectLocationCustomJobDeleteCall), [*locations custom jobs get*](api::ProjectLocationCustomJobGetCall), [*locations custom jobs list*](api::ProjectLocationCustomJobListCall), [*locations custom jobs operations cancel*](api::ProjectLocationCustomJobOperationCancelCall), [*locations custom jobs operations delete*](api::ProjectLocationCustomJobOperationDeleteCall), [*locations custom jobs operations get*](api::ProjectLocationCustomJobOperationGetCall), [*locations custom jobs operations list*](api::ProjectLocationCustomJobOperationListCall), [*locations custom jobs operations wait*](api::ProjectLocationCustomJobOperationWaitCall), [*locations data labeling jobs cancel*](api::ProjectLocationDataLabelingJobCancelCall), [*locations data labeling jobs create*](api::ProjectLocationDataLabelingJobCreateCall), [*locations data labeling jobs delete*](api::ProjectLocationDataLabelingJobDeleteCall), [*locations data labeling jobs get*](api::ProjectLocationDataLabelingJobGetCall), [*locations data labeling jobs list*](api::ProjectLocationDataLabelingJobListCall), [*locations data labeling jobs operations cancel*](api::ProjectLocationDataLabelingJobOperationCancelCall), [*locations data labeling jobs operations delete*](api::ProjectLocationDataLabelingJobOperationDeleteCall), [*locations data labeling jobs operations get*](api::ProjectLocationDataLabelingJobOperationGetCall), [*locations data labeling jobs operations list*](api::ProjectLocationDataLabelingJobOperationListCall), [*locations data labeling jobs operations wait*](api::ProjectLocationDataLabelingJobOperationWaitCall), [*locations datasets annotation specs get*](api::ProjectLocationDatasetAnnotationSpecGetCall), [*locations datasets annotation specs operations cancel*](api::ProjectLocationDatasetAnnotationSpecOperationCancelCall), [*locations datasets annotation specs operations delete*](api::ProjectLocationDatasetAnnotationSpecOperationDeleteCall), [*locations datasets annotation specs operations get*](api::ProjectLocationDatasetAnnotationSpecOperationGetCall), [*locations datasets annotation specs operations list*](api::ProjectLocationDatasetAnnotationSpecOperationListCall), [*locations datasets annotation specs operations wait*](api::ProjectLocationDatasetAnnotationSpecOperationWaitCall), [*locations datasets assemble*](api::ProjectLocationDatasetAssembleCall), [*locations datasets assess*](api::ProjectLocationDatasetAssesCall), [*locations datasets create*](api::ProjectLocationDatasetCreateCall), [*locations datasets data items annotations list*](api::ProjectLocationDatasetDataItemAnnotationListCall), [*locations datasets data items annotations operations cancel*](api::ProjectLocationDatasetDataItemAnnotationOperationCancelCall), [*locations datasets data items annotations operations delete*](api::ProjectLocationDatasetDataItemAnnotationOperationDeleteCall), [*locations datasets data items annotations operations get*](api::ProjectLocationDatasetDataItemAnnotationOperationGetCall), [*locations datasets data items annotations operations list*](api::ProjectLocationDatasetDataItemAnnotationOperationListCall), [*locations datasets data items annotations operations wait*](api::ProjectLocationDatasetDataItemAnnotationOperationWaitCall), [*locations datasets data items list*](api::ProjectLocationDatasetDataItemListCall), [*locations datasets data items operations cancel*](api::ProjectLocationDatasetDataItemOperationCancelCall), [*locations datasets data items operations delete*](api::ProjectLocationDatasetDataItemOperationDeleteCall), [*locations datasets data items operations get*](api::ProjectLocationDatasetDataItemOperationGetCall), [*locations datasets data items operations list*](api::ProjectLocationDatasetDataItemOperationListCall), [*locations datasets data items operations wait*](api::ProjectLocationDatasetDataItemOperationWaitCall), [*locations datasets dataset versions create*](api::ProjectLocationDatasetDatasetVersionCreateCall), [*locations datasets dataset versions delete*](api::ProjectLocationDatasetDatasetVersionDeleteCall), [*locations datasets dataset versions get*](api::ProjectLocationDatasetDatasetVersionGetCall), [*locations datasets dataset versions list*](api::ProjectLocationDatasetDatasetVersionListCall), [*locations datasets dataset versions patch*](api::ProjectLocationDatasetDatasetVersionPatchCall), [*locations datasets dataset versions restore*](api::ProjectLocationDatasetDatasetVersionRestoreCall), [*locations datasets delete*](api::ProjectLocationDatasetDeleteCall), [*locations datasets export*](api::ProjectLocationDatasetExportCall), [*locations datasets get*](api::ProjectLocationDatasetGetCall), [*locations datasets import*](api::ProjectLocationDatasetImportCall), [*locations datasets list*](api::ProjectLocationDatasetListCall), [*locations datasets operations cancel*](api::ProjectLocationDatasetOperationCancelCall), [*locations datasets operations delete*](api::ProjectLocationDatasetOperationDeleteCall), [*locations datasets operations get*](api::ProjectLocationDatasetOperationGetCall), [*locations datasets operations list*](api::ProjectLocationDatasetOperationListCall), [*locations datasets operations wait*](api::ProjectLocationDatasetOperationWaitCall), [*locations datasets patch*](api::ProjectLocationDatasetPatchCall), [*locations datasets saved queries delete*](api::ProjectLocationDatasetSavedQueryDeleteCall), [*locations datasets saved queries list*](api::ProjectLocationDatasetSavedQueryListCall), [*locations datasets saved queries operations cancel*](api::ProjectLocationDatasetSavedQueryOperationCancelCall), [*locations datasets saved queries operations delete*](api::ProjectLocationDatasetSavedQueryOperationDeleteCall), [*locations datasets saved queries operations get*](api::ProjectLocationDatasetSavedQueryOperationGetCall), [*locations datasets saved queries operations list*](api::ProjectLocationDatasetSavedQueryOperationListCall), [*locations datasets saved queries operations wait*](api::ProjectLocationDatasetSavedQueryOperationWaitCall), [*locations datasets search data items*](api::ProjectLocationDatasetSearchDataItemCall), [*locations deploy*](api::ProjectLocationDeployCall), [*locations deploy publisher model*](api::ProjectLocationDeployPublisherModelCall), [*locations deployment resource pools create*](api::ProjectLocationDeploymentResourcePoolCreateCall), [*locations deployment resource pools delete*](api::ProjectLocationDeploymentResourcePoolDeleteCall), [*locations deployment resource pools get*](api::ProjectLocationDeploymentResourcePoolGetCall), [*locations deployment resource pools list*](api::ProjectLocationDeploymentResourcePoolListCall), [*locations deployment resource pools operations cancel*](api::ProjectLocationDeploymentResourcePoolOperationCancelCall), [*locations deployment resource pools operations delete*](api::ProjectLocationDeploymentResourcePoolOperationDeleteCall), [*locations deployment resource pools operations get*](api::ProjectLocationDeploymentResourcePoolOperationGetCall), [*locations deployment resource pools operations list*](api::ProjectLocationDeploymentResourcePoolOperationListCall), [*locations deployment resource pools operations wait*](api::ProjectLocationDeploymentResourcePoolOperationWaitCall), [*locations deployment resource pools patch*](api::ProjectLocationDeploymentResourcePoolPatchCall), [*locations deployment resource pools query deployed models*](api::ProjectLocationDeploymentResourcePoolQueryDeployedModelCall), [*locations edge devices operations cancel*](api::ProjectLocationEdgeDeviceOperationCancelCall), [*locations edge devices operations delete*](api::ProjectLocationEdgeDeviceOperationDeleteCall), [*locations edge devices operations get*](api::ProjectLocationEdgeDeviceOperationGetCall), [*locations edge devices operations list*](api::ProjectLocationEdgeDeviceOperationListCall), [*locations edge devices operations wait*](api::ProjectLocationEdgeDeviceOperationWaitCall), [*locations endpoints chat completions*](api::ProjectLocationEndpointChatCompletionCall), [*locations endpoints compute tokens*](api::ProjectLocationEndpointComputeTokenCall), [*locations endpoints count tokens*](api::ProjectLocationEndpointCountTokenCall), [*locations endpoints create*](api::ProjectLocationEndpointCreateCall), [*locations endpoints delete*](api::ProjectLocationEndpointDeleteCall), [*locations endpoints deploy model*](api::ProjectLocationEndpointDeployModelCall), [*locations endpoints deployed models invoke invoke*](api::ProjectLocationEndpointDeployedModelInvokeInvokeCall), [*locations endpoints direct predict*](api::ProjectLocationEndpointDirectPredictCall), [*locations endpoints direct raw predict*](api::ProjectLocationEndpointDirectRawPredictCall), [*locations endpoints explain*](api::ProjectLocationEndpointExplainCall), [*locations endpoints fetch predict operation*](api::ProjectLocationEndpointFetchPredictOperationCall), [*locations endpoints generate content*](api::ProjectLocationEndpointGenerateContentCall), [*locations endpoints get*](api::ProjectLocationEndpointGetCall), [*locations endpoints get iam policy*](api::ProjectLocationEndpointGetIamPolicyCall), [*locations endpoints invoke invoke*](api::ProjectLocationEndpointInvokeInvokeCall), [*locations endpoints list*](api::ProjectLocationEndpointListCall), [*locations endpoints mutate deployed model*](api::ProjectLocationEndpointMutateDeployedModelCall), [*locations endpoints openapi embeddings*](api::ProjectLocationEndpointOpenapiEmbeddingCall), [*locations endpoints operations cancel*](api::ProjectLocationEndpointOperationCancelCall), [*locations endpoints operations delete*](api::ProjectLocationEndpointOperationDeleteCall), [*locations endpoints operations get*](api::ProjectLocationEndpointOperationGetCall), [*locations endpoints operations list*](api::ProjectLocationEndpointOperationListCall), [*locations endpoints operations wait*](api::ProjectLocationEndpointOperationWaitCall), [*locations endpoints patch*](api::ProjectLocationEndpointPatchCall), [*locations endpoints predict*](api::ProjectLocationEndpointPredictCall), [*locations endpoints predict long running*](api::ProjectLocationEndpointPredictLongRunningCall), [*locations endpoints raw predict*](api::ProjectLocationEndpointRawPredictCall), [*locations endpoints server streaming predict*](api::ProjectLocationEndpointServerStreamingPredictCall), [*locations endpoints set iam policy*](api::ProjectLocationEndpointSetIamPolicyCall), [*locations endpoints stream generate content*](api::ProjectLocationEndpointStreamGenerateContentCall), [*locations endpoints stream raw predict*](api::ProjectLocationEndpointStreamRawPredictCall), [*locations endpoints test iam permissions*](api::ProjectLocationEndpointTestIamPermissionCall), [*locations endpoints undeploy model*](api::ProjectLocationEndpointUndeployModelCall), [*locations endpoints update*](api::ProjectLocationEndpointUpdateCall), [*locations evaluate dataset*](api::ProjectLocationEvaluateDatasetCall), [*locations evaluate instances*](api::ProjectLocationEvaluateInstanceCall), [*locations evaluation items create*](api::ProjectLocationEvaluationItemCreateCall), [*locations evaluation items delete*](api::ProjectLocationEvaluationItemDeleteCall), [*locations evaluation items get*](api::ProjectLocationEvaluationItemGetCall), [*locations evaluation items list*](api::ProjectLocationEvaluationItemListCall), [*locations evaluation items operations delete*](api::ProjectLocationEvaluationItemOperationDeleteCall), [*locations evaluation items operations get*](api::ProjectLocationEvaluationItemOperationGetCall), [*locations evaluation items operations list*](api::ProjectLocationEvaluationItemOperationListCall), [*locations evaluation items operations wait*](api::ProjectLocationEvaluationItemOperationWaitCall), [*locations evaluation runs cancel*](api::ProjectLocationEvaluationRunCancelCall), [*locations evaluation runs create*](api::ProjectLocationEvaluationRunCreateCall), [*locations evaluation runs delete*](api::ProjectLocationEvaluationRunDeleteCall), [*locations evaluation runs get*](api::ProjectLocationEvaluationRunGetCall), [*locations evaluation runs list*](api::ProjectLocationEvaluationRunListCall), [*locations evaluation runs operations delete*](api::ProjectLocationEvaluationRunOperationDeleteCall), [*locations evaluation runs operations get*](api::ProjectLocationEvaluationRunOperationGetCall), [*locations evaluation runs operations list*](api::ProjectLocationEvaluationRunOperationListCall), [*locations evaluation runs operations wait*](api::ProjectLocationEvaluationRunOperationWaitCall), [*locations evaluation sets create*](api::ProjectLocationEvaluationSetCreateCall), [*locations evaluation sets delete*](api::ProjectLocationEvaluationSetDeleteCall), [*locations evaluation sets get*](api::ProjectLocationEvaluationSetGetCall), [*locations evaluation sets list*](api::ProjectLocationEvaluationSetListCall), [*locations evaluation sets operations delete*](api::ProjectLocationEvaluationSetOperationDeleteCall), [*locations evaluation sets operations get*](api::ProjectLocationEvaluationSetOperationGetCall), [*locations evaluation sets operations list*](api::ProjectLocationEvaluationSetOperationListCall), [*locations evaluation sets operations wait*](api::ProjectLocationEvaluationSetOperationWaitCall), [*locations evaluation sets patch*](api::ProjectLocationEvaluationSetPatchCall), [*locations evaluation tasks operations delete*](api::ProjectLocationEvaluationTaskOperationDeleteCall), [*locations evaluation tasks operations get*](api::ProjectLocationEvaluationTaskOperationGetCall), [*locations evaluation tasks operations list*](api::ProjectLocationEvaluationTaskOperationListCall), [*locations evaluation tasks operations wait*](api::ProjectLocationEvaluationTaskOperationWaitCall), [*locations example stores create*](api::ProjectLocationExampleStoreCreateCall), [*locations example stores delete*](api::ProjectLocationExampleStoreDeleteCall), [*locations example stores fetch examples*](api::ProjectLocationExampleStoreFetchExampleCall), [*locations example stores get*](api::ProjectLocationExampleStoreGetCall), [*locations example stores list*](api::ProjectLocationExampleStoreListCall), [*locations example stores operations cancel*](api::ProjectLocationExampleStoreOperationCancelCall), [*locations example stores operations delete*](api::ProjectLocationExampleStoreOperationDeleteCall), [*locations example stores operations get*](api::ProjectLocationExampleStoreOperationGetCall), [*locations example stores operations list*](api::ProjectLocationExampleStoreOperationListCall), [*locations example stores operations wait*](api::ProjectLocationExampleStoreOperationWaitCall), [*locations example stores patch*](api::ProjectLocationExampleStorePatchCall), [*locations example stores remove examples*](api::ProjectLocationExampleStoreRemoveExampleCall), [*locations example stores search examples*](api::ProjectLocationExampleStoreSearchExampleCall), [*locations example stores upsert examples*](api::ProjectLocationExampleStoreUpsertExampleCall), [*locations extension controllers operations cancel*](api::ProjectLocationExtensionControllerOperationCancelCall), [*locations extension controllers operations delete*](api::ProjectLocationExtensionControllerOperationDeleteCall), [*locations extension controllers operations get*](api::ProjectLocationExtensionControllerOperationGetCall), [*locations extension controllers operations list*](api::ProjectLocationExtensionControllerOperationListCall), [*locations extension controllers operations wait*](api::ProjectLocationExtensionControllerOperationWaitCall), [*locations extensions delete*](api::ProjectLocationExtensionDeleteCall), [*locations extensions execute*](api::ProjectLocationExtensionExecuteCall), [*locations extensions get*](api::ProjectLocationExtensionGetCall), [*locations extensions import*](api::ProjectLocationExtensionImportCall), [*locations extensions list*](api::ProjectLocationExtensionListCall), [*locations extensions operations cancel*](api::ProjectLocationExtensionOperationCancelCall), [*locations extensions operations delete*](api::ProjectLocationExtensionOperationDeleteCall), [*locations extensions operations get*](api::ProjectLocationExtensionOperationGetCall), [*locations extensions operations list*](api::ProjectLocationExtensionOperationListCall), [*locations extensions operations wait*](api::ProjectLocationExtensionOperationWaitCall), [*locations extensions patch*](api::ProjectLocationExtensionPatchCall), [*locations extensions query*](api::ProjectLocationExtensionQueryCall), [*locations feature groups create*](api::ProjectLocationFeatureGroupCreateCall), [*locations feature groups delete*](api::ProjectLocationFeatureGroupDeleteCall), [*locations feature groups feature monitors create*](api::ProjectLocationFeatureGroupFeatureMonitorCreateCall), [*locations feature groups feature monitors delete*](api::ProjectLocationFeatureGroupFeatureMonitorDeleteCall), [*locations feature groups feature monitors feature monitor jobs create*](api::ProjectLocationFeatureGroupFeatureMonitorFeatureMonitorJobCreateCall), [*locations feature groups feature monitors feature monitor jobs get*](api::ProjectLocationFeatureGroupFeatureMonitorFeatureMonitorJobGetCall), [*locations feature groups feature monitors feature monitor jobs list*](api::ProjectLocationFeatureGroupFeatureMonitorFeatureMonitorJobListCall), [*locations feature groups feature monitors get*](api::ProjectLocationFeatureGroupFeatureMonitorGetCall), [*locations feature groups feature monitors list*](api::ProjectLocationFeatureGroupFeatureMonitorListCall), [*locations feature groups feature monitors operations delete*](api::ProjectLocationFeatureGroupFeatureMonitorOperationDeleteCall), [*locations feature groups feature monitors operations get*](api::ProjectLocationFeatureGroupFeatureMonitorOperationGetCall), [*locations feature groups feature monitors operations list*](api::ProjectLocationFeatureGroupFeatureMonitorOperationListCall), [*locations feature groups feature monitors operations wait*](api::ProjectLocationFeatureGroupFeatureMonitorOperationWaitCall), [*locations feature groups feature monitors patch*](api::ProjectLocationFeatureGroupFeatureMonitorPatchCall), [*locations feature groups features batch create*](api::ProjectLocationFeatureGroupFeatureBatchCreateCall), [*locations feature groups features create*](api::ProjectLocationFeatureGroupFeatureCreateCall), [*locations feature groups features delete*](api::ProjectLocationFeatureGroupFeatureDeleteCall), [*locations feature groups features get*](api::ProjectLocationFeatureGroupFeatureGetCall), [*locations feature groups features list*](api::ProjectLocationFeatureGroupFeatureListCall), [*locations feature groups features operations delete*](api::ProjectLocationFeatureGroupFeatureOperationDeleteCall), [*locations feature groups features operations get*](api::ProjectLocationFeatureGroupFeatureOperationGetCall), [*locations feature groups features operations list*](api::ProjectLocationFeatureGroupFeatureOperationListCall), [*locations feature groups features operations wait*](api::ProjectLocationFeatureGroupFeatureOperationWaitCall), [*locations feature groups features patch*](api::ProjectLocationFeatureGroupFeaturePatchCall), [*locations feature groups get*](api::ProjectLocationFeatureGroupGetCall), [*locations feature groups get iam policy*](api::ProjectLocationFeatureGroupGetIamPolicyCall), [*locations feature groups list*](api::ProjectLocationFeatureGroupListCall), [*locations feature groups operations delete*](api::ProjectLocationFeatureGroupOperationDeleteCall), [*locations feature groups operations get*](api::ProjectLocationFeatureGroupOperationGetCall), [*locations feature groups operations list*](api::ProjectLocationFeatureGroupOperationListCall), [*locations feature groups operations wait*](api::ProjectLocationFeatureGroupOperationWaitCall), [*locations feature groups patch*](api::ProjectLocationFeatureGroupPatchCall), [*locations feature groups set iam policy*](api::ProjectLocationFeatureGroupSetIamPolicyCall), [*locations feature groups test iam permissions*](api::ProjectLocationFeatureGroupTestIamPermissionCall), [*locations feature online stores create*](api::ProjectLocationFeatureOnlineStoreCreateCall), [*locations feature online stores delete*](api::ProjectLocationFeatureOnlineStoreDeleteCall), [*locations feature online stores feature views create*](api::ProjectLocationFeatureOnlineStoreFeatureViewCreateCall), [*locations feature online stores feature views delete*](api::ProjectLocationFeatureOnlineStoreFeatureViewDeleteCall), [*locations feature online stores feature views direct write*](api::ProjectLocationFeatureOnlineStoreFeatureViewDirectWriteCall), [*locations feature online stores feature views feature view syncs get*](api::ProjectLocationFeatureOnlineStoreFeatureViewFeatureViewSyncGetCall), [*locations feature online stores feature views feature view syncs list*](api::ProjectLocationFeatureOnlineStoreFeatureViewFeatureViewSyncListCall), [*locations feature online stores feature views fetch feature values*](api::ProjectLocationFeatureOnlineStoreFeatureViewFetchFeatureValueCall), [*locations feature online stores feature views generate fetch access token*](api::ProjectLocationFeatureOnlineStoreFeatureViewGenerateFetchAccessTokenCall), [*locations feature online stores feature views get*](api::ProjectLocationFeatureOnlineStoreFeatureViewGetCall), [*locations feature online stores feature views get iam policy*](api::ProjectLocationFeatureOnlineStoreFeatureViewGetIamPolicyCall), [*locations feature online stores feature views list*](api::ProjectLocationFeatureOnlineStoreFeatureViewListCall), [*locations feature online stores feature views operations delete*](api::ProjectLocationFeatureOnlineStoreFeatureViewOperationDeleteCall), [*locations feature online stores feature views operations get*](api::ProjectLocationFeatureOnlineStoreFeatureViewOperationGetCall), [*locations feature online stores feature views operations list*](api::ProjectLocationFeatureOnlineStoreFeatureViewOperationListCall), [*locations feature online stores feature views operations wait*](api::ProjectLocationFeatureOnlineStoreFeatureViewOperationWaitCall), [*locations feature online stores feature views patch*](api::ProjectLocationFeatureOnlineStoreFeatureViewPatchCall), [*locations feature online stores feature views search nearest entities*](api::ProjectLocationFeatureOnlineStoreFeatureViewSearchNearestEntityCall), [*locations feature online stores feature views set iam policy*](api::ProjectLocationFeatureOnlineStoreFeatureViewSetIamPolicyCall), [*locations feature online stores feature views streaming fetch feature values*](api::ProjectLocationFeatureOnlineStoreFeatureViewStreamingFetchFeatureValueCall), [*locations feature online stores feature views sync*](api::ProjectLocationFeatureOnlineStoreFeatureViewSyncCall), [*locations feature online stores feature views test iam permissions*](api::ProjectLocationFeatureOnlineStoreFeatureViewTestIamPermissionCall), [*locations feature online stores get*](api::ProjectLocationFeatureOnlineStoreGetCall), [*locations feature online stores get iam policy*](api::ProjectLocationFeatureOnlineStoreGetIamPolicyCall), [*locations feature online stores list*](api::ProjectLocationFeatureOnlineStoreListCall), [*locations feature online stores operations delete*](api::ProjectLocationFeatureOnlineStoreOperationDeleteCall), [*locations feature online stores operations get*](api::ProjectLocationFeatureOnlineStoreOperationGetCall), [*locations feature online stores operations list*](api::ProjectLocationFeatureOnlineStoreOperationListCall), [*locations feature online stores operations wait*](api::ProjectLocationFeatureOnlineStoreOperationWaitCall), [*locations feature online stores patch*](api::ProjectLocationFeatureOnlineStorePatchCall), [*locations feature online stores set iam policy*](api::ProjectLocationFeatureOnlineStoreSetIamPolicyCall), [*locations feature online stores test iam permissions*](api::ProjectLocationFeatureOnlineStoreTestIamPermissionCall), [*locations featurestores batch read feature values*](api::ProjectLocationFeaturestoreBatchReadFeatureValueCall), [*locations featurestores create*](api::ProjectLocationFeaturestoreCreateCall), [*locations featurestores delete*](api::ProjectLocationFeaturestoreDeleteCall), [*locations featurestores entity types create*](api::ProjectLocationFeaturestoreEntityTypeCreateCall), [*locations featurestores entity types delete*](api::ProjectLocationFeaturestoreEntityTypeDeleteCall), [*locations featurestores entity types delete feature values*](api::ProjectLocationFeaturestoreEntityTypeDeleteFeatureValueCall), [*locations featurestores entity types export feature values*](api::ProjectLocationFeaturestoreEntityTypeExportFeatureValueCall), [*locations featurestores entity types features batch create*](api::ProjectLocationFeaturestoreEntityTypeFeatureBatchCreateCall), [*locations featurestores entity types features create*](api::ProjectLocationFeaturestoreEntityTypeFeatureCreateCall), [*locations featurestores entity types features delete*](api::ProjectLocationFeaturestoreEntityTypeFeatureDeleteCall), [*locations featurestores entity types features get*](api::ProjectLocationFeaturestoreEntityTypeFeatureGetCall), [*locations featurestores entity types features list*](api::ProjectLocationFeaturestoreEntityTypeFeatureListCall), [*locations featurestores entity types features operations cancel*](api::ProjectLocationFeaturestoreEntityTypeFeatureOperationCancelCall), [*locations featurestores entity types features operations delete*](api::ProjectLocationFeaturestoreEntityTypeFeatureOperationDeleteCall), [*locations featurestores entity types features operations get*](api::ProjectLocationFeaturestoreEntityTypeFeatureOperationGetCall), [*locations featurestores entity types features operations list*](api::ProjectLocationFeaturestoreEntityTypeFeatureOperationListCall), [*locations featurestores entity types features operations wait*](api::ProjectLocationFeaturestoreEntityTypeFeatureOperationWaitCall), [*locations featurestores entity types features patch*](api::ProjectLocationFeaturestoreEntityTypeFeaturePatchCall), [*locations featurestores entity types get*](api::ProjectLocationFeaturestoreEntityTypeGetCall), [*locations featurestores entity types get iam policy*](api::ProjectLocationFeaturestoreEntityTypeGetIamPolicyCall), [*locations featurestores entity types import feature values*](api::ProjectLocationFeaturestoreEntityTypeImportFeatureValueCall), [*locations featurestores entity types list*](api::ProjectLocationFeaturestoreEntityTypeListCall), [*locations featurestores entity types operations cancel*](api::ProjectLocationFeaturestoreEntityTypeOperationCancelCall), [*locations featurestores entity types operations delete*](api::ProjectLocationFeaturestoreEntityTypeOperationDeleteCall), [*locations featurestores entity types operations get*](api::ProjectLocationFeaturestoreEntityTypeOperationGetCall), [*locations featurestores entity types operations list*](api::ProjectLocationFeaturestoreEntityTypeOperationListCall), [*locations featurestores entity types operations wait*](api::ProjectLocationFeaturestoreEntityTypeOperationWaitCall), [*locations featurestores entity types patch*](api::ProjectLocationFeaturestoreEntityTypePatchCall), [*locations featurestores entity types read feature values*](api::ProjectLocationFeaturestoreEntityTypeReadFeatureValueCall), [*locations featurestores entity types set iam policy*](api::ProjectLocationFeaturestoreEntityTypeSetIamPolicyCall), [*locations featurestores entity types streaming read feature values*](api::ProjectLocationFeaturestoreEntityTypeStreamingReadFeatureValueCall), [*locations featurestores entity types test iam permissions*](api::ProjectLocationFeaturestoreEntityTypeTestIamPermissionCall), [*locations featurestores entity types write feature values*](api::ProjectLocationFeaturestoreEntityTypeWriteFeatureValueCall), [*locations featurestores get*](api::ProjectLocationFeaturestoreGetCall), [*locations featurestores get iam policy*](api::ProjectLocationFeaturestoreGetIamPolicyCall), [*locations featurestores list*](api::ProjectLocationFeaturestoreListCall), [*locations featurestores operations cancel*](api::ProjectLocationFeaturestoreOperationCancelCall), [*locations featurestores operations delete*](api::ProjectLocationFeaturestoreOperationDeleteCall), [*locations featurestores operations get*](api::ProjectLocationFeaturestoreOperationGetCall), [*locations featurestores operations list*](api::ProjectLocationFeaturestoreOperationListCall), [*locations featurestores operations wait*](api::ProjectLocationFeaturestoreOperationWaitCall), [*locations featurestores patch*](api::ProjectLocationFeaturestorePatchCall), [*locations featurestores search features*](api::ProjectLocationFeaturestoreSearchFeatureCall), [*locations featurestores set iam policy*](api::ProjectLocationFeaturestoreSetIamPolicyCall), [*locations featurestores test iam permissions*](api::ProjectLocationFeaturestoreTestIamPermissionCall), [*locations generate instance rubrics*](api::ProjectLocationGenerateInstanceRubricCall), [*locations generate synthetic data*](api::ProjectLocationGenerateSyntheticDataCall), [*locations get*](api::ProjectLocationGetCall), [*locations get rag engine config*](api::ProjectLocationGetRagEngineConfigCall), [*locations hyperparameter tuning jobs cancel*](api::ProjectLocationHyperparameterTuningJobCancelCall), [*locations hyperparameter tuning jobs create*](api::ProjectLocationHyperparameterTuningJobCreateCall), [*locations hyperparameter tuning jobs delete*](api::ProjectLocationHyperparameterTuningJobDeleteCall), [*locations hyperparameter tuning jobs get*](api::ProjectLocationHyperparameterTuningJobGetCall), [*locations hyperparameter tuning jobs list*](api::ProjectLocationHyperparameterTuningJobListCall), [*locations hyperparameter tuning jobs operations cancel*](api::ProjectLocationHyperparameterTuningJobOperationCancelCall), [*locations hyperparameter tuning jobs operations delete*](api::ProjectLocationHyperparameterTuningJobOperationDeleteCall), [*locations hyperparameter tuning jobs operations get*](api::ProjectLocationHyperparameterTuningJobOperationGetCall), [*locations hyperparameter tuning jobs operations list*](api::ProjectLocationHyperparameterTuningJobOperationListCall), [*locations hyperparameter tuning jobs operations wait*](api::ProjectLocationHyperparameterTuningJobOperationWaitCall), [*locations index endpoints create*](api::ProjectLocationIndexEndpointCreateCall), [*locations index endpoints delete*](api::ProjectLocationIndexEndpointDeleteCall), [*locations index endpoints deploy index*](api::ProjectLocationIndexEndpointDeployIndexCall), [*locations index endpoints find neighbors*](api::ProjectLocationIndexEndpointFindNeighborCall), [*locations index endpoints get*](api::ProjectLocationIndexEndpointGetCall), [*locations index endpoints list*](api::ProjectLocationIndexEndpointListCall), [*locations index endpoints mutate deployed index*](api::ProjectLocationIndexEndpointMutateDeployedIndexCall), [*locations index endpoints operations cancel*](api::ProjectLocationIndexEndpointOperationCancelCall), [*locations index endpoints operations delete*](api::ProjectLocationIndexEndpointOperationDeleteCall), [*locations index endpoints operations get*](api::ProjectLocationIndexEndpointOperationGetCall), [*locations index endpoints operations list*](api::ProjectLocationIndexEndpointOperationListCall), [*locations index endpoints operations wait*](api::ProjectLocationIndexEndpointOperationWaitCall), [*locations index endpoints patch*](api::ProjectLocationIndexEndpointPatchCall), [*locations index endpoints read index datapoints*](api::ProjectLocationIndexEndpointReadIndexDatapointCall), [*locations index endpoints undeploy index*](api::ProjectLocationIndexEndpointUndeployIndexCall), [*locations indexes create*](api::ProjectLocationIndexCreateCall), [*locations indexes delete*](api::ProjectLocationIndexDeleteCall), [*locations indexes get*](api::ProjectLocationIndexGetCall), [*locations indexes import*](api::ProjectLocationIndexImportCall), [*locations indexes list*](api::ProjectLocationIndexListCall), [*locations indexes operations cancel*](api::ProjectLocationIndexOperationCancelCall), [*locations indexes operations delete*](api::ProjectLocationIndexOperationDeleteCall), [*locations indexes operations get*](api::ProjectLocationIndexOperationGetCall), [*locations indexes operations list*](api::ProjectLocationIndexOperationListCall), [*locations indexes operations wait*](api::ProjectLocationIndexOperationWaitCall), [*locations indexes patch*](api::ProjectLocationIndexPatchCall), [*locations indexes remove datapoints*](api::ProjectLocationIndexRemoveDatapointCall), [*locations indexes upsert datapoints*](api::ProjectLocationIndexUpsertDatapointCall), [*locations list*](api::ProjectLocationListCall), [*locations metadata stores artifacts create*](api::ProjectLocationMetadataStoreArtifactCreateCall), [*locations metadata stores artifacts delete*](api::ProjectLocationMetadataStoreArtifactDeleteCall), [*locations metadata stores artifacts get*](api::ProjectLocationMetadataStoreArtifactGetCall), [*locations metadata stores artifacts list*](api::ProjectLocationMetadataStoreArtifactListCall), [*locations metadata stores artifacts operations cancel*](api::ProjectLocationMetadataStoreArtifactOperationCancelCall), [*locations metadata stores artifacts operations delete*](api::ProjectLocationMetadataStoreArtifactOperationDeleteCall), [*locations metadata stores artifacts operations get*](api::ProjectLocationMetadataStoreArtifactOperationGetCall), [*locations metadata stores artifacts operations list*](api::ProjectLocationMetadataStoreArtifactOperationListCall), [*locations metadata stores artifacts operations wait*](api::ProjectLocationMetadataStoreArtifactOperationWaitCall), [*locations metadata stores artifacts patch*](api::ProjectLocationMetadataStoreArtifactPatchCall), [*locations metadata stores artifacts purge*](api::ProjectLocationMetadataStoreArtifactPurgeCall), [*locations metadata stores artifacts query artifact lineage subgraph*](api::ProjectLocationMetadataStoreArtifactQueryArtifactLineageSubgraphCall), [*locations metadata stores contexts add context artifacts and executions*](api::ProjectLocationMetadataStoreContextAddContextArtifactsAndExecutionCall), [*locations metadata stores contexts add context children*](api::ProjectLocationMetadataStoreContextAddContextChildrenCall), [*locations metadata stores contexts create*](api::ProjectLocationMetadataStoreContextCreateCall), [*locations metadata stores contexts delete*](api::ProjectLocationMetadataStoreContextDeleteCall), [*locations metadata stores contexts get*](api::ProjectLocationMetadataStoreContextGetCall), [*locations metadata stores contexts list*](api::ProjectLocationMetadataStoreContextListCall), [*locations metadata stores contexts operations cancel*](api::ProjectLocationMetadataStoreContextOperationCancelCall), [*locations metadata stores contexts operations delete*](api::ProjectLocationMetadataStoreContextOperationDeleteCall), [*locations metadata stores contexts operations get*](api::ProjectLocationMetadataStoreContextOperationGetCall), [*locations metadata stores contexts operations list*](api::ProjectLocationMetadataStoreContextOperationListCall), [*locations metadata stores contexts operations wait*](api::ProjectLocationMetadataStoreContextOperationWaitCall), [*locations metadata stores contexts patch*](api::ProjectLocationMetadataStoreContextPatchCall), [*locations metadata stores contexts purge*](api::ProjectLocationMetadataStoreContextPurgeCall), [*locations metadata stores contexts query context lineage subgraph*](api::ProjectLocationMetadataStoreContextQueryContextLineageSubgraphCall), [*locations metadata stores contexts remove context children*](api::ProjectLocationMetadataStoreContextRemoveContextChildrenCall), [*locations metadata stores create*](api::ProjectLocationMetadataStoreCreateCall), [*locations metadata stores delete*](api::ProjectLocationMetadataStoreDeleteCall), [*locations metadata stores executions add execution events*](api::ProjectLocationMetadataStoreExecutionAddExecutionEventCall), [*locations metadata stores executions create*](api::ProjectLocationMetadataStoreExecutionCreateCall), [*locations metadata stores executions delete*](api::ProjectLocationMetadataStoreExecutionDeleteCall), [*locations metadata stores executions get*](api::ProjectLocationMetadataStoreExecutionGetCall), [*locations metadata stores executions list*](api::ProjectLocationMetadataStoreExecutionListCall), [*locations metadata stores executions operations cancel*](api::ProjectLocationMetadataStoreExecutionOperationCancelCall), [*locations metadata stores executions operations delete*](api::ProjectLocationMetadataStoreExecutionOperationDeleteCall), [*locations metadata stores executions operations get*](api::ProjectLocationMetadataStoreExecutionOperationGetCall), [*locations metadata stores executions operations list*](api::ProjectLocationMetadataStoreExecutionOperationListCall), [*locations metadata stores executions operations wait*](api::ProjectLocationMetadataStoreExecutionOperationWaitCall), [*locations metadata stores executions patch*](api::ProjectLocationMetadataStoreExecutionPatchCall), [*locations metadata stores executions purge*](api::ProjectLocationMetadataStoreExecutionPurgeCall), [*locations metadata stores executions query execution inputs and outputs*](api::ProjectLocationMetadataStoreExecutionQueryExecutionInputsAndOutputCall), [*locations metadata stores get*](api::ProjectLocationMetadataStoreGetCall), [*locations metadata stores list*](api::ProjectLocationMetadataStoreListCall), [*locations metadata stores metadata schemas create*](api::ProjectLocationMetadataStoreMetadataSchemaCreateCall), [*locations metadata stores metadata schemas get*](api::ProjectLocationMetadataStoreMetadataSchemaGetCall), [*locations metadata stores metadata schemas list*](api::ProjectLocationMetadataStoreMetadataSchemaListCall), [*locations metadata stores operations cancel*](api::ProjectLocationMetadataStoreOperationCancelCall), [*locations metadata stores operations delete*](api::ProjectLocationMetadataStoreOperationDeleteCall), [*locations metadata stores operations get*](api::ProjectLocationMetadataStoreOperationGetCall), [*locations metadata stores operations list*](api::ProjectLocationMetadataStoreOperationListCall), [*locations metadata stores operations wait*](api::ProjectLocationMetadataStoreOperationWaitCall), [*locations migratable resources batch migrate*](api::ProjectLocationMigratableResourceBatchMigrateCall), [*locations migratable resources operations cancel*](api::ProjectLocationMigratableResourceOperationCancelCall), [*locations migratable resources operations delete*](api::ProjectLocationMigratableResourceOperationDeleteCall), [*locations migratable resources operations get*](api::ProjectLocationMigratableResourceOperationGetCall), [*locations migratable resources operations list*](api::ProjectLocationMigratableResourceOperationListCall), [*locations migratable resources operations wait*](api::ProjectLocationMigratableResourceOperationWaitCall), [*locations migratable resources search*](api::ProjectLocationMigratableResourceSearchCall), [*locations model deployment monitoring jobs create*](api::ProjectLocationModelDeploymentMonitoringJobCreateCall), [*locations model deployment monitoring jobs delete*](api::ProjectLocationModelDeploymentMonitoringJobDeleteCall), [*locations model deployment monitoring jobs get*](api::ProjectLocationModelDeploymentMonitoringJobGetCall), [*locations model deployment monitoring jobs list*](api::ProjectLocationModelDeploymentMonitoringJobListCall), [*locations model deployment monitoring jobs operations cancel*](api::ProjectLocationModelDeploymentMonitoringJobOperationCancelCall), [*locations model deployment monitoring jobs operations delete*](api::ProjectLocationModelDeploymentMonitoringJobOperationDeleteCall), [*locations model deployment monitoring jobs operations get*](api::ProjectLocationModelDeploymentMonitoringJobOperationGetCall), [*locations model deployment monitoring jobs operations list*](api::ProjectLocationModelDeploymentMonitoringJobOperationListCall), [*locations model deployment monitoring jobs operations wait*](api::ProjectLocationModelDeploymentMonitoringJobOperationWaitCall), [*locations model deployment monitoring jobs patch*](api::ProjectLocationModelDeploymentMonitoringJobPatchCall), [*locations model deployment monitoring jobs pause*](api::ProjectLocationModelDeploymentMonitoringJobPauseCall), [*locations model deployment monitoring jobs resume*](api::ProjectLocationModelDeploymentMonitoringJobResumeCall), [*locations model deployment monitoring jobs search model deployment monitoring stats anomalies*](api::ProjectLocationModelDeploymentMonitoringJobSearchModelDeploymentMonitoringStatsAnomalyCall), [*locations model monitors create*](api::ProjectLocationModelMonitorCreateCall), [*locations model monitors delete*](api::ProjectLocationModelMonitorDeleteCall), [*locations model monitors get*](api::ProjectLocationModelMonitorGetCall), [*locations model monitors list*](api::ProjectLocationModelMonitorListCall), [*locations model monitors model monitoring jobs create*](api::ProjectLocationModelMonitorModelMonitoringJobCreateCall), [*locations model monitors model monitoring jobs delete*](api::ProjectLocationModelMonitorModelMonitoringJobDeleteCall), [*locations model monitors model monitoring jobs get*](api::ProjectLocationModelMonitorModelMonitoringJobGetCall), [*locations model monitors model monitoring jobs list*](api::ProjectLocationModelMonitorModelMonitoringJobListCall), [*locations model monitors operations cancel*](api::ProjectLocationModelMonitorOperationCancelCall), [*locations model monitors operations delete*](api::ProjectLocationModelMonitorOperationDeleteCall), [*locations model monitors operations get*](api::ProjectLocationModelMonitorOperationGetCall), [*locations model monitors operations list*](api::ProjectLocationModelMonitorOperationListCall), [*locations model monitors operations wait*](api::ProjectLocationModelMonitorOperationWaitCall), [*locations model monitors patch*](api::ProjectLocationModelMonitorPatchCall), [*locations model monitors search model monitoring alerts*](api::ProjectLocationModelMonitorSearchModelMonitoringAlertCall), [*locations model monitors search model monitoring stats*](api::ProjectLocationModelMonitorSearchModelMonitoringStatCall), [*locations models copy*](api::ProjectLocationModelCopyCall), [*locations models delete*](api::ProjectLocationModelDeleteCall), [*locations models delete version*](api::ProjectLocationModelDeleteVersionCall), [*locations models evaluations get*](api::ProjectLocationModelEvaluationGetCall), [*locations models evaluations import*](api::ProjectLocationModelEvaluationImportCall), [*locations models evaluations list*](api::ProjectLocationModelEvaluationListCall), [*locations models evaluations operations cancel*](api::ProjectLocationModelEvaluationOperationCancelCall), [*locations models evaluations operations delete*](api::ProjectLocationModelEvaluationOperationDeleteCall), [*locations models evaluations operations get*](api::ProjectLocationModelEvaluationOperationGetCall), [*locations models evaluations operations list*](api::ProjectLocationModelEvaluationOperationListCall), [*locations models evaluations operations wait*](api::ProjectLocationModelEvaluationOperationWaitCall), [*locations models evaluations slices batch import*](api::ProjectLocationModelEvaluationSliceBatchImportCall), [*locations models evaluations slices get*](api::ProjectLocationModelEvaluationSliceGetCall), [*locations models evaluations slices list*](api::ProjectLocationModelEvaluationSliceListCall), [*locations models export*](api::ProjectLocationModelExportCall), [*locations models get*](api::ProjectLocationModelGetCall), [*locations models get iam policy*](api::ProjectLocationModelGetIamPolicyCall), [*locations models list*](api::ProjectLocationModelListCall), [*locations models list checkpoints*](api::ProjectLocationModelListCheckpointCall), [*locations models list versions*](api::ProjectLocationModelListVersionCall), [*locations models merge version aliases*](api::ProjectLocationModelMergeVersionAliaseCall), [*locations models operations cancel*](api::ProjectLocationModelOperationCancelCall), [*locations models operations delete*](api::ProjectLocationModelOperationDeleteCall), [*locations models operations get*](api::ProjectLocationModelOperationGetCall), [*locations models operations list*](api::ProjectLocationModelOperationListCall), [*locations models operations wait*](api::ProjectLocationModelOperationWaitCall), [*locations models patch*](api::ProjectLocationModelPatchCall), [*locations models set iam policy*](api::ProjectLocationModelSetIamPolicyCall), [*locations models test iam permissions*](api::ProjectLocationModelTestIamPermissionCall), [*locations models update explanation dataset*](api::ProjectLocationModelUpdateExplanationDatasetCall), [*locations models upload*](api::ProjectLocationModelUploadCall), [*locations nas jobs cancel*](api::ProjectLocationNasJobCancelCall), [*locations nas jobs create*](api::ProjectLocationNasJobCreateCall), [*locations nas jobs delete*](api::ProjectLocationNasJobDeleteCall), [*locations nas jobs get*](api::ProjectLocationNasJobGetCall), [*locations nas jobs list*](api::ProjectLocationNasJobListCall), [*locations nas jobs nas trial details get*](api::ProjectLocationNasJobNasTrialDetailGetCall), [*locations nas jobs nas trial details list*](api::ProjectLocationNasJobNasTrialDetailListCall), [*locations notebook execution jobs create*](api::ProjectLocationNotebookExecutionJobCreateCall), [*locations notebook execution jobs delete*](api::ProjectLocationNotebookExecutionJobDeleteCall), [*locations notebook execution jobs generate access token*](api::ProjectLocationNotebookExecutionJobGenerateAccessTokenCall), [*locations notebook execution jobs get*](api::ProjectLocationNotebookExecutionJobGetCall), [*locations notebook execution jobs list*](api::ProjectLocationNotebookExecutionJobListCall), [*locations notebook execution jobs operations cancel*](api::ProjectLocationNotebookExecutionJobOperationCancelCall), [*locations notebook execution jobs operations delete*](api::ProjectLocationNotebookExecutionJobOperationDeleteCall), [*locations notebook execution jobs operations get*](api::ProjectLocationNotebookExecutionJobOperationGetCall), [*locations notebook execution jobs operations list*](api::ProjectLocationNotebookExecutionJobOperationListCall), [*locations notebook execution jobs operations wait*](api::ProjectLocationNotebookExecutionJobOperationWaitCall), [*locations notebook execution jobs report event*](api::ProjectLocationNotebookExecutionJobReportEventCall), [*locations notebook runtime templates create*](api::ProjectLocationNotebookRuntimeTemplateCreateCall), [*locations notebook runtime templates delete*](api::ProjectLocationNotebookRuntimeTemplateDeleteCall), [*locations notebook runtime templates get*](api::ProjectLocationNotebookRuntimeTemplateGetCall), [*locations notebook runtime templates get iam policy*](api::ProjectLocationNotebookRuntimeTemplateGetIamPolicyCall), [*locations notebook runtime templates list*](api::ProjectLocationNotebookRuntimeTemplateListCall), [*locations notebook runtime templates operations cancel*](api::ProjectLocationNotebookRuntimeTemplateOperationCancelCall), [*locations notebook runtime templates operations delete*](api::ProjectLocationNotebookRuntimeTemplateOperationDeleteCall), [*locations notebook runtime templates operations get*](api::ProjectLocationNotebookRuntimeTemplateOperationGetCall), [*locations notebook runtime templates operations list*](api::ProjectLocationNotebookRuntimeTemplateOperationListCall), [*locations notebook runtime templates operations wait*](api::ProjectLocationNotebookRuntimeTemplateOperationWaitCall), [*locations notebook runtime templates patch*](api::ProjectLocationNotebookRuntimeTemplatePatchCall), [*locations notebook runtime templates set iam policy*](api::ProjectLocationNotebookRuntimeTemplateSetIamPolicyCall), [*locations notebook runtime templates test iam permissions*](api::ProjectLocationNotebookRuntimeTemplateTestIamPermissionCall), [*locations notebook runtimes assign*](api::ProjectLocationNotebookRuntimeAssignCall), [*locations notebook runtimes delete*](api::ProjectLocationNotebookRuntimeDeleteCall), [*locations notebook runtimes generate access token*](api::ProjectLocationNotebookRuntimeGenerateAccessTokenCall), [*locations notebook runtimes get*](api::ProjectLocationNotebookRuntimeGetCall), [*locations notebook runtimes list*](api::ProjectLocationNotebookRuntimeListCall), [*locations notebook runtimes operations cancel*](api::ProjectLocationNotebookRuntimeOperationCancelCall), [*locations notebook runtimes operations delete*](api::ProjectLocationNotebookRuntimeOperationDeleteCall), [*locations notebook runtimes operations get*](api::ProjectLocationNotebookRuntimeOperationGetCall), [*locations notebook runtimes operations list*](api::ProjectLocationNotebookRuntimeOperationListCall), [*locations notebook runtimes operations wait*](api::ProjectLocationNotebookRuntimeOperationWaitCall), [*locations notebook runtimes report event*](api::ProjectLocationNotebookRuntimeReportEventCall), [*locations notebook runtimes start*](api::ProjectLocationNotebookRuntimeStartCall), [*locations notebook runtimes stop*](api::ProjectLocationNotebookRuntimeStopCall), [*locations notebook runtimes upgrade*](api::ProjectLocationNotebookRuntimeUpgradeCall), [*locations operations cancel*](api::ProjectLocationOperationCancelCall), [*locations operations delete*](api::ProjectLocationOperationDeleteCall), [*locations operations get*](api::ProjectLocationOperationGetCall), [*locations operations list*](api::ProjectLocationOperationListCall), [*locations operations wait*](api::ProjectLocationOperationWaitCall), [*locations persistent resources create*](api::ProjectLocationPersistentResourceCreateCall), [*locations persistent resources delete*](api::ProjectLocationPersistentResourceDeleteCall), [*locations persistent resources get*](api::ProjectLocationPersistentResourceGetCall), [*locations persistent resources list*](api::ProjectLocationPersistentResourceListCall), [*locations persistent resources operations cancel*](api::ProjectLocationPersistentResourceOperationCancelCall), [*locations persistent resources operations delete*](api::ProjectLocationPersistentResourceOperationDeleteCall), [*locations persistent resources operations get*](api::ProjectLocationPersistentResourceOperationGetCall), [*locations persistent resources operations list*](api::ProjectLocationPersistentResourceOperationListCall), [*locations persistent resources operations wait*](api::ProjectLocationPersistentResourceOperationWaitCall), [*locations persistent resources patch*](api::ProjectLocationPersistentResourcePatchCall), [*locations persistent resources reboot*](api::ProjectLocationPersistentResourceRebootCall), [*locations pipeline jobs batch cancel*](api::ProjectLocationPipelineJobBatchCancelCall), [*locations pipeline jobs batch delete*](api::ProjectLocationPipelineJobBatchDeleteCall), [*locations pipeline jobs cancel*](api::ProjectLocationPipelineJobCancelCall), [*locations pipeline jobs create*](api::ProjectLocationPipelineJobCreateCall), [*locations pipeline jobs delete*](api::ProjectLocationPipelineJobDeleteCall), [*locations pipeline jobs get*](api::ProjectLocationPipelineJobGetCall), [*locations pipeline jobs list*](api::ProjectLocationPipelineJobListCall), [*locations pipeline jobs operations cancel*](api::ProjectLocationPipelineJobOperationCancelCall), [*locations pipeline jobs operations delete*](api::ProjectLocationPipelineJobOperationDeleteCall), [*locations pipeline jobs operations get*](api::ProjectLocationPipelineJobOperationGetCall), [*locations pipeline jobs operations list*](api::ProjectLocationPipelineJobOperationListCall), [*locations pipeline jobs operations wait*](api::ProjectLocationPipelineJobOperationWaitCall), [*locations publishers models compute tokens*](api::ProjectLocationPublisherModelComputeTokenCall), [*locations publishers models count tokens*](api::ProjectLocationPublisherModelCountTokenCall), [*locations publishers models embed content*](api::ProjectLocationPublisherModelEmbedContentCall), [*locations publishers models export*](api::ProjectLocationPublisherModelExportCall), [*locations publishers models fetch predict operation*](api::ProjectLocationPublisherModelFetchPredictOperationCall), [*locations publishers models fetch publisher model config*](api::ProjectLocationPublisherModelFetchPublisherModelConfigCall), [*locations publishers models generate content*](api::ProjectLocationPublisherModelGenerateContentCall), [*locations publishers models get iam policy*](api::ProjectLocationPublisherModelGetIamPolicyCall), [*locations publishers models predict*](api::ProjectLocationPublisherModelPredictCall), [*locations publishers models predict long running*](api::ProjectLocationPublisherModelPredictLongRunningCall), [*locations publishers models raw predict*](api::ProjectLocationPublisherModelRawPredictCall), [*locations publishers models server streaming predict*](api::ProjectLocationPublisherModelServerStreamingPredictCall), [*locations publishers models set publisher model config*](api::ProjectLocationPublisherModelSetPublisherModelConfigCall), [*locations publishers models stream generate content*](api::ProjectLocationPublisherModelStreamGenerateContentCall), [*locations publishers models stream raw predict*](api::ProjectLocationPublisherModelStreamRawPredictCall), [*locations rag corpora create*](api::ProjectLocationRagCorporaCreateCall), [*locations rag corpora delete*](api::ProjectLocationRagCorporaDeleteCall), [*locations rag corpora get*](api::ProjectLocationRagCorporaGetCall), [*locations rag corpora list*](api::ProjectLocationRagCorporaListCall), [*locations rag corpora operations cancel*](api::ProjectLocationRagCorporaOperationCancelCall), [*locations rag corpora operations delete*](api::ProjectLocationRagCorporaOperationDeleteCall), [*locations rag corpora operations get*](api::ProjectLocationRagCorporaOperationGetCall), [*locations rag corpora operations list*](api::ProjectLocationRagCorporaOperationListCall), [*locations rag corpora operations wait*](api::ProjectLocationRagCorporaOperationWaitCall), [*locations rag corpora patch*](api::ProjectLocationRagCorporaPatchCall), [*locations rag corpora rag files delete*](api::ProjectLocationRagCorporaRagFileDeleteCall), [*locations rag corpora rag files get*](api::ProjectLocationRagCorporaRagFileGetCall), [*locations rag corpora rag files import*](api::ProjectLocationRagCorporaRagFileImportCall), [*locations rag corpora rag files list*](api::ProjectLocationRagCorporaRagFileListCall), [*locations rag corpora rag files operations cancel*](api::ProjectLocationRagCorporaRagFileOperationCancelCall), [*locations rag corpora rag files operations delete*](api::ProjectLocationRagCorporaRagFileOperationDeleteCall), [*locations rag corpora rag files operations get*](api::ProjectLocationRagCorporaRagFileOperationGetCall), [*locations rag corpora rag files operations list*](api::ProjectLocationRagCorporaRagFileOperationListCall), [*locations rag corpora rag files operations wait*](api::ProjectLocationRagCorporaRagFileOperationWaitCall), [*locations rag engine config operations cancel*](api::ProjectLocationRagEngineConfigOperationCancelCall), [*locations rag engine config operations delete*](api::ProjectLocationRagEngineConfigOperationDeleteCall), [*locations rag engine config operations get*](api::ProjectLocationRagEngineConfigOperationGetCall), [*locations rag engine config operations list*](api::ProjectLocationRagEngineConfigOperationListCall), [*locations rag engine config operations wait*](api::ProjectLocationRagEngineConfigOperationWaitCall), [*locations reasoning engines a2a v1 card*](api::ProjectLocationReasoningEngineA2aV1CardCall), [*locations reasoning engines a2a v1 message send*](api::ProjectLocationReasoningEngineA2aV1MessageSendCall), [*locations reasoning engines a2a v1 message stream*](api::ProjectLocationReasoningEngineA2aV1MessageStreamCall), [*locations reasoning engines a2a v1 tasks a2a get reasoning engine*](api::ProjectLocationReasoningEngineA2aV1TaskA2aGetReasoningEngineCall), [*locations reasoning engines a2a v1 tasks cancel*](api::ProjectLocationReasoningEngineA2aV1TaskCancelCall), [*locations reasoning engines a2a v1 tasks push notification configs*](api::ProjectLocationReasoningEngineA2aV1TaskPushNotificationConfigCall), [*locations reasoning engines a2a v1 tasks push notification configs a2a get reasoning engine*](api::ProjectLocationReasoningEngineA2aV1TaskPushNotificationConfigA2aGetReasoningEngineCall), [*locations reasoning engines a2a v1 tasks subscribe*](api::ProjectLocationReasoningEngineA2aV1TaskSubscribeCall), [*locations reasoning engines create*](api::ProjectLocationReasoningEngineCreateCall), [*locations reasoning engines delete*](api::ProjectLocationReasoningEngineDeleteCall), [*locations reasoning engines examples operations cancel*](api::ProjectLocationReasoningEngineExampleOperationCancelCall), [*locations reasoning engines examples operations delete*](api::ProjectLocationReasoningEngineExampleOperationDeleteCall), [*locations reasoning engines examples operations get*](api::ProjectLocationReasoningEngineExampleOperationGetCall), [*locations reasoning engines examples operations wait*](api::ProjectLocationReasoningEngineExampleOperationWaitCall), [*locations reasoning engines get*](api::ProjectLocationReasoningEngineGetCall), [*locations reasoning engines list*](api::ProjectLocationReasoningEngineListCall), [*locations reasoning engines memories create*](api::ProjectLocationReasoningEngineMemoryCreateCall), [*locations reasoning engines memories delete*](api::ProjectLocationReasoningEngineMemoryDeleteCall), [*locations reasoning engines memories generate*](api::ProjectLocationReasoningEngineMemoryGenerateCall), [*locations reasoning engines memories get*](api::ProjectLocationReasoningEngineMemoryGetCall), [*locations reasoning engines memories list*](api::ProjectLocationReasoningEngineMemoryListCall), [*locations reasoning engines memories operations cancel*](api::ProjectLocationReasoningEngineMemoryOperationCancelCall), [*locations reasoning engines memories operations delete*](api::ProjectLocationReasoningEngineMemoryOperationDeleteCall), [*locations reasoning engines memories operations get*](api::ProjectLocationReasoningEngineMemoryOperationGetCall), [*locations reasoning engines memories operations list*](api::ProjectLocationReasoningEngineMemoryOperationListCall), [*locations reasoning engines memories operations wait*](api::ProjectLocationReasoningEngineMemoryOperationWaitCall), [*locations reasoning engines memories patch*](api::ProjectLocationReasoningEngineMemoryPatchCall), [*locations reasoning engines memories retrieve*](api::ProjectLocationReasoningEngineMemoryRetrieveCall), [*locations reasoning engines memories revisions get*](api::ProjectLocationReasoningEngineMemoryRevisionGetCall), [*locations reasoning engines memories revisions list*](api::ProjectLocationReasoningEngineMemoryRevisionListCall), [*locations reasoning engines memories rollback*](api::ProjectLocationReasoningEngineMemoryRollbackCall), [*locations reasoning engines operations cancel*](api::ProjectLocationReasoningEngineOperationCancelCall), [*locations reasoning engines operations delete*](api::ProjectLocationReasoningEngineOperationDeleteCall), [*locations reasoning engines operations get*](api::ProjectLocationReasoningEngineOperationGetCall), [*locations reasoning engines operations list*](api::ProjectLocationReasoningEngineOperationListCall), [*locations reasoning engines operations wait*](api::ProjectLocationReasoningEngineOperationWaitCall), [*locations reasoning engines patch*](api::ProjectLocationReasoningEnginePatchCall), [*locations reasoning engines query*](api::ProjectLocationReasoningEngineQueryCall), [*locations reasoning engines sandbox environments create*](api::ProjectLocationReasoningEngineSandboxEnvironmentCreateCall), [*locations reasoning engines sandbox environments delete*](api::ProjectLocationReasoningEngineSandboxEnvironmentDeleteCall), [*locations reasoning engines sandbox environments execute*](api::ProjectLocationReasoningEngineSandboxEnvironmentExecuteCall), [*locations reasoning engines sandbox environments get*](api::ProjectLocationReasoningEngineSandboxEnvironmentGetCall), [*locations reasoning engines sandbox environments list*](api::ProjectLocationReasoningEngineSandboxEnvironmentListCall), [*locations reasoning engines sandbox environments operations cancel*](api::ProjectLocationReasoningEngineSandboxEnvironmentOperationCancelCall), [*locations reasoning engines sandbox environments operations delete*](api::ProjectLocationReasoningEngineSandboxEnvironmentOperationDeleteCall), [*locations reasoning engines sandbox environments operations get*](api::ProjectLocationReasoningEngineSandboxEnvironmentOperationGetCall), [*locations reasoning engines sandbox environments operations list*](api::ProjectLocationReasoningEngineSandboxEnvironmentOperationListCall), [*locations reasoning engines sandbox environments operations wait*](api::ProjectLocationReasoningEngineSandboxEnvironmentOperationWaitCall), [*locations reasoning engines sessions append event*](api::ProjectLocationReasoningEngineSessionAppendEventCall), [*locations reasoning engines sessions create*](api::ProjectLocationReasoningEngineSessionCreateCall), [*locations reasoning engines sessions delete*](api::ProjectLocationReasoningEngineSessionDeleteCall), [*locations reasoning engines sessions events list*](api::ProjectLocationReasoningEngineSessionEventListCall), [*locations reasoning engines sessions get*](api::ProjectLocationReasoningEngineSessionGetCall), [*locations reasoning engines sessions list*](api::ProjectLocationReasoningEngineSessionListCall), [*locations reasoning engines sessions operations cancel*](api::ProjectLocationReasoningEngineSessionOperationCancelCall), [*locations reasoning engines sessions operations delete*](api::ProjectLocationReasoningEngineSessionOperationDeleteCall), [*locations reasoning engines sessions operations get*](api::ProjectLocationReasoningEngineSessionOperationGetCall), [*locations reasoning engines sessions operations list*](api::ProjectLocationReasoningEngineSessionOperationListCall), [*locations reasoning engines sessions operations wait*](api::ProjectLocationReasoningEngineSessionOperationWaitCall), [*locations reasoning engines sessions patch*](api::ProjectLocationReasoningEngineSessionPatchCall), [*locations reasoning engines stream query*](api::ProjectLocationReasoningEngineStreamQueryCall), [*locations recommend spec*](api::ProjectLocationRecommendSpecCall), [*locations retrieve contexts*](api::ProjectLocationRetrieveContextCall), [*locations schedules create*](api::ProjectLocationScheduleCreateCall), [*locations schedules delete*](api::ProjectLocationScheduleDeleteCall), [*locations schedules get*](api::ProjectLocationScheduleGetCall), [*locations schedules list*](api::ProjectLocationScheduleListCall), [*locations schedules operations cancel*](api::ProjectLocationScheduleOperationCancelCall), [*locations schedules operations delete*](api::ProjectLocationScheduleOperationDeleteCall), [*locations schedules operations get*](api::ProjectLocationScheduleOperationGetCall), [*locations schedules operations list*](api::ProjectLocationScheduleOperationListCall), [*locations schedules operations wait*](api::ProjectLocationScheduleOperationWaitCall), [*locations schedules patch*](api::ProjectLocationSchedulePatchCall), [*locations schedules pause*](api::ProjectLocationSchedulePauseCall), [*locations schedules resume*](api::ProjectLocationScheduleResumeCall), [*locations solvers operations delete*](api::ProjectLocationSolverOperationDeleteCall), [*locations solvers operations get*](api::ProjectLocationSolverOperationGetCall), [*locations solvers operations list*](api::ProjectLocationSolverOperationListCall), [*locations specialist pools create*](api::ProjectLocationSpecialistPoolCreateCall), [*locations specialist pools delete*](api::ProjectLocationSpecialistPoolDeleteCall), [*locations specialist pools get*](api::ProjectLocationSpecialistPoolGetCall), [*locations specialist pools list*](api::ProjectLocationSpecialistPoolListCall), [*locations specialist pools operations cancel*](api::ProjectLocationSpecialistPoolOperationCancelCall), [*locations specialist pools operations delete*](api::ProjectLocationSpecialistPoolOperationDeleteCall), [*locations specialist pools operations get*](api::ProjectLocationSpecialistPoolOperationGetCall), [*locations specialist pools operations list*](api::ProjectLocationSpecialistPoolOperationListCall), [*locations specialist pools operations wait*](api::ProjectLocationSpecialistPoolOperationWaitCall), [*locations specialist pools patch*](api::ProjectLocationSpecialistPoolPatchCall), [*locations studies create*](api::ProjectLocationStudyCreateCall), [*locations studies delete*](api::ProjectLocationStudyDeleteCall), [*locations studies get*](api::ProjectLocationStudyGetCall), [*locations studies list*](api::ProjectLocationStudyListCall), [*locations studies lookup*](api::ProjectLocationStudyLookupCall), [*locations studies operations cancel*](api::ProjectLocationStudyOperationCancelCall), [*locations studies operations delete*](api::ProjectLocationStudyOperationDeleteCall), [*locations studies operations get*](api::ProjectLocationStudyOperationGetCall), [*locations studies operations list*](api::ProjectLocationStudyOperationListCall), [*locations studies operations wait*](api::ProjectLocationStudyOperationWaitCall), [*locations studies trials add trial measurement*](api::ProjectLocationStudyTrialAddTrialMeasurementCall), [*locations studies trials check trial early stopping state*](api::ProjectLocationStudyTrialCheckTrialEarlyStoppingStateCall), [*locations studies trials complete*](api::ProjectLocationStudyTrialCompleteCall), [*locations studies trials create*](api::ProjectLocationStudyTrialCreateCall), [*locations studies trials delete*](api::ProjectLocationStudyTrialDeleteCall), [*locations studies trials get*](api::ProjectLocationStudyTrialGetCall), [*locations studies trials list*](api::ProjectLocationStudyTrialListCall), [*locations studies trials list optimal trials*](api::ProjectLocationStudyTrialListOptimalTrialCall), [*locations studies trials operations cancel*](api::ProjectLocationStudyTrialOperationCancelCall), [*locations studies trials operations delete*](api::ProjectLocationStudyTrialOperationDeleteCall), [*locations studies trials operations get*](api::ProjectLocationStudyTrialOperationGetCall), [*locations studies trials operations list*](api::ProjectLocationStudyTrialOperationListCall), [*locations studies trials operations wait*](api::ProjectLocationStudyTrialOperationWaitCall), [*locations studies trials stop*](api::ProjectLocationStudyTrialStopCall), [*locations studies trials suggest*](api::ProjectLocationStudyTrialSuggestCall), [*locations tensorboards batch read*](api::ProjectLocationTensorboardBatchReadCall), [*locations tensorboards create*](api::ProjectLocationTensorboardCreateCall), [*locations tensorboards delete*](api::ProjectLocationTensorboardDeleteCall), [*locations tensorboards experiments batch create*](api::ProjectLocationTensorboardExperimentBatchCreateCall), [*locations tensorboards experiments create*](api::ProjectLocationTensorboardExperimentCreateCall), [*locations tensorboards experiments delete*](api::ProjectLocationTensorboardExperimentDeleteCall), [*locations tensorboards experiments get*](api::ProjectLocationTensorboardExperimentGetCall), [*locations tensorboards experiments list*](api::ProjectLocationTensorboardExperimentListCall), [*locations tensorboards experiments operations cancel*](api::ProjectLocationTensorboardExperimentOperationCancelCall), [*locations tensorboards experiments operations delete*](api::ProjectLocationTensorboardExperimentOperationDeleteCall), [*locations tensorboards experiments operations get*](api::ProjectLocationTensorboardExperimentOperationGetCall), [*locations tensorboards experiments operations list*](api::ProjectLocationTensorboardExperimentOperationListCall), [*locations tensorboards experiments operations wait*](api::ProjectLocationTensorboardExperimentOperationWaitCall), [*locations tensorboards experiments patch*](api::ProjectLocationTensorboardExperimentPatchCall), [*locations tensorboards experiments runs batch create*](api::ProjectLocationTensorboardExperimentRunBatchCreateCall), [*locations tensorboards experiments runs create*](api::ProjectLocationTensorboardExperimentRunCreateCall), [*locations tensorboards experiments runs delete*](api::ProjectLocationTensorboardExperimentRunDeleteCall), [*locations tensorboards experiments runs get*](api::ProjectLocationTensorboardExperimentRunGetCall), [*locations tensorboards experiments runs list*](api::ProjectLocationTensorboardExperimentRunListCall), [*locations tensorboards experiments runs operations cancel*](api::ProjectLocationTensorboardExperimentRunOperationCancelCall), [*locations tensorboards experiments runs operations delete*](api::ProjectLocationTensorboardExperimentRunOperationDeleteCall), [*locations tensorboards experiments runs operations get*](api::ProjectLocationTensorboardExperimentRunOperationGetCall), [*locations tensorboards experiments runs operations list*](api::ProjectLocationTensorboardExperimentRunOperationListCall), [*locations tensorboards experiments runs operations wait*](api::ProjectLocationTensorboardExperimentRunOperationWaitCall), [*locations tensorboards experiments runs patch*](api::ProjectLocationTensorboardExperimentRunPatchCall), [*locations tensorboards experiments runs time series create*](api::ProjectLocationTensorboardExperimentRunTimeSeryCreateCall), [*locations tensorboards experiments runs time series delete*](api::ProjectLocationTensorboardExperimentRunTimeSeryDeleteCall), [*locations tensorboards experiments runs time series export tensorboard time series*](api::ProjectLocationTensorboardExperimentRunTimeSeryExportTensorboardTimeSeryCall), [*locations tensorboards experiments runs time series get*](api::ProjectLocationTensorboardExperimentRunTimeSeryGetCall), [*locations tensorboards experiments runs time series list*](api::ProjectLocationTensorboardExperimentRunTimeSeryListCall), [*locations tensorboards experiments runs time series operations cancel*](api::ProjectLocationTensorboardExperimentRunTimeSeryOperationCancelCall), [*locations tensorboards experiments runs time series operations delete*](api::ProjectLocationTensorboardExperimentRunTimeSeryOperationDeleteCall), [*locations tensorboards experiments runs time series operations get*](api::ProjectLocationTensorboardExperimentRunTimeSeryOperationGetCall), [*locations tensorboards experiments runs time series operations list*](api::ProjectLocationTensorboardExperimentRunTimeSeryOperationListCall), [*locations tensorboards experiments runs time series operations wait*](api::ProjectLocationTensorboardExperimentRunTimeSeryOperationWaitCall), [*locations tensorboards experiments runs time series patch*](api::ProjectLocationTensorboardExperimentRunTimeSeryPatchCall), [*locations tensorboards experiments runs time series read*](api::ProjectLocationTensorboardExperimentRunTimeSeryReadCall), [*locations tensorboards experiments runs time series read blob data*](api::ProjectLocationTensorboardExperimentRunTimeSeryReadBlobDataCall), [*locations tensorboards experiments runs write*](api::ProjectLocationTensorboardExperimentRunWriteCall), [*locations tensorboards experiments write*](api::ProjectLocationTensorboardExperimentWriteCall), [*locations tensorboards get*](api::ProjectLocationTensorboardGetCall), [*locations tensorboards list*](api::ProjectLocationTensorboardListCall), [*locations tensorboards operations cancel*](api::ProjectLocationTensorboardOperationCancelCall), [*locations tensorboards operations delete*](api::ProjectLocationTensorboardOperationDeleteCall), [*locations tensorboards operations get*](api::ProjectLocationTensorboardOperationGetCall), [*locations tensorboards operations list*](api::ProjectLocationTensorboardOperationListCall), [*locations tensorboards operations wait*](api::ProjectLocationTensorboardOperationWaitCall), [*locations tensorboards patch*](api::ProjectLocationTensorboardPatchCall), [*locations tensorboards read size*](api::ProjectLocationTensorboardReadSizeCall), [*locations tensorboards read usage*](api::ProjectLocationTensorboardReadUsageCall), [*locations training pipelines cancel*](api::ProjectLocationTrainingPipelineCancelCall), [*locations training pipelines create*](api::ProjectLocationTrainingPipelineCreateCall), [*locations training pipelines delete*](api::ProjectLocationTrainingPipelineDeleteCall), [*locations training pipelines get*](api::ProjectLocationTrainingPipelineGetCall), [*locations training pipelines list*](api::ProjectLocationTrainingPipelineListCall), [*locations training pipelines operations cancel*](api::ProjectLocationTrainingPipelineOperationCancelCall), [*locations training pipelines operations delete*](api::ProjectLocationTrainingPipelineOperationDeleteCall), [*locations training pipelines operations get*](api::ProjectLocationTrainingPipelineOperationGetCall), [*locations training pipelines operations list*](api::ProjectLocationTrainingPipelineOperationListCall), [*locations training pipelines operations wait*](api::ProjectLocationTrainingPipelineOperationWaitCall), [*locations tuning jobs cancel*](api::ProjectLocationTuningJobCancelCall), [*locations tuning jobs create*](api::ProjectLocationTuningJobCreateCall), [*locations tuning jobs get*](api::ProjectLocationTuningJobGetCall), [*locations tuning jobs list*](api::ProjectLocationTuningJobListCall), [*locations tuning jobs operations delete*](api::ProjectLocationTuningJobOperationDeleteCall), [*locations tuning jobs optimize prompt*](api::ProjectLocationTuningJobOptimizePromptCall), [*locations tuning jobs rebase tuned model*](api::ProjectLocationTuningJobRebaseTunedModelCall), [*locations update rag engine config*](api::ProjectLocationUpdateRagEngineConfigCall), [*model garden eula accept*](api::ProjectModelGardenEulaAcceptCall), [*model garden eula check*](api::ProjectModelGardenEulaCheckCall), [*publishers models enable model*](api::ProjectPublisherModelEnableModelCall), [*set publisher model config*](api::ProjectSetPublisherModelConfigCall) and [*update cache config*](api::ProjectUpdateCacheConfigCall)
//! * publishers
//!  * [*models compute tokens*](api::PublisherModelComputeTokenCall), [*models count tokens*](api::PublisherModelCountTokenCall), [*models fetch predict operation*](api::PublisherModelFetchPredictOperationCall), [*models generate content*](api::PublisherModelGenerateContentCall), [*models get*](api::PublisherModelGetCall), [*models list*](api::PublisherModelListCall), [*models predict*](api::PublisherModelPredictCall), [*models predict long running*](api::PublisherModelPredictLongRunningCall) and [*models stream generate content*](api::PublisherModelStreamGenerateContentCall)
//! * rag corpora
//!  * [*operations cancel*](api::RagCorporaOperationCancelCall), [*operations delete*](api::RagCorporaOperationDeleteCall), [*operations get*](api::RagCorporaOperationGetCall), [*operations list*](api::RagCorporaOperationListCall), [*operations wait*](api::RagCorporaOperationWaitCall), [*rag files operations cancel*](api::RagCorporaRagFileOperationCancelCall), [*rag files operations delete*](api::RagCorporaRagFileOperationDeleteCall), [*rag files operations get*](api::RagCorporaRagFileOperationGetCall), [*rag files operations list*](api::RagCorporaRagFileOperationListCall) and [*rag files operations wait*](api::RagCorporaRagFileOperationWaitCall)
//! * rag engine config
//!  * [*operations cancel*](api::RagEngineConfigOperationCancelCall), [*operations delete*](api::RagEngineConfigOperationDeleteCall), [*operations get*](api::RagEngineConfigOperationGetCall), [*operations list*](api::RagEngineConfigOperationListCall) and [*operations wait*](api::RagEngineConfigOperationWaitCall)
//! * reasoning engines
//!  * [*a2a v1 card*](api::ReasoningEngineA2aV1CardCall), [*a2a v1 message send*](api::ReasoningEngineA2aV1MessageSendCall), [*a2a v1 message stream*](api::ReasoningEngineA2aV1MessageStreamCall), [*a2a v1 tasks a2a get reasoning engine*](api::ReasoningEngineA2aV1TaskA2aGetReasoningEngineCall), [*a2a v1 tasks cancel*](api::ReasoningEngineA2aV1TaskCancelCall), [*a2a v1 tasks push notification configs*](api::ReasoningEngineA2aV1TaskPushNotificationConfigCall), [*a2a v1 tasks push notification configs a2a get reasoning engine*](api::ReasoningEngineA2aV1TaskPushNotificationConfigA2aGetReasoningEngineCall), [*a2a v1 tasks subscribe*](api::ReasoningEngineA2aV1TaskSubscribeCall), [*create*](api::ReasoningEngineCreateCall), [*delete*](api::ReasoningEngineDeleteCall), [*examples operations cancel*](api::ReasoningEngineExampleOperationCancelCall), [*examples operations delete*](api::ReasoningEngineExampleOperationDeleteCall), [*examples operations get*](api::ReasoningEngineExampleOperationGetCall), [*examples operations wait*](api::ReasoningEngineExampleOperationWaitCall), [*get*](api::ReasoningEngineGetCall), [*list*](api::ReasoningEngineListCall), [*memories create*](api::ReasoningEngineMemoryCreateCall), [*memories delete*](api::ReasoningEngineMemoryDeleteCall), [*memories generate*](api::ReasoningEngineMemoryGenerateCall), [*memories get*](api::ReasoningEngineMemoryGetCall), [*memories list*](api::ReasoningEngineMemoryListCall), [*memories operations cancel*](api::ReasoningEngineMemoryOperationCancelCall), [*memories operations delete*](api::ReasoningEngineMemoryOperationDeleteCall), [*memories operations get*](api::ReasoningEngineMemoryOperationGetCall), [*memories operations list*](api::ReasoningEngineMemoryOperationListCall), [*memories operations wait*](api::ReasoningEngineMemoryOperationWaitCall), [*memories patch*](api::ReasoningEngineMemoryPatchCall), [*memories retrieve*](api::ReasoningEngineMemoryRetrieveCall), [*memories revisions get*](api::ReasoningEngineMemoryRevisionGetCall), [*memories revisions list*](api::ReasoningEngineMemoryRevisionListCall), [*memories rollback*](api::ReasoningEngineMemoryRollbackCall), [*operations cancel*](api::ReasoningEngineOperationCancelCall), [*operations delete*](api::ReasoningEngineOperationDeleteCall), [*operations get*](api::ReasoningEngineOperationGetCall), [*operations list*](api::ReasoningEngineOperationListCall), [*operations wait*](api::ReasoningEngineOperationWaitCall), [*patch*](api::ReasoningEnginePatchCall), [*query*](api::ReasoningEngineQueryCall), [*sandbox environments operations cancel*](api::ReasoningEngineSandboxEnvironmentOperationCancelCall), [*sandbox environments operations delete*](api::ReasoningEngineSandboxEnvironmentOperationDeleteCall), [*sandbox environments operations get*](api::ReasoningEngineSandboxEnvironmentOperationGetCall), [*sandbox environments operations list*](api::ReasoningEngineSandboxEnvironmentOperationListCall), [*sandbox environments operations wait*](api::ReasoningEngineSandboxEnvironmentOperationWaitCall), [*sessions append event*](api::ReasoningEngineSessionAppendEventCall), [*sessions create*](api::ReasoningEngineSessionCreateCall), [*sessions delete*](api::ReasoningEngineSessionDeleteCall), [*sessions events list*](api::ReasoningEngineSessionEventListCall), [*sessions get*](api::ReasoningEngineSessionGetCall), [*sessions list*](api::ReasoningEngineSessionListCall), [*sessions operations cancel*](api::ReasoningEngineSessionOperationCancelCall), [*sessions operations delete*](api::ReasoningEngineSessionOperationDeleteCall), [*sessions operations get*](api::ReasoningEngineSessionOperationGetCall), [*sessions operations list*](api::ReasoningEngineSessionOperationListCall), [*sessions operations wait*](api::ReasoningEngineSessionOperationWaitCall), [*sessions patch*](api::ReasoningEngineSessionPatchCall) and [*stream query*](api::ReasoningEngineStreamQueryCall)
//! * schedules
//!  * [*operations cancel*](api::ScheduleOperationCancelCall), [*operations delete*](api::ScheduleOperationDeleteCall), [*operations get*](api::ScheduleOperationGetCall), [*operations list*](api::ScheduleOperationListCall) and [*operations wait*](api::ScheduleOperationWaitCall)
//! * solvers
//!  * [*operations delete*](api::SolverOperationDeleteCall), [*operations get*](api::SolverOperationGetCall) and [*operations list*](api::SolverOperationListCall)
//! * specialist pools
//!  * [*operations cancel*](api::SpecialistPoolOperationCancelCall), [*operations delete*](api::SpecialistPoolOperationDeleteCall), [*operations get*](api::SpecialistPoolOperationGetCall), [*operations list*](api::SpecialistPoolOperationListCall) and [*operations wait*](api::SpecialistPoolOperationWaitCall)
//! * studies
//!  * [*operations cancel*](api::StudyOperationCancelCall), [*operations delete*](api::StudyOperationDeleteCall), [*operations get*](api::StudyOperationGetCall), [*operations list*](api::StudyOperationListCall), [*operations wait*](api::StudyOperationWaitCall), [*trials operations cancel*](api::StudyTrialOperationCancelCall), [*trials operations delete*](api::StudyTrialOperationDeleteCall), [*trials operations get*](api::StudyTrialOperationGetCall), [*trials operations list*](api::StudyTrialOperationListCall) and [*trials operations wait*](api::StudyTrialOperationWaitCall)
//! * tensorboards
//!  * [*experiments operations cancel*](api::TensorboardExperimentOperationCancelCall), [*experiments operations delete*](api::TensorboardExperimentOperationDeleteCall), [*experiments operations get*](api::TensorboardExperimentOperationGetCall), [*experiments operations list*](api::TensorboardExperimentOperationListCall), [*experiments operations wait*](api::TensorboardExperimentOperationWaitCall), [*experiments runs operations cancel*](api::TensorboardExperimentRunOperationCancelCall), [*experiments runs operations delete*](api::TensorboardExperimentRunOperationDeleteCall), [*experiments runs operations get*](api::TensorboardExperimentRunOperationGetCall), [*experiments runs operations list*](api::TensorboardExperimentRunOperationListCall), [*experiments runs operations wait*](api::TensorboardExperimentRunOperationWaitCall), [*experiments runs time series operations cancel*](api::TensorboardExperimentRunTimeSeryOperationCancelCall), [*experiments runs time series operations delete*](api::TensorboardExperimentRunTimeSeryOperationDeleteCall), [*experiments runs time series operations get*](api::TensorboardExperimentRunTimeSeryOperationGetCall), [*experiments runs time series operations list*](api::TensorboardExperimentRunTimeSeryOperationListCall), [*experiments runs time series operations wait*](api::TensorboardExperimentRunTimeSeryOperationWaitCall), [*operations cancel*](api::TensorboardOperationCancelCall), [*operations delete*](api::TensorboardOperationDeleteCall), [*operations get*](api::TensorboardOperationGetCall), [*operations list*](api::TensorboardOperationListCall) and [*operations wait*](api::TensorboardOperationWaitCall)
//! * training pipelines
//!  * [*operations cancel*](api::TrainingPipelineOperationCancelCall), [*operations delete*](api::TrainingPipelineOperationDeleteCall), [*operations get*](api::TrainingPipelineOperationGetCall), [*operations list*](api::TrainingPipelineOperationListCall) and [*operations wait*](api::TrainingPipelineOperationWaitCall)
//! * tuning jobs
//!  * [*operations delete*](api::TuningJobOperationDeleteCall)
//!
//!
//! Upload supported by ...
//!
//! * [*upload media*](api::MediaUploadCall)
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
//! let r = hub.agents().operations_get(...).doit().await
//! let r = hub.agents().operations_wait(...).doit().await
//! let r = hub.apps().operations_get(...).doit().await
//! let r = hub.apps().operations_wait(...).doit().await
//! let r = hub.custom_jobs().operations_get(...).doit().await
//! let r = hub.custom_jobs().operations_wait(...).doit().await
//! let r = hub.data_labeling_jobs().operations_get(...).doit().await
//! let r = hub.data_labeling_jobs().operations_wait(...).doit().await
//! let r = hub.datasets().annotation_specs_operations_get(...).doit().await
//! let r = hub.datasets().annotation_specs_operations_wait(...).doit().await
//! let r = hub.datasets().data_items_annotations_operations_get(...).doit().await
//! let r = hub.datasets().data_items_annotations_operations_wait(...).doit().await
//! let r = hub.datasets().data_items_operations_get(...).doit().await
//! let r = hub.datasets().data_items_operations_wait(...).doit().await
//! let r = hub.datasets().dataset_versions_create(...).doit().await
//! let r = hub.datasets().dataset_versions_delete(...).doit().await
//! let r = hub.datasets().dataset_versions_restore(...).doit().await
//! let r = hub.datasets().operations_get(...).doit().await
//! let r = hub.datasets().operations_wait(...).doit().await
//! let r = hub.datasets().saved_queries_operations_get(...).doit().await
//! let r = hub.datasets().saved_queries_operations_wait(...).doit().await
//! let r = hub.datasets().create(...).doit().await
//! let r = hub.datasets().delete(...).doit().await
//! let r = hub.deployment_resource_pools().operations_get(...).doit().await
//! let r = hub.deployment_resource_pools().operations_wait(...).doit().await
//! let r = hub.edge_devices().operations_get(...).doit().await
//! let r = hub.edge_devices().operations_wait(...).doit().await
//! let r = hub.endpoints().operations_get(...).doit().await
//! let r = hub.endpoints().operations_wait(...).doit().await
//! let r = hub.endpoints().fetch_predict_operation(...).doit().await
//! let r = hub.endpoints().predict_long_running(...).doit().await
//! let r = hub.evaluation_items().operations_get(...).doit().await
//! let r = hub.evaluation_items().operations_wait(...).doit().await
//! let r = hub.evaluation_runs().operations_get(...).doit().await
//! let r = hub.evaluation_runs().operations_wait(...).doit().await
//! let r = hub.evaluation_sets().operations_get(...).doit().await
//! let r = hub.evaluation_sets().operations_wait(...).doit().await
//! let r = hub.evaluation_tasks().operations_get(...).doit().await
//! let r = hub.evaluation_tasks().operations_wait(...).doit().await
//! let r = hub.example_stores().operations_get(...).doit().await
//! let r = hub.example_stores().operations_wait(...).doit().await
//! let r = hub.extension_controllers().operations_get(...).doit().await
//! let r = hub.extension_controllers().operations_wait(...).doit().await
//! let r = hub.extensions().operations_get(...).doit().await
//! let r = hub.extensions().operations_wait(...).doit().await
//! let r = hub.feature_groups().feature_monitors_operations_get(...).doit().await
//! let r = hub.feature_groups().feature_monitors_operations_wait(...).doit().await
//! let r = hub.feature_groups().features_operations_get(...).doit().await
//! let r = hub.feature_groups().features_operations_wait(...).doit().await
//! let r = hub.feature_groups().operations_get(...).doit().await
//! let r = hub.feature_groups().operations_wait(...).doit().await
//! let r = hub.feature_online_stores().feature_views_operations_get(...).doit().await
//! let r = hub.feature_online_stores().feature_views_operations_wait(...).doit().await
//! let r = hub.feature_online_stores().operations_get(...).doit().await
//! let r = hub.feature_online_stores().operations_wait(...).doit().await
//! let r = hub.featurestores().entity_types_features_operations_get(...).doit().await
//! let r = hub.featurestores().entity_types_features_operations_wait(...).doit().await
//! let r = hub.featurestores().entity_types_operations_get(...).doit().await
//! let r = hub.featurestores().entity_types_operations_wait(...).doit().await
//! let r = hub.featurestores().operations_get(...).doit().await
//! let r = hub.featurestores().operations_wait(...).doit().await
//! let r = hub.hyperparameter_tuning_jobs().operations_get(...).doit().await
//! let r = hub.hyperparameter_tuning_jobs().operations_wait(...).doit().await
//! let r = hub.index_endpoints().operations_get(...).doit().await
//! let r = hub.index_endpoints().operations_wait(...).doit().await
//! let r = hub.indexes().operations_get(...).doit().await
//! let r = hub.indexes().operations_wait(...).doit().await
//! let r = hub.metadata_stores().artifacts_operations_get(...).doit().await
//! let r = hub.metadata_stores().artifacts_operations_wait(...).doit().await
//! let r = hub.metadata_stores().contexts_operations_get(...).doit().await
//! let r = hub.metadata_stores().contexts_operations_wait(...).doit().await
//! let r = hub.metadata_stores().executions_operations_get(...).doit().await
//! let r = hub.metadata_stores().executions_operations_wait(...).doit().await
//! let r = hub.metadata_stores().operations_get(...).doit().await
//! let r = hub.metadata_stores().operations_wait(...).doit().await
//! let r = hub.migratable_resources().operations_get(...).doit().await
//! let r = hub.migratable_resources().operations_wait(...).doit().await
//! let r = hub.model_deployment_monitoring_jobs().operations_get(...).doit().await
//! let r = hub.model_deployment_monitoring_jobs().operations_wait(...).doit().await
//! let r = hub.model_monitors().operations_get(...).doit().await
//! let r = hub.model_monitors().operations_wait(...).doit().await
//! let r = hub.models().evaluations_operations_get(...).doit().await
//! let r = hub.models().evaluations_operations_wait(...).doit().await
//! let r = hub.models().operations_get(...).doit().await
//! let r = hub.models().operations_wait(...).doit().await
//! let r = hub.notebook_execution_jobs().operations_get(...).doit().await
//! let r = hub.notebook_execution_jobs().operations_wait(...).doit().await
//! let r = hub.notebook_runtime_templates().operations_get(...).doit().await
//! let r = hub.notebook_runtime_templates().operations_wait(...).doit().await
//! let r = hub.notebook_runtimes().operations_get(...).doit().await
//! let r = hub.notebook_runtimes().operations_wait(...).doit().await
//! let r = hub.operations().get(...).doit().await
//! let r = hub.operations().wait(...).doit().await
//! let r = hub.persistent_resources().operations_get(...).doit().await
//! let r = hub.persistent_resources().operations_wait(...).doit().await
//! let r = hub.pipeline_jobs().operations_get(...).doit().await
//! let r = hub.pipeline_jobs().operations_wait(...).doit().await
//! let r = hub.projects().locations_agents_operations_get(...).doit().await
//! let r = hub.projects().locations_agents_operations_wait(...).doit().await
//! let r = hub.projects().locations_apps_operations_get(...).doit().await
//! let r = hub.projects().locations_apps_operations_wait(...).doit().await
//! let r = hub.projects().locations_batch_prediction_jobs_delete(...).doit().await
//! let r = hub.projects().locations_custom_jobs_operations_get(...).doit().await
//! let r = hub.projects().locations_custom_jobs_operations_wait(...).doit().await
//! let r = hub.projects().locations_custom_jobs_delete(...).doit().await
//! let r = hub.projects().locations_data_labeling_jobs_operations_get(...).doit().await
//! let r = hub.projects().locations_data_labeling_jobs_operations_wait(...).doit().await
//! let r = hub.projects().locations_data_labeling_jobs_delete(...).doit().await
//! let r = hub.projects().locations_datasets_annotation_specs_operations_get(...).doit().await
//! let r = hub.projects().locations_datasets_annotation_specs_operations_wait(...).doit().await
//! let r = hub.projects().locations_datasets_data_items_annotations_operations_get(...).doit().await
//! let r = hub.projects().locations_datasets_data_items_annotations_operations_wait(...).doit().await
//! let r = hub.projects().locations_datasets_data_items_operations_get(...).doit().await
//! let r = hub.projects().locations_datasets_data_items_operations_wait(...).doit().await
//! let r = hub.projects().locations_datasets_dataset_versions_create(...).doit().await
//! let r = hub.projects().locations_datasets_dataset_versions_delete(...).doit().await
//! let r = hub.projects().locations_datasets_dataset_versions_restore(...).doit().await
//! let r = hub.projects().locations_datasets_operations_get(...).doit().await
//! let r = hub.projects().locations_datasets_operations_wait(...).doit().await
//! let r = hub.projects().locations_datasets_saved_queries_operations_get(...).doit().await
//! let r = hub.projects().locations_datasets_saved_queries_operations_wait(...).doit().await
//! let r = hub.projects().locations_datasets_saved_queries_delete(...).doit().await
//! let r = hub.projects().locations_datasets_assemble(...).doit().await
//! let r = hub.projects().locations_datasets_assess(...).doit().await
//! let r = hub.projects().locations_datasets_create(...).doit().await
//! let r = hub.projects().locations_datasets_delete(...).doit().await
//! let r = hub.projects().locations_datasets_export(...).doit().await
//! let r = hub.projects().locations_datasets_import(...).doit().await
//! let r = hub.projects().locations_deployment_resource_pools_operations_get(...).doit().await
//! let r = hub.projects().locations_deployment_resource_pools_operations_wait(...).doit().await
//! let r = hub.projects().locations_deployment_resource_pools_create(...).doit().await
//! let r = hub.projects().locations_deployment_resource_pools_delete(...).doit().await
//! let r = hub.projects().locations_deployment_resource_pools_patch(...).doit().await
//! let r = hub.projects().locations_edge_devices_operations_get(...).doit().await
//! let r = hub.projects().locations_edge_devices_operations_wait(...).doit().await
//! let r = hub.projects().locations_endpoints_operations_get(...).doit().await
//! let r = hub.projects().locations_endpoints_operations_wait(...).doit().await
//! let r = hub.projects().locations_endpoints_create(...).doit().await
//! let r = hub.projects().locations_endpoints_delete(...).doit().await
//! let r = hub.projects().locations_endpoints_deploy_model(...).doit().await
//! let r = hub.projects().locations_endpoints_fetch_predict_operation(...).doit().await
//! let r = hub.projects().locations_endpoints_mutate_deployed_model(...).doit().await
//! let r = hub.projects().locations_endpoints_predict_long_running(...).doit().await
//! let r = hub.projects().locations_endpoints_undeploy_model(...).doit().await
//! let r = hub.projects().locations_endpoints_update(...).doit().await
//! let r = hub.projects().locations_evaluation_items_operations_get(...).doit().await
//! let r = hub.projects().locations_evaluation_items_operations_wait(...).doit().await
//! let r = hub.projects().locations_evaluation_items_delete(...).doit().await
//! let r = hub.projects().locations_evaluation_runs_operations_get(...).doit().await
//! let r = hub.projects().locations_evaluation_runs_operations_wait(...).doit().await
//! let r = hub.projects().locations_evaluation_runs_delete(...).doit().await
//! let r = hub.projects().locations_evaluation_sets_operations_get(...).doit().await
//! let r = hub.projects().locations_evaluation_sets_operations_wait(...).doit().await
//! let r = hub.projects().locations_evaluation_sets_delete(...).doit().await
//! let r = hub.projects().locations_evaluation_tasks_operations_get(...).doit().await
//! let r = hub.projects().locations_evaluation_tasks_operations_wait(...).doit().await
//! let r = hub.projects().locations_example_stores_operations_get(...).doit().await
//! let r = hub.projects().locations_example_stores_operations_wait(...).doit().await
//! let r = hub.projects().locations_example_stores_create(...).doit().await
//! let r = hub.projects().locations_example_stores_delete(...).doit().await
//! let r = hub.projects().locations_example_stores_patch(...).doit().await
//! let r = hub.projects().locations_extension_controllers_operations_get(...).doit().await
//! let r = hub.projects().locations_extension_controllers_operations_wait(...).doit().await
//! let r = hub.projects().locations_extensions_operations_get(...).doit().await
//! let r = hub.projects().locations_extensions_operations_wait(...).doit().await
//! let r = hub.projects().locations_extensions_delete(...).doit().await
//! let r = hub.projects().locations_extensions_import(...).doit().await
//! let r = hub.projects().locations_feature_groups_feature_monitors_operations_get(...).doit().await
//! let r = hub.projects().locations_feature_groups_feature_monitors_operations_wait(...).doit().await
//! let r = hub.projects().locations_feature_groups_feature_monitors_create(...).doit().await
//! let r = hub.projects().locations_feature_groups_feature_monitors_delete(...).doit().await
//! let r = hub.projects().locations_feature_groups_feature_monitors_patch(...).doit().await
//! let r = hub.projects().locations_feature_groups_features_operations_get(...).doit().await
//! let r = hub.projects().locations_feature_groups_features_operations_wait(...).doit().await
//! let r = hub.projects().locations_feature_groups_features_batch_create(...).doit().await
//! let r = hub.projects().locations_feature_groups_features_create(...).doit().await
//! let r = hub.projects().locations_feature_groups_features_delete(...).doit().await
//! let r = hub.projects().locations_feature_groups_features_patch(...).doit().await
//! let r = hub.projects().locations_feature_groups_operations_get(...).doit().await
//! let r = hub.projects().locations_feature_groups_operations_wait(...).doit().await
//! let r = hub.projects().locations_feature_groups_create(...).doit().await
//! let r = hub.projects().locations_feature_groups_delete(...).doit().await
//! let r = hub.projects().locations_feature_groups_patch(...).doit().await
//! let r = hub.projects().locations_feature_online_stores_feature_views_operations_get(...).doit().await
//! let r = hub.projects().locations_feature_online_stores_feature_views_operations_wait(...).doit().await
//! let r = hub.projects().locations_feature_online_stores_feature_views_create(...).doit().await
//! let r = hub.projects().locations_feature_online_stores_feature_views_delete(...).doit().await
//! let r = hub.projects().locations_feature_online_stores_feature_views_patch(...).doit().await
//! let r = hub.projects().locations_feature_online_stores_operations_get(...).doit().await
//! let r = hub.projects().locations_feature_online_stores_operations_wait(...).doit().await
//! let r = hub.projects().locations_feature_online_stores_create(...).doit().await
//! let r = hub.projects().locations_feature_online_stores_delete(...).doit().await
//! let r = hub.projects().locations_feature_online_stores_patch(...).doit().await
//! let r = hub.projects().locations_featurestores_entity_types_features_operations_get(...).doit().await
//! let r = hub.projects().locations_featurestores_entity_types_features_operations_wait(...).doit().await
//! let r = hub.projects().locations_featurestores_entity_types_features_batch_create(...).doit().await
//! let r = hub.projects().locations_featurestores_entity_types_features_create(...).doit().await
//! let r = hub.projects().locations_featurestores_entity_types_features_delete(...).doit().await
//! let r = hub.projects().locations_featurestores_entity_types_operations_get(...).doit().await
//! let r = hub.projects().locations_featurestores_entity_types_operations_wait(...).doit().await
//! let r = hub.projects().locations_featurestores_entity_types_create(...).doit().await
//! let r = hub.projects().locations_featurestores_entity_types_delete(...).doit().await
//! let r = hub.projects().locations_featurestores_entity_types_delete_feature_values(...).doit().await
//! let r = hub.projects().locations_featurestores_entity_types_export_feature_values(...).doit().await
//! let r = hub.projects().locations_featurestores_entity_types_import_feature_values(...).doit().await
//! let r = hub.projects().locations_featurestores_operations_get(...).doit().await
//! let r = hub.projects().locations_featurestores_operations_wait(...).doit().await
//! let r = hub.projects().locations_featurestores_batch_read_feature_values(...).doit().await
//! let r = hub.projects().locations_featurestores_create(...).doit().await
//! let r = hub.projects().locations_featurestores_delete(...).doit().await
//! let r = hub.projects().locations_featurestores_patch(...).doit().await
//! let r = hub.projects().locations_hyperparameter_tuning_jobs_operations_get(...).doit().await
//! let r = hub.projects().locations_hyperparameter_tuning_jobs_operations_wait(...).doit().await
//! let r = hub.projects().locations_hyperparameter_tuning_jobs_delete(...).doit().await
//! let r = hub.projects().locations_index_endpoints_operations_get(...).doit().await
//! let r = hub.projects().locations_index_endpoints_operations_wait(...).doit().await
//! let r = hub.projects().locations_index_endpoints_create(...).doit().await
//! let r = hub.projects().locations_index_endpoints_delete(...).doit().await
//! let r = hub.projects().locations_index_endpoints_deploy_index(...).doit().await
//! let r = hub.projects().locations_index_endpoints_mutate_deployed_index(...).doit().await
//! let r = hub.projects().locations_index_endpoints_undeploy_index(...).doit().await
//! let r = hub.projects().locations_indexes_operations_get(...).doit().await
//! let r = hub.projects().locations_indexes_operations_wait(...).doit().await
//! let r = hub.projects().locations_indexes_create(...).doit().await
//! let r = hub.projects().locations_indexes_delete(...).doit().await
//! let r = hub.projects().locations_indexes_import(...).doit().await
//! let r = hub.projects().locations_indexes_patch(...).doit().await
//! let r = hub.projects().locations_metadata_stores_artifacts_operations_get(...).doit().await
//! let r = hub.projects().locations_metadata_stores_artifacts_operations_wait(...).doit().await
//! let r = hub.projects().locations_metadata_stores_artifacts_delete(...).doit().await
//! let r = hub.projects().locations_metadata_stores_artifacts_purge(...).doit().await
//! let r = hub.projects().locations_metadata_stores_contexts_operations_get(...).doit().await
//! let r = hub.projects().locations_metadata_stores_contexts_operations_wait(...).doit().await
//! let r = hub.projects().locations_metadata_stores_contexts_delete(...).doit().await
//! let r = hub.projects().locations_metadata_stores_contexts_purge(...).doit().await
//! let r = hub.projects().locations_metadata_stores_executions_operations_get(...).doit().await
//! let r = hub.projects().locations_metadata_stores_executions_operations_wait(...).doit().await
//! let r = hub.projects().locations_metadata_stores_executions_delete(...).doit().await
//! let r = hub.projects().locations_metadata_stores_executions_purge(...).doit().await
//! let r = hub.projects().locations_metadata_stores_operations_get(...).doit().await
//! let r = hub.projects().locations_metadata_stores_operations_wait(...).doit().await
//! let r = hub.projects().locations_metadata_stores_create(...).doit().await
//! let r = hub.projects().locations_metadata_stores_delete(...).doit().await
//! let r = hub.projects().locations_migratable_resources_operations_get(...).doit().await
//! let r = hub.projects().locations_migratable_resources_operations_wait(...).doit().await
//! let r = hub.projects().locations_migratable_resources_batch_migrate(...).doit().await
//! let r = hub.projects().locations_model_deployment_monitoring_jobs_operations_get(...).doit().await
//! let r = hub.projects().locations_model_deployment_monitoring_jobs_operations_wait(...).doit().await
//! let r = hub.projects().locations_model_deployment_monitoring_jobs_delete(...).doit().await
//! let r = hub.projects().locations_model_deployment_monitoring_jobs_patch(...).doit().await
//! let r = hub.projects().locations_model_monitors_model_monitoring_jobs_delete(...).doit().await
//! let r = hub.projects().locations_model_monitors_operations_get(...).doit().await
//! let r = hub.projects().locations_model_monitors_operations_wait(...).doit().await
//! let r = hub.projects().locations_model_monitors_create(...).doit().await
//! let r = hub.projects().locations_model_monitors_delete(...).doit().await
//! let r = hub.projects().locations_model_monitors_patch(...).doit().await
//! let r = hub.projects().locations_models_evaluations_operations_get(...).doit().await
//! let r = hub.projects().locations_models_evaluations_operations_wait(...).doit().await
//! let r = hub.projects().locations_models_operations_get(...).doit().await
//! let r = hub.projects().locations_models_operations_wait(...).doit().await
//! let r = hub.projects().locations_models_copy(...).doit().await
//! let r = hub.projects().locations_models_delete(...).doit().await
//! let r = hub.projects().locations_models_delete_version(...).doit().await
//! let r = hub.projects().locations_models_export(...).doit().await
//! let r = hub.projects().locations_models_update_explanation_dataset(...).doit().await
//! let r = hub.projects().locations_models_upload(...).doit().await
//! let r = hub.projects().locations_nas_jobs_delete(...).doit().await
//! let r = hub.projects().locations_notebook_execution_jobs_operations_get(...).doit().await
//! let r = hub.projects().locations_notebook_execution_jobs_operations_wait(...).doit().await
//! let r = hub.projects().locations_notebook_execution_jobs_create(...).doit().await
//! let r = hub.projects().locations_notebook_execution_jobs_delete(...).doit().await
//! let r = hub.projects().locations_notebook_runtime_templates_operations_get(...).doit().await
//! let r = hub.projects().locations_notebook_runtime_templates_operations_wait(...).doit().await
//! let r = hub.projects().locations_notebook_runtime_templates_create(...).doit().await
//! let r = hub.projects().locations_notebook_runtime_templates_delete(...).doit().await
//! let r = hub.projects().locations_notebook_runtimes_operations_get(...).doit().await
//! let r = hub.projects().locations_notebook_runtimes_operations_wait(...).doit().await
//! let r = hub.projects().locations_notebook_runtimes_assign(...).doit().await
//! let r = hub.projects().locations_notebook_runtimes_delete(...).doit().await
//! let r = hub.projects().locations_notebook_runtimes_start(...).doit().await
//! let r = hub.projects().locations_notebook_runtimes_stop(...).doit().await
//! let r = hub.projects().locations_notebook_runtimes_upgrade(...).doit().await
//! let r = hub.projects().locations_operations_get(...).doit().await
//! let r = hub.projects().locations_operations_wait(...).doit().await
//! let r = hub.projects().locations_persistent_resources_operations_get(...).doit().await
//! let r = hub.projects().locations_persistent_resources_operations_wait(...).doit().await
//! let r = hub.projects().locations_persistent_resources_create(...).doit().await
//! let r = hub.projects().locations_persistent_resources_delete(...).doit().await
//! let r = hub.projects().locations_persistent_resources_patch(...).doit().await
//! let r = hub.projects().locations_persistent_resources_reboot(...).doit().await
//! let r = hub.projects().locations_pipeline_jobs_operations_get(...).doit().await
//! let r = hub.projects().locations_pipeline_jobs_operations_wait(...).doit().await
//! let r = hub.projects().locations_pipeline_jobs_batch_cancel(...).doit().await
//! let r = hub.projects().locations_pipeline_jobs_batch_delete(...).doit().await
//! let r = hub.projects().locations_pipeline_jobs_delete(...).doit().await
//! let r = hub.projects().locations_publishers_models_export(...).doit().await
//! let r = hub.projects().locations_publishers_models_fetch_predict_operation(...).doit().await
//! let r = hub.projects().locations_publishers_models_predict_long_running(...).doit().await
//! let r = hub.projects().locations_publishers_models_set_publisher_model_config(...).doit().await
//! let r = hub.projects().locations_rag_corpora_operations_get(...).doit().await
//! let r = hub.projects().locations_rag_corpora_operations_wait(...).doit().await
//! let r = hub.projects().locations_rag_corpora_rag_files_operations_get(...).doit().await
//! let r = hub.projects().locations_rag_corpora_rag_files_operations_wait(...).doit().await
//! let r = hub.projects().locations_rag_corpora_rag_files_delete(...).doit().await
//! let r = hub.projects().locations_rag_corpora_rag_files_import(...).doit().await
//! let r = hub.projects().locations_rag_corpora_create(...).doit().await
//! let r = hub.projects().locations_rag_corpora_delete(...).doit().await
//! let r = hub.projects().locations_rag_corpora_patch(...).doit().await
//! let r = hub.projects().locations_rag_engine_config_operations_get(...).doit().await
//! let r = hub.projects().locations_rag_engine_config_operations_wait(...).doit().await
//! let r = hub.projects().locations_reasoning_engines_examples_operations_get(...).doit().await
//! let r = hub.projects().locations_reasoning_engines_examples_operations_wait(...).doit().await
//! let r = hub.projects().locations_reasoning_engines_memories_operations_get(...).doit().await
//! let r = hub.projects().locations_reasoning_engines_memories_operations_wait(...).doit().await
//! let r = hub.projects().locations_reasoning_engines_memories_create(...).doit().await
//! let r = hub.projects().locations_reasoning_engines_memories_delete(...).doit().await
//! let r = hub.projects().locations_reasoning_engines_memories_generate(...).doit().await
//! let r = hub.projects().locations_reasoning_engines_memories_patch(...).doit().await
//! let r = hub.projects().locations_reasoning_engines_memories_rollback(...).doit().await
//! let r = hub.projects().locations_reasoning_engines_operations_get(...).doit().await
//! let r = hub.projects().locations_reasoning_engines_operations_wait(...).doit().await
//! let r = hub.projects().locations_reasoning_engines_sandbox_environments_operations_get(...).doit().await
//! let r = hub.projects().locations_reasoning_engines_sandbox_environments_operations_wait(...).doit().await
//! let r = hub.projects().locations_reasoning_engines_sandbox_environments_create(...).doit().await
//! let r = hub.projects().locations_reasoning_engines_sandbox_environments_delete(...).doit().await
//! let r = hub.projects().locations_reasoning_engines_sessions_operations_get(...).doit().await
//! let r = hub.projects().locations_reasoning_engines_sessions_operations_wait(...).doit().await
//! let r = hub.projects().locations_reasoning_engines_sessions_create(...).doit().await
//! let r = hub.projects().locations_reasoning_engines_sessions_delete(...).doit().await
//! let r = hub.projects().locations_reasoning_engines_create(...).doit().await
//! let r = hub.projects().locations_reasoning_engines_delete(...).doit().await
//! let r = hub.projects().locations_reasoning_engines_patch(...).doit().await
//! let r = hub.projects().locations_schedules_operations_get(...).doit().await
//! let r = hub.projects().locations_schedules_operations_wait(...).doit().await
//! let r = hub.projects().locations_schedules_delete(...).doit().await
//! let r = hub.projects().locations_solvers_operations_get(...).doit().await
//! let r = hub.projects().locations_specialist_pools_operations_get(...).doit().await
//! let r = hub.projects().locations_specialist_pools_operations_wait(...).doit().await
//! let r = hub.projects().locations_specialist_pools_create(...).doit().await
//! let r = hub.projects().locations_specialist_pools_delete(...).doit().await
//! let r = hub.projects().locations_specialist_pools_patch(...).doit().await
//! let r = hub.projects().locations_studies_operations_get(...).doit().await
//! let r = hub.projects().locations_studies_operations_wait(...).doit().await
//! let r = hub.projects().locations_studies_trials_operations_get(...).doit().await
//! let r = hub.projects().locations_studies_trials_operations_wait(...).doit().await
//! let r = hub.projects().locations_studies_trials_check_trial_early_stopping_state(...).doit().await
//! let r = hub.projects().locations_studies_trials_suggest(...).doit().await
//! let r = hub.projects().locations_tensorboards_experiments_operations_get(...).doit().await
//! let r = hub.projects().locations_tensorboards_experiments_operations_wait(...).doit().await
//! let r = hub.projects().locations_tensorboards_experiments_runs_operations_get(...).doit().await
//! let r = hub.projects().locations_tensorboards_experiments_runs_operations_wait(...).doit().await
//! let r = hub.projects().locations_tensorboards_experiments_runs_time_series_operations_get(...).doit().await
//! let r = hub.projects().locations_tensorboards_experiments_runs_time_series_operations_wait(...).doit().await
//! let r = hub.projects().locations_tensorboards_experiments_runs_time_series_delete(...).doit().await
//! let r = hub.projects().locations_tensorboards_experiments_runs_delete(...).doit().await
//! let r = hub.projects().locations_tensorboards_experiments_delete(...).doit().await
//! let r = hub.projects().locations_tensorboards_operations_get(...).doit().await
//! let r = hub.projects().locations_tensorboards_operations_wait(...).doit().await
//! let r = hub.projects().locations_tensorboards_create(...).doit().await
//! let r = hub.projects().locations_tensorboards_delete(...).doit().await
//! let r = hub.projects().locations_tensorboards_patch(...).doit().await
//! let r = hub.projects().locations_training_pipelines_operations_get(...).doit().await
//! let r = hub.projects().locations_training_pipelines_operations_wait(...).doit().await
//! let r = hub.projects().locations_training_pipelines_delete(...).doit().await
//! let r = hub.projects().locations_tuning_jobs_rebase_tuned_model(...).doit().await
//! let r = hub.projects().locations_deploy(...).doit().await
//! let r = hub.projects().locations_deploy_publisher_model(...).doit().await
//! let r = hub.projects().locations_evaluate_dataset(...).doit().await
//! let r = hub.projects().locations_update_rag_engine_config(...).doit().await
//! let r = hub.projects().set_publisher_model_config(...).doit().await
//! let r = hub.projects().update_cache_config(...).doit().await
//! let r = hub.publishers().models_fetch_predict_operation(...).doit().await
//! let r = hub.publishers().models_predict_long_running(...).doit().await
//! let r = hub.rag_corpora().operations_get(...).doit().await
//! let r = hub.rag_corpora().operations_wait(...).doit().await
//! let r = hub.rag_corpora().rag_files_operations_get(...).doit().await
//! let r = hub.rag_corpora().rag_files_operations_wait(...).doit().await
//! let r = hub.rag_engine_config().operations_get(...).doit().await
//! let r = hub.rag_engine_config().operations_wait(...).doit().await
//! let r = hub.reasoning_engines().examples_operations_get(...).doit().await
//! let r = hub.reasoning_engines().examples_operations_wait(...).doit().await
//! let r = hub.reasoning_engines().memories_operations_get(...).doit().await
//! let r = hub.reasoning_engines().memories_operations_wait(...).doit().await
//! let r = hub.reasoning_engines().memories_create(...).doit().await
//! let r = hub.reasoning_engines().memories_delete(...).doit().await
//! let r = hub.reasoning_engines().memories_generate(...).doit().await
//! let r = hub.reasoning_engines().memories_patch(...).doit().await
//! let r = hub.reasoning_engines().memories_rollback(...).doit().await
//! let r = hub.reasoning_engines().operations_get(...).doit().await
//! let r = hub.reasoning_engines().operations_wait(...).doit().await
//! let r = hub.reasoning_engines().sandbox_environments_operations_get(...).doit().await
//! let r = hub.reasoning_engines().sandbox_environments_operations_wait(...).doit().await
//! let r = hub.reasoning_engines().sessions_operations_get(...).doit().await
//! let r = hub.reasoning_engines().sessions_operations_wait(...).doit().await
//! let r = hub.reasoning_engines().sessions_create(...).doit().await
//! let r = hub.reasoning_engines().sessions_delete(...).doit().await
//! let r = hub.reasoning_engines().create(...).doit().await
//! let r = hub.reasoning_engines().delete(...).doit().await
//! let r = hub.reasoning_engines().patch(...).doit().await
//! let r = hub.schedules().operations_get(...).doit().await
//! let r = hub.schedules().operations_wait(...).doit().await
//! let r = hub.solvers().operations_get(...).doit().await
//! let r = hub.specialist_pools().operations_get(...).doit().await
//! let r = hub.specialist_pools().operations_wait(...).doit().await
//! let r = hub.studies().operations_get(...).doit().await
//! let r = hub.studies().operations_wait(...).doit().await
//! let r = hub.studies().trials_operations_get(...).doit().await
//! let r = hub.studies().trials_operations_wait(...).doit().await
//! let r = hub.tensorboards().experiments_operations_get(...).doit().await
//! let r = hub.tensorboards().experiments_operations_wait(...).doit().await
//! let r = hub.tensorboards().experiments_runs_operations_get(...).doit().await
//! let r = hub.tensorboards().experiments_runs_operations_wait(...).doit().await
//! let r = hub.tensorboards().experiments_runs_time_series_operations_get(...).doit().await
//! let r = hub.tensorboards().experiments_runs_time_series_operations_wait(...).doit().await
//! let r = hub.tensorboards().operations_get(...).doit().await
//! let r = hub.tensorboards().operations_wait(...).doit().await
//! let r = hub.training_pipelines().operations_get(...).doit().await
//! let r = hub.training_pipelines().operations_wait(...).doit().await
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
//! google-aiplatform1_beta1 = "*"
//! serde = "1"
//! serde_json = "1"
//! ```
//!
//! ## A complete example
//!
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate google_aiplatform1_beta1 as aiplatform1_beta1;
//! use aiplatform1_beta1::{Result, Error};
//! # async fn dox() {
//! use aiplatform1_beta1::{Aiplatform, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
//! let mut hub = Aiplatform::new(client, auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.projects().locations_example_stores_create("parent")
//!              .example_store_update_time(chrono::Utc::now())
//!              .example_store_name("sanctus")
//!              .example_store_example_store_config_vertex_embedding_model("sed")
//!              .example_store_display_name("amet.")
//!              .example_store_description("takimata")
//!              .example_store_create_time(chrono::Utc::now())
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
pub use api::Aiplatform;
