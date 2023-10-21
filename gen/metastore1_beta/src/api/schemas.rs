use super::*;
/// Request message for DataprocMetastore.AlterMetadataResourceLocation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations services alter location projects](ProjectLocationServiceAlterLocationCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AlterMetadataResourceLocationRequest {
    /// Required. The new location URI for the metadata resource.
    #[serde(rename="locationUri")]
    
    pub location_uri: Option<String>,
    /// Required. The relative metadata resource name in the following format.databases/{database_id} or databases/{database_id}/tables/{table_id} or databases/{database_id}/tables/{table_id}/partitions/{partition_id}
    #[serde(rename="resourceName")]
    
    pub resource_name: Option<String>,
}

impl client::RequestValue for AlterMetadataResourceLocationRequest {}


/// Specifies the audit configuration for a service. The configuration determines which permission types are logged, and what identities, if any, are exempted from logging. An AuditConfig must have one or more AuditLogConfigs.If there are AuditConfigs for both allServices and a specific service, the union of the two AuditConfigs is used for that service: the log_types specified in each AuditConfig are enabled, and the exempted_members in each AuditLogConfig are exempted.Example Policy with multiple AuditConfigs: { "audit_configs": [ { "service": "allServices", "audit_log_configs": [ { "log_type": "DATA_READ", "exempted_members": [ "user:jose@example.com" ] }, { "log_type": "DATA_WRITE" }, { "log_type": "ADMIN_READ" } ] }, { "service": "sampleservice.googleapis.com", "audit_log_configs": [ { "log_type": "DATA_READ" }, { "log_type": "DATA_WRITE", "exempted_members": [ "user:aliya@example.com" ] } ] } ] } For sampleservice, this policy enables DATA_READ, DATA_WRITE and ADMIN_READ logging. It also exempts jose@example.com from DATA_READ logging, and aliya@example.com from DATA_WRITE logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuditConfig {
    /// The configuration for logging of each type of permission.
    #[serde(rename="auditLogConfigs")]
    
    pub audit_log_configs: Option<Vec<AuditLogConfig>>,
    /// Specifies a service that will be enabled for audit logging. For example, storage.googleapis.com, cloudsql.googleapis.com. allServices is a special value that covers all services.
    
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


/// Configuration information for the auxiliary service versions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuxiliaryVersionConfig {
    /// A mapping of Hive metastore configuration key-value pairs to apply to the auxiliary Hive metastore (configured in hive-site.xml) in addition to the primary version's overrides. If keys are present in both the auxiliary version's overrides and the primary version's overrides, the value from the auxiliary version's overrides takes precedence.
    #[serde(rename="configOverrides")]
    
    pub config_overrides: Option<HashMap<String, String>>,
    /// Output only. The network configuration contains the endpoint URI(s) of the auxiliary Hive metastore service.
    #[serde(rename="networkConfig")]
    
    pub network_config: Option<NetworkConfig>,
    /// The Hive metastore version of the auxiliary service. It must be less than the primary Hive metastore service's version.
    
    pub version: Option<String>,
}

impl client::Part for AuxiliaryVersionConfig {}


/// Represents a backend metastore for the federation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BackendMetastore {
    /// The type of the backend metastore.
    #[serde(rename="metastoreType")]
    
    pub metastore_type: Option<String>,
    /// The relative resource name of the metastore that is being federated. The formats of the relative resource names for the currently supported metastores are listed below: BigQuery projects/{project_id} Dataproc Metastore projects/{project_id}/locations/{location}/services/{service_id}
    
    pub name: Option<String>,
}

impl client::Part for BackendMetastore {}


/// The details of a backup resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations services backups create projects](ProjectLocationServiceBackupCreateCall) (request)
/// * [locations services backups get projects](ProjectLocationServiceBackupGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Backup {
    /// Output only. The time when the backup was started.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The description of the backup.
    
    pub description: Option<String>,
    /// Output only. The time when the backup finished creating.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Immutable. The relative resource name of the backup, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}/backups/{backup_id}
    
    pub name: Option<String>,
    /// Output only. Services that are restoring from the backup.
    #[serde(rename="restoringServices")]
    
    pub restoring_services: Option<Vec<String>>,
    /// Output only. The revision of the service at the time of backup.
    #[serde(rename="serviceRevision")]
    
    pub service_revision: Option<Service>,
    /// Output only. The current state of the backup.
    
    pub state: Option<String>,
}

impl client::RequestValue for Backup {}
impl client::ResponseResult for Backup {}


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


/// Contains information of the customer's network configurations.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Consumer {
    /// Output only. The URI of the endpoint used to access the metastore service.
    #[serde(rename="endpointUri")]
    
    pub endpoint_uri: Option<String>,
    /// Immutable. The subnetwork of the customer project from which an IP address is reserved and used as the Dataproc Metastore service's endpoint. It is accessible to hosts in the subnet and to all hosts in a subnet in the same region and same network. There must be at least one IP address available in the subnet's primary range. The subnet is specified in the following form:projects/{project_number}/regions/{region_id}/subnetworks/{subnetwork_id}
    
    pub subnetwork: Option<String>,
}

impl client::Part for Consumer {}


/// Specifies how metastore metadata should be integrated with the Data Catalog service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataCatalogConfig {
    /// Defines whether the metastore metadata should be synced to Data Catalog. The default value is to disable syncing metastore metadata to Data Catalog.
    
    pub enabled: Option<bool>,
}

impl client::Part for DataCatalogConfig {}


/// A specification of the location of and metadata about a database dump from a relational database management system.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatabaseDump {
    /// The type of the database.
    #[serde(rename="databaseType")]
    
    pub database_type: Option<String>,
    /// A Cloud Storage object or folder URI that specifies the source from which to import metadata. It must begin with gs://.
    #[serde(rename="gcsUri")]
    
    pub gcs_uri: Option<String>,
    /// The name of the source database.
    #[serde(rename="sourceDatabase")]
    
    pub source_database: Option<String>,
    /// Optional. The type of the database dump. If unspecified, defaults to MYSQL.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for DatabaseDump {}


/// Specifies how metastore metadata should be integrated with the Dataplex service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataplexConfig {
    /// A reference to the Lake resources that this metastore service is attached to. The key is the lake resource name. Example: projects/{project_number}/locations/{location_id}/lakes/{lake_id}.
    #[serde(rename="lakeResources")]
    
    pub lake_resources: Option<HashMap<String, Lake>>,
}

impl client::Part for DataplexConfig {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } 
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations delete projects](ProjectLocationOperationDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// Encryption settings for the service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EncryptionConfig {
    /// The fully qualified customer provided Cloud KMS key name to use for customer data encryption, in the following form:projects/{project_number}/locations/{location_id}/keyRings/{key_ring_id}/cryptoKeys/{crypto_key_id}.
    #[serde(rename="kmsKey")]
    
    pub kms_key: Option<String>,
}

impl client::Part for EncryptionConfig {}


/// Request message for DataprocMetastore.ExportMetadata.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations services export metadata projects](ProjectLocationServiceExportMetadataCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExportMetadataRequest {
    /// Optional. The type of the database dump. If unspecified, defaults to MYSQL.
    #[serde(rename="databaseDumpType")]
    
    pub database_dump_type: Option<String>,
    /// A Cloud Storage URI of a folder, in the format gs:///. A sub-folder containing exported files will be created below it.
    #[serde(rename="destinationGcsFolder")]
    
    pub destination_gcs_folder: Option<String>,
    /// Optional. A request ID. Specify a unique request ID to allow the server to ignore the request if it has completed. The server will ignore subsequent requests that provide a duplicate request ID for at least 60 minutes after the first request.For example, if an initial request times out, followed by another request with the same request ID, the server ignores the second request to prevent the creation of duplicate commitments.The request ID must be a valid UUID (https://en.wikipedia.org/wiki/Universally_unique_identifier#Format). A zero UUID (00000000-0000-0000-0000-000000000000) is not supported.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::RequestValue for ExportMetadataRequest {}


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


/// Represents a federation of multiple backend metastores.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations federations create projects](ProjectLocationFederationCreateCall) (request)
/// * [locations federations get projects](ProjectLocationFederationGetCall) (response)
/// * [locations federations patch projects](ProjectLocationFederationPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Federation {
    /// A map from BackendMetastore rank to BackendMetastores from which the federation service serves metadata at query time. The map key represents the order in which BackendMetastores should be evaluated to resolve database names at query time and should be greater than or equal to zero. A BackendMetastore with a lower number will be evaluated before a BackendMetastore with a higher number.
    #[serde(rename="backendMetastores")]
    
    pub backend_metastores: Option<HashMap<String, BackendMetastore>>,
    /// Output only. The time when the metastore federation was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The federation endpoint.
    #[serde(rename="endpointUri")]
    
    pub endpoint_uri: Option<String>,
    /// User-defined labels for the metastore federation.
    
    pub labels: Option<HashMap<String, String>>,
    /// Immutable. The relative resource name of the federation, of the form: projects/{project_number}/locations/{location_id}/federations/{federation_id}`.
    
    pub name: Option<String>,
    /// Output only. The current state of the federation.
    
    pub state: Option<String>,
    /// Output only. Additional information about the current state of the metastore federation, if available.
    #[serde(rename="stateMessage")]
    
    pub state_message: Option<String>,
    /// Output only. The globally unique resource identifier of the metastore federation.
    
    pub uid: Option<String>,
    /// Output only. The time when the metastore federation was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Immutable. The Apache Hive metastore version of the federation. All backend metastore versions must be compatible with the federation version.
    
    pub version: Option<String>,
}

impl client::RequestValue for Federation {}
impl client::ResponseResult for Federation {}


/// Specifies configuration information specific to running Hive metastore software as the metastore service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HiveMetastoreConfig {
    /// A mapping of Hive metastore version to the auxiliary version configuration. When specified, a secondary Hive metastore service is created along with the primary service. All auxiliary versions must be less than the service's primary version. The key is the auxiliary service name and it must match the regular expression a-z?. This means that the first character must be a lowercase letter, and all the following characters must be hyphens, lowercase letters, or digits, except the last character, which cannot be a hyphen.
    #[serde(rename="auxiliaryVersions")]
    
    pub auxiliary_versions: Option<HashMap<String, AuxiliaryVersionConfig>>,
    /// A mapping of Hive metastore configuration key-value pairs to apply to the Hive metastore (configured in hive-site.xml). The mappings override system defaults (some keys cannot be overridden). These overrides are also applied to auxiliary versions and can be further customized in the auxiliary version's AuxiliaryVersionConfig.
    #[serde(rename="configOverrides")]
    
    pub config_overrides: Option<HashMap<String, String>>,
    /// The protocol to use for the metastore service endpoint. If unspecified, defaults to THRIFT.
    #[serde(rename="endpointProtocol")]
    
    pub endpoint_protocol: Option<String>,
    /// Information used to configure the Hive metastore service as a service principal in a Kerberos realm. To disable Kerberos, use the UpdateService method and specify this field's path (hive_metastore_config.kerberos_config) in the request's update_mask while omitting this field from the request's service.
    #[serde(rename="kerberosConfig")]
    
    pub kerberos_config: Option<KerberosConfig>,
    /// Immutable. The Hive metastore schema version.
    
    pub version: Option<String>,
}

impl client::Part for HiveMetastoreConfig {}


/// Configuration information for a Kerberos principal.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KerberosConfig {
    /// A Kerberos keytab file that can be used to authenticate a service principal with a Kerberos Key Distribution Center (KDC).
    
    pub keytab: Option<Secret>,
    /// A Cloud Storage URI that specifies the path to a krb5.conf file. It is of the form gs://{bucket_name}/path/to/krb5.conf, although the file does not need to be named krb5.conf explicitly.
    #[serde(rename="krb5ConfigGcsUri")]
    
    pub krb5_config_gcs_uri: Option<String>,
    /// A Kerberos principal that exists in the both the keytab the KDC to authenticate as. A typical principal is of the form primary/instance@REALM, but there is no exact format.
    
    pub principal: Option<String>,
}

impl client::Part for KerberosConfig {}


/// Represents a Lake resource
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Lake {
    /// The Lake resource name. Example: projects/{project_number}/locations/{location_id}/lakes/{lake_id}
    
    pub name: Option<String>,
}

impl client::Part for Lake {}


/// Response message for DataprocMetastore.ListBackups.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations services backups list projects](ProjectLocationServiceBackupListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListBackupsResponse {
    /// The backups of the specified service.
    
    pub backups: Option<Vec<Backup>>,
    /// A token that can be sent as page_token to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListBackupsResponse {}


/// Response message for ListFederations
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations federations list projects](ProjectLocationFederationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListFederationsResponse {
    /// The services in the specified location.
    
    pub federations: Option<Vec<Federation>>,
    /// A token that can be sent as page_token to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListFederationsResponse {}


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


/// Response message for DataprocMetastore.ListMetadataImports.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations services metadata imports list projects](ProjectLocationServiceMetadataImportListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListMetadataImportsResponse {
    /// The imports in the specified service.
    #[serde(rename="metadataImports")]
    
    pub metadata_imports: Option<Vec<MetadataImport>>,
    /// A token that can be sent as page_token to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListMetadataImportsResponse {}


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


/// Response message for DataprocMetastore.ListServices.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations services list projects](ProjectLocationServiceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListServicesResponse {
    /// A token that can be sent as page_token to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The services in the specified location.
    
    pub services: Option<Vec<Service>>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListServicesResponse {}


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
    /// The canonical id for this location. For example: "us-east1".
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
    /// Service-specific metadata. For example the available capacity at the given location.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// Resource name for the location, which may vary between implementations. For example: "projects/example-project/locations/us-east1"
    
    pub name: Option<String>,
}

impl client::ResponseResult for Location {}


/// Maintenance window. This specifies when Dataproc Metastore may perform system maintenance operation to the service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MaintenanceWindow {
    /// The day of week, when the window starts.
    #[serde(rename="dayOfWeek")]
    
    pub day_of_week: Option<String>,
    /// The hour of day (0-23) when the window starts.
    #[serde(rename="hourOfDay")]
    
    pub hour_of_day: Option<i32>,
}

impl client::Part for MaintenanceWindow {}


/// The details of a metadata export operation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetadataExport {
    /// Output only. The type of the database dump.
    #[serde(rename="databaseDumpType")]
    
    pub database_dump_type: Option<String>,
    /// Output only. A Cloud Storage URI of a folder that metadata are exported to, in the form of gs:////, where is automatically generated.
    #[serde(rename="destinationGcsUri")]
    
    pub destination_gcs_uri: Option<String>,
    /// Output only. The time when the export ended.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The time when the export started.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The current state of the export.
    
    pub state: Option<String>,
}

impl client::Part for MetadataExport {}


/// A metastore resource that imports metadata.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations services metadata imports create projects](ProjectLocationServiceMetadataImportCreateCall) (request)
/// * [locations services metadata imports get projects](ProjectLocationServiceMetadataImportGetCall) (response)
/// * [locations services metadata imports patch projects](ProjectLocationServiceMetadataImportPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetadataImport {
    /// Output only. The time when the metadata import was started.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Immutable. A database dump from a pre-existing metastore's database.
    #[serde(rename="databaseDump")]
    
    pub database_dump: Option<DatabaseDump>,
    /// The description of the metadata import.
    
    pub description: Option<String>,
    /// Output only. The time when the metadata import finished.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Immutable. The relative resource name of the metadata import, of the form:projects/{project_number}/locations/{location_id}/services/{service_id}/metadataImports/{metadata_import_id}.
    
    pub name: Option<String>,
    /// Output only. The current state of the metadata import.
    
    pub state: Option<String>,
    /// Output only. The time when the metadata import was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for MetadataImport {}
impl client::ResponseResult for MetadataImport {}


/// Specifies how metastore metadata should be integrated with external services.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetadataIntegration {
    /// The integration config for the Data Catalog service.
    #[serde(rename="dataCatalogConfig")]
    
    pub data_catalog_config: Option<DataCatalogConfig>,
    /// The integration config for the Dataplex service.
    #[serde(rename="dataplexConfig")]
    
    pub dataplex_config: Option<DataplexConfig>,
}

impl client::Part for MetadataIntegration {}


/// The metadata management activities of the metastore service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetadataManagementActivity {
    /// Output only. The latest metadata exports of the metastore service.
    #[serde(rename="metadataExports")]
    
    pub metadata_exports: Option<Vec<MetadataExport>>,
    /// Output only. The latest restores of the metastore service.
    
    pub restores: Option<Vec<Restore>>,
}

impl client::Part for MetadataManagementActivity {}


/// Request message for DataprocMetastore.MoveTableToDatabase.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations services move table to database projects](ProjectLocationServiceMoveTableToDatabaseCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MoveTableToDatabaseRequest {
    /// Required. The name of the database where the table resides.
    #[serde(rename="dbName")]
    
    pub db_name: Option<String>,
    /// Required. The name of the database where the table should be moved.
    #[serde(rename="destinationDbName")]
    
    pub destination_db_name: Option<String>,
    /// Required. The name of the table to be moved.
    #[serde(rename="tableName")]
    
    pub table_name: Option<String>,
}

impl client::RequestValue for MoveTableToDatabaseRequest {}


/// Network configuration for the Dataproc Metastore service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Immutable. The consumer-side network configuration for the Dataproc Metastore instance.
    
    pub consumers: Option<Vec<Consumer>>,
}

impl client::Part for NetworkConfig {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations federations create projects](ProjectLocationFederationCreateCall) (response)
/// * [locations federations delete projects](ProjectLocationFederationDeleteCall) (response)
/// * [locations federations patch projects](ProjectLocationFederationPatchCall) (response)
/// * [locations operations get projects](ProjectLocationOperationGetCall) (response)
/// * [locations services backups create projects](ProjectLocationServiceBackupCreateCall) (response)
/// * [locations services backups delete projects](ProjectLocationServiceBackupDeleteCall) (response)
/// * [locations services metadata imports create projects](ProjectLocationServiceMetadataImportCreateCall) (response)
/// * [locations services metadata imports patch projects](ProjectLocationServiceMetadataImportPatchCall) (response)
/// * [locations services alter location projects](ProjectLocationServiceAlterLocationCall) (response)
/// * [locations services create projects](ProjectLocationServiceCreateCall) (response)
/// * [locations services delete projects](ProjectLocationServiceDeleteCall) (response)
/// * [locations services export metadata projects](ProjectLocationServiceExportMetadataCall) (response)
/// * [locations services move table to database projects](ProjectLocationServiceMoveTableToDatabaseCall) (response)
/// * [locations services patch projects](ProjectLocationServicePatchCall) (response)
/// * [locations services query metadata projects](ProjectLocationServiceQueryMetadataCall) (response)
/// * [locations services restore projects](ProjectLocationServiceRestoreCall) (response)
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


/// An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources.A Policy is a collection of bindings. A binding binds one or more members, or principals, to a single role. Principals can be user accounts, service accounts, Google groups, and domains (such as G Suite). A role is a named list of permissions; each role can be an IAM predefined role or a user-created custom role.For some types of Google Cloud resources, a binding can also specify a condition, which is a logical expression that allows access to a resource only if the expression evaluates to true. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the IAM documentation (https://cloud.google.com/iam/help/conditions/resource-policies).JSON example: { “bindings”: \[ { “role”: “roles/resourcemanager.organizationAdmin”, “members”: \[ “user:mike@example.com”, “group:admins@example.com”, “domain:google.com”, “serviceAccount:my-project-id@appspot.gserviceaccount.com” \] }, { “role”: “roles/resourcemanager.organizationViewer”, “members”: \[ “user:eve@example.com” \], “condition”: { “title”: “expirable access”, “description”: “Does not grant access after Sep 2020”, “expression”: “request.time \< timestamp(‘2020-10-01T00:00:00.000Z’)”, } } \], “etag”: “BwWWja0YfJA=”, “version”: 3 } YAML example: bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time \< timestamp(‘2020-10-01T00:00:00.000Z’) etag: BwWWja0YfJA= version: 3 For a description of IAM and its features, see the IAM documentation (https://cloud.google.com/iam/docs/).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations federations get iam policy projects](ProjectLocationFederationGetIamPolicyCall) (response)
/// * [locations federations set iam policy projects](ProjectLocationFederationSetIamPolicyCall) (response)
/// * [locations services backups get iam policy projects](ProjectLocationServiceBackupGetIamPolicyCall) (response)
/// * [locations services backups set iam policy projects](ProjectLocationServiceBackupSetIamPolicyCall) (response)
/// * [locations services databases tables get iam policy projects](ProjectLocationServiceDatabaseTableGetIamPolicyCall) (response)
/// * [locations services databases tables set iam policy projects](ProjectLocationServiceDatabaseTableSetIamPolicyCall) (response)
/// * [locations services databases get iam policy projects](ProjectLocationServiceDatabaseGetIamPolicyCall) (response)
/// * [locations services databases set iam policy projects](ProjectLocationServiceDatabaseSetIamPolicyCall) (response)
/// * [locations services get iam policy projects](ProjectLocationServiceGetIamPolicyCall) (response)
/// * [locations services set iam policy projects](ProjectLocationServiceSetIamPolicyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Policy {
    /// Specifies cloud audit logging configuration for this policy.
    #[serde(rename="auditConfigs")]
    
    pub audit_configs: Option<Vec<AuditConfig>>,
    /// Associates a list of members, or principals, with a role. Optionally, may specify a condition that determines how and when the bindings are applied. Each of the bindings must contain at least one principal.The bindings in a Policy can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the bindings grant 50 different roles to user:alice@example.com, and not to any other principal, then you can add another 1,450 principals to the bindings in the Policy.
    
    pub bindings: Option<Vec<Binding>>,
    /// etag is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the etag in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An etag is returned in the response to getIamPolicy, and systems are expected to put that etag in the request to setIamPolicy to ensure that their change will be applied to the same version of the policy.Important: If you use IAM Conditions, you must include the etag field whenever you call setIamPolicy. If you omit this field, then IAM allows you to overwrite a version 3 policy with a version 1 policy, and all of the conditions in the version 3 policy are lost.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
    /// Specifies the format of the policy.Valid values are 0, 1, and 3. Requests that specify an invalid value are rejected.Any operation that affects conditional role bindings must specify version 3. This requirement applies to the following operations: Getting a policy that includes a conditional role binding Adding a conditional role binding to a policy Changing a conditional role binding in a policy Removing any role binding, with or without a condition, from a policy that includes conditionsImportant: If you use IAM Conditions, you must include the etag field whenever you call setIamPolicy. If you omit this field, then IAM allows you to overwrite a version 3 policy with a version 1 policy, and all of the conditions in the version 3 policy are lost.If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset.To learn which resources support conditions in their IAM policies, see the IAM documentation (https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub version: Option<i32>,
}

impl client::ResponseResult for Policy {}


/// Request message for DataprocMetastore.QueryMetadata.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations services query metadata projects](ProjectLocationServiceQueryMetadataCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryMetadataRequest {
    /// Required. A read-only SQL query to execute against the metadata database. The query cannot change or mutate the data.
    
    pub query: Option<String>,
}

impl client::RequestValue for QueryMetadataRequest {}


/// Request message for DataprocMetastore.RemoveIamPolicy.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations services remove iam policy projects](ProjectLocationServiceRemoveIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RemoveIamPolicyRequest {
    /// Optional. Removes IAM policy attached to database or table asynchronously when it is set. The default is false.
    
    pub asynchronous: Option<bool>,
}

impl client::RequestValue for RemoveIamPolicyRequest {}


/// Response message for DataprocMetastore.RemoveIamPolicy.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations services remove iam policy projects](ProjectLocationServiceRemoveIamPolicyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RemoveIamPolicyResponse {
    /// True if the policy is successfully removed.
    
    pub success: Option<bool>,
}

impl client::ResponseResult for RemoveIamPolicyResponse {}


/// The details of a metadata restore operation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Restore {
    /// Output only. The relative resource name of the metastore service backup to restore from, in the following form:projects/{project_id}/locations/{location_id}/services/{service_id}/backups/{backup_id}.
    
    pub backup: Option<String>,
    /// Output only. The restore details containing the revision of the service to be restored to, in format of JSON.
    
    pub details: Option<String>,
    /// Output only. The time when the restore ended.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The time when the restore started.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The current state of the restore.
    
    pub state: Option<String>,
    /// Output only. The type of restore.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for Restore {}


/// Request message for DataprocMetastore.Restore.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations services restore projects](ProjectLocationServiceRestoreCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RestoreServiceRequest {
    /// Required. The relative resource name of the metastore service backup to restore from, in the following form:projects/{project_id}/locations/{location_id}/services/{service_id}/backups/{backup_id}.
    
    pub backup: Option<String>,
    /// Optional. A request ID. Specify a unique request ID to allow the server to ignore the request if it has completed. The server will ignore subsequent requests that provide a duplicate request ID for at least 60 minutes after the first request.For example, if an initial request times out, followed by another request with the same request ID, the server ignores the second request to prevent the creation of duplicate commitments.The request ID must be a valid UUID (https://en.wikipedia.org/wiki/Universally_unique_identifier#Format). A zero UUID (00000000-0000-0000-0000-000000000000) is not supported.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
    /// Optional. The type of restore. If unspecified, defaults to METADATA_ONLY.
    #[serde(rename="restoreType")]
    
    pub restore_type: Option<String>,
}

impl client::RequestValue for RestoreServiceRequest {}


/// A securely stored value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Secret {
    /// The relative resource name of a Secret Manager secret version, in the following form:projects/{project_number}/secrets/{secret_id}/versions/{version_id}.
    #[serde(rename="cloudSecret")]
    
    pub cloud_secret: Option<String>,
}

impl client::Part for Secret {}


/// A managed metastore service that serves metadata queries.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations services create projects](ProjectLocationServiceCreateCall) (request)
/// * [locations services get projects](ProjectLocationServiceGetCall) (response)
/// * [locations services patch projects](ProjectLocationServicePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Service {
    /// Output only. A Cloud Storage URI (starting with gs://) that specifies where artifacts related to the metastore service are stored.
    #[serde(rename="artifactGcsUri")]
    
    pub artifact_gcs_uri: Option<String>,
    /// Output only. The time when the metastore service was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Immutable. The database type that the Metastore service stores its data.
    #[serde(rename="databaseType")]
    
    pub database_type: Option<String>,
    /// Immutable. Information used to configure the Dataproc Metastore service to encrypt customer data at rest. Cannot be updated.
    #[serde(rename="encryptionConfig")]
    
    pub encryption_config: Option<EncryptionConfig>,
    /// Output only. The URI of the endpoint used to access the metastore service.
    #[serde(rename="endpointUri")]
    
    pub endpoint_uri: Option<String>,
    /// Configuration information specific to running Hive metastore software as the metastore service.
    #[serde(rename="hiveMetastoreConfig")]
    
    pub hive_metastore_config: Option<HiveMetastoreConfig>,
    /// User-defined labels for the metastore service.
    
    pub labels: Option<HashMap<String, String>>,
    /// The one hour maintenance window of the metastore service. This specifies when the service can be restarted for maintenance purposes in UTC time. Maintenance window is not needed for services with the SPANNER database type.
    #[serde(rename="maintenanceWindow")]
    
    pub maintenance_window: Option<MaintenanceWindow>,
    /// The setting that defines how metastore metadata should be integrated with external services and systems.
    #[serde(rename="metadataIntegration")]
    
    pub metadata_integration: Option<MetadataIntegration>,
    /// Output only. The metadata management activities of the metastore service.
    #[serde(rename="metadataManagementActivity")]
    
    pub metadata_management_activity: Option<MetadataManagementActivity>,
    /// Immutable. The relative resource name of the metastore service, in the following format:projects/{project_number}/locations/{location_id}/services/{service_id}.
    
    pub name: Option<String>,
    /// Immutable. The relative resource name of the VPC network on which the instance can be accessed. It is specified in the following form:projects/{project_number}/global/networks/{network_id}.
    
    pub network: Option<String>,
    /// The configuration specifying the network settings for the Dataproc Metastore service.
    #[serde(rename="networkConfig")]
    
    pub network_config: Option<NetworkConfig>,
    /// The TCP port at which the metastore service is reached. Default: 9083.
    
    pub port: Option<i32>,
    /// Immutable. The release channel of the service. If unspecified, defaults to STABLE.
    #[serde(rename="releaseChannel")]
    
    pub release_channel: Option<String>,
    /// Output only. The current state of the metastore service.
    
    pub state: Option<String>,
    /// Output only. Additional information about the current state of the metastore service, if available.
    #[serde(rename="stateMessage")]
    
    pub state_message: Option<String>,
    /// The configuration specifying telemetry settings for the Dataproc Metastore service. If unspecified defaults to JSON.
    #[serde(rename="telemetryConfig")]
    
    pub telemetry_config: Option<TelemetryConfig>,
    /// The tier of the service.
    
    pub tier: Option<String>,
    /// Output only. The globally unique resource identifier of the metastore service.
    
    pub uid: Option<String>,
    /// Output only. The time when the metastore service was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Service {}
impl client::ResponseResult for Service {}


/// Request message for SetIamPolicy method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations federations set iam policy projects](ProjectLocationFederationSetIamPolicyCall) (request)
/// * [locations services backups set iam policy projects](ProjectLocationServiceBackupSetIamPolicyCall) (request)
/// * [locations services databases tables set iam policy projects](ProjectLocationServiceDatabaseTableSetIamPolicyCall) (request)
/// * [locations services databases set iam policy projects](ProjectLocationServiceDatabaseSetIamPolicyCall) (request)
/// * [locations services set iam policy projects](ProjectLocationServiceSetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetIamPolicyRequest {
    /// REQUIRED: The complete policy to be applied to the resource. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them.
    
    pub policy: Option<Policy>,
    /// OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used:paths: "bindings, etag"
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for SetIamPolicyRequest {}


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


/// Telemetry Configuration for the Dataproc Metastore service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TelemetryConfig {
    /// The output format of the Dataproc Metastore service's logs.
    #[serde(rename="logFormat")]
    
    pub log_format: Option<String>,
}

impl client::Part for TelemetryConfig {}


/// Request message for TestIamPermissions method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations federations test iam permissions projects](ProjectLocationFederationTestIamPermissionCall) (request)
/// * [locations services backups test iam permissions projects](ProjectLocationServiceBackupTestIamPermissionCall) (request)
/// * [locations services databases tables test iam permissions projects](ProjectLocationServiceDatabaseTableTestIamPermissionCall) (request)
/// * [locations services databases test iam permissions projects](ProjectLocationServiceDatabaseTestIamPermissionCall) (request)
/// * [locations services test iam permissions projects](ProjectLocationServiceTestIamPermissionCall) (request)
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
/// * [locations federations test iam permissions projects](ProjectLocationFederationTestIamPermissionCall) (response)
/// * [locations services backups test iam permissions projects](ProjectLocationServiceBackupTestIamPermissionCall) (response)
/// * [locations services databases tables test iam permissions projects](ProjectLocationServiceDatabaseTableTestIamPermissionCall) (response)
/// * [locations services databases test iam permissions projects](ProjectLocationServiceDatabaseTestIamPermissionCall) (response)
/// * [locations services test iam permissions projects](ProjectLocationServiceTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsResponse {
    /// A subset of TestPermissionsRequest.permissions that the caller is allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for TestIamPermissionsResponse {}


