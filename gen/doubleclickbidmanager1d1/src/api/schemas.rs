use super::*;
/// A channel grouping defines a set of rules that can be used to categorize events in a path report.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelGrouping {
    /// The name to apply to an event that does not match any of the rules in the channel grouping.
    #[serde(rename="fallbackName")]
    
    pub fallback_name: Option<String>,
    /// Channel Grouping name.
    
    pub name: Option<String>,
    /// Rules within Channel Grouping. There is a limit of 100 rules that can be set per channel grouping.
    
    pub rules: Option<Vec<Rule>>,
}

impl client::Part for ChannelGrouping {}


/// DisjunctiveMatchStatement that OR's all contained filters.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DisjunctiveMatchStatement {
    /// Filters. There is a limit of 100 filters that can be set per disjunctive match statement.
    #[serde(rename="eventFilters")]
    
    pub event_filters: Option<Vec<EventFilter>>,
}

impl client::Part for DisjunctiveMatchStatement {}


/// Defines the type of filter to be applied to the path, a DV360 event dimension filter.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventFilter {
    /// Filter on a dimension.
    #[serde(rename="dimensionFilter")]
    
    pub dimension_filter: Option<PathQueryOptionsFilter>,
}

impl client::Part for EventFilter {}


/// Filter used to match traffic data in your report.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FilterPair {
    /// Filter type.
    #[serde(rename="type")]
    
    pub type_: Option<FilterPairTypeEnum>,
    /// Filter value.
    
    pub value: Option<String>,
}

impl client::Part for FilterPair {}


/// List queries response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [listqueries queries](QueryListqueryCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListQueriesResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "doubleclickbidmanager#listQueriesResponse".
    
    pub kind: Option<String>,
    /// Next page's pagination token if one exists.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Retrieved queries.
    
    pub queries: Option<Vec<Query>>,
}

impl client::ResponseResult for ListQueriesResponse {}


/// List reports response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [listreports reports](ReportListreportCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListReportsResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "doubleclickbidmanager#listReportsResponse".
    
    pub kind: Option<String>,
    /// Next page's pagination token if one exists.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Retrieved reports.
    
    pub reports: Option<Vec<Report>>,
}

impl client::ResponseResult for ListReportsResponse {}


/// Additional query options.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Options {
    /// Set to true and filter your report by `FILTER_INSERTION_ORDER` or `FILTER_LINE_ITEM` to include data for audience lists specifically targeted by those items.
    #[serde(rename="includeOnlyTargetedUserLists")]
    
    pub include_only_targeted_user_lists: Option<bool>,
    /// Options that contain Path Filters and Custom Channel Groupings.
    #[serde(rename="pathQueryOptions")]
    
    pub path_query_options: Option<PathQueryOptions>,
}

impl client::Part for Options {}


/// Parameters of a query or report.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Parameters {
    /// Filters used to match traffic data in your report.
    
    pub filters: Option<Vec<FilterPair>>,
    /// Data is grouped by the filters listed in this field.
    #[serde(rename="groupBys")]
    
    pub group_bys: Option<Vec<ParameterGroupBysEnum>>,
    /// Deprecated. This field is no longer in use.
    #[serde(rename="includeInviteData")]
    
    pub include_invite_data: Option<bool>,
    /// Metrics to include as columns in your report.
    
    pub metrics: Option<Vec<ParameterMetricsEnum>>,
    /// Additional query options.
    
    pub options: Option<Options>,
    /// Report type.
    #[serde(rename="type")]
    
    pub type_: Option<ParameterTypeEnum>,
}

impl client::Part for Parameters {}


/// Path filters specify which paths to include in a report. A path is the result of combining DV360 events based on User ID to create a workflow of users' actions. When a path filter is set, the resulting report will only include paths that match the specified event at the specified position. All other paths will be excluded.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PathFilter {
    /// Filter on an event to be applied to some part of the path.
    #[serde(rename="eventFilters")]
    
    pub event_filters: Option<Vec<EventFilter>>,
    /// Indicates the position of the path the filter should match to (first, last, or any event in path).
    #[serde(rename="pathMatchPosition")]
    
    pub path_match_position: Option<PathFilterPathMatchPositionEnum>,
}

impl client::Part for PathFilter {}


/// Path Query Options for Report Options.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PathQueryOptions {
    /// Custom Channel Groupings.
    #[serde(rename="channelGrouping")]
    
    pub channel_grouping: Option<ChannelGrouping>,
    /// Path Filters. There is a limit of 100 path filters that can be set per report.
    #[serde(rename="pathFilters")]
    
    pub path_filters: Option<Vec<PathFilter>>,
}

impl client::Part for PathQueryOptions {}


/// Dimension Filter on path events.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PathQueryOptionsFilter {
    /// Dimension the filter is applied to.
    
    pub filter: Option<PathQueryOptionsFilterFilterEnum>,
    /// Indicates how the filter should be matched to the value.
    #[serde(rename="match")]
    
    pub match_: Option<PathQueryOptionsFilterMatchEnum>,
    /// Value to filter on.
    
    pub values: Option<Vec<String>>,
}

impl client::Part for PathQueryOptionsFilter {}


/// Represents a query.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [createquery queries](QueryCreatequeryCall) (request|response)
/// * [getquery queries](QueryGetqueryCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Query {
    /// Identifies what kind of resource this is. Value: the fixed string "doubleclickbidmanager#query".
    
    pub kind: Option<String>,
    /// Query metadata.
    
    pub metadata: Option<QueryMetadata>,
    /// Query parameters.
    
    pub params: Option<Parameters>,
    /// Query ID.
    #[serde(rename="queryId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub query_id: Option<i64>,
    /// The ending time for the data that is shown in the report. Note, reportDataEndTimeMs is required if metadata.dataRange is CUSTOM_DATES and ignored otherwise.
    #[serde(rename="reportDataEndTimeMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub report_data_end_time_ms: Option<i64>,
    /// The starting time for the data that is shown in the report. Note, reportDataStartTimeMs is required if metadata.dataRange is CUSTOM_DATES and ignored otherwise.
    #[serde(rename="reportDataStartTimeMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub report_data_start_time_ms: Option<i64>,
    /// Information on how often and when to run a query.
    
    pub schedule: Option<QuerySchedule>,
    /// Canonical timezone code for report data time. Defaults to America/New_York.
    #[serde(rename="timezoneCode")]
    
    pub timezone_code: Option<String>,
}

impl client::RequestValue for Query {}
impl client::ResponseResult for Query {}


/// Query metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryMetadata {
    /// Range of report data.
    #[serde(rename="dataRange")]
    
    pub data_range: Option<QueryMetadataDataRangeEnum>,
    /// Format of the generated report.
    
    pub format: Option<QueryMetadataFormatEnum>,
    /// The path to the location in Google Cloud Storage where the latest report is stored.
    #[serde(rename="googleCloudStoragePathForLatestReport")]
    
    pub google_cloud_storage_path_for_latest_report: Option<String>,
    /// The path in Google Drive for the latest report.
    #[serde(rename="googleDrivePathForLatestReport")]
    
    pub google_drive_path_for_latest_report: Option<String>,
    /// The time when the latest report started to run.
    #[serde(rename="latestReportRunTimeMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub latest_report_run_time_ms: Option<i64>,
    /// Locale of the generated reports. Valid values are cs CZECH de GERMAN en ENGLISH es SPANISH fr FRENCH it ITALIAN ja JAPANESE ko KOREAN pl POLISH pt-BR BRAZILIAN_PORTUGUESE ru RUSSIAN tr TURKISH uk UKRAINIAN zh-CN CHINA_CHINESE zh-TW TAIWAN_CHINESE An locale string not in the list above will generate reports in English.
    
    pub locale: Option<String>,
    /// Number of reports that have been generated for the query.
    #[serde(rename="reportCount")]
    
    pub report_count: Option<i32>,
    /// Whether the latest report is currently running.
    
    pub running: Option<bool>,
    /// Whether to send an email notification when a report is ready. Default to false.
    #[serde(rename="sendNotification")]
    
    pub send_notification: Option<bool>,
    /// List of email addresses which are sent email notifications when the report is finished. Separate from sendNotification.
    #[serde(rename="shareEmailAddress")]
    
    pub share_email_address: Option<Vec<String>>,
    /// Query title. It is used to name the reports generated from this query.
    
    pub title: Option<String>,
}

impl client::Part for QueryMetadata {}


/// Information on how frequently and when to run a query.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QuerySchedule {
    /// Datetime to periodically run the query until.
    #[serde(rename="endTimeMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub end_time_ms: Option<i64>,
    /// How often the query is run.
    
    pub frequency: Option<QueryScheduleFrequencyEnum>,
    /// Time of day at which a new report will be generated, represented as minutes past midnight. Range is 0 to 1439. Only applies to scheduled reports.
    #[serde(rename="nextRunMinuteOfDay")]
    
    pub next_run_minute_of_day: Option<i32>,
    /// Canonical timezone code for report generation time. Defaults to America/New_York.
    #[serde(rename="nextRunTimezoneCode")]
    
    pub next_run_timezone_code: Option<String>,
    /// When to start running the query. Not applicable to `ONE_TIME` frequency.
    #[serde(rename="startTimeMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub start_time_ms: Option<i64>,
}

impl client::Part for QuerySchedule {}


/// Represents a report.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [listreports reports](ReportListreportCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Report {
    /// Key used to identify a report.
    
    pub key: Option<ReportKey>,
    /// Report metadata.
    
    pub metadata: Option<ReportMetadata>,
    /// Report parameters.
    
    pub params: Option<Parameters>,
}

impl client::Resource for Report {}


/// An explanation of a report failure.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportFailure {
    /// Error code that shows why the report was not created.
    #[serde(rename="errorCode")]
    
    pub error_code: Option<ReportFailureErrorCodeEnum>,
}

impl client::Part for ReportFailure {}


/// Key used to identify a report.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportKey {
    /// Query ID.
    #[serde(rename="queryId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub query_id: Option<i64>,
    /// Report ID.
    #[serde(rename="reportId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub report_id: Option<i64>,
}

impl client::Part for ReportKey {}


/// Report metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportMetadata {
    /// The path to the location in Google Cloud Storage where the report is stored.
    #[serde(rename="googleCloudStoragePath")]
    
    pub google_cloud_storage_path: Option<String>,
    /// The ending time for the data that is shown in the report.
    #[serde(rename="reportDataEndTimeMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub report_data_end_time_ms: Option<i64>,
    /// The starting time for the data that is shown in the report.
    #[serde(rename="reportDataStartTimeMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub report_data_start_time_ms: Option<i64>,
    /// Report status.
    
    pub status: Option<ReportStatus>,
}

impl client::Part for ReportMetadata {}


/// Report status.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportStatus {
    /// If the report failed, this records the cause.
    
    pub failure: Option<ReportFailure>,
    /// The time when this report either completed successfully or failed.
    #[serde(rename="finishTimeMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub finish_time_ms: Option<i64>,
    /// The file type of the report.
    
    pub format: Option<ReportStatuFormatEnum>,
    /// The state of the report.
    
    pub state: Option<ReportStatuStateEnum>,
}

impl client::Part for ReportStatus {}


/// A Rule defines a name, and a boolean expression in [conjunctive normal form](http: //mathworld.wolfram.com/ConjunctiveNormalForm.html){.external} that can be // applied to a path event to determine if that name should be applied.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Rule {
    /// no description provided
    #[serde(rename="disjunctiveMatchStatements")]
    
    pub disjunctive_match_statements: Option<Vec<DisjunctiveMatchStatement>>,
    /// Rule name.
    
    pub name: Option<String>,
}

impl client::Part for Rule {}


/// Request to run a stored query to generate a report.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [runquery queries](QueryRunqueryCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RunQueryRequest {
    /// Report data range used to generate the report.
    #[serde(rename="dataRange")]
    
    pub data_range: Option<RunQueryRequestDataRangeEnum>,
    /// The ending time for the data that is shown in the report. Note, reportDataEndTimeMs is required if dataRange is CUSTOM_DATES and ignored otherwise.
    #[serde(rename="reportDataEndTimeMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub report_data_end_time_ms: Option<i64>,
    /// The starting time for the data that is shown in the report. Note, reportDataStartTimeMs is required if dataRange is CUSTOM_DATES and ignored otherwise.
    #[serde(rename="reportDataStartTimeMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub report_data_start_time_ms: Option<i64>,
    /// Canonical timezone code for report data time. Defaults to America/New_York.
    #[serde(rename="timezoneCode")]
    
    pub timezone_code: Option<String>,
}

impl client::RequestValue for RunQueryRequest {}


