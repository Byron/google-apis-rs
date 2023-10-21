use super::*;
/// Request to fetch stored line items.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [downloadlineitems lineitems](LineitemDownloadlineitemCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DownloadLineItemsRequest {
    /// File specification (column names, types, order) in which the line items will be returned. Default to EWF.
    #[serde(rename="fileSpec")]
    
    pub file_spec: Option<String>,
    /// Ids of the specified filter type used to filter line items to fetch. If omitted, all the line items will be returned.
    #[serde(rename="filterIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub filter_ids: Option<Vec<i64>>,
    /// Filter type used to filter line items to fetch.
    #[serde(rename="filterType")]
    
    pub filter_type: Option<String>,
    /// Format in which the line items will be returned. Default to CSV.
    
    pub format: Option<String>,
}

impl client::RequestValue for DownloadLineItemsRequest {}


/// Download line items response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [downloadlineitems lineitems](LineitemDownloadlineitemCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DownloadLineItemsResponse {
    /// Retrieved line items in CSV format. For more information about file formats, see Entity Write File Format.
    #[serde(rename="lineItems")]
    
    pub line_items: Option<String>,
}

impl client::ResponseResult for DownloadLineItemsResponse {}


/// Request to fetch stored inventory sources, campaigns, insertion orders, line items, YouTube ad groups and ads.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [download sdf](SdfDownloadCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DownloadRequest {
    /// File types that will be returned. If INVENTORY_SOURCE is requested, no other file types may be requested. Acceptable values are: - "AD" - "AD_GROUP" - "CAMPAIGN" - "INSERTION_ORDER" - "INVENTORY_SOURCE" - "LINE_ITEM" 
    #[serde(rename="fileTypes")]
    
    pub file_types: Option<Vec<String>>,
    /// The IDs of the specified filter type. This is used to filter entities to fetch. At least one ID must be specified.
    #[serde(rename="filterIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub filter_ids: Option<Vec<i64>>,
    /// Filter type used to filter entities to fetch. PARTNER_ID and INVENTORY_SOURCE_ID may only be used when downloading inventory sources.
    #[serde(rename="filterType")]
    
    pub filter_type: Option<String>,
    /// SDF Version (column names, types, order) in which the entities will be returned. Default to 5.
    
    pub version: Option<String>,
}

impl client::RequestValue for DownloadRequest {}


/// Download response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [download sdf](SdfDownloadCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DownloadResponse {
    /// Retrieved ad groups in SDF format.
    #[serde(rename="adGroups")]
    
    pub ad_groups: Option<String>,
    /// Retrieved ads in SDF format.
    
    pub ads: Option<String>,
    /// Retrieved campaigns in SDF format.
    
    pub campaigns: Option<String>,
    /// Retrieved insertion orders in SDF format.
    #[serde(rename="insertionOrders")]
    
    pub insertion_orders: Option<String>,
    /// no description provided
    #[serde(rename="inventorySources")]
    
    pub inventory_sources: Option<String>,
    /// Retrieved line items in SDF format.
    #[serde(rename="lineItems")]
    
    pub line_items: Option<String>,
}

impl client::ResponseResult for DownloadResponse {}


/// Filter used to match traffic data in your report.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FilterPair {
    /// Filter type.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
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
    /// Retrieved reports.
    
    pub reports: Option<Vec<Report>>,
}

impl client::ResponseResult for ListReportsResponse {}


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
    
    pub group_bys: Option<Vec<String>>,
    /// Deprecated. This field is no longer in use.
    #[serde(rename="includeInviteData")]
    
    pub include_invite_data: Option<bool>,
    /// Metrics to include as columns in your report.
    
    pub metrics: Option<Vec<String>>,
    /// Report type.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for Parameters {}


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
    
    pub data_range: Option<String>,
    /// Format of the generated report.
    
    pub format: Option<String>,
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
    
    pub frequency: Option<String>,
    /// Time of day at which a new report will be generated, represented as minutes past midnight. Range is 0 to 1439. Only applies to scheduled reports.
    #[serde(rename="nextRunMinuteOfDay")]
    
    pub next_run_minute_of_day: Option<i32>,
    /// Canonical timezone code for report generation time. Defaults to America/New_York.
    #[serde(rename="nextRunTimezoneCode")]
    
    pub next_run_timezone_code: Option<String>,
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
    
    pub error_code: Option<String>,
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
    
    pub format: Option<String>,
    /// The state of the report.
    
    pub state: Option<String>,
}

impl client::Part for ReportStatus {}


/// Represents the upload status of a row in the request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RowStatus {
    /// Whether the stored entity is changed as a result of upload.
    
    pub changed: Option<bool>,
    /// Entity Id.
    #[serde(rename="entityId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub entity_id: Option<i64>,
    /// Entity name.
    #[serde(rename="entityName")]
    
    pub entity_name: Option<String>,
    /// Reasons why the entity can't be uploaded.
    
    pub errors: Option<Vec<String>>,
    /// Whether the entity is persisted.
    
    pub persisted: Option<bool>,
    /// Row number.
    #[serde(rename="rowNumber")]
    
    pub row_number: Option<i32>,
}

impl client::Part for RowStatus {}


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
    
    pub data_range: Option<String>,
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


/// Request to upload line items.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [uploadlineitems lineitems](LineitemUploadlineitemCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UploadLineItemsRequest {
    /// Set to true to get upload status without actually persisting the line items.
    #[serde(rename="dryRun")]
    
    pub dry_run: Option<bool>,
    /// Format the line items are in. Default to CSV.
    
    pub format: Option<String>,
    /// Line items in CSV to upload. Refer to Entity Write File Format for more information on file format.
    #[serde(rename="lineItems")]
    
    pub line_items: Option<String>,
}

impl client::RequestValue for UploadLineItemsRequest {}


/// Upload line items response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [uploadlineitems lineitems](LineitemUploadlineitemCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UploadLineItemsResponse {
    /// Status of upload.
    #[serde(rename="uploadStatus")]
    
    pub upload_status: Option<UploadStatus>,
}

impl client::ResponseResult for UploadLineItemsResponse {}


/// Represents the status of upload.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UploadStatus {
    /// Reasons why upload can't be completed.
    
    pub errors: Option<Vec<String>>,
    /// Per-row upload status.
    #[serde(rename="rowStatus")]
    
    pub row_status: Option<Vec<RowStatus>>,
}

impl client::Part for UploadStatus {}


