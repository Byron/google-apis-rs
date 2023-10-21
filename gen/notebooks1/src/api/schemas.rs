use super::*;
/// Definition of a hardware accelerator. Note that not all combinations of `type` and `core_count` are valid. Check [GPUs on Compute Engine](https://cloud.google.com/compute/docs/gpus/#gpus-list) to find a valid combination. TPUs are not supported.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AcceleratorConfig {
    /// Count of cores of this accelerator.
    #[serde(rename="coreCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub core_count: Option<i64>,
    /// Type of this accelerator.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for AcceleratorConfig {}


/// Associates `members`, or principals, with a `role`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Binding {
    /// The condition that is associated with this binding. If the condition evaluates to `true`, then this binding applies to the current request. If the condition evaluates to `false`, then this binding does not apply to the current request. However, a different role binding might grant the same role to one or more of the principals in this binding. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub condition: Option<Expr>,
    /// Specifies the principals requesting access for a Google Cloud resource. `members` can have the following values: * `allUsers`: A special identifier that represents anyone who is on the internet; with or without a Google account. * `allAuthenticatedUsers`: A special identifier that represents anyone who is authenticated with a Google account or a service account. Does not include identities that come from external identity providers (IdPs) through identity federation. * `user:{emailid}`: An email address that represents a specific Google account. For example, `alice@example.com` . * `serviceAccount:{emailid}`: An email address that represents a Google service account. For example, `my-other-app@appspot.gserviceaccount.com`. * `serviceAccount:{projectid}.svc.id.goog[{namespace}/{kubernetes-sa}]`: An identifier for a [Kubernetes service account](https://cloud.google.com/kubernetes-engine/docs/how-to/kubernetes-service-accounts). For example, `my-project.svc.id.goog[my-namespace/my-kubernetes-sa]`. * `group:{emailid}`: An email address that represents a Google group. For example, `admins@example.com`. * `deleted:user:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a user that has been recently deleted. For example, `alice@example.com?uid=123456789012345678901`. If the user is recovered, this value reverts to `user:{emailid}` and the recovered user retains the role in the binding. * `deleted:serviceAccount:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a service account that has been recently deleted. For example, `my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901`. If the service account is undeleted, this value reverts to `serviceAccount:{emailid}` and the undeleted service account retains the role in the binding. * `deleted:group:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a Google group that has been recently deleted. For example, `admins@example.com?uid=123456789012345678901`. If the group is recovered, this value reverts to `group:{emailid}` and the recovered group retains the role in the binding. * `domain:{domain}`: The G Suite domain (primary) that represents all the users of that domain. For example, `google.com` or `example.com`. 
    
    pub members: Option<Vec<String>>,
    /// Role that is assigned to the list of `members`, or principals. For example, `roles/viewer`, `roles/editor`, or `roles/owner`.
    
    pub role: Option<String>,
}

impl client::Part for Binding {}


/// Definition of the boot image used by the Runtime. Used to facilitate runtime upgradeability.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BootImage { _never_set: Option<bool> }

impl client::Part for BootImage {}


/// The request message for Operations.CancelOperation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations cancel projects](ProjectLocationOperationCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelOperationRequest { _never_set: Option<bool> }

impl client::RequestValue for CancelOperationRequest {}


/// Definition of a container image for starting a notebook instance with the environment installed in a container.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContainerImage {
    /// Required. The path to the container image repository. For example: `gcr.io/{project_id}/{image_name}`
    
    pub repository: Option<String>,
    /// The tag of the container image. If not specified, this defaults to the latest tag.
    
    pub tag: Option<String>,
}

impl client::Part for ContainerImage {}


/// Parameters used in Dataproc JobType executions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataprocParameters {
    /// URI for cluster used to run Dataproc execution. Format: `projects/{PROJECT_ID}/regions/{REGION}/clusters/{CLUSTER_NAME}`
    
    pub cluster: Option<String>,
}

impl client::Part for DataprocParameters {}


/// Request for creating a notebook instance diagnostic file.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances diagnose projects](ProjectLocationInstanceDiagnoseCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DiagnoseInstanceRequest {
    /// Required. Defines flags that are used to run the diagnostic tool
    #[serde(rename="diagnosticConfig")]
    
    pub diagnostic_config: Option<DiagnosticConfig>,
}

impl client::RequestValue for DiagnoseInstanceRequest {}


/// Request for creating a notebook instance diagnostic file.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations runtimes diagnose projects](ProjectLocationRuntimeDiagnoseCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DiagnoseRuntimeRequest {
    /// Required. Defines flags that are used to run the diagnostic tool
    #[serde(rename="diagnosticConfig")]
    
    pub diagnostic_config: Option<DiagnosticConfig>,
}

impl client::RequestValue for DiagnoseRuntimeRequest {}


/// Defines flags that are used to run the diagnostic tool
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DiagnosticConfig {
    /// Optional. Enables flag to copy all `/home/jupyter` folder contents
    #[serde(rename="copyHomeFilesFlagEnabled")]
    
    pub copy_home_files_flag_enabled: Option<bool>,
    /// Required. User Cloud Storage bucket location (REQUIRED). Must be formatted with path prefix (`gs://$GCS_BUCKET`). Permissions: User Managed Notebooks: - storage.buckets.writer: Must be given to the project's service account attached to VM. Google Managed Notebooks: - storage.buckets.writer: Must be given to the project's service account or user credentials attached to VM depending on authentication mode. Cloud Storage bucket Log file will be written to `gs://$GCS_BUCKET/$RELATIVE_PATH/$VM_DATE_$TIME.tar.gz`
    #[serde(rename="gcsBucket")]
    
    pub gcs_bucket: Option<String>,
    /// Optional. Enables flag to capture packets from the instance for 30 seconds
    #[serde(rename="packetCaptureFlagEnabled")]
    
    pub packet_capture_flag_enabled: Option<bool>,
    /// Optional. Defines the relative storage path in the Cloud Storage bucket where the diagnostic logs will be written: Default path will be the root directory of the Cloud Storage bucket (`gs://$GCS_BUCKET/$DATE_$TIME.tar.gz`) Example of full path where Log file will be written: `gs://$GCS_BUCKET/$RELATIVE_PATH/`
    #[serde(rename="relativePath")]
    
    pub relative_path: Option<String>,
    /// Optional. Enables flag to repair service for instance
    #[serde(rename="repairFlagEnabled")]
    
    pub repair_flag_enabled: Option<bool>,
}

impl client::Part for DiagnosticConfig {}


/// An instance-attached disk resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Disk {
    /// Indicates whether the disk will be auto-deleted when the instance is deleted (but not when the disk is detached from the instance).
    #[serde(rename="autoDelete")]
    
    pub auto_delete: Option<bool>,
    /// Indicates that this is a boot disk. The virtual machine will use the first partition of the disk for its root filesystem.
    
    pub boot: Option<bool>,
    /// Indicates a unique device name of your choice that is reflected into the /dev/disk/by-id/google-* tree of a Linux operating system running within the instance. This name can be used to reference the device for mounting, resizing, and so on, from within the instance. If not specified, the server chooses a default device name to apply to this disk, in the form persistent-disk-x, where x is a number assigned by Google Compute Engine.This field is only applicable for persistent disks.
    #[serde(rename="deviceName")]
    
    pub device_name: Option<String>,
    /// Indicates the size of the disk in base-2 GB.
    #[serde(rename="diskSizeGb")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub disk_size_gb: Option<i64>,
    /// Indicates a list of features to enable on the guest operating system. Applicable only for bootable images. Read Enabling guest operating system features to see a list of available options.
    #[serde(rename="guestOsFeatures")]
    
    pub guest_os_features: Option<Vec<GuestOsFeature>>,
    /// A zero-based index to this disk, where 0 is reserved for the boot disk. If you have many disks attached to an instance, each disk would have a unique index number.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub index: Option<i64>,
    /// Indicates the disk interface to use for attaching this disk, which is either SCSI or NVME. The default is SCSI. Persistent disks must always use SCSI and the request will fail if you attempt to attach a persistent disk in any other format than SCSI. Local SSDs can use either NVME or SCSI. For performance characteristics of SCSI over NVMe, see Local SSD performance. Valid values: * NVME * SCSI
    
    pub interface: Option<String>,
    /// Type of the resource. Always compute#attachedDisk for attached disks.
    
    pub kind: Option<String>,
    /// A list of publicly visible licenses. Reserved for Google's use. A License represents billing and aggregate usage data for public and marketplace images.
    
    pub licenses: Option<Vec<String>>,
    /// The mode in which to attach this disk, either READ_WRITE or READ_ONLY. If not specified, the default is to attach the disk in READ_WRITE mode. Valid values: * READ_ONLY * READ_WRITE
    
    pub mode: Option<String>,
    /// Indicates a valid partial or full URL to an existing Persistent Disk resource.
    
    pub source: Option<String>,
    /// Indicates the type of the disk, either SCRATCH or PERSISTENT. Valid values: * PERSISTENT * SCRATCH
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for Disk {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations cancel projects](ProjectLocationOperationCancelCall) (response)
/// * [locations operations delete projects](ProjectLocationOperationDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// Represents a custom encryption key configuration that can be applied to a resource. This will encrypt all disks in Virtual Machine.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EncryptionConfig {
    /// The Cloud KMS resource identifier of the customer-managed encryption key used to protect a resource, such as a disks. It has the following format: `projects/{PROJECT_ID}/locations/{REGION}/keyRings/{KEY_RING_NAME}/cryptoKeys/{KEY_NAME}`
    #[serde(rename="kmsKey")]
    
    pub kms_key: Option<String>,
}

impl client::Part for EncryptionConfig {}


/// Definition of a software environment that is used to start a notebook instance.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations environments create projects](ProjectLocationEnvironmentCreateCall) (request)
/// * [locations environments get projects](ProjectLocationEnvironmentGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Environment {
    /// Use a container image to start the notebook instance.
    #[serde(rename="containerImage")]
    
    pub container_image: Option<ContainerImage>,
    /// Output only. The time at which this environment was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A brief description of this environment.
    
    pub description: Option<String>,
    /// Display name of this environment for the UI.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Name of this environment. Format: `projects/{project_id}/locations/{location}/environments/{environment_id}`
    
    pub name: Option<String>,
    /// Path to a Bash script that automatically runs after a notebook instance fully boots up. The path must be a URL or Cloud Storage path. Example: `"gs://path-to-file/file-name"`
    #[serde(rename="postStartupScript")]
    
    pub post_startup_script: Option<String>,
    /// Use a Compute Engine VM image to start the notebook instance.
    #[serde(rename="vmImage")]
    
    pub vm_image: Option<VmImage>,
}

impl client::RequestValue for Environment {}
impl client::ResponseResult for Environment {}


/// The definition of an Event for a managed / semi-managed notebook instance.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    /// Optional. Event details. This field is used to pass event information.
    
    pub details: Option<HashMap<String, String>>,
    /// Event report time.
    #[serde(rename="reportTime")]
    
    pub report_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Event type.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for Event {}


/// The definition of a single executed notebook.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations executions create projects](ProjectLocationExecutionCreateCall) (request)
/// * [locations executions get projects](ProjectLocationExecutionGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Execution {
    /// Output only. Time the Execution was instantiated.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A brief description of this execution.
    
    pub description: Option<String>,
    /// Output only. Name used for UI purposes. Name can only contain alphanumeric characters and underscores '_'.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// execute metadata including name, hardware spec, region, labels, etc.
    #[serde(rename="executionTemplate")]
    
    pub execution_template: Option<ExecutionTemplate>,
    /// Output only. The URI of the external job used to execute the notebook.
    #[serde(rename="jobUri")]
    
    pub job_uri: Option<String>,
    /// Output only. The resource name of the execute. Format: `projects/{project_id}/locations/{location}/executions/{execution_id}`
    
    pub name: Option<String>,
    /// Output notebook file generated by this execution
    #[serde(rename="outputNotebookFile")]
    
    pub output_notebook_file: Option<String>,
    /// Output only. State of the underlying AI Platform job.
    
    pub state: Option<String>,
    /// Output only. Time the Execution was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Execution {}
impl client::ResponseResult for Execution {}


/// The description a notebook execution workload.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionTemplate {
    /// Configuration (count and accelerator type) for hardware running notebook execution.
    #[serde(rename="acceleratorConfig")]
    
    pub accelerator_config: Option<SchedulerAcceleratorConfig>,
    /// Container Image URI to a DLVM Example: 'gcr.io/deeplearning-platform-release/base-cu100' More examples can be found at: https://cloud.google.com/ai-platform/deep-learning-containers/docs/choosing-container
    #[serde(rename="containerImageUri")]
    
    pub container_image_uri: Option<String>,
    /// Parameters used in Dataproc JobType executions.
    #[serde(rename="dataprocParameters")]
    
    pub dataproc_parameters: Option<DataprocParameters>,
    /// Path to the notebook file to execute. Must be in a Google Cloud Storage bucket. Format: `gs://{bucket_name}/{folder}/{notebook_file_name}` Ex: `gs://notebook_user/scheduled_notebooks/sentiment_notebook.ipynb`
    #[serde(rename="inputNotebookFile")]
    
    pub input_notebook_file: Option<String>,
    /// The type of Job to be used on this execution.
    #[serde(rename="jobType")]
    
    pub job_type: Option<String>,
    /// Name of the kernel spec to use. This must be specified if the kernel spec name on the execution target does not match the name in the input notebook file.
    #[serde(rename="kernelSpec")]
    
    pub kernel_spec: Option<String>,
    /// Labels for execution. If execution is scheduled, a field included will be 'nbs-scheduled'. Otherwise, it is an immediate execution, and an included field will be 'nbs-immediate'. Use fields to efficiently index between various types of executions.
    
    pub labels: Option<HashMap<String, String>>,
    /// Specifies the type of virtual machine to use for your training job's master worker. You must specify this field when `scaleTier` is set to `CUSTOM`. You can use certain Compute Engine machine types directly in this field. The following types are supported: - `n1-standard-4` - `n1-standard-8` - `n1-standard-16` - `n1-standard-32` - `n1-standard-64` - `n1-standard-96` - `n1-highmem-2` - `n1-highmem-4` - `n1-highmem-8` - `n1-highmem-16` - `n1-highmem-32` - `n1-highmem-64` - `n1-highmem-96` - `n1-highcpu-16` - `n1-highcpu-32` - `n1-highcpu-64` - `n1-highcpu-96` Alternatively, you can use the following legacy machine types: - `standard` - `large_model` - `complex_model_s` - `complex_model_m` - `complex_model_l` - `standard_gpu` - `complex_model_m_gpu` - `complex_model_l_gpu` - `standard_p100` - `complex_model_m_p100` - `standard_v100` - `large_model_v100` - `complex_model_m_v100` - `complex_model_l_v100` Finally, if you want to use a TPU for training, specify `cloud_tpu` in this field. Learn more about the [special configuration options for training with TPU](https://cloud.google.com/ai-platform/training/docs/using-tpus#configuring_a_custom_tpu_machine).
    #[serde(rename="masterType")]
    
    pub master_type: Option<String>,
    /// Path to the notebook folder to write to. Must be in a Google Cloud Storage bucket path. Format: `gs://{bucket_name}/{folder}` Ex: `gs://notebook_user/scheduled_notebooks`
    #[serde(rename="outputNotebookFolder")]
    
    pub output_notebook_folder: Option<String>,
    /// Parameters used within the 'input_notebook_file' notebook.
    
    pub parameters: Option<String>,
    /// Parameters to be overridden in the notebook during execution. Ref https://papermill.readthedocs.io/en/latest/usage-parameterize.html on how to specifying parameters in the input notebook and pass them here in an YAML file. Ex: `gs://notebook_user/scheduled_notebooks/sentiment_notebook_params.yaml`
    #[serde(rename="paramsYamlFile")]
    
    pub params_yaml_file: Option<String>,
    /// Required. Scale tier of the hardware used for notebook execution. DEPRECATED Will be discontinued. As right now only CUSTOM is supported.
    #[serde(rename="scaleTier")]
    
    pub scale_tier: Option<String>,
    /// The email address of a service account to use when running the execution. You must have the `iam.serviceAccounts.actAs` permission for the specified service account.
    #[serde(rename="serviceAccount")]
    
    pub service_account: Option<String>,
    /// The name of a Vertex AI [Tensorboard] resource to which this execution will upload Tensorboard logs. Format: `projects/{project}/locations/{location}/tensorboards/{tensorboard}`
    
    pub tensorboard: Option<String>,
    /// Parameters used in Vertex AI JobType executions.
    #[serde(rename="vertexAiParameters")]
    
    pub vertex_ai_parameters: Option<VertexAIParameters>,
}

impl client::Part for ExecutionTemplate {}


/// Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language. The syntax and semantics of CEL are documented at https://github.com/google/cel-spec. Example (Comparison): title: "Summary size limit" description: "Determines if a summary is less than 100 chars" expression: "document.summary.size() < 100" Example (Equality): title: "Requestor is owner" description: "Determines if requestor is the document owner" expression: "document.owner == request.auth.claims.email" Example (Logic): title: "Public documents" description: "Determine whether the document should be publicly visible" expression: "document.type != 'private' && document.type != 'internal'" Example (Data Manipulation): title: "Notification string" description: "Create a notification string with a timestamp." expression: "'New message received at ' + string(document.create_time)" The exact variables and functions that may be referenced within an expression are determined by the service that evaluates it. See the service documentation for additional information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Expr {
    /// Optional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI.
    
    pub description: Option<String>,
    /// Textual representation of an expression in Common Expression Language syntax.
    
    pub expression: Option<String>,
    /// Optional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file.
    
    pub location: Option<String>,
    /// Optional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression.
    
    pub title: Option<String>,
}

impl client::Part for Expr {}


/// Response for checking if a notebook instance is healthy.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances get instance health projects](ProjectLocationInstanceGetInstanceHealthCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetInstanceHealthResponse {
    /// Output only. Additional information about instance health. Example: healthInfo": { "docker_proxy_agent_status": "1", "docker_status": "1", "jupyterlab_api_status": "-1", "jupyterlab_status": "-1", "updated": "2020-10-18 09:40:03.573409" }
    #[serde(rename="healthInfo")]
    
    pub health_info: Option<HashMap<String, String>>,
    /// Output only. Runtime health_state.
    #[serde(rename="healthState")]
    
    pub health_state: Option<String>,
}

impl client::ResponseResult for GetInstanceHealthResponse {}


/// Guest OS features for boot disk.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GuestOsFeature {
    /// The ID of a supported feature. Read Enabling guest operating system features to see a list of available options. Valid values: * FEATURE_TYPE_UNSPECIFIED * MULTI_IP_SUBNET * SECURE_BOOT * UEFI_COMPATIBLE * VIRTIO_SCSI_MULTIQUEUE * WINDOWS
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GuestOsFeature {}


/// The definition of a notebook instance.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances create projects](ProjectLocationInstanceCreateCall) (request)
/// * [locations instances get projects](ProjectLocationInstanceGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Instance {
    /// The hardware accelerator used on this instance. If you use accelerators, make sure that your configuration has [enough vCPUs and memory to support the `machine_type` you have selected](https://cloud.google.com/compute/docs/gpus/#gpus-list).
    #[serde(rename="acceleratorConfig")]
    
    pub accelerator_config: Option<AcceleratorConfig>,
    /// Input only. The size of the boot disk in GB attached to this instance, up to a maximum of 64000 GB (64 TB). The minimum recommended value is 100 GB. If not specified, this defaults to 100.
    #[serde(rename="bootDiskSizeGb")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub boot_disk_size_gb: Option<i64>,
    /// Input only. The type of the boot disk attached to this instance, defaults to standard persistent disk (`PD_STANDARD`).
    #[serde(rename="bootDiskType")]
    
    pub boot_disk_type: Option<String>,
    /// Optional. Flag to enable ip forwarding or not, default false/off. https://cloud.google.com/vpc/docs/using-routes#canipforward
    #[serde(rename="canIpForward")]
    
    pub can_ip_forward: Option<bool>,
    /// Use a container image to start the notebook instance.
    #[serde(rename="containerImage")]
    
    pub container_image: Option<ContainerImage>,
    /// Output only. Instance creation time.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Email address of entity that sent original CreateInstance request.
    
    pub creator: Option<String>,
    /// Specify a custom Cloud Storage path where the GPU driver is stored. If not specified, we'll automatically choose from official GPU drivers.
    #[serde(rename="customGpuDriverPath")]
    
    pub custom_gpu_driver_path: Option<String>,
    /// Input only. The size of the data disk in GB attached to this instance, up to a maximum of 64000 GB (64 TB). You can choose the size of the data disk based on how big your notebooks and data are. If not specified, this defaults to 100.
    #[serde(rename="dataDiskSizeGb")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub data_disk_size_gb: Option<i64>,
    /// Input only. The type of the data disk attached to this instance, defaults to standard persistent disk (`PD_STANDARD`).
    #[serde(rename="dataDiskType")]
    
    pub data_disk_type: Option<String>,
    /// Input only. Disk encryption method used on the boot and data disks, defaults to GMEK.
    #[serde(rename="diskEncryption")]
    
    pub disk_encryption: Option<String>,
    /// Output only. Attached disks to notebook instance.
    
    pub disks: Option<Vec<Disk>>,
    /// Whether the end user authorizes Google Cloud to install GPU driver on this instance. If this field is empty or set to false, the GPU driver won't be installed. Only applicable to instances with GPUs.
    #[serde(rename="installGpuDriver")]
    
    pub install_gpu_driver: Option<bool>,
    /// Input only. The owner of this instance after creation. Format: `alias@example.com` Currently supports one owner only. If not specified, all of the service account users of your VM instance's service account can use the instance.
    #[serde(rename="instanceOwners")]
    
    pub instance_owners: Option<Vec<String>>,
    /// Input only. The KMS key used to encrypt the disks, only applicable if disk_encryption is CMEK. Format: `projects/{project_id}/locations/{location}/keyRings/{key_ring_id}/cryptoKeys/{key_id}` Learn more about [using your own encryption keys](https://cloud.google.com/kms/docs/quickstart).
    #[serde(rename="kmsKey")]
    
    pub kms_key: Option<String>,
    /// Labels to apply to this instance. These can be later modified by the setLabels method.
    
    pub labels: Option<HashMap<String, String>>,
    /// Required. The [Compute Engine machine type](https://cloud.google.com/compute/docs/machine-types) of this instance.
    #[serde(rename="machineType")]
    
    pub machine_type: Option<String>,
    /// Custom metadata to apply to this instance.
    
    pub metadata: Option<HashMap<String, String>>,
    /// Output only. The name of this notebook instance. Format: `projects/{project_id}/locations/{location}/instances/{instance_id}`
    
    pub name: Option<String>,
    /// The name of the VPC that this instance is in. Format: `projects/{project_id}/global/networks/{network_id}`
    
    pub network: Option<String>,
    /// Optional. The type of vNIC to be used on this interface. This may be gVNIC or VirtioNet.
    #[serde(rename="nicType")]
    
    pub nic_type: Option<String>,
    /// If true, the notebook instance will not register with the proxy.
    #[serde(rename="noProxyAccess")]
    
    pub no_proxy_access: Option<bool>,
    /// If true, no public IP will be assigned to this instance.
    #[serde(rename="noPublicIp")]
    
    pub no_public_ip: Option<bool>,
    /// Input only. If true, the data disk will not be auto deleted when deleting the instance.
    #[serde(rename="noRemoveDataDisk")]
    
    pub no_remove_data_disk: Option<bool>,
    /// Path to a Bash script that automatically runs after a notebook instance fully boots up. The path must be a URL or Cloud Storage path (`gs://path-to-file/file-name`).
    #[serde(rename="postStartupScript")]
    
    pub post_startup_script: Option<String>,
    /// Output only. The proxy endpoint that is used to access the Jupyter notebook.
    #[serde(rename="proxyUri")]
    
    pub proxy_uri: Option<String>,
    /// Optional. The optional reservation affinity. Setting this field will apply the specified [Zonal Compute Reservation](https://cloud.google.com/compute/docs/instances/reserving-zonal-resources) to this notebook instance.
    #[serde(rename="reservationAffinity")]
    
    pub reservation_affinity: Option<ReservationAffinity>,
    /// The service account on this instance, giving access to other Google Cloud services. You can use any service account within the same project, but you must have the service account user permission to use the instance. If not specified, the [Compute Engine default service account](https://cloud.google.com/compute/docs/access/service-accounts#default_service_account) is used.
    #[serde(rename="serviceAccount")]
    
    pub service_account: Option<String>,
    /// Optional. The URIs of service account scopes to be included in Compute Engine instances. If not specified, the following [scopes](https://cloud.google.com/compute/docs/access/service-accounts#accesscopesiam) are defined: - https://www.googleapis.com/auth/cloud-platform - https://www.googleapis.com/auth/userinfo.email If not using default scopes, you need at least: https://www.googleapis.com/auth/compute
    #[serde(rename="serviceAccountScopes")]
    
    pub service_account_scopes: Option<Vec<String>>,
    /// Optional. Shielded VM configuration. [Images using supported Shielded VM features](https://cloud.google.com/compute/docs/instances/modifying-shielded-vm).
    #[serde(rename="shieldedInstanceConfig")]
    
    pub shielded_instance_config: Option<ShieldedInstanceConfig>,
    /// Output only. The state of this instance.
    
    pub state: Option<String>,
    /// The name of the subnet that this instance is in. Format: `projects/{project_id}/regions/{region}/subnetworks/{subnetwork_id}`
    
    pub subnet: Option<String>,
    /// Optional. The Compute Engine tags to add to runtime (see [Tagging instances](https://cloud.google.com/compute/docs/label-or-tag-resources#tags)).
    
    pub tags: Option<Vec<String>>,
    /// Output only. Instance update time.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The upgrade history of this instance.
    #[serde(rename="upgradeHistory")]
    
    pub upgrade_history: Option<Vec<UpgradeHistoryEntry>>,
    /// Use a Compute Engine VM image to start the notebook instance.
    #[serde(rename="vmImage")]
    
    pub vm_image: Option<VmImage>,
}

impl client::RequestValue for Instance {}
impl client::ResponseResult for Instance {}


/// Notebook instance configurations that can be updated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstanceConfig {
    /// Verifies core internal services are running.
    #[serde(rename="enableHealthMonitoring")]
    
    pub enable_health_monitoring: Option<bool>,
    /// Cron expression in UTC timezone, used to schedule instance auto upgrade. Please follow the [cron format](https://en.wikipedia.org/wiki/Cron).
    #[serde(rename="notebookUpgradeSchedule")]
    
    pub notebook_upgrade_schedule: Option<String>,
}

impl client::Part for InstanceConfig {}


/// Response for checking if a notebook instance is upgradeable.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances is upgradeable projects](ProjectLocationInstanceIsUpgradeableCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IsInstanceUpgradeableResponse {
    /// The new image self link this instance will be upgraded to if calling the upgrade endpoint. This field will only be populated if field upgradeable is true.
    #[serde(rename="upgradeImage")]
    
    pub upgrade_image: Option<String>,
    /// Additional information about upgrade.
    #[serde(rename="upgradeInfo")]
    
    pub upgrade_info: Option<String>,
    /// The version this instance will be upgraded to if calling the upgrade endpoint. This field will only be populated if field upgradeable is true.
    #[serde(rename="upgradeVersion")]
    
    pub upgrade_version: Option<String>,
    /// If an instance is upgradeable.
    
    pub upgradeable: Option<bool>,
}

impl client::ResponseResult for IsInstanceUpgradeableResponse {}


/// Response for listing environments.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations environments list projects](ProjectLocationEnvironmentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListEnvironmentsResponse {
    /// A list of returned environments.
    
    pub environments: Option<Vec<Environment>>,
    /// A page token that can be used to continue listing from the last result in the next list call.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListEnvironmentsResponse {}


/// Response for listing scheduled notebook executions
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations executions list projects](ProjectLocationExecutionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListExecutionsResponse {
    /// A list of returned instances.
    
    pub executions: Option<Vec<Execution>>,
    /// Page token that can be used to continue listing from the last result in the next list call.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Executions IDs that could not be reached. For example: ['projects/{project_id}/location/{location}/executions/imagenet_test1', 'projects/{project_id}/location/{location}/executions/classifier_train1']
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListExecutionsResponse {}


/// Response for listing notebook instances.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances list projects](ProjectLocationInstanceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListInstancesResponse {
    /// A list of returned instances.
    
    pub instances: Option<Vec<Instance>>,
    /// Page token that can be used to continue listing from the last result in the next list call.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached. For example, ['us-west1-a', 'us-central1-b']. A ListInstancesResponse will only contain either instances or unreachables,
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListInstancesResponse {}


/// The response message for Locations.ListLocations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations list projects](ProjectLocationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListLocationsResponse {
    /// A list of locations that matches the specified filter in the request.
    
    pub locations: Option<Vec<Location>>,
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListLocationsResponse {}


/// The response message for Operations.ListOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations list projects](ProjectLocationOperationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListOperationsResponse {
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of operations that matches the specified filter in the request.
    
    pub operations: Option<Vec<Operation>>,
}

impl client::ResponseResult for ListOperationsResponse {}


/// Response for listing Managed Notebook Runtimes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations runtimes list projects](ProjectLocationRuntimeListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListRuntimesResponse {
    /// Page token that can be used to continue listing from the last result in the next list call.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of returned Runtimes.
    
    pub runtimes: Option<Vec<Runtime>>,
    /// Locations that could not be reached. For example, ['us-west1', 'us-central1']. A ListRuntimesResponse will only contain either runtimes or unreachables,
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListRuntimesResponse {}


/// Response for listing scheduled notebook job.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations schedules list projects](ProjectLocationScheduleListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListSchedulesResponse {
    /// Page token that can be used to continue listing from the last result in the next list call.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of returned instances.
    
    pub schedules: Option<Vec<Schedule>>,
    /// Schedules that could not be reached. For example: ['projects/{project_id}/location/{location}/schedules/monthly_digest', 'projects/{project_id}/location/{location}/schedules/weekly_sentiment']
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListSchedulesResponse {}


/// A Local attached disk resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocalDisk {
    /// Optional. Output only. Specifies whether the disk will be auto-deleted when the instance is deleted (but not when the disk is detached from the instance).
    #[serde(rename="autoDelete")]
    
    pub auto_delete: Option<bool>,
    /// Optional. Output only. Indicates that this is a boot disk. The virtual machine will use the first partition of the disk for its root filesystem.
    
    pub boot: Option<bool>,
    /// Optional. Output only. Specifies a unique device name of your choice that is reflected into the /dev/disk/by-id/google-* tree of a Linux operating system running within the instance. This name can be used to reference the device for mounting, resizing, and so on, from within the instance. If not specified, the server chooses a default device name to apply to this disk, in the form persistent-disk-x, where x is a number assigned by Google Compute Engine. This field is only applicable for persistent disks.
    #[serde(rename="deviceName")]
    
    pub device_name: Option<String>,
    /// Output only. Indicates a list of features to enable on the guest operating system. Applicable only for bootable images. Read Enabling guest operating system features to see a list of available options.
    #[serde(rename="guestOsFeatures")]
    
    pub guest_os_features: Option<Vec<RuntimeGuestOsFeature>>,
    /// Output only. A zero-based index to this disk, where 0 is reserved for the boot disk. If you have many disks attached to an instance, each disk would have a unique index number.
    
    pub index: Option<i32>,
    /// Input only. Specifies the parameters for a new disk that will be created alongside the new instance. Use initialization parameters to create boot disks or local SSDs attached to the new instance. This property is mutually exclusive with the source property; you can only define one or the other, but not both.
    #[serde(rename="initializeParams")]
    
    pub initialize_params: Option<LocalDiskInitializeParams>,
    /// Specifies the disk interface to use for attaching this disk, which is either SCSI or NVME. The default is SCSI. Persistent disks must always use SCSI and the request will fail if you attempt to attach a persistent disk in any other format than SCSI. Local SSDs can use either NVME or SCSI. For performance characteristics of SCSI over NVMe, see Local SSD performance. Valid values: * NVME * SCSI
    
    pub interface: Option<String>,
    /// Output only. Type of the resource. Always compute#attachedDisk for attached disks.
    
    pub kind: Option<String>,
    /// Output only. Any valid publicly visible licenses.
    
    pub licenses: Option<Vec<String>>,
    /// The mode in which to attach this disk, either READ_WRITE or READ_ONLY. If not specified, the default is to attach the disk in READ_WRITE mode. Valid values: * READ_ONLY * READ_WRITE
    
    pub mode: Option<String>,
    /// Specifies a valid partial or full URL to an existing Persistent Disk resource.
    
    pub source: Option<String>,
    /// Specifies the type of the disk, either SCRATCH or PERSISTENT. If not specified, the default is PERSISTENT. Valid values: * PERSISTENT * SCRATCH
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for LocalDisk {}


/// Input only. Specifies the parameters for a new disk that will be created alongside the new instance. Use initialization parameters to create boot disks or local SSDs attached to the new runtime. This property is mutually exclusive with the source property; you can only define one or the other, but not both.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocalDiskInitializeParams {
    /// Optional. Provide this property when creating the disk.
    
    pub description: Option<String>,
    /// Optional. Specifies the disk name. If not specified, the default is to use the name of the instance. If the disk with the instance name exists already in the given zone/region, a new name will be automatically generated.
    #[serde(rename="diskName")]
    
    pub disk_name: Option<String>,
    /// Optional. Specifies the size of the disk in base-2 GB. If not specified, the disk will be the same size as the image (usually 10GB). If specified, the size must be equal to or larger than 10GB. Default 100 GB.
    #[serde(rename="diskSizeGb")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub disk_size_gb: Option<i64>,
    /// Input only. The type of the boot disk attached to this instance, defaults to standard persistent disk (`PD_STANDARD`).
    #[serde(rename="diskType")]
    
    pub disk_type: Option<String>,
    /// Optional. Labels to apply to this disk. These can be later modified by the disks.setLabels method. This field is only applicable for persistent disks.
    
    pub labels: Option<HashMap<String, String>>,
}

impl client::Part for LocalDiskInitializeParams {}


/// A resource that represents Google Cloud Platform location.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations get projects](ProjectLocationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    /// The friendly name for this location, typically a nearby city name. For example, "Tokyo".
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"}
    
    pub labels: Option<HashMap<String, String>>,
    /// The canonical id for this location. For example: `"us-east1"`.
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
    /// Service-specific metadata. For example the available capacity at the given location.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"`
    
    pub name: Option<String>,
}

impl client::ResponseResult for Location {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations environments create projects](ProjectLocationEnvironmentCreateCall) (response)
/// * [locations environments delete projects](ProjectLocationEnvironmentDeleteCall) (response)
/// * [locations executions create projects](ProjectLocationExecutionCreateCall) (response)
/// * [locations executions delete projects](ProjectLocationExecutionDeleteCall) (response)
/// * [locations instances create projects](ProjectLocationInstanceCreateCall) (response)
/// * [locations instances delete projects](ProjectLocationInstanceDeleteCall) (response)
/// * [locations instances diagnose projects](ProjectLocationInstanceDiagnoseCall) (response)
/// * [locations instances register projects](ProjectLocationInstanceRegisterCall) (response)
/// * [locations instances report projects](ProjectLocationInstanceReportCall) (response)
/// * [locations instances reset projects](ProjectLocationInstanceResetCall) (response)
/// * [locations instances rollback projects](ProjectLocationInstanceRollbackCall) (response)
/// * [locations instances set accelerator projects](ProjectLocationInstanceSetAcceleratorCall) (response)
/// * [locations instances set labels projects](ProjectLocationInstanceSetLabelCall) (response)
/// * [locations instances set machine type projects](ProjectLocationInstanceSetMachineTypeCall) (response)
/// * [locations instances start projects](ProjectLocationInstanceStartCall) (response)
/// * [locations instances stop projects](ProjectLocationInstanceStopCall) (response)
/// * [locations instances update config projects](ProjectLocationInstanceUpdateConfigCall) (response)
/// * [locations instances update shielded instance config projects](ProjectLocationInstanceUpdateShieldedInstanceConfigCall) (response)
/// * [locations instances upgrade projects](ProjectLocationInstanceUpgradeCall) (response)
/// * [locations instances upgrade internal projects](ProjectLocationInstanceUpgradeInternalCall) (response)
/// * [locations operations get projects](ProjectLocationOperationGetCall) (response)
/// * [locations runtimes create projects](ProjectLocationRuntimeCreateCall) (response)
/// * [locations runtimes delete projects](ProjectLocationRuntimeDeleteCall) (response)
/// * [locations runtimes diagnose projects](ProjectLocationRuntimeDiagnoseCall) (response)
/// * [locations runtimes patch projects](ProjectLocationRuntimePatchCall) (response)
/// * [locations runtimes report event projects](ProjectLocationRuntimeReportEventCall) (response)
/// * [locations runtimes reset projects](ProjectLocationRuntimeResetCall) (response)
/// * [locations runtimes start projects](ProjectLocationRuntimeStartCall) (response)
/// * [locations runtimes stop projects](ProjectLocationRuntimeStopCall) (response)
/// * [locations runtimes switch projects](ProjectLocationRuntimeSwitchCall) (response)
/// * [locations runtimes upgrade projects](ProjectLocationRuntimeUpgradeCall) (response)
/// * [locations schedules create projects](ProjectLocationScheduleCreateCall) (response)
/// * [locations schedules delete projects](ProjectLocationScheduleDeleteCall) (response)
/// * [locations schedules trigger projects](ProjectLocationScheduleTriggerCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available.
    
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    
    pub error: Option<Status>,
    /// Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`.
    
    pub name: Option<String>,
    /// The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`.
    
    pub response: Option<HashMap<String, json::Value>>,
}

impl client::ResponseResult for Operation {}


/// An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members`, or principals, to a single `role`. Principals can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { “bindings”: \[ { “role”: “roles/resourcemanager.organizationAdmin”, “members”: \[ “user:mike@example.com”, “group:admins@example.com”, “domain:google.com”, “serviceAccount:my-project-id@appspot.gserviceaccount.com” \] }, { “role”: “roles/resourcemanager.organizationViewer”, “members”: \[ “user:eve@example.com” \], “condition”: { “title”: “expirable access”, “description”: “Does not grant access after Sep 2020”, “expression”: “request.time \< timestamp(‘2020-10-01T00:00:00.000Z’)”, } } \], “etag”: “BwWWja0YfJA=”, “version”: 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time \< timestamp(‘2020-10-01T00:00:00.000Z’) etag: BwWWja0YfJA= version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances get iam policy projects](ProjectLocationInstanceGetIamPolicyCall) (response)
/// * [locations instances set iam policy projects](ProjectLocationInstanceSetIamPolicyCall) (response)
/// * [locations runtimes get iam policy projects](ProjectLocationRuntimeGetIamPolicyCall) (response)
/// * [locations runtimes set iam policy projects](ProjectLocationRuntimeSetIamPolicyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Policy {
    /// Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`.
    
    pub bindings: Option<Vec<Binding>>,
    /// `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
    /// Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub version: Option<i32>,
}

impl client::ResponseResult for Policy {}


/// Request for getting a new access token.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations runtimes refresh runtime token internal projects](ProjectLocationRuntimeRefreshRuntimeTokenInternalCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RefreshRuntimeTokenInternalRequest {
    /// Required. The VM hardware token for authenticating the VM. https://cloud.google.com/compute/docs/instances/verifying-instance-identity
    #[serde(rename="vmId")]
    
    pub vm_id: Option<String>,
}

impl client::RequestValue for RefreshRuntimeTokenInternalRequest {}


/// Response with a new access token.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations runtimes refresh runtime token internal projects](ProjectLocationRuntimeRefreshRuntimeTokenInternalCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RefreshRuntimeTokenInternalResponse {
    /// The OAuth 2.0 access token.
    #[serde(rename="accessToken")]
    
    pub access_token: Option<String>,
    /// Output only. Token expiration time.
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for RefreshRuntimeTokenInternalResponse {}


/// Request for registering a notebook instance.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances register projects](ProjectLocationInstanceRegisterCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RegisterInstanceRequest {
    /// Required. User defined unique ID of this instance. The `instance_id` must be 1 to 63 characters long and contain only lowercase letters, numeric characters, and dashes. The first character must be a lowercase letter and the last character cannot be a dash.
    #[serde(rename="instanceId")]
    
    pub instance_id: Option<String>,
}

impl client::RequestValue for RegisterInstanceRequest {}


/// Request for notebook instances to report information to Notebooks API.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances report projects](ProjectLocationInstanceReportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportInstanceInfoRequest {
    /// The metadata reported to Notebooks API. This will be merged to the instance metadata store
    
    pub metadata: Option<HashMap<String, String>>,
    /// Required. The VM hardware token for authenticating the VM. https://cloud.google.com/compute/docs/instances/verifying-instance-identity
    #[serde(rename="vmId")]
    
    pub vm_id: Option<String>,
}

impl client::RequestValue for ReportInstanceInfoRequest {}


/// Request for reporting a Managed Notebook Event.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations runtimes report event projects](ProjectLocationRuntimeReportEventCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportRuntimeEventRequest {
    /// Required. The Event to be reported.
    
    pub event: Option<Event>,
    /// Required. The VM hardware token for authenticating the VM. https://cloud.google.com/compute/docs/instances/verifying-instance-identity
    #[serde(rename="vmId")]
    
    pub vm_id: Option<String>,
}

impl client::RequestValue for ReportRuntimeEventRequest {}


/// Reservation Affinity for consuming Zonal reservation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReservationAffinity {
    /// Optional. Type of reservation to consume
    #[serde(rename="consumeReservationType")]
    
    pub consume_reservation_type: Option<String>,
    /// Optional. Corresponds to the label key of reservation resource.
    
    pub key: Option<String>,
    /// Optional. Corresponds to the label values of reservation resource.
    
    pub values: Option<Vec<String>>,
}

impl client::Part for ReservationAffinity {}


/// Request for resetting a notebook instance
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances reset projects](ProjectLocationInstanceResetCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResetInstanceRequest { _never_set: Option<bool> }

impl client::RequestValue for ResetInstanceRequest {}


/// Request for resetting a Managed Notebook Runtime.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations runtimes reset projects](ProjectLocationRuntimeResetCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResetRuntimeRequest {
    /// Idempotent request UUID.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::RequestValue for ResetRuntimeRequest {}


/// Request for rollbacking a notebook instance
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances rollback projects](ProjectLocationInstanceRollbackCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RollbackInstanceRequest {
    /// Required. The snapshot for rollback. Example: "projects/test-project/global/snapshots/krwlzipynril".
    #[serde(rename="targetSnapshot")]
    
    pub target_snapshot: Option<String>,
}

impl client::RequestValue for RollbackInstanceRequest {}


/// The definition of a Runtime for a managed notebook instance.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations runtimes create projects](ProjectLocationRuntimeCreateCall) (request)
/// * [locations runtimes get projects](ProjectLocationRuntimeGetCall) (response)
/// * [locations runtimes patch projects](ProjectLocationRuntimePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Runtime {
    /// The config settings for accessing runtime.
    #[serde(rename="accessConfig")]
    
    pub access_config: Option<RuntimeAccessConfig>,
    /// Output only. Runtime creation time.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Runtime health_state.
    #[serde(rename="healthState")]
    
    pub health_state: Option<String>,
    /// Output only. Contains Runtime daemon metrics such as Service status and JupyterLab stats.
    
    pub metrics: Option<RuntimeMetrics>,
    /// Output only. The resource name of the runtime. Format: `projects/{project}/locations/{location}/runtimes/{runtimeId}`
    
    pub name: Option<String>,
    /// The config settings for software inside the runtime.
    #[serde(rename="softwareConfig")]
    
    pub software_config: Option<RuntimeSoftwareConfig>,
    /// Output only. Runtime state.
    
    pub state: Option<String>,
    /// Output only. Runtime update time.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Use a Compute Engine VM image to start the managed notebook instance.
    #[serde(rename="virtualMachine")]
    
    pub virtual_machine: Option<VirtualMachine>,
}

impl client::RequestValue for Runtime {}
impl client::ResponseResult for Runtime {}


/// Definition of the types of hardware accelerators that can be used. Definition of the types of hardware accelerators that can be used. See [Compute Engine AcceleratorTypes](https://cloud.google.com/compute/docs/reference/beta/acceleratorTypes). Examples: * `nvidia-tesla-k80` * `nvidia-tesla-p100` * `nvidia-tesla-v100` * `nvidia-tesla-p4` * `nvidia-tesla-t4` * `nvidia-tesla-a100`
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeAcceleratorConfig {
    /// Count of cores of this accelerator.
    #[serde(rename="coreCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub core_count: Option<i64>,
    /// Accelerator model.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for RuntimeAcceleratorConfig {}


/// Specifies the login configuration for Runtime
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeAccessConfig {
    /// The type of access mode this instance.
    #[serde(rename="accessType")]
    
    pub access_type: Option<String>,
    /// Output only. The proxy endpoint that is used to access the runtime.
    #[serde(rename="proxyUri")]
    
    pub proxy_uri: Option<String>,
    /// The owner of this runtime after creation. Format: `alias@example.com` Currently supports one owner only.
    #[serde(rename="runtimeOwner")]
    
    pub runtime_owner: Option<String>,
}

impl client::Part for RuntimeAccessConfig {}


/// Optional. A list of features to enable on the guest operating system. Applicable only for bootable images. Read [Enabling guest operating system features](https://cloud.google.com/compute/docs/images/create-delete-deprecate-private-images#guest-os-features) to see a list of available options. Guest OS features for boot disk.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeGuestOsFeature {
    /// The ID of a supported feature. Read [Enabling guest operating system features](https://cloud.google.com/compute/docs/images/create-delete-deprecate-private-images#guest-os-features) to see a list of available options. Valid values: * FEATURE_TYPE_UNSPECIFIED * MULTI_IP_SUBNET * SECURE_BOOT * UEFI_COMPATIBLE * VIRTIO_SCSI_MULTIQUEUE * WINDOWS
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for RuntimeGuestOsFeature {}


/// Contains runtime daemon metrics, such as OS and kernels and sessions stats.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeMetrics {
    /// Output only. The system metrics.
    #[serde(rename="systemMetrics")]
    
    pub system_metrics: Option<HashMap<String, String>>,
}

impl client::Part for RuntimeMetrics {}


/// A set of Shielded Instance options. Check [Images using supported Shielded VM features](https://cloud.google.com/compute/docs/instances/modifying-shielded-vm). Not all combinations are valid.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeShieldedInstanceConfig {
    /// Defines whether the instance has integrity monitoring enabled. Enables monitoring and attestation of the boot integrity of the instance. The attestation is performed against the integrity policy baseline. This baseline is initially derived from the implicitly trusted boot image when the instance is created. Enabled by default.
    #[serde(rename="enableIntegrityMonitoring")]
    
    pub enable_integrity_monitoring: Option<bool>,
    /// Defines whether the instance has Secure Boot enabled. Secure Boot helps ensure that the system only runs authentic software by verifying the digital signature of all boot components, and halting the boot process if signature verification fails. Disabled by default.
    #[serde(rename="enableSecureBoot")]
    
    pub enable_secure_boot: Option<bool>,
    /// Defines whether the instance has the vTPM enabled. Enabled by default.
    #[serde(rename="enableVtpm")]
    
    pub enable_vtpm: Option<bool>,
}

impl client::Part for RuntimeShieldedInstanceConfig {}


/// Specifies the selection and configuration of software inside the runtime. The properties to set on runtime. Properties keys are specified in `key:value` format, for example: * `idle_shutdown: true` * `idle_shutdown_timeout: 180` * `enable_health_monitoring: true`
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeSoftwareConfig {
    /// Specify a custom Cloud Storage path where the GPU driver is stored. If not specified, we'll automatically choose from official GPU drivers.
    #[serde(rename="customGpuDriverPath")]
    
    pub custom_gpu_driver_path: Option<String>,
    /// Bool indicating whether JupyterLab terminal will be available or not. Default: False
    #[serde(rename="disableTerminal")]
    
    pub disable_terminal: Option<bool>,
    /// Verifies core internal services are running. Default: True
    #[serde(rename="enableHealthMonitoring")]
    
    pub enable_health_monitoring: Option<bool>,
    /// Runtime will automatically shutdown after idle_shutdown_time. Default: True
    #[serde(rename="idleShutdown")]
    
    pub idle_shutdown: Option<bool>,
    /// Time in minutes to wait before shutting down runtime. Default: 180 minutes
    #[serde(rename="idleShutdownTimeout")]
    
    pub idle_shutdown_timeout: Option<i32>,
    /// Install Nvidia Driver automatically. Default: True
    #[serde(rename="installGpuDriver")]
    
    pub install_gpu_driver: Option<bool>,
    /// Optional. Use a list of container images to use as Kernels in the notebook instance.
    
    pub kernels: Option<Vec<ContainerImage>>,
    /// Cron expression in UTC timezone, used to schedule instance auto upgrade. Please follow the [cron format](https://en.wikipedia.org/wiki/Cron).
    #[serde(rename="notebookUpgradeSchedule")]
    
    pub notebook_upgrade_schedule: Option<String>,
    /// Path to a Bash script that automatically runs after a notebook instance fully boots up. The path must be a URL or Cloud Storage path (`gs://path-to-file/file-name`).
    #[serde(rename="postStartupScript")]
    
    pub post_startup_script: Option<String>,
    /// Behavior for the post startup script.
    #[serde(rename="postStartupScriptBehavior")]
    
    pub post_startup_script_behavior: Option<String>,
    /// Output only. Bool indicating whether an newer image is available in an image family.
    
    pub upgradeable: Option<bool>,
    /// Output only. version of boot image such as M100, from release label of the image.
    
    pub version: Option<String>,
}

impl client::Part for RuntimeSoftwareConfig {}


/// The definition of a schedule.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations schedules create projects](ProjectLocationScheduleCreateCall) (request)
/// * [locations schedules get projects](ProjectLocationScheduleGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Schedule {
    /// Output only. Time the schedule was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Cron-tab formatted schedule by which the job will execute. Format: minute, hour, day of month, month, day of week, e.g. 0 0 * * WED = every Wednesday More examples: https://crontab.guru/examples.html
    #[serde(rename="cronSchedule")]
    
    pub cron_schedule: Option<String>,
    /// A brief description of this environment.
    
    pub description: Option<String>,
    /// Output only. Display name used for UI purposes. Name can only contain alphanumeric characters, hyphens '-', and underscores '_'.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Notebook Execution Template corresponding to this schedule.
    #[serde(rename="executionTemplate")]
    
    pub execution_template: Option<ExecutionTemplate>,
    /// Output only. The name of this schedule. Format: `projects/{project_id}/locations/{location}/schedules/{schedule_id}`
    
    pub name: Option<String>,
    /// Output only. The most recent execution names triggered from this schedule and their corresponding states.
    #[serde(rename="recentExecutions")]
    
    pub recent_executions: Option<Vec<Execution>>,
    /// no description provided
    
    pub state: Option<String>,
    /// Timezone on which the cron_schedule. The value of this field must be a time zone name from the tz database. TZ Database: https://en.wikipedia.org/wiki/List_of_tz_database_time_zones Note that some time zones include a provision for daylight savings time. The rules for daylight saving time are determined by the chosen tz. For UTC use the string "utc". If a time zone is not specified, the default will be in UTC (also known as GMT).
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<String>,
    /// Output only. Time the schedule was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Schedule {}
impl client::ResponseResult for Schedule {}


/// Definition of a hardware accelerator. Note that not all combinations of `type` and `core_count` are valid. Check [GPUs on Compute Engine](https://cloud.google.com/compute/docs/gpus) to find a valid combination. TPUs are not supported.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SchedulerAcceleratorConfig {
    /// Count of cores of this accelerator.
    #[serde(rename="coreCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub core_count: Option<i64>,
    /// Type of this accelerator.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for SchedulerAcceleratorConfig {}


/// Request message for `SetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances set iam policy projects](ProjectLocationInstanceSetIamPolicyCall) (request)
/// * [locations runtimes set iam policy projects](ProjectLocationRuntimeSetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetIamPolicyRequest {
    /// REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them.
    
    pub policy: Option<Policy>,
}

impl client::RequestValue for SetIamPolicyRequest {}


/// Request for setting instance accelerator.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances set accelerator projects](ProjectLocationInstanceSetAcceleratorCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetInstanceAcceleratorRequest {
    /// Required. Count of cores of this accelerator. Note that not all combinations of `type` and `core_count` are valid. Check [GPUs on Compute Engine](https://cloud.google.com/compute/docs/gpus/#gpus-list) to find a valid combination. TPUs are not supported.
    #[serde(rename="coreCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub core_count: Option<i64>,
    /// Required. Type of this accelerator.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::RequestValue for SetInstanceAcceleratorRequest {}


/// Request for setting instance labels.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances set labels projects](ProjectLocationInstanceSetLabelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetInstanceLabelsRequest {
    /// Labels to apply to this instance. These can be later modified by the setLabels method
    
    pub labels: Option<HashMap<String, String>>,
}

impl client::RequestValue for SetInstanceLabelsRequest {}


/// Request for setting instance machine type.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances set machine type projects](ProjectLocationInstanceSetMachineTypeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetInstanceMachineTypeRequest {
    /// Required. The [Compute Engine machine type](https://cloud.google.com/compute/docs/machine-types).
    #[serde(rename="machineType")]
    
    pub machine_type: Option<String>,
}

impl client::RequestValue for SetInstanceMachineTypeRequest {}


/// A set of Shielded Instance options. Check [Images using supported Shielded VM features] Not all combinations are valid.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ShieldedInstanceConfig {
    /// Defines whether the instance has integrity monitoring enabled. Enables monitoring and attestation of the boot integrity of the instance. The attestation is performed against the integrity policy baseline. This baseline is initially derived from the implicitly trusted boot image when the instance is created. Enabled by default.
    #[serde(rename="enableIntegrityMonitoring")]
    
    pub enable_integrity_monitoring: Option<bool>,
    /// Defines whether the instance has Secure Boot enabled. Secure Boot helps ensure that the system only runs authentic software by verifying the digital signature of all boot components, and halting the boot process if signature verification fails. Disabled by default.
    #[serde(rename="enableSecureBoot")]
    
    pub enable_secure_boot: Option<bool>,
    /// Defines whether the instance has the vTPM enabled. Enabled by default.
    #[serde(rename="enableVtpm")]
    
    pub enable_vtpm: Option<bool>,
}

impl client::Part for ShieldedInstanceConfig {}


/// Request for starting a notebook instance
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances start projects](ProjectLocationInstanceStartCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StartInstanceRequest { _never_set: Option<bool> }

impl client::RequestValue for StartInstanceRequest {}


/// Request for starting a Managed Notebook Runtime.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations runtimes start projects](ProjectLocationRuntimeStartCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StartRuntimeRequest {
    /// Idempotent request UUID.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::RequestValue for StartRuntimeRequest {}


/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Status {
    /// The status code, which should be an enum value of google.rpc.Code.
    
    pub code: Option<i32>,
    /// A list of messages that carry the error details. There is a common set of message types for APIs to use.
    
    pub details: Option<Vec<HashMap<String, json::Value>>>,
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client.
    
    pub message: Option<String>,
}

impl client::Part for Status {}


/// Request for stopping a notebook instance
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances stop projects](ProjectLocationInstanceStopCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StopInstanceRequest { _never_set: Option<bool> }

impl client::RequestValue for StopInstanceRequest {}


/// Request for stopping a Managed Notebook Runtime.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations runtimes stop projects](ProjectLocationRuntimeStopCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StopRuntimeRequest {
    /// Idempotent request UUID.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::RequestValue for StopRuntimeRequest {}


/// Request for switching a Managed Notebook Runtime.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations runtimes switch projects](ProjectLocationRuntimeSwitchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SwitchRuntimeRequest {
    /// accelerator config.
    #[serde(rename="acceleratorConfig")]
    
    pub accelerator_config: Option<RuntimeAcceleratorConfig>,
    /// machine type.
    #[serde(rename="machineType")]
    
    pub machine_type: Option<String>,
    /// Idempotent request UUID.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::RequestValue for SwitchRuntimeRequest {}


/// Request message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances test iam permissions projects](ProjectLocationInstanceTestIamPermissionCall) (request)
/// * [locations runtimes test iam permissions projects](ProjectLocationRuntimeTestIamPermissionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsRequest {
    /// The set of permissions to check for the `resource`. Permissions with wildcards (such as `*` or `storage.*`) are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions).
    
    pub permissions: Option<Vec<String>>,
}

impl client::RequestValue for TestIamPermissionsRequest {}


/// Response message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances test iam permissions projects](ProjectLocationInstanceTestIamPermissionCall) (response)
/// * [locations runtimes test iam permissions projects](ProjectLocationRuntimeTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsResponse {
    /// A subset of `TestPermissionsRequest.permissions` that the caller is allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for TestIamPermissionsResponse {}


/// Request for created scheduled notebooks
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations schedules trigger projects](ProjectLocationScheduleTriggerCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TriggerScheduleRequest { _never_set: Option<bool> }

impl client::RequestValue for TriggerScheduleRequest {}


/// Request for updating instance configurations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances update config projects](ProjectLocationInstanceUpdateConfigCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateInstanceConfigRequest {
    /// The instance configurations to be updated.
    
    pub config: Option<InstanceConfig>,
}

impl client::RequestValue for UpdateInstanceConfigRequest {}


/// Request for adding/changing metadata items for an instance.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances update metadata items projects](ProjectLocationInstanceUpdateMetadataItemCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateInstanceMetadataItemsRequest {
    /// Metadata items to add/update for the instance.
    
    pub items: Option<HashMap<String, String>>,
}

impl client::RequestValue for UpdateInstanceMetadataItemsRequest {}


/// Response for adding/changing metadata items for an instance.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances update metadata items projects](ProjectLocationInstanceUpdateMetadataItemCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateInstanceMetadataItemsResponse {
    /// Map of items that were added/updated to/in the metadata.
    
    pub items: Option<HashMap<String, String>>,
}

impl client::ResponseResult for UpdateInstanceMetadataItemsResponse {}


/// Request for updating the Shielded Instance config for a notebook instance. You can only use this method on a stopped instance
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances update shielded instance config projects](ProjectLocationInstanceUpdateShieldedInstanceConfigCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateShieldedInstanceConfigRequest {
    /// ShieldedInstance configuration to be updated.
    #[serde(rename="shieldedInstanceConfig")]
    
    pub shielded_instance_config: Option<ShieldedInstanceConfig>,
}

impl client::RequestValue for UpdateShieldedInstanceConfigRequest {}


/// The entry of VM image upgrade history.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpgradeHistoryEntry {
    /// Action. Rolloback or Upgrade.
    
    pub action: Option<String>,
    /// The container image before this instance upgrade.
    #[serde(rename="containerImage")]
    
    pub container_image: Option<String>,
    /// The time that this instance upgrade history entry is created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The framework of this notebook instance.
    
    pub framework: Option<String>,
    /// The snapshot of the boot disk of this notebook instance before upgrade.
    
    pub snapshot: Option<String>,
    /// The state of this instance upgrade history entry.
    
    pub state: Option<String>,
    /// Target VM Image. Format: ainotebooks-vm/project/image-name/name.
    #[serde(rename="targetImage")]
    
    pub target_image: Option<String>,
    /// Target VM Version, like m63.
    #[serde(rename="targetVersion")]
    
    pub target_version: Option<String>,
    /// The version of the notebook instance before this upgrade.
    
    pub version: Option<String>,
    /// The VM image before this instance upgrade.
    #[serde(rename="vmImage")]
    
    pub vm_image: Option<String>,
}

impl client::Part for UpgradeHistoryEntry {}


/// Request for upgrading a notebook instance from within the VM
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances upgrade internal projects](ProjectLocationInstanceUpgradeInternalCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpgradeInstanceInternalRequest {
    /// Optional. The optional UpgradeType. Setting this field will search for additional compute images to upgrade this instance.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Required. The VM hardware token for authenticating the VM. https://cloud.google.com/compute/docs/instances/verifying-instance-identity
    #[serde(rename="vmId")]
    
    pub vm_id: Option<String>,
}

impl client::RequestValue for UpgradeInstanceInternalRequest {}


/// Request for upgrading a notebook instance
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances upgrade projects](ProjectLocationInstanceUpgradeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpgradeInstanceRequest {
    /// Optional. The optional UpgradeType. Setting this field will search for additional compute images to upgrade this instance.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::RequestValue for UpgradeInstanceRequest {}


/// Request for upgrading a Managed Notebook Runtime to the latest version. option (google.api.message_visibility).restriction = “TRUSTED_TESTER,SPECIAL_TESTER”;
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations runtimes upgrade projects](ProjectLocationRuntimeUpgradeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpgradeRuntimeRequest {
    /// Idempotent request UUID.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::RequestValue for UpgradeRuntimeRequest {}


/// Parameters used in Vertex AI JobType executions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VertexAIParameters {
    /// Environment variables. At most 100 environment variables can be specified and unique. Example: GCP_BUCKET=gs://my-bucket/samples/
    
    pub env: Option<HashMap<String, String>>,
    /// The full name of the Compute Engine [network](https://cloud.google.com/compute/docs/networks-and-firewalls#networks) to which the Job should be peered. For example, `projects/12345/global/networks/myVPC`. [Format](https://cloud.google.com/compute/docs/reference/rest/v1/networks/insert) is of the form `projects/{project}/global/networks/{network}`. Where {project} is a project number, as in `12345`, and {network} is a network name. Private services access must already be configured for the network. If left unspecified, the job is not peered with any network.
    
    pub network: Option<String>,
}

impl client::Part for VertexAIParameters {}


/// Runtime using Virtual Machine for computing.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VirtualMachine {
    /// Output only. The unique identifier of the Managed Compute Engine instance.
    #[serde(rename="instanceId")]
    
    pub instance_id: Option<String>,
    /// Output only. The user-friendly name of the Managed Compute Engine instance.
    #[serde(rename="instanceName")]
    
    pub instance_name: Option<String>,
    /// Virtual Machine configuration settings.
    #[serde(rename="virtualMachineConfig")]
    
    pub virtual_machine_config: Option<VirtualMachineConfig>,
}

impl client::Part for VirtualMachine {}


/// The config settings for virtual machine.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VirtualMachineConfig {
    /// Optional. The Compute Engine accelerator configuration for this runtime.
    #[serde(rename="acceleratorConfig")]
    
    pub accelerator_config: Option<RuntimeAcceleratorConfig>,
    /// Optional. Boot image metadata used for runtime upgradeability.
    #[serde(rename="bootImage")]
    
    pub boot_image: Option<BootImage>,
    /// Optional. Use a list of container images to use as Kernels in the notebook instance.
    #[serde(rename="containerImages")]
    
    pub container_images: Option<Vec<ContainerImage>>,
    /// Required. Data disk option configuration settings.
    #[serde(rename="dataDisk")]
    
    pub data_disk: Option<LocalDisk>,
    /// Optional. Encryption settings for virtual machine data disk.
    #[serde(rename="encryptionConfig")]
    
    pub encryption_config: Option<EncryptionConfig>,
    /// Output only. The Compute Engine guest attributes. (see [Project and instance guest attributes](https://cloud.google.com/compute/docs/storing-retrieving-metadata#guest_attributes)).
    #[serde(rename="guestAttributes")]
    
    pub guest_attributes: Option<HashMap<String, String>>,
    /// Optional. If true, runtime will only have internal IP addresses. By default, runtimes are not restricted to internal IP addresses, and will have ephemeral external IP addresses assigned to each vm. This `internal_ip_only` restriction can only be enabled for subnetwork enabled networks, and all dependencies must be configured to be accessible without external IP addresses.
    #[serde(rename="internalIpOnly")]
    
    pub internal_ip_only: Option<bool>,
    /// Optional. The labels to associate with this runtime. Label **keys** must contain 1 to 63 characters, and must conform to [RFC 1035](https://www.ietf.org/rfc/rfc1035.txt). Label **values** may be empty, but, if present, must contain 1 to 63 characters, and must conform to [RFC 1035](https://www.ietf.org/rfc/rfc1035.txt). No more than 32 labels can be associated with a cluster.
    
    pub labels: Option<HashMap<String, String>>,
    /// Required. The Compute Engine machine type used for runtimes. Short name is valid. Examples: * `n1-standard-2` * `e2-standard-8`
    #[serde(rename="machineType")]
    
    pub machine_type: Option<String>,
    /// Optional. The Compute Engine metadata entries to add to virtual machine. (see [Project and instance metadata](https://cloud.google.com/compute/docs/storing-retrieving-metadata#project_and_instance_metadata)).
    
    pub metadata: Option<HashMap<String, String>>,
    /// Optional. The Compute Engine network to be used for machine communications. Cannot be specified with subnetwork. If neither `network` nor `subnet` is specified, the "default" network of the project is used, if it exists. A full URL or partial URI. Examples: * `https://www.googleapis.com/compute/v1/projects/[project_id]/global/networks/default` * `projects/[project_id]/global/networks/default` Runtimes are managed resources inside Google Infrastructure. Runtimes support the following network configurations: * Google Managed Network (Network & subnet are empty) * Consumer Project VPC (network & subnet are required). Requires configuring Private Service Access. * Shared VPC (network & subnet are required). Requires configuring Private Service Access.
    
    pub network: Option<String>,
    /// Optional. The type of vNIC to be used on this interface. This may be gVNIC or VirtioNet.
    #[serde(rename="nicType")]
    
    pub nic_type: Option<String>,
    /// Optional. Reserved IP Range name is used for VPC Peering. The subnetwork allocation will use the range *name* if it's assigned. Example: managed-notebooks-range-c PEERING_RANGE_NAME_3=managed-notebooks-range-c gcloud compute addresses create $PEERING_RANGE_NAME_3 \ --global \ --prefix-length=24 \ --description="Google Cloud Managed Notebooks Range 24 c" \ --network=$NETWORK \ --addresses=192.168.0.0 \ --purpose=VPC_PEERING Field value will be: `managed-notebooks-range-c`
    #[serde(rename="reservedIpRange")]
    
    pub reserved_ip_range: Option<String>,
    /// Optional. Shielded VM Instance configuration settings.
    #[serde(rename="shieldedInstanceConfig")]
    
    pub shielded_instance_config: Option<RuntimeShieldedInstanceConfig>,
    /// Optional. The Compute Engine subnetwork to be used for machine communications. Cannot be specified with network. A full URL or partial URI are valid. Examples: * `https://www.googleapis.com/compute/v1/projects/[project_id]/regions/us-east1/subnetworks/sub0` * `projects/[project_id]/regions/us-east1/subnetworks/sub0`
    
    pub subnet: Option<String>,
    /// Optional. The Compute Engine tags to add to runtime (see [Tagging instances](https://cloud.google.com/compute/docs/label-or-tag-resources#tags)).
    
    pub tags: Option<Vec<String>>,
    /// Output only. The zone where the virtual machine is located. If using regional request, the notebooks service will pick a location in the corresponding runtime region. On a get request, zone will always be present. Example: * `us-central1-b`
    
    pub zone: Option<String>,
}

impl client::Part for VirtualMachineConfig {}


/// Definition of a custom Compute Engine virtual machine image for starting a notebook instance with the environment installed directly on the VM.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VmImage {
    /// Use this VM image family to find the image; the newest image in this family will be used.
    #[serde(rename="imageFamily")]
    
    pub image_family: Option<String>,
    /// Use VM image name to find the image.
    #[serde(rename="imageName")]
    
    pub image_name: Option<String>,
    /// Required. The name of the Google Cloud project that this VM image belongs to. Format: `{project_id}`
    
    pub project: Option<String>,
}

impl client::Part for VmImage {}


