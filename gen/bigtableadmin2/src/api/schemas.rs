use super::*;
/// A configuration object describing how Cloud Bigtable should treat traffic from a particular end user application.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances app profiles create projects](ProjectInstanceAppProfileCreateCall) (request|response)
/// * [instances app profiles get projects](ProjectInstanceAppProfileGetCall) (response)
/// * [instances app profiles patch projects](ProjectInstanceAppProfilePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppProfile {
    /// Long form description of the use case for this AppProfile.
    
    pub description: Option<String>,
    /// Strongly validated etag for optimistic concurrency control. Preserve the value returned from `GetAppProfile` when calling `UpdateAppProfile` to fail the request if there has been a modification in the mean time. The `update_mask` of the request need not include `etag` for this protection to apply. See [Wikipedia](https://en.wikipedia.org/wiki/HTTP_ETag) and [RFC 7232](https://tools.ietf.org/html/rfc7232#section-2.3) for more details.
    
    pub etag: Option<String>,
    /// Use a multi-cluster routing policy.
    #[serde(rename="multiClusterRoutingUseAny")]
    
    pub multi_cluster_routing_use_any: Option<MultiClusterRoutingUseAny>,
    /// The unique name of the app profile. Values are of the form `projects/{project}/instances/{instance}/appProfiles/_a-zA-Z0-9*`.
    
    pub name: Option<String>,
    /// Use a single-cluster routing policy.
    #[serde(rename="singleClusterRouting")]
    
    pub single_cluster_routing: Option<SingleClusterRouting>,
}

impl client::RequestValue for AppProfile {}
impl client::ResponseResult for AppProfile {}


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


/// Limits for the number of nodes a Cluster can autoscale up/down to.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AutoscalingLimits {
    /// Required. Maximum number of nodes to scale up to.
    #[serde(rename="maxServeNodes")]
    
    pub max_serve_nodes: Option<i32>,
    /// Required. Minimum number of nodes to scale down to.
    #[serde(rename="minServeNodes")]
    
    pub min_serve_nodes: Option<i32>,
}

impl client::Part for AutoscalingLimits {}


/// The Autoscaling targets for a Cluster. These determine the recommended nodes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AutoscalingTargets {
    /// The cpu utilization that the Autoscaler should be trying to achieve. This number is on a scale from 0 (no utilization) to 100 (total utilization), and is limited between 10 and 80, otherwise it will return INVALID_ARGUMENT error.
    #[serde(rename="cpuUtilizationPercent")]
    
    pub cpu_utilization_percent: Option<i32>,
    /// The storage utilization that the Autoscaler should be trying to achieve. This number is limited between 2560 (2.5TiB) and 5120 (5TiB) for a SSD cluster and between 8192 (8TiB) and 16384 (16TiB) for an HDD cluster, otherwise it will return INVALID_ARGUMENT error. If this value is set to 0, it will be treated as if it were set to the default value: 2560 for SSD, 8192 for HDD.
    #[serde(rename="storageUtilizationGibPerNode")]
    
    pub storage_utilization_gib_per_node: Option<i32>,
}

impl client::Part for AutoscalingTargets {}


/// A backup of a Cloud Bigtable table.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances clusters backups create projects](ProjectInstanceClusterBackupCreateCall) (request)
/// * [instances clusters backups get projects](ProjectInstanceClusterBackupGetCall) (response)
/// * [instances clusters backups patch projects](ProjectInstanceClusterBackupPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Backup {
    /// Output only. The encryption information for the backup.
    #[serde(rename="encryptionInfo")]
    
    pub encryption_info: Option<EncryptionInfo>,
    /// Output only. `end_time` is the time that the backup was finished. The row data in the backup will be no newer than this timestamp.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. The expiration time of the backup, with microseconds granularity that must be at least 6 hours and at most 30 days from the time the request is received. Once the `expire_time` has passed, Cloud Bigtable will delete the backup and free the resources used by the backup.
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A globally unique identifier for the backup which cannot be changed. Values are of the form `projects/{project}/instances/{instance}/clusters/{cluster}/ backups/_a-zA-Z0-9*` The final segment of the name must be between 1 and 50 characters in length. The backup is stored in the cluster identified by the prefix of the backup name of the form `projects/{project}/instances/{instance}/clusters/{cluster}`.
    
    pub name: Option<String>,
    /// Output only. Size of the backup in bytes.
    #[serde(rename="sizeBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub size_bytes: Option<i64>,
    /// Output only. Name of the backup from which this backup was copied. If a backup is not created by copying a backup, this field will be empty. Values are of the form: projects//instances//backups/.
    #[serde(rename="sourceBackup")]
    
    pub source_backup: Option<String>,
    /// Required. Immutable. Name of the table from which this backup was created. This needs to be in the same instance as the backup. Values are of the form `projects/{project}/instances/{instance}/tables/{source_table}`.
    #[serde(rename="sourceTable")]
    
    pub source_table: Option<String>,
    /// Output only. `start_time` is the time that the backup was started (i.e. approximately the time the CreateBackup request is received). The row data in this backup will be no older than this timestamp.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The current state of the backup.
    
    pub state: Option<String>,
}

impl client::RequestValue for Backup {}
impl client::ResponseResult for Backup {}


/// Information about a backup.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BackupInfo {
    /// Output only. Name of the backup.
    
    pub backup: Option<String>,
    /// Output only. This time that the backup was finished. Row data in the backup will be no newer than this timestamp.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Name of the backup from which this backup was copied. If a backup is not created by copying a backup, this field will be empty. Values are of the form: projects//instances//backups/.
    #[serde(rename="sourceBackup")]
    
    pub source_backup: Option<String>,
    /// Output only. Name of the table the backup was created from.
    #[serde(rename="sourceTable")]
    
    pub source_table: Option<String>,
    /// Output only. The time that the backup was started. Row data in the backup will be no older than this timestamp.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for BackupInfo {}


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


/// Request message for google.bigtable.admin.v2.BigtableTableAdmin.CheckConsistency
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances tables check consistency projects](ProjectInstanceTableCheckConsistencyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CheckConsistencyRequest {
    /// Required. The token created using GenerateConsistencyToken for the Table.
    #[serde(rename="consistencyToken")]
    
    pub consistency_token: Option<String>,
}

impl client::RequestValue for CheckConsistencyRequest {}


/// Response message for google.bigtable.admin.v2.BigtableTableAdmin.CheckConsistency
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances tables check consistency projects](ProjectInstanceTableCheckConsistencyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CheckConsistencyResponse {
    /// True only if the token is consistent. A token is consistent if replication has caught up with the restrictions specified in the request.
    
    pub consistent: Option<bool>,
}

impl client::ResponseResult for CheckConsistencyResponse {}


/// A resizable group of nodes in a particular cloud location, capable of serving all Tables in the parent Instance.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances clusters create projects](ProjectInstanceClusterCreateCall) (request)
/// * [instances clusters get projects](ProjectInstanceClusterGetCall) (response)
/// * [instances clusters partial update cluster projects](ProjectInstanceClusterPartialUpdateClusterCall) (request)
/// * [instances clusters update projects](ProjectInstanceClusterUpdateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Cluster {
    /// Configuration for this cluster.
    #[serde(rename="clusterConfig")]
    
    pub cluster_config: Option<ClusterConfig>,
    /// Immutable. The type of storage used by this cluster to serve its parent instance's tables, unless explicitly overridden.
    #[serde(rename="defaultStorageType")]
    
    pub default_storage_type: Option<String>,
    /// Immutable. The encryption configuration for CMEK-protected clusters.
    #[serde(rename="encryptionConfig")]
    
    pub encryption_config: Option<EncryptionConfig>,
    /// Immutable. The location where this cluster's nodes and storage reside. For best performance, clients should be located as close as possible to this cluster. Currently only zones are supported, so values should be of the form `projects/{project}/locations/{zone}`.
    
    pub location: Option<String>,
    /// The unique name of the cluster. Values are of the form `projects/{project}/instances/{instance}/clusters/a-z*`.
    
    pub name: Option<String>,
    /// The number of nodes allocated to this cluster. More nodes enable higher throughput and more consistent performance.
    #[serde(rename="serveNodes")]
    
    pub serve_nodes: Option<i32>,
    /// Output only. The current state of the cluster.
    
    pub state: Option<String>,
}

impl client::RequestValue for Cluster {}
impl client::ResponseResult for Cluster {}


/// Autoscaling config for a cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClusterAutoscalingConfig {
    /// Required. Autoscaling limits for this cluster.
    #[serde(rename="autoscalingLimits")]
    
    pub autoscaling_limits: Option<AutoscalingLimits>,
    /// Required. Autoscaling targets for this cluster.
    #[serde(rename="autoscalingTargets")]
    
    pub autoscaling_targets: Option<AutoscalingTargets>,
}

impl client::Part for ClusterAutoscalingConfig {}


/// Configuration for a cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClusterConfig {
    /// Autoscaling configuration for this cluster.
    #[serde(rename="clusterAutoscalingConfig")]
    
    pub cluster_autoscaling_config: Option<ClusterAutoscalingConfig>,
}

impl client::Part for ClusterConfig {}


/// The state of a table's data in a particular cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClusterState {
    /// Output only. The encryption information for the table in this cluster. If the encryption key protecting this resource is customer managed, then its version can be rotated in Cloud Key Management Service (Cloud KMS). The primary version of the key and its status will be reflected here when changes propagate from Cloud KMS.
    #[serde(rename="encryptionInfo")]
    
    pub encryption_info: Option<Vec<EncryptionInfo>>,
    /// Output only. The state of replication for the table in this cluster.
    #[serde(rename="replicationState")]
    
    pub replication_state: Option<String>,
}

impl client::Part for ClusterState {}


/// A set of columns within a table which share a common configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ColumnFamily {
    /// Garbage collection rule specified as a protobuf. Must serialize to at most 500 bytes. NOTE: Garbage collection executes opportunistically in the background, and so it's possible for reads to return a cell even if it matches the active GC expression for its family.
    #[serde(rename="gcRule")]
    
    pub gc_rule: Option<GcRule>,
    /// Only available with STATS_VIEW, this includes summary statistics about column family contents. For statistics over an entire table, see TableStats above.
    
    pub stats: Option<ColumnFamilyStats>,
}

impl client::Part for ColumnFamily {}


/// Approximate statistics related to a single column family within a table. This information may change rapidly, interpreting these values at a point in time may already preset out-of-date information. Everything below is approximate, unless otherwise specified.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ColumnFamilyStats {
    /// How many cells are present per column qualifier in this column family, averaged over all rows containing any column in the column family. e.g. For column family "family" in a table with 3 rows: * A row with 3 cells in "family:col" and 1 cell in "other:col" (3 cells / 1 column in "family") * A row with 1 cell in "family:col", 7 cells in "family:other_col", and 7 cells in "other:data" (8 cells / 2 columns in "family") * A row with 3 cells in "other:col" (0 columns in "family", "family" not present) would report (3 + 8 + 0)/(1 + 2 + 0) = 3.66 in this field.
    #[serde(rename="averageCellsPerColumn")]
    
    pub average_cells_per_column: Option<f64>,
    /// How many column qualifiers are present in this column family, averaged over all rows in the table. e.g. For column family "family" in a table with 3 rows: * A row with cells in "family:col" and "other:col" (1 column in "family") * A row with cells in "family:col", "family:other_col", and "other:data" (2 columns in "family") * A row with cells in "other:col" (0 columns in "family", "family" not present) would report (1 + 2 + 0)/3 = 1.5 in this field.
    #[serde(rename="averageColumnsPerRow")]
    
    pub average_columns_per_row: Option<f64>,
    /// How much space the data in the column family occupies. This is roughly how many bytes would be needed to read the contents of the entire column family (e.g. by streaming all contents out).
    #[serde(rename="logicalDataBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub logical_data_bytes: Option<i64>,
}

impl client::Part for ColumnFamilyStats {}


/// The request for CopyBackup.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances clusters backups copy projects](ProjectInstanceClusterBackupCopyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CopyBackupRequest {
    /// Required. The id of the new backup. The `backup_id` along with `parent` are combined as {parent}/backups/{backup_id} to create the full backup name, of the form: `projects/{project}/instances/{instance}/clusters/{cluster}/backups/{backup_id}`. This string must be between 1 and 50 characters in length and match the regex _a-zA-Z0-9*.
    #[serde(rename="backupId")]
    
    pub backup_id: Option<String>,
    /// Required. Required. The expiration time of the copied backup with microsecond granularity that must be at least 6 hours and at most 30 days from the time the request is received. Once the `expire_time` has passed, Cloud Bigtable will delete the backup and free the resources used by the backup.
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. The source backup to be copied from. The source backup needs to be in READY state for it to be copied. Copying a copied backup is not allowed. Once CopyBackup is in progress, the source backup cannot be deleted or cleaned up on expiration until CopyBackup is finished. Values are of the form: `projects//instances//clusters//backups/`.
    #[serde(rename="sourceBackup")]
    
    pub source_backup: Option<String>,
}

impl client::RequestValue for CopyBackupRequest {}


/// Request message for BigtableInstanceAdmin.CreateInstance.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances create projects](ProjectInstanceCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateInstanceRequest {
    /// Required. The clusters to be created within the instance, mapped by desired cluster ID, e.g., just `mycluster` rather than `projects/myproject/instances/myinstance/clusters/mycluster`. Fields marked `OutputOnly` must be left blank.
    
    pub clusters: Option<HashMap<String, Cluster>>,
    /// Required. The instance to create. Fields marked `OutputOnly` must be left blank.
    
    pub instance: Option<Instance>,
    /// Required. The ID to be used when referring to the new instance within its project, e.g., just `myinstance` rather than `projects/myproject/instances/myinstance`.
    #[serde(rename="instanceId")]
    
    pub instance_id: Option<String>,
    /// Required. The unique name of the project in which to create the new instance. Values are of the form `projects/{project}`.
    
    pub parent: Option<String>,
}

impl client::RequestValue for CreateInstanceRequest {}


/// Request message for google.bigtable.admin.v2.BigtableTableAdmin.CreateTable
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances tables create projects](ProjectInstanceTableCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateTableRequest {
    /// The optional list of row keys that will be used to initially split the table into several tablets (tablets are similar to HBase regions). Given two split keys, `s1` and `s2`, three tablets will be created, spanning the key ranges: `[, s1), [s1, s2), [s2, )`. Example: * Row keys := `["a", "apple", "custom", "customer_1", "customer_2",` `"other", "zz"]` * initial_split_keys := `["apple", "customer_1", "customer_2", "other"]` * Key assignment: - Tablet 1 `[, apple) => {"a"}.` - Tablet 2 `[apple, customer_1) => {"apple", "custom"}.` - Tablet 3 `[customer_1, customer_2) => {"customer_1"}.` - Tablet 4 `[customer_2, other) => {"customer_2"}.` - Tablet 5 `[other, ) => {"other", "zz"}.`
    #[serde(rename="initialSplits")]
    
    pub initial_splits: Option<Vec<Split>>,
    /// Required. The Table to create.
    
    pub table: Option<Table>,
    /// Required. The name by which the new table should be referred to within the parent instance, e.g., `foobar` rather than `{parent}/tables/foobar`. Maximum 50 characters.
    #[serde(rename="tableId")]
    
    pub table_id: Option<String>,
}

impl client::RequestValue for CreateTableRequest {}


/// Request message for google.bigtable.admin.v2.BigtableTableAdmin.DropRowRange
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances tables drop row range projects](ProjectInstanceTableDropRowRangeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DropRowRangeRequest {
    /// Delete all rows in the table. Setting this to false is a no-op.
    #[serde(rename="deleteAllDataFromTable")]
    
    pub delete_all_data_from_table: Option<bool>,
    /// Delete all rows that start with this row key prefix. Prefix cannot be zero length.
    #[serde(rename="rowKeyPrefix")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub row_key_prefix: Option<Vec<u8>>,
}

impl client::RequestValue for DropRowRangeRequest {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [cancel operations](OperationCancelCall) (response)
/// * [delete operations](OperationDeleteCall) (response)
/// * [instances app profiles delete projects](ProjectInstanceAppProfileDeleteCall) (response)
/// * [instances clusters backups delete projects](ProjectInstanceClusterBackupDeleteCall) (response)
/// * [instances clusters delete projects](ProjectInstanceClusterDeleteCall) (response)
/// * [instances tables delete projects](ProjectInstanceTableDeleteCall) (response)
/// * [instances tables drop row range projects](ProjectInstanceTableDropRowRangeCall) (response)
/// * [instances delete projects](ProjectInstanceDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// Cloud Key Management Service (Cloud KMS) settings for a CMEK-protected cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EncryptionConfig {
    /// Describes the Cloud KMS encryption key that will be used to protect the destination Bigtable cluster. The requirements for this key are: 1) The Cloud Bigtable service account associated with the project that contains this cluster must be granted the `cloudkms.cryptoKeyEncrypterDecrypter` role on the CMEK key. 2) Only regional keys can be used and the region of the CMEK key must match the region of the cluster. Values are of the form `projects/{project}/locations/{location}/keyRings/{keyring}/cryptoKeys/{key}`
    #[serde(rename="kmsKeyName")]
    
    pub kms_key_name: Option<String>,
}

impl client::Part for EncryptionConfig {}


/// Encryption information for a given resource. If this resource is protected with customer managed encryption, the in-use Cloud Key Management Service (Cloud KMS) key version is specified along with its status.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EncryptionInfo {
    /// Output only. The status of encrypt/decrypt calls on underlying data for this resource. Regardless of status, the existing data is always encrypted at rest.
    #[serde(rename="encryptionStatus")]
    
    pub encryption_status: Option<Status>,
    /// Output only. The type of encryption used to protect this resource.
    #[serde(rename="encryptionType")]
    
    pub encryption_type: Option<String>,
    /// Output only. The version of the Cloud KMS key specified in the parent cluster that is in use for the data underlying this table.
    #[serde(rename="kmsKeyVersion")]
    
    pub kms_key_version: Option<String>,
}

impl client::Part for EncryptionInfo {}


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


/// Rule for determining which cells to delete during garbage collection.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GcRule {
    /// Delete cells that would be deleted by every nested rule.
    
    pub intersection: Option<Intersection>,
    /// Delete cells in a column older than the given age. Values must be at least one millisecond, and will be truncated to microsecond granularity.
    #[serde(rename="maxAge")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub max_age: Option<client::chrono::Duration>,
    /// Delete all cells in a column except the most recent N.
    #[serde(rename="maxNumVersions")]
    
    pub max_num_versions: Option<i32>,
    /// Delete cells that would be deleted by any nested rule.
    
    pub union: Option<Union>,
}

impl client::Part for GcRule {}


/// Request message for google.bigtable.admin.v2.BigtableTableAdmin.GenerateConsistencyToken
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances tables generate consistency token projects](ProjectInstanceTableGenerateConsistencyTokenCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GenerateConsistencyTokenRequest { _never_set: Option<bool> }

impl client::RequestValue for GenerateConsistencyTokenRequest {}


/// Response message for google.bigtable.admin.v2.BigtableTableAdmin.GenerateConsistencyToken
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances tables generate consistency token projects](ProjectInstanceTableGenerateConsistencyTokenCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GenerateConsistencyTokenResponse {
    /// The generated consistency token.
    #[serde(rename="consistencyToken")]
    
    pub consistency_token: Option<String>,
}

impl client::ResponseResult for GenerateConsistencyTokenResponse {}


/// Request message for `GetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances clusters backups get iam policy projects](ProjectInstanceClusterBackupGetIamPolicyCall) (request)
/// * [instances tables get iam policy projects](ProjectInstanceTableGetIamPolicyCall) (request)
/// * [instances get iam policy projects](ProjectInstanceGetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetIamPolicyRequest {
    /// OPTIONAL: A `GetPolicyOptions` object for specifying options to `GetIamPolicy`.
    
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
    /// Optional. The maximum policy version that will be used to format the policy. Valid values are 0, 1, and 3. Requests specifying an invalid value will be rejected. Requests for policies with any conditional role bindings must specify version 3. Policies with no conditional role bindings may specify any valid value or leave the field unset. The policy in the response might use the policy version that you specified, or it might use a lower policy version. For example, if you specify version 3, but the policy has no conditional role bindings, the response uses version 1. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    #[serde(rename="requestedPolicyVersion")]
    
    pub requested_policy_version: Option<i32>,
}

impl client::Part for GetPolicyOptions {}


/// A tablet is a defined by a start and end key and is explained in https://cloud.google.com/bigtable/docs/overview#architecture and https://cloud.google.com/bigtable/docs/performance#optimization. A Hot tablet is a tablet that exhibits high average cpu usage during the time interval from start time to end time.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HotTablet {
    /// Tablet End Key (inclusive).
    #[serde(rename="endKey")]
    
    pub end_key: Option<String>,
    /// Output only. The end time of the hot tablet.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The unique name of the hot tablet. Values are of the form `projects/{project}/instances/{instance}/clusters/{cluster}/hotTablets/[a-zA-Z0-9_-]*`.
    
    pub name: Option<String>,
    /// Output only. The average CPU usage spent by a node on this tablet over the start_time to end_time time range. The percentage is the amount of CPU used by the node to serve the tablet, from 0% (tablet was not interacted with) to 100% (the node spent all cycles serving the hot tablet).
    #[serde(rename="nodeCpuUsagePercent")]
    
    pub node_cpu_usage_percent: Option<f32>,
    /// Tablet Start Key (inclusive).
    #[serde(rename="startKey")]
    
    pub start_key: Option<String>,
    /// Output only. The start time of the hot tablet.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Name of the table that contains the tablet. Values are of the form `projects/{project}/instances/{instance}/tables/_a-zA-Z0-9*`.
    #[serde(rename="tableName")]
    
    pub table_name: Option<String>,
}

impl client::Part for HotTablet {}


/// A collection of Bigtable Tables and the resources that serve them. All tables in an instance are served from all Clusters in the instance.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances get projects](ProjectInstanceGetCall) (response)
/// * [instances partial update instance projects](ProjectInstancePartialUpdateInstanceCall) (request)
/// * [instances update projects](ProjectInstanceUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Instance {
    /// Output only. A commit timestamp representing when this Instance was created. For instances created before this field was added (August 2021), this value is `seconds: 0, nanos: 1`.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. The descriptive name for this instance as it appears in UIs. Can be changed at any time, but should be kept globally unique to avoid confusion.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Labels are a flexible and lightweight mechanism for organizing cloud resources into groups that reflect a customer's organizational needs and deployment strategies. They can be used to filter resources and aggregate metrics. * Label keys must be between 1 and 63 characters long and must conform to the regular expression: `\p{Ll}\p{Lo}{0,62}`. * Label values must be between 0 and 63 characters long and must conform to the regular expression: `[\p{Ll}\p{Lo}\p{N}_-]{0,63}`. * No more than 64 labels can be associated with a given resource. * Keys and values must both be under 128 bytes.
    
    pub labels: Option<HashMap<String, String>>,
    /// The unique name of the instance. Values are of the form `projects/{project}/instances/a-z+[a-z0-9]`.
    
    pub name: Option<String>,
    /// Output only. Reserved for future use.
    #[serde(rename="satisfiesPzs")]
    
    pub satisfies_pzs: Option<bool>,
    /// Output only. The current state of the instance.
    
    pub state: Option<String>,
    /// The type of the instance. Defaults to `PRODUCTION`.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::RequestValue for Instance {}
impl client::ResponseResult for Instance {}


/// A GcRule which deletes cells matching all of the given rules.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Intersection {
    /// Only delete cells which would be deleted by every element of `rules`.
    
    pub rules: Option<Vec<GcRule>>,
}

impl client::Part for Intersection {}


/// Response message for BigtableInstanceAdmin.ListAppProfiles.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances app profiles list projects](ProjectInstanceAppProfileListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAppProfilesResponse {
    /// The list of requested app profiles.
    #[serde(rename="appProfiles")]
    
    pub app_profiles: Option<Vec<AppProfile>>,
    /// Locations from which AppProfile information could not be retrieved, due to an outage or some other transient condition. AppProfiles from these locations may be missing from `app_profiles`. Values are of the form `projects//locations/`
    #[serde(rename="failedLocations")]
    
    pub failed_locations: Option<Vec<String>>,
    /// Set if not all app profiles could be returned in a single response. Pass this value to `page_token` in another request to get the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListAppProfilesResponse {}


/// The response for ListBackups.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances clusters backups list projects](ProjectInstanceClusterBackupListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListBackupsResponse {
    /// The list of matching backups.
    
    pub backups: Option<Vec<Backup>>,
    /// `next_page_token` can be sent in a subsequent ListBackups call to fetch more of the matching backups.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListBackupsResponse {}


/// Response message for BigtableInstanceAdmin.ListClusters.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances clusters list projects](ProjectInstanceClusterListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListClustersResponse {
    /// The list of requested clusters.
    
    pub clusters: Option<Vec<Cluster>>,
    /// Locations from which Cluster information could not be retrieved, due to an outage or some other transient condition. Clusters from these locations may be missing from `clusters`, or may only have partial information returned. Values are of the form `projects//locations/`
    #[serde(rename="failedLocations")]
    
    pub failed_locations: Option<Vec<String>>,
    /// DEPRECATED: This field is unused and ignored.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListClustersResponse {}


/// Response message for BigtableInstanceAdmin.ListHotTablets.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances clusters hot tablets list projects](ProjectInstanceClusterHotTabletListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListHotTabletsResponse {
    /// List of hot tablets in the tables of the requested cluster that fall within the requested time range. Hot tablets are ordered by node cpu usage percent. If there are multiple hot tablets that correspond to the same tablet within a 15-minute interval, only the hot tablet with the highest node cpu usage will be included in the response.
    #[serde(rename="hotTablets")]
    
    pub hot_tablets: Option<Vec<HotTablet>>,
    /// Set if not all hot tablets could be returned in a single response. Pass this value to `page_token` in another request to get the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListHotTabletsResponse {}


/// Response message for BigtableInstanceAdmin.ListInstances.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances list projects](ProjectInstanceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListInstancesResponse {
    /// Locations from which Instance information could not be retrieved, due to an outage or some other transient condition. Instances whose Clusters are all in one of the failed locations may be missing from `instances`, and Instances with at least one Cluster in a failed location may only have partial information returned. Values are of the form `projects//locations/`
    #[serde(rename="failedLocations")]
    
    pub failed_locations: Option<Vec<String>>,
    /// The list of requested instances.
    
    pub instances: Option<Vec<Instance>>,
    /// DEPRECATED: This field is unused and ignored.
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
/// * [projects operations list operations](OperationProjectOperationListCall) (response)
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


/// Response message for google.bigtable.admin.v2.BigtableTableAdmin.ListTables
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances tables list projects](ProjectInstanceTableListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTablesResponse {
    /// Set if not all tables could be returned in a single response. Pass this value to `page_token` in another request to get the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The tables present in the requested instance.
    
    pub tables: Option<Vec<Table>>,
}

impl client::ResponseResult for ListTablesResponse {}


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


/// A create, update, or delete of a particular column family.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Modification {
    /// Create a new column family with the specified schema, or fail if one already exists with the given ID.
    
    pub create: Option<ColumnFamily>,
    /// Drop (delete) the column family with the given ID, or fail if no such family exists.
    
    pub drop: Option<bool>,
    /// The ID of the column family to be modified.
    
    pub id: Option<String>,
    /// Update an existing column family to the specified schema, or fail if no column family exists with the given ID.
    
    pub update: Option<ColumnFamily>,
}

impl client::Part for Modification {}


/// Request message for google.bigtable.admin.v2.BigtableTableAdmin.ModifyColumnFamilies
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances tables modify column families projects](ProjectInstanceTableModifyColumnFamilyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ModifyColumnFamiliesRequest {
    /// Required. Modifications to be atomically applied to the specified table's families. Entries are applied in order, meaning that earlier modifications can be masked by later ones (in the case of repeated updates to the same family, for example).
    
    pub modifications: Option<Vec<Modification>>,
}

impl client::RequestValue for ModifyColumnFamiliesRequest {}


/// Read/write requests are routed to the nearest cluster in the instance, and will fail over to the nearest cluster that is available in the event of transient errors or delays. Clusters in a region are considered equidistant. Choosing this option sacrifices read-your-writes consistency to improve availability.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MultiClusterRoutingUseAny {
    /// The set of clusters to route to. The order is ignored; clusters will be tried in order of distance. If left empty, all clusters are eligible.
    #[serde(rename="clusterIds")]
    
    pub cluster_ids: Option<Vec<String>>,
}

impl client::Part for MultiClusterRoutingUseAny {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [projects operations list operations](OperationProjectOperationListCall) (none)
/// * [cancel operations](OperationCancelCall) (none)
/// * [delete operations](OperationDeleteCall) (none)
/// * [get operations](OperationGetCall) (response)
/// * [instances app profiles patch projects](ProjectInstanceAppProfilePatchCall) (response)
/// * [instances clusters backups copy projects](ProjectInstanceClusterBackupCopyCall) (response)
/// * [instances clusters backups create projects](ProjectInstanceClusterBackupCreateCall) (response)
/// * [instances clusters create projects](ProjectInstanceClusterCreateCall) (response)
/// * [instances clusters partial update cluster projects](ProjectInstanceClusterPartialUpdateClusterCall) (response)
/// * [instances clusters update projects](ProjectInstanceClusterUpdateCall) (response)
/// * [instances tables patch projects](ProjectInstanceTablePatchCall) (response)
/// * [instances tables restore projects](ProjectInstanceTableRestoreCall) (response)
/// * [instances tables undelete projects](ProjectInstanceTableUndeleteCall) (response)
/// * [instances create projects](ProjectInstanceCreateCall) (response)
/// * [instances partial update instance projects](ProjectInstancePartialUpdateInstanceCall) (response)
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

impl client::Resource for Operation {}
impl client::ResponseResult for Operation {}


/// An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members`, or principals, to a single `role`. Principals can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { “bindings”: \[ { “role”: “roles/resourcemanager.organizationAdmin”, “members”: \[ “user:mike@example.com”, “group:admins@example.com”, “domain:google.com”, “serviceAccount:my-project-id@appspot.gserviceaccount.com” \] }, { “role”: “roles/resourcemanager.organizationViewer”, “members”: \[ “user:eve@example.com” \], “condition”: { “title”: “expirable access”, “description”: “Does not grant access after Sep 2020”, “expression”: “request.time \< timestamp(‘2020-10-01T00:00:00.000Z’)”, } } \], “etag”: “BwWWja0YfJA=”, “version”: 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time \< timestamp(‘2020-10-01T00:00:00.000Z’) etag: BwWWja0YfJA= version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances clusters backups get iam policy projects](ProjectInstanceClusterBackupGetIamPolicyCall) (response)
/// * [instances clusters backups set iam policy projects](ProjectInstanceClusterBackupSetIamPolicyCall) (response)
/// * [instances tables get iam policy projects](ProjectInstanceTableGetIamPolicyCall) (response)
/// * [instances tables set iam policy projects](ProjectInstanceTableSetIamPolicyCall) (response)
/// * [instances get iam policy projects](ProjectInstanceGetIamPolicyCall) (response)
/// * [instances set iam policy projects](ProjectInstanceSetIamPolicyCall) (response)
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


/// Information about a table restore.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RestoreInfo {
    /// Information about the backup used to restore the table. The backup may no longer exist.
    #[serde(rename="backupInfo")]
    
    pub backup_info: Option<BackupInfo>,
    /// The type of the restore source.
    #[serde(rename="sourceType")]
    
    pub source_type: Option<String>,
}

impl client::Part for RestoreInfo {}


/// The request for RestoreTable.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances tables restore projects](ProjectInstanceTableRestoreCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RestoreTableRequest {
    /// Name of the backup from which to restore. Values are of the form `projects//instances//clusters//backups/`.
    
    pub backup: Option<String>,
    /// Required. The id of the table to create and restore to. This table must not already exist. The `table_id` appended to `parent` forms the full table name of the form `projects//instances//tables/`.
    #[serde(rename="tableId")]
    
    pub table_id: Option<String>,
}

impl client::RequestValue for RestoreTableRequest {}


/// Request message for `SetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances clusters backups set iam policy projects](ProjectInstanceClusterBackupSetIamPolicyCall) (request)
/// * [instances tables set iam policy projects](ProjectInstanceTableSetIamPolicyCall) (request)
/// * [instances set iam policy projects](ProjectInstanceSetIamPolicyCall) (request)
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


/// Unconditionally routes all read/write requests to a specific cluster. This option preserves read-your-writes consistency but does not improve availability.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SingleClusterRouting {
    /// Whether or not `CheckAndMutateRow` and `ReadModifyWriteRow` requests are allowed by this app profile. It is unsafe to send these requests to the same table/row/column in multiple clusters.
    #[serde(rename="allowTransactionalWrites")]
    
    pub allow_transactional_writes: Option<bool>,
    /// The cluster to which read/write requests should be routed.
    #[serde(rename="clusterId")]
    
    pub cluster_id: Option<String>,
}

impl client::Part for SingleClusterRouting {}


/// An initial split point for a newly created table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Split {
    /// Row key to use as an initial tablet boundary.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub key: Option<Vec<u8>>,
}

impl client::Part for Split {}


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


/// A collection of user data indexed by row, column, and timestamp. Each table is served using the resources of its parent cluster.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances tables create projects](ProjectInstanceTableCreateCall) (response)
/// * [instances tables get projects](ProjectInstanceTableGetCall) (response)
/// * [instances tables modify column families projects](ProjectInstanceTableModifyColumnFamilyCall) (response)
/// * [instances tables patch projects](ProjectInstanceTablePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Table {
    /// Output only. Map from cluster ID to per-cluster table state. If it could not be determined whether or not the table has data in a particular cluster (for example, if its zone is unavailable), then there will be an entry for the cluster with UNKNOWN `replication_status`. Views: `REPLICATION_VIEW`, `ENCRYPTION_VIEW`, `FULL`
    #[serde(rename="clusterStates")]
    
    pub cluster_states: Option<HashMap<String, ClusterState>>,
    /// The column families configured for this table, mapped by column family ID. Views: `SCHEMA_VIEW`, `STATS_VIEW`, `FULL`
    #[serde(rename="columnFamilies")]
    
    pub column_families: Option<HashMap<String, ColumnFamily>>,
    /// Set to true to make the table protected against data loss. i.e. deleting the following resources through Admin APIs are prohibited: * The table. * The column families in the table. * The instance containing the table. Note one can still delete the data stored in the table through Data APIs.
    #[serde(rename="deletionProtection")]
    
    pub deletion_protection: Option<bool>,
    /// Immutable. The granularity (i.e. `MILLIS`) at which timestamps are stored in this table. Timestamps not matching the granularity will be rejected. If unspecified at creation time, the value will be set to `MILLIS`. Views: `SCHEMA_VIEW`, `FULL`.
    
    pub granularity: Option<String>,
    /// The unique name of the table. Values are of the form `projects/{project}/instances/{instance}/tables/_a-zA-Z0-9*`. Views: `NAME_ONLY`, `SCHEMA_VIEW`, `REPLICATION_VIEW`, `STATS_VIEW`, `FULL`
    
    pub name: Option<String>,
    /// Output only. If this table was restored from another data source (e.g. a backup), this field will be populated with information about the restore.
    #[serde(rename="restoreInfo")]
    
    pub restore_info: Option<RestoreInfo>,
    /// Only available with STATS_VIEW, this includes summary statistics about the entire table contents. For statistics about a specific column family, see ColumnFamilyStats in the mapped ColumnFamily collection above.
    
    pub stats: Option<TableStats>,
}

impl client::RequestValue for Table {}
impl client::ResponseResult for Table {}


/// Approximate statistics related to a table. These statistics are calculated infrequently, while simultaneously, data in the table can change rapidly. Thus the values reported here (e.g. row count) are very likely out-of date, even the instant they are received in this API. Thus, only treat these values as approximate. IMPORTANT: Everything below is approximate, unless otherwise specified.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableStats {
    /// How many cells are present per column (column family, column qualifier) combinations, averaged over all columns in all rows in the table. e.g. A table with 2 rows: * A row with 3 cells in "family:col" and 1 cell in "other:col" (4 cells / 2 columns) * A row with 1 cell in "family:col", 7 cells in "family:other_col", and 7 cells in "other:data" (15 cells / 3 columns) would report (4 + 15)/(2 + 3) = 3.8 in this field.
    #[serde(rename="averageCellsPerColumn")]
    
    pub average_cells_per_column: Option<f64>,
    /// How many (column family, column qualifier) combinations are present per row in the table, averaged over all rows in the table. e.g. A table with 2 rows: * A row with cells in "family:col" and "other:col" (2 distinct columns) * A row with cells in "family:col", "family:other_col", and "other:data" (3 distinct columns) would report (2 + 3)/2 = 2.5 in this field.
    #[serde(rename="averageColumnsPerRow")]
    
    pub average_columns_per_row: Option<f64>,
    /// This is roughly how many bytes would be needed to read the entire table (e.g. by streaming all contents out).
    #[serde(rename="logicalDataBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub logical_data_bytes: Option<i64>,
    /// How many rows are in the table.
    #[serde(rename="rowCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub row_count: Option<i64>,
}

impl client::Part for TableStats {}


/// Request message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances clusters backups test iam permissions projects](ProjectInstanceClusterBackupTestIamPermissionCall) (request)
/// * [instances tables test iam permissions projects](ProjectInstanceTableTestIamPermissionCall) (request)
/// * [instances test iam permissions projects](ProjectInstanceTestIamPermissionCall) (request)
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
/// * [instances clusters backups test iam permissions projects](ProjectInstanceClusterBackupTestIamPermissionCall) (response)
/// * [instances tables test iam permissions projects](ProjectInstanceTableTestIamPermissionCall) (response)
/// * [instances test iam permissions projects](ProjectInstanceTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsResponse {
    /// A subset of `TestPermissionsRequest.permissions` that the caller is allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for TestIamPermissionsResponse {}


/// Request message for google.bigtable.admin.v2.BigtableTableAdmin.UndeleteTable
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances tables undelete projects](ProjectInstanceTableUndeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UndeleteTableRequest { _never_set: Option<bool> }

impl client::RequestValue for UndeleteTableRequest {}


/// A GcRule which deletes cells matching any of the given rules.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Union {
    /// Delete cells which would be deleted by any element of `rules`.
    
    pub rules: Option<Vec<GcRule>>,
}

impl client::Part for Union {}


