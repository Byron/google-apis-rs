use super::*;
/// An entry for an Access Control list.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AclEntry {
    /// The time when this access control entry expires in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`.
    #[serde(rename="expirationTime")]
    
    pub expiration_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// This is always `sql#aclEntry`.
    
    pub kind: Option<String>,
    /// Optional. A label to identify this entry.
    
    pub name: Option<String>,
    /// The allowlisted value for the access control list.
    
    pub value: Option<String>,
}

impl client::Part for AclEntry {}


/// An Admin API warning message.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApiWarning {
    /// Code to uniquely identify the warning type.
    
    pub code: Option<ApiWarningCodeEnum>,
    /// The warning message.
    
    pub message: Option<String>,
    /// The region name for REGION_UNREACHABLE warning.
    
    pub region: Option<String>,
}

impl client::Part for ApiWarning {}


/// Database instance backup configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BackupConfiguration {
    /// Backup retention settings.
    #[serde(rename="backupRetentionSettings")]
    
    pub backup_retention_settings: Option<BackupRetentionSettings>,
    /// (MySQL only) Whether binary log is enabled. If backup configuration is disabled, binarylog must be disabled as well.
    #[serde(rename="binaryLogEnabled")]
    
    pub binary_log_enabled: Option<bool>,
    /// Whether this configuration is enabled.
    
    pub enabled: Option<bool>,
    /// This is always `sql#backupConfiguration`.
    
    pub kind: Option<String>,
    /// Location of the backup
    
    pub location: Option<String>,
    /// (Postgres only) Whether point in time recovery is enabled.
    #[serde(rename="pointInTimeRecoveryEnabled")]
    
    pub point_in_time_recovery_enabled: Option<bool>,
    /// Reserved for future use.
    #[serde(rename="replicationLogArchivingEnabled")]
    
    pub replication_log_archiving_enabled: Option<bool>,
    /// Start time for the daily backup configuration in UTC timezone in the 24 hour format - `HH:MM`.
    #[serde(rename="startTime")]
    
    pub start_time: Option<String>,
    /// The number of days of transaction logs we retain for point in time restore, from 1-7.
    #[serde(rename="transactionLogRetentionDays")]
    
    pub transaction_log_retention_days: Option<i32>,
}

impl client::Part for BackupConfiguration {}


/// Backup context.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BackupContext {
    /// The identifier of the backup.
    #[serde(rename="backupId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub backup_id: Option<i64>,
    /// This is always `sql#backupContext`.
    
    pub kind: Option<String>,
}

impl client::Part for BackupContext {}


/// We currently only support backup retention by specifying the number of backups we will retain.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BackupRetentionSettings {
    /// Depending on the value of retention_unit, this is used to determine if a backup needs to be deleted. If retention_unit is 'COUNT', we will retain this many backups.
    #[serde(rename="retainedBackups")]
    
    pub retained_backups: Option<i32>,
    /// The unit that 'retained_backups' represents.
    #[serde(rename="retentionUnit")]
    
    pub retention_unit: Option<BackupRetentionSettingRetentionUnitEnum>,
}

impl client::Part for BackupRetentionSettings {}


/// A BackupRun resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete backup runs](BackupRunDeleteCall) (none)
/// * [get backup runs](BackupRunGetCall) (response)
/// * [insert backup runs](BackupRunInsertCall) (request)
/// * [list backup runs](BackupRunListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BackupRun {
    /// Specifies the kind of backup, PHYSICAL or DEFAULT_SNAPSHOT.
    #[serde(rename="backupKind")]
    
    pub backup_kind: Option<BackupRunBackupKindEnum>,
    /// The description of this run, only applicable to on-demand backups.
    
    pub description: Option<String>,
    /// Encryption configuration specific to a backup.
    #[serde(rename="diskEncryptionConfiguration")]
    
    pub disk_encryption_configuration: Option<DiskEncryptionConfiguration>,
    /// Encryption status specific to a backup.
    #[serde(rename="diskEncryptionStatus")]
    
    pub disk_encryption_status: Option<DiskEncryptionStatus>,
    /// The time the backup operation completed in UTC timezone in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The time the run was enqueued in UTC timezone in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`.
    #[serde(rename="enqueuedTime")]
    
    pub enqueued_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Information about why the backup operation failed. This is only present if the run has the FAILED status.
    
    pub error: Option<OperationError>,
    /// The identifier for this backup run. Unique only for a specific Cloud SQL instance.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Name of the database instance.
    
    pub instance: Option<String>,
    /// This is always `sql#backupRun`.
    
    pub kind: Option<String>,
    /// Location of the backups.
    
    pub location: Option<String>,
    /// The URI of this resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// The time the backup operation actually started in UTC timezone in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The status of this run.
    
    pub status: Option<BackupRunStatusEnum>,
    /// Backup time zone to prevent restores to an instance with a different time zone. Now relevant only for SQL Server.
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<String>,
    /// The type of this run; can be either "AUTOMATED" or "ON_DEMAND" or "FINAL". This field defaults to "ON_DEMAND" and is ignored, when specified for insert requests.
    #[serde(rename="type")]
    
    pub type_: Option<BackupRunTypeEnum>,
    /// The start time of the backup window during which this the backup was attempted in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`.
    #[serde(rename="windowStartTime")]
    
    pub window_start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for BackupRun {}
impl client::Resource for BackupRun {}
impl client::ResponseResult for BackupRun {}


/// Backup run list results.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list backup runs](BackupRunListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BackupRunsListResponse {
    /// A list of backup runs in reverse chronological order of the enqueued time.
    
    pub items: Option<Vec<BackupRun>>,
    /// This is always `sql#backupRunsList`.
    
    pub kind: Option<String>,
    /// The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for BackupRunsListResponse {}


/// Binary log coordinates.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BinLogCoordinates {
    /// Name of the binary log file for a Cloud SQL instance.
    #[serde(rename="binLogFileName")]
    
    pub bin_log_file_name: Option<String>,
    /// Position (offset) within the binary log file.
    #[serde(rename="binLogPosition")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bin_log_position: Option<i64>,
    /// This is always `sql#binLogCoordinates`.
    
    pub kind: Option<String>,
}

impl client::Part for BinLogCoordinates {}


/// Database instance clone context.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloneContext {
    /// The name of the allocated ip range for the private ip Cloud SQL instance. For example: “google-managed-services-default”. If set, the cloned instance ip will be created in the allocated range. The range name must comply with [RFC 1035](https://tools.ietf.org/html/rfc1035). Specifically, the name must be 1-63 characters long and match the regular expression [a-z]([-a-z0-9]*[a-z0-9])?. Reserved for future use.
    #[serde(rename="allocatedIpRange")]
    
    pub allocated_ip_range: Option<String>,
    /// Binary log coordinates, if specified, identify the position up to which the source instance is cloned. If not specified, the source instance is cloned up to the most recent binary log coordinates.
    #[serde(rename="binLogCoordinates")]
    
    pub bin_log_coordinates: Option<BinLogCoordinates>,
    /// (SQL Server only) Clone only the specified databases from the source instance. Clone all databases if empty.
    #[serde(rename="databaseNames")]
    
    pub database_names: Option<Vec<String>>,
    /// Name of the Cloud SQL instance to be created as a clone.
    #[serde(rename="destinationInstanceName")]
    
    pub destination_instance_name: Option<String>,
    /// This is always `sql#cloneContext`.
    
    pub kind: Option<String>,
    /// Reserved for future use.
    #[serde(rename="pitrTimestampMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub pitr_timestamp_ms: Option<i64>,
    /// Timestamp, if specified, identifies the time to which the source instance is cloned.
    #[serde(rename="pointInTime")]
    
    pub point_in_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for CloneContext {}


/// Connect settings retrieval response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get connect](ConnectGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConnectSettings {
    /// `SECOND_GEN`: Cloud SQL database instance. `EXTERNAL`: A database server that is not managed by Google. This property is read-only; use the `tier` property in the `settings` object to determine the database type.
    #[serde(rename="backendType")]
    
    pub backend_type: Option<ConnectSettingBackendTypeEnum>,
    /// The database engine type and version. The `databaseVersion` field cannot be changed after instance creation. MySQL instances: `MYSQL_8_0`, `MYSQL_5_7` (default), or `MYSQL_5_6`. PostgreSQL instances: `POSTGRES_9_6`, `POSTGRES_10`, `POSTGRES_11`, `POSTGRES_12` (default), `POSTGRES_13`, or `POSTGRES_14`. SQL Server instances: `SQLSERVER_2017_STANDARD` (default), `SQLSERVER_2017_ENTERPRISE`, `SQLSERVER_2017_EXPRESS`, `SQLSERVER_2017_WEB`, `SQLSERVER_2019_STANDARD`, `SQLSERVER_2019_ENTERPRISE`, `SQLSERVER_2019_EXPRESS`, or `SQLSERVER_2019_WEB`.
    #[serde(rename="databaseVersion")]
    
    pub database_version: Option<ConnectSettingDatabaseVersionEnum>,
    /// The assigned IP addresses for the instance.
    #[serde(rename="ipAddresses")]
    
    pub ip_addresses: Option<Vec<IpMapping>>,
    /// This is always `sql#connectSettings`.
    
    pub kind: Option<String>,
    /// The cloud region for the instance. For example, `us-central1`, `europe-west1`. The region cannot be changed after instance creation.
    
    pub region: Option<String>,
    /// SSL configuration.
    #[serde(rename="serverCaCert")]
    
    pub server_ca_cert: Option<SslCert>,
}

impl client::ResponseResult for ConnectSettings {}


/// Represents a SQL database on the Cloud SQL instance.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete databases](DatabaseDeleteCall) (none)
/// * [get databases](DatabaseGetCall) (response)
/// * [insert databases](DatabaseInsertCall) (request)
/// * [list databases](DatabaseListCall) (none)
/// * [patch databases](DatabasePatchCall) (request)
/// * [update databases](DatabaseUpdateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Database {
    /// The Cloud SQL charset value.
    
    pub charset: Option<String>,
    /// The Cloud SQL collation value.
    
    pub collation: Option<String>,
    /// This field is deprecated and will be removed from a future version of the API.
    
    pub etag: Option<String>,
    /// The name of the Cloud SQL instance. This does not include the project ID.
    
    pub instance: Option<String>,
    /// This is always `sql#database`.
    
    pub kind: Option<String>,
    /// The name of the database in the Cloud SQL instance. This does not include the project ID or instance name.
    
    pub name: Option<String>,
    /// The project ID of the project containing the Cloud SQL database. The Google apps domain is prefixed if applicable.
    
    pub project: Option<String>,
    /// The URI of this resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// no description provided
    #[serde(rename="sqlserverDatabaseDetails")]
    
    pub sqlserver_database_details: Option<SqlServerDatabaseDetails>,
}

impl client::RequestValue for Database {}
impl client::Resource for Database {}
impl client::ResponseResult for Database {}


/// Database flags for Cloud SQL instances.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatabaseFlags {
    /// The name of the flag. These flags are passed at instance startup, so include both server options and system variables. Flags are specified with underscores, not hyphens. For more information, see [Configuring Database Flags](https://cloud.google.com/sql/docs/mysql/flags) in the Cloud SQL documentation.
    
    pub name: Option<String>,
    /// The value of the flag. Boolean flags are set to `on` for true and `off` for false. This field must be omitted if the flag doesn't take a value.
    
    pub value: Option<String>,
}

impl client::Part for DatabaseFlags {}


/// A Cloud SQL instance resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get instances](InstanceGetCall) (response)
/// * [insert instances](InstanceInsertCall) (request)
/// * [patch instances](InstancePatchCall) (request)
/// * [update instances](InstanceUpdateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatabaseInstance {
    /// List all maintenance versions applicable on the instance
    #[serde(rename="availableMaintenanceVersions")]
    
    pub available_maintenance_versions: Option<Vec<String>>,
    /// The backend type. `SECOND_GEN`: Cloud SQL database instance. `EXTERNAL`: A database server that is not managed by Google. This property is read-only; use the `tier` property in the `settings` object to determine the database type.
    #[serde(rename="backendType")]
    
    pub backend_type: Option<DatabaseInstanceBackendTypeEnum>,
    /// Connection name of the Cloud SQL instance used in connection strings.
    #[serde(rename="connectionName")]
    
    pub connection_name: Option<String>,
    /// Output only. The time when the instance was created in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The current disk usage of the instance in bytes. This property has been deprecated. Use the "cloudsql.googleapis.com/database/disk/bytes_used" metric in Cloud Monitoring API instead. Please see [this announcement](https://groups.google.com/d/msg/google-cloud-sql-announce/I_7-F9EBhT0/BtvFtdFeAgAJ) for details.
    #[serde(rename="currentDiskSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub current_disk_size: Option<i64>,
    /// Output only. Stores the current database version running on the instance including minor version such as `MYSQL_8_0_18`.
    #[serde(rename="databaseInstalledVersion")]
    
    pub database_installed_version: Option<String>,
    /// The database engine type and version. The `databaseVersion` field cannot be changed after instance creation.
    #[serde(rename="databaseVersion")]
    
    pub database_version: Option<DatabaseInstanceDatabaseVersionEnum>,
    /// Disk encryption configuration specific to an instance.
    #[serde(rename="diskEncryptionConfiguration")]
    
    pub disk_encryption_configuration: Option<DiskEncryptionConfiguration>,
    /// Disk encryption status specific to an instance.
    #[serde(rename="diskEncryptionStatus")]
    
    pub disk_encryption_status: Option<DiskEncryptionStatus>,
    /// This field is deprecated and will be removed from a future version of the API. Use the `settings.settingsVersion` field instead.
    
    pub etag: Option<String>,
    /// The name and status of the failover replica.
    #[serde(rename="failoverReplica")]
    
    pub failover_replica: Option<DatabaseInstanceFailoverReplica>,
    /// The Compute Engine zone that the instance is currently serving from. This value could be different from the zone that was specified when the instance was created if the instance has failed over to its secondary zone. WARNING: Changing this might restart the instance.
    #[serde(rename="gceZone")]
    
    pub gce_zone: Option<String>,
    /// The instance type.
    #[serde(rename="instanceType")]
    
    pub instance_type: Option<DatabaseInstanceInstanceTypeEnum>,
    /// The assigned IP addresses for the instance.
    #[serde(rename="ipAddresses")]
    
    pub ip_addresses: Option<Vec<IpMapping>>,
    /// The IPv6 address assigned to the instance. (Deprecated) This property was applicable only to First Generation instances.
    #[serde(rename="ipv6Address")]
    
    pub ipv6_address: Option<String>,
    /// This is always `sql#instance`.
    
    pub kind: Option<String>,
    /// The current software version on the instance.
    #[serde(rename="maintenanceVersion")]
    
    pub maintenance_version: Option<String>,
    /// The name of the instance which will act as primary in the replication setup.
    #[serde(rename="masterInstanceName")]
    
    pub master_instance_name: Option<String>,
    /// The maximum disk size of the instance in bytes.
    #[serde(rename="maxDiskSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_disk_size: Option<i64>,
    /// Name of the Cloud SQL instance. This does not include the project ID.
    
    pub name: Option<String>,
    /// Configuration specific to on-premises instances.
    #[serde(rename="onPremisesConfiguration")]
    
    pub on_premises_configuration: Option<OnPremisesConfiguration>,
    /// This field represents the report generated by the proactive database wellness job for OutOfDisk issues. * Writers: * the proactive database wellness job for OOD. * Readers: * the proactive database wellness job
    #[serde(rename="outOfDiskReport")]
    
    pub out_of_disk_report: Option<SqlOutOfDiskReport>,
    /// The project ID of the project containing the Cloud SQL instance. The Google apps domain is prefixed if applicable.
    
    pub project: Option<String>,
    /// The geographical region. Can be: * `us-central` (`FIRST_GEN` instances only) * `us-central1` (`SECOND_GEN` instances only) * `asia-east1` or `europe-west1`. Defaults to `us-central` or `us-central1` depending on the instance type. The region cannot be changed after instance creation.
    
    pub region: Option<String>,
    /// Configuration specific to failover replicas and read replicas.
    #[serde(rename="replicaConfiguration")]
    
    pub replica_configuration: Option<ReplicaConfiguration>,
    /// The replicas of the instance.
    #[serde(rename="replicaNames")]
    
    pub replica_names: Option<Vec<String>>,
    /// Initial root password. Use only on creation. You must set root passwords before you can connect to PostgreSQL instances.
    #[serde(rename="rootPassword")]
    
    pub root_password: Option<String>,
    /// The status indicating if instance satisfiesPzs. Reserved for future use.
    #[serde(rename="satisfiesPzs")]
    
    pub satisfies_pzs: Option<bool>,
    /// The start time of any upcoming scheduled maintenance for this instance.
    #[serde(rename="scheduledMaintenance")]
    
    pub scheduled_maintenance: Option<SqlScheduledMaintenance>,
    /// The Compute Engine zone that the failover instance is currently serving from for a regional instance. This value could be different from the zone that was specified when the instance was created if the instance has failed over to its secondary/failover zone.
    #[serde(rename="secondaryGceZone")]
    
    pub secondary_gce_zone: Option<String>,
    /// The URI of this resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// SSL configuration.
    #[serde(rename="serverCaCert")]
    
    pub server_ca_cert: Option<SslCert>,
    /// The service account email address assigned to the instance.\This property is read-only.
    #[serde(rename="serviceAccountEmailAddress")]
    
    pub service_account_email_address: Option<String>,
    /// The user settings.
    
    pub settings: Option<Settings>,
    /// The current serving state of the Cloud SQL instance.
    
    pub state: Option<DatabaseInstanceStateEnum>,
    /// If the instance state is SUSPENDED, the reason for the suspension.
    #[serde(rename="suspensionReason")]
    
    pub suspension_reason: Option<Vec<DatabaseInstanceSuspensionReasonEnum>>,
}

impl client::RequestValue for DatabaseInstance {}
impl client::ResponseResult for DatabaseInstance {}


/// Database list response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list databases](DatabaseListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatabasesListResponse {
    /// List of database resources in the instance.
    
    pub items: Option<Vec<Database>>,
    /// This is always `sql#databasesList`.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for DatabasesListResponse {}


/// Read-replica configuration for connecting to the on-premises primary instance.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DemoteMasterConfiguration {
    /// This is always `sql#demoteMasterConfiguration`.
    
    pub kind: Option<String>,
    /// MySQL specific configuration when replicating from a MySQL on-premises primary instance. Replication configuration information such as the username, password, certificates, and keys are not stored in the instance metadata. The configuration information is used only to set up the replication connection and is stored by MySQL in a file named `master.info` in the data directory.
    #[serde(rename="mysqlReplicaConfiguration")]
    
    pub mysql_replica_configuration: Option<DemoteMasterMySqlReplicaConfiguration>,
}

impl client::Part for DemoteMasterConfiguration {}


/// Database instance demote primary instance context.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DemoteMasterContext {
    /// This is always `sql#demoteMasterContext`.
    
    pub kind: Option<String>,
    /// The name of the instance which will act as on-premises primary instance in the replication setup.
    #[serde(rename="masterInstanceName")]
    
    pub master_instance_name: Option<String>,
    /// Configuration specific to read-replicas replicating from the on-premises primary instance.
    #[serde(rename="replicaConfiguration")]
    
    pub replica_configuration: Option<DemoteMasterConfiguration>,
    /// Flag to skip replication setup on the instance.
    #[serde(rename="skipReplicationSetup")]
    
    pub skip_replication_setup: Option<bool>,
    /// Verify the GTID consistency for demote operation. Default value: `True`. Setting this flag to `false` enables you to bypass the GTID consistency check between on-premises primary instance and Cloud SQL instance during the demotion operation but also exposes you to the risk of future replication failures. Change the value only if you know the reason for the GTID divergence and are confident that doing so will not cause any replication issues.
    #[serde(rename="verifyGtidConsistency")]
    
    pub verify_gtid_consistency: Option<bool>,
}

impl client::Part for DemoteMasterContext {}


/// Read-replica configuration specific to MySQL databases.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DemoteMasterMySqlReplicaConfiguration {
    /// PEM representation of the trusted CA's x509 certificate.
    #[serde(rename="caCertificate")]
    
    pub ca_certificate: Option<String>,
    /// PEM representation of the replica's x509 certificate.
    #[serde(rename="clientCertificate")]
    
    pub client_certificate: Option<String>,
    /// PEM representation of the replica's private key. The corresponsing public key is encoded in the client's certificate. The format of the replica's private key can be either PKCS #1 or PKCS #8.
    #[serde(rename="clientKey")]
    
    pub client_key: Option<String>,
    /// This is always `sql#demoteMasterMysqlReplicaConfiguration`.
    
    pub kind: Option<String>,
    /// The password for the replication connection.
    
    pub password: Option<String>,
    /// The username for the replication connection.
    
    pub username: Option<String>,
}

impl client::Part for DemoteMasterMySqlReplicaConfiguration {}


/// Deny maintenance Periods. This specifies a date range during when all CSA rollout will be denied.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DenyMaintenancePeriod {
    /// "deny maintenance period" end date. If the year of the end date is empty, the year of the start date also must be empty. In this case, it means the no maintenance interval recurs every year. The date is in format yyyy-mm-dd i.e., 2020-11-01, or mm-dd, i.e., 11-01
    #[serde(rename="endDate")]
    
    pub end_date: Option<String>,
    /// "deny maintenance period" start date. If the year of the start date is empty, the year of the end date also must be empty. In this case, it means the deny maintenance period recurs every year. The date is in format yyyy-mm-dd i.e., 2020-11-01, or mm-dd, i.e., 11-01
    #[serde(rename="startDate")]
    
    pub start_date: Option<String>,
    /// Time in UTC when the "deny maintenance period" starts on start_date and ends on end_date. The time is in format: HH:mm:SS, i.e., 00:00:00
    
    pub time: Option<String>,
}

impl client::Part for DenyMaintenancePeriod {}


/// Disk encryption configuration for an instance.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DiskEncryptionConfiguration {
    /// This is always `sql#diskEncryptionConfiguration`.
    
    pub kind: Option<String>,
    /// Resource name of KMS key for disk encryption
    #[serde(rename="kmsKeyName")]
    
    pub kms_key_name: Option<String>,
}

impl client::Part for DiskEncryptionConfiguration {}


/// Disk encryption status for an instance.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DiskEncryptionStatus {
    /// This is always `sql#diskEncryptionStatus`.
    
    pub kind: Option<String>,
    /// KMS key version used to encrypt the Cloud SQL instance resource
    #[serde(rename="kmsKeyVersionName")]
    
    pub kms_key_version_name: Option<String>,
}

impl client::Part for DiskEncryptionStatus {}


/// Database instance export context.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExportContext {
    /// Options for exporting BAK files (SQL Server-only)
    #[serde(rename="bakExportOptions")]
    
    pub bak_export_options: Option<ExportContextBakExportOptions>,
    /// Options for exporting data as CSV. `MySQL` and `PostgreSQL` instances only.
    #[serde(rename="csvExportOptions")]
    
    pub csv_export_options: Option<ExportContextCsvExportOptions>,
    /// Databases to be exported. `MySQL instances:` If `fileType` is `SQL` and no database is specified, all databases are exported, except for the `mysql` system database. If `fileType` is `CSV`, you can specify one database, either by using this property or by using the `csvExportOptions.selectQuery` property, which takes precedence over this property. `PostgreSQL instances:` You must specify one database to be exported. If `fileType` is `CSV`, this database must match the one specified in the `csvExportOptions.selectQuery` property. `SQL Server instances:` You must specify one database to be exported, and the `fileType` must be `BAK`.
    
    pub databases: Option<Vec<String>>,
    /// The file type for the specified uri.
    #[serde(rename="fileType")]
    
    pub file_type: Option<ExportContextFileTypeEnum>,
    /// This is always `sql#exportContext`.
    
    pub kind: Option<String>,
    /// Option for export offload.
    
    pub offload: Option<bool>,
    /// Options for exporting data as SQL statements.
    #[serde(rename="sqlExportOptions")]
    
    pub sql_export_options: Option<ExportContextSqlExportOptions>,
    /// The path to the file in Google Cloud Storage where the export will be stored. The URI is in the form `gs://bucketName/fileName`. If the file already exists, the request succeeds, but the operation fails. If `fileType` is `SQL` and the filename ends with .gz, the contents are compressed.
    
    pub uri: Option<String>,
}

impl client::Part for ExportContext {}


/// Database instance failover context.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FailoverContext {
    /// This is always `sql#failoverContext`.
    
    pub kind: Option<String>,
    /// The current settings version of this instance. Request will be rejected if this version doesn't match the current settings version.
    #[serde(rename="settingsVersion")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub settings_version: Option<i64>,
}

impl client::Part for FailoverContext {}


/// A flag resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list flags](FlagListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Flag {
    /// Use this field if only certain integers are accepted. Can be combined with min_value and max_value to add additional values.
    #[serde(rename="allowedIntValues")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub allowed_int_values: Option<Vec<i64>>,
    /// For `STRING` flags, a list of strings that the value can be set to.
    #[serde(rename="allowedStringValues")]
    
    pub allowed_string_values: Option<Vec<String>>,
    /// The database version this flag applies to. Can be MySQL instances: `MYSQL_8_0`, `MYSQL_8_0_18`, `MYSQL_8_0_26`, `MYSQL_5_7`, or `MYSQL_5_6`. PostgreSQL instances: `POSTGRES_9_6`, `POSTGRES_10`, `POSTGRES_11` or `POSTGRES_12`. SQL Server instances: `SQLSERVER_2017_STANDARD`, `SQLSERVER_2017_ENTERPRISE`, `SQLSERVER_2017_EXPRESS`, `SQLSERVER_2017_WEB`, `SQLSERVER_2019_STANDARD`, `SQLSERVER_2019_ENTERPRISE`, `SQLSERVER_2019_EXPRESS`, or `SQLSERVER_2019_WEB`. See [the complete list](https://developers.google.com/sql/docs/mysql/admin-api/rest/v1/SqlDatabaseVersion).
    #[serde(rename="appliesTo")]
    
    pub applies_to: Option<Vec<FlagAppliesToEnum>>,
    /// Whether or not the flag is considered in beta.
    #[serde(rename="inBeta")]
    
    pub in_beta: Option<bool>,
    /// This is always `sql#flag`.
    
    pub kind: Option<String>,
    /// For `INTEGER` flags, the maximum allowed value.
    #[serde(rename="maxValue")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_value: Option<i64>,
    /// For `INTEGER` flags, the minimum allowed value.
    #[serde(rename="minValue")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub min_value: Option<i64>,
    /// This is the name of the flag. Flag names always use underscores, not hyphens, for example: `max_allowed_packet`
    
    pub name: Option<String>,
    /// Indicates whether changing this flag will trigger a database restart. Only applicable to Second Generation instances.
    #[serde(rename="requiresRestart")]
    
    pub requires_restart: Option<bool>,
    /// The type of the flag. Flags are typed to being `BOOLEAN`, `STRING`, `INTEGER` or `NONE`. `NONE` is used for flags that do not take a value, such as `skip_grant_tables`.
    #[serde(rename="type")]
    
    pub type_: Option<FlagTypeEnum>,
}

impl client::Resource for Flag {}


/// Flags list response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list flags](FlagListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FlagsListResponse {
    /// List of flags.
    
    pub items: Option<Vec<Flag>>,
    /// This is always `sql#flagsList`.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for FlagsListResponse {}


/// Ephemeral certificate creation request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [generate ephemeral connect](ConnectGenerateEphemeralCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GenerateEphemeralCertRequest {
    /// Optional. Access token to include in the signed certificate.
    
    pub access_token: Option<String>,
    /// PEM encoded public key to include in the signed certificate.
    
    pub public_key: Option<String>,
    /// Optional. Optional snapshot read timestamp to trade freshness for performance.
    #[serde(rename="readTime")]
    
    pub read_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. If set, it will contain the cert valid duration.
    #[serde(rename="validDuration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub valid_duration: Option<client::chrono::Duration>,
}

impl client::RequestValue for GenerateEphemeralCertRequest {}


/// Ephemeral certificate creation request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [generate ephemeral connect](ConnectGenerateEphemeralCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GenerateEphemeralCertResponse {
    /// Generated cert
    #[serde(rename="ephemeralCert")]
    
    pub ephemeral_cert: Option<SslCert>,
}

impl client::ResponseResult for GenerateEphemeralCertResponse {}


/// Database instance import context.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImportContext {
    /// Import parameters specific to SQL Server .BAK files
    #[serde(rename="bakImportOptions")]
    
    pub bak_import_options: Option<ImportContextBakImportOptions>,
    /// Options for importing data as CSV.
    #[serde(rename="csvImportOptions")]
    
    pub csv_import_options: Option<ImportContextCsvImportOptions>,
    /// The target database for the import. If `fileType` is `SQL`, this field is required only if the import file does not specify a database, and is overridden by any database specification in the import file. If `fileType` is `CSV`, one database must be specified.
    
    pub database: Option<String>,
    /// The file type for the specified uri.\`SQL`: The file contains SQL statements. \`CSV`: The file contains CSV data.
    #[serde(rename="fileType")]
    
    pub file_type: Option<ImportContextFileTypeEnum>,
    /// The PostgreSQL user for this import operation. PostgreSQL instances only.
    #[serde(rename="importUser")]
    
    pub import_user: Option<String>,
    /// This is always `sql#importContext`.
    
    pub kind: Option<String>,
    /// Path to the import file in Cloud Storage, in the form `gs://bucketName/fileName`. Compressed gzip files (.gz) are supported when `fileType` is `SQL`. The instance must have write permissions to the bucket and read access to the file.
    
    pub uri: Option<String>,
}

impl client::Part for ImportContext {}


/// Insights configuration. This specifies when Cloud SQL Insights feature is enabled and optional configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InsightsConfig {
    /// Whether Query Insights feature is enabled.
    #[serde(rename="queryInsightsEnabled")]
    
    pub query_insights_enabled: Option<bool>,
    /// Number of query execution plans captured by Insights per minute for all queries combined. Default is 5.
    #[serde(rename="queryPlansPerMinute")]
    
    pub query_plans_per_minute: Option<i32>,
    /// Maximum query length stored in bytes. Default value: 1024 bytes. Range: 256-4500 bytes. Query length more than this field value will be truncated to this value. When unset, query length will be the default value. Changing query length will restart the database.
    #[serde(rename="queryStringLength")]
    
    pub query_string_length: Option<i32>,
    /// Whether Query Insights will record application tags from query when enabled.
    #[serde(rename="recordApplicationTags")]
    
    pub record_application_tags: Option<bool>,
    /// Whether Query Insights will record client address when enabled.
    #[serde(rename="recordClientAddress")]
    
    pub record_client_address: Option<bool>,
}

impl client::Part for InsightsConfig {}


/// Reference to another Cloud SQL instance.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstanceReference {
    /// The name of the Cloud SQL instance being referenced. This does not include the project ID.
    
    pub name: Option<String>,
    /// The project ID of the Cloud SQL instance being referenced. The default is the same project ID as the instance references it.
    
    pub project: Option<String>,
    /// The region of the Cloud SQL instance being referenced.
    
    pub region: Option<String>,
}

impl client::Part for InstanceReference {}


/// Database instance clone request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [clone instances](InstanceCloneCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstancesCloneRequest {
    /// Contains details about the clone operation.
    #[serde(rename="cloneContext")]
    
    pub clone_context: Option<CloneContext>,
}

impl client::RequestValue for InstancesCloneRequest {}


/// Database demote primary instance request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [demote master instances](InstanceDemoteMasterCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstancesDemoteMasterRequest {
    /// Contains details about the demoteMaster operation.
    #[serde(rename="demoteMasterContext")]
    
    pub demote_master_context: Option<DemoteMasterContext>,
}

impl client::RequestValue for InstancesDemoteMasterRequest {}


/// Database instance export request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [export instances](InstanceExportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstancesExportRequest {
    /// Contains details about the export operation.
    #[serde(rename="exportContext")]
    
    pub export_context: Option<ExportContext>,
}

impl client::RequestValue for InstancesExportRequest {}


/// Instance failover request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [failover instances](InstanceFailoverCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstancesFailoverRequest {
    /// Failover Context.
    #[serde(rename="failoverContext")]
    
    pub failover_context: Option<FailoverContext>,
}

impl client::RequestValue for InstancesFailoverRequest {}


/// Database instance import request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [import instances](InstanceImportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstancesImportRequest {
    /// Contains details about the import operation.
    #[serde(rename="importContext")]
    
    pub import_context: Option<ImportContext>,
}

impl client::RequestValue for InstancesImportRequest {}


/// Database instances list response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list instances](InstanceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstancesListResponse {
    /// List of database instance resources.
    
    pub items: Option<Vec<DatabaseInstance>>,
    /// This is always `sql#instancesList`.
    
    pub kind: Option<String>,
    /// The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of warnings that occurred while handling the request.
    
    pub warnings: Option<Vec<ApiWarning>>,
}

impl client::ResponseResult for InstancesListResponse {}


/// Instances ListServerCas response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list server cas instances](InstanceListServerCaCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstancesListServerCasResponse {
    /// no description provided
    #[serde(rename="activeVersion")]
    
    pub active_version: Option<String>,
    /// List of server CA certificates for the instance.
    
    pub certs: Option<Vec<SslCert>>,
    /// This is always `sql#instancesListServerCas`.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for InstancesListServerCasResponse {}


/// Database instance restore backup request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [restore backup instances](InstanceRestoreBackupCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstancesRestoreBackupRequest {
    /// Parameters required to perform the restore backup operation.
    #[serde(rename="restoreBackupContext")]
    
    pub restore_backup_context: Option<RestoreBackupContext>,
}

impl client::RequestValue for InstancesRestoreBackupRequest {}


/// Rotate server CA request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [rotate server ca instances](InstanceRotateServerCaCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstancesRotateServerCaRequest {
    /// Contains details about the rotate server CA operation.
    #[serde(rename="rotateServerCaContext")]
    
    pub rotate_server_ca_context: Option<RotateServerCaContext>,
}

impl client::RequestValue for InstancesRotateServerCaRequest {}


/// Instance truncate log request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [truncate log instances](InstanceTruncateLogCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstancesTruncateLogRequest {
    /// Contains details about the truncate log operation.
    #[serde(rename="truncateLogContext")]
    
    pub truncate_log_context: Option<TruncateLogContext>,
}

impl client::RequestValue for InstancesTruncateLogRequest {}


/// IP Management configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IpConfiguration {
    /// The name of the allocated ip range for the private ip Cloud SQL instance. For example: “google-managed-services-default”. If set, the instance ip will be created in the allocated range. The range name must comply with [RFC 1035](https://tools.ietf.org/html/rfc1035). Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?.`
    #[serde(rename="allocatedIpRange")]
    
    pub allocated_ip_range: Option<String>,
    /// The list of external networks that are allowed to connect to the instance using the IP. In 'CIDR' notation, also known as 'slash' notation (for example: `157.197.200.0/24`).
    #[serde(rename="authorizedNetworks")]
    
    pub authorized_networks: Option<Vec<AclEntry>>,
    /// Controls connectivity to private IP instances from Google services, such as BigQuery.
    #[serde(rename="enablePrivatePathForGoogleCloudServices")]
    
    pub enable_private_path_for_google_cloud_services: Option<bool>,
    /// Whether the instance is assigned a public IP address or not.
    #[serde(rename="ipv4Enabled")]
    
    pub ipv4_enabled: Option<bool>,
    /// The resource link for the VPC network from which the Cloud SQL instance is accessible for private IP. For example, `/projects/myProject/global/networks/default`. This setting can be updated, but it cannot be removed after it is set.
    #[serde(rename="privateNetwork")]
    
    pub private_network: Option<String>,
    /// Whether SSL connections over IP are enforced or not.
    #[serde(rename="requireSsl")]
    
    pub require_ssl: Option<bool>,
}

impl client::Part for IpConfiguration {}


/// Database instance IP Mapping.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IpMapping {
    /// The IP address assigned.
    #[serde(rename="ipAddress")]
    
    pub ip_address: Option<String>,
    /// The due time for this IP to be retired in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`. This field is only available when the IP is scheduled to be retired.
    #[serde(rename="timeToRetire")]
    
    pub time_to_retire: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The type of this IP address. A `PRIMARY` address is a public address that can accept incoming connections. A `PRIVATE` address is a private address that can accept incoming connections. An `OUTGOING` address is the source address of connections originating from the instance, if supported.
    #[serde(rename="type")]
    
    pub type_: Option<IpMappingTypeEnum>,
}

impl client::Part for IpMapping {}


/// Preferred location. This specifies where a Cloud SQL instance is located. Note that if the preferred location is not available, the instance will be located as close as possible within the region. Only one location may be specified.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocationPreference {
    /// The App Engine application to follow, it must be in the same region as the Cloud SQL instance. WARNING: Changing this might restart the instance.
    #[serde(rename="followGaeApplication")]
    
    pub follow_gae_application: Option<String>,
    /// This is always `sql#locationPreference`.
    
    pub kind: Option<String>,
    /// The preferred Compute Engine zone for the secondary/failover (for example: us-central1-a, us-central1-b, etc.).
    #[serde(rename="secondaryZone")]
    
    pub secondary_zone: Option<String>,
    /// The preferred Compute Engine zone (for example: us-central1-a, us-central1-b, etc.). WARNING: Changing this might restart the instance.
    
    pub zone: Option<String>,
}

impl client::Part for LocationPreference {}


/// Maintenance window. This specifies when a Cloud SQL instance is restarted for system maintenance purposes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MaintenanceWindow {
    /// day of week (1-7), starting on Monday.
    
    pub day: Option<i32>,
    /// hour of day - 0 to 23.
    
    pub hour: Option<i32>,
    /// This is always `sql#maintenanceWindow`.
    
    pub kind: Option<String>,
    /// Maintenance timing setting: `canary` (Earlier) or `stable` (Later). [Learn more](https://cloud.google.com/sql/docs/mysql/instance-settings#maintenance-timing-2ndgen).
    #[serde(rename="updateTrack")]
    
    pub update_track: Option<MaintenanceWindowUpdateTrackEnum>,
}

impl client::Part for MaintenanceWindow {}


/// Read-replica configuration specific to MySQL databases.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MySqlReplicaConfiguration {
    /// PEM representation of the trusted CA's x509 certificate.
    #[serde(rename="caCertificate")]
    
    pub ca_certificate: Option<String>,
    /// PEM representation of the replica's x509 certificate.
    #[serde(rename="clientCertificate")]
    
    pub client_certificate: Option<String>,
    /// PEM representation of the replica's private key. The corresponsing public key is encoded in the client's certificate.
    #[serde(rename="clientKey")]
    
    pub client_key: Option<String>,
    /// Seconds to wait between connect retries. MySQL's default is 60 seconds.
    #[serde(rename="connectRetryInterval")]
    
    pub connect_retry_interval: Option<i32>,
    /// Path to a SQL dump file in Google Cloud Storage from which the replica instance is to be created. The URI is in the form gs://bucketName/fileName. Compressed gzip files (.gz) are also supported. Dumps have the binlog co-ordinates from which replication begins. This can be accomplished by setting --master-data to 1 when using mysqldump.
    #[serde(rename="dumpFilePath")]
    
    pub dump_file_path: Option<String>,
    /// This is always `sql#mysqlReplicaConfiguration`.
    
    pub kind: Option<String>,
    /// Interval in milliseconds between replication heartbeats.
    #[serde(rename="masterHeartbeatPeriod")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub master_heartbeat_period: Option<i64>,
    /// The password for the replication connection.
    
    pub password: Option<String>,
    /// A list of permissible ciphers to use for SSL encryption.
    #[serde(rename="sslCipher")]
    
    pub ssl_cipher: Option<String>,
    /// The username for the replication connection.
    
    pub username: Option<String>,
    /// Whether or not to check the primary instance's Common Name value in the certificate that it sends during the SSL handshake.
    #[serde(rename="verifyServerCertificate")]
    
    pub verify_server_certificate: Option<bool>,
}

impl client::Part for MySqlReplicaConfiguration {}


/// MySQL-specific external server sync settings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MySqlSyncConfig {
    /// Flags to use for the initial dump.
    #[serde(rename="initialSyncFlags")]
    
    pub initial_sync_flags: Option<Vec<SyncFlags>>,
}

impl client::Part for MySqlSyncConfig {}


/// On-premises instance configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OnPremisesConfiguration {
    /// PEM representation of the trusted CA's x509 certificate.
    #[serde(rename="caCertificate")]
    
    pub ca_certificate: Option<String>,
    /// PEM representation of the replica's x509 certificate.
    #[serde(rename="clientCertificate")]
    
    pub client_certificate: Option<String>,
    /// PEM representation of the replica's private key. The corresponsing public key is encoded in the client's certificate.
    #[serde(rename="clientKey")]
    
    pub client_key: Option<String>,
    /// The dump file to create the Cloud SQL replica.
    #[serde(rename="dumpFilePath")]
    
    pub dump_file_path: Option<String>,
    /// The host and port of the on-premises instance in host:port format
    #[serde(rename="hostPort")]
    
    pub host_port: Option<String>,
    /// This is always `sql#onPremisesConfiguration`.
    
    pub kind: Option<String>,
    /// The password for connecting to on-premises instance.
    
    pub password: Option<String>,
    /// The reference to Cloud SQL instance if the source is Cloud SQL.
    #[serde(rename="sourceInstance")]
    
    pub source_instance: Option<InstanceReference>,
    /// The username for connecting to on-premises instance.
    
    pub username: Option<String>,
}

impl client::Part for OnPremisesConfiguration {}


/// An Operation resource. For successful operations that return an Operation resource, only the fields relevant to the operation are populated in the resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete backup runs](BackupRunDeleteCall) (response)
/// * [insert backup runs](BackupRunInsertCall) (response)
/// * [delete databases](DatabaseDeleteCall) (response)
/// * [insert databases](DatabaseInsertCall) (response)
/// * [patch databases](DatabasePatchCall) (response)
/// * [update databases](DatabaseUpdateCall) (response)
/// * [add server ca instances](InstanceAddServerCaCall) (response)
/// * [clone instances](InstanceCloneCall) (response)
/// * [delete instances](InstanceDeleteCall) (response)
/// * [demote master instances](InstanceDemoteMasterCall) (response)
/// * [export instances](InstanceExportCall) (response)
/// * [failover instances](InstanceFailoverCall) (response)
/// * [import instances](InstanceImportCall) (response)
/// * [insert instances](InstanceInsertCall) (response)
/// * [patch instances](InstancePatchCall) (response)
/// * [promote replica instances](InstancePromoteReplicaCall) (response)
/// * [reset ssl config instances](InstanceResetSslConfigCall) (response)
/// * [restart instances](InstanceRestartCall) (response)
/// * [restore backup instances](InstanceRestoreBackupCall) (response)
/// * [rotate server ca instances](InstanceRotateServerCaCall) (response)
/// * [start replica instances](InstanceStartReplicaCall) (response)
/// * [stop replica instances](InstanceStopReplicaCall) (response)
/// * [truncate log instances](InstanceTruncateLogCall) (response)
/// * [update instances](InstanceUpdateCall) (response)
/// * [get operations](OperationGetCall) (response)
/// * [list operations](OperationListCall) (none)
/// * [instances reschedule maintenance projects](ProjectInstanceRescheduleMaintenanceCall) (response)
/// * [instances start external sync projects](ProjectInstanceStartExternalSyncCall) (response)
/// * [delete ssl certs](SslCertDeleteCall) (response)
/// * [delete users](UserDeleteCall) (response)
/// * [insert users](UserInsertCall) (response)
/// * [update users](UserUpdateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// The context for backup operation, if applicable.
    #[serde(rename="backupContext")]
    
    pub backup_context: Option<BackupContext>,
    /// The time this operation finished in UTC timezone in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// If errors occurred during processing of this operation, this field will be populated.
    
    pub error: Option<OperationErrors>,
    /// The context for export operation, if applicable.
    #[serde(rename="exportContext")]
    
    pub export_context: Option<ExportContext>,
    /// The context for import operation, if applicable.
    #[serde(rename="importContext")]
    
    pub import_context: Option<ImportContext>,
    /// The time this operation was enqueued in UTC timezone in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`.
    #[serde(rename="insertTime")]
    
    pub insert_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// This is always `sql#operation`.
    
    pub kind: Option<String>,
    /// An identifier that uniquely identifies the operation. You can use this identifier to retrieve the Operations resource that has information about the operation.
    
    pub name: Option<String>,
    /// The type of the operation. Valid values are: * `CREATE` * `DELETE` * `UPDATE` * `RESTART` * `IMPORT` * `EXPORT` * `BACKUP_VOLUME` * `RESTORE_VOLUME` * `CREATE_USER` * `DELETE_USER` * `CREATE_DATABASE` * `DELETE_DATABASE`
    #[serde(rename="operationType")]
    
    pub operation_type: Option<OperationOperationTypeEnum>,
    /// The URI of this resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// The time this operation actually started in UTC timezone in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The status of an operation.
    
    pub status: Option<OperationStatusEnum>,
    /// Name of the database instance related to this operation.
    #[serde(rename="targetId")]
    
    pub target_id: Option<String>,
    /// no description provided
    #[serde(rename="targetLink")]
    
    pub target_link: Option<String>,
    /// The project ID of the target instance related to this operation.
    #[serde(rename="targetProject")]
    
    pub target_project: Option<String>,
    /// The email address of the user who initiated this operation.
    
    pub user: Option<String>,
}

impl client::Resource for Operation {}
impl client::ResponseResult for Operation {}


/// Database instance operation error.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperationError {
    /// Identifies the specific error that occurred.
    
    pub code: Option<String>,
    /// This is always `sql#operationError`.
    
    pub kind: Option<String>,
    /// Additional information about the error encountered.
    
    pub message: Option<String>,
}

impl client::Part for OperationError {}


/// Database instance operation errors list wrapper.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperationErrors {
    /// The list of errors encountered while processing this operation.
    
    pub errors: Option<Vec<OperationError>>,
    /// This is always `sql#operationErrors`.
    
    pub kind: Option<String>,
}

impl client::Part for OperationErrors {}


/// Operations list response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list operations](OperationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperationsListResponse {
    /// List of operation resources.
    
    pub items: Option<Vec<Operation>>,
    /// This is always `sql#operationsList`.
    
    pub kind: Option<String>,
    /// The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for OperationsListResponse {}


/// Read-only password status.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PasswordStatus {
    /// If true, user does not have login privileges.
    
    pub locked: Option<bool>,
    /// The expiration time of the current password.
    #[serde(rename="passwordExpirationTime")]
    
    pub password_expiration_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for PasswordStatus {}


/// Database instance local user password validation policy
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PasswordValidationPolicy {
    /// The complexity of the password.
    
    pub complexity: Option<PasswordValidationPolicyComplexityEnum>,
    /// Disallow username as a part of the password.
    #[serde(rename="disallowUsernameSubstring")]
    
    pub disallow_username_substring: Option<bool>,
    /// Whether the password policy is enabled or not.
    #[serde(rename="enablePasswordPolicy")]
    
    pub enable_password_policy: Option<bool>,
    /// Minimum number of characters allowed.
    #[serde(rename="minLength")]
    
    pub min_length: Option<i32>,
    /// Minimum interval after which the password can be changed. This flag is only supported for PostgreSQL.
    #[serde(rename="passwordChangeInterval")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub password_change_interval: Option<client::chrono::Duration>,
    /// Number of previous passwords that cannot be reused.
    #[serde(rename="reuseInterval")]
    
    pub reuse_interval: Option<i32>,
}

impl client::Part for PasswordValidationPolicy {}


/// Read-replica configuration for connecting to the primary instance.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReplicaConfiguration {
    /// Specifies if the replica is the failover target. If the field is set to `true`, the replica will be designated as a failover replica. In case the primary instance fails, the replica instance will be promoted as the new primary instance. Only one replica can be specified as failover target, and the replica has to be in different zone with the primary instance.
    #[serde(rename="failoverTarget")]
    
    pub failover_target: Option<bool>,
    /// This is always `sql#replicaConfiguration`.
    
    pub kind: Option<String>,
    /// MySQL specific configuration when replicating from a MySQL on-premises primary instance. Replication configuration information such as the username, password, certificates, and keys are not stored in the instance metadata. The configuration information is used only to set up the replication connection and is stored by MySQL in a file named `master.info` in the data directory.
    #[serde(rename="mysqlReplicaConfiguration")]
    
    pub mysql_replica_configuration: Option<MySqlReplicaConfiguration>,
}

impl client::Part for ReplicaConfiguration {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Reschedule {
    /// Required. The type of the reschedule.
    #[serde(rename="rescheduleType")]
    
    pub reschedule_type: Option<RescheduleRescheduleTypeEnum>,
    /// Optional. Timestamp when the maintenance shall be rescheduled to if reschedule_type=SPECIFIC_TIME, in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`.
    #[serde(rename="scheduleTime")]
    
    pub schedule_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for Reschedule {}


/// Database instance restore from backup context. Backup context contains source instance id and project id.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RestoreBackupContext {
    /// The ID of the backup run to restore from.
    #[serde(rename="backupRunId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub backup_run_id: Option<i64>,
    /// The ID of the instance that the backup was taken from.
    #[serde(rename="instanceId")]
    
    pub instance_id: Option<String>,
    /// This is always `sql#restoreBackupContext`.
    
    pub kind: Option<String>,
    /// The full project ID of the source instance.
    
    pub project: Option<String>,
}

impl client::Part for RestoreBackupContext {}


/// Instance rotate server CA context.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RotateServerCaContext {
    /// This is always `sql#rotateServerCaContext`.
    
    pub kind: Option<String>,
    /// The fingerprint of the next version to be rotated to. If left unspecified, will be rotated to the most recently added server CA version.
    #[serde(rename="nextVersion")]
    
    pub next_version: Option<String>,
}

impl client::Part for RotateServerCaContext {}


/// Database instance settings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    /// The activation policy specifies when the instance is activated; it is applicable only when the instance state is RUNNABLE. Valid values: * `ALWAYS`: The instance is on, and remains so even in the absence of connection requests. * `NEVER`: The instance is off; it is not activated, even if a connection request arrives.
    #[serde(rename="activationPolicy")]
    
    pub activation_policy: Option<SettingActivationPolicyEnum>,
    /// Active Directory configuration, relevant only for Cloud SQL for SQL Server.
    #[serde(rename="activeDirectoryConfig")]
    
    pub active_directory_config: Option<SqlActiveDirectoryConfig>,
    /// The App Engine app IDs that can access this instance. (Deprecated) Applied to First Generation instances only.
    #[serde(rename="authorizedGaeApplications")]
    
    pub authorized_gae_applications: Option<Vec<String>>,
    /// Availability type. Potential values: * `ZONAL`: The instance serves data from only one zone. Outages in that zone affect data accessibility. * `REGIONAL`: The instance can serve data from more than one zone in a region (it is highly available)./ For more information, see [Overview of the High Availability Configuration](https://cloud.google.com/sql/docs/mysql/high-availability).
    #[serde(rename="availabilityType")]
    
    pub availability_type: Option<SettingAvailabilityTypeEnum>,
    /// The daily backup configuration for the instance.
    #[serde(rename="backupConfiguration")]
    
    pub backup_configuration: Option<BackupConfiguration>,
    /// The name of server Instance collation.
    
    pub collation: Option<String>,
    /// Specifies if connections must use Cloud SQL connectors. Option values include the following: `NOT_REQUIRED` (Cloud SQL instances can be connected without Cloud SQL Connectors) and `REQUIRED` (Only allow connections that use Cloud SQL Connectors). Note that using REQUIRED disables all existing authorized networks. If this field is not specified when creating a new instance, NOT_REQUIRED is used. If this field is not specified when patching or updating an existing instance, it is left unchanged in the instance.
    #[serde(rename="connectorEnforcement")]
    
    pub connector_enforcement: Option<SettingConnectorEnforcementEnum>,
    /// Configuration specific to read replica instances. Indicates whether database flags for crash-safe replication are enabled. This property was only applicable to First Generation instances.
    #[serde(rename="crashSafeReplicationEnabled")]
    
    pub crash_safe_replication_enabled: Option<bool>,
    /// The size of data disk, in GB. The data disk size minimum is 10GB.
    #[serde(rename="dataDiskSizeGb")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub data_disk_size_gb: Option<i64>,
    /// The type of data disk: `PD_SSD` (default) or `PD_HDD`. Not used for First Generation instances.
    #[serde(rename="dataDiskType")]
    
    pub data_disk_type: Option<SettingDataDiskTypeEnum>,
    /// The database flags passed to the instance at startup.
    #[serde(rename="databaseFlags")]
    
    pub database_flags: Option<Vec<DatabaseFlags>>,
    /// Configuration specific to read replica instances. Indicates whether replication is enabled or not. WARNING: Changing this restarts the instance.
    #[serde(rename="databaseReplicationEnabled")]
    
    pub database_replication_enabled: Option<bool>,
    /// Configuration to protect against accidental instance deletion.
    #[serde(rename="deletionProtectionEnabled")]
    
    pub deletion_protection_enabled: Option<bool>,
    /// Deny maintenance periods
    #[serde(rename="denyMaintenancePeriods")]
    
    pub deny_maintenance_periods: Option<Vec<DenyMaintenancePeriod>>,
    /// Insights configuration, for now relevant only for Postgres.
    #[serde(rename="insightsConfig")]
    
    pub insights_config: Option<InsightsConfig>,
    /// The settings for IP Management. This allows to enable or disable the instance IP and manage which external networks can connect to the instance. The IPv4 address cannot be disabled for Second Generation instances.
    #[serde(rename="ipConfiguration")]
    
    pub ip_configuration: Option<IpConfiguration>,
    /// This is always `sql#settings`.
    
    pub kind: Option<String>,
    /// The location preference settings. This allows the instance to be located as near as possible to either an App Engine app or Compute Engine zone for better performance. App Engine co-location was only applicable to First Generation instances.
    #[serde(rename="locationPreference")]
    
    pub location_preference: Option<LocationPreference>,
    /// The maintenance window for this instance. This specifies when the instance can be restarted for maintenance purposes.
    #[serde(rename="maintenanceWindow")]
    
    pub maintenance_window: Option<MaintenanceWindow>,
    /// The local user password validation policy of the instance.
    #[serde(rename="passwordValidationPolicy")]
    
    pub password_validation_policy: Option<PasswordValidationPolicy>,
    /// The pricing plan for this instance. This can be either `PER_USE` or `PACKAGE`. Only `PER_USE` is supported for Second Generation instances.
    #[serde(rename="pricingPlan")]
    
    pub pricing_plan: Option<SettingPricingPlanEnum>,
    /// The type of replication this instance uses. This can be either `ASYNCHRONOUS` or `SYNCHRONOUS`. (Deprecated) This property was only applicable to First Generation instances.
    #[serde(rename="replicationType")]
    
    pub replication_type: Option<SettingReplicationTypeEnum>,
    /// The version of instance settings. This is a required field for update method to make sure concurrent updates are handled properly. During update, use the most recent settingsVersion value for this instance and do not try to update this value.
    #[serde(rename="settingsVersion")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub settings_version: Option<i64>,
    /// SQL Server specific audit configuration.
    #[serde(rename="sqlServerAuditConfig")]
    
    pub sql_server_audit_config: Option<SqlServerAuditConfig>,
    /// Configuration to increase storage size automatically. The default value is true.
    #[serde(rename="storageAutoResize")]
    
    pub storage_auto_resize: Option<bool>,
    /// The maximum size to which storage capacity can be automatically increased. The default value is 0, which specifies that there is no limit.
    #[serde(rename="storageAutoResizeLimit")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub storage_auto_resize_limit: Option<i64>,
    /// The tier (or machine type) for this instance, for example `db-custom-1-3840`. WARNING: Changing this restarts the instance.
    
    pub tier: Option<String>,
    /// Server timezone, relevant only for Cloud SQL for SQL Server.
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<String>,
    /// User-provided labels, represented as a dictionary where each label is a single key value pair.
    #[serde(rename="userLabels")]
    
    pub user_labels: Option<HashMap<String, String>>,
}

impl client::Part for Settings {}


/// Active Directory configuration, relevant only for Cloud SQL for SQL Server.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SqlActiveDirectoryConfig {
    /// The name of the domain (e.g., mydomain.com).
    
    pub domain: Option<String>,
    /// This is always sql#activeDirectoryConfig.
    
    pub kind: Option<String>,
}

impl client::Part for SqlActiveDirectoryConfig {}


/// External primary instance migration setting error/warning.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SqlExternalSyncSettingError {
    /// Additional information about the error encountered.
    
    pub detail: Option<String>,
    /// Can be `sql#externalSyncSettingError` or `sql#externalSyncSettingWarning`.
    
    pub kind: Option<String>,
    /// Identifies the specific error that occurred.
    #[serde(rename="type")]
    
    pub type_: Option<SqlExternalSyncSettingErrorTypeEnum>,
}

impl client::Part for SqlExternalSyncSettingError {}


/// Reschedule options for maintenance windows.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances reschedule maintenance projects](ProjectInstanceRescheduleMaintenanceCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SqlInstancesRescheduleMaintenanceRequestBody {
    /// Required. The type of the reschedule the user wants.
    
    pub reschedule: Option<Reschedule>,
}

impl client::RequestValue for SqlInstancesRescheduleMaintenanceRequestBody {}


/// Instance start external sync request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances start external sync projects](ProjectInstanceStartExternalSyncCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SqlInstancesStartExternalSyncRequest {
    /// MySQL-specific settings for start external sync.
    #[serde(rename="mysqlSyncConfig")]
    
    pub mysql_sync_config: Option<MySqlSyncConfig>,
    /// Whether to skip the verification step (VESS).
    #[serde(rename="skipVerification")]
    
    pub skip_verification: Option<bool>,
    /// External sync mode.
    #[serde(rename="syncMode")]
    
    pub sync_mode: Option<SqlInstancesStartExternalSyncRequestSyncModeEnum>,
}

impl client::RequestValue for SqlInstancesStartExternalSyncRequest {}


/// Instance verify external sync settings request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances verify external sync settings projects](ProjectInstanceVerifyExternalSyncSettingCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SqlInstancesVerifyExternalSyncSettingsRequest {
    /// Optional. MySQL-specific settings for start external sync.
    #[serde(rename="mysqlSyncConfig")]
    
    pub mysql_sync_config: Option<MySqlSyncConfig>,
    /// External sync mode
    #[serde(rename="syncMode")]
    
    pub sync_mode: Option<SqlInstancesVerifyExternalSyncSettingsRequestSyncModeEnum>,
    /// Flag to enable verifying connection only
    #[serde(rename="verifyConnectionOnly")]
    
    pub verify_connection_only: Option<bool>,
    /// Optional. Flag to verify settings required by replication setup only
    #[serde(rename="verifyReplicationOnly")]
    
    pub verify_replication_only: Option<bool>,
}

impl client::RequestValue for SqlInstancesVerifyExternalSyncSettingsRequest {}


/// Instance verify external sync settings response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances verify external sync settings projects](ProjectInstanceVerifyExternalSyncSettingCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SqlInstancesVerifyExternalSyncSettingsResponse {
    /// List of migration violations.
    
    pub errors: Option<Vec<SqlExternalSyncSettingError>>,
    /// This is always `sql#migrationSettingErrorList`.
    
    pub kind: Option<String>,
    /// List of migration warnings.
    
    pub warnings: Option<Vec<SqlExternalSyncSettingError>>,
}

impl client::ResponseResult for SqlInstancesVerifyExternalSyncSettingsResponse {}


/// This message wraps up the information written by out-of-disk detection job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SqlOutOfDiskReport {
    /// The minimum recommended increase size in GigaBytes This field is consumed by the frontend * Writers: * the proactive database wellness job for OOD. * Readers:
    #[serde(rename="sqlMinRecommendedIncreaseSizeGb")]
    
    pub sql_min_recommended_increase_size_gb: Option<i32>,
    /// This field represents the state generated by the proactive database wellness job for OutOfDisk issues. * Writers: * the proactive database wellness job for OOD. * Readers: * the proactive database wellness job
    #[serde(rename="sqlOutOfDiskState")]
    
    pub sql_out_of_disk_state: Option<SqlOutOfDiskReportSqlOutOfDiskStateEnum>,
}

impl client::Part for SqlOutOfDiskReport {}


/// Any scheduled maintenance for this instance.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SqlScheduledMaintenance {
    /// no description provided
    #[serde(rename="canDefer")]
    
    pub can_defer: Option<bool>,
    /// If the scheduled maintenance can be rescheduled.
    #[serde(rename="canReschedule")]
    
    pub can_reschedule: Option<bool>,
    /// Maintenance cannot be rescheduled to start beyond this deadline.
    #[serde(rename="scheduleDeadlineTime")]
    
    pub schedule_deadline_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The start time of any upcoming scheduled maintenance for this instance.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for SqlScheduledMaintenance {}


/// SQL Server specific audit configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SqlServerAuditConfig {
    /// The name of the destination bucket (e.g., gs://mybucket).
    
    pub bucket: Option<String>,
    /// This is always sql#sqlServerAuditConfig
    
    pub kind: Option<String>,
    /// How long to keep generated audit files.
    #[serde(rename="retentionInterval")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub retention_interval: Option<client::chrono::Duration>,
    /// How often to upload generated audit files.
    #[serde(rename="uploadInterval")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub upload_interval: Option<client::chrono::Duration>,
}

impl client::Part for SqlServerAuditConfig {}


/// Represents a Sql Server database on the Cloud SQL instance.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SqlServerDatabaseDetails {
    /// The version of SQL Server with which the database is to be made compatible
    #[serde(rename="compatibilityLevel")]
    
    pub compatibility_level: Option<i32>,
    /// The recovery model of a SQL Server database
    #[serde(rename="recoveryModel")]
    
    pub recovery_model: Option<String>,
}

impl client::Part for SqlServerDatabaseDetails {}


/// Represents a Sql Server user on the Cloud SQL instance.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SqlServerUserDetails {
    /// If the user has been disabled
    
    pub disabled: Option<bool>,
    /// The server roles for this user
    #[serde(rename="serverRoles")]
    
    pub server_roles: Option<Vec<String>>,
}

impl client::Part for SqlServerUserDetails {}


/// SslCerts Resource
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create ephemeral ssl certs](SslCertCreateEphemeralCall) (response)
/// * [delete ssl certs](SslCertDeleteCall) (none)
/// * [get ssl certs](SslCertGetCall) (response)
/// * [insert ssl certs](SslCertInsertCall) (none)
/// * [list ssl certs](SslCertListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SslCert {
    /// PEM representation.
    
    pub cert: Option<String>,
    /// Serial number, as extracted from the certificate.
    #[serde(rename="certSerialNumber")]
    
    pub cert_serial_number: Option<String>,
    /// User supplied name. Constrained to [a-zA-Z.-_ ]+.
    #[serde(rename="commonName")]
    
    pub common_name: Option<String>,
    /// The time when the certificate was created in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The time when the certificate expires in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`.
    #[serde(rename="expirationTime")]
    
    pub expiration_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Name of the database instance.
    
    pub instance: Option<String>,
    /// This is always `sql#sslCert`.
    
    pub kind: Option<String>,
    /// The URI of this resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Sha1 Fingerprint.
    #[serde(rename="sha1Fingerprint")]
    
    pub sha1_fingerprint: Option<String>,
}

impl client::Resource for SslCert {}
impl client::ResponseResult for SslCert {}


/// SslCertDetail.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SslCertDetail {
    /// The public information about the cert.
    #[serde(rename="certInfo")]
    
    pub cert_info: Option<SslCert>,
    /// The private key for the client cert, in pem format. Keep private in order to protect your security.
    #[serde(rename="certPrivateKey")]
    
    pub cert_private_key: Option<String>,
}

impl client::Part for SslCertDetail {}


/// SslCerts create ephemeral certificate request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create ephemeral ssl certs](SslCertCreateEphemeralCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SslCertsCreateEphemeralRequest {
    /// Access token to include in the signed certificate.
    
    pub access_token: Option<String>,
    /// PEM encoded public key to include in the signed certificate.
    
    pub public_key: Option<String>,
}

impl client::RequestValue for SslCertsCreateEphemeralRequest {}


/// SslCerts insert request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insert ssl certs](SslCertInsertCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SslCertsInsertRequest {
    /// User supplied name. Must be a distinct name from the other certificates for this instance.
    #[serde(rename="commonName")]
    
    pub common_name: Option<String>,
}

impl client::RequestValue for SslCertsInsertRequest {}


/// SslCert insert response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insert ssl certs](SslCertInsertCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SslCertsInsertResponse {
    /// The new client certificate and private key.
    #[serde(rename="clientCert")]
    
    pub client_cert: Option<SslCertDetail>,
    /// This is always `sql#sslCertsInsert`.
    
    pub kind: Option<String>,
    /// The operation to track the ssl certs insert request.
    
    pub operation: Option<Operation>,
    /// The server Certificate Authority's certificate. If this is missing you can force a new one to be generated by calling resetSslConfig method on instances resource.
    #[serde(rename="serverCaCert")]
    
    pub server_ca_cert: Option<SslCert>,
}

impl client::ResponseResult for SslCertsInsertResponse {}


/// SslCerts list response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list ssl certs](SslCertListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SslCertsListResponse {
    /// List of client certificates for the instance.
    
    pub items: Option<Vec<SslCert>>,
    /// This is always `sql#sslCertsList`.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for SslCertsListResponse {}


/// Initial sync flags for certain Cloud SQL APIs. Currently used for the MySQL external server initial dump.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SyncFlags {
    /// The name of the flag.
    
    pub name: Option<String>,
    /// The value of the flag. This field must be omitted if the flag doesn't take a value.
    
    pub value: Option<String>,
}

impl client::Part for SyncFlags {}


/// A Google Cloud SQL service tier resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list tiers](TierListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Tier {
    /// The maximum disk size of this tier in bytes.
    #[serde(rename="DiskQuota")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub disk_quota: Option<i64>,
    /// The maximum RAM usage of this tier in bytes.
    #[serde(rename="RAM")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub ram: Option<i64>,
    /// This is always `sql#tier`.
    
    pub kind: Option<String>,
    /// The applicable regions for this tier.
    
    pub region: Option<Vec<String>>,
    /// An identifier for the machine type, for example, `db-custom-1-3840`. For related information, see [Pricing](https://developers.google.com/sql/pricing).
    
    pub tier: Option<String>,
}

impl client::Resource for Tier {}


/// Tiers list response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list tiers](TierListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TiersListResponse {
    /// List of tiers.
    
    pub items: Option<Vec<Tier>>,
    /// This is always `sql#tiersList`.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for TiersListResponse {}


/// Database Instance truncate log context.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TruncateLogContext {
    /// This is always `sql#truncateLogContext`.
    
    pub kind: Option<String>,
    /// The type of log to truncate. Valid values are `MYSQL_GENERAL_TABLE` and `MYSQL_SLOW_TABLE`.
    #[serde(rename="logType")]
    
    pub log_type: Option<String>,
}

impl client::Part for TruncateLogContext {}


/// A Cloud SQL user resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete users](UserDeleteCall) (none)
/// * [get users](UserGetCall) (response)
/// * [insert users](UserInsertCall) (request)
/// * [list users](UserListCall) (none)
/// * [update users](UserUpdateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct User {
    /// Dual password status for the user.
    #[serde(rename="dualPasswordType")]
    
    pub dual_password_type: Option<UserDualPasswordTypeEnum>,
    /// This field is deprecated and will be removed from a future version of the API.
    
    pub etag: Option<String>,
    /// Optional. The host from which the user can connect. For `insert` operations, host defaults to an empty string. For `update` operations, host is specified as part of the request URL. The host name cannot be updated after insertion. For a MySQL instance, it's required; for a PostgreSQL or SQL Server instance, it's optional.
    
    pub host: Option<String>,
    /// The name of the Cloud SQL instance. This does not include the project ID. Can be omitted for `update` because it is already specified on the URL.
    
    pub instance: Option<String>,
    /// This is always `sql#user`.
    
    pub kind: Option<String>,
    /// The name of the user in the Cloud SQL instance. Can be omitted for `update` because it is already specified in the URL.
    
    pub name: Option<String>,
    /// The password for the user.
    
    pub password: Option<String>,
    /// User level password validation policy.
    #[serde(rename="passwordPolicy")]
    
    pub password_policy: Option<UserPasswordValidationPolicy>,
    /// The project ID of the project containing the Cloud SQL database. The Google apps domain is prefixed if applicable. Can be omitted for `update` because it is already specified on the URL.
    
    pub project: Option<String>,
    /// no description provided
    #[serde(rename="sqlserverUserDetails")]
    
    pub sqlserver_user_details: Option<SqlServerUserDetails>,
    /// The user type. It determines the method to authenticate the user during login. The default is the database's built-in user type.
    #[serde(rename="type")]
    
    pub type_: Option<UserTypeEnum>,
}

impl client::RequestValue for User {}
impl client::Resource for User {}
impl client::ResponseResult for User {}


/// User level password validation policy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserPasswordValidationPolicy {
    /// Number of failed login attempts allowed before user get locked.
    #[serde(rename="allowedFailedAttempts")]
    
    pub allowed_failed_attempts: Option<i32>,
    /// If true, failed login attempts check will be enabled.
    #[serde(rename="enableFailedAttemptsCheck")]
    
    pub enable_failed_attempts_check: Option<bool>,
    /// If true, the user must specify the current password before changing the password. This flag is supported only for MySQL.
    #[serde(rename="enablePasswordVerification")]
    
    pub enable_password_verification: Option<bool>,
    /// Expiration duration after password is updated.
    #[serde(rename="passwordExpirationDuration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub password_expiration_duration: Option<client::chrono::Duration>,
    /// Output only. Read-only password status.
    
    pub status: Option<PasswordStatus>,
}

impl client::Part for UserPasswordValidationPolicy {}


/// User list response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list users](UserListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UsersListResponse {
    /// List of user resources in the instance.
    
    pub items: Option<Vec<User>>,
    /// This is always `sql#usersList`.
    
    pub kind: Option<String>,
    /// An identifier that uniquely identifies the operation. You can use this identifier to retrieve the Operations resource that has information about the operation.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for UsersListResponse {}


/// The name and status of the failover replica.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatabaseInstanceFailoverReplica {
    /// The availability status of the failover replica. A false status indicates that the failover replica is out of sync. The primary instance can only failover to the failover replica when the status is true.
    
    pub available: Option<bool>,
    /// The name of the failover replica. If specified at instance creation, a failover replica is created for the instance. The name doesn't include the project ID.
    
    pub name: Option<String>,
}

impl client::NestedType for DatabaseInstanceFailoverReplica {}
impl client::Part for DatabaseInstanceFailoverReplica {}


/// Options for exporting BAK files (SQL Server-only)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExportContextBakExportOptions {
    /// Option for specifying how many stripes to use for the export. If blank, and the value of the striped field is true, the number of stripes is automatically chosen.
    #[serde(rename="stripeCount")]
    
    pub stripe_count: Option<i32>,
    /// Whether or not the export should be striped.
    
    pub striped: Option<bool>,
}

impl client::NestedType for ExportContextBakExportOptions {}
impl client::Part for ExportContextBakExportOptions {}


/// Options for exporting data as CSV. `MySQL` and `PostgreSQL` instances only.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExportContextCsvExportOptions {
    /// Specifies the character that should appear before a data character that needs to be escaped.
    #[serde(rename="escapeCharacter")]
    
    pub escape_character: Option<String>,
    /// Specifies the character that separates columns within each row (line) of the file.
    #[serde(rename="fieldsTerminatedBy")]
    
    pub fields_terminated_by: Option<String>,
    /// This is used to separate lines. If a line does not contain all fields, the rest of the columns are set to their default values.
    #[serde(rename="linesTerminatedBy")]
    
    pub lines_terminated_by: Option<String>,
    /// Specifies the quoting character to be used when a data value is quoted.
    #[serde(rename="quoteCharacter")]
    
    pub quote_character: Option<String>,
    /// The select query used to extract the data.
    #[serde(rename="selectQuery")]
    
    pub select_query: Option<String>,
}

impl client::NestedType for ExportContextCsvExportOptions {}
impl client::Part for ExportContextCsvExportOptions {}


/// Options for exporting data as SQL statements.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExportContextSqlExportOptions {
    /// Options for exporting from MySQL.
    #[serde(rename="mysqlExportOptions")]
    
    pub mysql_export_options: Option<ExportContextSqlExportOptionsMysqlExportOptions>,
    /// Export only schemas.
    #[serde(rename="schemaOnly")]
    
    pub schema_only: Option<bool>,
    /// Tables to export, or that were exported, from the specified database. If you specify tables, specify one and only one database. For PostgreSQL instances, you can specify only one table.
    
    pub tables: Option<Vec<String>>,
}

impl client::NestedType for ExportContextSqlExportOptions {}
impl client::Part for ExportContextSqlExportOptions {}


/// Options for exporting from MySQL.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExportContextSqlExportOptionsMysqlExportOptions {
    /// Option to include SQL statement required to set up replication. If set to `1`, the dump file includes a CHANGE MASTER TO statement with the binary log coordinates, and --set-gtid-purged is set to ON. If set to `2`, the CHANGE MASTER TO statement is written as a SQL comment and has no effect. If set to any value other than `1`, --set-gtid-purged is set to OFF.
    #[serde(rename="masterData")]
    
    pub master_data: Option<i32>,
}

impl client::NestedType for ExportContextSqlExportOptionsMysqlExportOptions {}
impl client::Part for ExportContextSqlExportOptionsMysqlExportOptions {}


/// Import parameters specific to SQL Server .BAK files
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImportContextBakImportOptions {
    /// no description provided
    #[serde(rename="encryptionOptions")]
    
    pub encryption_options: Option<ImportContextBakImportOptionsEncryptionOptions>,
    /// Whether or not the backup set being restored is striped. Applies only to Cloud SQL for SQL Server.
    
    pub striped: Option<bool>,
}

impl client::NestedType for ImportContextBakImportOptions {}
impl client::Part for ImportContextBakImportOptions {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImportContextBakImportOptionsEncryptionOptions {
    /// Path to the Certificate (.cer) in Cloud Storage, in the form `gs://bucketName/fileName`. The instance must have write permissions to the bucket and read access to the file.
    #[serde(rename="certPath")]
    
    pub cert_path: Option<String>,
    /// Password that encrypts the private key
    #[serde(rename="pvkPassword")]
    
    pub pvk_password: Option<String>,
    /// Path to the Certificate Private Key (.pvk) in Cloud Storage, in the form `gs://bucketName/fileName`. The instance must have write permissions to the bucket and read access to the file.
    #[serde(rename="pvkPath")]
    
    pub pvk_path: Option<String>,
}

impl client::NestedType for ImportContextBakImportOptionsEncryptionOptions {}
impl client::Part for ImportContextBakImportOptionsEncryptionOptions {}


/// Options for importing data as CSV.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImportContextCsvImportOptions {
    /// The columns to which CSV data is imported. If not specified, all columns of the database table are loaded with CSV data.
    
    pub columns: Option<Vec<String>>,
    /// Specifies the character that should appear before a data character that needs to be escaped.
    #[serde(rename="escapeCharacter")]
    
    pub escape_character: Option<String>,
    /// Specifies the character that separates columns within each row (line) of the file.
    #[serde(rename="fieldsTerminatedBy")]
    
    pub fields_terminated_by: Option<String>,
    /// This is used to separate lines. If a line does not contain all fields, the rest of the columns are set to their default values.
    #[serde(rename="linesTerminatedBy")]
    
    pub lines_terminated_by: Option<String>,
    /// Specifies the quoting character to be used when a data value is quoted.
    #[serde(rename="quoteCharacter")]
    
    pub quote_character: Option<String>,
    /// The table to which CSV data is imported.
    
    pub table: Option<String>,
}

impl client::NestedType for ImportContextCsvImportOptions {}
impl client::Part for ImportContextCsvImportOptions {}


