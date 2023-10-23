use super::*;
/// AVRO file format configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AvroFileFormat { _never_set: Option<bool> }

impl client::Part for AvroFileFormat {}


/// Backfill strategy to automatically backfill the Stream's objects. Specific objects can be excluded.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BackfillAllStrategy {
    /// MySQL data source objects to avoid backfilling.
    #[serde(rename="mysqlExcludedObjects")]
    
    pub mysql_excluded_objects: Option<MysqlRdbms>,
    /// Oracle data source objects to avoid backfilling.
    #[serde(rename="oracleExcludedObjects")]
    
    pub oracle_excluded_objects: Option<OracleRdbms>,
    /// PostgreSQL data source objects to avoid backfilling.
    #[serde(rename="postgresqlExcludedObjects")]
    
    pub postgresql_excluded_objects: Option<PostgresqlRdbms>,
}

impl client::Part for BackfillAllStrategy {}


/// Represents a backfill job on a specific stream object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BackfillJob {
    /// Output only. Errors which caused the backfill job to fail.
    
    pub errors: Option<Vec<Error>>,
    /// Output only. Backfill job's end time.
    #[serde(rename="lastEndTime")]
    
    pub last_end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Backfill job's start time.
    #[serde(rename="lastStartTime")]
    
    pub last_start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Backfill job state.
    
    pub state: Option<BackfillJobStateEnum>,
    /// Backfill job's triggering reason.
    
    pub trigger: Option<BackfillJobTriggerEnum>,
}

impl client::Part for BackfillJob {}


/// Backfill strategy to disable automatic backfill for the Stream's objects.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BackfillNoneStrategy { _never_set: Option<bool> }

impl client::Part for BackfillNoneStrategy {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BigQueryDestinationConfig {
    /// The guaranteed data freshness (in seconds) when querying tables created by the stream. Editing this field will only affect new tables created in the future, but existing tables will not be impacted. Lower values mean that queries will return fresher data, but may result in higher cost.
    #[serde(rename="dataFreshness")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub data_freshness: Option<client::chrono::Duration>,
    /// Single destination dataset.
    #[serde(rename="singleTargetDataset")]
    
    pub single_target_dataset: Option<SingleTargetDataset>,
    /// Source hierarchy datasets.
    #[serde(rename="sourceHierarchyDatasets")]
    
    pub source_hierarchy_datasets: Option<SourceHierarchyDatasets>,
}

impl client::Part for BigQueryDestinationConfig {}


/// BigQuery warehouse profile.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BigQueryProfile { _never_set: Option<bool> }

impl client::Part for BigQueryProfile {}


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


/// A set of reusable connection configurations to be used as a source or destination for a stream.
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
    /// BigQuery Connection Profile configuration.
    #[serde(rename="bigqueryProfile")]
    
    pub bigquery_profile: Option<BigQueryProfile>,
    /// Output only. The create time of the resource.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. Display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Forward SSH tunnel connectivity.
    #[serde(rename="forwardSshConnectivity")]
    
    pub forward_ssh_connectivity: Option<ForwardSshTunnelConnectivity>,
    /// Cloud Storage ConnectionProfile configuration.
    #[serde(rename="gcsProfile")]
    
    pub gcs_profile: Option<GcsProfile>,
    /// Labels.
    
    pub labels: Option<HashMap<String, String>>,
    /// MySQL ConnectionProfile configuration.
    #[serde(rename="mysqlProfile")]
    
    pub mysql_profile: Option<MysqlProfile>,
    /// Output only. The resource's name.
    
    pub name: Option<String>,
    /// Oracle ConnectionProfile configuration.
    #[serde(rename="oracleProfile")]
    
    pub oracle_profile: Option<OracleProfile>,
    /// PostgreSQL Connection Profile configuration.
    #[serde(rename="postgresqlProfile")]
    
    pub postgresql_profile: Option<PostgresqlProfile>,
    /// Private connectivity.
    #[serde(rename="privateConnectivity")]
    
    pub private_connectivity: Option<PrivateConnectivity>,
    /// Static Service IP connectivity.
    #[serde(rename="staticServiceIpConnectivity")]
    
    pub static_service_ip_connectivity: Option<StaticServiceIpConnectivity>,
    /// Output only. The update time of the resource.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for ConnectionProfile {}
impl client::ResponseResult for ConnectionProfile {}


/// Dataset template used for dynamic dataset creation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatasetTemplate {
    /// If supplied, every created dataset will have its name prefixed by the provided value. The prefix and name will be separated by an underscore. i.e. _.
    #[serde(rename="datasetIdPrefix")]
    
    pub dataset_id_prefix: Option<String>,
    /// Describes the Cloud KMS encryption key that will be used to protect destination BigQuery table. The BigQuery Service Account associated with your project requires access to this encryption key. i.e. projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{cryptoKey}. See https://cloud.google.com/bigquery/docs/customer-managed-encryption for more information.
    #[serde(rename="kmsKeyName")]
    
    pub kms_key_name: Option<String>,
    /// Required. The geographic location where the dataset should reside. See https://cloud.google.com/bigquery/docs/locations for supported locations.
    
    pub location: Option<String>,
}

impl client::Part for DatasetTemplate {}


/// The configuration of the stream destination.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DestinationConfig {
    /// BigQuery destination configuration.
    #[serde(rename="bigqueryDestinationConfig")]
    
    pub bigquery_destination_config: Option<BigQueryDestinationConfig>,
    /// Required. Destination connection profile resource. Format: `projects/{project}/locations/{location}/connectionProfiles/{name}`
    #[serde(rename="destinationConnectionProfile")]
    
    pub destination_connection_profile: Option<String>,
    /// A configuration for how data should be loaded to Cloud Storage.
    #[serde(rename="gcsDestinationConfig")]
    
    pub gcs_destination_config: Option<GcsDestinationConfig>,
}

impl client::Part for DestinationConfig {}


/// Request message for ‘discover’ ConnectionProfile request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations connection profiles discover projects](ProjectLocationConnectionProfileDiscoverCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DiscoverConnectionProfileRequest {
    /// An ad-hoc connection profile configuration.
    #[serde(rename="connectionProfile")]
    
    pub connection_profile: Option<ConnectionProfile>,
    /// A reference to an existing connection profile.
    #[serde(rename="connectionProfileName")]
    
    pub connection_profile_name: Option<String>,
    /// Whether to retrieve the full hierarchy of data objects (TRUE) or only the current level (FALSE).
    #[serde(rename="fullHierarchy")]
    
    pub full_hierarchy: Option<bool>,
    /// The number of hierarchy levels below the current level to be retrieved.
    #[serde(rename="hierarchyDepth")]
    
    pub hierarchy_depth: Option<i32>,
    /// MySQL RDBMS to enrich with child data objects and metadata.
    #[serde(rename="mysqlRdbms")]
    
    pub mysql_rdbms: Option<MysqlRdbms>,
    /// Oracle RDBMS to enrich with child data objects and metadata.
    #[serde(rename="oracleRdbms")]
    
    pub oracle_rdbms: Option<OracleRdbms>,
    /// PostgreSQL RDBMS to enrich with child data objects and metadata.
    #[serde(rename="postgresqlRdbms")]
    
    pub postgresql_rdbms: Option<PostgresqlRdbms>,
}

impl client::RequestValue for DiscoverConnectionProfileRequest {}


/// Response from a discover request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations connection profiles discover projects](ProjectLocationConnectionProfileDiscoverCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DiscoverConnectionProfileResponse {
    /// Enriched MySQL RDBMS object.
    #[serde(rename="mysqlRdbms")]
    
    pub mysql_rdbms: Option<MysqlRdbms>,
    /// Enriched Oracle RDBMS object.
    #[serde(rename="oracleRdbms")]
    
    pub oracle_rdbms: Option<OracleRdbms>,
    /// Enriched PostgreSQL RDBMS object.
    #[serde(rename="postgresqlRdbms")]
    
    pub postgresql_rdbms: Option<PostgresqlRdbms>,
}

impl client::ResponseResult for DiscoverConnectionProfileResponse {}


/// Configuration to drop large object values.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DropLargeObjects { _never_set: Option<bool> }

impl client::Part for DropLargeObjects {}


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


/// Represent a user-facing Error.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Error {
    /// Additional information about the error.
    
    pub details: Option<HashMap<String, String>>,
    /// The time when the error occurred.
    #[serde(rename="errorTime")]
    
    pub error_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A unique identifier for this specific error, allowing it to be traced throughout the system in logs and API responses.
    #[serde(rename="errorUuid")]
    
    pub error_uuid: Option<String>,
    /// A message containing more information about the error that occurred.
    
    pub message: Option<String>,
    /// A title that explains the reason for the error.
    
    pub reason: Option<String>,
}

impl client::Part for Error {}


/// Response message for a ‘FetchStaticIps’ response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations fetch static ips projects](ProjectLocationFetchStaticIpCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FetchStaticIpsResponse {
    /// A token that can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// list of static ips by account
    #[serde(rename="staticIps")]
    
    pub static_ips: Option<Vec<String>>,
}

impl client::ResponseResult for FetchStaticIpsResponse {}


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


/// Google Cloud Storage destination configuration
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GcsDestinationConfig {
    /// AVRO file format configuration.
    #[serde(rename="avroFileFormat")]
    
    pub avro_file_format: Option<AvroFileFormat>,
    /// The maximum duration for which new events are added before a file is closed and a new file is created.
    #[serde(rename="fileRotationInterval")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub file_rotation_interval: Option<client::chrono::Duration>,
    /// The maximum file size to be saved in the bucket.
    #[serde(rename="fileRotationMb")]
    
    pub file_rotation_mb: Option<i32>,
    /// JSON file format configuration.
    #[serde(rename="jsonFileFormat")]
    
    pub json_file_format: Option<JsonFileFormat>,
    /// Path inside the Cloud Storage bucket to write data to.
    
    pub path: Option<String>,
}

impl client::Part for GcsDestinationConfig {}


/// Cloud Storage bucket profile.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GcsProfile {
    /// Required. The Cloud Storage bucket name.
    
    pub bucket: Option<String>,
    /// The root path inside the Cloud Storage bucket.
    #[serde(rename="rootPath")]
    
    pub root_path: Option<String>,
}

impl client::Part for GcsProfile {}


/// JSON file format configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JsonFileFormat {
    /// Compression of the loaded JSON file.
    
    pub compression: Option<JsonFileFormatCompressionEnum>,
    /// The schema file format along JSON data files.
    #[serde(rename="schemaFileFormat")]
    
    pub schema_file_format: Option<JsonFileFormatSchemaFileFormatEnum>,
}

impl client::Part for JsonFileFormat {}


/// Response message for listing connection profiles.
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
    /// List of connection profiles.
    #[serde(rename="connectionProfiles")]
    
    pub connection_profiles: Option<Vec<ConnectionProfile>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListConnectionProfilesResponse {}


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


/// Response containing a list of private connection configurations.
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
    /// List of private connectivity configurations.
    #[serde(rename="privateConnections")]
    
    pub private_connections: Option<Vec<PrivateConnection>>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListPrivateConnectionsResponse {}


/// Route list response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations private connections routes list projects](ProjectLocationPrivateConnectionRouteListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListRoutesResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of Routes.
    
    pub routes: Option<Vec<Route>>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListRoutesResponse {}


/// Response containing the objects for a stream.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations streams objects list projects](ProjectLocationStreamObjectListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListStreamObjectsResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of stream objects.
    #[serde(rename="streamObjects")]
    
    pub stream_objects: Option<Vec<StreamObject>>,
}

impl client::ResponseResult for ListStreamObjectsResponse {}


/// Response message for listing streams.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations streams list projects](ProjectLocationStreamListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListStreamsResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of streams
    
    pub streams: Option<Vec<Stream>>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListStreamsResponse {}


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


/// Request for looking up a specific stream object by its source object identifier.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations streams objects lookup projects](ProjectLocationStreamObjectLookupCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LookupStreamObjectRequest {
    /// Required. The source object identifier which maps to the stream object.
    #[serde(rename="sourceObjectIdentifier")]
    
    pub source_object_identifier: Option<SourceObjectIdentifier>,
}

impl client::RequestValue for LookupStreamObjectRequest {}


/// MySQL Column.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MysqlColumn {
    /// Column collation.
    
    pub collation: Option<String>,
    /// Column name.
    
    pub column: Option<String>,
    /// The MySQL data type. Full data types list can be found here: https://dev.mysql.com/doc/refman/8.0/en/data-types.html
    #[serde(rename="dataType")]
    
    pub data_type: Option<String>,
    /// Column length.
    
    pub length: Option<i32>,
    /// Whether or not the column can accept a null value.
    
    pub nullable: Option<bool>,
    /// The ordinal position of the column in the table.
    #[serde(rename="ordinalPosition")]
    
    pub ordinal_position: Option<i32>,
    /// Whether or not the column represents a primary key.
    #[serde(rename="primaryKey")]
    
    pub primary_key: Option<bool>,
}

impl client::Part for MysqlColumn {}


/// MySQL database.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MysqlDatabase {
    /// Database name.
    
    pub database: Option<String>,
    /// Tables in the database.
    #[serde(rename="mysqlTables")]
    
    pub mysql_tables: Option<Vec<MysqlTable>>,
}

impl client::Part for MysqlDatabase {}


/// Mysql data source object identifier.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MysqlObjectIdentifier {
    /// Required. The database name.
    
    pub database: Option<String>,
    /// Required. The table name.
    
    pub table: Option<String>,
}

impl client::Part for MysqlObjectIdentifier {}


/// MySQL database profile.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MysqlProfile {
    /// Required. Hostname for the MySQL connection.
    
    pub hostname: Option<String>,
    /// Required. Input only. Password for the MySQL connection.
    
    pub password: Option<String>,
    /// Port for the MySQL connection, default value is 3306.
    
    pub port: Option<i32>,
    /// SSL configuration for the MySQL connection.
    #[serde(rename="sslConfig")]
    
    pub ssl_config: Option<MysqlSslConfig>,
    /// Required. Username for the MySQL connection.
    
    pub username: Option<String>,
}

impl client::Part for MysqlProfile {}


/// MySQL database structure
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MysqlRdbms {
    /// Mysql databases on the server
    #[serde(rename="mysqlDatabases")]
    
    pub mysql_databases: Option<Vec<MysqlDatabase>>,
}

impl client::Part for MysqlRdbms {}


/// MySQL source configuration
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MysqlSourceConfig {
    /// MySQL objects to exclude from the stream.
    #[serde(rename="excludeObjects")]
    
    pub exclude_objects: Option<MysqlRdbms>,
    /// MySQL objects to retrieve from the source.
    #[serde(rename="includeObjects")]
    
    pub include_objects: Option<MysqlRdbms>,
    /// Maximum number of concurrent CDC tasks. The number should be non negative. If not set (or set to 0), the system's default value will be used.
    #[serde(rename="maxConcurrentCdcTasks")]
    
    pub max_concurrent_cdc_tasks: Option<i32>,
}

impl client::Part for MysqlSourceConfig {}


/// MySQL SSL configuration information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MysqlSslConfig {
    /// Input only. PEM-encoded certificate of the CA that signed the source database server's certificate.
    #[serde(rename="caCertificate")]
    
    pub ca_certificate: Option<String>,
    /// Output only. Indicates whether the ca_certificate field is set.
    #[serde(rename="caCertificateSet")]
    
    pub ca_certificate_set: Option<bool>,
    /// Input only. PEM-encoded certificate that will be used by the replica to authenticate against the source database server. If this field is used then the 'client_key' and the 'ca_certificate' fields are mandatory.
    #[serde(rename="clientCertificate")]
    
    pub client_certificate: Option<String>,
    /// Output only. Indicates whether the client_certificate field is set.
    #[serde(rename="clientCertificateSet")]
    
    pub client_certificate_set: Option<bool>,
    /// Input only. PEM-encoded private key associated with the Client Certificate. If this field is used then the 'client_certificate' and the 'ca_certificate' fields are mandatory.
    #[serde(rename="clientKey")]
    
    pub client_key: Option<String>,
    /// Output only. Indicates whether the client_key field is set.
    #[serde(rename="clientKeySet")]
    
    pub client_key_set: Option<bool>,
}

impl client::Part for MysqlSslConfig {}


/// MySQL table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MysqlTable {
    /// MySQL columns in the database. When unspecified as part of include/exclude objects, includes/excludes everything.
    #[serde(rename="mysqlColumns")]
    
    pub mysql_columns: Option<Vec<MysqlColumn>>,
    /// Table name.
    
    pub table: Option<String>,
}

impl client::Part for MysqlTable {}


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
/// * [locations operations get projects](ProjectLocationOperationGetCall) (response)
/// * [locations private connections routes create projects](ProjectLocationPrivateConnectionRouteCreateCall) (response)
/// * [locations private connections routes delete projects](ProjectLocationPrivateConnectionRouteDeleteCall) (response)
/// * [locations private connections create projects](ProjectLocationPrivateConnectionCreateCall) (response)
/// * [locations private connections delete projects](ProjectLocationPrivateConnectionDeleteCall) (response)
/// * [locations streams create projects](ProjectLocationStreamCreateCall) (response)
/// * [locations streams delete projects](ProjectLocationStreamDeleteCall) (response)
/// * [locations streams patch projects](ProjectLocationStreamPatchCall) (response)
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


/// Oracle Column.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OracleColumn {
    /// Column name.
    
    pub column: Option<String>,
    /// The Oracle data type.
    #[serde(rename="dataType")]
    
    pub data_type: Option<String>,
    /// Column encoding.
    
    pub encoding: Option<String>,
    /// Column length.
    
    pub length: Option<i32>,
    /// Whether or not the column can accept a null value.
    
    pub nullable: Option<bool>,
    /// The ordinal position of the column in the table.
    #[serde(rename="ordinalPosition")]
    
    pub ordinal_position: Option<i32>,
    /// Column precision.
    
    pub precision: Option<i32>,
    /// Whether or not the column represents a primary key.
    #[serde(rename="primaryKey")]
    
    pub primary_key: Option<bool>,
    /// Column scale.
    
    pub scale: Option<i32>,
}

impl client::Part for OracleColumn {}


/// Oracle data source object identifier.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OracleObjectIdentifier {
    /// Required. The schema name.
    
    pub schema: Option<String>,
    /// Required. The table name.
    
    pub table: Option<String>,
}

impl client::Part for OracleObjectIdentifier {}


/// Oracle database profile.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OracleProfile {
    /// Connection string attributes
    #[serde(rename="connectionAttributes")]
    
    pub connection_attributes: Option<HashMap<String, String>>,
    /// Required. Database for the Oracle connection.
    #[serde(rename="databaseService")]
    
    pub database_service: Option<String>,
    /// Required. Hostname for the Oracle connection.
    
    pub hostname: Option<String>,
    /// Required. Password for the Oracle connection.
    
    pub password: Option<String>,
    /// Port for the Oracle connection, default value is 1521.
    
    pub port: Option<i32>,
    /// Required. Username for the Oracle connection.
    
    pub username: Option<String>,
}

impl client::Part for OracleProfile {}


/// Oracle database structure.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OracleRdbms {
    /// Oracle schemas/databases in the database server.
    #[serde(rename="oracleSchemas")]
    
    pub oracle_schemas: Option<Vec<OracleSchema>>,
}

impl client::Part for OracleRdbms {}


/// Oracle schema.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OracleSchema {
    /// Tables in the schema.
    #[serde(rename="oracleTables")]
    
    pub oracle_tables: Option<Vec<OracleTable>>,
    /// Schema name.
    
    pub schema: Option<String>,
}

impl client::Part for OracleSchema {}


/// Oracle data source configuration
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OracleSourceConfig {
    /// Drop large object values.
    #[serde(rename="dropLargeObjects")]
    
    pub drop_large_objects: Option<DropLargeObjects>,
    /// Oracle objects to exclude from the stream.
    #[serde(rename="excludeObjects")]
    
    pub exclude_objects: Option<OracleRdbms>,
    /// Oracle objects to include in the stream.
    #[serde(rename="includeObjects")]
    
    pub include_objects: Option<OracleRdbms>,
    /// Maximum number of concurrent CDC tasks. The number should be non negative. If not set (or set to 0), the system's default value will be used.
    #[serde(rename="maxConcurrentCdcTasks")]
    
    pub max_concurrent_cdc_tasks: Option<i32>,
    /// Stream large object values. NOTE: This feature is currently experimental.
    #[serde(rename="streamLargeObjects")]
    
    pub stream_large_objects: Option<StreamLargeObjects>,
}

impl client::Part for OracleSourceConfig {}


/// Oracle table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OracleTable {
    /// Oracle columns in the schema. When unspecified as part of include/exclude objects, includes/excludes everything.
    #[serde(rename="oracleColumns")]
    
    pub oracle_columns: Option<Vec<OracleColumn>>,
    /// Table name.
    
    pub table: Option<String>,
}

impl client::Part for OracleTable {}


/// PostgreSQL Column.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PostgresqlColumn {
    /// Column name.
    
    pub column: Option<String>,
    /// The PostgreSQL data type.
    #[serde(rename="dataType")]
    
    pub data_type: Option<String>,
    /// Column length.
    
    pub length: Option<i32>,
    /// Whether or not the column can accept a null value.
    
    pub nullable: Option<bool>,
    /// The ordinal position of the column in the table.
    #[serde(rename="ordinalPosition")]
    
    pub ordinal_position: Option<i32>,
    /// Column precision.
    
    pub precision: Option<i32>,
    /// Whether or not the column represents a primary key.
    #[serde(rename="primaryKey")]
    
    pub primary_key: Option<bool>,
    /// Column scale.
    
    pub scale: Option<i32>,
}

impl client::Part for PostgresqlColumn {}


/// PostgreSQL data source object identifier.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PostgresqlObjectIdentifier {
    /// Required. The schema name.
    
    pub schema: Option<String>,
    /// Required. The table name.
    
    pub table: Option<String>,
}

impl client::Part for PostgresqlObjectIdentifier {}


/// PostgreSQL database profile.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PostgresqlProfile {
    /// Required. Database for the PostgreSQL connection.
    
    pub database: Option<String>,
    /// Required. Hostname for the PostgreSQL connection.
    
    pub hostname: Option<String>,
    /// Required. Password for the PostgreSQL connection.
    
    pub password: Option<String>,
    /// Port for the PostgreSQL connection, default value is 5432.
    
    pub port: Option<i32>,
    /// Required. Username for the PostgreSQL connection.
    
    pub username: Option<String>,
}

impl client::Part for PostgresqlProfile {}


/// PostgreSQL database structure.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PostgresqlRdbms {
    /// PostgreSQL schemas in the database server.
    #[serde(rename="postgresqlSchemas")]
    
    pub postgresql_schemas: Option<Vec<PostgresqlSchema>>,
}

impl client::Part for PostgresqlRdbms {}


/// PostgreSQL schema.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PostgresqlSchema {
    /// Tables in the schema.
    #[serde(rename="postgresqlTables")]
    
    pub postgresql_tables: Option<Vec<PostgresqlTable>>,
    /// Schema name.
    
    pub schema: Option<String>,
}

impl client::Part for PostgresqlSchema {}


/// PostgreSQL data source configuration
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PostgresqlSourceConfig {
    /// PostgreSQL objects to exclude from the stream.
    #[serde(rename="excludeObjects")]
    
    pub exclude_objects: Option<PostgresqlRdbms>,
    /// PostgreSQL objects to include in the stream.
    #[serde(rename="includeObjects")]
    
    pub include_objects: Option<PostgresqlRdbms>,
    /// Required. The name of the publication that includes the set of all tables that are defined in the stream's include_objects.
    
    pub publication: Option<String>,
    /// Required. Immutable. The name of the logical replication slot that's configured with the pgoutput plugin.
    #[serde(rename="replicationSlot")]
    
    pub replication_slot: Option<String>,
}

impl client::Part for PostgresqlSourceConfig {}


/// PostgreSQL table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PostgresqlTable {
    /// PostgreSQL columns in the schema. When unspecified as part of include/exclude objects, includes/excludes everything.
    #[serde(rename="postgresqlColumns")]
    
    pub postgresql_columns: Option<Vec<PostgresqlColumn>>,
    /// Table name.
    
    pub table: Option<String>,
}

impl client::Part for PostgresqlTable {}


/// The PrivateConnection resource is used to establish private connectivity between Datastream and a customer’s network.
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
    /// Required. Display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. In case of error, the details of the error in a user-friendly format.
    
    pub error: Option<Error>,
    /// Labels.
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. The resource's name.
    
    pub name: Option<String>,
    /// Output only. The state of the Private Connection.
    
    pub state: Option<PrivateConnectionStateEnum>,
    /// Output only. The update time of the resource.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// VPC Peering Config.
    #[serde(rename="vpcPeeringConfig")]
    
    pub vpc_peering_config: Option<VpcPeeringConfig>,
}

impl client::RequestValue for PrivateConnection {}
impl client::ResponseResult for PrivateConnection {}


/// Private Connectivity
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PrivateConnectivity {
    /// Required. A reference to a private connection resource. Format: `projects/{project}/locations/{location}/privateConnections/{name}`
    #[serde(rename="privateConnection")]
    
    pub private_connection: Option<String>,
}

impl client::Part for PrivateConnectivity {}


/// The route resource is the child of the private connection resource, used for defining a route for a private connection.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations private connections routes create projects](ProjectLocationPrivateConnectionRouteCreateCall) (request)
/// * [locations private connections routes get projects](ProjectLocationPrivateConnectionRouteGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Route {
    /// Output only. The create time of the resource.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. Destination address for connection
    #[serde(rename="destinationAddress")]
    
    pub destination_address: Option<String>,
    /// Destination port for connection
    #[serde(rename="destinationPort")]
    
    pub destination_port: Option<i32>,
    /// Required. Display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Labels.
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. The resource's name.
    
    pub name: Option<String>,
    /// Output only. The update time of the resource.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Route {}
impl client::ResponseResult for Route {}


/// A single target dataset to which all data will be streamed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SingleTargetDataset {
    /// The dataset ID of the target dataset.
    #[serde(rename="datasetId")]
    
    pub dataset_id: Option<String>,
}

impl client::Part for SingleTargetDataset {}


/// The configuration of the stream source.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SourceConfig {
    /// MySQL data source configuration.
    #[serde(rename="mysqlSourceConfig")]
    
    pub mysql_source_config: Option<MysqlSourceConfig>,
    /// Oracle data source configuration.
    #[serde(rename="oracleSourceConfig")]
    
    pub oracle_source_config: Option<OracleSourceConfig>,
    /// PostgreSQL data source configuration.
    #[serde(rename="postgresqlSourceConfig")]
    
    pub postgresql_source_config: Option<PostgresqlSourceConfig>,
    /// Required. Source connection profile resoource. Format: `projects/{project}/locations/{location}/connectionProfiles/{name}`
    #[serde(rename="sourceConnectionProfile")]
    
    pub source_connection_profile: Option<String>,
}

impl client::Part for SourceConfig {}


/// Destination datasets are created so that hierarchy of the destination data objects matches the source hierarchy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SourceHierarchyDatasets {
    /// no description provided
    #[serde(rename="datasetTemplate")]
    
    pub dataset_template: Option<DatasetTemplate>,
}

impl client::Part for SourceHierarchyDatasets {}


/// Represents an identifier of an object in the data source.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SourceObjectIdentifier {
    /// Mysql data source object identifier.
    #[serde(rename="mysqlIdentifier")]
    
    pub mysql_identifier: Option<MysqlObjectIdentifier>,
    /// Oracle data source object identifier.
    #[serde(rename="oracleIdentifier")]
    
    pub oracle_identifier: Option<OracleObjectIdentifier>,
    /// PostgreSQL data source object identifier.
    #[serde(rename="postgresqlIdentifier")]
    
    pub postgresql_identifier: Option<PostgresqlObjectIdentifier>,
}

impl client::Part for SourceObjectIdentifier {}


/// Request for manually initiating a backfill job for a specific stream object.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations streams objects start backfill job projects](ProjectLocationStreamObjectStartBackfillJobCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StartBackfillJobRequest { _never_set: Option<bool> }

impl client::RequestValue for StartBackfillJobRequest {}


/// Response for manually initiating a backfill job for a specific stream object.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations streams objects start backfill job projects](ProjectLocationStreamObjectStartBackfillJobCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StartBackfillJobResponse {
    /// The stream object resource a backfill job was started for.
    
    pub object: Option<StreamObject>,
}

impl client::ResponseResult for StartBackfillJobResponse {}


/// Static IP address connectivity.
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


/// Request for manually stopping a running backfill job for a specific stream object.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations streams objects stop backfill job projects](ProjectLocationStreamObjectStopBackfillJobCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StopBackfillJobRequest { _never_set: Option<bool> }

impl client::RequestValue for StopBackfillJobRequest {}


/// Response for manually stop a backfill job for a specific stream object.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations streams objects stop backfill job projects](ProjectLocationStreamObjectStopBackfillJobCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StopBackfillJobResponse {
    /// The stream object resource the backfill job was stopped for.
    
    pub object: Option<StreamObject>,
}

impl client::ResponseResult for StopBackfillJobResponse {}


/// A resource representing streaming data from a source to a destination.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations streams create projects](ProjectLocationStreamCreateCall) (request)
/// * [locations streams get projects](ProjectLocationStreamGetCall) (response)
/// * [locations streams patch projects](ProjectLocationStreamPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Stream {
    /// Automatically backfill objects included in the stream source configuration. Specific objects can be excluded.
    #[serde(rename="backfillAll")]
    
    pub backfill_all: Option<BackfillAllStrategy>,
    /// Do not automatically backfill any objects.
    #[serde(rename="backfillNone")]
    
    pub backfill_none: Option<BackfillNoneStrategy>,
    /// Output only. The creation time of the stream.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Immutable. A reference to a KMS encryption key. If provided, it will be used to encrypt the data. If left blank, data will be encrypted using an internal Stream-specific encryption key provisioned through KMS.
    #[serde(rename="customerManagedEncryptionKey")]
    
    pub customer_managed_encryption_key: Option<String>,
    /// Required. Destination connection profile configuration.
    #[serde(rename="destinationConfig")]
    
    pub destination_config: Option<DestinationConfig>,
    /// Required. Display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Errors on the Stream.
    
    pub errors: Option<Vec<Error>>,
    /// Labels.
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. The stream's name.
    
    pub name: Option<String>,
    /// Required. Source connection profile configuration.
    #[serde(rename="sourceConfig")]
    
    pub source_config: Option<SourceConfig>,
    /// The state of the stream.
    
    pub state: Option<StreamStateEnum>,
    /// Output only. The last update time of the stream.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Stream {}
impl client::ResponseResult for Stream {}


/// Configuration to stream large object values.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StreamLargeObjects { _never_set: Option<bool> }

impl client::Part for StreamLargeObjects {}


/// A specific stream object (e.g a specific DB table).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations streams objects get projects](ProjectLocationStreamObjectGetCall) (response)
/// * [locations streams objects lookup projects](ProjectLocationStreamObjectLookupCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StreamObject {
    /// The latest backfill job that was initiated for the stream object.
    #[serde(rename="backfillJob")]
    
    pub backfill_job: Option<BackfillJob>,
    /// Output only. The creation time of the object.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. Display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Active errors on the object.
    
    pub errors: Option<Vec<Error>>,
    /// Output only. The object resource's name.
    
    pub name: Option<String>,
    /// The object identifier in the data source.
    #[serde(rename="sourceObject")]
    
    pub source_object: Option<SourceObjectIdentifier>,
    /// Output only. The last update time of the object.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for StreamObject {}


/// The VPC Peering configuration is used to create VPC peering between Datastream and the consumer's VPC.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VpcPeeringConfig {
    /// Required. A free subnet for peering. (CIDR of /29)
    
    pub subnet: Option<String>,
    /// Required. Fully qualified name of the VPC that Datastream will peer to. Format: `projects/{project}/global/{networks}/{name}`
    
    pub vpc: Option<String>,
}

impl client::Part for VpcPeeringConfig {}


