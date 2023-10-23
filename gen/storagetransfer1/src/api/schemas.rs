use super::*;
/// Represents an On-Premises Agent pool.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [agent pools create projects](ProjectAgentPoolCreateCall) (request|response)
/// * [agent pools get projects](ProjectAgentPoolGetCall) (response)
/// * [agent pools patch projects](ProjectAgentPoolPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AgentPool {
    /// Specifies the bandwidth limit details. If this field is unspecified, the default value is set as 'No Limit'.
    #[serde(rename="bandwidthLimit")]
    
    pub bandwidth_limit: Option<BandwidthLimit>,
    /// Specifies the client-specified AgentPool description.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Required. Specifies a unique string that identifies the agent pool. Format: `projects/{project_id}/agentPools/{agent_pool_id}`
    
    pub name: Option<String>,
    /// Output only. Specifies the state of the AgentPool.
    
    pub state: Option<AgentPoolStateEnum>,
}

impl client::RequestValue for AgentPool {}
impl client::ResponseResult for AgentPool {}


/// AWS access key (see [AWS Security Credentials](https://docs.aws.amazon.com/general/latest/gr/aws-security-credentials.html)). For information on our data retention policy for user credentials, see [User credentials](https://cloud.google.com/storage-transfer/docs/data-retention#user-credentials).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AwsAccessKey {
    /// Required. AWS access key ID.
    #[serde(rename="accessKeyId")]
    
    pub access_key_id: Option<String>,
    /// Required. AWS secret access key. This field is not returned in RPC responses.
    #[serde(rename="secretAccessKey")]
    
    pub secret_access_key: Option<String>,
}

impl client::Part for AwsAccessKey {}


/// An AwsS3CompatibleData resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AwsS3CompatibleData {
    /// Required. Specifies the name of the bucket.
    #[serde(rename="bucketName")]
    
    pub bucket_name: Option<String>,
    /// Required. Specifies the endpoint of the storage service.
    
    pub endpoint: Option<String>,
    /// Specifies the root path to transfer objects. Must be an empty string or full path name that ends with a '/'. This field is treated as an object prefix. As such, it should generally not begin with a '/'.
    
    pub path: Option<String>,
    /// Specifies the region to sign requests with. This can be left blank if requests should be signed with an empty region.
    
    pub region: Option<String>,
    /// A S3 compatible metadata.
    #[serde(rename="s3Metadata")]
    
    pub s3_metadata: Option<S3CompatibleMetadata>,
}

impl client::Part for AwsS3CompatibleData {}


/// An AwsS3Data resource can be a data source, but not a data sink. In an AwsS3Data resource, an object's name is the S3 object's key name.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AwsS3Data {
    /// Input only. AWS access key used to sign the API requests to the AWS S3 bucket. Permissions on the bucket must be granted to the access ID of the AWS access key. For information on our data retention policy for user credentials, see [User credentials](https://cloud.google.com/storage-transfer/docs/data-retention#user-credentials).
    #[serde(rename="awsAccessKey")]
    
    pub aws_access_key: Option<AwsAccessKey>,
    /// Required. S3 Bucket name (see [Creating a bucket](https://docs.aws.amazon.com/AmazonS3/latest/dev/create-bucket-get-location-example.html)).
    #[serde(rename="bucketName")]
    
    pub bucket_name: Option<String>,
    /// Root path to transfer objects. Must be an empty string or full path name that ends with a '/'. This field is treated as an object prefix. As such, it should generally not begin with a '/'.
    
    pub path: Option<String>,
    /// The Amazon Resource Name (ARN) of the role to support temporary credentials via `AssumeRoleWithWebIdentity`. For more information about ARNs, see [IAM ARNs](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-arns). When a role ARN is provided, Transfer Service fetches temporary credentials for the session using a `AssumeRoleWithWebIdentity` call for the provided role using the GoogleServiceAccount for this project.
    #[serde(rename="roleArn")]
    
    pub role_arn: Option<String>,
}

impl client::Part for AwsS3Data {}


/// An AzureBlobStorageData resource can be a data source, but not a data sink. An AzureBlobStorageData resource represents one Azure container. The storage account determines the [Azure endpoint](https://docs.microsoft.com/en-us/azure/storage/common/storage-create-storage-account#storage-account-endpoints). In an AzureBlobStorageData resource, a blobs's name is the [Azure Blob Storage blob's key name](https://docs.microsoft.com/en-us/rest/api/storageservices/naming-and-referencing-containers--blobs--and-metadata#blob-names).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AzureBlobStorageData {
    /// Required. Input only. Credentials used to authenticate API requests to Azure. For information on our data retention policy for user credentials, see [User credentials](https://cloud.google.com/storage-transfer/docs/data-retention#user-credentials).
    #[serde(rename="azureCredentials")]
    
    pub azure_credentials: Option<AzureCredentials>,
    /// Required. The container to transfer from the Azure Storage account.
    
    pub container: Option<String>,
    /// Root path to transfer objects. Must be an empty string or full path name that ends with a '/'. This field is treated as an object prefix. As such, it should generally not begin with a '/'.
    
    pub path: Option<String>,
    /// Required. The name of the Azure Storage account.
    #[serde(rename="storageAccount")]
    
    pub storage_account: Option<String>,
}

impl client::Part for AzureBlobStorageData {}


/// Azure credentials For information on our data retention policy for user credentials, see [User credentials](https://cloud.google.com/storage-transfer/docs/data-retention#user-credentials).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AzureCredentials {
    /// Required. Azure shared access signature (SAS). For more information about SAS, see [Grant limited access to Azure Storage resources using shared access signatures (SAS)](https://docs.microsoft.com/en-us/azure/storage/common/storage-sas-overview).
    #[serde(rename="sasToken")]
    
    pub sas_token: Option<String>,
}

impl client::Part for AzureCredentials {}


/// Specifies a bandwidth limit for an agent pool.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BandwidthLimit {
    /// Bandwidth rate in megabytes per second, distributed across all the agents in the pool.
    #[serde(rename="limitMbps")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub limit_mbps: Option<i64>,
}

impl client::Part for BandwidthLimit {}


/// The request message for Operations.CancelOperation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [cancel transfer operations](TransferOperationCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelOperationRequest { _never_set: Option<bool> }

impl client::RequestValue for CancelOperationRequest {}


/// Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values. * A month and day, with a zero year (for example, an anniversary). * A year on its own, with a zero month and a zero day. * A year and month, with a zero day (for example, a credit card expiration date). Related types: * google.type.TimeOfDay * google.type.DateTime * google.protobuf.Timestamp
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Date {
    /// Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant.
    
    pub day: Option<i32>,
    /// Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day.
    
    pub month: Option<i32>,
    /// Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year.
    
    pub year: Option<i32>,
}

impl client::Part for Date {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [agent pools delete projects](ProjectAgentPoolDeleteCall) (response)
/// * [delete transfer jobs](TransferJobDeleteCall) (response)
/// * [cancel transfer operations](TransferOperationCancelCall) (response)
/// * [pause transfer operations](TransferOperationPauseCall) (response)
/// * [resume transfer operations](TransferOperationResumeCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// An entry describing an error that has occurred.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ErrorLogEntry {
    /// A list of messages that carry the error details.
    #[serde(rename="errorDetails")]
    
    pub error_details: Option<Vec<String>>,
    /// Required. A URL that refers to the target (a data source, a data sink, or an object) with which the error is associated.
    
    pub url: Option<String>,
}

impl client::Part for ErrorLogEntry {}


/// A summary of errors by error code, plus a count and sample error log entries.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ErrorSummary {
    /// Required.
    #[serde(rename="errorCode")]
    
    pub error_code: Option<ErrorSummaryErrorCodeEnum>,
    /// Required. Count of this type of error.
    #[serde(rename="errorCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub error_count: Option<i64>,
    /// Error samples. At most 5 error log entries are recorded for a given error code for a single transfer operation.
    #[serde(rename="errorLogEntries")]
    
    pub error_log_entries: Option<Vec<ErrorLogEntry>>,
}

impl client::Part for ErrorSummary {}


/// Specifies the Event-driven transfer options. Event-driven transfers listen to an event stream to transfer updated files.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventStream {
    /// Specifies the data and time at which Storage Transfer Service stops listening for events from this stream. After this time, any transfers in progress will complete, but no new transfers are initiated.
    #[serde(rename="eventStreamExpirationTime")]
    
    pub event_stream_expiration_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Specifies the date and time that Storage Transfer Service starts listening for events from this stream. If no start time is specified or start time is in the past, Storage Transfer Service starts listening immediately.
    #[serde(rename="eventStreamStartTime")]
    
    pub event_stream_start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. Specifies a unique name of the resource such as AWS SQS ARN in the form 'arn:aws:sqs:region:account_id:queue_name', or Pub/Sub subscription resource name in the form 'projects/{project}/subscriptions/{sub}'.
    
    pub name: Option<String>,
}

impl client::Part for EventStream {}


/// In a GcsData resource, an object's name is the Cloud Storage object's name and its "last modification time" refers to the object's `updated` property of Cloud Storage objects, which changes when the content or the metadata of the object is updated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GcsData {
    /// Required. Cloud Storage bucket name. Must meet [Bucket Name Requirements](https://cloud.google.com/storage/docs/naming#requirements).
    #[serde(rename="bucketName")]
    
    pub bucket_name: Option<String>,
    /// Root path to transfer objects. Must be an empty string or full path name that ends with a ‘/’. This field is treated as an object prefix. As such, it should generally not begin with a ‘/’. The root path value must meet [Object Name Requirements](https://cloud.google.com/storage/docs/naming#objectnames).
    
    pub path: Option<String>,
}

impl client::Part for GcsData {}


/// Google service account
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get google service accounts](GoogleServiceAccountGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleServiceAccount {
    /// Email address of the service account.
    #[serde(rename="accountEmail")]
    
    pub account_email: Option<String>,
    /// Unique identifier for the service account.
    #[serde(rename="subjectId")]
    
    pub subject_id: Option<String>,
}

impl client::Resource for GoogleServiceAccount {}
impl client::ResponseResult for GoogleServiceAccount {}


/// An HttpData resource specifies a list of objects on the web to be transferred over HTTP. The information of the objects to be transferred is contained in a file referenced by a URL. The first line in the file must be `"TsvHttpData-1.0"`, which specifies the format of the file. Subsequent lines specify the information of the list of objects, one object per list entry. Each entry has the following tab-delimited fields: * **HTTP URL** — The location of the object. * **Length** — The size of the object in bytes. * **MD5** — The base64-encoded MD5 hash of the object. For an example of a valid TSV file, see [Transferring data from URLs](https://cloud.google.com/storage-transfer/docs/create-url-list). When transferring data based on a URL list, keep the following in mind: * When an object located at `http(s)://hostname:port/` is transferred to a data sink, the name of the object at the data sink is `/`. * If the specified size of an object does not match the actual size of the object fetched, the object is not transferred. * If the specified MD5 does not match the MD5 computed from the transferred bytes, the object transfer fails. * Ensure that each URL you specify is publicly accessible. For example, in Cloud Storage you can \[share an object publicly\] (/storage/docs/cloud-console#\_sharingdata) and get a link to it. * Storage Transfer Service obeys `robots.txt` rules and requires the source HTTP server to support `Range` requests and to return a `Content-Length` header in each response. * ObjectConditions have no effect when filtering objects to transfer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HttpData {
    /// Required. The URL that points to the file that stores the object list entries. This file must allow public access. Currently, only URLs with HTTP and HTTPS schemes are supported.
    #[serde(rename="listUrl")]
    
    pub list_url: Option<String>,
}

impl client::Part for HttpData {}


/// Response from ListAgentPools.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [agent pools list projects](ProjectAgentPoolListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAgentPoolsResponse {
    /// A list of agent pools.
    #[serde(rename="agentPools")]
    
    pub agent_pools: Option<Vec<AgentPool>>,
    /// The list next page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListAgentPoolsResponse {}


/// The response message for Operations.ListOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list transfer operations](TransferOperationListCall) (response)
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


/// Response from ListTransferJobs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list transfer jobs](TransferJobListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTransferJobsResponse {
    /// The list next page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of transfer jobs.
    #[serde(rename="transferJobs")]
    
    pub transfer_jobs: Option<Vec<TransferJob>>,
}

impl client::ResponseResult for ListTransferJobsResponse {}


/// Specifies the logging behavior for transfer operations. For cloud-to-cloud transfers, logs are sent to Cloud Logging. See [Read transfer logs](https://cloud.google.com/storage-transfer/docs/read-transfer-logs) for details. For transfers to or from a POSIX file system, logs are stored in the Cloud Storage bucket that is the source or sink of the transfer. See [Managing Transfer for on-premises jobs] (https://cloud.google.com/storage-transfer/docs/managing-on-prem-jobs#viewing-logs) for details.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// For transfers with a PosixFilesystem source, this option enables the Cloud Storage transfer logs for this transfer.
    #[serde(rename="enableOnpremGcsTransferLogs")]
    
    pub enable_onprem_gcs_transfer_logs: Option<bool>,
    /// States in which `log_actions` are logged. If empty, no logs are generated. Not supported for transfers with PosixFilesystem data sources; use enable_onprem_gcs_transfer_logs instead.
    #[serde(rename="logActionStates")]
    
    pub log_action_states: Option<Vec<LoggingConfigLogActionStatesEnum>>,
    /// Specifies the actions to be logged. If empty, no logs are generated. Not supported for transfers with PosixFilesystem data sources; use enable_onprem_gcs_transfer_logs instead.
    #[serde(rename="logActions")]
    
    pub log_actions: Option<Vec<LoggingConfigLogActionsEnum>>,
}

impl client::Part for LoggingConfig {}


/// Specifies the metadata options for running a transfer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetadataOptions {
    /// Specifies how each object's ACLs should be preserved for transfers between Google Cloud Storage buckets. If unspecified, the default behavior is the same as ACL_DESTINATION_BUCKET_DEFAULT.
    
    pub acl: Option<MetadataOptionAclEnum>,
    /// Specifies how each file's POSIX group ID (GID) attribute should be handled by the transfer. By default, GID is not preserved. Only applicable to transfers involving POSIX file systems, and ignored for other transfers.
    
    pub gid: Option<MetadataOptionGidEnum>,
    /// Specifies how each object's Cloud KMS customer-managed encryption key (CMEK) is preserved for transfers between Google Cloud Storage buckets. If unspecified, the default behavior is the same as KMS_KEY_DESTINATION_BUCKET_DEFAULT.
    #[serde(rename="kmsKey")]
    
    pub kms_key: Option<MetadataOptionKmsKeyEnum>,
    /// Specifies how each file's mode attribute should be handled by the transfer. By default, mode is not preserved. Only applicable to transfers involving POSIX file systems, and ignored for other transfers.
    
    pub mode: Option<MetadataOptionModeEnum>,
    /// Specifies the storage class to set on objects being transferred to Google Cloud Storage buckets. If unspecified, the default behavior is the same as STORAGE_CLASS_DESTINATION_BUCKET_DEFAULT.
    #[serde(rename="storageClass")]
    
    pub storage_class: Option<MetadataOptionStorageClassEnum>,
    /// Specifies how symlinks should be handled by the transfer. By default, symlinks are not preserved. Only applicable to transfers involving POSIX file systems, and ignored for other transfers.
    
    pub symlink: Option<MetadataOptionSymlinkEnum>,
    /// Specifies how each object's temporary hold status should be preserved for transfers between Google Cloud Storage buckets. If unspecified, the default behavior is the same as TEMPORARY_HOLD_PRESERVE.
    #[serde(rename="temporaryHold")]
    
    pub temporary_hold: Option<MetadataOptionTemporaryHoldEnum>,
    /// Specifies how each object's `timeCreated` metadata is preserved for transfers between Google Cloud Storage buckets. If unspecified, the default behavior is the same as TIME_CREATED_SKIP.
    #[serde(rename="timeCreated")]
    
    pub time_created: Option<MetadataOptionTimeCreatedEnum>,
    /// Specifies how each file's POSIX user ID (UID) attribute should be handled by the transfer. By default, UID is not preserved. Only applicable to transfers involving POSIX file systems, and ignored for other transfers.
    
    pub uid: Option<MetadataOptionUidEnum>,
}

impl client::Part for MetadataOptions {}


/// Specification to configure notifications published to Pub/Sub. Notifications are published to the customer-provided topic using the following `PubsubMessage.attributes`: * `"eventType"`: one of the EventType values * `"payloadFormat"`: one of the PayloadFormat values * `"projectId"`: the project_id of the `TransferOperation` * `"transferJobName"`: the transfer_job_name of the `TransferOperation` * `"transferOperationName"`: the name of the `TransferOperation` The `PubsubMessage.data` contains a TransferOperation resource formatted according to the specified `PayloadFormat`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NotificationConfig {
    /// Event types for which a notification is desired. If empty, send notifications for all event types.
    #[serde(rename="eventTypes")]
    
    pub event_types: Option<Vec<NotificationConfigEventTypesEnum>>,
    /// Required. The desired format of the notification message payloads.
    #[serde(rename="payloadFormat")]
    
    pub payload_format: Option<NotificationConfigPayloadFormatEnum>,
    /// Required. The `Topic.name` of the Pub/Sub topic to which to publish notifications. Must be of the format: `projects/{project}/topics/{topic}`. Not matching this format results in an INVALID_ARGUMENT error.
    #[serde(rename="pubsubTopic")]
    
    pub pubsub_topic: Option<String>,
}

impl client::Part for NotificationConfig {}


/// Conditions that determine which objects are transferred. Applies only to Cloud Data Sources such as S3, Azure, and Cloud Storage. The "last modification time" refers to the time of the last change to the object's content or metadata — specifically, this is the `updated` property of Cloud Storage objects, the `LastModified` field of S3 objects, and the `Last-Modified` header of Azure blobs. Transfers with a PosixFilesystem source or destination don't support `ObjectConditions`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ObjectConditions {
    /// If you specify `exclude_prefixes`, Storage Transfer Service uses the items in the `exclude_prefixes` array to determine which objects to exclude from a transfer. Objects must not start with one of the matching `exclude_prefixes` for inclusion in a transfer. The following are requirements of `exclude_prefixes`: * Each exclude-prefix can contain any sequence of Unicode characters, to a max length of 1024 bytes when UTF8-encoded, and must not contain Carriage Return or Line Feed characters. Wildcard matching and regular expression matching are not supported. * Each exclude-prefix must omit the leading slash. For example, to exclude the object `s3://my-aws-bucket/logs/y=2015/requests.gz`, specify the exclude-prefix as `logs/y=2015/requests.gz`. * None of the exclude-prefix values can be empty, if specified. * Each exclude-prefix must exclude a distinct portion of the object namespace. No exclude-prefix may be a prefix of another exclude-prefix. * If include_prefixes is specified, then each exclude-prefix must start with the value of a path explicitly included by `include_prefixes`. The max size of `exclude_prefixes` is 1000. For more information, see [Filtering objects from transfers](https://cloud.google.com/storage-transfer/docs/filtering-objects-from-transfers).
    #[serde(rename="excludePrefixes")]
    
    pub exclude_prefixes: Option<Vec<String>>,
    /// If you specify `include_prefixes`, Storage Transfer Service uses the items in the `include_prefixes` array to determine which objects to include in a transfer. Objects must start with one of the matching `include_prefixes` for inclusion in the transfer. If exclude_prefixes is specified, objects must not start with any of the `exclude_prefixes` specified for inclusion in the transfer. The following are requirements of `include_prefixes`: * Each include-prefix can contain any sequence of Unicode characters, to a max length of 1024 bytes when UTF8-encoded, and must not contain Carriage Return or Line Feed characters. Wildcard matching and regular expression matching are not supported. * Each include-prefix must omit the leading slash. For example, to include the object `s3://my-aws-bucket/logs/y=2015/requests.gz`, specify the include-prefix as `logs/y=2015/requests.gz`. * None of the include-prefix values can be empty, if specified. * Each include-prefix must include a distinct portion of the object namespace. No include-prefix may be a prefix of another include-prefix. The max size of `include_prefixes` is 1000. For more information, see [Filtering objects from transfers](https://cloud.google.com/storage-transfer/docs/filtering-objects-from-transfers).
    #[serde(rename="includePrefixes")]
    
    pub include_prefixes: Option<Vec<String>>,
    /// If specified, only objects with a "last modification time" before this timestamp and objects that don't have a "last modification time" are transferred.
    #[serde(rename="lastModifiedBefore")]
    
    pub last_modified_before: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// If specified, only objects with a "last modification time" on or after this timestamp and objects that don't have a "last modification time" are transferred. The `last_modified_since` and `last_modified_before` fields can be used together for chunked data processing. For example, consider a script that processes each day's worth of data at a time. For that you'd set each of the fields as follows: * `last_modified_since` to the start of the day * `last_modified_before` to the end of the day
    #[serde(rename="lastModifiedSince")]
    
    pub last_modified_since: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Ensures that objects are not transferred if a specific maximum time has elapsed since the "last modification time". When a TransferOperation begins, objects with a "last modification time" are transferred only if the elapsed time between the start_time of the `TransferOperation`and the "last modification time" of the object is less than the value of max_time_elapsed_since_last_modification`. Objects that do not have a "last modification time" are also transferred.
    #[serde(rename="maxTimeElapsedSinceLastModification")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub max_time_elapsed_since_last_modification: Option<client::chrono::Duration>,
    /// Ensures that objects are not transferred until a specific minimum time has elapsed after the "last modification time". When a TransferOperation begins, objects with a "last modification time" are transferred only if the elapsed time between the start_time of the `TransferOperation` and the "last modification time" of the object is equal to or greater than the value of min_time_elapsed_since_last_modification`. Objects that do not have a "last modification time" are also transferred.
    #[serde(rename="minTimeElapsedSinceLastModification")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub min_time_elapsed_since_last_modification: Option<client::chrono::Duration>,
}

impl client::Part for ObjectConditions {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [run transfer jobs](TransferJobRunCall) (response)
/// * [get transfer operations](TransferOperationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available.
    
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    
    pub error: Option<Status>,
    /// Represents the transfer operation object. To request a TransferOperation object, use transferOperations.get.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// The server-assigned unique name. The format of `name` is `transferOperations/some/unique/name`.
    
    pub name: Option<String>,
    /// The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`.
    
    pub response: Option<HashMap<String, json::Value>>,
}

impl client::ResponseResult for Operation {}


/// Request passed to PauseTransferOperation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [pause transfer operations](TransferOperationPauseCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PauseTransferOperationRequest { _never_set: Option<bool> }

impl client::RequestValue for PauseTransferOperationRequest {}


/// A POSIX filesystem resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PosixFilesystem {
    /// Root directory path to the filesystem.
    #[serde(rename="rootDirectory")]
    
    pub root_directory: Option<String>,
}

impl client::Part for PosixFilesystem {}


/// Request passed to ResumeTransferOperation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [resume transfer operations](TransferOperationResumeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResumeTransferOperationRequest { _never_set: Option<bool> }

impl client::RequestValue for ResumeTransferOperationRequest {}


/// Request passed to RunTransferJob.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [run transfer jobs](TransferJobRunCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RunTransferJobRequest {
    /// Required. The ID of the Google Cloud project that owns the transfer job.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
}

impl client::RequestValue for RunTransferJobRequest {}


/// S3CompatibleMetadata contains the metadata fields that apply to the basic types of S3-compatible data providers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct S3CompatibleMetadata {
    /// Specifies the authentication and authorization method used by the storage service. When not specified, Transfer Service will attempt to determine right auth method to use.
    #[serde(rename="authMethod")]
    
    pub auth_method: Option<S3CompatibleMetadataAuthMethodEnum>,
    /// The Listing API to use for discovering objects. When not specified, Transfer Service will attempt to determine the right API to use.
    #[serde(rename="listApi")]
    
    pub list_api: Option<S3CompatibleMetadataListApiEnum>,
    /// Specifies the network protocol of the agent. When not specified, the default value of NetworkProtocol NETWORK_PROTOCOL_HTTPS is used.
    
    pub protocol: Option<S3CompatibleMetadataProtocolEnum>,
    /// Specifies the API request model used to call the storage service. When not specified, the default value of RequestModel REQUEST_MODEL_VIRTUAL_HOSTED_STYLE is used.
    #[serde(rename="requestModel")]
    
    pub request_model: Option<S3CompatibleMetadataRequestModelEnum>,
}

impl client::Part for S3CompatibleMetadata {}


/// Transfers can be scheduled to recur or to run just once.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Schedule {
    /// The time in UTC that no further transfer operations are scheduled. Combined with schedule_end_date, `end_time_of_day` specifies the end date and time for starting new transfer operations. This field must be greater than or equal to the timestamp corresponding to the combintation of schedule_start_date and start_time_of_day, and is subject to the following: * If `end_time_of_day` is not set and `schedule_end_date` is set, then a default value of `23:59:59` is used for `end_time_of_day`. * If `end_time_of_day` is set and `schedule_end_date` is not set, then INVALID_ARGUMENT is returned.
    #[serde(rename="endTimeOfDay")]
    
    pub end_time_of_day: Option<TimeOfDay>,
    /// Interval between the start of each scheduled TransferOperation. If unspecified, the default value is 24 hours. This value may not be less than 1 hour.
    #[serde(rename="repeatInterval")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub repeat_interval: Option<client::chrono::Duration>,
    /// The last day a transfer runs. Date boundaries are determined relative to UTC time. A job runs once per 24 hours within the following guidelines: * If `schedule_end_date` and schedule_start_date are the same and in the future relative to UTC, the transfer is executed only one time. * If `schedule_end_date` is later than `schedule_start_date` and `schedule_end_date` is in the future relative to UTC, the job runs each day at start_time_of_day through `schedule_end_date`.
    #[serde(rename="scheduleEndDate")]
    
    pub schedule_end_date: Option<Date>,
    /// Required. The start date of a transfer. Date boundaries are determined relative to UTC time. If `schedule_start_date` and start_time_of_day are in the past relative to the job's creation time, the transfer starts the day after you schedule the transfer request. **Note:** When starting jobs at or near midnight UTC it is possible that a job starts later than expected. For example, if you send an outbound request on June 1 one millisecond prior to midnight UTC and the Storage Transfer Service server receives the request on June 2, then it creates a TransferJob with `schedule_start_date` set to June 2 and a `start_time_of_day` set to midnight UTC. The first scheduled TransferOperation takes place on June 3 at midnight UTC.
    #[serde(rename="scheduleStartDate")]
    
    pub schedule_start_date: Option<Date>,
    /// The time in UTC that a transfer job is scheduled to run. Transfers may start later than this time. If `start_time_of_day` is not specified: * One-time transfers run immediately. * Recurring transfers run immediately, and each day at midnight UTC, through schedule_end_date. If `start_time_of_day` is specified: * One-time transfers run at the specified time. * Recurring transfers run at the specified time each day, through `schedule_end_date`.
    #[serde(rename="startTimeOfDay")]
    
    pub start_time_of_day: Option<TimeOfDay>,
}

impl client::Part for Schedule {}


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


/// Represents a time of day. The date and time zone are either not significant or are specified elsewhere. An API may choose to allow leap seconds. Related types are google.type.Date and `google.protobuf.Timestamp`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeOfDay {
    /// Hours of day in 24 hour format. Should be from 0 to 23. An API may choose to allow the value "24:00:00" for scenarios like business closing time.
    
    pub hours: Option<i32>,
    /// Minutes of hour of day. Must be from 0 to 59.
    
    pub minutes: Option<i32>,
    /// Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999.
    
    pub nanos: Option<i32>,
    /// Seconds of minutes of the time. Must normally be from 0 to 59. An API may allow the value 60 if it allows leap-seconds.
    
    pub seconds: Option<i32>,
}

impl client::Part for TimeOfDay {}


/// A collection of counters that report the progress of a transfer operation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TransferCounters {
    /// Bytes that are copied to the data sink.
    #[serde(rename="bytesCopiedToSink")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bytes_copied_to_sink: Option<i64>,
    /// Bytes that are deleted from the data sink.
    #[serde(rename="bytesDeletedFromSink")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bytes_deleted_from_sink: Option<i64>,
    /// Bytes that are deleted from the data source.
    #[serde(rename="bytesDeletedFromSource")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bytes_deleted_from_source: Option<i64>,
    /// Bytes that failed to be deleted from the data sink.
    #[serde(rename="bytesFailedToDeleteFromSink")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bytes_failed_to_delete_from_sink: Option<i64>,
    /// Bytes found in the data source that are scheduled to be transferred, excluding any that are filtered based on object conditions or skipped due to sync.
    #[serde(rename="bytesFoundFromSource")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bytes_found_from_source: Option<i64>,
    /// Bytes found only in the data sink that are scheduled to be deleted.
    #[serde(rename="bytesFoundOnlyFromSink")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bytes_found_only_from_sink: Option<i64>,
    /// Bytes in the data source that failed to be transferred or that failed to be deleted after being transferred.
    #[serde(rename="bytesFromSourceFailed")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bytes_from_source_failed: Option<i64>,
    /// Bytes in the data source that are not transferred because they already exist in the data sink.
    #[serde(rename="bytesFromSourceSkippedBySync")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bytes_from_source_skipped_by_sync: Option<i64>,
    /// For transfers involving PosixFilesystem only. Number of listing failures for each directory found at the source. Potential failures when listing a directory include permission failure or block failure. If listing a directory fails, no files in the directory are transferred.
    #[serde(rename="directoriesFailedToListFromSource")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub directories_failed_to_list_from_source: Option<i64>,
    /// For transfers involving PosixFilesystem only. Number of directories found while listing. For example, if the root directory of the transfer is `base/` and there are two other directories, `a/` and `b/` under this directory, the count after listing `base/`, `base/a/` and `base/b/` is 3.
    #[serde(rename="directoriesFoundFromSource")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub directories_found_from_source: Option<i64>,
    /// For transfers involving PosixFilesystem only. Number of successful listings for each directory found at the source.
    #[serde(rename="directoriesSuccessfullyListedFromSource")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub directories_successfully_listed_from_source: Option<i64>,
    /// Number of successfully cleaned up intermediate objects.
    #[serde(rename="intermediateObjectsCleanedUp")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub intermediate_objects_cleaned_up: Option<i64>,
    /// Number of intermediate objects failed cleaned up.
    #[serde(rename="intermediateObjectsFailedCleanedUp")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub intermediate_objects_failed_cleaned_up: Option<i64>,
    /// Objects that are copied to the data sink.
    #[serde(rename="objectsCopiedToSink")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub objects_copied_to_sink: Option<i64>,
    /// Objects that are deleted from the data sink.
    #[serde(rename="objectsDeletedFromSink")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub objects_deleted_from_sink: Option<i64>,
    /// Objects that are deleted from the data source.
    #[serde(rename="objectsDeletedFromSource")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub objects_deleted_from_source: Option<i64>,
    /// Objects that failed to be deleted from the data sink.
    #[serde(rename="objectsFailedToDeleteFromSink")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub objects_failed_to_delete_from_sink: Option<i64>,
    /// Objects found in the data source that are scheduled to be transferred, excluding any that are filtered based on object conditions or skipped due to sync.
    #[serde(rename="objectsFoundFromSource")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub objects_found_from_source: Option<i64>,
    /// Objects found only in the data sink that are scheduled to be deleted.
    #[serde(rename="objectsFoundOnlyFromSink")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub objects_found_only_from_sink: Option<i64>,
    /// Objects in the data source that failed to be transferred or that failed to be deleted after being transferred.
    #[serde(rename="objectsFromSourceFailed")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub objects_from_source_failed: Option<i64>,
    /// Objects in the data source that are not transferred because they already exist in the data sink.
    #[serde(rename="objectsFromSourceSkippedBySync")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub objects_from_source_skipped_by_sync: Option<i64>,
}

impl client::Part for TransferCounters {}


/// This resource represents the configuration of a transfer job that runs periodically.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create transfer jobs](TransferJobCreateCall) (request|response)
/// * [delete transfer jobs](TransferJobDeleteCall) (none)
/// * [get transfer jobs](TransferJobGetCall) (response)
/// * [list transfer jobs](TransferJobListCall) (none)
/// * [patch transfer jobs](TransferJobPatchCall) (response)
/// * [run transfer jobs](TransferJobRunCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TransferJob {
    /// Output only. The time that the transfer job was created.
    #[serde(rename="creationTime")]
    
    pub creation_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The time that the transfer job was deleted.
    #[serde(rename="deletionTime")]
    
    pub deletion_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A description provided by the user for the job. Its max length is 1024 bytes when Unicode-encoded.
    
    pub description: Option<String>,
    /// Specifies the event stream for the transfer job for event-driven transfers. When EventStream is specified, the Schedule fields are ignored.
    #[serde(rename="eventStream")]
    
    pub event_stream: Option<EventStream>,
    /// Output only. The time that the transfer job was last modified.
    #[serde(rename="lastModificationTime")]
    
    pub last_modification_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The name of the most recently started TransferOperation of this JobConfig. Present if a TransferOperation has been created for this JobConfig.
    #[serde(rename="latestOperationName")]
    
    pub latest_operation_name: Option<String>,
    /// Logging configuration.
    #[serde(rename="loggingConfig")]
    
    pub logging_config: Option<LoggingConfig>,
    /// A unique name (within the transfer project) assigned when the job is created. If this field is empty in a CreateTransferJobRequest, Storage Transfer Service assigns a unique name. Otherwise, the specified name is used as the unique name for this job. If the specified name is in use by a job, the creation request fails with an ALREADY_EXISTS error. This name must start with `"transferJobs/"` prefix and end with a letter or a number, and should be no more than 128 characters. For transfers involving PosixFilesystem, this name must start with `transferJobs/OPI` specifically. For all other transfer types, this name must not start with `transferJobs/OPI`. Non-PosixFilesystem example: `"transferJobs/^(?!OPI)[A-Za-z0-9-._~]*[A-Za-z0-9]$"` PosixFilesystem example: `"transferJobs/OPI^[A-Za-z0-9-._~]*[A-Za-z0-9]$"` Applications must not rely on the enforcement of naming requirements involving OPI. Invalid job names fail with an INVALID_ARGUMENT error.
    
    pub name: Option<String>,
    /// Notification configuration. This is not supported for transfers involving PosixFilesystem.
    #[serde(rename="notificationConfig")]
    
    pub notification_config: Option<NotificationConfig>,
    /// The ID of the Google Cloud project that owns the job.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Specifies schedule for the transfer job. This is an optional field. When the field is not set, the job never executes a transfer, unless you invoke RunTransferJob or update the job to have a non-empty schedule.
    
    pub schedule: Option<Schedule>,
    /// Status of the job. This value MUST be specified for `CreateTransferJobRequests`. **Note:** The effect of the new job status takes place during a subsequent job run. For example, if you change the job status from ENABLED to DISABLED, and an operation spawned by the transfer is running, the status change would not affect the current operation.
    
    pub status: Option<TransferJobStatusEnum>,
    /// Transfer specification.
    #[serde(rename="transferSpec")]
    
    pub transfer_spec: Option<TransferSpec>,
}

impl client::RequestValue for TransferJob {}
impl client::Resource for TransferJob {}
impl client::ResponseResult for TransferJob {}


/// Specifies where the manifest is located.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TransferManifest {
    /// Specifies the path to the manifest in Cloud Storage. The Google-managed service account for the transfer must have `storage.objects.get` permission for this object. An example path is `gs://bucket_name/path/manifest.csv`.
    
    pub location: Option<String>,
}

impl client::Part for TransferManifest {}


/// A description of the execution of a transfer.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [cancel transfer operations](TransferOperationCancelCall) (none)
/// * [get transfer operations](TransferOperationGetCall) (none)
/// * [list transfer operations](TransferOperationListCall) (none)
/// * [pause transfer operations](TransferOperationPauseCall) (none)
/// * [resume transfer operations](TransferOperationResumeCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TransferOperation {
    /// Information about the progress of the transfer operation.
    
    pub counters: Option<TransferCounters>,
    /// End time of this transfer execution.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Summarizes errors encountered with sample error log entries.
    #[serde(rename="errorBreakdowns")]
    
    pub error_breakdowns: Option<Vec<ErrorSummary>>,
    /// A globally unique ID assigned by the system.
    
    pub name: Option<String>,
    /// Notification configuration.
    #[serde(rename="notificationConfig")]
    
    pub notification_config: Option<NotificationConfig>,
    /// The ID of the Google Cloud project that owns the operation.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Start time of this transfer execution.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Status of the transfer operation.
    
    pub status: Option<TransferOperationStatusEnum>,
    /// The name of the transfer job that triggers this transfer operation.
    #[serde(rename="transferJobName")]
    
    pub transfer_job_name: Option<String>,
    /// Transfer specification.
    #[serde(rename="transferSpec")]
    
    pub transfer_spec: Option<TransferSpec>,
}

impl client::Resource for TransferOperation {}


/// TransferOptions define the actions to be performed on objects in a transfer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TransferOptions {
    /// Whether objects should be deleted from the source after they are transferred to the sink. **Note:** This option and delete_objects_unique_in_sink are mutually exclusive.
    #[serde(rename="deleteObjectsFromSourceAfterTransfer")]
    
    pub delete_objects_from_source_after_transfer: Option<bool>,
    /// Whether objects that exist only in the sink should be deleted. **Note:** This option and delete_objects_from_source_after_transfer are mutually exclusive.
    #[serde(rename="deleteObjectsUniqueInSink")]
    
    pub delete_objects_unique_in_sink: Option<bool>,
    /// Represents the selected metadata options for a transfer job.
    #[serde(rename="metadataOptions")]
    
    pub metadata_options: Option<MetadataOptions>,
    /// When to overwrite objects that already exist in the sink. The default is that only objects that are different from the source are ovewritten. If true, all objects in the sink whose name matches an object in the source are overwritten with the source object.
    #[serde(rename="overwriteObjectsAlreadyExistingInSink")]
    
    pub overwrite_objects_already_existing_in_sink: Option<bool>,
    /// When to overwrite objects that already exist in the sink. If not set, overwrite behavior is determined by overwrite_objects_already_existing_in_sink.
    #[serde(rename="overwriteWhen")]
    
    pub overwrite_when: Option<TransferOptionOverwriteWhenEnum>,
}

impl client::Part for TransferOptions {}


/// Configuration for running a transfer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TransferSpec {
    /// An AWS S3 compatible data source.
    #[serde(rename="awsS3CompatibleDataSource")]
    
    pub aws_s3_compatible_data_source: Option<AwsS3CompatibleData>,
    /// An AWS S3 data source.
    #[serde(rename="awsS3DataSource")]
    
    pub aws_s3_data_source: Option<AwsS3Data>,
    /// An Azure Blob Storage data source.
    #[serde(rename="azureBlobStorageDataSource")]
    
    pub azure_blob_storage_data_source: Option<AzureBlobStorageData>,
    /// A Cloud Storage data sink.
    #[serde(rename="gcsDataSink")]
    
    pub gcs_data_sink: Option<GcsData>,
    /// A Cloud Storage data source.
    #[serde(rename="gcsDataSource")]
    
    pub gcs_data_source: Option<GcsData>,
    /// Cloud Storage intermediate data location.
    #[serde(rename="gcsIntermediateDataLocation")]
    
    pub gcs_intermediate_data_location: Option<GcsData>,
    /// An HTTP URL data source.
    #[serde(rename="httpDataSource")]
    
    pub http_data_source: Option<HttpData>,
    /// Only objects that satisfy these object conditions are included in the set of data source and data sink objects. Object conditions based on objects' "last modification time" do not exclude objects in a data sink.
    #[serde(rename="objectConditions")]
    
    pub object_conditions: Option<ObjectConditions>,
    /// A POSIX Filesystem data sink.
    #[serde(rename="posixDataSink")]
    
    pub posix_data_sink: Option<PosixFilesystem>,
    /// A POSIX Filesystem data source.
    #[serde(rename="posixDataSource")]
    
    pub posix_data_source: Option<PosixFilesystem>,
    /// Specifies the agent pool name associated with the posix data sink. When unspecified, the default name is used.
    #[serde(rename="sinkAgentPoolName")]
    
    pub sink_agent_pool_name: Option<String>,
    /// Specifies the agent pool name associated with the posix data source. When unspecified, the default name is used.
    #[serde(rename="sourceAgentPoolName")]
    
    pub source_agent_pool_name: Option<String>,
    /// A manifest file provides a list of objects to be transferred from the data source. This field points to the location of the manifest file. Otherwise, the entire source bucket is used. ObjectConditions still apply.
    #[serde(rename="transferManifest")]
    
    pub transfer_manifest: Option<TransferManifest>,
    /// If the option delete_objects_unique_in_sink is `true` and time-based object conditions such as 'last modification time' are specified, the request fails with an INVALID_ARGUMENT error.
    #[serde(rename="transferOptions")]
    
    pub transfer_options: Option<TransferOptions>,
}

impl client::Part for TransferSpec {}


/// Request passed to UpdateTransferJob.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [patch transfer jobs](TransferJobPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateTransferJobRequest {
    /// Required. The ID of the Google Cloud project that owns the job.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Required. The job to update. `transferJob` is expected to specify one or more of five fields: description, transfer_spec, notification_config, logging_config, and status. An `UpdateTransferJobRequest` that specifies other fields are rejected with the error INVALID_ARGUMENT. Updating a job status to DELETED requires `storagetransfer.jobs.delete` permission.
    #[serde(rename="transferJob")]
    
    pub transfer_job: Option<TransferJob>,
    /// The field mask of the fields in `transferJob` that are to be updated in this request. Fields in `transferJob` that can be updated are: description, transfer_spec, notification_config, logging_config, and status. To update the `transfer_spec` of the job, a complete transfer specification must be provided. An incomplete specification missing any required fields is rejected with the error INVALID_ARGUMENT.
    #[serde(rename="updateTransferJobFieldMask")]
    
    pub update_transfer_job_field_mask: Option<client::FieldMask>,
}

impl client::RequestValue for UpdateTransferJobRequest {}


