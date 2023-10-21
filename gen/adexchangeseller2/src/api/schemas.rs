use super::*;
/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [adclients list accounts](AccountAdclientListCall) (none)
/// * [alerts list accounts](AccountAlertListCall) (none)
/// * [customchannels get accounts](AccountCustomchannelGetCall) (none)
/// * [customchannels list accounts](AccountCustomchannelListCall) (none)
/// * [metadata dimensions list accounts](AccountMetadataDimensionListCall) (none)
/// * [metadata metrics list accounts](AccountMetadataMetricListCall) (none)
/// * [preferreddeals get accounts](AccountPreferreddealGetCall) (none)
/// * [preferreddeals list accounts](AccountPreferreddealListCall) (none)
/// * [reports saved generate accounts](AccountReportSavedGenerateCall) (none)
/// * [reports saved list accounts](AccountReportSavedListCall) (none)
/// * [reports generate accounts](AccountReportGenerateCall) (none)
/// * [urlchannels list accounts](AccountUrlchannelListCall) (none)
/// * [get accounts](AccountGetCall) (response)
/// * [list accounts](AccountListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    /// Unique identifier of this account.
    
    pub id: Option<String>,
    /// Kind of resource this is, in this case adexchangeseller#account.
    
    pub kind: Option<String>,
    /// Name of this account.
    
    pub name: Option<String>,
}

impl client::Resource for Account {}
impl client::ResponseResult for Account {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list accounts](AccountListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Accounts {
    /// ETag of this response for caching purposes.
    
    pub etag: Option<String>,
    /// The accounts returned in this list response.
    
    pub items: Option<Vec<Account>>,
    /// Kind of list this is, in this case adexchangeseller#accounts.
    
    pub kind: Option<String>,
    /// Continuation token used to page through accounts. To retrieve the next page of results, set the next request's "pageToken" value to this.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for Accounts {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdClient {
    /// Whether this ad client is opted in to ARC.
    #[serde(rename="arcOptIn")]
    
    pub arc_opt_in: Option<bool>,
    /// Unique identifier of this ad client.
    
    pub id: Option<String>,
    /// Kind of resource this is, in this case adexchangeseller#adClient.
    
    pub kind: Option<String>,
    /// This ad client's product code, which corresponds to the PRODUCT_CODE report dimension.
    #[serde(rename="productCode")]
    
    pub product_code: Option<String>,
    /// Whether this ad client supports being reported on.
    #[serde(rename="supportsReporting")]
    
    pub supports_reporting: Option<bool>,
}

impl client::Part for AdClient {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [adclients list accounts](AccountAdclientListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdClients {
    /// ETag of this response for caching purposes.
    
    pub etag: Option<String>,
    /// The ad clients returned in this list response.
    
    pub items: Option<Vec<AdClient>>,
    /// Kind of list this is, in this case adexchangeseller#adClients.
    
    pub kind: Option<String>,
    /// Continuation token used to page through ad clients. To retrieve the next page of results, set the next request's "pageToken" value to this.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for AdClients {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Alert {
    /// Unique identifier of this alert. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format.
    
    pub id: Option<String>,
    /// Kind of resource this is, in this case adexchangeseller#alert.
    
    pub kind: Option<String>,
    /// The localized alert message.
    
    pub message: Option<String>,
    /// Severity of this alert. Possible values: INFO, WARNING, SEVERE.
    
    pub severity: Option<String>,
    /// Type of this alert. Possible values: SELF_HOLD, MIGRATED_TO_BILLING3, ADDRESS_PIN_VERIFICATION, PHONE_PIN_VERIFICATION, CORPORATE_ENTITY, GRAYLISTED_PUBLISHER, API_HOLD.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for Alert {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [alerts list accounts](AccountAlertListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Alerts {
    /// The alerts returned in this list response.
    
    pub items: Option<Vec<Alert>>,
    /// Kind of list this is, in this case adexchangeseller#alerts.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for Alerts {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [customchannels get accounts](AccountCustomchannelGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomChannel {
    /// Code of this custom channel, not necessarily unique across ad clients.
    
    pub code: Option<String>,
    /// Unique identifier of this custom channel. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format.
    
    pub id: Option<String>,
    /// Kind of resource this is, in this case adexchangeseller#customChannel.
    
    pub kind: Option<String>,
    /// Name of this custom channel.
    
    pub name: Option<String>,
    /// The targeting information of this custom channel, if activated.
    #[serde(rename="targetingInfo")]
    
    pub targeting_info: Option<CustomChannelTargetingInfo>,
}

impl client::ResponseResult for CustomChannel {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [customchannels list accounts](AccountCustomchannelListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomChannels {
    /// ETag of this response for caching purposes.
    
    pub etag: Option<String>,
    /// The custom channels returned in this list response.
    
    pub items: Option<Vec<CustomChannel>>,
    /// Kind of list this is, in this case adexchangeseller#customChannels.
    
    pub kind: Option<String>,
    /// Continuation token used to page through custom channels. To retrieve the next page of results, set the next request's "pageToken" value to this.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for CustomChannels {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [metadata dimensions list accounts](AccountMetadataDimensionListCall) (response)
/// * [metadata metrics list accounts](AccountMetadataMetricListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    /// no description provided
    
    pub items: Option<Vec<ReportingMetadataEntry>>,
    /// Kind of list this is, in this case adexchangeseller#metadata.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for Metadata {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [preferreddeals get accounts](AccountPreferreddealGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PreferredDeal {
    /// The name of the advertiser this deal is for.
    #[serde(rename="advertiserName")]
    
    pub advertiser_name: Option<String>,
    /// The name of the buyer network this deal is for.
    #[serde(rename="buyerNetworkName")]
    
    pub buyer_network_name: Option<String>,
    /// The currency code that applies to the fixed_cpm value. If not set then assumed to be USD.
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// Time when this deal stops being active in seconds since the epoch (GMT). If not set then this deal is valid until manually disabled by the publisher.
    #[serde(rename="endTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub end_time: Option<u64>,
    /// The fixed price for this preferred deal. In cpm micros of currency according to currencyCode. If set, then this preferred deal is eligible for the fixed price tier of buying (highest priority, pay exactly the configured fixed price).
    #[serde(rename="fixedCpm")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub fixed_cpm: Option<i64>,
    /// Unique identifier of this preferred deal.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Kind of resource this is, in this case adexchangeseller#preferredDeal.
    
    pub kind: Option<String>,
    /// Time when this deal becomes active in seconds since the epoch (GMT). If not set then this deal is active immediately upon creation.
    #[serde(rename="startTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub start_time: Option<u64>,
}

impl client::ResponseResult for PreferredDeal {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [preferreddeals list accounts](AccountPreferreddealListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PreferredDeals {
    /// The preferred deals returned in this list response.
    
    pub items: Option<Vec<PreferredDeal>>,
    /// Kind of list this is, in this case adexchangeseller#preferredDeals.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for PreferredDeals {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reports saved generate accounts](AccountReportSavedGenerateCall) (response)
/// * [reports generate accounts](AccountReportGenerateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Report {
    /// The averages of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty.
    
    pub averages: Option<Vec<String>>,
    /// The header information of the columns requested in the report. This is a list of headers; one for each dimension in the request, followed by one for each metric in the request.
    
    pub headers: Option<Vec<ReportHeaders>>,
    /// Kind this is, in this case adexchangeseller#report.
    
    pub kind: Option<String>,
    /// The output rows of the report. Each row is a list of cells; one for each dimension in the request, followed by one for each metric in the request. The dimension cells contain strings, and the metric cells contain numbers.
    
    pub rows: Option<Vec<Vec<String>>>,
    /// The total number of rows matched by the report request. Fewer rows may be returned in the response due to being limited by the row count requested or the report row limit.
    #[serde(rename="totalMatchedRows")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_matched_rows: Option<i64>,
    /// The totals of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty.
    
    pub totals: Option<Vec<String>>,
    /// Any warnings associated with generation of the report.
    
    pub warnings: Option<Vec<String>>,
}

impl client::ResponseResult for Report {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportingMetadataEntry {
    /// For metrics this is a list of dimension IDs which the metric is compatible with, for dimensions it is a list of compatibility groups the dimension belongs to.
    #[serde(rename="compatibleDimensions")]
    
    pub compatible_dimensions: Option<Vec<String>>,
    /// The names of the metrics the dimension or metric this reporting metadata entry describes is compatible with.
    #[serde(rename="compatibleMetrics")]
    
    pub compatible_metrics: Option<Vec<String>>,
    /// Unique identifier of this reporting metadata entry, corresponding to the name of the appropriate dimension or metric.
    
    pub id: Option<String>,
    /// Kind of resource this is, in this case adexchangeseller#reportingMetadataEntry.
    
    pub kind: Option<String>,
    /// The names of the dimensions which the dimension or metric this reporting metadata entry describes requires to also be present in order for the report to be valid. Omitting these will not cause an error or warning, but may result in data which cannot be correctly interpreted.
    #[serde(rename="requiredDimensions")]
    
    pub required_dimensions: Option<Vec<String>>,
    /// The names of the metrics which the dimension or metric this reporting metadata entry describes requires to also be present in order for the report to be valid. Omitting these will not cause an error or warning, but may result in data which cannot be correctly interpreted.
    #[serde(rename="requiredMetrics")]
    
    pub required_metrics: Option<Vec<String>>,
    /// The codes of the projects supported by the dimension or metric this reporting metadata entry describes.
    #[serde(rename="supportedProducts")]
    
    pub supported_products: Option<Vec<String>>,
}

impl client::Part for ReportingMetadataEntry {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SavedReport {
    /// Unique identifier of this saved report.
    
    pub id: Option<String>,
    /// Kind of resource this is, in this case adexchangeseller#savedReport.
    
    pub kind: Option<String>,
    /// This saved report's name.
    
    pub name: Option<String>,
}

impl client::Part for SavedReport {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reports saved list accounts](AccountReportSavedListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SavedReports {
    /// ETag of this response for caching purposes.
    
    pub etag: Option<String>,
    /// The saved reports returned in this list response.
    
    pub items: Option<Vec<SavedReport>>,
    /// Kind of list this is, in this case adexchangeseller#savedReports.
    
    pub kind: Option<String>,
    /// Continuation token used to page through saved reports. To retrieve the next page of results, set the next request's "pageToken" value to this.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for SavedReports {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UrlChannel {
    /// Unique identifier of this URL channel. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format.
    
    pub id: Option<String>,
    /// Kind of resource this is, in this case adexchangeseller#urlChannel.
    
    pub kind: Option<String>,
    /// URL Pattern of this URL channel. Does not include "http://" or "https://". Example: www.example.com/home
    #[serde(rename="urlPattern")]
    
    pub url_pattern: Option<String>,
}

impl client::Part for UrlChannel {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [urlchannels list accounts](AccountUrlchannelListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UrlChannels {
    /// ETag of this response for caching purposes.
    
    pub etag: Option<String>,
    /// The URL channels returned in this list response.
    
    pub items: Option<Vec<UrlChannel>>,
    /// Kind of list this is, in this case adexchangeseller#urlChannels.
    
    pub kind: Option<String>,
    /// Continuation token used to page through URL channels. To retrieve the next page of results, set the next request's "pageToken" value to this.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for UrlChannels {}


/// The targeting information of this custom channel, if activated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomChannelTargetingInfo {
    /// The name used to describe this channel externally.
    #[serde(rename="adsAppearOn")]
    
    pub ads_appear_on: Option<String>,
    /// The external description of the channel.
    
    pub description: Option<String>,
    /// The locations in which ads appear. (Only valid for content and mobile content ads). Acceptable values for content ads are: TOP_LEFT, TOP_CENTER, TOP_RIGHT, MIDDLE_LEFT, MIDDLE_CENTER, MIDDLE_RIGHT, BOTTOM_LEFT, BOTTOM_CENTER, BOTTOM_RIGHT, MULTIPLE_LOCATIONS. Acceptable values for mobile content ads are: TOP, MIDDLE, BOTTOM, MULTIPLE_LOCATIONS.
    
    pub location: Option<String>,
    /// The language of the sites ads will be displayed on.
    #[serde(rename="siteLanguage")]
    
    pub site_language: Option<String>,
}

impl client::NestedType for CustomChannelTargetingInfo {}
impl client::Part for CustomChannelTargetingInfo {}


/// The header information of the columns requested in the report. This is a list of headers; one for each dimension in the request, followed by one for each metric in the request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportHeaders {
    /// The currency of this column. Only present if the header type is METRIC_CURRENCY.
    
    pub currency: Option<String>,
    /// The name of the header.
    
    pub name: Option<String>,
    /// The type of the header; one of DIMENSION, METRIC_TALLY, METRIC_RATIO, or METRIC_CURRENCY.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::NestedType for ReportHeaders {}
impl client::Part for ReportHeaders {}


