use super::*;
/// A metric actively restricted in creating the report.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActiveMetricRestriction {
    /// The name of the restricted metric.
    #[serde(rename="metricName")]
    
    pub metric_name: Option<String>,
    /// The reason for this metric's restriction.
    #[serde(rename="restrictedMetricTypes")]
    
    pub restricted_metric_types: Option<Vec<ActiveMetricRestrictionRestrictedMetricTypesEnum>>,
}

impl client::Part for ActiveMetricRestriction {}


/// The batch request containing multiple pivot report requests.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch run pivot reports properties](PropertyBatchRunPivotReportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchRunPivotReportsRequest {
    /// Individual requests. Each request has a separate pivot report response. Each batch request is allowed up to 5 requests.
    
    pub requests: Option<Vec<RunPivotReportRequest>>,
}

impl client::RequestValue for BatchRunPivotReportsRequest {}


/// The batch response containing multiple pivot reports.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch run pivot reports properties](PropertyBatchRunPivotReportCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchRunPivotReportsResponse {
    /// Identifies what kind of resource this message is. This `kind` is always the fixed string "analyticsData#batchRunPivotReports". Useful to distinguish between response types in JSON.
    
    pub kind: Option<String>,
    /// Individual responses. Each response has a separate pivot report request.
    #[serde(rename="pivotReports")]
    
    pub pivot_reports: Option<Vec<RunPivotReportResponse>>,
}

impl client::ResponseResult for BatchRunPivotReportsResponse {}


/// The batch request containing multiple report requests.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch run reports properties](PropertyBatchRunReportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchRunReportsRequest {
    /// Individual requests. Each request has a separate report response. Each batch request is allowed up to 5 requests.
    
    pub requests: Option<Vec<RunReportRequest>>,
}

impl client::RequestValue for BatchRunReportsRequest {}


/// The batch response containing multiple reports.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch run reports properties](PropertyBatchRunReportCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchRunReportsResponse {
    /// Identifies what kind of resource this message is. This `kind` is always the fixed string "analyticsData#batchRunReports". Useful to distinguish between response types in JSON.
    
    pub kind: Option<String>,
    /// Individual responses. Each response has a separate report request.
    
    pub reports: Option<Vec<RunReportResponse>>,
}

impl client::ResponseResult for BatchRunReportsResponse {}


/// To express that the result needs to be between two numbers (inclusive).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BetweenFilter {
    /// Begins with this number.
    #[serde(rename="fromValue")]
    
    pub from_value: Option<NumericValue>,
    /// Ends with this number.
    #[serde(rename="toValue")]
    
    pub to_value: Option<NumericValue>,
}

impl client::Part for BetweenFilter {}


/// Used to convert a dimension value to a single case.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CaseExpression {
    /// Name of a dimension. The name must refer back to a name in dimensions field of the request.
    #[serde(rename="dimensionName")]
    
    pub dimension_name: Option<String>,
}

impl client::Part for CaseExpression {}


/// The request for compatibility information for a report’s dimensions and metrics. Check compatibility provides a preview of the compatibility of a report; fields shared with the `runReport` request should be the same values as in your `runReport` request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [check compatibility properties](PropertyCheckCompatibilityCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CheckCompatibilityRequest {
    /// Filters the dimensions and metrics in the response to just this compatibility. Commonly used as `”compatibilityFilter”: “COMPATIBLE”` to only return compatible dimensions & metrics.
    #[serde(rename="compatibilityFilter")]
    
    pub compatibility_filter: Option<CheckCompatibilityRequestCompatibilityFilterEnum>,
    /// The filter clause of dimensions. `dimensionFilter` should be the same value as in your `runReport` request.
    #[serde(rename="dimensionFilter")]
    
    pub dimension_filter: Option<FilterExpression>,
    /// The dimensions in this report. `dimensions` should be the same value as in your `runReport` request.
    
    pub dimensions: Option<Vec<Dimension>>,
    /// The filter clause of metrics. `metricFilter` should be the same value as in your `runReport` request
    #[serde(rename="metricFilter")]
    
    pub metric_filter: Option<FilterExpression>,
    /// The metrics in this report. `metrics` should be the same value as in your `runReport` request.
    
    pub metrics: Option<Vec<Metric>>,
}

impl client::RequestValue for CheckCompatibilityRequest {}


/// The compatibility response with the compatibility of each dimension & metric.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [check compatibility properties](PropertyCheckCompatibilityCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CheckCompatibilityResponse {
    /// The compatibility of each dimension.
    #[serde(rename="dimensionCompatibilities")]
    
    pub dimension_compatibilities: Option<Vec<DimensionCompatibility>>,
    /// The compatibility of each metric.
    #[serde(rename="metricCompatibilities")]
    
    pub metric_compatibilities: Option<Vec<MetricCompatibility>>,
}

impl client::ResponseResult for CheckCompatibilityResponse {}


/// Defines a cohort selection criteria. A cohort is a group of users who share a common characteristic. For example, users with the same `firstSessionDate` belong to the same cohort.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Cohort {
    /// The cohort selects users whose first touch date is between start date and end date defined in the `dateRange`. This `dateRange` does not specify the full date range of event data that is present in a cohort report. In a cohort report, this `dateRange` is extended by the granularity and offset present in the `cohortsRange`; event data for the extended reporting date range is present in a cohort report. In a cohort request, this `dateRange` is required and the `dateRanges` in the `RunReportRequest` or `RunPivotReportRequest` must be unspecified. This `dateRange` should generally be aligned with the cohort's granularity. If `CohortsRange` uses daily granularity, this `dateRange` can be a single day. If `CohortsRange` uses weekly granularity, this `dateRange` can be aligned to a week boundary, starting at Sunday and ending Saturday. If `CohortsRange` uses monthly granularity, this `dateRange` can be aligned to a month, starting at the first and ending on the last day of the month.
    #[serde(rename="dateRange")]
    
    pub date_range: Option<DateRange>,
    /// Dimension used by the cohort. Required and only supports `firstSessionDate`.
    
    pub dimension: Option<String>,
    /// Assigns a name to this cohort. The dimension `cohort` is valued to this name in a report response. If set, cannot begin with `cohort_` or `RESERVED_`. If not set, cohorts are named by their zero based index `cohort_0`, `cohort_1`, etc.
    
    pub name: Option<String>,
}

impl client::Part for Cohort {}


/// Optional settings of a cohort report.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CohortReportSettings {
    /// If true, accumulates the result from first touch day to the end day. Not supported in `RunReportRequest`.
    
    pub accumulate: Option<bool>,
}

impl client::Part for CohortReportSettings {}


/// The specification of cohorts for a cohort report. Cohort reports create a time series of user retention for the cohort. For example, you could select the cohort of users that were acquired in the first week of September and follow that cohort for the next six weeks. Selecting the users acquired in the first week of September cohort is specified in the `cohort` object. Following that cohort for the next six weeks is specified in the `cohortsRange` object. For examples, see [Cohort Report Examples](https://developers.google.com/analytics/devguides/reporting/data/v1/advanced#cohort_report_examples). The report response could show a weekly time series where say your app has retained 60% of this cohort after three weeks and 25% of this cohort after six weeks. These two percentages can be calculated by the metric `cohortActiveUsers/cohortTotalUsers` and will be separate rows in the report.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CohortSpec {
    /// Optional settings for a cohort report.
    #[serde(rename="cohortReportSettings")]
    
    pub cohort_report_settings: Option<CohortReportSettings>,
    /// Defines the selection criteria to group users into cohorts. Most cohort reports define only a single cohort. If multiple cohorts are specified, each cohort can be recognized in the report by their name.
    
    pub cohorts: Option<Vec<Cohort>>,
    /// Cohort reports follow cohorts over an extended reporting date range. This range specifies an offset duration to follow the cohorts over.
    #[serde(rename="cohortsRange")]
    
    pub cohorts_range: Option<CohortsRange>,
}

impl client::Part for CohortSpec {}


/// Configures the extended reporting date range for a cohort report. Specifies an offset duration to follow the cohorts over.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CohortsRange {
    /// Required. `endOffset` specifies the end date of the extended reporting date range for a cohort report. `endOffset` can be any positive integer but is commonly set to 5 to 10 so that reports contain data on the cohort for the next several granularity time periods. If `granularity` is `DAILY`, the `endDate` of the extended reporting date range is `endDate` of the cohort plus `endOffset` days. If `granularity` is `WEEKLY`, the `endDate` of the extended reporting date range is `endDate` of the cohort plus `endOffset * 7` days. If `granularity` is `MONTHLY`, the `endDate` of the extended reporting date range is `endDate` of the cohort plus `endOffset * 30` days.
    #[serde(rename="endOffset")]
    
    pub end_offset: Option<i32>,
    /// Required. The granularity used to interpret the `startOffset` and `endOffset` for the extended reporting date range for a cohort report.
    
    pub granularity: Option<CohortsRangeGranularityEnum>,
    /// `startOffset` specifies the start date of the extended reporting date range for a cohort report. `startOffset` is commonly set to 0 so that reports contain data from the acquisition of the cohort forward. If `granularity` is `DAILY`, the `startDate` of the extended reporting date range is `startDate` of the cohort plus `startOffset` days. If `granularity` is `WEEKLY`, the `startDate` of the extended reporting date range is `startDate` of the cohort plus `startOffset * 7` days. If `granularity` is `MONTHLY`, the `startDate` of the extended reporting date range is `startDate` of the cohort plus `startOffset * 30` days.
    #[serde(rename="startOffset")]
    
    pub start_offset: Option<i32>,
}

impl client::Part for CohortsRange {}


/// Used to combine dimension values to a single dimension.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConcatenateExpression {
    /// The delimiter placed between dimension names. Delimiters are often single characters such as "|" or "," but can be longer strings. If a dimension value contains the delimiter, both will be present in response with no distinction. For example if dimension 1 value = "US,FR", dimension 2 value = "JP", and delimiter = ",", then the response will contain "US,FR,JP".
    
    pub delimiter: Option<String>,
    /// Names of dimensions. The names must refer back to names in the dimensions field of the request.
    #[serde(rename="dimensionNames")]
    
    pub dimension_names: Option<Vec<String>>,
}

impl client::Part for ConcatenateExpression {}


/// A contiguous set of days: startDate, startDate + 1, ..., endDate. Requests are allowed up to 4 date ranges.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DateRange {
    /// The inclusive end date for the query in the format `YYYY-MM-DD`. Cannot be before `start_date`. The format `NdaysAgo`, `yesterday`, or `today` is also accepted, and in that case, the date is inferred based on the property's reporting time zone.
    #[serde(rename="endDate")]
    
    pub end_date: Option<String>,
    /// Assigns a name to this date range. The dimension `dateRange` is valued to this name in a report response. If set, cannot begin with `date_range_` or `RESERVED_`. If not set, date ranges are named by their zero based index in the request: `date_range_0`, `date_range_1`, etc.
    
    pub name: Option<String>,
    /// The inclusive start date for the query in the format `YYYY-MM-DD`. Cannot be after `end_date`. The format `NdaysAgo`, `yesterday`, or `today` is also accepted, and in that case, the date is inferred based on the property's reporting time zone.
    #[serde(rename="startDate")]
    
    pub start_date: Option<String>,
}

impl client::Part for DateRange {}


/// Dimensions are attributes of your data. For example, the dimension city indicates the city from which an event originates. Dimension values in report responses are strings; for example, the city could be "Paris" or "New York". Requests are allowed up to 9 dimensions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Dimension {
    /// One dimension can be the result of an expression of multiple dimensions. For example, dimension "country, city": concatenate(country, ", ", city).
    #[serde(rename="dimensionExpression")]
    
    pub dimension_expression: Option<DimensionExpression>,
    /// The name of the dimension. See the [API Dimensions](https://developers.google.com/analytics/devguides/reporting/data/v1/api-schema#dimensions) for the list of dimension names. If `dimensionExpression` is specified, `name` can be any string that you would like within the allowed character set. For example if a `dimensionExpression` concatenates `country` and `city`, you could call that dimension `countryAndCity`. Dimension names that you choose must match the regular expression `^[a-zA-Z0-9_]$`. Dimensions are referenced by `name` in `dimensionFilter`, `orderBys`, `dimensionExpression`, and `pivots`.
    
    pub name: Option<String>,
}

impl client::Part for Dimension {}


/// The compatibility for a single dimension.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DimensionCompatibility {
    /// The compatibility of this dimension. If the compatibility is COMPATIBLE, this dimension can be successfully added to the report.
    
    pub compatibility: Option<DimensionCompatibilityCompatibilityEnum>,
    /// The dimension metadata contains the API name for this compatibility information. The dimension metadata also contains other helpful information like the UI name and description.
    #[serde(rename="dimensionMetadata")]
    
    pub dimension_metadata: Option<DimensionMetadata>,
}

impl client::Part for DimensionCompatibility {}


/// Used to express a dimension which is the result of a formula of multiple dimensions. Example usages: 1) lower_case(dimension) 2) concatenate(dimension1, symbol, dimension2).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DimensionExpression {
    /// Used to combine dimension values to a single dimension. For example, dimension "country, city": concatenate(country, ", ", city).
    
    pub concatenate: Option<ConcatenateExpression>,
    /// Used to convert a dimension value to lower case.
    #[serde(rename="lowerCase")]
    
    pub lower_case: Option<CaseExpression>,
    /// Used to convert a dimension value to upper case.
    #[serde(rename="upperCase")]
    
    pub upper_case: Option<CaseExpression>,
}

impl client::Part for DimensionExpression {}


/// Describes a dimension column in the report. Dimensions requested in a report produce column entries within rows and DimensionHeaders. However, dimensions used exclusively within filters or expressions do not produce columns in a report; correspondingly, those dimensions do not produce headers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DimensionHeader {
    /// The dimension's name.
    
    pub name: Option<String>,
}

impl client::Part for DimensionHeader {}


/// Explains a dimension.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DimensionMetadata {
    /// This dimension’s name. Useable in [Dimension](#Dimension)’s `name`. For example, `eventName`.
    #[serde(rename="apiName")]
    
    pub api_name: Option<String>,
    /// The display name of the category that this dimension belongs to. Similar dimensions and metrics are categorized together.
    
    pub category: Option<String>,
    /// True if the dimension is a custom dimension for this property.
    #[serde(rename="customDefinition")]
    
    pub custom_definition: Option<bool>,
    /// Still usable but deprecated names for this dimension. If populated, this dimension is available by either `apiName` or one of `deprecatedApiNames` for a period of time. After the deprecation period, the dimension will be available only by `apiName`.
    #[serde(rename="deprecatedApiNames")]
    
    pub deprecated_api_names: Option<Vec<String>>,
    /// Description of how this dimension is used and calculated.
    
    pub description: Option<String>,
    /// This dimension's name within the Google Analytics user interface. For example, `Event name`.
    #[serde(rename="uiName")]
    
    pub ui_name: Option<String>,
}

impl client::Part for DimensionMetadata {}


/// Sorts by dimension values.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DimensionOrderBy {
    /// A dimension name in the request to order by.
    #[serde(rename="dimensionName")]
    
    pub dimension_name: Option<String>,
    /// Controls the rule for dimension value ordering.
    #[serde(rename="orderType")]
    
    pub order_type: Option<DimensionOrderByOrderTypeEnum>,
}

impl client::Part for DimensionOrderBy {}


/// The value of a dimension.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DimensionValue {
    /// Value as a string if the dimension type is a string.
    
    pub value: Option<String>,
}

impl client::Part for DimensionValue {}


/// An expression to filter dimension or metric values.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Filter {
    /// A filter for two values.
    #[serde(rename="betweenFilter")]
    
    pub between_filter: Option<BetweenFilter>,
    /// The dimension name or metric name. In most methods, dimensions & metrics can be used for the first time in this field. However in a RunPivotReportRequest, this field must be additionally specified by name in the RunPivotReportRequest's dimensions or metrics.
    #[serde(rename="fieldName")]
    
    pub field_name: Option<String>,
    /// A filter for in list values.
    #[serde(rename="inListFilter")]
    
    pub in_list_filter: Option<InListFilter>,
    /// A filter for numeric or date values.
    #[serde(rename="numericFilter")]
    
    pub numeric_filter: Option<NumericFilter>,
    /// Strings related filter.
    #[serde(rename="stringFilter")]
    
    pub string_filter: Option<StringFilter>,
}

impl client::Part for Filter {}


/// To express dimension or metric filters. The fields in the same FilterExpression need to be either all dimensions or all metrics.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FilterExpression {
    /// The FilterExpressions in and_group have an AND relationship.
    #[serde(rename="andGroup")]
    
    pub and_group: Option<FilterExpressionList>,
    /// A primitive filter. In the same FilterExpression, all of the filter's field names need to be either all dimensions or all metrics.
    
    pub filter: Option<Filter>,
    /// The FilterExpression is NOT of not_expression.
    #[serde(rename="notExpression")]
    
    pub not_expression: Option<Option<Box<FilterExpression>>>,
    /// The FilterExpressions in or_group have an OR relationship.
    #[serde(rename="orGroup")]
    
    pub or_group: Option<FilterExpressionList>,
}

impl client::Part for FilterExpression {}


/// A list of filter expressions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FilterExpressionList {
    /// A list of filter expressions.
    
    pub expressions: Option<Vec<FilterExpression>>,
}

impl client::Part for FilterExpressionList {}


/// The result needs to be in a list of string values.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InListFilter {
    /// If true, the string value is case sensitive.
    #[serde(rename="caseSensitive")]
    
    pub case_sensitive: Option<bool>,
    /// The list of string values. Must be non-empty.
    
    pub values: Option<Vec<String>>,
}

impl client::Part for InListFilter {}


/// The dimensions and metrics currently accepted in reporting methods.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get metadata properties](PropertyGetMetadataCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    /// The dimension descriptions.
    
    pub dimensions: Option<Vec<DimensionMetadata>>,
    /// The metric descriptions.
    
    pub metrics: Option<Vec<MetricMetadata>>,
    /// Resource name of this metadata.
    
    pub name: Option<String>,
}

impl client::ResponseResult for Metadata {}


/// The quantitative measurements of a report. For example, the metric `eventCount` is the total number of events. Requests are allowed up to 10 metrics.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Metric {
    /// A mathematical expression for derived metrics. For example, the metric Event count per user is `eventCount/totalUsers`.
    
    pub expression: Option<String>,
    /// Indicates if a metric is invisible in the report response. If a metric is invisible, the metric will not produce a column in the response, but can be used in `metricFilter`, `orderBys`, or a metric `expression`.
    
    pub invisible: Option<bool>,
    /// The name of the metric. See the [API Metrics](https://developers.google.com/analytics/devguides/reporting/data/v1/api-schema#metrics) for the list of metric names. If `expression` is specified, `name` can be any string that you would like within the allowed character set. For example if `expression` is `screenPageViews/sessions`, you could call that metric's name = `viewsPerSession`. Metric names that you choose must match the regular expression `^[a-zA-Z0-9_]$`. Metrics are referenced by `name` in `metricFilter`, `orderBys`, and metric `expression`.
    
    pub name: Option<String>,
}

impl client::Part for Metric {}


/// The compatibility for a single metric.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricCompatibility {
    /// The compatibility of this metric. If the compatibility is COMPATIBLE, this metric can be successfully added to the report.
    
    pub compatibility: Option<MetricCompatibilityCompatibilityEnum>,
    /// The metric metadata contains the API name for this compatibility information. The metric metadata also contains other helpful information like the UI name and description.
    #[serde(rename="metricMetadata")]
    
    pub metric_metadata: Option<MetricMetadata>,
}

impl client::Part for MetricCompatibility {}


/// Describes a metric column in the report. Visible metrics requested in a report produce column entries within rows and MetricHeaders. However, metrics used exclusively within filters or expressions do not produce columns in a report; correspondingly, those metrics do not produce headers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricHeader {
    /// The metric's name.
    
    pub name: Option<String>,
    /// The metric's data type.
    #[serde(rename="type")]
    
    pub type_: Option<MetricHeaderTypeEnum>,
}

impl client::Part for MetricHeader {}


/// Explains a metric.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricMetadata {
    /// A metric name. Useable in [Metric](#Metric)’s `name`. For example, `eventCount`.
    #[serde(rename="apiName")]
    
    pub api_name: Option<String>,
    /// If reasons are specified, your access is blocked to this metric for this property. API requests from you to this property for this metric will succeed; however, the report will contain only zeros for this metric. API requests with metric filters on blocked metrics will fail. If reasons are empty, you have access to this metric. To learn more, see [Access and data-restriction management](https://support.google.com/analytics/answer/10851388).
    #[serde(rename="blockedReasons")]
    
    pub blocked_reasons: Option<Vec<MetricMetadataBlockedReasonsEnum>>,
    /// The display name of the category that this metrics belongs to. Similar dimensions and metrics are categorized together.
    
    pub category: Option<String>,
    /// True if the metric is a custom metric for this property.
    #[serde(rename="customDefinition")]
    
    pub custom_definition: Option<bool>,
    /// Still usable but deprecated names for this metric. If populated, this metric is available by either `apiName` or one of `deprecatedApiNames` for a period of time. After the deprecation period, the metric will be available only by `apiName`.
    #[serde(rename="deprecatedApiNames")]
    
    pub deprecated_api_names: Option<Vec<String>>,
    /// Description of how this metric is used and calculated.
    
    pub description: Option<String>,
    /// The mathematical expression for this derived metric. Can be used in [Metric](#Metric)’s `expression` field for equivalent reports. Most metrics are not expressions, and for non-expressions, this field is empty.
    
    pub expression: Option<String>,
    /// The type of this metric.
    #[serde(rename="type")]
    
    pub type_: Option<MetricMetadataTypeEnum>,
    /// This metric's name within the Google Analytics user interface. For example, `Event count`.
    #[serde(rename="uiName")]
    
    pub ui_name: Option<String>,
}

impl client::Part for MetricMetadata {}


/// Sorts by metric values.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricOrderBy {
    /// A metric name in the request to order by.
    #[serde(rename="metricName")]
    
    pub metric_name: Option<String>,
}

impl client::Part for MetricOrderBy {}


/// The value of a metric.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricValue {
    /// Measurement value. See MetricHeader for type.
    
    pub value: Option<String>,
}

impl client::Part for MetricValue {}


/// A contiguous set of minutes: startMinutesAgo, startMinutesAgo + 1, ..., endMinutesAgo. Requests are allowed up to 2 minute ranges.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MinuteRange {
    /// The inclusive end minute for the query as a number of minutes before now. Cannot be before `startMinutesAgo`. For example, `"endMinutesAgo": 15` specifies the report should include event data from prior to 15 minutes ago. If unspecified, `endMinutesAgo` is defaulted to 0. Standard Analytics properties can request any minute in the last 30 minutes of event data (`endMinutesAgo <= 29`), and 360 Analytics properties can request any minute in the last 60 minutes of event data (`endMinutesAgo <= 59`).
    #[serde(rename="endMinutesAgo")]
    
    pub end_minutes_ago: Option<i32>,
    /// Assigns a name to this minute range. The dimension `dateRange` is valued to this name in a report response. If set, cannot begin with `date_range_` or `RESERVED_`. If not set, minute ranges are named by their zero based index in the request: `date_range_0`, `date_range_1`, etc.
    
    pub name: Option<String>,
    /// The inclusive start minute for the query as a number of minutes before now. For example, `"startMinutesAgo": 29` specifies the report should include event data from 29 minutes ago and after. Cannot be after `endMinutesAgo`. If unspecified, `startMinutesAgo` is defaulted to 29. Standard Analytics properties can request up to the last 30 minutes of event data (`startMinutesAgo <= 29`), and 360 Analytics properties can request up to the last 60 minutes of event data (`startMinutesAgo <= 59`).
    #[serde(rename="startMinutesAgo")]
    
    pub start_minutes_ago: Option<i32>,
}

impl client::Part for MinuteRange {}


/// Filters for numeric or date values.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NumericFilter {
    /// The operation type for this filter.
    
    pub operation: Option<NumericFilterOperationEnum>,
    /// A numeric value or a date value.
    
    pub value: Option<NumericValue>,
}

impl client::Part for NumericFilter {}


/// To represent a number.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NumericValue {
    /// Double value
    #[serde(rename="doubleValue")]
    
    pub double_value: Option<f64>,
    /// Integer value
    #[serde(rename="int64Value")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub int64_value: Option<i64>,
}

impl client::Part for NumericValue {}


/// Order bys define how rows will be sorted in the response. For example, ordering rows by descending event count is one ordering, and ordering rows by the event name string is a different ordering.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderBy {
    /// If true, sorts by descending order.
    
    pub desc: Option<bool>,
    /// Sorts results by a dimension's values.
    
    pub dimension: Option<DimensionOrderBy>,
    /// Sorts results by a metric's values.
    
    pub metric: Option<MetricOrderBy>,
    /// Sorts results by a metric's values within a pivot column group.
    
    pub pivot: Option<PivotOrderBy>,
}

impl client::Part for OrderBy {}


/// Describes the visible dimension columns and rows in the report response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Pivot {
    /// Dimension names for visible columns in the report response. Including "dateRange" produces a date range column; for each row in the response, dimension values in the date range column will indicate the corresponding date range from the request.
    #[serde(rename="fieldNames")]
    
    pub field_names: Option<Vec<String>>,
    /// The number of unique combinations of dimension values to return in this pivot. The `limit` parameter is required. A `limit` of 10,000 is common for single pivot requests. The product of the `limit` for each `pivot` in a `RunPivotReportRequest` must not exceed 100,000. For example, a two pivot request with `limit: 1000` in each pivot will fail because the product is `1,000,000`.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub limit: Option<i64>,
    /// Aggregate the metrics by dimensions in this pivot using the specified metric_aggregations.
    #[serde(rename="metricAggregations")]
    
    pub metric_aggregations: Option<Vec<PivotMetricAggregationsEnum>>,
    /// The row count of the start row. The first row is counted as row 0.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub offset: Option<i64>,
    /// Specifies how dimensions are ordered in the pivot. In the first Pivot, the OrderBys determine Row and PivotDimensionHeader ordering; in subsequent Pivots, the OrderBys determine only PivotDimensionHeader ordering. Dimensions specified in these OrderBys must be a subset of Pivot.field_names.
    #[serde(rename="orderBys")]
    
    pub order_bys: Option<Vec<OrderBy>>,
}

impl client::Part for Pivot {}


/// Summarizes dimension values from a row for this pivot.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PivotDimensionHeader {
    /// Values of multiple dimensions in a pivot.
    #[serde(rename="dimensionValues")]
    
    pub dimension_values: Option<Vec<DimensionValue>>,
}

impl client::Part for PivotDimensionHeader {}


/// Dimensions' values in a single pivot.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PivotHeader {
    /// The size is the same as the cardinality of the corresponding dimension combinations.
    #[serde(rename="pivotDimensionHeaders")]
    
    pub pivot_dimension_headers: Option<Vec<PivotDimensionHeader>>,
    /// The cardinality of the pivot. The total number of rows for this pivot's fields regardless of how the parameters `offset` and `limit` are specified in the request.
    #[serde(rename="rowCount")]
    
    pub row_count: Option<i32>,
}

impl client::Part for PivotHeader {}


/// Sorts by a pivot column group.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PivotOrderBy {
    /// In the response to order by, order rows by this column. Must be a metric name from the request.
    #[serde(rename="metricName")]
    
    pub metric_name: Option<String>,
    /// Used to select a dimension name and value pivot. If multiple pivot selections are given, the sort occurs on rows where all pivot selection dimension name and value pairs match the row's dimension name and value pair.
    #[serde(rename="pivotSelections")]
    
    pub pivot_selections: Option<Vec<PivotSelection>>,
}

impl client::Part for PivotOrderBy {}


/// A pair of dimension names and values. Rows with this dimension pivot pair are ordered by the metric's value. For example if pivots = {{"browser", "Chrome"}} and metric_name = "Sessions", then the rows will be sorted based on Sessions in Chrome. ---------|----------|----------------|----------|---------------- | Chrome | Chrome | Safari | Safari ---------|----------|----------------|----------|---------------- Country | Sessions | Pages/Sessions | Sessions | Pages/Sessions ---------|----------|----------------|----------|---------------- US | 2 | 2 | 3 | 1 ---------|----------|----------------|----------|---------------- Canada | 3 | 1 | 4 | 1 ---------|----------|----------------|----------|----------------
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PivotSelection {
    /// Must be a dimension name from the request.
    #[serde(rename="dimensionName")]
    
    pub dimension_name: Option<String>,
    /// Order by only when the named dimension is this value.
    #[serde(rename="dimensionValue")]
    
    pub dimension_value: Option<String>,
}

impl client::Part for PivotSelection {}


/// Current state of all quotas for this Analytics Property. If any quota for a property is exhausted, all requests to that property will return Resource Exhausted errors.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PropertyQuota {
    /// Standard Analytics Properties can send up to 10 concurrent requests; Analytics 360 Properties can use up to 50 concurrent requests.
    #[serde(rename="concurrentRequests")]
    
    pub concurrent_requests: Option<QuotaStatus>,
    /// Analytics Properties can send up to 120 requests with potentially thresholded dimensions per hour. In a batch request, each report request is individually counted for this quota if the request contains potentially thresholded dimensions.
    #[serde(rename="potentiallyThresholdedRequestsPerHour")]
    
    pub potentially_thresholded_requests_per_hour: Option<QuotaStatus>,
    /// Standard Analytics Properties and cloud project pairs can have up to 10 server errors per hour; Analytics 360 Properties and cloud project pairs can have up to 50 server errors per hour.
    #[serde(rename="serverErrorsPerProjectPerHour")]
    
    pub server_errors_per_project_per_hour: Option<QuotaStatus>,
    /// Standard Analytics Properties can use up to 25,000 tokens per day; Analytics 360 Properties can use 250,000 tokens per day. Most requests consume fewer than 10 tokens.
    #[serde(rename="tokensPerDay")]
    
    pub tokens_per_day: Option<QuotaStatus>,
    /// Standard Analytics Properties can use up to 5,000 tokens per hour; Analytics 360 Properties can use 50,000 tokens per hour. An API request consumes a single number of tokens, and that number is deducted from all of the hourly, daily, and per project hourly quotas.
    #[serde(rename="tokensPerHour")]
    
    pub tokens_per_hour: Option<QuotaStatus>,
    /// Analytics Properties can use up to 25% of their tokens per project per hour. This amounts to standard Analytics Properties can use up to 1,250 tokens per project per hour, and Analytics 360 Properties can use 12,500 tokens per project per hour. An API request consumes a single number of tokens, and that number is deducted from all of the hourly, daily, and per project hourly quotas.
    #[serde(rename="tokensPerProjectPerHour")]
    
    pub tokens_per_project_per_hour: Option<QuotaStatus>,
}

impl client::Part for PropertyQuota {}


/// Current state for a particular quota group.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QuotaStatus {
    /// Quota consumed by this request.
    
    pub consumed: Option<i32>,
    /// Quota remaining after this request.
    
    pub remaining: Option<i32>,
}

impl client::Part for QuotaStatus {}


/// Response's metadata carrying additional information about the report content.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResponseMetaData {
    /// The currency code used in this report. Intended to be used in formatting currency metrics like `purchaseRevenue` for visualization. If currency_code was specified in the request, this response parameter will echo the request parameter; otherwise, this response parameter is the property's current currency_code. Currency codes are string encodings of currency types from the ISO 4217 standard (https://en.wikipedia.org/wiki/ISO_4217); for example "USD", "EUR", "JPY". To learn more, see https://support.google.com/analytics/answer/9796179.
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// If true, indicates some buckets of dimension combinations are rolled into "(other)" row. This can happen for high cardinality reports.
    #[serde(rename="dataLossFromOtherRow")]
    
    pub data_loss_from_other_row: Option<bool>,
    /// If empty reason is specified, the report is empty for this reason.
    #[serde(rename="emptyReason")]
    
    pub empty_reason: Option<String>,
    /// Describes the schema restrictions actively enforced in creating this report. To learn more, see [Access and data-restriction management](https://support.google.com/analytics/answer/10851388).
    #[serde(rename="schemaRestrictionResponse")]
    
    pub schema_restriction_response: Option<SchemaRestrictionResponse>,
    /// If `subjectToThresholding` is true, this report is subject to thresholding and only returns data that meets the minimum aggregation thresholds. It is possible for a request to be subject to thresholding thresholding and no data is absent from the report, and this happens when all data is above the thresholds. To learn more, see [Data thresholds](https://support.google.com/analytics/answer/9383630) and [About Demographics and Interests](https://support.google.com/analytics/answer/2799357).
    #[serde(rename="subjectToThresholding")]
    
    pub subject_to_thresholding: Option<bool>,
    /// The property's current timezone. Intended to be used to interpret time-based dimensions like `hour` and `minute`. Formatted as strings from the IANA Time Zone database (https://www.iana.org/time-zones); for example "America/New_York" or "Asia/Tokyo".
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<String>,
}

impl client::Part for ResponseMetaData {}


/// Report data for each row. For example if RunReportRequest contains: ```none "dimensions": [ { "name": "eventName" }, { "name": "countryId" } ], "metrics": [ { "name": "eventCount" } ] ``` One row with 'in_app_purchase' as the eventName, 'JP' as the countryId, and 15 as the eventCount, would be: ```none "dimensionValues": [ { "value": "in_app_purchase" }, { "value": "JP" } ], "metricValues": [ { "value": "15" } ] ```
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Row {
    /// List of requested dimension values. In a PivotReport, dimension_values are only listed for dimensions included in a pivot.
    #[serde(rename="dimensionValues")]
    
    pub dimension_values: Option<Vec<DimensionValue>>,
    /// List of requested visible metric values.
    #[serde(rename="metricValues")]
    
    pub metric_values: Option<Vec<MetricValue>>,
}

impl client::Part for Row {}


/// The request to generate a pivot report.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [run pivot report properties](PropertyRunPivotReportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RunPivotReportRequest {
    /// Cohort group associated with this request. If there is a cohort group in the request the 'cohort' dimension must be present.
    #[serde(rename="cohortSpec")]
    
    pub cohort_spec: Option<CohortSpec>,
    /// A currency code in ISO4217 format, such as "AED", "USD", "JPY". If the field is empty, the report uses the property's default currency.
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// The date range to retrieve event data for the report. If multiple date ranges are specified, event data from each date range is used in the report. A special dimension with field name "dateRange" can be included in a Pivot's field names; if included, the report compares between date ranges. In a cohort request, this `dateRanges` must be unspecified.
    #[serde(rename="dateRanges")]
    
    pub date_ranges: Option<Vec<DateRange>>,
    /// The filter clause of dimensions. Dimensions must be requested to be used in this filter. Metrics cannot be used in this filter.
    #[serde(rename="dimensionFilter")]
    
    pub dimension_filter: Option<FilterExpression>,
    /// The dimensions requested. All defined dimensions must be used by one of the following: dimension_expression, dimension_filter, pivots, order_bys.
    
    pub dimensions: Option<Vec<Dimension>>,
    /// If false or unspecified, each row with all metrics equal to 0 will not be returned. If true, these rows will be returned if they are not separately removed by a filter.
    #[serde(rename="keepEmptyRows")]
    
    pub keep_empty_rows: Option<bool>,
    /// The filter clause of metrics. Applied at post aggregation phase, similar to SQL having-clause. Metrics must be requested to be used in this filter. Dimensions cannot be used in this filter.
    #[serde(rename="metricFilter")]
    
    pub metric_filter: Option<FilterExpression>,
    /// The metrics requested, at least one metric needs to be specified. All defined metrics must be used by one of the following: metric_expression, metric_filter, order_bys.
    
    pub metrics: Option<Vec<Metric>>,
    /// Describes the visual format of the report's dimensions in columns or rows. The union of the fieldNames (dimension names) in all pivots must be a subset of dimension names defined in Dimensions. No two pivots can share a dimension. A dimension is only visible if it appears in a pivot.
    
    pub pivots: Option<Vec<Pivot>>,
    /// A Google Analytics GA4 property identifier whose events are tracked. Specified in the URL path and not the body. To learn more, see [where to find your Property ID](https://developers.google.com/analytics/devguides/reporting/data/v1/property-id). Within a batch request, this property should either be unspecified or consistent with the batch-level property. Example: properties/1234
    
    pub property: Option<String>,
    /// Toggles whether to return the current state of this Analytics Property’s quota. Quota is returned in [PropertyQuota](#PropertyQuota).
    #[serde(rename="returnPropertyQuota")]
    
    pub return_property_quota: Option<bool>,
}

impl client::RequestValue for RunPivotReportRequest {}


/// The response pivot report table corresponding to a pivot request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [run pivot report properties](PropertyRunPivotReportCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RunPivotReportResponse {
    /// Aggregation of metric values. Can be totals, minimums, or maximums. The returned aggregations are controlled by the metric_aggregations in the pivot. The type of aggregation returned in each row is shown by the dimension_values which are set to "RESERVED_".
    
    pub aggregates: Option<Vec<Row>>,
    /// Describes dimension columns. The number of DimensionHeaders and ordering of DimensionHeaders matches the dimensions present in rows.
    #[serde(rename="dimensionHeaders")]
    
    pub dimension_headers: Option<Vec<DimensionHeader>>,
    /// Identifies what kind of resource this message is. This `kind` is always the fixed string "analyticsData#runPivotReport". Useful to distinguish between response types in JSON.
    
    pub kind: Option<String>,
    /// Metadata for the report.
    
    pub metadata: Option<ResponseMetaData>,
    /// Describes metric columns. The number of MetricHeaders and ordering of MetricHeaders matches the metrics present in rows.
    #[serde(rename="metricHeaders")]
    
    pub metric_headers: Option<Vec<MetricHeader>>,
    /// Summarizes the columns and rows created by a pivot. Each pivot in the request produces one header in the response. If we have a request like this: "pivots": [{ "fieldNames": ["country", "city"] }, { "fieldNames": "eventName" }] We will have the following `pivotHeaders` in the response: "pivotHeaders" : [{ "dimensionHeaders": [{ "dimensionValues": [ { "value": "United Kingdom" }, { "value": "London" } ] }, { "dimensionValues": [ { "value": "Japan" }, { "value": "Osaka" } ] }] }, { "dimensionHeaders": [{ "dimensionValues": [{ "value": "session_start" }] }, { "dimensionValues": [{ "value": "scroll" }] }] }]
    #[serde(rename="pivotHeaders")]
    
    pub pivot_headers: Option<Vec<PivotHeader>>,
    /// This Analytics Property's quota state including this request.
    #[serde(rename="propertyQuota")]
    
    pub property_quota: Option<PropertyQuota>,
    /// Rows of dimension value combinations and metric values in the report.
    
    pub rows: Option<Vec<Row>>,
}

impl client::ResponseResult for RunPivotReportResponse {}


/// The request to generate a realtime report.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [run realtime report properties](PropertyRunRealtimeReportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RunRealtimeReportRequest {
    /// The filter clause of dimensions. Metrics cannot be used in this filter.
    #[serde(rename="dimensionFilter")]
    
    pub dimension_filter: Option<FilterExpression>,
    /// The dimensions requested and displayed.
    
    pub dimensions: Option<Vec<Dimension>>,
    /// The number of rows to return. If unspecified, 10,000 rows are returned. The API returns a maximum of 100,000 rows per request, no matter how many you ask for. `limit` must be positive. The API can also return fewer rows than the requested `limit`, if there aren't as many dimension values as the `limit`. For instance, there are fewer than 300 possible values for the dimension `country`, so when reporting on only `country`, you can't get more than 300 rows, even if you set `limit` to a higher value.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub limit: Option<i64>,
    /// Aggregation of metrics. Aggregated metric values will be shown in rows where the dimension_values are set to "RESERVED_(MetricAggregation)".
    #[serde(rename="metricAggregations")]
    
    pub metric_aggregations: Option<Vec<RunRealtimeReportRequestMetricAggregationsEnum>>,
    /// The filter clause of metrics. Applied at post aggregation phase, similar to SQL having-clause. Dimensions cannot be used in this filter.
    #[serde(rename="metricFilter")]
    
    pub metric_filter: Option<FilterExpression>,
    /// The metrics requested and displayed.
    
    pub metrics: Option<Vec<Metric>>,
    /// The minute ranges of event data to read. If unspecified, one minute range for the last 30 minutes will be used. If multiple minute ranges are requested, each response row will contain a zero based minute range index. If two minute ranges overlap, the event data for the overlapping minutes is included in the response rows for both minute ranges.
    #[serde(rename="minuteRanges")]
    
    pub minute_ranges: Option<Vec<MinuteRange>>,
    /// Specifies how rows are ordered in the response.
    #[serde(rename="orderBys")]
    
    pub order_bys: Option<Vec<OrderBy>>,
    /// Toggles whether to return the current state of this Analytics Property’s Realtime quota. Quota is returned in [PropertyQuota](#PropertyQuota).
    #[serde(rename="returnPropertyQuota")]
    
    pub return_property_quota: Option<bool>,
}

impl client::RequestValue for RunRealtimeReportRequest {}


/// The response realtime report table corresponding to a request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [run realtime report properties](PropertyRunRealtimeReportCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RunRealtimeReportResponse {
    /// Describes dimension columns. The number of DimensionHeaders and ordering of DimensionHeaders matches the dimensions present in rows.
    #[serde(rename="dimensionHeaders")]
    
    pub dimension_headers: Option<Vec<DimensionHeader>>,
    /// Identifies what kind of resource this message is. This `kind` is always the fixed string "analyticsData#runRealtimeReport". Useful to distinguish between response types in JSON.
    
    pub kind: Option<String>,
    /// If requested, the maximum values of metrics.
    
    pub maximums: Option<Vec<Row>>,
    /// Describes metric columns. The number of MetricHeaders and ordering of MetricHeaders matches the metrics present in rows.
    #[serde(rename="metricHeaders")]
    
    pub metric_headers: Option<Vec<MetricHeader>>,
    /// If requested, the minimum values of metrics.
    
    pub minimums: Option<Vec<Row>>,
    /// This Analytics Property's Realtime quota state including this request.
    #[serde(rename="propertyQuota")]
    
    pub property_quota: Option<PropertyQuota>,
    /// The total number of rows in the query result. `rowCount` is independent of the number of rows returned in the response and the `limit` request parameter. For example if a query returns 175 rows and includes `limit` of 50 in the API request, the response will contain `rowCount` of 175 but only 50 rows.
    #[serde(rename="rowCount")]
    
    pub row_count: Option<i32>,
    /// Rows of dimension value combinations and metric values in the report.
    
    pub rows: Option<Vec<Row>>,
    /// If requested, the totaled values of metrics.
    
    pub totals: Option<Vec<Row>>,
}

impl client::ResponseResult for RunRealtimeReportResponse {}


/// The request to generate a report.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [run report properties](PropertyRunReportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RunReportRequest {
    /// Cohort group associated with this request. If there is a cohort group in the request the 'cohort' dimension must be present.
    #[serde(rename="cohortSpec")]
    
    pub cohort_spec: Option<CohortSpec>,
    /// A currency code in ISO4217 format, such as "AED", "USD", "JPY". If the field is empty, the report uses the property's default currency.
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// Date ranges of data to read. If multiple date ranges are requested, each response row will contain a zero based date range index. If two date ranges overlap, the event data for the overlapping days is included in the response rows for both date ranges. In a cohort request, this `dateRanges` must be unspecified.
    #[serde(rename="dateRanges")]
    
    pub date_ranges: Option<Vec<DateRange>>,
    /// Dimension filters allow you to ask for only specific dimension values in the report. To learn more, see [Fundamentals of Dimension Filters](https://developers.google.com/analytics/devguides/reporting/data/v1/basics#dimension_filters) for examples. Metrics cannot be used in this filter.
    #[serde(rename="dimensionFilter")]
    
    pub dimension_filter: Option<FilterExpression>,
    /// The dimensions requested and displayed.
    
    pub dimensions: Option<Vec<Dimension>>,
    /// If false or unspecified, each row with all metrics equal to 0 will not be returned. If true, these rows will be returned if they are not separately removed by a filter.
    #[serde(rename="keepEmptyRows")]
    
    pub keep_empty_rows: Option<bool>,
    /// The number of rows to return. If unspecified, 10,000 rows are returned. The API returns a maximum of 100,000 rows per request, no matter how many you ask for. `limit` must be positive. The API can also return fewer rows than the requested `limit`, if there aren't as many dimension values as the `limit`. For instance, there are fewer than 300 possible values for the dimension `country`, so when reporting on only `country`, you can't get more than 300 rows, even if you set `limit` to a higher value. To learn more about this pagination parameter, see [Pagination](https://developers.google.com/analytics/devguides/reporting/data/v1/basics#pagination).
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub limit: Option<i64>,
    /// Aggregation of metrics. Aggregated metric values will be shown in rows where the dimension_values are set to "RESERVED_(MetricAggregation)".
    #[serde(rename="metricAggregations")]
    
    pub metric_aggregations: Option<Vec<RunReportRequestMetricAggregationsEnum>>,
    /// The filter clause of metrics. Applied after aggregating the report's rows, similar to SQL having-clause. Dimensions cannot be used in this filter.
    #[serde(rename="metricFilter")]
    
    pub metric_filter: Option<FilterExpression>,
    /// The metrics requested and displayed.
    
    pub metrics: Option<Vec<Metric>>,
    /// The row count of the start row. The first row is counted as row 0. When paging, the first request does not specify offset; or equivalently, sets offset to 0; the first request returns the first `limit` of rows. The second request sets offset to the `limit` of the first request; the second request returns the second `limit` of rows. To learn more about this pagination parameter, see [Pagination](https://developers.google.com/analytics/devguides/reporting/data/v1/basics#pagination).
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub offset: Option<i64>,
    /// Specifies how rows are ordered in the response.
    #[serde(rename="orderBys")]
    
    pub order_bys: Option<Vec<OrderBy>>,
    /// A Google Analytics GA4 property identifier whose events are tracked. Specified in the URL path and not the body. To learn more, see [where to find your Property ID](https://developers.google.com/analytics/devguides/reporting/data/v1/property-id). Within a batch request, this property should either be unspecified or consistent with the batch-level property. Example: properties/1234
    
    pub property: Option<String>,
    /// Toggles whether to return the current state of this Analytics Property’s quota. Quota is returned in [PropertyQuota](#PropertyQuota).
    #[serde(rename="returnPropertyQuota")]
    
    pub return_property_quota: Option<bool>,
}

impl client::RequestValue for RunReportRequest {}


/// The response report table corresponding to a request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [run report properties](PropertyRunReportCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RunReportResponse {
    /// Describes dimension columns. The number of DimensionHeaders and ordering of DimensionHeaders matches the dimensions present in rows.
    #[serde(rename="dimensionHeaders")]
    
    pub dimension_headers: Option<Vec<DimensionHeader>>,
    /// Identifies what kind of resource this message is. This `kind` is always the fixed string "analyticsData#runReport". Useful to distinguish between response types in JSON.
    
    pub kind: Option<String>,
    /// If requested, the maximum values of metrics.
    
    pub maximums: Option<Vec<Row>>,
    /// Metadata for the report.
    
    pub metadata: Option<ResponseMetaData>,
    /// Describes metric columns. The number of MetricHeaders and ordering of MetricHeaders matches the metrics present in rows.
    #[serde(rename="metricHeaders")]
    
    pub metric_headers: Option<Vec<MetricHeader>>,
    /// If requested, the minimum values of metrics.
    
    pub minimums: Option<Vec<Row>>,
    /// This Analytics Property's quota state including this request.
    #[serde(rename="propertyQuota")]
    
    pub property_quota: Option<PropertyQuota>,
    /// The total number of rows in the query result. `rowCount` is independent of the number of rows returned in the response, the `limit` request parameter, and the `offset` request parameter. For example if a query returns 175 rows and includes `limit` of 50 in the API request, the response will contain `rowCount` of 175 but only 50 rows. To learn more about this pagination parameter, see [Pagination](https://developers.google.com/analytics/devguides/reporting/data/v1/basics#pagination).
    #[serde(rename="rowCount")]
    
    pub row_count: Option<i32>,
    /// Rows of dimension value combinations and metric values in the report.
    
    pub rows: Option<Vec<Row>>,
    /// If requested, the totaled values of metrics.
    
    pub totals: Option<Vec<Row>>,
}

impl client::ResponseResult for RunReportResponse {}


/// The schema restrictions actively enforced in creating this report. To learn more, see [Access and data-restriction management](https://support.google.com/analytics/answer/10851388).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SchemaRestrictionResponse {
    /// All restrictions actively enforced in creating the report. For example, `purchaseRevenue` always has the restriction type `REVENUE_DATA`. However, this active response restriction is only populated if the user's custom role disallows access to `REVENUE_DATA`.
    #[serde(rename="activeMetricRestrictions")]
    
    pub active_metric_restrictions: Option<Vec<ActiveMetricRestriction>>,
}

impl client::Part for SchemaRestrictionResponse {}


/// The filter for string
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StringFilter {
    /// If true, the string value is case sensitive.
    #[serde(rename="caseSensitive")]
    
    pub case_sensitive: Option<bool>,
    /// The match type for this filter.
    #[serde(rename="matchType")]
    
    pub match_type: Option<StringFilterMatchTypeEnum>,
    /// The string value used for the matching.
    
    pub value: Option<String>,
}

impl client::Part for StringFilter {}


