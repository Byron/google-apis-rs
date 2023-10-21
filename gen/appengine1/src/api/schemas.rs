use super::*;
/// Google Cloud Endpoints (https://cloud.google.com/endpoints) configuration for API handlers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApiConfigHandler {
    /// Action to take when users access resources that require authentication. Defaults to redirect.
    #[serde(rename="authFailAction")]
    
    pub auth_fail_action: Option<String>,
    /// Level of login required to access this resource. Defaults to optional.
    
    pub login: Option<String>,
    /// Path to the script from the application root directory.
    
    pub script: Option<String>,
    /// Security (HTTPS) enforcement for this URL.
    #[serde(rename="securityLevel")]
    
    pub security_level: Option<String>,
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
/// * [locations applications get projects](ProjectLocationApplicationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Application {
    /// Google Apps authentication domain that controls which users can access this application.Defaults to open access for any Google Account.
    #[serde(rename="authDomain")]
    
    pub auth_domain: Option<String>,
    /// Google Cloud Storage bucket that can be used for storing files associated with this application. This bucket is associated with the application and can be used by the gcloud deployment commands.@OutputOnly
    #[serde(rename="codeBucket")]
    
    pub code_bucket: Option<String>,
    /// The type of the Cloud Firestore or Cloud Datastore database associated with this application.
    #[serde(rename="databaseType")]
    
    pub database_type: Option<String>,
    /// Google Cloud Storage bucket that can be used by this application to store content.@OutputOnly
    #[serde(rename="defaultBucket")]
    
    pub default_bucket: Option<String>,
    /// Cookie expiration policy for this application.
    #[serde(rename="defaultCookieExpiration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub default_cookie_expiration: Option<client::chrono::Duration>,
    /// Hostname used to reach this application, as resolved by App Engine.@OutputOnly
    #[serde(rename="defaultHostname")]
    
    pub default_hostname: Option<String>,
    /// HTTP path dispatch rules for requests to the application that do not explicitly target a service or version. Rules are order-dependent. Up to 20 dispatch rules can be supported.
    #[serde(rename="dispatchRules")]
    
    pub dispatch_rules: Option<Vec<UrlDispatchRule>>,
    /// The feature specific settings to be used in the application.
    #[serde(rename="featureSettings")]
    
    pub feature_settings: Option<FeatureSettings>,
    /// The Google Container Registry domain used for storing managed build docker images for this application.
    #[serde(rename="gcrDomain")]
    
    pub gcr_domain: Option<String>,
    /// no description provided
    
    pub iap: Option<IdentityAwareProxy>,
    /// Identifier of the Application resource. This identifier is equivalent to the project ID of the Google Cloud Platform project where you want to deploy your application. Example: myapp.
    
    pub id: Option<String>,
    /// Location from which this application runs. Application instances run out of the data centers in the specified location, which is also where all of the application's end user content is stored.Defaults to us-central.View the list of supported locations (https://cloud.google.com/appengine/docs/locations).
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
    /// Full path to the Application resource in the API. Example: apps/myapp.@OutputOnly
    
    pub name: Option<String>,
    /// The service account associated with the application. This is the app-level default identity. If no identity provided during create version, Admin API will fallback to this one.
    #[serde(rename="serviceAccount")]
    
    pub service_account: Option<String>,
    /// Serving status of this application.
    #[serde(rename="servingStatus")]
    
    pub serving_status: Option<String>,
}

impl client::RequestValue for Application {}
impl client::ResponseResult for Application {}


/// An SSL certificate that a user has been authorized to administer. A user is authorized to administer any certificate that applies to one of their authorized domains.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [authorized certificates create apps](AppAuthorizedCertificateCreateCall) (request|response)
/// * [authorized certificates get apps](AppAuthorizedCertificateGetCall) (response)
/// * [authorized certificates patch apps](AppAuthorizedCertificatePatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuthorizedCertificate {
    /// The SSL certificate serving the AuthorizedCertificate resource. This must be obtained independently from a certificate authority.
    #[serde(rename="certificateRawData")]
    
    pub certificate_raw_data: Option<CertificateRawData>,
    /// The user-specified display name of the certificate. This is not guaranteed to be unique. Example: My Certificate.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Aggregate count of the domain mappings with this certificate mapped. This count includes domain mappings on applications for which the user does not have VIEWER permissions.Only returned by GET or LIST requests when specifically requested by the view=FULL_CERTIFICATE option.@OutputOnly
    #[serde(rename="domainMappingsCount")]
    
    pub domain_mappings_count: Option<i32>,
    /// Topmost applicable domains of this certificate. This certificate applies to these domains and their subdomains. Example: example.com.@OutputOnly
    #[serde(rename="domainNames")]
    
    pub domain_names: Option<Vec<String>>,
    /// The time when this certificate expires. To update the renewal time on this certificate, upload an SSL certificate with a different expiration time using AuthorizedCertificates.UpdateAuthorizedCertificate.@OutputOnly
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Relative name of the certificate. This is a unique value autogenerated on AuthorizedCertificate resource creation. Example: 12345.@OutputOnly
    
    pub id: Option<String>,
    /// Only applicable if this certificate is managed by App Engine. Managed certificates are tied to the lifecycle of a DomainMapping and cannot be updated or deleted via the AuthorizedCertificates API. If this certificate is manually administered by the user, this field will be empty.@OutputOnly
    #[serde(rename="managedCertificate")]
    
    pub managed_certificate: Option<ManagedCertificate>,
    /// Full path to the AuthorizedCertificate resource in the API. Example: apps/myapp/authorizedCertificates/12345.@OutputOnly
    
    pub name: Option<String>,
    /// The full paths to user visible Domain Mapping resources that have this certificate mapped. Example: apps/myapp/domainMappings/example.com.This may not represent the full list of mapped domain mappings if the user does not have VIEWER permissions on all of the applications that have this certificate mapped. See domain_mappings_count for a complete count.Only returned by GET or LIST requests when specifically requested by the view=FULL_CERTIFICATE option.@OutputOnly
    #[serde(rename="visibleDomainMappings")]
    
    pub visible_domain_mappings: Option<Vec<String>>,
}

impl client::RequestValue for AuthorizedCertificate {}
impl client::ResponseResult for AuthorizedCertificate {}


/// A domain that a user has been authorized to administer. To authorize use of a domain, verify ownership via Webmaster Central (https://www.google.com/webmasters/verification/home).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuthorizedDomain {
    /// Fully qualified domain name of the domain authorized for use. Example: example.com.
    
    pub id: Option<String>,
    /// Full path to the AuthorizedDomain resource in the API. Example: apps/myapp/authorizedDomains/example.com.@OutputOnly
    
    pub name: Option<String>,
}

impl client::Part for AuthorizedDomain {}


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
    /// Maximum number of instances that should be started to handle requests for this version.
    #[serde(rename="maxTotalInstances")]
    
    pub max_total_instances: Option<i32>,
    /// Minimum number of idle instances that should be maintained for this version. Only applicable for the default version of a service.
    #[serde(rename="minIdleInstances")]
    
    pub min_idle_instances: Option<i32>,
    /// Minimum amount of time a request should wait in the pending queue before starting a new instance to handle it.
    #[serde(rename="minPendingLatency")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub min_pending_latency: Option<client::chrono::Duration>,
    /// Minimum number of running instances that should be maintained for this version.
    #[serde(rename="minTotalInstances")]
    
    pub min_total_instances: Option<i32>,
    /// Target scaling by network usage.
    #[serde(rename="networkUtilization")]
    
    pub network_utilization: Option<NetworkUtilization>,
    /// Target scaling by request utilization.
    #[serde(rename="requestUtilization")]
    
    pub request_utilization: Option<RequestUtilization>,
    /// Scheduler settings for standard environment.
    #[serde(rename="standardSchedulerSettings")]
    
    pub standard_scheduler_settings: Option<StandardSchedulerSettings>,
}

impl client::Part for AutomaticScaling {}


/// A service with basic scaling will create an instance when the application receives a request. The instance will be turned down when the app becomes idle. Basic scaling is ideal for work that is intermittent or driven by user activity.
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


/// Request message for Firewall.BatchUpdateIngressRules.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [firewall ingress rules batch update apps](AppFirewallIngressRuleBatchUpdateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchUpdateIngressRulesRequest {
    /// A list of FirewallRules to replace the existing set.
    #[serde(rename="ingressRules")]
    
    pub ingress_rules: Option<Vec<FirewallRule>>,
}

impl client::RequestValue for BatchUpdateIngressRulesRequest {}


/// Response message for Firewall.UpdateAllIngressRules.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [firewall ingress rules batch update apps](AppFirewallIngressRuleBatchUpdateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchUpdateIngressRulesResponse {
    /// The full list of ingress FirewallRules for this application.
    #[serde(rename="ingressRules")]
    
    pub ingress_rules: Option<Vec<FirewallRule>>,
}

impl client::ResponseResult for BatchUpdateIngressRulesResponse {}


/// An SSL certificate obtained from a certificate authority.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CertificateRawData {
    /// Unencrypted PEM encoded RSA private key. This field is set once on certificate creation and then encrypted. The key size must be 2048 bits or fewer. Must include the header and footer. Example: -----BEGIN RSA PRIVATE KEY----- -----END RSA PRIVATE KEY----- @InputOnly
    #[serde(rename="privateKey")]
    
    pub private_key: Option<String>,
    /// PEM encoded x.509 public key certificate. This field is set once on certificate creation. Must include the header and footer. Example: -----BEGIN CERTIFICATE----- -----END CERTIFICATE----- 
    #[serde(rename="publicCertificate")]
    
    pub public_certificate: Option<String>,
}

impl client::Part for CertificateRawData {}


/// Options for the build operations performed as a part of the version deployment. Only applicable for App Engine flexible environment when creating a version using source code directly.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloudBuildOptions {
    /// Path to the yaml file used in deployment, used to determine runtime configuration details.Required for flexible environment builds.See https://cloud.google.com/appengine/docs/standard/python/config/appref for more details.
    #[serde(rename="appYamlPath")]
    
    pub app_yaml_path: Option<String>,
    /// The Cloud Build timeout used as part of any dependent builds performed by version creation. Defaults to 10 minutes.
    #[serde(rename="cloudBuildTimeout")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub cloud_build_timeout: Option<client::chrono::Duration>,
}

impl client::Part for CloudBuildOptions {}


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
/// * [services versions instances debug apps](AppServiceVersionInstanceDebugCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DebugInstanceRequest {
    /// Public SSH key to add to the instance. Examples: [USERNAME]:ssh-rsa [KEY_VALUE] [USERNAME] [USERNAME]:ssh-rsa [KEY_VALUE] google-ssh {"userName":"[USERNAME]","expireOn":"[EXPIRE_TIME]"}For more information, see Adding and Removing SSH Keys (https://cloud.google.com/compute/docs/instances/adding-removing-ssh-keys).
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
    /// Options for any Google Cloud Build builds created as a part of this deployment.These options will only be used if a new build is created, such as when deploying to the App Engine flexible environment using files or zip.
    #[serde(rename="cloudBuildOptions")]
    
    pub cloud_build_options: Option<CloudBuildOptions>,
    /// The Docker image for the container that runs the version. Only applicable for instances running in the App Engine flexible environment.
    
    pub container: Option<ContainerInfo>,
    /// Manifest of the files stored in Google Cloud Storage that are included as part of this version. All files must be readable using the credentials supplied with this call.
    
    pub files: Option<HashMap<String, FileInfo>>,
    /// The zip file for this deployment, if this is a zip deployment.
    
    pub zip: Option<ZipInfo>,
}

impl client::Part for Deployment {}


/// Target scaling by disk usage. Only applicable in the App Engine flexible environment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DiskUtilization {
    /// Target bytes read per second.
    #[serde(rename="targetReadBytesPerSecond")]
    
    pub target_read_bytes_per_second: Option<i32>,
    /// Target ops read per seconds.
    #[serde(rename="targetReadOpsPerSecond")]
    
    pub target_read_ops_per_second: Option<i32>,
    /// Target bytes written per second.
    #[serde(rename="targetWriteBytesPerSecond")]
    
    pub target_write_bytes_per_second: Option<i32>,
    /// Target ops written per second.
    #[serde(rename="targetWriteOpsPerSecond")]
    
    pub target_write_ops_per_second: Option<i32>,
}

impl client::Part for DiskUtilization {}


/// A domain serving an App Engine application.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [domain mappings create apps](AppDomainMappingCreateCall) (request)
/// * [domain mappings get apps](AppDomainMappingGetCall) (response)
/// * [domain mappings patch apps](AppDomainMappingPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DomainMapping {
    /// Relative name of the domain serving the application. Example: example.com.
    
    pub id: Option<String>,
    /// Full path to the DomainMapping resource in the API. Example: apps/myapp/domainMapping/example.com.@OutputOnly
    
    pub name: Option<String>,
    /// The resource records required to configure this domain mapping. These records must be added to the domain's DNS configuration in order to serve the application via this domain mapping.@OutputOnly
    #[serde(rename="resourceRecords")]
    
    pub resource_records: Option<Vec<ResourceRecord>>,
    /// SSL configuration for this domain. If unconfigured, this domain will not serve with SSL.
    #[serde(rename="sslSettings")]
    
    pub ssl_settings: Option<SslSettings>,
}

impl client::RequestValue for DomainMapping {}
impl client::ResponseResult for DomainMapping {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } 
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [authorized certificates delete apps](AppAuthorizedCertificateDeleteCall) (response)
/// * [firewall ingress rules delete apps](AppFirewallIngressRuleDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// Google Cloud Endpoints (https://cloud.google.com/endpoints) configuration. The Endpoints API Service provides tooling for serving Open API and gRPC endpoints via an NGINX proxy. Only valid for App Engine Flexible environment deployments.The fields here refer to the name and configuration ID of a "service" resource in the Service Management API (https://cloud.google.com/service-management/overview).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EndpointsApiService {
    /// Endpoints service configuration ID as specified by the Service Management API. For example "2016-09-19r1".By default, the rollout strategy for Endpoints is RolloutStrategy.FIXED. This means that Endpoints starts up with a particular configuration ID. When a new configuration is rolled out, Endpoints must be given the new configuration ID. The config_id field is used to give the configuration ID and is required in this case.Endpoints also has a rollout strategy called RolloutStrategy.MANAGED. When using this, Endpoints fetches the latest configuration and does not need the configuration ID. In this case, config_id must be omitted.
    #[serde(rename="configId")]
    
    pub config_id: Option<String>,
    /// Enable or disable trace sampling. By default, this is set to false for enabled.
    #[serde(rename="disableTraceSampling")]
    
    pub disable_trace_sampling: Option<bool>,
    /// Endpoints service name which is the name of the "service" resource in the Service Management API. For example "myapi.endpoints.myproject.cloud.goog"
    
    pub name: Option<String>,
    /// Endpoints rollout strategy. If FIXED, config_id must be specified. If MANAGED, config_id must be omitted.
    #[serde(rename="rolloutStrategy")]
    
    pub rollout_strategy: Option<String>,
}

impl client::Part for EndpointsApiService {}


/// The entrypoint for the application.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Entrypoint {
    /// The format should be a shell command that can be fed to bash -c.
    
    pub shell: Option<String>,
}

impl client::Part for Entrypoint {}


/// Custom static error page to be served when an error occurs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ErrorHandler {
    /// Error condition this handler applies to.
    #[serde(rename="errorCode")]
    
    pub error_code: Option<String>,
    /// MIME type of file. Defaults to text/html.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
    /// Static file content to be served for this error.
    #[serde(rename="staticFile")]
    
    pub static_file: Option<String>,
}

impl client::Part for ErrorHandler {}


/// The feature specific settings to be used in the application. These define behaviors that are user configurable.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FeatureSettings {
    /// Boolean value indicating if split health checks should be used instead of the legacy health checks. At an app.yaml level, this means defaulting to 'readiness_check' and 'liveness_check' values instead of 'health_check' ones. Once the legacy 'health_check' behavior is deprecated, and this value is always true, this setting can be removed.
    #[serde(rename="splitHealthChecks")]
    
    pub split_health_checks: Option<bool>,
    /// If true, use Container-Optimized OS (https://cloud.google.com/container-optimized-os/) base image for VMs, rather than a base Debian image.
    #[serde(rename="useContainerOptimizedOs")]
    
    pub use_container_optimized_os: Option<bool>,
}

impl client::Part for FeatureSettings {}


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
    /// URL source to use to fetch this file. Must be a URL to a resource in Google Cloud Storage in the form 'http(s)://storage.googleapis.com//'.
    #[serde(rename="sourceUrl")]
    
    pub source_url: Option<String>,
}

impl client::Part for FileInfo {}


/// A single firewall rule that is evaluated against incoming traffic and provides an action to take on matched requests.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [firewall ingress rules create apps](AppFirewallIngressRuleCreateCall) (request|response)
/// * [firewall ingress rules get apps](AppFirewallIngressRuleGetCall) (response)
/// * [firewall ingress rules patch apps](AppFirewallIngressRulePatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FirewallRule {
    /// The action to take on matched requests.
    
    pub action: Option<String>,
    /// An optional string description of this rule. This field has a maximum length of 400 characters.
    
    pub description: Option<String>,
    /// A positive integer between 1, Int32.MaxValue-1 that defines the order of rule evaluation. Rules with the lowest priority are evaluated first.A default rule at priority Int32.MaxValue matches all IPv4 and IPv6 traffic when no previous rule matches. Only the action of this rule can be modified by the user.
    
    pub priority: Option<i32>,
    /// IP address or range, defined using CIDR notation, of requests that this rule applies to. You can use the wildcard character "*" to match all IPs equivalent to "0/0" and "::/0" together. Examples: 192.168.1.1 or 192.168.0.0/16 or 2001:db8::/32 or 2001:0db8:0000:0042:0000:8a2e:0370:7334. Truncation will be silently performed on addresses which are not properly truncated. For example, 1.2.3.4/24 is accepted as the same address as 1.2.3.0/24. Similarly, for IPv6, 2001:db8::1/32 is accepted as the same address as 2001:db8::/32.
    #[serde(rename="sourceRange")]
    
    pub source_range: Option<String>,
}

impl client::RequestValue for FirewallRule {}
impl client::ResponseResult for FirewallRule {}


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
    /// OAuth2 client secret to use for the authentication flow.For security reasons, this value cannot be retrieved via the API. Instead, the SHA-256 hash of the value is returned in the oauth2_client_secret_sha256 field.@InputOnly
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
/// * [services versions instances get apps](AppServiceVersionInstanceGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Instance {
    /// Output only. App Engine release this instance is running on.
    #[serde(rename="appEngineRelease")]
    
    pub app_engine_release: Option<String>,
    /// Output only. Availability of the instance.
    
    pub availability: Option<String>,
    /// Output only. Average latency (ms) over the last minute.
    #[serde(rename="averageLatency")]
    
    pub average_latency: Option<i32>,
    /// Output only. Number of errors since this instance was started.
    
    pub errors: Option<i32>,
    /// Output only. Relative name of the instance within the version. Example: instance-1.
    
    pub id: Option<String>,
    /// Output only. Total memory in use (bytes).
    #[serde(rename="memoryUsage")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub memory_usage: Option<i64>,
    /// Output only. Full path to the Instance resource in the API. Example: apps/myapp/services/default/versions/v1/instances/instance-1.
    
    pub name: Option<String>,
    /// Output only. Average queries per second (QPS) over the last minute.
    
    pub qps: Option<f32>,
    /// Output only. Number of requests since this instance was started.
    
    pub requests: Option<i32>,
    /// Output only. Time that this instance was started.@OutputOnly
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Whether this instance is in debug mode. Only applicable for instances in App Engine flexible environment.
    #[serde(rename="vmDebugEnabled")]
    
    pub vm_debug_enabled: Option<bool>,
    /// Output only. Virtual machine ID of this instance. Only applicable for instances in App Engine flexible environment.
    #[serde(rename="vmId")]
    
    pub vm_id: Option<String>,
    /// Output only. The IP address of this instance. Only applicable for instances in App Engine flexible environment.
    #[serde(rename="vmIp")]
    
    pub vm_ip: Option<String>,
    /// Output only. The liveness health check of this instance. Only applicable for instances in App Engine flexible environment.
    #[serde(rename="vmLiveness")]
    
    pub vm_liveness: Option<String>,
    /// Output only. Name of the virtual machine where this instance lives. Only applicable for instances in App Engine flexible environment.
    #[serde(rename="vmName")]
    
    pub vm_name: Option<String>,
    /// Output only. Status of the virtual machine where this instance lives. Only applicable for instances in App Engine flexible environment.
    #[serde(rename="vmStatus")]
    
    pub vm_status: Option<String>,
    /// Output only. Zone where the virtual machine is located. Only applicable for instances in App Engine flexible environment.
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


/// Response message for AuthorizedCertificates.ListAuthorizedCertificates.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [authorized certificates list apps](AppAuthorizedCertificateListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAuthorizedCertificatesResponse {
    /// The SSL certificates the user is authorized to administer.
    
    pub certificates: Option<Vec<AuthorizedCertificate>>,
    /// Continuation token for fetching the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListAuthorizedCertificatesResponse {}


/// Response message for AuthorizedDomains.ListAuthorizedDomains.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [authorized domains list apps](AppAuthorizedDomainListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAuthorizedDomainsResponse {
    /// The authorized domains belonging to the user.
    
    pub domains: Option<Vec<AuthorizedDomain>>,
    /// Continuation token for fetching the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListAuthorizedDomainsResponse {}


/// Response message for DomainMappings.ListDomainMappings.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [domain mappings list apps](AppDomainMappingListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListDomainMappingsResponse {
    /// The domain mappings for the application.
    #[serde(rename="domainMappings")]
    
    pub domain_mappings: Option<Vec<DomainMapping>>,
    /// Continuation token for fetching the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListDomainMappingsResponse {}


/// Response message for Firewall.ListIngressRules.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [firewall ingress rules list apps](AppFirewallIngressRuleListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListIngressRulesResponse {
    /// The ingress FirewallRules for this application.
    #[serde(rename="ingressRules")]
    
    pub ingress_rules: Option<Vec<FirewallRule>>,
    /// Continuation token for fetching the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListIngressRulesResponse {}


/// Response message for Instances.ListInstances.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [services versions instances list apps](AppServiceVersionInstanceListCall) (response)
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


/// Response message for Services.ListServices.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [services list apps](AppServiceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListServicesResponse {
    /// Continuation token for fetching the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The services belonging to the requested application.
    
    pub services: Option<Vec<Service>>,
}

impl client::ResponseResult for ListServicesResponse {}


/// Response message for Versions.ListVersions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [services versions list apps](AppServiceVersionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListVersionsResponse {
    /// Continuation token for fetching the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The versions belonging to the requested service.
    
    pub versions: Option<Vec<Version>>,
}

impl client::ResponseResult for ListVersionsResponse {}


/// Health checking configuration for VM instances. Unhealthy instances are killed and replaced with new instances.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LivenessCheck {
    /// Interval between health checks.
    #[serde(rename="checkInterval")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub check_interval: Option<client::chrono::Duration>,
    /// Number of consecutive failed checks required before considering the VM unhealthy.
    #[serde(rename="failureThreshold")]
    
    pub failure_threshold: Option<u32>,
    /// Host header to send when performing a HTTP Liveness check. Example: "myapp.appspot.com"
    
    pub host: Option<String>,
    /// The initial delay before starting to execute the checks.
    #[serde(rename="initialDelay")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub initial_delay: Option<client::chrono::Duration>,
    /// The request path.
    
    pub path: Option<String>,
    /// Number of consecutive successful checks required before considering the VM healthy.
    #[serde(rename="successThreshold")]
    
    pub success_threshold: Option<u32>,
    /// Time before the check is considered failed.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub timeout: Option<client::chrono::Duration>,
}

impl client::Part for LivenessCheck {}


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
    /// Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} 
    
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


/// A certificate managed by App Engine.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManagedCertificate {
    /// Time at which the certificate was last renewed. The renewal process is fully managed. Certificate renewal will automatically occur before the certificate expires. Renewal errors can be tracked via ManagementStatus.@OutputOnly
    #[serde(rename="lastRenewalTime")]
    
    pub last_renewal_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Status of certificate management. Refers to the most recent certificate acquisition or renewal attempt.@OutputOnly
    
    pub status: Option<String>,
}

impl client::Part for ManagedCertificate {}


/// A service with manual scaling runs continuously, allowing you to perform complex initialization and rely on the state of its memory over time.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManualScaling {
    /// Number of instances to assign to the service at the start. This number can later be altered by using the Modules API (https://cloud.google.com/appengine/docs/python/modules/functions) set_num_instances() function.
    
    pub instances: Option<i32>,
}

impl client::Part for ManualScaling {}


/// Extra network settings. Only applicable in the App Engine flexible environment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Network {
    /// List of ports, or port pairs, to forward from the virtual machine to the application container. Only applicable in the App Engine flexible environment.
    #[serde(rename="forwardedPorts")]
    
    pub forwarded_ports: Option<Vec<String>>,
    /// The IP mode for instances. Only applicable in the App Engine flexible environment.
    #[serde(rename="instanceIpMode")]
    
    pub instance_ip_mode: Option<String>,
    /// Tag to apply to the instance during creation. Only applicable in the App Engine flexible environment.
    #[serde(rename="instanceTag")]
    
    pub instance_tag: Option<String>,
    /// Google Compute Engine network where the virtual machines are created. Specify the short name, not the resource path.Defaults to default.
    
    pub name: Option<String>,
    /// Enable session affinity. Only applicable in the App Engine flexible environment.
    #[serde(rename="sessionAffinity")]
    
    pub session_affinity: Option<bool>,
    /// Google Cloud Platform sub-network where the virtual machines are created. Specify the short name, not the resource path.If a subnetwork name is specified, a network name will also be required unless it is for the default network. If the network that the instance is being created in is a Legacy network, then the IP address is allocated from the IPv4Range. If the network that the instance is being created in is an auto Subnet Mode Network, then only network name should be specified (not the subnetwork_name) and the IP address is created from the IPCidrRange of the subnetwork that exists in that zone for that network. If the network that the instance is being created in is a custom Subnet Mode Network, then the subnetwork_name must be specified and the IP address is created from the IPCidrRange of the subnetwork.If specified, the subnetwork must exist in the same region as the App Engine flexible environment application.
    #[serde(rename="subnetworkName")]
    
    pub subnetwork_name: Option<String>,
}

impl client::Part for Network {}


/// A NetworkSettings resource is a container for ingress settings for a version or service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkSettings {
    /// The ingress settings for version or service.
    #[serde(rename="ingressTrafficAllowed")]
    
    pub ingress_traffic_allowed: Option<String>,
}

impl client::Part for NetworkSettings {}


/// Target scaling by network usage. Only applicable in the App Engine flexible environment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkUtilization {
    /// Target bytes received per second.
    #[serde(rename="targetReceivedBytesPerSecond")]
    
    pub target_received_bytes_per_second: Option<i32>,
    /// Target packets received per second.
    #[serde(rename="targetReceivedPacketsPerSecond")]
    
    pub target_received_packets_per_second: Option<i32>,
    /// Target bytes sent per second.
    #[serde(rename="targetSentBytesPerSecond")]
    
    pub target_sent_bytes_per_second: Option<i32>,
    /// Target packets sent per second.
    #[serde(rename="targetSentPacketsPerSecond")]
    
    pub target_sent_packets_per_second: Option<i32>,
}

impl client::Part for NetworkUtilization {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [domain mappings create apps](AppDomainMappingCreateCall) (response)
/// * [domain mappings delete apps](AppDomainMappingDeleteCall) (response)
/// * [domain mappings patch apps](AppDomainMappingPatchCall) (response)
/// * [operations get apps](AppOperationGetCall) (response)
/// * [services versions instances debug apps](AppServiceVersionInstanceDebugCall) (response)
/// * [services versions instances delete apps](AppServiceVersionInstanceDeleteCall) (response)
/// * [services versions create apps](AppServiceVersionCreateCall) (response)
/// * [services versions delete apps](AppServiceVersionDeleteCall) (response)
/// * [services versions patch apps](AppServiceVersionPatchCall) (response)
/// * [services delete apps](AppServiceDeleteCall) (response)
/// * [services patch apps](AppServicePatchCall) (response)
/// * [create apps](AppCreateCall) (response)
/// * [patch apps](AppPatchCall) (response)
/// * [repair apps](AppRepairCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// If the value is false, it means the operation is still in progress. If true, the operation is completed, and either error or response is available.
    
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    
    pub error: Option<Status>,
    /// Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the name should be a resource name ending with operations/{unique_id}.
    
    pub name: Option<String>,
    /// The normal response of the operation in case of success. If the original method returns no data on success, such as Delete, the response is google.protobuf.Empty. If the original method is standard Get/Create/Update, the response should be the resource. For other methods, the response should have the type XxxResponse, where Xxx is the original method name. For example, if the original method name is TakeSnapshot(), the inferred response type is TakeSnapshotResponse.
    
    pub response: Option<HashMap<String, json::Value>>,
}

impl client::ResponseResult for Operation {}


/// Readiness checking configuration for VM instances. Unhealthy instances are removed from traffic rotation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReadinessCheck {
    /// A maximum time limit on application initialization, measured from moment the application successfully replies to a healthcheck until it is ready to serve traffic.
    #[serde(rename="appStartTimeout")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub app_start_timeout: Option<client::chrono::Duration>,
    /// Interval between health checks.
    #[serde(rename="checkInterval")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub check_interval: Option<client::chrono::Duration>,
    /// Number of consecutive failed checks required before removing traffic.
    #[serde(rename="failureThreshold")]
    
    pub failure_threshold: Option<u32>,
    /// Host header to send when performing a HTTP Readiness check. Example: "myapp.appspot.com"
    
    pub host: Option<String>,
    /// The request path.
    
    pub path: Option<String>,
    /// Number of consecutive successful checks required before receiving traffic.
    #[serde(rename="successThreshold")]
    
    pub success_threshold: Option<u32>,
    /// Time before the check is considered failed.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub timeout: Option<client::chrono::Duration>,
}

impl client::Part for ReadinessCheck {}


/// Request message for ‘Applications.RepairApplication’.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [repair apps](AppRepairCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RepairApplicationRequest { _never_set: Option<bool> }

impl client::RequestValue for RepairApplicationRequest {}


/// Target scaling by request utilization. Only applicable in the App Engine flexible environment.
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
    #[serde(rename="targetRequestCountPerSecond")]
    
    pub target_request_count_per_second: Option<i32>,
}

impl client::Part for RequestUtilization {}


/// A DNS resource record.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResourceRecord {
    /// Relative name of the object affected by this record. Only applicable for CNAME records. Example: 'www'.
    
    pub name: Option<String>,
    /// Data for this record. Values vary by record type, as defined in RFC 1035 (section 5) and RFC 1034 (section 3.6.1).
    
    pub rrdata: Option<String>,
    /// Resource record type. Example: AAAA.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for ResourceRecord {}


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
    /// The name of the encryption key that is stored in Google Cloud KMS. Only should be used by Cloud Composer to encrypt the vm disk
    #[serde(rename="kmsKeyReference")]
    
    pub kms_key_reference: Option<String>,
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


/// A Service resource is a logical component of an application that can share state and communicate in a secure fashion with other services. For example, an application that handles customer requests might include separate services to handle tasks such as backend data analysis or API requests from mobile devices. Each service has a collection of versions that define a specific set of code used to implement the functionality of that service.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [services get apps](AppServiceGetCall) (response)
/// * [services patch apps](AppServicePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Service {
    /// Relative name of the service within the application. Example: default.@OutputOnly
    
    pub id: Option<String>,
    /// A set of labels to apply to this service. Labels are key/value pairs that describe the service and all resources that belong to it (e.g., versions). The labels can be used to search and group resources, and are propagated to the usage and billing reports, enabling fine-grain analysis of costs. An example of using labels is to tag resources belonging to different environments (e.g., "env=prod", "env=qa"). Label keys and values can be no longer than 63 characters and can only contain lowercase letters, numeric characters, underscores, dashes, and international characters. Label keys must start with a lowercase letter or an international character. Each service can have at most 32 labels.
    
    pub labels: Option<HashMap<String, String>>,
    /// Full path to the Service resource in the API. Example: apps/myapp/services/default.@OutputOnly
    
    pub name: Option<String>,
    /// Ingress settings for this service. Will apply to all versions.
    #[serde(rename="networkSettings")]
    
    pub network_settings: Option<NetworkSettings>,
    /// Mapping that defines fractional HTTP traffic diversion to different versions within the service.
    
    pub split: Option<TrafficSplit>,
}

impl client::RequestValue for Service {}
impl client::ResponseResult for Service {}


/// SSL configuration for a DomainMapping resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SslSettings {
    /// ID of the AuthorizedCertificate resource configuring SSL for the application. Clearing this field will remove SSL support.By default, a managed certificate is automatically created for every domain mapping. To omit SSL support or to configure SSL manually, specify SslManagementType.MANUAL on a CREATE or UPDATE request. You must be authorized to administer the AuthorizedCertificate resource to manually map it to a DomainMapping resource. Example: 12345.
    #[serde(rename="certificateId")]
    
    pub certificate_id: Option<String>,
    /// ID of the managed AuthorizedCertificate resource currently being provisioned, if applicable. Until the new managed certificate has been successfully provisioned, the previous SSL state will be preserved. Once the provisioning process completes, the certificate_id field will reflect the new managed certificate and this field will be left empty. To remove SSL support while there is still a pending managed certificate, clear the certificate_id field with an UpdateDomainMappingRequest.@OutputOnly
    #[serde(rename="pendingManagedCertificateId")]
    
    pub pending_managed_certificate_id: Option<String>,
    /// SSL management type for this domain. If AUTOMATIC, a managed certificate is automatically provisioned. If MANUAL, certificate_id must be manually specified in order to configure SSL for this domain.
    #[serde(rename="sslManagementType")]
    
    pub ssl_management_type: Option<String>,
}

impl client::Part for SslSettings {}


/// Scheduler settings for standard environment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StandardSchedulerSettings {
    /// Maximum number of instances to run for this version. Set to zero to disable max_instances configuration.
    #[serde(rename="maxInstances")]
    
    pub max_instances: Option<i32>,
    /// Minimum number of instances to run for this version. Set to zero to disable min_instances configuration.
    #[serde(rename="minInstances")]
    
    pub min_instances: Option<i32>,
    /// Target CPU utilization ratio to maintain when scaling.
    #[serde(rename="targetCpuUtilization")]
    
    pub target_cpu_utilization: Option<f64>,
    /// Target throughput utilization ratio to maintain when scaling
    #[serde(rename="targetThroughputUtilization")]
    
    pub target_throughput_utilization: Option<f64>,
}

impl client::Part for StandardSchedulerSettings {}


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
    /// Time a static file served by this handler should be cached by web proxies and browsers.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub expiration: Option<client::chrono::Duration>,
    /// HTTP headers to use for all responses from these URLs.
    #[serde(rename="httpHeaders")]
    
    pub http_headers: Option<HashMap<String, String>>,
    /// MIME type used to serve all files served by this handler.Defaults to file-specific MIME types, which are derived from each file's filename extension.
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


/// The Status type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by gRPC (https://github.com/grpc). Each Status message contains three pieces of data: error code, error message, and error details.You can find out more about this error model and how to work with it in the API Design Guide (https://cloud.google.com/apis/design/errors).
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


/// Traffic routing configuration for versions within a single service. Traffic splits define how traffic directed to the service is assigned to versions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TrafficSplit {
    /// Mapping from version IDs within the service to fractional (0.000, 1] allocations of traffic for that version. Each version can be specified only once, but some versions in the service may not have any traffic allocation. Services that have traffic allocated cannot be deleted until either the service is deleted or their traffic allocation is removed. Allocations must sum to 1. Up to two decimal place precision is supported for IP-based splits and up to three decimal places is supported for cookie-based splits.
    
    pub allocations: Option<HashMap<String, f64>>,
    /// Mechanism used to determine which version a request is sent to. The traffic selection algorithm will be stable for either type until allocations are changed.
    #[serde(rename="shardBy")]
    
    pub shard_by: Option<String>,
}

impl client::Part for TrafficSplit {}


/// Rules to match an HTTP request and dispatch that request to a service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UrlDispatchRule {
    /// Domain name to match against. The wildcard "*" is supported if specified before a period: "*.".Defaults to matching all domains: "*".
    
    pub domain: Option<String>,
    /// Pathname within the host. Must start with a "/". A single "*" can be included at the end of the path.The sum of the lengths of the domain and path may not exceed 100 characters.
    
    pub path: Option<String>,
    /// Resource ID of a service in this application that should serve the matched request. The service must already exist. Example: default.
    
    pub service: Option<String>,
}

impl client::Part for UrlDispatchRule {}


/// URL pattern and description of how the URL should be handled. App Engine can handle URLs by executing application code or by serving static files uploaded with the version, such as images, CSS, or JavaScript.
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
    
    pub auth_fail_action: Option<String>,
    /// Level of login required to access this resource. Not supported for Node.js in the App Engine standard environment.
    
    pub login: Option<String>,
    /// 30x code to use when performing redirects for the secure field. Defaults to 302.
    #[serde(rename="redirectHttpResponseCode")]
    
    pub redirect_http_response_code: Option<String>,
    /// Executes a script to handle the requests that match this URL pattern. Only the auto value is supported for Node.js in the App Engine standard environment, for example "script": "auto".
    
    pub script: Option<ScriptHandler>,
    /// Security (HTTPS) enforcement for this URL.
    #[serde(rename="securityLevel")]
    
    pub security_level: Option<String>,
    /// Returns the contents of a file, such as an image, as the response.
    #[serde(rename="staticFiles")]
    
    pub static_files: Option<StaticFilesHandler>,
    /// URL prefix. Uses regular expression syntax, which means regexp special characters must be escaped, but should not contain groupings. All URLs that begin with this prefix are handled by this handler, using the portion of the URL after the prefix as part of the file path.
    #[serde(rename="urlRegex")]
    
    pub url_regex: Option<String>,
}

impl client::Part for UrlMap {}


/// A Version resource is a specific set of source code and configuration files that are deployed into a service.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [services versions create apps](AppServiceVersionCreateCall) (request)
/// * [services versions get apps](AppServiceVersionGetCall) (response)
/// * [services versions patch apps](AppServiceVersionPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Version {
    /// Serving configuration for Google Cloud Endpoints (https://cloud.google.com/endpoints).Only returned in GET requests if view=FULL is set.
    #[serde(rename="apiConfig")]
    
    pub api_config: Option<ApiConfigHandler>,
    /// Allows App Engine second generation runtimes to access the legacy bundled services.
    #[serde(rename="appEngineApis")]
    
    pub app_engine_apis: Option<bool>,
    /// Automatic scaling is based on request rate, response latencies, and other application metrics. Instances are dynamically created and destroyed as needed in order to handle traffic.
    #[serde(rename="automaticScaling")]
    
    pub automatic_scaling: Option<AutomaticScaling>,
    /// A service with basic scaling will create an instance when the application receives a request. The instance will be turned down when the app becomes idle. Basic scaling is ideal for work that is intermittent or driven by user activity.
    #[serde(rename="basicScaling")]
    
    pub basic_scaling: Option<BasicScaling>,
    /// Metadata settings that are supplied to this version to enable beta runtime features.
    #[serde(rename="betaSettings")]
    
    pub beta_settings: Option<HashMap<String, String>>,
    /// Environment variables available to the build environment.Only returned in GET requests if view=FULL is set.
    #[serde(rename="buildEnvVariables")]
    
    pub build_env_variables: Option<HashMap<String, String>>,
    /// Time that this version was created.@OutputOnly
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Email address of the user who created this version.@OutputOnly
    #[serde(rename="createdBy")]
    
    pub created_by: Option<String>,
    /// Duration that static files should be cached by web proxies and browsers. Only applicable if the corresponding StaticFilesHandler (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#StaticFilesHandler) does not specify its own expiration time.Only returned in GET requests if view=FULL is set.
    #[serde(rename="defaultExpiration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub default_expiration: Option<client::chrono::Duration>,
    /// Code and application artifacts that make up this version.Only returned in GET requests if view=FULL is set.
    
    pub deployment: Option<Deployment>,
    /// Total size in bytes of all the files that are included in this version and currently hosted on the App Engine disk.@OutputOnly
    #[serde(rename="diskUsageBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub disk_usage_bytes: Option<i64>,
    /// Cloud Endpoints configuration.If endpoints_api_service is set, the Cloud Endpoints Extensible Service Proxy will be provided to serve the API implemented by the app.
    #[serde(rename="endpointsApiService")]
    
    pub endpoints_api_service: Option<EndpointsApiService>,
    /// The entrypoint for the application.
    
    pub entrypoint: Option<Entrypoint>,
    /// App Engine execution environment for this version.Defaults to standard.
    
    pub env: Option<String>,
    /// Environment variables available to the application.Only returned in GET requests if view=FULL is set.
    #[serde(rename="envVariables")]
    
    pub env_variables: Option<HashMap<String, String>>,
    /// Custom static error pages. Limited to 10KB per page.Only returned in GET requests if view=FULL is set.
    #[serde(rename="errorHandlers")]
    
    pub error_handlers: Option<Vec<ErrorHandler>>,
    /// An ordered list of URL-matching patterns that should be applied to incoming requests. The first matching URL handles the request and other request handlers are not attempted.Only returned in GET requests if view=FULL is set.
    
    pub handlers: Option<Vec<UrlMap>>,
    /// Configures health checking for instances. Unhealthy instances are stopped and replaced with new instances. Only applicable in the App Engine flexible environment.Only returned in GET requests if view=FULL is set.
    #[serde(rename="healthCheck")]
    
    pub health_check: Option<HealthCheck>,
    /// Relative name of the version within the service. Example: v1. Version names can contain only lowercase letters, numbers, or hyphens. Reserved names: "default", "latest", and any name with the prefix "ah-".
    
    pub id: Option<String>,
    /// Before an application can receive email or XMPP messages, the application must be configured to enable the service.
    #[serde(rename="inboundServices")]
    
    pub inbound_services: Option<Vec<String>>,
    /// Instance class that is used to run this version. Valid values are: AutomaticScaling: F1, F2, F4, F4_1G ManualScaling or BasicScaling: B1, B2, B4, B8, B4_1GDefaults to F1 for AutomaticScaling and B1 for ManualScaling or BasicScaling.
    #[serde(rename="instanceClass")]
    
    pub instance_class: Option<String>,
    /// Configuration for third-party Python runtime libraries that are required by the application.Only returned in GET requests if view=FULL is set.
    
    pub libraries: Option<Vec<Library>>,
    /// Configures liveness health checking for instances. Unhealthy instances are stopped and replaced with new instancesOnly returned in GET requests if view=FULL is set.
    #[serde(rename="livenessCheck")]
    
    pub liveness_check: Option<LivenessCheck>,
    /// A service with manual scaling runs continuously, allowing you to perform complex initialization and rely on the state of its memory over time. Manually scaled versions are sometimes referred to as "backends".
    #[serde(rename="manualScaling")]
    
    pub manual_scaling: Option<ManualScaling>,
    /// Full path to the Version resource in the API. Example: apps/myapp/services/default/versions/v1.@OutputOnly
    
    pub name: Option<String>,
    /// Extra network settings. Only applicable in the App Engine flexible environment.
    
    pub network: Option<Network>,
    /// Files that match this pattern will not be built into this version. Only applicable for Go runtimes.Only returned in GET requests if view=FULL is set.
    #[serde(rename="nobuildFilesRegex")]
    
    pub nobuild_files_regex: Option<String>,
    /// Configures readiness health checking for instances. Unhealthy instances are not put into the backend traffic rotation.Only returned in GET requests if view=FULL is set.
    #[serde(rename="readinessCheck")]
    
    pub readiness_check: Option<ReadinessCheck>,
    /// Machine resources for this version. Only applicable in the App Engine flexible environment.
    
    pub resources: Option<Resources>,
    /// Desired runtime. Example: python27.
    
    pub runtime: Option<String>,
    /// The version of the API in the given runtime environment. Please see the app.yaml reference for valid values at https://cloud.google.com/appengine/docs/standard//config/appref
    #[serde(rename="runtimeApiVersion")]
    
    pub runtime_api_version: Option<String>,
    /// The channel of the runtime to use. Only available for some runtimes. Defaults to the default channel.
    #[serde(rename="runtimeChannel")]
    
    pub runtime_channel: Option<String>,
    /// The path or name of the app's main executable.
    #[serde(rename="runtimeMainExecutablePath")]
    
    pub runtime_main_executable_path: Option<String>,
    /// The identity that the deployed version will run as. Admin API will use the App Engine Appspot service account as default if this field is neither provided in app.yaml file nor through CLI flag.
    #[serde(rename="serviceAccount")]
    
    pub service_account: Option<String>,
    /// Current serving status of this version. Only the versions with a SERVING status create instances and can be billed.SERVING_STATUS_UNSPECIFIED is an invalid value. Defaults to SERVING.
    #[serde(rename="servingStatus")]
    
    pub serving_status: Option<String>,
    /// Whether multiple requests can be dispatched to this version at once.
    
    pub threadsafe: Option<bool>,
    /// Serving URL for this version. Example: "https://myversion-dot-myservice-dot-myapp.appspot.com"@OutputOnly
    #[serde(rename="versionUrl")]
    
    pub version_url: Option<String>,
    /// Whether to deploy this version in a container on a virtual machine.
    
    pub vm: Option<bool>,
    /// Enables VPC connectivity for standard apps.
    #[serde(rename="vpcAccessConnector")]
    
    pub vpc_access_connector: Option<VpcAccessConnector>,
    /// The Google Compute Engine zones that are supported by this version in the App Engine flexible environment. Deprecated.
    
    pub zones: Option<Vec<String>>,
}

impl client::RequestValue for Version {}
impl client::ResponseResult for Version {}


/// Volumes mounted within the app container. Only applicable in the App Engine flexible environment.
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


/// VPC access connector specification.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VpcAccessConnector {
    /// The egress setting for the connector, controlling what traffic is diverted through it.
    #[serde(rename="egressSetting")]
    
    pub egress_setting: Option<String>,
    /// Full Serverless VPC Access Connector name e.g. /projects/my-project/locations/us-central1/connectors/c1.
    
    pub name: Option<String>,
}

impl client::Part for VpcAccessConnector {}


/// The zip file information for a zip deployment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ZipInfo {
    /// An estimate of the number of files in a zip for a zip deployment. If set, must be greater than or equal to the actual number of files. Used for optimizing performance; if not provided, deployment may be slow.
    #[serde(rename="filesCount")]
    
    pub files_count: Option<i32>,
    /// URL of the zip file to deploy from. Must be a URL to a resource in Google Cloud Storage in the form 'http(s)://storage.googleapis.com//'.
    #[serde(rename="sourceUrl")]
    
    pub source_url: Option<String>,
}

impl client::Part for ZipInfo {}


