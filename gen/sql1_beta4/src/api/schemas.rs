use super::*;
/// An entry for an Access Control list.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AclEntry {
    /// The time when this access control entry expires in <a
    /// href="https://tools.ietf.org/html/rfc3339">RFC 3339</a> format, for example
    /// <code>2012-11-15T16:19:00.094Z</code>.
    #[serde(rename="expirationTime")]
    
    pub expiration_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// This is always <code>sql#aclEntry</code>.
    
    pub kind: Option<String>,
    /// Optional. A label to identify this entry.
    
    pub name: Option<String>,
    /// The whitelisted value for the access control list.
    
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
}

impl client::Part for ApiWarning {}


/// Database instance backup configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BackupConfiguration {
    /// (MySQL only) Whether binary log is enabled. If backup configuration is
    /// disabled, binarylog must be disabled as well.
    #[serde(rename="binaryLogEnabled")]
    
    pub binary_log_enabled: Option<bool>,
    /// Whether this configuration is enabled.
    
    pub enabled: Option<bool>,
    /// This is always <code>sql#backupConfiguration</code>.
    
    pub kind: Option<String>,
    /// Location of the backup
    
    pub location: Option<String>,
    /// Reserved for future use.
    #[serde(rename="pointInTimeRecoveryEnabled")]
    
    pub point_in_time_recovery_enabled: Option<bool>,
    /// Reserved for future use.
    #[serde(rename="replicationLogArchivingEnabled")]
    
    pub replication_log_archiving_enabled: Option<bool>,
    /// Start time for the daily backup configuration in UTC timezone in the 24
    /// hour format - <code>HH:MM</code>.
    #[serde(rename="startTime")]
    
    pub start_time: Option<String>,
}

impl client::Part for BackupConfiguration {}


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
    /// The description of this run, only applicable to on-demand backups.
    
    pub description: Option<String>,
    /// Encryption configuration specific to a backup.
    /// Applies only to Second Generation instances.
    #[serde(rename="diskEncryptionConfiguration")]
    
    pub disk_encryption_configuration: Option<DiskEncryptionConfiguration>,
    /// Encryption status specific to a backup.
    /// Applies only to Second Generation instances.
    #[serde(rename="diskEncryptionStatus")]
    
    pub disk_encryption_status: Option<DiskEncryptionStatus>,
    /// The time the backup operation completed in UTC timezone in <a
    /// href="https://tools.ietf.org/html/rfc3339">RFC 3339</a> format, for example
    /// <code>2012-11-15T16:19:00.094Z</code>.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The time the run was enqueued in UTC timezone in <a
    /// href="https://tools.ietf.org/html/rfc3339">RFC 3339</a> format, for example
    /// <code>2012-11-15T16:19:00.094Z</code>.
    #[serde(rename="enqueuedTime")]
    
    pub enqueued_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Information about why the backup operation failed. This is only present if
    /// the run has the FAILED status.
    
    pub error: Option<OperationError>,
    /// The identifier for this backup run. Unique only for a specific Cloud SQL
    /// instance.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Name of the database instance.
    
    pub instance: Option<String>,
    /// This is always <code>sql#backupRun</code>.
    
    pub kind: Option<String>,
    /// Location of the backups.
    
    pub location: Option<String>,
    /// The URI of this resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// The time the backup operation actually started in UTC timezone in <a
    /// href="https://tools.ietf.org/html/rfc3339">RFC 3339</a> format, for example
    /// <code>2012-11-15T16:19:00.094Z</code>.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The status of this run.
    
    pub status: Option<BackupRunStatusEnum>,
    /// The type of this run; can be either "AUTOMATED" or "ON_DEMAND".
    #[serde(rename="type")]
    
    pub type_: Option<BackupRunTypeEnum>,
    /// The start time of the backup window during which this the backup was
    /// attempted in <a href="https://tools.ietf.org/html/rfc3339">RFC 3339</a>
    /// format, for example <code>2012-11-15T16:19:00.094Z</code>.
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
    /// This is always <code>sql#backupRunsList</code>.
    
    pub kind: Option<String>,
    /// The continuation token, used to page through large result sets. Provide
    /// this value in a subsequent request to return the next page of results.
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
    /// This is always <code>sql#binLogCoordinates</code>.
    
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
    /// Binary log coordinates, if specified, identify the position up to which the
    /// source instance should be cloned. If not specified, the source instance is
    /// cloned up to the most recent binary log coordinates.
    #[serde(rename="binLogCoordinates")]
    
    pub bin_log_coordinates: Option<BinLogCoordinates>,
    /// Name of the Cloud SQL instance to be created as a clone.
    #[serde(rename="destinationInstanceName")]
    
    pub destination_instance_name: Option<String>,
    /// This is always <code>sql#cloneContext</code>.
    
    pub kind: Option<String>,
    /// Reserved for future use.
    #[serde(rename="pitrTimestampMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub pitr_timestamp_ms: Option<i64>,
    /// Reserved for future use.
    #[serde(rename="pointInTime")]
    
    pub point_in_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for CloneContext {}


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
    /// The MySQL charset value.
    
    pub charset: Option<String>,
    /// The MySQL collation value.
    
    pub collation: Option<String>,
    /// This field is deprecated and will be removed from a future version of the
    /// API.
    
    pub etag: Option<String>,
    /// The name of the Cloud SQL instance. This does not include the project ID.
    
    pub instance: Option<String>,
    /// This is always <code>sql#database</code>.
    
    pub kind: Option<String>,
    /// The name of the database in the Cloud SQL instance. This does not include
    /// the project ID or instance name.
    
    pub name: Option<String>,
    /// The project ID of the project containing the Cloud SQL database. The Google
    /// apps domain is prefixed if applicable.
    
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
    /// The name of the flag. These flags are passed at instance startup, so
    /// include both server options and system variables for MySQL. Flags should be
    /// specified with underscores, not hyphens. For more information, see <a
    /// href="/sql/docs/mysql/flags">Configuring Database Flags</a> in the Cloud
    /// SQL documentation.
    
    pub name: Option<String>,
    /// The value of the flag. Booleans should be set to <code>on</code> for true
    /// and <code>off</code> for false. This field must be omitted if the flag
    /// doesn't take a value.
    
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
    /// <code>FIRST_GEN</code>: First Generation instance. MySQL only. <br
    /// /><code>SECOND_GEN</code>: Second Generation instance or PostgreSQL
    /// instance. <br /><code>EXTERNAL</code>: A database server that is not
    /// managed by Google. <br>This property is read-only; use the
    /// <code>tier</code> property in the <code>settings</code> object to determine
    /// the database type and Second or First Generation.
    #[serde(rename="backendType")]
    
    pub backend_type: Option<DatabaseInstanceBackendTypeEnum>,
    /// Connection name of the Cloud SQL instance used in connection strings.
    #[serde(rename="connectionName")]
    
    pub connection_name: Option<String>,
    /// The current disk usage of the instance in bytes. This property has been
    /// deprecated. Users should use the
    /// "cloudsql.googleapis.com/database/disk/bytes_used" metric in Cloud
    /// Monitoring API instead. Please see <a
    /// href="https://groups.google.com/d/msg/google-cloud-sql-announce/I_7-F9EBhT0/BtvFtdFeAgAJ">this
    /// announcement</a> for details.
    #[serde(rename="currentDiskSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub current_disk_size: Option<i64>,
    /// The database engine type and version. The <code>databaseVersion</code>
    /// field can not be changed after instance creation.  MySQL Second Generation
    /// instances: <code>MYSQL_5_7</code> (default) or <code>MYSQL_5_6</code>.
    /// PostgreSQL instances: <code>POSTGRES_9_6</code> (default) or
    /// <code>POSTGRES_11 Beta</code> MySQL First Generation
    /// instances: <code>MYSQL_5_6</code> (default) or <code>MYSQL_5_5</code>
    #[serde(rename="databaseVersion")]
    
    pub database_version: Option<DatabaseInstanceDatabaseVersionEnum>,
    /// Disk encryption configuration specific to an instance.
    /// Applies only to Second Generation instances.
    #[serde(rename="diskEncryptionConfiguration")]
    
    pub disk_encryption_configuration: Option<DiskEncryptionConfiguration>,
    /// Disk encryption status specific to an instance.
    /// Applies only to Second Generation instances.
    #[serde(rename="diskEncryptionStatus")]
    
    pub disk_encryption_status: Option<DiskEncryptionStatus>,
    /// This field is deprecated and will be removed from a future version of the
    /// API. Use the <code>settings.settingsVersion</code> field instead.
    
    pub etag: Option<String>,
    /// The name and status of the failover replica. This property is applicable
    /// only to Second Generation instances.
    #[serde(rename="failoverReplica")]
    
    pub failover_replica: Option<DatabaseInstanceFailoverReplica>,
    /// The Compute Engine zone that the instance is currently serving from. This
    /// value could be different from the zone that was specified when the instance
    /// was created if the instance has failed over to its secondary zone.
    #[serde(rename="gceZone")]
    
    pub gce_zone: Option<String>,
    /// The instance type. This can be one of the following.
    /// <br><code>CLOUD_SQL_INSTANCE</code>: A Cloud SQL instance that is not
    /// replicating from a master. <br><code>ON_PREMISES_INSTANCE</code>: An
    /// instance running on the
    /// customer's premises. <br><code>READ_REPLICA_INSTANCE</code>: A Cloud SQL
    /// instance configured as a read-replica.
    #[serde(rename="instanceType")]
    
    pub instance_type: Option<DatabaseInstanceInstanceTypeEnum>,
    /// The assigned IP addresses for the instance.
    #[serde(rename="ipAddresses")]
    
    pub ip_addresses: Option<Vec<IpMapping>>,
    /// The IPv6 address assigned to the instance. This property is applicable only
    /// to First Generation instances.
    #[serde(rename="ipv6Address")]
    
    pub ipv6_address: Option<String>,
    /// This is always <code>sql#instance</code>.
    
    pub kind: Option<String>,
    /// The name of the instance which will act as master in the replication setup.
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
    /// The project ID of the project containing the Cloud SQL instance. The Google
    /// apps domain is prefixed if applicable.
    
    pub project: Option<String>,
    /// The geographical region. Can be <code>us-central</code>
    /// (<code>FIRST_GEN</code> instances only), <code>us-central1</code>
    /// (<code>SECOND_GEN</code> instances only), <code>asia-east1</code> or
    /// <code>europe-west1</code>. Defaults to <code>us-central</code> or
    /// <code>us-central1</code> depending on the instance type (First Generation
    /// or Second Generation). The region can not be changed after instance
    /// creation.
    
    pub region: Option<String>,
    /// Configuration specific to failover replicas and read replicas.
    #[serde(rename="replicaConfiguration")]
    
    pub replica_configuration: Option<ReplicaConfiguration>,
    /// The replicas of the instance.
    #[serde(rename="replicaNames")]
    
    pub replica_names: Option<Vec<String>>,
    /// Initial root password. Use only on creation.
    #[serde(rename="rootPassword")]
    
    pub root_password: Option<String>,
    /// The start time of any upcoming scheduled maintenance for this instance.
    #[serde(rename="scheduledMaintenance")]
    
    pub scheduled_maintenance: Option<SqlScheduledMaintenance>,
    /// The URI of this resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// SSL configuration.
    #[serde(rename="serverCaCert")]
    
    pub server_ca_cert: Option<SslCert>,
    /// The service account email address assigned to the instance. This property
    /// is applicable only to Second Generation instances.
    #[serde(rename="serviceAccountEmailAddress")]
    
    pub service_account_email_address: Option<String>,
    /// The user settings.
    
    pub settings: Option<Settings>,
    /// The current serving state of the Cloud SQL instance. This can be one of the
    /// following. <br><code>RUNNABLE</code>: The instance is running, or is ready
    /// to run when accessed. <br><code>SUSPENDED</code>: The instance is not
    /// available, for example due to problems with billing.
    /// <br><code>PENDING_CREATE</code>: The instance is being created.
    /// <br><code>MAINTENANCE</code>: The instance is down for maintenance.
    /// <br><code>FAILED</code>: The instance creation failed.
    /// <br><code>UNKNOWN_STATE</code>: The state of the instance is unknown.
    
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
    /// This is always <code>sql#databasesList</code>.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for DatabasesListResponse {}


/// Read-replica configuration for connecting to the on-premises master.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DemoteMasterConfiguration {
    /// This is always <code>sql#demoteMasterConfiguration</code>.
    
    pub kind: Option<String>,
    /// MySQL specific configuration when replicating from a MySQL on-premises
    /// master. Replication configuration information such as the username,
    /// password, certificates, and keys are not stored in the instance metadata.
    /// The configuration information is used only to set up the replication
    /// connection and is stored by MySQL in a file named <code>master.info</code>
    /// in the data directory.
    #[serde(rename="mysqlReplicaConfiguration")]
    
    pub mysql_replica_configuration: Option<DemoteMasterMySqlReplicaConfiguration>,
}

impl client::Part for DemoteMasterConfiguration {}


/// Database instance demote master context.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DemoteMasterContext {
    /// This is always <code>sql#demoteMasterContext</code>.
    
    pub kind: Option<String>,
    /// The name of the instance which will act as on-premises master in the
    /// replication setup.
    #[serde(rename="masterInstanceName")]
    
    pub master_instance_name: Option<String>,
    /// Configuration specific to read-replicas replicating from the on-premises
    /// master.
    #[serde(rename="replicaConfiguration")]
    
    pub replica_configuration: Option<DemoteMasterConfiguration>,
    /// Verify GTID consistency for demote operation. Default value:
    /// <code>True</code>. Second Generation instances only.  Setting this flag to
    /// false enables you to bypass GTID consistency check between on-premises
    /// master and Cloud SQL instance during the demotion operation but also
    /// exposes you to the risk of future replication failures. Change the value
    /// only if you know the reason for the GTID divergence and are confident that
    /// doing so will not cause any replication issues.
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
    /// PEM representation of the slave's x509 certificate.
    #[serde(rename="clientCertificate")]
    
    pub client_certificate: Option<String>,
    /// PEM representation of the slave's private key. The corresponsing public key
    /// is encoded in the client's certificate. The format of the slave's private
    /// key can be either PKCS #1 or PKCS #8.
    #[serde(rename="clientKey")]
    
    pub client_key: Option<String>,
    /// This is always <code>sql#demoteMasterMysqlReplicaConfiguration</code>.
    
    pub kind: Option<String>,
    /// The password for the replication connection.
    
    pub password: Option<String>,
    /// The username for the replication connection.
    
    pub username: Option<String>,
}

impl client::Part for DemoteMasterMySqlReplicaConfiguration {}


/// Disk encryption configuration for an instance.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DiskEncryptionConfiguration {
    /// This is always <code>sql#diskEncryptionConfiguration</code>.
    
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
    /// This is always <code>sql#diskEncryptionStatus</code>.
    
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
    /// Options for exporting data as CSV.
    #[serde(rename="csvExportOptions")]
    
    pub csv_export_options: Option<ExportContextCsvExportOptions>,
    /// Databases to be exported. <br /> <b>MySQL instances:</b> If
    /// <code>fileType</code> is <code>SQL</code> and no database is specified, all
    /// databases are exported, except for the <code>mysql</code> system database.
    /// If <code>fileType</code> is <code>CSV</code>, you can specify one database,
    /// either by using this property or by using the
    /// <code>csvExportOptions.selectQuery</code> property, which takes precedence
    /// over this property. <br /> <b>PostgreSQL instances:</b> You must specify
    /// one database to be exported. If <code>fileType</code> is <code>CSV</code>,
    /// this database must match the one specified in the
    /// <code>csvExportOptions.selectQuery</code> property.
    
    pub databases: Option<Vec<String>>,
    /// The file type for the specified uri. <br><code>SQL</code>: The file
    /// contains SQL statements. <br><code>CSV</code>: The file contains CSV data.
    #[serde(rename="fileType")]
    
    pub file_type: Option<ExportContextFileTypeEnum>,
    /// This is always <code>sql#exportContext</code>.
    
    pub kind: Option<String>,
    /// Options for exporting data as SQL statements.
    #[serde(rename="sqlExportOptions")]
    
    pub sql_export_options: Option<ExportContextSqlExportOptions>,
    /// The path to the file in Google Cloud Storage where the export will be
    /// stored. The URI is in the form <code>gs:
    /// //bucketName/fileName</code>. If the file already exists, the requests
    /// // succeeds, but the operation fails. If <code>fileType</code> is
    /// // <code>SQL</code> and the filename ends with .gz, the contents are
    /// // compressed.
    
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
    /// This is always <code>sql#failoverContext</code>.
    
    pub kind: Option<String>,
    /// The current settings version of this instance. Request will be rejected if
    /// this version doesn't match the current settings version.
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
    /// Use this field if only certain integers are accepted. Can be combined
    /// with min_value and max_value to add additional values.
    #[serde(rename="allowedIntValues")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub allowed_int_values: Option<Vec<i64>>,
    /// For <code>STRING</code> flags, a list of strings that the value can be set
    /// to.
    #[serde(rename="allowedStringValues")]
    
    pub allowed_string_values: Option<Vec<String>>,
    /// The database version this flag applies to. Can be <code>MYSQL_5_5</code>,
    /// <code>MYSQL_5_6</code>, or <code>MYSQL_5_7</code>. <code>MYSQL_5_7</code>
    /// is applicable only to Second Generation instances.
    #[serde(rename="appliesTo")]
    
    pub applies_to: Option<Vec<FlagAppliesToEnum>>,
    /// Whether or not the flag is considered in beta.
    #[serde(rename="inBeta")]
    
    pub in_beta: Option<bool>,
    /// This is always <code>sql#flag</code>.
    
    pub kind: Option<String>,
    /// For <code>INTEGER</code> flags, the maximum allowed value.
    #[serde(rename="maxValue")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_value: Option<i64>,
    /// For <code>INTEGER</code> flags, the minimum allowed value.
    #[serde(rename="minValue")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub min_value: Option<i64>,
    /// This is the name of the flag. Flag names always use underscores, not
    /// hyphens, e.g. <code>max_allowed_packet</code>
    
    pub name: Option<String>,
    /// Indicates whether changing this flag will trigger a database restart. Only
    /// applicable to Second Generation instances.
    #[serde(rename="requiresRestart")]
    
    pub requires_restart: Option<bool>,
    /// The type of the flag. Flags are typed to being <code>BOOLEAN</code>,
    /// <code>STRING</code>, <code>INTEGER</code> or <code>NONE</code>.
    /// <code>NONE</code> is used for flags which do not take a value, such as
    /// <code>skip_grant_tables</code>.
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
    /// This is always <code>sql#flagsList</code>.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for FlagsListResponse {}


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
    /// The target database for the import. If <code>fileType</code> is
    /// <code>SQL</code>, this field is required only if the import file does not
    /// specify a database, and is overridden by any database specification in the
    /// import file. If <code>fileType</code> is <code>CSV</code>, one database
    /// must be specified.
    
    pub database: Option<String>,
    /// The file type for the specified uri. <br><code>SQL</code>: The file
    /// contains SQL statements. <br><code>CSV</code>: The file contains CSV data.
    #[serde(rename="fileType")]
    
    pub file_type: Option<ImportContextFileTypeEnum>,
    /// The PostgreSQL user for this import operation. PostgreSQL instances only.
    #[serde(rename="importUser")]
    
    pub import_user: Option<String>,
    /// This is always <code>sql#importContext</code>.
    
    pub kind: Option<String>,
    /// Path to the import file in Cloud Storage, in the form
    /// <code>gs:
    /// //bucketName/fileName</code>. Compressed gzip files (.gz) are supported
    /// // when <code>fileType</code> is <code>SQL</code>. The instance must have
    /// // write permissions to the bucket and read access to the file.
    
    pub uri: Option<String>,
}

impl client::Part for ImportContext {}


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


/// Database demote master request.
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
    /// This is always <code>sql#instancesList</code>.
    
    pub kind: Option<String>,
    /// The continuation token, used to page through large result sets. Provide
    /// this value in a subsequent request to return the next page of results.
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
    /// This is always <code>sql#instancesListServerCas</code>.
    
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


/// Rotate Server CA request.
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
    /// The list of external networks that are allowed to connect to the instance
    /// using the IP. In <a
    /// href="http://en.wikipedia.org/wiki/CIDR_notation#CIDR_notation">CIDR
    /// notation</a>, also known as 'slash' notation (e.g.
    /// <code>192.168.100.0/24</code>).
    #[serde(rename="authorizedNetworks")]
    
    pub authorized_networks: Option<Vec<AclEntry>>,
    /// Whether the instance should be assigned an IP address or not.
    #[serde(rename="ipv4Enabled")]
    
    pub ipv4_enabled: Option<bool>,
    /// The resource link for the VPC network from which the Cloud SQL instance is
    /// accessible for private IP. For example,
    /// <code>/projects/myProject/global/networks/default</code>. This setting can
    /// be updated, but it cannot be removed after it is set.
    #[serde(rename="privateNetwork")]
    
    pub private_network: Option<String>,
    /// Whether SSL connections over IP should be enforced or not.
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
    /// The due time for this IP to be retired in <a
    /// href="https://tools.ietf.org/html/rfc3339">RFC 3339</a> format, for example
    /// <code>2012-11-15T16:19:00.094Z</code>. This field is only available when
    /// the IP is scheduled to be retired.
    #[serde(rename="timeToRetire")]
    
    pub time_to_retire: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The type of this IP address. A <code>PRIMARY</code> address is a public
    /// address that can accept incoming connections. A <code>PRIVATE</code>
    /// address is a private address that can accept incoming connections. An
    /// <code>OUTGOING</code> address is the source address of connections
    /// originating from the instance, if supported.
    #[serde(rename="type")]
    
    pub type_: Option<IpMappingTypeEnum>,
}

impl client::Part for IpMapping {}


/// Preferred location. This specifies where a Cloud SQL instance should
/// preferably be located, either in a specific Compute Engine zone, or
/// co-located with an App Engine application. Note that if the preferred
/// location is not available, the instance will be located as close as possible
/// within the region. Only one location may be specified.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocationPreference {
    /// The AppEngine application to follow, it must be in the same region as the
    /// Cloud SQL instance.
    #[serde(rename="followGaeApplication")]
    
    pub follow_gae_application: Option<String>,
    /// This is always <code>sql#locationPreference</code>.
    
    pub kind: Option<String>,
    /// The preferred Compute Engine zone (e.g. us-central1-a, us-central1-b,
    /// etc.).
    
    pub zone: Option<String>,
}

impl client::Part for LocationPreference {}


/// Maintenance window. This specifies when a v2 Cloud SQL instance should
/// preferably be restarted for system maintenance purposes.
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
    /// This is always <code>sql#maintenanceWindow</code>.
    
    pub kind: Option<String>,
    /// Maintenance timing setting: <code>canary</code> (Earlier) or
    /// <code>stable</code> (Later). <br /><a
    /// href="/sql/docs/db_path/instance-settings#maintenance-timing-2ndgen">
    /// Learn more</a>.
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
    /// PEM representation of the slave's x509 certificate.
    #[serde(rename="clientCertificate")]
    
    pub client_certificate: Option<String>,
    /// PEM representation of the slave's private key. The corresponsing public key
    /// is encoded in the client's certificate.
    #[serde(rename="clientKey")]
    
    pub client_key: Option<String>,
    /// Seconds to wait between connect retries. MySQL's default is 60 seconds.
    #[serde(rename="connectRetryInterval")]
    
    pub connect_retry_interval: Option<i32>,
    /// Path to a SQL dump file in Google Cloud Storage from which the slave
    /// instance is to be created. The URI is in the form gs:
    /// //bucketName/fileName. Compressed gzip files (.gz) are also supported.
    /// // Dumps should have the binlog co-ordinates from which replication should
    /// // begin. This can be accomplished by setting --master-data to 1 when using
    /// // mysqldump.
    #[serde(rename="dumpFilePath")]
    
    pub dump_file_path: Option<String>,
    /// This is always <code>sql#mysqlReplicaConfiguration</code>.
    
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
    /// Whether or not to check the master's Common Name value in the certificate
    /// that it sends during the SSL handshake.
    #[serde(rename="verifyServerCertificate")]
    
    pub verify_server_certificate: Option<bool>,
}

impl client::Part for MySqlReplicaConfiguration {}


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
    /// PEM representation of the slave's x509 certificate.
    #[serde(rename="clientCertificate")]
    
    pub client_certificate: Option<String>,
    /// PEM representation of the slave's private key. The corresponsing public key
    /// is encoded in the client's certificate.
    #[serde(rename="clientKey")]
    
    pub client_key: Option<String>,
    /// The dump file to create the Cloud SQL replica.
    #[serde(rename="dumpFilePath")]
    
    pub dump_file_path: Option<String>,
    /// The host and port of the on-premises instance in host:port format
    #[serde(rename="hostPort")]
    
    pub host_port: Option<String>,
    /// This is always <code>sql#onPremisesConfiguration</code>.
    
    pub kind: Option<String>,
    /// The password for connecting to on-premises instance.
    
    pub password: Option<String>,
    /// The username for connecting to on-premises instance.
    
    pub username: Option<String>,
}

impl client::Part for OnPremisesConfiguration {}


/// An Operation resource. For successful operations that return an
/// Operation resource, only the fields relevant to the operation are populated
/// in the resource.
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
    /// The time this operation finished in UTC timezone in <a
    /// href="https://tools.ietf.org/html/rfc3339">RFC 3339</a> format, for example
    /// <code>2012-11-15T16:19:00.094Z</code>.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// If errors occurred during processing of this operation, this field will be
    /// populated.
    
    pub error: Option<OperationErrors>,
    /// The context for export operation, if applicable.
    #[serde(rename="exportContext")]
    
    pub export_context: Option<ExportContext>,
    /// The context for import operation, if applicable.
    #[serde(rename="importContext")]
    
    pub import_context: Option<ImportContext>,
    /// The time this operation was enqueued in UTC timezone in <a
    /// href="https://tools.ietf.org/html/rfc3339">RFC 3339</a> format, for example
    /// <code>2012-11-15T16:19:00.094Z</code>.
    #[serde(rename="insertTime")]
    
    pub insert_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// This is always <code>sql#operation</code>.
    
    pub kind: Option<String>,
    /// An identifier that uniquely identifies the operation. You can use this
    /// identifier to retrieve the Operations resource that has information about
    /// the operation.
    
    pub name: Option<String>,
    /// The type of the operation. Valid values are <code>CREATE</code>,
    /// <code>DELETE</code>, <code>UPDATE</code>, <code>RESTART</code>,
    /// <code>IMPORT</code>, <code>EXPORT</code>, <code>BACKUP_VOLUME</code>,
    /// <code>RESTORE_VOLUME</code>, <code>CREATE_USER</code>,
    /// <code>DELETE_USER</code>, <code>CREATE_DATABASE</code>,
    /// <code>DELETE_DATABASE</code> .
    #[serde(rename="operationType")]
    
    pub operation_type: Option<OperationOperationTypeEnum>,
    /// The URI of this resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// The time this operation actually started in UTC timezone in <a
    /// href="https://tools.ietf.org/html/rfc3339">RFC 3339</a> format, for example
    /// <code>2012-11-15T16:19:00.094Z</code>.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The status of an operation. Valid values are <code>PENDING</code>,
    /// <code>RUNNING</code>, <code>DONE</code>,
    /// <code>SQL_OPERATION_STATUS_UNSPECIFIED</code>.
    
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
    /// This is always <code>sql#operationError</code>.
    
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
    /// This is always <code>sql#operationErrors</code>.
    
    pub kind: Option<String>,
}

impl client::Part for OperationErrors {}


/// Database instance list operations response.
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
    /// This is always <code>sql#operationsList</code>.
    
    pub kind: Option<String>,
    /// The continuation token, used to page through large result sets. Provide
    /// this value in a subsequent request to return the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for OperationsListResponse {}


/// Read-replica configuration for connecting to the master.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReplicaConfiguration {
    /// Specifies if the replica is the failover target. If the field is set to
    /// <code>true</code> the replica will be designated as a failover replica. In
    /// case the master instance fails, the replica instance will be promoted as
    /// the new master instance.  <p>Only one replica can be specified as failover
    /// target, and the replica has to be in different zone with the master
    /// instance.
    #[serde(rename="failoverTarget")]
    
    pub failover_target: Option<bool>,
    /// This is always <code>sql#replicaConfiguration</code>.
    
    pub kind: Option<String>,
    /// MySQL specific configuration when replicating from a MySQL on-premises
    /// master. Replication configuration information such as the username,
    /// password, certificates, and keys are not stored in the instance metadata.
    /// The configuration information is used only to set up the replication
    /// connection and is stored by MySQL in a file named <code>master.info</code>
    /// in the data directory.
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
    /// Optional. Timestamp when the maintenance shall be rescheduled to if
    /// reschedule_type=SPECIFIC_TIME, in <a
    /// href="https://tools.ietf.org/html/rfc3339">RFC 3339</a> format, for
    /// example <code>2012-11-15T16:19:00.094Z</code>.
    #[serde(rename="scheduleTime")]
    
    pub schedule_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for Reschedule {}


/// Database instance restore from backup context.
/// Backup context contains source instance id and project id.
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
    /// This is always <code>sql#restoreBackupContext</code>.
    
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
    /// This is always <code>sql#rotateServerCaContext</code>.
    
    pub kind: Option<String>,
    /// The fingerprint of the next version to be rotated to. If left unspecified,
    /// will be rotated to the most recently added server CA version.
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
    /// The activation policy specifies when the instance is activated; it is
    /// applicable only when the instance state is <code>RUNNABLE</code>. Valid
    /// values: <br><code>ALWAYS</code>: The instance is on, and remains so even in
    /// the absence of connection requests. <br><code>NEVER</code>: The instance is
    /// off; it is not activated, even if a connection request arrives.
    /// <br><code>ON_DEMAND</code>: First Generation instances only. The instance
    /// responds to incoming requests, and turns itself off when not in use.
    /// Instances with <code>PER_USE</code> pricing turn off after 15 minutes of
    /// inactivity. Instances with <code>PER_PACKAGE</code> pricing turn off after
    /// 12 hours of inactivity.
    #[serde(rename="activationPolicy")]
    
    pub activation_policy: Option<SettingActivationPolicyEnum>,
    /// The App Engine app IDs that can access this instance. First Generation
    /// instances only.
    #[serde(rename="authorizedGaeApplications")]
    
    pub authorized_gae_applications: Option<Vec<String>>,
    /// Availability type (PostgreSQL and MySQL instances only). Potential values:
    /// <br><code>ZONAL</code>: The instance serves data from only one zone.
    /// Outages in that zone affect data accessibility. <br><code>REGIONAL</code>:
    /// The instance can serve data from more than one zone in a region (it is
    /// highly available). <br>For more information, see <a
    /// href="https://cloud.google.com/sql/docs/postgres/high-availability">Overview
    /// of the High Availability Configuration</a>.
    #[serde(rename="availabilityType")]
    
    pub availability_type: Option<SettingAvailabilityTypeEnum>,
    /// The daily backup configuration for the instance.
    #[serde(rename="backupConfiguration")]
    
    pub backup_configuration: Option<BackupConfiguration>,
    /// Configuration specific to read replica instances. Indicates whether
    /// database flags for crash-safe replication are enabled. This property is
    /// only applicable to First Generation instances.
    #[serde(rename="crashSafeReplicationEnabled")]
    
    pub crash_safe_replication_enabled: Option<bool>,
    /// The size of data disk, in GB. The data disk size minimum is 10GB. Not used
    /// for First Generation instances.
    #[serde(rename="dataDiskSizeGb")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub data_disk_size_gb: Option<i64>,
    /// The type of data disk: <code>PD_SSD</code> (default) or
    /// <code>PD_HDD</code>. Not used for First Generation instances.
    #[serde(rename="dataDiskType")]
    
    pub data_disk_type: Option<SettingDataDiskTypeEnum>,
    /// The database flags passed to the instance at startup.
    #[serde(rename="databaseFlags")]
    
    pub database_flags: Option<Vec<DatabaseFlags>>,
    /// Configuration specific to read replica instances. Indicates whether
    /// replication is enabled or not.
    #[serde(rename="databaseReplicationEnabled")]
    
    pub database_replication_enabled: Option<bool>,
    /// The settings for IP Management. This allows to enable or disable the
    /// instance IP and manage which external networks can connect to the instance.
    /// The IPv4 address cannot be disabled for Second Generation instances.
    #[serde(rename="ipConfiguration")]
    
    pub ip_configuration: Option<IpConfiguration>,
    /// This is always <code>sql#settings</code>.
    
    pub kind: Option<String>,
    /// The location preference settings. This allows the instance to be located as
    /// near as possible to either an App Engine app or Compute Engine zone for
    /// better performance. App Engine co-location is only applicable to First
    /// Generation instances.
    #[serde(rename="locationPreference")]
    
    pub location_preference: Option<LocationPreference>,
    /// The maintenance window for this instance. This specifies when the instance
    /// can be restarted for maintenance purposes. Not used for First Generation
    /// instances.
    #[serde(rename="maintenanceWindow")]
    
    pub maintenance_window: Option<MaintenanceWindow>,
    /// The pricing plan for this instance. This can be either <code>PER_USE</code>
    /// or <code>PACKAGE</code>. Only <code>PER_USE</code> is supported for Second
    /// Generation instances.
    #[serde(rename="pricingPlan")]
    
    pub pricing_plan: Option<SettingPricingPlanEnum>,
    /// The type of replication this instance uses. This can be either
    /// <code>ASYNCHRONOUS</code> or <code>SYNCHRONOUS</code>. This property is
    /// only applicable to First Generation instances.
    #[serde(rename="replicationType")]
    
    pub replication_type: Option<SettingReplicationTypeEnum>,
    /// The version of instance settings. This is a required field for update
    /// method to make sure concurrent updates are handled properly. During update,
    /// use the most recent settingsVersion value for this instance and do not try
    /// to update this value.
    #[serde(rename="settingsVersion")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub settings_version: Option<i64>,
    /// Configuration to increase storage size automatically. The default value is
    /// true. Not used for First Generation instances.
    #[serde(rename="storageAutoResize")]
    
    pub storage_auto_resize: Option<bool>,
    /// The maximum size to which storage capacity can be automatically increased.
    /// The default value is 0, which specifies that there is no limit. Not used
    /// for First Generation instances.
    #[serde(rename="storageAutoResizeLimit")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub storage_auto_resize_limit: Option<i64>,
    /// The tier (or machine type) for this instance, for example
    /// <code>db-n1-standard-1</code> (MySQL instances) or
    /// <code>db-custom-1-3840</code> (PostgreSQL instances). For MySQL instances,
    /// this property determines whether the instance is First or Second
    /// Generation. For more information, see <a
    /// href="/sql/docs/db_path/instance-settings">Instance Settings</a>.
    
    pub tier: Option<String>,
    /// User-provided labels, represented as a dictionary where each label is a
    /// single key value pair.
    #[serde(rename="userLabels")]
    
    pub user_labels: Option<HashMap<String, String>>,
}

impl client::Part for Settings {}


/// External master migration setting error.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SqlExternalSyncSettingError {
    /// Additional information about the error encountered.
    
    pub detail: Option<String>,
    /// This is always <code>sql#migrationSettingError</code>.
    
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
    /// This is always <code>sql#migrationSettingErrorList</code>.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for SqlInstancesVerifyExternalSyncSettingsResponse {}


/// Any scheduled maintenancce for this instance.
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
    /// The start time of any upcoming scheduled maintenance for this instance.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for SqlScheduledMaintenance {}


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
    /// User supplied name.  Constrained to [a-zA-Z.-_ ]+.
    #[serde(rename="commonName")]
    
    pub common_name: Option<String>,
    /// The time when the certificate was created in <a
    /// href="https://tools.ietf.org/html/rfc3339">RFC 3339</a> format, for example
    /// <code>2012-11-15T16:19:00.094Z</code>
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The time when the certificate expires in <a
    /// href="https://tools.ietf.org/html/rfc3339">RFC 3339</a> format, for example
    /// <code>2012-11-15T16:19:00.094Z</code>.
    #[serde(rename="expirationTime")]
    
    pub expiration_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Name of the database instance.
    
    pub instance: Option<String>,
    /// This is always <code>sql#sslCert</code>.
    
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
    /// The private key for the client cert, in pem format.  Keep private in order
    /// to protect your security.
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
    /// User supplied name.  Must be a distinct name from the other certificates
    /// for this instance.
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
    /// The new client certificate and private key.  For First Generation
    /// instances, the new certificate does not take effect until the instance is
    /// restarted.
    #[serde(rename="clientCert")]
    
    pub client_cert: Option<SslCertDetail>,
    /// This is always <code>sql#sslCertsInsert</code>.
    
    pub kind: Option<String>,
    /// The operation to track the ssl certs insert request.
    
    pub operation: Option<Operation>,
    /// The server Certificate Authority's certificate.  If this is missing you can
    /// force a new one to be generated by calling resetSslConfig method on
    /// instances resource.
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
    /// This is always <code>sql#sslCertsList</code>.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for SslCertsListResponse {}


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
    /// This is always <code>sql#tier</code>.
    
    pub kind: Option<String>,
    /// The applicable regions for this tier.
    
    pub region: Option<Vec<String>>,
    /// An identifier for the machine type, for example, db-n1-standard-1. For
    /// related information, see <a href="/sql/pricing">Pricing</a>.
    
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
    /// This is always <code>sql#tiersList</code>.
    
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
    /// This is always <code>sql#truncateLogContext</code>.
    
    pub kind: Option<String>,
    /// The type of log to truncate. Valid values are
    /// <code>MYSQL_GENERAL_TABLE</code> and <code>MYSQL_SLOW_TABLE</code>.
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
/// * [insert users](UserInsertCall) (request)
/// * [list users](UserListCall) (none)
/// * [update users](UserUpdateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct User {
    /// This field is deprecated and will be removed from a future version of the
    /// API.
    
    pub etag: Option<String>,
    /// The host name from which the user can connect. For <code>insert</code>
    /// operations, host defaults to an empty string. For <code>update</code>
    /// operations, host is specified as part of the request URL. The host name
    /// cannot be updated after insertion.
    
    pub host: Option<String>,
    /// The name of the Cloud SQL instance. This does not include the project ID.
    /// Can be omitted for <code>update</code> since it is already specified on the
    /// URL.
    
    pub instance: Option<String>,
    /// This is always <code>sql#user</code>.
    
    pub kind: Option<String>,
    /// The name of the user in the Cloud SQL instance. Can be omitted for
    /// <code>update</code> since it is already specified in the URL.
    
    pub name: Option<String>,
    /// The password for the user.
    
    pub password: Option<String>,
    /// The project ID of the project containing the Cloud SQL database. The Google
    /// apps domain is prefixed if applicable. Can be omitted for
    /// <code>update</code> since it is already specified on the URL.
    
    pub project: Option<String>,
    /// no description provided
    #[serde(rename="sqlserverUserDetails")]
    
    pub sqlserver_user_details: Option<SqlServerUserDetails>,
}

impl client::RequestValue for User {}
impl client::Resource for User {}


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
    /// This is always <code>sql#usersList</code>.
    
    pub kind: Option<String>,
    /// An identifier that uniquely identifies the operation. You can use this
    /// identifier to retrieve the Operations resource that has information about
    /// the operation.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for UsersListResponse {}


/// The name and status of the failover replica. This property is applicable
/// only to Second Generation instances.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatabaseInstanceFailoverReplica {
    /// The availability status of the failover replica. A false status indicates
    /// that the failover replica is out of sync. The master can only failover to
    /// the failover replica when the status is true.
    
    pub available: Option<bool>,
    /// The name of the failover replica. If specified at instance creation, a
    /// failover replica is created for the instance. The name
    /// doesn't include the project ID. This property is applicable only to
    /// Second Generation instances.
    
    pub name: Option<String>,
}

impl client::NestedType for DatabaseInstanceFailoverReplica {}
impl client::Part for DatabaseInstanceFailoverReplica {}


/// Options for exporting data as CSV.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExportContextCsvExportOptions {
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
    /// Tables to export, or that were exported, from the specified database. If
    /// you specify tables, specify one and only one database. For PostgreSQL
    /// instances, you can specify only one table.
    
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
    /// Option to include SQL statement required to set up replication.
    /// If set to <code>1</code>, the dump file includes
    ///  a CHANGE MASTER TO statement with the binary log coordinates.
    /// If set to <code>2</code>, the CHANGE MASTER TO statement is written as
    ///  a SQL comment, and has no effect.
    /// All other values are ignored.
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
    /// Path to the Certificate (.cer) in Cloud Storage, in the form
    /// <code>gs://bucketName/fileName</code>. The instance must have
    /// write permissions to the bucket and read access to the file.
    #[serde(rename="certPath")]
    
    pub cert_path: Option<String>,
    /// Password that encrypts the private key
    #[serde(rename="pvkPassword")]
    
    pub pvk_password: Option<String>,
    /// Path to the Certificate Private Key (.pvk)  in Cloud Storage, in the
    /// form <code>gs://bucketName/fileName</code>. The instance must have
    /// write permissions to the bucket and read access to the file.
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
    /// The columns to which CSV data is imported. If not specified, all columns
    /// of the database table are loaded with CSV data.
    
    pub columns: Option<Vec<String>>,
    /// The table to which CSV data is imported.
    
    pub table: Option<String>,
}

impl client::NestedType for ImportContextCsvImportOptions {}
impl client::Part for ImportContextCsvImportOptions {}


