use super::*;
/// Metrics aggregated over the input time range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AggregateMetrics {
    /// Total count of answered calls.
    #[serde(rename="answeredCallsCount")]
    
    pub answered_calls_count: Option<i32>,
    /// End date for this metric.
    #[serde(rename="endDate")]
    
    pub end_date: Option<Date>,
    /// A list of metrics by hour of day.
    #[serde(rename="hourlyMetrics")]
    
    pub hourly_metrics: Option<Vec<HourlyMetrics>>,
    /// Total count of missed calls.
    #[serde(rename="missedCallsCount")]
    
    pub missed_calls_count: Option<i32>,
    /// Date for this metric. If metric is monthly, only year and month are used.
    #[serde(rename="startDate")]
    
    pub start_date: Option<Date>,
    /// A list of metrics by day of week.
    #[serde(rename="weekdayMetrics")]
    
    pub weekday_metrics: Option<Vec<WeekDayMetrics>>,
}

impl client::Part for AggregateMetrics {}


/// Insights for calls made to a location.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BusinessCallsInsights {
    /// Metric for the time range based on start_date and end_date.
    #[serde(rename="aggregateMetrics")]
    
    pub aggregate_metrics: Option<AggregateMetrics>,
    /// The metric for which the value applies.
    #[serde(rename="metricType")]
    
    pub metric_type: Option<BusinessCallsInsightMetricTypeEnum>,
    /// Required. The resource name of the calls insights. Format: locations/{location}/businesscallsinsights
    
    pub name: Option<String>,
}

impl client::Part for BusinessCallsInsights {}


/// Business calls settings for a location.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get businesscallssettings locations](LocationGetBusinesscallssettingCall) (response)
/// * [update businesscallssettings locations](LocationUpdateBusinesscallssettingCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BusinessCallsSettings {
    /// Required. The state of this location's enrollment in Business calls.
    #[serde(rename="callsState")]
    
    pub calls_state: Option<BusinessCallsSettingCallsStateEnum>,
    /// Input only. Time when the end user provided consent to the API user to enable business calls.
    #[serde(rename="consentTime")]
    
    pub consent_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. The resource name of the calls settings. Format: locations/{location}/businesscallssettings
    
    pub name: Option<String>,
}

impl client::RequestValue for BusinessCallsSettings {}
impl client::ResponseResult for BusinessCallsSettings {}


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


/// Metrics for an hour.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HourlyMetrics {
    /// Hour of the day. Allowed values are 0-23.
    
    pub hour: Option<i32>,
    /// Total count of missed calls for this hour.
    #[serde(rename="missedCallsCount")]
    
    pub missed_calls_count: Option<i32>,
}

impl client::Part for HourlyMetrics {}


/// Response message for ListBusinessCallsInsights.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [businesscallsinsights list locations](LocationBusinesscallsinsightListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListBusinessCallsInsightsResponse {
    /// A collection of business calls insights for the location.
    #[serde(rename="businessCallsInsights")]
    
    pub business_calls_insights: Option<Vec<BusinessCallsInsights>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. Some of the metric_types (e.g, AGGREGATE_COUNT) returns a single page. For these metrics, the next_page_token will be empty.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListBusinessCallsInsightsResponse {}


/// Metrics for a week day.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WeekDayMetrics {
    /// Day of the week. Allowed values are Sunday - Saturday.
    
    pub day: Option<WeekDayMetricDayEnum>,
    /// Total count of missed calls for this hour.
    #[serde(rename="missedCallsCount")]
    
    pub missed_calls_count: Option<i32>,
}

impl client::Part for WeekDayMetrics {}


