use super::*;
/// Information for connecting over HTTP(s).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Addressable {
    /// no description provided
    
    pub url: Option<String>,
}

impl client::Part for Addressable {}


/// Specifies the audit configuration for a service. The configuration determines which permission types are logged, and what identities, if any, are exempted from logging. An AuditConfig must have one or more AuditLogConfigs. If there are AuditConfigs for both `allServices` and a specific service, the union of the two AuditConfigs is used for that service: the log_types specified in each AuditConfig are enabled, and the exempted_members in each AuditLogConfig are exempted. Example Policy with multiple AuditConfigs: { "audit_configs": [ { "service": "allServices", "audit_log_configs": [ { "log_type": "DATA_READ", "exempted_members": [ "user:jose@example.com" ] }, { "log_type": "DATA_WRITE" }, { "log_type": "ADMIN_READ" } ] }, { "service": "sampleservice.googleapis.com", "audit_log_configs": [ { "log_type": "DATA_READ" }, { "log_type": "DATA_WRITE", "exempted_members": [ "user:aliya@example.com" ] } ] } ] } For sampleservice, this policy enables DATA_READ, DATA_WRITE and ADMIN_READ logging. It also exempts `jose@example.com` from DATA_READ logging, and `aliya@example.com` from DATA_WRITE logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuditConfig {
    /// The configuration for logging of each type of permission.
    #[serde(rename="auditLogConfigs")]
    
    pub audit_log_configs: Option<Vec<AuditLogConfig>>,
    /// Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services.
    
    pub service: Option<String>,
}

impl client::Part for AuditConfig {}


/// Provides the configuration for logging a type of permissions. Example: { "audit_log_configs": [ { "log_type": "DATA_READ", "exempted_members": [ "user:jose@example.com" ] }, { "log_type": "DATA_WRITE" } ] } This enables 'DATA_READ' and 'DATA_WRITE' logging, while exempting jose@example.com from DATA_READ logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuditLogConfig {
    /// Specifies the identities that do not cause logging for this type of permission. Follows the same format of Binding.members.
    #[serde(rename="exemptedMembers")]
    
    pub exempted_members: Option<Vec<String>>,
    /// The log type that this config enables.
    #[serde(rename="logType")]
    
    pub log_type: Option<String>,
}

impl client::Part for AuditLogConfig {}


/// A domain that a user has been authorized to administer. To authorize use of a domain, verify ownership via [Webmaster Central](https://www.google.com/webmasters/verification/home).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuthorizedDomain {
    /// Relative name of the domain authorized for use. Example: `example.com`.
    
    pub id: Option<String>,
    /// Deprecated Read only. Full path to the `AuthorizedDomain` resource in the API. Example: `projects/myproject/authorizedDomains/example.com`.
    
    pub name: Option<String>,
}

impl client::Part for AuthorizedDomain {}


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


/// Request message for cancelling an execution.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [executions cancel namespaces](NamespaceExecutionCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelExecutionRequest { _never_set: Option<bool> }

impl client::RequestValue for CancelExecutionRequest {}


/// Not supported by Cloud Run. ConfigMapEnvSource selects a ConfigMap to populate the environment variables with. The contents of the target ConfigMap's Data field will represent the key-value pairs as environment variables.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigMapEnvSource {
    /// This field should not be used directly as it is meant to be inlined directly into the message. Use the "name" field instead.
    #[serde(rename="localObjectReference")]
    
    pub local_object_reference: Option<LocalObjectReference>,
    /// The ConfigMap to select from.
    
    pub name: Option<String>,
    /// Specify whether the ConfigMap must be defined.
    
    pub optional: Option<bool>,
}

impl client::Part for ConfigMapEnvSource {}


/// Not supported by Cloud Run.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigMapKeySelector {
    /// Required. Not supported by Cloud Run.
    
    pub key: Option<String>,
    /// Not supported by Cloud Run.
    #[serde(rename="localObjectReference")]
    
    pub local_object_reference: Option<LocalObjectReference>,
    /// Required. Not supported by Cloud Run.
    
    pub name: Option<String>,
    /// Not supported by Cloud Run.
    
    pub optional: Option<bool>,
}

impl client::Part for ConfigMapKeySelector {}


/// Not supported by Cloud Run. Adapts a ConfigMap into a volume. The contents of the target ConfigMap's Data field will be presented in a volume as files using the keys in the Data field as the file names, unless the items element is populated with specific mappings of keys to paths.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigMapVolumeSource {
    /// (Optional) Integer representation of mode bits to use on created files by default. Must be a value between 01 and 0777 (octal). If 0 or not set, it will default to 0644. Directories within the path are not affected by this setting. Notes * Internally, a umask of 0222 will be applied to any non-zero value. * This is an integer representation of the mode bits. So, the octal integer value should look exactly as the chmod numeric notation with a leading zero. Some examples: for chmod 777 (a=rwx), set to 0777 (octal) or 511 (base-10). For chmod 640 (u=rw,g=r), set to 0640 (octal) or 416 (base-10). For chmod 755 (u=rwx,g=rx,o=rx), set to 0755 (octal) or 493 (base-10). * This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    #[serde(rename="defaultMode")]
    
    pub default_mode: Option<i32>,
    /// (Optional) If unspecified, each key-value pair in the Data field of the referenced Secret will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified that is not present in the Secret, the volume setup will error unless it is marked optional.
    
    pub items: Option<Vec<KeyToPath>>,
    /// Name of the config.
    
    pub name: Option<String>,
    /// (Optional) Specify whether the Secret or its keys must be defined.
    
    pub optional: Option<bool>,
}

impl client::Part for ConfigMapVolumeSource {}


/// Configuration represents the “floating HEAD” of a linear history of Revisions, and optionally how the containers those revisions reference are built. Users create new Revisions by updating the Configuration’s spec. The “latest created” revision’s name is available under status, as is the “latest ready” revision’s name. See also: https://github.com/knative/specs/blob/main/specs/serving/overview.md#configuration
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [configurations get namespaces](NamespaceConfigurationGetCall) (response)
/// * [locations configurations get projects](ProjectLocationConfigurationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Configuration {
    /// The API version for this call such as "serving.knative.dev/v1".
    #[serde(rename="apiVersion")]
    
    pub api_version: Option<String>,
    /// The kind of resource, in this case always "Configuration".
    
    pub kind: Option<String>,
    /// Metadata associated with this Configuration, including name, namespace, labels, and annotations.
    
    pub metadata: Option<ObjectMeta>,
    /// Spec holds the desired state of the Configuration (from the client).
    
    pub spec: Option<ConfigurationSpec>,
    /// Status communicates the observed state of the Configuration (from the controller).
    
    pub status: Option<ConfigurationStatus>,
}

impl client::ResponseResult for Configuration {}


/// ConfigurationSpec holds the desired state of the Configuration (from the client).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigurationSpec {
    /// Template holds the latest specification for the Revision to be stamped out.
    
    pub template: Option<RevisionTemplate>,
}

impl client::Part for ConfigurationSpec {}


/// ConfigurationStatus communicates the observed state of the Configuration (from the controller).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigurationStatus {
    /// Conditions communicate information about ongoing/complete reconciliation processes that bring the "spec" inline with the observed state of the world.
    
    pub conditions: Option<Vec<GoogleCloudRunV1Condition>>,
    /// LatestCreatedRevisionName is the last revision that was created from this Configuration. It might not be ready yet, so for the latest ready revision, use LatestReadyRevisionName.
    #[serde(rename="latestCreatedRevisionName")]
    
    pub latest_created_revision_name: Option<String>,
    /// LatestReadyRevisionName holds the name of the latest Revision stamped out from this Configuration that has had its "Ready" condition become "True".
    #[serde(rename="latestReadyRevisionName")]
    
    pub latest_ready_revision_name: Option<String>,
    /// ObservedGeneration is the 'Generation' of the Configuration that was last processed by the controller. The observed generation is updated even if the controller failed to process the spec and create the Revision. Clients polling for completed reconciliation should poll until observedGeneration = metadata.generation, and the Ready condition's status is True or False.
    #[serde(rename="observedGeneration")]
    
    pub observed_generation: Option<i32>,
}

impl client::Part for ConfigurationStatus {}


/// A single application container. This specifies both the container to run, the command to run in the container and the arguments to supply to it. Note that additional arguments may be supplied by the system to the container at runtime.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Container {
    /// Arguments to the entrypoint. The docker image's CMD is used if this is not provided. Variable references are not supported in Cloud Run.
    
    pub args: Option<Vec<String>>,
    /// Entrypoint array. Not executed within a shell. The docker image's ENTRYPOINT is used if this is not provided. Variable references are not supported in Cloud Run.
    
    pub command: Option<Vec<String>>,
    /// List of environment variables to set in the container. EnvVar with duplicate names are generally allowed; if referencing a secret, the name must be unique for the container. For non-secret EnvVar names, the Container will only get the last-declared one.
    
    pub env: Option<Vec<EnvVar>>,
    /// Not supported by Cloud Run.
    #[serde(rename="envFrom")]
    
    pub env_from: Option<Vec<EnvFromSource>>,
    /// Required. URL of the Container image in Google Container Registry or Google Artifact Registry. More info: https://kubernetes.io/docs/concepts/containers/images
    
    pub image: Option<String>,
    /// Image pull policy. One of Always, Never, IfNotPresent. Defaults to Always if :latest tag is specified, or IfNotPresent otherwise. More info: https://kubernetes.io/docs/concepts/containers/images#updating-images
    #[serde(rename="imagePullPolicy")]
    
    pub image_pull_policy: Option<String>,
    /// Periodic probe of container liveness. Container will be restarted if the probe fails. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[serde(rename="livenessProbe")]
    
    pub liveness_probe: Option<Probe>,
    /// Name of the container specified as a DNS_LABEL (RFC 1123). More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#dns-label-names
    
    pub name: Option<String>,
    /// List of ports to expose from the container. Only a single port can be specified. The specified ports must be listening on all interfaces (0.0.0.0) within the container to be accessible. If omitted, a port number will be chosen and passed to the container through the PORT environment variable for the container to listen on.
    
    pub ports: Option<Vec<ContainerPort>>,
    /// Not supported by Cloud Run.
    #[serde(rename="readinessProbe")]
    
    pub readiness_probe: Option<Probe>,
    /// Compute Resources required by this container. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#resources
    
    pub resources: Option<ResourceRequirements>,
    /// Not supported by Cloud Run.
    #[serde(rename="securityContext")]
    
    pub security_context: Option<SecurityContext>,
    /// Startup probe of application within the container. All other probes are disabled if a startup probe is provided, until it succeeds. Container will not receive traffic if the probe fails. If not provided, a default startup probe with TCP socket action is used. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[serde(rename="startupProbe")]
    
    pub startup_probe: Option<Probe>,
    /// Path at which the file to which the container's termination message will be written is mounted into the container's filesystem. Message written is intended to be brief final status, such as an assertion failure message. Will be truncated by the node if greater than 4096 bytes. The total message length across all containers will be limited to 12kb. Defaults to /dev/termination-log.
    #[serde(rename="terminationMessagePath")]
    
    pub termination_message_path: Option<String>,
    /// Indicate how the termination message should be populated. File will use the contents of terminationMessagePath to populate the container status message on both success and failure. FallbackToLogsOnError will use the last chunk of container log output if the termination message file is empty and the container exited with an error. The log output is limited to 2048 bytes or 80 lines, whichever is smaller. Defaults to File. Cannot be updated.
    #[serde(rename="terminationMessagePolicy")]
    
    pub termination_message_policy: Option<String>,
    /// Volume to mount into the container's filesystem. Only supports SecretVolumeSources. Pod volumes to mount into the container's filesystem.
    #[serde(rename="volumeMounts")]
    
    pub volume_mounts: Option<Vec<VolumeMount>>,
    /// Container's working directory. If not specified, the container runtime's default will be used, which might be configured in the container image.
    #[serde(rename="workingDir")]
    
    pub working_dir: Option<String>,
}

impl client::Part for Container {}


/// ContainerPort represents a network port in a single container.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContainerPort {
    /// Port number the container listens on. This must be a valid port number, 0 < x < 65536.
    #[serde(rename="containerPort")]
    
    pub container_port: Option<i32>,
    /// If specified, used to specify which protocol to use. Allowed values are "http1" and "h2c".
    
    pub name: Option<String>,
    /// Protocol for port. Must be "TCP". Defaults to "TCP".
    
    pub protocol: Option<String>,
}

impl client::Part for ContainerPort {}


/// Resource to hold the state and status of a user’s domain mapping. NOTE: This resource is currently in Beta.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [domainmappings create namespaces](NamespaceDomainmappingCreateCall) (request|response)
/// * [domainmappings get namespaces](NamespaceDomainmappingGetCall) (response)
/// * [locations domainmappings create projects](ProjectLocationDomainmappingCreateCall) (request|response)
/// * [locations domainmappings get projects](ProjectLocationDomainmappingGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DomainMapping {
    /// The API version for this call such as "domains.cloudrun.com/v1".
    #[serde(rename="apiVersion")]
    
    pub api_version: Option<String>,
    /// The kind of resource, in this case "DomainMapping".
    
    pub kind: Option<String>,
    /// Metadata associated with this BuildTemplate.
    
    pub metadata: Option<ObjectMeta>,
    /// The spec for this DomainMapping.
    
    pub spec: Option<DomainMappingSpec>,
    /// The current status of the DomainMapping.
    
    pub status: Option<DomainMappingStatus>,
}

impl client::RequestValue for DomainMapping {}
impl client::ResponseResult for DomainMapping {}


/// The desired state of the Domain Mapping.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DomainMappingSpec {
    /// The mode of the certificate.
    #[serde(rename="certificateMode")]
    
    pub certificate_mode: Option<String>,
    /// If set, the mapping will override any mapping set before this spec was set. It is recommended that the user leaves this empty to receive an error warning about a potential conflict and only set it once the respective UI has given such a warning.
    #[serde(rename="forceOverride")]
    
    pub force_override: Option<bool>,
    /// The name of the Knative Route that this DomainMapping applies to. The route must exist.
    #[serde(rename="routeName")]
    
    pub route_name: Option<String>,
}

impl client::Part for DomainMappingSpec {}


/// The current state of the Domain Mapping.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DomainMappingStatus {
    /// Array of observed DomainMappingConditions, indicating the current state of the DomainMapping.
    
    pub conditions: Option<Vec<GoogleCloudRunV1Condition>>,
    /// The name of the route that the mapping currently points to.
    #[serde(rename="mappedRouteName")]
    
    pub mapped_route_name: Option<String>,
    /// ObservedGeneration is the 'Generation' of the DomainMapping that was last processed by the controller. Clients polling for completed reconciliation should poll until observedGeneration = metadata.generation and the Ready condition's status is True or False.
    #[serde(rename="observedGeneration")]
    
    pub observed_generation: Option<i32>,
    /// The resource records required to configure this domain mapping. These records must be added to the domain's DNS configuration in order to serve the application via this domain mapping.
    #[serde(rename="resourceRecords")]
    
    pub resource_records: Option<Vec<ResourceRecord>>,
    /// Optional. Not supported by Cloud Run.
    
    pub url: Option<String>,
}

impl client::Part for DomainMappingStatus {}


/// Not supported by Cloud Run. EnvFromSource represents the source of a set of ConfigMaps
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnvFromSource {
    /// The ConfigMap to select from
    #[serde(rename="configMapRef")]
    
    pub config_map_ref: Option<ConfigMapEnvSource>,
    /// An optional identifier to prepend to each key in the ConfigMap. Must be a C_IDENTIFIER.
    
    pub prefix: Option<String>,
    /// The Secret to select from
    #[serde(rename="secretRef")]
    
    pub secret_ref: Option<SecretEnvSource>,
}

impl client::Part for EnvFromSource {}


/// EnvVar represents an environment variable present in a Container.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnvVar {
    /// Required. Name of the environment variable. Must be a C_IDENTIFIER.
    
    pub name: Option<String>,
    /// Value of the environment variable. Defaults to "". Variable references are not supported in Cloud Run.
    
    pub value: Option<String>,
    /// Source for the environment variable's value. Only supports secret_key_ref. Cannot be used if value is not empty.
    #[serde(rename="valueFrom")]
    
    pub value_from: Option<EnvVarSource>,
}

impl client::Part for EnvVar {}


/// EnvVarSource represents a source for the value of an EnvVar.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnvVarSource {
    /// Not supported by Cloud Run. Not supported in Cloud Run.
    #[serde(rename="configMapKeyRef")]
    
    pub config_map_key_ref: Option<ConfigMapKeySelector>,
    /// Selects a key (version) of a secret in Secret Manager.
    #[serde(rename="secretKeyRef")]
    
    pub secret_key_ref: Option<SecretKeySelector>,
}

impl client::Part for EnvVarSource {}


/// Not supported by Cloud Run. ExecAction describes a "run in container" action.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExecAction {
    /// Command is the command line to execute inside the container, the working directory for the command is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
    
    pub command: Option<Vec<String>>,
}

impl client::Part for ExecAction {}


/// Execution represents the configuration of a single execution. An execution is an immutable resource that references a container image which is run to completion.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [executions cancel namespaces](NamespaceExecutionCancelCall) (response)
/// * [executions get namespaces](NamespaceExecutionGetCall) (response)
/// * [jobs run namespaces](NamespaceJobRunCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Execution {
    /// Optional. APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
    #[serde(rename="apiVersion")]
    
    pub api_version: Option<String>,
    /// Optional. Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    
    pub kind: Option<String>,
    /// Optional. Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    
    pub metadata: Option<ObjectMeta>,
    /// Optional. Specification of the desired behavior of an execution. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    
    pub spec: Option<ExecutionSpec>,
    /// Output only. Current status of an execution. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    
    pub status: Option<ExecutionStatus>,
}

impl client::ResponseResult for Execution {}


/// Reference to an Execution. Use /Executions.GetExecution with the given name to get full execution including the latest status.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionReference {
    /// Optional. Completion timestamp of the execution.
    #[serde(rename="completionTimestamp")]
    
    pub completion_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Creation timestamp of the execution.
    #[serde(rename="creationTimestamp")]
    
    pub creation_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Name of the execution.
    
    pub name: Option<String>,
}

impl client::Part for ExecutionReference {}


/// ExecutionSpec describes how the execution will look.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionSpec {
    /// Optional. Specifies the maximum desired number of tasks the execution should run at given time. Must be <= task_count. When the job is run, if this field is 0 or unset, the maximum possible value will be used for that execution. The actual number of tasks running in steady state will be less than this number when there are fewer tasks waiting to be completed, i.e. when the work left to do is less than max parallelism.
    
    pub parallelism: Option<i32>,
    /// Optional. Specifies the desired number of tasks the execution should run. Setting to 1 means that parallelism is limited to 1 and the success of that task signals the success of the execution.
    #[serde(rename="taskCount")]
    
    pub task_count: Option<i32>,
    /// Optional. The template used to create tasks for this execution.
    
    pub template: Option<TaskTemplateSpec>,
}

impl client::Part for ExecutionSpec {}


/// ExecutionStatus represents the current state of an Execution.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionStatus {
    /// Optional. The number of tasks which reached phase Cancelled.
    #[serde(rename="cancelledCount")]
    
    pub cancelled_count: Option<i32>,
    /// Optional. Represents the time that the execution was completed. It is not guaranteed to be set in happens-before order across separate operations. It is represented in RFC3339 form and is in UTC. +optional
    #[serde(rename="completionTime")]
    
    pub completion_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Conditions communicate information about ongoing/complete reconciliation processes that bring the "spec" inline with the observed state of the world. Execution-specific conditions include: * `ResourcesAvailable`: `True` when underlying resources have been provisioned. * `Started`: `True` when the execution has started to execute. * `Completed`: `True` when the execution has succeeded. `False` when the execution has failed.
    
    pub conditions: Option<Vec<GoogleCloudRunV1Condition>>,
    /// Optional. The number of tasks which reached phase Failed.
    #[serde(rename="failedCount")]
    
    pub failed_count: Option<i32>,
    /// Optional. URI where logs for this execution can be found in Cloud Console.
    #[serde(rename="logUri")]
    
    pub log_uri: Option<String>,
    /// Optional. The 'generation' of the execution that was last processed by the controller.
    #[serde(rename="observedGeneration")]
    
    pub observed_generation: Option<i32>,
    /// Optional. The number of tasks which have retried at least once.
    #[serde(rename="retriedCount")]
    
    pub retried_count: Option<i32>,
    /// Optional. The number of actively running tasks.
    #[serde(rename="runningCount")]
    
    pub running_count: Option<i32>,
    /// Optional. Represents the time that the execution started to run. It is not guaranteed to be set in happens-before order across separate operations. It is represented in RFC3339 form and is in UTC.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. The number of tasks which reached phase Succeeded.
    #[serde(rename="succeededCount")]
    
    pub succeeded_count: Option<i32>,
}

impl client::Part for ExecutionStatus {}


/// ExecutionTemplateSpec describes the metadata and spec an Execution should have when created from a job. Based on: https://github.com/kubernetes/api/blob/e771f807/core/v1/types.go#L3179-L3190
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionTemplateSpec {
    /// Optional. Optional metadata for this Execution, including labels and annotations. The following annotation keys set properties of the created execution: * `run.googleapis.com/cloudsql-instances` sets Cloud SQL connections. Multiple values should be comma separated. * `run.googleapis.com/vpc-access-connector` sets a Serverless VPC Access connector. * `run.googleapis.com/vpc-access-egress` sets VPC egress. Supported values are `all-traffic`, `all` (deprecated), and `private-ranges-only`. `all-traffic` and `all` provide the same functionality. `all` is deprecated but will continue to be supported. Prefer `all-traffic`.
    
    pub metadata: Option<ObjectMeta>,
    /// Required. ExecutionSpec holds the desired configuration for executions of this job.
    
    pub spec: Option<ExecutionSpec>,
}

impl client::Part for ExecutionTemplateSpec {}


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


/// GRPCAction describes an action involving a GRPC port.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GRPCAction {
    /// Port number of the gRPC service. Number must be in the range 1 to 65535.
    
    pub port: Option<i32>,
    /// Service is the name of the service to place in the gRPC HealthCheckRequest (see https://github.com/grpc/grpc/blob/master/doc/health-checking.md). If this is not specified, the default behavior is defined by gRPC.
    
    pub service: Option<String>,
}

impl client::Part for GRPCAction {}


/// Conditions show the status of reconciliation progress on a given resource. Most resource use a top-level condition type "Ready" or "Completed" to show overall status with other conditions to checkpoint each stage of reconciliation. Note that if metadata.Generation does not equal status.ObservedGeneration, the conditions shown may not be relevant for the current spec.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRunV1Condition {
    /// Optional. Last time the condition transitioned from one status to another.
    #[serde(rename="lastTransitionTime")]
    
    pub last_transition_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Human readable message indicating details about the current status.
    
    pub message: Option<String>,
    /// Optional. One-word CamelCase reason for the condition's last transition. These are intended to be stable, unique values which the client may use to trigger error handling logic, whereas messages which may be changed later by the server.
    
    pub reason: Option<String>,
    /// Optional. How to interpret this condition. One of Error, Warning, or Info. Conditions of severity Info do not contribute to resource readiness.
    
    pub severity: Option<String>,
    /// Status of the condition, one of True, False, Unknown.
    
    pub status: Option<String>,
    /// type is used to communicate the status of the reconciliation process. See also: https://github.com/knative/serving/blob/main/docs/spec/errors.md#error-conditions-and-reporting Types common to all resources include: * "Ready" or "Completed": True when the Resource is ready.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleCloudRunV1Condition {}


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


/// HTTPGetAction describes an action based on HTTP Get requests.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HTTPGetAction {
    /// Not supported by Cloud Run.
    
    pub host: Option<String>,
    /// Custom headers to set in the request. HTTP allows repeated headers.
    #[serde(rename="httpHeaders")]
    
    pub http_headers: Option<Vec<HTTPHeader>>,
    /// Path to access on the HTTP server.
    
    pub path: Option<String>,
    /// Not supported by Cloud Run.
    
    pub scheme: Option<String>,
}

impl client::Part for HTTPGetAction {}


/// HTTPHeader describes a custom header to be used in HTTP probes
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HTTPHeader {
    /// Required. The header field name
    
    pub name: Option<String>,
    /// The header field value
    
    pub value: Option<String>,
}

impl client::Part for HTTPHeader {}


/// Job represents the configuration of a single job, which references a container image which is run to completion.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [jobs create namespaces](NamespaceJobCreateCall) (request|response)
/// * [jobs get namespaces](NamespaceJobGetCall) (response)
/// * [jobs replace job namespaces](NamespaceJobReplaceJobCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Job {
    /// Optional. APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
    #[serde(rename="apiVersion")]
    
    pub api_version: Option<String>,
    /// Optional. Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    
    pub kind: Option<String>,
    /// Optional. Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    
    pub metadata: Option<ObjectMeta>,
    /// Optional. Specification of the desired behavior of a job. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    
    pub spec: Option<JobSpec>,
    /// Output only. Current status of a job. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    
    pub status: Option<JobStatus>,
}

impl client::RequestValue for Job {}
impl client::ResponseResult for Job {}


/// JobSpec describes how the job will look.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobSpec {
    /// Optional. Describes the execution that will be created when running a job.
    
    pub template: Option<ExecutionTemplateSpec>,
}

impl client::Part for JobSpec {}


/// JobStatus represents the current state of a Job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobStatus {
    /// Conditions communicate information about ongoing/complete reconciliation processes that bring the "spec" inline with the observed state of the world. Job-specific conditions include: * `Ready`: `True` when the job is ready to be executed.
    
    pub conditions: Option<Vec<GoogleCloudRunV1Condition>>,
    /// Number of executions created for this job.
    #[serde(rename="executionCount")]
    
    pub execution_count: Option<i32>,
    /// A pointer to the most recently created execution for this job. This is set regardless of the eventual state of the execution.
    #[serde(rename="latestCreatedExecution")]
    
    pub latest_created_execution: Option<ExecutionReference>,
    /// The 'generation' of the job that was last processed by the controller.
    #[serde(rename="observedGeneration")]
    
    pub observed_generation: Option<i32>,
}

impl client::Part for JobStatus {}


/// Maps a string key to a path within a volume.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KeyToPath {
    /// The Cloud Secret Manager secret version. Can be 'latest' for the latest value, or an integer or a secret alias for a specific version. The key to project.
    
    pub key: Option<String>,
    /// (Optional) Mode bits to use on this file, must be a value between 01 and 0777 (octal). If 0 or not set, the Volume's default mode will be used. Notes * Internally, a umask of 0222 will be applied to any non-zero value. * This is an integer representation of the mode bits. So, the octal integer value should look exactly as the chmod numeric notation with a leading zero. Some examples: for chmod 777 (a=rwx), set to 0777 (octal) or 511 (base-10). For chmod 640 (u=rw,g=r), set to 0640 (octal) or 416 (base-10). For chmod 755 (u=rwx,g=rx,o=rx), set to 0755 (octal) or 493 (base-10). * This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    
    pub mode: Option<i32>,
    /// The relative path of the file to map the key to. May not be an absolute path. May not contain the path element '..'. May not start with the string '..'.
    
    pub path: Option<String>,
}

impl client::Part for KeyToPath {}


/// A list of Authorized Domains.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [authorizeddomains list namespaces](NamespaceAuthorizeddomainListCall) (response)
/// * [authorizeddomains list projects](ProjectAuthorizeddomainListCall) (response)
/// * [locations authorizeddomains list projects](ProjectLocationAuthorizeddomainListCall) (response)
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


/// ListConfigurationsResponse is a list of Configuration resources.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [configurations list namespaces](NamespaceConfigurationListCall) (response)
/// * [locations configurations list projects](ProjectLocationConfigurationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListConfigurationsResponse {
    /// The API version for this call such as "serving.knative.dev/v1".
    #[serde(rename="apiVersion")]
    
    pub api_version: Option<String>,
    /// List of Configurations.
    
    pub items: Option<Vec<Configuration>>,
    /// The kind of this resource, in this case "ConfigurationList".
    
    pub kind: Option<String>,
    /// Metadata associated with this Configuration list.
    
    pub metadata: Option<ListMeta>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListConfigurationsResponse {}


/// ListDomainMappingsResponse is a list of DomainMapping resources.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [domainmappings list namespaces](NamespaceDomainmappingListCall) (response)
/// * [locations domainmappings list projects](ProjectLocationDomainmappingListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListDomainMappingsResponse {
    /// The API version for this call such as "domains.cloudrun.com/v1".
    #[serde(rename="apiVersion")]
    
    pub api_version: Option<String>,
    /// List of DomainMappings.
    
    pub items: Option<Vec<DomainMapping>>,
    /// The kind of this resource, in this case "DomainMappingList".
    
    pub kind: Option<String>,
    /// Metadata associated with this DomainMapping list.
    
    pub metadata: Option<ListMeta>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListDomainMappingsResponse {}


/// ListExecutionsResponse is a list of Executions resources.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [executions list namespaces](NamespaceExecutionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListExecutionsResponse {
    /// The API version for this call such as "run.googleapis.com/v1".
    #[serde(rename="apiVersion")]
    
    pub api_version: Option<String>,
    /// List of Executions.
    
    pub items: Option<Vec<Execution>>,
    /// The kind of this resource, in this case "ExecutionsList".
    
    pub kind: Option<String>,
    /// Metadata associated with this executions list.
    
    pub metadata: Option<ListMeta>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListExecutionsResponse {}


/// ListJobsResponse is a list of Jobs resources.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [jobs list namespaces](NamespaceJobListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListJobsResponse {
    /// The API version for this call such as "run.googleapis.com/v1".
    #[serde(rename="apiVersion")]
    
    pub api_version: Option<String>,
    /// List of Jobs.
    
    pub items: Option<Vec<Job>>,
    /// The kind of this resource, in this case "JobsList".
    
    pub kind: Option<String>,
    /// Metadata associated with this jobs list.
    
    pub metadata: Option<ListMeta>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListJobsResponse {}


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


/// Metadata for synthetic resources like List. In Cloud Run, all List Resources Responses will have a ListMeta instead of ObjectMeta.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListMeta {
    /// Continuation token is a value emitted when the count of items is larger than the user/system limit. To retrieve the next page of items, pass the value of `continue` as the next request's `page_token`.
    #[serde(rename="continue")]
    
    pub continue_: Option<String>,
    /// Opaque string that identifies the server's internal version of this object. It can be used by clients to determine when objects have changed. If the message is passed back to the server, it must be left unmodified. https://git.k8s.io/community/contributors/devel/api-conventions.md#concurrency-control-and-consistency
    #[serde(rename="resourceVersion")]
    
    pub resource_version: Option<String>,
    /// URL representing this object.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::Part for ListMeta {}


/// ListRevisionsResponse is a list of Revision resources.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [revisions list namespaces](NamespaceRevisionListCall) (response)
/// * [locations revisions list projects](ProjectLocationRevisionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListRevisionsResponse {
    /// The API version for this call such as "serving.knative.dev/v1".
    #[serde(rename="apiVersion")]
    
    pub api_version: Option<String>,
    /// List of Revisions.
    
    pub items: Option<Vec<Revision>>,
    /// The kind of this resource, in this case "RevisionList".
    
    pub kind: Option<String>,
    /// Metadata associated with this revision list.
    
    pub metadata: Option<ListMeta>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListRevisionsResponse {}


/// ListRoutesResponse is a list of Route resources.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [routes list namespaces](NamespaceRouteListCall) (response)
/// * [locations routes list projects](ProjectLocationRouteListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListRoutesResponse {
    /// The API version for this call such as "serving.knative.dev/v1".
    #[serde(rename="apiVersion")]
    
    pub api_version: Option<String>,
    /// List of Routes.
    
    pub items: Option<Vec<Route>>,
    /// The kind of this resource, in this case always "RouteList".
    
    pub kind: Option<String>,
    /// Metadata associated with this Route list.
    
    pub metadata: Option<ListMeta>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListRoutesResponse {}


/// A list of Service resources.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [services list namespaces](NamespaceServiceListCall) (response)
/// * [locations services list projects](ProjectLocationServiceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListServicesResponse {
    /// The API version for this call; returns "serving.knative.dev/v1".
    #[serde(rename="apiVersion")]
    
    pub api_version: Option<String>,
    /// List of Services.
    
    pub items: Option<Vec<Service>>,
    /// The kind of this resource; returns "ServiceList".
    
    pub kind: Option<String>,
    /// Metadata associated with this Service list.
    
    pub metadata: Option<ListMeta>,
    /// For calls against the global endpoint, returns the list of Cloud locations that could not be reached. For regional calls, this field is not used.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListServicesResponse {}


/// ListTasksResponse is a list of Tasks resources.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [tasks list namespaces](NamespaceTaskListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTasksResponse {
    /// The API version for this call such as "run.googleapis.com/v1".
    #[serde(rename="apiVersion")]
    
    pub api_version: Option<String>,
    /// List of Tasks.
    
    pub items: Option<Vec<Task>>,
    /// The kind of this resource, in this case "TasksList".
    
    pub kind: Option<String>,
    /// Metadata associated with this tasks list.
    
    pub metadata: Option<ListMeta>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListTasksResponse {}


/// Not supported by Cloud Run. LocalObjectReference contains enough information to let you locate the referenced object inside the same namespace.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocalObjectReference {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    
    pub name: Option<String>,
}

impl client::Part for LocalObjectReference {}


/// A resource that represents Google Cloud Platform location.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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

impl client::Part for Location {}


/// k8s.io.apimachinery.pkg.apis.meta.v1.ObjectMeta is metadata that all persisted resources must have, which includes all objects users must create.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ObjectMeta {
    /// Unstructured key value map stored with a resource that may be set by external tools to store and retrieve arbitrary metadata. They are not queryable and should be preserved when modifying objects. In Cloud Run, annotations with 'run.googleapis.com/' and 'autoscaling.knative.dev' are restricted, and the accepted annotations will be different depending on the resource type. * `autoscaling.knative.dev/maxScale`: Revision. * `autoscaling.knative.dev/minScale`: Revision. * `run.googleapis.com/binary-authorization-breakglass`: Service, Job, * `run.googleapis.com/binary-authorization`: Service, Job, Execution. * `run.googleapis.com/client-name`: All resources. * `run.googleapis.com/cloudsql-instances`: Revision, Execution. * `run.googleapis.com/cpu-throttling`: Revision. * `run.googleapis.com/custom-audiences`: Service. * `run.googleapis.com/description`: Service. * `run.googleapis.com/encryption-key-shutdown-hours`: Revision * `run.googleapis.com/encryption-key`: Revision, Execution. * `run.googleapis.com/execution-environment`: Revision, Execution. * `run.googleapis.com/gc-traffic-tags`: Service. * `run.googleapis.com/ingress`: Service. * `run.googleapis.com/network-interfaces`: Revision, Execution. * `run.googleapis.com/post-key-revocation-action-type`: Revision. * `run.googleapis.com/secrets`: Revision, Execution. * `run.googleapis.com/secure-session-agent`: Revision. * `run.googleapis.com/sessionAffinity`: Revision. * `run.googleapis.com/startup-cpu-boost`: Revision. * `run.googleapis.com/vpc-access-connector`: Revision, Execution. * `run.googleapis.com/vpc-access-egress`: Revision, Execution. Execution. More info: https://kubernetes.io/docs/user-guide/annotations
    
    pub annotations: Option<HashMap<String, String>>,
    /// Not supported by Cloud Run
    #[serde(rename="clusterName")]
    
    pub cluster_name: Option<String>,
    /// UTC timestamp representing the server time when this object was created. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    #[serde(rename="creationTimestamp")]
    
    pub creation_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Not supported by Cloud Run
    #[serde(rename="deletionGracePeriodSeconds")]
    
    pub deletion_grace_period_seconds: Option<i32>,
    /// The read-only soft deletion timestamp for this resource. In Cloud Run, users are not able to set this field. Instead, they must call the corresponding Delete API.
    #[serde(rename="deletionTimestamp")]
    
    pub deletion_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Not supported by Cloud Run
    
    pub finalizers: Option<Vec<String>>,
    /// Not supported by Cloud Run
    #[serde(rename="generateName")]
    
    pub generate_name: Option<String>,
    /// A system-provided sequence number representing a specific generation of the desired state.
    
    pub generation: Option<i32>,
    /// Map of string keys and values that can be used to organize and categorize (scope and select) objects. May match selectors of replication controllers and routes. More info: https://kubernetes.io/docs/user-guide/labels
    
    pub labels: Option<HashMap<String, String>>,
    /// Required. The name of the resource. In Cloud Run, name is required when creating top-level resources (Service, Job), must be unique within a Cloud Run project/region, and cannot be changed once created. More info: https://kubernetes.io/docs/user-guide/identifiers#names
    
    pub name: Option<String>,
    /// Required. Defines the space within each name must be unique within a Cloud Run region. In Cloud Run, it must be project ID or number.
    
    pub namespace: Option<String>,
    /// Not supported by Cloud Run
    #[serde(rename="ownerReferences")]
    
    pub owner_references: Option<Vec<OwnerReference>>,
    /// Opaque, system-generated value that represents the internal version of this object that can be used by clients to determine when objects have changed. May be used for optimistic concurrency, change detection, and the watch operation on a resource or set of resources. Clients must treat these values as opaque and passed unmodified back to the server or omit the value to disable conflict-detection. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(rename="resourceVersion")]
    
    pub resource_version: Option<String>,
    /// URL representing this object.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Unique, system-generated identifier for this resource. More info: https://kubernetes.io/docs/user-guide/identifiers#uids
    
    pub uid: Option<String>,
}

impl client::Part for ObjectMeta {}


/// This is not supported or used by Cloud Run.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OwnerReference {
    /// This is not supported or used by Cloud Run.
    #[serde(rename="apiVersion")]
    
    pub api_version: Option<String>,
    /// This is not supported or used by Cloud Run.
    #[serde(rename="blockOwnerDeletion")]
    
    pub block_owner_deletion: Option<bool>,
    /// This is not supported or used by Cloud Run.
    
    pub controller: Option<bool>,
    /// This is not supported or used by Cloud Run.
    
    pub kind: Option<String>,
    /// This is not supported or used by Cloud Run.
    
    pub name: Option<String>,
    /// This is not supported or used by Cloud Run.
    
    pub uid: Option<String>,
}

impl client::Part for OwnerReference {}


/// An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members`, or principals, to a single `role`. Principals can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { “bindings”: \[ { “role”: “roles/resourcemanager.organizationAdmin”, “members”: \[ “user:mike@example.com”, “group:admins@example.com”, “domain:google.com”, “serviceAccount:my-project-id@appspot.gserviceaccount.com” \] }, { “role”: “roles/resourcemanager.organizationViewer”, “members”: \[ “user:eve@example.com” \], “condition”: { “title”: “expirable access”, “description”: “Does not grant access after Sep 2020”, “expression”: “request.time \< timestamp(‘2020-10-01T00:00:00.000Z’)”, } } \], “etag”: “BwWWja0YfJA=”, “version”: 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time \< timestamp(‘2020-10-01T00:00:00.000Z’) etag: BwWWja0YfJA= version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations jobs get iam policy projects](ProjectLocationJobGetIamPolicyCall) (response)
/// * [locations jobs set iam policy projects](ProjectLocationJobSetIamPolicyCall) (response)
/// * [locations services get iam policy projects](ProjectLocationServiceGetIamPolicyCall) (response)
/// * [locations services set iam policy projects](ProjectLocationServiceSetIamPolicyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Policy {
    /// Specifies cloud audit logging configuration for this policy.
    #[serde(rename="auditConfigs")]
    
    pub audit_configs: Option<Vec<AuditConfig>>,
    /// Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`.
    
    pub bindings: Option<Vec<Binding>>,
    /// `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
    /// Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub version: Option<i32>,
}

impl client::ResponseResult for Policy {}


/// Probe describes a health check to be performed against a container to determine whether it is alive or ready to receive traffic.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Probe {
    /// Not supported by Cloud Run.
    
    pub exec: Option<ExecAction>,
    /// Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.
    #[serde(rename="failureThreshold")]
    
    pub failure_threshold: Option<i32>,
    /// GRPCAction specifies an action involving a GRPC port.
    
    pub grpc: Option<GRPCAction>,
    /// HTTPGet specifies the http request to perform.
    #[serde(rename="httpGet")]
    
    pub http_get: Option<HTTPGetAction>,
    /// Number of seconds after the container has started before the probe is initiated. Defaults to 0 seconds. Minimum value is 0. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[serde(rename="initialDelaySeconds")]
    
    pub initial_delay_seconds: Option<i32>,
    /// How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240. Must be greater or equal than timeout_seconds.
    #[serde(rename="periodSeconds")]
    
    pub period_seconds: Option<i32>,
    /// Minimum consecutive successes for the probe to be considered successful after having failed. Must be 1 if set.
    #[serde(rename="successThreshold")]
    
    pub success_threshold: Option<i32>,
    /// TCPSocket specifies an action involving a TCP port.
    #[serde(rename="tcpSocket")]
    
    pub tcp_socket: Option<TCPSocketAction>,
    /// Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. Maximum value is 3600. Must be smaller than period_seconds. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[serde(rename="timeoutSeconds")]
    
    pub timeout_seconds: Option<i32>,
}

impl client::Part for Probe {}


/// A DNS resource record.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResourceRecord {
    /// Relative name of the object affected by this record. Only applicable for `CNAME` records. Example: 'www'.
    
    pub name: Option<String>,
    /// Data for this record. Values vary by record type, as defined in RFC 1035 (section 5) and RFC 1034 (section 3.6.1).
    
    pub rrdata: Option<String>,
    /// Resource record type. Example: `AAAA`.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for ResourceRecord {}


/// ResourceRequirements describes the compute resource requirements.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResourceRequirements {
    /// Only memory and CPU are supported. Limits describes the maximum amount of compute resources allowed. The values of the map is string form of the 'quantity' k8s type: https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/apimachinery/pkg/api/resource/quantity.go
    
    pub limits: Option<HashMap<String, String>>,
    /// Only memory and CPU are supported. Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. The values of the map is string form of the 'quantity' k8s type: https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/apimachinery/pkg/api/resource/quantity.go
    
    pub requests: Option<HashMap<String, String>>,
}

impl client::Part for ResourceRequirements {}


/// Revision is an immutable snapshot of code and configuration. A revision references a container image. Revisions are created by updates to a Configuration. See also: https://github.com/knative/specs/blob/main/specs/serving/overview.md#revision
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [revisions get namespaces](NamespaceRevisionGetCall) (response)
/// * [locations revisions get projects](ProjectLocationRevisionGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Revision {
    /// The API version for this call such as "serving.knative.dev/v1".
    #[serde(rename="apiVersion")]
    
    pub api_version: Option<String>,
    /// The kind of this resource, in this case "Revision".
    
    pub kind: Option<String>,
    /// Metadata associated with this Revision, including name, namespace, labels, and annotations.
    
    pub metadata: Option<ObjectMeta>,
    /// Spec holds the desired state of the Revision (from the client).
    
    pub spec: Option<RevisionSpec>,
    /// Status communicates the observed state of the Revision (from the controller).
    
    pub status: Option<RevisionStatus>,
}

impl client::ResponseResult for Revision {}


/// RevisionSpec holds the desired state of the Revision (from the client).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RevisionSpec {
    /// ContainerConcurrency specifies the maximum allowed in-flight (concurrent) requests per container instance of the Revision. If not specified, defaults to 80.
    #[serde(rename="containerConcurrency")]
    
    pub container_concurrency: Option<i32>,
    /// Containers holds the single container that defines the unit of execution for this Revision. In the context of a Revision, we disallow a number of fields on this Container, including: name and lifecycle. In Cloud Run, only a single container may be provided. The runtime contract is documented here: https://github.com/knative/specs/blob/main/specs/serving/runtime-contract.md
    
    pub containers: Option<Vec<Container>>,
    /// Not supported by Cloud Run.
    #[serde(rename="enableServiceLinks")]
    
    pub enable_service_links: Option<bool>,
    /// Not supported by Cloud Run.
    #[serde(rename="imagePullSecrets")]
    
    pub image_pull_secrets: Option<Vec<LocalObjectReference>>,
    /// Email address of the IAM service account associated with the revision of the service. The service account represents the identity of the running revision, and determines what permissions the revision has. If not provided, the revision will use the project's default service account.
    #[serde(rename="serviceAccountName")]
    
    pub service_account_name: Option<String>,
    /// TimeoutSeconds holds the max duration the instance is allowed for responding to a request. Cloud Run: defaults to 300 seconds (5 minutes). Maximum allowed value is 3600 seconds (1 hour).
    #[serde(rename="timeoutSeconds")]
    
    pub timeout_seconds: Option<i32>,
    /// no description provided
    
    pub volumes: Option<Vec<Volume>>,
}

impl client::Part for RevisionSpec {}


/// RevisionStatus communicates the observed state of the Revision (from the controller).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RevisionStatus {
    /// Conditions communicate information about ongoing/complete reconciliation processes that bring the "spec" inline with the observed state of the world. As a Revision is being prepared, it will incrementally update conditions. Revision-specific conditions include: * `ResourcesAvailable`: `True` when underlying resources have been provisioned. * `ContainerHealthy`: `True` when the Revision readiness check completes. * `Active`: `True` when the Revision may receive traffic.
    
    pub conditions: Option<Vec<GoogleCloudRunV1Condition>>,
    /// ImageDigest holds the resolved digest for the image specified within .Spec.Container.Image. The digest is resolved during the creation of Revision. This field holds the digest value regardless of whether a tag or digest was originally specified in the Container object.
    #[serde(rename="imageDigest")]
    
    pub image_digest: Option<String>,
    /// Optional. Specifies the generated logging url for this particular revision based on the revision url template specified in the controller's config.
    #[serde(rename="logUrl")]
    
    pub log_url: Option<String>,
    /// ObservedGeneration is the 'Generation' of the Revision that was last processed by the controller. Clients polling for completed reconciliation should poll until observedGeneration = metadata.generation, and the Ready condition's status is True or False.
    #[serde(rename="observedGeneration")]
    
    pub observed_generation: Option<i32>,
    /// Not currently used by Cloud Run.
    #[serde(rename="serviceName")]
    
    pub service_name: Option<String>,
}

impl client::Part for RevisionStatus {}


/// RevisionTemplateSpec describes the data a revision should have when created from a template. Based on: https://github.com/kubernetes/api/blob/e771f807/core/v1/types.go#L3179-L3190
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RevisionTemplate {
    /// Optional metadata for this Revision, including labels and annotations. Name will be generated by the Configuration. The following annotation keys set properties of the created revision: * `autoscaling.knative.dev/minScale` sets the minimum number of instances. * `autoscaling.knative.dev/maxScale` sets the maximum number of instances. * `run.googleapis.com/cloudsql-instances` sets Cloud SQL connections. Multiple values should be comma separated. * `run.googleapis.com/vpc-access-connector` sets a Serverless VPC Access connector. * `run.googleapis.com/vpc-access-egress` sets VPC egress. Supported values are `all-traffic`, `all` (deprecated), and `private-ranges-only`. `all-traffic` and `all` provide the same functionality. `all` is deprecated but will continue to be supported. Prefer `all-traffic`.
    
    pub metadata: Option<ObjectMeta>,
    /// RevisionSpec holds the desired state of the Revision (from the client).
    
    pub spec: Option<RevisionSpec>,
}

impl client::Part for RevisionTemplate {}


/// Route is responsible for configuring ingress over a collection of Revisions. Some of the Revisions a Route distributes traffic over may be specified by referencing the Configuration responsible for creating them; in these cases the Route is additionally responsible for monitoring the Configuration for “latest ready” revision changes, and smoothly rolling out latest revisions. See also: https://github.com/knative/specs/blob/main/specs/serving/overview.md#route Cloud Run currently supports referencing a single Configuration to automatically deploy the “latest ready” Revision from that Configuration.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [routes get namespaces](NamespaceRouteGetCall) (response)
/// * [locations routes get projects](ProjectLocationRouteGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Route {
    /// The API version for this call such as "serving.knative.dev/v1".
    #[serde(rename="apiVersion")]
    
    pub api_version: Option<String>,
    /// The kind of this resource, in this case always "Route".
    
    pub kind: Option<String>,
    /// Metadata associated with this Route, including name, namespace, labels, and annotations.
    
    pub metadata: Option<ObjectMeta>,
    /// Spec holds the desired state of the Route (from the client).
    
    pub spec: Option<RouteSpec>,
    /// Status communicates the observed state of the Route (from the controller).
    
    pub status: Option<RouteStatus>,
}

impl client::ResponseResult for Route {}


/// RouteSpec holds the desired state of the Route (from the client).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RouteSpec {
    /// Traffic specifies how to distribute traffic over a collection of Knative Revisions and Configurations. Cloud Run currently supports a single configurationName.
    
    pub traffic: Option<Vec<TrafficTarget>>,
}

impl client::Part for RouteSpec {}


/// RouteStatus communicates the observed state of the Route (from the controller).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RouteStatus {
    /// Similar to url, information on where the service is available on HTTP.
    
    pub address: Option<Addressable>,
    /// Conditions communicates information about ongoing/complete reconciliation processes that bring the "spec" inline with the observed state of the world.
    
    pub conditions: Option<Vec<GoogleCloudRunV1Condition>>,
    /// ObservedGeneration is the 'Generation' of the Route that was last processed by the controller. Clients polling for completed reconciliation should poll until observedGeneration = metadata.generation and the Ready condition's status is True or False. Note that providing a TrafficTarget that has latest_revision=True will result in a Route that does not increment either its metadata.generation or its observedGeneration, as new "latest ready" revisions from the Configuration are processed without an update to the Route's spec.
    #[serde(rename="observedGeneration")]
    
    pub observed_generation: Option<i32>,
    /// Traffic holds the configured traffic distribution. These entries will always contain RevisionName references. When ConfigurationName appears in the spec, this will hold the LatestReadyRevisionName that was last observed.
    
    pub traffic: Option<Vec<TrafficTarget>>,
    /// URL holds the url that will distribute traffic over the provided traffic targets. It generally has the form: https://{route-hash}-{project-hash}-{cluster-level-suffix}.a.run.app
    
    pub url: Option<String>,
}

impl client::Part for RouteStatus {}


/// Request message for creating a new execution of a job.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [jobs run namespaces](NamespaceJobRunCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RunJobRequest { _never_set: Option<bool> }

impl client::RequestValue for RunJobRequest {}


/// Not supported by Cloud Run. SecretEnvSource selects a Secret to populate the environment variables with. The contents of the target Secret's Data field will represent the key-value pairs as environment variables.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SecretEnvSource {
    /// This field should not be used directly as it is meant to be inlined directly into the message. Use the "name" field instead.
    #[serde(rename="localObjectReference")]
    
    pub local_object_reference: Option<LocalObjectReference>,
    /// The Secret to select from.
    
    pub name: Option<String>,
    /// Specify whether the Secret must be defined
    
    pub optional: Option<bool>,
}

impl client::Part for SecretEnvSource {}


/// SecretKeySelector selects a key of a Secret.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SecretKeySelector {
    /// Required. A Cloud Secret Manager secret version. Must be 'latest' for the latest version, an integer for a specific version, or a version alias. The key of the secret to select from. Must be a valid secret key.
    
    pub key: Option<String>,
    /// This field should not be used directly as it is meant to be inlined directly into the message. Use the "name" field instead.
    #[serde(rename="localObjectReference")]
    
    pub local_object_reference: Option<LocalObjectReference>,
    /// The name of the secret in Cloud Secret Manager. By default, the secret is assumed to be in the same project. If the secret is in another project, you must define an alias. An alias definition has the form: :projects//secrets/. If multiple alias definitions are needed, they must be separated by commas. The alias definitions must be set on the run.googleapis.com/secrets annotation. The name of the secret in the pod's namespace to select from.
    
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined.
    
    pub optional: Option<bool>,
}

impl client::Part for SecretKeySelector {}


/// A volume representing a secret stored in Google Secret Manager. The secret's value will be presented as the content of a file whose name is defined in the item path. If no items are defined, the name of the file is the secret_name. The contents of the target Secret's Data field will be presented in a volume as files using the keys in the Data field as the file names.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SecretVolumeSource {
    /// Integer representation of mode bits to use on created files by default. Must be a value between 01 and 0777 (octal). If 0 or not set, it will default to 0444. Directories within the path are not affected by this setting. Notes * Internally, a umask of 0222 will be applied to any non-zero value. * This is an integer representation of the mode bits. So, the octal integer value should look exactly as the chmod numeric notation with a leading zero. Some examples: for chmod 777 (a=rwx), set to 0777 (octal) or 511 (base-10). For chmod 640 (u=rw,g=r), set to 0640 (octal) or 416 (base-10). For chmod 755 (u=rwx,g=rx,o=rx), set to 0755 (octal) or 493 (base-10). * This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    #[serde(rename="defaultMode")]
    
    pub default_mode: Option<i32>,
    /// A list of secret versions to mount in the volume. If no items are specified, the volume will expose a file with the same name as the secret name. The contents of the file will be the data in the latest version of the secret. If items are specified, the key will be used as the version to fetch from Cloud Secret Manager and the path will be the name of the file exposed in the volume. When items are defined, they must specify both a key and a path.
    
    pub items: Option<Vec<KeyToPath>>,
    /// Not supported by Cloud Run.
    
    pub optional: Option<bool>,
    /// The name of the secret in Cloud Secret Manager. By default, the secret is assumed to be in the same project. If the secret is in another project, you must define an alias. An alias definition has the form: :projects//secrets/. If multiple alias definitions are needed, they must be separated by commas. The alias definitions must be set on the run.googleapis.com/secrets annotation. Name of the secret in the container's namespace to use.
    #[serde(rename="secretName")]
    
    pub secret_name: Option<String>,
}

impl client::Part for SecretVolumeSource {}


/// Not supported by Cloud Run. SecurityContext holds security configuration that will be applied to a container. Some fields are present in both SecurityContext and PodSecurityContext. When both are set, the values in SecurityContext take precedence.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SecurityContext {
    /// The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in PodSecurityContext. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    #[serde(rename="runAsUser")]
    
    pub run_as_user: Option<i32>,
}

impl client::Part for SecurityContext {}


/// Service acts as a top-level container that manages a set of Routes and Configurations which implement a network service. Service exists to provide a singular abstraction which can be access controlled, reasoned about, and which encapsulates software lifecycle decisions such as rollout policy and team resource ownership. Service acts only as an orchestrator of the underlying Routes and Configurations (much as a kubernetes Deployment orchestrates ReplicaSets). The Service’s controller will track the statuses of its owned Configuration and Route, reflecting their statuses and conditions as its own. See also: https://github.com/knative/serving/blob/main/docs/spec/overview.md#service
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [services create namespaces](NamespaceServiceCreateCall) (request|response)
/// * [services get namespaces](NamespaceServiceGetCall) (response)
/// * [services replace service namespaces](NamespaceServiceReplaceServiceCall) (request|response)
/// * [locations services create projects](ProjectLocationServiceCreateCall) (request|response)
/// * [locations services get projects](ProjectLocationServiceGetCall) (response)
/// * [locations services replace service projects](ProjectLocationServiceReplaceServiceCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Service {
    /// The API version for this call. It must be "serving.knative.dev/v1".
    #[serde(rename="apiVersion")]
    
    pub api_version: Option<String>,
    /// The kind of resource. It must be "Service".
    
    pub kind: Option<String>,
    /// Metadata associated with this Service, including name, namespace, labels, and annotations. In Cloud Run, annotations with ‘run.googleapis.com/’ and ‘autoscaling.knative.dev’ are restricted, and the accepted annotations will be different depending on the resource type. The following Cloud Run-specific annotations are accepted in Service.metadata.annotations. * `run.googleapis.com/binary-authorization-breakglass` * `run.googleapis.com/binary-authorization` * `run.googleapis.com/client-name` * `run.googleapis.com/custom-audiences` * `run.googleapis.com/description` * `run.googleapis.com/gc-traffic-tags` * `run.googleapis.com/ingress` * `run.googleapis.com/ingress` sets the ingress settings for the Service. See [the ingress settings documentation](https://cloud.google.com/run/docs/securing/ingress) for details on configuring ingress settings. * `run.googleapis.com/ingress-status` is output-only and contains the currently active ingress settings for the Service. `run.googleapis.com/ingress-status` may differ from `run.googleapis.com/ingress` while the system is processing a change to `run.googleapis.com/ingress` or if the system failed to process a change to `run.googleapis.com/ingress`. When the system has processed all changes successfully `run.googleapis.com/ingress-status` and `run.googleapis.com/ingress` are equal.
    
    pub metadata: Option<ObjectMeta>,
    /// Holds the desired state of the Service (from the client).
    
    pub spec: Option<ServiceSpec>,
    /// Communicates the system-controlled state of the Service.
    
    pub status: Option<ServiceStatus>,
}

impl client::RequestValue for Service {}
impl client::ResponseResult for Service {}


/// ServiceSpec holds the desired state of the Route (from the client), which is used to manipulate the underlying Route and Configuration(s).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceSpec {
    /// Holds the latest specification for the Revision to be stamped out.
    
    pub template: Option<RevisionTemplate>,
    /// Specifies how to distribute traffic over a collection of Knative Revisions and Configurations to the Service's main URL.
    
    pub traffic: Option<Vec<TrafficTarget>>,
}

impl client::Part for ServiceSpec {}


/// The current state of the Service. Output only.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceStatus {
    /// Similar to url, information on where the service is available on HTTP.
    
    pub address: Option<Addressable>,
    /// Conditions communicate information about ongoing/complete reconciliation processes that bring the `spec` inline with the observed state of the world. Service-specific conditions include: * `ConfigurationsReady`: `True` when the underlying Configuration is ready. * `RoutesReady`: `True` when the underlying Route is ready. * `Ready`: `True` when all underlying resources are ready.
    
    pub conditions: Option<Vec<GoogleCloudRunV1Condition>>,
    /// Name of the last revision that was created from this Service's Configuration. It might not be ready yet, for that use LatestReadyRevisionName.
    #[serde(rename="latestCreatedRevisionName")]
    
    pub latest_created_revision_name: Option<String>,
    /// Name of the latest Revision from this Service's Configuration that has had its `Ready` condition become `True`.
    #[serde(rename="latestReadyRevisionName")]
    
    pub latest_ready_revision_name: Option<String>,
    /// Returns the generation last fully processed by the system. This will only match metadata.generation when reconciliation is complete. Clients polling for completed reconciliation should poll until observedGeneration = metadata.generation and the Ready condition's status is True or False.
    #[serde(rename="observedGeneration")]
    
    pub observed_generation: Option<i32>,
    /// Holds the configured traffic distribution. These entries will always contain RevisionName references. When ConfigurationName appears in the spec, this will hold the LatestReadyRevisionName that we last observed.
    
    pub traffic: Option<Vec<TrafficTarget>>,
    /// URL that will distribute traffic over the provided traffic targets. It generally has the form https://{route-hash}-{project-hash}-{cluster-level-suffix}.a.run.app
    
    pub url: Option<String>,
}

impl client::Part for ServiceStatus {}


/// Request message for `SetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations jobs set iam policy projects](ProjectLocationJobSetIamPolicyCall) (request)
/// * [locations services set iam policy projects](ProjectLocationServiceSetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetIamPolicyRequest {
    /// REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them.
    
    pub policy: Option<Policy>,
    /// OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: "bindings, etag"`
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for SetIamPolicyRequest {}


/// Status is a return value for calls that don’t return other objects.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [domainmappings delete namespaces](NamespaceDomainmappingDeleteCall) (response)
/// * [executions delete namespaces](NamespaceExecutionDeleteCall) (response)
/// * [jobs delete namespaces](NamespaceJobDeleteCall) (response)
/// * [revisions delete namespaces](NamespaceRevisionDeleteCall) (response)
/// * [services delete namespaces](NamespaceServiceDeleteCall) (response)
/// * [locations domainmappings delete projects](ProjectLocationDomainmappingDeleteCall) (response)
/// * [locations revisions delete projects](ProjectLocationRevisionDeleteCall) (response)
/// * [locations services delete projects](ProjectLocationServiceDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Status {
    /// Suggested HTTP return code for this status, 0 if not set.
    
    pub code: Option<i32>,
    /// Extended data associated with the reason. Each reason may define its own extended details. This field is optional and the data returned is not guaranteed to conform to any schema except that defined by the reason type.
    
    pub details: Option<StatusDetails>,
    /// A human-readable description of the status of this operation.
    
    pub message: Option<String>,
    /// Standard list metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    
    pub metadata: Option<ListMeta>,
    /// A machine-readable description of why this operation is in the "Failure" status. If this value is empty there is no information available. A Reason clarifies an HTTP status code but does not override it.
    
    pub reason: Option<String>,
    /// Status of the operation. One of: "Success" or "Failure". More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    
    pub status: Option<String>,
}

impl client::ResponseResult for Status {}


/// StatusCause provides more information about an api.Status failure, including cases when multiple errors are encountered.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StatusCause {
    /// The field of the resource that has caused this error, as named by its JSON serialization. May include dot and postfix notation for nested attributes. Arrays are zero-indexed. Fields may appear more than once in an array of causes due to fields having multiple errors. Examples: "name" - the field "name" on the current resource "items[0].name" - the field "name" on the first array entry in "items"
    
    pub field: Option<String>,
    /// A human-readable description of the cause of the error. This field may be presented as-is to a reader.
    
    pub message: Option<String>,
    /// A machine-readable description of the cause of the error. If this value is empty there is no information available.
    
    pub reason: Option<String>,
}

impl client::Part for StatusCause {}


/// StatusDetails is a set of additional properties that MAY be set by the server to provide additional information about a response. The Reason field of a Status object defines what attributes will be set. Clients must ignore fields that do not match the defined type of each attribute, and should assume that any attribute may be empty, invalid, or under defined.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StatusDetails {
    /// The Causes array includes more details associated with the StatusReason failure. Not all StatusReasons may provide detailed causes.
    
    pub causes: Option<Vec<StatusCause>>,
    /// The group attribute of the resource associated with the status StatusReason.
    
    pub group: Option<String>,
    /// The kind attribute of the resource associated with the status StatusReason. On some operations may differ from the requested resource Kind. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    
    pub kind: Option<String>,
    /// The name attribute of the resource associated with the status StatusReason (when there is a single name which can be described).
    
    pub name: Option<String>,
    /// If specified, the time in seconds before the operation should be retried. Some errors may indicate the client must take an alternate action - for those errors this field may indicate how long to wait before taking the alternate action.
    #[serde(rename="retryAfterSeconds")]
    
    pub retry_after_seconds: Option<i32>,
    /// UID of the resource. (when there is a single resource which can be described). More info: https://kubernetes.io/docs/user-guide/identifiers#uids
    
    pub uid: Option<String>,
}

impl client::Part for StatusDetails {}


/// TCPSocketAction describes an action based on opening a socket
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TCPSocketAction {
    /// Not supported by Cloud Run.
    
    pub host: Option<String>,
    /// Port number to access on the container. Number must be in the range 1 to 65535.
    
    pub port: Option<i32>,
}

impl client::Part for TCPSocketAction {}


/// Task represents a single run of a container to completion.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [tasks get namespaces](NamespaceTaskGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    /// Optional. APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
    #[serde(rename="apiVersion")]
    
    pub api_version: Option<String>,
    /// Optional. Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    
    pub kind: Option<String>,
    /// Optional. Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    
    pub metadata: Option<ObjectMeta>,
    /// Optional. Specification of the desired behavior of a task. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    
    pub spec: Option<TaskSpec>,
    /// Output only. Current status of a task. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    
    pub status: Option<TaskStatus>,
}

impl client::ResponseResult for Task {}


/// Result of a task attempt.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TaskAttemptResult {
    /// Optional. The exit code of this attempt. This may be unset if the container was unable to exit cleanly with a code due to some other failure. See status field for possible failure details.
    #[serde(rename="exitCode")]
    
    pub exit_code: Option<i32>,
    /// Optional. The status of this attempt. If the status code is OK, then the attempt succeeded.
    
    pub status: Option<GoogleRpcStatus>,
}

impl client::Part for TaskAttemptResult {}


/// TaskSpec is a description of a task.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TaskSpec {
    /// Optional. List of containers belonging to the task. We disallow a number of fields on this Container. Only a single container may be provided.
    
    pub containers: Option<Vec<Container>>,
    /// Optional. Number of retries allowed per task, before marking this job failed.
    #[serde(rename="maxRetries")]
    
    pub max_retries: Option<i32>,
    /// Optional. Email address of the IAM service account associated with the task of a job execution. The service account represents the identity of the running task, and determines what permissions the task has. If not provided, the task will use the project's default service account.
    #[serde(rename="serviceAccountName")]
    
    pub service_account_name: Option<String>,
    /// Optional. Duration in seconds the task may be active before the system will actively try to mark it failed and kill associated containers. This applies per attempt of a task, meaning each retry can run for the full timeout.
    #[serde(rename="timeoutSeconds")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub timeout_seconds: Option<i64>,
    /// Optional. List of volumes that can be mounted by containers belonging to the task. More info: https://kubernetes.io/docs/concepts/storage/volumes
    
    pub volumes: Option<Vec<Volume>>,
}

impl client::Part for TaskSpec {}


/// TaskStatus represents the status of a task.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TaskStatus {
    /// Optional. Represents time when the task was completed. It is not guaranteed to be set in happens-before order across separate operations. It is represented in RFC3339 form and is in UTC.
    #[serde(rename="completionTime")]
    
    pub completion_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Conditions communicate information about ongoing/complete reconciliation processes that bring the "spec" inline with the observed state of the world. Task-specific conditions include: * `Started`: `True` when the task has started to execute. * `Completed`: `True` when the task has succeeded. `False` when the task has failed.
    
    pub conditions: Option<Vec<GoogleCloudRunV1Condition>>,
    /// Required. Index of the task, unique per execution, and beginning at 0.
    
    pub index: Option<i32>,
    /// Optional. Result of the last attempt of this task.
    #[serde(rename="lastAttemptResult")]
    
    pub last_attempt_result: Option<TaskAttemptResult>,
    /// Optional. URI where logs for this task can be found in Cloud Console.
    #[serde(rename="logUri")]
    
    pub log_uri: Option<String>,
    /// Optional. The 'generation' of the task that was last processed by the controller.
    #[serde(rename="observedGeneration")]
    
    pub observed_generation: Option<i32>,
    /// Optional. The number of times this task was retried. Instances are retried when they fail up to the maxRetries limit.
    
    pub retried: Option<i32>,
    /// Optional. Represents time when the task started to run. It is not guaranteed to be set in happens-before order across separate operations. It is represented in RFC3339 form and is in UTC.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for TaskStatus {}


/// TaskTemplateSpec describes the data a task should have when created from a template.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TaskTemplateSpec {
    /// Optional. Specification of the desired behavior of the task. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    
    pub spec: Option<TaskSpec>,
}

impl client::Part for TaskTemplateSpec {}


/// Request message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations jobs test iam permissions projects](ProjectLocationJobTestIamPermissionCall) (request)
/// * [locations services test iam permissions projects](ProjectLocationServiceTestIamPermissionCall) (request)
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
/// * [locations jobs test iam permissions projects](ProjectLocationJobTestIamPermissionCall) (response)
/// * [locations services test iam permissions projects](ProjectLocationServiceTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsResponse {
    /// A subset of `TestPermissionsRequest.permissions` that the caller is allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for TestIamPermissionsResponse {}


/// TrafficTarget holds a single entry of the routing table for a Route.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TrafficTarget {
    /// [Deprecated] Not supported in Cloud Run. It must be empty.
    #[serde(rename="configurationName")]
    
    pub configuration_name: Option<String>,
    /// Uses the "status.latestReadyRevisionName" of the Service to determine the traffic target. When it changes, traffic will automatically migrate from the prior "latest ready" revision to the new one. This field must be false if RevisionName is set. This field defaults to true otherwise. If the field is set to true on Status, this means that the Revision was resolved from the Service's latest ready revision.
    #[serde(rename="latestRevision")]
    
    pub latest_revision: Option<bool>,
    /// Percent specifies percent of the traffic to this Revision or Configuration. This defaults to zero if unspecified.
    
    pub percent: Option<i32>,
    /// Points this traffic target to a specific Revision. This field is mutually exclusive with latest_revision.
    #[serde(rename="revisionName")]
    
    pub revision_name: Option<String>,
    /// Tag is used to expose a dedicated url for referencing this target exclusively.
    
    pub tag: Option<String>,
    /// Output only. URL displays the URL for accessing tagged traffic targets. URL is displayed in status, and is disallowed on spec. URL must contain a scheme (e.g. https://) and a hostname, but may not contain anything else (e.g. basic auth, url path, etc.)
    
    pub url: Option<String>,
}

impl client::Part for TrafficTarget {}


/// Volume represents a named volume in a container.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Volume {
    /// Not supported in Cloud Run.
    #[serde(rename="configMap")]
    
    pub config_map: Option<ConfigMapVolumeSource>,
    /// Volume's name. In Cloud Run Fully Managed, the name 'cloudsql' is reserved.
    
    pub name: Option<String>,
    /// The secret's value will be presented as the content of a file whose name is defined in the item path. If no items are defined, the name of the file is the secretName.
    
    pub secret: Option<SecretVolumeSource>,
}

impl client::Part for Volume {}


/// VolumeMount describes a mounting of a Volume within a container.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeMount {
    /// Required. Path within the container at which the volume should be mounted. Must not contain ':'.
    #[serde(rename="mountPath")]
    
    pub mount_path: Option<String>,
    /// Required. The name of the volume. There must be a corresponding Volume with the same name.
    
    pub name: Option<String>,
    /// Only true is accepted for Secret Volumes. Defaults to true for Secrets Volumes.
    #[serde(rename="readOnly")]
    
    pub read_only: Option<bool>,
    /// Path within the volume from which the container's volume should be mounted. Defaults to "" (volume's root).
    #[serde(rename="subPath")]
    
    pub sub_path: Option<String>,
}

impl client::Part for VolumeMount {}


