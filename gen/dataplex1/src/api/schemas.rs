use super::*;
/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } 
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations lakes content delete projects](ProjectLocationLakeContentDeleteCall) (response)
/// * [locations lakes contentitems delete projects](ProjectLocationLakeContentitemDeleteCall) (response)
/// * [locations lakes tasks jobs cancel projects](ProjectLocationLakeTaskJobCancelCall) (response)
/// * [locations lakes zones entities partitions delete projects](ProjectLocationLakeZoneEntityPartitionDeleteCall) (response)
/// * [locations lakes zones entities delete projects](ProjectLocationLakeZoneEntityDeleteCall) (response)
/// * [locations operations cancel projects](ProjectLocationOperationCancelCall) (response)
/// * [locations operations delete projects](ProjectLocationOperationDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// Action represents an issue requiring administrator action for resolution.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1Action {
    /// Output only. The relative resource name of the asset, of the form: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}/assets/{asset_id}.
    
    pub asset: Option<String>,
    /// The category of issue associated with the action.
    
    pub category: Option<String>,
    /// The list of data locations associated with this action. Cloud Storage locations are represented as URI paths(E.g. gs://bucket/table1/year=2020/month=Jan/). BigQuery locations refer to resource names(E.g. bigquery.googleapis.com/projects/project-id/datasets/dataset-id).
    #[serde(rename="dataLocations")]
    
    pub data_locations: Option<Vec<String>>,
    /// The time that the issue was detected.
    #[serde(rename="detectTime")]
    
    pub detect_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Details for issues related to applying security policy.
    #[serde(rename="failedSecurityPolicyApply")]
    
    pub failed_security_policy_apply: Option<GoogleCloudDataplexV1ActionFailedSecurityPolicyApply>,
    /// Details for issues related to incompatible schemas detected within data.
    #[serde(rename="incompatibleDataSchema")]
    
    pub incompatible_data_schema: Option<GoogleCloudDataplexV1ActionIncompatibleDataSchema>,
    /// Details for issues related to invalid or unsupported data formats.
    #[serde(rename="invalidDataFormat")]
    
    pub invalid_data_format: Option<GoogleCloudDataplexV1ActionInvalidDataFormat>,
    /// Details for issues related to invalid data arrangement.
    #[serde(rename="invalidDataOrganization")]
    
    pub invalid_data_organization: Option<GoogleCloudDataplexV1ActionInvalidDataOrganization>,
    /// Details for issues related to invalid or unsupported data partition structure.
    #[serde(rename="invalidDataPartition")]
    
    pub invalid_data_partition: Option<GoogleCloudDataplexV1ActionInvalidDataPartition>,
    /// Detailed description of the issue requiring action.
    
    pub issue: Option<String>,
    /// Output only. The relative resource name of the lake, of the form: projects/{project_number}/locations/{location_id}/lakes/{lake_id}.
    
    pub lake: Option<String>,
    /// Details for issues related to absence of data within managed resources.
    #[serde(rename="missingData")]
    
    pub missing_data: Option<GoogleCloudDataplexV1ActionMissingData>,
    /// Details for issues related to absence of a managed resource.
    #[serde(rename="missingResource")]
    
    pub missing_resource: Option<GoogleCloudDataplexV1ActionMissingResource>,
    /// Output only. The relative resource name of the action, of the form: projects/{project}/locations/{location}/lakes/{lake}/actions/{action} projects/{project}/locations/{location}/lakes/{lake}/zones/{zone}/actions/{action} projects/{project}/locations/{location}/lakes/{lake}/zones/{zone}/assets/{asset}/actions/{action}.
    
    pub name: Option<String>,
    /// Details for issues related to lack of permissions to access data resources.
    #[serde(rename="unauthorizedResource")]
    
    pub unauthorized_resource: Option<GoogleCloudDataplexV1ActionUnauthorizedResource>,
    /// Output only. The relative resource name of the zone, of the form: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}.
    
    pub zone: Option<String>,
}

impl client::Part for GoogleCloudDataplexV1Action {}


/// Failed to apply security policy to the managed resource(s) under a lake, zone or an asset. For a lake or zone resource, one or more underlying assets has a failure applying security policy to the associated managed resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1ActionFailedSecurityPolicyApply {
    /// Resource name of one of the assets with failing security policy application. Populated for a lake or zone resource only.
    
    pub asset: Option<String>,
}

impl client::Part for GoogleCloudDataplexV1ActionFailedSecurityPolicyApply {}


/// Action details for incompatible schemas detected by discovery.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1ActionIncompatibleDataSchema {
    /// The existing and expected schema of the table. The schema is provided as a JSON formatted structure listing columns and data types.
    #[serde(rename="existingSchema")]
    
    pub existing_schema: Option<String>,
    /// The new and incompatible schema within the table. The schema is provided as a JSON formatted structured listing columns and data types.
    #[serde(rename="newSchema")]
    
    pub new_schema: Option<String>,
    /// The list of data locations sampled and used for format/schema inference.
    #[serde(rename="sampledDataLocations")]
    
    pub sampled_data_locations: Option<Vec<String>>,
    /// Whether the action relates to a schema that is incompatible or modified.
    #[serde(rename="schemaChange")]
    
    pub schema_change: Option<String>,
    /// The name of the table containing invalid data.
    
    pub table: Option<String>,
}

impl client::Part for GoogleCloudDataplexV1ActionIncompatibleDataSchema {}


/// Action details for invalid or unsupported data files detected by discovery.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1ActionInvalidDataFormat {
    /// The expected data format of the entity.
    #[serde(rename="expectedFormat")]
    
    pub expected_format: Option<String>,
    /// The new unexpected data format within the entity.
    #[serde(rename="newFormat")]
    
    pub new_format: Option<String>,
    /// The list of data locations sampled and used for format/schema inference.
    #[serde(rename="sampledDataLocations")]
    
    pub sampled_data_locations: Option<Vec<String>>,
}

impl client::Part for GoogleCloudDataplexV1ActionInvalidDataFormat {}


/// Action details for invalid data arrangement.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1ActionInvalidDataOrganization { _never_set: Option<bool> }

impl client::Part for GoogleCloudDataplexV1ActionInvalidDataOrganization {}


/// Action details for invalid or unsupported partitions detected by discovery.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1ActionInvalidDataPartition {
    /// The issue type of InvalidDataPartition.
    #[serde(rename="expectedStructure")]
    
    pub expected_structure: Option<String>,
}

impl client::Part for GoogleCloudDataplexV1ActionInvalidDataPartition {}


/// Action details for absence of data detected by discovery.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1ActionMissingData { _never_set: Option<bool> }

impl client::Part for GoogleCloudDataplexV1ActionMissingData {}


/// Action details for resource references in assets that cannot be located.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1ActionMissingResource { _never_set: Option<bool> }

impl client::Part for GoogleCloudDataplexV1ActionMissingResource {}


/// Action details for unauthorized resource issues raised to indicate that the service account associated with the lake instance is not authorized to access or manage the resource associated with an asset.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1ActionUnauthorizedResource { _never_set: Option<bool> }

impl client::Part for GoogleCloudDataplexV1ActionUnauthorizedResource {}


/// An asset represents a cloud resource that is being managed within a lake as a member of a zone.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations lakes zones assets create projects](ProjectLocationLakeZoneAssetCreateCall) (request)
/// * [locations lakes zones assets get projects](ProjectLocationLakeZoneAssetGetCall) (response)
/// * [locations lakes zones assets patch projects](ProjectLocationLakeZoneAssetPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1Asset {
    /// Output only. The time when the asset was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Description of the asset.
    
    pub description: Option<String>,
    /// Optional. Specification of the discovery feature applied to data referenced by this asset. When this spec is left unset, the asset will use the spec set on the parent zone.
    #[serde(rename="discoverySpec")]
    
    pub discovery_spec: Option<GoogleCloudDataplexV1AssetDiscoverySpec>,
    /// Output only. Status of the discovery feature applied to data referenced by this asset.
    #[serde(rename="discoveryStatus")]
    
    pub discovery_status: Option<GoogleCloudDataplexV1AssetDiscoveryStatus>,
    /// Optional. User friendly display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Optional. User defined labels for the asset.
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. The relative resource name of the asset, of the form: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}/assets/{asset_id}.
    
    pub name: Option<String>,
    /// Required. Specification of the resource that is referenced by this asset.
    #[serde(rename="resourceSpec")]
    
    pub resource_spec: Option<GoogleCloudDataplexV1AssetResourceSpec>,
    /// Output only. Status of the resource referenced by this asset.
    #[serde(rename="resourceStatus")]
    
    pub resource_status: Option<GoogleCloudDataplexV1AssetResourceStatus>,
    /// Output only. Status of the security policy applied to resource referenced by this asset.
    #[serde(rename="securityStatus")]
    
    pub security_status: Option<GoogleCloudDataplexV1AssetSecurityStatus>,
    /// Output only. Current state of the asset.
    
    pub state: Option<String>,
    /// Output only. System generated globally unique ID for the asset. This ID will be different if the asset is deleted and re-created with the same name.
    
    pub uid: Option<String>,
    /// Output only. The time when the asset was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GoogleCloudDataplexV1Asset {}
impl client::ResponseResult for GoogleCloudDataplexV1Asset {}


/// Settings to manage the metadata discovery and publishing for an asset.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1AssetDiscoverySpec {
    /// Optional. Configuration for CSV data.
    #[serde(rename="csvOptions")]
    
    pub csv_options: Option<GoogleCloudDataplexV1AssetDiscoverySpecCsvOptions>,
    /// Optional. Whether discovery is enabled.
    
    pub enabled: Option<bool>,
    /// Optional. The list of patterns to apply for selecting data to exclude during discovery. For Cloud Storage bucket assets, these are interpreted as glob patterns used to match object names. For BigQuery dataset assets, these are interpreted as patterns to match table names.
    #[serde(rename="excludePatterns")]
    
    pub exclude_patterns: Option<Vec<String>>,
    /// Optional. The list of patterns to apply for selecting data to include during discovery if only a subset of the data should considered. For Cloud Storage bucket assets, these are interpreted as glob patterns used to match object names. For BigQuery dataset assets, these are interpreted as patterns to match table names.
    #[serde(rename="includePatterns")]
    
    pub include_patterns: Option<Vec<String>>,
    /// Optional. Configuration for Json data.
    #[serde(rename="jsonOptions")]
    
    pub json_options: Option<GoogleCloudDataplexV1AssetDiscoverySpecJsonOptions>,
    /// Optional. Cron schedule (https://en.wikipedia.org/wiki/Cron) for running discovery periodically. Successive discovery runs must be scheduled at least 60 minutes apart. The default value is to run discovery every 60 minutes. To explicitly set a timezone to the cron tab, apply a prefix in the cron tab: "CRON_TZ=${IANA_TIME_ZONE}" or TZ=${IANA_TIME_ZONE}". The ${IANA_TIME_ZONE} may only be a valid string from IANA time zone database. For example, CRON_TZ=America/New_York 1 * * * *, or TZ=America/New_York 1 * * * *.
    
    pub schedule: Option<String>,
}

impl client::Part for GoogleCloudDataplexV1AssetDiscoverySpec {}


/// Describe CSV and similar semi-structured data formats.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1AssetDiscoverySpecCsvOptions {
    /// Optional. The delimiter being used to separate values. This defaults to ','.
    
    pub delimiter: Option<String>,
    /// Optional. Whether to disable the inference of data type for CSV data. If true, all columns will be registered as strings.
    #[serde(rename="disableTypeInference")]
    
    pub disable_type_inference: Option<bool>,
    /// Optional. The character encoding of the data. The default is UTF-8.
    
    pub encoding: Option<String>,
    /// Optional. The number of rows to interpret as header rows that should be skipped when reading data rows.
    #[serde(rename="headerRows")]
    
    pub header_rows: Option<i32>,
}

impl client::Part for GoogleCloudDataplexV1AssetDiscoverySpecCsvOptions {}


/// Describe JSON data format.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1AssetDiscoverySpecJsonOptions {
    /// Optional. Whether to disable the inference of data type for Json data. If true, all columns will be registered as their primitive types (strings, number or boolean).
    #[serde(rename="disableTypeInference")]
    
    pub disable_type_inference: Option<bool>,
    /// Optional. The character encoding of the data. The default is UTF-8.
    
    pub encoding: Option<String>,
}

impl client::Part for GoogleCloudDataplexV1AssetDiscoverySpecJsonOptions {}


/// Status of discovery for an asset.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1AssetDiscoveryStatus {
    /// The duration of the last discovery run.
    #[serde(rename="lastRunDuration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub last_run_duration: Option<client::chrono::Duration>,
    /// The start time of the last discovery run.
    #[serde(rename="lastRunTime")]
    
    pub last_run_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Additional information about the current state.
    
    pub message: Option<String>,
    /// The current status of the discovery feature.
    
    pub state: Option<String>,
    /// Data Stats of the asset reported by discovery.
    
    pub stats: Option<GoogleCloudDataplexV1AssetDiscoveryStatusStats>,
    /// Last update time of the status.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GoogleCloudDataplexV1AssetDiscoveryStatus {}


/// The aggregated data statistics for the asset reported by discovery.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1AssetDiscoveryStatusStats {
    /// The count of data items within the referenced resource.
    #[serde(rename="dataItems")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub data_items: Option<i64>,
    /// The number of stored data bytes within the referenced resource.
    #[serde(rename="dataSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub data_size: Option<i64>,
    /// The count of fileset entities within the referenced resource.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub filesets: Option<i64>,
    /// The count of table entities within the referenced resource.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub tables: Option<i64>,
}

impl client::Part for GoogleCloudDataplexV1AssetDiscoveryStatusStats {}


/// Identifies the cloud resource that is referenced by this asset.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1AssetResourceSpec {
    /// Immutable. Relative name of the cloud resource that contains the data that is being managed within a lake. For example: projects/{project_number}/buckets/{bucket_id} projects/{project_number}/datasets/{dataset_id}
    
    pub name: Option<String>,
    /// Optional. Determines how read permissions are handled for each asset and their associated tables. Only available to storage buckets assets.
    #[serde(rename="readAccessMode")]
    
    pub read_access_mode: Option<String>,
    /// Required. Immutable. Type of resource.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleCloudDataplexV1AssetResourceSpec {}


/// Status of the resource referenced by an asset.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1AssetResourceStatus {
    /// Additional information about the current state.
    
    pub message: Option<String>,
    /// The current state of the managed resource.
    
    pub state: Option<String>,
    /// Last update time of the status.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GoogleCloudDataplexV1AssetResourceStatus {}


/// Security policy status of the asset. Data security policy, i.e., readers, writers & owners, should be specified in the lake/zone/asset IAM policy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1AssetSecurityStatus {
    /// Additional information about the current state.
    
    pub message: Option<String>,
    /// The current state of the security policy applied to the attached resource.
    
    pub state: Option<String>,
    /// Last update time of the status.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GoogleCloudDataplexV1AssetSecurityStatus {}


/// Aggregated status of the underlying assets of a lake or zone.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1AssetStatus {
    /// Number of active assets.
    #[serde(rename="activeAssets")]
    
    pub active_assets: Option<i32>,
    /// Number of assets that are in process of updating the security policy on attached resources.
    #[serde(rename="securityPolicyApplyingAssets")]
    
    pub security_policy_applying_assets: Option<i32>,
    /// Last update time of the status.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GoogleCloudDataplexV1AssetStatus {}


/// Cancel task jobs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations lakes tasks jobs cancel projects](ProjectLocationLakeTaskJobCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1CancelJobRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleCloudDataplexV1CancelJobRequest {}


/// Content represents a user-visible notebook or a sql script
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations lakes content create projects](ProjectLocationLakeContentCreateCall) (request|response)
/// * [locations lakes content get projects](ProjectLocationLakeContentGetCall) (response)
/// * [locations lakes content patch projects](ProjectLocationLakeContentPatchCall) (request|response)
/// * [locations lakes contentitems create projects](ProjectLocationLakeContentitemCreateCall) (request|response)
/// * [locations lakes contentitems get projects](ProjectLocationLakeContentitemGetCall) (response)
/// * [locations lakes contentitems patch projects](ProjectLocationLakeContentitemPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1Content {
    /// Output only. Content creation time.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. Content data in string format.
    #[serde(rename="dataText")]
    
    pub data_text: Option<String>,
    /// Optional. Description of the content.
    
    pub description: Option<String>,
    /// Optional. User defined labels for the content.
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. The relative resource name of the content, of the form: projects/{project_id}/locations/{location_id}/lakes/{lake_id}/content/{content_id}
    
    pub name: Option<String>,
    /// Notebook related configurations.
    
    pub notebook: Option<GoogleCloudDataplexV1ContentNotebook>,
    /// Required. The path for the Content file, represented as directory structure. Unique within a lake. Limited to alphanumerics, hyphens, underscores, dots and slashes.
    
    pub path: Option<String>,
    /// Sql Script related configurations.
    #[serde(rename="sqlScript")]
    
    pub sql_script: Option<GoogleCloudDataplexV1ContentSqlScript>,
    /// Output only. System generated globally unique ID for the content. This ID will be different if the content is deleted and re-created with the same name.
    
    pub uid: Option<String>,
    /// Output only. The time when the content was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GoogleCloudDataplexV1Content {}
impl client::ResponseResult for GoogleCloudDataplexV1Content {}


/// Configuration for Notebook content.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1ContentNotebook {
    /// Required. Kernel Type of the notebook.
    #[serde(rename="kernelType")]
    
    pub kernel_type: Option<String>,
}

impl client::Part for GoogleCloudDataplexV1ContentNotebook {}


/// Configuration for the Sql Script content.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1ContentSqlScript {
    /// Required. Query Engine to be used for the Sql Query.
    
    pub engine: Option<String>,
}

impl client::Part for GoogleCloudDataplexV1ContentSqlScript {}


/// DataAccessSpec holds the access control configuration to be enforced on data stored within resources (eg: rows, columns in BigQuery Tables). When associated with data,the data is only accessible to principals explicitly granted access through the DataAttribute. Principals with access to the containing resource are not implicitly granted access.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1DataAccessSpec {
    /// Optional. The format of strings follows the pattern followed by IAM in the bindings. user:{email}, serviceAccount:{email} group:{email}. The set of principals to be granted reader role on data stored within resources.
    
    pub readers: Option<Vec<String>>,
}

impl client::Part for GoogleCloudDataplexV1DataAccessSpec {}


/// Denotes one dataAttribute in a dataTaxonomy, for example, PII. DataAttribute resources can be defined in a hierarchy. A single dataAttribute resource can contain specs of multiple types PII - ResourceAccessSpec : - readers :foo@bar.com - DataAccessSpec : - readers :bar@foo.com 
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations data taxonomies attributes create projects](ProjectLocationDataTaxonomyAttributeCreateCall) (request)
/// * [locations data taxonomies attributes get projects](ProjectLocationDataTaxonomyAttributeGetCall) (response)
/// * [locations data taxonomies attributes patch projects](ProjectLocationDataTaxonomyAttributePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1DataAttribute {
    /// Output only. The number of child attributes present for this attribute.
    #[serde(rename="attributeCount")]
    
    pub attribute_count: Option<i32>,
    /// Output only. The time when the DataAttribute was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Specified when applied to data stored on the resource (eg: rows, columns in BigQuery Tables).
    #[serde(rename="dataAccessSpec")]
    
    pub data_access_spec: Option<GoogleCloudDataplexV1DataAccessSpec>,
    /// Optional. Description of the DataAttribute.
    
    pub description: Option<String>,
    /// Optional. User friendly display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
    
    pub etag: Option<String>,
    /// Optional. User-defined labels for the DataAttribute.
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. The relative resource name of the dataAttribute, of the form: projects/{project_number}/locations/{location_id}/dataTaxonomies/{dataTaxonomy}/attributes/{data_attribute_id}.
    
    pub name: Option<String>,
    /// Optional. The ID of the parent DataAttribute resource, should belong to the same data taxonomy. Circular dependency in parent chain is not valid. Maximum depth of the hierarchy allowed is 4. a -> b -> c -> d -> e, depth = 4
    #[serde(rename="parentId")]
    
    pub parent_id: Option<String>,
    /// Optional. Specified when applied to a resource (eg: Cloud Storage bucket, BigQuery dataset, BigQuery table).
    #[serde(rename="resourceAccessSpec")]
    
    pub resource_access_spec: Option<GoogleCloudDataplexV1ResourceAccessSpec>,
    /// Output only. System generated globally unique ID for the DataAttribute. This ID will be different if the DataAttribute is deleted and re-created with the same name.
    
    pub uid: Option<String>,
    /// Output only. The time when the DataAttribute was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GoogleCloudDataplexV1DataAttribute {}
impl client::ResponseResult for GoogleCloudDataplexV1DataAttribute {}


/// DataAttributeBinding represents binding of attributes to resources. Eg: Bind ‘CustomerInfo’ entity with ‘PII’ attribute.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations data attribute bindings create projects](ProjectLocationDataAttributeBindingCreateCall) (request)
/// * [locations data attribute bindings get projects](ProjectLocationDataAttributeBindingGetCall) (response)
/// * [locations data attribute bindings patch projects](ProjectLocationDataAttributeBindingPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1DataAttributeBinding {
    /// Optional. List of attributes to be associated with the resource, provided in the form: projects/{project}/locations/{location}/dataTaxonomies/{dataTaxonomy}/attributes/{data_attribute_id}
    
    pub attributes: Option<Vec<String>>,
    /// Output only. The time when the DataAttributeBinding was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Description of the DataAttributeBinding.
    
    pub description: Option<String>,
    /// Optional. User friendly display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. Etags must be used when calling the DeleteDataAttributeBinding and the UpdateDataAttributeBinding method.
    
    pub etag: Option<String>,
    /// Optional. User-defined labels for the DataAttributeBinding.
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. The relative resource name of the Data Attribute Binding, of the form: projects/{project_number}/locations/{location}/dataAttributeBindings/{data_attribute_binding_id}
    
    pub name: Option<String>,
    /// Optional. The list of paths for items within the associated resource (eg. columns within a table) along with attribute bindings.
    
    pub paths: Option<Vec<GoogleCloudDataplexV1DataAttributeBindingPath>>,
    /// Optional. Immutable. The resource name of the resource that is binded to attributes. Presently, only entity resource is supported in the form: projects/{project}/locations/{location}/lakes/{lake}/zones/{zone}/entities/{entity_id} Must belong in the same project and region as the attribute binding, and there can only exist one active binding for a resource.
    
    pub resource: Option<String>,
    /// Output only. System generated globally unique ID for the DataAttributeBinding. This ID will be different if the DataAttributeBinding is deleted and re-created with the same name.
    
    pub uid: Option<String>,
    /// Output only. The time when the DataAttributeBinding was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GoogleCloudDataplexV1DataAttributeBinding {}
impl client::ResponseResult for GoogleCloudDataplexV1DataAttributeBinding {}


/// Represents a subresource of a given resource, and associated bindings with it.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1DataAttributeBindingPath {
    /// Optional. List of attributes to be associated with the path of the resource, provided in the form: projects/{project}/locations/{location}/dataTaxonomies/{dataTaxonomy}/attributes/{data_attribute_id}
    
    pub attributes: Option<Vec<String>>,
    /// Required. The name identifier of the path. Nested columns should be of the form: 'country.state.city'.
    
    pub name: Option<String>,
}

impl client::Part for GoogleCloudDataplexV1DataAttributeBindingPath {}


/// DataProfileResult defines the output of DataProfileScan. Each field of the table will have field type specific profile result.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1DataProfileResult {
    /// The profile information per field.
    
    pub profile: Option<GoogleCloudDataplexV1DataProfileResultProfile>,
    /// The count of rows scanned.
    #[serde(rename="rowCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub row_count: Option<i64>,
    /// The data scanned for this result.
    #[serde(rename="scannedData")]
    
    pub scanned_data: Option<GoogleCloudDataplexV1ScannedData>,
}

impl client::Part for GoogleCloudDataplexV1DataProfileResult {}


/// Contains name, type, mode and field type specific profile information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1DataProfileResultProfile {
    /// List of fields with structural and profile information for each field.
    
    pub fields: Option<Vec<GoogleCloudDataplexV1DataProfileResultProfileField>>,
}

impl client::Part for GoogleCloudDataplexV1DataProfileResultProfile {}


/// A field within a table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1DataProfileResultProfileField {
    /// The mode of the field. Possible values include: REQUIRED, if it is a required field. NULLABLE, if it is an optional field. REPEATED, if it is a repeated field.
    
    pub mode: Option<String>,
    /// The name of the field.
    
    pub name: Option<String>,
    /// Profile information for the corresponding field.
    
    pub profile: Option<GoogleCloudDataplexV1DataProfileResultProfileFieldProfileInfo>,
    /// The field data type. Possible values include: STRING BYTE INT64 INT32 INT16 DOUBLE FLOAT DECIMAL BOOLEAN BINARY TIMESTAMP DATE TIME NULL RECORD
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleCloudDataplexV1DataProfileResultProfileField {}


/// The profile information for each field type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1DataProfileResultProfileFieldProfileInfo {
    /// Ratio of rows with distinct values against total scanned rows. Not available for complex non-groupable field type RECORD and fields with REPEATABLE mode.
    #[serde(rename="distinctRatio")]
    
    pub distinct_ratio: Option<f64>,
    /// Double type field information.
    #[serde(rename="doubleProfile")]
    
    pub double_profile: Option<GoogleCloudDataplexV1DataProfileResultProfileFieldProfileInfoDoubleFieldInfo>,
    /// Integer type field information.
    #[serde(rename="integerProfile")]
    
    pub integer_profile: Option<GoogleCloudDataplexV1DataProfileResultProfileFieldProfileInfoIntegerFieldInfo>,
    /// Ratio of rows with null value against total scanned rows.
    #[serde(rename="nullRatio")]
    
    pub null_ratio: Option<f64>,
    /// String type field information.
    #[serde(rename="stringProfile")]
    
    pub string_profile: Option<GoogleCloudDataplexV1DataProfileResultProfileFieldProfileInfoStringFieldInfo>,
    /// The list of top N non-null values and number of times they occur in the scanned data. N is 10 or equal to the number of distinct values in the field, whichever is smaller. Not available for complex non-groupable field type RECORD and fields with REPEATABLE mode.
    #[serde(rename="topNValues")]
    
    pub top_n_values: Option<Vec<GoogleCloudDataplexV1DataProfileResultProfileFieldProfileInfoTopNValue>>,
}

impl client::Part for GoogleCloudDataplexV1DataProfileResultProfileFieldProfileInfo {}


/// The profile information for a double type field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1DataProfileResultProfileFieldProfileInfoDoubleFieldInfo {
    /// Average of non-null values in the scanned data. NaN, if the field has a NaN.
    
    pub average: Option<f64>,
    /// Maximum of non-null values in the scanned data. NaN, if the field has a NaN.
    
    pub max: Option<f64>,
    /// Minimum of non-null values in the scanned data. NaN, if the field has a NaN.
    
    pub min: Option<f64>,
    /// A quartile divides the number of data points into four parts, or quarters, of more-or-less equal size. Three main quartiles used are: The first quartile (Q1) splits off the lowest 25% of data from the highest 75%. It is also known as the lower or 25th empirical quartile, as 25% of the data is below this point. The second quartile (Q2) is the median of a data set. So, 50% of the data lies below this point. The third quartile (Q3) splits off the highest 25% of data from the lowest 75%. It is known as the upper or 75th empirical quartile, as 75% of the data lies below this point. Here, the quartiles is provided as an ordered list of quartile values for the scanned data, occurring in order Q1, median, Q3.
    
    pub quartiles: Option<Vec<f64>>,
    /// Standard deviation of non-null values in the scanned data. NaN, if the field has a NaN.
    #[serde(rename="standardDeviation")]
    
    pub standard_deviation: Option<f64>,
}

impl client::Part for GoogleCloudDataplexV1DataProfileResultProfileFieldProfileInfoDoubleFieldInfo {}


/// The profile information for an integer type field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1DataProfileResultProfileFieldProfileInfoIntegerFieldInfo {
    /// Average of non-null values in the scanned data. NaN, if the field has a NaN.
    
    pub average: Option<f64>,
    /// Maximum of non-null values in the scanned data. NaN, if the field has a NaN.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max: Option<i64>,
    /// Minimum of non-null values in the scanned data. NaN, if the field has a NaN.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub min: Option<i64>,
    /// A quartile divides the number of data points into four parts, or quarters, of more-or-less equal size. Three main quartiles used are: The first quartile (Q1) splits off the lowest 25% of data from the highest 75%. It is also known as the lower or 25th empirical quartile, as 25% of the data is below this point. The second quartile (Q2) is the median of a data set. So, 50% of the data lies below this point. The third quartile (Q3) splits off the highest 25% of data from the lowest 75%. It is known as the upper or 75th empirical quartile, as 75% of the data lies below this point. Here, the quartiles is provided as an ordered list of quartile values for the scanned data, occurring in order Q1, median, Q3.
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub quartiles: Option<Vec<i64>>,
    /// Standard deviation of non-null values in the scanned data. NaN, if the field has a NaN.
    #[serde(rename="standardDeviation")]
    
    pub standard_deviation: Option<f64>,
}

impl client::Part for GoogleCloudDataplexV1DataProfileResultProfileFieldProfileInfoIntegerFieldInfo {}


/// The profile information for a string type field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1DataProfileResultProfileFieldProfileInfoStringFieldInfo {
    /// Average length of non-null values in the scanned data.
    #[serde(rename="averageLength")]
    
    pub average_length: Option<f64>,
    /// Maximum length of non-null values in the scanned data.
    #[serde(rename="maxLength")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_length: Option<i64>,
    /// Minimum length of non-null values in the scanned data.
    #[serde(rename="minLength")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub min_length: Option<i64>,
}

impl client::Part for GoogleCloudDataplexV1DataProfileResultProfileFieldProfileInfoStringFieldInfo {}


/// Top N non-null values in the scanned data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1DataProfileResultProfileFieldProfileInfoTopNValue {
    /// Count of the corresponding value in the scanned data.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
    /// String value of a top N non-null value.
    
    pub value: Option<String>,
}

impl client::Part for GoogleCloudDataplexV1DataProfileResultProfileFieldProfileInfoTopNValue {}


/// DataProfileScan related setting.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1DataProfileSpec { _never_set: Option<bool> }

impl client::Part for GoogleCloudDataplexV1DataProfileSpec {}


/// DataQualityDimensionResult provides a more detailed, per-dimension view of the results.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1DataQualityDimensionResult {
    /// Whether the dimension passed or failed.
    
    pub passed: Option<bool>,
}

impl client::Part for GoogleCloudDataplexV1DataQualityDimensionResult {}


/// The output of a DataQualityScan.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1DataQualityResult {
    /// A list of results at the dimension level.
    
    pub dimensions: Option<Vec<GoogleCloudDataplexV1DataQualityDimensionResult>>,
    /// Overall data quality result -- true if all rules passed.
    
    pub passed: Option<bool>,
    /// The count of rows processed.
    #[serde(rename="rowCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub row_count: Option<i64>,
    /// A list of all the rules in a job, and their results.
    
    pub rules: Option<Vec<GoogleCloudDataplexV1DataQualityRuleResult>>,
    /// The data scanned for this result.
    #[serde(rename="scannedData")]
    
    pub scanned_data: Option<GoogleCloudDataplexV1ScannedData>,
}

impl client::Part for GoogleCloudDataplexV1DataQualityResult {}


/// A rule captures data quality intent about a data source.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1DataQualityRule {
    /// Optional. The unnested column which this rule is evaluated against.
    
    pub column: Option<String>,
    /// Required. The dimension a rule belongs to. Results are also aggregated at the dimension level. Supported dimensions are "COMPLETENESS", "ACCURACY", "CONSISTENCY", "VALIDITY", "UNIQUENESS", "INTEGRITY"
    
    pub dimension: Option<String>,
    /// Optional. Rows with null values will automatically fail a rule, unless ignore_null is true. In that case, such null rows are trivially considered passing.Only applicable to ColumnMap rules.
    #[serde(rename="ignoreNull")]
    
    pub ignore_null: Option<bool>,
    /// ColumnMap rule which evaluates whether each column value is null.
    #[serde(rename="nonNullExpectation")]
    
    pub non_null_expectation: Option<GoogleCloudDataplexV1DataQualityRuleNonNullExpectation>,
    /// ColumnMap rule which evaluates whether each column value lies between a specified range.
    #[serde(rename="rangeExpectation")]
    
    pub range_expectation: Option<GoogleCloudDataplexV1DataQualityRuleRangeExpectation>,
    /// ColumnMap rule which evaluates whether each column value matches a specified regex.
    #[serde(rename="regexExpectation")]
    
    pub regex_expectation: Option<GoogleCloudDataplexV1DataQualityRuleRegexExpectation>,
    /// Table rule which evaluates whether each row passes the specified condition.
    #[serde(rename="rowConditionExpectation")]
    
    pub row_condition_expectation: Option<GoogleCloudDataplexV1DataQualityRuleRowConditionExpectation>,
    /// ColumnMap rule which evaluates whether each column value is contained by a specified set.
    #[serde(rename="setExpectation")]
    
    pub set_expectation: Option<GoogleCloudDataplexV1DataQualityRuleSetExpectation>,
    /// ColumnAggregate rule which evaluates whether the column aggregate statistic lies between a specified range.
    #[serde(rename="statisticRangeExpectation")]
    
    pub statistic_range_expectation: Option<GoogleCloudDataplexV1DataQualityRuleStatisticRangeExpectation>,
    /// Table rule which evaluates whether the provided expression is true.
    #[serde(rename="tableConditionExpectation")]
    
    pub table_condition_expectation: Option<GoogleCloudDataplexV1DataQualityRuleTableConditionExpectation>,
    /// Optional. The minimum ratio of passing_rows / total_rows required to pass this rule, with a range of 0.0, 1.0.0 indicates default value (i.e. 1.0).
    
    pub threshold: Option<f64>,
    /// ColumnAggregate rule which evaluates whether the column has duplicates.
    #[serde(rename="uniquenessExpectation")]
    
    pub uniqueness_expectation: Option<GoogleCloudDataplexV1DataQualityRuleUniquenessExpectation>,
}

impl client::Part for GoogleCloudDataplexV1DataQualityRule {}


/// Evaluates whether each column value is null.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1DataQualityRuleNonNullExpectation { _never_set: Option<bool> }

impl client::Part for GoogleCloudDataplexV1DataQualityRuleNonNullExpectation {}


/// Evaluates whether each column value lies between a specified range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1DataQualityRuleRangeExpectation {
    /// Optional. The maximum column value allowed for a row to pass this validation. At least one of min_value and max_value need to be provided.
    #[serde(rename="maxValue")]
    
    pub max_value: Option<String>,
    /// Optional. The minimum column value allowed for a row to pass this validation. At least one of min_value and max_value need to be provided.
    #[serde(rename="minValue")]
    
    pub min_value: Option<String>,
    /// Optional. Whether each value needs to be strictly lesser than ('<') the maximum, or if equality is allowed.Only relevant if a max_value has been defined. Default = false.
    #[serde(rename="strictMaxEnabled")]
    
    pub strict_max_enabled: Option<bool>,
    /// Optional. Whether each value needs to be strictly greater than ('>') the minimum, or if equality is allowed.Only relevant if a min_value has been defined. Default = false.
    #[serde(rename="strictMinEnabled")]
    
    pub strict_min_enabled: Option<bool>,
}

impl client::Part for GoogleCloudDataplexV1DataQualityRuleRangeExpectation {}


/// Evaluates whether each column value matches a specified regex.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1DataQualityRuleRegexExpectation {
    /// A regular expression the column value is expected to match.
    
    pub regex: Option<String>,
}

impl client::Part for GoogleCloudDataplexV1DataQualityRuleRegexExpectation {}


/// DataQualityRuleResult provides a more detailed, per-rule view of the results.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1DataQualityRuleResult {
    /// The number of rows a rule was evaluated against. This field is only valid for ColumnMap type rules.Evaluated count can be configured to either include all rows (default) - with null rows automatically failing rule evaluation, or exclude null rows from the evaluated_count, by setting ignore_nulls = true.
    #[serde(rename="evaluatedCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub evaluated_count: Option<i64>,
    /// The query to find rows that did not pass this rule. Only applies to ColumnMap and RowCondition rules.
    #[serde(rename="failingRowsQuery")]
    
    pub failing_rows_query: Option<String>,
    /// The number of rows with null values in the specified column.
    #[serde(rename="nullCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub null_count: Option<i64>,
    /// The ratio of passed_count / evaluated_count. This field is only valid for ColumnMap type rules.
    #[serde(rename="passRatio")]
    
    pub pass_ratio: Option<f64>,
    /// Whether the rule passed or failed.
    
    pub passed: Option<bool>,
    /// The number of rows which passed a rule evaluation. This field is only valid for ColumnMap type rules.
    #[serde(rename="passedCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub passed_count: Option<i64>,
    /// The rule specified in the DataQualitySpec, as is.
    
    pub rule: Option<GoogleCloudDataplexV1DataQualityRule>,
}

impl client::Part for GoogleCloudDataplexV1DataQualityRuleResult {}


/// Evaluates whether each row passes the specified condition.The SQL expression needs to use BigQuery standard SQL syntax and should produce a boolean value per row as the result.Example: col1 >= 0 AND col2 < 10
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1DataQualityRuleRowConditionExpectation {
    /// The SQL expression.
    #[serde(rename="sqlExpression")]
    
    pub sql_expression: Option<String>,
}

impl client::Part for GoogleCloudDataplexV1DataQualityRuleRowConditionExpectation {}


/// Evaluates whether each column value is contained by a specified set.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1DataQualityRuleSetExpectation {
    /// Expected values for the column value.
    
    pub values: Option<Vec<String>>,
}

impl client::Part for GoogleCloudDataplexV1DataQualityRuleSetExpectation {}


/// Evaluates whether the column aggregate statistic lies between a specified range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1DataQualityRuleStatisticRangeExpectation {
    /// The maximum column statistic value allowed for a row to pass this validation.At least one of min_value and max_value need to be provided.
    #[serde(rename="maxValue")]
    
    pub max_value: Option<String>,
    /// The minimum column statistic value allowed for a row to pass this validation.At least one of min_value and max_value need to be provided.
    #[serde(rename="minValue")]
    
    pub min_value: Option<String>,
    /// no description provided
    
    pub statistic: Option<String>,
    /// Whether column statistic needs to be strictly lesser than ('<') the maximum, or if equality is allowed.Only relevant if a max_value has been defined. Default = false.
    #[serde(rename="strictMaxEnabled")]
    
    pub strict_max_enabled: Option<bool>,
    /// Whether column statistic needs to be strictly greater than ('>') the minimum, or if equality is allowed.Only relevant if a min_value has been defined. Default = false.
    #[serde(rename="strictMinEnabled")]
    
    pub strict_min_enabled: Option<bool>,
}

impl client::Part for GoogleCloudDataplexV1DataQualityRuleStatisticRangeExpectation {}


/// Evaluates whether the provided expression is true.The SQL expression needs to use BigQuery standard SQL syntax and should produce a scalar boolean result.Example: MIN(col1) >= 0
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1DataQualityRuleTableConditionExpectation {
    /// The SQL expression.
    #[serde(rename="sqlExpression")]
    
    pub sql_expression: Option<String>,
}

impl client::Part for GoogleCloudDataplexV1DataQualityRuleTableConditionExpectation {}


/// Evaluates whether the column has duplicates.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1DataQualityRuleUniquenessExpectation { _never_set: Option<bool> }

impl client::Part for GoogleCloudDataplexV1DataQualityRuleUniquenessExpectation {}


/// DataQualityScan related setting.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1DataQualitySpec {
    /// The list of rules to evaluate against a data source. At least one rule is required.
    
    pub rules: Option<Vec<GoogleCloudDataplexV1DataQualityRule>>,
}

impl client::Part for GoogleCloudDataplexV1DataQualitySpec {}


/// Represents a user-visible job which provides the insights for the related data source.For example: Data Quality: generates queries based on the rules and runs against the data to get data quality check results. Data Profile: analyzes the data in table(s) and generates insights about the structure, content and relationships (such as null percent, cardinality, min/max/mean, etc).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations data scans create projects](ProjectLocationDataScanCreateCall) (request)
/// * [locations data scans get projects](ProjectLocationDataScanGetCall) (response)
/// * [locations data scans patch projects](ProjectLocationDataScanPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1DataScan {
    /// Output only. The time when the scan was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. The data source for DataScan.
    
    pub data: Option<GoogleCloudDataplexV1DataSource>,
    /// Output only. The result of the data profile scan.
    #[serde(rename="dataProfileResult")]
    
    pub data_profile_result: Option<GoogleCloudDataplexV1DataProfileResult>,
    /// DataProfileScan related setting.
    #[serde(rename="dataProfileSpec")]
    
    pub data_profile_spec: Option<GoogleCloudDataplexV1DataProfileSpec>,
    /// Output only. The result of the data quality scan.
    #[serde(rename="dataQualityResult")]
    
    pub data_quality_result: Option<GoogleCloudDataplexV1DataQualityResult>,
    /// DataQualityScan related setting.
    #[serde(rename="dataQualitySpec")]
    
    pub data_quality_spec: Option<GoogleCloudDataplexV1DataQualitySpec>,
    /// Optional. Description of the scan. Must be between 1-1024 characters.
    
    pub description: Option<String>,
    /// Optional. User friendly display name. Must be between 1-256 characters.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Optional. DataScan execution settings.If not specified, the fields in it will use their default values.
    #[serde(rename="executionSpec")]
    
    pub execution_spec: Option<GoogleCloudDataplexV1DataScanExecutionSpec>,
    /// Output only. Status of the data scan execution.
    #[serde(rename="executionStatus")]
    
    pub execution_status: Option<GoogleCloudDataplexV1DataScanExecutionStatus>,
    /// Optional. User-defined labels for the scan.
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. The relative resource name of the scan, of the form: projects/{project}/locations/{location_id}/dataScans/{datascan_id}, where project refers to a project_id or project_number and location_id refers to a GCP region.
    
    pub name: Option<String>,
    /// Output only. Current state of the DataScan.
    
    pub state: Option<String>,
    /// Output only. The type of DataScan.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Output only. System generated globally unique ID for the scan. This ID will be different if the scan is deleted and re-created with the same name.
    
    pub uid: Option<String>,
    /// Output only. The time when the scan was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GoogleCloudDataplexV1DataScan {}
impl client::ResponseResult for GoogleCloudDataplexV1DataScan {}


/// DataScan execution settings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1DataScanExecutionSpec {
    /// Immutable. The unnested field (of type Date or Timestamp) that contains values which monotonically increase over time.If not specified, a data scan will run for all data in the table.
    
    pub field: Option<String>,
    /// Optional. Spec related to how often and when a scan should be triggered.If not specified, the default is OnDemand, which means the scan will not run until the user calls RunDataScan API.
    
    pub trigger: Option<GoogleCloudDataplexV1Trigger>,
}

impl client::Part for GoogleCloudDataplexV1DataScanExecutionSpec {}


/// Status of the data scan execution.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1DataScanExecutionStatus {
    /// The time when the latest DataScanJob ended.
    #[serde(rename="latestJobEndTime")]
    
    pub latest_job_end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The time when the latest DataScanJob started.
    #[serde(rename="latestJobStartTime")]
    
    pub latest_job_start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GoogleCloudDataplexV1DataScanExecutionStatus {}


/// A DataScanJob represents an instance of DataScan execution.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations data scans jobs get projects](ProjectLocationDataScanJobGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1DataScanJob {
    /// Output only. The result of the data profile scan.
    #[serde(rename="dataProfileResult")]
    
    pub data_profile_result: Option<GoogleCloudDataplexV1DataProfileResult>,
    /// Output only. DataProfileScan related setting.
    #[serde(rename="dataProfileSpec")]
    
    pub data_profile_spec: Option<GoogleCloudDataplexV1DataProfileSpec>,
    /// Output only. The result of the data quality scan.
    #[serde(rename="dataQualityResult")]
    
    pub data_quality_result: Option<GoogleCloudDataplexV1DataQualityResult>,
    /// Output only. DataQualityScan related setting.
    #[serde(rename="dataQualitySpec")]
    
    pub data_quality_spec: Option<GoogleCloudDataplexV1DataQualitySpec>,
    /// Output only. The time when the DataScanJob ended.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Additional information about the current state.
    
    pub message: Option<String>,
    /// Output only. The relative resource name of the DataScanJob, of the form: projects/{project}/locations/{location_id}/dataScans/{datascan_id}/jobs/{job_id}, where project refers to a project_id or project_number and location_id refers to a GCP region.
    
    pub name: Option<String>,
    /// Output only. The time when the DataScanJob was started.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Execution state for the DataScanJob.
    
    pub state: Option<String>,
    /// Output only. The type of the parent DataScan.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Output only. System generated globally unique ID for the DataScanJob.
    
    pub uid: Option<String>,
}

impl client::ResponseResult for GoogleCloudDataplexV1DataScanJob {}


/// The data source for DataScan.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1DataSource {
    /// Immutable. The Dataplex entity that represents the data source (e.g. BigQuery table) for DataScan, of the form: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}/entities/{entity_id}.
    
    pub entity: Option<String>,
}

impl client::Part for GoogleCloudDataplexV1DataSource {}


/// DataTaxonomy represents a set of hierarchical DataAttributes resources, grouped with a common theme Eg: ‘SensitiveDataTaxonomy’ can have attributes to manage PII data. It is defined at project level.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations data taxonomies create projects](ProjectLocationDataTaxonomyCreateCall) (request)
/// * [locations data taxonomies get projects](ProjectLocationDataTaxonomyGetCall) (response)
/// * [locations data taxonomies patch projects](ProjectLocationDataTaxonomyPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1DataTaxonomy {
    /// Output only. The number of attributes in the DataTaxonomy.
    #[serde(rename="attributeCount")]
    
    pub attribute_count: Option<i32>,
    /// Output only. The time when the DataTaxonomy was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Description of the DataTaxonomy.
    
    pub description: Option<String>,
    /// Optional. User friendly display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
    
    pub etag: Option<String>,
    /// Optional. User-defined labels for the DataTaxonomy.
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. The relative resource name of the DataTaxonomy, of the form: projects/{project_number}/locations/{location_id}/dataTaxonomies/{data_taxonomy_id}.
    
    pub name: Option<String>,
    /// Output only. System generated globally unique ID for the dataTaxonomy. This ID will be different if the DataTaxonomy is deleted and re-created with the same name.
    
    pub uid: Option<String>,
    /// Output only. The time when the DataTaxonomy was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GoogleCloudDataplexV1DataTaxonomy {}
impl client::ResponseResult for GoogleCloudDataplexV1DataTaxonomy {}


/// Represents tables and fileset metadata contained within a zone.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations lakes zones entities create projects](ProjectLocationLakeZoneEntityCreateCall) (request|response)
/// * [locations lakes zones entities get projects](ProjectLocationLakeZoneEntityGetCall) (response)
/// * [locations lakes zones entities update projects](ProjectLocationLakeZoneEntityUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1Entity {
    /// Output only. Identifies the access mechanism to the entity. Not user settable.
    
    pub access: Option<GoogleCloudDataplexV1StorageAccess>,
    /// Required. Immutable. The ID of the asset associated with the storage location containing the entity data. The entity must be with in the same zone with the asset.
    
    pub asset: Option<String>,
    /// Output only. The name of the associated Data Catalog entry.
    #[serde(rename="catalogEntry")]
    
    pub catalog_entry: Option<String>,
    /// Output only. Metadata stores that the entity is compatible with.
    
    pub compatibility: Option<GoogleCloudDataplexV1EntityCompatibilityStatus>,
    /// Output only. The time when the entity was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. Immutable. The storage path of the entity data. For Cloud Storage data, this is the fully-qualified path to the entity, such as gs://bucket/path/to/data. For BigQuery data, this is the name of the table resource, such as projects/project_id/datasets/dataset_id/tables/table_id.
    #[serde(rename="dataPath")]
    
    pub data_path: Option<String>,
    /// Optional. The set of items within the data path constituting the data in the entity, represented as a glob path. Example: gs://bucket/path/to/data/**/*.csv.
    #[serde(rename="dataPathPattern")]
    
    pub data_path_pattern: Option<String>,
    /// Optional. User friendly longer description text. Must be shorter than or equal to 1024 characters.
    
    pub description: Option<String>,
    /// Optional. Display name must be shorter than or equal to 256 characters.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Optional. The etag associated with the entity, which can be retrieved with a GetEntity request. Required for update and delete requests.
    
    pub etag: Option<String>,
    /// Required. Identifies the storage format of the entity data. It does not apply to entities with data stored in BigQuery.
    
    pub format: Option<GoogleCloudDataplexV1StorageFormat>,
    /// Required. A user-provided entity ID. It is mutable, and will be used as the published table name. Specifying a new ID in an update entity request will override the existing value. The ID must contain only letters (a-z, A-Z), numbers (0-9), and underscores. Must begin with a letter and consist of 256 or fewer characters.
    
    pub id: Option<String>,
    /// Output only. The resource name of the entity, of the form: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}/entities/{id}.
    
    pub name: Option<String>,
    /// Required. The description of the data structure and layout. The schema is not included in list responses. It is only included in SCHEMA and FULL entity views of a GetEntity response.
    
    pub schema: Option<GoogleCloudDataplexV1Schema>,
    /// Required. Immutable. Identifies the storage system of the entity data.
    
    pub system: Option<String>,
    /// Required. Immutable. The type of entity.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Output only. System generated unique ID for the Entity. This ID will be different if the Entity is deleted and re-created with the same name.
    
    pub uid: Option<String>,
    /// Output only. The time when the entity was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GoogleCloudDataplexV1Entity {}
impl client::ResponseResult for GoogleCloudDataplexV1Entity {}


/// Provides compatibility information for various metadata stores.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1EntityCompatibilityStatus {
    /// Output only. Whether this entity is compatible with BigQuery.
    
    pub bigquery: Option<GoogleCloudDataplexV1EntityCompatibilityStatusCompatibility>,
    /// Output only. Whether this entity is compatible with Hive Metastore.
    #[serde(rename="hiveMetastore")]
    
    pub hive_metastore: Option<GoogleCloudDataplexV1EntityCompatibilityStatusCompatibility>,
}

impl client::Part for GoogleCloudDataplexV1EntityCompatibilityStatus {}


/// Provides compatibility information for a specific metadata store.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1EntityCompatibilityStatusCompatibility {
    /// Output only. Whether the entity is compatible and can be represented in the metadata store.
    
    pub compatible: Option<bool>,
    /// Output only. Provides additional detail if the entity is incompatible with the metadata store.
    
    pub reason: Option<String>,
}

impl client::Part for GoogleCloudDataplexV1EntityCompatibilityStatusCompatibility {}


/// Environment represents a user-visible compute infrastructure for analytics within a lake.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations lakes environments create projects](ProjectLocationLakeEnvironmentCreateCall) (request)
/// * [locations lakes environments get projects](ProjectLocationLakeEnvironmentGetCall) (response)
/// * [locations lakes environments patch projects](ProjectLocationLakeEnvironmentPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1Environment {
    /// Output only. Environment creation time.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Description of the environment.
    
    pub description: Option<String>,
    /// Optional. User friendly display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. URI Endpoints to access sessions associated with the Environment.
    
    pub endpoints: Option<GoogleCloudDataplexV1EnvironmentEndpoints>,
    /// Required. Infrastructure specification for the Environment.
    #[serde(rename="infrastructureSpec")]
    
    pub infrastructure_spec: Option<GoogleCloudDataplexV1EnvironmentInfrastructureSpec>,
    /// Optional. User defined labels for the environment.
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. The relative resource name of the environment, of the form: projects/{project_id}/locations/{location_id}/lakes/{lake_id}/environment/{environment_id}
    
    pub name: Option<String>,
    /// Optional. Configuration for sessions created for this environment.
    #[serde(rename="sessionSpec")]
    
    pub session_spec: Option<GoogleCloudDataplexV1EnvironmentSessionSpec>,
    /// Output only. Status of sessions created for this environment.
    #[serde(rename="sessionStatus")]
    
    pub session_status: Option<GoogleCloudDataplexV1EnvironmentSessionStatus>,
    /// Output only. Current state of the environment.
    
    pub state: Option<String>,
    /// Output only. System generated globally unique ID for the environment. This ID will be different if the environment is deleted and re-created with the same name.
    
    pub uid: Option<String>,
    /// Output only. The time when the environment was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GoogleCloudDataplexV1Environment {}
impl client::ResponseResult for GoogleCloudDataplexV1Environment {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1EnvironmentEndpoints {
    /// Output only. URI to serve notebook APIs
    
    pub notebooks: Option<String>,
    /// Output only. URI to serve SQL APIs
    
    pub sql: Option<String>,
}

impl client::Part for GoogleCloudDataplexV1EnvironmentEndpoints {}


/// Configuration for the underlying infrastructure used to run workloads.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1EnvironmentInfrastructureSpec {
    /// Optional. Compute resources needed for analyze interactive workloads.
    
    pub compute: Option<GoogleCloudDataplexV1EnvironmentInfrastructureSpecComputeResources>,
    /// Required. Software Runtime Configuration for analyze interactive workloads.
    #[serde(rename="osImage")]
    
    pub os_image: Option<GoogleCloudDataplexV1EnvironmentInfrastructureSpecOsImageRuntime>,
}

impl client::Part for GoogleCloudDataplexV1EnvironmentInfrastructureSpec {}


/// Compute resources associated with the analyze interactive workloads.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1EnvironmentInfrastructureSpecComputeResources {
    /// Optional. Size in GB of the disk. Default is 100 GB.
    #[serde(rename="diskSizeGb")]
    
    pub disk_size_gb: Option<i32>,
    /// Optional. Max configurable nodes. If max_node_count > node_count, then auto-scaling is enabled.
    #[serde(rename="maxNodeCount")]
    
    pub max_node_count: Option<i32>,
    /// Optional. Total number of nodes in the sessions created for this environment.
    #[serde(rename="nodeCount")]
    
    pub node_count: Option<i32>,
}

impl client::Part for GoogleCloudDataplexV1EnvironmentInfrastructureSpecComputeResources {}


/// Software Runtime Configuration to run Analyze.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1EnvironmentInfrastructureSpecOsImageRuntime {
    /// Required. Dataplex Image version.
    #[serde(rename="imageVersion")]
    
    pub image_version: Option<String>,
    /// Optional. List of Java jars to be included in the runtime environment. Valid input includes Cloud Storage URIs to Jar binaries. For example, gs://bucket-name/my/path/to/file.jar
    #[serde(rename="javaLibraries")]
    
    pub java_libraries: Option<Vec<String>>,
    /// Optional. Spark properties to provide configuration for use in sessions created for this environment. The properties to set on daemon config files. Property keys are specified in prefix:property format. The prefix must be "spark".
    
    pub properties: Option<HashMap<String, String>>,
    /// Optional. A list of python packages to be installed. Valid formats include Cloud Storage URI to a PIP installable library. For example, gs://bucket-name/my/path/to/lib.tar.gz
    #[serde(rename="pythonPackages")]
    
    pub python_packages: Option<Vec<String>>,
}

impl client::Part for GoogleCloudDataplexV1EnvironmentInfrastructureSpecOsImageRuntime {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1EnvironmentSessionSpec {
    /// Optional. If True, this causes sessions to be pre-created and available for faster startup to enable interactive exploration use-cases. This defaults to False to avoid additional billed charges. These can only be set to True for the environment with name set to "default", and with default configuration.
    #[serde(rename="enableFastStartup")]
    
    pub enable_fast_startup: Option<bool>,
    /// Optional. The idle time configuration of the session. The session will be auto-terminated at the end of this period.
    #[serde(rename="maxIdleDuration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub max_idle_duration: Option<client::chrono::Duration>,
}

impl client::Part for GoogleCloudDataplexV1EnvironmentSessionSpec {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1EnvironmentSessionStatus {
    /// Output only. Queries over sessions to mark whether the environment is currently active or not
    
    pub active: Option<bool>,
}

impl client::Part for GoogleCloudDataplexV1EnvironmentSessionStatus {}


/// A job represents an instance of a task.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations lakes tasks jobs get projects](ProjectLocationLakeTaskJobGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1Job {
    /// Output only. The time when the job ended.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Additional information about the current state.
    
    pub message: Option<String>,
    /// Output only. The relative resource name of the job, of the form: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/tasks/{task_id}/jobs/{job_id}.
    
    pub name: Option<String>,
    /// Output only. The number of times the job has been retried (excluding the initial attempt).
    #[serde(rename="retryCount")]
    
    pub retry_count: Option<u32>,
    /// Output only. The underlying service running a job.
    
    pub service: Option<String>,
    /// Output only. The full resource name for the job run under a particular service.
    #[serde(rename="serviceJob")]
    
    pub service_job: Option<String>,
    /// Output only. The time when the job was started.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Execution state for the job.
    
    pub state: Option<String>,
    /// Output only. System generated globally unique ID for the job.
    
    pub uid: Option<String>,
}

impl client::ResponseResult for GoogleCloudDataplexV1Job {}


/// A lake is a centralized repository for managing enterprise data across the organization distributed across many cloud projects, and stored in a variety of storage services such as Google Cloud Storage and BigQuery. The resources attached to a lake are referred to as managed resources. Data within these managed resources can be structured or unstructured. A lake provides data admins with tools to organize, secure and manage their data at scale, and provides data scientists and data engineers an integrated experience to easily search, discover, analyze and transform data and associated metadata.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations lakes create projects](ProjectLocationLakeCreateCall) (request)
/// * [locations lakes get projects](ProjectLocationLakeGetCall) (response)
/// * [locations lakes patch projects](ProjectLocationLakePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1Lake {
    /// Output only. Aggregated status of the underlying assets of the lake.
    #[serde(rename="assetStatus")]
    
    pub asset_status: Option<GoogleCloudDataplexV1AssetStatus>,
    /// Output only. The time when the lake was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Description of the lake.
    
    pub description: Option<String>,
    /// Optional. User friendly display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Optional. User-defined labels for the lake.
    
    pub labels: Option<HashMap<String, String>>,
    /// Optional. Settings to manage lake and Dataproc Metastore service instance association.
    
    pub metastore: Option<GoogleCloudDataplexV1LakeMetastore>,
    /// Output only. Metastore status of the lake.
    #[serde(rename="metastoreStatus")]
    
    pub metastore_status: Option<GoogleCloudDataplexV1LakeMetastoreStatus>,
    /// Output only. The relative resource name of the lake, of the form: projects/{project_number}/locations/{location_id}/lakes/{lake_id}.
    
    pub name: Option<String>,
    /// Output only. Service account associated with this lake. This service account must be authorized to access or operate on resources managed by the lake.
    #[serde(rename="serviceAccount")]
    
    pub service_account: Option<String>,
    /// Output only. Current state of the lake.
    
    pub state: Option<String>,
    /// Output only. System generated globally unique ID for the lake. This ID will be different if the lake is deleted and re-created with the same name.
    
    pub uid: Option<String>,
    /// Output only. The time when the lake was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GoogleCloudDataplexV1Lake {}
impl client::ResponseResult for GoogleCloudDataplexV1Lake {}


/// Settings to manage association of Dataproc Metastore with a lake.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1LakeMetastore {
    /// Optional. A relative reference to the Dataproc Metastore (https://cloud.google.com/dataproc-metastore/docs) service associated with the lake: projects/{project_id}/locations/{location_id}/services/{service_id}
    
    pub service: Option<String>,
}

impl client::Part for GoogleCloudDataplexV1LakeMetastore {}


/// Status of Lake and Dataproc Metastore service instance association.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1LakeMetastoreStatus {
    /// The URI of the endpoint used to access the Metastore service.
    
    pub endpoint: Option<String>,
    /// Additional information about the current status.
    
    pub message: Option<String>,
    /// Current state of association.
    
    pub state: Option<String>,
    /// Last update time of the metastore status of the lake.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GoogleCloudDataplexV1LakeMetastoreStatus {}


/// List actions response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations lakes actions list projects](ProjectLocationLakeActionListCall) (response)
/// * [locations lakes zones actions list projects](ProjectLocationLakeZoneActionListCall) (response)
/// * [locations lakes zones assets actions list projects](ProjectLocationLakeZoneAssetActionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1ListActionsResponse {
    /// Actions under the given parent lake/zone/asset.
    
    pub actions: Option<Vec<GoogleCloudDataplexV1Action>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDataplexV1ListActionsResponse {}


/// List assets response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations lakes zones assets list projects](ProjectLocationLakeZoneAssetListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1ListAssetsResponse {
    /// Asset under the given parent zone.
    
    pub assets: Option<Vec<GoogleCloudDataplexV1Asset>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDataplexV1ListAssetsResponse {}


/// List content response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations lakes content list projects](ProjectLocationLakeContentListCall) (response)
/// * [locations lakes contentitems list projects](ProjectLocationLakeContentitemListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1ListContentResponse {
    /// Content under the given parent lake.
    
    pub content: Option<Vec<GoogleCloudDataplexV1Content>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDataplexV1ListContentResponse {}


/// List DataAttributeBindings response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations data attribute bindings list projects](ProjectLocationDataAttributeBindingListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1ListDataAttributeBindingsResponse {
    /// DataAttributeBindings under the given parent Location.
    #[serde(rename="dataAttributeBindings")]
    
    pub data_attribute_bindings: Option<Vec<GoogleCloudDataplexV1DataAttributeBinding>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    #[serde(rename="unreachableLocations")]
    
    pub unreachable_locations: Option<Vec<String>>,
}

impl client::ResponseResult for GoogleCloudDataplexV1ListDataAttributeBindingsResponse {}


/// List DataAttributes response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations data taxonomies attributes list projects](ProjectLocationDataTaxonomyAttributeListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1ListDataAttributesResponse {
    /// DataAttributes under the given parent DataTaxonomy.
    #[serde(rename="dataAttributes")]
    
    pub data_attributes: Option<Vec<GoogleCloudDataplexV1DataAttribute>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    #[serde(rename="unreachableLocations")]
    
    pub unreachable_locations: Option<Vec<String>>,
}

impl client::ResponseResult for GoogleCloudDataplexV1ListDataAttributesResponse {}


/// List DataScanJobs response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations data scans jobs list projects](ProjectLocationDataScanJobListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1ListDataScanJobsResponse {
    /// DataScanJobs (BASIC view only) under a given dataScan.
    #[serde(rename="dataScanJobs")]
    
    pub data_scan_jobs: Option<Vec<GoogleCloudDataplexV1DataScanJob>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDataplexV1ListDataScanJobsResponse {}


/// List dataScans response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations data scans list projects](ProjectLocationDataScanListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1ListDataScansResponse {
    /// DataScans (BASIC view only) under the given parent location.
    #[serde(rename="dataScans")]
    
    pub data_scans: Option<Vec<GoogleCloudDataplexV1DataScan>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for GoogleCloudDataplexV1ListDataScansResponse {}


/// List DataTaxonomies response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations data taxonomies list projects](ProjectLocationDataTaxonomyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1ListDataTaxonomiesResponse {
    /// DataTaxonomies under the given parent location.
    #[serde(rename="dataTaxonomies")]
    
    pub data_taxonomies: Option<Vec<GoogleCloudDataplexV1DataTaxonomy>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    #[serde(rename="unreachableLocations")]
    
    pub unreachable_locations: Option<Vec<String>>,
}

impl client::ResponseResult for GoogleCloudDataplexV1ListDataTaxonomiesResponse {}


/// List metadata entities response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations lakes zones entities list projects](ProjectLocationLakeZoneEntityListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1ListEntitiesResponse {
    /// Entities in the specified parent zone.
    
    pub entities: Option<Vec<GoogleCloudDataplexV1Entity>>,
    /// Token to retrieve the next page of results, or empty if there are no remaining results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDataplexV1ListEntitiesResponse {}


/// List environments response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations lakes environments list projects](ProjectLocationLakeEnvironmentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1ListEnvironmentsResponse {
    /// Environments under the given parent lake.
    
    pub environments: Option<Vec<GoogleCloudDataplexV1Environment>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDataplexV1ListEnvironmentsResponse {}


/// List jobs response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations lakes tasks jobs list projects](ProjectLocationLakeTaskJobListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1ListJobsResponse {
    /// Jobs under a given task.
    
    pub jobs: Option<Vec<GoogleCloudDataplexV1Job>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDataplexV1ListJobsResponse {}


/// List lakes response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations lakes list projects](ProjectLocationLakeListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1ListLakesResponse {
    /// Lakes under the given parent location.
    
    pub lakes: Option<Vec<GoogleCloudDataplexV1Lake>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    #[serde(rename="unreachableLocations")]
    
    pub unreachable_locations: Option<Vec<String>>,
}

impl client::ResponseResult for GoogleCloudDataplexV1ListLakesResponse {}


/// List metadata partitions response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations lakes zones entities partitions list projects](ProjectLocationLakeZoneEntityPartitionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1ListPartitionsResponse {
    /// Token to retrieve the next page of results, or empty if there are no remaining results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Partitions under the specified parent entity.
    
    pub partitions: Option<Vec<GoogleCloudDataplexV1Partition>>,
}

impl client::ResponseResult for GoogleCloudDataplexV1ListPartitionsResponse {}


/// List sessions response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations lakes environments sessions list projects](ProjectLocationLakeEnvironmentSessionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1ListSessionsResponse {
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Sessions under a given environment.
    
    pub sessions: Option<Vec<GoogleCloudDataplexV1Session>>,
}

impl client::ResponseResult for GoogleCloudDataplexV1ListSessionsResponse {}


/// List tasks response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations lakes tasks list projects](ProjectLocationLakeTaskListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1ListTasksResponse {
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Tasks under the given parent lake.
    
    pub tasks: Option<Vec<GoogleCloudDataplexV1Task>>,
    /// Locations that could not be reached.
    #[serde(rename="unreachableLocations")]
    
    pub unreachable_locations: Option<Vec<String>>,
}

impl client::ResponseResult for GoogleCloudDataplexV1ListTasksResponse {}


/// List zones response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations lakes zones list projects](ProjectLocationLakeZoneListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1ListZonesResponse {
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Zones under the given parent lake.
    
    pub zones: Option<Vec<GoogleCloudDataplexV1Zone>>,
}

impl client::ResponseResult for GoogleCloudDataplexV1ListZonesResponse {}


/// Represents partition metadata contained within entity instances.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations lakes zones entities partitions create projects](ProjectLocationLakeZoneEntityPartitionCreateCall) (request|response)
/// * [locations lakes zones entities partitions get projects](ProjectLocationLakeZoneEntityPartitionGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1Partition {
    /// Optional. The etag for this partition.
    
    pub etag: Option<String>,
    /// Required. Immutable. The location of the entity data within the partition, for example, gs://bucket/path/to/entity/key1=value1/key2=value2. Or projects//datasets//tables/
    
    pub location: Option<String>,
    /// Output only. Partition values used in the HTTP URL must be double encoded. For example, url_encode(url_encode(value)) can be used to encode "US:CA/CA#Sunnyvale so that the request URL ends with "/partitions/US%253ACA/CA%2523Sunnyvale". The name field in the response retains the encoded format.
    
    pub name: Option<String>,
    /// Required. Immutable. The set of values representing the partition, which correspond to the partition schema defined in the parent entity.
    
    pub values: Option<Vec<String>>,
}

impl client::RequestValue for GoogleCloudDataplexV1Partition {}
impl client::ResponseResult for GoogleCloudDataplexV1Partition {}


/// ResourceAccessSpec holds the access control configuration to be enforced on the resources, for example, Cloud Storage bucket, BigQuery dataset, BigQuery table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1ResourceAccessSpec {
    /// Optional. The set of principals to be granted owner role on the resource.
    
    pub owners: Option<Vec<String>>,
    /// Optional. The format of strings follows the pattern followed by IAM in the bindings. user:{email}, serviceAccount:{email} group:{email}. The set of principals to be granted reader role on the resource.
    
    pub readers: Option<Vec<String>>,
    /// Optional. The set of principals to be granted writer role on the resource.
    
    pub writers: Option<Vec<String>>,
}

impl client::Part for GoogleCloudDataplexV1ResourceAccessSpec {}


/// Run DataScan Request
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations data scans run projects](ProjectLocationDataScanRunCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1RunDataScanRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleCloudDataplexV1RunDataScanRequest {}


/// Run DataScan Response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations data scans run projects](ProjectLocationDataScanRunCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1RunDataScanResponse {
    /// DataScanJob created by RunDataScan request.
    
    pub job: Option<GoogleCloudDataplexV1DataScanJob>,
}

impl client::ResponseResult for GoogleCloudDataplexV1RunDataScanResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations lakes tasks run projects](ProjectLocationLakeTaskRunCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1RunTaskRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleCloudDataplexV1RunTaskRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations lakes tasks run projects](ProjectLocationLakeTaskRunCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1RunTaskResponse {
    /// Jobs created by RunTask API.
    
    pub job: Option<GoogleCloudDataplexV1Job>,
}

impl client::ResponseResult for GoogleCloudDataplexV1RunTaskResponse {}


/// The data scanned during processing (e.g. in incremental DataScan)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1ScannedData {
    /// The range denoted by values of an incremental field
    #[serde(rename="incrementalField")]
    
    pub incremental_field: Option<GoogleCloudDataplexV1ScannedDataIncrementalField>,
}

impl client::Part for GoogleCloudDataplexV1ScannedData {}


/// A data range denoted by a pair of start/end values of a field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1ScannedDataIncrementalField {
    /// Value that marks the end of the range.
    
    pub end: Option<String>,
    /// The field that contains values which monotonically increases over time (e.g. a timestamp column).
    
    pub field: Option<String>,
    /// Value that marks the start of the range.
    
    pub start: Option<String>,
}

impl client::Part for GoogleCloudDataplexV1ScannedDataIncrementalField {}


/// Schema information describing the structure and layout of the data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1Schema {
    /// Optional. The sequence of fields describing data in table entities. Note: BigQuery SchemaFields are immutable.
    
    pub fields: Option<Vec<GoogleCloudDataplexV1SchemaSchemaField>>,
    /// Optional. The sequence of fields describing the partition structure in entities. If this field is empty, there are no partitions within the data.
    #[serde(rename="partitionFields")]
    
    pub partition_fields: Option<Vec<GoogleCloudDataplexV1SchemaPartitionField>>,
    /// Optional. The structure of paths containing partition data within the entity.
    #[serde(rename="partitionStyle")]
    
    pub partition_style: Option<String>,
    /// Required. Set to true if user-managed or false if managed by Dataplex. The default is false (managed by Dataplex). Set to falseto enable Dataplex discovery to update the schema. including new data discovery, schema inference, and schema evolution. Users retain the ability to input and edit the schema. Dataplex treats schema input by the user as though produced by a previous Dataplex discovery operation, and it will evolve the schema and take action based on that treatment. Set to true to fully manage the entity schema. This setting guarantees that Dataplex will not change schema fields.
    #[serde(rename="userManaged")]
    
    pub user_managed: Option<bool>,
}

impl client::Part for GoogleCloudDataplexV1Schema {}


/// Represents a key field within the entity's partition structure. You could have up to 20 partition fields, but only the first 10 partitions have the filtering ability due to performance consideration. Note: Partition fields are immutable.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1SchemaPartitionField {
    /// Required. Partition field name must consist of letters, numbers, and underscores only, with a maximum of length of 256 characters, and must begin with a letter or underscore..
    
    pub name: Option<String>,
    /// Required. Immutable. The type of field.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleCloudDataplexV1SchemaPartitionField {}


/// Represents a column field within a table schema.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1SchemaSchemaField {
    /// Optional. User friendly field description. Must be less than or equal to 1024 characters.
    
    pub description: Option<String>,
    /// Optional. Any nested field for complex types.
    
    pub fields: Option<Vec<GoogleCloudDataplexV1SchemaSchemaField>>,
    /// Required. Additional field semantics.
    
    pub mode: Option<String>,
    /// Required. The name of the field. Must contain only letters, numbers and underscores, with a maximum length of 767 characters, and must begin with a letter or underscore.
    
    pub name: Option<String>,
    /// Required. The type of field.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleCloudDataplexV1SchemaSchemaField {}


/// Represents an active analyze session running for a user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1Session {
    /// Output only. Session start time.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The relative resource name of the content, of the form: projects/{project_id}/locations/{location_id}/lakes/{lake_id}/environment/{environment_id}/sessions/{session_id}
    
    pub name: Option<String>,
    /// no description provided
    
    pub state: Option<String>,
    /// Output only. Email of user running the session.
    #[serde(rename="userId")]
    
    pub user_id: Option<String>,
}

impl client::Part for GoogleCloudDataplexV1Session {}


/// Describes the access mechanism of the data within its storage location.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1StorageAccess {
    /// Output only. Describes the read access mechanism of the data. Not user settable.
    
    pub read: Option<String>,
}

impl client::Part for GoogleCloudDataplexV1StorageAccess {}


/// Describes the format of the data within its storage location.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1StorageFormat {
    /// Optional. The compression type associated with the stored data. If unspecified, the data is uncompressed.
    #[serde(rename="compressionFormat")]
    
    pub compression_format: Option<String>,
    /// Optional. Additional information about CSV formatted data.
    
    pub csv: Option<GoogleCloudDataplexV1StorageFormatCsvOptions>,
    /// Output only. The data format associated with the stored data, which represents content type values. The value is inferred from mime type.
    
    pub format: Option<String>,
    /// Optional. Additional information about iceberg tables.
    
    pub iceberg: Option<GoogleCloudDataplexV1StorageFormatIcebergOptions>,
    /// Optional. Additional information about CSV formatted data.
    
    pub json: Option<GoogleCloudDataplexV1StorageFormatJsonOptions>,
    /// Required. The mime type descriptor for the data. Must match the pattern {type}/{subtype}. Supported values: application/x-parquet application/x-avro application/x-orc application/x-tfrecord application/x-parquet+iceberg application/x-avro+iceberg application/x-orc+iceberg application/json application/{subtypes} text/csv text/ image/{image subtype} video/{video subtype} audio/{audio subtype}
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
}

impl client::Part for GoogleCloudDataplexV1StorageFormat {}


/// Describes CSV and similar semi-structured data formats.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1StorageFormatCsvOptions {
    /// Optional. The delimiter used to separate values. Defaults to ','.
    
    pub delimiter: Option<String>,
    /// Optional. The character encoding of the data. Accepts "US-ASCII", "UTF-8", and "ISO-8859-1". Defaults to UTF-8 if unspecified.
    
    pub encoding: Option<String>,
    /// Optional. The number of rows to interpret as header rows that should be skipped when reading data rows. Defaults to 0.
    #[serde(rename="headerRows")]
    
    pub header_rows: Option<i32>,
    /// Optional. The character used to quote column values. Accepts '"' (double quotation mark) or ''' (single quotation mark). Defaults to '"' (double quotation mark) if unspecified.
    
    pub quote: Option<String>,
}

impl client::Part for GoogleCloudDataplexV1StorageFormatCsvOptions {}


/// Describes Iceberg data format.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1StorageFormatIcebergOptions {
    /// Optional. The location of where the iceberg metadata is present, must be within the table path
    #[serde(rename="metadataLocation")]
    
    pub metadata_location: Option<String>,
}

impl client::Part for GoogleCloudDataplexV1StorageFormatIcebergOptions {}


/// Describes JSON data format.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1StorageFormatJsonOptions {
    /// Optional. The character encoding of the data. Accepts "US-ASCII", "UTF-8" and "ISO-8859-1". Defaults to UTF-8 if not specified.
    
    pub encoding: Option<String>,
}

impl client::Part for GoogleCloudDataplexV1StorageFormatJsonOptions {}


/// A task represents a user-visible job.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations lakes tasks create projects](ProjectLocationLakeTaskCreateCall) (request)
/// * [locations lakes tasks get projects](ProjectLocationLakeTaskGetCall) (response)
/// * [locations lakes tasks patch projects](ProjectLocationLakeTaskPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1Task {
    /// Output only. The time when the task was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Description of the task.
    
    pub description: Option<String>,
    /// Optional. User friendly display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Required. Spec related to how a task is executed.
    #[serde(rename="executionSpec")]
    
    pub execution_spec: Option<GoogleCloudDataplexV1TaskExecutionSpec>,
    /// Output only. Status of the latest task executions.
    #[serde(rename="executionStatus")]
    
    pub execution_status: Option<GoogleCloudDataplexV1TaskExecutionStatus>,
    /// Optional. User-defined labels for the task.
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. The relative resource name of the task, of the form: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/ tasks/{task_id}.
    
    pub name: Option<String>,
    /// Config related to running scheduled Notebooks.
    
    pub notebook: Option<GoogleCloudDataplexV1TaskNotebookTaskConfig>,
    /// Config related to running custom Spark tasks.
    
    pub spark: Option<GoogleCloudDataplexV1TaskSparkTaskConfig>,
    /// Output only. Current state of the task.
    
    pub state: Option<String>,
    /// Required. Spec related to how often and when a task should be triggered.
    #[serde(rename="triggerSpec")]
    
    pub trigger_spec: Option<GoogleCloudDataplexV1TaskTriggerSpec>,
    /// Output only. System generated globally unique ID for the task. This ID will be different if the task is deleted and re-created with the same name.
    
    pub uid: Option<String>,
    /// Output only. The time when the task was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GoogleCloudDataplexV1Task {}
impl client::ResponseResult for GoogleCloudDataplexV1Task {}


/// Execution related settings, like retry and service_account.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1TaskExecutionSpec {
    /// Optional. The arguments to pass to the task. The args can use placeholders of the format ${placeholder} as part of key/value string. These will be interpolated before passing the args to the driver. Currently supported placeholders: - ${task_id} - ${job_time} To pass positional args, set the key as TASK_ARGS. The value should be a comma-separated string of all the positional arguments. To use a delimiter other than comma, refer to https://cloud.google.com/sdk/gcloud/reference/topic/escaping. In case of other keys being present in the args, then TASK_ARGS will be passed as the last argument.
    
    pub args: Option<HashMap<String, String>>,
    /// Optional. The Cloud KMS key to use for encryption, of the form: projects/{project_number}/locations/{location_id}/keyRings/{key-ring-name}/cryptoKeys/{key-name}.
    #[serde(rename="kmsKey")]
    
    pub kms_key: Option<String>,
    /// Optional. The maximum duration after which the job execution is expired.
    #[serde(rename="maxJobExecutionLifetime")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub max_job_execution_lifetime: Option<client::chrono::Duration>,
    /// Optional. The project in which jobs are run. By default, the project containing the Lake is used. If a project is provided, the ExecutionSpec.service_account must belong to this project.
    
    pub project: Option<String>,
    /// Required. Service account to use to execute a task. If not provided, the default Compute service account for the project is used.
    #[serde(rename="serviceAccount")]
    
    pub service_account: Option<String>,
}

impl client::Part for GoogleCloudDataplexV1TaskExecutionSpec {}


/// Status of the task execution (e.g. Jobs).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1TaskExecutionStatus {
    /// Output only. latest job execution
    #[serde(rename="latestJob")]
    
    pub latest_job: Option<GoogleCloudDataplexV1Job>,
    /// Output only. Last update time of the status.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GoogleCloudDataplexV1TaskExecutionStatus {}


/// Configuration for the underlying infrastructure used to run workloads.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1TaskInfrastructureSpec {
    /// Compute resources needed for a Task when using Dataproc Serverless.
    
    pub batch: Option<GoogleCloudDataplexV1TaskInfrastructureSpecBatchComputeResources>,
    /// Container Image Runtime Configuration.
    #[serde(rename="containerImage")]
    
    pub container_image: Option<GoogleCloudDataplexV1TaskInfrastructureSpecContainerImageRuntime>,
    /// Vpc network.
    #[serde(rename="vpcNetwork")]
    
    pub vpc_network: Option<GoogleCloudDataplexV1TaskInfrastructureSpecVpcNetwork>,
}

impl client::Part for GoogleCloudDataplexV1TaskInfrastructureSpec {}


/// Batch compute resources associated with the task.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1TaskInfrastructureSpecBatchComputeResources {
    /// Optional. Total number of job executors. Executor Count should be between 2 and 100. Default=2
    #[serde(rename="executorsCount")]
    
    pub executors_count: Option<i32>,
    /// Optional. Max configurable executors. If max_executors_count > executors_count, then auto-scaling is enabled. Max Executor Count should be between 2 and 1000. Default=1000
    #[serde(rename="maxExecutorsCount")]
    
    pub max_executors_count: Option<i32>,
}

impl client::Part for GoogleCloudDataplexV1TaskInfrastructureSpecBatchComputeResources {}


/// Container Image Runtime Configuration used with Batch execution.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1TaskInfrastructureSpecContainerImageRuntime {
    /// Optional. Container image to use.
    
    pub image: Option<String>,
    /// Optional. A list of Java JARS to add to the classpath. Valid input includes Cloud Storage URIs to Jar binaries. For example, gs://bucket-name/my/path/to/file.jar
    #[serde(rename="javaJars")]
    
    pub java_jars: Option<Vec<String>>,
    /// Optional. Override to common configuration of open source components installed on the Dataproc cluster. The properties to set on daemon config files. Property keys are specified in prefix:property format, for example core:hadoop.tmp.dir. For more information, see Cluster properties (https://cloud.google.com/dataproc/docs/concepts/cluster-properties).
    
    pub properties: Option<HashMap<String, String>>,
    /// Optional. A list of python packages to be installed. Valid formats include Cloud Storage URI to a PIP installable library. For example, gs://bucket-name/my/path/to/lib.tar.gz
    #[serde(rename="pythonPackages")]
    
    pub python_packages: Option<Vec<String>>,
}

impl client::Part for GoogleCloudDataplexV1TaskInfrastructureSpecContainerImageRuntime {}


/// Cloud VPC Network used to run the infrastructure.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1TaskInfrastructureSpecVpcNetwork {
    /// Optional. The Cloud VPC network in which the job is run. By default, the Cloud VPC network named Default within the project is used.
    
    pub network: Option<String>,
    /// Optional. List of network tags to apply to the job.
    #[serde(rename="networkTags")]
    
    pub network_tags: Option<Vec<String>>,
    /// Optional. The Cloud VPC sub-network in which the job is run.
    #[serde(rename="subNetwork")]
    
    pub sub_network: Option<String>,
}

impl client::Part for GoogleCloudDataplexV1TaskInfrastructureSpecVpcNetwork {}


/// Config for running scheduled notebooks.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1TaskNotebookTaskConfig {
    /// Optional. Cloud Storage URIs of archives to be extracted into the working directory of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip.
    #[serde(rename="archiveUris")]
    
    pub archive_uris: Option<Vec<String>>,
    /// Optional. Cloud Storage URIs of files to be placed in the working directory of each executor.
    #[serde(rename="fileUris")]
    
    pub file_uris: Option<Vec<String>>,
    /// Optional. Infrastructure specification for the execution.
    #[serde(rename="infrastructureSpec")]
    
    pub infrastructure_spec: Option<GoogleCloudDataplexV1TaskInfrastructureSpec>,
    /// Required. Path to input notebook. This can be the Cloud Storage URI of the notebook file or the path to a Notebook Content. The execution args are accessible as environment variables (TASK_key=value).
    
    pub notebook: Option<String>,
}

impl client::Part for GoogleCloudDataplexV1TaskNotebookTaskConfig {}


/// User-specified config for running a Spark task.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1TaskSparkTaskConfig {
    /// Optional. Cloud Storage URIs of archives to be extracted into the working directory of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip.
    #[serde(rename="archiveUris")]
    
    pub archive_uris: Option<Vec<String>>,
    /// Optional. Cloud Storage URIs of files to be placed in the working directory of each executor.
    #[serde(rename="fileUris")]
    
    pub file_uris: Option<Vec<String>>,
    /// Optional. Infrastructure specification for the execution.
    #[serde(rename="infrastructureSpec")]
    
    pub infrastructure_spec: Option<GoogleCloudDataplexV1TaskInfrastructureSpec>,
    /// The name of the driver's main class. The jar file that contains the class must be in the default CLASSPATH or specified in jar_file_uris. The execution args are passed in as a sequence of named process arguments (--key=value).
    #[serde(rename="mainClass")]
    
    pub main_class: Option<String>,
    /// The Cloud Storage URI of the jar file that contains the main class. The execution args are passed in as a sequence of named process arguments (--key=value).
    #[serde(rename="mainJarFileUri")]
    
    pub main_jar_file_uri: Option<String>,
    /// The Gcloud Storage URI of the main Python file to use as the driver. Must be a .py file. The execution args are passed in as a sequence of named process arguments (--key=value).
    #[serde(rename="pythonScriptFile")]
    
    pub python_script_file: Option<String>,
    /// The query text. The execution args are used to declare a set of script variables (set key="value";).
    #[serde(rename="sqlScript")]
    
    pub sql_script: Option<String>,
    /// A reference to a query file. This can be the Cloud Storage URI of the query file or it can the path to a SqlScript Content. The execution args are used to declare a set of script variables (set key="value";).
    #[serde(rename="sqlScriptFile")]
    
    pub sql_script_file: Option<String>,
}

impl client::Part for GoogleCloudDataplexV1TaskSparkTaskConfig {}


/// Task scheduling and trigger settings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1TaskTriggerSpec {
    /// Optional. Prevent the task from executing. This does not cancel already running tasks. It is intended to temporarily disable RECURRING tasks.
    
    pub disabled: Option<bool>,
    /// Optional. Number of retry attempts before aborting. Set to zero to never attempt to retry a failed task.
    #[serde(rename="maxRetries")]
    
    pub max_retries: Option<i32>,
    /// Optional. Cron schedule (https://en.wikipedia.org/wiki/Cron) for running tasks periodically. To explicitly set a timezone to the cron tab, apply a prefix in the cron tab: "CRON_TZ=${IANA_TIME_ZONE}" or "TZ=${IANA_TIME_ZONE}". The ${IANA_TIME_ZONE} may only be a valid string from IANA time zone database. For example, CRON_TZ=America/New_York 1 * * * *, or TZ=America/New_York 1 * * * *. This field is required for RECURRING tasks.
    
    pub schedule: Option<String>,
    /// Optional. The first run of the task will be after this time. If not specified, the task will run shortly after being submitted if ON_DEMAND and based on the schedule if RECURRING.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. Immutable. Trigger type of the user-specified Task.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleCloudDataplexV1TaskTriggerSpec {}


/// DataScan scheduling and trigger settings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1Trigger {
    /// The scan runs once via RunDataScan API.
    #[serde(rename="onDemand")]
    
    pub on_demand: Option<GoogleCloudDataplexV1TriggerOnDemand>,
    /// The scan is scheduled to run periodically.
    
    pub schedule: Option<GoogleCloudDataplexV1TriggerSchedule>,
}

impl client::Part for GoogleCloudDataplexV1Trigger {}


/// The scan runs once via RunDataScan API.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1TriggerOnDemand { _never_set: Option<bool> }

impl client::Part for GoogleCloudDataplexV1TriggerOnDemand {}


/// The scan is scheduled to run periodically.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1TriggerSchedule {
    /// Required. Cron (https://en.wikipedia.org/wiki/Cron) schedule for running scans periodically.To explicitly set a timezone in the cron tab, apply a prefix in the cron tab: "CRON_TZ=${IANA_TIME_ZONE}" or "TZ=${IANA_TIME_ZONE}". The ${IANA_TIME_ZONE} may only be a valid string from IANA time zone database (wikipedia (https://en.wikipedia.org/wiki/List_of_tz_database_time_zones#List)). For example, CRON_TZ=America/New_York 1 * * * *, or TZ=America/New_York 1 * * * *.This field is required for Schedule scans.
    
    pub cron: Option<String>,
}

impl client::Part for GoogleCloudDataplexV1TriggerSchedule {}


/// A zone represents a logical group of related assets within a lake. A zone can be used to map to organizational structure or represent stages of data readiness from raw to curated. It provides managing behavior that is shared or inherited by all contained assets.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations lakes zones create projects](ProjectLocationLakeZoneCreateCall) (request)
/// * [locations lakes zones get projects](ProjectLocationLakeZoneGetCall) (response)
/// * [locations lakes zones patch projects](ProjectLocationLakeZonePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1Zone {
    /// Output only. Aggregated status of the underlying assets of the zone.
    #[serde(rename="assetStatus")]
    
    pub asset_status: Option<GoogleCloudDataplexV1AssetStatus>,
    /// Output only. The time when the zone was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Description of the zone.
    
    pub description: Option<String>,
    /// Optional. Specification of the discovery feature applied to data in this zone.
    #[serde(rename="discoverySpec")]
    
    pub discovery_spec: Option<GoogleCloudDataplexV1ZoneDiscoverySpec>,
    /// Optional. User friendly display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Optional. User defined labels for the zone.
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. The relative resource name of the zone, of the form: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}.
    
    pub name: Option<String>,
    /// Required. Specification of the resources that are referenced by the assets within this zone.
    #[serde(rename="resourceSpec")]
    
    pub resource_spec: Option<GoogleCloudDataplexV1ZoneResourceSpec>,
    /// Output only. Current state of the zone.
    
    pub state: Option<String>,
    /// Required. Immutable. The type of the zone.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Output only. System generated globally unique ID for the zone. This ID will be different if the zone is deleted and re-created with the same name.
    
    pub uid: Option<String>,
    /// Output only. The time when the zone was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GoogleCloudDataplexV1Zone {}
impl client::ResponseResult for GoogleCloudDataplexV1Zone {}


/// Settings to manage the metadata discovery and publishing in a zone.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1ZoneDiscoverySpec {
    /// Optional. Configuration for CSV data.
    #[serde(rename="csvOptions")]
    
    pub csv_options: Option<GoogleCloudDataplexV1ZoneDiscoverySpecCsvOptions>,
    /// Required. Whether discovery is enabled.
    
    pub enabled: Option<bool>,
    /// Optional. The list of patterns to apply for selecting data to exclude during discovery. For Cloud Storage bucket assets, these are interpreted as glob patterns used to match object names. For BigQuery dataset assets, these are interpreted as patterns to match table names.
    #[serde(rename="excludePatterns")]
    
    pub exclude_patterns: Option<Vec<String>>,
    /// Optional. The list of patterns to apply for selecting data to include during discovery if only a subset of the data should considered. For Cloud Storage bucket assets, these are interpreted as glob patterns used to match object names. For BigQuery dataset assets, these are interpreted as patterns to match table names.
    #[serde(rename="includePatterns")]
    
    pub include_patterns: Option<Vec<String>>,
    /// Optional. Configuration for Json data.
    #[serde(rename="jsonOptions")]
    
    pub json_options: Option<GoogleCloudDataplexV1ZoneDiscoverySpecJsonOptions>,
    /// Optional. Cron schedule (https://en.wikipedia.org/wiki/Cron) for running discovery periodically. Successive discovery runs must be scheduled at least 60 minutes apart. The default value is to run discovery every 60 minutes. To explicitly set a timezone to the cron tab, apply a prefix in the cron tab: "CRON_TZ=${IANA_TIME_ZONE}" or TZ=${IANA_TIME_ZONE}". The ${IANA_TIME_ZONE} may only be a valid string from IANA time zone database. For example, CRON_TZ=America/New_York 1 * * * *, or TZ=America/New_York 1 * * * *.
    
    pub schedule: Option<String>,
}

impl client::Part for GoogleCloudDataplexV1ZoneDiscoverySpec {}


/// Describe CSV and similar semi-structured data formats.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1ZoneDiscoverySpecCsvOptions {
    /// Optional. The delimiter being used to separate values. This defaults to ','.
    
    pub delimiter: Option<String>,
    /// Optional. Whether to disable the inference of data type for CSV data. If true, all columns will be registered as strings.
    #[serde(rename="disableTypeInference")]
    
    pub disable_type_inference: Option<bool>,
    /// Optional. The character encoding of the data. The default is UTF-8.
    
    pub encoding: Option<String>,
    /// Optional. The number of rows to interpret as header rows that should be skipped when reading data rows.
    #[serde(rename="headerRows")]
    
    pub header_rows: Option<i32>,
}

impl client::Part for GoogleCloudDataplexV1ZoneDiscoverySpecCsvOptions {}


/// Describe JSON data format.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1ZoneDiscoverySpecJsonOptions {
    /// Optional. Whether to disable the inference of data type for Json data. If true, all columns will be registered as their primitive types (strings, number or boolean).
    #[serde(rename="disableTypeInference")]
    
    pub disable_type_inference: Option<bool>,
    /// Optional. The character encoding of the data. The default is UTF-8.
    
    pub encoding: Option<String>,
}

impl client::Part for GoogleCloudDataplexV1ZoneDiscoverySpecJsonOptions {}


/// Settings for resources attached as assets within a zone.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDataplexV1ZoneResourceSpec {
    /// Required. Immutable. The location type of the resources that are allowed to be attached to the assets within this zone.
    #[serde(rename="locationType")]
    
    pub location_type: Option<String>,
}

impl client::Part for GoogleCloudDataplexV1ZoneResourceSpec {}


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
pub struct GoogleCloudLocationListLocationsResponse {
    /// A list of locations that matches the specified filter in the request.
    
    pub locations: Option<Vec<GoogleCloudLocationLocation>>,
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudLocationListLocationsResponse {}


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
pub struct GoogleCloudLocationLocation {
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

impl client::ResponseResult for GoogleCloudLocationLocation {}


/// Specifies the audit configuration for a service. The configuration determines which permission types are logged, and what identities, if any, are exempted from logging. An AuditConfig must have one or more AuditLogConfigs.If there are AuditConfigs for both allServices and a specific service, the union of the two AuditConfigs is used for that service: the log_types specified in each AuditConfig are enabled, and the exempted_members in each AuditLogConfig are exempted.Example Policy with multiple AuditConfigs: { "audit_configs": [ { "service": "allServices", "audit_log_configs": [ { "log_type": "DATA_READ", "exempted_members": [ "user:jose@example.com" ] }, { "log_type": "DATA_WRITE" }, { "log_type": "ADMIN_READ" } ] }, { "service": "sampleservice.googleapis.com", "audit_log_configs": [ { "log_type": "DATA_READ" }, { "log_type": "DATA_WRITE", "exempted_members": [ "user:aliya@example.com" ] } ] } ] } For sampleservice, this policy enables DATA_READ, DATA_WRITE and ADMIN_READ logging. It also exempts jose@example.com from DATA_READ logging, and aliya@example.com from DATA_WRITE logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1AuditConfig {
    /// The configuration for logging of each type of permission.
    #[serde(rename="auditLogConfigs")]
    
    pub audit_log_configs: Option<Vec<GoogleIamV1AuditLogConfig>>,
    /// Specifies a service that will be enabled for audit logging. For example, storage.googleapis.com, cloudsql.googleapis.com. allServices is a special value that covers all services.
    
    pub service: Option<String>,
}

impl client::Part for GoogleIamV1AuditConfig {}


/// Provides the configuration for logging a type of permissions. Example: { "audit_log_configs": [ { "log_type": "DATA_READ", "exempted_members": [ "user:jose@example.com" ] }, { "log_type": "DATA_WRITE" } ] } This enables 'DATA_READ' and 'DATA_WRITE' logging, while exempting jose@example.com from DATA_READ logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1AuditLogConfig {
    /// Specifies the identities that do not cause logging for this type of permission. Follows the same format of Binding.members.
    #[serde(rename="exemptedMembers")]
    
    pub exempted_members: Option<Vec<String>>,
    /// The log type that this config enables.
    #[serde(rename="logType")]
    
    pub log_type: Option<String>,
}

impl client::Part for GoogleIamV1AuditLogConfig {}


/// Associates members, or principals, with a role.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1Binding {
    /// The condition that is associated with this binding.If the condition evaluates to true, then this binding applies to the current request.If the condition evaluates to false, then this binding does not apply to the current request. However, a different role binding might grant the same role to one or more of the principals in this binding.To learn which resources support conditions in their IAM policies, see the IAM documentation (https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub condition: Option<GoogleTypeExpr>,
    /// Specifies the principals requesting access for a Google Cloud resource. members can have the following values: allUsers: A special identifier that represents anyone who is on the internet; with or without a Google account. allAuthenticatedUsers: A special identifier that represents anyone who is authenticated with a Google account or a service account. Does not include identities that come from external identity providers (IdPs) through identity federation. user:{emailid}: An email address that represents a specific Google account. For example, alice@example.com . serviceAccount:{emailid}: An email address that represents a Google service account. For example, my-other-app@appspot.gserviceaccount.com. serviceAccount:{projectid}.svc.id.goog[{namespace}/{kubernetes-sa}]: An identifier for a Kubernetes service account (https://cloud.google.com/kubernetes-engine/docs/how-to/kubernetes-service-accounts). For example, my-project.svc.id.goog[my-namespace/my-kubernetes-sa]. group:{emailid}: An email address that represents a Google group. For example, admins@example.com. deleted:user:{emailid}?uid={uniqueid}: An email address (plus unique identifier) representing a user that has been recently deleted. For example, alice@example.com?uid=123456789012345678901. If the user is recovered, this value reverts to user:{emailid} and the recovered user retains the role in the binding. deleted:serviceAccount:{emailid}?uid={uniqueid}: An email address (plus unique identifier) representing a service account that has been recently deleted. For example, my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901. If the service account is undeleted, this value reverts to serviceAccount:{emailid} and the undeleted service account retains the role in the binding. deleted:group:{emailid}?uid={uniqueid}: An email address (plus unique identifier) representing a Google group that has been recently deleted. For example, admins@example.com?uid=123456789012345678901. If the group is recovered, this value reverts to group:{emailid} and the recovered group retains the role in the binding. domain:{domain}: The G Suite domain (primary) that represents all the users of that domain. For example, google.com or example.com.
    
    pub members: Option<Vec<String>>,
    /// Role that is assigned to the list of members, or principals. For example, roles/viewer, roles/editor, or roles/owner.
    
    pub role: Option<String>,
}

impl client::Part for GoogleIamV1Binding {}


/// An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources.A Policy is a collection of bindings. A binding binds one or more members, or principals, to a single role. Principals can be user accounts, service accounts, Google groups, and domains (such as G Suite). A role is a named list of permissions; each role can be an IAM predefined role or a user-created custom role.For some types of Google Cloud resources, a binding can also specify a condition, which is a logical expression that allows access to a resource only if the expression evaluates to true. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the IAM documentation (https://cloud.google.com/iam/help/conditions/resource-policies).JSON example: { “bindings”: \[ { “role”: “roles/resourcemanager.organizationAdmin”, “members”: \[ “user:mike@example.com”, “group:admins@example.com”, “domain:google.com”, “serviceAccount:my-project-id@appspot.gserviceaccount.com” \] }, { “role”: “roles/resourcemanager.organizationViewer”, “members”: \[ “user:eve@example.com” \], “condition”: { “title”: “expirable access”, “description”: “Does not grant access after Sep 2020”, “expression”: “request.time \< timestamp(‘2020-10-01T00:00:00.000Z’)”, } } \], “etag”: “BwWWja0YfJA=”, “version”: 3 } YAML example: bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time \< timestamp(‘2020-10-01T00:00:00.000Z’) etag: BwWWja0YfJA= version: 3 For a description of IAM and its features, see the IAM documentation (https://cloud.google.com/iam/docs/).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations data attribute bindings get iam policy projects](ProjectLocationDataAttributeBindingGetIamPolicyCall) (response)
/// * [locations data attribute bindings set iam policy projects](ProjectLocationDataAttributeBindingSetIamPolicyCall) (response)
/// * [locations data scans get iam policy projects](ProjectLocationDataScanGetIamPolicyCall) (response)
/// * [locations data scans set iam policy projects](ProjectLocationDataScanSetIamPolicyCall) (response)
/// * [locations data taxonomies attributes get iam policy projects](ProjectLocationDataTaxonomyAttributeGetIamPolicyCall) (response)
/// * [locations data taxonomies attributes set iam policy projects](ProjectLocationDataTaxonomyAttributeSetIamPolicyCall) (response)
/// * [locations data taxonomies get iam policy projects](ProjectLocationDataTaxonomyGetIamPolicyCall) (response)
/// * [locations data taxonomies set iam policy projects](ProjectLocationDataTaxonomySetIamPolicyCall) (response)
/// * [locations lakes content get iam policy projects](ProjectLocationLakeContentGetIamPolicyCall) (response)
/// * [locations lakes content set iam policy projects](ProjectLocationLakeContentSetIamPolicyCall) (response)
/// * [locations lakes contentitems get iam policy projects](ProjectLocationLakeContentitemGetIamPolicyCall) (response)
/// * [locations lakes contentitems set iam policy projects](ProjectLocationLakeContentitemSetIamPolicyCall) (response)
/// * [locations lakes environments get iam policy projects](ProjectLocationLakeEnvironmentGetIamPolicyCall) (response)
/// * [locations lakes environments set iam policy projects](ProjectLocationLakeEnvironmentSetIamPolicyCall) (response)
/// * [locations lakes tasks get iam policy projects](ProjectLocationLakeTaskGetIamPolicyCall) (response)
/// * [locations lakes tasks set iam policy projects](ProjectLocationLakeTaskSetIamPolicyCall) (response)
/// * [locations lakes zones assets get iam policy projects](ProjectLocationLakeZoneAssetGetIamPolicyCall) (response)
/// * [locations lakes zones assets set iam policy projects](ProjectLocationLakeZoneAssetSetIamPolicyCall) (response)
/// * [locations lakes zones get iam policy projects](ProjectLocationLakeZoneGetIamPolicyCall) (response)
/// * [locations lakes zones set iam policy projects](ProjectLocationLakeZoneSetIamPolicyCall) (response)
/// * [locations lakes get iam policy projects](ProjectLocationLakeGetIamPolicyCall) (response)
/// * [locations lakes set iam policy projects](ProjectLocationLakeSetIamPolicyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1Policy {
    /// Specifies cloud audit logging configuration for this policy.
    #[serde(rename="auditConfigs")]
    
    pub audit_configs: Option<Vec<GoogleIamV1AuditConfig>>,
    /// Associates a list of members, or principals, with a role. Optionally, may specify a condition that determines how and when the bindings are applied. Each of the bindings must contain at least one principal.The bindings in a Policy can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the bindings grant 50 different roles to user:alice@example.com, and not to any other principal, then you can add another 1,450 principals to the bindings in the Policy.
    
    pub bindings: Option<Vec<GoogleIamV1Binding>>,
    /// etag is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the etag in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An etag is returned in the response to getIamPolicy, and systems are expected to put that etag in the request to setIamPolicy to ensure that their change will be applied to the same version of the policy.Important: If you use IAM Conditions, you must include the etag field whenever you call setIamPolicy. If you omit this field, then IAM allows you to overwrite a version 3 policy with a version 1 policy, and all of the conditions in the version 3 policy are lost.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
    /// Specifies the format of the policy.Valid values are 0, 1, and 3. Requests that specify an invalid value are rejected.Any operation that affects conditional role bindings must specify version 3. This requirement applies to the following operations: Getting a policy that includes a conditional role binding Adding a conditional role binding to a policy Changing a conditional role binding in a policy Removing any role binding, with or without a condition, from a policy that includes conditionsImportant: If you use IAM Conditions, you must include the etag field whenever you call setIamPolicy. If you omit this field, then IAM allows you to overwrite a version 3 policy with a version 1 policy, and all of the conditions in the version 3 policy are lost.If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset.To learn which resources support conditions in their IAM policies, see the IAM documentation (https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub version: Option<i32>,
}

impl client::ResponseResult for GoogleIamV1Policy {}


/// Request message for SetIamPolicy method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations data attribute bindings set iam policy projects](ProjectLocationDataAttributeBindingSetIamPolicyCall) (request)
/// * [locations data scans set iam policy projects](ProjectLocationDataScanSetIamPolicyCall) (request)
/// * [locations data taxonomies attributes set iam policy projects](ProjectLocationDataTaxonomyAttributeSetIamPolicyCall) (request)
/// * [locations data taxonomies set iam policy projects](ProjectLocationDataTaxonomySetIamPolicyCall) (request)
/// * [locations lakes content set iam policy projects](ProjectLocationLakeContentSetIamPolicyCall) (request)
/// * [locations lakes contentitems set iam policy projects](ProjectLocationLakeContentitemSetIamPolicyCall) (request)
/// * [locations lakes environments set iam policy projects](ProjectLocationLakeEnvironmentSetIamPolicyCall) (request)
/// * [locations lakes tasks set iam policy projects](ProjectLocationLakeTaskSetIamPolicyCall) (request)
/// * [locations lakes zones assets set iam policy projects](ProjectLocationLakeZoneAssetSetIamPolicyCall) (request)
/// * [locations lakes zones set iam policy projects](ProjectLocationLakeZoneSetIamPolicyCall) (request)
/// * [locations lakes set iam policy projects](ProjectLocationLakeSetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1SetIamPolicyRequest {
    /// REQUIRED: The complete policy to be applied to the resource. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them.
    
    pub policy: Option<GoogleIamV1Policy>,
    /// OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used:paths: "bindings, etag"
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for GoogleIamV1SetIamPolicyRequest {}


/// Request message for TestIamPermissions method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations data attribute bindings test iam permissions projects](ProjectLocationDataAttributeBindingTestIamPermissionCall) (request)
/// * [locations data scans test iam permissions projects](ProjectLocationDataScanTestIamPermissionCall) (request)
/// * [locations data taxonomies attributes test iam permissions projects](ProjectLocationDataTaxonomyAttributeTestIamPermissionCall) (request)
/// * [locations data taxonomies test iam permissions projects](ProjectLocationDataTaxonomyTestIamPermissionCall) (request)
/// * [locations lakes content test iam permissions projects](ProjectLocationLakeContentTestIamPermissionCall) (request)
/// * [locations lakes contentitems test iam permissions projects](ProjectLocationLakeContentitemTestIamPermissionCall) (request)
/// * [locations lakes environments test iam permissions projects](ProjectLocationLakeEnvironmentTestIamPermissionCall) (request)
/// * [locations lakes tasks test iam permissions projects](ProjectLocationLakeTaskTestIamPermissionCall) (request)
/// * [locations lakes zones assets test iam permissions projects](ProjectLocationLakeZoneAssetTestIamPermissionCall) (request)
/// * [locations lakes zones test iam permissions projects](ProjectLocationLakeZoneTestIamPermissionCall) (request)
/// * [locations lakes test iam permissions projects](ProjectLocationLakeTestIamPermissionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1TestIamPermissionsRequest {
    /// The set of permissions to check for the resource. Permissions with wildcards (such as * or storage.*) are not allowed. For more information see IAM Overview (https://cloud.google.com/iam/docs/overview#permissions).
    
    pub permissions: Option<Vec<String>>,
}

impl client::RequestValue for GoogleIamV1TestIamPermissionsRequest {}


/// Response message for TestIamPermissions method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations data attribute bindings test iam permissions projects](ProjectLocationDataAttributeBindingTestIamPermissionCall) (response)
/// * [locations data scans test iam permissions projects](ProjectLocationDataScanTestIamPermissionCall) (response)
/// * [locations data taxonomies attributes test iam permissions projects](ProjectLocationDataTaxonomyAttributeTestIamPermissionCall) (response)
/// * [locations data taxonomies test iam permissions projects](ProjectLocationDataTaxonomyTestIamPermissionCall) (response)
/// * [locations lakes content test iam permissions projects](ProjectLocationLakeContentTestIamPermissionCall) (response)
/// * [locations lakes contentitems test iam permissions projects](ProjectLocationLakeContentitemTestIamPermissionCall) (response)
/// * [locations lakes environments test iam permissions projects](ProjectLocationLakeEnvironmentTestIamPermissionCall) (response)
/// * [locations lakes tasks test iam permissions projects](ProjectLocationLakeTaskTestIamPermissionCall) (response)
/// * [locations lakes zones assets test iam permissions projects](ProjectLocationLakeZoneAssetTestIamPermissionCall) (response)
/// * [locations lakes zones test iam permissions projects](ProjectLocationLakeZoneTestIamPermissionCall) (response)
/// * [locations lakes test iam permissions projects](ProjectLocationLakeTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1TestIamPermissionsResponse {
    /// A subset of TestPermissionsRequest.permissions that the caller is allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for GoogleIamV1TestIamPermissionsResponse {}


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
pub struct GoogleLongrunningCancelOperationRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleLongrunningCancelOperationRequest {}


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
pub struct GoogleLongrunningListOperationsResponse {
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of operations that matches the specified filter in the request.
    
    pub operations: Option<Vec<GoogleLongrunningOperation>>,
}

impl client::ResponseResult for GoogleLongrunningListOperationsResponse {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations data attribute bindings create projects](ProjectLocationDataAttributeBindingCreateCall) (response)
/// * [locations data attribute bindings delete projects](ProjectLocationDataAttributeBindingDeleteCall) (response)
/// * [locations data attribute bindings patch projects](ProjectLocationDataAttributeBindingPatchCall) (response)
/// * [locations data scans create projects](ProjectLocationDataScanCreateCall) (response)
/// * [locations data scans delete projects](ProjectLocationDataScanDeleteCall) (response)
/// * [locations data scans patch projects](ProjectLocationDataScanPatchCall) (response)
/// * [locations data taxonomies attributes create projects](ProjectLocationDataTaxonomyAttributeCreateCall) (response)
/// * [locations data taxonomies attributes delete projects](ProjectLocationDataTaxonomyAttributeDeleteCall) (response)
/// * [locations data taxonomies attributes patch projects](ProjectLocationDataTaxonomyAttributePatchCall) (response)
/// * [locations data taxonomies create projects](ProjectLocationDataTaxonomyCreateCall) (response)
/// * [locations data taxonomies delete projects](ProjectLocationDataTaxonomyDeleteCall) (response)
/// * [locations data taxonomies patch projects](ProjectLocationDataTaxonomyPatchCall) (response)
/// * [locations lakes environments create projects](ProjectLocationLakeEnvironmentCreateCall) (response)
/// * [locations lakes environments delete projects](ProjectLocationLakeEnvironmentDeleteCall) (response)
/// * [locations lakes environments patch projects](ProjectLocationLakeEnvironmentPatchCall) (response)
/// * [locations lakes tasks create projects](ProjectLocationLakeTaskCreateCall) (response)
/// * [locations lakes tasks delete projects](ProjectLocationLakeTaskDeleteCall) (response)
/// * [locations lakes tasks patch projects](ProjectLocationLakeTaskPatchCall) (response)
/// * [locations lakes zones assets create projects](ProjectLocationLakeZoneAssetCreateCall) (response)
/// * [locations lakes zones assets delete projects](ProjectLocationLakeZoneAssetDeleteCall) (response)
/// * [locations lakes zones assets patch projects](ProjectLocationLakeZoneAssetPatchCall) (response)
/// * [locations lakes zones create projects](ProjectLocationLakeZoneCreateCall) (response)
/// * [locations lakes zones delete projects](ProjectLocationLakeZoneDeleteCall) (response)
/// * [locations lakes zones patch projects](ProjectLocationLakeZonePatchCall) (response)
/// * [locations lakes create projects](ProjectLocationLakeCreateCall) (response)
/// * [locations lakes delete projects](ProjectLocationLakeDeleteCall) (response)
/// * [locations lakes patch projects](ProjectLocationLakePatchCall) (response)
/// * [locations operations get projects](ProjectLocationOperationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleLongrunningOperation {
    /// If the value is false, it means the operation is still in progress. If true, the operation is completed, and either error or response is available.
    
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    
    pub error: Option<GoogleRpcStatus>,
    /// Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the name should be a resource name ending with operations/{unique_id}.
    
    pub name: Option<String>,
    /// The normal response of the operation in case of success. If the original method returns no data on success, such as Delete, the response is google.protobuf.Empty. If the original method is standard Get/Create/Update, the response should be the resource. For other methods, the response should have the type XxxResponse, where Xxx is the original method name. For example, if the original method name is TakeSnapshot(), the inferred response type is TakeSnapshotResponse.
    
    pub response: Option<HashMap<String, json::Value>>,
}

impl client::ResponseResult for GoogleLongrunningOperation {}


/// The Status type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by gRPC (https://github.com/grpc). Each Status message contains three pieces of data: error code, error message, and error details.You can find out more about this error model and how to work with it in the API Design Guide (https://cloud.google.com/apis/design/errors).
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


/// Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language. The syntax and semantics of CEL are documented at https://github.com/google/cel-spec.Example (Comparison): title: "Summary size limit" description: "Determines if a summary is less than 100 chars" expression: "document.summary.size() < 100" Example (Equality): title: "Requestor is owner" description: "Determines if requestor is the document owner" expression: "document.owner == request.auth.claims.email" Example (Logic): title: "Public documents" description: "Determine whether the document should be publicly visible" expression: "document.type != 'private' && document.type != 'internal'" Example (Data Manipulation): title: "Notification string" description: "Create a notification string with a timestamp." expression: "'New message received at ' + string(document.create_time)" The exact variables and functions that may be referenced within an expression are determined by the service that evaluates it. See the service documentation for additional information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeExpr {
    /// Optional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI.
    
    pub description: Option<String>,
    /// Textual representation of an expression in Common Expression Language syntax.
    
    pub expression: Option<String>,
    /// Optional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file.
    
    pub location: Option<String>,
    /// Optional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression.
    
    pub title: Option<String>,
}

impl client::Part for GoogleTypeExpr {}


