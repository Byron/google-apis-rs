use super::*;
/// Specifies required connection parameters, and the parameters required to create an AlloyDB destination cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AlloyDbConnectionProfile {
    /// Required. The AlloyDB cluster ID that this connection profile is associated with.
    #[serde(rename="clusterId")]
    
    pub cluster_id: Option<String>,
    /// Immutable. Metadata used to create the destination AlloyDB cluster.
    
    pub settings: Option<AlloyDbSettings>,
}

impl client::Part for AlloyDbConnectionProfile {}


/// Settings for creating an AlloyDB cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AlloyDbSettings {
    /// Required. Input only. Initial user to setup during cluster creation. Required.
    #[serde(rename="initialUser")]
    
    pub initial_user: Option<UserPassword>,
    /// Labels for the AlloyDB cluster created by DMS. An object containing a list of 'key', 'value' pairs.
    
    pub labels: Option<HashMap<String, String>>,
    /// no description provided
    #[serde(rename="primaryInstanceSettings")]
    
    pub primary_instance_settings: Option<PrimaryInstanceSettings>,
    /// Required. The resource link for the VPC network in which cluster resources are created and from which they are accessible via Private IP. The network must belong to the same project as the cluster. It is specified in the form: "projects/{project_number}/global/networks/{network_id}". This is required to create a cluster.
    #[serde(rename="vpcNetwork")]
    
    pub vpc_network: Option<String>,
}

impl client::Part for AlloyDbSettings {}


/// Request message for ‘ApplyConversionWorkspace’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations conversion workspaces apply projects](ProjectLocationConversionWorkspaceApplyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApplyConversionWorkspaceRequest {
    /// Fully qualified (Uri) name of the destination connection profile.
    #[serde(rename="connectionProfile")]
    
    pub connection_profile: Option<String>,
    /// Filter which entities to apply. Leaving this field empty will apply all of the entities. Supports Google AIP 160 based filtering.
    
    pub filter: Option<String>,
}

impl client::RequestValue for ApplyConversionWorkspaceRequest {}


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


/// Execution log of a background job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BackgroundJobLogEntry {
    /// Job completion comment, such as how many entities were seeded, how many warnings were found during conversion and similar information.
    #[serde(rename="completionComment")]
    
    pub completion_comment: Option<String>,
    /// Job completion state, i.e. the final state after the job completed.
    #[serde(rename="completionState")]
    
    pub completion_state: Option<String>,
    /// The timestamp when the background job was finished.
    #[serde(rename="finishTime")]
    
    pub finish_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The background job log entry id
    
    pub id: Option<String>,
    /// Import rules job details
    #[serde(rename="importRulesJobDetails")]
    
    pub import_rules_job_details: Option<ImportRulesJobDetails>,
    /// The type of job that was executed.
    #[serde(rename="jobType")]
    
    pub job_type: Option<String>,
    /// Whether the client requested the conversion workspace to be committed after a successful completion of the job.
    #[serde(rename="requestAutocommit")]
    
    pub request_autocommit: Option<bool>,
    /// Seed job details
    #[serde(rename="seedJobDetails")]
    
    pub seed_job_details: Option<SeedJobDetails>,
    /// The timestamp when the background job was started.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for BackgroundJobLogEntry {}


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


/// Specifies required connection parameters, and, optionally, the parameters required to create a Cloud SQL destination database instance.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloudSqlConnectionProfile {
    /// Output only. The Cloud SQL database instance's additional (outgoing) public IP. Used when the Cloud SQL database availability type is REGIONAL (i.e. multiple zones / highly available).
    #[serde(rename="additionalPublicIp")]
    
    pub additional_public_ip: Option<String>,
    /// Output only. The Cloud SQL instance ID that this connection profile is associated with.
    #[serde(rename="cloudSqlId")]
    
    pub cloud_sql_id: Option<String>,
    /// Output only. The Cloud SQL database instance's private IP.
    #[serde(rename="privateIp")]
    
    pub private_ip: Option<String>,
    /// Output only. The Cloud SQL database instance's public IP.
    #[serde(rename="publicIp")]
    
    pub public_ip: Option<String>,
    /// Immutable. Metadata used to create the destination Cloud SQL database.
    
    pub settings: Option<CloudSqlSettings>,
}

impl client::Part for CloudSqlConnectionProfile {}


/// Settings for creating a Cloud SQL database instance.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloudSqlSettings {
    /// The activation policy specifies when the instance is activated; it is applicable only when the instance state is 'RUNNABLE'. Valid values: 'ALWAYS': The instance is on, and remains so even in the absence of connection requests. `NEVER`: The instance is off; it is not activated, even if a connection request arrives.
    #[serde(rename="activationPolicy")]
    
    pub activation_policy: Option<String>,
    /// [default: ON] If you enable this setting, Cloud SQL checks your available storage every 30 seconds. If the available storage falls below a threshold size, Cloud SQL automatically adds additional storage capacity. If the available storage repeatedly falls below the threshold size, Cloud SQL continues to add storage until it reaches the maximum of 30 TB.
    #[serde(rename="autoStorageIncrease")]
    
    pub auto_storage_increase: Option<bool>,
    /// Optional. Availability type. Potential values: * `ZONAL`: The instance serves data from only one zone. Outages in that zone affect data availability. * `REGIONAL`: The instance can serve data from more than one zone in a region (it is highly available).
    #[serde(rename="availabilityType")]
    
    pub availability_type: Option<String>,
    /// The KMS key name used for the csql instance.
    #[serde(rename="cmekKeyName")]
    
    pub cmek_key_name: Option<String>,
    /// The Cloud SQL default instance level collation.
    
    pub collation: Option<String>,
    /// The storage capacity available to the database, in GB. The minimum (and default) size is 10GB.
    #[serde(rename="dataDiskSizeGb")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub data_disk_size_gb: Option<i64>,
    /// The type of storage: `PD_SSD` (default) or `PD_HDD`.
    #[serde(rename="dataDiskType")]
    
    pub data_disk_type: Option<String>,
    /// The database flags passed to the Cloud SQL instance at startup. An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
    #[serde(rename="databaseFlags")]
    
    pub database_flags: Option<HashMap<String, String>>,
    /// The database engine type and version.
    #[serde(rename="databaseVersion")]
    
    pub database_version: Option<String>,
    /// The settings for IP Management. This allows to enable or disable the instance IP and manage which external networks can connect to the instance. The IPv4 address cannot be disabled.
    #[serde(rename="ipConfig")]
    
    pub ip_config: Option<SqlIpConfig>,
    /// Input only. Initial root password.
    #[serde(rename="rootPassword")]
    
    pub root_password: Option<String>,
    /// Output only. Indicates If this connection profile root password is stored.
    #[serde(rename="rootPasswordSet")]
    
    pub root_password_set: Option<bool>,
    /// Optional. The Google Cloud Platform zone where the failover Cloud SQL database instance is located. Used when the Cloud SQL database availability type is REGIONAL (i.e. multiple zones / highly available).
    #[serde(rename="secondaryZone")]
    
    pub secondary_zone: Option<String>,
    /// The Database Migration Service source connection profile ID, in the format: `projects/my_project_name/locations/us-central1/connectionProfiles/connection_profile_ID`
    #[serde(rename="sourceId")]
    
    pub source_id: Option<String>,
    /// The maximum size to which storage capacity can be automatically increased. The default value is 0, which specifies that there is no limit.
    #[serde(rename="storageAutoResizeLimit")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub storage_auto_resize_limit: Option<i64>,
    /// The tier (or machine type) for this instance, for example: `db-n1-standard-1` (MySQL instances) or `db-custom-1-3840` (PostgreSQL instances). For more information, see [Cloud SQL Instance Settings](https://cloud.google.com/sql/docs/mysql/instance-settings).
    
    pub tier: Option<String>,
    /// The resource labels for a Cloud SQL instance to use to annotate any related underlying resources such as Compute Engine VMs. An object containing a list of "key": "value" pairs. Example: `{ "name": "wrench", "mass": "18kg", "count": "3" }`.
    #[serde(rename="userLabels")]
    
    pub user_labels: Option<HashMap<String, String>>,
    /// The Google Cloud Platform zone where your Cloud SQL database instance is located.
    
    pub zone: Option<String>,
}

impl client::Part for CloudSqlSettings {}


/// Column is not used as an independent entity, it is retrieved as part of a Table entity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ColumnEntity {
    /// Is the column of array type
    
    pub array: Option<bool>,
    /// If the column is array, of which length
    #[serde(rename="arrayLength")]
    
    pub array_length: Option<i32>,
    /// Is the column auto-generated/identity
    #[serde(rename="autoGenerated")]
    
    pub auto_generated: Option<bool>,
    /// Charset override - instead of table level charset
    
    pub charset: Option<String>,
    /// Collation override - instead of table level collation
    
    pub collation: Option<String>,
    /// Comment associated with the column
    
    pub comment: Option<String>,
    /// Custom engine specific features
    #[serde(rename="customFeatures")]
    
    pub custom_features: Option<HashMap<String, json::Value>>,
    /// Column data type
    #[serde(rename="dataType")]
    
    pub data_type: Option<String>,
    /// Default value of the column
    #[serde(rename="defaultValue")]
    
    pub default_value: Option<String>,
    /// Column fractional second precision - used for timestamp based datatypes
    #[serde(rename="fractionalSecondsPrecision")]
    
    pub fractional_seconds_precision: Option<i32>,
    /// Column length - e.g. varchar (50)
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub length: Option<i64>,
    /// Column name
    
    pub name: Option<String>,
    /// Is the column nullable
    
    pub nullable: Option<bool>,
    /// Column order in the table
    #[serde(rename="ordinalPosition")]
    
    pub ordinal_position: Option<i32>,
    /// Column precision - when relevant
    
    pub precision: Option<i32>,
    /// Column scale - when relevant
    
    pub scale: Option<i32>,
    /// Specifies the list of values allowed in the column. List is empty if set values is not required
    #[serde(rename="setValues")]
    
    pub set_values: Option<Vec<String>>,
    /// Is the column a UDT
    
    pub udt: Option<bool>,
}

impl client::Part for ColumnEntity {}


/// Request message for ‘CommitConversionWorkspace’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations conversion workspaces commit projects](ProjectLocationConversionWorkspaceCommitCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommitConversionWorkspaceRequest {
    /// Optional. Optional name of the commit.
    #[serde(rename="commitName")]
    
    pub commit_name: Option<String>,
}

impl client::RequestValue for CommitConversionWorkspaceRequest {}


/// A connection profile definition.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations connection profiles create projects](ProjectLocationConnectionProfileCreateCall) (request)
/// * [locations connection profiles get projects](ProjectLocationConnectionProfileGetCall) (response)
/// * [locations connection profiles patch projects](ProjectLocationConnectionProfilePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConnectionProfile {
    /// An AlloyDB cluster connection profile.
    
    pub alloydb: Option<AlloyDbConnectionProfile>,
    /// A CloudSQL database connection profile.
    
    pub cloudsql: Option<CloudSqlConnectionProfile>,
    /// Output only. The timestamp when the resource was created. A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z".
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The connection profile display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. The error details in case of state FAILED.
    
    pub error: Option<Status>,
    /// The resource labels for connection profile to use to annotate any related underlying resources such as Compute Engine VMs. An object containing a list of "key": "value" pairs. Example: `{ "name": "wrench", "mass": "1.3kg", "count": "3" }`.
    
    pub labels: Option<HashMap<String, String>>,
    /// A MySQL database connection profile.
    
    pub mysql: Option<MySqlConnectionProfile>,
    /// The name of this connection profile resource in the form of projects/{project}/locations/{location}/connectionProfiles/{connectionProfile}.
    
    pub name: Option<String>,
    /// An Oracle database connection profile.
    
    pub oracle: Option<OracleConnectionProfile>,
    /// A PostgreSQL database connection profile.
    
    pub postgresql: Option<PostgreSqlConnectionProfile>,
    /// The database provider.
    
    pub provider: Option<String>,
    /// The current connection profile state (e.g. DRAFT, READY, or FAILED).
    
    pub state: Option<String>,
    /// Output only. The timestamp when the resource was last updated. A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z".
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for ConnectionProfile {}
impl client::ResponseResult for ConnectionProfile {}


/// Constraint is not used as an independent entity, it is retrieved as part of another entity such as Table or View.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConstraintEntity {
    /// Custom engine specific features
    #[serde(rename="customFeatures")]
    
    pub custom_features: Option<HashMap<String, json::Value>>,
    /// The name of the table constraint
    
    pub name: Option<String>,
    /// Reference Columns which may be associated with the constraint. eg: if the constraint is a FOREIGN_KEY, this represents the list of full names of referenced columns by the foreign key.
    #[serde(rename="referenceColumns")]
    
    pub reference_columns: Option<Vec<String>>,
    /// Reference table which may be associated with the constraint. eg: if the constraint is a FOREIGN_KEY, this represents the list of full name of the referenced table by the foreign key.
    #[serde(rename="referenceTable")]
    
    pub reference_table: Option<String>,
    /// Table columns used as part of the Constraint for e.g. primary key constraint should list the columns which constitutes the key
    #[serde(rename="tableColumns")]
    
    pub table_columns: Option<Vec<String>>,
    /// Table which is associated with the constraint. In case the constraint is defined on a table, this field is left empty as this information is stored in parent_name. However, if constraint is defined on a view, this field stores the table name on which the view is defined.
    #[serde(rename="tableName")]
    
    pub table_name: Option<String>,
    /// Type of constraint - e.g. unique, primary key, foreign key (currently only primary key is supported)
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for ConstraintEntity {}


/// The main conversion workspace resource entity.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations conversion workspaces create projects](ProjectLocationConversionWorkspaceCreateCall) (request)
/// * [locations conversion workspaces get projects](ProjectLocationConversionWorkspaceGetCall) (response)
/// * [locations conversion workspaces patch projects](ProjectLocationConversionWorkspacePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConversionWorkspace {
    /// Output only. The timestamp when the workspace resource was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. The destination engine details.
    
    pub destination: Option<DatabaseEngineInfo>,
    /// The display name for the workspace
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// A generic list of settings for the workspace. The settings are database pair dependant and can indicate default behavior for the mapping rules engine or turn on or off specific features. Such examples can be: convert_foreign_key_to_interleave=true, skip_triggers=false, ignore_non_table_synonyms=true
    #[serde(rename="globalSettings")]
    
    pub global_settings: Option<HashMap<String, String>>,
    /// Output only. Whether the workspace has uncommitted changes (changes which were made after the workspace was committed)
    #[serde(rename="hasUncommittedChanges")]
    
    pub has_uncommitted_changes: Option<bool>,
    /// Output only. The latest commit id
    #[serde(rename="latestCommitId")]
    
    pub latest_commit_id: Option<String>,
    /// Output only. The timestamp when the workspace was committed.
    #[serde(rename="latestCommitTime")]
    
    pub latest_commit_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Full name of the workspace resource, in the form of: projects/{project}/locations/{location}/conversionWorkspaces/{conversion_workspace}.
    
    pub name: Option<String>,
    /// Required. The source engine details.
    
    pub source: Option<DatabaseEngineInfo>,
    /// Output only. The timestamp when the workspace resource was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for ConversionWorkspace {}
impl client::ResponseResult for ConversionWorkspace {}


/// A conversion workspace's version.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConversionWorkspaceInfo {
    /// The commit ID of the conversion workspace.
    #[serde(rename="commitId")]
    
    pub commit_id: Option<String>,
    /// The resource name (URI) of the conversion workspace.
    
    pub name: Option<String>,
}

impl client::Part for ConversionWorkspaceInfo {}


/// Request message for ‘ConvertConversionWorkspace’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations conversion workspaces convert projects](ProjectLocationConversionWorkspaceConvertCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConvertConversionWorkspaceRequest {
    /// Should the conversion workspace be committed automatically after the conversion.
    #[serde(rename="autoCommit")]
    
    pub auto_commit: Option<bool>,
    /// Filter the entities to convert. Leaving this field empty will convert all of the entities. Supports Google AIP-160 style filtering.
    
    pub filter: Option<String>,
}

impl client::RequestValue for ConvertConversionWorkspaceRequest {}


/// The type and version of a source or destination DB.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatabaseEngineInfo {
    /// Required. Engine Type.
    
    pub engine: Option<String>,
    /// Required. Engine named version, for e.g. 12.c.1
    
    pub version: Option<String>,
}

impl client::Part for DatabaseEngineInfo {}


/// The base entity type for all the database related entities The message contains the entity name, the name of its parent, its type and the specific details per its type
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatabaseEntity {
    /// Function
    #[serde(rename="databaseFunction")]
    
    pub database_function: Option<FunctionEntity>,
    /// Package
    #[serde(rename="databasePackage")]
    
    pub database_package: Option<PackageEntity>,
    /// The type of the database entity (table, view, index, ...).
    #[serde(rename="entityType")]
    
    pub entity_type: Option<String>,
    /// Details about entity mappings. For source tree entities, this holds the draft entities which were generated by the mapping rules. For draft tree entities, this holds the source entities which were converted to form the draft entity. Destination entities will have no mapping details.
    
    pub mappings: Option<Vec<EntityMapping>>,
    /// The full name of the parent entity (e.g. schema name).
    #[serde(rename="parentEntity")]
    
    pub parent_entity: Option<String>,
    /// Schema.
    
    pub schema: Option<SchemaEntity>,
    /// Sequence
    
    pub sequence: Option<SequenceEntity>,
    /// The short name (e.g. table name) of the entity.
    #[serde(rename="shortName")]
    
    pub short_name: Option<String>,
    /// Stored Procedure
    #[serde(rename="storedProcedure")]
    
    pub stored_procedure: Option<StoredProcedureEntity>,
    /// Synonym
    
    pub synonym: Option<SynonymEntity>,
    /// Table.
    
    pub table: Option<TableEntity>,
    /// The type of tree the entity belongs to.
    
    pub tree: Option<String>,
    /// View
    
    pub view: Option<ViewEntity>,
}

impl client::Part for DatabaseEntity {}


/// A message defining the database engine and provider.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatabaseType {
    /// The database engine.
    
    pub engine: Option<String>,
    /// The database provider.
    
    pub provider: Option<String>,
}

impl client::Part for DatabaseType {}


/// Response message for ‘DescribeConversionWorkspaceRevisions’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations conversion workspaces describe conversion workspace revisions projects](ProjectLocationConversionWorkspaceDescribeConversionWorkspaceRevisionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DescribeConversionWorkspaceRevisionsResponse {
    /// The list of conversion workspace revisions.
    
    pub revisions: Option<Vec<ConversionWorkspace>>,
}

impl client::ResponseResult for DescribeConversionWorkspaceRevisionsResponse {}


/// Response message for ‘DescribeDatabaseEntities’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations conversion workspaces describe database entities projects](ProjectLocationConversionWorkspaceDescribeDatabaseEntityCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DescribeDatabaseEntitiesResponse {
    /// The list of database entities for the conversion workspace.
    #[serde(rename="databaseEntities")]
    
    pub database_entities: Option<Vec<DatabaseEntity>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for DescribeDatabaseEntitiesResponse {}


/// Dump flag definition.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DumpFlag {
    /// The name of the flag
    
    pub name: Option<String>,
    /// The value of the flag.
    
    pub value: Option<String>,
}

impl client::Part for DumpFlag {}


/// Dump flags definition.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DumpFlags {
    /// The flags for the initial dump.
    #[serde(rename="dumpFlags")]
    
    pub dump_flags: Option<Vec<DumpFlag>>,
}

impl client::Part for DumpFlags {}


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


/// Details of the mappings of a database entity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EntityMapping {
    /// Target entity full name. The draft entity can also include a column, index or constraint using the same naming notation schema.table.column
    #[serde(rename="draftEntity")]
    
    pub draft_entity: Option<String>,
    /// Entity mapping log entries. Multiple rules can be effective and contribute changes to a converted entity such as, a rule can handle the entity name, another rule can handle an entity type. In addition, rules which did not change the entity are also logged along the with the reason preventing them to do so.
    #[serde(rename="mappingLog")]
    
    pub mapping_log: Option<Vec<EntityMappingLogEntry>>,
    /// Source entity full name. The source entity can also be a column, index or constraint using the same naming notation schema.table.column
    #[serde(rename="sourceEntity")]
    
    pub source_entity: Option<String>,
}

impl client::Part for EntityMapping {}


/// A single record of a rule which was used for a mapping.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EntityMappingLogEntry {
    /// Comment.
    #[serde(rename="mappingComment")]
    
    pub mapping_comment: Option<String>,
    /// Which rule caused it.
    #[serde(rename="ruleId")]
    
    pub rule_id: Option<String>,
    /// Rule revision id
    #[serde(rename="ruleRevisionId")]
    
    pub rule_revision_id: Option<String>,
}

impl client::Part for EntityMappingLogEntry {}


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


/// Forward SSH Tunnel connectivity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ForwardSshTunnelConnectivity {
    /// Required. Hostname for the SSH tunnel.
    
    pub hostname: Option<String>,
    /// Input only. SSH password.
    
    pub password: Option<String>,
    /// Port for the SSH tunnel, default value is 22.
    
    pub port: Option<i32>,
    /// Input only. SSH private key.
    #[serde(rename="privateKey")]
    
    pub private_key: Option<String>,
    /// Required. Username for the SSH tunnel.
    
    pub username: Option<String>,
}

impl client::Part for ForwardSshTunnelConnectivity {}


/// Function's parent is a schema.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FunctionEntity {
    /// Custom engine specific features
    #[serde(rename="customFeatures")]
    
    pub custom_features: Option<HashMap<String, json::Value>>,
    /// The SQL code which creates the function
    #[serde(rename="sqlCode")]
    
    pub sql_code: Option<String>,
}

impl client::Part for FunctionEntity {}


/// Request message for ‘GenerateSshScript’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations migration jobs generate ssh script projects](ProjectLocationMigrationJobGenerateSshScriptCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GenerateSshScriptRequest {
    /// Required. Bastion VM Instance name to use or to create.
    
    pub vm: Option<String>,
    /// The VM creation configuration
    #[serde(rename="vmCreationConfig")]
    
    pub vm_creation_config: Option<VmCreationConfig>,
    /// The port that will be open on the bastion host
    #[serde(rename="vmPort")]
    
    pub vm_port: Option<i32>,
    /// The VM selection configuration
    #[serde(rename="vmSelectionConfig")]
    
    pub vm_selection_config: Option<VmSelectionConfig>,
}

impl client::RequestValue for GenerateSshScriptRequest {}


/// Request message for ‘ImportMappingRules’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations conversion workspaces mapping rules import projects](ProjectLocationConversionWorkspaceMappingRuleImportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImportMappingRulesRequest {
    /// Should the conversion workspace be committed automatically after the import operation.
    #[serde(rename="autoCommit")]
    
    pub auto_commit: Option<bool>,
    /// One or more rules files
    #[serde(rename="rulesFiles")]
    
    pub rules_files: Option<Vec<RulesFile>>,
    /// The format of the rules content file.
    #[serde(rename="rulesFormat")]
    
    pub rules_format: Option<String>,
}

impl client::RequestValue for ImportMappingRulesRequest {}


/// Details regarding an Import Rules background job
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImportRulesJobDetails {
    /// The requested file format
    #[serde(rename="fileFormat")]
    
    pub file_format: Option<String>,
    /// File names used for the import rules job
    
    pub files: Option<Vec<String>>,
}

impl client::Part for ImportRulesJobDetails {}


/// Index is not used as an independent entity, it is retrieved as part of a Table entity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IndexEntity {
    /// Custom engine specific features
    #[serde(rename="customFeatures")]
    
    pub custom_features: Option<HashMap<String, json::Value>>,
    /// The name of the index
    
    pub name: Option<String>,
    /// Table columns used as part of the Index for e.g. B-TREE index should list the columns which constitutes the index.
    #[serde(rename="tableColumns")]
    
    pub table_columns: Option<Vec<String>>,
    /// Type of index - e.g. B-TREE
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// boolean value indicating whether the index is unique
    
    pub unique: Option<bool>,
}

impl client::Part for IndexEntity {}


/// Response message for ‘ListConnectionProfiles’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations connection profiles list projects](ProjectLocationConnectionProfileListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListConnectionProfilesResponse {
    /// The response list of connection profiles.
    #[serde(rename="connectionProfiles")]
    
    pub connection_profiles: Option<Vec<ConnectionProfile>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListConnectionProfilesResponse {}


/// Response message for ‘ListConversionWorkspaces’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations conversion workspaces list projects](ProjectLocationConversionWorkspaceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListConversionWorkspacesResponse {
    /// The list of conversion workspace objects.
    #[serde(rename="conversionWorkspaces")]
    
    pub conversion_workspaces: Option<Vec<ConversionWorkspace>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListConversionWorkspacesResponse {}


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


/// Response message for ‘ListMigrationJobs’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations migration jobs list projects](ProjectLocationMigrationJobListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListMigrationJobsResponse {
    /// The list of migration jobs objects.
    #[serde(rename="migrationJobs")]
    
    pub migration_jobs: Option<Vec<MigrationJob>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListMigrationJobsResponse {}


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


/// Response message for ‘ListPrivateConnections’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations private connections list projects](ProjectLocationPrivateConnectionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListPrivateConnectionsResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of private connections.
    #[serde(rename="privateConnections")]
    
    pub private_connections: Option<Vec<PrivateConnection>>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListPrivateConnectionsResponse {}


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


/// MachineConfig describes the configuration of a machine.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MachineConfig {
    /// The number of CPU's in the VM instance.
    #[serde(rename="cpuCount")]
    
    pub cpu_count: Option<i32>,
}

impl client::Part for MachineConfig {}


/// Represents a Database Migration Service migration job object.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations migration jobs create projects](ProjectLocationMigrationJobCreateCall) (request)
/// * [locations migration jobs get projects](ProjectLocationMigrationJobGetCall) (response)
/// * [locations migration jobs patch projects](ProjectLocationMigrationJobPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MigrationJob {
    /// The conversion workspace used by the migration.
    #[serde(rename="conversionWorkspace")]
    
    pub conversion_workspace: Option<ConversionWorkspaceInfo>,
    /// Output only. The timestamp when the migration job resource was created. A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z".
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. The resource name (URI) of the destination connection profile.
    
    pub destination: Option<String>,
    /// The database engine type and provider of the destination.
    #[serde(rename="destinationDatabase")]
    
    pub destination_database: Option<DatabaseType>,
    /// The migration job display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The initial dump flags. This field and the "dump_path" field are mutually exclusive.
    #[serde(rename="dumpFlags")]
    
    pub dump_flags: Option<DumpFlags>,
    /// The path to the dump file in Google Cloud Storage, in the format: (gs://[BUCKET_NAME]/[OBJECT_NAME]). This field and the "dump_flags" field are mutually exclusive.
    #[serde(rename="dumpPath")]
    
    pub dump_path: Option<String>,
    /// Output only. The duration of the migration job (in seconds). A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s".
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub duration: Option<client::chrono::Duration>,
    /// Output only. If the migration job is completed, the time when it was completed.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The error details in case of state FAILED.
    
    pub error: Option<Status>,
    /// This field can be used to select the entities to migrate as part of the migration job. It uses AIP-160 notation to select a subset of the entities configured on the associated conversion-workspace. This field should not be set on migration-jobs that are not associated with a conversion workspace.
    
    pub filter: Option<String>,
    /// The resource labels for migration job to use to annotate any related underlying resources such as Compute Engine VMs. An object containing a list of "key": "value" pairs. Example: `{ "name": "wrench", "mass": "1.3kg", "count": "3" }`.
    
    pub labels: Option<HashMap<String, String>>,
    /// The name (URI) of this migration job resource, in the form of: projects/{project}/locations/{location}/migrationJobs/{migrationJob}.
    
    pub name: Option<String>,
    /// Output only. The current migration job phase.
    
    pub phase: Option<String>,
    /// The details needed to communicate to the source over Reverse SSH tunnel connectivity.
    #[serde(rename="reverseSshConnectivity")]
    
    pub reverse_ssh_connectivity: Option<ReverseSshConnectivity>,
    /// Required. The resource name (URI) of the source connection profile.
    
    pub source: Option<String>,
    /// The database engine type and provider of the source.
    #[serde(rename="sourceDatabase")]
    
    pub source_database: Option<DatabaseType>,
    /// The current migration job state.
    
    pub state: Option<String>,
    /// static ip connectivity data (default, no additional details needed).
    #[serde(rename="staticIpConnectivity")]
    
    pub static_ip_connectivity: Option<StaticIpConnectivity>,
    /// Required. The migration job type.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Output only. The timestamp when the migration job resource was last updated. A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z".
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The details of the VPC network that the source database is located in.
    #[serde(rename="vpcPeeringConnectivity")]
    
    pub vpc_peering_connectivity: Option<VpcPeeringConnectivity>,
}

impl client::RequestValue for MigrationJob {}
impl client::ResponseResult for MigrationJob {}


/// Specifies connection parameters required specifically for MySQL databases.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MySqlConnectionProfile {
    /// If the source is a Cloud SQL database, use this field to provide the Cloud SQL instance ID of the source.
    #[serde(rename="cloudSqlId")]
    
    pub cloud_sql_id: Option<String>,
    /// Required. The IP or hostname of the source MySQL database.
    
    pub host: Option<String>,
    /// Required. Input only. The password for the user that Database Migration Service will be using to connect to the database. This field is not returned on request, and the value is encrypted when stored in Database Migration Service.
    
    pub password: Option<String>,
    /// Output only. Indicates If this connection profile password is stored.
    #[serde(rename="passwordSet")]
    
    pub password_set: Option<bool>,
    /// Required. The network port of the source MySQL database.
    
    pub port: Option<i32>,
    /// SSL configuration for the destination to connect to the source database.
    
    pub ssl: Option<SslConfig>,
    /// Required. The username that Database Migration Service will use to connect to the database. The value is encrypted when stored in Database Migration Service.
    
    pub username: Option<String>,
}

impl client::Part for MySqlConnectionProfile {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations connection profiles create projects](ProjectLocationConnectionProfileCreateCall) (response)
/// * [locations connection profiles delete projects](ProjectLocationConnectionProfileDeleteCall) (response)
/// * [locations connection profiles patch projects](ProjectLocationConnectionProfilePatchCall) (response)
/// * [locations conversion workspaces mapping rules import projects](ProjectLocationConversionWorkspaceMappingRuleImportCall) (response)
/// * [locations conversion workspaces apply projects](ProjectLocationConversionWorkspaceApplyCall) (response)
/// * [locations conversion workspaces commit projects](ProjectLocationConversionWorkspaceCommitCall) (response)
/// * [locations conversion workspaces convert projects](ProjectLocationConversionWorkspaceConvertCall) (response)
/// * [locations conversion workspaces create projects](ProjectLocationConversionWorkspaceCreateCall) (response)
/// * [locations conversion workspaces delete projects](ProjectLocationConversionWorkspaceDeleteCall) (response)
/// * [locations conversion workspaces patch projects](ProjectLocationConversionWorkspacePatchCall) (response)
/// * [locations conversion workspaces rollback projects](ProjectLocationConversionWorkspaceRollbackCall) (response)
/// * [locations conversion workspaces seed projects](ProjectLocationConversionWorkspaceSeedCall) (response)
/// * [locations migration jobs create projects](ProjectLocationMigrationJobCreateCall) (response)
/// * [locations migration jobs delete projects](ProjectLocationMigrationJobDeleteCall) (response)
/// * [locations migration jobs patch projects](ProjectLocationMigrationJobPatchCall) (response)
/// * [locations migration jobs promote projects](ProjectLocationMigrationJobPromoteCall) (response)
/// * [locations migration jobs restart projects](ProjectLocationMigrationJobRestartCall) (response)
/// * [locations migration jobs resume projects](ProjectLocationMigrationJobResumeCall) (response)
/// * [locations migration jobs start projects](ProjectLocationMigrationJobStartCall) (response)
/// * [locations migration jobs stop projects](ProjectLocationMigrationJobStopCall) (response)
/// * [locations migration jobs verify projects](ProjectLocationMigrationJobVerifyCall) (response)
/// * [locations operations get projects](ProjectLocationOperationGetCall) (response)
/// * [locations private connections create projects](ProjectLocationPrivateConnectionCreateCall) (response)
/// * [locations private connections delete projects](ProjectLocationPrivateConnectionDeleteCall) (response)
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


/// Specifies connection parameters required specifically for Oracle databases.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OracleConnectionProfile {
    /// Required. Database service for the Oracle connection.
    #[serde(rename="databaseService")]
    
    pub database_service: Option<String>,
    /// Forward SSH tunnel connectivity.
    #[serde(rename="forwardSshConnectivity")]
    
    pub forward_ssh_connectivity: Option<ForwardSshTunnelConnectivity>,
    /// Required. The IP or hostname of the source Oracle database.
    
    pub host: Option<String>,
    /// Required. Input only. The password for the user that Database Migration Service will be using to connect to the database. This field is not returned on request, and the value is encrypted when stored in Database Migration Service.
    
    pub password: Option<String>,
    /// Output only. Indicates whether a new password is included in the request.
    #[serde(rename="passwordSet")]
    
    pub password_set: Option<bool>,
    /// Required. The network port of the source Oracle database.
    
    pub port: Option<i32>,
    /// Private connectivity.
    #[serde(rename="privateConnectivity")]
    
    pub private_connectivity: Option<PrivateConnectivity>,
    /// Static Service IP connectivity.
    #[serde(rename="staticServiceIpConnectivity")]
    
    pub static_service_ip_connectivity: Option<StaticServiceIpConnectivity>,
    /// Required. The username that Database Migration Service will use to connect to the database. The value is encrypted when stored in Database Migration Service.
    
    pub username: Option<String>,
}

impl client::Part for OracleConnectionProfile {}


/// Package's parent is a schema.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PackageEntity {
    /// Custom engine specific features
    #[serde(rename="customFeatures")]
    
    pub custom_features: Option<HashMap<String, json::Value>>,
    /// The SQL code which creates the package body. If the package specification has cursors or subprograms, then the package body is mandatory.
    #[serde(rename="packageBody")]
    
    pub package_body: Option<String>,
    /// The SQL code which creates the package
    #[serde(rename="packageSqlCode")]
    
    pub package_sql_code: Option<String>,
}

impl client::Part for PackageEntity {}


/// An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members`, or principals, to a single `role`. Principals can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { “bindings”: \[ { “role”: “roles/resourcemanager.organizationAdmin”, “members”: \[ “user:mike@example.com”, “group:admins@example.com”, “domain:google.com”, “serviceAccount:my-project-id@appspot.gserviceaccount.com” \] }, { “role”: “roles/resourcemanager.organizationViewer”, “members”: \[ “user:eve@example.com” \], “condition”: { “title”: “expirable access”, “description”: “Does not grant access after Sep 2020”, “expression”: “request.time \< timestamp(‘2020-10-01T00:00:00.000Z’)”, } } \], “etag”: “BwWWja0YfJA=”, “version”: 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time \< timestamp(‘2020-10-01T00:00:00.000Z’) etag: BwWWja0YfJA= version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations connection profiles get iam policy projects](ProjectLocationConnectionProfileGetIamPolicyCall) (response)
/// * [locations connection profiles set iam policy projects](ProjectLocationConnectionProfileSetIamPolicyCall) (response)
/// * [locations migration jobs get iam policy projects](ProjectLocationMigrationJobGetIamPolicyCall) (response)
/// * [locations migration jobs set iam policy projects](ProjectLocationMigrationJobSetIamPolicyCall) (response)
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


/// Specifies connection parameters required specifically for PostgreSQL databases.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PostgreSqlConnectionProfile {
    /// If the source is a Cloud SQL database, use this field to provide the Cloud SQL instance ID of the source.
    #[serde(rename="cloudSqlId")]
    
    pub cloud_sql_id: Option<String>,
    /// Required. The IP or hostname of the source PostgreSQL database.
    
    pub host: Option<String>,
    /// Output only. If the source is a Cloud SQL database, this field indicates the network architecture it's associated with.
    #[serde(rename="networkArchitecture")]
    
    pub network_architecture: Option<String>,
    /// Required. Input only. The password for the user that Database Migration Service will be using to connect to the database. This field is not returned on request, and the value is encrypted when stored in Database Migration Service.
    
    pub password: Option<String>,
    /// Output only. Indicates If this connection profile password is stored.
    #[serde(rename="passwordSet")]
    
    pub password_set: Option<bool>,
    /// Required. The network port of the source PostgreSQL database.
    
    pub port: Option<i32>,
    /// Private service connect connectivity.
    #[serde(rename="privateServiceConnectConnectivity")]
    
    pub private_service_connect_connectivity: Option<PrivateServiceConnectConnectivity>,
    /// SSL configuration for the destination to connect to the source database.
    
    pub ssl: Option<SslConfig>,
    /// Static ip connectivity data (default, no additional details needed).
    #[serde(rename="staticIpConnectivity")]
    
    pub static_ip_connectivity: Option<StaticIpConnectivity>,
    /// Required. The username that Database Migration Service will use to connect to the database. The value is encrypted when stored in Database Migration Service.
    
    pub username: Option<String>,
}

impl client::Part for PostgreSqlConnectionProfile {}


/// Settings for the cluster's primary instance
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PrimaryInstanceSettings {
    /// Database flags to pass to AlloyDB when DMS is creating the AlloyDB cluster and instances. See the AlloyDB documentation for how these can be used.
    #[serde(rename="databaseFlags")]
    
    pub database_flags: Option<HashMap<String, String>>,
    /// Required. The ID of the AlloyDB primary instance. The ID must satisfy the regex expression "[a-z0-9-]+".
    
    pub id: Option<String>,
    /// Labels for the AlloyDB primary instance created by DMS. An object containing a list of 'key', 'value' pairs.
    
    pub labels: Option<HashMap<String, String>>,
    /// Configuration for the machines that host the underlying database engine.
    #[serde(rename="machineConfig")]
    
    pub machine_config: Option<MachineConfig>,
    /// Output only. The private IP address for the Instance. This is the connection endpoint for an end-user application.
    #[serde(rename="privateIp")]
    
    pub private_ip: Option<String>,
}

impl client::Part for PrimaryInstanceSettings {}


/// The PrivateConnection resource is used to establish private connectivity with the customer’s network.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations private connections create projects](ProjectLocationPrivateConnectionCreateCall) (request)
/// * [locations private connections get projects](ProjectLocationPrivateConnectionGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PrivateConnection {
    /// Output only. The create time of the resource.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The private connection display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. The error details in case of state FAILED.
    
    pub error: Option<Status>,
    /// The resource labels for private connections to use to annotate any related underlying resources such as Compute Engine VMs. An object containing a list of "key": "value" pairs. Example: `{ "name": "wrench", "mass": "1.3kg", "count": "3" }`.
    
    pub labels: Option<HashMap<String, String>>,
    /// The resource's name.
    
    pub name: Option<String>,
    /// Output only. The state of the Private Connection.
    
    pub state: Option<String>,
    /// Output only. The last update time of the resource.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// VPC Peering Config.
    #[serde(rename="vpcPeeringConfig")]
    
    pub vpc_peering_config: Option<VpcPeeringConfig>,
}

impl client::RequestValue for PrivateConnection {}
impl client::ResponseResult for PrivateConnection {}


/// Private Connectivity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PrivateConnectivity {
    /// Required. The resource name (URI) of the private connection.
    #[serde(rename="privateConnection")]
    
    pub private_connection: Option<String>,
}

impl client::Part for PrivateConnectivity {}


/// Private Service Connect connectivity (https://cloud.google.com/vpc/docs/private-service-connect#benefits-services)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PrivateServiceConnectConnectivity {
    /// Required. A service attachment that exposes a database, and has the following format: projects/{project}/regions/{region}/serviceAttachments/{service_attachment_name}
    #[serde(rename="serviceAttachment")]
    
    pub service_attachment: Option<String>,
}

impl client::Part for PrivateServiceConnectConnectivity {}


/// Request message for ‘PromoteMigrationJob’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations migration jobs promote projects](ProjectLocationMigrationJobPromoteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PromoteMigrationJobRequest { _never_set: Option<bool> }

impl client::RequestValue for PromoteMigrationJobRequest {}


/// Request message for ‘RestartMigrationJob’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations migration jobs restart projects](ProjectLocationMigrationJobRestartCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RestartMigrationJobRequest { _never_set: Option<bool> }

impl client::RequestValue for RestartMigrationJobRequest {}


/// Request message for ‘ResumeMigrationJob’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations migration jobs resume projects](ProjectLocationMigrationJobResumeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResumeMigrationJobRequest { _never_set: Option<bool> }

impl client::RequestValue for ResumeMigrationJobRequest {}


/// The details needed to configure a reverse SSH tunnel between the source and destination databases. These details will be used when calling the generateSshScript method (see https://cloud.google.com/database-migration/docs/reference/rest/v1/projects.locations.migrationJobs/generateSshScript) to produce the script that will help set up the reverse SSH tunnel, and to set up the VPC peering between the Cloud SQL private network and the VPC.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReverseSshConnectivity {
    /// The name of the virtual machine (Compute Engine) used as the bastion server for the SSH tunnel.
    
    pub vm: Option<String>,
    /// Required. The IP of the virtual machine (Compute Engine) used as the bastion server for the SSH tunnel.
    #[serde(rename="vmIp")]
    
    pub vm_ip: Option<String>,
    /// Required. The forwarding port of the virtual machine (Compute Engine) used as the bastion server for the SSH tunnel.
    #[serde(rename="vmPort")]
    
    pub vm_port: Option<i32>,
    /// The name of the VPC to peer with the Cloud SQL private network.
    
    pub vpc: Option<String>,
}

impl client::Part for ReverseSshConnectivity {}


/// Request message for ‘RollbackConversionWorkspace’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations conversion workspaces rollback projects](ProjectLocationConversionWorkspaceRollbackCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RollbackConversionWorkspaceRequest { _never_set: Option<bool> }

impl client::RequestValue for RollbackConversionWorkspaceRequest {}


/// Details of a single rules file
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RulesFile {
    /// The text content of the rules that needs to be converted
    #[serde(rename="rulesContent")]
    
    pub rules_content: Option<String>,
    /// The filename of the rules that needs to be converted. This is used mainly so future logs of the import rules job will contain this detail and can therefore be searched by it later
    #[serde(rename="rulesSourceFilename")]
    
    pub rules_source_filename: Option<String>,
}

impl client::Part for RulesFile {}


/// Schema typically has no parent entity, but can have a parent entity DatabaseInstance (for database engines which supports it). For some database engines the term schema and user can be used interchangeably when they refer to a namespace or a collection of other database entities. Can store additional information which is schema specific.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SchemaEntity {
    /// Custom engine specific features
    #[serde(rename="customFeatures")]
    
    pub custom_features: Option<HashMap<String, json::Value>>,
}

impl client::Part for SchemaEntity {}


/// Response message for ‘SearchBackgroundJobs’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations conversion workspaces search background jobs projects](ProjectLocationConversionWorkspaceSearchBackgroundJobCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchBackgroundJobsResponse {
    /// The list of conversion workspace mapping rules.
    
    pub jobs: Option<Vec<BackgroundJobLogEntry>>,
}

impl client::ResponseResult for SearchBackgroundJobsResponse {}


/// Request message for ‘SeedConversionWorkspace’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations conversion workspaces seed projects](ProjectLocationConversionWorkspaceSeedCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SeedConversionWorkspaceRequest {
    /// Should the conversion workspace be committed automatically after the seed operation.
    #[serde(rename="autoCommit")]
    
    pub auto_commit: Option<bool>,
    /// Fully qualified (Uri) name of the destination connection profile.
    #[serde(rename="destinationConnectionProfile")]
    
    pub destination_connection_profile: Option<String>,
    /// Fully qualified (Uri) name of the source connection profile.
    #[serde(rename="sourceConnectionProfile")]
    
    pub source_connection_profile: Option<String>,
}

impl client::RequestValue for SeedConversionWorkspaceRequest {}


/// Details regarding a Seed background job
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SeedJobDetails {
    /// The connection profile which was used for the seed job
    #[serde(rename="connectionProfile")]
    
    pub connection_profile: Option<String>,
}

impl client::Part for SeedJobDetails {}


/// Sequence's parent is a schema.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SequenceEntity {
    /// Indicates number of entries to cache / precreate
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub cache: Option<i64>,
    /// Custom engine specific features
    #[serde(rename="customFeatures")]
    
    pub custom_features: Option<HashMap<String, json::Value>>,
    /// Indicates whether the sequence value should cycle through
    
    pub cycle: Option<bool>,
    /// Increment value for the sequence
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub increment: Option<i64>,
    /// Maximum number for the sequence represented as bytes to accommodate large numbers
    #[serde(rename="maxValue")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub max_value: Option<Vec<u8>>,
    /// Minimum number for the sequence represented as bytes to accommodate large numbers
    #[serde(rename="minValue")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub min_value: Option<Vec<u8>>,
    /// Start number for the sequence represented as bytes to accommodate large numbers
    #[serde(rename="startValue")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub start_value: Option<Vec<u8>>,
}

impl client::Part for SequenceEntity {}


/// Request message for `SetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations connection profiles set iam policy projects](ProjectLocationConnectionProfileSetIamPolicyCall) (request)
/// * [locations migration jobs set iam policy projects](ProjectLocationMigrationJobSetIamPolicyCall) (request)
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


/// An entry for an Access Control list.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SqlAclEntry {
    /// The time when this access control entry expires in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example: `2012-11-15T16:19:00.094Z`.
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A label to identify this entry.
    
    pub label: Option<String>,
    /// Input only. The time-to-leave of this access control entry.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub ttl: Option<client::chrono::Duration>,
    /// The allowlisted value for the access control list.
    
    pub value: Option<String>,
}

impl client::Part for SqlAclEntry {}


/// IP Management configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SqlIpConfig {
    /// The list of external networks that are allowed to connect to the instance using the IP. See https://en.wikipedia.org/wiki/CIDR_notation#CIDR_notation, also known as 'slash' notation (e.g. `192.168.100.0/24`).
    #[serde(rename="authorizedNetworks")]
    
    pub authorized_networks: Option<Vec<SqlAclEntry>>,
    /// Whether the instance should be assigned an IPv4 address or not.
    #[serde(rename="enableIpv4")]
    
    pub enable_ipv4: Option<bool>,
    /// The resource link for the VPC network from which the Cloud SQL instance is accessible for private IP. For example, `projects/myProject/global/networks/default`. This setting can be updated, but it cannot be removed after it is set.
    #[serde(rename="privateNetwork")]
    
    pub private_network: Option<String>,
    /// Whether SSL connections over IP should be enforced or not.
    #[serde(rename="requireSsl")]
    
    pub require_ssl: Option<bool>,
}

impl client::Part for SqlIpConfig {}


/// Response message for ‘GenerateSshScript’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations migration jobs generate ssh script projects](ProjectLocationMigrationJobGenerateSshScriptCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SshScript {
    /// The ssh configuration script.
    
    pub script: Option<String>,
}

impl client::ResponseResult for SshScript {}


/// SSL configuration information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SslConfig {
    /// Required. Input only. The x509 PEM-encoded certificate of the CA that signed the source database server's certificate. The replica will use this certificate to verify it's connecting to the right host.
    #[serde(rename="caCertificate")]
    
    pub ca_certificate: Option<String>,
    /// Input only. The x509 PEM-encoded certificate that will be used by the replica to authenticate against the source database server.If this field is used then the 'client_key' field is mandatory.
    #[serde(rename="clientCertificate")]
    
    pub client_certificate: Option<String>,
    /// Input only. The unencrypted PKCS#1 or PKCS#8 PEM-encoded private key associated with the Client Certificate. If this field is used then the 'client_certificate' field is mandatory.
    #[serde(rename="clientKey")]
    
    pub client_key: Option<String>,
    /// Output only. The ssl config type according to 'client_key', 'client_certificate' and 'ca_certificate'.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for SslConfig {}


/// Request message for ‘StartMigrationJob’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations migration jobs start projects](ProjectLocationMigrationJobStartCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StartMigrationJobRequest { _never_set: Option<bool> }

impl client::RequestValue for StartMigrationJobRequest {}


/// The source database will allow incoming connections from the destination database's public IP. You can retrieve the Cloud SQL instance's public IP from the Cloud SQL console or using Cloud SQL APIs. No additional configuration is required.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StaticIpConnectivity { _never_set: Option<bool> }

impl client::Part for StaticIpConnectivity {}


/// Static IP address connectivity configured on service project.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StaticServiceIpConnectivity { _never_set: Option<bool> }

impl client::Part for StaticServiceIpConnectivity {}


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


/// Request message for ‘StopMigrationJob’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations migration jobs stop projects](ProjectLocationMigrationJobStopCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StopMigrationJobRequest { _never_set: Option<bool> }

impl client::RequestValue for StopMigrationJobRequest {}


/// Stored procedure's parent is a schema.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StoredProcedureEntity {
    /// Custom engine specific features
    #[serde(rename="customFeatures")]
    
    pub custom_features: Option<HashMap<String, json::Value>>,
    /// The SQL code which creates the stored procedure
    #[serde(rename="sqlCode")]
    
    pub sql_code: Option<String>,
}

impl client::Part for StoredProcedureEntity {}


/// Synonym's parent is a schema.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SynonymEntity {
    /// Custom engine specific features
    #[serde(rename="customFeatures")]
    
    pub custom_features: Option<HashMap<String, json::Value>>,
    /// The name of the entity for which the synonym is being created (the source)
    #[serde(rename="sourceEntity")]
    
    pub source_entity: Option<String>,
    /// The type of the entity for which the synonym is being created (usually a table or a sequence)
    #[serde(rename="sourceType")]
    
    pub source_type: Option<String>,
}

impl client::Part for SynonymEntity {}


/// Table's parent is a schema.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableEntity {
    /// Table Columns.
    
    pub columns: Option<Vec<ColumnEntity>>,
    /// Comment associated with the table
    
    pub comment: Option<String>,
    /// Table Constraints.
    
    pub constraints: Option<Vec<ConstraintEntity>>,
    /// Custom engine specific features
    #[serde(rename="customFeatures")]
    
    pub custom_features: Option<HashMap<String, json::Value>>,
    /// Table Indices.
    
    pub indices: Option<Vec<IndexEntity>>,
    /// Table triggers.
    
    pub triggers: Option<Vec<TriggerEntity>>,
}

impl client::Part for TableEntity {}


/// Request message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations connection profiles test iam permissions projects](ProjectLocationConnectionProfileTestIamPermissionCall) (request)
/// * [locations migration jobs test iam permissions projects](ProjectLocationMigrationJobTestIamPermissionCall) (request)
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
/// * [locations connection profiles test iam permissions projects](ProjectLocationConnectionProfileTestIamPermissionCall) (response)
/// * [locations migration jobs test iam permissions projects](ProjectLocationMigrationJobTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsResponse {
    /// A subset of `TestPermissionsRequest.permissions` that the caller is allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for TestIamPermissionsResponse {}


/// Trigger is not used as an independent entity, it is retrieved as part of a Table entity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TriggerEntity {
    /// Custom engine specific features
    #[serde(rename="customFeatures")]
    
    pub custom_features: Option<HashMap<String, json::Value>>,
    /// The name of the trigger
    
    pub name: Option<String>,
    /// The SQL code which creates the trigger
    #[serde(rename="sqlCode")]
    
    pub sql_code: Option<String>,
    /// Indicates when the trigger fires, e.g. BEFORE STATEMENT, AFTER EACH ROW
    #[serde(rename="triggerType")]
    
    pub trigger_type: Option<String>,
    /// The DML, DDL, or database events that fires the trigger, e.g. INSERT, UPDATE
    #[serde(rename="triggeringEvents")]
    
    pub triggering_events: Option<Vec<String>>,
}

impl client::Part for TriggerEntity {}


/// The username/password for a database user. Used for specifying initial users at cluster creation time.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserPassword {
    /// The initial password for the user.
    
    pub password: Option<String>,
    /// Output only. Indicates if the initial_user.password field has been set.
    #[serde(rename="passwordSet")]
    
    pub password_set: Option<bool>,
    /// The database username.
    
    pub user: Option<String>,
}

impl client::Part for UserPassword {}


/// Request message for ‘VerifyMigrationJob’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations migration jobs verify projects](ProjectLocationMigrationJobVerifyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VerifyMigrationJobRequest { _never_set: Option<bool> }

impl client::RequestValue for VerifyMigrationJobRequest {}


/// View's parent is a schema.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ViewEntity {
    /// View Constraints.
    
    pub constraints: Option<Vec<ConstraintEntity>>,
    /// Custom engine specific features
    #[serde(rename="customFeatures")]
    
    pub custom_features: Option<HashMap<String, json::Value>>,
    /// The SQL code which creates the view.
    #[serde(rename="sqlCode")]
    
    pub sql_code: Option<String>,
}

impl client::Part for ViewEntity {}


/// VM creation configuration message
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VmCreationConfig {
    /// The subnet name the vm needs to be created in.
    
    pub subnet: Option<String>,
    /// Required. VM instance machine type to create.
    #[serde(rename="vmMachineType")]
    
    pub vm_machine_type: Option<String>,
    /// The Google Cloud Platform zone to create the VM in.
    #[serde(rename="vmZone")]
    
    pub vm_zone: Option<String>,
}

impl client::Part for VmCreationConfig {}


/// VM selection configuration message
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VmSelectionConfig {
    /// Required. The Google Cloud Platform zone the VM is located.
    #[serde(rename="vmZone")]
    
    pub vm_zone: Option<String>,
}

impl client::Part for VmSelectionConfig {}


/// The VPC Peering configuration is used to create VPC peering with the consumer's VPC.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VpcPeeringConfig {
    /// Required. A free subnet for peering. (CIDR of /29)
    
    pub subnet: Option<String>,
    /// Required. Fully qualified name of the VPC DMS will peer to.
    #[serde(rename="vpcName")]
    
    pub vpc_name: Option<String>,
}

impl client::Part for VpcPeeringConfig {}


/// The details of the VPC where the source database is located in Google Cloud. We will use this information to set up the VPC peering connection between Cloud SQL and this VPC.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VpcPeeringConnectivity {
    /// The name of the VPC network to peer with the Cloud SQL private network.
    
    pub vpc: Option<String>,
}

impl client::Part for VpcPeeringConnectivity {}


