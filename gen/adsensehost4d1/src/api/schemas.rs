use super::*;
/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [adclients get accounts](AccountAdclientGetCall) (none)
/// * [adclients list accounts](AccountAdclientListCall) (none)
/// * [adunits delete accounts](AccountAdunitDeleteCall) (none)
/// * [adunits get accounts](AccountAdunitGetCall) (none)
/// * [adunits get ad code accounts](AccountAdunitGetAdCodeCall) (none)
/// * [adunits insert accounts](AccountAdunitInsertCall) (none)
/// * [adunits list accounts](AccountAdunitListCall) (none)
/// * [adunits patch accounts](AccountAdunitPatchCall) (none)
/// * [adunits update accounts](AccountAdunitUpdateCall) (none)
/// * [reports generate accounts](AccountReportGenerateCall) (none)
/// * [get accounts](AccountGetCall) (response)
/// * [list accounts](AccountListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    /// Unique identifier of this account.
    
    pub id: Option<String>,
    /// Kind of resource this is, in this case adsensehost#account.
    
    pub kind: Option<String>,
    /// Name of this account.
    
    pub name: Option<String>,
    /// Approval status of this account. One of: PENDING, APPROVED, DISABLED.
    
    pub status: Option<String>,
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
    /// Kind of list this is, in this case adsensehost#accounts.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for Accounts {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [adclients get accounts](AccountAdclientGetCall) (response)
/// * [get adclients](AdclientGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdClient {
    /// Whether this ad client is opted in to ARC.
    #[serde(rename="arcOptIn")]
    
    pub arc_opt_in: Option<bool>,
    /// Unique identifier of this ad client.
    
    pub id: Option<String>,
    /// Kind of resource this is, in this case adsensehost#adClient.
    
    pub kind: Option<String>,
    /// This ad client's product code, which corresponds to the PRODUCT_CODE report dimension.
    #[serde(rename="productCode")]
    
    pub product_code: Option<String>,
    /// Whether this ad client supports being reported on.
    #[serde(rename="supportsReporting")]
    
    pub supports_reporting: Option<bool>,
}

impl client::Resource for AdClient {}
impl client::ResponseResult for AdClient {}


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
    /// Kind of list this is, in this case adsensehost#adClients.
    
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
/// * [adunits get ad code accounts](AccountAdunitGetAdCodeCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdCode {
    /// The ad code snippet.
    #[serde(rename="adCode")]
    
    pub ad_code: Option<String>,
    /// Kind this is, in this case adsensehost#adCode.
    
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
    /// The colors included in the style. These are represented as six hexadecimal characters, similar to HTML color codes, but without the leading hash.
    
    pub colors: Option<AdStyleColors>,
    /// The style of the corners in the ad (deprecated: never populated, ignored).
    
    pub corners: Option<String>,
    /// The font which is included in the style.
    
    pub font: Option<AdStyleFont>,
    /// Kind this is, in this case adsensehost#adStyle.
    
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
/// * [adunits delete accounts](AccountAdunitDeleteCall) (response)
/// * [adunits get accounts](AccountAdunitGetCall) (response)
/// * [adunits insert accounts](AccountAdunitInsertCall) (request|response)
/// * [adunits patch accounts](AccountAdunitPatchCall) (request|response)
/// * [adunits update accounts](AccountAdunitUpdateCall) (request|response)
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
    /// Unique identifier of this ad unit. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format.
    
    pub id: Option<String>,
    /// Kind of resource this is, in this case adsensehost#adUnit.
    
    pub kind: Option<String>,
    /// Settings specific to WAP mobile content ads (AFMC - deprecated).
    #[serde(rename="mobileContentAdsSettings")]
    
    pub mobile_content_ads_settings: Option<AdUnitMobileContentAdsSettings>,
    /// Name of this ad unit.
    
    pub name: Option<String>,
    /// Status of this ad unit. Possible values are:
    /// NEW: Indicates that the ad unit was created within the last seven days and does not yet have any activity associated with it.
    /// 
    /// ACTIVE: Indicates that there has been activity on this ad unit in the last seven days.
    /// 
    /// INACTIVE: Indicates that there has been no activity on this ad unit in the last seven days.
    
    pub status: Option<String>,
}

impl client::RequestValue for AdUnit {}
impl client::ResponseResult for AdUnit {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [adunits list accounts](AccountAdunitListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdUnits {
    /// ETag of this response for caching purposes.
    
    pub etag: Option<String>,
    /// The ad units returned in this list response.
    
    pub items: Option<Vec<AdUnit>>,
    /// Kind of list this is, in this case adsensehost#adUnits.
    
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
/// * [start associationsessions](AssociationsessionStartCall) (response)
/// * [verify associationsessions](AssociationsessionVerifyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AssociationSession {
    /// Hosted account id of the associated publisher after association. Present if status is ACCEPTED.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// Unique identifier of this association session.
    
    pub id: Option<String>,
    /// Kind of resource this is, in this case adsensehost#associationSession.
    
    pub kind: Option<String>,
    /// The products to associate with the user. Options: AFC, AFG, AFV, AFS (deprecated), AFMC (deprecated)
    #[serde(rename="productCodes")]
    
    pub product_codes: Option<Vec<String>>,
    /// Redirect URL of this association session. Used to redirect users into the AdSense association flow.
    #[serde(rename="redirectUrl")]
    
    pub redirect_url: Option<String>,
    /// Status of the completed association, available once the association callback token has been verified. One of ACCEPTED, REJECTED, or ERROR.
    
    pub status: Option<String>,
    /// The preferred locale of the user themselves when going through the AdSense association flow.
    #[serde(rename="userLocale")]
    
    pub user_locale: Option<String>,
    /// The locale of the user's hosted website.
    #[serde(rename="websiteLocale")]
    
    pub website_locale: Option<String>,
    /// The URL of the user's hosted website.
    #[serde(rename="websiteUrl")]
    
    pub website_url: Option<String>,
}

impl client::Resource for AssociationSession {}
impl client::ResponseResult for AssociationSession {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete customchannels](CustomchannelDeleteCall) (response)
/// * [get customchannels](CustomchannelGetCall) (response)
/// * [insert customchannels](CustomchannelInsertCall) (request|response)
/// * [patch customchannels](CustomchannelPatchCall) (request|response)
/// * [update customchannels](CustomchannelUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomChannel {
    /// Code of this custom channel, not necessarily unique across ad clients.
    
    pub code: Option<String>,
    /// Unique identifier of this custom channel. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format.
    
    pub id: Option<String>,
    /// Kind of resource this is, in this case adsensehost#customChannel.
    
    pub kind: Option<String>,
    /// Name of this custom channel.
    
    pub name: Option<String>,
}

impl client::RequestValue for CustomChannel {}
impl client::Resource for CustomChannel {}
impl client::ResponseResult for CustomChannel {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list customchannels](CustomchannelListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomChannels {
    /// ETag of this response for caching purposes.
    
    pub etag: Option<String>,
    /// The custom channels returned in this list response.
    
    pub items: Option<Vec<CustomChannel>>,
    /// Kind of list this is, in this case adsensehost#customChannels.
    
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
/// * [reports generate accounts](AccountReportGenerateCall) (response)
/// * [generate reports](ReportGenerateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Report {
    /// The averages of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty.
    
    pub averages: Option<Vec<String>>,
    /// The header information of the columns requested in the report. This is a list of headers; one for each dimension in the request, followed by one for each metric in the request.
    
    pub headers: Option<Vec<ReportHeaders>>,
    /// Kind this is, in this case adsensehost#report.
    
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

impl client::Resource for Report {}
impl client::ResponseResult for Report {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete urlchannels](UrlchannelDeleteCall) (response)
/// * [insert urlchannels](UrlchannelInsertCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UrlChannel {
    /// Unique identifier of this URL channel. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format.
    
    pub id: Option<String>,
    /// Kind of resource this is, in this case adsensehost#urlChannel.
    
    pub kind: Option<String>,
    /// URL Pattern of this URL channel. Does not include "http://" or "https://". Example: www.example.com/home
    #[serde(rename="urlPattern")]
    
    pub url_pattern: Option<String>,
}

impl client::RequestValue for UrlChannel {}
impl client::Resource for UrlChannel {}
impl client::ResponseResult for UrlChannel {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list urlchannels](UrlchannelListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UrlChannels {
    /// ETag of this response for caching purposes.
    
    pub etag: Option<String>,
    /// The URL channels returned in this list response.
    
    pub items: Option<Vec<UrlChannel>>,
    /// Kind of list this is, in this case adsensehost#urlChannels.
    
    pub kind: Option<String>,
    /// Continuation token used to page through URL channels. To retrieve the next page of results, set the next request's "pageToken" value to this.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for UrlChannels {}


/// The colors included in the style. These are represented as six hexadecimal characters, similar to HTML color codes, but without the leading hash.
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
    /// The family of the font. Possible values are: ACCOUNT_DEFAULT_FAMILY, ADSENSE_DEFAULT_FAMILY, ARIAL, TIMES and VERDANA.
    
    pub family: Option<String>,
    /// The size of the font. Possible values are: ACCOUNT_DEFAULT_SIZE, ADSENSE_DEFAULT_SIZE, SMALL, MEDIUM and LARGE.
    
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
    /// Size of this ad unit. Size values are in the form SIZE_{width}_{height}.
    
    pub size: Option<String>,
    /// Type of this ad unit. Possible values are TEXT, TEXT_IMAGE, IMAGE and LINK.
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
    /// Color to use when type is set to COLOR. These are represented as six hexadecimal characters, similar to HTML color codes, but without the leading hash.
    
    pub color: Option<String>,
    /// Type of the backup option. Possible values are BLANK, COLOR and URL.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// URL to use when type is set to URL.
    
    pub url: Option<String>,
}

impl client::NestedType for AdUnitContentAdsSettingsBackupOption {}
impl client::Part for AdUnitContentAdsSettingsBackupOption {}


/// Settings specific to WAP mobile content ads (AFMC - deprecated).
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


