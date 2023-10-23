use super::*;
/// Specifies the type and number of accelerator cards attached to the instances of an instance. See GPUs on Compute Engine (https://cloud.google.com/compute/docs/gpus/).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AcceleratorConfig {
    /// The number of the accelerator cards of this type exposed to this instance.
    #[serde(rename="acceleratorCount")]
    
    pub accelerator_count: Option<i32>,
    /// Full URL, partial URI, or short name of the accelerator type resource to expose to this instance. See Compute Engine AcceleratorTypes (https://cloud.google.com/compute/docs/reference/beta/acceleratorTypes).Examples: https://www.googleapis.com/compute/beta/projects/[project_id]/zones/us-east1-a/acceleratorTypes/nvidia-tesla-k80 projects/[project_id]/zones/us-east1-a/acceleratorTypes/nvidia-tesla-k80 nvidia-tesla-k80Auto Zone Exception: If you are using the Dataproc Auto Zone Placement (https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/auto-zone#using_auto_zone_placement) feature, you must use the short name of the accelerator type resource, for example, nvidia-tesla-k80.
    #[serde(rename="acceleratorTypeUri")]
    
    pub accelerator_type_uri: Option<String>,
}

impl client::Part for AcceleratorConfig {}


/// Autoscaling Policy config associated with the cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AutoscalingConfig {
    /// Optional. The autoscaling policy used by the cluster.Only resource names including projectid and location (region) are valid. Examples: https://www.googleapis.com/compute/v1/projects/[project_id]/locations/[dataproc_region]/autoscalingPolicies/[policy_id] projects/[project_id]/locations/[dataproc_region]/autoscalingPolicies/[policy_id]Note that the policy must be in the same project and Dataproc region.
    #[serde(rename="policyUri")]
    
    pub policy_uri: Option<String>,
}

impl client::Part for AutoscalingConfig {}


/// Describes an autoscaling policy for Dataproc cluster autoscaler.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations autoscaling policies create projects](ProjectLocationAutoscalingPolicyCreateCall) (request|response)
/// * [locations autoscaling policies get projects](ProjectLocationAutoscalingPolicyGetCall) (response)
/// * [locations autoscaling policies update projects](ProjectLocationAutoscalingPolicyUpdateCall) (request|response)
/// * [regions autoscaling policies create projects](ProjectRegionAutoscalingPolicyCreateCall) (request|response)
/// * [regions autoscaling policies get projects](ProjectRegionAutoscalingPolicyGetCall) (response)
/// * [regions autoscaling policies update projects](ProjectRegionAutoscalingPolicyUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AutoscalingPolicy {
    /// no description provided
    #[serde(rename="basicAlgorithm")]
    
    pub basic_algorithm: Option<BasicAutoscalingAlgorithm>,
    /// Required. The policy id.The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), and hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of between 3 and 50 characters.
    
    pub id: Option<String>,
    /// Optional. The labels to associate with this autoscaling policy. Label keys must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). Label values may be empty, but, if present, must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). No more than 32 labels can be associated with an autoscaling policy.
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. The "resource name" of the autoscaling policy, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.autoscalingPolicies, the resource name of the policy has the following format: projects/{project_id}/regions/{region}/autoscalingPolicies/{policy_id} For projects.locations.autoscalingPolicies, the resource name of the policy has the following format: projects/{project_id}/locations/{location}/autoscalingPolicies/{policy_id}
    
    pub name: Option<String>,
    /// Optional. Describes how the autoscaler will operate for secondary workers.
    #[serde(rename="secondaryWorkerConfig")]
    
    pub secondary_worker_config: Option<InstanceGroupAutoscalingPolicyConfig>,
    /// Required. Describes how the autoscaler will operate for primary workers.
    #[serde(rename="workerConfig")]
    
    pub worker_config: Option<InstanceGroupAutoscalingPolicyConfig>,
}

impl client::RequestValue for AutoscalingPolicy {}
impl client::ResponseResult for AutoscalingPolicy {}


/// Node group identification and configuration information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuxiliaryNodeGroup {
    /// Required. Node group configuration.
    #[serde(rename="nodeGroup")]
    
    pub node_group: Option<NodeGroup>,
    /// Optional. A node group ID. Generated if not specified.The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), and hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of from 3 to 33 characters.
    #[serde(rename="nodeGroupId")]
    
    pub node_group_id: Option<String>,
}

impl client::Part for AuxiliaryNodeGroup {}


/// Auxiliary services configuration for a Cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuxiliaryServicesConfig {
    /// Optional. The Hive Metastore configuration for this workload.
    #[serde(rename="metastoreConfig")]
    
    pub metastore_config: Option<MetastoreConfig>,
    /// Optional. The Spark History Server configuration for the workload.
    #[serde(rename="sparkHistoryServerConfig")]
    
    pub spark_history_server_config: Option<SparkHistoryServerConfig>,
}

impl client::Part for AuxiliaryServicesConfig {}


/// Basic algorithm for autoscaling.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BasicAutoscalingAlgorithm {
    /// Optional. Duration between scaling events. A scaling period starts after the update operation from the previous event has completed.Bounds: 2m, 1d. Default: 2m.
    #[serde(rename="cooldownPeriod")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub cooldown_period: Option<client::chrono::Duration>,
    /// Optional. Spark Standalone autoscaling configuration
    #[serde(rename="sparkStandaloneConfig")]
    
    pub spark_standalone_config: Option<SparkStandaloneAutoscalingConfig>,
    /// Optional. YARN autoscaling configuration.
    #[serde(rename="yarnConfig")]
    
    pub yarn_config: Option<BasicYarnAutoscalingConfig>,
}

impl client::Part for BasicAutoscalingAlgorithm {}


/// Basic autoscaling configurations for YARN.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BasicYarnAutoscalingConfig {
    /// Required. Timeout for YARN graceful decommissioning of Node Managers. Specifies the duration to wait for jobs to complete before forcefully removing workers (and potentially interrupting jobs). Only applicable to downscaling operations.Bounds: 0s, 1d.
    #[serde(rename="gracefulDecommissionTimeout")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub graceful_decommission_timeout: Option<client::chrono::Duration>,
    /// Required. Fraction of average YARN pending memory in the last cooldown period for which to remove workers. A scale-down factor of 1 will result in scaling down so that there is no available memory remaining after the update (more aggressive scaling). A scale-down factor of 0 disables removing workers, which can be beneficial for autoscaling a single job. See How autoscaling works (https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/autoscaling#how_autoscaling_works) for more information.Bounds: 0.0, 1.0.
    #[serde(rename="scaleDownFactor")]
    
    pub scale_down_factor: Option<f64>,
    /// Optional. Minimum scale-down threshold as a fraction of total cluster size before scaling occurs. For example, in a 20-worker cluster, a threshold of 0.1 means the autoscaler must recommend at least a 2 worker scale-down for the cluster to scale. A threshold of 0 means the autoscaler will scale down on any recommended change.Bounds: 0.0, 1.0. Default: 0.0.
    #[serde(rename="scaleDownMinWorkerFraction")]
    
    pub scale_down_min_worker_fraction: Option<f64>,
    /// Required. Fraction of average YARN pending memory in the last cooldown period for which to add workers. A scale-up factor of 1.0 will result in scaling up so that there is no pending memory remaining after the update (more aggressive scaling). A scale-up factor closer to 0 will result in a smaller magnitude of scaling up (less aggressive scaling). See How autoscaling works (https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/autoscaling#how_autoscaling_works) for more information.Bounds: 0.0, 1.0.
    #[serde(rename="scaleUpFactor")]
    
    pub scale_up_factor: Option<f64>,
    /// Optional. Minimum scale-up threshold as a fraction of total cluster size before scaling occurs. For example, in a 20-worker cluster, a threshold of 0.1 means the autoscaler must recommend at least a 2-worker scale-up for the cluster to scale. A threshold of 0 means the autoscaler will scale up on any recommended change.Bounds: 0.0, 1.0. Default: 0.0.
    #[serde(rename="scaleUpMinWorkerFraction")]
    
    pub scale_up_min_worker_fraction: Option<f64>,
}

impl client::Part for BasicYarnAutoscalingConfig {}


/// A representation of a batch workload in the service.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations batches create projects](ProjectLocationBatchCreateCall) (request)
/// * [locations batches get projects](ProjectLocationBatchGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Batch {
    /// Output only. The time when the batch was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The email address of the user who created the batch.
    
    pub creator: Option<String>,
    /// Optional. Environment configuration for the batch execution.
    #[serde(rename="environmentConfig")]
    
    pub environment_config: Option<EnvironmentConfig>,
    /// Optional. The labels to associate with this batch. Label keys must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). Label values may be empty, but, if present, must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). No more than 32 labels can be associated with a batch.
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. The resource name of the batch.
    
    pub name: Option<String>,
    /// Output only. The resource name of the operation associated with this batch.
    
    pub operation: Option<String>,
    /// Optional. PySpark batch config.
    #[serde(rename="pysparkBatch")]
    
    pub pyspark_batch: Option<PySparkBatch>,
    /// Optional. Runtime configuration for the batch execution.
    #[serde(rename="runtimeConfig")]
    
    pub runtime_config: Option<RuntimeConfig>,
    /// Output only. Runtime information about batch execution.
    #[serde(rename="runtimeInfo")]
    
    pub runtime_info: Option<RuntimeInfo>,
    /// Optional. Spark batch config.
    #[serde(rename="sparkBatch")]
    
    pub spark_batch: Option<SparkBatch>,
    /// Optional. SparkR batch config.
    #[serde(rename="sparkRBatch")]
    
    pub spark_r_batch: Option<SparkRBatch>,
    /// Optional. SparkSql batch config.
    #[serde(rename="sparkSqlBatch")]
    
    pub spark_sql_batch: Option<SparkSqlBatch>,
    /// Output only. The state of the batch.
    
    pub state: Option<BatchStateEnum>,
    /// Output only. Historical state information for the batch.
    #[serde(rename="stateHistory")]
    
    pub state_history: Option<Vec<StateHistory>>,
    /// Output only. Batch state details, such as a failure description if the state is FAILED.
    #[serde(rename="stateMessage")]
    
    pub state_message: Option<String>,
    /// Output only. The time when the batch entered a current state.
    #[serde(rename="stateTime")]
    
    pub state_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. A batch UUID (Unique Universal Identifier). The service generates this value when it creates the batch.
    
    pub uuid: Option<String>,
}

impl client::RequestValue for Batch {}
impl client::ResponseResult for Batch {}


/// Associates members, or principals, with a role.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Binding {
    /// The condition that is associated with this binding.If the condition evaluates to true, then this binding applies to the current request.If the condition evaluates to false, then this binding does not apply to the current request. However, a different role binding might grant the same role to one or more of the principals in this binding.To learn which resources support conditions in their IAM policies, see the IAM documentation (https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub condition: Option<Expr>,
    /// Specifies the principals requesting access for a Google Cloud resource. members can have the following values: allUsers: A special identifier that represents anyone who is on the internet; with or without a Google account. allAuthenticatedUsers: A special identifier that represents anyone who is authenticated with a Google account or a service account. Does not include identities that come from external identity providers (IdPs) through identity federation. user:{emailid}: An email address that represents a specific Google account. For example, alice@example.com . serviceAccount:{emailid}: An email address that represents a Google service account. For example, my-other-app@appspot.gserviceaccount.com. serviceAccount:{projectid}.svc.id.goog[{namespace}/{kubernetes-sa}]: An identifier for a Kubernetes service account (https://cloud.google.com/kubernetes-engine/docs/how-to/kubernetes-service-accounts). For example, my-project.svc.id.goog[my-namespace/my-kubernetes-sa]. group:{emailid}: An email address that represents a Google group. For example, admins@example.com. deleted:user:{emailid}?uid={uniqueid}: An email address (plus unique identifier) representing a user that has been recently deleted. For example, alice@example.com?uid=123456789012345678901. If the user is recovered, this value reverts to user:{emailid} and the recovered user retains the role in the binding. deleted:serviceAccount:{emailid}?uid={uniqueid}: An email address (plus unique identifier) representing a service account that has been recently deleted. For example, my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901. If the service account is undeleted, this value reverts to serviceAccount:{emailid} and the undeleted service account retains the role in the binding. deleted:group:{emailid}?uid={uniqueid}: An email address (plus unique identifier) representing a Google group that has been recently deleted. For example, admins@example.com?uid=123456789012345678901. If the group is recovered, this value reverts to group:{emailid} and the recovered group retains the role in the binding. domain:{domain}: The G Suite domain (primary) that represents all the users of that domain. For example, google.com or example.com.
    
    pub members: Option<Vec<String>>,
    /// Role that is assigned to the list of members, or principals. For example, roles/viewer, roles/editor, or roles/owner.
    
    pub role: Option<String>,
}

impl client::Part for Binding {}


/// A request to cancel a job.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [regions jobs cancel projects](ProjectRegionJobCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelJobRequest { _never_set: Option<bool> }

impl client::RequestValue for CancelJobRequest {}


/// Describes the identifying information, config, and status of a Dataproc cluster
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [regions clusters create projects](ProjectRegionClusterCreateCall) (request)
/// * [regions clusters get projects](ProjectRegionClusterGetCall) (response)
/// * [regions clusters patch projects](ProjectRegionClusterPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Cluster {
    /// Required. The cluster name, which must be unique within a project. The name must start with a lowercase letter, and can contain up to 51 lowercase letters, numbers, and hyphens. It cannot end with a hyphen. The name of a deleted cluster can be reused.
    #[serde(rename="clusterName")]
    
    pub cluster_name: Option<String>,
    /// Output only. A cluster UUID (Unique Universal Identifier). Dataproc generates this value when it creates the cluster.
    #[serde(rename="clusterUuid")]
    
    pub cluster_uuid: Option<String>,
    /// Optional. The cluster config for a cluster of Compute Engine Instances. Note that Dataproc may set default values, and values may change when clusters are updated.Exactly one of ClusterConfig or VirtualClusterConfig must be specified.
    
    pub config: Option<ClusterConfig>,
    /// Optional. The labels to associate with this cluster. Label keys must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). Label values may be empty, but, if present, must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). No more than 32 labels can be associated with a cluster.
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. Contains cluster daemon metrics such as HDFS and YARN stats.Beta Feature: This report is available for testing purposes only. It may be changed before final release.
    
    pub metrics: Option<ClusterMetrics>,
    /// Required. The Google Cloud Platform project ID that the cluster belongs to.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Output only. Cluster status.
    
    pub status: Option<ClusterStatus>,
    /// Output only. The previous cluster status.
    #[serde(rename="statusHistory")]
    
    pub status_history: Option<Vec<ClusterStatus>>,
    /// Optional. The virtual cluster config is used when creating a Dataproc cluster that does not directly control the underlying compute resources, for example, when creating a Dataproc-on-GKE cluster (https://cloud.google.com/dataproc/docs/guides/dpgke/dataproc-gke). Dataproc may set default values, and values may change when clusters are updated. Exactly one of config or virtual_cluster_config must be specified.
    #[serde(rename="virtualClusterConfig")]
    
    pub virtual_cluster_config: Option<VirtualClusterConfig>,
}

impl client::RequestValue for Cluster {}
impl client::ResponseResult for Cluster {}


/// The cluster config.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClusterConfig {
    /// Optional. Autoscaling config for the policy associated with the cluster. Cluster does not autoscale if this field is unset.
    #[serde(rename="autoscalingConfig")]
    
    pub autoscaling_config: Option<AutoscalingConfig>,
    /// Optional. The node group settings.
    #[serde(rename="auxiliaryNodeGroups")]
    
    pub auxiliary_node_groups: Option<Vec<AuxiliaryNodeGroup>>,
    /// Optional. A Cloud Storage bucket used to stage job dependencies, config files, and job driver console output. If you do not specify a staging bucket, Cloud Dataproc will determine a Cloud Storage location (US, ASIA, or EU) for your cluster's staging bucket according to the Compute Engine zone where your cluster is deployed, and then create and manage this project-level, per-location bucket (see Dataproc staging and temp buckets (https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/staging-bucket)). This field requires a Cloud Storage bucket name, not a gs://... URI to a Cloud Storage bucket.
    #[serde(rename="configBucket")]
    
    pub config_bucket: Option<String>,
    /// Optional. The config for Dataproc metrics.
    #[serde(rename="dataprocMetricConfig")]
    
    pub dataproc_metric_config: Option<DataprocMetricConfig>,
    /// Optional. Encryption settings for the cluster.
    #[serde(rename="encryptionConfig")]
    
    pub encryption_config: Option<EncryptionConfig>,
    /// Optional. Port/endpoint configuration for this cluster
    #[serde(rename="endpointConfig")]
    
    pub endpoint_config: Option<EndpointConfig>,
    /// Optional. The shared Compute Engine config settings for all instances in a cluster.
    #[serde(rename="gceClusterConfig")]
    
    pub gce_cluster_config: Option<GceClusterConfig>,
    /// Optional. BETA. The Kubernetes Engine config for Dataproc clusters deployed to The Kubernetes Engine config for Dataproc clusters deployed to Kubernetes. These config settings are mutually exclusive with Compute Engine-based options, such as gce_cluster_config, master_config, worker_config, secondary_worker_config, and autoscaling_config.
    #[serde(rename="gkeClusterConfig")]
    
    pub gke_cluster_config: Option<GkeClusterConfig>,
    /// Optional. Commands to execute on each node after config is completed. By default, executables are run on master and all worker nodes. You can test a node's role metadata to run an executable on a master or worker node, as shown below using curl (you can also use wget): ROLE=$(curl -H Metadata-Flavor:Google http://metadata/computeMetadata/v1/instance/attributes/dataproc-role) if [[ "${ROLE}" == 'Master' ]]; then ... master specific actions ... else ... worker specific actions ... fi 
    #[serde(rename="initializationActions")]
    
    pub initialization_actions: Option<Vec<NodeInitializationAction>>,
    /// Optional. Lifecycle setting for the cluster.
    #[serde(rename="lifecycleConfig")]
    
    pub lifecycle_config: Option<LifecycleConfig>,
    /// Optional. The Compute Engine config settings for the cluster's master instance.
    #[serde(rename="masterConfig")]
    
    pub master_config: Option<InstanceGroupConfig>,
    /// Optional. Metastore configuration.
    #[serde(rename="metastoreConfig")]
    
    pub metastore_config: Option<MetastoreConfig>,
    /// Optional. The Compute Engine config settings for a cluster's secondary worker instances
    #[serde(rename="secondaryWorkerConfig")]
    
    pub secondary_worker_config: Option<InstanceGroupConfig>,
    /// Optional. Security settings for the cluster.
    #[serde(rename="securityConfig")]
    
    pub security_config: Option<SecurityConfig>,
    /// Optional. The config settings for cluster software.
    #[serde(rename="softwareConfig")]
    
    pub software_config: Option<SoftwareConfig>,
    /// Optional. A Cloud Storage bucket used to store ephemeral cluster and jobs data, such as Spark and MapReduce history files. If you do not specify a temp bucket, Dataproc will determine a Cloud Storage location (US, ASIA, or EU) for your cluster's temp bucket according to the Compute Engine zone where your cluster is deployed, and then create and manage this project-level, per-location bucket. The default bucket has a TTL of 90 days, but you can use any TTL (or none) if you specify a bucket (see Dataproc staging and temp buckets (https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/staging-bucket)). This field requires a Cloud Storage bucket name, not a gs://... URI to a Cloud Storage bucket.
    #[serde(rename="tempBucket")]
    
    pub temp_bucket: Option<String>,
    /// Optional. The Compute Engine config settings for the cluster's worker instances.
    #[serde(rename="workerConfig")]
    
    pub worker_config: Option<InstanceGroupConfig>,
}

impl client::Part for ClusterConfig {}


/// Contains cluster daemon metrics, such as HDFS and YARN stats.Beta Feature: This report is available for testing purposes only. It may be changed before final release.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClusterMetrics {
    /// The HDFS metrics.
    #[serde(rename="hdfsMetrics")]
    
    #[serde_as(as = "Option<HashMap<_, ::client::serde_with::DisplayFromStr>>")]
    pub hdfs_metrics: Option<HashMap<String, i64>>,
    /// YARN metrics.
    #[serde(rename="yarnMetrics")]
    
    #[serde_as(as = "Option<HashMap<_, ::client::serde_with::DisplayFromStr>>")]
    pub yarn_metrics: Option<HashMap<String, i64>>,
}

impl client::Part for ClusterMetrics {}


/// A selector that chooses target cluster for jobs based on metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClusterSelector {
    /// Required. The cluster labels. Cluster must have all labels to match.
    #[serde(rename="clusterLabels")]
    
    pub cluster_labels: Option<HashMap<String, String>>,
    /// Optional. The zone where workflow process executes. This parameter does not affect the selection of the cluster.If unspecified, the zone of the first cluster matching the selector is used.
    
    pub zone: Option<String>,
}

impl client::Part for ClusterSelector {}


/// The status of a cluster and its instances.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClusterStatus {
    /// Optional. Output only. Details of cluster's state.
    
    pub detail: Option<String>,
    /// Output only. The cluster's state.
    
    pub state: Option<ClusterStatuStateEnum>,
    /// Output only. Time when this state was entered (see JSON representation of Timestamp (https://developers.google.com/protocol-buffers/docs/proto3#json)).
    #[serde(rename="stateStartTime")]
    
    pub state_start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Additional state information that includes status reported by the agent.
    
    pub substate: Option<ClusterStatuSubstateEnum>,
}

impl client::Part for ClusterStatus {}


/// Confidential Instance Config for clusters using Confidential VMs (https://cloud.google.com/compute/confidential-vm/docs)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfidentialInstanceConfig {
    /// Optional. Defines whether the instance should have confidential compute enabled.
    #[serde(rename="enableConfidentialCompute")]
    
    pub enable_confidential_compute: Option<bool>,
}

impl client::Part for ConfidentialInstanceConfig {}


/// Dataproc metric config.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataprocMetricConfig {
    /// Required. Metrics sources to enable.
    
    pub metrics: Option<Vec<Metric>>,
}

impl client::Part for DataprocMetricConfig {}


/// A request to collect cluster diagnostic information.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [regions clusters diagnose projects](ProjectRegionClusterDiagnoseCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DiagnoseClusterRequest { _never_set: Option<bool> }

impl client::RequestValue for DiagnoseClusterRequest {}


/// Specifies the config of disk options for a group of VM instances.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DiskConfig {
    /// Optional. Size in GB of the boot disk (default is 500GB).
    #[serde(rename="bootDiskSizeGb")]
    
    pub boot_disk_size_gb: Option<i32>,
    /// Optional. Type of the boot disk (default is "pd-standard"). Valid values: "pd-balanced" (Persistent Disk Balanced Solid State Drive), "pd-ssd" (Persistent Disk Solid State Drive), or "pd-standard" (Persistent Disk Hard Disk Drive). See Disk types (https://cloud.google.com/compute/docs/disks#disk-types).
    #[serde(rename="bootDiskType")]
    
    pub boot_disk_type: Option<String>,
    /// Optional. Interface type of local SSDs (default is "scsi"). Valid values: "scsi" (Small Computer System Interface), "nvme" (Non-Volatile Memory Express). See local SSD performance (https://cloud.google.com/compute/docs/disks/local-ssd#performance).
    #[serde(rename="localSsdInterface")]
    
    pub local_ssd_interface: Option<String>,
    /// Optional. Number of attached SSDs, from 0 to 8 (default is 0). If SSDs are not attached, the boot disk is used to store runtime logs and HDFS (https://hadoop.apache.org/docs/r1.2.1/hdfs_user_guide.html) data. If one or more SSDs are attached, this runtime bulk data is spread across them, and the boot disk contains only basic config and installed binaries.Note: Local SSD options may vary by machine type and number of vCPUs selected.
    #[serde(rename="numLocalSsds")]
    
    pub num_local_ssds: Option<i32>,
}

impl client::Part for DiskConfig {}


/// Driver scheduling configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DriverSchedulingConfig {
    /// Required. The amount of memory in MB the driver is requesting.
    #[serde(rename="memoryMb")]
    
    pub memory_mb: Option<i32>,
    /// Required. The number of vCPUs the driver is requesting.
    
    pub vcores: Option<i32>,
}

impl client::Part for DriverSchedulingConfig {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } 
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations autoscaling policies delete projects](ProjectLocationAutoscalingPolicyDeleteCall) (response)
/// * [locations batches delete projects](ProjectLocationBatchDeleteCall) (response)
/// * [locations operations cancel projects](ProjectLocationOperationCancelCall) (response)
/// * [locations operations delete projects](ProjectLocationOperationDeleteCall) (response)
/// * [locations workflow templates delete projects](ProjectLocationWorkflowTemplateDeleteCall) (response)
/// * [regions autoscaling policies delete projects](ProjectRegionAutoscalingPolicyDeleteCall) (response)
/// * [regions jobs delete projects](ProjectRegionJobDeleteCall) (response)
/// * [regions operations cancel projects](ProjectRegionOperationCancelCall) (response)
/// * [regions operations delete projects](ProjectRegionOperationDeleteCall) (response)
/// * [regions workflow templates delete projects](ProjectRegionWorkflowTemplateDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// Encryption settings for the cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EncryptionConfig {
    /// Optional. The Cloud KMS key name to use for PD disk encryption for all instances in the cluster.
    #[serde(rename="gcePdKmsKeyName")]
    
    pub gce_pd_kms_key_name: Option<String>,
}

impl client::Part for EncryptionConfig {}


/// Endpoint config for this cluster
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EndpointConfig {
    /// Optional. If true, enable http access to specific ports on the cluster from external sources. Defaults to false.
    #[serde(rename="enableHttpPortAccess")]
    
    pub enable_http_port_access: Option<bool>,
    /// Output only. The map of port descriptions to URLs. Will only be populated if enable_http_port_access is true.
    #[serde(rename="httpPorts")]
    
    pub http_ports: Option<HashMap<String, String>>,
}

impl client::Part for EndpointConfig {}


/// Environment configuration for a workload.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnvironmentConfig {
    /// Optional. Execution configuration for a workload.
    #[serde(rename="executionConfig")]
    
    pub execution_config: Option<ExecutionConfig>,
    /// Optional. Peripherals configuration that workload has access to.
    #[serde(rename="peripheralsConfig")]
    
    pub peripherals_config: Option<PeripheralsConfig>,
}

impl client::Part for EnvironmentConfig {}


/// Execution configuration for a workload.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionConfig {
    /// Optional. The duration to keep the session alive while it's idling. Passing this threshold will cause the session to be terminated. Minimum value is 10 minutes; maximum value is 14 days (see JSON representation of Duration (https://developers.google.com/protocol-buffers/docs/proto3#json)). Defaults to 4 hours if not set. If both ttl and idle_ttl are specified, the conditions are treated as and OR: the workload will be terminated when it has been idle for idle_ttl or when the ttl has passed, whichever comes first.
    #[serde(rename="idleTtl")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub idle_ttl: Option<client::chrono::Duration>,
    /// Optional. The Cloud KMS key to use for encryption.
    #[serde(rename="kmsKey")]
    
    pub kms_key: Option<String>,
    /// Optional. Tags used for network traffic control.
    #[serde(rename="networkTags")]
    
    pub network_tags: Option<Vec<String>>,
    /// Optional. Network URI to connect workload to.
    #[serde(rename="networkUri")]
    
    pub network_uri: Option<String>,
    /// Optional. Service account that used to execute workload.
    #[serde(rename="serviceAccount")]
    
    pub service_account: Option<String>,
    /// Optional. Subnetwork URI to connect workload to.
    #[serde(rename="subnetworkUri")]
    
    pub subnetwork_uri: Option<String>,
}

impl client::Part for ExecutionConfig {}


/// Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language. The syntax and semantics of CEL are documented at https://github.com/google/cel-spec.Example (Comparison): title: "Summary size limit" description: "Determines if a summary is less than 100 chars" expression: "document.summary.size() < 100" Example (Equality): title: "Requestor is owner" description: "Determines if requestor is the document owner" expression: "document.owner == request.auth.claims.email" Example (Logic): title: "Public documents" description: "Determine whether the document should be publicly visible" expression: "document.type != 'private' && document.type != 'internal'" Example (Data Manipulation): title: "Notification string" description: "Create a notification string with a timestamp." expression: "'New message received at ' + string(document.create_time)" The exact variables and functions that may be referenced within an expression are determined by the service that evaluates it. See the service documentation for additional information.
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


/// Common config settings for resources of Compute Engine cluster instances, applicable to all instances in the cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GceClusterConfig {
    /// Optional. Confidential Instance Config for clusters using Confidential VMs (https://cloud.google.com/compute/confidential-vm/docs).
    #[serde(rename="confidentialInstanceConfig")]
    
    pub confidential_instance_config: Option<ConfidentialInstanceConfig>,
    /// Optional. If true, all instances in the cluster will only have internal IP addresses. By default, clusters are not restricted to internal IP addresses, and will have ephemeral external IP addresses assigned to each instance. This internal_ip_only restriction can only be enabled for subnetwork enabled networks, and all off-cluster dependencies must be configured to be accessible without external IP addresses.
    #[serde(rename="internalIpOnly")]
    
    pub internal_ip_only: Option<bool>,
    /// The Compute Engine metadata entries to add to all instances (see Project and instance metadata (https://cloud.google.com/compute/docs/storing-retrieving-metadata#project_and_instance_metadata)).
    
    pub metadata: Option<HashMap<String, String>>,
    /// Optional. The Compute Engine network to be used for machine communications. Cannot be specified with subnetwork_uri. If neither network_uri nor subnetwork_uri is specified, the "default" network of the project is used, if it exists. Cannot be a "Custom Subnet Network" (see Using Subnetworks (https://cloud.google.com/compute/docs/subnetworks) for more information).A full URL, partial URI, or short name are valid. Examples: https://www.googleapis.com/compute/v1/projects/[project_id]/regions/global/default projects/[project_id]/regions/global/default default
    #[serde(rename="networkUri")]
    
    pub network_uri: Option<String>,
    /// Optional. Node Group Affinity for sole-tenant clusters.
    #[serde(rename="nodeGroupAffinity")]
    
    pub node_group_affinity: Option<NodeGroupAffinity>,
    /// Optional. The type of IPv6 access for a cluster.
    #[serde(rename="privateIpv6GoogleAccess")]
    
    pub private_ipv6_google_access: Option<GceClusterConfigPrivateIpv6GoogleAccessEnum>,
    /// Optional. Reservation Affinity for consuming Zonal reservation.
    #[serde(rename="reservationAffinity")]
    
    pub reservation_affinity: Option<ReservationAffinity>,
    /// Optional. The Dataproc service account (https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/service-accounts#service_accounts_in_dataproc) (also see VM Data Plane identity (https://cloud.google.com/dataproc/docs/concepts/iam/dataproc-principals#vm_service_account_data_plane_identity)) used by Dataproc cluster VM instances to access Google Cloud Platform services.If not specified, the Compute Engine default service account (https://cloud.google.com/compute/docs/access/service-accounts#default_service_account) is used.
    #[serde(rename="serviceAccount")]
    
    pub service_account: Option<String>,
    /// Optional. The URIs of service account scopes to be included in Compute Engine instances. The following base set of scopes is always included: https://www.googleapis.com/auth/cloud.useraccounts.readonly https://www.googleapis.com/auth/devstorage.read_write https://www.googleapis.com/auth/logging.writeIf no scopes are specified, the following defaults are also provided: https://www.googleapis.com/auth/bigquery https://www.googleapis.com/auth/bigtable.admin.table https://www.googleapis.com/auth/bigtable.data https://www.googleapis.com/auth/devstorage.full_control
    #[serde(rename="serviceAccountScopes")]
    
    pub service_account_scopes: Option<Vec<String>>,
    /// Optional. Shielded Instance Config for clusters using Compute Engine Shielded VMs (https://cloud.google.com/security/shielded-cloud/shielded-vm).
    #[serde(rename="shieldedInstanceConfig")]
    
    pub shielded_instance_config: Option<ShieldedInstanceConfig>,
    /// Optional. The Compute Engine subnetwork to be used for machine communications. Cannot be specified with network_uri.A full URL, partial URI, or short name are valid. Examples: https://www.googleapis.com/compute/v1/projects/[project_id]/regions/us-east1/subnetworks/sub0 projects/[project_id]/regions/us-east1/subnetworks/sub0 sub0
    #[serde(rename="subnetworkUri")]
    
    pub subnetwork_uri: Option<String>,
    /// The Compute Engine tags to add to all instances (see Tagging instances (https://cloud.google.com/compute/docs/label-or-tag-resources#tags)).
    
    pub tags: Option<Vec<String>>,
    /// Optional. The zone where the Compute Engine cluster will be located. On a create request, it is required in the "global" region. If omitted in a non-global Dataproc region, the service will pick a zone in the corresponding Compute Engine region. On a get request, zone will always be present.A full URL, partial URI, or short name are valid. Examples: https://www.googleapis.com/compute/v1/projects/[project_id]/zones/[zone] projects/[project_id]/zones/[zone] us-central1-f
    #[serde(rename="zoneUri")]
    
    pub zone_uri: Option<String>,
}

impl client::Part for GceClusterConfig {}


/// Request message for GetIamPolicy method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations autoscaling policies get iam policy projects](ProjectLocationAutoscalingPolicyGetIamPolicyCall) (request)
/// * [locations workflow templates get iam policy projects](ProjectLocationWorkflowTemplateGetIamPolicyCall) (request)
/// * [regions autoscaling policies get iam policy projects](ProjectRegionAutoscalingPolicyGetIamPolicyCall) (request)
/// * [regions clusters get iam policy projects](ProjectRegionClusterGetIamPolicyCall) (request)
/// * [regions jobs get iam policy projects](ProjectRegionJobGetIamPolicyCall) (request)
/// * [regions operations get iam policy projects](ProjectRegionOperationGetIamPolicyCall) (request)
/// * [regions workflow templates get iam policy projects](ProjectRegionWorkflowTemplateGetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetIamPolicyRequest {
    /// OPTIONAL: A GetPolicyOptions object for specifying options to GetIamPolicy.
    
    pub options: Option<GetPolicyOptions>,
}

impl client::RequestValue for GetIamPolicyRequest {}


/// Encapsulates settings provided to GetIamPolicy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetPolicyOptions {
    /// Optional. The maximum policy version that will be used to format the policy.Valid values are 0, 1, and 3. Requests specifying an invalid value will be rejected.Requests for policies with any conditional role bindings must specify version 3. Policies with no conditional role bindings may specify any valid value or leave the field unset.The policy in the response might use the policy version that you specified, or it might use a lower policy version. For example, if you specify version 3, but the policy has no conditional role bindings, the response uses version 1.To learn which resources support conditions in their IAM policies, see the IAM documentation (https://cloud.google.com/iam/help/conditions/resource-policies).
    #[serde(rename="requestedPolicyVersion")]
    
    pub requested_policy_version: Option<i32>,
}

impl client::Part for GetPolicyOptions {}


/// The cluster's GKE config.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GkeClusterConfig {
    /// Optional. A target GKE cluster to deploy to. It must be in the same project and region as the Dataproc cluster (the GKE cluster can be zonal or regional). Format: 'projects/{project}/locations/{location}/clusters/{cluster_id}'
    #[serde(rename="gkeClusterTarget")]
    
    pub gke_cluster_target: Option<String>,
    /// Optional. Deprecated. Use gkeClusterTarget. Used only for the deprecated beta. A target for the deployment.
    #[serde(rename="namespacedGkeDeploymentTarget")]
    
    pub namespaced_gke_deployment_target: Option<NamespacedGkeDeploymentTarget>,
    /// Optional. GKE node pools where workloads will be scheduled. At least one node pool must be assigned the DEFAULT GkeNodePoolTarget.Role. If a GkeNodePoolTarget is not specified, Dataproc constructs a DEFAULT GkeNodePoolTarget. Each role can be given to only one GkeNodePoolTarget. All node pools must have the same location settings.
    #[serde(rename="nodePoolTarget")]
    
    pub node_pool_target: Option<Vec<GkeNodePoolTarget>>,
}

impl client::Part for GkeClusterConfig {}


/// Parameters that describe cluster nodes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GkeNodeConfig {
    /// Optional. A list of hardware accelerators (https://cloud.google.com/compute/docs/gpus) to attach to each node.
    
    pub accelerators: Option<Vec<GkeNodePoolAcceleratorConfig>>,
    /// Optional. The Customer Managed Encryption Key (CMEK) (https://cloud.google.com/kubernetes-engine/docs/how-to/using-cmek) used to encrypt the boot disk attached to each node in the node pool. Specify the key using the following format: projects/KEY_PROJECT_ID/locations/LOCATION /keyRings/RING_NAME/cryptoKeys/KEY_NAME.
    #[serde(rename="bootDiskKmsKey")]
    
    pub boot_disk_kms_key: Option<String>,
    /// Optional. The number of local SSD disks to attach to the node, which is limited by the maximum number of disks allowable per zone (see Adding Local SSDs (https://cloud.google.com/compute/docs/disks/local-ssd)).
    #[serde(rename="localSsdCount")]
    
    pub local_ssd_count: Option<i32>,
    /// Optional. The name of a Compute Engine machine type (https://cloud.google.com/compute/docs/machine-types).
    #[serde(rename="machineType")]
    
    pub machine_type: Option<String>,
    /// Optional. Minimum CPU platform (https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform) to be used by this instance. The instance may be scheduled on the specified or a newer CPU platform. Specify the friendly names of CPU platforms, such as "Intel Haswell"` or Intel Sandy Bridge".
    #[serde(rename="minCpuPlatform")]
    
    pub min_cpu_platform: Option<String>,
    /// Optional. Whether the nodes are created as legacy preemptible VM instances (https://cloud.google.com/compute/docs/instances/preemptible). Also see Spot VMs, preemptible VM instances without a maximum lifetime. Legacy and Spot preemptible nodes cannot be used in a node pool with the CONTROLLER role or in the DEFAULT node pool if the CONTROLLER role is not assigned (the DEFAULT node pool will assume the CONTROLLER role).
    
    pub preemptible: Option<bool>,
    /// Optional. Whether the nodes are created as Spot VM instances (https://cloud.google.com/compute/docs/instances/spot). Spot VMs are the latest update to legacy preemptible VMs. Spot VMs do not have a maximum lifetime. Legacy and Spot preemptible nodes cannot be used in a node pool with the CONTROLLER role or in the DEFAULT node pool if the CONTROLLER role is not assigned (the DEFAULT node pool will assume the CONTROLLER role).
    
    pub spot: Option<bool>,
}

impl client::Part for GkeNodeConfig {}


/// A GkeNodeConfigAcceleratorConfig represents a Hardware Accelerator request for a node pool.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GkeNodePoolAcceleratorConfig {
    /// The number of accelerator cards exposed to an instance.
    #[serde(rename="acceleratorCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub accelerator_count: Option<i64>,
    /// The accelerator type resource namename (see GPUs on Compute Engine).
    #[serde(rename="acceleratorType")]
    
    pub accelerator_type: Option<String>,
    /// Size of partitions to create on the GPU. Valid values are described in the NVIDIA mig user guide (https://docs.nvidia.com/datacenter/tesla/mig-user-guide/#partitioning).
    #[serde(rename="gpuPartitionSize")]
    
    pub gpu_partition_size: Option<String>,
}

impl client::Part for GkeNodePoolAcceleratorConfig {}


/// GkeNodePoolAutoscaling contains information the cluster autoscaler needs to adjust the size of the node pool to the current cluster usage.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GkeNodePoolAutoscalingConfig {
    /// The maximum number of nodes in the node pool. Must be >= min_node_count, and must be > 0. Note: Quota must be sufficient to scale up the cluster.
    #[serde(rename="maxNodeCount")]
    
    pub max_node_count: Option<i32>,
    /// The minimum number of nodes in the node pool. Must be >= 0 and <= max_node_count.
    #[serde(rename="minNodeCount")]
    
    pub min_node_count: Option<i32>,
}

impl client::Part for GkeNodePoolAutoscalingConfig {}


/// The configuration of a GKE node pool used by a Dataproc-on-GKE cluster (https://cloud.google.com/dataproc/docs/concepts/jobs/dataproc-gke#create-a-dataproc-on-gke-cluster).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GkeNodePoolConfig {
    /// Optional. The autoscaler configuration for this node pool. The autoscaler is enabled only when a valid configuration is present.
    
    pub autoscaling: Option<GkeNodePoolAutoscalingConfig>,
    /// Optional. The node pool configuration.
    
    pub config: Option<GkeNodeConfig>,
    /// Optional. The list of Compute Engine zones (https://cloud.google.com/compute/docs/zones#available) where node pool nodes associated with a Dataproc on GKE virtual cluster will be located.Note: All node pools associated with a virtual cluster must be located in the same region as the virtual cluster, and they must be located in the same zone within that region.If a location is not specified during node pool creation, Dataproc on GKE will choose the zone.
    
    pub locations: Option<Vec<String>>,
}

impl client::Part for GkeNodePoolConfig {}


/// GKE node pools that Dataproc workloads run on.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GkeNodePoolTarget {
    /// Required. The target GKE node pool. Format: 'projects/{project}/locations/{location}/clusters/{cluster}/nodePools/{node_pool}'
    #[serde(rename="nodePool")]
    
    pub node_pool: Option<String>,
    /// Input only. The configuration for the GKE node pool.If specified, Dataproc attempts to create a node pool with the specified shape. If one with the same name already exists, it is verified against all specified fields. If a field differs, the virtual cluster creation will fail.If omitted, any node pool with the specified name is used. If a node pool with the specified name does not exist, Dataproc create a node pool with default values.This is an input only field. It will not be returned by the API.
    #[serde(rename="nodePoolConfig")]
    
    pub node_pool_config: Option<GkeNodePoolConfig>,
    /// Required. The roles associated with the GKE node pool.
    
    pub roles: Option<Vec<GkeNodePoolTargetRolesEnum>>,
}

impl client::Part for GkeNodePoolTarget {}


/// A Dataproc job for running Apache Hadoop MapReduce (https://hadoop.apache.org/docs/current/hadoop-mapreduce-client/hadoop-mapreduce-client-core/MapReduceTutorial.html) jobs on Apache Hadoop YARN (https://hadoop.apache.org/docs/r2.7.1/hadoop-yarn/hadoop-yarn-site/YARN.html).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HadoopJob {
    /// Optional. HCFS URIs of archives to be extracted in the working directory of Hadoop drivers and tasks. Supported file types: .jar, .tar, .tar.gz, .tgz, or .zip.
    #[serde(rename="archiveUris")]
    
    pub archive_uris: Option<Vec<String>>,
    /// Optional. The arguments to pass to the driver. Do not include arguments, such as -libjars or -Dfoo=bar, that can be set as job properties, since a collision may occur that causes an incorrect job submission.
    
    pub args: Option<Vec<String>>,
    /// Optional. HCFS (Hadoop Compatible Filesystem) URIs of files to be copied to the working directory of Hadoop drivers and distributed tasks. Useful for naively parallel tasks.
    #[serde(rename="fileUris")]
    
    pub file_uris: Option<Vec<String>>,
    /// Optional. Jar file URIs to add to the CLASSPATHs of the Hadoop driver and tasks.
    #[serde(rename="jarFileUris")]
    
    pub jar_file_uris: Option<Vec<String>>,
    /// Optional. The runtime log config for job execution.
    #[serde(rename="loggingConfig")]
    
    pub logging_config: Option<LoggingConfig>,
    /// The name of the driver's main class. The jar file containing the class must be in the default CLASSPATH or specified in jar_file_uris.
    #[serde(rename="mainClass")]
    
    pub main_class: Option<String>,
    /// The HCFS URI of the jar file containing the main class. Examples: 'gs://foo-bucket/analytics-binaries/extract-useful-metrics-mr.jar' 'hdfs:/tmp/test-samples/custom-wordcount.jar' 'file:///home/usr/lib/hadoop-mapreduce/hadoop-mapreduce-examples.jar'
    #[serde(rename="mainJarFileUri")]
    
    pub main_jar_file_uri: Option<String>,
    /// Optional. A mapping of property names to values, used to configure Hadoop. Properties that conflict with values set by the Dataproc API may be overwritten. Can include properties set in /etc/hadoop/conf/*-site and classes in user code.
    
    pub properties: Option<HashMap<String, String>>,
}

impl client::Part for HadoopJob {}


/// A Dataproc job for running Apache Hive (https://hive.apache.org/) queries on YARN.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HiveJob {
    /// Optional. Whether to continue executing queries if a query fails. The default value is false. Setting to true can be useful when executing independent parallel queries.
    #[serde(rename="continueOnFailure")]
    
    pub continue_on_failure: Option<bool>,
    /// Optional. HCFS URIs of jar files to add to the CLASSPATH of the Hive server and Hadoop MapReduce (MR) tasks. Can contain Hive SerDes and UDFs.
    #[serde(rename="jarFileUris")]
    
    pub jar_file_uris: Option<Vec<String>>,
    /// Optional. A mapping of property names and values, used to configure Hive. Properties that conflict with values set by the Dataproc API may be overwritten. Can include properties set in /etc/hadoop/conf/*-site.xml, /etc/hive/conf/hive-site.xml, and classes in user code.
    
    pub properties: Option<HashMap<String, String>>,
    /// The HCFS URI of the script that contains Hive queries.
    #[serde(rename="queryFileUri")]
    
    pub query_file_uri: Option<String>,
    /// A list of queries.
    #[serde(rename="queryList")]
    
    pub query_list: Option<QueryList>,
    /// Optional. Mapping of query variable names to values (equivalent to the Hive command: SET name="value";).
    #[serde(rename="scriptVariables")]
    
    pub script_variables: Option<HashMap<String, String>>,
}

impl client::Part for HiveJob {}


/// Identity related configuration, including service account based secure multi-tenancy user mappings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IdentityConfig {
    /// Required. Map of user to service account.
    #[serde(rename="userServiceAccountMapping")]
    
    pub user_service_account_mapping: Option<HashMap<String, String>>,
}

impl client::Part for IdentityConfig {}


/// A request to inject credentials into a cluster.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [regions clusters inject credentials projects](ProjectRegionClusterInjectCredentialCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InjectCredentialsRequest {
    /// Required. The cluster UUID.
    #[serde(rename="clusterUuid")]
    
    pub cluster_uuid: Option<String>,
    /// Required. The encrypted credentials being injected in to the cluster.The client is responsible for encrypting the credentials in a way that is supported by the cluster.A wrapped value is used here so that the actual contents of the encrypted credentials are not written to audit logs.
    #[serde(rename="credentialsCiphertext")]
    
    pub credentials_ciphertext: Option<String>,
}

impl client::RequestValue for InjectCredentialsRequest {}


/// Configuration for the size bounds of an instance group, including its proportional size to other groups.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstanceGroupAutoscalingPolicyConfig {
    /// Required. Maximum number of instances for this group. Required for primary workers. Note that by default, clusters will not use secondary workers. Required for secondary workers if the minimum secondary instances is set.Primary workers - Bounds: [min_instances, ). Secondary workers - Bounds: [min_instances, ). Default: 0.
    #[serde(rename="maxInstances")]
    
    pub max_instances: Option<i32>,
    /// Optional. Minimum number of instances for this group.Primary workers - Bounds: 2, max_instances. Default: 2. Secondary workers - Bounds: 0, max_instances. Default: 0.
    #[serde(rename="minInstances")]
    
    pub min_instances: Option<i32>,
    /// Optional. Weight for the instance group, which is used to determine the fraction of total workers in the cluster from this instance group. For example, if primary workers have weight 2, and secondary workers have weight 1, the cluster will have approximately 2 primary workers for each secondary worker.The cluster may not reach the specified balance if constrained by min/max bounds or other autoscaling settings. For example, if max_instances for secondary workers is 0, then only primary workers will be added. The cluster can also be out of balance when created.If weight is not set on any instance group, the cluster will default to equal weight for all groups: the cluster will attempt to maintain an equal number of workers in each group within the configured size bounds for each group. If weight is set for one group only, the cluster will default to zero weight on the unset group. For example if weight is set only on primary workers, the cluster will use primary workers only and no secondary workers.
    
    pub weight: Option<i32>,
}

impl client::Part for InstanceGroupAutoscalingPolicyConfig {}


/// The config settings for Compute Engine resources in an instance group, such as a master or worker group.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstanceGroupConfig {
    /// Optional. The Compute Engine accelerator configuration for these instances.
    
    pub accelerators: Option<Vec<AcceleratorConfig>>,
    /// Optional. Disk option config settings.
    #[serde(rename="diskConfig")]
    
    pub disk_config: Option<DiskConfig>,
    /// Optional. The Compute Engine image resource used for cluster instances.The URI can represent an image or image family.Image examples: https://www.googleapis.com/compute/beta/projects/[project_id]/global/images/[image-id] projects/[project_id]/global/images/[image-id] image-idImage family examples. Dataproc will use the most recent image from the family: https://www.googleapis.com/compute/beta/projects/[project_id]/global/images/family/[custom-image-family-name] projects/[project_id]/global/images/family/[custom-image-family-name]If the URI is unspecified, it will be inferred from SoftwareConfig.image_version or the system default.
    #[serde(rename="imageUri")]
    
    pub image_uri: Option<String>,
    /// Output only. The list of instance names. Dataproc derives the names from cluster_name, num_instances, and the instance group.
    #[serde(rename="instanceNames")]
    
    pub instance_names: Option<Vec<String>>,
    /// Output only. List of references to Compute Engine instances.
    #[serde(rename="instanceReferences")]
    
    pub instance_references: Option<Vec<InstanceReference>>,
    /// Output only. Specifies that this instance group contains preemptible instances.
    #[serde(rename="isPreemptible")]
    
    pub is_preemptible: Option<bool>,
    /// Optional. The Compute Engine machine type used for cluster instances.A full URL, partial URI, or short name are valid. Examples: https://www.googleapis.com/compute/v1/projects/[project_id]/zones/us-east1-a/machineTypes/n1-standard-2 projects/[project_id]/zones/us-east1-a/machineTypes/n1-standard-2 n1-standard-2Auto Zone Exception: If you are using the Dataproc Auto Zone Placement (https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/auto-zone#using_auto_zone_placement) feature, you must use the short name of the machine type resource, for example, n1-standard-2.
    #[serde(rename="machineTypeUri")]
    
    pub machine_type_uri: Option<String>,
    /// Output only. The config for Compute Engine Instance Group Manager that manages this group. This is only used for preemptible instance groups.
    #[serde(rename="managedGroupConfig")]
    
    pub managed_group_config: Option<ManagedGroupConfig>,
    /// Optional. Specifies the minimum cpu platform for the Instance Group. See Dataproc -> Minimum CPU Platform (https://cloud.google.com/dataproc/docs/concepts/compute/dataproc-min-cpu).
    #[serde(rename="minCpuPlatform")]
    
    pub min_cpu_platform: Option<String>,
    /// Optional. The number of VM instances in the instance group. For HA cluster master_config groups, must be set to 3. For standard cluster master_config groups, must be set to 1.
    #[serde(rename="numInstances")]
    
    pub num_instances: Option<i32>,
    /// Optional. Specifies the preemptibility of the instance group.The default value for master and worker groups is NON_PREEMPTIBLE. This default cannot be changed.The default value for secondary instances is PREEMPTIBLE.
    
    pub preemptibility: Option<InstanceGroupConfigPreemptibilityEnum>,
}

impl client::Part for InstanceGroupConfig {}


/// A reference to a Compute Engine instance.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstanceReference {
    /// The unique identifier of the Compute Engine instance.
    #[serde(rename="instanceId")]
    
    pub instance_id: Option<String>,
    /// The user-friendly name of the Compute Engine instance.
    #[serde(rename="instanceName")]
    
    pub instance_name: Option<String>,
    /// The public ECIES key used for sharing data with this instance.
    #[serde(rename="publicEciesKey")]
    
    pub public_ecies_key: Option<String>,
    /// The public RSA key used for sharing data with this instance.
    #[serde(rename="publicKey")]
    
    pub public_key: Option<String>,
}

impl client::Part for InstanceReference {}


/// A request to instantiate a workflow template.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations workflow templates instantiate projects](ProjectLocationWorkflowTemplateInstantiateCall) (request)
/// * [regions workflow templates instantiate projects](ProjectRegionWorkflowTemplateInstantiateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstantiateWorkflowTemplateRequest {
    /// Optional. Map from parameter names to values that should be used for those parameters. Values may not exceed 1000 characters.
    
    pub parameters: Option<HashMap<String, String>>,
    /// Optional. A tag that prevents multiple concurrent workflow instances with the same tag from running. This mitigates risk of concurrent instances started due to retries.It is recommended to always set this value to a UUID (https://en.wikipedia.org/wiki/Universally_unique_identifier).The tag must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), and hyphens (-). The maximum length is 40 characters.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
    /// Optional. The version of workflow template to instantiate. If specified, the workflow will be instantiated only if the current version of the workflow template has the supplied version.This option cannot be used to instantiate a previous version of workflow template.
    
    pub version: Option<i32>,
}

impl client::RequestValue for InstantiateWorkflowTemplateRequest {}


/// A Dataproc job resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [regions jobs cancel projects](ProjectRegionJobCancelCall) (response)
/// * [regions jobs get projects](ProjectRegionJobGetCall) (response)
/// * [regions jobs patch projects](ProjectRegionJobPatchCall) (request|response)
/// * [regions jobs submit projects](ProjectRegionJobSubmitCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Job {
    /// Output only. Indicates whether the job is completed. If the value is false, the job is still in progress. If true, the job is completed, and status.state field will indicate if it was successful, failed, or cancelled.
    
    pub done: Option<bool>,
    /// Output only. If present, the location of miscellaneous control files which may be used as part of job setup and handling. If not present, control files may be placed in the same location as driver_output_uri.
    #[serde(rename="driverControlFilesUri")]
    
    pub driver_control_files_uri: Option<String>,
    /// Output only. A URI pointing to the location of the stdout of the job's driver program.
    #[serde(rename="driverOutputResourceUri")]
    
    pub driver_output_resource_uri: Option<String>,
    /// Optional. Driver scheduling configuration.
    #[serde(rename="driverSchedulingConfig")]
    
    pub driver_scheduling_config: Option<DriverSchedulingConfig>,
    /// Optional. Job is a Hadoop job.
    #[serde(rename="hadoopJob")]
    
    pub hadoop_job: Option<HadoopJob>,
    /// Optional. Job is a Hive job.
    #[serde(rename="hiveJob")]
    
    pub hive_job: Option<HiveJob>,
    /// Output only. A UUID that uniquely identifies a job within the project over time. This is in contrast to a user-settable reference.job_id that may be reused over time.
    #[serde(rename="jobUuid")]
    
    pub job_uuid: Option<String>,
    /// Optional. The labels to associate with this job. Label keys must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). Label values may be empty, but, if present, must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). No more than 32 labels can be associated with a job.
    
    pub labels: Option<HashMap<String, String>>,
    /// Optional. Job is a Pig job.
    #[serde(rename="pigJob")]
    
    pub pig_job: Option<PigJob>,
    /// Required. Job information, including how, when, and where to run the job.
    
    pub placement: Option<JobPlacement>,
    /// Optional. Job is a Presto job.
    #[serde(rename="prestoJob")]
    
    pub presto_job: Option<PrestoJob>,
    /// Optional. Job is a PySpark job.
    #[serde(rename="pysparkJob")]
    
    pub pyspark_job: Option<PySparkJob>,
    /// Optional. The fully qualified reference to the job, which can be used to obtain the equivalent REST path of the job resource. If this property is not specified when a job is created, the server generates a job_id.
    
    pub reference: Option<JobReference>,
    /// Optional. Job scheduling configuration.
    
    pub scheduling: Option<JobScheduling>,
    /// Optional. Job is a Spark job.
    #[serde(rename="sparkJob")]
    
    pub spark_job: Option<SparkJob>,
    /// Optional. Job is a SparkR job.
    #[serde(rename="sparkRJob")]
    
    pub spark_r_job: Option<SparkRJob>,
    /// Optional. Job is a SparkSql job.
    #[serde(rename="sparkSqlJob")]
    
    pub spark_sql_job: Option<SparkSqlJob>,
    /// Output only. The job status. Additional application-specific status information may be contained in the type_job and yarn_applications fields.
    
    pub status: Option<JobStatus>,
    /// Output only. The previous job status.
    #[serde(rename="statusHistory")]
    
    pub status_history: Option<Vec<JobStatus>>,
    /// Optional. Job is a Trino job.
    #[serde(rename="trinoJob")]
    
    pub trino_job: Option<TrinoJob>,
    /// Output only. The collection of YARN applications spun up by this job.Beta Feature: This report is available for testing purposes only. It may be changed before final release.
    #[serde(rename="yarnApplications")]
    
    pub yarn_applications: Option<Vec<YarnApplication>>,
}

impl client::RequestValue for Job {}
impl client::ResponseResult for Job {}


/// Dataproc job config.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobPlacement {
    /// Optional. Cluster labels to identify a cluster where the job will be submitted.
    #[serde(rename="clusterLabels")]
    
    pub cluster_labels: Option<HashMap<String, String>>,
    /// Required. The name of the cluster where the job will be submitted.
    #[serde(rename="clusterName")]
    
    pub cluster_name: Option<String>,
    /// Output only. A cluster UUID generated by the Dataproc service when the job is submitted.
    #[serde(rename="clusterUuid")]
    
    pub cluster_uuid: Option<String>,
}

impl client::Part for JobPlacement {}


/// Encapsulates the full scoping used to reference a job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobReference {
    /// Optional. The job ID, which must be unique within the project.The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), or hyphens (-). The maximum length is 100 characters.If not specified by the caller, the job ID will be provided by the server.
    #[serde(rename="jobId")]
    
    pub job_id: Option<String>,
    /// Optional. The ID of the Google Cloud Platform project that the job belongs to. If specified, must match the request project ID.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
}

impl client::Part for JobReference {}


/// Job scheduling options.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobScheduling {
    /// Optional. Maximum number of times per hour a driver may be restarted as a result of driver exiting with non-zero code before job is reported failed.A job may be reported as thrashing if the driver exits with a non-zero code four times within a 10-minute window.Maximum value is 10.Note: This restartable job option is not supported in Dataproc workflow templates (https://cloud.google.com/dataproc/docs/concepts/workflows/using-workflows#adding_jobs_to_a_template).
    #[serde(rename="maxFailuresPerHour")]
    
    pub max_failures_per_hour: Option<i32>,
    /// Optional. Maximum total number of times a driver may be restarted as a result of the driver exiting with a non-zero code. After the maximum number is reached, the job will be reported as failed.Maximum value is 240.Note: Currently, this restartable job option is not supported in Dataproc workflow templates (https://cloud.google.com/dataproc/docs/concepts/workflows/using-workflows#adding_jobs_to_a_template).
    #[serde(rename="maxFailuresTotal")]
    
    pub max_failures_total: Option<i32>,
}

impl client::Part for JobScheduling {}


/// Dataproc job status.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobStatus {
    /// Optional. Output only. Job state details, such as an error description if the state is ERROR.
    
    pub details: Option<String>,
    /// Output only. A state message specifying the overall job state.
    
    pub state: Option<JobStatuStateEnum>,
    /// Output only. The time when this state was entered.
    #[serde(rename="stateStartTime")]
    
    pub state_start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Additional state information, which includes status reported by the agent.
    
    pub substate: Option<JobStatuSubstateEnum>,
}

impl client::Part for JobStatus {}


/// Specifies Kerberos related configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KerberosConfig {
    /// Optional. The admin server (IP or hostname) for the remote trusted realm in a cross realm trust relationship.
    #[serde(rename="crossRealmTrustAdminServer")]
    
    pub cross_realm_trust_admin_server: Option<String>,
    /// Optional. The KDC (IP or hostname) for the remote trusted realm in a cross realm trust relationship.
    #[serde(rename="crossRealmTrustKdc")]
    
    pub cross_realm_trust_kdc: Option<String>,
    /// Optional. The remote realm the Dataproc on-cluster KDC will trust, should the user enable cross realm trust.
    #[serde(rename="crossRealmTrustRealm")]
    
    pub cross_realm_trust_realm: Option<String>,
    /// Optional. The Cloud Storage URI of a KMS encrypted file containing the shared password between the on-cluster Kerberos realm and the remote trusted realm, in a cross realm trust relationship.
    #[serde(rename="crossRealmTrustSharedPasswordUri")]
    
    pub cross_realm_trust_shared_password_uri: Option<String>,
    /// Optional. Flag to indicate whether to Kerberize the cluster (default: false). Set this field to true to enable Kerberos on a cluster.
    #[serde(rename="enableKerberos")]
    
    pub enable_kerberos: Option<bool>,
    /// Optional. The Cloud Storage URI of a KMS encrypted file containing the master key of the KDC database.
    #[serde(rename="kdcDbKeyUri")]
    
    pub kdc_db_key_uri: Option<String>,
    /// Optional. The Cloud Storage URI of a KMS encrypted file containing the password to the user provided key. For the self-signed certificate, this password is generated by Dataproc.
    #[serde(rename="keyPasswordUri")]
    
    pub key_password_uri: Option<String>,
    /// Optional. The Cloud Storage URI of a KMS encrypted file containing the password to the user provided keystore. For the self-signed certificate, this password is generated by Dataproc.
    #[serde(rename="keystorePasswordUri")]
    
    pub keystore_password_uri: Option<String>,
    /// Optional. The Cloud Storage URI of the keystore file used for SSL encryption. If not provided, Dataproc will provide a self-signed certificate.
    #[serde(rename="keystoreUri")]
    
    pub keystore_uri: Option<String>,
    /// Optional. The uri of the KMS key used to encrypt various sensitive files.
    #[serde(rename="kmsKeyUri")]
    
    pub kms_key_uri: Option<String>,
    /// Optional. The name of the on-cluster Kerberos realm. If not specified, the uppercased domain of hostnames will be the realm.
    
    pub realm: Option<String>,
    /// Optional. The Cloud Storage URI of a KMS encrypted file containing the root principal password.
    #[serde(rename="rootPrincipalPasswordUri")]
    
    pub root_principal_password_uri: Option<String>,
    /// Optional. The lifetime of the ticket granting ticket, in hours. If not specified, or user specifies 0, then default value 10 will be used.
    #[serde(rename="tgtLifetimeHours")]
    
    pub tgt_lifetime_hours: Option<i32>,
    /// Optional. The Cloud Storage URI of a KMS encrypted file containing the password to the user provided truststore. For the self-signed certificate, this password is generated by Dataproc.
    #[serde(rename="truststorePasswordUri")]
    
    pub truststore_password_uri: Option<String>,
    /// Optional. The Cloud Storage URI of the truststore file used for SSL encryption. If not provided, Dataproc will provide a self-signed certificate.
    #[serde(rename="truststoreUri")]
    
    pub truststore_uri: Option<String>,
}

impl client::Part for KerberosConfig {}


/// The configuration for running the Dataproc cluster on Kubernetes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KubernetesClusterConfig {
    /// Required. The configuration for running the Dataproc cluster on GKE.
    #[serde(rename="gkeClusterConfig")]
    
    pub gke_cluster_config: Option<GkeClusterConfig>,
    /// Optional. A namespace within the Kubernetes cluster to deploy into. If this namespace does not exist, it is created. If it exists, Dataproc verifies that another Dataproc VirtualCluster is not installed into it. If not specified, the name of the Dataproc Cluster is used.
    #[serde(rename="kubernetesNamespace")]
    
    pub kubernetes_namespace: Option<String>,
    /// Optional. The software configuration for this Dataproc cluster running on Kubernetes.
    #[serde(rename="kubernetesSoftwareConfig")]
    
    pub kubernetes_software_config: Option<KubernetesSoftwareConfig>,
}

impl client::Part for KubernetesClusterConfig {}


/// The software configuration for this Dataproc cluster running on Kubernetes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KubernetesSoftwareConfig {
    /// The components that should be installed in this Dataproc cluster. The key must be a string from the KubernetesComponent enumeration. The value is the version of the software to be installed. At least one entry must be specified.
    #[serde(rename="componentVersion")]
    
    pub component_version: Option<HashMap<String, String>>,
    /// The properties to set on daemon config files.Property keys are specified in prefix:property format, for example spark:spark.kubernetes.container.image. The following are supported prefixes and their mappings: spark: spark-defaults.confFor more information, see Cluster properties (https://cloud.google.com/dataproc/docs/concepts/cluster-properties).
    
    pub properties: Option<HashMap<String, String>>,
}

impl client::Part for KubernetesSoftwareConfig {}


/// Specifies the cluster auto-delete schedule configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LifecycleConfig {
    /// Optional. The time when cluster will be auto-deleted (see JSON representation of Timestamp (https://developers.google.com/protocol-buffers/docs/proto3#json)).
    #[serde(rename="autoDeleteTime")]
    
    pub auto_delete_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. The lifetime duration of cluster. The cluster will be auto-deleted at the end of this period. Minimum value is 10 minutes; maximum value is 14 days (see JSON representation of Duration (https://developers.google.com/protocol-buffers/docs/proto3#json)).
    #[serde(rename="autoDeleteTtl")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub auto_delete_ttl: Option<client::chrono::Duration>,
    /// Optional. The duration to keep the cluster alive while idling (when no jobs are running). Passing this threshold will cause the cluster to be deleted. Minimum value is 5 minutes; maximum value is 14 days (see JSON representation of Duration (https://developers.google.com/protocol-buffers/docs/proto3#json)).
    #[serde(rename="idleDeleteTtl")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub idle_delete_ttl: Option<client::chrono::Duration>,
    /// Output only. The time when cluster became idle (most recent job finished) and became eligible for deletion due to idleness (see JSON representation of Timestamp (https://developers.google.com/protocol-buffers/docs/proto3#json)).
    #[serde(rename="idleStartTime")]
    
    pub idle_start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for LifecycleConfig {}


/// A response to a request to list autoscaling policies in a project.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations autoscaling policies list projects](ProjectLocationAutoscalingPolicyListCall) (response)
/// * [regions autoscaling policies list projects](ProjectRegionAutoscalingPolicyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAutoscalingPoliciesResponse {
    /// Output only. This token is included in the response if there are more results to fetch.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Output only. Autoscaling policies list.
    
    pub policies: Option<Vec<AutoscalingPolicy>>,
}

impl client::ResponseResult for ListAutoscalingPoliciesResponse {}


/// A list of batch workloads.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations batches list projects](ProjectLocationBatchListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListBatchesResponse {
    /// The batches from the specified collection.
    
    pub batches: Option<Vec<Batch>>,
    /// A token, which can be sent as page_token to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListBatchesResponse {}


/// The list of all clusters in a project.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [regions clusters list projects](ProjectRegionClusterListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListClustersResponse {
    /// Output only. The clusters in the project.
    
    pub clusters: Option<Vec<Cluster>>,
    /// Output only. This token is included in the response if there are more results to fetch. To fetch additional results, provide this value as the page_token in a subsequent ListClustersRequest.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListClustersResponse {}


/// A list of jobs in a project.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [regions jobs list projects](ProjectRegionJobListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListJobsResponse {
    /// Output only. Jobs list.
    
    pub jobs: Option<Vec<Job>>,
    /// Optional. This token is included in the response if there are more results to fetch. To fetch additional results, provide this value as the page_token in a subsequent ListJobsRequest.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListJobsResponse {}


/// The response message for Operations.ListOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations list projects](ProjectLocationOperationListCall) (response)
/// * [regions operations list projects](ProjectRegionOperationListCall) (response)
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


/// A response to a request to list workflow templates in a project.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations workflow templates list projects](ProjectLocationWorkflowTemplateListCall) (response)
/// * [regions workflow templates list projects](ProjectRegionWorkflowTemplateListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListWorkflowTemplatesResponse {
    /// Output only. This token is included in the response if there are more results to fetch. To fetch additional results, provide this value as the page_token in a subsequent ListWorkflowTemplatesRequest.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Output only. WorkflowTemplates list.
    
    pub templates: Option<Vec<WorkflowTemplate>>,
}

impl client::ResponseResult for ListWorkflowTemplatesResponse {}


/// The runtime logging config of the job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// The per-package log levels for the driver. This may include "root" package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'
    #[serde(rename="driverLogLevels")]
    
    pub driver_log_levels: Option<HashMap<String, LoggingConfigDriverLogLevelsEnum>>,
}

impl client::Part for LoggingConfig {}


/// Cluster that is managed by the workflow.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManagedCluster {
    /// Required. The cluster name prefix. A unique cluster name will be formed by appending a random suffix.The name must contain only lower-case letters (a-z), numbers (0-9), and hyphens (-). Must begin with a letter. Cannot begin or end with hyphen. Must consist of between 2 and 35 characters.
    #[serde(rename="clusterName")]
    
    pub cluster_name: Option<String>,
    /// Required. The cluster configuration.
    
    pub config: Option<ClusterConfig>,
    /// Optional. The labels to associate with this cluster.Label keys must be between 1 and 63 characters long, and must conform to the following PCRE regular expression: \p{Ll}\p{Lo}{0,62}Label values must be between 1 and 63 characters long, and must conform to the following PCRE regular expression: \p{Ll}\p{Lo}\p{N}_-{0,63}No more than 32 labels can be associated with a given cluster.
    
    pub labels: Option<HashMap<String, String>>,
}

impl client::Part for ManagedCluster {}


/// Specifies the resources used to actively manage an instance group.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManagedGroupConfig {
    /// Output only. The name of the Instance Group Manager for this group.
    #[serde(rename="instanceGroupManagerName")]
    
    pub instance_group_manager_name: Option<String>,
    /// Output only. The name of the Instance Template used for the Managed Instance Group.
    #[serde(rename="instanceTemplateName")]
    
    pub instance_template_name: Option<String>,
}

impl client::Part for ManagedGroupConfig {}


/// Specifies a Metastore configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetastoreConfig {
    /// Required. Resource name of an existing Dataproc Metastore service.Example: projects/[project_id]/locations/[dataproc_region]/services/[service-name]
    #[serde(rename="dataprocMetastoreService")]
    
    pub dataproc_metastore_service: Option<String>,
}

impl client::Part for MetastoreConfig {}


/// A Dataproc OSS metric.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Metric {
    /// Optional. Specify one or more available OSS metrics (https://cloud.google.com/dataproc/docs/guides/monitoring#available_oss_metrics) to collect for the metric course (for the SPARK metric source, any Spark metric (https://spark.apache.org/docs/latest/monitoring.html#metrics) can be specified).Provide metrics in the following format: METRIC_SOURCE: INSTANCE:GROUP:METRIC Use camelcase as appropriate.Examples: yarn:ResourceManager:QueueMetrics:AppsCompleted spark:driver:DAGScheduler:job.allJobs sparkHistoryServer:JVM:Memory:NonHeapMemoryUsage.committed hiveserver2:JVM:Memory:NonHeapMemoryUsage.used Notes: Only the specified overridden metrics will be collected for the metric source. For example, if one or more spark:executive metrics are listed as metric overrides, other SPARK metrics will not be collected. The collection of the default metrics for other OSS metric sources is unaffected. For example, if both SPARK andd YARN metric sources are enabled, and overrides are provided for Spark metrics only, all default YARN metrics will be collected.
    #[serde(rename="metricOverrides")]
    
    pub metric_overrides: Option<Vec<String>>,
    /// Required. Default metrics are collected unless metricOverrides are specified for the metric source (see Available OSS metrics (https://cloud.google.com/dataproc/docs/guides/monitoring#available_oss_metrics) for more information).
    #[serde(rename="metricSource")]
    
    pub metric_source: Option<MetricMetricSourceEnum>,
}

impl client::Part for Metric {}


/// Deprecated. Used only for the deprecated beta. A full, namespace-isolated deployment target for an existing GKE cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NamespacedGkeDeploymentTarget {
    /// Optional. A namespace within the GKE cluster to deploy into.
    #[serde(rename="clusterNamespace")]
    
    pub cluster_namespace: Option<String>,
    /// Optional. The target GKE cluster to deploy to. Format: 'projects/{project}/locations/{location}/clusters/{cluster_id}'
    #[serde(rename="targetGkeCluster")]
    
    pub target_gke_cluster: Option<String>,
}

impl client::Part for NamespacedGkeDeploymentTarget {}


/// Dataproc Node Group. The Dataproc NodeGroup resource is not related to the Dataproc NodeGroupAffinity resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [regions clusters node groups create projects](ProjectRegionClusterNodeGroupCreateCall) (request)
/// * [regions clusters node groups get projects](ProjectRegionClusterNodeGroupGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NodeGroup {
    /// Optional. Node group labels. Label keys must consist of from 1 to 63 characters and conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). Label values can be empty. If specified, they must consist of from 1 to 63 characters and conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). The node group must have no more than 32 labelsn.
    
    pub labels: Option<HashMap<String, String>>,
    /// The Node group resource name (https://aip.dev/122).
    
    pub name: Option<String>,
    /// Optional. The node group instance group configuration.
    #[serde(rename="nodeGroupConfig")]
    
    pub node_group_config: Option<InstanceGroupConfig>,
    /// Required. Node group roles.
    
    pub roles: Option<Vec<NodeGroupRolesEnum>>,
}

impl client::RequestValue for NodeGroup {}
impl client::ResponseResult for NodeGroup {}


/// Node Group Affinity for clusters using sole-tenant node groups. The Dataproc NodeGroupAffinity resource is not related to the Dataproc NodeGroup resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NodeGroupAffinity {
    /// Required. The URI of a sole-tenant node group resource (https://cloud.google.com/compute/docs/reference/rest/v1/nodeGroups) that the cluster will be created on.A full URL, partial URI, or node group name are valid. Examples: https://www.googleapis.com/compute/v1/projects/[project_id]/zones/us-central1-a/nodeGroups/node-group-1 projects/[project_id]/zones/us-central1-a/nodeGroups/node-group-1 node-group-1
    #[serde(rename="nodeGroupUri")]
    
    pub node_group_uri: Option<String>,
}

impl client::Part for NodeGroupAffinity {}


/// Specifies an executable to run on a fully configured node and a timeout period for executable completion.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NodeInitializationAction {
    /// Required. Cloud Storage URI of executable file.
    #[serde(rename="executableFile")]
    
    pub executable_file: Option<String>,
    /// Optional. Amount of time executable has to complete. Default is 10 minutes (see JSON representation of Duration (https://developers.google.com/protocol-buffers/docs/proto3#json)).Cluster creation fails with an explanatory error message (the name of the executable that caused the error and the exceeded timeout period) if the executable is not completed at end of the timeout period.
    #[serde(rename="executionTimeout")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub execution_timeout: Option<client::chrono::Duration>,
}

impl client::Part for NodeInitializationAction {}


/// indicating a list of workers of same type
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NodePool {
    /// Required. A unique id of the node pool. Primary and Secondary workers can be specified using special reserved ids PRIMARY_WORKER_POOL and SECONDARY_WORKER_POOL respectively. Aux node pools can be referenced using corresponding pool id.
    
    pub id: Option<String>,
    /// Name of instances to be repaired. These instances must belong to specified node pool.
    #[serde(rename="instanceNames")]
    
    pub instance_names: Option<Vec<String>>,
    /// Required. Repair action to take on specified resources of the node pool.
    #[serde(rename="repairAction")]
    
    pub repair_action: Option<NodePoolRepairActionEnum>,
}

impl client::Part for NodePool {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations batches create projects](ProjectLocationBatchCreateCall) (response)
/// * [locations operations get projects](ProjectLocationOperationGetCall) (response)
/// * [locations workflow templates instantiate projects](ProjectLocationWorkflowTemplateInstantiateCall) (response)
/// * [locations workflow templates instantiate inline projects](ProjectLocationWorkflowTemplateInstantiateInlineCall) (response)
/// * [regions clusters node groups create projects](ProjectRegionClusterNodeGroupCreateCall) (response)
/// * [regions clusters node groups resize projects](ProjectRegionClusterNodeGroupResizeCall) (response)
/// * [regions clusters create projects](ProjectRegionClusterCreateCall) (response)
/// * [regions clusters delete projects](ProjectRegionClusterDeleteCall) (response)
/// * [regions clusters diagnose projects](ProjectRegionClusterDiagnoseCall) (response)
/// * [regions clusters inject credentials projects](ProjectRegionClusterInjectCredentialCall) (response)
/// * [regions clusters patch projects](ProjectRegionClusterPatchCall) (response)
/// * [regions clusters repair projects](ProjectRegionClusterRepairCall) (response)
/// * [regions clusters start projects](ProjectRegionClusterStartCall) (response)
/// * [regions clusters stop projects](ProjectRegionClusterStopCall) (response)
/// * [regions jobs submit as operation projects](ProjectRegionJobSubmitAsOperationCall) (response)
/// * [regions operations get projects](ProjectRegionOperationGetCall) (response)
/// * [regions workflow templates instantiate projects](ProjectRegionWorkflowTemplateInstantiateCall) (response)
/// * [regions workflow templates instantiate inline projects](ProjectRegionWorkflowTemplateInstantiateInlineCall) (response)
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


/// A job executed by the workflow.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderedJob {
    /// Optional. Job is a Hadoop job.
    #[serde(rename="hadoopJob")]
    
    pub hadoop_job: Option<HadoopJob>,
    /// Optional. Job is a Hive job.
    #[serde(rename="hiveJob")]
    
    pub hive_job: Option<HiveJob>,
    /// Optional. The labels to associate with this job.Label keys must be between 1 and 63 characters long, and must conform to the following regular expression: \p{Ll}\p{Lo}{0,62}Label values must be between 1 and 63 characters long, and must conform to the following regular expression: \p{Ll}\p{Lo}\p{N}_-{0,63}No more than 32 labels can be associated with a given job.
    
    pub labels: Option<HashMap<String, String>>,
    /// Optional. Job is a Pig job.
    #[serde(rename="pigJob")]
    
    pub pig_job: Option<PigJob>,
    /// Optional. The optional list of prerequisite job step_ids. If not specified, the job will start at the beginning of workflow.
    #[serde(rename="prerequisiteStepIds")]
    
    pub prerequisite_step_ids: Option<Vec<String>>,
    /// Optional. Job is a Presto job.
    #[serde(rename="prestoJob")]
    
    pub presto_job: Option<PrestoJob>,
    /// Optional. Job is a PySpark job.
    #[serde(rename="pysparkJob")]
    
    pub pyspark_job: Option<PySparkJob>,
    /// Optional. Job scheduling configuration.
    
    pub scheduling: Option<JobScheduling>,
    /// Optional. Job is a Spark job.
    #[serde(rename="sparkJob")]
    
    pub spark_job: Option<SparkJob>,
    /// Optional. Job is a SparkR job.
    #[serde(rename="sparkRJob")]
    
    pub spark_r_job: Option<SparkRJob>,
    /// Optional. Job is a SparkSql job.
    #[serde(rename="sparkSqlJob")]
    
    pub spark_sql_job: Option<SparkSqlJob>,
    /// Required. The step id. The id must be unique among all jobs within the template.The step id is used as prefix for job id, as job goog-dataproc-workflow-step-id label, and in prerequisiteStepIds field from other steps.The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), and hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of between 3 and 50 characters.
    #[serde(rename="stepId")]
    
    pub step_id: Option<String>,
    /// Optional. Job is a Trino job.
    #[serde(rename="trinoJob")]
    
    pub trino_job: Option<TrinoJob>,
}

impl client::Part for OrderedJob {}


/// Configuration for parameter validation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ParameterValidation {
    /// Validation based on regular expressions.
    
    pub regex: Option<RegexValidation>,
    /// Validation based on a list of allowed values.
    
    pub values: Option<ValueValidation>,
}

impl client::Part for ParameterValidation {}


/// Auxiliary services configuration for a workload.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PeripheralsConfig {
    /// Optional. Resource name of an existing Dataproc Metastore service.Example: projects/[project_id]/locations/[region]/services/[service_id]
    #[serde(rename="metastoreService")]
    
    pub metastore_service: Option<String>,
    /// Optional. The Spark History Server configuration for the workload.
    #[serde(rename="sparkHistoryServerConfig")]
    
    pub spark_history_server_config: Option<SparkHistoryServerConfig>,
}

impl client::Part for PeripheralsConfig {}


/// A Dataproc job for running Apache Pig (https://pig.apache.org/) queries on YARN.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PigJob {
    /// Optional. Whether to continue executing queries if a query fails. The default value is false. Setting to true can be useful when executing independent parallel queries.
    #[serde(rename="continueOnFailure")]
    
    pub continue_on_failure: Option<bool>,
    /// Optional. HCFS URIs of jar files to add to the CLASSPATH of the Pig Client and Hadoop MapReduce (MR) tasks. Can contain Pig UDFs.
    #[serde(rename="jarFileUris")]
    
    pub jar_file_uris: Option<Vec<String>>,
    /// Optional. The runtime log config for job execution.
    #[serde(rename="loggingConfig")]
    
    pub logging_config: Option<LoggingConfig>,
    /// Optional. A mapping of property names to values, used to configure Pig. Properties that conflict with values set by the Dataproc API may be overwritten. Can include properties set in /etc/hadoop/conf/*-site.xml, /etc/pig/conf/pig.properties, and classes in user code.
    
    pub properties: Option<HashMap<String, String>>,
    /// The HCFS URI of the script that contains the Pig queries.
    #[serde(rename="queryFileUri")]
    
    pub query_file_uri: Option<String>,
    /// A list of queries.
    #[serde(rename="queryList")]
    
    pub query_list: Option<QueryList>,
    /// Optional. Mapping of query variable names to values (equivalent to the Pig command: name=[value]).
    #[serde(rename="scriptVariables")]
    
    pub script_variables: Option<HashMap<String, String>>,
}

impl client::Part for PigJob {}


/// An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources.A Policy is a collection of bindings. A binding binds one or more members, or principals, to a single role. Principals can be user accounts, service accounts, Google groups, and domains (such as G Suite). A role is a named list of permissions; each role can be an IAM predefined role or a user-created custom role.For some types of Google Cloud resources, a binding can also specify a condition, which is a logical expression that allows access to a resource only if the expression evaluates to true. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the IAM documentation (https://cloud.google.com/iam/help/conditions/resource-policies).JSON example: { “bindings”: \[ { “role”: “roles/resourcemanager.organizationAdmin”, “members”: \[ “user:mike@example.com”, “group:admins@example.com”, “domain:google.com”, “serviceAccount:my-project-id@appspot.gserviceaccount.com” \] }, { “role”: “roles/resourcemanager.organizationViewer”, “members”: \[ “user:eve@example.com” \], “condition”: { “title”: “expirable access”, “description”: “Does not grant access after Sep 2020”, “expression”: “request.time \< timestamp(‘2020-10-01T00:00:00.000Z’)”, } } \], “etag”: “BwWWja0YfJA=”, “version”: 3 } YAML example: bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time \< timestamp(‘2020-10-01T00:00:00.000Z’) etag: BwWWja0YfJA= version: 3 For a description of IAM and its features, see the IAM documentation (https://cloud.google.com/iam/docs/).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations autoscaling policies get iam policy projects](ProjectLocationAutoscalingPolicyGetIamPolicyCall) (response)
/// * [locations autoscaling policies set iam policy projects](ProjectLocationAutoscalingPolicySetIamPolicyCall) (response)
/// * [locations workflow templates get iam policy projects](ProjectLocationWorkflowTemplateGetIamPolicyCall) (response)
/// * [locations workflow templates set iam policy projects](ProjectLocationWorkflowTemplateSetIamPolicyCall) (response)
/// * [regions autoscaling policies get iam policy projects](ProjectRegionAutoscalingPolicyGetIamPolicyCall) (response)
/// * [regions autoscaling policies set iam policy projects](ProjectRegionAutoscalingPolicySetIamPolicyCall) (response)
/// * [regions clusters get iam policy projects](ProjectRegionClusterGetIamPolicyCall) (response)
/// * [regions clusters set iam policy projects](ProjectRegionClusterSetIamPolicyCall) (response)
/// * [regions jobs get iam policy projects](ProjectRegionJobGetIamPolicyCall) (response)
/// * [regions jobs set iam policy projects](ProjectRegionJobSetIamPolicyCall) (response)
/// * [regions operations get iam policy projects](ProjectRegionOperationGetIamPolicyCall) (response)
/// * [regions operations set iam policy projects](ProjectRegionOperationSetIamPolicyCall) (response)
/// * [regions workflow templates get iam policy projects](ProjectRegionWorkflowTemplateGetIamPolicyCall) (response)
/// * [regions workflow templates set iam policy projects](ProjectRegionWorkflowTemplateSetIamPolicyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Policy {
    /// Associates a list of members, or principals, with a role. Optionally, may specify a condition that determines how and when the bindings are applied. Each of the bindings must contain at least one principal.The bindings in a Policy can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the bindings grant 50 different roles to user:alice@example.com, and not to any other principal, then you can add another 1,450 principals to the bindings in the Policy.
    
    pub bindings: Option<Vec<Binding>>,
    /// etag is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the etag in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An etag is returned in the response to getIamPolicy, and systems are expected to put that etag in the request to setIamPolicy to ensure that their change will be applied to the same version of the policy.Important: If you use IAM Conditions, you must include the etag field whenever you call setIamPolicy. If you omit this field, then IAM allows you to overwrite a version 3 policy with a version 1 policy, and all of the conditions in the version 3 policy are lost.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
    /// Specifies the format of the policy.Valid values are 0, 1, and 3. Requests that specify an invalid value are rejected.Any operation that affects conditional role bindings must specify version 3. This requirement applies to the following operations: Getting a policy that includes a conditional role binding Adding a conditional role binding to a policy Changing a conditional role binding in a policy Removing any role binding, with or without a condition, from a policy that includes conditionsImportant: If you use IAM Conditions, you must include the etag field whenever you call setIamPolicy. If you omit this field, then IAM allows you to overwrite a version 3 policy with a version 1 policy, and all of the conditions in the version 3 policy are lost.If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset.To learn which resources support conditions in their IAM policies, see the IAM documentation (https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub version: Option<i32>,
}

impl client::ResponseResult for Policy {}


/// A Dataproc job for running Presto (https://prestosql.io/) queries. IMPORTANT: The Dataproc Presto Optional Component (https://cloud.google.com/dataproc/docs/concepts/components/presto) must be enabled when the cluster is created to submit a Presto job to the cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PrestoJob {
    /// Optional. Presto client tags to attach to this query
    #[serde(rename="clientTags")]
    
    pub client_tags: Option<Vec<String>>,
    /// Optional. Whether to continue executing queries if a query fails. The default value is false. Setting to true can be useful when executing independent parallel queries.
    #[serde(rename="continueOnFailure")]
    
    pub continue_on_failure: Option<bool>,
    /// Optional. The runtime log config for job execution.
    #[serde(rename="loggingConfig")]
    
    pub logging_config: Option<LoggingConfig>,
    /// Optional. The format in which query output will be displayed. See the Presto documentation for supported output formats
    #[serde(rename="outputFormat")]
    
    pub output_format: Option<String>,
    /// Optional. A mapping of property names to values. Used to set Presto session properties (https://prestodb.io/docs/current/sql/set-session.html) Equivalent to using the --session flag in the Presto CLI
    
    pub properties: Option<HashMap<String, String>>,
    /// The HCFS URI of the script that contains SQL queries.
    #[serde(rename="queryFileUri")]
    
    pub query_file_uri: Option<String>,
    /// A list of queries.
    #[serde(rename="queryList")]
    
    pub query_list: Option<QueryList>,
}

impl client::Part for PrestoJob {}


/// A configuration for running an Apache PySpark (https://spark.apache.org/docs/latest/api/python/getting_started/quickstart.html) batch workload.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PySparkBatch {
    /// Optional. HCFS URIs of archives to be extracted into the working directory of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip.
    #[serde(rename="archiveUris")]
    
    pub archive_uris: Option<Vec<String>>,
    /// Optional. The arguments to pass to the driver. Do not include arguments that can be set as batch properties, such as --conf, since a collision can occur that causes an incorrect batch submission.
    
    pub args: Option<Vec<String>>,
    /// Optional. HCFS URIs of files to be placed in the working directory of each executor.
    #[serde(rename="fileUris")]
    
    pub file_uris: Option<Vec<String>>,
    /// Optional. HCFS URIs of jar files to add to the classpath of the Spark driver and tasks.
    #[serde(rename="jarFileUris")]
    
    pub jar_file_uris: Option<Vec<String>>,
    /// Required. The HCFS URI of the main Python file to use as the Spark driver. Must be a .py file.
    #[serde(rename="mainPythonFileUri")]
    
    pub main_python_file_uri: Option<String>,
    /// Optional. HCFS file URIs of Python files to pass to the PySpark framework. Supported file types: .py, .egg, and .zip.
    #[serde(rename="pythonFileUris")]
    
    pub python_file_uris: Option<Vec<String>>,
}

impl client::Part for PySparkBatch {}


/// A Dataproc job for running Apache PySpark (https://spark.apache.org/docs/0.9.0/python-programming-guide.html) applications on YARN.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PySparkJob {
    /// Optional. HCFS URIs of archives to be extracted into the working directory of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip.
    #[serde(rename="archiveUris")]
    
    pub archive_uris: Option<Vec<String>>,
    /// Optional. The arguments to pass to the driver. Do not include arguments, such as --conf, that can be set as job properties, since a collision may occur that causes an incorrect job submission.
    
    pub args: Option<Vec<String>>,
    /// Optional. HCFS URIs of files to be placed in the working directory of each executor. Useful for naively parallel tasks.
    #[serde(rename="fileUris")]
    
    pub file_uris: Option<Vec<String>>,
    /// Optional. HCFS URIs of jar files to add to the CLASSPATHs of the Python driver and tasks.
    #[serde(rename="jarFileUris")]
    
    pub jar_file_uris: Option<Vec<String>>,
    /// Optional. The runtime log config for job execution.
    #[serde(rename="loggingConfig")]
    
    pub logging_config: Option<LoggingConfig>,
    /// Required. The HCFS URI of the main Python file to use as the driver. Must be a .py file.
    #[serde(rename="mainPythonFileUri")]
    
    pub main_python_file_uri: Option<String>,
    /// Optional. A mapping of property names to values, used to configure PySpark. Properties that conflict with values set by the Dataproc API may be overwritten. Can include properties set in /etc/spark/conf/spark-defaults.conf and classes in user code.
    
    pub properties: Option<HashMap<String, String>>,
    /// Optional. HCFS file URIs of Python files to pass to the PySpark framework. Supported file types: .py, .egg, and .zip.
    #[serde(rename="pythonFileUris")]
    
    pub python_file_uris: Option<Vec<String>>,
}

impl client::Part for PySparkJob {}


/// A list of queries to run on a cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryList {
    /// Required. The queries to execute. You do not need to end a query expression with a semicolon. Multiple queries can be specified in one string by separating each with a semicolon. Here is an example of a Dataproc API snippet that uses a QueryList to specify a HiveJob: "hiveJob": { "queryList": { "queries": [ "query1", "query2", "query3;query4", ] } } 
    
    pub queries: Option<Vec<String>>,
}

impl client::Part for QueryList {}


/// Validation based on regular expressions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RegexValidation {
    /// Required. RE2 regular expressions used to validate the parameter's value. The value must match the regex in its entirety (substring matches are not sufficient).
    
    pub regexes: Option<Vec<String>>,
}

impl client::Part for RegexValidation {}


/// A request to repair a cluster.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [regions clusters repair projects](ProjectRegionClusterRepairCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RepairClusterRequest {
    /// Optional. Specifying the cluster_uuid means the RPC will fail (with error NOT_FOUND) if a cluster with the specified UUID does not exist.
    #[serde(rename="clusterUuid")]
    
    pub cluster_uuid: Option<String>,
    /// Optional. Timeout for graceful YARN decommissioning. Graceful decommissioning facilitates the removal of cluster nodes without interrupting jobs in progress. The timeout specifies the amount of time to wait for jobs finish before forcefully removing nodes. The default timeout is 0 for forceful decommissioning, and the maximum timeout period is 1 day. (see JSON Mapping—Duration (https://developers.google.com/protocol-buffers/docs/proto3#json)).graceful_decommission_timeout is supported in Dataproc image versions 1.2+.
    #[serde(rename="gracefulDecommissionTimeout")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub graceful_decommission_timeout: Option<client::chrono::Duration>,
    /// Optional. Node pools and corresponding repair action to be taken. All node pools should be unique in this request. i.e. Multiple entries for the same node pool id are not allowed.
    #[serde(rename="nodePools")]
    
    pub node_pools: Option<Vec<NodePool>>,
    /// Optional. operation id of the parent operation sending the repair request
    #[serde(rename="parentOperationId")]
    
    pub parent_operation_id: Option<String>,
    /// Optional. A unique ID used to identify the request. If the server receives two RepairClusterRequests with the same ID, the second request is ignored, and the first google.longrunning.Operation created and stored in the backend is returned.Recommendation: Set this value to a UUID (https://en.wikipedia.org/wiki/Universally_unique_identifier).The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), and hyphens (-). The maximum length is 40 characters.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::RequestValue for RepairClusterRequest {}


/// Reservation Affinity for consuming Zonal reservation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReservationAffinity {
    /// Optional. Type of reservation to consume
    #[serde(rename="consumeReservationType")]
    
    pub consume_reservation_type: Option<ReservationAffinityConsumeReservationTypeEnum>,
    /// Optional. Corresponds to the label key of reservation resource.
    
    pub key: Option<String>,
    /// Optional. Corresponds to the label values of reservation resource.
    
    pub values: Option<Vec<String>>,
}

impl client::Part for ReservationAffinity {}


/// A request to resize a node group.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [regions clusters node groups resize projects](ProjectRegionClusterNodeGroupResizeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResizeNodeGroupRequest {
    /// Optional. Timeout for graceful YARN decomissioning. Graceful decommissioning (https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/scaling-clusters#graceful_decommissioning) allows the removal of nodes from the Compute Engine node group without interrupting jobs in progress. This timeout specifies how long to wait for jobs in progress to finish before forcefully removing nodes (and potentially interrupting jobs). Default timeout is 0 (for forceful decommission), and the maximum allowed timeout is 1 day. (see JSON representation of Duration (https://developers.google.com/protocol-buffers/docs/proto3#json)).Only supported on Dataproc image versions 1.2 and higher.
    #[serde(rename="gracefulDecommissionTimeout")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub graceful_decommission_timeout: Option<client::chrono::Duration>,
    /// Optional. A unique ID used to identify the request. If the server receives two ResizeNodeGroupRequest (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#google.cloud.dataproc.v1.ResizeNodeGroupRequests) with the same ID, the second request is ignored and the first google.longrunning.Operation created and stored in the backend is returned.Recommendation: Set this value to a UUID (https://en.wikipedia.org/wiki/Universally_unique_identifier).The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), and hyphens (-). The maximum length is 40 characters.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
    /// Required. The number of running instances for the node group to maintain. The group adds or removes instances to maintain the number of instances specified by this parameter.
    
    pub size: Option<i32>,
}

impl client::RequestValue for ResizeNodeGroupRequest {}


/// Runtime configuration for a workload.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeConfig {
    /// Optional. Optional custom container image for the job runtime environment. If not specified, a default container image will be used.
    #[serde(rename="containerImage")]
    
    pub container_image: Option<String>,
    /// Optional. A mapping of property names to values, which are used to configure workload execution.
    
    pub properties: Option<HashMap<String, String>>,
    /// Optional. Version of the batch runtime.
    
    pub version: Option<String>,
}

impl client::Part for RuntimeConfig {}


/// Runtime information about workload execution.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeInfo {
    /// Output only. Approximate workload resource usage calculated after workload finishes (see Dataproc Serverless pricing (https://cloud.google.com/dataproc-serverless/pricing)).
    #[serde(rename="approximateUsage")]
    
    pub approximate_usage: Option<UsageMetrics>,
    /// Output only. Snapshot of current workload resource usage.
    #[serde(rename="currentUsage")]
    
    pub current_usage: Option<UsageSnapshot>,
    /// Output only. A URI pointing to the location of the diagnostics tarball.
    #[serde(rename="diagnosticOutputUri")]
    
    pub diagnostic_output_uri: Option<String>,
    /// Output only. Map of remote access endpoints (such as web interfaces and APIs) to their URIs.
    
    pub endpoints: Option<HashMap<String, String>>,
    /// Output only. A URI pointing to the location of the stdout and stderr of the workload.
    #[serde(rename="outputUri")]
    
    pub output_uri: Option<String>,
}

impl client::Part for RuntimeInfo {}


/// Security related configuration, including encryption, Kerberos, etc.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Optional. Identity related configuration, including service account based secure multi-tenancy user mappings.
    #[serde(rename="identityConfig")]
    
    pub identity_config: Option<IdentityConfig>,
    /// Optional. Kerberos related configuration.
    #[serde(rename="kerberosConfig")]
    
    pub kerberos_config: Option<KerberosConfig>,
}

impl client::Part for SecurityConfig {}


/// Request message for SetIamPolicy method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations autoscaling policies set iam policy projects](ProjectLocationAutoscalingPolicySetIamPolicyCall) (request)
/// * [locations workflow templates set iam policy projects](ProjectLocationWorkflowTemplateSetIamPolicyCall) (request)
/// * [regions autoscaling policies set iam policy projects](ProjectRegionAutoscalingPolicySetIamPolicyCall) (request)
/// * [regions clusters set iam policy projects](ProjectRegionClusterSetIamPolicyCall) (request)
/// * [regions jobs set iam policy projects](ProjectRegionJobSetIamPolicyCall) (request)
/// * [regions operations set iam policy projects](ProjectRegionOperationSetIamPolicyCall) (request)
/// * [regions workflow templates set iam policy projects](ProjectRegionWorkflowTemplateSetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetIamPolicyRequest {
    /// REQUIRED: The complete policy to be applied to the resource. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them.
    
    pub policy: Option<Policy>,
}

impl client::RequestValue for SetIamPolicyRequest {}


/// Shielded Instance Config for clusters using Compute Engine Shielded VMs (https://cloud.google.com/security/shielded-cloud/shielded-vm).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ShieldedInstanceConfig {
    /// Optional. Defines whether instances have integrity monitoring enabled.
    #[serde(rename="enableIntegrityMonitoring")]
    
    pub enable_integrity_monitoring: Option<bool>,
    /// Optional. Defines whether instances have Secure Boot enabled.
    #[serde(rename="enableSecureBoot")]
    
    pub enable_secure_boot: Option<bool>,
    /// Optional. Defines whether instances have the vTPM enabled.
    #[serde(rename="enableVtpm")]
    
    pub enable_vtpm: Option<bool>,
}

impl client::Part for ShieldedInstanceConfig {}


/// Specifies the selection and config of software inside the cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SoftwareConfig {
    /// Optional. The version of software inside the cluster. It must be one of the supported Dataproc Versions (https://cloud.google.com/dataproc/docs/concepts/versioning/dataproc-versions#supported_dataproc_versions), such as "1.2" (including a subminor version, such as "1.2.29"), or the "preview" version (https://cloud.google.com/dataproc/docs/concepts/versioning/dataproc-versions#other_versions). If unspecified, it defaults to the latest Debian version.
    #[serde(rename="imageVersion")]
    
    pub image_version: Option<String>,
    /// Optional. The set of components to activate on the cluster.
    #[serde(rename="optionalComponents")]
    
    pub optional_components: Option<Vec<SoftwareConfigOptionalComponentsEnum>>,
    /// Optional. The properties to set on daemon config files.Property keys are specified in prefix:property format, for example core:hadoop.tmp.dir. The following are supported prefixes and their mappings: capacity-scheduler: capacity-scheduler.xml core: core-site.xml distcp: distcp-default.xml hdfs: hdfs-site.xml hive: hive-site.xml mapred: mapred-site.xml pig: pig.properties spark: spark-defaults.conf yarn: yarn-site.xmlFor more information, see Cluster properties (https://cloud.google.com/dataproc/docs/concepts/cluster-properties).
    
    pub properties: Option<HashMap<String, String>>,
}

impl client::Part for SoftwareConfig {}


/// A configuration for running an Apache Spark (https://spark.apache.org/) batch workload.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SparkBatch {
    /// Optional. HCFS URIs of archives to be extracted into the working directory of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip.
    #[serde(rename="archiveUris")]
    
    pub archive_uris: Option<Vec<String>>,
    /// Optional. The arguments to pass to the driver. Do not include arguments that can be set as batch properties, such as --conf, since a collision can occur that causes an incorrect batch submission.
    
    pub args: Option<Vec<String>>,
    /// Optional. HCFS URIs of files to be placed in the working directory of each executor.
    #[serde(rename="fileUris")]
    
    pub file_uris: Option<Vec<String>>,
    /// Optional. HCFS URIs of jar files to add to the classpath of the Spark driver and tasks.
    #[serde(rename="jarFileUris")]
    
    pub jar_file_uris: Option<Vec<String>>,
    /// Optional. The name of the driver main class. The jar file that contains the class must be in the classpath or specified in jar_file_uris.
    #[serde(rename="mainClass")]
    
    pub main_class: Option<String>,
    /// Optional. The HCFS URI of the jar file that contains the main class.
    #[serde(rename="mainJarFileUri")]
    
    pub main_jar_file_uri: Option<String>,
}

impl client::Part for SparkBatch {}


/// Spark History Server configuration for the workload.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SparkHistoryServerConfig {
    /// Optional. Resource name of an existing Dataproc Cluster to act as a Spark History Server for the workload.Example: projects/[project_id]/regions/[region]/clusters/[cluster_name]
    #[serde(rename="dataprocCluster")]
    
    pub dataproc_cluster: Option<String>,
}

impl client::Part for SparkHistoryServerConfig {}


/// A Dataproc job for running Apache Spark (https://spark.apache.org/) applications on YARN.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SparkJob {
    /// Optional. HCFS URIs of archives to be extracted into the working directory of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip.
    #[serde(rename="archiveUris")]
    
    pub archive_uris: Option<Vec<String>>,
    /// Optional. The arguments to pass to the driver. Do not include arguments, such as --conf, that can be set as job properties, since a collision may occur that causes an incorrect job submission.
    
    pub args: Option<Vec<String>>,
    /// Optional. HCFS URIs of files to be placed in the working directory of each executor. Useful for naively parallel tasks.
    #[serde(rename="fileUris")]
    
    pub file_uris: Option<Vec<String>>,
    /// Optional. HCFS URIs of jar files to add to the CLASSPATHs of the Spark driver and tasks.
    #[serde(rename="jarFileUris")]
    
    pub jar_file_uris: Option<Vec<String>>,
    /// Optional. The runtime log config for job execution.
    #[serde(rename="loggingConfig")]
    
    pub logging_config: Option<LoggingConfig>,
    /// The name of the driver's main class. The jar file that contains the class must be in the default CLASSPATH or specified in jar_file_uris.
    #[serde(rename="mainClass")]
    
    pub main_class: Option<String>,
    /// The HCFS URI of the jar file that contains the main class.
    #[serde(rename="mainJarFileUri")]
    
    pub main_jar_file_uri: Option<String>,
    /// Optional. A mapping of property names to values, used to configure Spark. Properties that conflict with values set by the Dataproc API may be overwritten. Can include properties set in /etc/spark/conf/spark-defaults.conf and classes in user code.
    
    pub properties: Option<HashMap<String, String>>,
}

impl client::Part for SparkJob {}


/// A configuration for running an Apache SparkR (https://spark.apache.org/docs/latest/sparkr.html) batch workload.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SparkRBatch {
    /// Optional. HCFS URIs of archives to be extracted into the working directory of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip.
    #[serde(rename="archiveUris")]
    
    pub archive_uris: Option<Vec<String>>,
    /// Optional. The arguments to pass to the Spark driver. Do not include arguments that can be set as batch properties, such as --conf, since a collision can occur that causes an incorrect batch submission.
    
    pub args: Option<Vec<String>>,
    /// Optional. HCFS URIs of files to be placed in the working directory of each executor.
    #[serde(rename="fileUris")]
    
    pub file_uris: Option<Vec<String>>,
    /// Required. The HCFS URI of the main R file to use as the driver. Must be a .R or .r file.
    #[serde(rename="mainRFileUri")]
    
    pub main_r_file_uri: Option<String>,
}

impl client::Part for SparkRBatch {}


/// A Dataproc job for running Apache SparkR (https://spark.apache.org/docs/latest/sparkr.html) applications on YARN.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SparkRJob {
    /// Optional. HCFS URIs of archives to be extracted into the working directory of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip.
    #[serde(rename="archiveUris")]
    
    pub archive_uris: Option<Vec<String>>,
    /// Optional. The arguments to pass to the driver. Do not include arguments, such as --conf, that can be set as job properties, since a collision may occur that causes an incorrect job submission.
    
    pub args: Option<Vec<String>>,
    /// Optional. HCFS URIs of files to be placed in the working directory of each executor. Useful for naively parallel tasks.
    #[serde(rename="fileUris")]
    
    pub file_uris: Option<Vec<String>>,
    /// Optional. The runtime log config for job execution.
    #[serde(rename="loggingConfig")]
    
    pub logging_config: Option<LoggingConfig>,
    /// Required. The HCFS URI of the main R file to use as the driver. Must be a .R file.
    #[serde(rename="mainRFileUri")]
    
    pub main_r_file_uri: Option<String>,
    /// Optional. A mapping of property names to values, used to configure SparkR. Properties that conflict with values set by the Dataproc API may be overwritten. Can include properties set in /etc/spark/conf/spark-defaults.conf and classes in user code.
    
    pub properties: Option<HashMap<String, String>>,
}

impl client::Part for SparkRJob {}


/// A configuration for running Apache Spark SQL (https://spark.apache.org/sql/) queries as a batch workload.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SparkSqlBatch {
    /// Optional. HCFS URIs of jar files to be added to the Spark CLASSPATH.
    #[serde(rename="jarFileUris")]
    
    pub jar_file_uris: Option<Vec<String>>,
    /// Required. The HCFS URI of the script that contains Spark SQL queries to execute.
    #[serde(rename="queryFileUri")]
    
    pub query_file_uri: Option<String>,
    /// Optional. Mapping of query variable names to values (equivalent to the Spark SQL command: SET name="value";).
    #[serde(rename="queryVariables")]
    
    pub query_variables: Option<HashMap<String, String>>,
}

impl client::Part for SparkSqlBatch {}


/// A Dataproc job for running Apache Spark SQL (https://spark.apache.org/sql/) queries.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SparkSqlJob {
    /// Optional. HCFS URIs of jar files to be added to the Spark CLASSPATH.
    #[serde(rename="jarFileUris")]
    
    pub jar_file_uris: Option<Vec<String>>,
    /// Optional. The runtime log config for job execution.
    #[serde(rename="loggingConfig")]
    
    pub logging_config: Option<LoggingConfig>,
    /// Optional. A mapping of property names to values, used to configure Spark SQL's SparkConf. Properties that conflict with values set by the Dataproc API may be overwritten.
    
    pub properties: Option<HashMap<String, String>>,
    /// The HCFS URI of the script that contains SQL queries.
    #[serde(rename="queryFileUri")]
    
    pub query_file_uri: Option<String>,
    /// A list of queries.
    #[serde(rename="queryList")]
    
    pub query_list: Option<QueryList>,
    /// Optional. Mapping of query variable names to values (equivalent to the Spark SQL command: SET name="value";).
    #[serde(rename="scriptVariables")]
    
    pub script_variables: Option<HashMap<String, String>>,
}

impl client::Part for SparkSqlJob {}


/// Basic autoscaling configurations for Spark Standalone.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SparkStandaloneAutoscalingConfig {
    /// Required. Timeout for Spark graceful decommissioning of spark workers. Specifies the duration to wait for spark worker to complete spark decomissioning tasks before forcefully removing workers. Only applicable to downscaling operations.Bounds: 0s, 1d.
    #[serde(rename="gracefulDecommissionTimeout")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub graceful_decommission_timeout: Option<client::chrono::Duration>,
    /// Required. Fraction of required executors to remove from Spark Serverless clusters. A scale-down factor of 1.0 will result in scaling down so that there are no more executors for the Spark Job.(more aggressive scaling). A scale-down factor closer to 0 will result in a smaller magnitude of scaling donw (less aggressive scaling).Bounds: 0.0, 1.0.
    #[serde(rename="scaleDownFactor")]
    
    pub scale_down_factor: Option<f64>,
    /// Optional. Minimum scale-down threshold as a fraction of total cluster size before scaling occurs. For example, in a 20-worker cluster, a threshold of 0.1 means the autoscaler must recommend at least a 2 worker scale-down for the cluster to scale. A threshold of 0 means the autoscaler will scale down on any recommended change.Bounds: 0.0, 1.0. Default: 0.0.
    #[serde(rename="scaleDownMinWorkerFraction")]
    
    pub scale_down_min_worker_fraction: Option<f64>,
    /// Required. Fraction of required workers to add to Spark Standalone clusters. A scale-up factor of 1.0 will result in scaling up so that there are no more required workers for the Spark Job (more aggressive scaling). A scale-up factor closer to 0 will result in a smaller magnitude of scaling up (less aggressive scaling).Bounds: 0.0, 1.0.
    #[serde(rename="scaleUpFactor")]
    
    pub scale_up_factor: Option<f64>,
    /// Optional. Minimum scale-up threshold as a fraction of total cluster size before scaling occurs. For example, in a 20-worker cluster, a threshold of 0.1 means the autoscaler must recommend at least a 2-worker scale-up for the cluster to scale. A threshold of 0 means the autoscaler will scale up on any recommended change.Bounds: 0.0, 1.0. Default: 0.0.
    #[serde(rename="scaleUpMinWorkerFraction")]
    
    pub scale_up_min_worker_fraction: Option<f64>,
}

impl client::Part for SparkStandaloneAutoscalingConfig {}


/// A request to start a cluster.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [regions clusters start projects](ProjectRegionClusterStartCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StartClusterRequest {
    /// Optional. Specifying the cluster_uuid means the RPC will fail (with error NOT_FOUND) if a cluster with the specified UUID does not exist.
    #[serde(rename="clusterUuid")]
    
    pub cluster_uuid: Option<String>,
    /// Optional. A unique ID used to identify the request. If the server receives two StartClusterRequest (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#google.cloud.dataproc.v1.StartClusterRequest)s with the same id, then the second request will be ignored and the first google.longrunning.Operation created and stored in the backend is returned.Recommendation: Set this value to a UUID (https://en.wikipedia.org/wiki/Universally_unique_identifier).The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), and hyphens (-). The maximum length is 40 characters.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::RequestValue for StartClusterRequest {}


/// Historical state information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StateHistory {
    /// Output only. The state of the batch at this point in history.
    
    pub state: Option<StateHistoryStateEnum>,
    /// Output only. Details about the state at this point in history.
    #[serde(rename="stateMessage")]
    
    pub state_message: Option<String>,
    /// Output only. The time when the batch entered the historical state.
    #[serde(rename="stateStartTime")]
    
    pub state_start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for StateHistory {}


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


/// A request to stop a cluster.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [regions clusters stop projects](ProjectRegionClusterStopCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StopClusterRequest {
    /// Optional. Specifying the cluster_uuid means the RPC will fail (with error NOT_FOUND) if a cluster with the specified UUID does not exist.
    #[serde(rename="clusterUuid")]
    
    pub cluster_uuid: Option<String>,
    /// Optional. A unique ID used to identify the request. If the server receives two StopClusterRequest (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#google.cloud.dataproc.v1.StopClusterRequest)s with the same id, then the second request will be ignored and the first google.longrunning.Operation created and stored in the backend is returned.Recommendation: Set this value to a UUID (https://en.wikipedia.org/wiki/Universally_unique_identifier).The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), and hyphens (-). The maximum length is 40 characters.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::RequestValue for StopClusterRequest {}


/// A request to submit a job.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [regions jobs submit projects](ProjectRegionJobSubmitCall) (request)
/// * [regions jobs submit as operation projects](ProjectRegionJobSubmitAsOperationCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubmitJobRequest {
    /// Required. The job resource.
    
    pub job: Option<Job>,
    /// Optional. A unique id used to identify the request. If the server receives two SubmitJobRequest (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#google.cloud.dataproc.v1.SubmitJobRequest)s with the same id, then the second request will be ignored and the first Job created and stored in the backend is returned.It is recommended to always set this value to a UUID (https://en.wikipedia.org/wiki/Universally_unique_identifier).The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), and hyphens (-). The maximum length is 40 characters.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::RequestValue for SubmitJobRequest {}


/// A configurable parameter that replaces one or more fields in the template. Parameterizable fields: - Labels - File uris - Job properties - Job arguments - Script variables - Main class (in HadoopJob and SparkJob) - Zone (in ClusterSelector)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TemplateParameter {
    /// Optional. Brief description of the parameter. Must not exceed 1024 characters.
    
    pub description: Option<String>,
    /// Required. Paths to all fields that the parameter replaces. A field is allowed to appear in at most one parameter's list of field paths.A field path is similar in syntax to a google.protobuf.FieldMask. For example, a field path that references the zone field of a workflow template's cluster selector would be specified as placement.clusterSelector.zone.Also, field paths can reference fields using the following syntax: Values in maps can be referenced by key: labels'key' placement.clusterSelector.clusterLabels'key' placement.managedCluster.labels'key' placement.clusterSelector.clusterLabels'key' jobs'step-id'.labels'key' Jobs in the jobs list can be referenced by step-id: jobs'step-id'.hadoopJob.mainJarFileUri jobs'step-id'.hiveJob.queryFileUri jobs'step-id'.pySparkJob.mainPythonFileUri jobs'step-id'.hadoopJob.jarFileUris0 jobs'step-id'.hadoopJob.archiveUris0 jobs'step-id'.hadoopJob.fileUris0 jobs'step-id'.pySparkJob.pythonFileUris0 Items in repeated fields can be referenced by a zero-based index: jobs'step-id'.sparkJob.args0 Other examples: jobs'step-id'.hadoopJob.properties'key' jobs'step-id'.hadoopJob.args0 jobs'step-id'.hiveJob.scriptVariables'key' jobs'step-id'.hadoopJob.mainJarFileUri placement.clusterSelector.zoneIt may not be possible to parameterize maps and repeated fields in their entirety since only individual map values and individual items in repeated fields can be referenced. For example, the following field paths are invalid: placement.clusterSelector.clusterLabels jobs'step-id'.sparkJob.args
    
    pub fields: Option<Vec<String>>,
    /// Required. Parameter name. The parameter name is used as the key, and paired with the parameter value, which are passed to the template when the template is instantiated. The name must contain only capital letters (A-Z), numbers (0-9), and underscores (_), and must not start with a number. The maximum length is 40 characters.
    
    pub name: Option<String>,
    /// Optional. Validation rules to be applied to this parameter's value.
    
    pub validation: Option<ParameterValidation>,
}

impl client::Part for TemplateParameter {}


/// Request message for TestIamPermissions method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations autoscaling policies test iam permissions projects](ProjectLocationAutoscalingPolicyTestIamPermissionCall) (request)
/// * [locations workflow templates test iam permissions projects](ProjectLocationWorkflowTemplateTestIamPermissionCall) (request)
/// * [regions autoscaling policies test iam permissions projects](ProjectRegionAutoscalingPolicyTestIamPermissionCall) (request)
/// * [regions clusters test iam permissions projects](ProjectRegionClusterTestIamPermissionCall) (request)
/// * [regions jobs test iam permissions projects](ProjectRegionJobTestIamPermissionCall) (request)
/// * [regions operations test iam permissions projects](ProjectRegionOperationTestIamPermissionCall) (request)
/// * [regions workflow templates test iam permissions projects](ProjectRegionWorkflowTemplateTestIamPermissionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsRequest {
    /// The set of permissions to check for the resource. Permissions with wildcards (such as * or storage.*) are not allowed. For more information see IAM Overview (https://cloud.google.com/iam/docs/overview#permissions).
    
    pub permissions: Option<Vec<String>>,
}

impl client::RequestValue for TestIamPermissionsRequest {}


/// Response message for TestIamPermissions method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations autoscaling policies test iam permissions projects](ProjectLocationAutoscalingPolicyTestIamPermissionCall) (response)
/// * [locations workflow templates test iam permissions projects](ProjectLocationWorkflowTemplateTestIamPermissionCall) (response)
/// * [regions autoscaling policies test iam permissions projects](ProjectRegionAutoscalingPolicyTestIamPermissionCall) (response)
/// * [regions clusters test iam permissions projects](ProjectRegionClusterTestIamPermissionCall) (response)
/// * [regions jobs test iam permissions projects](ProjectRegionJobTestIamPermissionCall) (response)
/// * [regions operations test iam permissions projects](ProjectRegionOperationTestIamPermissionCall) (response)
/// * [regions workflow templates test iam permissions projects](ProjectRegionWorkflowTemplateTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsResponse {
    /// A subset of TestPermissionsRequest.permissions that the caller is allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for TestIamPermissionsResponse {}


/// A Dataproc job for running Trino (https://trino.io/) queries. IMPORTANT: The Dataproc Trino Optional Component (https://cloud.google.com/dataproc/docs/concepts/components/trino) must be enabled when the cluster is created to submit a Trino job to the cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TrinoJob {
    /// Optional. Trino client tags to attach to this query
    #[serde(rename="clientTags")]
    
    pub client_tags: Option<Vec<String>>,
    /// Optional. Whether to continue executing queries if a query fails. The default value is false. Setting to true can be useful when executing independent parallel queries.
    #[serde(rename="continueOnFailure")]
    
    pub continue_on_failure: Option<bool>,
    /// Optional. The runtime log config for job execution.
    #[serde(rename="loggingConfig")]
    
    pub logging_config: Option<LoggingConfig>,
    /// Optional. The format in which query output will be displayed. See the Trino documentation for supported output formats
    #[serde(rename="outputFormat")]
    
    pub output_format: Option<String>,
    /// Optional. A mapping of property names to values. Used to set Trino session properties (https://trino.io/docs/current/sql/set-session.html) Equivalent to using the --session flag in the Trino CLI
    
    pub properties: Option<HashMap<String, String>>,
    /// The HCFS URI of the script that contains SQL queries.
    #[serde(rename="queryFileUri")]
    
    pub query_file_uri: Option<String>,
    /// A list of queries.
    #[serde(rename="queryList")]
    
    pub query_list: Option<QueryList>,
}

impl client::Part for TrinoJob {}


/// Usage metrics represent approximate total resources consumed by a workload.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UsageMetrics {
    /// Optional. DCU (Dataproc Compute Units) usage in (milliDCU x seconds) (see Dataproc Serverless pricing (https://cloud.google.com/dataproc-serverless/pricing)).
    #[serde(rename="milliDcuSeconds")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub milli_dcu_seconds: Option<i64>,
    /// Optional. Shuffle storage usage in (GB x seconds) (see Dataproc Serverless pricing (https://cloud.google.com/dataproc-serverless/pricing)).
    #[serde(rename="shuffleStorageGbSeconds")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub shuffle_storage_gb_seconds: Option<i64>,
}

impl client::Part for UsageMetrics {}


/// The usage snaphot represents the resources consumed by a workload at a specified time.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UsageSnapshot {
    /// Optional. Milli (one-thousandth) Dataproc Compute Units (DCUs) (see Dataproc Serverless pricing (https://cloud.google.com/dataproc-serverless/pricing)).
    #[serde(rename="milliDcu")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub milli_dcu: Option<i64>,
    /// Optional. Shuffle Storage in gigabytes (GB). (see Dataproc Serverless pricing (https://cloud.google.com/dataproc-serverless/pricing))
    #[serde(rename="shuffleStorageGb")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub shuffle_storage_gb: Option<i64>,
    /// Optional. The timestamp of the usage snapshot.
    #[serde(rename="snapshotTime")]
    
    pub snapshot_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for UsageSnapshot {}


/// Validation based on a list of allowed values.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ValueValidation {
    /// Required. List of allowed values for the parameter.
    
    pub values: Option<Vec<String>>,
}

impl client::Part for ValueValidation {}


/// The Dataproc cluster config for a cluster that does not directly control the underlying compute resources, such as a Dataproc-on-GKE cluster (https://cloud.google.com/dataproc/docs/guides/dpgke/dataproc-gke).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VirtualClusterConfig {
    /// Optional. Configuration of auxiliary services used by this cluster.
    #[serde(rename="auxiliaryServicesConfig")]
    
    pub auxiliary_services_config: Option<AuxiliaryServicesConfig>,
    /// Required. The configuration for running the Dataproc cluster on Kubernetes.
    #[serde(rename="kubernetesClusterConfig")]
    
    pub kubernetes_cluster_config: Option<KubernetesClusterConfig>,
    /// Optional. A Cloud Storage bucket used to stage job dependencies, config files, and job driver console output. If you do not specify a staging bucket, Cloud Dataproc will determine a Cloud Storage location (US, ASIA, or EU) for your cluster's staging bucket according to the Compute Engine zone where your cluster is deployed, and then create and manage this project-level, per-location bucket (see Dataproc staging and temp buckets (https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/staging-bucket)). This field requires a Cloud Storage bucket name, not a gs://... URI to a Cloud Storage bucket.
    #[serde(rename="stagingBucket")]
    
    pub staging_bucket: Option<String>,
}

impl client::Part for VirtualClusterConfig {}


/// A Dataproc workflow template resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations workflow templates create projects](ProjectLocationWorkflowTemplateCreateCall) (request|response)
/// * [locations workflow templates get projects](ProjectLocationWorkflowTemplateGetCall) (response)
/// * [locations workflow templates instantiate inline projects](ProjectLocationWorkflowTemplateInstantiateInlineCall) (request)
/// * [locations workflow templates update projects](ProjectLocationWorkflowTemplateUpdateCall) (request|response)
/// * [regions workflow templates create projects](ProjectRegionWorkflowTemplateCreateCall) (request|response)
/// * [regions workflow templates get projects](ProjectRegionWorkflowTemplateGetCall) (response)
/// * [regions workflow templates instantiate inline projects](ProjectRegionWorkflowTemplateInstantiateInlineCall) (request)
/// * [regions workflow templates update projects](ProjectRegionWorkflowTemplateUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WorkflowTemplate {
    /// Output only. The time template was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Timeout duration for the DAG of jobs, expressed in seconds (see JSON representation of duration (https://developers.google.com/protocol-buffers/docs/proto3#json)). The timeout duration must be from 10 minutes ("600s") to 24 hours ("86400s"). The timer begins when the first job is submitted. If the workflow is running at the end of the timeout period, any remaining jobs are cancelled, the workflow is ended, and if the workflow was running on a managed cluster, the cluster is deleted.
    #[serde(rename="dagTimeout")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub dag_timeout: Option<client::chrono::Duration>,
    /// no description provided
    
    pub id: Option<String>,
    /// Required. The Directed Acyclic Graph of Jobs to submit.
    
    pub jobs: Option<Vec<OrderedJob>>,
    /// Optional. The labels to associate with this template. These labels will be propagated to all jobs and clusters created by the workflow instance.Label keys must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt).Label values may be empty, but, if present, must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt).No more than 32 labels can be associated with a template.
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. The resource name of the workflow template, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates, the resource name of the template has the following format: projects/{project_id}/regions/{region}/workflowTemplates/{template_id} For projects.locations.workflowTemplates, the resource name of the template has the following format: projects/{project_id}/locations/{location}/workflowTemplates/{template_id}
    
    pub name: Option<String>,
    /// Optional. Template parameters whose values are substituted into the template. Values for parameters must be provided when the template is instantiated.
    
    pub parameters: Option<Vec<TemplateParameter>>,
    /// Required. WorkflowTemplate scheduling information.
    
    pub placement: Option<WorkflowTemplatePlacement>,
    /// Output only. The time template was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Used to perform a consistent read-modify-write.This field should be left blank for a CreateWorkflowTemplate request. It is required for an UpdateWorkflowTemplate request, and must match the current server version. A typical update template flow would fetch the current template with a GetWorkflowTemplate request, which will return the current template with the version field filled in with the current server version. The user updates other fields in the template, then returns it as part of the UpdateWorkflowTemplate request.
    
    pub version: Option<i32>,
}

impl client::RequestValue for WorkflowTemplate {}
impl client::ResponseResult for WorkflowTemplate {}


/// Specifies workflow execution target.Either managed_cluster or cluster_selector is required.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WorkflowTemplatePlacement {
    /// Optional. A selector that chooses target cluster for jobs based on metadata.The selector is evaluated at the time each job is submitted.
    #[serde(rename="clusterSelector")]
    
    pub cluster_selector: Option<ClusterSelector>,
    /// A cluster that is managed by the workflow.
    #[serde(rename="managedCluster")]
    
    pub managed_cluster: Option<ManagedCluster>,
}

impl client::Part for WorkflowTemplatePlacement {}


/// A YARN application created by a job. Application information is a subset of org.apache.hadoop.yarn.proto.YarnProtos.ApplicationReportProto.Beta Feature: This report is available for testing purposes only. It may be changed before final release.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct YarnApplication {
    /// Required. The application name.
    
    pub name: Option<String>,
    /// Required. The numerical progress of the application, from 1 to 100.
    
    pub progress: Option<f32>,
    /// Required. The application state.
    
    pub state: Option<YarnApplicationStateEnum>,
    /// Optional. The HTTP URL of the ApplicationMaster, HistoryServer, or TimelineServer that provides application-specific information. The URL uses the internal hostname, and requires a proxy server for resolution and, possibly, access.
    #[serde(rename="trackingUrl")]
    
    pub tracking_url: Option<String>,
}

impl client::Part for YarnApplication {}


