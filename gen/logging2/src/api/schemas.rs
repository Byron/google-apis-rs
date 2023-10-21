use super::*;
/// Options that change functionality of a sink exporting data to BigQuery.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BigQueryOptions {
    /// Optional. Whether to use BigQuery's partition tables (https://cloud.google.com/bigquery/docs/partitioned-tables). By default, Cloud Logging creates dated tables based on the log entries' timestamps, e.g. syslog_20170523. With partitioned tables the date suffix is no longer present and special query syntax (https://cloud.google.com/bigquery/docs/querying-partitioned-tables) has to be used instead. In both cases, tables are sharded based on UTC timezone.
    #[serde(rename="usePartitionedTables")]
    
    pub use_partitioned_tables: Option<bool>,
    /// Output only. True if new timestamp column based partitioning is in use, false if legacy ingestion-time partitioning is in use.All new sinks will have this field set true and will use timestamp column based partitioning. If use_partitioned_tables is false, this value has no meaning and will be false. Legacy sinks using partitioned tables will have this field set to false.
    #[serde(rename="usesTimestampColumnPartitioning")]
    
    pub uses_timestamp_column_partitioning: Option<bool>,
}

impl client::Part for BigQueryOptions {}


/// BucketOptions describes the bucket boundaries used to create a histogram for the distribution. The buckets can be in a linear sequence, an exponential sequence, or each bucket can be specified explicitly. BucketOptions does not include the number of values in each bucket.A bucket has an inclusive lower bound and exclusive upper bound for the values that are counted for that bucket. The upper bound of a bucket must be strictly greater than the lower bound. The sequence of N buckets for a distribution consists of an underflow bucket (number 0), zero or more finite buckets (number 1 through N - 2) and an overflow bucket (number N - 1). The buckets are contiguous: the lower bound of bucket i (i > 0) is the same as the upper bound of bucket i - 1. The buckets span the whole range of finite values: lower bound of the underflow bucket is -infinity and the upper bound of the overflow bucket is +infinity. The finite buckets are so-called because both bounds are finite.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketOptions {
    /// The explicit buckets.
    #[serde(rename="explicitBuckets")]
    
    pub explicit_buckets: Option<Explicit>,
    /// The exponential buckets.
    #[serde(rename="exponentialBuckets")]
    
    pub exponential_buckets: Option<Exponential>,
    /// The linear bucket.
    #[serde(rename="linearBuckets")]
    
    pub linear_buckets: Option<Linear>,
}

impl client::Part for BucketOptions {}


/// The request message for Operations.CancelOperation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations cancel billing accounts](BillingAccountLocationOperationCancelCall) (request)
/// * [locations operations cancel folders](FolderLocationOperationCancelCall) (request)
/// * [operations cancel locations](LocationOperationCancelCall) (request)
/// * [locations operations cancel organizations](OrganizationLocationOperationCancelCall) (request)
/// * [locations operations cancel projects](ProjectLocationOperationCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelOperationRequest { _never_set: Option<bool> }

impl client::RequestValue for CancelOperationRequest {}


/// Describes the customer-managed encryption key (CMEK) settings associated with a project, folder, organization, billing account, or flexible resource.Note: CMEK for the Log Router can currently only be configured for Google Cloud organizations. Once configured, it applies to all projects and folders in the Google Cloud organization.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get cmek settings billing accounts](BillingAccountGetCmekSettingCall) (response)
/// * [get cmek settings folders](FolderGetCmekSettingCall) (response)
/// * [get cmek settings organizations](OrganizationGetCmekSettingCall) (response)
/// * [update cmek settings organizations](OrganizationUpdateCmekSettingCall) (request|response)
/// * [get cmek settings projects](ProjectGetCmekSettingCall) (response)
/// * [get cmek settings](MethodGetCmekSettingCall) (response)
/// * [update cmek settings](MethodUpdateCmekSettingCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CmekSettings {
    /// The resource name for the configured Cloud KMS key.KMS key name format: "projects/[PROJECT_ID]/locations/[LOCATION]/keyRings/[KEYRING]/cryptoKeys/[KEY]" For example:"projects/my-project/locations/us-central1/keyRings/my-ring/cryptoKeys/my-key"To enable CMEK for the Log Router, set this field to a valid kms_key_name for which the associated service account has the required cloudkms.cryptoKeyEncrypterDecrypter roles assigned for the key.The Cloud KMS key used by the Log Router can be updated by changing the kms_key_name to a new valid key name or disabled by setting the key name to an empty string. Encryption operations that are in progress will be completed with the key that was in use when they started. Decryption operations will be completed using the key that was used at the time of encryption unless access to that key has been revoked.To disable CMEK for the Log Router, set this field to an empty string.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.
    #[serde(rename="kmsKeyName")]
    
    pub kms_key_name: Option<String>,
    /// The CryptoKeyVersion resource name for the configured Cloud KMS key.KMS key name format: "projects/[PROJECT_ID]/locations/[LOCATION]/keyRings/[KEYRING]/cryptoKeys/[KEY]/cryptoKeyVersions/[VERSION]" For example:"projects/my-project/locations/us-central1/keyRings/my-ring/cryptoKeys/my-key/cryptoKeyVersions/1"This is a read-only field used to convey the specific configured CryptoKeyVersion of kms_key that has been configured. It will be populated in cases where the CMEK settings are bound to a single key version.
    #[serde(rename="kmsKeyVersionName")]
    
    pub kms_key_version_name: Option<String>,
    /// Output only. The resource name of the CMEK settings.
    
    pub name: Option<String>,
    /// Output only. The service account that will be used by the Log Router to access your Cloud KMS key.Before enabling CMEK for Log Router, you must first assign the cloudkms.cryptoKeyEncrypterDecrypter role to the service account that the Log Router will use to access your Cloud KMS key. Use GetCmekSettings to obtain the service account ID.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.
    #[serde(rename="serviceAccountId")]
    
    pub service_account_id: Option<String>,
}

impl client::RequestValue for CmekSettings {}
impl client::ResponseResult for CmekSettings {}


/// The parameters to CopyLogEntries.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [copy entries](EntryCopyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CopyLogEntriesRequest {
    /// Required. Destination to which to copy log entries.
    
    pub destination: Option<String>,
    /// Optional. A filter specifying which log entries to copy. The filter must be no more than 20k characters. An empty filter matches all log entries.
    
    pub filter: Option<String>,
    /// Required. Log bucket from which to copy log entries.For example:"projects/my-project/locations/global/buckets/my-source-bucket"
    
    pub name: Option<String>,
}

impl client::RequestValue for CopyLogEntriesRequest {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } 
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [exclusions delete billing accounts](BillingAccountExclusionDeleteCall) (response)
/// * [locations buckets views delete billing accounts](BillingAccountLocationBucketViewDeleteCall) (response)
/// * [locations buckets delete billing accounts](BillingAccountLocationBucketDeleteCall) (response)
/// * [locations buckets undelete billing accounts](BillingAccountLocationBucketUndeleteCall) (response)
/// * [locations operations cancel billing accounts](BillingAccountLocationOperationCancelCall) (response)
/// * [logs delete billing accounts](BillingAccountLogDeleteCall) (response)
/// * [sinks delete billing accounts](BillingAccountSinkDeleteCall) (response)
/// * [delete exclusions](ExclusionDeleteCall) (response)
/// * [exclusions delete folders](FolderExclusionDeleteCall) (response)
/// * [locations buckets views delete folders](FolderLocationBucketViewDeleteCall) (response)
/// * [locations buckets delete folders](FolderLocationBucketDeleteCall) (response)
/// * [locations buckets undelete folders](FolderLocationBucketUndeleteCall) (response)
/// * [locations operations cancel folders](FolderLocationOperationCancelCall) (response)
/// * [logs delete folders](FolderLogDeleteCall) (response)
/// * [sinks delete folders](FolderSinkDeleteCall) (response)
/// * [buckets views delete locations](LocationBucketViewDeleteCall) (response)
/// * [buckets delete locations](LocationBucketDeleteCall) (response)
/// * [buckets undelete locations](LocationBucketUndeleteCall) (response)
/// * [operations cancel locations](LocationOperationCancelCall) (response)
/// * [delete logs](LogDeleteCall) (response)
/// * [exclusions delete organizations](OrganizationExclusionDeleteCall) (response)
/// * [locations buckets views delete organizations](OrganizationLocationBucketViewDeleteCall) (response)
/// * [locations buckets delete organizations](OrganizationLocationBucketDeleteCall) (response)
/// * [locations buckets undelete organizations](OrganizationLocationBucketUndeleteCall) (response)
/// * [locations operations cancel organizations](OrganizationLocationOperationCancelCall) (response)
/// * [logs delete organizations](OrganizationLogDeleteCall) (response)
/// * [sinks delete organizations](OrganizationSinkDeleteCall) (response)
/// * [exclusions delete projects](ProjectExclusionDeleteCall) (response)
/// * [locations buckets views delete projects](ProjectLocationBucketViewDeleteCall) (response)
/// * [locations buckets delete projects](ProjectLocationBucketDeleteCall) (response)
/// * [locations buckets undelete projects](ProjectLocationBucketUndeleteCall) (response)
/// * [locations operations cancel projects](ProjectLocationOperationCancelCall) (response)
/// * [logs delete projects](ProjectLogDeleteCall) (response)
/// * [metrics delete projects](ProjectMetricDeleteCall) (response)
/// * [sinks delete projects](ProjectSinkDeleteCall) (response)
/// * [delete sinks](SinkDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// Specifies a set of buckets with arbitrary widths.There are size(bounds) + 1 (= N) buckets. Bucket i has the following boundaries:Upper bound (0 <= i < N-1): boundsi Lower bound (1 <= i < N); boundsi - 1The bounds field must contain at least one element. If bounds has only one element, then there are no finite buckets, and that single element is the common boundary of the overflow and underflow buckets.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Explicit {
    /// The values must be monotonically increasing.
    
    pub bounds: Option<Vec<f64>>,
}

impl client::Part for Explicit {}


/// Specifies an exponential sequence of buckets that have a width that is proportional to the value of the lower bound. Each bucket represents a constant relative uncertainty on a specific value in the bucket.There are num_finite_buckets + 2 (= N) buckets. Bucket i has the following boundaries:Upper bound (0 <= i < N-1): scale * (growth_factor ^ i). Lower bound (1 <= i < N): scale * (growth_factor ^ (i - 1)).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Exponential {
    /// Must be greater than 1.
    #[serde(rename="growthFactor")]
    
    pub growth_factor: Option<f64>,
    /// Must be greater than 0.
    #[serde(rename="numFiniteBuckets")]
    
    pub num_finite_buckets: Option<i32>,
    /// Must be greater than 0.
    
    pub scale: Option<f64>,
}

impl client::Part for Exponential {}


/// A common proto for logging HTTP requests. Only contains semantics defined by the HTTP specification. Product-specific logging information MUST be defined in a separate message.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HttpRequest {
    /// The number of HTTP response bytes inserted into cache. Set only when a cache fill was attempted.
    #[serde(rename="cacheFillBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub cache_fill_bytes: Option<i64>,
    /// Whether or not an entity was served from cache (with or without validation).
    #[serde(rename="cacheHit")]
    
    pub cache_hit: Option<bool>,
    /// Whether or not a cache lookup was attempted.
    #[serde(rename="cacheLookup")]
    
    pub cache_lookup: Option<bool>,
    /// Whether or not the response was validated with the origin server before being served from cache. This field is only meaningful if cache_hit is True.
    #[serde(rename="cacheValidatedWithOriginServer")]
    
    pub cache_validated_with_origin_server: Option<bool>,
    /// The request processing latency on the server, from the time the request was received until the response was sent.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub latency: Option<client::chrono::Duration>,
    /// Protocol used for the request. Examples: "HTTP/1.1", "HTTP/2", "websocket"
    
    pub protocol: Option<String>,
    /// The referer URL of the request, as defined in HTTP/1.1 Header Field Definitions (http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html).
    
    pub referer: Option<String>,
    /// The IP address (IPv4 or IPv6) of the client that issued the HTTP request. This field can include port information. Examples: "192.168.1.1", "10.0.0.1:80", "FE80::0202:B3FF:FE1E:8329".
    #[serde(rename="remoteIp")]
    
    pub remote_ip: Option<String>,
    /// The request method. Examples: "GET", "HEAD", "PUT", "POST".
    #[serde(rename="requestMethod")]
    
    pub request_method: Option<String>,
    /// The size of the HTTP request message in bytes, including the request headers and the request body.
    #[serde(rename="requestSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub request_size: Option<i64>,
    /// The scheme (http, https), the host name, the path and the query portion of the URL that was requested. Example: "http://example.com/some/info?color=red".
    #[serde(rename="requestUrl")]
    
    pub request_url: Option<String>,
    /// The size of the HTTP response message sent back to the client, in bytes, including the response headers and the response body.
    #[serde(rename="responseSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub response_size: Option<i64>,
    /// The IP address (IPv4 or IPv6) of the origin server that the request was sent to. This field can include port information. Examples: "192.168.1.1", "10.0.0.1:80", "FE80::0202:B3FF:FE1E:8329".
    #[serde(rename="serverIp")]
    
    pub server_ip: Option<String>,
    /// The response code indicating the status of response. Examples: 200, 404.
    
    pub status: Option<i32>,
    /// The user agent sent by the client. Example: "Mozilla/4.0 (compatible; MSIE 6.0; Windows 98; Q312461; .NET CLR 1.0.3705)".
    #[serde(rename="userAgent")]
    
    pub user_agent: Option<String>,
}

impl client::Part for HttpRequest {}


/// Configuration for an indexed field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IndexConfig {
    /// Output only. The timestamp when the index was last modified.This is used to return the timestamp, and will be ignored if supplied during update.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. The LogEntry field path to index.Note that some paths are automatically indexed, and other paths are not eligible for indexing. See indexing documentation( https://cloud.google.com/logging/docs/view/advanced-queries#indexed-fields) for details.For example: jsonPayload.request.status
    #[serde(rename="fieldPath")]
    
    pub field_path: Option<String>,
    /// Required. The type of data in this index.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for IndexConfig {}


/// A description of a label.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LabelDescriptor {
    /// A human-readable description for the label.
    
    pub description: Option<String>,
    /// The label key.
    
    pub key: Option<String>,
    /// The type of data that can be assigned to the label.
    #[serde(rename="valueType")]
    
    pub value_type: Option<String>,
}

impl client::Part for LabelDescriptor {}


/// Specifies a linear sequence of buckets that all have the same width (except overflow and underflow). Each bucket represents a constant absolute uncertainty on the specific value in the bucket.There are num_finite_buckets + 2 (= N) buckets. Bucket i has the following boundaries:Upper bound (0 <= i < N-1): offset + (width * i). Lower bound (1 <= i < N): offset + (width * (i - 1)).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Linear {
    /// Must be greater than 0.
    #[serde(rename="numFiniteBuckets")]
    
    pub num_finite_buckets: Option<i32>,
    /// Lower bound of the first bucket.
    
    pub offset: Option<f64>,
    /// Must be greater than 0.
    
    pub width: Option<f64>,
}

impl client::Part for Linear {}


/// The response from ListBuckets.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations buckets list billing accounts](BillingAccountLocationBucketListCall) (response)
/// * [locations buckets list folders](FolderLocationBucketListCall) (response)
/// * [buckets list locations](LocationBucketListCall) (response)
/// * [locations buckets list organizations](OrganizationLocationBucketListCall) (response)
/// * [locations buckets list projects](ProjectLocationBucketListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListBucketsResponse {
    /// A list of buckets.
    
    pub buckets: Option<Vec<LogBucket>>,
    /// If there might be more results than appear in this response, then nextPageToken is included. To get the next set of results, call the same method again using the value of nextPageToken as pageToken.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListBucketsResponse {}


/// Result returned from ListExclusions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [exclusions list billing accounts](BillingAccountExclusionListCall) (response)
/// * [list exclusions](ExclusionListCall) (response)
/// * [exclusions list folders](FolderExclusionListCall) (response)
/// * [exclusions list organizations](OrganizationExclusionListCall) (response)
/// * [exclusions list projects](ProjectExclusionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListExclusionsResponse {
    /// A list of exclusions.
    
    pub exclusions: Option<Vec<LogExclusion>>,
    /// If there might be more results than appear in this response, then nextPageToken is included. To get the next set of results, call the same method again using the value of nextPageToken as pageToken.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListExclusionsResponse {}


/// The response message for Locations.ListLocations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations list billing accounts](BillingAccountLocationListCall) (response)
/// * [locations list folders](FolderLocationListCall) (response)
/// * [list locations](LocationListCall) (response)
/// * [locations list organizations](OrganizationLocationListCall) (response)
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


/// The parameters to ListLogEntries.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list entries](EntryListCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListLogEntriesRequest {
    /// Optional. Only log entries that match the filter are returned. An empty filter matches all log entries in the resources listed in resource_names. Referencing a parent resource that is not listed in resource_names will cause the filter to return no results. The maximum length of a filter is 20,000 characters.
    
    pub filter: Option<String>,
    /// Optional. How the results should be sorted. Presently, the only permitted values are "timestamp asc" (default) and "timestamp desc". The first option returns entries in order of increasing values of LogEntry.timestamp (oldest first), and the second option returns entries in order of decreasing timestamps (newest first). Entries with equal timestamps are returned in order of their insert_id values.
    #[serde(rename="orderBy")]
    
    pub order_by: Option<String>,
    /// Optional. The maximum number of results to return from this request. Default is 50. If the value is negative or exceeds 1000, the request is rejected. The presence of next_page_token in the response indicates that more results might be available.
    #[serde(rename="pageSize")]
    
    pub page_size: Option<i32>,
    /// Optional. If present, then retrieve the next batch of results from the preceding call to this method. page_token must be the value of next_page_token from the previous response. The values of other method parameters should be identical to those in the previous call.
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
    /// Optional. Deprecated. Use resource_names instead. One or more project identifiers or project numbers from which to retrieve log entries. Example: "my-project-1A".
    #[serde(rename="projectIds")]
    
    pub project_ids: Option<Vec<String>>,
    /// Required. Names of one or more parent resources from which to retrieve log entries: projects/[PROJECT_ID] organizations/[ORGANIZATION_ID] billingAccounts/[BILLING_ACCOUNT_ID] folders/[FOLDER_ID]May alternatively be one or more views: projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/views/[VIEW_ID] organizations/[ORGANIZATION_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/views/[VIEW_ID] billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/views/[VIEW_ID] folders/[FOLDER_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/views/[VIEW_ID]Projects listed in the project_ids field are added to this list.
    #[serde(rename="resourceNames")]
    
    pub resource_names: Option<Vec<String>>,
}

impl client::RequestValue for ListLogEntriesRequest {}


/// Result returned from ListLogEntries.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list entries](EntryListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListLogEntriesResponse {
    /// A list of log entries. If entries is empty, nextPageToken may still be returned, indicating that more entries may exist. See nextPageToken for more information.
    
    pub entries: Option<Vec<LogEntry>>,
    /// If there might be more results than those appearing in this response, then nextPageToken is included. To get the next set of results, call this method again using the value of nextPageToken as pageToken.If a value for next_page_token appears and the entries field is empty, it means that the search found no log entries so far but it did not have time to search all the possible log entries. Retry the method with this value for page_token to continue the search. Alternatively, consider speeding up the search by changing your filter to specify a single log name or resource type, or to narrow the time range of the search.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListLogEntriesResponse {}


/// Result returned from ListLogMetrics.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [metrics list projects](ProjectMetricListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListLogMetricsResponse {
    /// A list of logs-based metrics.
    
    pub metrics: Option<Vec<LogMetric>>,
    /// If there might be more results than appear in this response, then nextPageToken is included. To get the next set of results, call this method again using the value of nextPageToken as pageToken.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListLogMetricsResponse {}


/// Result returned from ListLogs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations buckets views logs list billing accounts](BillingAccountLocationBucketViewLogListCall) (response)
/// * [logs list billing accounts](BillingAccountLogListCall) (response)
/// * [locations buckets views logs list folders](FolderLocationBucketViewLogListCall) (response)
/// * [logs list folders](FolderLogListCall) (response)
/// * [list logs](LogListCall) (response)
/// * [locations buckets views logs list organizations](OrganizationLocationBucketViewLogListCall) (response)
/// * [logs list organizations](OrganizationLogListCall) (response)
/// * [locations buckets views logs list projects](ProjectLocationBucketViewLogListCall) (response)
/// * [logs list projects](ProjectLogListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListLogsResponse {
    /// A list of log names. For example, "projects/my-project/logs/syslog" or "organizations/123/logs/cloudresourcemanager.googleapis.com%2Factivity".
    #[serde(rename="logNames")]
    
    pub log_names: Option<Vec<String>>,
    /// If there might be more results than those appearing in this response, then nextPageToken is included. To get the next set of results, call this method again using the value of nextPageToken as pageToken.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListLogsResponse {}


/// Result returned from ListMonitoredResourceDescriptors.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list monitored resource descriptors](MonitoredResourceDescriptorListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListMonitoredResourceDescriptorsResponse {
    /// If there might be more results than those appearing in this response, then nextPageToken is included. To get the next set of results, call this method again using the value of nextPageToken as pageToken.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of resource descriptors.
    #[serde(rename="resourceDescriptors")]
    
    pub resource_descriptors: Option<Vec<MonitoredResourceDescriptor>>,
}

impl client::ResponseResult for ListMonitoredResourceDescriptorsResponse {}


/// The response message for Operations.ListOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations list billing accounts](BillingAccountLocationOperationListCall) (response)
/// * [locations operations list folders](FolderLocationOperationListCall) (response)
/// * [operations list locations](LocationOperationListCall) (response)
/// * [locations operations list organizations](OrganizationLocationOperationListCall) (response)
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


/// Result returned from ListSinks.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sinks list billing accounts](BillingAccountSinkListCall) (response)
/// * [sinks list folders](FolderSinkListCall) (response)
/// * [sinks list organizations](OrganizationSinkListCall) (response)
/// * [sinks list projects](ProjectSinkListCall) (response)
/// * [list sinks](SinkListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListSinksResponse {
    /// If there might be more results than appear in this response, then nextPageToken is included. To get the next set of results, call the same method again using the value of nextPageToken as pageToken.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of sinks.
    
    pub sinks: Option<Vec<LogSink>>,
}

impl client::ResponseResult for ListSinksResponse {}


/// The response from ListViews.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations buckets views list billing accounts](BillingAccountLocationBucketViewListCall) (response)
/// * [locations buckets views list folders](FolderLocationBucketViewListCall) (response)
/// * [buckets views list locations](LocationBucketViewListCall) (response)
/// * [locations buckets views list organizations](OrganizationLocationBucketViewListCall) (response)
/// * [locations buckets views list projects](ProjectLocationBucketViewListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListViewsResponse {
    /// If there might be more results than appear in this response, then nextPageToken is included. To get the next set of results, call the same method again using the value of nextPageToken as pageToken.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of views.
    
    pub views: Option<Vec<LogView>>,
}

impl client::ResponseResult for ListViewsResponse {}


/// A resource that represents Google Cloud Platform location.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations get billing accounts](BillingAccountLocationGetCall) (response)
/// * [locations get folders](FolderLocationGetCall) (response)
/// * [buckets views create locations](LocationBucketViewCreateCall) (none)
/// * [buckets views delete locations](LocationBucketViewDeleteCall) (none)
/// * [buckets views get locations](LocationBucketViewGetCall) (none)
/// * [buckets views list locations](LocationBucketViewListCall) (none)
/// * [buckets views patch locations](LocationBucketViewPatchCall) (none)
/// * [buckets create locations](LocationBucketCreateCall) (none)
/// * [buckets delete locations](LocationBucketDeleteCall) (none)
/// * [buckets get locations](LocationBucketGetCall) (none)
/// * [buckets list locations](LocationBucketListCall) (none)
/// * [buckets patch locations](LocationBucketPatchCall) (none)
/// * [buckets undelete locations](LocationBucketUndeleteCall) (none)
/// * [operations cancel locations](LocationOperationCancelCall) (none)
/// * [operations get locations](LocationOperationGetCall) (none)
/// * [operations list locations](LocationOperationListCall) (none)
/// * [get locations](LocationGetCall) (response)
/// * [list locations](LocationListCall) (none)
/// * [locations get organizations](OrganizationLocationGetCall) (response)
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

impl client::Resource for Location {}
impl client::ResponseResult for Location {}


/// Describes a repository in which log entries are stored.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations buckets create billing accounts](BillingAccountLocationBucketCreateCall) (request|response)
/// * [locations buckets get billing accounts](BillingAccountLocationBucketGetCall) (response)
/// * [locations buckets patch billing accounts](BillingAccountLocationBucketPatchCall) (request|response)
/// * [locations buckets create folders](FolderLocationBucketCreateCall) (request|response)
/// * [locations buckets get folders](FolderLocationBucketGetCall) (response)
/// * [locations buckets patch folders](FolderLocationBucketPatchCall) (request|response)
/// * [buckets create locations](LocationBucketCreateCall) (request|response)
/// * [buckets get locations](LocationBucketGetCall) (response)
/// * [buckets patch locations](LocationBucketPatchCall) (request|response)
/// * [locations buckets create organizations](OrganizationLocationBucketCreateCall) (request|response)
/// * [locations buckets get organizations](OrganizationLocationBucketGetCall) (response)
/// * [locations buckets patch organizations](OrganizationLocationBucketPatchCall) (request|response)
/// * [locations buckets create projects](ProjectLocationBucketCreateCall) (request|response)
/// * [locations buckets get projects](ProjectLocationBucketGetCall) (response)
/// * [locations buckets patch projects](ProjectLocationBucketPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LogBucket {
    /// The CMEK settings of the log bucket. If present, new log entries written to this log bucket are encrypted using the CMEK key provided in this configuration. If a log bucket has CMEK settings, the CMEK settings cannot be disabled later by updating the log bucket. Changing the KMS key is allowed.
    #[serde(rename="cmekSettings")]
    
    pub cmek_settings: Option<CmekSettings>,
    /// Output only. The creation timestamp of the bucket. This is not set for any of the default buckets.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Describes this bucket.
    
    pub description: Option<String>,
    /// A list of indexed fields and related configuration data.
    #[serde(rename="indexConfigs")]
    
    pub index_configs: Option<Vec<IndexConfig>>,
    /// Output only. The bucket lifecycle state.
    #[serde(rename="lifecycleState")]
    
    pub lifecycle_state: Option<String>,
    /// Whether the bucket is locked.The retention period on a locked bucket cannot be changed. Locked buckets may only be deleted if they are empty.
    
    pub locked: Option<bool>,
    /// Output only. The resource name of the bucket.For example:projects/my-project/locations/global/buckets/my-bucketFor a list of supported locations, see Supported Regions (https://cloud.google.com/logging/docs/region-support)For the location of global it is unspecified where log entries are actually stored.After a bucket has been created, the location cannot be changed.
    
    pub name: Option<String>,
    /// Log entry field paths that are denied access in this bucket.The following fields and their children are eligible: textPayload, jsonPayload, protoPayload, httpRequest, labels, sourceLocation.Restricting a repeated field will restrict all values. Adding a parent will block all child fields. (e.g. foo.bar will block foo.bar.baz)
    #[serde(rename="restrictedFields")]
    
    pub restricted_fields: Option<Vec<String>>,
    /// Logs will be retained by default for this amount of time, after which they will automatically be deleted. The minimum retention period is 1 day. If this value is set to zero at bucket creation time, the default time of 30 days will be used.
    #[serde(rename="retentionDays")]
    
    pub retention_days: Option<i32>,
    /// Output only. The last update timestamp of the bucket.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for LogBucket {}
impl client::ResponseResult for LogBucket {}


/// An individual entry in a log.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LogEntry {
    /// Optional. Information about the HTTP request associated with this log entry, if applicable.
    #[serde(rename="httpRequest")]
    
    pub http_request: Option<HttpRequest>,
    /// Optional. A unique identifier for the log entry. If you provide a value, then Logging considers other log entries in the same project, with the same timestamp, and with the same insert_id to be duplicates which are removed in a single query result. However, there are no guarantees of de-duplication in the export of logs.If the insert_id is omitted when writing a log entry, the Logging API assigns its own unique identifier in this field.In queries, the insert_id is also used to order log entries that have the same log_name and timestamp values.
    #[serde(rename="insertId")]
    
    pub insert_id: Option<String>,
    /// The log entry payload, represented as a structure that is expressed as a JSON object.
    #[serde(rename="jsonPayload")]
    
    pub json_payload: Option<HashMap<String, json::Value>>,
    /// Optional. A map of key, value pairs that provides additional information about the log entry. The labels can be user-defined or system-defined.User-defined labels are arbitrary key, value pairs that you can use to classify logs.System-defined labels are defined by GCP services for platform logs. They have two components - a service namespace component and the attribute name. For example: compute.googleapis.com/resource_name.Cloud Logging truncates label keys that exceed 512 B and label values that exceed 64 KB upon their associated log entry being written. The truncation is indicated by an ellipsis at the end of the character string.
    
    pub labels: Option<HashMap<String, String>>,
    /// Required. The resource name of the log to which this log entry belongs: "projects/[PROJECT_ID]/logs/[LOG_ID]" "organizations/[ORGANIZATION_ID]/logs/[LOG_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/logs/[LOG_ID]" "folders/[FOLDER_ID]/logs/[LOG_ID]" A project number may be used in place of PROJECT_ID. The project number is translated to its corresponding PROJECT_ID internally and the log_name field will contain PROJECT_ID in queries and exports.[LOG_ID] must be URL-encoded within log_name. Example: "organizations/1234567890/logs/cloudresourcemanager.googleapis.com%2Factivity".[LOG_ID] must be less than 512 characters long and can only include the following characters: upper and lower case alphanumeric characters, forward-slash, underscore, hyphen, and period.For backward compatibility, if log_name begins with a forward-slash, such as /projects/..., then the log entry is ingested as usual, but the forward-slash is removed. Listing the log entry will not show the leading slash and filtering for a log name with a leading slash will never return any results.
    #[serde(rename="logName")]
    
    pub log_name: Option<String>,
    /// Output only. Deprecated. This field is not used by Logging. Any value written to it is cleared.
    
    pub metadata: Option<MonitoredResourceMetadata>,
    /// Optional. Information about an operation associated with the log entry, if applicable.
    
    pub operation: Option<LogEntryOperation>,
    /// The log entry payload, represented as a protocol buffer. Some Google Cloud Platform services use this field for their log entry payloads.The following protocol buffer types are supported; user-defined types are not supported:"type.googleapis.com/google.cloud.audit.AuditLog" "type.googleapis.com/google.appengine.logging.v1.RequestLog"
    #[serde(rename="protoPayload")]
    
    pub proto_payload: Option<HashMap<String, json::Value>>,
    /// Output only. The time the log entry was received by Logging.
    #[serde(rename="receiveTimestamp")]
    
    pub receive_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. The monitored resource that produced this log entry.Example: a log entry that reports a database error would be associated with the monitored resource designating the particular database that reported the error.
    
    pub resource: Option<MonitoredResource>,
    /// Optional. The severity of the log entry. The default value is LogSeverity.DEFAULT.
    
    pub severity: Option<String>,
    /// Optional. Source code location information associated with the log entry, if any.
    #[serde(rename="sourceLocation")]
    
    pub source_location: Option<LogEntrySourceLocation>,
    /// Optional. The ID of the Cloud Trace (https://cloud.google.com/trace) span associated with the current operation in which the log is being written. For example, if a span has the REST resource name of "projects/some-project/traces/some-trace/spans/some-span-id", then the span_id field is "some-span-id".A Span (https://cloud.google.com/trace/docs/reference/v2/rest/v2/projects.traces/batchWrite#Span) represents a single operation within a trace. Whereas a trace may involve multiple different microservices running on multiple different machines, a span generally corresponds to a single logical operation being performed in a single instance of a microservice on one specific machine. Spans are the nodes within the tree that is a trace.Applications that are instrumented for tracing (https://cloud.google.com/trace/docs/setup) will generally assign a new, unique span ID on each incoming request. It is also common to create and record additional spans corresponding to internal processing elements as well as issuing requests to dependencies.The span ID is expected to be a 16-character, hexadecimal encoding of an 8-byte array and should not be zero. It should be unique within the trace and should, ideally, be generated in a manner that is uniformly random.Example values: 000000000000004a 7a2190356c3fc94b 0000f00300090021 d39223e101960076
    #[serde(rename="spanId")]
    
    pub span_id: Option<String>,
    /// Optional. Information indicating this LogEntry is part of a sequence of multiple log entries split from a single LogEntry.
    
    pub split: Option<LogSplit>,
    /// The log entry payload, represented as a Unicode string (UTF-8).
    #[serde(rename="textPayload")]
    
    pub text_payload: Option<String>,
    /// Optional. The time the event described by the log entry occurred. This time is used to compute the log entry's age and to enforce the logs retention period. If this field is omitted in a new log entry, then Logging assigns it the current time. Timestamps have nanosecond accuracy, but trailing zeros in the fractional seconds might be omitted when the timestamp is displayed.Incoming log entries must have timestamps that don't exceed the logs retention period (https://cloud.google.com/logging/quotas#logs_retention_periods) in the past, and that don't exceed 24 hours in the future. Log entries outside those time boundaries aren't ingested by Logging.
    
    pub timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. The REST resource name of the trace being written to Cloud Trace (https://cloud.google.com/trace) in association with this log entry. For example, if your trace data is stored in the Cloud project "my-trace-project" and if the service that is creating the log entry receives a trace header that includes the trace ID "12345", then the service should use "projects/my-tracing-project/traces/12345".The trace field provides the link between logs and traces. By using this field, you can navigate from a log entry to a trace.
    
    pub trace: Option<String>,
    /// Optional. The sampling decision of the trace associated with the log entry.True means that the trace resource name in the trace field was sampled for storage in a trace backend. False means that the trace was not sampled for storage when this log entry was written, or the sampling decision was unknown at the time. A non-sampled trace value is still useful as a request correlation identifier. The default is False.
    #[serde(rename="traceSampled")]
    
    pub trace_sampled: Option<bool>,
}

impl client::Part for LogEntry {}


/// Additional information about a potentially long-running operation with which a log entry is associated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LogEntryOperation {
    /// Optional. Set this to True if this is the first log entry in the operation.
    
    pub first: Option<bool>,
    /// Optional. An arbitrary operation identifier. Log entries with the same identifier are assumed to be part of the same operation.
    
    pub id: Option<String>,
    /// Optional. Set this to True if this is the last log entry in the operation.
    
    pub last: Option<bool>,
    /// Optional. An arbitrary producer identifier. The combination of id and producer must be globally unique. Examples for producer: "MyDivision.MyBigCompany.com", "github.com/MyProject/MyApplication".
    
    pub producer: Option<String>,
}

impl client::Part for LogEntryOperation {}


/// Additional information about the source code location that produced the log entry.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LogEntrySourceLocation {
    /// Optional. Source file name. Depending on the runtime environment, this might be a simple name or a fully-qualified name.
    
    pub file: Option<String>,
    /// Optional. Human-readable name of the function or method being invoked, with optional context such as the class or package name. This information may be used in contexts such as the logs viewer, where a file and line number are less meaningful. The format can vary by language. For example: qual.if.ied.Class.method (Java), dir/package.func (Go), function (Python).
    
    pub function: Option<String>,
    /// Optional. Line within the source file. 1-based; 0 indicates no line number available.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub line: Option<i64>,
}

impl client::Part for LogEntrySourceLocation {}


/// Specifies a set of log entries that are filtered out by a sink. If your Google Cloud resource receives a large volume of log entries, you can use exclusions to reduce your chargeable logs. Note that exclusions on organization-level and folder-level sinks don’t apply to child resources. Note also that you cannot modify the \_Required sink or exclude logs from it.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [exclusions create billing accounts](BillingAccountExclusionCreateCall) (request|response)
/// * [exclusions get billing accounts](BillingAccountExclusionGetCall) (response)
/// * [exclusions patch billing accounts](BillingAccountExclusionPatchCall) (request|response)
/// * [create exclusions](ExclusionCreateCall) (request|response)
/// * [get exclusions](ExclusionGetCall) (response)
/// * [patch exclusions](ExclusionPatchCall) (request|response)
/// * [exclusions create folders](FolderExclusionCreateCall) (request|response)
/// * [exclusions get folders](FolderExclusionGetCall) (response)
/// * [exclusions patch folders](FolderExclusionPatchCall) (request|response)
/// * [exclusions create organizations](OrganizationExclusionCreateCall) (request|response)
/// * [exclusions get organizations](OrganizationExclusionGetCall) (response)
/// * [exclusions patch organizations](OrganizationExclusionPatchCall) (request|response)
/// * [exclusions create projects](ProjectExclusionCreateCall) (request|response)
/// * [exclusions get projects](ProjectExclusionGetCall) (response)
/// * [exclusions patch projects](ProjectExclusionPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LogExclusion {
    /// Output only. The creation timestamp of the exclusion.This field may not be present for older exclusions.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. A description of this exclusion.
    
    pub description: Option<String>,
    /// Optional. If set to True, then this exclusion is disabled and it does not exclude any log entries. You can update an exclusion to change the value of this field.
    
    pub disabled: Option<bool>,
    /// Required. An advanced logs filter (https://cloud.google.com/logging/docs/view/advanced-queries) that matches the log entries to be excluded. By using the sample function (https://cloud.google.com/logging/docs/view/advanced-queries#sample), you can exclude less than 100% of the matching log entries.For example, the following query matches 99% of low-severity log entries from Google Cloud Storage buckets:resource.type=gcs_bucket severity<ERROR sample(insertId, 0.99)
    
    pub filter: Option<String>,
    /// Required. A client-assigned identifier, such as "load-balancer-exclusion". Identifiers are limited to 100 characters and can include only letters, digits, underscores, hyphens, and periods. First character has to be alphanumeric.
    
    pub name: Option<String>,
    /// Output only. The last update timestamp of the exclusion.This field may not be present for older exclusions.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for LogExclusion {}
impl client::ResponseResult for LogExclusion {}


/// Describes a logs-based metric. The value of the metric is the number of log entries that match a logs filter in a given time interval.Logs-based metrics can also be used to extract values from logs and create a distribution of the values. The distribution records the statistics of the extracted values along with an optional histogram of the values as specified by the bucket options.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [metrics create projects](ProjectMetricCreateCall) (request|response)
/// * [metrics get projects](ProjectMetricGetCall) (response)
/// * [metrics update projects](ProjectMetricUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LogMetric {
    /// Optional. The resource name of the Log Bucket that owns the Log Metric. Only Log Buckets in projects are supported. The bucket has to be in the same project as the metric.For example:projects/my-project/locations/global/buckets/my-bucketIf empty, then the Log Metric is considered a non-Bucket Log Metric.
    #[serde(rename="bucketName")]
    
    pub bucket_name: Option<String>,
    /// Optional. The bucket_options are required when the logs-based metric is using a DISTRIBUTION value type and it describes the bucket boundaries used to create a histogram of the extracted values.
    #[serde(rename="bucketOptions")]
    
    pub bucket_options: Option<BucketOptions>,
    /// Output only. The creation timestamp of the metric.This field may not be present for older metrics.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. A description of this metric, which is used in documentation. The maximum length of the description is 8000 characters.
    
    pub description: Option<String>,
    /// Optional. If set to True, then this metric is disabled and it does not generate any points.
    
    pub disabled: Option<bool>,
    /// Required. An advanced logs filter (https://cloud.google.com/logging/docs/view/advanced_filters) which is used to match log entries. Example: "resource.type=gae_app AND severity>=ERROR" The maximum length of the filter is 20000 characters.
    
    pub filter: Option<String>,
    /// Optional. A map from a label key string to an extractor expression which is used to extract data from a log entry field and assign as the label value. Each label key specified in the LabelDescriptor must have an associated extractor expression in this map. The syntax of the extractor expression is the same as for the value_extractor field.The extracted value is converted to the type defined in the label descriptor. If either the extraction or the type conversion fails, the label will have a default value. The default value for a string label is an empty string, for an integer label its 0, and for a boolean label its false.Note that there are upper bounds on the maximum number of labels and the number of active time series that are allowed in a project.
    #[serde(rename="labelExtractors")]
    
    pub label_extractors: Option<HashMap<String, String>>,
    /// Optional. The metric descriptor associated with the logs-based metric. If unspecified, it uses a default metric descriptor with a DELTA metric kind, INT64 value type, with no labels and a unit of "1". Such a metric counts the number of log entries matching the filter expression.The name, type, and description fields in the metric_descriptor are output only, and is constructed using the name and description field in the LogMetric.To create a logs-based metric that records a distribution of log values, a DELTA metric kind with a DISTRIBUTION value type must be used along with a value_extractor expression in the LogMetric.Each label in the metric descriptor must have a matching label name as the key and an extractor expression as the value in the label_extractors map.The metric_kind and value_type fields in the metric_descriptor cannot be updated once initially configured. New labels can be added in the metric_descriptor, but existing labels cannot be modified except for their description.
    #[serde(rename="metricDescriptor")]
    
    pub metric_descriptor: Option<MetricDescriptor>,
    /// Required. The client-assigned metric identifier. Examples: "error_count", "nginx/requests".Metric identifiers are limited to 100 characters and can include only the following characters: A-Z, a-z, 0-9, and the special characters _-.,+!*',()%/. The forward-slash character (/) denotes a hierarchy of name pieces, and it cannot be the first character of the name.This field is the [METRIC_ID] part of a metric resource name in the format "projects/PROJECT_ID/metrics/METRIC_ID". Example: If the resource name of a metric is "projects/my-project/metrics/nginx%2Frequests", this field's value is "nginx/requests".
    
    pub name: Option<String>,
    /// Output only. The last update timestamp of the metric.This field may not be present for older metrics.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. A value_extractor is required when using a distribution logs-based metric to extract the values to record from a log entry. Two functions are supported for value extraction: EXTRACT(field) or REGEXP_EXTRACT(field, regex). The arguments are: field: The name of the log entry field from which the value is to be extracted. regex: A regular expression using the Google RE2 syntax (https://github.com/google/re2/wiki/Syntax) with a single capture group to extract data from the specified log entry field. The value of the field is converted to a string before applying the regex. It is an error to specify a regex that does not include exactly one capture group.The result of the extraction must be convertible to a double type, as the distribution always records double values. If either the extraction or the conversion to double fails, then those values are not recorded in the distribution.Example: REGEXP_EXTRACT(jsonPayload.request, ".*quantity=(\d+).*")
    #[serde(rename="valueExtractor")]
    
    pub value_extractor: Option<String>,
    /// Deprecated. The API version that created or updated this metric. The v2 format is used by default and cannot be changed.
    
    pub version: Option<String>,
}

impl client::RequestValue for LogMetric {}
impl client::ResponseResult for LogMetric {}


/// Describes a sink used to export log entries to one of the following destinations in any project: a Cloud Storage bucket, a BigQuery dataset, a Pub/Sub topic or a Cloud Logging log bucket. A logs filter controls which log entries are exported. The sink must be created within a project, organization, billing account, or folder.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sinks create billing accounts](BillingAccountSinkCreateCall) (request|response)
/// * [sinks get billing accounts](BillingAccountSinkGetCall) (response)
/// * [sinks patch billing accounts](BillingAccountSinkPatchCall) (request|response)
/// * [sinks update billing accounts](BillingAccountSinkUpdateCall) (request|response)
/// * [sinks create folders](FolderSinkCreateCall) (request|response)
/// * [sinks get folders](FolderSinkGetCall) (response)
/// * [sinks patch folders](FolderSinkPatchCall) (request|response)
/// * [sinks update folders](FolderSinkUpdateCall) (request|response)
/// * [sinks create organizations](OrganizationSinkCreateCall) (request|response)
/// * [sinks get organizations](OrganizationSinkGetCall) (response)
/// * [sinks patch organizations](OrganizationSinkPatchCall) (request|response)
/// * [sinks update organizations](OrganizationSinkUpdateCall) (request|response)
/// * [sinks create projects](ProjectSinkCreateCall) (request|response)
/// * [sinks get projects](ProjectSinkGetCall) (response)
/// * [sinks patch projects](ProjectSinkPatchCall) (request|response)
/// * [sinks update projects](ProjectSinkUpdateCall) (request|response)
/// * [create sinks](SinkCreateCall) (request|response)
/// * [get sinks](SinkGetCall) (response)
/// * [update sinks](SinkUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LogSink {
    /// Optional. Options that affect sinks exporting data to BigQuery.
    #[serde(rename="bigqueryOptions")]
    
    pub bigquery_options: Option<BigQueryOptions>,
    /// Output only. The creation timestamp of the sink.This field may not be present for older sinks.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. A description of this sink.The maximum length of the description is 8000 characters.
    
    pub description: Option<String>,
    /// Required. The export destination: "storage.googleapis.com/[GCS_BUCKET]" "bigquery.googleapis.com/projects/[PROJECT_ID]/datasets/[DATASET]" "pubsub.googleapis.com/projects/[PROJECT_ID]/topics/[TOPIC_ID]" The sink's writer_identity, set when the sink is created, must have permission to write to the destination or else the log entries are not exported. For more information, see Exporting Logs with Sinks (https://cloud.google.com/logging/docs/api/tasks/exporting-logs).
    
    pub destination: Option<String>,
    /// Optional. If set to true, then this sink is disabled and it does not export any log entries.
    
    pub disabled: Option<bool>,
    /// Optional. Log entries that match any of these exclusion filters will not be exported.If a log entry is matched by both filter and one of exclusion_filters it will not be exported.
    
    pub exclusions: Option<Vec<LogExclusion>>,
    /// Optional. An advanced logs filter (https://cloud.google.com/logging/docs/view/advanced-queries). The only exported log entries are those that are in the resource owning the sink and that match the filter.For example:logName="projects/[PROJECT_ID]/logs/[LOG_ID]" AND severity>=ERROR
    
    pub filter: Option<String>,
    /// Optional. This field applies only to sinks owned by organizations and folders. If the field is false, the default, only the logs owned by the sink's parent resource are available for export. If the field is true, then log entries from all the projects, folders, and billing accounts contained in the sink's parent resource are also available for export. Whether a particular log entry from the children is exported depends on the sink's filter expression.For example, if this field is true, then the filter resource.type=gce_instance would export all Compute Engine VM instance log entries from all projects in the sink's parent.To only export entries from certain child projects, filter on the project part of the log name:logName:("projects/test-project1/" OR "projects/test-project2/") AND resource.type=gce_instance
    #[serde(rename="includeChildren")]
    
    pub include_children: Option<bool>,
    /// Required. The client-assigned sink identifier, unique within the project.For example: "my-syslog-errors-to-pubsub". Sink identifiers are limited to 100 characters and can include only the following characters: upper and lower-case alphanumeric characters, underscores, hyphens, and periods. First character has to be alphanumeric.
    
    pub name: Option<String>,
    /// Deprecated. This field is unused.
    #[serde(rename="outputVersionFormat")]
    
    pub output_version_format: Option<String>,
    /// Output only. The last update timestamp of the sink.This field may not be present for older sinks.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. An IAM identity—a service account or group—under which Cloud Logging writes the exported log entries to the sink's destination. This field is either set by specifying custom_writer_identity or set automatically by sinks.create and sinks.update based on the value of unique_writer_identity in those methods.Until you grant this identity write-access to the destination, log entry exports from this sink will fail. For more information, see Granting Access for a Resource (https://cloud.google.com/iam/docs/granting-roles-to-service-accounts#granting_access_to_a_service_account_for_a_resource). Consult the destination service's documentation to determine the appropriate IAM roles to assign to the identity.Sinks that have a destination that is a log bucket in the same project as the sink cannot have a writer_identity and no additional permissions are required.
    #[serde(rename="writerIdentity")]
    
    pub writer_identity: Option<String>,
}

impl client::RequestValue for LogSink {}
impl client::ResponseResult for LogSink {}


/// Additional information used to correlate multiple log entries. Used when a single LogEntry would exceed the Google Cloud Logging size limit and is split across multiple log entries.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LogSplit {
    /// The index of this LogEntry in the sequence of split log entries. Log entries are given |index| values 0, 1, ..., n-1 for a sequence of n log entries.
    
    pub index: Option<i32>,
    /// The total number of log entries that the original LogEntry was split into.
    #[serde(rename="totalSplits")]
    
    pub total_splits: Option<i32>,
    /// A globally unique identifier for all log entries in a sequence of split log entries. All log entries with the same |LogSplit.uid| are assumed to be part of the same sequence of split log entries.
    
    pub uid: Option<String>,
}

impl client::Part for LogSplit {}


/// Describes a view over log entries in a bucket.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations buckets views create billing accounts](BillingAccountLocationBucketViewCreateCall) (request|response)
/// * [locations buckets views get billing accounts](BillingAccountLocationBucketViewGetCall) (response)
/// * [locations buckets views patch billing accounts](BillingAccountLocationBucketViewPatchCall) (request|response)
/// * [locations buckets views create folders](FolderLocationBucketViewCreateCall) (request|response)
/// * [locations buckets views get folders](FolderLocationBucketViewGetCall) (response)
/// * [locations buckets views patch folders](FolderLocationBucketViewPatchCall) (request|response)
/// * [buckets views create locations](LocationBucketViewCreateCall) (request|response)
/// * [buckets views get locations](LocationBucketViewGetCall) (response)
/// * [buckets views patch locations](LocationBucketViewPatchCall) (request|response)
/// * [locations buckets views create organizations](OrganizationLocationBucketViewCreateCall) (request|response)
/// * [locations buckets views get organizations](OrganizationLocationBucketViewGetCall) (response)
/// * [locations buckets views patch organizations](OrganizationLocationBucketViewPatchCall) (request|response)
/// * [locations buckets views create projects](ProjectLocationBucketViewCreateCall) (request|response)
/// * [locations buckets views get projects](ProjectLocationBucketViewGetCall) (response)
/// * [locations buckets views patch projects](ProjectLocationBucketViewPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LogView {
    /// Output only. The creation timestamp of the view.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Describes this view.
    
    pub description: Option<String>,
    /// Filter that restricts which log entries in a bucket are visible in this view.Filters are restricted to be a logical AND of ==/!= of any of the following: originating project/folder/organization/billing account. resource type log idFor example:SOURCE("projects/myproject") AND resource.type = "gce_instance" AND LOG_ID("stdout")
    
    pub filter: Option<String>,
    /// The resource name of the view.For example:projects/my-project/locations/global/buckets/my-bucket/views/my-view
    
    pub name: Option<String>,
    /// Output only. The last update timestamp of the view.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for LogView {}
impl client::ResponseResult for LogView {}


/// Defines a metric type and its schema. Once a metric descriptor is created, deleting or altering it stops data collection and makes the metric type's existing data unusable.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricDescriptor {
    /// A detailed description of the metric, which can be used in documentation.
    
    pub description: Option<String>,
    /// A concise name for the metric, which can be displayed in user interfaces. Use sentence case without an ending period, for example "Request count". This field is optional but it is recommended to be set for any metrics associated with user-visible concepts, such as Quota.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The set of labels that can be used to describe a specific instance of this metric type. For example, the appengine.googleapis.com/http/server/response_latencies metric type has a label for the HTTP response code, response_code, so you can look at latencies for successful responses or just for responses that failed.
    
    pub labels: Option<Vec<LabelDescriptor>>,
    /// Optional. The launch stage of the metric definition.
    #[serde(rename="launchStage")]
    
    pub launch_stage: Option<String>,
    /// Optional. Metadata which can be used to guide usage of the metric.
    
    pub metadata: Option<MetricDescriptorMetadata>,
    /// Whether the metric records instantaneous values, changes to a value, etc. Some combinations of metric_kind and value_type might not be supported.
    #[serde(rename="metricKind")]
    
    pub metric_kind: Option<String>,
    /// Read-only. If present, then a time series, which is identified partially by a metric type and a MonitoredResourceDescriptor, that is associated with this metric type can only be associated with one of the monitored resource types listed here.
    #[serde(rename="monitoredResourceTypes")]
    
    pub monitored_resource_types: Option<Vec<String>>,
    /// The resource name of the metric descriptor.
    
    pub name: Option<String>,
    /// The metric type, including its DNS name prefix. The type is not URL-encoded. All user-defined metric types have the DNS name custom.googleapis.com or external.googleapis.com. Metric types should use a natural hierarchical grouping. For example: "custom.googleapis.com/invoice/paid/amount" "external.googleapis.com/prometheus/up" "appengine.googleapis.com/http/server/response_latencies" 
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The units in which the metric value is reported. It is only applicable if the value_type is INT64, DOUBLE, or DISTRIBUTION. The unit defines the representation of the stored metric values.Different systems might scale the values to be more easily displayed (so a value of 0.02kBy might be displayed as 20By, and a value of 3523kBy might be displayed as 3.5MBy). However, if the unit is kBy, then the value of the metric is always in thousands of bytes, no matter how it might be displayed.If you want a custom metric to record the exact number of CPU-seconds used by a job, you can create an INT64 CUMULATIVE metric whose unit is s{CPU} (or equivalently 1s{CPU} or just s). If the job uses 12,005 CPU-seconds, then the value is written as 12005.Alternatively, if you want a custom metric to record data in a more granular way, you can create a DOUBLE CUMULATIVE metric whose unit is ks{CPU}, and then write the value 12.005 (which is 12005/1000), or use Kis{CPU} and write 11.723 (which is 12005/1024).The supported units are a subset of The Unified Code for Units of Measure (https://unitsofmeasure.org/ucum.html) standard:Basic units (UNIT) bit bit By byte s second min minute h hour d day 1 dimensionlessPrefixes (PREFIX) k kilo (10^3) M mega (10^6) G giga (10^9) T tera (10^12) P peta (10^15) E exa (10^18) Z zetta (10^21) Y yotta (10^24) m milli (10^-3) u micro (10^-6) n nano (10^-9) p pico (10^-12) f femto (10^-15) a atto (10^-18) z zepto (10^-21) y yocto (10^-24) Ki kibi (2^10) Mi mebi (2^20) Gi gibi (2^30) Ti tebi (2^40) Pi pebi (2^50)GrammarThe grammar also includes these connectors: / division or ratio (as an infix operator). For examples, kBy/{email} or MiBy/10ms (although you should almost never have /s in a metric unit; rates should always be computed at query time from the underlying cumulative or delta value). . multiplication or composition (as an infix operator). For examples, GBy.d or k{watt}.h.The grammar for a unit is as follows: Expression = Component { "." Component } { "/" Component } ; Component = ( [ PREFIX ] UNIT | "%" ) [ Annotation ] | Annotation | "1" ; Annotation = "{" NAME "}" ; Notes: Annotation is just a comment if it follows a UNIT. If the annotation is used alone, then the unit is equivalent to 1. For examples, {request}/s == 1/s, By{transmitted}/s == By/s. NAME is a sequence of non-blank printable ASCII characters not containing { or }. 1 represents a unitary dimensionless unit (https://en.wikipedia.org/wiki/Dimensionless_quantity) of 1, such as in 1/s. It is typically used when none of the basic units are appropriate. For example, "new users per day" can be represented as 1/d or {new-users}/d (and a metric value 5 would mean "5 new users). Alternatively, "thousands of page views per day" would be represented as 1000/d or k1/d or k{page_views}/d (and a metric value of 5.3 would mean "5300 page views per day"). % represents dimensionless value of 1/100, and annotates values giving a percentage (so the metric values are typically in the range of 0..100, and a metric value 3 means "3 percent"). 10^2.% indicates a metric contains a ratio, typically in the range 0..1, that will be multiplied by 100 and displayed as a percentage (so a metric value 0.03 means "3 percent").
    
    pub unit: Option<String>,
    /// Whether the measurement is an integer, a floating-point number, etc. Some combinations of metric_kind and value_type might not be supported.
    #[serde(rename="valueType")]
    
    pub value_type: Option<String>,
}

impl client::Part for MetricDescriptor {}


/// Additional annotations that can be used to guide the usage of a metric.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricDescriptorMetadata {
    /// The delay of data points caused by ingestion. Data points older than this age are guaranteed to be ingested and available to be read, excluding data loss due to errors.
    #[serde(rename="ingestDelay")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub ingest_delay: Option<client::chrono::Duration>,
    /// Deprecated. Must use the MetricDescriptor.launch_stage instead.
    #[serde(rename="launchStage")]
    
    pub launch_stage: Option<String>,
    /// The sampling period of metric data points. For metrics which are written periodically, consecutive data points are stored at this time interval, excluding data loss due to errors. Metrics with a higher granularity have a smaller sampling period.
    #[serde(rename="samplePeriod")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub sample_period: Option<client::chrono::Duration>,
}

impl client::Part for MetricDescriptorMetadata {}


/// An object representing a resource that can be used for monitoring, logging, billing, or other purposes. Examples include virtual machine instances, databases, and storage devices such as disks. The type field identifies a MonitoredResourceDescriptor object that describes the resource's schema. Information in the labels field identifies the actual resource and its attributes according to the schema. For example, a particular Compute Engine VM instance could be represented by the following object, because the MonitoredResourceDescriptor for "gce_instance" has labels "project_id", "instance_id" and "zone": { "type": "gce_instance", "labels": { "project_id": "my-project", "instance_id": "12345678901234", "zone": "us-central1-a" }} 
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MonitoredResource {
    /// Required. Values for all of the labels listed in the associated monitored resource descriptor. For example, Compute Engine VM instances use the labels "project_id", "instance_id", and "zone".
    
    pub labels: Option<HashMap<String, String>>,
    /// Required. The monitored resource type. This field must match the type field of a MonitoredResourceDescriptor object. For example, the type of a Compute Engine VM instance is gce_instance. Some descriptors include the service name in the type; for example, the type of a Datastream stream is datastream.googleapis.com/Stream.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for MonitoredResource {}


/// An object that describes the schema of a MonitoredResource object using a type name and a set of labels. For example, the monitored resource descriptor for Google Compute Engine VM instances has a type of “gce_instance” and specifies the use of the labels “instance_id” and “zone” to identify particular VM instances.Different APIs can support different monitored resource types. APIs generally provide a list method that returns the monitored resource descriptors used by the API.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list monitored resource descriptors](MonitoredResourceDescriptorListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MonitoredResourceDescriptor {
    /// Optional. A detailed description of the monitored resource type that might be used in documentation.
    
    pub description: Option<String>,
    /// Optional. A concise name for the monitored resource type that might be displayed in user interfaces. It should be a Title Cased Noun Phrase, without any article or other determiners. For example, "Google Cloud SQL Database".
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Required. A set of labels used to describe instances of this monitored resource type. For example, an individual Google Cloud SQL database is identified by values for the labels "database_id" and "zone".
    
    pub labels: Option<Vec<LabelDescriptor>>,
    /// Optional. The launch stage of the monitored resource definition.
    #[serde(rename="launchStage")]
    
    pub launch_stage: Option<String>,
    /// Optional. The resource name of the monitored resource descriptor: "projects/{project_id}/monitoredResourceDescriptors/{type}" where {type} is the value of the type field in this object and {project_id} is a project ID that provides API-specific context for accessing the type. APIs that do not use project information can use the resource name format "monitoredResourceDescriptors/{type}".
    
    pub name: Option<String>,
    /// Required. The monitored resource type. For example, the type "cloudsql_database" represents databases in Google Cloud SQL. For a list of types, see Monitoring resource types (https://cloud.google.com/monitoring/api/resources) and Logging resource types (https://cloud.google.com/logging/docs/api/v2/resource-list).
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Resource for MonitoredResourceDescriptor {}


/// Auxiliary metadata for a MonitoredResource object. MonitoredResource objects contain the minimum set of information to uniquely identify a monitored resource instance. There is some other useful auxiliary metadata. Monitoring and Logging use an ingestion pipeline to extract metadata for cloud resources of all types, and store the metadata in this message.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MonitoredResourceMetadata {
    /// Output only. Values for predefined system metadata labels. System labels are a kind of metadata extracted by Google, including "machine_image", "vpc", "subnet_id", "security_group", "name", etc. System label values can be only strings, Boolean values, or a list of strings. For example: { "name": "my-test-instance", "security_group": ["a", "b", "c"], "spot_instance": false } 
    #[serde(rename="systemLabels")]
    
    pub system_labels: Option<HashMap<String, json::Value>>,
    /// Output only. A map of user-defined metadata labels.
    #[serde(rename="userLabels")]
    
    pub user_labels: Option<HashMap<String, String>>,
}

impl client::Part for MonitoredResourceMetadata {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations get billing accounts](BillingAccountLocationOperationGetCall) (response)
/// * [copy entries](EntryCopyCall) (response)
/// * [locations operations get folders](FolderLocationOperationGetCall) (response)
/// * [operations get locations](LocationOperationGetCall) (response)
/// * [locations operations get organizations](OrganizationLocationOperationGetCall) (response)
/// * [locations operations get projects](ProjectLocationOperationGetCall) (response)
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


/// Describes the settings associated with a project, folder, organization, billing account, or flexible resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get settings billing accounts](BillingAccountGetSettingCall) (response)
/// * [get settings folders](FolderGetSettingCall) (response)
/// * [update settings folders](FolderUpdateSettingCall) (request|response)
/// * [get settings organizations](OrganizationGetSettingCall) (response)
/// * [update settings organizations](OrganizationUpdateSettingCall) (request|response)
/// * [get settings projects](ProjectGetSettingCall) (response)
/// * [get settings](MethodGetSettingCall) (response)
/// * [update settings](MethodUpdateSettingCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    /// Optional. If set to true, the _Default sink in newly created projects and folders will created in a disabled state. This can be used to automatically disable log ingestion if there is already an aggregated sink configured in the hierarchy. The _Default sink can be re-enabled manually if needed.
    #[serde(rename="disableDefaultSink")]
    
    pub disable_default_sink: Option<bool>,
    /// Optional. The resource name for the configured Cloud KMS key.KMS key name format: "projects/[PROJECT_ID]/locations/[LOCATION]/keyRings/[KEYRING]/cryptoKeys/[KEY]" For example:"projects/my-project/locations/us-central1/keyRings/my-ring/cryptoKeys/my-key"To enable CMEK for the Log Router, set this field to a valid kms_key_name for which the associated service account has the required roles/cloudkms.cryptoKeyEncrypterDecrypter role assigned for the key.The Cloud KMS key used by the Log Router can be updated by changing the kms_key_name to a new valid key name. Encryption operations that are in progress will be completed with the key that was in use when they started. Decryption operations will be completed using the key that was used at the time of encryption unless access to that key has been revoked.To disable CMEK for the Log Router, set this field to an empty string.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.
    #[serde(rename="kmsKeyName")]
    
    pub kms_key_name: Option<String>,
    /// Output only. The service account that will be used by the Log Router to access your Cloud KMS key.Before enabling CMEK for Log Router, you must first assign the role roles/cloudkms.cryptoKeyEncrypterDecrypter to the service account that the Log Router will use to access your Cloud KMS key. Use GetSettings to obtain the service account ID.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.
    #[serde(rename="kmsServiceAccountId")]
    
    pub kms_service_account_id: Option<String>,
    /// Output only. The resource name of the settings.
    
    pub name: Option<String>,
    /// Optional. The Cloud region that will be used for _Default and _Required log buckets for newly created projects and folders. For example europe-west1. This setting does not affect the location of custom log buckets.
    #[serde(rename="storageLocation")]
    
    pub storage_location: Option<String>,
}

impl client::RequestValue for Settings {}
impl client::ResponseResult for Settings {}


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


/// Information about entries that were omitted from the session.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SuppressionInfo {
    /// The reason that entries were omitted from the session.
    
    pub reason: Option<String>,
    /// A lower bound on the count of entries omitted due to reason.
    #[serde(rename="suppressedCount")]
    
    pub suppressed_count: Option<i32>,
}

impl client::Part for SuppressionInfo {}


/// The parameters to TailLogEntries.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [tail entries](EntryTailCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TailLogEntriesRequest {
    /// Optional. The amount of time to buffer log entries at the server before being returned to prevent out of order results due to late arriving log entries. Valid values are between 0-60000 milliseconds. Defaults to 2000 milliseconds.
    #[serde(rename="bufferWindow")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub buffer_window: Option<client::chrono::Duration>,
    /// Optional. Only log entries that match the filter are returned. An empty filter matches all log entries in the resources listed in resource_names. Referencing a parent resource that is not listed in resource_names will cause the filter to return no results. The maximum length of a filter is 20,000 characters.
    
    pub filter: Option<String>,
    /// Required. Name of a parent resource from which to retrieve log entries: projects/[PROJECT_ID] organizations/[ORGANIZATION_ID] billingAccounts/[BILLING_ACCOUNT_ID] folders/[FOLDER_ID]May alternatively be one or more views: projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/views/[VIEW_ID] organizations/[ORGANIZATION_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/views/[VIEW_ID] billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/views/[VIEW_ID] folders/[FOLDER_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/views/[VIEW_ID]
    #[serde(rename="resourceNames")]
    
    pub resource_names: Option<Vec<String>>,
}

impl client::RequestValue for TailLogEntriesRequest {}


/// Result returned from TailLogEntries.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [tail entries](EntryTailCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TailLogEntriesResponse {
    /// A list of log entries. Each response in the stream will order entries with increasing values of LogEntry.timestamp. Ordering is not guaranteed between separate responses.
    
    pub entries: Option<Vec<LogEntry>>,
    /// If entries that otherwise would have been included in the session were not sent back to the client, counts of relevant entries omitted from the session with the reason that they were not included. There will be at most one of each reason per response. The counts represent the number of suppressed entries since the last streamed response.
    #[serde(rename="suppressionInfo")]
    
    pub suppression_info: Option<Vec<SuppressionInfo>>,
}

impl client::ResponseResult for TailLogEntriesResponse {}


/// The parameters to UndeleteBucket.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations buckets undelete billing accounts](BillingAccountLocationBucketUndeleteCall) (request)
/// * [locations buckets undelete folders](FolderLocationBucketUndeleteCall) (request)
/// * [buckets undelete locations](LocationBucketUndeleteCall) (request)
/// * [locations buckets undelete organizations](OrganizationLocationBucketUndeleteCall) (request)
/// * [locations buckets undelete projects](ProjectLocationBucketUndeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UndeleteBucketRequest { _never_set: Option<bool> }

impl client::RequestValue for UndeleteBucketRequest {}


/// The parameters to WriteLogEntries.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [write entries](EntryWriteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WriteLogEntriesRequest {
    /// Optional. If true, the request should expect normal response, but the entries won't be persisted nor exported. Useful for checking whether the logging API endpoints are working properly before sending valuable data.
    #[serde(rename="dryRun")]
    
    pub dry_run: Option<bool>,
    /// Required. The log entries to send to Logging. The order of log entries in this list does not matter. Values supplied in this method's log_name, resource, and labels fields are copied into those log entries in this list that do not include values for their corresponding fields. For more information, see the LogEntry type.If the timestamp or insert_id fields are missing in log entries, then this method supplies the current time or a unique identifier, respectively. The supplied values are chosen so that, among the log entries that did not supply their own values, the entries earlier in the list will sort before the entries later in the list. See the entries.list method.Log entries with timestamps that are more than the logs retention period (https://cloud.google.com/logging/quotas) in the past or more than 24 hours in the future will not be available when calling entries.list. However, those log entries can still be exported with LogSinks (https://cloud.google.com/logging/docs/api/tasks/exporting-logs).To improve throughput and to avoid exceeding the quota limit (https://cloud.google.com/logging/quotas) for calls to entries.write, you should try to include several log entries in this list, rather than calling this method for each individual log entry.
    
    pub entries: Option<Vec<LogEntry>>,
    /// Optional. Default labels that are added to the labels field of all log entries in entries. If a log entry already has a label with the same key as a label in this parameter, then the log entry's label is not changed. See LogEntry.
    
    pub labels: Option<HashMap<String, String>>,
    /// Optional. A default log resource name that is assigned to all log entries in entries that do not specify a value for log_name: projects/[PROJECT_ID]/logs/[LOG_ID] organizations/[ORGANIZATION_ID]/logs/[LOG_ID] billingAccounts/[BILLING_ACCOUNT_ID]/logs/[LOG_ID] folders/[FOLDER_ID]/logs/[LOG_ID][LOG_ID] must be URL-encoded. For example: "projects/my-project-id/logs/syslog" "organizations/123/logs/cloudaudit.googleapis.com%2Factivity" The permission logging.logEntries.create is needed on each project, organization, billing account, or folder that is receiving new log entries, whether the resource is specified in logName or in an individual log entry.
    #[serde(rename="logName")]
    
    pub log_name: Option<String>,
    /// Optional. Whether a batch's valid entries should be written even if some other entry failed due to a permanent error such as INVALID_ARGUMENT or PERMISSION_DENIED. If any entry failed, then the response status is the response status of one of the failed entries. The response will include error details in WriteLogEntriesPartialErrors.log_entry_errors keyed by the entries' zero-based index in the entries. Failed requests for which no entries are written will not include per-entry errors.
    #[serde(rename="partialSuccess")]
    
    pub partial_success: Option<bool>,
    /// Optional. A default monitored resource object that is assigned to all log entries in entries that do not specify a value for resource. Example: { "type": "gce_instance", "labels": { "zone": "us-central1-a", "instance_id": "00000000000000000000" }} See LogEntry.
    
    pub resource: Option<MonitoredResource>,
}

impl client::RequestValue for WriteLogEntriesRequest {}


/// Result returned from WriteLogEntries.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [write entries](EntryWriteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WriteLogEntriesResponse { _never_set: Option<bool> }

impl client::ResponseResult for WriteLogEntriesResponse {}


