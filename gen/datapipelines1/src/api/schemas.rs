use super::*;
/// Pipeline job details specific to the Dataflow API. This is encapsulated here to allow for more executors to store their specific details separately.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatapipelinesV1DataflowJobDetails {
    /// Output only. The current number of workers used to run the jobs. Only set to a value if the job is still running.
    #[serde(rename="currentWorkers")]
    
    pub current_workers: Option<i32>,
    /// Cached version of all the metrics of interest for the job. This value gets stored here when the job is terminated. As long as the job is running, this field is populated from the Dataflow API.
    #[serde(rename="resourceInfo")]
    
    pub resource_info: Option<HashMap<String, f64>>,
    /// Output only. The SDK version used to run the job.
    #[serde(rename="sdkVersion")]
    
    pub sdk_version: Option<GoogleCloudDatapipelinesV1SdkVersion>,
}

impl client::Part for GoogleCloudDatapipelinesV1DataflowJobDetails {}


/// The environment values to be set at runtime for a Flex Template.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatapipelinesV1FlexTemplateRuntimeEnvironment {
    /// Additional experiment flags for the job.
    #[serde(rename="additionalExperiments")]
    
    pub additional_experiments: Option<Vec<String>>,
    /// Additional user labels to be specified for the job. Keys and values must follow the restrictions specified in the [labeling restrictions](https://cloud.google.com/compute/docs/labeling-resources#restrictions). An object containing a list of key/value pairs. Example: `{ "name": "wrench", "mass": "1kg", "count": "3" }`.
    #[serde(rename="additionalUserLabels")]
    
    pub additional_user_labels: Option<HashMap<String, String>>,
    /// Whether to enable Streaming Engine for the job.
    #[serde(rename="enableStreamingEngine")]
    
    pub enable_streaming_engine: Option<bool>,
    /// Set FlexRS goal for the job. https://cloud.google.com/dataflow/docs/guides/flexrs
    #[serde(rename="flexrsGoal")]
    
    pub flexrs_goal: Option<GoogleCloudDatapipelinesV1FlexTemplateRuntimeEnvironmentFlexrsGoalEnum>,
    /// Configuration for VM IPs.
    #[serde(rename="ipConfiguration")]
    
    pub ip_configuration: Option<GoogleCloudDatapipelinesV1FlexTemplateRuntimeEnvironmentIpConfigurationEnum>,
    /// Name for the Cloud KMS key for the job. Key format is: projects//locations//keyRings//cryptoKeys/
    #[serde(rename="kmsKeyName")]
    
    pub kms_key_name: Option<String>,
    /// The machine type to use for the job. Defaults to the value from the template if not specified.
    #[serde(rename="machineType")]
    
    pub machine_type: Option<String>,
    /// The maximum number of Compute Engine instances to be made available to your pipeline during execution, from 1 to 1000.
    #[serde(rename="maxWorkers")]
    
    pub max_workers: Option<i32>,
    /// Network to which VMs will be assigned. If empty or unspecified, the service will use the network "default".
    
    pub network: Option<String>,
    /// The initial number of Compute Engine instances for the job.
    #[serde(rename="numWorkers")]
    
    pub num_workers: Option<i32>,
    /// The email address of the service account to run the job as.
    #[serde(rename="serviceAccountEmail")]
    
    pub service_account_email: Option<String>,
    /// Subnetwork to which VMs will be assigned, if desired. You can specify a subnetwork using either a complete URL or an abbreviated path. Expected to be of the form "https://www.googleapis.com/compute/v1/projects/HOST_PROJECT_ID/regions/REGION/subnetworks/SUBNETWORK" or "regions/REGION/subnetworks/SUBNETWORK". If the subnetwork is located in a Shared VPC network, you must use the complete URL.
    
    pub subnetwork: Option<String>,
    /// The Cloud Storage path to use for temporary files. Must be a valid Cloud Storage URL, beginning with `gs://`.
    #[serde(rename="tempLocation")]
    
    pub temp_location: Option<String>,
    /// The Compute Engine region (https://cloud.google.com/compute/docs/regions-zones/regions-zones) in which worker processing should occur, e.g. "us-west1". Mutually exclusive with worker_zone. If neither worker_region nor worker_zone is specified, defaults to the control plane region.
    #[serde(rename="workerRegion")]
    
    pub worker_region: Option<String>,
    /// The Compute Engine zone (https://cloud.google.com/compute/docs/regions-zones/regions-zones) in which worker processing should occur, e.g. "us-west1-a". Mutually exclusive with worker_region. If neither worker_region nor worker_zone is specified, a zone in the control plane region is chosen based on available capacity. If both `worker_zone` and `zone` are set, `worker_zone` takes precedence.
    #[serde(rename="workerZone")]
    
    pub worker_zone: Option<String>,
    /// The Compute Engine [availability zone](https://cloud.google.com/compute/docs/regions-zones/regions-zones) for launching worker instances to run your pipeline. In the future, worker_zone will take precedence.
    
    pub zone: Option<String>,
}

impl client::Part for GoogleCloudDatapipelinesV1FlexTemplateRuntimeEnvironment {}


/// Definition of the job information maintained by the pipeline. Fields in this entity are retrieved from the executor API (e.g. Dataflow API).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatapipelinesV1Job {
    /// Output only. The time of job creation.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// All the details that are specific to a Dataflow job.
    #[serde(rename="dataflowJobDetails")]
    
    pub dataflow_job_details: Option<GoogleCloudDatapipelinesV1DataflowJobDetails>,
    /// Output only. The time of job termination. This is absent if the job is still running.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The internal ID for the job.
    
    pub id: Option<String>,
    /// Required. The fully qualified resource name for the job.
    
    pub name: Option<String>,
    /// The current state of the job.
    
    pub state: Option<GoogleCloudDatapipelinesV1JobStateEnum>,
    /// Status capturing any error code or message related to job creation or execution.
    
    pub status: Option<GoogleRpcStatus>,
}

impl client::Part for GoogleCloudDatapipelinesV1Job {}


/// Launch Flex Template parameter.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatapipelinesV1LaunchFlexTemplateParameter {
    /// Cloud Storage path to a file with a JSON-serialized ContainerSpec as content.
    #[serde(rename="containerSpecGcsPath")]
    
    pub container_spec_gcs_path: Option<String>,
    /// The runtime environment for the Flex Template job.
    
    pub environment: Option<GoogleCloudDatapipelinesV1FlexTemplateRuntimeEnvironment>,
    /// Required. The job name to use for the created job. For an update job request, the job name should be the same as the existing running job.
    #[serde(rename="jobName")]
    
    pub job_name: Option<String>,
    /// Launch options for this Flex Template job. This is a common set of options across languages and templates. This should not be used to pass job parameters.
    #[serde(rename="launchOptions")]
    
    pub launch_options: Option<HashMap<String, String>>,
    /// The parameters for the Flex Template. Example: `{"num_workers":"5"}`
    
    pub parameters: Option<HashMap<String, String>>,
    /// Use this to pass transform name mappings for streaming update jobs. Example: `{"oldTransformName":"newTransformName",...}`
    #[serde(rename="transformNameMappings")]
    
    pub transform_name_mappings: Option<HashMap<String, String>>,
    /// Set this to true if you are sending a request to update a running streaming job. When set, the job name should be the same as the running job.
    
    pub update: Option<bool>,
}

impl client::Part for GoogleCloudDatapipelinesV1LaunchFlexTemplateParameter {}


/// A request to launch a Dataflow job from a Flex Template.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatapipelinesV1LaunchFlexTemplateRequest {
    /// Required. Parameter to launch a job from a Flex Template.
    #[serde(rename="launchParameter")]
    
    pub launch_parameter: Option<GoogleCloudDatapipelinesV1LaunchFlexTemplateParameter>,
    /// Required. The [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) to which to direct the request. For example, `us-central1`, `us-west1`.
    
    pub location: Option<String>,
    /// Required. The ID of the Cloud Platform project that the job belongs to.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// If true, the request is validated but not actually executed. Defaults to false.
    #[serde(rename="validateOnly")]
    
    pub validate_only: Option<bool>,
}

impl client::Part for GoogleCloudDatapipelinesV1LaunchFlexTemplateRequest {}


/// Parameters to provide to the template being launched.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatapipelinesV1LaunchTemplateParameters {
    /// The runtime environment for the job.
    
    pub environment: Option<GoogleCloudDatapipelinesV1RuntimeEnvironment>,
    /// Required. The job name to use for the created job.
    #[serde(rename="jobName")]
    
    pub job_name: Option<String>,
    /// The runtime parameters to pass to the job.
    
    pub parameters: Option<HashMap<String, String>>,
    /// Map of transform name prefixes of the job to be replaced to the corresponding name prefixes of the new job. Only applicable when updating a pipeline.
    #[serde(rename="transformNameMapping")]
    
    pub transform_name_mapping: Option<HashMap<String, String>>,
    /// If set, replace the existing pipeline with the name specified by jobName with this pipeline, preserving state.
    
    pub update: Option<bool>,
}

impl client::Part for GoogleCloudDatapipelinesV1LaunchTemplateParameters {}


/// A request to launch a template.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatapipelinesV1LaunchTemplateRequest {
    /// A Cloud Storage path to the template from which to create the job. Must be a valid Cloud Storage URL, beginning with 'gs://'.
    #[serde(rename="gcsPath")]
    
    pub gcs_path: Option<String>,
    /// The parameters of the template to launch. This should be part of the body of the POST request.
    #[serde(rename="launchParameters")]
    
    pub launch_parameters: Option<GoogleCloudDatapipelinesV1LaunchTemplateParameters>,
    /// The [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) to which to direct the request.
    
    pub location: Option<String>,
    /// Required. The ID of the Cloud Platform project that the job belongs to.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// If true, the request is validated but not actually executed. Defaults to false.
    #[serde(rename="validateOnly")]
    
    pub validate_only: Option<bool>,
}

impl client::Part for GoogleCloudDatapipelinesV1LaunchTemplateRequest {}


/// Response message for ListJobs
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations pipelines jobs list projects](ProjectLocationPipelineJobListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatapipelinesV1ListJobsResponse {
    /// Results that were accessible to the caller. Results are always in descending order of job creation date.
    
    pub jobs: Option<Vec<GoogleCloudDatapipelinesV1Job>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDatapipelinesV1ListJobsResponse {}


/// Response message for ListPipelines.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations pipelines list projects](ProjectLocationPipelineListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatapipelinesV1ListPipelinesResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Results that matched the filter criteria and were accessible to the caller. Results are always in descending order of pipeline creation date.
    
    pub pipelines: Option<Vec<GoogleCloudDatapipelinesV1Pipeline>>,
}

impl client::ResponseResult for GoogleCloudDatapipelinesV1ListPipelinesResponse {}


/// The main pipeline entity and all the necessary metadata for launching and managing linked jobs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations pipelines create projects](ProjectLocationPipelineCreateCall) (request|response)
/// * [locations pipelines get projects](ProjectLocationPipelineGetCall) (response)
/// * [locations pipelines patch projects](ProjectLocationPipelinePatchCall) (request|response)
/// * [locations pipelines stop projects](ProjectLocationPipelineStopCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatapipelinesV1Pipeline {
    /// Output only. Immutable. The timestamp when the pipeline was initially created. Set by the Data Pipelines service.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. The display name of the pipeline. It can contain only letters ([A-Za-z]), numbers ([0-9]), hyphens (-), and underscores (_).
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Number of jobs.
    #[serde(rename="jobCount")]
    
    pub job_count: Option<i32>,
    /// Output only. Immutable. The timestamp when the pipeline was last modified. Set by the Data Pipelines service.
    #[serde(rename="lastUpdateTime")]
    
    pub last_update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The pipeline name. For example: `projects/PROJECT_ID/locations/LOCATION_ID/pipelines/PIPELINE_ID`. * `PROJECT_ID` can contain letters ([A-Za-z]), numbers ([0-9]), hyphens (-), colons (:), and periods (.). For more information, see [Identifying projects](https://cloud.google.com/resource-manager/docs/creating-managing-projects#identifying_projects). * `LOCATION_ID` is the canonical ID for the pipeline's location. The list of available locations can be obtained by calling `google.cloud.location.Locations.ListLocations`. Note that the Data Pipelines service is not available in all regions. It depends on Cloud Scheduler, an App Engine application, so it's only available in [App Engine regions](https://cloud.google.com/about/locations#region). * `PIPELINE_ID` is the ID of the pipeline. Must be unique for the selected project and location.
    
    pub name: Option<String>,
    /// Immutable. The sources of the pipeline (for example, Dataplex). The keys and values are set by the corresponding sources during pipeline creation.
    #[serde(rename="pipelineSources")]
    
    pub pipeline_sources: Option<HashMap<String, String>>,
    /// Internal scheduling information for a pipeline. If this information is provided, periodic jobs will be created per the schedule. If not, users are responsible for creating jobs externally.
    #[serde(rename="scheduleInfo")]
    
    pub schedule_info: Option<GoogleCloudDatapipelinesV1ScheduleSpec>,
    /// Optional. A service account email to be used with the Cloud Scheduler job. If not specified, the default compute engine service account will be used.
    #[serde(rename="schedulerServiceAccountEmail")]
    
    pub scheduler_service_account_email: Option<String>,
    /// Required. The state of the pipeline. When the pipeline is created, the state is set to 'PIPELINE_STATE_ACTIVE' by default. State changes can be requested by setting the state to stopping, paused, or resuming. State cannot be changed through UpdatePipeline requests.
    
    pub state: Option<GoogleCloudDatapipelinesV1PipelineStateEnum>,
    /// Required. The type of the pipeline. This field affects the scheduling of the pipeline and the type of metrics to show for the pipeline.
    #[serde(rename="type")]
    
    pub type_: Option<GoogleCloudDatapipelinesV1PipelineTypeEnum>,
    /// Workload information for creating new jobs.
    
    pub workload: Option<GoogleCloudDatapipelinesV1Workload>,
}

impl client::RequestValue for GoogleCloudDatapipelinesV1Pipeline {}
impl client::ResponseResult for GoogleCloudDatapipelinesV1Pipeline {}


/// Request message for RunPipeline
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations pipelines run projects](ProjectLocationPipelineRunCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatapipelinesV1RunPipelineRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleCloudDatapipelinesV1RunPipelineRequest {}


/// Response message for RunPipeline
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations pipelines run projects](ProjectLocationPipelineRunCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatapipelinesV1RunPipelineResponse {
    /// Job that was created as part of RunPipeline operation.
    
    pub job: Option<GoogleCloudDatapipelinesV1Job>,
}

impl client::ResponseResult for GoogleCloudDatapipelinesV1RunPipelineResponse {}


/// The environment values to set at runtime.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatapipelinesV1RuntimeEnvironment {
    /// Additional experiment flags for the job.
    #[serde(rename="additionalExperiments")]
    
    pub additional_experiments: Option<Vec<String>>,
    /// Additional user labels to be specified for the job. Keys and values should follow the restrictions specified in the [labeling restrictions](https://cloud.google.com/compute/docs/labeling-resources#restrictions) page. An object containing a list of key/value pairs. Example: { "name": "wrench", "mass": "1kg", "count": "3" }.
    #[serde(rename="additionalUserLabels")]
    
    pub additional_user_labels: Option<HashMap<String, String>>,
    /// Whether to bypass the safety checks for the job's temporary directory. Use with caution.
    #[serde(rename="bypassTempDirValidation")]
    
    pub bypass_temp_dir_validation: Option<bool>,
    /// Whether to enable Streaming Engine for the job.
    #[serde(rename="enableStreamingEngine")]
    
    pub enable_streaming_engine: Option<bool>,
    /// Configuration for VM IPs.
    #[serde(rename="ipConfiguration")]
    
    pub ip_configuration: Option<GoogleCloudDatapipelinesV1RuntimeEnvironmentIpConfigurationEnum>,
    /// Name for the Cloud KMS key for the job. The key format is: projects//locations//keyRings//cryptoKeys/
    #[serde(rename="kmsKeyName")]
    
    pub kms_key_name: Option<String>,
    /// The machine type to use for the job. Defaults to the value from the template if not specified.
    #[serde(rename="machineType")]
    
    pub machine_type: Option<String>,
    /// The maximum number of Compute Engine instances to be made available to your pipeline during execution, from 1 to 1000.
    #[serde(rename="maxWorkers")]
    
    pub max_workers: Option<i32>,
    /// Network to which VMs will be assigned. If empty or unspecified, the service will use the network "default".
    
    pub network: Option<String>,
    /// The initial number of Compute Engine instances for the job.
    #[serde(rename="numWorkers")]
    
    pub num_workers: Option<i32>,
    /// The email address of the service account to run the job as.
    #[serde(rename="serviceAccountEmail")]
    
    pub service_account_email: Option<String>,
    /// Subnetwork to which VMs will be assigned, if desired. You can specify a subnetwork using either a complete URL or an abbreviated path. Expected to be of the form "https://www.googleapis.com/compute/v1/projects/HOST_PROJECT_ID/regions/REGION/subnetworks/SUBNETWORK" or "regions/REGION/subnetworks/SUBNETWORK". If the subnetwork is located in a Shared VPC network, you must use the complete URL.
    
    pub subnetwork: Option<String>,
    /// The Cloud Storage path to use for temporary files. Must be a valid Cloud Storage URL, beginning with `gs://`.
    #[serde(rename="tempLocation")]
    
    pub temp_location: Option<String>,
    /// The Compute Engine region (https://cloud.google.com/compute/docs/regions-zones/regions-zones) in which worker processing should occur, e.g. "us-west1". Mutually exclusive with worker_zone. If neither worker_region nor worker_zone is specified, default to the control plane's region.
    #[serde(rename="workerRegion")]
    
    pub worker_region: Option<String>,
    /// The Compute Engine zone (https://cloud.google.com/compute/docs/regions-zones/regions-zones) in which worker processing should occur, e.g. "us-west1-a". Mutually exclusive with worker_region. If neither worker_region nor worker_zone is specified, a zone in the control plane's region is chosen based on available capacity. If both `worker_zone` and `zone` are set, `worker_zone` takes precedence.
    #[serde(rename="workerZone")]
    
    pub worker_zone: Option<String>,
    /// The Compute Engine [availability zone](https://cloud.google.com/compute/docs/regions-zones/regions-zones) for launching worker instances to run your pipeline. In the future, worker_zone will take precedence.
    
    pub zone: Option<String>,
}

impl client::Part for GoogleCloudDatapipelinesV1RuntimeEnvironment {}


/// Details of the schedule the pipeline runs on.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatapipelinesV1ScheduleSpec {
    /// Output only. When the next Scheduler job is going to run.
    #[serde(rename="nextJobTime")]
    
    pub next_job_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Unix-cron format of the schedule. This information is retrieved from the linked Cloud Scheduler.
    
    pub schedule: Option<String>,
    /// Timezone ID. This matches the timezone IDs used by the Cloud Scheduler API. If empty, UTC time is assumed.
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<String>,
}

impl client::Part for GoogleCloudDatapipelinesV1ScheduleSpec {}


/// The version of the SDK used to run the job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatapipelinesV1SdkVersion {
    /// The support status for this SDK version.
    #[serde(rename="sdkSupportStatus")]
    
    pub sdk_support_status: Option<GoogleCloudDatapipelinesV1SdkVersionSdkSupportStatusEnum>,
    /// The version of the SDK used to run the job.
    
    pub version: Option<String>,
    /// A readable string describing the version of the SDK.
    #[serde(rename="versionDisplayName")]
    
    pub version_display_name: Option<String>,
}

impl client::Part for GoogleCloudDatapipelinesV1SdkVersion {}


/// Request message for StopPipeline.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations pipelines stop projects](ProjectLocationPipelineStopCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatapipelinesV1StopPipelineRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleCloudDatapipelinesV1StopPipelineRequest {}


/// Workload details for creating the pipeline jobs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatapipelinesV1Workload {
    /// Template information and additional parameters needed to launch a Dataflow job using the flex launch API.
    #[serde(rename="dataflowFlexTemplateRequest")]
    
    pub dataflow_flex_template_request: Option<GoogleCloudDatapipelinesV1LaunchFlexTemplateRequest>,
    /// Template information and additional parameters needed to launch a Dataflow job using the standard launch API.
    #[serde(rename="dataflowLaunchTemplateRequest")]
    
    pub dataflow_launch_template_request: Option<GoogleCloudDatapipelinesV1LaunchTemplateRequest>,
}

impl client::Part for GoogleCloudDatapipelinesV1Workload {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations pipelines delete projects](ProjectLocationPipelineDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleProtobufEmpty { _never_set: Option<bool> }

impl client::ResponseResult for GoogleProtobufEmpty {}


/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleRpcStatus {
    /// The status code, which should be an enum value of google.rpc.Code.
    
    pub code: Option<i32>,
    /// A list of messages that carry the error details. There is a common set of message types for APIs to use.
    
    pub details: Option<Vec<HashMap<String, json::Value>>>,
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client.
    
    pub message: Option<String>,
}

impl client::Part for GoogleRpcStatus {}


