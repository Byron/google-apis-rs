use super::*;
/// Carries information about an accelerator that can be attached to a VM.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Accelerator {
    /// How many accelerators of this type to attach.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
    /// The accelerator type string (for example, "nvidia-tesla-k80"). Only NVIDIA GPU accelerators are currently supported. If an NVIDIA GPU is attached, the required runtime libraries will be made available to all containers under `/usr/local/nvidia`. The driver version to install must be specified using the NVIDIA driver version parameter on the virtual machine specification. Note that attaching a GPU increases the worker VM startup time by a few minutes.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for Accelerator {}


/// Specifies a single action that runs a Docker container.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Action {
    /// By default, after an action fails, no further actions are run. This flag indicates that this action must be run even if the pipeline has already failed. This is useful for actions that copy output files off of the VM or for debugging. Note that no actions will be run if image prefetching fails.
    #[serde(rename="alwaysRun")]
    
    pub always_run: Option<bool>,
    /// Prevents the container from accessing the external network.
    #[serde(rename="blockExternalNetwork")]
    
    pub block_external_network: Option<bool>,
    /// If specified, overrides the `CMD` specified in the container. If the container also has an `ENTRYPOINT` the values are used as entrypoint arguments. Otherwise, they are used as a command and arguments to run inside the container.
    
    pub commands: Option<Vec<String>>,
    /// An optional name for the container. The container hostname will be set to this name, making it useful for inter-container communication. The name must contain only upper and lowercase alphanumeric characters and hyphens and cannot start with a hyphen.
    #[serde(rename="containerName")]
    
    pub container_name: Option<String>,
    /// If the specified image is hosted on a private registry other than Google Container Registry, the credentials required to pull the image must be specified here as an encrypted secret. The secret must decrypt to a JSON-encoded dictionary containing both `username` and `password` keys.
    
    pub credentials: Option<Secret>,
    /// All container images are typically downloaded before any actions are executed. This helps prevent typos in URIs or issues like lack of disk space from wasting large amounts of compute resources. If set, this flag prevents the worker from downloading the image until just before the action is executed.
    #[serde(rename="disableImagePrefetch")]
    
    pub disable_image_prefetch: Option<bool>,
    /// A small portion of the container's standard error stream is typically captured and returned inside the `ContainerStoppedEvent`. Setting this flag disables this functionality.
    #[serde(rename="disableStandardErrorCapture")]
    
    pub disable_standard_error_capture: Option<bool>,
    /// Enable access to the FUSE device for this action. Filesystems can then be mounted into disks shared with other actions. The other actions do not need the `enable_fuse` flag to access the mounted filesystem. This has the effect of causing the container to be executed with `CAP_SYS_ADMIN` and exposes `/dev/fuse` to the container, so use it only for containers you trust.
    #[serde(rename="enableFuse")]
    
    pub enable_fuse: Option<bool>,
    /// The encrypted environment to pass into the container. This environment is merged with values specified in the google.cloud.lifesciences.v2beta.Pipeline message, overwriting any duplicate values. The secret must decrypt to a JSON-encoded dictionary where key-value pairs serve as environment variable names and their values. The decoded environment variables can overwrite the values specified by the `environment` field.
    #[serde(rename="encryptedEnvironment")]
    
    pub encrypted_environment: Option<Secret>,
    /// If specified, overrides the `ENTRYPOINT` specified in the container.
    
    pub entrypoint: Option<String>,
    /// The environment to pass into the container. This environment is merged with values specified in the google.cloud.lifesciences.v2beta.Pipeline message, overwriting any duplicate values. In addition to the values passed here, a few other values are automatically injected into the environment. These cannot be hidden or overwritten. `GOOGLE_PIPELINE_FAILED` will be set to "1" if the pipeline failed because an action has exited with a non-zero status (and did not have the `IGNORE_EXIT_STATUS` flag set). This can be used to determine if additional debug or logging actions should execute. `GOOGLE_LAST_EXIT_STATUS` will be set to the exit status of the last non-background action that executed. This can be used by workflow engine authors to determine whether an individual action has succeeded or failed.
    
    pub environment: Option<HashMap<String, String>>,
    /// Normally, a non-zero exit status causes the pipeline to fail. This flag allows execution of other actions to continue instead.
    #[serde(rename="ignoreExitStatus")]
    
    pub ignore_exit_status: Option<bool>,
    /// Required. The URI to pull the container image from. Note that all images referenced by actions in the pipeline are pulled before the first action runs. If multiple actions reference the same image, it is only pulled once, ensuring that the same image is used for all actions in a single pipeline. The image URI can be either a complete host and image specification (e.g., quay.io/biocontainers/samtools), a library and image name (e.g., google/cloud-sdk) or a bare image name ('bash') to pull from the default library. No schema is required in any of these cases. If the specified image is not public, the service account specified for the Virtual Machine must have access to pull the images from GCR, or appropriate credentials must be specified in the google.cloud.lifesciences.v2beta.Action.credentials field.
    #[serde(rename="imageUri")]
    
    pub image_uri: Option<String>,
    /// Labels to associate with the action. This field is provided to assist workflow engine authors in identifying actions (for example, to indicate what sort of action they perform, such as localization or debugging). They are returned in the operation metadata, but are otherwise ignored.
    
    pub labels: Option<HashMap<String, String>>,
    /// A list of mounts to make available to the action. In addition to the values specified here, every action has a special virtual disk mounted under `/google` that contains log files and other operational components. - /google/logs All logs written during the pipeline execution. - /google/logs/output The combined standard output and standard error of all actions run as part of the pipeline execution. - /google/logs/action/*/stdout The complete contents of each individual action's standard output. - /google/logs/action/*/stderr The complete contents of each individual action's standard error output. 
    
    pub mounts: Option<Vec<Mount>>,
    /// An optional identifier for a PID namespace to run the action inside. Multiple actions should use the same string to share a namespace. If unspecified, a separate isolated namespace is used.
    #[serde(rename="pidNamespace")]
    
    pub pid_namespace: Option<String>,
    /// A map of containers to host port mappings for this container. If the container already specifies exposed ports, use the `PUBLISH_EXPOSED_PORTS` flag instead. The host port number must be less than 65536. If it is zero, an unused random port is assigned. To determine the resulting port number, consult the `ContainerStartedEvent` in the operation metadata.
    #[serde(rename="portMappings")]
    
    pub port_mappings: Option<HashMap<String, i32>>,
    /// Exposes all ports specified by `EXPOSE` statements in the container. To discover the host side port numbers, consult the `ACTION_STARTED` event in the operation metadata.
    #[serde(rename="publishExposedPorts")]
    
    pub publish_exposed_ports: Option<bool>,
    /// This flag allows an action to continue running in the background while executing subsequent actions. This is useful to provide services to other actions (or to provide debugging support tools like SSH servers).
    #[serde(rename="runInBackground")]
    
    pub run_in_background: Option<bool>,
    /// The maximum amount of time to give the action to complete. If the action fails to complete before the timeout, it will be terminated and the exit status will be non-zero. The pipeline will continue or terminate based on the rules defined by the `ALWAYS_RUN` and `IGNORE_EXIT_STATUS` flags.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub timeout: Option<client::chrono::Duration>,
}

impl client::Part for Action {}


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


/// Carries information about a disk that can be attached to a VM. See https://cloud.google.com/compute/docs/disks/performance for more information about disk type, size, and performance considerations. Specify either `Volume` or `Disk`, but not both.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Disk {
    /// A user-supplied name for the disk. Used when mounting the disk into actions. The name must contain only upper and lowercase alphanumeric characters and hyphens and cannot start with a hyphen.
    
    pub name: Option<String>,
    /// The size, in GB, of the disk to attach. If the size is not specified, a default is chosen to ensure reasonable I/O performance. If the disk type is specified as `local-ssd`, multiple local drives are automatically combined to provide the requested size. Note, however, that each physical SSD is 375GB in size, and no more than 8 drives can be attached to a single instance.
    #[serde(rename="sizeGb")]
    
    pub size_gb: Option<i32>,
    /// An optional image to put on the disk before attaching it to the VM.
    #[serde(rename="sourceImage")]
    
    pub source_image: Option<String>,
    /// The Compute Engine disk type. If unspecified, `pd-standard` is used.
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
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// Configuration for an existing disk to be attached to the VM.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExistingDisk {
    /// If `disk` contains slashes, the Cloud Life Sciences API assumes that it is a complete URL for the disk. If `disk` does not contain slashes, the Cloud Life Sciences API assumes that the disk is a zonal disk and a URL will be generated of the form `zones//disks/`, where `` is the zone in which the instance is allocated. The disk must be ext4 formatted. If all `Mount` references to this disk have the `read_only` flag set to true, the disk will be attached in `read-only` mode and can be shared with other instances. Otherwise, the disk will be available for writing but cannot be shared.
    
    pub disk: Option<String>,
}

impl client::Part for ExistingDisk {}


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


/// Carries information about a particular disk mount inside a container.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Mount {
    /// The name of the disk to mount, as specified in the resources section.
    
    pub disk: Option<String>,
    /// The path to mount the disk inside the container.
    
    pub path: Option<String>,
    /// If true, the disk is mounted read-only inside the container.
    #[serde(rename="readOnly")]
    
    pub read_only: Option<bool>,
}

impl client::Part for Mount {}


/// Configuration for an `NFSMount` to be attached to the VM.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NFSMount {
    /// A target NFS mount. The target must be specified as `address:/mount".
    
    pub target: Option<String>,
}

impl client::Part for NFSMount {}


/// VM networking options.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Network {
    /// The network name to attach the VM's network interface to. The value will be prefixed with `global/networks/` unless it contains a `/`, in which case it is assumed to be a fully specified network resource URL. If unspecified, the global default network is used.
    
    pub network: Option<String>,
    /// If the specified network is configured for custom subnet creation, the name of the subnetwork to attach the instance to must be specified here. The value is prefixed with `regions/*/subnetworks/` unless it contains a `/`, in which case it is assumed to be a fully specified subnetwork resource URL. If the `*` character appears in the value, it is replaced with the region that the virtual machine has been allocated in.
    
    pub subnetwork: Option<String>,
    /// If set to true, do not attach a public IP address to the VM. Note that without a public IP address, additional configuration is required to allow the VM to access Google services. See https://cloud.google.com/vpc/docs/configure-private-google-access for more information.
    #[serde(rename="usePrivateAddress")]
    
    pub use_private_address: Option<bool>,
}

impl client::Part for Network {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations get projects](ProjectLocationOperationGetCall) (response)
/// * [locations pipelines run projects](ProjectLocationPipelineRunCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available.
    
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    
    pub error: Option<Status>,
    /// An Metadata object. This will always be returned with the Operation.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// The server-assigned name for the operation. This may be passed to the other operation methods to retrieve information about the operation's status.
    
    pub name: Option<String>,
    /// An Empty object.
    
    pub response: Option<HashMap<String, json::Value>>,
}

impl client::ResponseResult for Operation {}


/// Configuration for a persistent disk to be attached to the VM. See https://cloud.google.com/compute/docs/disks/performance for more information about disk type, size, and performance considerations.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PersistentDisk {
    /// The size, in GB, of the disk to attach. If the size is not specified, a default is chosen to ensure reasonable I/O performance. If the disk type is specified as `local-ssd`, multiple local drives are automatically combined to provide the requested size. Note, however, that each physical SSD is 375GB in size, and no more than 8 drives can be attached to a single instance.
    #[serde(rename="sizeGb")]
    
    pub size_gb: Option<i32>,
    /// An image to put on the disk before attaching it to the VM.
    #[serde(rename="sourceImage")]
    
    pub source_image: Option<String>,
    /// The Compute Engine disk type. If unspecified, `pd-standard` is used.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for PersistentDisk {}


/// Specifies a series of actions to execute, expressed as Docker containers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Pipeline {
    /// The list of actions to execute, in the order they are specified.
    
    pub actions: Option<Vec<Action>>,
    /// The encrypted environment to pass into every action. Each action can also specify its own encrypted environment. The secret must decrypt to a JSON-encoded dictionary where key-value pairs serve as environment variable names and their values. The decoded environment variables can overwrite the values specified by the `environment` field.
    #[serde(rename="encryptedEnvironment")]
    
    pub encrypted_environment: Option<Secret>,
    /// The environment to pass into every action. Each action can also specify additional environment variables but cannot delete an entry from this map (though they can overwrite it with a different value).
    
    pub environment: Option<HashMap<String, String>>,
    /// The resources required for execution.
    
    pub resources: Option<Resources>,
    /// The maximum amount of time to give the pipeline to complete. This includes the time spent waiting for a worker to be allocated. If the pipeline fails to complete before the timeout, it will be cancelled and the error code will be set to DEADLINE_EXCEEDED. If unspecified, it will default to 7 days.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub timeout: Option<client::chrono::Duration>,
}

impl client::Part for Pipeline {}


/// The system resources for the pipeline run. At least one zone or region must be specified or the pipeline run will fail.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Resources {
    /// The list of regions allowed for VM allocation. If set, the `zones` field must not be set.
    
    pub regions: Option<Vec<String>>,
    /// The virtual machine specification.
    #[serde(rename="virtualMachine")]
    
    pub virtual_machine: Option<VirtualMachine>,
    /// The list of zones allowed for VM allocation. If set, the `regions` field must not be set.
    
    pub zones: Option<Vec<String>>,
}

impl client::Part for Resources {}


/// The arguments to the `RunPipeline` method. The requesting user must have the `iam.serviceAccounts.actAs` permission for the Cloud Life Sciences service account or the request will fail.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations pipelines run projects](ProjectLocationPipelineRunCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RunPipelineRequest {
    /// User-defined labels to associate with the returned operation. These labels are not propagated to any Google Cloud Platform resources used by the operation, and can be modified at any time. To associate labels with resources created while executing the operation, see the appropriate resource message (for example, `VirtualMachine`).
    
    pub labels: Option<HashMap<String, String>>,
    /// Required. The description of the pipeline to run.
    
    pub pipeline: Option<Pipeline>,
    /// The name of an existing Pub/Sub topic. The server will publish messages to this topic whenever the status of the operation changes. The Life Sciences Service Agent account must have publisher permissions to the specified topic or notifications will not be sent.
    #[serde(rename="pubSubTopic")]
    
    pub pub_sub_topic: Option<String>,
}

impl client::RequestValue for RunPipelineRequest {}


/// Holds encrypted information that is only decrypted and stored in RAM by the worker VM when running the pipeline.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Secret {
    /// The value of the cipherText response from the `encrypt` method. This field is intentionally unaudited.
    #[serde(rename="cipherText")]
    
    pub cipher_text: Option<String>,
    /// The name of the Cloud KMS key that will be used to decrypt the secret value. The VM service account must have the required permissions and authentication scopes to invoke the `decrypt` method on the specified key.
    #[serde(rename="keyName")]
    
    pub key_name: Option<String>,
}

impl client::Part for Secret {}


/// Carries information about a Google Cloud service account.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceAccount {
    /// Email address of the service account. If not specified, the default Compute Engine service account for the project will be used.
    
    pub email: Option<String>,
    /// List of scopes to be enabled for this service account on the VM, in addition to the cloud-platform API scope that will be added by default.
    
    pub scopes: Option<Vec<String>>,
}

impl client::Part for ServiceAccount {}


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


/// Carries information about a Compute Engine VM resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VirtualMachine {
    /// The list of accelerators to attach to the VM.
    
    pub accelerators: Option<Vec<Accelerator>>,
    /// The size of the boot disk, in GB. The boot disk must be large enough to accommodate all of the Docker images from each action in the pipeline at the same time. If not specified, a small but reasonable default value is used.
    #[serde(rename="bootDiskSizeGb")]
    
    pub boot_disk_size_gb: Option<i32>,
    /// The host operating system image to use. Currently, only Container-Optimized OS images can be used. The default value is `projects/cos-cloud/global/images/family/cos-stable`, which selects the latest stable release of Container-Optimized OS. This option is provided to allow testing against the beta release of the operating system to ensure that the new version does not interact negatively with production pipelines. To test a pipeline against the beta release of Container-Optimized OS, use the value `projects/cos-cloud/global/images/family/cos-beta`.
    #[serde(rename="bootImage")]
    
    pub boot_image: Option<String>,
    /// The CPU platform to request. An instance based on a newer platform can be allocated, but never one with fewer capabilities. The value of this parameter must be a valid Compute Engine CPU platform name (such as "Intel Skylake"). This parameter is only useful for carefully optimized work loads where the CPU platform has a significant impact. For more information about the effect of this parameter, see https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform.
    #[serde(rename="cpuPlatform")]
    
    pub cpu_platform: Option<String>,
    /// The list of disks to create and attach to the VM. Specify either the `volumes[]` field or the `disks[]` field, but not both.
    
    pub disks: Option<Vec<Disk>>,
    /// The Compute Engine Disk Images to use as a Docker cache. The disks will be mounted into the Docker folder in a way that the images present in the cache will not need to be pulled. The digests of the cached images must match those of the tags used or the latest version will still be pulled. The root directory of the ext4 image must contain `image` and `overlay2` directories copied from the Docker directory of a VM where the desired Docker images have already been pulled. Any images pulled that are not cached will be stored on the first cache disk instead of the boot disk. Only a single image is supported.
    #[serde(rename="dockerCacheImages")]
    
    pub docker_cache_images: Option<Vec<String>>,
    /// Whether Stackdriver monitoring should be enabled on the VM.
    #[serde(rename="enableStackdriverMonitoring")]
    
    pub enable_stackdriver_monitoring: Option<bool>,
    /// Optional set of labels to apply to the VM and any attached disk resources. These labels must adhere to the [name and value restrictions](https://cloud.google.com/compute/docs/labeling-resources) on VM labels imposed by Compute Engine. Labels keys with the prefix 'google-' are reserved for use by Google. Labels applied at creation time to the VM. Applied on a best-effort basis to attached disk resources shortly after VM creation.
    
    pub labels: Option<HashMap<String, String>>,
    /// Required. The machine type of the virtual machine to create. Must be the short name of a standard machine type (such as "n1-standard-1") or a custom machine type (such as "custom-1-4096", where "1" indicates the number of vCPUs and "4096" indicates the memory in MB). See [Creating an instance with a custom machine type](https://cloud.google.com/compute/docs/instances/creating-instance-with-custom-machine-type#create) for more specifications on creating a custom machine type.
    #[serde(rename="machineType")]
    
    pub machine_type: Option<String>,
    /// The VM network configuration.
    
    pub network: Option<Network>,
    /// The NVIDIA driver version to use when attaching an NVIDIA GPU accelerator. The version specified here must be compatible with the GPU libraries contained in the container being executed, and must be one of the drivers hosted in the `nvidia-drivers-us-public` bucket on Google Cloud Storage.
    #[serde(rename="nvidiaDriverVersion")]
    
    pub nvidia_driver_version: Option<String>,
    /// If true, allocate a preemptible VM.
    
    pub preemptible: Option<bool>,
    /// If specified, the VM will only be allocated inside the matching reservation. It will fail if the VM parameters don't match the reservation.
    
    pub reservation: Option<String>,
    /// The service account to install on the VM. This account does not need any permissions other than those required by the pipeline.
    #[serde(rename="serviceAccount")]
    
    pub service_account: Option<ServiceAccount>,
    /// The list of disks and other storage to create or attach to the VM. Specify either the `volumes[]` field or the `disks[]` field, but not both.
    
    pub volumes: Option<Vec<Volume>>,
}

impl client::Part for VirtualMachine {}


/// Carries information about storage that can be attached to a VM. Specify either `Volume` or `Disk`, but not both.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Volume {
    /// Configuration for a existing disk.
    #[serde(rename="existingDisk")]
    
    pub existing_disk: Option<ExistingDisk>,
    /// Configuration for an NFS mount.
    #[serde(rename="nfsMount")]
    
    pub nfs_mount: Option<NFSMount>,
    /// Configuration for a persistent disk.
    #[serde(rename="persistentDisk")]
    
    pub persistent_disk: Option<PersistentDisk>,
    /// A user-supplied name for the volume. Used when mounting the volume into `Actions`. The name must contain only upper and lowercase alphanumeric characters and hyphens and cannot start with a hyphen.
    
    pub volume: Option<String>,
}

impl client::Part for Volume {}


