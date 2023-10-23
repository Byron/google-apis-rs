use super::*;
/// Describes an AdMob ad unit.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdUnit {
    /// AdFormat of the ad unit. Possible values are as follows: "APP_OPEN" - App Open ad format. "BANNER" - Banner ad format. "BANNER_INTERSTITIAL" - Legacy format that can be used as either banner or interstitial. This format can no longer be created but can be targeted by mediation groups. "INTERSTITIAL" - A full screen ad. Supported ad types are "RICH_MEDIA" and "VIDEO". "NATIVE" - Native ad format. "REWARDED" - An ad that, once viewed, gets a callback verifying the view so that a reward can be given to the user. Supported ad types are "RICH_MEDIA" (interactive) and video where video can not be excluded. "REWARDED_INTERSTITIAL" - Rewarded Interstitial ad format. Only supports video ad type. See https://support.google.com/admob/answer/9884467.
    #[serde(rename="adFormat")]
    
    pub ad_format: Option<String>,
    /// Ad media type supported by this ad unit. Possible values as follows: "RICH_MEDIA" - Text, image, and other non-video media. "VIDEO" - Video media.
    #[serde(rename="adTypes")]
    
    pub ad_types: Option<Vec<String>>,
    /// The externally visible ID of the ad unit which can be used to integrate with the AdMob SDK. This is a read only property. Example: ca-app-pub-9876543210987654/0123456789
    #[serde(rename="adUnitId")]
    
    pub ad_unit_id: Option<String>,
    /// The externally visible ID of the app this ad unit is associated with. Example: ca-app-pub-9876543210987654~0123456789
    #[serde(rename="appId")]
    
    pub app_id: Option<String>,
    /// The display name of the ad unit as shown in the AdMob UI, which is provided by the user. The maximum length allowed is 80 characters.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Resource name for this ad unit. Format is accounts/{publisher_id}/adUnits/{ad_unit_id_fragment} Example: accounts/pub-9876543210987654/adUnits/0123456789
    
    pub name: Option<String>,
}

impl client::Part for AdUnit {}


/// Describes an AdMob app for a specific platform (For example: Android or iOS).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct App {
    /// Output only. The approval state for the app.
    #[serde(rename="appApprovalState")]
    
    pub app_approval_state: Option<AppAppApprovalStateEnum>,
    /// The externally visible ID of the app which can be used to integrate with the AdMob SDK. This is a read only property. Example: ca-app-pub-9876543210987654~0123456789
    #[serde(rename="appId")]
    
    pub app_id: Option<String>,
    /// Immutable. The information for an app that is linked to an app store. This field is present if and only if the app is linked to an app store.
    #[serde(rename="linkedAppInfo")]
    
    pub linked_app_info: Option<AppLinkedAppInfo>,
    /// The information for an app that is not linked to any app store. After an app is linked, this information is still retrivable. If no name is provided for the app upon creation, a placeholder name will be used.
    #[serde(rename="manualAppInfo")]
    
    pub manual_app_info: Option<AppManualAppInfo>,
    /// Resource name for this app. Format is accounts/{publisher_id}/apps/{app_id_fragment} Example: accounts/pub-9876543210987654/apps/0123456789
    
    pub name: Option<String>,
    /// Describes the platform of the app. Limited to "IOS" and "ANDROID".
    
    pub platform: Option<String>,
}

impl client::Part for App {}


/// Information from the app store if the app is linked to an app store.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppLinkedAppInfo {
    /// The app store ID of the app; present if and only if the app is linked to an app store. If the app is added to the Google Play store, it will be the application ID of the app. For example: "com.example.myapp". See https://developer.android.com/studio/build/application-id. If the app is added to the Apple App Store, it will be app store ID. For example "105169111". Note that setting the app store id is considered an irreversible action. Once an app is linked, it cannot be unlinked.
    #[serde(rename="appStoreId")]
    
    pub app_store_id: Option<String>,
    /// Output only. Display name of the app as it appears in the app store. This is an output-only field, and may be empty if the app cannot be found in the store.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
}

impl client::Part for AppLinkedAppInfo {}


/// Information provided for manual apps which are not linked to an application store (Example: Google Play, App Store).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppManualAppInfo {
    /// The display name of the app as shown in the AdMob UI, which is provided by the user. The maximum length allowed is 80 characters.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
}

impl client::Part for AppManualAppInfo {}


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


/// Specification of a single date range. Both dates are inclusive.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DateRange {
    /// End date of the date range, inclusive. Must be greater than or equal to the start date.
    #[serde(rename="endDate")]
    
    pub end_date: Option<Date>,
    /// Start date of the date range, inclusive. Must be less than or equal to the end date.
    #[serde(rename="startDate")]
    
    pub start_date: Option<Date>,
}

impl client::Part for DateRange {}


/// Request to generate an AdMob mediation report.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [mediation report generate accounts](AccountMediationReportGenerateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GenerateMediationReportRequest {
    /// Network report specification.
    #[serde(rename="reportSpec")]
    
    pub report_spec: Option<MediationReportSpec>,
}

impl client::RequestValue for GenerateMediationReportRequest {}


/// The streaming response for the AdMob mediation report where the first response contains the report header, then a stream of row responses, and finally a footer as the last response message. For example: \[{ “header”: { “date_range”: { “start_date”: {“year”: 2018, “month”: 9, “day”: 1}, “end_date”: {“year”: 2018, “month”: 9, “day”: 1} }, “localization_settings”: { “currency_code”: “USD”, “language_code”: “en-US” } } }, { “row”: { “dimension_values”: { “DATE”: {“value”: “20180918”}, “APP”: { “value”: “ca-app-pub-8123415297019784~1001342552”, “display_label”: “My app name!” } }, “metric_values”: { “ESTIMATED_EARNINGS”: {“decimal_value”: “1324746”} } } }, { “footer”: {“matching_row_count”: 1} }\]
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [mediation report generate accounts](AccountMediationReportGenerateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GenerateMediationReportResponse {
    /// Additional information about the generated report, such as warnings about the data.
    
    pub footer: Option<ReportFooter>,
    /// Report generation settings that describes the report contents, such as the report date range and localization settings.
    
    pub header: Option<ReportHeader>,
    /// Actual report data.
    
    pub row: Option<ReportRow>,
}

impl client::ResponseResult for GenerateMediationReportResponse {}


/// Request to generate an AdMob Network report.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [network report generate accounts](AccountNetworkReportGenerateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GenerateNetworkReportRequest {
    /// Network report specification.
    #[serde(rename="reportSpec")]
    
    pub report_spec: Option<NetworkReportSpec>,
}

impl client::RequestValue for GenerateNetworkReportRequest {}


/// The streaming response for the AdMob Network report where the first response contains the report header, then a stream of row responses, and finally a footer as the last response message. For example: \[{ “header”: { “dateRange”: { “startDate”: {“year”: 2018, “month”: 9, “day”: 1}, “endDate”: {“year”: 2018, “month”: 9, “day”: 1} }, “localizationSettings”: { “currencyCode”: “USD”, “languageCode”: “en-US” } } }, { “row”: { “dimensionValues”: { “DATE”: {“value”: “20180918”}, “APP”: { “value”: “ca-app-pub-8123415297019784~1001342552”, displayLabel: “My app name!” } }, “metricValues”: { “ESTIMATED_EARNINGS”: {“microsValue”: 6500000} } } }, { “footer”: {“matchingRowCount”: 1} }\]
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [network report generate accounts](AccountNetworkReportGenerateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GenerateNetworkReportResponse {
    /// Additional information about the generated report, such as warnings about the data.
    
    pub footer: Option<ReportFooter>,
    /// Report generation settings that describes the report contents, such as the report date range and localization settings.
    
    pub header: Option<ReportHeader>,
    /// Actual report data.
    
    pub row: Option<ReportRow>,
}

impl client::ResponseResult for GenerateNetworkReportResponse {}


/// Response for the ad units list request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [ad units list accounts](AccountAdUnitListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAdUnitsResponse {
    /// The resulting ad units for the requested account.
    #[serde(rename="adUnits")]
    
    pub ad_units: Option<Vec<AdUnit>>,
    /// If not empty, indicates that there may be more ad units for the request; this value should be passed in a new `ListAdUnitsRequest`.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListAdUnitsResponse {}


/// Response for the apps list request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps list accounts](AccountAppListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAppsResponse {
    /// The resulting apps for the requested account.
    
    pub apps: Option<Vec<App>>,
    /// If not empty, indicates that there may be more apps for the request; this value should be passed in a new `ListAppsRequest`.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListAppsResponse {}


/// Response for the publisher account list request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list accounts](AccountListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListPublisherAccountsResponse {
    /// Publisher that the client credentials can access.
    
    pub account: Option<Vec<PublisherAccount>>,
    /// If not empty, indicates that there might be more accounts for the request; you must pass this value in a new `ListPublisherAccountsRequest`.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListPublisherAccountsResponse {}


/// Localization settings for reports, such as currency and language. It affects how metrics are calculated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocalizationSettings {
    /// Currency code of the earning related metrics, which is the 3-letter code defined in ISO 4217. The daily average rate is used for the currency conversion. Defaults to the account currency code if unspecified.
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// Language used for any localized text, such as some dimension value display labels. The language tag defined in the IETF BCP47. Defaults to 'en-US' if unspecified.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
}

impl client::Part for LocalizationSettings {}


/// The specification for generating an AdMob Mediation report. For example, the specification to get observed ECPM sliced by ad source and app for the 'US' and 'CN' countries can look like the following example: { "date_range": { "start_date": {"year": 2021, "month": 9, "day": 1}, "end_date": {"year": 2021, "month": 9, "day": 30} }, "dimensions": ["AD_SOURCE", "APP", "COUNTRY"], "metrics": ["OBSERVED_ECPM"], "dimension_filters": [ { "dimension": "COUNTRY", "matches_any": {"values": [{"value": "US", "value": "CN"}]} } ], "sort_conditions": [ {"dimension":"APP", order: "ASCENDING"} ], "localization_settings": { "currency_code": "USD", "language_code": "en-US" } } For a better understanding, you can treat the preceding specification like the following pseudo SQL: SELECT AD_SOURCE, APP, COUNTRY, OBSERVED_ECPM FROM MEDIATION_REPORT WHERE DATE >= '2021-09-01' AND DATE <= '2021-09-30' AND COUNTRY IN ('US', 'CN') GROUP BY AD_SOURCE, APP, COUNTRY ORDER BY APP ASC;
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MediationReportSpec {
    /// The date range for which the report is generated.
    #[serde(rename="dateRange")]
    
    pub date_range: Option<DateRange>,
    /// Describes which report rows to match based on their dimension values.
    #[serde(rename="dimensionFilters")]
    
    pub dimension_filters: Option<Vec<MediationReportSpecDimensionFilter>>,
    /// List of dimensions of the report. The value combination of these dimensions determines the row of the report. If no dimensions are specified, the report returns a single row of requested metrics for the entire account.
    
    pub dimensions: Option<Vec<MediationReportSpecDimensionsEnum>>,
    /// Localization settings of the report.
    #[serde(rename="localizationSettings")]
    
    pub localization_settings: Option<LocalizationSettings>,
    /// Maximum number of report data rows to return. If the value is not set, the API returns as many rows as possible, up to 100000. Acceptable values are 1-100000, inclusive. Values larger than 100000 return an error.
    #[serde(rename="maxReportRows")]
    
    pub max_report_rows: Option<i32>,
    /// List of metrics of the report. A report must specify at least one metric.
    
    pub metrics: Option<Vec<MediationReportSpecMetricsEnum>>,
    /// Describes the sorting of report rows. The order of the condition in the list defines its precedence; the earlier the condition, the higher its precedence. If no sort conditions are specified, the row ordering is undefined.
    #[serde(rename="sortConditions")]
    
    pub sort_conditions: Option<Vec<MediationReportSpecSortCondition>>,
    /// A report time zone. Accepts an IANA TZ name values, such as "America/Los_Angeles." If no time zone is defined, the account default takes effect. Check default value by the get account action. **Warning:** The "America/Los_Angeles" is the only supported value at the moment.
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<String>,
}

impl client::Part for MediationReportSpec {}


/// Describes which report rows to match based on their dimension values.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MediationReportSpecDimensionFilter {
    /// Applies the filter criterion to the specified dimension.
    
    pub dimension: Option<MediationReportSpecDimensionFilterDimensionEnum>,
    /// Matches a row if its value for the specified dimension is in one of the values specified in this condition.
    #[serde(rename="matchesAny")]
    
    pub matches_any: Option<StringList>,
}

impl client::Part for MediationReportSpecDimensionFilter {}


/// Sorting direction to be applied on a dimension or a metric.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MediationReportSpecSortCondition {
    /// Sort by the specified dimension.
    
    pub dimension: Option<MediationReportSpecSortConditionDimensionEnum>,
    /// Sort by the specified metric.
    
    pub metric: Option<MediationReportSpecSortConditionMetricEnum>,
    /// Sorting order of the dimension or metric.
    
    pub order: Option<MediationReportSpecSortConditionOrderEnum>,
}

impl client::Part for MediationReportSpecSortCondition {}


/// The specification for generating an AdMob Network report. For example, the specification to get clicks and estimated earnings for only the 'US' and 'CN' countries can look like the following example: { 'date_range': { 'start_date': {'year': 2021, 'month': 9, 'day': 1}, 'end_date': {'year': 2021, 'month': 9, 'day': 30} }, 'dimensions': ['DATE', 'APP', 'COUNTRY'], 'metrics': ['CLICKS', 'ESTIMATED_EARNINGS'], 'dimension_filters': [ { 'dimension': 'COUNTRY', 'matches_any': {'values': [{'value': 'US', 'value': 'CN'}]} } ], 'sort_conditions': [ {'dimension':'APP', order: 'ASCENDING'}, {'metric':'CLICKS', order: 'DESCENDING'} ], 'localization_settings': { 'currency_code': 'USD', 'language_code': 'en-US' } } For a better understanding, you can treat the preceding specification like the following pseudo SQL: SELECT DATE, APP, COUNTRY, CLICKS, ESTIMATED_EARNINGS FROM NETWORK_REPORT WHERE DATE >= '2021-09-01' AND DATE <= '2021-09-30' AND COUNTRY IN ('US', 'CN') GROUP BY DATE, APP, COUNTRY ORDER BY APP ASC, CLICKS DESC;
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkReportSpec {
    /// The date range for which the report is generated.
    #[serde(rename="dateRange")]
    
    pub date_range: Option<DateRange>,
    /// Describes which report rows to match based on their dimension values.
    #[serde(rename="dimensionFilters")]
    
    pub dimension_filters: Option<Vec<NetworkReportSpecDimensionFilter>>,
    /// List of dimensions of the report. The value combination of these dimensions determines the row of the report. If no dimensions are specified, the report returns a single row of requested metrics for the entire account.
    
    pub dimensions: Option<Vec<NetworkReportSpecDimensionsEnum>>,
    /// Localization settings of the report.
    #[serde(rename="localizationSettings")]
    
    pub localization_settings: Option<LocalizationSettings>,
    /// Maximum number of report data rows to return. If the value is not set, the API returns as many rows as possible, up to 100000. Acceptable values are 1-100000, inclusive. Values larger than 100000 return an error.
    #[serde(rename="maxReportRows")]
    
    pub max_report_rows: Option<i32>,
    /// List of metrics of the report. A report must specify at least one metric.
    
    pub metrics: Option<Vec<NetworkReportSpecMetricsEnum>>,
    /// Describes the sorting of report rows. The order of the condition in the list defines its precedence; the earlier the condition, the higher its precedence. If no sort conditions are specified, the row ordering is undefined.
    #[serde(rename="sortConditions")]
    
    pub sort_conditions: Option<Vec<NetworkReportSpecSortCondition>>,
    /// A report time zone. Accepts an IANA TZ name values, such as "America/Los_Angeles." If no time zone is defined, the account default takes effect. Check default value by the get account action. **Warning:** The "America/Los_Angeles" is the only supported value at the moment.
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<String>,
}

impl client::Part for NetworkReportSpec {}


/// Describes which report rows to match based on their dimension values.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkReportSpecDimensionFilter {
    /// Applies the filter criterion to the specified dimension.
    
    pub dimension: Option<NetworkReportSpecDimensionFilterDimensionEnum>,
    /// Matches a row if its value for the specified dimension is in one of the values specified in this condition.
    #[serde(rename="matchesAny")]
    
    pub matches_any: Option<StringList>,
}

impl client::Part for NetworkReportSpecDimensionFilter {}


/// Sorting direction to be applied on a dimension or a metric.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkReportSpecSortCondition {
    /// Sort by the specified dimension.
    
    pub dimension: Option<NetworkReportSpecSortConditionDimensionEnum>,
    /// Sort by the specified metric.
    
    pub metric: Option<NetworkReportSpecSortConditionMetricEnum>,
    /// Sorting order of the dimension or metric.
    
    pub order: Option<NetworkReportSpecSortConditionOrderEnum>,
}

impl client::Part for NetworkReportSpecSortCondition {}


/// A publisher account contains information relevant to the use of this API, such as the time zone used for the reports.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get accounts](AccountGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PublisherAccount {
    /// Currency code of the earning-related metrics, which is the 3-letter code defined in ISO 4217. The daily average rate is used for the currency conversion.
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// Resource name of this account. Format is accounts/{publisher_id}.
    
    pub name: Option<String>,
    /// The unique ID by which this publisher account can be identified in the API requests (for example, pub-1234567890).
    #[serde(rename="publisherId")]
    
    pub publisher_id: Option<String>,
    /// The time zone that is used in reports that are generated for this account. The value is a time-zone ID as specified by the CLDR project, for example, "America/Los_Angeles".
    #[serde(rename="reportingTimeZone")]
    
    pub reporting_time_zone: Option<String>,
}

impl client::ResponseResult for PublisherAccount {}


/// Groups data available after report generation, for example, warnings and row counts. Always sent as the last message in the stream response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportFooter {
    /// Total number of rows that matched the request. Warning: This count does NOT always match the number of rows in the response. Do not make that assumption when processing the response.
    #[serde(rename="matchingRowCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub matching_row_count: Option<i64>,
    /// Warnings associated with generation of the report.
    
    pub warnings: Option<Vec<ReportWarning>>,
}

impl client::Part for ReportFooter {}


/// Groups data helps to treat the generated report. Always sent as a first message in the stream response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportHeader {
    /// The date range for which the report is generated. This is identical to the range specified in the report request.
    #[serde(rename="dateRange")]
    
    pub date_range: Option<DateRange>,
    /// Localization settings of the report. This is identical to the settings in the report request.
    #[serde(rename="localizationSettings")]
    
    pub localization_settings: Option<LocalizationSettings>,
    /// The report time zone. The value is a time-zone ID as specified by the CLDR project, for example, "America/Los_Angeles".
    #[serde(rename="reportingTimeZone")]
    
    pub reporting_time_zone: Option<String>,
}

impl client::Part for ReportHeader {}


/// A row of the returning report.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportRow {
    /// Map of dimension values in a row, with keys as enum name of the dimensions.
    #[serde(rename="dimensionValues")]
    
    pub dimension_values: Option<HashMap<String, ReportRowDimensionValue>>,
    /// Map of metric values in a row, with keys as enum name of the metrics. If a metric being requested has no value returned, the map will not include it.
    #[serde(rename="metricValues")]
    
    pub metric_values: Option<HashMap<String, ReportRowMetricValue>>,
}

impl client::Part for ReportRow {}


/// Representation of a dimension value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportRowDimensionValue {
    /// The localized string representation of the value. If unspecified, the display label should be derived from the value.
    #[serde(rename="displayLabel")]
    
    pub display_label: Option<String>,
    /// Dimension value in the format specified in the report's spec Dimension enum.
    
    pub value: Option<String>,
}

impl client::Part for ReportRowDimensionValue {}


/// Representation of a metric value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportRowMetricValue {
    /// Double precision (approximate) decimal values. Rates are from 0 to 1.
    #[serde(rename="doubleValue")]
    
    pub double_value: Option<f64>,
    /// Metric integer value.
    #[serde(rename="integerValue")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub integer_value: Option<i64>,
    /// Amount in micros. One million is equivalent to one unit. Currency value is in the unit (USD, EUR or other) specified by the request. For example, $6.50 whould be represented as 6500000 micros.
    #[serde(rename="microsValue")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub micros_value: Option<i64>,
}

impl client::Part for ReportRowMetricValue {}


/// Warnings associated with generation of the report.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportWarning {
    /// Describes the details of the warning message, in English.
    
    pub description: Option<String>,
    /// Type of the warning.
    #[serde(rename="type")]
    
    pub type_: Option<ReportWarningTypeEnum>,
}

impl client::Part for ReportWarning {}


/// List of string values.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StringList {
    /// The string values.
    
    pub values: Option<Vec<String>>,
}

impl client::Part for StringList {}


