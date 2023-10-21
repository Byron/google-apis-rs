use super::*;
/// Representation of an account.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [adclients adunits create accounts](AccountAdclientAdunitCreateCall) (none)
/// * [adclients adunits get accounts](AccountAdclientAdunitGetCall) (none)
/// * [adclients adunits get adcode accounts](AccountAdclientAdunitGetAdcodeCall) (none)
/// * [adclients adunits list accounts](AccountAdclientAdunitListCall) (none)
/// * [adclients adunits list linked custom channels accounts](AccountAdclientAdunitListLinkedCustomChannelCall) (none)
/// * [adclients adunits patch accounts](AccountAdclientAdunitPatchCall) (none)
/// * [adclients customchannels create accounts](AccountAdclientCustomchannelCreateCall) (none)
/// * [adclients customchannels delete accounts](AccountAdclientCustomchannelDeleteCall) (none)
/// * [adclients customchannels get accounts](AccountAdclientCustomchannelGetCall) (none)
/// * [adclients customchannels list accounts](AccountAdclientCustomchannelListCall) (none)
/// * [adclients customchannels list linked ad units accounts](AccountAdclientCustomchannelListLinkedAdUnitCall) (none)
/// * [adclients customchannels patch accounts](AccountAdclientCustomchannelPatchCall) (none)
/// * [adclients urlchannels get accounts](AccountAdclientUrlchannelGetCall) (none)
/// * [adclients urlchannels list accounts](AccountAdclientUrlchannelListCall) (none)
/// * [adclients get accounts](AccountAdclientGetCall) (none)
/// * [adclients get adcode accounts](AccountAdclientGetAdcodeCall) (none)
/// * [adclients list accounts](AccountAdclientListCall) (none)
/// * [alerts list accounts](AccountAlertListCall) (none)
/// * [payments list accounts](AccountPaymentListCall) (none)
/// * [reports saved generate accounts](AccountReportSavedGenerateCall) (none)
/// * [reports saved generate csv accounts](AccountReportSavedGenerateCsvCall) (none)
/// * [reports saved list accounts](AccountReportSavedListCall) (none)
/// * [reports generate accounts](AccountReportGenerateCall) (none)
/// * [reports generate csv accounts](AccountReportGenerateCsvCall) (none)
/// * [reports get saved accounts](AccountReportGetSavedCall) (none)
/// * [sites get accounts](AccountSiteGetCall) (none)
/// * [sites list accounts](AccountSiteListCall) (none)
/// * [get accounts](AccountGetCall) (response)
/// * [get ad blocking recovery tag accounts](AccountGetAdBlockingRecoveryTagCall) (none)
/// * [list accounts](AccountListCall) (none)
/// * [list child accounts accounts](AccountListChildAccountCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    /// Output only. Creation time of the account.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Display name of this account.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Resource name of the account. Format: accounts/pub-[0-9]+
    
    pub name: Option<String>,
    /// Output only. Outstanding tasks that need to be completed as part of the sign-up process for a new account. e.g. "billing-profile-creation", "phone-pin-verification".
    #[serde(rename="pendingTasks")]
    
    pub pending_tasks: Option<Vec<String>>,
    /// Output only. Whether this account is premium.
    
    pub premium: Option<bool>,
    /// Output only. State of the account.
    
    pub state: Option<String>,
    /// The account time zone, as used by reporting. For more information, see [changing the time zone of your reports](https://support.google.com/adsense/answer/9830725).
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<TimeZone>,
}

impl client::Resource for Account {}
impl client::ResponseResult for Account {}


/// Representation of an ad blocking recovery tag. See https://support.google.com/adsense/answer/11575177.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get ad blocking recovery tag accounts](AccountGetAdBlockingRecoveryTagCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdBlockingRecoveryTag {
    /// Error protection code that can be used in conjunction with the tag. It'll display a message to users if an [ad blocking extension blocks their access to your site](https://support.google.com/adsense/answer/11575480).
    #[serde(rename="errorProtectionCode")]
    
    pub error_protection_code: Option<String>,
    /// The ad blocking recovery tag. Note that the message generated by the tag can be blocked by an ad blocking extension. If this is not your desired outcome, then you'll need to use it in conjunction with the error protection code.
    
    pub tag: Option<String>,
}

impl client::ResponseResult for AdBlockingRecoveryTag {}


/// Representation of an ad client. An ad client represents a user’s subscription with a specific AdSense product.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [adclients get accounts](AccountAdclientGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdClient {
    /// Output only. Resource name of the ad client. Format: accounts/{account}/adclients/{adclient}
    
    pub name: Option<String>,
    /// Output only. Reporting product code of the ad client. For example, "AFC" for AdSense for Content. Corresponds to the `PRODUCT_CODE` dimension, and present only if the ad client supports reporting.
    #[serde(rename="productCode")]
    
    pub product_code: Option<String>,
    /// Output only. Unique ID of the ad client as used in the `AD_CLIENT_ID` reporting dimension. Present only if the ad client supports reporting.
    #[serde(rename="reportingDimensionId")]
    
    pub reporting_dimension_id: Option<String>,
    /// Output only. State of the ad client.
    
    pub state: Option<String>,
}

impl client::ResponseResult for AdClient {}


/// Representation of the AdSense code for a given ad client. For more information, see [About the AdSense code](https://support.google.com/adsense/answer/9274634).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [adclients get adcode accounts](AccountAdclientGetAdcodeCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdClientAdCode {
    /// Output only. The AdSense code snippet to add to the head of an HTML page.
    #[serde(rename="adCode")]
    
    pub ad_code: Option<String>,
    /// Output only. The AdSense code snippet to add to the body of an AMP page.
    #[serde(rename="ampBody")]
    
    pub amp_body: Option<String>,
    /// Output only. The AdSense code snippet to add to the head of an AMP page.
    #[serde(rename="ampHead")]
    
    pub amp_head: Option<String>,
}

impl client::ResponseResult for AdClientAdCode {}


/// Representation of an ad unit. An ad unit represents a saved ad unit with a specific set of ad settings that have been customized within an account.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [adclients adunits create accounts](AccountAdclientAdunitCreateCall) (request|response)
/// * [adclients adunits get accounts](AccountAdclientAdunitGetCall) (response)
/// * [adclients adunits patch accounts](AccountAdclientAdunitPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdUnit {
    /// Required. Settings specific to content ads (AFC).
    #[serde(rename="contentAdsSettings")]
    
    pub content_ads_settings: Option<ContentAdsSettings>,
    /// Required. Display name of the ad unit, as provided when the ad unit was created.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Resource name of the ad unit. Format: accounts/{account}/adclients/{adclient}/adunits/{adunit}
    
    pub name: Option<String>,
    /// Output only. Unique ID of the ad unit as used in the `AD_UNIT_ID` reporting dimension.
    #[serde(rename="reportingDimensionId")]
    
    pub reporting_dimension_id: Option<String>,
    /// State of the ad unit.
    
    pub state: Option<String>,
}

impl client::RequestValue for AdUnit {}
impl client::ResponseResult for AdUnit {}


/// Representation of the ad unit code for a given ad unit. For more information, see [About the AdSense code](https://support.google.com/adsense/answer/9274634) and [Where to place the ad code in your HTML](https://support.google.com/adsense/answer/9190028).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [adclients adunits get adcode accounts](AccountAdclientAdunitGetAdcodeCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdUnitAdCode {
    /// Output only. The code snippet to add to the body of an HTML page.
    #[serde(rename="adCode")]
    
    pub ad_code: Option<String>,
}

impl client::ResponseResult for AdUnitAdCode {}


/// Representation of an alert.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Alert {
    /// Output only. The localized alert message. This may contain HTML markup, such as phrase elements or links.
    
    pub message: Option<String>,
    /// Output only. Resource name of the alert. Format: accounts/{account}/alerts/{alert}
    
    pub name: Option<String>,
    /// Output only. Severity of this alert.
    
    pub severity: Option<String>,
    /// Output only. Type of alert. This identifies the broad type of this alert, and provides a stable machine-readable identifier that will not be translated. For example, "payment-hold".
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for Alert {}


/// Cell representation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Cell {
    /// Value in the cell. The dimension cells contain strings, and the metric cells contain numbers.
    
    pub value: Option<String>,
}

impl client::Part for Cell {}


/// Settings specific to content ads (AFC).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContentAdsSettings {
    /// Required. Size of the ad unit. e.g. "728x90", "1x3" (for responsive ad units).
    
    pub size: Option<String>,
    /// Required. Type of the ad unit.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for ContentAdsSettings {}


/// Representation of a custom channel.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [adclients customchannels create accounts](AccountAdclientCustomchannelCreateCall) (request|response)
/// * [adclients customchannels get accounts](AccountAdclientCustomchannelGetCall) (response)
/// * [adclients customchannels patch accounts](AccountAdclientCustomchannelPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomChannel {
    /// Whether the custom channel is active and collecting data. See https://support.google.com/adsense/answer/10077192.
    
    pub active: Option<bool>,
    /// Required. Display name of the custom channel.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Resource name of the custom channel. Format: accounts/{account}/adclients/{adclient}/customchannels/{customchannel}
    
    pub name: Option<String>,
    /// Output only. Unique ID of the custom channel as used in the `CUSTOM_CHANNEL_ID` reporting dimension.
    #[serde(rename="reportingDimensionId")]
    
    pub reporting_dimension_id: Option<String>,
}

impl client::RequestValue for CustomChannel {}
impl client::ResponseResult for CustomChannel {}


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
/// * [adclients customchannels delete accounts](AccountAdclientCustomchannelDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// The header information of the columns requested in the report.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Header {
    /// The [ISO-4217 currency code](https://en.wikipedia.org/wiki/ISO_4217) of this column. Only present if the header type is METRIC_CURRENCY.
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// Required. Name of the header.
    
    pub name: Option<String>,
    /// Required. Type of the header.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for Header {}


/// Message that represents an arbitrary HTTP body. It should only be used for payload formats that can’t be represented as JSON, such as raw binary or an HTML page. This message can be used both in streaming and non-streaming API methods in the request as well as the response. It can be used as a top-level request field, which is convenient if one wants to extract parameters from either the URL or HTTP template into the request fields and also want access to the raw HTTP body. Example: message GetResourceRequest { // A unique request id. string request_id = 1; // The raw HTTP body is bound to this field. google.api.HttpBody http_body = 2; } service ResourceService { rpc GetResource(GetResourceRequest) returns (google.api.HttpBody); rpc UpdateResource(google.api.HttpBody) returns (google.protobuf.Empty); } Example with streaming methods: service CaldavService { rpc GetCalendar(stream google.api.HttpBody) returns (stream google.api.HttpBody); rpc UpdateCalendar(stream google.api.HttpBody) returns (stream google.api.HttpBody); } Use of this type only changes how the request and response bodies are handled, all other features will continue to work unchanged.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reports saved generate csv accounts](AccountReportSavedGenerateCsvCall) (response)
/// * [reports generate csv accounts](AccountReportGenerateCsvCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HttpBody {
    /// The HTTP Content-Type header value specifying the content type of the body.
    #[serde(rename="contentType")]
    
    pub content_type: Option<String>,
    /// The HTTP request/response body as raw binary.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub data: Option<Vec<u8>>,
    /// Application specific response metadata. Must be set in the first response for streaming APIs.
    
    pub extensions: Option<Vec<HashMap<String, json::Value>>>,
}

impl client::ResponseResult for HttpBody {}


/// Response definition for the account list rpc.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list accounts](AccountListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAccountsResponse {
    /// The accounts returned in this list response.
    
    pub accounts: Option<Vec<Account>>,
    /// Continuation token used to page through accounts. To retrieve the next page of the results, set the next request's "page_token" value to this.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListAccountsResponse {}


/// Response definition for the ad client list rpc.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [adclients list accounts](AccountAdclientListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAdClientsResponse {
    /// The ad clients returned in this list response.
    #[serde(rename="adClients")]
    
    pub ad_clients: Option<Vec<AdClient>>,
    /// Continuation token used to page through ad clients. To retrieve the next page of the results, set the next request's "page_token" value to this.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListAdClientsResponse {}


/// Response definition for the adunit list rpc.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [adclients adunits list accounts](AccountAdclientAdunitListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAdUnitsResponse {
    /// The ad units returned in the list response.
    #[serde(rename="adUnits")]
    
    pub ad_units: Option<Vec<AdUnit>>,
    /// Continuation token used to page through ad units. To retrieve the next page of the results, set the next request's "page_token" value to this.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListAdUnitsResponse {}


/// Response definition for the alerts list rpc.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [alerts list accounts](AccountAlertListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAlertsResponse {
    /// The alerts returned in this list response.
    
    pub alerts: Option<Vec<Alert>>,
}

impl client::ResponseResult for ListAlertsResponse {}


/// Response definition for the child account list rpc.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list child accounts accounts](AccountListChildAccountCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListChildAccountsResponse {
    /// The accounts returned in this list response.
    
    pub accounts: Option<Vec<Account>>,
    /// Continuation token used to page through accounts. To retrieve the next page of the results, set the next request's "page_token" value to this.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListChildAccountsResponse {}


/// Response definition for the custom channel list rpc.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [adclients customchannels list accounts](AccountAdclientCustomchannelListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCustomChannelsResponse {
    /// The custom channels returned in this list response.
    #[serde(rename="customChannels")]
    
    pub custom_channels: Option<Vec<CustomChannel>>,
    /// Continuation token used to page through alerts. To retrieve the next page of the results, set the next request's "page_token" value to this.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListCustomChannelsResponse {}


/// Response definition for the ad units linked to a custom channel list rpc.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [adclients customchannels list linked ad units accounts](AccountAdclientCustomchannelListLinkedAdUnitCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListLinkedAdUnitsResponse {
    /// The ad units returned in the list response.
    #[serde(rename="adUnits")]
    
    pub ad_units: Option<Vec<AdUnit>>,
    /// Continuation token used to page through ad units. To retrieve the next page of the results, set the next request's "page_token" value to this.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListLinkedAdUnitsResponse {}


/// Response definition for the custom channels linked to an adunit list rpc.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [adclients adunits list linked custom channels accounts](AccountAdclientAdunitListLinkedCustomChannelCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListLinkedCustomChannelsResponse {
    /// The custom channels returned in this list response.
    #[serde(rename="customChannels")]
    
    pub custom_channels: Option<Vec<CustomChannel>>,
    /// Continuation token used to page through alerts. To retrieve the next page of the results, set the next request's "page_token" value to this.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListLinkedCustomChannelsResponse {}


/// Response definition for the payments list rpc.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [payments list accounts](AccountPaymentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListPaymentsResponse {
    /// The payments returned in this list response.
    
    pub payments: Option<Vec<Payment>>,
}

impl client::ResponseResult for ListPaymentsResponse {}


/// Response definition for the saved reports list rpc.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reports saved list accounts](AccountReportSavedListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListSavedReportsResponse {
    /// Continuation token used to page through reports. To retrieve the next page of the results, set the next request's "page_token" value to this.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The reports returned in this list response.
    #[serde(rename="savedReports")]
    
    pub saved_reports: Option<Vec<SavedReport>>,
}

impl client::ResponseResult for ListSavedReportsResponse {}


/// Response definition for the sites list rpc.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sites list accounts](AccountSiteListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListSitesResponse {
    /// Continuation token used to page through sites. To retrieve the next page of the results, set the next request's "page_token" value to this.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The sites returned in this list response.
    
    pub sites: Option<Vec<Site>>,
}

impl client::ResponseResult for ListSitesResponse {}


/// Response definition for the url channels list rpc.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [adclients urlchannels list accounts](AccountAdclientUrlchannelListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListUrlChannelsResponse {
    /// Continuation token used to page through url channels. To retrieve the next page of the results, set the next request's "page_token" value to this.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The url channels returned in this list response.
    #[serde(rename="urlChannels")]
    
    pub url_channels: Option<Vec<UrlChannel>>,
}

impl client::ResponseResult for ListUrlChannelsResponse {}


/// Representation of an unpaid or paid payment. See [Payment timelines for AdSense](https://support.google.com/adsense/answer/7164703) for more information about payments and the [YouTube homepage and payments account](https://support.google.com/adsense/answer/11622510) article for information about dedicated payments accounts for YouTube.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Payment {
    /// Output only. The amount of unpaid or paid earnings, as a formatted string, including the currency. E.g. "¥1,235 JPY", "$1,234.57", "£87.65".
    
    pub amount: Option<String>,
    /// Output only. For paid earnings, the date that the payment was credited. For unpaid earnings, this field is empty. Payment dates are always returned in the billing timezone (America/Los_Angeles).
    
    pub date: Option<Date>,
    /// Output only. Resource name of the payment. Format: - accounts/{account}/payments/unpaid for unpaid (current) AdSense earnings. - accounts/{account}/payments/youtube-unpaid for unpaid (current) YouTube earnings. - accounts/{account}/payments/yyyy-MM-dd for paid AdSense earnings. - accounts/{account}/payments/youtube-yyyy-MM-dd for paid YouTube earnings.
    
    pub name: Option<String>,
}

impl client::Part for Payment {}


/// Result of a generated report.
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
pub struct ReportResult {
    /// The averages of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty.
    
    pub averages: Option<Row>,
    /// Required. End date of the range (inclusive).
    #[serde(rename="endDate")]
    
    pub end_date: Option<Date>,
    /// The header information; one for each dimension in the request, followed by one for each metric in the request.
    
    pub headers: Option<Vec<Header>>,
    /// The output rows of the report. Each row is a list of cells; one for each dimension in the request, followed by one for each metric in the request.
    
    pub rows: Option<Vec<Row>>,
    /// Required. Start date of the range (inclusive).
    #[serde(rename="startDate")]
    
    pub start_date: Option<Date>,
    /// The total number of rows matched by the report request.
    #[serde(rename="totalMatchedRows")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_matched_rows: Option<i64>,
    /// The totals of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty.
    
    pub totals: Option<Row>,
    /// Any warnings associated with generation of the report. These warnings are always returned in English.
    
    pub warnings: Option<Vec<String>>,
}

impl client::ResponseResult for ReportResult {}


/// Row representation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Row {
    /// Cells in the row.
    
    pub cells: Option<Vec<Cell>>,
}

impl client::Part for Row {}


/// Representation of a saved report.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reports get saved accounts](AccountReportGetSavedCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SavedReport {
    /// Output only. Resource name of the report. Format: accounts/{account}/reports/{report}
    
    pub name: Option<String>,
    /// Report title as specified by publisher.
    
    pub title: Option<String>,
}

impl client::ResponseResult for SavedReport {}


/// Representation of a Site.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sites get accounts](AccountSiteGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Site {
    /// Whether auto ads is turned on for the site.
    #[serde(rename="autoAdsEnabled")]
    
    pub auto_ads_enabled: Option<bool>,
    /// Domain (or subdomain) of the site, e.g. "example.com" or "www.example.com". This is used in the `OWNED_SITE_DOMAIN_NAME` reporting dimension.
    
    pub domain: Option<String>,
    /// Output only. Resource name of a site. Format: accounts/{account}/sites/{site}
    
    pub name: Option<String>,
    /// Output only. Unique ID of the site as used in the `OWNED_SITE_ID` reporting dimension.
    #[serde(rename="reportingDimensionId")]
    
    pub reporting_dimension_id: Option<String>,
    /// Output only. State of a site.
    
    pub state: Option<String>,
}

impl client::ResponseResult for Site {}


/// Represents a time zone from the [IANA Time Zone Database](https://www.iana.org/time-zones).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeZone {
    /// IANA Time Zone Database time zone, e.g. "America/New_York".
    
    pub id: Option<String>,
    /// Optional. IANA Time Zone Database version number, e.g. "2019a".
    
    pub version: Option<String>,
}

impl client::Part for TimeZone {}


/// Representation of a URL channel. URL channels allow you to track the performance of particular pages in your site; see [URL channels](https://support.google.com/adsense/answer/2923836) for more information.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [adclients urlchannels get accounts](AccountAdclientUrlchannelGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UrlChannel {
    /// Output only. Resource name of the URL channel. Format: accounts/{account}/adclients/{adclient}/urlchannels/{urlchannel}
    
    pub name: Option<String>,
    /// Output only. Unique ID of the custom channel as used in the `URL_CHANNEL_ID` reporting dimension.
    #[serde(rename="reportingDimensionId")]
    
    pub reporting_dimension_id: Option<String>,
    /// URI pattern of the channel. Does not include "http://" or "https://". Example: www.example.com/home
    #[serde(rename="uriPattern")]
    
    pub uri_pattern: Option<String>,
}

impl client::ResponseResult for UrlChannel {}


