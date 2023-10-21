use super::*;
/// A message containing availability data relevant to DoubleClick Search.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Availability {
    /// DS advertiser ID.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// DS agency ID.
    #[serde(rename="agencyId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub agency_id: Option<i64>,
    /// The time by which all conversions have been uploaded, in epoch millis UTC.
    #[serde(rename="availabilityTimestamp")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub availability_timestamp: Option<i64>,
    /// Customer ID of a client account in the new Search Ads 360 experience.
    #[serde(rename="customerId")]
    
    pub customer_id: Option<String>,
    /// The numeric segmentation identifier (for example, DoubleClick Search Floodlight activity ID).
    #[serde(rename="segmentationId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub segmentation_id: Option<i64>,
    /// The friendly segmentation identifier (for example, DoubleClick Search Floodlight activity name).
    #[serde(rename="segmentationName")]
    
    pub segmentation_name: Option<String>,
    /// The segmentation type that this availability is for (its default value is `FLOODLIGHT`).
    #[serde(rename="segmentationType")]
    
    pub segmentation_type: Option<String>,
}

impl client::Part for Availability {}


/// A conversion containing data relevant to DoubleClick Search.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Conversion {
    /// DS ad group ID.
    #[serde(rename="adGroupId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub ad_group_id: Option<i64>,
    /// DS ad ID.
    #[serde(rename="adId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub ad_id: Option<i64>,
    /// DS advertiser ID.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// DS agency ID.
    #[serde(rename="agencyId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub agency_id: Option<i64>,
    /// Available to advertisers only after contacting DoubleClick Search customer support.
    #[serde(rename="attributionModel")]
    
    pub attribution_model: Option<String>,
    /// DS campaign ID.
    #[serde(rename="campaignId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub campaign_id: Option<i64>,
    /// Sales channel for the product. Acceptable values are: - "`local`": a physical store - "`online`": an online store 
    
    pub channel: Option<String>,
    /// DS click ID for the conversion.
    #[serde(rename="clickId")]
    
    pub click_id: Option<String>,
    /// For offline conversions, advertisers provide this ID. Advertisers can specify any ID that is meaningful to them. Each conversion in a request must specify a unique ID, and the combination of ID and timestamp must be unique amongst all conversions within the advertiser. For online conversions, DS copies the `dsConversionId` or `floodlightOrderId` into this property depending on the advertiser's Floodlight instructions.
    #[serde(rename="conversionId")]
    
    pub conversion_id: Option<String>,
    /// The time at which the conversion was last modified, in epoch millis UTC.
    #[serde(rename="conversionModifiedTimestamp")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub conversion_modified_timestamp: Option<i64>,
    /// The time at which the conversion took place, in epoch millis UTC.
    #[serde(rename="conversionTimestamp")]
    
    pub conversion_timestamp: Option<String>,
    /// Available to advertisers only after contacting DoubleClick Search customer support.
    #[serde(rename="countMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count_millis: Option<i64>,
    /// DS criterion (keyword) ID.
    #[serde(rename="criterionId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub criterion_id: Option<i64>,
    /// The currency code for the conversion's revenue. Should be in ISO 4217 alphabetic (3-char) format.
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// Custom dimensions for the conversion, which can be used to filter data in a report.
    #[serde(rename="customDimension")]
    
    pub custom_dimension: Option<Vec<CustomDimension>>,
    /// Custom metrics for the conversion.
    #[serde(rename="customMetric")]
    
    pub custom_metric: Option<Vec<CustomMetric>>,
    /// Customer ID of a client account in the new Search Ads 360 experience.
    #[serde(rename="customerId")]
    
    pub customer_id: Option<String>,
    /// The type of device on which the conversion occurred.
    #[serde(rename="deviceType")]
    
    pub device_type: Option<String>,
    /// ID that DoubleClick Search generates for each conversion.
    #[serde(rename="dsConversionId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub ds_conversion_id: Option<i64>,
    /// DS engine account ID.
    #[serde(rename="engineAccountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub engine_account_id: Option<i64>,
    /// The Floodlight order ID provided by the advertiser for the conversion.
    #[serde(rename="floodlightOrderId")]
    
    pub floodlight_order_id: Option<String>,
    /// ID that DS generates and uses to uniquely identify the inventory account that contains the product.
    #[serde(rename="inventoryAccountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub inventory_account_id: Option<i64>,
    /// The country registered for the Merchant Center feed that contains the product. Use an ISO 3166 code to specify a country.
    #[serde(rename="productCountry")]
    
    pub product_country: Option<String>,
    /// DS product group ID.
    #[serde(rename="productGroupId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub product_group_id: Option<i64>,
    /// The product ID (SKU).
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
    /// The language registered for the Merchant Center feed that contains the product. Use an ISO 639 code to specify a language.
    #[serde(rename="productLanguage")]
    
    pub product_language: Option<String>,
    /// The quantity of this conversion, in millis.
    #[serde(rename="quantityMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub quantity_millis: Option<i64>,
    /// The revenue amount of this `TRANSACTION` conversion, in micros (value multiplied by 1000000, no decimal). For example, to specify a revenue value of "10" enter "10000000" (10 million) in your request.
    #[serde(rename="revenueMicros")]
    
    pub revenue_micros: Option<String>,
    /// The numeric segmentation identifier (for example, DoubleClick Search Floodlight activity ID).
    #[serde(rename="segmentationId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub segmentation_id: Option<i64>,
    /// The friendly segmentation identifier (for example, DoubleClick Search Floodlight activity name).
    #[serde(rename="segmentationName")]
    
    pub segmentation_name: Option<String>,
    /// The segmentation type of this conversion (for example, `FLOODLIGHT`).
    #[serde(rename="segmentationType")]
    
    pub segmentation_type: Option<String>,
    /// The state of the conversion, that is, either `ACTIVE` or `REMOVED`. Note: state DELETED is deprecated.
    
    pub state: Option<String>,
    /// The ID of the local store for which the product was advertised. Applicable only when the channel is "`local`".
    #[serde(rename="storeId")]
    
    pub store_id: Option<String>,
    /// The type of the conversion, that is, either `ACTION` or `TRANSACTION`. An `ACTION` conversion is an action by the user that has no monetarily quantifiable value, while a `TRANSACTION` conversion is an action that does have a monetarily quantifiable value. Examples are email list signups (`ACTION`) versus ecommerce purchases (`TRANSACTION`).
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for Conversion {}


/// A list of conversions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get conversion](ConversionGetCall) (response)
/// * [get by customer id conversion](ConversionGetByCustomerIdCall) (response)
/// * [insert conversion](ConversionInsertCall) (request|response)
/// * [update conversion](ConversionUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConversionList {
    /// The conversions being requested.
    
    pub conversion: Option<Vec<Conversion>>,
    /// Identifies this as a ConversionList resource. Value: the fixed string doubleclicksearch#conversionList.
    
    pub kind: Option<String>,
}

impl client::RequestValue for ConversionList {}
impl client::ResponseResult for ConversionList {}


/// A message containing the custom dimension.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomDimension {
    /// Custom dimension name.
    
    pub name: Option<String>,
    /// Custom dimension value.
    
    pub value: Option<String>,
}

impl client::Part for CustomDimension {}


/// A message containing the custom metric.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomMetric {
    /// Custom metric name.
    
    pub name: Option<String>,
    /// Custom metric numeric value.
    
    pub value: Option<f64>,
}

impl client::Part for CustomMetric {}


/// File returned to https://developers.google.com/search-ads/v2/reference/reports/getIdMappingFile.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get id mapping file reports](ReportGetIdMappingFileCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IdMappingFile { _never_set: Option<bool> }

impl client::ResponseResult for IdMappingFile {}


/// A DoubleClick Search report. This object contains the report request, some report metadata such as currency code, and the generated report rows or report files.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [generate reports](ReportGenerateCall) (response)
/// * [get reports](ReportGetCall) (response)
/// * [get file reports](ReportGetFileCall) (none)
/// * [get id mapping file reports](ReportGetIdMappingFileCall) (none)
/// * [request reports](ReportRequestCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Report {
    /// Asynchronous report only. Contains a list of generated report files once the report has successfully completed.
    
    pub files: Option<Vec<ReportFiles>>,
    /// Asynchronous report only. Id of the report.
    
    pub id: Option<String>,
    /// Asynchronous report only. True if and only if the report has completed successfully and the report files are ready to be downloaded.
    #[serde(rename="isReportReady")]
    
    pub is_report_ready: Option<bool>,
    /// Identifies this as a Report resource. Value: the fixed string `doubleclicksearch#report`.
    
    pub kind: Option<String>,
    /// The request that created the report. Optional fields not specified in the original request are filled with default values.
    
    pub request: Option<ReportRequest>,
    /// The number of report rows generated by the report, not including headers.
    #[serde(rename="rowCount")]
    
    pub row_count: Option<i32>,
    /// Synchronous report only. Generated report rows.
    
    pub rows: Option<Vec<ReportRow>>,
    /// The currency code of all monetary values produced in the report, including values that are set by users (e.g., keyword bid settings) and metrics (e.g., cost and revenue). The currency code of a report is determined by the `statisticsCurrency` field of the report request.
    #[serde(rename="statisticsCurrencyCode")]
    
    pub statistics_currency_code: Option<String>,
    /// If all statistics of the report are sourced from the same time zone, this would be it. Otherwise the field is unset.
    #[serde(rename="statisticsTimeZone")]
    
    pub statistics_time_zone: Option<String>,
}

impl client::Resource for Report {}
impl client::ResponseResult for Report {}


/// A request object used to create a DoubleClick Search report.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportApiColumnSpec {
    /// Name of a DoubleClick Search column to include in the report.
    #[serde(rename="columnName")]
    
    pub column_name: Option<String>,
    /// Segments a report by a custom dimension. The report must be scoped to an advertiser or lower, and the custom dimension must already be set up in DoubleClick Search. The custom dimension name, which appears in DoubleClick Search, is case sensitive.\ If used in a conversion report, returns the value of the specified custom dimension for the given conversion, if set. This column does not segment the conversion report.
    #[serde(rename="customDimensionName")]
    
    pub custom_dimension_name: Option<String>,
    /// Name of a custom metric to include in the report. The report must be scoped to an advertiser or lower, and the custom metric must already be set up in DoubleClick Search. The custom metric name, which appears in DoubleClick Search, is case sensitive.
    #[serde(rename="customMetricName")]
    
    pub custom_metric_name: Option<String>,
    /// Inclusive day in YYYY-MM-DD format. When provided, this overrides the overall time range of the report for this column only. Must be provided together with `startDate`.
    #[serde(rename="endDate")]
    
    pub end_date: Option<String>,
    /// Synchronous report only. Set to `true` to group by this column. Defaults to `false`.
    #[serde(rename="groupByColumn")]
    
    pub group_by_column: Option<bool>,
    /// Text used to identify this column in the report output; defaults to `columnName` or `savedColumnName` when not specified. This can be used to prevent collisions between DoubleClick Search columns and saved columns with the same name.
    #[serde(rename="headerText")]
    
    pub header_text: Option<String>,
    /// The platform that is used to provide data for the custom dimension. Acceptable values are "floodlight".
    #[serde(rename="platformSource")]
    
    pub platform_source: Option<String>,
    /// Returns metrics only for a specific type of product activity. Accepted values are: - "`sold`": returns metrics only for products that were sold - "`advertised`": returns metrics only for products that were advertised in a Shopping campaign, and that might or might not have been sold 
    #[serde(rename="productReportPerspective")]
    
    pub product_report_perspective: Option<String>,
    /// Name of a saved column to include in the report. The report must be scoped at advertiser or lower, and this saved column must already be created in the DoubleClick Search UI.
    #[serde(rename="savedColumnName")]
    
    pub saved_column_name: Option<String>,
    /// Inclusive date in YYYY-MM-DD format. When provided, this overrides the overall time range of the report for this column only. Must be provided together with `endDate`.
    #[serde(rename="startDate")]
    
    pub start_date: Option<String>,
}

impl client::Part for ReportApiColumnSpec {}


/// A request object used to create a DoubleClick Search report.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [generate reports](ReportGenerateCall) (request)
/// * [request reports](ReportRequestCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportRequest {
    /// The columns to include in the report. This includes both DoubleClick Search columns and saved columns. For DoubleClick Search columns, only the `columnName` parameter is required. For saved columns only the `savedColumnName` parameter is required. Both `columnName` and `savedColumnName` cannot be set in the same stanza.\ The maximum number of columns per request is 300.
    
    pub columns: Option<Vec<ReportApiColumnSpec>>,
    /// Format that the report should be returned in. Currently `csv` or `tsv` is supported.
    #[serde(rename="downloadFormat")]
    
    pub download_format: Option<String>,
    /// A list of filters to be applied to the report.\ The maximum number of filters per request is 300.
    
    pub filters: Option<Vec<ReportRequestFilters>>,
    /// Determines if removed entities should be included in the report. Defaults to `false`. Deprecated, please use `includeRemovedEntities` instead.
    #[serde(rename="includeDeletedEntities")]
    
    pub include_deleted_entities: Option<bool>,
    /// Determines if removed entities should be included in the report. Defaults to `false`.
    #[serde(rename="includeRemovedEntities")]
    
    pub include_removed_entities: Option<bool>,
    /// Asynchronous report only. The maximum number of rows per report file. A large report is split into many files based on this field. Acceptable values are `1000000` to `100000000`, inclusive.
    #[serde(rename="maxRowsPerFile")]
    
    pub max_rows_per_file: Option<i32>,
    /// Synchronous report only. A list of columns and directions defining sorting to be performed on the report rows.\ The maximum number of orderings per request is 300.
    #[serde(rename="orderBy")]
    
    pub order_by: Option<Vec<ReportRequestOrderBy>>,
    /// The reportScope is a set of IDs that are used to determine which subset of entities will be returned in the report. The full lineage of IDs from the lowest scoped level desired up through agency is required.
    #[serde(rename="reportScope")]
    
    pub report_scope: Option<ReportRequestReportScope>,
    /// Determines the type of rows that are returned in the report. For example, if you specify `reportType: keyword`, each row in the report will contain data about a keyword. See the [Types of Reports](https://developers.google.com/search-ads/v2/report-types/) reference for the columns that are available for each type.
    #[serde(rename="reportType")]
    
    pub report_type: Option<String>,
    /// Synchronous report only. The maximum number of rows to return; additional rows are dropped. Acceptable values are `0` to `10000`, inclusive. Defaults to `10000`.
    #[serde(rename="rowCount")]
    
    pub row_count: Option<i32>,
    /// Synchronous report only. Zero-based index of the first row to return. Acceptable values are `0` to `50000`, inclusive. Defaults to `0`.
    #[serde(rename="startRow")]
    
    pub start_row: Option<i32>,
    /// Specifies the currency in which monetary will be returned. Possible values are: `usd`, `agency` (valid if the report is scoped to agency or lower), `advertiser` (valid if the report is scoped to * advertiser or lower), or `account` (valid if the report is scoped to engine account or lower).
    #[serde(rename="statisticsCurrency")]
    
    pub statistics_currency: Option<String>,
    /// If metrics are requested in a report, this argument will be used to restrict the metrics to a specific time range.
    #[serde(rename="timeRange")]
    
    pub time_range: Option<ReportRequestTimeRange>,
    /// If `true`, the report would only be created if all the requested stat data are sourced from a single timezone. Defaults to `false`.
    #[serde(rename="verifySingleTimeZone")]
    
    pub verify_single_time_zone: Option<bool>,
}

impl client::RequestValue for ReportRequest {}


/// A row in a DoubleClick Search report.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportRow(pub Option<HashMap<String, json::Value>>);

impl client::Part for ReportRow {}


/// A saved column
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list saved columns](SavedColumnListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SavedColumn {
    /// Identifies this as a SavedColumn resource. Value: the fixed string doubleclicksearch#savedColumn.
    
    pub kind: Option<String>,
    /// The name of the saved column.
    #[serde(rename="savedColumnName")]
    
    pub saved_column_name: Option<String>,
    /// The type of data this saved column will produce.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Resource for SavedColumn {}


/// A list of saved columns. Advertisers create saved columns to report on Floodlight activities, Google Analytics goals, or custom KPIs. To request reports with saved columns, you’ll need the saved column names that are available from this list.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list saved columns](SavedColumnListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SavedColumnList {
    /// The saved columns being requested.
    
    pub items: Option<Vec<SavedColumn>>,
    /// Identifies this as a SavedColumnList resource. Value: the fixed string doubleclicksearch#savedColumnList.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for SavedColumnList {}


/// The request to update availability.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [update availability conversion](ConversionUpdateAvailabilityCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateAvailabilityRequest {
    /// The availabilities being requested.
    
    pub availabilities: Option<Vec<Availability>>,
}

impl client::RequestValue for UpdateAvailabilityRequest {}


/// The response to a update availability request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [update availability conversion](ConversionUpdateAvailabilityCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateAvailabilityResponse {
    /// The availabilities being returned.
    
    pub availabilities: Option<Vec<Availability>>,
}

impl client::ResponseResult for UpdateAvailabilityResponse {}


/// Asynchronous report only. Contains a list of generated report files once the report has successfully completed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportFiles {
    /// The size of this report file in bytes.
    #[serde(rename="byteCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub byte_count: Option<i64>,
    /// Use this url to download the report file.
    
    pub url: Option<String>,
}

impl client::NestedType for ReportFiles {}
impl client::Part for ReportFiles {}


/// A list of filters to be applied to the report.\ The maximum number of filters per request is 300.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportRequestFilters {
    /// Column to perform the filter on. This can be a DoubleClick Search column or a saved column.
    
    pub column: Option<ReportApiColumnSpec>,
    /// Operator to use in the filter. See the filter reference for a list of available operators.
    
    pub operator: Option<String>,
    /// A list of values to filter the column value against.\ The maximum number of filter values per request is 300.
    
    pub values: Option<Vec<json::Value>>,
}

impl client::NestedType for ReportRequestFilters {}
impl client::Part for ReportRequestFilters {}


/// Synchronous report only. A list of columns and directions defining sorting to be performed on the report rows.\ The maximum number of orderings per request is 300.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportRequestOrderBy {
    /// Column to perform the sort on. This can be a DoubleClick Search-defined column or a saved column.
    
    pub column: Option<ReportApiColumnSpec>,
    /// The sort direction, which is either `ascending` or `descending`.
    #[serde(rename="sortOrder")]
    
    pub sort_order: Option<String>,
}

impl client::NestedType for ReportRequestOrderBy {}
impl client::Part for ReportRequestOrderBy {}


/// The reportScope is a set of IDs that are used to determine which subset of entities will be returned in the report. The full lineage of IDs from the lowest scoped level desired up through agency is required.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportRequestReportScope {
    /// DS ad group ID.
    #[serde(rename="adGroupId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub ad_group_id: Option<i64>,
    /// DS ad ID.
    #[serde(rename="adId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub ad_id: Option<i64>,
    /// DS advertiser ID.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// DS agency ID.
    #[serde(rename="agencyId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub agency_id: Option<i64>,
    /// DS campaign ID.
    #[serde(rename="campaignId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub campaign_id: Option<i64>,
    /// DS engine account ID.
    #[serde(rename="engineAccountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub engine_account_id: Option<i64>,
    /// DS keyword ID.
    #[serde(rename="keywordId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub keyword_id: Option<i64>,
}

impl client::NestedType for ReportRequestReportScope {}
impl client::Part for ReportRequestReportScope {}


/// If metrics are requested in a report, this argument will be used to restrict the metrics to a specific time range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportRequestTimeRange {
    /// Inclusive UTC timestamp in RFC format, e.g., `2013-07-16T10:16:23.555Z`. See additional references on how changed attribute reports work.
    #[serde(rename="changedAttributesSinceTimestamp")]
    
    pub changed_attributes_since_timestamp: Option<String>,
    /// Inclusive UTC timestamp in RFC format, e.g., `2013-07-16T10:16:23.555Z`. See additional references on how changed metrics reports work.
    #[serde(rename="changedMetricsSinceTimestamp")]
    
    pub changed_metrics_since_timestamp: Option<String>,
    /// Inclusive date in YYYY-MM-DD format.
    #[serde(rename="endDate")]
    
    pub end_date: Option<String>,
    /// Inclusive date in YYYY-MM-DD format.
    #[serde(rename="startDate")]
    
    pub start_date: Option<String>,
}

impl client::NestedType for ReportRequestTimeRange {}
impl client::Part for ReportRequestTimeRange {}


