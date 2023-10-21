use super::*;
/// A request to determine whether the user has valid credentials. This method is used to limit the number of OAuth popups in the user interface. The user id is inferred from the API call context. If the data source has the Google+ authorization type, this method returns false, as it cannot be determined whether the credentials are already valid merely based on the user id.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [data sources check valid creds projects](ProjectDataSourceCheckValidCredCall) (request)
/// * [locations data sources check valid creds projects](ProjectLocationDataSourceCheckValidCredCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CheckValidCredsRequest { _never_set: Option<bool> }

impl client::RequestValue for CheckValidCredsRequest {}


/// A response indicating whether the credentials exist and are valid.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [data sources check valid creds projects](ProjectDataSourceCheckValidCredCall) (response)
/// * [locations data sources check valid creds projects](ProjectLocationDataSourceCheckValidCredCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CheckValidCredsResponse {
    /// If set to `true`, the credentials exist and are valid.
    #[serde(rename="hasValidCreds")]
    
    pub has_valid_creds: Option<bool>,
}

impl client::ResponseResult for CheckValidCredsResponse {}


/// Defines the properties and custom parameters for a data source.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [data sources get projects](ProjectDataSourceGetCall) (response)
/// * [locations data sources get projects](ProjectLocationDataSourceGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataSource {
    /// Indicates the type of authorization.
    #[serde(rename="authorizationType")]
    
    pub authorization_type: Option<String>,
    /// Data source client id which should be used to receive refresh token.
    #[serde(rename="clientId")]
    
    pub client_id: Option<String>,
    /// Specifies whether the data source supports automatic data refresh for the past few days, and how it's supported. For some data sources, data might not be complete until a few days later, so it's useful to refresh data automatically.
    #[serde(rename="dataRefreshType")]
    
    pub data_refresh_type: Option<String>,
    /// Data source id.
    #[serde(rename="dataSourceId")]
    
    pub data_source_id: Option<String>,
    /// Default data refresh window on days. Only meaningful when `data_refresh_type` = `SLIDING_WINDOW`.
    #[serde(rename="defaultDataRefreshWindowDays")]
    
    pub default_data_refresh_window_days: Option<i32>,
    /// Default data transfer schedule. Examples of valid schedules include: `1st,3rd monday of month 15:30`, `every wed,fri of jan,jun 13:15`, and `first sunday of quarter 00:00`.
    #[serde(rename="defaultSchedule")]
    
    pub default_schedule: Option<String>,
    /// User friendly data source description string.
    
    pub description: Option<String>,
    /// User friendly data source name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Url for the help document for this data source.
    #[serde(rename="helpUrl")]
    
    pub help_url: Option<String>,
    /// Disables backfilling and manual run scheduling for the data source.
    #[serde(rename="manualRunsDisabled")]
    
    pub manual_runs_disabled: Option<bool>,
    /// The minimum interval for scheduler to schedule runs.
    #[serde(rename="minimumScheduleInterval")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub minimum_schedule_interval: Option<client::chrono::Duration>,
    /// Output only. Data source resource name.
    
    pub name: Option<String>,
    /// Data source parameters.
    
    pub parameters: Option<Vec<DataSourceParameter>>,
    /// Api auth scopes for which refresh token needs to be obtained. These are scopes needed by a data source to prepare data and ingest them into BigQuery, e.g., https://www.googleapis.com/auth/bigquery
    
    pub scopes: Option<Vec<String>>,
    /// Specifies whether the data source supports a user defined schedule, or operates on the default schedule. When set to `true`, user can override default schedule.
    #[serde(rename="supportsCustomSchedule")]
    
    pub supports_custom_schedule: Option<bool>,
    /// Deprecated. This field has no effect.
    #[serde(rename="supportsMultipleTransfers")]
    
    pub supports_multiple_transfers: Option<bool>,
    /// Deprecated. This field has no effect.
    #[serde(rename="transferType")]
    
    pub transfer_type: Option<String>,
    /// The number of seconds to wait for an update from the data source before the Data Transfer Service marks the transfer as FAILED.
    #[serde(rename="updateDeadlineSeconds")]
    
    pub update_deadline_seconds: Option<i32>,
}

impl client::ResponseResult for DataSource {}


/// A parameter used to define custom fields in a data source definition.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataSourceParameter {
    /// All possible values for the parameter.
    #[serde(rename="allowedValues")]
    
    pub allowed_values: Option<Vec<String>>,
    /// If true, it should not be used in new transfers, and it should not be visible to users.
    
    pub deprecated: Option<bool>,
    /// Parameter description.
    
    pub description: Option<String>,
    /// Parameter display name in the user interface.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Deprecated. This field has no effect.
    
    pub fields: Option<Vec<DataSourceParameter>>,
    /// Cannot be changed after initial creation.
    
    pub immutable: Option<bool>,
    /// For integer and double values specifies maximum allowed value.
    #[serde(rename="maxValue")]
    
    pub max_value: Option<f64>,
    /// For integer and double values specifies minimum allowed value.
    #[serde(rename="minValue")]
    
    pub min_value: Option<f64>,
    /// Parameter identifier.
    #[serde(rename="paramId")]
    
    pub param_id: Option<String>,
    /// Deprecated. This field has no effect.
    
    pub recurse: Option<bool>,
    /// Deprecated. This field has no effect.
    
    pub repeated: Option<bool>,
    /// Is parameter required.
    
    pub required: Option<bool>,
    /// Parameter type.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Description of the requirements for this field, in case the user input does not fulfill the regex pattern or min/max values.
    #[serde(rename="validationDescription")]
    
    pub validation_description: Option<String>,
    /// URL to a help document to further explain the naming requirements.
    #[serde(rename="validationHelpUrl")]
    
    pub validation_help_url: Option<String>,
    /// Regular expression which can be used for parameter validation.
    #[serde(rename="validationRegex")]
    
    pub validation_regex: Option<String>,
}

impl client::Part for DataSourceParameter {}


/// Represents preferences for sending email notifications for transfer run events.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EmailPreferences {
    /// If true, email notifications will be sent on transfer run failures.
    #[serde(rename="enableFailureEmail")]
    
    pub enable_failure_email: Option<bool>,
}

impl client::Part for EmailPreferences {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations transfer configs runs delete projects](ProjectLocationTransferConfigRunDeleteCall) (response)
/// * [locations transfer configs delete projects](ProjectLocationTransferConfigDeleteCall) (response)
/// * [locations enroll data sources projects](ProjectLocationEnrollDataSourceCall) (response)
/// * [transfer configs runs delete projects](ProjectTransferConfigRunDeleteCall) (response)
/// * [transfer configs delete projects](ProjectTransferConfigDeleteCall) (response)
/// * [enroll data sources projects](ProjectEnrollDataSourceCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// A request to enroll a set of data sources so they are visible in the BigQuery UI’s `Transfer` tab.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations enroll data sources projects](ProjectLocationEnrollDataSourceCall) (request)
/// * [enroll data sources projects](ProjectEnrollDataSourceCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnrollDataSourcesRequest {
    /// Data sources that are enrolled. It is required to provide at least one data source id.
    #[serde(rename="dataSourceIds")]
    
    pub data_source_ids: Option<Vec<String>>,
}

impl client::RequestValue for EnrollDataSourcesRequest {}


/// Returns list of supported data sources and their metadata.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [data sources list projects](ProjectDataSourceListCall) (response)
/// * [locations data sources list projects](ProjectLocationDataSourceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListDataSourcesResponse {
    /// List of supported data sources and their transfer settings.
    #[serde(rename="dataSources")]
    
    pub data_sources: Option<Vec<DataSource>>,
    /// Output only. The next-pagination token. For multiple-page list results, this token can be used as the `ListDataSourcesRequest.page_token` to request the next page of list results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListDataSourcesResponse {}


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


/// The returned list of pipelines in the project.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations transfer configs list projects](ProjectLocationTransferConfigListCall) (response)
/// * [transfer configs list projects](ProjectTransferConfigListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTransferConfigsResponse {
    /// Output only. The next-pagination token. For multiple-page list results, this token can be used as the `ListTransferConfigsRequest.page_token` to request the next page of list results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Output only. The stored pipeline transfer configurations.
    #[serde(rename="transferConfigs")]
    
    pub transfer_configs: Option<Vec<TransferConfig>>,
}

impl client::ResponseResult for ListTransferConfigsResponse {}


/// The returned list transfer run messages.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations transfer configs runs transfer logs list projects](ProjectLocationTransferConfigRunTransferLogListCall) (response)
/// * [transfer configs runs transfer logs list projects](ProjectTransferConfigRunTransferLogListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTransferLogsResponse {
    /// Output only. The next-pagination token. For multiple-page list results, this token can be used as the `GetTransferRunLogRequest.page_token` to request the next page of list results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Output only. The stored pipeline transfer messages.
    #[serde(rename="transferMessages")]
    
    pub transfer_messages: Option<Vec<TransferMessage>>,
}

impl client::ResponseResult for ListTransferLogsResponse {}


/// The returned list of pipelines in the project.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations transfer configs runs list projects](ProjectLocationTransferConfigRunListCall) (response)
/// * [transfer configs runs list projects](ProjectTransferConfigRunListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTransferRunsResponse {
    /// Output only. The next-pagination token. For multiple-page list results, this token can be used as the `ListTransferRunsRequest.page_token` to request the next page of list results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Output only. The stored pipeline transfer runs.
    #[serde(rename="transferRuns")]
    
    pub transfer_runs: Option<Vec<TransferRun>>,
}

impl client::ResponseResult for ListTransferRunsResponse {}


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


/// Options customizing the data transfer schedule.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ScheduleOptions {
    /// If true, automatic scheduling of data transfer runs for this configuration will be disabled. The runs can be started on ad-hoc basis using StartManualTransferRuns API. When automatic scheduling is disabled, the TransferConfig.schedule field will be ignored.
    #[serde(rename="disableAutoScheduling")]
    
    pub disable_auto_scheduling: Option<bool>,
    /// Defines time to stop scheduling transfer runs. A transfer run cannot be scheduled at or after the end time. The end time can be changed at any moment. The time when a data transfer can be trigerred manually is not limited by this option.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Specifies time to start scheduling transfer runs. The first run will be scheduled at or after the start time according to a recurrence pattern defined in the schedule string. The start time can be changed at any moment. The time when a data transfer can be trigerred manually is not limited by this option.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for ScheduleOptions {}


/// A request to schedule transfer runs for a time range.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations transfer configs schedule runs projects](ProjectLocationTransferConfigScheduleRunCall) (request)
/// * [transfer configs schedule runs projects](ProjectTransferConfigScheduleRunCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ScheduleTransferRunsRequest {
    /// Required. End time of the range of transfer runs. For example, `"2017-05-30T00:00:00+00:00"`.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. Start time of the range of transfer runs. For example, `"2017-05-25T00:00:00+00:00"`.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for ScheduleTransferRunsRequest {}


/// A response to schedule transfer runs for a time range.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations transfer configs schedule runs projects](ProjectLocationTransferConfigScheduleRunCall) (response)
/// * [transfer configs schedule runs projects](ProjectTransferConfigScheduleRunCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ScheduleTransferRunsResponse {
    /// The transfer runs that were scheduled.
    
    pub runs: Option<Vec<TransferRun>>,
}

impl client::ResponseResult for ScheduleTransferRunsResponse {}


/// A request to start manual transfer runs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations transfer configs start manual runs projects](ProjectLocationTransferConfigStartManualRunCall) (request)
/// * [transfer configs start manual runs projects](ProjectTransferConfigStartManualRunCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StartManualTransferRunsRequest {
    /// Specific run_time for a transfer run to be started. The requested_run_time must not be in the future.
    #[serde(rename="requestedRunTime")]
    
    pub requested_run_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Time range for the transfer runs that should be started.
    #[serde(rename="requestedTimeRange")]
    
    pub requested_time_range: Option<TimeRange>,
}

impl client::RequestValue for StartManualTransferRunsRequest {}


/// A response to start manual transfer runs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations transfer configs start manual runs projects](ProjectLocationTransferConfigStartManualRunCall) (response)
/// * [transfer configs start manual runs projects](ProjectTransferConfigStartManualRunCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StartManualTransferRunsResponse {
    /// The transfer runs that were created.
    
    pub runs: Option<Vec<TransferRun>>,
}

impl client::ResponseResult for StartManualTransferRunsResponse {}


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


/// A specification for a time range, this will request transfer runs with run_time between start_time (inclusive) and end_time (exclusive).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeRange {
    /// End time of the range of transfer runs. For example, `"2017-05-30T00:00:00+00:00"`. The end_time must not be in the future. Creates transfer runs where run_time is in the range between start_time (inclusive) and end_time (exclusive).
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Start time of the range of transfer runs. For example, `"2017-05-25T00:00:00+00:00"`. The start_time must be strictly less than the end_time. Creates transfer runs where run_time is in the range between start_time (inclusive) and end_time (exclusive).
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for TimeRange {}


/// Represents a data transfer configuration. A transfer configuration contains all metadata needed to perform a data transfer. For example, `destination_dataset_id` specifies where data should be stored. When a new transfer configuration is created, the specified `destination_dataset_id` is created when needed and shared with the appropriate data source service account.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations transfer configs create projects](ProjectLocationTransferConfigCreateCall) (request|response)
/// * [locations transfer configs get projects](ProjectLocationTransferConfigGetCall) (response)
/// * [locations transfer configs patch projects](ProjectLocationTransferConfigPatchCall) (request|response)
/// * [transfer configs create projects](ProjectTransferConfigCreateCall) (request|response)
/// * [transfer configs get projects](ProjectTransferConfigGetCall) (response)
/// * [transfer configs patch projects](ProjectTransferConfigPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TransferConfig {
    /// The number of days to look back to automatically refresh the data. For example, if `data_refresh_window_days = 10`, then every day BigQuery reingests data for [today-10, today-1], rather than ingesting data for just [today-1]. Only valid if the data source supports the feature. Set the value to 0 to use the default value.
    #[serde(rename="dataRefreshWindowDays")]
    
    pub data_refresh_window_days: Option<i32>,
    /// Data source ID. This cannot be changed once data transfer is created. The full list of available data source IDs can be returned through an API call: https://cloud.google.com/bigquery-transfer/docs/reference/datatransfer/rest/v1/projects.locations.dataSources/list
    #[serde(rename="dataSourceId")]
    
    pub data_source_id: Option<String>,
    /// Output only. Region in which BigQuery dataset is located.
    #[serde(rename="datasetRegion")]
    
    pub dataset_region: Option<String>,
    /// The BigQuery target dataset id.
    #[serde(rename="destinationDatasetId")]
    
    pub destination_dataset_id: Option<String>,
    /// Is this config disabled. When set to true, no runs are scheduled for a given transfer.
    
    pub disabled: Option<bool>,
    /// User specified display name for the data transfer.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Email notifications will be sent according to these preferences to the email address of the user who owns this transfer config.
    #[serde(rename="emailPreferences")]
    
    pub email_preferences: Option<EmailPreferences>,
    /// The resource name of the transfer config. Transfer config names have the form `projects/{project_id}/locations/{region}/transferConfigs/{config_id}`. Where `config_id` is usually a uuid, even though it is not guaranteed or required. The name is ignored when creating a transfer config.
    
    pub name: Option<String>,
    /// Output only. Next time when data transfer will run.
    #[serde(rename="nextRunTime")]
    
    pub next_run_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Pub/Sub topic where notifications will be sent after transfer runs associated with this transfer config finish. The format for specifying a pubsub topic is: `projects/{project}/topics/{topic}`
    #[serde(rename="notificationPubsubTopic")]
    
    pub notification_pubsub_topic: Option<String>,
    /// Output only. Information about the user whose credentials are used to transfer data. Populated only for `transferConfigs.get` requests. In case the user information is not available, this field will not be populated.
    #[serde(rename="ownerInfo")]
    
    pub owner_info: Option<UserInfo>,
    /// Parameters specific to each data source. For more information see the bq tab in the 'Setting up a data transfer' section for each data source. For example the parameters for Cloud Storage transfers are listed here: https://cloud.google.com/bigquery-transfer/docs/cloud-storage-transfer#bq
    
    pub params: Option<HashMap<String, json::Value>>,
    /// Data transfer schedule. If the data source does not support a custom schedule, this should be empty. If it is empty, the default value for the data source will be used. The specified times are in UTC. Examples of valid format: `1st,3rd monday of month 15:30`, `every wed,fri of jan,jun 13:15`, and `first sunday of quarter 00:00`. See more explanation about the format here: https://cloud.google.com/appengine/docs/flexible/python/scheduling-jobs-with-cron-yaml#the_schedule_format NOTE: The minimum interval time between recurring transfers depends on the data source; refer to the documentation for your data source.
    
    pub schedule: Option<String>,
    /// Options customizing the data transfer schedule.
    #[serde(rename="scheduleOptions")]
    
    pub schedule_options: Option<ScheduleOptions>,
    /// Output only. State of the most recently updated transfer run.
    
    pub state: Option<String>,
    /// Output only. Data transfer modification time. Ignored by server on input.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Deprecated. Unique ID of the user on whose behalf transfer is done.
    #[serde(rename="userId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub user_id: Option<i64>,
}

impl client::RequestValue for TransferConfig {}
impl client::ResponseResult for TransferConfig {}


/// Represents a user facing message for a particular data transfer run.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TransferMessage {
    /// Message text.
    #[serde(rename="messageText")]
    
    pub message_text: Option<String>,
    /// Time when message was logged.
    #[serde(rename="messageTime")]
    
    pub message_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Message severity.
    
    pub severity: Option<String>,
}

impl client::Part for TransferMessage {}


/// Represents a data transfer run.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations transfer configs runs get projects](ProjectLocationTransferConfigRunGetCall) (response)
/// * [transfer configs runs get projects](ProjectTransferConfigRunGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TransferRun {
    /// Output only. Data source id.
    #[serde(rename="dataSourceId")]
    
    pub data_source_id: Option<String>,
    /// Output only. The BigQuery target dataset id.
    #[serde(rename="destinationDatasetId")]
    
    pub destination_dataset_id: Option<String>,
    /// Output only. Email notifications will be sent according to these preferences to the email address of the user who owns the transfer config this run was derived from.
    #[serde(rename="emailPreferences")]
    
    pub email_preferences: Option<EmailPreferences>,
    /// Output only. Time when transfer run ended. Parameter ignored by server for input requests.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Status of the transfer run.
    #[serde(rename="errorStatus")]
    
    pub error_status: Option<Status>,
    /// The resource name of the transfer run. Transfer run names have the form `projects/{project_id}/locations/{location}/transferConfigs/{config_id}/runs/{run_id}`. The name is ignored when creating a transfer run.
    
    pub name: Option<String>,
    /// Output only. Pub/Sub topic where a notification will be sent after this transfer run finishes. The format for specifying a pubsub topic is: `projects/{project}/topics/{topic}`
    #[serde(rename="notificationPubsubTopic")]
    
    pub notification_pubsub_topic: Option<String>,
    /// Output only. Parameters specific to each data source. For more information see the bq tab in the 'Setting up a data transfer' section for each data source. For example the parameters for Cloud Storage transfers are listed here: https://cloud.google.com/bigquery-transfer/docs/cloud-storage-transfer#bq
    
    pub params: Option<HashMap<String, json::Value>>,
    /// For batch transfer runs, specifies the date and time of the data should be ingested.
    #[serde(rename="runTime")]
    
    pub run_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Describes the schedule of this transfer run if it was created as part of a regular schedule. For batch transfer runs that are scheduled manually, this is empty. NOTE: the system might choose to delay the schedule depending on the current load, so `schedule_time` doesn't always match this.
    
    pub schedule: Option<String>,
    /// Minimum time after which a transfer run can be started.
    #[serde(rename="scheduleTime")]
    
    pub schedule_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Time when transfer run was started. Parameter ignored by server for input requests.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Data transfer run state. Ignored for input requests.
    
    pub state: Option<String>,
    /// Output only. Last time the data transfer run state was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Deprecated. Unique ID of the user on whose behalf transfer is done.
    #[serde(rename="userId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub user_id: Option<i64>,
}

impl client::ResponseResult for TransferRun {}


/// Information about a user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserInfo {
    /// E-mail address of the user.
    
    pub email: Option<String>,
}

impl client::Part for UserInfo {}


