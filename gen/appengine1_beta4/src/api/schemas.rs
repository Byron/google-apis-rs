use super::*;
/// Google Cloud Endpoints (https://cloud.google.com/appengine/docs/python/endpoints/) configuration for API handlers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApiConfigHandler {
    /// Action to take when users access resources that require authentication. Defaults to redirect.
    #[serde(rename="authFailAction")]
    
    pub auth_fail_action: Option<ApiConfigHandlerAuthFailActionEnum>,
    /// Level of login required to access this resource. Defaults to optional.
    
    pub login: Option<ApiConfigHandlerLoginEnum>,
    /// Path to the script from the application root directory.
    
    pub script: Option<String>,
    /// Security (HTTPS) enforcement for this URL.
    #[serde(rename="securityLevel")]
    
    pub security_level: Option<ApiConfigHandlerSecurityLevelEnum>,
    /// URL to serve the endpoint at.
    
    pub url: Option<String>,
}

impl client::Part for ApiConfigHandler {}


/// Uses Google Cloud Endpoints to handle requests.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApiEndpointHandler {
    /// Path to the script from the application root directory.
    #[serde(rename="scriptPath")]
    
    pub script_path: Option<String>,
}

impl client::Part for ApiEndpointHandler {}


/// An Application resource contains the top-level configuration of an App Engine application.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create apps](AppCreateCall) (request)
/// * [get apps](AppGetCall) (response)
/// * [patch apps](AppPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Application {
    /// Google Apps authentication domain that controls which users can access this application.Defaults to open access for any Google Account.
    #[serde(rename="authDomain")]
    
    pub auth_domain: Option<String>,
    /// Google Cloud Storage bucket that can be used for storing files associated with this application. This bucket is associated with the application and can be used by the gcloud deployment commands.@OutputOnly
    #[serde(rename="codeBucket")]
    
    pub code_bucket: Option<String>,
    /// Google Cloud Storage bucket that can be used by this application to store content.@OutputOnly
    #[serde(rename="defaultBucket")]
    
    pub default_bucket: Option<String>,
    /// Cookie expiration policy for this application.
    #[serde(rename="defaultCookieExpiration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub default_cookie_expiration: Option<client::chrono::Duration>,
    /// Hostname used to reach the application, as resolved by App Engine.@OutputOnly
    #[serde(rename="defaultHostname")]
    
    pub default_hostname: Option<String>,
    /// HTTP path dispatch rules for requests to the application that do not explicitly target a module or version. Rules are order-dependent.@OutputOnly
    #[serde(rename="dispatchRules")]
    
    pub dispatch_rules: Option<Vec<UrlDispatchRule>>,
    /// no description provided
    
    pub iap: Option<IdentityAwareProxy>,
    /// Identifier of the Application resource. This identifier is equivalent to the project ID of the Google Cloud Platform project where you want to deploy your application. Example: myapp.
    
    pub id: Option<String>,
    /// Location from which this application will be run. Application instances will run out of data centers in the chosen location, which is also where all of the application's end user content is stored.Defaults to us-central.Options are:us-central - Central USeurope-west - Western Europeus-east1 - Eastern US
    
    pub location: Option<String>,
    /// Full path to the Application resource in the API. Example: apps/myapp.@OutputOnly
    
    pub name: Option<String>,
}

impl client::RequestValue for Application {}
impl client::ResponseResult for Application {}


/// Automatic scaling is based on request rate, response latencies, and other application metrics.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AutomaticScaling {
    /// The time period that the Autoscaler (https://cloud.google.com/compute/docs/autoscaler/) should wait before it starts collecting information from a new instance. This prevents the autoscaler from collecting information when the instance is initializing, during which the collected usage would not be reliable. Only applicable in the App Engine flexible environment.
    #[serde(rename="coolDownPeriod")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub cool_down_period: Option<client::chrono::Duration>,
    /// Target scaling by CPU usage.
    #[serde(rename="cpuUtilization")]
    
    pub cpu_utilization: Option<CpuUtilization>,
    /// Target scaling by disk usage.
    #[serde(rename="diskUtilization")]
    
    pub disk_utilization: Option<DiskUtilization>,
    /// Number of concurrent requests an automatic scaling instance can accept before the scheduler spawns a new instance.Defaults to a runtime-specific value.
    #[serde(rename="maxConcurrentRequests")]
    
    pub max_concurrent_requests: Option<i32>,
    /// Maximum number of idle instances that should be maintained for this version.
    #[serde(rename="maxIdleInstances")]
    
    pub max_idle_instances: Option<i32>,
    /// Maximum amount of time that a request should wait in the pending queue before starting a new instance to handle it.
    #[serde(rename="maxPendingLatency")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub max_pending_latency: Option<client::chrono::Duration>,
    /// Maximum number of instances that should be started to handle requests.
    #[serde(rename="maxTotalInstances")]
    
    pub max_total_instances: Option<i32>,
    /// Minimum number of idle instances that should be maintained for this version. Only applicable for the default version of a module.
    #[serde(rename="minIdleInstances")]
    
    pub min_idle_instances: Option<i32>,
    /// Minimum amount of time a request should wait in the pending queue before starting a new instance to handle it.
    #[serde(rename="minPendingLatency")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub min_pending_latency: Option<client::chrono::Duration>,
    /// Minimum number of instances that should be maintained for this version.
    #[serde(rename="minTotalInstances")]
    
    pub min_total_instances: Option<i32>,
    /// Target scaling by network usage.
    #[serde(rename="networkUtilization")]
    
    pub network_utilization: Option<NetworkUtilization>,
    /// Target scaling by request utilization.
    #[serde(rename="requestUtilization")]
    
    pub request_utilization: Option<RequestUtilization>,
}

impl client::Part for AutomaticScaling {}


/// A module with basic scaling will create an instance when the application receives a request. The instance will be turned down when the app becomes idle. Basic scaling is ideal for work that is intermittent or driven by user activity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BasicScaling {
    /// Duration of time after the last request that an instance must wait before the instance is shut down.
    #[serde(rename="idleTimeout")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub idle_timeout: Option<client::chrono::Duration>,
    /// Maximum number of instances to create for this version.
    #[serde(rename="maxInstances")]
    
    pub max_instances: Option<i32>,
}

impl client::Part for BasicScaling {}


/// Docker image that is used to create a container and start a VM instance for the version that you deploy. Only applicable for instances running in the App Engine flexible environment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContainerInfo {
    /// URI to the hosted container image in Google Container Registry. The URI must be fully qualified and include a tag or digest. Examples: "gcr.io/my-project/image:tag" or "gcr.io/my-project/image@digest"
    
    pub image: Option<String>,
}

impl client::Part for ContainerInfo {}


/// Target scaling by CPU usage.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CpuUtilization {
    /// Period of time over which CPU utilization is calculated.
    #[serde(rename="aggregationWindowLength")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub aggregation_window_length: Option<client::chrono::Duration>,
    /// Target CPU utilization ratio to maintain when scaling. Must be between 0 and 1.
    #[serde(rename="targetUtilization")]
    
    pub target_utilization: Option<f64>,
}

impl client::Part for CpuUtilization {}


/// Request message for Instances.DebugInstance.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [modules versions instances debug apps](AppModuleVersionInstanceDebugCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DebugInstanceRequest {
    /// Public SSH key to add to the instance. Examples:
    /// [USERNAME]:ssh-rsa [KEY_VALUE] [USERNAME]
    /// [USERNAME]:ssh-rsa [KEY_VALUE] google-ssh {"userName":"[USERNAME]","expireOn":"[EXPIRE_TIME]"}For more information, see Adding and Removing SSH Keys (https://cloud.google.com/compute/docs/instances/adding-removing-ssh-keys).
    #[serde(rename="sshKey")]
    
    pub ssh_key: Option<String>,
}

impl client::RequestValue for DebugInstanceRequest {}


/// Code and application artifacts used to deploy a version to App Engine.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Deployment {
    /// The Docker image for the container that runs the version. Only applicable for instances running in the App Engine flexible environment.
    
    pub container: Option<ContainerInfo>,
    /// Manifest of the files stored in Google Cloud Storage that are included as part of this version. All files must be readable using the credentials supplied with this call.
    
    pub files: Option<HashMap<String, FileInfo>>,
    /// Origin of the source code for this deployment. There can be more than one source reference per version if source code is distributed among multiple repositories.
    #[serde(rename="sourceReferences")]
    
    pub source_references: Option<Vec<SourceReference>>,
}

impl client::Part for Deployment {}


/// Target scaling by disk usage. Only applicable for VM runtimes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DiskUtilization {
    /// Target bytes read per second.
    #[serde(rename="targetReadBytesPerSec")]
    
    pub target_read_bytes_per_sec: Option<i32>,
    /// Target ops read per second.
    #[serde(rename="targetReadOpsPerSec")]
    
    pub target_read_ops_per_sec: Option<i32>,
    /// Target bytes written per second.
    #[serde(rename="targetWriteBytesPerSec")]
    
    pub target_write_bytes_per_sec: Option<i32>,
    /// Target ops written per second.
    #[serde(rename="targetWriteOpsPerSec")]
    
    pub target_write_ops_per_sec: Option<i32>,
}

impl client::Part for DiskUtilization {}


/// Cloud Endpoints (https://cloud.google.com/endpoints) configuration. The Endpoints API Service provides tooling for serving Open API and gRPC endpoints via an NGINX proxy. Only valid for App Engine Flexible environment deployments..The fields here refer to the name and configuration id of a "service" resource in the Service Management API (https://cloud.google.com/service-management/overview).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EndpointsApiService {
    /// Endpoints service configuration id as specified by the Service Management API. For example "2016-09-19r1"By default, the Endpoints service configuration id is fixed and config_id must be specified. To keep the Endpoints service configuration id updated with each rollout, specify RolloutStrategy.MANAGED and omit config_id.
    #[serde(rename="configId")]
    
    pub config_id: Option<String>,
    /// Enable or disable trace sampling. By default, this is set to false for enabled.
    #[serde(rename="disableTraceSampling")]
    
    pub disable_trace_sampling: Option<bool>,
    /// Endpoints service name which is the name of the "service" resource in the Service Management API. For example "myapi.endpoints.myproject.cloud.goog"
    
    pub name: Option<String>,
    /// Endpoints rollout strategy. If FIXED, config_id must be specified. If MANAGED, config_id must be omitted.
    #[serde(rename="rolloutStrategy")]
    
    pub rollout_strategy: Option<EndpointsApiServiceRolloutStrategyEnum>,
}

impl client::Part for EndpointsApiService {}


/// Custom static error page to be served when an error occurs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ErrorHandler {
    /// Error condition this handler applies to.
    #[serde(rename="errorCode")]
    
    pub error_code: Option<ErrorHandlerErrorCodeEnum>,
    /// MIME type of file. Defaults to text/html.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
    /// Static file content to be served for this error.
    #[serde(rename="staticFile")]
    
    pub static_file: Option<String>,
}

impl client::Part for ErrorHandler {}


/// Single source file that is part of the version to be deployed. Each source file that is deployed must be specified separately.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileInfo {
    /// The MIME type of the file.Defaults to the value from Google Cloud Storage.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
    /// The SHA1 hash of the file, in hex.
    #[serde(rename="sha1Sum")]
    
    pub sha1_sum: Option<String>,
    /// URL source to use to fetch this file. Must be a URL to a resource in Google Cloud Storage in the form 'http(s)://storage.googleapis.com/<bucket>/<object>'.
    #[serde(rename="sourceUrl")]
    
    pub source_url: Option<String>,
}

impl client::Part for FileInfo {}


/// Health checking configuration for VM instances. Unhealthy instances are killed and replaced with new instances. Only applicable for instances in App Engine flexible environment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HealthCheck {
    /// Interval between health checks.
    #[serde(rename="checkInterval")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub check_interval: Option<client::chrono::Duration>,
    /// Whether to explicitly disable health checks for this instance.
    #[serde(rename="disableHealthCheck")]
    
    pub disable_health_check: Option<bool>,
    /// Number of consecutive successful health checks required before receiving traffic.
    #[serde(rename="healthyThreshold")]
    
    pub healthy_threshold: Option<u32>,
    /// Host header to send when performing an HTTP health check. Example: "myapp.appspot.com"
    
    pub host: Option<String>,
    /// Number of consecutive failed health checks required before an instance is restarted.
    #[serde(rename="restartThreshold")]
    
    pub restart_threshold: Option<u32>,
    /// Time before the health check is considered failed.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub timeout: Option<client::chrono::Duration>,
    /// Number of consecutive failed health checks required before removing traffic.
    #[serde(rename="unhealthyThreshold")]
    
    pub unhealthy_threshold: Option<u32>,
}

impl client::Part for HealthCheck {}


/// Identity-Aware Proxy
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IdentityAwareProxy {
    /// Whether the serving infrastructure will authenticate and authorize all incoming requests.If true, the oauth2_client_id and oauth2_client_secret fields must be non-empty.
    
    pub enabled: Option<bool>,
    /// OAuth2 client ID to use for the authentication flow.
    #[serde(rename="oauth2ClientId")]
    
    pub oauth2_client_id: Option<String>,
    /// For security reasons, this value cannot be retrieved via the API. Instead, the SHA-256 hash of the value is returned in the oauth2_client_secret_sha256 field.@InputOnly
    #[serde(rename="oauth2ClientSecret")]
    
    pub oauth2_client_secret: Option<String>,
    /// Hex-encoded SHA-256 hash of the client secret.@OutputOnly
    #[serde(rename="oauth2ClientSecretSha256")]
    
    pub oauth2_client_secret_sha256: Option<String>,
}

impl client::Part for IdentityAwareProxy {}


/// An Instance resource is the computing unit that App Engine uses to automatically scale an application.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [modules versions instances get apps](AppModuleVersionInstanceGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Instance {
    /// App Engine release this instance is running on.@OutputOnly
    #[serde(rename="appEngineRelease")]
    
    pub app_engine_release: Option<String>,
    /// Availability of the instance.@OutputOnly
    
    pub availability: Option<InstanceAvailabilityEnum>,
    /// Average latency (ms) over the last minute.@OutputOnly
    #[serde(rename="averageLatency")]
    
    pub average_latency: Option<i32>,
    /// Number of errors since this instance was started.@OutputOnly
    
    pub errors: Option<u32>,
    /// Relative name of the instance within the version. Example: instance-1.@OutputOnly
    
    pub id: Option<String>,
    /// Total memory in use (bytes).@OutputOnly
    #[serde(rename="memoryUsage")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub memory_usage: Option<i64>,
    /// Full path to the Instance resource in the API. Example: apps/myapp/modules/default/versions/v1/instances/instance-1.@OutputOnly
    
    pub name: Option<String>,
    /// Average queries per second (QPS) over the last minute.@OutputOnly
    
    pub qps: Option<f32>,
    /// Number of requests since this instance was started.@OutputOnly
    
    pub requests: Option<i32>,
    /// Time that this instance was started.@OutputOnly
    #[serde(rename="startTimestamp")]
    
    pub start_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Virtual machine ID of this instance. Only applicable for instances in App Engine flexible environment.@OutputOnly
    #[serde(rename="vmId")]
    
    pub vm_id: Option<String>,
    /// The IP address of this instance. Only applicable for instances in App Engine flexible environment.@OutputOnly
    #[serde(rename="vmIp")]
    
    pub vm_ip: Option<String>,
    /// Name of the virtual machine where this instance lives. Only applicable for instances in App Engine flexible environment.@OutputOnly
    #[serde(rename="vmName")]
    
    pub vm_name: Option<String>,
    /// Status of the virtual machine where this instance lives. Only applicable for instances in App Engine flexible environment.@OutputOnly
    #[serde(rename="vmStatus")]
    
    pub vm_status: Option<String>,
    /// Whether this instance is in debug mode. Only applicable for instances in App Engine flexible environment.@OutputOnly
    #[serde(rename="vmUnlocked")]
    
    pub vm_unlocked: Option<bool>,
    /// Zone where the virtual machine is located. Only applicable for instances in App Engine flexible environment.@OutputOnly
    #[serde(rename="vmZoneName")]
    
    pub vm_zone_name: Option<String>,
}

impl client::ResponseResult for Instance {}


/// Third-party Python runtime library that is required by the application.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Library {
    /// Name of the library. Example: "django".
    
    pub name: Option<String>,
    /// Version of the library to select, or "latest".
    
    pub version: Option<String>,
}

impl client::Part for Library {}


/// Response message for Instances.ListInstances.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [modules versions instances list apps](AppModuleVersionInstanceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListInstancesResponse {
    /// The instances belonging to the requested version.
    
    pub instances: Option<Vec<Instance>>,
    /// Continuation token for fetching the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListInstancesResponse {}


/// The response message for Locations.ListLocations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations list apps](AppLocationListCall) (response)
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


/// Response message for Modules.ListModules.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [modules list apps](AppModuleListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListModulesResponse {
    /// The modules belonging to the requested application.
    
    pub modules: Option<Vec<Module>>,
    /// Continuation token for fetching the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListModulesResponse {}


/// The response message for Operations.ListOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [operations list apps](AppOperationListCall) (response)
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


/// Response message for Versions.ListVersions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [modules versions list apps](AppModuleVersionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListVersionsResponse {
    /// Continuation token for fetching the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The versions belonging to the requested module.
    
    pub versions: Option<Vec<Version>>,
}

impl client::ResponseResult for ListVersionsResponse {}


/// A resource that represents Google Cloud Platform location.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations get apps](AppLocationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    /// The friendly name for this location, typically a nearby city name. For example, "Tokyo".
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Cross-service attributes for the location. For example
    /// {"cloud.googleapis.com/region": "us-east1"}
    /// 
    
    pub labels: Option<HashMap<String, String>>,
    /// The canonical id for this location. For example: "us-east1".
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
    /// Service-specific metadata. For example the available capacity at the given location.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// Resource name for the location, which may vary between implementations. For example: "projects/example-project/locations/us-east1"
    
    pub name: Option<String>,
}

impl client::ResponseResult for Location {}


/// A module with manual scaling runs continuously, allowing you to perform complex initialization and rely on the state of its memory over time.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManualScaling {
    /// Number of instances to assign to the module at the start. This number can later be altered by using the Modules API (https://cloud.google.com/appengine/docs/python/modules/functions) set_num_instances() function.
    
    pub instances: Option<i32>,
}

impl client::Part for ManualScaling {}


/// A Module resource is a logical component of an application that can share state and communicate in a secure fashion with other modules. For example, an application that handles customer requests might include separate modules to handle tasks such as backend data analysis or API requests from mobile devices. Each module has a collection of versions that define a specific set of code used to implement the functionality of that module.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [modules get apps](AppModuleGetCall) (response)
/// * [modules patch apps](AppModulePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Module {
    /// Relative name of the module within the application. Example: default.@OutputOnly
    
    pub id: Option<String>,
    /// Full path to the Module resource in the API. Example: apps/myapp/modules/default.@OutputOnly
    
    pub name: Option<String>,
    /// Mapping that defines fractional HTTP traffic diversion to different versions within the module.
    
    pub split: Option<TrafficSplit>,
}

impl client::RequestValue for Module {}
impl client::ResponseResult for Module {}


/// Extra network settings. Only applicable for VM runtimes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Network {
    /// List of ports, or port pairs, to forward from the virtual machine to the application container.
    #[serde(rename="forwardedPorts")]
    
    pub forwarded_ports: Option<Vec<String>>,
    /// Tag to apply to the VM instance during creation.
    #[serde(rename="instanceTag")]
    
    pub instance_tag: Option<String>,
    /// Google Cloud Platform network where the virtual machines are created. Specify the short name, not the resource path.Defaults to default.
    
    pub name: Option<String>,
}

impl client::Part for Network {}


/// Target scaling by network usage. Only applicable for VM runtimes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkUtilization {
    /// Target bytes received per second.
    #[serde(rename="targetReceivedBytesPerSec")]
    
    pub target_received_bytes_per_sec: Option<i32>,
    /// Target packets received per second.
    #[serde(rename="targetReceivedPacketsPerSec")]
    
    pub target_received_packets_per_sec: Option<i32>,
    /// Target bytes sent per second.
    #[serde(rename="targetSentBytesPerSec")]
    
    pub target_sent_bytes_per_sec: Option<i32>,
    /// Target packets sent per second.
    #[serde(rename="targetSentPacketsPerSec")]
    
    pub target_sent_packets_per_sec: Option<i32>,
}

impl client::Part for NetworkUtilization {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [modules versions instances debug apps](AppModuleVersionInstanceDebugCall) (response)
/// * [modules versions instances delete apps](AppModuleVersionInstanceDeleteCall) (response)
/// * [modules versions create apps](AppModuleVersionCreateCall) (response)
/// * [modules versions delete apps](AppModuleVersionDeleteCall) (response)
/// * [modules versions patch apps](AppModuleVersionPatchCall) (response)
/// * [modules delete apps](AppModuleDeleteCall) (response)
/// * [modules patch apps](AppModulePatchCall) (response)
/// * [operations get apps](AppOperationGetCall) (response)
/// * [create apps](AppCreateCall) (response)
/// * [patch apps](AppPatchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// If the value is false, it means the operation is still in progress. If true, the operation is completed, and either error or response is available.
    
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    
    pub error: Option<Status>,
    /// Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the name should have the format of operations/some/unique/name.
    
    pub name: Option<String>,
    /// The normal response of the operation in case of success. If the original method returns no data on success, such as Delete, the response is google.protobuf.Empty. If the original method is standard Get/Create/Update, the response should be the resource. For other methods, the response should have the type XxxResponse, where Xxx is the original method name. For example, if the original method name is TakeSnapshot(), the inferred response type is TakeSnapshotResponse.
    
    pub response: Option<HashMap<String, json::Value>>,
}

impl client::ResponseResult for Operation {}


/// Target scaling by request utilization. Only applicable for VM runtimes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RequestUtilization {
    /// Target number of concurrent requests.
    #[serde(rename="targetConcurrentRequests")]
    
    pub target_concurrent_requests: Option<i32>,
    /// Target requests per second.
    #[serde(rename="targetRequestCountPerSec")]
    
    pub target_request_count_per_sec: Option<i32>,
}

impl client::Part for RequestUtilization {}


/// Machine resources for a version.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Resources {
    /// Number of CPU cores needed.
    
    pub cpu: Option<f64>,
    /// Disk size (GB) needed.
    #[serde(rename="diskGb")]
    
    pub disk_gb: Option<f64>,
    /// Memory (GB) needed.
    #[serde(rename="memoryGb")]
    
    pub memory_gb: Option<f64>,
    /// User specified volumes.
    
    pub volumes: Option<Vec<Volume>>,
}

impl client::Part for Resources {}


/// Executes a script to handle the request that matches the URL pattern.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ScriptHandler {
    /// Path to the script from the application root directory.
    #[serde(rename="scriptPath")]
    
    pub script_path: Option<String>,
}

impl client::Part for ScriptHandler {}


/// Reference to a particular snapshot of the source tree used to build and deploy the application.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SourceReference {
    /// URI string identifying the repository. Example: "https://source.developers.google.com/p/app-123/r/default"
    
    pub repository: Option<String>,
    /// The canonical, persistent identifier of the deployed revision. Aliases that include tags or branch names are not allowed. Example (git): "2198322f89e0bb2e25021667c2ed489d1fd34e6b"
    #[serde(rename="revisionId")]
    
    pub revision_id: Option<String>,
}

impl client::Part for SourceReference {}


/// Files served directly to the user for a given URL, such as images, CSS stylesheets, or JavaScript source files. Static directory handlers make it easy to serve the entire contents of a directory as static files.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StaticDirectoryHandler {
    /// Whether files should also be uploaded as code data. By default, files declared in static directory handlers are uploaded as static data and are only served to end users; they cannot be read by the application. If enabled, uploads are charged against both your code and static data storage resource quotas.
    #[serde(rename="applicationReadable")]
    
    pub application_readable: Option<bool>,
    /// Path to the directory containing the static files from the application root directory. Everything after the end of the matched URL pattern is appended to static_dir to form the full path to the requested file.
    
    pub directory: Option<String>,
    /// Time a static file served by this handler should be cached.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub expiration: Option<client::chrono::Duration>,
    /// HTTP headers to use for all responses from these URLs.
    #[serde(rename="httpHeaders")]
    
    pub http_headers: Option<HashMap<String, String>>,
    /// MIME type used to serve all files served by this handler. Defaults to file-specific MIME types, which are direved from each file's filename extension.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
    /// Whether this handler should match the request if the file referenced by the handler does not exist.
    #[serde(rename="requireMatchingFile")]
    
    pub require_matching_file: Option<bool>,
}

impl client::Part for StaticDirectoryHandler {}


/// Files served directly to the user for a given URL, such as images, CSS stylesheets, or JavaScript source files. Static file handlers describe which files in the application directory are static files, and which URLs serve them.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StaticFilesHandler {
    /// Whether files should also be uploaded as code data. By default, files declared in static file handlers are uploaded as static data and are only served to end users; they cannot be read by the application. If enabled, uploads are charged against both your code and static data storage resource quotas.
    #[serde(rename="applicationReadable")]
    
    pub application_readable: Option<bool>,
    /// Time a static file served by this handler should be cached.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub expiration: Option<client::chrono::Duration>,
    /// HTTP headers to use for all responses from these URLs.
    #[serde(rename="httpHeaders")]
    
    pub http_headers: Option<HashMap<String, String>>,
    /// MIME type used to serve all files served by this handler. Defaults to file-specific MIME types, which are derived from each file's filename extension.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
    /// Path to the static files matched by the URL pattern, from the application root directory. The path can refer to text matched in groupings in the URL pattern.
    
    pub path: Option<String>,
    /// Whether this handler should match the request if the file referenced by the handler does not exist.
    #[serde(rename="requireMatchingFile")]
    
    pub require_matching_file: Option<bool>,
    /// Regular expression that matches the file paths for all files that should be referenced by this handler.
    #[serde(rename="uploadPathRegex")]
    
    pub upload_path_regex: Option<String>,
}

impl client::Part for StaticFilesHandler {}


/// The Status type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by gRPC (https://github.com/grpc). The error model is designed to be:
/// Simple to use and understand for most users
/// Flexible enough to meet unexpected needsOverviewThe Status message contains three pieces of data: error code, error message, and error details. The error code should be an enum value of google.rpc.Code, but it may accept additional error codes if needed. The error message should be a developer-facing English message that helps developers understand and resolve the error. If a localized user-facing error message is needed, put the localized message in the error details or localize it in the client. The optional error details may contain arbitrary information about the error. There is a predefined set of error detail types in the package google.rpc that can be used for common error conditions.Language mappingThe Status message is the logical representation of the error model, but it is not necessarily the actual wire format. When the Status message is exposed in different client libraries and different wire protocols, it can be mapped differently. For example, it will likely be mapped to some exceptions in Java, but more likely mapped to some error codes in C.Other usesThe error model and the Status message can be used in a variety of environments, either with or without APIs, to provide a consistent developer experience across different environments.Example uses of this error model include:
/// Partial errors. If a service needs to return partial errors to the client, it may embed the Status in the normal response to indicate the partial errors.
/// Workflow errors. A typical workflow has multiple steps. Each step may have a Status message for error reporting.
/// Batch operations. If a client uses batch request and batch response, the Status message should be used directly inside batch response, one for each error sub-response.
/// Asynchronous operations. If an API call embeds asynchronous operation results in its response, the status of those operations should be represented directly using the Status message.
/// Logging. If some API errors are stored in logs, the message Status could be used directly after any stripping needed for security/privacy reasons.
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


/// Traffic routing configuration for versions within a single module. Traffic splits define how traffic directed to the module is assigned to versions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TrafficSplit {
    /// Mapping from version IDs within the module to fractional (0.000, 1] allocations of traffic for that version. Each version can be specified only once, but some versions in the module may not have any traffic allocation. Modules that have traffic allocated cannot be deleted until either the module is deleted or their traffic allocation is removed. Allocations must sum to 1. Up to two decimal place precision is supported for IP-based splits and up to three decimal places is supported for cookie-based splits.
    
    pub allocations: Option<HashMap<String, f64>>,
    /// Mechanism used to determine which version a request is sent to. The traffic selection algorithm will be stable for either type until allocations are changed.
    #[serde(rename="shardBy")]
    
    pub shard_by: Option<TrafficSplitShardByEnum>,
}

impl client::Part for TrafficSplit {}


/// Rules to match an HTTP request and dispatch that request to a module.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UrlDispatchRule {
    /// Domain name to match against. The wildcard "*" is supported if specified before a period: "*.".Defaults to matching all domains: "*".
    
    pub domain: Option<String>,
    /// Resource ID of a module in this application that should serve the matched request. The module must already exist. Example: default.
    
    pub module: Option<String>,
    /// Pathname within the host. Must start with a "/". A single "*" can be included at the end of the path. The sum of the lengths of the domain and path may not exceed 100 characters.
    
    pub path: Option<String>,
}

impl client::Part for UrlDispatchRule {}


/// URL pattern and description of how the URL should be handled. App Engine can handle URLs by executing application code, or by serving static files uploaded with the version, such as images, CSS, or JavaScript.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UrlMap {
    /// Uses API Endpoints to handle requests.
    #[serde(rename="apiEndpoint")]
    
    pub api_endpoint: Option<ApiEndpointHandler>,
    /// Action to take when users access resources that require authentication. Defaults to redirect.
    #[serde(rename="authFailAction")]
    
    pub auth_fail_action: Option<UrlMapAuthFailActionEnum>,
    /// Level of login required to access this resource.
    
    pub login: Option<UrlMapLoginEnum>,
    /// 30x code to use when performing redirects for the secure field. Defaults to 302.
    #[serde(rename="redirectHttpResponseCode")]
    
    pub redirect_http_response_code: Option<UrlMapRedirectHttpResponseCodeEnum>,
    /// Executes a script to handle the request that matches this URL pattern.
    
    pub script: Option<ScriptHandler>,
    /// Security (HTTPS) enforcement for this URL.
    #[serde(rename="securityLevel")]
    
    pub security_level: Option<UrlMapSecurityLevelEnum>,
    /// Serves the entire contents of a directory as static files.This attribute is deprecated. You can mimic the behavior of static directories using static files.
    #[serde(rename="staticDirectory")]
    
    pub static_directory: Option<StaticDirectoryHandler>,
    /// Returns the contents of a file, such as an image, as the response.
    #[serde(rename="staticFiles")]
    
    pub static_files: Option<StaticFilesHandler>,
    /// A URL prefix. Uses regular expression syntax, which means regexp special characters must be escaped, but should not contain groupings. All URLs that begin with this prefix are handled by this handler, using the portion of the URL after the prefix as part of the file path.
    #[serde(rename="urlRegex")]
    
    pub url_regex: Option<String>,
}

impl client::Part for UrlMap {}


/// A Version resource is a specific set of source code and configuration files that are deployed into a module.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [modules versions create apps](AppModuleVersionCreateCall) (request)
/// * [modules versions get apps](AppModuleVersionGetCall) (response)
/// * [modules versions patch apps](AppModuleVersionPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Version {
    /// Serving configuration for Google Cloud Endpoints (https://cloud.google.com/appengine/docs/python/endpoints/).Only returned in GET requests if view=FULL is set.
    #[serde(rename="apiConfig")]
    
    pub api_config: Option<ApiConfigHandler>,
    /// Automatic scaling is based on request rate, response latencies, and other application metrics.
    #[serde(rename="automaticScaling")]
    
    pub automatic_scaling: Option<AutomaticScaling>,
    /// A module with basic scaling will create an instance when the application receives a request. The instance will be turned down when the app becomes idle. Basic scaling is ideal for work that is intermittent or driven by user activity.
    #[serde(rename="basicScaling")]
    
    pub basic_scaling: Option<BasicScaling>,
    /// Metadata settings that are supplied to this version to enable beta runtime features.
    #[serde(rename="betaSettings")]
    
    pub beta_settings: Option<HashMap<String, String>>,
    /// Time that this version was created.@OutputOnly
    #[serde(rename="creationTime")]
    
    pub creation_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Duration that static files should be cached by web proxies and browsers. Only applicable if the corresponding StaticFilesHandler (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#staticfileshandler) does not specify its own expiration time.Only returned in GET requests if view=FULL is set.
    #[serde(rename="defaultExpiration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub default_expiration: Option<client::chrono::Duration>,
    /// Email address of the user who created this version.@OutputOnly
    
    pub deployer: Option<String>,
    /// Code and application artifacts that make up this version.Only returned in GET requests if view=FULL is set.
    
    pub deployment: Option<Deployment>,
    /// Cloud Endpoints configuration.If endpoints_api_service is set, the Cloud Endpoints Extensible Service Proxy will be provided to serve the API implemented by the app.
    #[serde(rename="endpointsApiService")]
    
    pub endpoints_api_service: Option<EndpointsApiService>,
    /// App Engine execution environment to use for this version.Defaults to 1.
    
    pub env: Option<String>,
    /// Environment variables made available to the application.Only returned in GET requests if view=FULL is set.
    #[serde(rename="envVariables")]
    
    pub env_variables: Option<HashMap<String, String>>,
    /// Custom static error pages. Limited to 10KB per page.Only returned in GET requests if view=FULL is set.
    #[serde(rename="errorHandlers")]
    
    pub error_handlers: Option<Vec<ErrorHandler>>,
    /// An ordered list of URL-matching patterns that should be applied to incoming requests. The first matching URL handles the request and other request handlers are not attempted.Only returned in GET requests if view=FULL is set.
    
    pub handlers: Option<Vec<UrlMap>>,
    /// Configures health checking for VM instances. Unhealthy instances are stopped and replaced with new instances. Only applicable for VM runtimes.Only returned in GET requests if view=FULL is set.
    #[serde(rename="healthCheck")]
    
    pub health_check: Option<HealthCheck>,
    /// Relative name of the version within the module. Example: v1. Version names can contain only lowercase letters, numbers, or hyphens. Reserved names: "default", "latest", and any name with the prefix "ah-".
    
    pub id: Option<String>,
    /// Before an application can receive email or XMPP messages, the application must be configured to enable the service.
    #[serde(rename="inboundServices")]
    
    pub inbound_services: Option<Vec<VersionInboundServicesEnum>>,
    /// Instance class that is used to run this version. Valid values are:
    /// AutomaticScaling: F1, F2, F4, F4_1G
    /// ManualScaling or BasicScaling: B1, B2, B4, B8, B4_1GDefaults to F1 for AutomaticScaling and B1 for ManualScaling or BasicScaling.
    #[serde(rename="instanceClass")]
    
    pub instance_class: Option<String>,
    /// Configuration for third-party Python runtime libraries required by the application.Only returned in GET requests if view=FULL is set.
    
    pub libraries: Option<Vec<Library>>,
    /// A module with manual scaling runs continuously, allowing you to perform complex initialization and rely on the state of its memory over time.
    #[serde(rename="manualScaling")]
    
    pub manual_scaling: Option<ManualScaling>,
    /// Full path to the Version resource in the API. Example: apps/myapp/modules/default/versions/v1.@OutputOnly
    
    pub name: Option<String>,
    /// Extra network settings. Only applicable for VM runtimes.
    
    pub network: Option<Network>,
    /// Files that match this pattern will not be built into this version. Only applicable for Go runtimes.Only returned in GET requests if view=FULL is set.
    #[serde(rename="nobuildFilesRegex")]
    
    pub nobuild_files_regex: Option<String>,
    /// Machine resources for this version. Only applicable for VM runtimes.
    
    pub resources: Option<Resources>,
    /// Desired runtime. Example: python27.
    
    pub runtime: Option<String>,
    /// The version of the API in the given runtime environment. Please see the app.yaml reference for valid values at https://cloud.google.com/appengine/docs/standard/<language>/config/appref
    #[serde(rename="runtimeApiVersion")]
    
    pub runtime_api_version: Option<String>,
    /// Current serving status of this version. Only the versions with a SERVING status create instances and can be billed.SERVING_STATUS_UNSPECIFIED is an invalid value. Defaults to SERVING.
    #[serde(rename="servingStatus")]
    
    pub serving_status: Option<VersionServingStatusEnum>,
    /// Whether multiple requests can be dispatched to this version at once.
    
    pub threadsafe: Option<bool>,
    /// Whether to deploy this version in a container on a virtual machine.
    
    pub vm: Option<bool>,
}

impl client::RequestValue for Version {}
impl client::ResponseResult for Version {}


/// Volumes mounted within the app container. Only applicable for VM runtimes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Volume {
    /// Unique name for the volume.
    
    pub name: Option<String>,
    /// Volume size in gigabytes.
    #[serde(rename="sizeGb")]
    
    pub size_gb: Option<f64>,
    /// Underlying volume type, e.g. 'tmpfs'.
    #[serde(rename="volumeType")]
    
    pub volume_type: Option<String>,
}

impl client::Part for Volume {}


