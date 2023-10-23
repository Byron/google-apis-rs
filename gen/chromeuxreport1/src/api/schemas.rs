use super::*;
/// A bin is a discrete portion of data spanning from start to end, or if no end is given, then from start to +inf. A bin's start and end values are given in the value type of the metric it represents. For example, "first contentful paint" is measured in milliseconds and exposed as ints, therefore its metric bins will use int32s for its start and end types. However, "cumulative layout shift" is measured in unitless decimals and is exposed as a decimal encoded as a string, therefore its metric bins will use strings for its value type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Bin {
    /// The proportion of users that experienced this bin's value for the given metric.
    
    pub density: Option<f64>,
    /// End is the end of the data bin. If end is not populated, then the bin has no end and is valid from start to +inf.
    
    pub end: Option<json::Value>,
    /// Start is the beginning of the data bin.
    
    pub start: Option<json::Value>,
}

impl client::Part for Bin {}


/// The collection period is a date range which includes the `first` and `last` day.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CollectionPeriod {
    /// The first day in the collection period, inclusive.
    #[serde(rename="firstDate")]
    
    pub first_date: Option<Date>,
    /// The last day in the collection period, inclusive.
    #[serde(rename="lastDate")]
    
    pub last_date: Option<Date>,
}

impl client::Part for CollectionPeriod {}


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


/// Key defines all the dimensions that identify this record as unique.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Key {
    /// The effective connection type is the general connection class that all users experienced for this record. This field uses the values ["offline", "slow-2G", "2G", "3G", "4G"] as specified in: https://wicg.github.io/netinfo/#effective-connection-types If the effective connection type is unspecified, then aggregated data over all effective connection types will be returned.
    #[serde(rename="effectiveConnectionType")]
    
    pub effective_connection_type: Option<String>,
    /// The form factor is the device class that all users used to access the site for this record. If the form factor is unspecified, then aggregated data over all form factors will be returned.
    #[serde(rename="formFactor")]
    
    pub form_factor: Option<KeyFormFactorEnum>,
    /// Origin specifies the origin that this record is for. Note: When specifying an origin, data for loads under this origin over all pages are aggregated into origin level user experience data.
    
    pub origin: Option<String>,
    /// Url specifies a specific url that this record is for. Note: When specifying a "url" only data for that specific url will be aggregated.
    
    pub url: Option<String>,
}

impl client::Part for Key {}


/// A `metric` is a set of user experience data for a single web performance metric, like "first contentful paint". It contains a summary histogram of real world Chrome usage as a series of `bins`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Metric {
    /// The histogram of user experiences for a metric. The histogram will have at least one bin and the densities of all bins will add up to ~1.
    
    pub histogram: Option<Vec<Bin>>,
    /// Commonly useful percentiles of the Metric. The value type for the percentiles will be the same as the value types given for the Histogram bins.
    
    pub percentiles: Option<Percentiles>,
}

impl client::Part for Metric {}


/// Percentiles contains synthetic values of a metric at a given statistical percentile. These are used for estimating a metric's value as experienced by a percentage of users out of the total number of users.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Percentiles {
    /// 75% of users experienced the given metric at or below this value.
    
    pub p75: Option<json::Value>,
}

impl client::Part for Percentiles {}


/// Request payload sent by a physical web client. This request includes all necessary context to load a particular user experience record.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [query record records](RecordQueryRecordCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryRequest {
    /// The effective connection type is a query dimension that specifies the effective network class that the record's data should belong to. This field uses the values ["offline", "slow-2G", "2G", "3G", "4G"] as specified in: https://wicg.github.io/netinfo/#effective-connection-types Note: If no effective connection type is specified, then a special record with aggregated data over all effective connection types will be returned.
    #[serde(rename="effectiveConnectionType")]
    
    pub effective_connection_type: Option<String>,
    /// The form factor is a query dimension that specifies the device class that the record's data should belong to. Note: If no form factor is specified, then a special record with aggregated data over all form factors will be returned.
    #[serde(rename="formFactor")]
    
    pub form_factor: Option<QueryRequestFormFactorEnum>,
    /// The metrics that should be included in the response. If none are specified then any metrics found will be returned. Allowed values: ["first_contentful_paint", "first_input_delay", "largest_contentful_paint", "cumulative_layout_shift", "experimental_time_to_first_byte", "experimental_interaction_to_next_paint"]
    
    pub metrics: Option<Vec<String>>,
    /// The url pattern "origin" refers to a url pattern that is the origin of a website. Examples: "https://example.com", "https://cloud.google.com"
    
    pub origin: Option<String>,
    /// The url pattern "url" refers to a url pattern that is any arbitrary url. Examples: "https://example.com/", "https://cloud.google.com/why-google-cloud/"
    
    pub url: Option<String>,
}

impl client::RequestValue for QueryRequest {}


/// Response payload sent back to a physical web client. This response contains the record found based on the identiers present in a `QueryRequest`. The returned response will have a record, and sometimes details on normalization actions taken on the request that were necessary to make the request successful.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [query record records](RecordQueryRecordCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryResponse {
    /// The record that was found.
    
    pub record: Option<Record>,
    /// These are details about automated normalization actions that were taken in order to make the requested `url_pattern` valid.
    #[serde(rename="urlNormalizationDetails")]
    
    pub url_normalization_details: Option<UrlNormalization>,
}

impl client::ResponseResult for QueryResponse {}


/// Record is a single Chrome UX report data record. It contains use experience statistics for a single url pattern and set of dimensions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [query record records](RecordQueryRecordCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Record {
    /// The collection period indicates when the data reflected in this record was collected.
    #[serde(rename="collectionPeriod")]
    
    pub collection_period: Option<CollectionPeriod>,
    /// Key defines all of the unique querying parameters needed to look up a user experience record.
    
    pub key: Option<Key>,
    /// Metrics is the map of user experience data available for the record defined in the key field. Metrics are keyed on the metric name. Allowed key values: ["first_contentful_paint", "first_input_delay", "largest_contentful_paint", "cumulative_layout_shift", "experimental_time_to_first_byte", "experimental_interaction_to_next_paint"]
    
    pub metrics: Option<HashMap<String, Metric>>,
}

impl client::Resource for Record {}


/// Object representing the normalization actions taken to normalize a url to achieve a higher chance of successful lookup. These are simple automated changes that are taken when looking up the provided `url_patten` would be known to fail. Complex actions like following redirects are not handled.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UrlNormalization {
    /// The URL after any normalization actions. This is a valid user experience URL that could reasonably be looked up.
    #[serde(rename="normalizedUrl")]
    
    pub normalized_url: Option<String>,
    /// The original requested URL prior to any normalization actions.
    #[serde(rename="originalUrl")]
    
    pub original_url: Option<String>,
}

impl client::Part for UrlNormalization {}


