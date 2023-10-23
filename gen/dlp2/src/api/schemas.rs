use super::*;
/// A task to execute on the completion of a job. See https://cloud.google.com/dlp/docs/concepts-actions to learn more.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2Action {
    /// Create a de-identified copy of the input data.
    
    pub deidentify: Option<GooglePrivacyDlpV2Deidentify>,
    /// Sends an email when the job completes. The email goes to IAM project owners and technical [Essential Contacts](https://cloud.google.com/resource-manager/docs/managing-notification-contacts).
    #[serde(rename="jobNotificationEmails")]
    
    pub job_notification_emails: Option<GooglePrivacyDlpV2JobNotificationEmails>,
    /// Publish a notification to a Pub/Sub topic.
    #[serde(rename="pubSub")]
    
    pub pub_sub: Option<GooglePrivacyDlpV2PublishToPubSub>,
    /// Publish findings to Cloud Datahub.
    #[serde(rename="publishFindingsToCloudDataCatalog")]
    
    pub publish_findings_to_cloud_data_catalog: Option<GooglePrivacyDlpV2PublishFindingsToCloudDataCatalog>,
    /// Publish summary to Cloud Security Command Center (Alpha).
    #[serde(rename="publishSummaryToCscc")]
    
    pub publish_summary_to_cscc: Option<GooglePrivacyDlpV2PublishSummaryToCscc>,
    /// Enable Stackdriver metric dlp.googleapis.com/finding_count.
    #[serde(rename="publishToStackdriver")]
    
    pub publish_to_stackdriver: Option<GooglePrivacyDlpV2PublishToStackdriver>,
    /// Save resulting findings in a provided location.
    #[serde(rename="saveFindings")]
    
    pub save_findings: Option<GooglePrivacyDlpV2SaveFindings>,
}

impl client::Part for GooglePrivacyDlpV2Action {}


/// Request message for ActivateJobTrigger.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [job triggers activate projects](ProjectJobTriggerActivateCall) (request)
/// * [locations job triggers activate projects](ProjectLocationJobTriggerActivateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2ActivateJobTriggerRequest { _never_set: Option<bool> }

impl client::RequestValue for GooglePrivacyDlpV2ActivateJobTriggerRequest {}


/// Apply transformation to all findings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2AllInfoTypes { _never_set: Option<bool> }

impl client::Part for GooglePrivacyDlpV2AllInfoTypes {}


/// Apply to all text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2AllText { _never_set: Option<bool> }

impl client::Part for GooglePrivacyDlpV2AllText {}


/// Result of a risk analysis operation request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2AnalyzeDataSourceRiskDetails {
    /// Categorical stats result
    #[serde(rename="categoricalStatsResult")]
    
    pub categorical_stats_result: Option<GooglePrivacyDlpV2CategoricalStatsResult>,
    /// Delta-presence result
    #[serde(rename="deltaPresenceEstimationResult")]
    
    pub delta_presence_estimation_result: Option<GooglePrivacyDlpV2DeltaPresenceEstimationResult>,
    /// K-anonymity result
    #[serde(rename="kAnonymityResult")]
    
    pub k_anonymity_result: Option<GooglePrivacyDlpV2KAnonymityResult>,
    /// K-map result
    #[serde(rename="kMapEstimationResult")]
    
    pub k_map_estimation_result: Option<GooglePrivacyDlpV2KMapEstimationResult>,
    /// L-divesity result
    #[serde(rename="lDiversityResult")]
    
    pub l_diversity_result: Option<GooglePrivacyDlpV2LDiversityResult>,
    /// Numerical stats result
    #[serde(rename="numericalStatsResult")]
    
    pub numerical_stats_result: Option<GooglePrivacyDlpV2NumericalStatsResult>,
    /// The configuration used for this job.
    #[serde(rename="requestedOptions")]
    
    pub requested_options: Option<GooglePrivacyDlpV2RequestedRiskAnalysisOptions>,
    /// Privacy metric to compute.
    #[serde(rename="requestedPrivacyMetric")]
    
    pub requested_privacy_metric: Option<GooglePrivacyDlpV2PrivacyMetric>,
    /// Input dataset to compute metrics over.
    #[serde(rename="requestedSourceTable")]
    
    pub requested_source_table: Option<GooglePrivacyDlpV2BigQueryTable>,
}

impl client::Part for GooglePrivacyDlpV2AnalyzeDataSourceRiskDetails {}


/// An auxiliary table contains statistical information on the relative frequency of different quasi-identifiers values. It has one or several quasi-identifiers columns, and one column that indicates the relative frequency of each quasi-identifier tuple. If a tuple is present in the data but not in the auxiliary table, the corresponding relative frequency is assumed to be zero (and thus, the tuple is highly reidentifiable).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2AuxiliaryTable {
    /// Required. Quasi-identifier columns.
    #[serde(rename="quasiIds")]
    
    pub quasi_ids: Option<Vec<GooglePrivacyDlpV2QuasiIdField>>,
    /// Required. The relative frequency column must contain a floating-point number between 0 and 1 (inclusive). Null values are assumed to be zero.
    #[serde(rename="relativeFrequency")]
    
    pub relative_frequency: Option<GooglePrivacyDlpV2FieldId>,
    /// Required. Auxiliary table location.
    
    pub table: Option<GooglePrivacyDlpV2BigQueryTable>,
}

impl client::Part for GooglePrivacyDlpV2AuxiliaryTable {}


/// Message defining a field of a BigQuery table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2BigQueryField {
    /// Designated field in the BigQuery table.
    
    pub field: Option<GooglePrivacyDlpV2FieldId>,
    /// Source table of the field.
    
    pub table: Option<GooglePrivacyDlpV2BigQueryTable>,
}

impl client::Part for GooglePrivacyDlpV2BigQueryField {}


/// Row key for identifying a record in BigQuery table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2BigQueryKey {
    /// Row number inferred at the time the table was scanned. This value is nondeterministic, cannot be queried, and may be null for inspection jobs. To locate findings within a table, specify `inspect_job.storage_config.big_query_options.identifying_fields` in `CreateDlpJobRequest`.
    #[serde(rename="rowNumber")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub row_number: Option<i64>,
    /// Complete BigQuery table reference.
    #[serde(rename="tableReference")]
    
    pub table_reference: Option<GooglePrivacyDlpV2BigQueryTable>,
}

impl client::Part for GooglePrivacyDlpV2BigQueryKey {}


/// Options defining BigQuery table and row identifiers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2BigQueryOptions {
    /// References to fields excluded from scanning. This allows you to skip inspection of entire columns which you know have no findings.
    #[serde(rename="excludedFields")]
    
    pub excluded_fields: Option<Vec<GooglePrivacyDlpV2FieldId>>,
    /// Table fields that may uniquely identify a row within the table. When `actions.saveFindings.outputConfig.table` is specified, the values of columns specified here are available in the output table under `location.content_locations.record_location.record_key.id_values`. Nested fields such as `person.birthdate.year` are allowed.
    #[serde(rename="identifyingFields")]
    
    pub identifying_fields: Option<Vec<GooglePrivacyDlpV2FieldId>>,
    /// Limit scanning only to these fields.
    #[serde(rename="includedFields")]
    
    pub included_fields: Option<Vec<GooglePrivacyDlpV2FieldId>>,
    /// Max number of rows to scan. If the table has more rows than this value, the rest of the rows are omitted. If not set, or if set to 0, all rows will be scanned. Only one of rows_limit and rows_limit_percent can be specified. Cannot be used in conjunction with TimespanConfig.
    #[serde(rename="rowsLimit")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub rows_limit: Option<i64>,
    /// Max percentage of rows to scan. The rest are omitted. The number of rows scanned is rounded down. Must be between 0 and 100, inclusively. Both 0 and 100 means no limit. Defaults to 0. Only one of rows_limit and rows_limit_percent can be specified. Cannot be used in conjunction with TimespanConfig.
    #[serde(rename="rowsLimitPercent")]
    
    pub rows_limit_percent: Option<i32>,
    /// no description provided
    #[serde(rename="sampleMethod")]
    
    pub sample_method: Option<GooglePrivacyDlpV2BigQueryOptionSampleMethodEnum>,
    /// Complete BigQuery table reference.
    #[serde(rename="tableReference")]
    
    pub table_reference: Option<GooglePrivacyDlpV2BigQueryTable>,
}

impl client::Part for GooglePrivacyDlpV2BigQueryOptions {}


/// Message defining the location of a BigQuery table. A table is uniquely identified by its project_id, dataset_id, and table_name. Within a query a table is often referenced with a string in the format of: `:.` or `..`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2BigQueryTable {
    /// Dataset ID of the table.
    #[serde(rename="datasetId")]
    
    pub dataset_id: Option<String>,
    /// The Google Cloud Platform project ID of the project containing the table. If omitted, project ID is inferred from the API call.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Name of the table.
    #[serde(rename="tableId")]
    
    pub table_id: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2BigQueryTable {}


/// Bounding box encompassing detected text within an image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2BoundingBox {
    /// Height of the bounding box in pixels.
    
    pub height: Option<i32>,
    /// Left coordinate of the bounding box. (0,0) is upper left.
    
    pub left: Option<i32>,
    /// Top coordinate of the bounding box. (0,0) is upper left.
    
    pub top: Option<i32>,
    /// Width of the bounding box in pixels.
    
    pub width: Option<i32>,
}

impl client::Part for GooglePrivacyDlpV2BoundingBox {}


/// Bucket is represented as a range, along with replacement values.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2Bucket {
    /// Upper bound of the range, exclusive; type must match min.
    
    pub max: Option<GooglePrivacyDlpV2Value>,
    /// Lower bound of the range, inclusive. Type should be the same as max if used.
    
    pub min: Option<GooglePrivacyDlpV2Value>,
    /// Required. Replacement value for this bucket.
    #[serde(rename="replacementValue")]
    
    pub replacement_value: Option<GooglePrivacyDlpV2Value>,
}

impl client::Part for GooglePrivacyDlpV2Bucket {}


/// Generalization function that buckets values based on ranges. The ranges and replacement values are dynamically provided by the user for custom behavior, such as 1-30 -> LOW 31-65 -> MEDIUM 66-100 -> HIGH This can be used on data of type: number, long, string, timestamp. If the bound `Value` type differs from the type of data being transformed, we will first attempt converting the type of the data to be transformed to match the type of the bound before comparing. See https://cloud.google.com/dlp/docs/concepts-bucketing to learn more.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2BucketingConfig {
    /// Set of buckets. Ranges must be non-overlapping.
    
    pub buckets: Option<Vec<GooglePrivacyDlpV2Bucket>>,
}

impl client::Part for GooglePrivacyDlpV2BucketingConfig {}


/// Container for bytes to inspect or redact.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2ByteContentItem {
    /// Content data to inspect or redact.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub data: Option<Vec<u8>>,
    /// The type of data stored in the bytes string. Default will be TEXT_UTF8.
    #[serde(rename="type")]
    
    pub type_: Option<GooglePrivacyDlpV2ByteContentItemTypeEnum>,
}

impl client::Part for GooglePrivacyDlpV2ByteContentItem {}


/// The request message for canceling a DLP job.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [dlp jobs cancel projects](ProjectDlpJobCancelCall) (request)
/// * [locations dlp jobs cancel projects](ProjectLocationDlpJobCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2CancelDlpJobRequest { _never_set: Option<bool> }

impl client::RequestValue for GooglePrivacyDlpV2CancelDlpJobRequest {}


/// Compute numerical stats over an individual column, including number of distinct values and value count distribution.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2CategoricalStatsConfig {
    /// Field to compute categorical stats on. All column types are supported except for arrays and structs. However, it may be more informative to use NumericalStats when the field type is supported, depending on the data.
    
    pub field: Option<GooglePrivacyDlpV2FieldId>,
}

impl client::Part for GooglePrivacyDlpV2CategoricalStatsConfig {}


/// Histogram of value frequencies in the column.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2CategoricalStatsHistogramBucket {
    /// Total number of values in this bucket.
    #[serde(rename="bucketSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bucket_size: Option<i64>,
    /// Total number of distinct values in this bucket.
    #[serde(rename="bucketValueCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bucket_value_count: Option<i64>,
    /// Sample of value frequencies in this bucket. The total number of values returned per bucket is capped at 20.
    #[serde(rename="bucketValues")]
    
    pub bucket_values: Option<Vec<GooglePrivacyDlpV2ValueFrequency>>,
    /// Lower bound on the value frequency of the values in this bucket.
    #[serde(rename="valueFrequencyLowerBound")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub value_frequency_lower_bound: Option<i64>,
    /// Upper bound on the value frequency of the values in this bucket.
    #[serde(rename="valueFrequencyUpperBound")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub value_frequency_upper_bound: Option<i64>,
}

impl client::Part for GooglePrivacyDlpV2CategoricalStatsHistogramBucket {}


/// Result of the categorical stats computation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2CategoricalStatsResult {
    /// Histogram of value frequencies in the column.
    #[serde(rename="valueFrequencyHistogramBuckets")]
    
    pub value_frequency_histogram_buckets: Option<Vec<GooglePrivacyDlpV2CategoricalStatsHistogramBucket>>,
}

impl client::Part for GooglePrivacyDlpV2CategoricalStatsResult {}


/// Partially mask a string by replacing a given number of characters with a fixed character. Masking can start from the beginning or end of the string. This can be used on data of any type (numbers, longs, and so on) and when de-identifying structured data we'll attempt to preserve the original data's type. (This allows you to take a long like 123 and modify it to a string like **3.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2CharacterMaskConfig {
    /// When masking a string, items in this list will be skipped when replacing characters. For example, if the input string is `555-555-5555` and you instruct Cloud DLP to skip `-` and mask 5 characters with `*`, Cloud DLP returns `***-**5-5555`.
    #[serde(rename="charactersToIgnore")]
    
    pub characters_to_ignore: Option<Vec<GooglePrivacyDlpV2CharsToIgnore>>,
    /// Character to use to mask the sensitive values—for example, `*` for an alphabetic string such as a name, or `0` for a numeric string such as ZIP code or credit card number. This string must have a length of 1. If not supplied, this value defaults to `*` for strings, and `0` for digits.
    #[serde(rename="maskingCharacter")]
    
    pub masking_character: Option<String>,
    /// Number of characters to mask. If not set, all matching chars will be masked. Skipped characters do not count towards this tally. If `number_to_mask` is negative, this denotes inverse masking. Cloud DLP masks all but a number of characters. For example, suppose you have the following values: - `masking_character` is `*` - `number_to_mask` is `-4` - `reverse_order` is `false` - `CharsToIgnore` includes `-` - Input string is `1234-5678-9012-3456` The resulting de-identified string is `****-****-****-3456`. Cloud DLP masks all but the last four characters. If `reverse_order` is `true`, all but the first four characters are masked as `1234-****-****-****`.
    #[serde(rename="numberToMask")]
    
    pub number_to_mask: Option<i32>,
    /// Mask characters in reverse order. For example, if `masking_character` is `0`, `number_to_mask` is `14`, and `reverse_order` is `false`, then the input string `1234-5678-9012-3456` is masked as `00000000000000-3456`. If `masking_character` is `*`, `number_to_mask` is `3`, and `reverse_order` is `true`, then the string `12345` is masked as `12***`.
    #[serde(rename="reverseOrder")]
    
    pub reverse_order: Option<bool>,
}

impl client::Part for GooglePrivacyDlpV2CharacterMaskConfig {}


/// Characters to skip when doing deidentification of a value. These will be left alone and skipped.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2CharsToIgnore {
    /// Characters to not transform when masking.
    #[serde(rename="charactersToSkip")]
    
    pub characters_to_skip: Option<String>,
    /// Common characters to not transform when masking. Useful to avoid removing punctuation.
    #[serde(rename="commonCharactersToIgnore")]
    
    pub common_characters_to_ignore: Option<GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnoreEnum>,
}

impl client::Part for GooglePrivacyDlpV2CharsToIgnore {}


/// Message representing a set of files in Cloud Storage.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2CloudStorageFileSet {
    /// The url, in the format `gs:///`. Trailing wildcard in the path is allowed.
    
    pub url: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2CloudStorageFileSet {}


/// Options defining a file or a set of files within a Cloud Storage bucket.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2CloudStorageOptions {
    /// Max number of bytes to scan from a file. If a scanned file's size is bigger than this value then the rest of the bytes are omitted. Only one of bytes_limit_per_file and bytes_limit_per_file_percent can be specified. Cannot be set if de-identification is requested.
    #[serde(rename="bytesLimitPerFile")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bytes_limit_per_file: Option<i64>,
    /// Max percentage of bytes to scan from a file. The rest are omitted. The number of bytes scanned is rounded down. Must be between 0 and 100, inclusively. Both 0 and 100 means no limit. Defaults to 0. Only one of bytes_limit_per_file and bytes_limit_per_file_percent can be specified. Cannot be set if de-identification is requested.
    #[serde(rename="bytesLimitPerFilePercent")]
    
    pub bytes_limit_per_file_percent: Option<i32>,
    /// The set of one or more files to scan.
    #[serde(rename="fileSet")]
    
    pub file_set: Option<GooglePrivacyDlpV2FileSet>,
    /// List of file type groups to include in the scan. If empty, all files are scanned and available data format processors are applied. In addition, the binary content of the selected files is always scanned as well. Images are scanned only as binary if the specified region does not support image inspection and no file_types were specified. Image inspection is restricted to 'global', 'us', 'asia', and 'europe'.
    #[serde(rename="fileTypes")]
    
    pub file_types: Option<Vec<GooglePrivacyDlpV2CloudStorageOptionFileTypesEnum>>,
    /// Limits the number of files to scan to this percentage of the input FileSet. Number of files scanned is rounded down. Must be between 0 and 100, inclusively. Both 0 and 100 means no limit. Defaults to 0.
    #[serde(rename="filesLimitPercent")]
    
    pub files_limit_percent: Option<i32>,
    /// no description provided
    #[serde(rename="sampleMethod")]
    
    pub sample_method: Option<GooglePrivacyDlpV2CloudStorageOptionSampleMethodEnum>,
}

impl client::Part for GooglePrivacyDlpV2CloudStorageOptions {}


/// Message representing a single file or path in Cloud Storage.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2CloudStoragePath {
    /// A url representing a file or path (no wildcards) in Cloud Storage. Example: gs://[BUCKET_NAME]/dictionary.txt
    
    pub path: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2CloudStoragePath {}


/// Message representing a set of files in a Cloud Storage bucket. Regular expressions are used to allow fine-grained control over which files in the bucket to include. Included files are those that match at least one item in `include_regex` and do not match any items in `exclude_regex`. Note that a file that matches items from both lists will _not_ be included. For a match to occur, the entire file path (i.e., everything in the url after the bucket name) must match the regular expression. For example, given the input `{bucket_name: "mybucket", include_regex: ["directory1/.*"], exclude_regex: ["directory1/excluded.*"]}`: * `gs://mybucket/directory1/myfile` will be included * `gs://mybucket/directory1/directory2/myfile` will be included (`.*` matches across `/`) * `gs://mybucket/directory0/directory1/myfile` will _not_ be included (the full path doesn't match any items in `include_regex`) * `gs://mybucket/directory1/excludedfile` will _not_ be included (the path matches an item in `exclude_regex`) If `include_regex` is left empty, it will match all files by default (this is equivalent to setting `include_regex: [".*"]`). Some other common use cases: * `{bucket_name: "mybucket", exclude_regex: [".*\.pdf"]}` will include all files in `mybucket` except for .pdf files * `{bucket_name: "mybucket", include_regex: ["directory/[^/]+"]}` will include all files directly under `gs://mybucket/directory/`, without matching across `/`
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2CloudStorageRegexFileSet {
    /// The name of a Cloud Storage bucket. Required.
    #[serde(rename="bucketName")]
    
    pub bucket_name: Option<String>,
    /// A list of regular expressions matching file paths to exclude. All files in the bucket that match at least one of these regular expressions will be excluded from the scan. Regular expressions use RE2 [syntax](https://github.com/google/re2/wiki/Syntax); a guide can be found under the google/re2 repository on GitHub.
    #[serde(rename="excludeRegex")]
    
    pub exclude_regex: Option<Vec<String>>,
    /// A list of regular expressions matching file paths to include. All files in the bucket that match at least one of these regular expressions will be included in the set of files, except for those that also match an item in `exclude_regex`. Leaving this field empty will match all files by default (this is equivalent to including `.*` in the list). Regular expressions use RE2 [syntax](https://github.com/google/re2/wiki/Syntax); a guide can be found under the google/re2 repository on GitHub.
    #[serde(rename="includeRegex")]
    
    pub include_regex: Option<Vec<String>>,
}

impl client::Part for GooglePrivacyDlpV2CloudStorageRegexFileSet {}


/// Represents a color in the RGB color space.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2Color {
    /// The amount of blue in the color as a value in the interval [0, 1].
    
    pub blue: Option<f32>,
    /// The amount of green in the color as a value in the interval [0, 1].
    
    pub green: Option<f32>,
    /// The amount of red in the color as a value in the interval [0, 1].
    
    pub red: Option<f32>,
}

impl client::Part for GooglePrivacyDlpV2Color {}


/// The field type of `value` and `field` do not need to match to be considered equal, but not all comparisons are possible. EQUAL_TO and NOT_EQUAL_TO attempt to compare even with incompatible types, but all other comparisons are invalid with incompatible types. A `value` of type: - `string` can be compared against all other types - `boolean` can only be compared against other booleans - `integer` can be compared against doubles or a string if the string value can be parsed as an integer. - `double` can be compared against integers or a string if the string can be parsed as a double. - `Timestamp` can be compared against strings in RFC 3339 date string format. - `TimeOfDay` can be compared against timestamps and strings in the format of 'HH:mm:ss'. If we fail to compare do to type mismatch, a warning will be given and the condition will evaluate to false.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2Condition {
    /// Required. Field within the record this condition is evaluated against.
    
    pub field: Option<GooglePrivacyDlpV2FieldId>,
    /// Required. Operator used to compare the field or infoType to the value.
    
    pub operator: Option<GooglePrivacyDlpV2ConditionOperatorEnum>,
    /// Value to compare against. [Mandatory, except for `EXISTS` tests.]
    
    pub value: Option<GooglePrivacyDlpV2Value>,
}

impl client::Part for GooglePrivacyDlpV2Condition {}


/// A collection of conditions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2Conditions {
    /// A collection of conditions.
    
    pub conditions: Option<Vec<GooglePrivacyDlpV2Condition>>,
}

impl client::Part for GooglePrivacyDlpV2Conditions {}


/// Represents a container that may contain DLP findings. Examples of a container include a file, table, or database record.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2Container {
    /// A string representation of the full container name. Examples: - BigQuery: 'Project:DataSetId.TableId' - Cloud Storage: 'gs://Bucket/folders/filename.txt'
    #[serde(rename="fullPath")]
    
    pub full_path: Option<String>,
    /// Project where the finding was found. Can be different from the project that owns the finding.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// The rest of the path after the root. Examples: - For BigQuery table `project_id:dataset_id.table_id`, the relative path is `table_id` - For Cloud Storage file `gs://bucket/folder/filename.txt`, the relative path is `folder/filename.txt`
    #[serde(rename="relativePath")]
    
    pub relative_path: Option<String>,
    /// The root of the container. Examples: - For BigQuery table `project_id:dataset_id.table_id`, the root is `dataset_id` - For Cloud Storage file `gs://bucket/folder/filename.txt`, the root is `gs://bucket`
    #[serde(rename="rootPath")]
    
    pub root_path: Option<String>,
    /// Container type, for example BigQuery or Cloud Storage.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Findings container modification timestamp, if applicable. For Cloud Storage, this field contains the last file modification timestamp. For a BigQuery table, this field contains the last_modified_time property. For Datastore, this field isn't populated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Findings container version, if available ("generation" for Cloud Storage).
    
    pub version: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2Container {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2ContentItem {
    /// Content data to inspect or redact. Replaces `type` and `data`.
    #[serde(rename="byteItem")]
    
    pub byte_item: Option<GooglePrivacyDlpV2ByteContentItem>,
    /// Structured content for inspection. See https://cloud.google.com/dlp/docs/inspecting-text#inspecting_a_table to learn more.
    
    pub table: Option<GooglePrivacyDlpV2Table>,
    /// String data to inspect or redact.
    
    pub value: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2ContentItem {}


/// Precise location of the finding within a document, record, image, or metadata container.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2ContentLocation {
    /// Name of the container where the finding is located. The top level name is the source file name or table name. Names of some common storage containers are formatted as follows: * BigQuery tables: `{project_id}:{dataset_id}.{table_id}` * Cloud Storage files: `gs://{bucket}/{path}` * Datastore namespace: {namespace} Nested names could be absent if the embedded object has no string identifier (for example, an image contained within a document).
    #[serde(rename="containerName")]
    
    pub container_name: Option<String>,
    /// Finding container modification timestamp, if applicable. For Cloud Storage, this field contains the last file modification timestamp. For a BigQuery table, this field contains the last_modified_time property. For Datastore, this field isn't populated.
    #[serde(rename="containerTimestamp")]
    
    pub container_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Finding container version, if available ("generation" for Cloud Storage).
    #[serde(rename="containerVersion")]
    
    pub container_version: Option<String>,
    /// Location data for document files.
    #[serde(rename="documentLocation")]
    
    pub document_location: Option<GooglePrivacyDlpV2DocumentLocation>,
    /// Location within an image's pixels.
    #[serde(rename="imageLocation")]
    
    pub image_location: Option<GooglePrivacyDlpV2ImageLocation>,
    /// Location within the metadata for inspected content.
    #[serde(rename="metadataLocation")]
    
    pub metadata_location: Option<GooglePrivacyDlpV2MetadataLocation>,
    /// Location within a row or record of a database table.
    #[serde(rename="recordLocation")]
    
    pub record_location: Option<GooglePrivacyDlpV2RecordLocation>,
}

impl client::Part for GooglePrivacyDlpV2ContentLocation {}


/// Request message for CreateDeidentifyTemplate.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [deidentify templates create organizations](OrganizationDeidentifyTemplateCreateCall) (request)
/// * [locations deidentify templates create organizations](OrganizationLocationDeidentifyTemplateCreateCall) (request)
/// * [deidentify templates create projects](ProjectDeidentifyTemplateCreateCall) (request)
/// * [locations deidentify templates create projects](ProjectLocationDeidentifyTemplateCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2CreateDeidentifyTemplateRequest {
    /// Required. The DeidentifyTemplate to create.
    #[serde(rename="deidentifyTemplate")]
    
    pub deidentify_template: Option<GooglePrivacyDlpV2DeidentifyTemplate>,
    /// Deprecated. This field has no effect.
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
    /// The template id can contain uppercase and lowercase letters, numbers, and hyphens; that is, it must match the regular expression: `[a-zA-Z\d-_]+`. The maximum length is 100 characters. Can be empty to allow the system to generate one.
    #[serde(rename="templateId")]
    
    pub template_id: Option<String>,
}

impl client::RequestValue for GooglePrivacyDlpV2CreateDeidentifyTemplateRequest {}


/// Request message for CreateDlpJobRequest. Used to initiate long running jobs such as calculating risk metrics or inspecting Google Cloud Storage.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [dlp jobs create projects](ProjectDlpJobCreateCall) (request)
/// * [locations dlp jobs create projects](ProjectLocationDlpJobCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2CreateDlpJobRequest {
    /// An inspection job scans a storage repository for InfoTypes.
    #[serde(rename="inspectJob")]
    
    pub inspect_job: Option<GooglePrivacyDlpV2InspectJobConfig>,
    /// The job id can contain uppercase and lowercase letters, numbers, and hyphens; that is, it must match the regular expression: `[a-zA-Z\d-_]+`. The maximum length is 100 characters. Can be empty to allow the system to generate one.
    #[serde(rename="jobId")]
    
    pub job_id: Option<String>,
    /// Deprecated. This field has no effect.
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
    /// A risk analysis job calculates re-identification risk metrics for a BigQuery table.
    #[serde(rename="riskJob")]
    
    pub risk_job: Option<GooglePrivacyDlpV2RiskAnalysisJobConfig>,
}

impl client::RequestValue for GooglePrivacyDlpV2CreateDlpJobRequest {}


/// Request message for CreateInspectTemplate.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [inspect templates create organizations](OrganizationInspectTemplateCreateCall) (request)
/// * [locations inspect templates create organizations](OrganizationLocationInspectTemplateCreateCall) (request)
/// * [inspect templates create projects](ProjectInspectTemplateCreateCall) (request)
/// * [locations inspect templates create projects](ProjectLocationInspectTemplateCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2CreateInspectTemplateRequest {
    /// Required. The InspectTemplate to create.
    #[serde(rename="inspectTemplate")]
    
    pub inspect_template: Option<GooglePrivacyDlpV2InspectTemplate>,
    /// Deprecated. This field has no effect.
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
    /// The template id can contain uppercase and lowercase letters, numbers, and hyphens; that is, it must match the regular expression: `[a-zA-Z\d-_]+`. The maximum length is 100 characters. Can be empty to allow the system to generate one.
    #[serde(rename="templateId")]
    
    pub template_id: Option<String>,
}

impl client::RequestValue for GooglePrivacyDlpV2CreateInspectTemplateRequest {}


/// Request message for CreateJobTrigger.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations job triggers create organizations](OrganizationLocationJobTriggerCreateCall) (request)
/// * [job triggers create projects](ProjectJobTriggerCreateCall) (request)
/// * [locations job triggers create projects](ProjectLocationJobTriggerCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2CreateJobTriggerRequest {
    /// Required. The JobTrigger to create.
    #[serde(rename="jobTrigger")]
    
    pub job_trigger: Option<GooglePrivacyDlpV2JobTrigger>,
    /// Deprecated. This field has no effect.
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
    /// The trigger id can contain uppercase and lowercase letters, numbers, and hyphens; that is, it must match the regular expression: `[a-zA-Z\d-_]+`. The maximum length is 100 characters. Can be empty to allow the system to generate one.
    #[serde(rename="triggerId")]
    
    pub trigger_id: Option<String>,
}

impl client::RequestValue for GooglePrivacyDlpV2CreateJobTriggerRequest {}


/// Request message for CreateStoredInfoType.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations stored info types create organizations](OrganizationLocationStoredInfoTypeCreateCall) (request)
/// * [stored info types create organizations](OrganizationStoredInfoTypeCreateCall) (request)
/// * [locations stored info types create projects](ProjectLocationStoredInfoTypeCreateCall) (request)
/// * [stored info types create projects](ProjectStoredInfoTypeCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2CreateStoredInfoTypeRequest {
    /// Required. Configuration of the storedInfoType to create.
    
    pub config: Option<GooglePrivacyDlpV2StoredInfoTypeConfig>,
    /// Deprecated. This field has no effect.
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
    /// The storedInfoType ID can contain uppercase and lowercase letters, numbers, and hyphens; that is, it must match the regular expression: `[a-zA-Z\d-_]+`. The maximum length is 100 characters. Can be empty to allow the system to generate one.
    #[serde(rename="storedInfoTypeId")]
    
    pub stored_info_type_id: Option<String>,
}

impl client::RequestValue for GooglePrivacyDlpV2CreateStoredInfoTypeRequest {}


/// Pseudonymization method that generates deterministic encryption for the given input. Outputs a base64 encoded representation of the encrypted output. Uses AES-SIV based on the RFC https://tools.ietf.org/html/rfc5297.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2CryptoDeterministicConfig {
    /// A context may be used for higher security and maintaining referential integrity such that the same identifier in two different contexts will be given a distinct surrogate. The context is appended to plaintext value being encrypted. On decryption the provided context is validated against the value used during encryption. If a context was provided during encryption, same context must be provided during decryption as well. If the context is not set, plaintext would be used as is for encryption. If the context is set but: 1. there is no record present when transforming a given value or 2. the field is not present when transforming a given value, plaintext would be used as is for encryption. Note that case (1) is expected when an `InfoTypeTransformation` is applied to both structured and unstructured `ContentItem`s.
    
    pub context: Option<GooglePrivacyDlpV2FieldId>,
    /// The key used by the encryption function. For deterministic encryption using AES-SIV, the provided key is internally expanded to 64 bytes prior to use.
    #[serde(rename="cryptoKey")]
    
    pub crypto_key: Option<GooglePrivacyDlpV2CryptoKey>,
    /// The custom info type to annotate the surrogate with. This annotation will be applied to the surrogate by prefixing it with the name of the custom info type followed by the number of characters comprising the surrogate. The following scheme defines the format: {info type name}({surrogate character count}):{surrogate} For example, if the name of custom info type is 'MY_TOKEN_INFO_TYPE' and the surrogate is 'abc', the full replacement value will be: 'MY_TOKEN_INFO_TYPE(3):abc' This annotation identifies the surrogate when inspecting content using the custom info type 'Surrogate'. This facilitates reversal of the surrogate when it occurs in free text. Note: For record transformations where the entire cell in a table is being transformed, surrogates are not mandatory. Surrogates are used to denote the location of the token and are necessary for re-identification in free form text. In order for inspection to work properly, the name of this info type must not occur naturally anywhere in your data; otherwise, inspection may either - reverse a surrogate that does not correspond to an actual identifier - be unable to parse the surrogate and result in an error Therefore, choose your custom info type name carefully after considering what your data looks like. One way to select a name that has a high chance of yielding reliable detection is to include one or more unicode characters that are highly improbable to exist in your data. For example, assuming your data is entered from a regular ASCII keyboard, the symbol with the hex code point 29DD might be used like so: ⧝MY_TOKEN_TYPE.
    #[serde(rename="surrogateInfoType")]
    
    pub surrogate_info_type: Option<GooglePrivacyDlpV2InfoType>,
}

impl client::Part for GooglePrivacyDlpV2CryptoDeterministicConfig {}


/// Pseudonymization method that generates surrogates via cryptographic hashing. Uses SHA-256. The key size must be either 32 or 64 bytes. Outputs a base64 encoded representation of the hashed output (for example, L7k0BHmF1ha5U3NfGykjro4xWi1MPVQPjhMAZbSV9mM=). Currently, only string and integer values can be hashed. See https://cloud.google.com/dlp/docs/pseudonymization to learn more.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2CryptoHashConfig {
    /// The key used by the hash function.
    #[serde(rename="cryptoKey")]
    
    pub crypto_key: Option<GooglePrivacyDlpV2CryptoKey>,
}

impl client::Part for GooglePrivacyDlpV2CryptoHashConfig {}


/// This is a data encryption key (DEK) (as opposed to a key encryption key (KEK) stored by Cloud Key Management Service (Cloud KMS). When using Cloud KMS to wrap or unwrap a DEK, be sure to set an appropriate IAM policy on the KEK to ensure an attacker cannot unwrap the DEK.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2CryptoKey {
    /// Key wrapped using Cloud KMS
    #[serde(rename="kmsWrapped")]
    
    pub kms_wrapped: Option<GooglePrivacyDlpV2KmsWrappedCryptoKey>,
    /// Transient crypto key
    
    pub transient: Option<GooglePrivacyDlpV2TransientCryptoKey>,
    /// Unwrapped crypto key
    
    pub unwrapped: Option<GooglePrivacyDlpV2UnwrappedCryptoKey>,
}

impl client::Part for GooglePrivacyDlpV2CryptoKey {}


/// Replaces an identifier with a surrogate using Format Preserving Encryption (FPE) with the FFX mode of operation; however when used in the `ReidentifyContent` API method, it serves the opposite function by reversing the surrogate back into the original identifier. The identifier must be encoded as ASCII. For a given crypto key and context, the same identifier will be replaced with the same surrogate. Identifiers must be at least two characters long. In the case that the identifier is the empty string, it will be skipped. See https://cloud.google.com/dlp/docs/pseudonymization to learn more. Note: We recommend using CryptoDeterministicConfig for all use cases which do not require preserving the input alphabet space and size, plus warrant referential integrity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2CryptoReplaceFfxFpeConfig {
    /// Common alphabets.
    #[serde(rename="commonAlphabet")]
    
    pub common_alphabet: Option<GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabetEnum>,
    /// The 'tweak', a context may be used for higher security since the same identifier in two different contexts won't be given the same surrogate. If the context is not set, a default tweak will be used. If the context is set but: 1. there is no record present when transforming a given value or 1. the field is not present when transforming a given value, a default tweak will be used. Note that case (1) is expected when an `InfoTypeTransformation` is applied to both structured and unstructured `ContentItem`s. Currently, the referenced field may be of value type integer or string. The tweak is constructed as a sequence of bytes in big endian byte order such that: - a 64 bit integer is encoded followed by a single byte of value 1 - a string is encoded in UTF-8 format followed by a single byte of value 2
    
    pub context: Option<GooglePrivacyDlpV2FieldId>,
    /// Required. The key used by the encryption algorithm.
    #[serde(rename="cryptoKey")]
    
    pub crypto_key: Option<GooglePrivacyDlpV2CryptoKey>,
    /// This is supported by mapping these to the alphanumeric characters that the FFX mode natively supports. This happens before/after encryption/decryption. Each character listed must appear only once. Number of characters must be in the range [2, 95]. This must be encoded as ASCII. The order of characters does not matter. The full list of allowed characters is: 0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz ~`!@#$%^&*()_-+={[}]|\:;"'<,>.?/
    #[serde(rename="customAlphabet")]
    
    pub custom_alphabet: Option<String>,
    /// The native way to select the alphabet. Must be in the range [2, 95].
    
    pub radix: Option<i32>,
    /// The custom infoType to annotate the surrogate with. This annotation will be applied to the surrogate by prefixing it with the name of the custom infoType followed by the number of characters comprising the surrogate. The following scheme defines the format: info_type_name(surrogate_character_count):surrogate For example, if the name of custom infoType is 'MY_TOKEN_INFO_TYPE' and the surrogate is 'abc', the full replacement value will be: 'MY_TOKEN_INFO_TYPE(3):abc' This annotation identifies the surrogate when inspecting content using the custom infoType [`SurrogateType`](https://cloud.google.com/dlp/docs/reference/rest/v2/InspectConfig#surrogatetype). This facilitates reversal of the surrogate when it occurs in free text. In order for inspection to work properly, the name of this infoType must not occur naturally anywhere in your data; otherwise, inspection may find a surrogate that does not correspond to an actual identifier. Therefore, choose your custom infoType name carefully after considering what your data looks like. One way to select a name that has a high chance of yielding reliable detection is to include one or more unicode characters that are highly improbable to exist in your data. For example, assuming your data is entered from a regular ASCII keyboard, the symbol with the hex code point 29DD might be used like so: ⧝MY_TOKEN_TYPE
    #[serde(rename="surrogateInfoType")]
    
    pub surrogate_info_type: Option<GooglePrivacyDlpV2InfoType>,
}

impl client::Part for GooglePrivacyDlpV2CryptoReplaceFfxFpeConfig {}


/// Custom information type provided by the user. Used to find domain-specific sensitive information configurable to the data in question.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2CustomInfoType {
    /// Set of detection rules to apply to all findings of this CustomInfoType. Rules are applied in order that they are specified. Not supported for the `surrogate_type` CustomInfoType.
    #[serde(rename="detectionRules")]
    
    pub detection_rules: Option<Vec<GooglePrivacyDlpV2DetectionRule>>,
    /// A list of phrases to detect as a CustomInfoType.
    
    pub dictionary: Option<GooglePrivacyDlpV2Dictionary>,
    /// If set to EXCLUSION_TYPE_EXCLUDE this infoType will not cause a finding to be returned. It still can be used for rules matching.
    #[serde(rename="exclusionType")]
    
    pub exclusion_type: Option<GooglePrivacyDlpV2CustomInfoTypeExclusionTypeEnum>,
    /// CustomInfoType can either be a new infoType, or an extension of built-in infoType, when the name matches one of existing infoTypes and that infoType is specified in `InspectContent.info_types` field. Specifying the latter adds findings to the one detected by the system. If built-in info type is not specified in `InspectContent.info_types` list then the name is treated as a custom info type.
    #[serde(rename="infoType")]
    
    pub info_type: Option<GooglePrivacyDlpV2InfoType>,
    /// Likelihood to return for this CustomInfoType. This base value can be altered by a detection rule if the finding meets the criteria specified by the rule. Defaults to `VERY_LIKELY` if not specified.
    
    pub likelihood: Option<GooglePrivacyDlpV2CustomInfoTypeLikelihoodEnum>,
    /// Regular expression based CustomInfoType.
    
    pub regex: Option<GooglePrivacyDlpV2Regex>,
    /// Load an existing `StoredInfoType` resource for use in `InspectDataSource`. Not currently supported in `InspectContent`.
    #[serde(rename="storedType")]
    
    pub stored_type: Option<GooglePrivacyDlpV2StoredType>,
    /// Message for detecting output from deidentification transformations that support reversing.
    #[serde(rename="surrogateType")]
    
    pub surrogate_type: Option<GooglePrivacyDlpV2SurrogateType>,
}

impl client::Part for GooglePrivacyDlpV2CustomInfoType {}


/// Record key for a finding in Cloud Datastore.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2DatastoreKey {
    /// Datastore entity key.
    #[serde(rename="entityKey")]
    
    pub entity_key: Option<GooglePrivacyDlpV2Key>,
}

impl client::Part for GooglePrivacyDlpV2DatastoreKey {}


/// Options defining a data set within Google Cloud Datastore.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2DatastoreOptions {
    /// The kind to process.
    
    pub kind: Option<GooglePrivacyDlpV2KindExpression>,
    /// A partition ID identifies a grouping of entities. The grouping is always by project and namespace, however the namespace ID may be empty.
    #[serde(rename="partitionId")]
    
    pub partition_id: Option<GooglePrivacyDlpV2PartitionId>,
}

impl client::Part for GooglePrivacyDlpV2DatastoreOptions {}


/// Shifts dates by random number of days, with option to be consistent for the same context. See https://cloud.google.com/dlp/docs/concepts-date-shifting to learn more.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2DateShiftConfig {
    /// Points to the field that contains the context, for example, an entity id. If set, must also set cryptoKey. If set, shift will be consistent for the given context.
    
    pub context: Option<GooglePrivacyDlpV2FieldId>,
    /// Causes the shift to be computed based on this key and the context. This results in the same shift for the same context and crypto_key. If set, must also set context. Can only be applied to table items.
    #[serde(rename="cryptoKey")]
    
    pub crypto_key: Option<GooglePrivacyDlpV2CryptoKey>,
    /// Required. For example, -5 means shift date to at most 5 days back in the past.
    #[serde(rename="lowerBoundDays")]
    
    pub lower_bound_days: Option<i32>,
    /// Required. Range of shift in days. Actual shift will be selected at random within this range (inclusive ends). Negative means shift to earlier in time. Must not be more than 365250 days (1000 years) each direction. For example, 3 means shift date to at most 3 days into the future.
    #[serde(rename="upperBoundDays")]
    
    pub upper_bound_days: Option<i32>,
}

impl client::Part for GooglePrivacyDlpV2DateShiftConfig {}


/// Message for a date time object. e.g. 2018-01-01, 5th August.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2DateTime {
    /// One or more of the following must be set. Must be a valid date or time value.
    
    pub date: Option<GoogleTypeDate>,
    /// Day of week
    #[serde(rename="dayOfWeek")]
    
    pub day_of_week: Option<GooglePrivacyDlpV2DateTimeDayOfWeekEnum>,
    /// Time of day
    
    pub time: Option<GoogleTypeTimeOfDay>,
    /// Time zone
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<GooglePrivacyDlpV2TimeZone>,
}

impl client::Part for GooglePrivacyDlpV2DateTime {}


/// Create a de-identified copy of the requested table or files. A TransformationDetail will be created for each transformation. If any rows in BigQuery are skipped during de-identification (transformation errors or row size exceeds BigQuery insert API limits) they are placed in the failure output table. If the original row exceeds the BigQuery insert API limit it will be truncated when written to the failure output table. The failure output table can be set in the action.deidentify.output.big_query_output.deidentified_failure_output_table field, if no table is set, a table will be automatically created in the same project and dataset as the original table. Compatible with: Inspect
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2Deidentify {
    /// Required. User settable Cloud Storage bucket and folders to store de-identified files. This field must be set for cloud storage deidentification. The output Cloud Storage bucket must be different from the input bucket. De-identified files will overwrite files in the output path. Form of: gs://bucket/folder/ or gs://bucket
    #[serde(rename="cloudStorageOutput")]
    
    pub cloud_storage_output: Option<String>,
    /// List of user-specified file type groups to transform. If specified, only the files with these filetypes will be transformed. If empty, all supported files will be transformed. Supported types may be automatically added over time. If a file type is set in this field that isn't supported by the Deidentify action then the job will fail and will not be successfully created/started. Currently the only filetypes supported are: IMAGES, TEXT_FILES, CSV, TSV.
    #[serde(rename="fileTypesToTransform")]
    
    pub file_types_to_transform: Option<Vec<GooglePrivacyDlpV2DeidentifyFileTypesToTransformEnum>>,
    /// User specified deidentify templates and configs for structured, unstructured, and image files.
    #[serde(rename="transformationConfig")]
    
    pub transformation_config: Option<GooglePrivacyDlpV2TransformationConfig>,
    /// Config for storing transformation details. This is separate from the de-identified content, and contains metadata about the successful transformations and/or failures that occurred while de-identifying. This needs to be set in order for users to access information about the status of each transformation (see TransformationDetails message for more information about what is noted).
    #[serde(rename="transformationDetailsStorageConfig")]
    
    pub transformation_details_storage_config: Option<GooglePrivacyDlpV2TransformationDetailsStorageConfig>,
}

impl client::Part for GooglePrivacyDlpV2Deidentify {}


/// The configuration that controls how the data will change.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2DeidentifyConfig {
    /// Treat the dataset as an image and redact.
    #[serde(rename="imageTransformations")]
    
    pub image_transformations: Option<GooglePrivacyDlpV2ImageTransformations>,
    /// Treat the dataset as free-form text and apply the same free text transformation everywhere.
    #[serde(rename="infoTypeTransformations")]
    
    pub info_type_transformations: Option<GooglePrivacyDlpV2InfoTypeTransformations>,
    /// Treat the dataset as structured. Transformations can be applied to specific locations within structured datasets, such as transforming a column within a table.
    #[serde(rename="recordTransformations")]
    
    pub record_transformations: Option<GooglePrivacyDlpV2RecordTransformations>,
    /// Mode for handling transformation errors. If left unspecified, the default mode is `TransformationErrorHandling.ThrowError`.
    #[serde(rename="transformationErrorHandling")]
    
    pub transformation_error_handling: Option<GooglePrivacyDlpV2TransformationErrorHandling>,
}

impl client::Part for GooglePrivacyDlpV2DeidentifyConfig {}


/// Request to de-identify a ContentItem.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [content deidentify projects](ProjectContentDeidentifyCall) (request)
/// * [locations content deidentify projects](ProjectLocationContentDeidentifyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2DeidentifyContentRequest {
    /// Configuration for the de-identification of the content item. Items specified here will override the template referenced by the deidentify_template_name argument.
    #[serde(rename="deidentifyConfig")]
    
    pub deidentify_config: Option<GooglePrivacyDlpV2DeidentifyConfig>,
    /// Template to use. Any configuration directly specified in deidentify_config will override those set in the template. Singular fields that are set in this request will replace their corresponding fields in the template. Repeated fields are appended. Singular sub-messages and groups are recursively merged.
    #[serde(rename="deidentifyTemplateName")]
    
    pub deidentify_template_name: Option<String>,
    /// Configuration for the inspector. Items specified here will override the template referenced by the inspect_template_name argument.
    #[serde(rename="inspectConfig")]
    
    pub inspect_config: Option<GooglePrivacyDlpV2InspectConfig>,
    /// Template to use. Any configuration directly specified in inspect_config will override those set in the template. Singular fields that are set in this request will replace their corresponding fields in the template. Repeated fields are appended. Singular sub-messages and groups are recursively merged.
    #[serde(rename="inspectTemplateName")]
    
    pub inspect_template_name: Option<String>,
    /// The item to de-identify. Will be treated as text. This value must be of type Table if your deidentify_config is a RecordTransformations object.
    
    pub item: Option<GooglePrivacyDlpV2ContentItem>,
    /// Deprecated. This field has no effect.
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
}

impl client::RequestValue for GooglePrivacyDlpV2DeidentifyContentRequest {}


/// Results of de-identifying a ContentItem.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [content deidentify projects](ProjectContentDeidentifyCall) (response)
/// * [locations content deidentify projects](ProjectLocationContentDeidentifyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2DeidentifyContentResponse {
    /// The de-identified item.
    
    pub item: Option<GooglePrivacyDlpV2ContentItem>,
    /// An overview of the changes that were made on the `item`.
    
    pub overview: Option<GooglePrivacyDlpV2TransformationOverview>,
}

impl client::ResponseResult for GooglePrivacyDlpV2DeidentifyContentResponse {}


/// DeidentifyTemplates contains instructions on how to de-identify content. See https://cloud.google.com/dlp/docs/concepts-templates to learn more.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [deidentify templates create organizations](OrganizationDeidentifyTemplateCreateCall) (response)
/// * [deidentify templates get organizations](OrganizationDeidentifyTemplateGetCall) (response)
/// * [deidentify templates patch organizations](OrganizationDeidentifyTemplatePatchCall) (response)
/// * [locations deidentify templates create organizations](OrganizationLocationDeidentifyTemplateCreateCall) (response)
/// * [locations deidentify templates get organizations](OrganizationLocationDeidentifyTemplateGetCall) (response)
/// * [locations deidentify templates patch organizations](OrganizationLocationDeidentifyTemplatePatchCall) (response)
/// * [deidentify templates create projects](ProjectDeidentifyTemplateCreateCall) (response)
/// * [deidentify templates get projects](ProjectDeidentifyTemplateGetCall) (response)
/// * [deidentify templates patch projects](ProjectDeidentifyTemplatePatchCall) (response)
/// * [locations deidentify templates create projects](ProjectLocationDeidentifyTemplateCreateCall) (response)
/// * [locations deidentify templates get projects](ProjectLocationDeidentifyTemplateGetCall) (response)
/// * [locations deidentify templates patch projects](ProjectLocationDeidentifyTemplatePatchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2DeidentifyTemplate {
    /// Output only. The creation timestamp of an inspectTemplate.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The core content of the template.
    #[serde(rename="deidentifyConfig")]
    
    pub deidentify_config: Option<GooglePrivacyDlpV2DeidentifyConfig>,
    /// Short description (max 256 chars).
    
    pub description: Option<String>,
    /// Display name (max 256 chars).
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. The template name. The template will have one of the following formats: `projects/PROJECT_ID/deidentifyTemplates/TEMPLATE_ID` OR `organizations/ORGANIZATION_ID/deidentifyTemplates/TEMPLATE_ID`
    
    pub name: Option<String>,
    /// Output only. The last update timestamp of an inspectTemplate.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for GooglePrivacyDlpV2DeidentifyTemplate {}


/// δ-presence metric, used to estimate how likely it is for an attacker to figure out that one given individual appears in a de-identified dataset. Similarly to the k-map metric, we cannot compute δ-presence exactly without knowing the attack dataset, so we use a statistical model instead.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2DeltaPresenceEstimationConfig {
    /// Several auxiliary tables can be used in the analysis. Each custom_tag used to tag a quasi-identifiers field must appear in exactly one field of one auxiliary table.
    #[serde(rename="auxiliaryTables")]
    
    pub auxiliary_tables: Option<Vec<GooglePrivacyDlpV2StatisticalTable>>,
    /// Required. Fields considered to be quasi-identifiers. No two fields can have the same tag.
    #[serde(rename="quasiIds")]
    
    pub quasi_ids: Option<Vec<GooglePrivacyDlpV2QuasiId>>,
    /// ISO 3166-1 alpha-2 region code to use in the statistical modeling. Set if no column is tagged with a region-specific InfoType (like US_ZIP_5) or a region code.
    #[serde(rename="regionCode")]
    
    pub region_code: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2DeltaPresenceEstimationConfig {}


/// A DeltaPresenceEstimationHistogramBucket message with the following values: min_probability: 0.1 max_probability: 0.2 frequency: 42 means that there are 42 records for which δ is in [0.1, 0.2). An important particular case is when min_probability = max_probability = 1: then, every individual who shares this quasi-identifier combination is in the dataset.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2DeltaPresenceEstimationHistogramBucket {
    /// Number of records within these probability bounds.
    #[serde(rename="bucketSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bucket_size: Option<i64>,
    /// Total number of distinct quasi-identifier tuple values in this bucket.
    #[serde(rename="bucketValueCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bucket_value_count: Option<i64>,
    /// Sample of quasi-identifier tuple values in this bucket. The total number of classes returned per bucket is capped at 20.
    #[serde(rename="bucketValues")]
    
    pub bucket_values: Option<Vec<GooglePrivacyDlpV2DeltaPresenceEstimationQuasiIdValues>>,
    /// Always greater than or equal to min_probability.
    #[serde(rename="maxProbability")]
    
    pub max_probability: Option<f64>,
    /// Between 0 and 1.
    #[serde(rename="minProbability")]
    
    pub min_probability: Option<f64>,
}

impl client::Part for GooglePrivacyDlpV2DeltaPresenceEstimationHistogramBucket {}


/// A tuple of values for the quasi-identifier columns.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2DeltaPresenceEstimationQuasiIdValues {
    /// The estimated probability that a given individual sharing these quasi-identifier values is in the dataset. This value, typically called δ, is the ratio between the number of records in the dataset with these quasi-identifier values, and the total number of individuals (inside *and* outside the dataset) with these quasi-identifier values. For example, if there are 15 individuals in the dataset who share the same quasi-identifier values, and an estimated 100 people in the entire population with these values, then δ is 0.15.
    #[serde(rename="estimatedProbability")]
    
    pub estimated_probability: Option<f64>,
    /// The quasi-identifier values.
    #[serde(rename="quasiIdsValues")]
    
    pub quasi_ids_values: Option<Vec<GooglePrivacyDlpV2Value>>,
}

impl client::Part for GooglePrivacyDlpV2DeltaPresenceEstimationQuasiIdValues {}


/// Result of the δ-presence computation. Note that these results are an estimation, not exact values.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2DeltaPresenceEstimationResult {
    /// The intervals [min_probability, max_probability) do not overlap. If a value doesn't correspond to any such interval, the associated frequency is zero. For example, the following records: {min_probability: 0, max_probability: 0.1, frequency: 17} {min_probability: 0.2, max_probability: 0.3, frequency: 42} {min_probability: 0.3, max_probability: 0.4, frequency: 99} mean that there are no record with an estimated probability in [0.1, 0.2) nor larger or equal to 0.4.
    #[serde(rename="deltaPresenceEstimationHistogram")]
    
    pub delta_presence_estimation_histogram: Option<Vec<GooglePrivacyDlpV2DeltaPresenceEstimationHistogramBucket>>,
}

impl client::Part for GooglePrivacyDlpV2DeltaPresenceEstimationResult {}


/// Deprecated; use `InspectionRuleSet` instead. Rule for modifying a `CustomInfoType` to alter behavior under certain circumstances, depending on the specific details of the rule. Not supported for the `surrogate_type` custom infoType.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2DetectionRule {
    /// Hotword-based detection rule.
    #[serde(rename="hotwordRule")]
    
    pub hotword_rule: Option<GooglePrivacyDlpV2HotwordRule>,
}

impl client::Part for GooglePrivacyDlpV2DetectionRule {}


/// Custom information type based on a dictionary of words or phrases. This can be used to match sensitive information specific to the data, such as a list of employee IDs or job titles. Dictionary words are case-insensitive and all characters other than letters and digits in the unicode [Basic Multilingual Plane](https://en.wikipedia.org/wiki/Plane_%28Unicode%29#Basic_Multilingual_Plane) will be replaced with whitespace when scanning for matches, so the dictionary phrase "Sam Johnson" will match all three phrases "sam johnson", "Sam, Johnson", and "Sam (Johnson)". Additionally, the characters surrounding any match must be of a different type than the adjacent characters within the word, so letters must be next to non-letters and digits next to non-digits. For example, the dictionary word "jen" will match the first three letters of the text "jen123" but will return no matches for "jennifer". Dictionary words containing a large number of characters that are not letters or digits may result in unexpected findings because such characters are treated as whitespace. The [limits](https://cloud.google.com/dlp/limits) page contains details about the size limits of dictionaries. For dictionaries that do not fit within these constraints, consider using `LargeCustomDictionaryConfig` in the `StoredInfoType` API.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2Dictionary {
    /// Newline-delimited file of words in Cloud Storage. Only a single file is accepted.
    #[serde(rename="cloudStoragePath")]
    
    pub cloud_storage_path: Option<GooglePrivacyDlpV2CloudStoragePath>,
    /// List of words or phrases to search for.
    #[serde(rename="wordList")]
    
    pub word_list: Option<GooglePrivacyDlpV2WordList>,
}

impl client::Part for GooglePrivacyDlpV2Dictionary {}


/// Combines all of the information about a DLP job.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [dlp jobs create projects](ProjectDlpJobCreateCall) (response)
/// * [dlp jobs get projects](ProjectDlpJobGetCall) (response)
/// * [job triggers activate projects](ProjectJobTriggerActivateCall) (response)
/// * [locations dlp jobs create projects](ProjectLocationDlpJobCreateCall) (response)
/// * [locations dlp jobs get projects](ProjectLocationDlpJobGetCall) (response)
/// * [locations job triggers activate projects](ProjectLocationJobTriggerActivateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2DlpJob {
    /// Time when the job was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Time when the job finished.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A stream of errors encountered running the job.
    
    pub errors: Option<Vec<GooglePrivacyDlpV2Error>>,
    /// Results from inspecting a data source.
    #[serde(rename="inspectDetails")]
    
    pub inspect_details: Option<GooglePrivacyDlpV2InspectDataSourceDetails>,
    /// If created by a job trigger, the resource name of the trigger that instantiated the job.
    #[serde(rename="jobTriggerName")]
    
    pub job_trigger_name: Option<String>,
    /// The server-assigned name.
    
    pub name: Option<String>,
    /// Results from analyzing risk of a data source.
    #[serde(rename="riskDetails")]
    
    pub risk_details: Option<GooglePrivacyDlpV2AnalyzeDataSourceRiskDetails>,
    /// Time when the job started.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// State of a job.
    
    pub state: Option<GooglePrivacyDlpV2DlpJobStateEnum>,
    /// The type of job.
    #[serde(rename="type")]
    
    pub type_: Option<GooglePrivacyDlpV2DlpJobTypeEnum>,
}

impl client::ResponseResult for GooglePrivacyDlpV2DlpJob {}


/// Location of a finding within a document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2DocumentLocation {
    /// Offset of the line, from the beginning of the file, where the finding is located.
    #[serde(rename="fileOffset")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub file_offset: Option<i64>,
}

impl client::Part for GooglePrivacyDlpV2DocumentLocation {}


/// An entity in a dataset is a field or set of fields that correspond to a single person. For example, in medical records the `EntityId` might be a patient identifier, or for financial records it might be an account identifier. This message is used when generalizations or analysis must take into account that multiple rows correspond to the same entity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2EntityId {
    /// Composite key indicating which field contains the entity identifier.
    
    pub field: Option<GooglePrivacyDlpV2FieldId>,
}

impl client::Part for GooglePrivacyDlpV2EntityId {}


/// Details information about an error encountered during job execution or the results of an unsuccessful activation of the JobTrigger.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2Error {
    /// Detailed error codes and messages.
    
    pub details: Option<GoogleRpcStatus>,
    /// The times the error occurred.
    
    pub timestamps: Option<Vec<client::chrono::DateTime<client::chrono::offset::Utc>>>,
}

impl client::Part for GooglePrivacyDlpV2Error {}


/// The rule to exclude findings based on a hotword. For record inspection of tables, column names are considered hotwords. An example of this is to exclude a finding if it belongs to a BigQuery column that matches a specific pattern.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2ExcludeByHotword {
    /// Regular expression pattern defining what qualifies as a hotword.
    #[serde(rename="hotwordRegex")]
    
    pub hotword_regex: Option<GooglePrivacyDlpV2Regex>,
    /// Range of characters within which the entire hotword must reside. The total length of the window cannot exceed 1000 characters. The windowBefore property in proximity should be set to 1 if the hotword needs to be included in a column header.
    
    pub proximity: Option<GooglePrivacyDlpV2Proximity>,
}

impl client::Part for GooglePrivacyDlpV2ExcludeByHotword {}


/// List of excluded infoTypes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2ExcludeInfoTypes {
    /// InfoType list in ExclusionRule rule drops a finding when it overlaps or contained within with a finding of an infoType from this list. For example, for `InspectionRuleSet.info_types` containing "PHONE_NUMBER"` and `exclusion_rule` containing `exclude_info_types.info_types` with "EMAIL_ADDRESS" the phone number findings are dropped if they overlap with EMAIL_ADDRESS finding. That leads to "555-222-2222@example.org" to generate only a single finding, namely email address.
    #[serde(rename="infoTypes")]
    
    pub info_types: Option<Vec<GooglePrivacyDlpV2InfoType>>,
}

impl client::Part for GooglePrivacyDlpV2ExcludeInfoTypes {}


/// The rule that specifies conditions when findings of infoTypes specified in `InspectionRuleSet` are removed from results.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2ExclusionRule {
    /// Dictionary which defines the rule.
    
    pub dictionary: Option<GooglePrivacyDlpV2Dictionary>,
    /// Drop if the hotword rule is contained in the proximate context. For tabular data, the context includes the column name.
    #[serde(rename="excludeByHotword")]
    
    pub exclude_by_hotword: Option<GooglePrivacyDlpV2ExcludeByHotword>,
    /// Set of infoTypes for which findings would affect this rule.
    #[serde(rename="excludeInfoTypes")]
    
    pub exclude_info_types: Option<GooglePrivacyDlpV2ExcludeInfoTypes>,
    /// How the rule is applied, see MatchingType documentation for details.
    #[serde(rename="matchingType")]
    
    pub matching_type: Option<GooglePrivacyDlpV2ExclusionRuleMatchingTypeEnum>,
    /// Regular expression which defines the rule.
    
    pub regex: Option<GooglePrivacyDlpV2Regex>,
}

impl client::Part for GooglePrivacyDlpV2ExclusionRule {}


/// An expression, consisting of an operator and conditions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2Expressions {
    /// Conditions to apply to the expression.
    
    pub conditions: Option<GooglePrivacyDlpV2Conditions>,
    /// The operator to apply to the result of conditions. Default and currently only supported value is `AND`.
    #[serde(rename="logicalOperator")]
    
    pub logical_operator: Option<GooglePrivacyDlpV2ExpressionLogicalOperatorEnum>,
}

impl client::Part for GooglePrivacyDlpV2Expressions {}


/// General identifier of a data field in a storage service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2FieldId {
    /// Name describing the field.
    
    pub name: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2FieldId {}


/// The transformation to apply to the field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2FieldTransformation {
    /// Only apply the transformation if the condition evaluates to true for the given `RecordCondition`. The conditions are allowed to reference fields that are not used in the actual transformation. Example Use Cases: - Apply a different bucket transformation to an age column if the zip code column for the same record is within a specific range. - Redact a field if the date of birth field is greater than 85.
    
    pub condition: Option<GooglePrivacyDlpV2RecordCondition>,
    /// Required. Input field(s) to apply the transformation to. When you have columns that reference their position within a list, omit the index from the FieldId. FieldId name matching ignores the index. For example, instead of "contact.nums[0].type", use "contact.nums.type".
    
    pub fields: Option<Vec<GooglePrivacyDlpV2FieldId>>,
    /// Treat the contents of the field as free text, and selectively transform content that matches an `InfoType`.
    #[serde(rename="infoTypeTransformations")]
    
    pub info_type_transformations: Option<GooglePrivacyDlpV2InfoTypeTransformations>,
    /// Apply the transformation to the entire field.
    #[serde(rename="primitiveTransformation")]
    
    pub primitive_transformation: Option<GooglePrivacyDlpV2PrimitiveTransformation>,
}

impl client::Part for GooglePrivacyDlpV2FieldTransformation {}


/// Set of files to scan.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2FileSet {
    /// The regex-filtered set of files to scan. Exactly one of `url` or `regex_file_set` must be set.
    #[serde(rename="regexFileSet")]
    
    pub regex_file_set: Option<GooglePrivacyDlpV2CloudStorageRegexFileSet>,
    /// The Cloud Storage url of the file(s) to scan, in the format `gs:///`. Trailing wildcard in the path is allowed. If the url ends in a trailing slash, the bucket or directory represented by the url will be scanned non-recursively (content in sub-directories will not be scanned). This means that `gs://mybucket/` is equivalent to `gs://mybucket/*`, and `gs://mybucket/directory/` is equivalent to `gs://mybucket/directory/*`. Exactly one of `url` or `regex_file_set` must be set.
    
    pub url: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2FileSet {}


/// Represents a piece of potentially sensitive content.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2Finding {
    /// Timestamp when finding was detected.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The unique finding id.
    #[serde(rename="findingId")]
    
    pub finding_id: Option<String>,
    /// The type of content that might have been found. Provided if `excluded_types` is false.
    #[serde(rename="infoType")]
    
    pub info_type: Option<GooglePrivacyDlpV2InfoType>,
    /// Time the job started that produced this finding.
    #[serde(rename="jobCreateTime")]
    
    pub job_create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The job that stored the finding.
    #[serde(rename="jobName")]
    
    pub job_name: Option<String>,
    /// The labels associated with this `Finding`. Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?`. Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`. No more than 10 labels can be associated with a given finding. Examples: * `"environment" : "production"` * `"pipeline" : "etl"`
    
    pub labels: Option<HashMap<String, String>>,
    /// Confidence of how likely it is that the `info_type` is correct.
    
    pub likelihood: Option<GooglePrivacyDlpV2FindingLikelihoodEnum>,
    /// Where the content was found.
    
    pub location: Option<GooglePrivacyDlpV2Location>,
    /// Resource name in format projects/{project}/locations/{location}/findings/{finding} Populated only when viewing persisted findings.
    
    pub name: Option<String>,
    /// The content that was found. Even if the content is not textual, it may be converted to a textual representation here. Provided if `include_quote` is true and the finding is less than or equal to 4096 bytes long. If the finding exceeds 4096 bytes in length, the quote may be omitted.
    
    pub quote: Option<String>,
    /// Contains data parsed from quotes. Only populated if include_quote was set to true and a supported infoType was requested. Currently supported infoTypes: DATE, DATE_OF_BIRTH and TIME.
    #[serde(rename="quoteInfo")]
    
    pub quote_info: Option<GooglePrivacyDlpV2QuoteInfo>,
    /// The job that stored the finding.
    #[serde(rename="resourceName")]
    
    pub resource_name: Option<String>,
    /// Job trigger name, if applicable, for this finding.
    #[serde(rename="triggerName")]
    
    pub trigger_name: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2Finding {}


/// Configuration to control the number of findings returned for inspection. This is not used for de-identification or data profiling. When redacting sensitive data from images, finding limits don't apply. They can cause unexpected or inconsistent results, where only some data is redacted. Don't include finding limits in RedactImage requests. Otherwise, Cloud DLP returns an error.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2FindingLimits {
    /// Configuration of findings limit given for specified infoTypes.
    #[serde(rename="maxFindingsPerInfoType")]
    
    pub max_findings_per_info_type: Option<Vec<GooglePrivacyDlpV2InfoTypeLimit>>,
    /// Max number of findings that will be returned for each item scanned. When set within `InspectJobConfig`, the maximum returned is 2000 regardless if this is set higher. When set within `InspectContentRequest`, this field is ignored.
    #[serde(rename="maxFindingsPerItem")]
    
    pub max_findings_per_item: Option<i32>,
    /// Max number of findings that will be returned per request/job. When set within `InspectContentRequest`, the maximum returned is 2000 regardless if this is set higher.
    #[serde(rename="maxFindingsPerRequest")]
    
    pub max_findings_per_request: Option<i32>,
}

impl client::Part for GooglePrivacyDlpV2FindingLimits {}


/// The request message for finishing a DLP hybrid job.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations dlp jobs finish projects](ProjectLocationDlpJobFinishCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2FinishDlpJobRequest { _never_set: Option<bool> }

impl client::RequestValue for GooglePrivacyDlpV2FinishDlpJobRequest {}


/// Buckets values based on fixed size ranges. The Bucketing transformation can provide all of this functionality, but requires more configuration. This message is provided as a convenience to the user for simple bucketing strategies. The transformed value will be a hyphenated string of {lower_bound}-{upper_bound}. For example, if lower_bound = 10 and upper_bound = 20, all values that are within this bucket will be replaced with "10-20". This can be used on data of type: double, long. If the bound Value type differs from the type of data being transformed, we will first attempt converting the type of the data to be transformed to match the type of the bound before comparing. See https://cloud.google.com/dlp/docs/concepts-bucketing to learn more.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2FixedSizeBucketingConfig {
    /// Required. Size of each bucket (except for minimum and maximum buckets). So if `lower_bound` = 10, `upper_bound` = 89, and `bucket_size` = 10, then the following buckets would be used: -10, 10-20, 20-30, 30-40, 40-50, 50-60, 60-70, 70-80, 80-89, 89+. Precision up to 2 decimals works.
    #[serde(rename="bucketSize")]
    
    pub bucket_size: Option<f64>,
    /// Required. Lower bound value of buckets. All values less than `lower_bound` are grouped together into a single bucket; for example if `lower_bound` = 10, then all values less than 10 are replaced with the value "-10".
    #[serde(rename="lowerBound")]
    
    pub lower_bound: Option<GooglePrivacyDlpV2Value>,
    /// Required. Upper bound value of buckets. All values greater than upper_bound are grouped together into a single bucket; for example if `upper_bound` = 89, then all values greater than 89 are replaced with the value "89+".
    #[serde(rename="upperBound")]
    
    pub upper_bound: Option<GooglePrivacyDlpV2Value>,
}

impl client::Part for GooglePrivacyDlpV2FixedSizeBucketingConfig {}


/// The rule that adjusts the likelihood of findings within a certain proximity of hotwords.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2HotwordRule {
    /// Regular expression pattern defining what qualifies as a hotword.
    #[serde(rename="hotwordRegex")]
    
    pub hotword_regex: Option<GooglePrivacyDlpV2Regex>,
    /// Likelihood adjustment to apply to all matching findings.
    #[serde(rename="likelihoodAdjustment")]
    
    pub likelihood_adjustment: Option<GooglePrivacyDlpV2LikelihoodAdjustment>,
    /// Range of characters within which the entire hotword must reside. The total length of the window cannot exceed 1000 characters. The finding itself will be included in the window, so that hotwords can be used to match substrings of the finding itself. Suppose you want Cloud DLP to promote the likelihood of the phone number regex "\(\d{3}\) \d{3}-\d{4}" if the area code is known to be the area code of a company's office. In this case, use the hotword regex "\(xxx\)", where "xxx" is the area code in question. For tabular data, if you want to modify the likelihood of an entire column of findngs, see [Hotword example: Set the match likelihood of a table column] (https://cloud.google.com/dlp/docs/creating-custom-infotypes-likelihood#match-column-values).
    
    pub proximity: Option<GooglePrivacyDlpV2Proximity>,
}

impl client::Part for GooglePrivacyDlpV2HotwordRule {}


/// An individual hybrid item to inspect. Will be stored temporarily during processing.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2HybridContentItem {
    /// Supplementary information that will be added to each finding.
    #[serde(rename="findingDetails")]
    
    pub finding_details: Option<GooglePrivacyDlpV2HybridFindingDetails>,
    /// The item to inspect.
    
    pub item: Option<GooglePrivacyDlpV2ContentItem>,
}

impl client::Part for GooglePrivacyDlpV2HybridContentItem {}


/// Populate to associate additional data with each finding.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2HybridFindingDetails {
    /// Details about the container where the content being inspected is from.
    #[serde(rename="containerDetails")]
    
    pub container_details: Option<GooglePrivacyDlpV2Container>,
    /// Offset in bytes of the line, from the beginning of the file, where the finding is located. Populate if the item being scanned is only part of a bigger item, such as a shard of a file and you want to track the absolute position of the finding.
    #[serde(rename="fileOffset")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub file_offset: Option<i64>,
    /// Labels to represent user provided metadata about the data being inspected. If configured by the job, some key values may be required. The labels associated with `Finding`’s produced by hybrid inspection. Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?`. Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`. No more than 10 labels can be associated with a given finding. Examples: * `"environment" : "production"` * `"pipeline" : "etl"`
    
    pub labels: Option<HashMap<String, String>>,
    /// Offset of the row for tables. Populate if the row(s) being scanned are part of a bigger dataset and you want to keep track of their absolute position.
    #[serde(rename="rowOffset")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub row_offset: Option<i64>,
    /// If the container is a table, additional information to make findings meaningful such as the columns that are primary keys. If not known ahead of time, can also be set within each inspect hybrid call and the two will be merged. Note that identifying_fields will only be stored to BigQuery, and only if the BigQuery action has been included.
    #[serde(rename="tableOptions")]
    
    pub table_options: Option<GooglePrivacyDlpV2TableOptions>,
}

impl client::Part for GooglePrivacyDlpV2HybridFindingDetails {}


/// Request to search for potentially sensitive info in a custom location.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations dlp jobs hybrid inspect projects](ProjectLocationDlpJobHybridInspectCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2HybridInspectDlpJobRequest {
    /// The item to inspect.
    #[serde(rename="hybridItem")]
    
    pub hybrid_item: Option<GooglePrivacyDlpV2HybridContentItem>,
}

impl client::RequestValue for GooglePrivacyDlpV2HybridInspectDlpJobRequest {}


/// Request to search for potentially sensitive info in a custom location.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations job triggers hybrid inspect projects](ProjectLocationJobTriggerHybridInspectCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2HybridInspectJobTriggerRequest {
    /// The item to inspect.
    #[serde(rename="hybridItem")]
    
    pub hybrid_item: Option<GooglePrivacyDlpV2HybridContentItem>,
}

impl client::RequestValue for GooglePrivacyDlpV2HybridInspectJobTriggerRequest {}


/// Quota exceeded errors will be thrown once quota has been met.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations dlp jobs hybrid inspect projects](ProjectLocationDlpJobHybridInspectCall) (response)
/// * [locations job triggers hybrid inspect projects](ProjectLocationJobTriggerHybridInspectCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2HybridInspectResponse { _never_set: Option<bool> }

impl client::ResponseResult for GooglePrivacyDlpV2HybridInspectResponse {}


/// Statistics related to processing hybrid inspect requests.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2HybridInspectStatistics {
    /// The number of hybrid inspection requests aborted because the job ran out of quota or was ended before they could be processed.
    #[serde(rename="abortedCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub aborted_count: Option<i64>,
    /// The number of hybrid requests currently being processed. Only populated when called via method `getDlpJob`. A burst of traffic may cause hybrid inspect requests to be enqueued. Processing will take place as quickly as possible, but resource limitations may impact how long a request is enqueued for.
    #[serde(rename="pendingCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub pending_count: Option<i64>,
    /// The number of hybrid inspection requests processed within this job.
    #[serde(rename="processedCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub processed_count: Option<i64>,
}

impl client::Part for GooglePrivacyDlpV2HybridInspectStatistics {}


/// Configuration to control jobs where the content being inspected is outside of Google Cloud Platform.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2HybridOptions {
    /// A short description of where the data is coming from. Will be stored once in the job. 256 max length.
    
    pub description: Option<String>,
    /// To organize findings, these labels will be added to each finding. Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?`. Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`. No more than 10 labels can be associated with a given finding. Examples: * `"environment" : "production"` * `"pipeline" : "etl"`
    
    pub labels: Option<HashMap<String, String>>,
    /// These are labels that each inspection request must include within their ‘finding_labels’ map. Request may contain others, but any missing one of these will be rejected. Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?`. No more than 10 keys can be required.
    #[serde(rename="requiredFindingLabelKeys")]
    
    pub required_finding_label_keys: Option<Vec<String>>,
    /// If the container is a table, additional information to make findings meaningful such as the columns that are primary keys.
    #[serde(rename="tableOptions")]
    
    pub table_options: Option<GooglePrivacyDlpV2TableOptions>,
}

impl client::Part for GooglePrivacyDlpV2HybridOptions {}


/// Location of the finding within an image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2ImageLocation {
    /// Bounding boxes locating the pixels within the image containing the finding.
    #[serde(rename="boundingBoxes")]
    
    pub bounding_boxes: Option<Vec<GooglePrivacyDlpV2BoundingBox>>,
}

impl client::Part for GooglePrivacyDlpV2ImageLocation {}


/// Configuration for determining how redaction of images should occur.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2ImageRedactionConfig {
    /// Only one per info_type should be provided per request. If not specified, and redact_all_text is false, the DLP API will redact all text that it matches against all info_types that are found, but not specified in another ImageRedactionConfig.
    #[serde(rename="infoType")]
    
    pub info_type: Option<GooglePrivacyDlpV2InfoType>,
    /// If true, all text found in the image, regardless whether it matches an info_type, is redacted. Only one should be provided.
    #[serde(rename="redactAllText")]
    
    pub redact_all_text: Option<bool>,
    /// The color to use when redacting content from an image. If not specified, the default is black.
    #[serde(rename="redactionColor")]
    
    pub redaction_color: Option<GooglePrivacyDlpV2Color>,
}

impl client::Part for GooglePrivacyDlpV2ImageRedactionConfig {}


/// Configuration for determining how redaction of images should occur.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2ImageTransformation {
    /// Apply transformation to all findings not specified in other ImageTransformation's selected_info_types. Only one instance is allowed within the ImageTransformations message.
    #[serde(rename="allInfoTypes")]
    
    pub all_info_types: Option<GooglePrivacyDlpV2AllInfoTypes>,
    /// Apply transformation to all text that doesn't match an infoType. Only one instance is allowed within the ImageTransformations message.
    #[serde(rename="allText")]
    
    pub all_text: Option<GooglePrivacyDlpV2AllText>,
    /// The color to use when redacting content from an image. If not specified, the default is black.
    #[serde(rename="redactionColor")]
    
    pub redaction_color: Option<GooglePrivacyDlpV2Color>,
    /// Apply transformation to the selected info_types.
    #[serde(rename="selectedInfoTypes")]
    
    pub selected_info_types: Option<GooglePrivacyDlpV2SelectedInfoTypes>,
}

impl client::Part for GooglePrivacyDlpV2ImageTransformation {}


/// A type of transformation that is applied over images.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2ImageTransformations {
    /// no description provided
    
    pub transforms: Option<Vec<GooglePrivacyDlpV2ImageTransformation>>,
}

impl client::Part for GooglePrivacyDlpV2ImageTransformations {}


/// Type of information detected by the API.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2InfoType {
    /// Name of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed at https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type. When sending Cloud DLP results to Data Catalog, infoType names should conform to the pattern `[A-Za-z0-9$_-]{1,64}`.
    
    pub name: Option<String>,
    /// Optional version name for this InfoType.
    
    pub version: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2InfoType {}


/// Classification of infoTypes to organize them according to geographic location, industry, and data type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2InfoTypeCategory {
    /// The group of relevant businesses where this infoType is commonly used
    #[serde(rename="industryCategory")]
    
    pub industry_category: Option<GooglePrivacyDlpV2InfoTypeCategoryIndustryCategoryEnum>,
    /// The region or country that issued the ID or document represented by the infoType.
    #[serde(rename="locationCategory")]
    
    pub location_category: Option<GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum>,
    /// The class of identifiers where this infoType belongs
    #[serde(rename="typeCategory")]
    
    pub type_category: Option<GooglePrivacyDlpV2InfoTypeCategoryTypeCategoryEnum>,
}

impl client::Part for GooglePrivacyDlpV2InfoTypeCategory {}


/// InfoType description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2InfoTypeDescription {
    /// The category of the infoType.
    
    pub categories: Option<Vec<GooglePrivacyDlpV2InfoTypeCategory>>,
    /// Description of the infotype. Translated when language is provided in the request.
    
    pub description: Option<String>,
    /// Human readable form of the infoType name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Internal name of the infoType.
    
    pub name: Option<String>,
    /// The default sensitivity of the infoType.
    #[serde(rename="sensitivityScore")]
    
    pub sensitivity_score: Option<GooglePrivacyDlpV2SensitivityScore>,
    /// Which parts of the API supports this InfoType.
    #[serde(rename="supportedBy")]
    
    pub supported_by: Option<Vec<GooglePrivacyDlpV2InfoTypeDescriptionSupportedByEnum>>,
    /// A list of available versions for the infotype.
    
    pub versions: Option<Vec<GooglePrivacyDlpV2VersionDescription>>,
}

impl client::Part for GooglePrivacyDlpV2InfoTypeDescription {}


/// Max findings configuration per infoType, per content item or long running DlpJob.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2InfoTypeLimit {
    /// Type of information the findings limit applies to. Only one limit per info_type should be provided. If InfoTypeLimit does not have an info_type, the DLP API applies the limit against all info_types that are found but not specified in another InfoTypeLimit.
    #[serde(rename="infoType")]
    
    pub info_type: Option<GooglePrivacyDlpV2InfoType>,
    /// Max findings limit for the given infoType.
    #[serde(rename="maxFindings")]
    
    pub max_findings: Option<i32>,
}

impl client::Part for GooglePrivacyDlpV2InfoTypeLimit {}


/// Statistics regarding a specific InfoType.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2InfoTypeStats {
    /// Number of findings for this infoType.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
    /// The type of finding this stat is for.
    #[serde(rename="infoType")]
    
    pub info_type: Option<GooglePrivacyDlpV2InfoType>,
}

impl client::Part for GooglePrivacyDlpV2InfoTypeStats {}


/// A transformation to apply to text that is identified as a specific info_type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2InfoTypeTransformation {
    /// InfoTypes to apply the transformation to. An empty list will cause this transformation to apply to all findings that correspond to infoTypes that were requested in `InspectConfig`.
    #[serde(rename="infoTypes")]
    
    pub info_types: Option<Vec<GooglePrivacyDlpV2InfoType>>,
    /// Required. Primitive transformation to apply to the infoType.
    #[serde(rename="primitiveTransformation")]
    
    pub primitive_transformation: Option<GooglePrivacyDlpV2PrimitiveTransformation>,
}

impl client::Part for GooglePrivacyDlpV2InfoTypeTransformation {}


/// A type of transformation that will scan unstructured text and apply various `PrimitiveTransformation`s to each finding, where the transformation is applied to only values that were identified as a specific info_type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2InfoTypeTransformations {
    /// Required. Transformation for each infoType. Cannot specify more than one for a given infoType.
    
    pub transformations: Option<Vec<GooglePrivacyDlpV2InfoTypeTransformation>>,
}

impl client::Part for GooglePrivacyDlpV2InfoTypeTransformations {}


/// Configuration description of the scanning process. When used with redactContent only info_types and min_likelihood are currently used.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2InspectConfig {
    /// Deprecated and unused.
    #[serde(rename="contentOptions")]
    
    pub content_options: Option<Vec<GooglePrivacyDlpV2InspectConfigContentOptionsEnum>>,
    /// CustomInfoTypes provided by the user. See https://cloud.google.com/dlp/docs/creating-custom-infotypes to learn more.
    #[serde(rename="customInfoTypes")]
    
    pub custom_info_types: Option<Vec<GooglePrivacyDlpV2CustomInfoType>>,
    /// When true, excludes type information of the findings. This is not used for data profiling.
    #[serde(rename="excludeInfoTypes")]
    
    pub exclude_info_types: Option<bool>,
    /// When true, a contextual quote from the data that triggered a finding is included in the response; see Finding.quote. This is not used for data profiling.
    #[serde(rename="includeQuote")]
    
    pub include_quote: Option<bool>,
    /// Restricts what info_types to look for. The values must correspond to InfoType values returned by ListInfoTypes or listed at https://cloud.google.com/dlp/docs/infotypes-reference. When no InfoTypes or CustomInfoTypes are specified in a request, the system may automatically choose what detectors to run. By default this may be all types, but may change over time as detectors are updated. If you need precise control and predictability as to what detectors are run you should specify specific InfoTypes listed in the reference, otherwise a default list will be used, which may change over time.
    #[serde(rename="infoTypes")]
    
    pub info_types: Option<Vec<GooglePrivacyDlpV2InfoType>>,
    /// Configuration to control the number of findings returned. This is not used for data profiling. When redacting sensitive data from images, finding limits don't apply. They can cause unexpected or inconsistent results, where only some data is redacted. Don't include finding limits in RedactImage requests. Otherwise, Cloud DLP returns an error.
    
    pub limits: Option<GooglePrivacyDlpV2FindingLimits>,
    /// Only returns findings equal or above this threshold. The default is POSSIBLE. See https://cloud.google.com/dlp/docs/likelihood to learn more.
    #[serde(rename="minLikelihood")]
    
    pub min_likelihood: Option<GooglePrivacyDlpV2InspectConfigMinLikelihoodEnum>,
    /// Set of rules to apply to the findings for this InspectConfig. Exclusion rules, contained in the set are executed in the end, other rules are executed in the order they are specified for each info type.
    #[serde(rename="ruleSet")]
    
    pub rule_set: Option<Vec<GooglePrivacyDlpV2InspectionRuleSet>>,
}

impl client::Part for GooglePrivacyDlpV2InspectConfig {}


/// Request to search for potentially sensitive info in a ContentItem.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [content inspect projects](ProjectContentInspectCall) (request)
/// * [locations content inspect projects](ProjectLocationContentInspectCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2InspectContentRequest {
    /// Configuration for the inspector. What specified here will override the template referenced by the inspect_template_name argument.
    #[serde(rename="inspectConfig")]
    
    pub inspect_config: Option<GooglePrivacyDlpV2InspectConfig>,
    /// Template to use. Any configuration directly specified in inspect_config will override those set in the template. Singular fields that are set in this request will replace their corresponding fields in the template. Repeated fields are appended. Singular sub-messages and groups are recursively merged.
    #[serde(rename="inspectTemplateName")]
    
    pub inspect_template_name: Option<String>,
    /// The item to inspect.
    
    pub item: Option<GooglePrivacyDlpV2ContentItem>,
    /// Deprecated. This field has no effect.
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
}

impl client::RequestValue for GooglePrivacyDlpV2InspectContentRequest {}


/// Results of inspecting an item.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [content inspect projects](ProjectContentInspectCall) (response)
/// * [locations content inspect projects](ProjectLocationContentInspectCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2InspectContentResponse {
    /// The findings.
    
    pub result: Option<GooglePrivacyDlpV2InspectResult>,
}

impl client::ResponseResult for GooglePrivacyDlpV2InspectContentResponse {}


/// The results of an inspect DataSource job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2InspectDataSourceDetails {
    /// The configuration used for this job.
    #[serde(rename="requestedOptions")]
    
    pub requested_options: Option<GooglePrivacyDlpV2RequestedOptions>,
    /// A summary of the outcome of this inspection job.
    
    pub result: Option<GooglePrivacyDlpV2Result>,
}

impl client::Part for GooglePrivacyDlpV2InspectDataSourceDetails {}


/// Controls what and how to inspect for findings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2InspectJobConfig {
    /// Actions to execute at the completion of the job.
    
    pub actions: Option<Vec<GooglePrivacyDlpV2Action>>,
    /// How and what to scan for.
    #[serde(rename="inspectConfig")]
    
    pub inspect_config: Option<GooglePrivacyDlpV2InspectConfig>,
    /// If provided, will be used as the default for all values in InspectConfig. `inspect_config` will be merged into the values persisted as part of the template.
    #[serde(rename="inspectTemplateName")]
    
    pub inspect_template_name: Option<String>,
    /// The data to scan.
    #[serde(rename="storageConfig")]
    
    pub storage_config: Option<GooglePrivacyDlpV2StorageConfig>,
}

impl client::Part for GooglePrivacyDlpV2InspectJobConfig {}


/// All the findings for a single scanned item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2InspectResult {
    /// List of findings for an item.
    
    pub findings: Option<Vec<GooglePrivacyDlpV2Finding>>,
    /// If true, then this item might have more findings than were returned, and the findings returned are an arbitrary subset of all findings. The findings list might be truncated because the input items were too large, or because the server reached the maximum amount of resources allowed for a single API call. For best results, divide the input into smaller batches.
    #[serde(rename="findingsTruncated")]
    
    pub findings_truncated: Option<bool>,
}

impl client::Part for GooglePrivacyDlpV2InspectResult {}


/// The inspectTemplate contains a configuration (set of types of sensitive data to be detected) to be used anywhere you otherwise would normally specify InspectConfig. See https://cloud.google.com/dlp/docs/concepts-templates to learn more.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [inspect templates create organizations](OrganizationInspectTemplateCreateCall) (response)
/// * [inspect templates get organizations](OrganizationInspectTemplateGetCall) (response)
/// * [inspect templates patch organizations](OrganizationInspectTemplatePatchCall) (response)
/// * [locations inspect templates create organizations](OrganizationLocationInspectTemplateCreateCall) (response)
/// * [locations inspect templates get organizations](OrganizationLocationInspectTemplateGetCall) (response)
/// * [locations inspect templates patch organizations](OrganizationLocationInspectTemplatePatchCall) (response)
/// * [inspect templates create projects](ProjectInspectTemplateCreateCall) (response)
/// * [inspect templates get projects](ProjectInspectTemplateGetCall) (response)
/// * [inspect templates patch projects](ProjectInspectTemplatePatchCall) (response)
/// * [locations inspect templates create projects](ProjectLocationInspectTemplateCreateCall) (response)
/// * [locations inspect templates get projects](ProjectLocationInspectTemplateGetCall) (response)
/// * [locations inspect templates patch projects](ProjectLocationInspectTemplatePatchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2InspectTemplate {
    /// Output only. The creation timestamp of an inspectTemplate.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Short description (max 256 chars).
    
    pub description: Option<String>,
    /// Display name (max 256 chars).
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The core content of the template. Configuration of the scanning process.
    #[serde(rename="inspectConfig")]
    
    pub inspect_config: Option<GooglePrivacyDlpV2InspectConfig>,
    /// Output only. The template name. The template will have one of the following formats: `projects/PROJECT_ID/inspectTemplates/TEMPLATE_ID` OR `organizations/ORGANIZATION_ID/inspectTemplates/TEMPLATE_ID`;
    
    pub name: Option<String>,
    /// Output only. The last update timestamp of an inspectTemplate.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for GooglePrivacyDlpV2InspectTemplate {}


/// A single inspection rule to be applied to infoTypes, specified in `InspectionRuleSet`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2InspectionRule {
    /// Exclusion rule.
    #[serde(rename="exclusionRule")]
    
    pub exclusion_rule: Option<GooglePrivacyDlpV2ExclusionRule>,
    /// Hotword-based detection rule.
    #[serde(rename="hotwordRule")]
    
    pub hotword_rule: Option<GooglePrivacyDlpV2HotwordRule>,
}

impl client::Part for GooglePrivacyDlpV2InspectionRule {}


/// Rule set for modifying a set of infoTypes to alter behavior under certain circumstances, depending on the specific details of the rules within the set.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2InspectionRuleSet {
    /// List of infoTypes this rule set is applied to.
    #[serde(rename="infoTypes")]
    
    pub info_types: Option<Vec<GooglePrivacyDlpV2InfoType>>,
    /// Set of rules to be applied to infoTypes. The rules are applied in order.
    
    pub rules: Option<Vec<GooglePrivacyDlpV2InspectionRule>>,
}

impl client::Part for GooglePrivacyDlpV2InspectionRuleSet {}


/// Sends an email when the job completes. The email goes to IAM project owners and technical [Essential Contacts](https://cloud.google.com/resource-manager/docs/managing-notification-contacts).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2JobNotificationEmails { _never_set: Option<bool> }

impl client::Part for GooglePrivacyDlpV2JobNotificationEmails {}


/// Contains a configuration to make dlp api calls on a repeating basis. See https://cloud.google.com/dlp/docs/concepts-job-triggers to learn more.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations job triggers create organizations](OrganizationLocationJobTriggerCreateCall) (response)
/// * [locations job triggers get organizations](OrganizationLocationJobTriggerGetCall) (response)
/// * [locations job triggers patch organizations](OrganizationLocationJobTriggerPatchCall) (response)
/// * [job triggers create projects](ProjectJobTriggerCreateCall) (response)
/// * [job triggers get projects](ProjectJobTriggerGetCall) (response)
/// * [job triggers patch projects](ProjectJobTriggerPatchCall) (response)
/// * [locations job triggers create projects](ProjectLocationJobTriggerCreateCall) (response)
/// * [locations job triggers get projects](ProjectLocationJobTriggerGetCall) (response)
/// * [locations job triggers patch projects](ProjectLocationJobTriggerPatchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2JobTrigger {
    /// Output only. The creation timestamp of a triggeredJob.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// User provided description (max 256 chars)
    
    pub description: Option<String>,
    /// Display name (max 100 chars)
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. A stream of errors encountered when the trigger was activated. Repeated errors may result in the JobTrigger automatically being paused. Will return the last 100 errors. Whenever the JobTrigger is modified this list will be cleared.
    
    pub errors: Option<Vec<GooglePrivacyDlpV2Error>>,
    /// For inspect jobs, a snapshot of the configuration.
    #[serde(rename="inspectJob")]
    
    pub inspect_job: Option<GooglePrivacyDlpV2InspectJobConfig>,
    /// Output only. The timestamp of the last time this trigger executed.
    #[serde(rename="lastRunTime")]
    
    pub last_run_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Unique resource name for the triggeredJob, assigned by the service when the triggeredJob is created, for example `projects/dlp-test-project/jobTriggers/53234423`.
    
    pub name: Option<String>,
    /// Required. A status for this trigger.
    
    pub status: Option<GooglePrivacyDlpV2JobTriggerStatusEnum>,
    /// A list of triggers which will be OR'ed together. Only one in the list needs to trigger for a job to be started. The list may contain only a single Schedule trigger and must have at least one object.
    
    pub triggers: Option<Vec<GooglePrivacyDlpV2Trigger>>,
    /// Output only. The last update timestamp of a triggeredJob.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for GooglePrivacyDlpV2JobTrigger {}


/// k-anonymity metric, used for analysis of reidentification risk.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2KAnonymityConfig {
    /// Message indicating that multiple rows might be associated to a single individual. If the same entity_id is associated to multiple quasi-identifier tuples over distinct rows, we consider the entire collection of tuples as the composite quasi-identifier. This collection is a multiset: the order in which the different tuples appear in the dataset is ignored, but their frequency is taken into account. Important note: a maximum of 1000 rows can be associated to a single entity ID. If more rows are associated with the same entity ID, some might be ignored.
    #[serde(rename="entityId")]
    
    pub entity_id: Option<GooglePrivacyDlpV2EntityId>,
    /// Set of fields to compute k-anonymity over. When multiple fields are specified, they are considered a single composite key. Structs and repeated data types are not supported; however, nested fields are supported so long as they are not structs themselves or nested within a repeated field.
    #[serde(rename="quasiIds")]
    
    pub quasi_ids: Option<Vec<GooglePrivacyDlpV2FieldId>>,
}

impl client::Part for GooglePrivacyDlpV2KAnonymityConfig {}


/// The set of columns' values that share the same ldiversity value
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2KAnonymityEquivalenceClass {
    /// Size of the equivalence class, for example number of rows with the above set of values.
    #[serde(rename="equivalenceClassSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub equivalence_class_size: Option<i64>,
    /// Set of values defining the equivalence class. One value per quasi-identifier column in the original KAnonymity metric message. The order is always the same as the original request.
    #[serde(rename="quasiIdsValues")]
    
    pub quasi_ids_values: Option<Vec<GooglePrivacyDlpV2Value>>,
}

impl client::Part for GooglePrivacyDlpV2KAnonymityEquivalenceClass {}


/// Histogram of k-anonymity equivalence classes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2KAnonymityHistogramBucket {
    /// Total number of equivalence classes in this bucket.
    #[serde(rename="bucketSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bucket_size: Option<i64>,
    /// Total number of distinct equivalence classes in this bucket.
    #[serde(rename="bucketValueCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bucket_value_count: Option<i64>,
    /// Sample of equivalence classes in this bucket. The total number of classes returned per bucket is capped at 20.
    #[serde(rename="bucketValues")]
    
    pub bucket_values: Option<Vec<GooglePrivacyDlpV2KAnonymityEquivalenceClass>>,
    /// Lower bound on the size of the equivalence classes in this bucket.
    #[serde(rename="equivalenceClassSizeLowerBound")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub equivalence_class_size_lower_bound: Option<i64>,
    /// Upper bound on the size of the equivalence classes in this bucket.
    #[serde(rename="equivalenceClassSizeUpperBound")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub equivalence_class_size_upper_bound: Option<i64>,
}

impl client::Part for GooglePrivacyDlpV2KAnonymityHistogramBucket {}


/// Result of the k-anonymity computation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2KAnonymityResult {
    /// Histogram of k-anonymity equivalence classes.
    #[serde(rename="equivalenceClassHistogramBuckets")]
    
    pub equivalence_class_histogram_buckets: Option<Vec<GooglePrivacyDlpV2KAnonymityHistogramBucket>>,
}

impl client::Part for GooglePrivacyDlpV2KAnonymityResult {}


/// Reidentifiability metric. This corresponds to a risk model similar to what is called "journalist risk" in the literature, except the attack dataset is statistically modeled instead of being perfectly known. This can be done using publicly available data (like the US Census), or using a custom statistical model (indicated as one or several BigQuery tables), or by extrapolating from the distribution of values in the input dataset.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2KMapEstimationConfig {
    /// Several auxiliary tables can be used in the analysis. Each custom_tag used to tag a quasi-identifiers column must appear in exactly one column of one auxiliary table.
    #[serde(rename="auxiliaryTables")]
    
    pub auxiliary_tables: Option<Vec<GooglePrivacyDlpV2AuxiliaryTable>>,
    /// Required. Fields considered to be quasi-identifiers. No two columns can have the same tag.
    #[serde(rename="quasiIds")]
    
    pub quasi_ids: Option<Vec<GooglePrivacyDlpV2TaggedField>>,
    /// ISO 3166-1 alpha-2 region code to use in the statistical modeling. Set if no column is tagged with a region-specific InfoType (like US_ZIP_5) or a region code.
    #[serde(rename="regionCode")]
    
    pub region_code: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2KMapEstimationConfig {}


/// A KMapEstimationHistogramBucket message with the following values: min_anonymity: 3 max_anonymity: 5 frequency: 42 means that there are 42 records whose quasi-identifier values correspond to 3, 4 or 5 people in the overlying population. An important particular case is when min_anonymity = max_anonymity = 1: the frequency field then corresponds to the number of uniquely identifiable records.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2KMapEstimationHistogramBucket {
    /// Number of records within these anonymity bounds.
    #[serde(rename="bucketSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bucket_size: Option<i64>,
    /// Total number of distinct quasi-identifier tuple values in this bucket.
    #[serde(rename="bucketValueCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bucket_value_count: Option<i64>,
    /// Sample of quasi-identifier tuple values in this bucket. The total number of classes returned per bucket is capped at 20.
    #[serde(rename="bucketValues")]
    
    pub bucket_values: Option<Vec<GooglePrivacyDlpV2KMapEstimationQuasiIdValues>>,
    /// Always greater than or equal to min_anonymity.
    #[serde(rename="maxAnonymity")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_anonymity: Option<i64>,
    /// Always positive.
    #[serde(rename="minAnonymity")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub min_anonymity: Option<i64>,
}

impl client::Part for GooglePrivacyDlpV2KMapEstimationHistogramBucket {}


/// A tuple of values for the quasi-identifier columns.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2KMapEstimationQuasiIdValues {
    /// The estimated anonymity for these quasi-identifier values.
    #[serde(rename="estimatedAnonymity")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub estimated_anonymity: Option<i64>,
    /// The quasi-identifier values.
    #[serde(rename="quasiIdsValues")]
    
    pub quasi_ids_values: Option<Vec<GooglePrivacyDlpV2Value>>,
}

impl client::Part for GooglePrivacyDlpV2KMapEstimationQuasiIdValues {}


/// Result of the reidentifiability analysis. Note that these results are an estimation, not exact values.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2KMapEstimationResult {
    /// The intervals [min_anonymity, max_anonymity] do not overlap. If a value doesn't correspond to any such interval, the associated frequency is zero. For example, the following records: {min_anonymity: 1, max_anonymity: 1, frequency: 17} {min_anonymity: 2, max_anonymity: 3, frequency: 42} {min_anonymity: 5, max_anonymity: 10, frequency: 99} mean that there are no record with an estimated anonymity of 4, 5, or larger than 10.
    #[serde(rename="kMapEstimationHistogram")]
    
    pub k_map_estimation_histogram: Option<Vec<GooglePrivacyDlpV2KMapEstimationHistogramBucket>>,
}

impl client::Part for GooglePrivacyDlpV2KMapEstimationResult {}


/// A unique identifier for a Datastore entity. If a key's partition ID or any of its path kinds or names are reserved/read-only, the key is reserved/read-only. A reserved/read-only key is forbidden in certain documented contexts.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2Key {
    /// Entities are partitioned into subsets, currently identified by a project ID and namespace ID. Queries are scoped to a single partition.
    #[serde(rename="partitionId")]
    
    pub partition_id: Option<GooglePrivacyDlpV2PartitionId>,
    /// The entity path. An entity path consists of one or more elements composed of a kind and a string or numerical identifier, which identify entities. The first element identifies a _root entity_, the second element identifies a _child_ of the root entity, the third element identifies a child of the second entity, and so forth. The entities identified by all prefixes of the path are called the element's _ancestors_. A path can never be empty, and a path can have at most 100 elements.
    
    pub path: Option<Vec<GooglePrivacyDlpV2PathElement>>,
}

impl client::Part for GooglePrivacyDlpV2Key {}


/// A representation of a Datastore kind.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2KindExpression {
    /// The name of the kind.
    
    pub name: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2KindExpression {}


/// Include to use an existing data crypto key wrapped by KMS. The wrapped key must be a 128-, 192-, or 256-bit key. Authorization requires the following IAM permissions when sending a request to perform a crypto transformation using a KMS-wrapped crypto key: dlp.kms.encrypt For more information, see [Creating a wrapped key] (https://cloud.google.com/dlp/docs/create-wrapped-key). Note: When you use Cloud KMS for cryptographic operations, [charges apply](https://cloud.google.com/kms/pricing).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2KmsWrappedCryptoKey {
    /// Required. The resource name of the KMS CryptoKey to use for unwrapping.
    #[serde(rename="cryptoKeyName")]
    
    pub crypto_key_name: Option<String>,
    /// Required. The wrapped data crypto key.
    #[serde(rename="wrappedKey")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub wrapped_key: Option<Vec<u8>>,
}

impl client::Part for GooglePrivacyDlpV2KmsWrappedCryptoKey {}


/// l-diversity metric, used for analysis of reidentification risk.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2LDiversityConfig {
    /// Set of quasi-identifiers indicating how equivalence classes are defined for the l-diversity computation. When multiple fields are specified, they are considered a single composite key.
    #[serde(rename="quasiIds")]
    
    pub quasi_ids: Option<Vec<GooglePrivacyDlpV2FieldId>>,
    /// Sensitive field for computing the l-value.
    #[serde(rename="sensitiveAttribute")]
    
    pub sensitive_attribute: Option<GooglePrivacyDlpV2FieldId>,
}

impl client::Part for GooglePrivacyDlpV2LDiversityConfig {}


/// The set of columns' values that share the same ldiversity value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2LDiversityEquivalenceClass {
    /// Size of the k-anonymity equivalence class.
    #[serde(rename="equivalenceClassSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub equivalence_class_size: Option<i64>,
    /// Number of distinct sensitive values in this equivalence class.
    #[serde(rename="numDistinctSensitiveValues")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub num_distinct_sensitive_values: Option<i64>,
    /// Quasi-identifier values defining the k-anonymity equivalence class. The order is always the same as the original request.
    #[serde(rename="quasiIdsValues")]
    
    pub quasi_ids_values: Option<Vec<GooglePrivacyDlpV2Value>>,
    /// Estimated frequencies of top sensitive values.
    #[serde(rename="topSensitiveValues")]
    
    pub top_sensitive_values: Option<Vec<GooglePrivacyDlpV2ValueFrequency>>,
}

impl client::Part for GooglePrivacyDlpV2LDiversityEquivalenceClass {}


/// Histogram of l-diversity equivalence class sensitive value frequencies.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2LDiversityHistogramBucket {
    /// Total number of equivalence classes in this bucket.
    #[serde(rename="bucketSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bucket_size: Option<i64>,
    /// Total number of distinct equivalence classes in this bucket.
    #[serde(rename="bucketValueCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bucket_value_count: Option<i64>,
    /// Sample of equivalence classes in this bucket. The total number of classes returned per bucket is capped at 20.
    #[serde(rename="bucketValues")]
    
    pub bucket_values: Option<Vec<GooglePrivacyDlpV2LDiversityEquivalenceClass>>,
    /// Lower bound on the sensitive value frequencies of the equivalence classes in this bucket.
    #[serde(rename="sensitiveValueFrequencyLowerBound")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub sensitive_value_frequency_lower_bound: Option<i64>,
    /// Upper bound on the sensitive value frequencies of the equivalence classes in this bucket.
    #[serde(rename="sensitiveValueFrequencyUpperBound")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub sensitive_value_frequency_upper_bound: Option<i64>,
}

impl client::Part for GooglePrivacyDlpV2LDiversityHistogramBucket {}


/// Result of the l-diversity computation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2LDiversityResult {
    /// Histogram of l-diversity equivalence class sensitive value frequencies.
    #[serde(rename="sensitiveValueFrequencyHistogramBuckets")]
    
    pub sensitive_value_frequency_histogram_buckets: Option<Vec<GooglePrivacyDlpV2LDiversityHistogramBucket>>,
}

impl client::Part for GooglePrivacyDlpV2LDiversityResult {}


/// Configuration for a custom dictionary created from a data source of any size up to the maximum size defined in the [limits](https://cloud.google.com/dlp/limits) page. The artifacts of dictionary creation are stored in the specified Cloud Storage location. Consider using `CustomInfoType.Dictionary` for smaller dictionaries that satisfy the size requirements.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2LargeCustomDictionaryConfig {
    /// Field in a BigQuery table where each cell represents a dictionary phrase.
    #[serde(rename="bigQueryField")]
    
    pub big_query_field: Option<GooglePrivacyDlpV2BigQueryField>,
    /// Set of files containing newline-delimited lists of dictionary phrases.
    #[serde(rename="cloudStorageFileSet")]
    
    pub cloud_storage_file_set: Option<GooglePrivacyDlpV2CloudStorageFileSet>,
    /// Location to store dictionary artifacts in Cloud Storage. These files will only be accessible by project owners and the DLP API. If any of these artifacts are modified, the dictionary is considered invalid and can no longer be used.
    #[serde(rename="outputPath")]
    
    pub output_path: Option<GooglePrivacyDlpV2CloudStoragePath>,
}

impl client::Part for GooglePrivacyDlpV2LargeCustomDictionaryConfig {}


/// Summary statistics of a custom dictionary.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2LargeCustomDictionaryStats {
    /// Approximate number of distinct phrases in the dictionary.
    #[serde(rename="approxNumPhrases")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub approx_num_phrases: Option<i64>,
}

impl client::Part for GooglePrivacyDlpV2LargeCustomDictionaryStats {}


/// Skips the data without modifying it if the requested transformation would cause an error. For example, if a `DateShift` transformation were applied an an IP address, this mode would leave the IP address unchanged in the response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2LeaveUntransformed { _never_set: Option<bool> }

impl client::Part for GooglePrivacyDlpV2LeaveUntransformed {}


/// Message for specifying an adjustment to the likelihood of a finding as part of a detection rule.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2LikelihoodAdjustment {
    /// Set the likelihood of a finding to a fixed value.
    #[serde(rename="fixedLikelihood")]
    
    pub fixed_likelihood: Option<GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihoodEnum>,
    /// Increase or decrease the likelihood by the specified number of levels. For example, if a finding would be `POSSIBLE` without the detection rule and `relative_likelihood` is 1, then it is upgraded to `LIKELY`, while a value of -1 would downgrade it to `UNLIKELY`. Likelihood may never drop below `VERY_UNLIKELY` or exceed `VERY_LIKELY`, so applying an adjustment of 1 followed by an adjustment of -1 when base likelihood is `VERY_LIKELY` will result in a final likelihood of `LIKELY`.
    #[serde(rename="relativeLikelihood")]
    
    pub relative_likelihood: Option<i32>,
}

impl client::Part for GooglePrivacyDlpV2LikelihoodAdjustment {}


/// Response message for ListDeidentifyTemplates.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [deidentify templates list organizations](OrganizationDeidentifyTemplateListCall) (response)
/// * [locations deidentify templates list organizations](OrganizationLocationDeidentifyTemplateListCall) (response)
/// * [deidentify templates list projects](ProjectDeidentifyTemplateListCall) (response)
/// * [locations deidentify templates list projects](ProjectLocationDeidentifyTemplateListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2ListDeidentifyTemplatesResponse {
    /// List of deidentify templates, up to page_size in ListDeidentifyTemplatesRequest.
    #[serde(rename="deidentifyTemplates")]
    
    pub deidentify_templates: Option<Vec<GooglePrivacyDlpV2DeidentifyTemplate>>,
    /// If the next page is available then the next page token to be used in following ListDeidentifyTemplates request.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GooglePrivacyDlpV2ListDeidentifyTemplatesResponse {}


/// The response message for listing DLP jobs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations dlp jobs list organizations](OrganizationLocationDlpJobListCall) (response)
/// * [dlp jobs list projects](ProjectDlpJobListCall) (response)
/// * [locations dlp jobs list projects](ProjectLocationDlpJobListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2ListDlpJobsResponse {
    /// A list of DlpJobs that matches the specified filter in the request.
    
    pub jobs: Option<Vec<GooglePrivacyDlpV2DlpJob>>,
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GooglePrivacyDlpV2ListDlpJobsResponse {}


/// Response to the ListInfoTypes request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list info types](InfoTypeListCall) (response)
/// * [info types list locations](LocationInfoTypeListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2ListInfoTypesResponse {
    /// Set of sensitive infoTypes.
    #[serde(rename="infoTypes")]
    
    pub info_types: Option<Vec<GooglePrivacyDlpV2InfoTypeDescription>>,
}

impl client::ResponseResult for GooglePrivacyDlpV2ListInfoTypesResponse {}


/// Response message for ListInspectTemplates.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [inspect templates list organizations](OrganizationInspectTemplateListCall) (response)
/// * [locations inspect templates list organizations](OrganizationLocationInspectTemplateListCall) (response)
/// * [inspect templates list projects](ProjectInspectTemplateListCall) (response)
/// * [locations inspect templates list projects](ProjectLocationInspectTemplateListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2ListInspectTemplatesResponse {
    /// List of inspectTemplates, up to page_size in ListInspectTemplatesRequest.
    #[serde(rename="inspectTemplates")]
    
    pub inspect_templates: Option<Vec<GooglePrivacyDlpV2InspectTemplate>>,
    /// If the next page is available then the next page token to be used in following ListInspectTemplates request.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GooglePrivacyDlpV2ListInspectTemplatesResponse {}


/// Response message for ListJobTriggers.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations job triggers list organizations](OrganizationLocationJobTriggerListCall) (response)
/// * [job triggers list projects](ProjectJobTriggerListCall) (response)
/// * [locations job triggers list projects](ProjectLocationJobTriggerListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2ListJobTriggersResponse {
    /// List of triggeredJobs, up to page_size in ListJobTriggersRequest.
    #[serde(rename="jobTriggers")]
    
    pub job_triggers: Option<Vec<GooglePrivacyDlpV2JobTrigger>>,
    /// If the next page is available then the next page token to be used in following ListJobTriggers request.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GooglePrivacyDlpV2ListJobTriggersResponse {}


/// Response message for ListStoredInfoTypes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations stored info types list organizations](OrganizationLocationStoredInfoTypeListCall) (response)
/// * [stored info types list organizations](OrganizationStoredInfoTypeListCall) (response)
/// * [locations stored info types list projects](ProjectLocationStoredInfoTypeListCall) (response)
/// * [stored info types list projects](ProjectStoredInfoTypeListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2ListStoredInfoTypesResponse {
    /// If the next page is available then the next page token to be used in following ListStoredInfoTypes request.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of storedInfoTypes, up to page_size in ListStoredInfoTypesRequest.
    #[serde(rename="storedInfoTypes")]
    
    pub stored_info_types: Option<Vec<GooglePrivacyDlpV2StoredInfoType>>,
}

impl client::ResponseResult for GooglePrivacyDlpV2ListStoredInfoTypesResponse {}


/// Specifies the location of the finding.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2Location {
    /// Zero-based byte offsets delimiting the finding. These are relative to the finding's containing element. Note that when the content is not textual, this references the UTF-8 encoded textual representation of the content. Omitted if content is an image.
    #[serde(rename="byteRange")]
    
    pub byte_range: Option<GooglePrivacyDlpV2Range>,
    /// Unicode character offsets delimiting the finding. These are relative to the finding's containing element. Provided when the content is text.
    #[serde(rename="codepointRange")]
    
    pub codepoint_range: Option<GooglePrivacyDlpV2Range>,
    /// Information about the container where this finding occurred, if available.
    
    pub container: Option<GooglePrivacyDlpV2Container>,
    /// List of nested objects pointing to the precise location of the finding within the file or record.
    #[serde(rename="contentLocations")]
    
    pub content_locations: Option<Vec<GooglePrivacyDlpV2ContentLocation>>,
}

impl client::Part for GooglePrivacyDlpV2Location {}


/// Job trigger option for hybrid jobs. Jobs must be manually created and finished.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2Manual { _never_set: Option<bool> }

impl client::Part for GooglePrivacyDlpV2Manual {}


/// Metadata Location
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2MetadataLocation {
    /// Storage metadata.
    #[serde(rename="storageLabel")]
    
    pub storage_label: Option<GooglePrivacyDlpV2StorageMetadataLabel>,
    /// Type of metadata containing the finding.
    #[serde(rename="type")]
    
    pub type_: Option<GooglePrivacyDlpV2MetadataLocationTypeEnum>,
}

impl client::Part for GooglePrivacyDlpV2MetadataLocation {}


/// Compute numerical stats over an individual column, including min, max, and quantiles.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2NumericalStatsConfig {
    /// Field to compute numerical stats on. Supported types are integer, float, date, datetime, timestamp, time.
    
    pub field: Option<GooglePrivacyDlpV2FieldId>,
}

impl client::Part for GooglePrivacyDlpV2NumericalStatsConfig {}


/// Result of the numerical stats computation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2NumericalStatsResult {
    /// Maximum value appearing in the column.
    #[serde(rename="maxValue")]
    
    pub max_value: Option<GooglePrivacyDlpV2Value>,
    /// Minimum value appearing in the column.
    #[serde(rename="minValue")]
    
    pub min_value: Option<GooglePrivacyDlpV2Value>,
    /// List of 99 values that partition the set of field values into 100 equal sized buckets.
    #[serde(rename="quantileValues")]
    
    pub quantile_values: Option<Vec<GooglePrivacyDlpV2Value>>,
}

impl client::Part for GooglePrivacyDlpV2NumericalStatsResult {}


/// Cloud repository for storing output.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2OutputStorageConfig {
    /// Schema used for writing the findings for Inspect jobs. This field is only used for Inspect and must be unspecified for Risk jobs. Columns are derived from the `Finding` object. If appending to an existing table, any columns from the predefined schema that are missing will be added. No columns in the existing table will be deleted. If unspecified, then all available columns will be used for a new table or an (existing) table with no schema, and no changes will be made to an existing table that has a schema. Only for use with external storage.
    #[serde(rename="outputSchema")]
    
    pub output_schema: Option<GooglePrivacyDlpV2OutputStorageConfigOutputSchemaEnum>,
    /// Store findings in an existing table or a new table in an existing dataset. If table_id is not set a new one will be generated for you with the following format: dlp_googleapis_yyyy_mm_dd_[dlp_job_id]. Pacific time zone will be used for generating the date details. For Inspect, each column in an existing output table must have the same name, type, and mode of a field in the `Finding` object. For Risk, an existing output table should be the output of a previous Risk analysis job run on the same source table, with the same privacy metric and quasi-identifiers. Risk jobs that analyze the same table but compute a different privacy metric, or use different sets of quasi-identifiers, cannot store their results in the same table.
    
    pub table: Option<GooglePrivacyDlpV2BigQueryTable>,
}

impl client::Part for GooglePrivacyDlpV2OutputStorageConfig {}


/// Datastore partition ID. A partition ID identifies a grouping of entities. The grouping is always by project and namespace, however the namespace ID may be empty. A partition ID contains several dimensions: project ID and namespace ID.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2PartitionId {
    /// If not empty, the ID of the namespace to which the entities belong.
    #[serde(rename="namespaceId")]
    
    pub namespace_id: Option<String>,
    /// The ID of the project to which the entities belong.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2PartitionId {}


/// A (kind, ID/name) pair used to construct a key path. If either name or ID is set, the element is complete. If neither is set, the element is incomplete.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2PathElement {
    /// The auto-allocated ID of the entity. Never equal to zero. Values less than zero are discouraged and may not be supported in the future.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// The kind of the entity. A kind matching regex `__.*__` is reserved/read-only. A kind must not contain more than 1500 bytes when UTF-8 encoded. Cannot be `""`.
    
    pub kind: Option<String>,
    /// The name of the entity. A name matching regex `__.*__` is reserved/read-only. A name must not be more than 1500 bytes when UTF-8 encoded. Cannot be `""`.
    
    pub name: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2PathElement {}


/// A rule for transforming a value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2PrimitiveTransformation {
    /// Bucketing
    #[serde(rename="bucketingConfig")]
    
    pub bucketing_config: Option<GooglePrivacyDlpV2BucketingConfig>,
    /// Mask
    #[serde(rename="characterMaskConfig")]
    
    pub character_mask_config: Option<GooglePrivacyDlpV2CharacterMaskConfig>,
    /// Deterministic Crypto
    #[serde(rename="cryptoDeterministicConfig")]
    
    pub crypto_deterministic_config: Option<GooglePrivacyDlpV2CryptoDeterministicConfig>,
    /// Crypto
    #[serde(rename="cryptoHashConfig")]
    
    pub crypto_hash_config: Option<GooglePrivacyDlpV2CryptoHashConfig>,
    /// Ffx-Fpe
    #[serde(rename="cryptoReplaceFfxFpeConfig")]
    
    pub crypto_replace_ffx_fpe_config: Option<GooglePrivacyDlpV2CryptoReplaceFfxFpeConfig>,
    /// Date Shift
    #[serde(rename="dateShiftConfig")]
    
    pub date_shift_config: Option<GooglePrivacyDlpV2DateShiftConfig>,
    /// Fixed size bucketing
    #[serde(rename="fixedSizeBucketingConfig")]
    
    pub fixed_size_bucketing_config: Option<GooglePrivacyDlpV2FixedSizeBucketingConfig>,
    /// Redact
    #[serde(rename="redactConfig")]
    
    pub redact_config: Option<GooglePrivacyDlpV2RedactConfig>,
    /// Replace with a specified value.
    #[serde(rename="replaceConfig")]
    
    pub replace_config: Option<GooglePrivacyDlpV2ReplaceValueConfig>,
    /// Replace with a value randomly drawn (with replacement) from a dictionary.
    #[serde(rename="replaceDictionaryConfig")]
    
    pub replace_dictionary_config: Option<GooglePrivacyDlpV2ReplaceDictionaryConfig>,
    /// Replace with infotype
    #[serde(rename="replaceWithInfoTypeConfig")]
    
    pub replace_with_info_type_config: Option<GooglePrivacyDlpV2ReplaceWithInfoTypeConfig>,
    /// Time extraction
    #[serde(rename="timePartConfig")]
    
    pub time_part_config: Option<GooglePrivacyDlpV2TimePartConfig>,
}

impl client::Part for GooglePrivacyDlpV2PrimitiveTransformation {}


/// Privacy metric to compute for reidentification risk analysis.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2PrivacyMetric {
    /// Categorical stats
    #[serde(rename="categoricalStatsConfig")]
    
    pub categorical_stats_config: Option<GooglePrivacyDlpV2CategoricalStatsConfig>,
    /// delta-presence
    #[serde(rename="deltaPresenceEstimationConfig")]
    
    pub delta_presence_estimation_config: Option<GooglePrivacyDlpV2DeltaPresenceEstimationConfig>,
    /// K-anonymity
    #[serde(rename="kAnonymityConfig")]
    
    pub k_anonymity_config: Option<GooglePrivacyDlpV2KAnonymityConfig>,
    /// k-map
    #[serde(rename="kMapEstimationConfig")]
    
    pub k_map_estimation_config: Option<GooglePrivacyDlpV2KMapEstimationConfig>,
    /// l-diversity
    #[serde(rename="lDiversityConfig")]
    
    pub l_diversity_config: Option<GooglePrivacyDlpV2LDiversityConfig>,
    /// Numerical stats
    #[serde(rename="numericalStatsConfig")]
    
    pub numerical_stats_config: Option<GooglePrivacyDlpV2NumericalStatsConfig>,
}

impl client::Part for GooglePrivacyDlpV2PrivacyMetric {}


/// Message for specifying a window around a finding to apply a detection rule.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2Proximity {
    /// Number of characters after the finding to consider.
    #[serde(rename="windowAfter")]
    
    pub window_after: Option<i32>,
    /// Number of characters before the finding to consider. For tabular data, if you want to modify the likelihood of an entire column of findngs, set this to 1. For more information, see [Hotword example: Set the match likelihood of a table column] (https://cloud.google.com/dlp/docs/creating-custom-infotypes-likelihood#match-column-values).
    #[serde(rename="windowBefore")]
    
    pub window_before: Option<i32>,
}

impl client::Part for GooglePrivacyDlpV2Proximity {}


/// Publish findings of a DlpJob to Data Catalog. In Data Catalog, tag templates are applied to the resource that Cloud DLP scanned. Data Catalog tag templates are stored in the same project and region where the BigQuery table exists. For Cloud DLP to create and apply the tag template, the Cloud DLP service agent must have the `roles/datacatalog.tagTemplateOwner` permission on the project. The tag template contains fields summarizing the results of the DlpJob. Any field values previously written by another DlpJob are deleted. InfoType naming patterns are strictly enforced when using this feature. Findings are persisted in Data Catalog storage and are governed by service-specific policies for Data Catalog. For more information, see [Service Specific Terms](https://cloud.google.com/terms/service-terms). Only a single instance of this action can be specified. This action is allowed only if all resources being scanned are BigQuery tables. Compatible with: Inspect
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2PublishFindingsToCloudDataCatalog { _never_set: Option<bool> }

impl client::Part for GooglePrivacyDlpV2PublishFindingsToCloudDataCatalog {}


/// Publish the result summary of a DlpJob to [Security Command Center](https://cloud.google.com/security-command-center). This action is available for only projects that belong to an organization. This action publishes the count of finding instances and their infoTypes. The summary of findings are persisted in Security Command Center and are governed by [service-specific policies for Security Command Center](https://cloud.google.com/terms/service-terms). Only a single instance of this action can be specified. Compatible with: Inspect
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2PublishSummaryToCscc { _never_set: Option<bool> }

impl client::Part for GooglePrivacyDlpV2PublishSummaryToCscc {}


/// Publish a message into a given Pub/Sub topic when DlpJob has completed. The message contains a single field, `DlpJobName`, which is equal to the finished job's [`DlpJob.name`](https://cloud.google.com/dlp/docs/reference/rest/v2/projects.dlpJobs#DlpJob). Compatible with: Inspect, Risk
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2PublishToPubSub {
    /// Cloud Pub/Sub topic to send notifications to. The topic must have given publishing access rights to the DLP API service account executing the long running DlpJob sending the notifications. Format is projects/{project}/topics/{topic}.
    
    pub topic: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2PublishToPubSub {}


/// Enable Stackdriver metric dlp.googleapis.com/finding_count. This will publish a metric to stack driver on each infotype requested and how many findings were found for it. CustomDetectors will be bucketed as 'Custom' under the Stackdriver label 'info_type'.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2PublishToStackdriver { _never_set: Option<bool> }

impl client::Part for GooglePrivacyDlpV2PublishToStackdriver {}


/// A column with a semantic tag attached.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2QuasiId {
    /// A column can be tagged with a custom tag. In this case, the user must indicate an auxiliary table that contains statistical information on the possible values of this column (below).
    #[serde(rename="customTag")]
    
    pub custom_tag: Option<String>,
    /// Required. Identifies the column.
    
    pub field: Option<GooglePrivacyDlpV2FieldId>,
    /// If no semantic tag is indicated, we infer the statistical model from the distribution of values in the input data
    
    pub inferred: Option<GoogleProtobufEmpty>,
    /// A column can be tagged with a InfoType to use the relevant public dataset as a statistical model of population, if available. We currently support US ZIP codes, region codes, ages and genders. To programmatically obtain the list of supported InfoTypes, use ListInfoTypes with the supported_by=RISK_ANALYSIS filter.
    #[serde(rename="infoType")]
    
    pub info_type: Option<GooglePrivacyDlpV2InfoType>,
}

impl client::Part for GooglePrivacyDlpV2QuasiId {}


/// A quasi-identifier column has a custom_tag, used to know which column in the data corresponds to which column in the statistical model.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2QuasiIdField {
    /// A auxiliary field.
    #[serde(rename="customTag")]
    
    pub custom_tag: Option<String>,
    /// Identifies the column.
    
    pub field: Option<GooglePrivacyDlpV2FieldId>,
}

impl client::Part for GooglePrivacyDlpV2QuasiIdField {}


/// A quasi-identifier column has a custom_tag, used to know which column in the data corresponds to which column in the statistical model.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2QuasiIdentifierField {
    /// A column can be tagged with a custom tag. In this case, the user must indicate an auxiliary table that contains statistical information on the possible values of this column (below).
    #[serde(rename="customTag")]
    
    pub custom_tag: Option<String>,
    /// Identifies the column.
    
    pub field: Option<GooglePrivacyDlpV2FieldId>,
}

impl client::Part for GooglePrivacyDlpV2QuasiIdentifierField {}


/// Message for infoType-dependent details parsed from quote.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2QuoteInfo {
    /// The date time indicated by the quote.
    #[serde(rename="dateTime")]
    
    pub date_time: Option<GooglePrivacyDlpV2DateTime>,
}

impl client::Part for GooglePrivacyDlpV2QuoteInfo {}


/// Generic half-open interval [start, end)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2Range {
    /// Index of the last character of the range (exclusive).
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub end: Option<i64>,
    /// Index of the first character of the range (inclusive).
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub start: Option<i64>,
}

impl client::Part for GooglePrivacyDlpV2Range {}


/// A condition for determining whether a transformation should be applied to a field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2RecordCondition {
    /// An expression.
    
    pub expressions: Option<GooglePrivacyDlpV2Expressions>,
}

impl client::Part for GooglePrivacyDlpV2RecordCondition {}


/// Message for a unique key indicating a record that contains a finding.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2RecordKey {
    /// no description provided
    #[serde(rename="bigQueryKey")]
    
    pub big_query_key: Option<GooglePrivacyDlpV2BigQueryKey>,
    /// no description provided
    #[serde(rename="datastoreKey")]
    
    pub datastore_key: Option<GooglePrivacyDlpV2DatastoreKey>,
    /// Values of identifying columns in the given row. Order of values matches the order of `identifying_fields` specified in the scanning request.
    #[serde(rename="idValues")]
    
    pub id_values: Option<Vec<String>>,
}

impl client::Part for GooglePrivacyDlpV2RecordKey {}


/// Location of a finding within a row or record.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2RecordLocation {
    /// Field id of the field containing the finding.
    #[serde(rename="fieldId")]
    
    pub field_id: Option<GooglePrivacyDlpV2FieldId>,
    /// Key of the finding.
    #[serde(rename="recordKey")]
    
    pub record_key: Option<GooglePrivacyDlpV2RecordKey>,
    /// Location within a `ContentItem.Table`.
    #[serde(rename="tableLocation")]
    
    pub table_location: Option<GooglePrivacyDlpV2TableLocation>,
}

impl client::Part for GooglePrivacyDlpV2RecordLocation {}


/// Configuration to suppress records whose suppression conditions evaluate to true.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2RecordSuppression {
    /// A condition that when it evaluates to true will result in the record being evaluated to be suppressed from the transformed content.
    
    pub condition: Option<GooglePrivacyDlpV2RecordCondition>,
}

impl client::Part for GooglePrivacyDlpV2RecordSuppression {}


/// A type of transformation that is applied over structured data such as a table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2RecordTransformations {
    /// Transform the record by applying various field transformations.
    #[serde(rename="fieldTransformations")]
    
    pub field_transformations: Option<Vec<GooglePrivacyDlpV2FieldTransformation>>,
    /// Configuration defining which records get suppressed entirely. Records that match any suppression rule are omitted from the output.
    #[serde(rename="recordSuppressions")]
    
    pub record_suppressions: Option<Vec<GooglePrivacyDlpV2RecordSuppression>>,
}

impl client::Part for GooglePrivacyDlpV2RecordTransformations {}


/// Redact a given value. For example, if used with an `InfoTypeTransformation` transforming PHONE_NUMBER, and input 'My phone number is 206-555-0123', the output would be 'My phone number is '.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2RedactConfig { _never_set: Option<bool> }

impl client::Part for GooglePrivacyDlpV2RedactConfig {}


/// Request to search for potentially sensitive info in an image and redact it by covering it with a colored rectangle.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [image redact projects](ProjectImageRedactCall) (request)
/// * [locations image redact projects](ProjectLocationImageRedactCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2RedactImageRequest {
    /// The content must be PNG, JPEG, SVG or BMP.
    #[serde(rename="byteItem")]
    
    pub byte_item: Option<GooglePrivacyDlpV2ByteContentItem>,
    /// The configuration for specifying what content to redact from images.
    #[serde(rename="imageRedactionConfigs")]
    
    pub image_redaction_configs: Option<Vec<GooglePrivacyDlpV2ImageRedactionConfig>>,
    /// Whether the response should include findings along with the redacted image.
    #[serde(rename="includeFindings")]
    
    pub include_findings: Option<bool>,
    /// Configuration for the inspector.
    #[serde(rename="inspectConfig")]
    
    pub inspect_config: Option<GooglePrivacyDlpV2InspectConfig>,
    /// Deprecated. This field has no effect.
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
}

impl client::RequestValue for GooglePrivacyDlpV2RedactImageRequest {}


/// Results of redacting an image.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [image redact projects](ProjectImageRedactCall) (response)
/// * [locations image redact projects](ProjectLocationImageRedactCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2RedactImageResponse {
    /// If an image was being inspected and the InspectConfig's include_quote was set to true, then this field will include all text, if any, that was found in the image.
    #[serde(rename="extractedText")]
    
    pub extracted_text: Option<String>,
    /// The findings. Populated when include_findings in the request is true.
    #[serde(rename="inspectResult")]
    
    pub inspect_result: Option<GooglePrivacyDlpV2InspectResult>,
    /// The redacted image. The type will be the same as the original image.
    #[serde(rename="redactedImage")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub redacted_image: Option<Vec<u8>>,
}

impl client::ResponseResult for GooglePrivacyDlpV2RedactImageResponse {}


/// Message defining a custom regular expression.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2Regex {
    /// The index of the submatch to extract as findings. When not specified, the entire match is returned. No more than 3 may be included.
    #[serde(rename="groupIndexes")]
    
    pub group_indexes: Option<Vec<i32>>,
    /// Pattern defining the regular expression. Its syntax (https://github.com/google/re2/wiki/Syntax) can be found under the google/re2 repository on GitHub.
    
    pub pattern: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2Regex {}


/// Request to re-identify an item.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [content reidentify projects](ProjectContentReidentifyCall) (request)
/// * [locations content reidentify projects](ProjectLocationContentReidentifyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2ReidentifyContentRequest {
    /// Configuration for the inspector.
    #[serde(rename="inspectConfig")]
    
    pub inspect_config: Option<GooglePrivacyDlpV2InspectConfig>,
    /// Template to use. Any configuration directly specified in `inspect_config` will override those set in the template. Singular fields that are set in this request will replace their corresponding fields in the template. Repeated fields are appended. Singular sub-messages and groups are recursively merged.
    #[serde(rename="inspectTemplateName")]
    
    pub inspect_template_name: Option<String>,
    /// The item to re-identify. Will be treated as text.
    
    pub item: Option<GooglePrivacyDlpV2ContentItem>,
    /// Deprecated. This field has no effect.
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
    /// Configuration for the re-identification of the content item. This field shares the same proto message type that is used for de-identification, however its usage here is for the reversal of the previous de-identification. Re-identification is performed by examining the transformations used to de-identify the items and executing the reverse. This requires that only reversible transformations be provided here. The reversible transformations are: - `CryptoDeterministicConfig` - `CryptoReplaceFfxFpeConfig`
    #[serde(rename="reidentifyConfig")]
    
    pub reidentify_config: Option<GooglePrivacyDlpV2DeidentifyConfig>,
    /// Template to use. References an instance of `DeidentifyTemplate`. Any configuration directly specified in `reidentify_config` or `inspect_config` will override those set in the template. The `DeidentifyTemplate` used must include only reversible transformations. Singular fields that are set in this request will replace their corresponding fields in the template. Repeated fields are appended. Singular sub-messages and groups are recursively merged.
    #[serde(rename="reidentifyTemplateName")]
    
    pub reidentify_template_name: Option<String>,
}

impl client::RequestValue for GooglePrivacyDlpV2ReidentifyContentRequest {}


/// Results of re-identifying an item.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [content reidentify projects](ProjectContentReidentifyCall) (response)
/// * [locations content reidentify projects](ProjectLocationContentReidentifyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2ReidentifyContentResponse {
    /// The re-identified item.
    
    pub item: Option<GooglePrivacyDlpV2ContentItem>,
    /// An overview of the changes that were made to the `item`.
    
    pub overview: Option<GooglePrivacyDlpV2TransformationOverview>,
}

impl client::ResponseResult for GooglePrivacyDlpV2ReidentifyContentResponse {}


/// Replace each input value with a value randomly selected from the dictionary.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2ReplaceDictionaryConfig {
    /// A list of words to select from for random replacement. The [limits](https://cloud.google.com/dlp/limits) page contains details about the size limits of dictionaries.
    #[serde(rename="wordList")]
    
    pub word_list: Option<GooglePrivacyDlpV2WordList>,
}

impl client::Part for GooglePrivacyDlpV2ReplaceDictionaryConfig {}


/// Replace each input value with a given `Value`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2ReplaceValueConfig {
    /// Value to replace it with.
    #[serde(rename="newValue")]
    
    pub new_value: Option<GooglePrivacyDlpV2Value>,
}

impl client::Part for GooglePrivacyDlpV2ReplaceValueConfig {}


/// Replace each matching finding with the name of the info_type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2ReplaceWithInfoTypeConfig { _never_set: Option<bool> }

impl client::Part for GooglePrivacyDlpV2ReplaceWithInfoTypeConfig {}


/// Snapshot of the inspection configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2RequestedOptions {
    /// Inspect config.
    #[serde(rename="jobConfig")]
    
    pub job_config: Option<GooglePrivacyDlpV2InspectJobConfig>,
    /// If run with an InspectTemplate, a snapshot of its state at the time of this run.
    #[serde(rename="snapshotInspectTemplate")]
    
    pub snapshot_inspect_template: Option<GooglePrivacyDlpV2InspectTemplate>,
}

impl client::Part for GooglePrivacyDlpV2RequestedOptions {}


/// Risk analysis options.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2RequestedRiskAnalysisOptions {
    /// The job config for the risk job.
    #[serde(rename="jobConfig")]
    
    pub job_config: Option<GooglePrivacyDlpV2RiskAnalysisJobConfig>,
}

impl client::Part for GooglePrivacyDlpV2RequestedRiskAnalysisOptions {}


/// All result fields mentioned below are updated while the job is processing.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2Result {
    /// Statistics related to the processing of hybrid inspect.
    #[serde(rename="hybridStats")]
    
    pub hybrid_stats: Option<GooglePrivacyDlpV2HybridInspectStatistics>,
    /// Statistics of how many instances of each info type were found during inspect job.
    #[serde(rename="infoTypeStats")]
    
    pub info_type_stats: Option<Vec<GooglePrivacyDlpV2InfoTypeStats>>,
    /// Total size in bytes that were processed.
    #[serde(rename="processedBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub processed_bytes: Option<i64>,
    /// Estimate of the number of bytes to process.
    #[serde(rename="totalEstimatedBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_estimated_bytes: Option<i64>,
}

impl client::Part for GooglePrivacyDlpV2Result {}


/// Configuration for a risk analysis job. See https://cloud.google.com/dlp/docs/concepts-risk-analysis to learn more.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2RiskAnalysisJobConfig {
    /// Actions to execute at the completion of the job. Are executed in the order provided.
    
    pub actions: Option<Vec<GooglePrivacyDlpV2Action>>,
    /// Privacy metric to compute.
    #[serde(rename="privacyMetric")]
    
    pub privacy_metric: Option<GooglePrivacyDlpV2PrivacyMetric>,
    /// Input dataset to compute metrics over.
    #[serde(rename="sourceTable")]
    
    pub source_table: Option<GooglePrivacyDlpV2BigQueryTable>,
}

impl client::Part for GooglePrivacyDlpV2RiskAnalysisJobConfig {}


/// Values of the row.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2Row {
    /// Individual cells.
    
    pub values: Option<Vec<GooglePrivacyDlpV2Value>>,
}

impl client::Part for GooglePrivacyDlpV2Row {}


/// If set, the detailed findings will be persisted to the specified OutputStorageConfig. Only a single instance of this action can be specified. Compatible with: Inspect, Risk
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2SaveFindings {
    /// Location to store findings outside of DLP.
    #[serde(rename="outputConfig")]
    
    pub output_config: Option<GooglePrivacyDlpV2OutputStorageConfig>,
}

impl client::Part for GooglePrivacyDlpV2SaveFindings {}


/// Schedule for inspect job triggers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2Schedule {
    /// With this option a job is started on a regular periodic basis. For example: every day (86400 seconds). A scheduled start time will be skipped if the previous execution has not ended when its scheduled time occurs. This value must be set to a time duration greater than or equal to 1 day and can be no longer than 60 days.
    #[serde(rename="recurrencePeriodDuration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub recurrence_period_duration: Option<client::chrono::Duration>,
}

impl client::Part for GooglePrivacyDlpV2Schedule {}


/// Apply transformation to the selected info_types.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2SelectedInfoTypes {
    /// Required. InfoTypes to apply the transformation to. Required. Provided InfoType must be unique within the ImageTransformations message.
    #[serde(rename="infoTypes")]
    
    pub info_types: Option<Vec<GooglePrivacyDlpV2InfoType>>,
}

impl client::Part for GooglePrivacyDlpV2SelectedInfoTypes {}


/// Score is a summary of all elements in the data profile. A higher number means more sensitive.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2SensitivityScore {
    /// The score applied to the resource.
    
    pub score: Option<GooglePrivacyDlpV2SensitivityScoreScoreEnum>,
}

impl client::Part for GooglePrivacyDlpV2SensitivityScore {}


/// An auxiliary table containing statistical information on the relative frequency of different quasi-identifiers values. It has one or several quasi-identifiers columns, and one column that indicates the relative frequency of each quasi-identifier tuple. If a tuple is present in the data but not in the auxiliary table, the corresponding relative frequency is assumed to be zero (and thus, the tuple is highly reidentifiable).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2StatisticalTable {
    /// Required. Quasi-identifier columns.
    #[serde(rename="quasiIds")]
    
    pub quasi_ids: Option<Vec<GooglePrivacyDlpV2QuasiIdentifierField>>,
    /// Required. The relative frequency column must contain a floating-point number between 0 and 1 (inclusive). Null values are assumed to be zero.
    #[serde(rename="relativeFrequency")]
    
    pub relative_frequency: Option<GooglePrivacyDlpV2FieldId>,
    /// Required. Auxiliary table location.
    
    pub table: Option<GooglePrivacyDlpV2BigQueryTable>,
}

impl client::Part for GooglePrivacyDlpV2StatisticalTable {}


/// Shared message indicating Cloud storage type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2StorageConfig {
    /// BigQuery options.
    #[serde(rename="bigQueryOptions")]
    
    pub big_query_options: Option<GooglePrivacyDlpV2BigQueryOptions>,
    /// Cloud Storage options.
    #[serde(rename="cloudStorageOptions")]
    
    pub cloud_storage_options: Option<GooglePrivacyDlpV2CloudStorageOptions>,
    /// Google Cloud Datastore options.
    #[serde(rename="datastoreOptions")]
    
    pub datastore_options: Option<GooglePrivacyDlpV2DatastoreOptions>,
    /// Hybrid inspection options.
    #[serde(rename="hybridOptions")]
    
    pub hybrid_options: Option<GooglePrivacyDlpV2HybridOptions>,
    /// no description provided
    #[serde(rename="timespanConfig")]
    
    pub timespan_config: Option<GooglePrivacyDlpV2TimespanConfig>,
}

impl client::Part for GooglePrivacyDlpV2StorageConfig {}


/// Storage metadata label to indicate which metadata entry contains findings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2StorageMetadataLabel {
    /// no description provided
    
    pub key: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2StorageMetadataLabel {}


/// StoredInfoType resource message that contains information about the current version and any pending updates.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations stored info types create organizations](OrganizationLocationStoredInfoTypeCreateCall) (response)
/// * [locations stored info types get organizations](OrganizationLocationStoredInfoTypeGetCall) (response)
/// * [locations stored info types patch organizations](OrganizationLocationStoredInfoTypePatchCall) (response)
/// * [stored info types create organizations](OrganizationStoredInfoTypeCreateCall) (response)
/// * [stored info types get organizations](OrganizationStoredInfoTypeGetCall) (response)
/// * [stored info types patch organizations](OrganizationStoredInfoTypePatchCall) (response)
/// * [locations stored info types create projects](ProjectLocationStoredInfoTypeCreateCall) (response)
/// * [locations stored info types get projects](ProjectLocationStoredInfoTypeGetCall) (response)
/// * [locations stored info types patch projects](ProjectLocationStoredInfoTypePatchCall) (response)
/// * [stored info types create projects](ProjectStoredInfoTypeCreateCall) (response)
/// * [stored info types get projects](ProjectStoredInfoTypeGetCall) (response)
/// * [stored info types patch projects](ProjectStoredInfoTypePatchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2StoredInfoType {
    /// Current version of the stored info type.
    #[serde(rename="currentVersion")]
    
    pub current_version: Option<GooglePrivacyDlpV2StoredInfoTypeVersion>,
    /// Resource name.
    
    pub name: Option<String>,
    /// Pending versions of the stored info type. Empty if no versions are pending.
    #[serde(rename="pendingVersions")]
    
    pub pending_versions: Option<Vec<GooglePrivacyDlpV2StoredInfoTypeVersion>>,
}

impl client::ResponseResult for GooglePrivacyDlpV2StoredInfoType {}


/// Configuration for stored infoTypes. All fields and subfield are provided by the user. For more information, see https://cloud.google.com/dlp/docs/creating-custom-infotypes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2StoredInfoTypeConfig {
    /// Description of the StoredInfoType (max 256 characters).
    
    pub description: Option<String>,
    /// Store dictionary-based CustomInfoType.
    
    pub dictionary: Option<GooglePrivacyDlpV2Dictionary>,
    /// Display name of the StoredInfoType (max 256 characters).
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// StoredInfoType where findings are defined by a dictionary of phrases.
    #[serde(rename="largeCustomDictionary")]
    
    pub large_custom_dictionary: Option<GooglePrivacyDlpV2LargeCustomDictionaryConfig>,
    /// Store regular expression-based StoredInfoType.
    
    pub regex: Option<GooglePrivacyDlpV2Regex>,
}

impl client::Part for GooglePrivacyDlpV2StoredInfoTypeConfig {}


/// Statistics for a StoredInfoType.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2StoredInfoTypeStats {
    /// StoredInfoType where findings are defined by a dictionary of phrases.
    #[serde(rename="largeCustomDictionary")]
    
    pub large_custom_dictionary: Option<GooglePrivacyDlpV2LargeCustomDictionaryStats>,
}

impl client::Part for GooglePrivacyDlpV2StoredInfoTypeStats {}


/// Version of a StoredInfoType, including the configuration used to build it, create timestamp, and current state.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2StoredInfoTypeVersion {
    /// StoredInfoType configuration.
    
    pub config: Option<GooglePrivacyDlpV2StoredInfoTypeConfig>,
    /// Create timestamp of the version. Read-only, determined by the system when the version is created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Errors that occurred when creating this storedInfoType version, or anomalies detected in the storedInfoType data that render it unusable. Only the five most recent errors will be displayed, with the most recent error appearing first. For example, some of the data for stored custom dictionaries is put in the user's Cloud Storage bucket, and if this data is modified or deleted by the user or another system, the dictionary becomes invalid. If any errors occur, fix the problem indicated by the error message and use the UpdateStoredInfoType API method to create another version of the storedInfoType to continue using it, reusing the same `config` if it was not the source of the error.
    
    pub errors: Option<Vec<GooglePrivacyDlpV2Error>>,
    /// Stored info type version state. Read-only, updated by the system during dictionary creation.
    
    pub state: Option<GooglePrivacyDlpV2StoredInfoTypeVersionStateEnum>,
    /// Statistics about this storedInfoType version.
    
    pub stats: Option<GooglePrivacyDlpV2StoredInfoTypeStats>,
}

impl client::Part for GooglePrivacyDlpV2StoredInfoTypeVersion {}


/// A reference to a StoredInfoType to use with scanning.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2StoredType {
    /// Timestamp indicating when the version of the `StoredInfoType` used for inspection was created. Output-only field, populated by the system.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Resource name of the requested `StoredInfoType`, for example `organizations/433245324/storedInfoTypes/432452342` or `projects/project-id/storedInfoTypes/432452342`.
    
    pub name: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2StoredType {}


/// A collection that informs the user the number of times a particular `TransformationResultCode` and error details occurred.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2SummaryResult {
    /// Outcome of the transformation.
    
    pub code: Option<GooglePrivacyDlpV2SummaryResultCodeEnum>,
    /// Number of transformations counted by this result.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
    /// A place for warnings or errors to show up if a transformation didn't work as expected.
    
    pub details: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2SummaryResult {}


/// Message for detecting output from deidentification transformations such as [`CryptoReplaceFfxFpeConfig`](https://cloud.google.com/dlp/docs/reference/rest/v2/organizations.deidentifyTemplates#cryptoreplaceffxfpeconfig). These types of transformations are those that perform pseudonymization, thereby producing a "surrogate" as output. This should be used in conjunction with a field on the transformation such as `surrogate_info_type`. This CustomInfoType does not support the use of `detection_rules`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2SurrogateType { _never_set: Option<bool> }

impl client::Part for GooglePrivacyDlpV2SurrogateType {}


/// Structured content to inspect. Up to 50,000 `Value`s per request allowed. See https://cloud.google.com/dlp/docs/inspecting-structured-text#inspecting_a_table to learn more.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2Table {
    /// Headers of the table.
    
    pub headers: Option<Vec<GooglePrivacyDlpV2FieldId>>,
    /// Rows of the table.
    
    pub rows: Option<Vec<GooglePrivacyDlpV2Row>>,
}

impl client::Part for GooglePrivacyDlpV2Table {}


/// Location of a finding within a table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2TableLocation {
    /// The zero-based index of the row where the finding is located. Only populated for resources that have a natural ordering, not BigQuery. In BigQuery, to identify the row a finding came from, populate BigQueryOptions.identifying_fields with your primary key column names and when you store the findings the value of those columns will be stored inside of Finding.
    #[serde(rename="rowIndex")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub row_index: Option<i64>,
}

impl client::Part for GooglePrivacyDlpV2TableLocation {}


/// Instructions regarding the table content being inspected.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2TableOptions {
    /// The columns that are the primary keys for table objects included in ContentItem. A copy of this cell's value will stored alongside alongside each finding so that the finding can be traced to the specific row it came from. No more than 3 may be provided.
    #[serde(rename="identifyingFields")]
    
    pub identifying_fields: Option<Vec<GooglePrivacyDlpV2FieldId>>,
}

impl client::Part for GooglePrivacyDlpV2TableOptions {}


/// A column with a semantic tag attached.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2TaggedField {
    /// A column can be tagged with a custom tag. In this case, the user must indicate an auxiliary table that contains statistical information on the possible values of this column (below).
    #[serde(rename="customTag")]
    
    pub custom_tag: Option<String>,
    /// Required. Identifies the column.
    
    pub field: Option<GooglePrivacyDlpV2FieldId>,
    /// If no semantic tag is indicated, we infer the statistical model from the distribution of values in the input data
    
    pub inferred: Option<GoogleProtobufEmpty>,
    /// A column can be tagged with a InfoType to use the relevant public dataset as a statistical model of population, if available. We currently support US ZIP codes, region codes, ages and genders. To programmatically obtain the list of supported InfoTypes, use ListInfoTypes with the supported_by=RISK_ANALYSIS filter.
    #[serde(rename="infoType")]
    
    pub info_type: Option<GooglePrivacyDlpV2InfoType>,
}

impl client::Part for GooglePrivacyDlpV2TaggedField {}


/// Throw an error and fail the request when a transformation error occurs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2ThrowError { _never_set: Option<bool> }

impl client::Part for GooglePrivacyDlpV2ThrowError {}


/// For use with `Date`, `Timestamp`, and `TimeOfDay`, extract or preserve a portion of the value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2TimePartConfig {
    /// The part of the time to keep.
    #[serde(rename="partToExtract")]
    
    pub part_to_extract: Option<GooglePrivacyDlpV2TimePartConfigPartToExtractEnum>,
}

impl client::Part for GooglePrivacyDlpV2TimePartConfig {}


/// Time zone of the date time object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2TimeZone {
    /// Set only if the offset can be determined. Positive for time ahead of UTC. E.g. For "UTC-9", this value is -540.
    #[serde(rename="offsetMinutes")]
    
    pub offset_minutes: Option<i32>,
}

impl client::Part for GooglePrivacyDlpV2TimeZone {}


/// Configuration of the timespan of the items to include in scanning. Currently only supported when inspecting Cloud Storage and BigQuery.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2TimespanConfig {
    /// When the job is started by a JobTrigger we will automatically figure out a valid start_time to avoid scanning files that have not been modified since the last time the JobTrigger executed. This will be based on the time of the execution of the last run of the JobTrigger or the timespan end_time used in the last run of the JobTrigger.
    #[serde(rename="enableAutoPopulationOfTimespanConfig")]
    
    pub enable_auto_population_of_timespan_config: Option<bool>,
    /// Exclude files, tables, or rows newer than this value. If not set, no upper time limit is applied.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Exclude files, tables, or rows older than this value. If not set, no lower time limit is applied.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Specification of the field containing the timestamp of scanned items. Used for data sources like Datastore and BigQuery. *For BigQuery* If this value is not specified and the table was modified between the given start and end times, the entire table will be scanned. If this value is specified, then rows are filtered based on the given start and end times. Rows with a `NULL` value in the provided BigQuery column are skipped. Valid data types of the provided BigQuery column are: `INTEGER`, `DATE`, `TIMESTAMP`, and `DATETIME`. If your BigQuery table is [partitioned at ingestion time](https://cloud.google.com/bigquery/docs/partitioned-tables#ingestion_time), you can use any of the following pseudo-columns as your timestamp field. When used with Cloud DLP, these pseudo-column names are case sensitive. - _PARTITIONTIME - _PARTITIONDATE - _PARTITION_LOAD_TIME *For Datastore* If this value is specified, then entities are filtered based on the given start and end times. If an entity does not contain the provided timestamp property or contains empty or invalid values, then it is included. Valid data types of the provided timestamp property are: `TIMESTAMP`. See the [known issue](https://cloud.google.com/dlp/docs/known-issues#bq-timespan) related to this operation.
    #[serde(rename="timestampField")]
    
    pub timestamp_field: Option<GooglePrivacyDlpV2FieldId>,
}

impl client::Part for GooglePrivacyDlpV2TimespanConfig {}


/// User specified templates and configs for how to deidentify structured, unstructures, and image files. User must provide either a unstructured deidentify template or at least one redact image config.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2TransformationConfig {
    /// De-identify template. If this template is specified, it will serve as the default de-identify template. This template cannot contain `record_transformations` since it can be used for unstructured content such as free-form text files. If this template is not set, a default `ReplaceWithInfoTypeConfig` will be used to de-identify unstructured content.
    #[serde(rename="deidentifyTemplate")]
    
    pub deidentify_template: Option<String>,
    /// Image redact template. If this template is specified, it will serve as the de-identify template for images. If this template is not set, all findings in the image will be redacted with a black box.
    #[serde(rename="imageRedactTemplate")]
    
    pub image_redact_template: Option<String>,
    /// Structured de-identify template. If this template is specified, it will serve as the de-identify template for structured content such as delimited files and tables. If this template is not set but the `deidentify_template` is set, then `deidentify_template` will also apply to the structured content. If neither template is set, a default `ReplaceWithInfoTypeConfig` will be used to de-identify structured content.
    #[serde(rename="structuredDeidentifyTemplate")]
    
    pub structured_deidentify_template: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2TransformationConfig {}


/// Config for storing transformation details.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2TransformationDetailsStorageConfig {
    /// The BigQuery table in which to store the output. This may be an existing table or in a new table in an existing dataset. If table_id is not set a new one will be generated for you with the following format: dlp_googleapis_transformation_details_yyyy_mm_dd_[dlp_job_id]. Pacific time zone will be used for generating the date details.
    
    pub table: Option<GooglePrivacyDlpV2BigQueryTable>,
}

impl client::Part for GooglePrivacyDlpV2TransformationDetailsStorageConfig {}


/// How to handle transformation errors during de-identification. A transformation error occurs when the requested transformation is incompatible with the data. For example, trying to de-identify an IP address using a `DateShift` transformation would result in a transformation error, since date info cannot be extracted from an IP address. Information about any incompatible transformations, and how they were handled, is returned in the response as part of the `TransformationOverviews`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2TransformationErrorHandling {
    /// Ignore errors
    #[serde(rename="leaveUntransformed")]
    
    pub leave_untransformed: Option<GooglePrivacyDlpV2LeaveUntransformed>,
    /// Throw an error
    #[serde(rename="throwError")]
    
    pub throw_error: Option<GooglePrivacyDlpV2ThrowError>,
}

impl client::Part for GooglePrivacyDlpV2TransformationErrorHandling {}


/// Overview of the modifications that occurred.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2TransformationOverview {
    /// Transformations applied to the dataset.
    #[serde(rename="transformationSummaries")]
    
    pub transformation_summaries: Option<Vec<GooglePrivacyDlpV2TransformationSummary>>,
    /// Total size in bytes that were transformed in some way.
    #[serde(rename="transformedBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub transformed_bytes: Option<i64>,
}

impl client::Part for GooglePrivacyDlpV2TransformationOverview {}


/// Summary of a single transformation. Only one of 'transformation', 'field_transformation', or 'record_suppress' will be set.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2TransformationSummary {
    /// Set if the transformation was limited to a specific FieldId.
    
    pub field: Option<GooglePrivacyDlpV2FieldId>,
    /// The field transformation that was applied. If multiple field transformations are requested for a single field, this list will contain all of them; otherwise, only one is supplied.
    #[serde(rename="fieldTransformations")]
    
    pub field_transformations: Option<Vec<GooglePrivacyDlpV2FieldTransformation>>,
    /// Set if the transformation was limited to a specific InfoType.
    #[serde(rename="infoType")]
    
    pub info_type: Option<GooglePrivacyDlpV2InfoType>,
    /// The specific suppression option these stats apply to.
    #[serde(rename="recordSuppress")]
    
    pub record_suppress: Option<GooglePrivacyDlpV2RecordSuppression>,
    /// Collection of all transformations that took place or had an error.
    
    pub results: Option<Vec<GooglePrivacyDlpV2SummaryResult>>,
    /// The specific transformation these stats apply to.
    
    pub transformation: Option<GooglePrivacyDlpV2PrimitiveTransformation>,
    /// Total size in bytes that were transformed in some way.
    #[serde(rename="transformedBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub transformed_bytes: Option<i64>,
}

impl client::Part for GooglePrivacyDlpV2TransformationSummary {}


/// Use this to have a random data crypto key generated. It will be discarded after the request finishes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2TransientCryptoKey {
    /// Required. Name of the key. This is an arbitrary string used to differentiate different keys. A unique key is generated per name: two separate `TransientCryptoKey` protos share the same generated key if their names are the same. When the data crypto key is generated, this name is not used in any way (repeating the api call will result in a different key being generated).
    
    pub name: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2TransientCryptoKey {}


/// What event needs to occur for a new job to be started.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2Trigger {
    /// For use with hybrid jobs. Jobs must be manually created and finished.
    
    pub manual: Option<GooglePrivacyDlpV2Manual>,
    /// Create a job on a repeating basis based on the elapse of time.
    
    pub schedule: Option<GooglePrivacyDlpV2Schedule>,
}

impl client::Part for GooglePrivacyDlpV2Trigger {}


/// Using raw keys is prone to security risks due to accidentally leaking the key. Choose another type of key if possible.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2UnwrappedCryptoKey {
    /// Required. A 128/192/256 bit key.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub key: Option<Vec<u8>>,
}

impl client::Part for GooglePrivacyDlpV2UnwrappedCryptoKey {}


/// Request message for UpdateDeidentifyTemplate.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [deidentify templates patch organizations](OrganizationDeidentifyTemplatePatchCall) (request)
/// * [locations deidentify templates patch organizations](OrganizationLocationDeidentifyTemplatePatchCall) (request)
/// * [deidentify templates patch projects](ProjectDeidentifyTemplatePatchCall) (request)
/// * [locations deidentify templates patch projects](ProjectLocationDeidentifyTemplatePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2UpdateDeidentifyTemplateRequest {
    /// New DeidentifyTemplate value.
    #[serde(rename="deidentifyTemplate")]
    
    pub deidentify_template: Option<GooglePrivacyDlpV2DeidentifyTemplate>,
    /// Mask to control which fields get updated.
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for GooglePrivacyDlpV2UpdateDeidentifyTemplateRequest {}


/// Request message for UpdateInspectTemplate.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [inspect templates patch organizations](OrganizationInspectTemplatePatchCall) (request)
/// * [locations inspect templates patch organizations](OrganizationLocationInspectTemplatePatchCall) (request)
/// * [inspect templates patch projects](ProjectInspectTemplatePatchCall) (request)
/// * [locations inspect templates patch projects](ProjectLocationInspectTemplatePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2UpdateInspectTemplateRequest {
    /// New InspectTemplate value.
    #[serde(rename="inspectTemplate")]
    
    pub inspect_template: Option<GooglePrivacyDlpV2InspectTemplate>,
    /// Mask to control which fields get updated.
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for GooglePrivacyDlpV2UpdateInspectTemplateRequest {}


/// Request message for UpdateJobTrigger.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations job triggers patch organizations](OrganizationLocationJobTriggerPatchCall) (request)
/// * [job triggers patch projects](ProjectJobTriggerPatchCall) (request)
/// * [locations job triggers patch projects](ProjectLocationJobTriggerPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2UpdateJobTriggerRequest {
    /// New JobTrigger value.
    #[serde(rename="jobTrigger")]
    
    pub job_trigger: Option<GooglePrivacyDlpV2JobTrigger>,
    /// Mask to control which fields get updated.
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for GooglePrivacyDlpV2UpdateJobTriggerRequest {}


/// Request message for UpdateStoredInfoType.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations stored info types patch organizations](OrganizationLocationStoredInfoTypePatchCall) (request)
/// * [stored info types patch organizations](OrganizationStoredInfoTypePatchCall) (request)
/// * [locations stored info types patch projects](ProjectLocationStoredInfoTypePatchCall) (request)
/// * [stored info types patch projects](ProjectStoredInfoTypePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2UpdateStoredInfoTypeRequest {
    /// Updated configuration for the storedInfoType. If not provided, a new version of the storedInfoType will be created with the existing configuration.
    
    pub config: Option<GooglePrivacyDlpV2StoredInfoTypeConfig>,
    /// Mask to control which fields get updated.
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for GooglePrivacyDlpV2UpdateStoredInfoTypeRequest {}


/// Set of primitive values supported by the system. Note that for the purposes of inspection or transformation, the number of bytes considered to comprise a 'Value' is based on its representation as a UTF-8 encoded string. For example, if 'integer_value' is set to 123456789, the number of bytes would be counted as 9, even though an int64 only holds up to 8 bytes of data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2Value {
    /// boolean
    #[serde(rename="booleanValue")]
    
    pub boolean_value: Option<bool>,
    /// date
    #[serde(rename="dateValue")]
    
    pub date_value: Option<GoogleTypeDate>,
    /// day of week
    #[serde(rename="dayOfWeekValue")]
    
    pub day_of_week_value: Option<GooglePrivacyDlpV2ValueDayOfWeekValueEnum>,
    /// float
    #[serde(rename="floatValue")]
    
    pub float_value: Option<f64>,
    /// integer
    #[serde(rename="integerValue")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub integer_value: Option<i64>,
    /// string
    #[serde(rename="stringValue")]
    
    pub string_value: Option<String>,
    /// time of day
    #[serde(rename="timeValue")]
    
    pub time_value: Option<GoogleTypeTimeOfDay>,
    /// timestamp
    #[serde(rename="timestampValue")]
    
    pub timestamp_value: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GooglePrivacyDlpV2Value {}


/// A value of a field, including its frequency.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2ValueFrequency {
    /// How many times the value is contained in the field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
    /// A value contained in the field in question.
    
    pub value: Option<GooglePrivacyDlpV2Value>,
}

impl client::Part for GooglePrivacyDlpV2ValueFrequency {}


/// Details about each available version for an infotype.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2VersionDescription {
    /// Description of the version.
    
    pub description: Option<String>,
    /// Name of the version
    
    pub version: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2VersionDescription {}


/// Message defining a list of words or phrases to search for in the data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2WordList {
    /// Words or phrases defining the dictionary. The dictionary must contain at least one phrase and every phrase must contain at least 2 characters that are letters or digits. [required]
    
    pub words: Option<Vec<String>>,
}

impl client::Part for GooglePrivacyDlpV2WordList {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [deidentify templates delete organizations](OrganizationDeidentifyTemplateDeleteCall) (response)
/// * [inspect templates delete organizations](OrganizationInspectTemplateDeleteCall) (response)
/// * [locations deidentify templates delete organizations](OrganizationLocationDeidentifyTemplateDeleteCall) (response)
/// * [locations inspect templates delete organizations](OrganizationLocationInspectTemplateDeleteCall) (response)
/// * [locations job triggers delete organizations](OrganizationLocationJobTriggerDeleteCall) (response)
/// * [locations stored info types delete organizations](OrganizationLocationStoredInfoTypeDeleteCall) (response)
/// * [stored info types delete organizations](OrganizationStoredInfoTypeDeleteCall) (response)
/// * [deidentify templates delete projects](ProjectDeidentifyTemplateDeleteCall) (response)
/// * [dlp jobs cancel projects](ProjectDlpJobCancelCall) (response)
/// * [dlp jobs delete projects](ProjectDlpJobDeleteCall) (response)
/// * [inspect templates delete projects](ProjectInspectTemplateDeleteCall) (response)
/// * [job triggers delete projects](ProjectJobTriggerDeleteCall) (response)
/// * [locations deidentify templates delete projects](ProjectLocationDeidentifyTemplateDeleteCall) (response)
/// * [locations dlp jobs cancel projects](ProjectLocationDlpJobCancelCall) (response)
/// * [locations dlp jobs delete projects](ProjectLocationDlpJobDeleteCall) (response)
/// * [locations dlp jobs finish projects](ProjectLocationDlpJobFinishCall) (response)
/// * [locations inspect templates delete projects](ProjectLocationInspectTemplateDeleteCall) (response)
/// * [locations job triggers delete projects](ProjectLocationJobTriggerDeleteCall) (response)
/// * [locations stored info types delete projects](ProjectLocationStoredInfoTypeDeleteCall) (response)
/// * [stored info types delete projects](ProjectStoredInfoTypeDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleProtobufEmpty { _never_set: Option<bool> }

impl client::ResponseResult for GoogleProtobufEmpty {}


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


/// Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values. * A month and day, with a zero year (for example, an anniversary). * A year on its own, with a zero month and a zero day. * A year and month, with a zero day (for example, a credit card expiration date). Related types: * google.type.TimeOfDay * google.type.DateTime * google.protobuf.Timestamp
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeDate {
    /// Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant.
    
    pub day: Option<i32>,
    /// Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day.
    
    pub month: Option<i32>,
    /// Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year.
    
    pub year: Option<i32>,
}

impl client::Part for GoogleTypeDate {}


/// Represents a time of day. The date and time zone are either not significant or are specified elsewhere. An API may choose to allow leap seconds. Related types are google.type.Date and `google.protobuf.Timestamp`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeTimeOfDay {
    /// Hours of day in 24 hour format. Should be from 0 to 23. An API may choose to allow the value "24:00:00" for scenarios like business closing time.
    
    pub hours: Option<i32>,
    /// Minutes of hour of day. Must be from 0 to 59.
    
    pub minutes: Option<i32>,
    /// Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999.
    
    pub nanos: Option<i32>,
    /// Seconds of minutes of the time. Must normally be from 0 to 59. An API may allow the value 60 if it allows leap-seconds.
    
    pub seconds: Option<i32>,
}

impl client::Part for GoogleTypeTimeOfDay {}


