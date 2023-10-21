use super::*;
/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [adclients get ad code accounts](AccountAdclientGetAdCodeCall) (none)
/// * [adclients list accounts](AccountAdclientListCall) (none)
/// * [adunits customchannels list accounts](AccountAdunitCustomchannelListCall) (none)
/// * [adunits get accounts](AccountAdunitGetCall) (none)
/// * [adunits get ad code accounts](AccountAdunitGetAdCodeCall) (none)
/// * [adunits list accounts](AccountAdunitListCall) (none)
/// * [alerts delete accounts](AccountAlertDeleteCall) (none)
/// * [alerts list accounts](AccountAlertListCall) (none)
/// * [customchannels adunits list accounts](AccountCustomchannelAdunitListCall) (none)
/// * [customchannels get accounts](AccountCustomchannelGetCall) (none)
/// * [customchannels list accounts](AccountCustomchannelListCall) (none)
/// * [payments list accounts](AccountPaymentListCall) (none)
/// * [reports saved generate accounts](AccountReportSavedGenerateCall) (none)
/// * [reports saved list accounts](AccountReportSavedListCall) (none)
/// * [reports generate accounts](AccountReportGenerateCall) (none)
/// * [savedadstyles get accounts](AccountSavedadstyleGetCall) (none)
/// * [savedadstyles list accounts](AccountSavedadstyleListCall) (none)
/// * [urlchannels list accounts](AccountUrlchannelListCall) (none)
/// * [get accounts](AccountGetCall) (response)
/// * [list accounts](AccountListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    /// no description provided
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub creation_time: Option<i64>,
    /// Unique identifier of this account.
    
    pub id: Option<String>,
    /// Kind of resource this is, in this case adsense#account.
    
    pub kind: Option<String>,
    /// Name of this account.
    
    pub name: Option<String>,
    /// Whether this account is premium.
    
    pub premium: Option<bool>,
    /// Sub accounts of the this account.
    #[serde(rename="subAccounts")]
    
    pub sub_accounts: Option<Vec<Account>>,
    /// AdSense timezone of this account.
    
    pub timezone: Option<String>,
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
    /// Kind of list this is, in this case adsense#accounts.
    
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
    /// Kind of resource this is, in this case adsense#adClient.
    
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
/// * [list adclients](AdclientListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdClients {
    /// ETag of this response for caching purposes.
    
    pub etag: Option<String>,
    /// The ad clients returned in this list response.
    
    pub items: Option<Vec<AdClient>>,
    /// Kind of list this is, in this case adsense#adClients.
    
    pub kind: Option<String>,
    /// Continuation token used to page through ad clients. To retrieve the next page of results, set the next request's "pageToken" value to this.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for AdClients {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [adclients get ad code accounts](AccountAdclientGetAdCodeCall) (response)
/// * [adunits get ad code accounts](AccountAdunitGetAdCodeCall) (response)
/// * [get ad code adunits](AdunitGetAdCodeCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdCode {
    /// The Auto ad code snippet. The ad code snippet.
    #[serde(rename="adCode")]
    
    pub ad_code: Option<String>,
    /// The AMP Auto ad code snippet that goes in the body of an AMP page.
    #[serde(rename="ampBody")]
    
    pub amp_body: Option<String>,
    /// The AMP Auto ad code snippet that goes in the head of an AMP page.
    #[serde(rename="ampHead")]
    
    pub amp_head: Option<String>,
    /// Kind this is, in this case adsense#adCode.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for AdCode {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdStyle {
    /// The colors which are included in the style. These are represented as six hexadecimal characters, similar to HTML color codes, but without the leading hash.
    
    pub colors: Option<AdStyleColors>,
    /// The style of the corners in the ad (deprecated: never populated, ignored).
    
    pub corners: Option<String>,
    /// The font which is included in the style.
    
    pub font: Option<AdStyleFont>,
    /// Kind this is, in this case adsense#adStyle.
    
    pub kind: Option<String>,
}

impl client::Part for AdStyle {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [adunits get accounts](AccountAdunitGetCall) (response)
/// * [get adunits](AdunitGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdUnit {
    /// Identity code of this ad unit, not necessarily unique across ad clients.
    
    pub code: Option<String>,
    /// Settings specific to content ads (AFC) and highend mobile content ads (AFMC - deprecated).
    #[serde(rename="contentAdsSettings")]
    
    pub content_ads_settings: Option<AdUnitContentAdsSettings>,
    /// Custom style information specific to this ad unit.
    #[serde(rename="customStyle")]
    
    pub custom_style: Option<AdStyle>,
    /// Settings specific to feed ads (AFF) - deprecated.
    #[serde(rename="feedAdsSettings")]
    
    pub feed_ads_settings: Option<AdUnitFeedAdsSettings>,
    /// Unique identifier of this ad unit. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format.
    
    pub id: Option<String>,
    /// Kind of resource this is, in this case adsense#adUnit.
    
    pub kind: Option<String>,
    /// Settings specific to WAP mobile content ads (AFMC) - deprecated.
    #[serde(rename="mobileContentAdsSettings")]
    
    pub mobile_content_ads_settings: Option<AdUnitMobileContentAdsSettings>,
    /// Name of this ad unit.
    
    pub name: Option<String>,
    /// ID of the saved ad style which holds this ad unit's style information.
    #[serde(rename="savedStyleId")]
    
    pub saved_style_id: Option<String>,
    /// Status of this ad unit. Possible values are:
    /// NEW: Indicates that the ad unit was created within the last seven days and does not yet have any activity associated with it.
    /// 
    /// ACTIVE: Indicates that there has been activity on this ad unit in the last seven days.
    /// 
    /// INACTIVE: Indicates that there has been no activity on this ad unit in the last seven days.
    
    pub status: Option<String>,
}

impl client::Resource for AdUnit {}
impl client::ResponseResult for AdUnit {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [adunits list accounts](AccountAdunitListCall) (response)
/// * [customchannels adunits list accounts](AccountCustomchannelAdunitListCall) (response)
/// * [list adunits](AdunitListCall) (response)
/// * [adunits list customchannels](CustomchannelAdunitListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdUnits {
    /// ETag of this response for caching purposes.
    
    pub etag: Option<String>,
    /// The ad units returned in this list response.
    
    pub items: Option<Vec<AdUnit>>,
    /// Kind of list this is, in this case adsense#adUnits.
    
    pub kind: Option<String>,
    /// Continuation token used to page through ad units. To retrieve the next page of results, set the next request's "pageToken" value to this.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for AdUnits {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reports saved generate accounts](AccountReportSavedGenerateCall) (response)
/// * [reports generate accounts](AccountReportGenerateCall) (response)
/// * [saved generate reports](ReportSavedGenerateCall) (response)
/// * [generate reports](ReportGenerateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdsenseReportsGenerateResponse {
    /// The averages of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty.
    
    pub averages: Option<Vec<String>>,
    /// The requested end date in yyyy-mm-dd format.
    #[serde(rename="endDate")]
    
    pub end_date: Option<String>,
    /// The header information of the columns requested in the report. This is a list of headers; one for each dimension in the request, followed by one for each metric in the request.
    
    pub headers: Option<Vec<AdsenseReportsGenerateResponseHeaders>>,
    /// Kind this is, in this case adsense#report.
    
    pub kind: Option<String>,
    /// The output rows of the report. Each row is a list of cells; one for each dimension in the request, followed by one for each metric in the request. The dimension cells contain strings, and the metric cells contain numbers.
    
    pub rows: Option<Vec<Vec<String>>>,
    /// The requested start date in yyyy-mm-dd format.
    #[serde(rename="startDate")]
    
    pub start_date: Option<String>,
    /// The total number of rows matched by the report request. Fewer rows may be returned in the response due to being limited by the row count requested or the report row limit.
    #[serde(rename="totalMatchedRows")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_matched_rows: Option<i64>,
    /// The totals of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty.
    
    pub totals: Option<Vec<String>>,
    /// Any warnings associated with generation of the report.
    
    pub warnings: Option<Vec<String>>,
}

impl client::ResponseResult for AdsenseReportsGenerateResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete alerts](AlertDeleteCall) (none)
/// * [list alerts](AlertListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Alert {
    /// Unique identifier of this alert. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format.
    
    pub id: Option<String>,
    /// Whether this alert can be dismissed.
    #[serde(rename="isDismissible")]
    
    pub is_dismissible: Option<bool>,
    /// Kind of resource this is, in this case adsense#alert.
    
    pub kind: Option<String>,
    /// The localized alert message.
    
    pub message: Option<String>,
    /// Severity of this alert. Possible values: INFO, WARNING, SEVERE.
    
    pub severity: Option<String>,
    /// Type of this alert. Possible values: SELF_HOLD, MIGRATED_TO_BILLING3, ADDRESS_PIN_VERIFICATION, PHONE_PIN_VERIFICATION, CORPORATE_ENTITY, GRAYLISTED_PUBLISHER, API_HOLD.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Resource for Alert {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [alerts list accounts](AccountAlertListCall) (response)
/// * [list alerts](AlertListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Alerts {
    /// The alerts returned in this list response.
    
    pub items: Option<Vec<Alert>>,
    /// Kind of list this is, in this case adsense#alerts.
    
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
/// * [get customchannels](CustomchannelGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomChannel {
    /// Code of this custom channel, not necessarily unique across ad clients.
    
    pub code: Option<String>,
    /// Unique identifier of this custom channel. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format.
    
    pub id: Option<String>,
    /// Kind of resource this is, in this case adsense#customChannel.
    
    pub kind: Option<String>,
    /// Name of this custom channel.
    
    pub name: Option<String>,
    /// The targeting information of this custom channel, if activated.
    #[serde(rename="targetingInfo")]
    
    pub targeting_info: Option<CustomChannelTargetingInfo>,
}

impl client::Resource for CustomChannel {}
impl client::ResponseResult for CustomChannel {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [adunits customchannels list accounts](AccountAdunitCustomchannelListCall) (response)
/// * [customchannels list accounts](AccountCustomchannelListCall) (response)
/// * [customchannels list adunits](AdunitCustomchannelListCall) (response)
/// * [list customchannels](CustomchannelListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomChannels {
    /// ETag of this response for caching purposes.
    
    pub etag: Option<String>,
    /// The custom channels returned in this list response.
    
    pub items: Option<Vec<CustomChannel>>,
    /// Kind of list this is, in this case adsense#customChannels.
    
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
/// * [dimensions list metadata](MetadataDimensionListCall) (response)
/// * [metrics list metadata](MetadataMetricListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    /// no description provided
    
    pub items: Option<Vec<ReportingMetadataEntry>>,
    /// Kind of list this is, in this case adsense#metadata.
    
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
/// * [list payments](PaymentListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Payment {
    /// Unique identifier of this Payment.
    
    pub id: Option<String>,
    /// Kind of resource this is, in this case adsense#payment.
    
    pub kind: Option<String>,
    /// The amount to be paid.
    #[serde(rename="paymentAmount")]
    
    pub payment_amount: Option<String>,
    /// The currency code for the amount to be paid.
    #[serde(rename="paymentAmountCurrencyCode")]
    
    pub payment_amount_currency_code: Option<String>,
    /// The date this payment was/will be credited to the user, or none if the payment threshold has not been met.
    #[serde(rename="paymentDate")]
    
    pub payment_date: Option<String>,
}

impl client::Resource for Payment {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [payments list accounts](AccountPaymentListCall) (response)
/// * [list payments](PaymentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Payments {
    /// The list of Payments for the account. One or both of a) the account's most recent payment; and b) the account's upcoming payment.
    
    pub items: Option<Vec<Payment>>,
    /// Kind of list this is, in this case adsense#payments.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for Payments {}


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
    /// Kind of resource this is, in this case adsense#reportingMetadataEntry.
    
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
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [savedadstyles get accounts](AccountSavedadstyleGetCall) (response)
/// * [get savedadstyles](SavedadstyleGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SavedAdStyle {
    /// The AdStyle itself.
    #[serde(rename="adStyle")]
    
    pub ad_style: Option<AdStyle>,
    /// Unique identifier of this saved ad style. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format.
    
    pub id: Option<String>,
    /// Kind of resource this is, in this case adsense#savedAdStyle.
    
    pub kind: Option<String>,
    /// The user selected name of this SavedAdStyle.
    
    pub name: Option<String>,
}

impl client::Resource for SavedAdStyle {}
impl client::ResponseResult for SavedAdStyle {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [savedadstyles list accounts](AccountSavedadstyleListCall) (response)
/// * [list savedadstyles](SavedadstyleListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SavedAdStyles {
    /// ETag of this response for caching purposes.
    
    pub etag: Option<String>,
    /// The saved ad styles returned in this list response.
    
    pub items: Option<Vec<SavedAdStyle>>,
    /// Kind of list this is, in this case adsense#savedAdStyles.
    
    pub kind: Option<String>,
    /// Continuation token used to page through ad units. To retrieve the next page of results, set the next request's "pageToken" value to this.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for SavedAdStyles {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SavedReport {
    /// Unique identifier of this saved report.
    
    pub id: Option<String>,
    /// Kind of resource this is, in this case adsense#savedReport.
    
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
/// * [saved list reports](ReportSavedListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SavedReports {
    /// ETag of this response for caching purposes.
    
    pub etag: Option<String>,
    /// The saved reports returned in this list response.
    
    pub items: Option<Vec<SavedReport>>,
    /// Kind of list this is, in this case adsense#savedReports.
    
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
    /// Kind of resource this is, in this case adsense#urlChannel.
    
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
/// * [list urlchannels](UrlchannelListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UrlChannels {
    /// ETag of this response for caching purposes.
    
    pub etag: Option<String>,
    /// The URL channels returned in this list response.
    
    pub items: Option<Vec<UrlChannel>>,
    /// Kind of list this is, in this case adsense#urlChannels.
    
    pub kind: Option<String>,
    /// Continuation token used to page through URL channels. To retrieve the next page of results, set the next request's "pageToken" value to this.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for UrlChannels {}


/// The colors which are included in the style. These are represented as six hexadecimal characters, similar to HTML color codes, but without the leading hash.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdStyleColors {
    /// The color of the ad background.
    
    pub background: Option<String>,
    /// The color of the ad border.
    
    pub border: Option<String>,
    /// The color of the ad text.
    
    pub text: Option<String>,
    /// The color of the ad title.
    
    pub title: Option<String>,
    /// The color of the ad url.
    
    pub url: Option<String>,
}

impl client::NestedType for AdStyleColors {}
impl client::Part for AdStyleColors {}


/// The font which is included in the style.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdStyleFont {
    /// The family of the font.
    
    pub family: Option<String>,
    /// The size of the font.
    
    pub size: Option<String>,
}

impl client::NestedType for AdStyleFont {}
impl client::Part for AdStyleFont {}


/// Settings specific to content ads (AFC) and highend mobile content ads (AFMC - deprecated).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdUnitContentAdsSettings {
    /// The backup option to be used in instances where no ad is available.
    #[serde(rename="backupOption")]
    
    pub backup_option: Option<AdUnitContentAdsSettingsBackupOption>,
    /// Size of this ad unit.
    
    pub size: Option<String>,
    /// Type of this ad unit.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::NestedType for AdUnitContentAdsSettings {}
impl client::Part for AdUnitContentAdsSettings {}


/// The backup option to be used in instances where no ad is available.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdUnitContentAdsSettingsBackupOption {
    /// Color to use when type is set to COLOR.
    
    pub color: Option<String>,
    /// Type of the backup option. Possible values are BLANK, COLOR and URL.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// URL to use when type is set to URL.
    
    pub url: Option<String>,
}

impl client::NestedType for AdUnitContentAdsSettingsBackupOption {}
impl client::Part for AdUnitContentAdsSettingsBackupOption {}


/// Settings specific to feed ads (AFF) - deprecated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdUnitFeedAdsSettings {
    /// The position of the ads relative to the feed entries.
    #[serde(rename="adPosition")]
    
    pub ad_position: Option<String>,
    /// The frequency at which ads should appear in the feed (i.e. every N entries).
    
    pub frequency: Option<i32>,
    /// The minimum length an entry should be in order to have attached ads.
    #[serde(rename="minimumWordCount")]
    
    pub minimum_word_count: Option<i32>,
    /// The type of ads which should appear.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::NestedType for AdUnitFeedAdsSettings {}
impl client::Part for AdUnitFeedAdsSettings {}


/// Settings specific to WAP mobile content ads (AFMC) - deprecated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdUnitMobileContentAdsSettings {
    /// The markup language to use for this ad unit.
    #[serde(rename="markupLanguage")]
    
    pub markup_language: Option<String>,
    /// The scripting language to use for this ad unit.
    #[serde(rename="scriptingLanguage")]
    
    pub scripting_language: Option<String>,
    /// Size of this ad unit.
    
    pub size: Option<String>,
    /// Type of this ad unit.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::NestedType for AdUnitMobileContentAdsSettings {}
impl client::Part for AdUnitMobileContentAdsSettings {}


/// The header information of the columns requested in the report. This is a list of headers; one for each dimension in the request, followed by one for each metric in the request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdsenseReportsGenerateResponseHeaders {
    /// The currency of this column. Only present if the header type is METRIC_CURRENCY.
    
    pub currency: Option<String>,
    /// The name of the header.
    
    pub name: Option<String>,
    /// The type of the header; one of DIMENSION, METRIC_TALLY, METRIC_RATIO, or METRIC_CURRENCY.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::NestedType for AdsenseReportsGenerateResponseHeaders {}
impl client::Part for AdsenseReportsGenerateResponseHeaders {}


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
    /// The locations in which ads appear. (Only valid for content and mobile content ads (deprecated)). Acceptable values for content ads are: TOP_LEFT, TOP_CENTER, TOP_RIGHT, MIDDLE_LEFT, MIDDLE_CENTER, MIDDLE_RIGHT, BOTTOM_LEFT, BOTTOM_CENTER, BOTTOM_RIGHT, MULTIPLE_LOCATIONS. Acceptable values for mobile content ads (deprecated) are: TOP, MIDDLE, BOTTOM, MULTIPLE_LOCATIONS.
    
    pub location: Option<String>,
    /// The language of the sites ads will be displayed on.
    #[serde(rename="siteLanguage")]
    
    pub site_language: Option<String>,
}

impl client::NestedType for CustomChannelTargetingInfo {}
impl client::Part for CustomChannelTargetingInfo {}


