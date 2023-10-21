use super::*;
/// Android app information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1AndroidAppInfo {
    /// Output only. Permissions requested by an Android app.
    
    pub permissions: Option<Vec<GoogleChromeManagementV1AndroidAppPermission>>,
}

impl client::Part for GoogleChromeManagementV1AndroidAppInfo {}


/// Permission requested by an Android app.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1AndroidAppPermission {
    /// Output only. The type of the permission.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleChromeManagementV1AndroidAppPermission {}


/// Resource representing app details.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps android get customers](CustomerAppAndroidGetCall) (response)
/// * [apps chrome get customers](CustomerAppChromeGetCall) (response)
/// * [apps web get customers](CustomerAppWebGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1AppDetails {
    /// Output only. Android app information.
    #[serde(rename="androidAppInfo")]
    
    pub android_app_info: Option<GoogleChromeManagementV1AndroidAppInfo>,
    /// Output only. Unique store identifier for the item. Examples: "gmbmikajjgmnabiglmofipeabaddhgne" for the Save to Google Drive Chrome extension, "com.google.android.apps.docs" for the Google Drive Android app.
    #[serde(rename="appId")]
    
    pub app_id: Option<String>,
    /// Output only. Chrome Web Store app information.
    #[serde(rename="chromeAppInfo")]
    
    pub chrome_app_info: Option<GoogleChromeManagementV1ChromeAppInfo>,
    /// Output only. App's description.
    
    pub description: Option<String>,
    /// Output only. The uri for the detail page of the item.
    #[serde(rename="detailUri")]
    
    pub detail_uri: Option<String>,
    /// Output only. App's display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. First published time.
    #[serde(rename="firstPublishTime")]
    
    pub first_publish_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Home page or Website uri.
    #[serde(rename="homepageUri")]
    
    pub homepage_uri: Option<String>,
    /// Output only. A link to an image that can be used as an icon for the product.
    #[serde(rename="iconUri")]
    
    pub icon_uri: Option<String>,
    /// Output only. Indicates if the app has to be paid for OR has paid content.
    #[serde(rename="isPaidApp")]
    
    pub is_paid_app: Option<bool>,
    /// Output only. Latest published time.
    #[serde(rename="latestPublishTime")]
    
    pub latest_publish_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Format: name=customers/{customer_id}/apps/{chrome|android|web}/{app_id}@{version}
    
    pub name: Option<String>,
    /// Output only. The URI pointing to the privacy policy of the app, if it was provided by the developer. Version-specific field that will only be set when the requested app version is found.
    #[serde(rename="privacyPolicyUri")]
    
    pub privacy_policy_uri: Option<String>,
    /// Output only. The publisher of the item.
    
    pub publisher: Option<String>,
    /// Output only. Number of reviews received. Chrome Web Store review information will always be for the latest version of an app.
    #[serde(rename="reviewNumber")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub review_number: Option<i64>,
    /// Output only. The rating of the app (on 5 stars). Chrome Web Store review information will always be for the latest version of an app.
    #[serde(rename="reviewRating")]
    
    pub review_rating: Option<f32>,
    /// Output only. App version. A new revision is committed whenever a new version of the app is published.
    #[serde(rename="revisionId")]
    
    pub revision_id: Option<String>,
    /// Output only. Information about a partial service error if applicable.
    #[serde(rename="serviceError")]
    
    pub service_error: Option<GoogleRpcStatus>,
    /// Output only. App type.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::ResponseResult for GoogleChromeManagementV1AppDetails {}


/// Status data for storage. * This field is telemetry information and this will change over time as the device is utilized. * Data for this field is controlled via policy: [ReportDeviceAudioStatus](https://chromeenterprise.google/policies/#ReportDeviceAudioStatus) * Data Collection Frequency: 10 minutes * Default Data Reporting Frequency: 3 hours - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: No * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1AudioStatusReport {
    /// Output only. Active input device's name.
    #[serde(rename="inputDevice")]
    
    pub input_device: Option<String>,
    /// Output only. Active input device's gain in [0, 100].
    #[serde(rename="inputGain")]
    
    pub input_gain: Option<i32>,
    /// Output only. Is active input device mute or not.
    #[serde(rename="inputMute")]
    
    pub input_mute: Option<bool>,
    /// Output only. Active output device's name.
    #[serde(rename="outputDevice")]
    
    pub output_device: Option<String>,
    /// Output only. Is active output device mute or not.
    #[serde(rename="outputMute")]
    
    pub output_mute: Option<bool>,
    /// Output only. Active output device's volume in [0, 100].
    #[serde(rename="outputVolume")]
    
    pub output_volume: Option<i32>,
    /// Output only. Timestamp of when the sample was collected on device.
    #[serde(rename="reportTime")]
    
    pub report_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GoogleChromeManagementV1AudioStatusReport {}


/// Information about the battery. * This field provides device information, which is static and will not change over time. * Data for this field is controlled via policy: [ReportDevicePowerStatus](https://chromeenterprise.google/policies/#ReportDevicePowerStatus) * Data Collection Frequency: Only at Upload * Default Data Reporting Frequency: 3 hours - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: No * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1BatteryInfo {
    /// Output only. Design capacity (mAmpere-hours).
    #[serde(rename="designCapacity")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub design_capacity: Option<i64>,
    /// Output only. Designed minimum output voltage (mV)
    #[serde(rename="designMinVoltage")]
    
    pub design_min_voltage: Option<i32>,
    /// Output only. The date the battery was manufactured.
    #[serde(rename="manufactureDate")]
    
    pub manufacture_date: Option<GoogleTypeDate>,
    /// Output only. Battery manufacturer.
    
    pub manufacturer: Option<String>,
    /// Output only. Battery serial number.
    #[serde(rename="serialNumber")]
    
    pub serial_number: Option<String>,
    /// Output only. Technology of the battery. Example: Li-ion
    
    pub technology: Option<String>,
}

impl client::Part for GoogleChromeManagementV1BatteryInfo {}


/// Sampling data for battery. * This field is telemetry information and this will change over time as the device is utilized. * Data for this field is controlled via policy: [ReportDevicePowerStatus](https://chromeenterprise.google/policies/#ReportDevicePowerStatus) * Data Collection Frequency: Only at Upload * Default Data Reporting Frequency: 3 hours - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: No * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1BatterySampleReport {
    /// Output only. Battery charge percentage.
    #[serde(rename="chargeRate")]
    
    pub charge_rate: Option<i32>,
    /// Output only. Battery current (mA).
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub current: Option<i64>,
    /// Output only. The battery discharge rate measured in mW. Positive if the battery is being discharged, negative if it's being charged.
    #[serde(rename="dischargeRate")]
    
    pub discharge_rate: Option<i32>,
    /// Output only. Battery remaining capacity (mAmpere-hours).
    #[serde(rename="remainingCapacity")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub remaining_capacity: Option<i64>,
    /// Output only. Timestamp of when the sample was collected on device
    #[serde(rename="reportTime")]
    
    pub report_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Battery status read from sysfs. Example: Discharging
    
    pub status: Option<String>,
    /// Output only. Temperature in Celsius degrees.
    
    pub temperature: Option<i32>,
    /// Output only. Battery voltage (millivolt).
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub voltage: Option<i64>,
}

impl client::Part for GoogleChromeManagementV1BatterySampleReport {}


/// Status data for battery. * This field is telemetry information and this will change over time as the device is utilized. * Data for this field is controlled via policy: [ReportDevicePowerStatus](https://chromeenterprise.google/policies/#ReportDevicePowerStatus) * Data Collection Frequency: Only at Upload * Default Data Reporting Frequency: 3 hours - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: No * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1BatteryStatusReport {
    /// Output only. Battery health.
    #[serde(rename="batteryHealth")]
    
    pub battery_health: Option<String>,
    /// Output only. Cycle count.
    #[serde(rename="cycleCount")]
    
    pub cycle_count: Option<i32>,
    /// Output only. Full charge capacity (mAmpere-hours).
    #[serde(rename="fullChargeCapacity")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub full_charge_capacity: Option<i64>,
    /// Output only. Timestamp of when the sample was collected on device
    #[serde(rename="reportTime")]
    
    pub report_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Sampling data for the battery sorted in a decreasing order of report_time.
    
    pub sample: Option<Vec<GoogleChromeManagementV1BatterySampleReport>>,
    /// Output only. Battery serial number.
    #[serde(rename="serialNumber")]
    
    pub serial_number: Option<String>,
}

impl client::Part for GoogleChromeManagementV1BatteryStatusReport {}


/// Boot performance report of a device. * This field is telemetry information and this will change over time as the device is utilized. * Data for this field is controlled via policy: [ReportDeviceBootMode](https://chromeenterprise.google/policies/#ReportDeviceBootMode) * Data Collection Frequency: On every boot up event * Default Data Reporting Frequency: 3 hours - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: Yes * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1BootPerformanceReport {
    /// Total time to boot up.
    #[serde(rename="bootUpDuration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub boot_up_duration: Option<client::chrono::Duration>,
    /// The timestamp when power came on.
    #[serde(rename="bootUpTime")]
    
    pub boot_up_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Timestamp when the report was collected.
    #[serde(rename="reportTime")]
    
    pub report_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Total time since shutdown start to power off.
    #[serde(rename="shutdownDuration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub shutdown_duration: Option<client::chrono::Duration>,
    /// The shutdown reason.
    #[serde(rename="shutdownReason")]
    
    pub shutdown_reason: Option<String>,
    /// The timestamp when shutdown.
    #[serde(rename="shutdownTime")]
    
    pub shutdown_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GoogleChromeManagementV1BootPerformanceReport {}


/// Describes a browser version and its install count.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1BrowserVersion {
    /// Output only. The release channel of the installed browser.
    
    pub channel: Option<String>,
    /// Output only. Count grouped by device_system and major version
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
    /// Output only. Version of the system-specified operating system.
    #[serde(rename="deviceOsVersion")]
    
    pub device_os_version: Option<String>,
    /// Output only. The device operating system.
    
    pub system: Option<String>,
    /// Output only. The full version of the installed browser.
    
    pub version: Option<String>,
}

impl client::Part for GoogleChromeManagementV1BrowserVersion {}


/// Chrome Web Store app information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1ChromeAppInfo {
    /// Output only. Whether the app or extension is built and maintained by Google. Version-specific field that will only be set when the requested app version is found.
    #[serde(rename="googleOwned")]
    
    pub google_owned: Option<bool>,
    /// Output only. Whether the app or extension is in a published state in the Chrome Web Store.
    #[serde(rename="isCwsHosted")]
    
    pub is_cws_hosted: Option<bool>,
    /// Output only. Whether an app supports policy for extensions.
    #[serde(rename="isExtensionPolicySupported")]
    
    pub is_extension_policy_supported: Option<bool>,
    /// Output only. Whether the app is only for Kiosk mode on ChromeOS devices
    #[serde(rename="isKioskOnly")]
    
    pub is_kiosk_only: Option<bool>,
    /// Output only. Whether the app or extension is a theme.
    #[serde(rename="isTheme")]
    
    pub is_theme: Option<bool>,
    /// Output only. Whether this app is enabled for Kiosk mode on ChromeOS devices
    #[serde(rename="kioskEnabled")]
    
    pub kiosk_enabled: Option<bool>,
    /// Output only. The minimum number of users using this app.
    #[serde(rename="minUserCount")]
    
    pub min_user_count: Option<i32>,
    /// Output only. Every custom permission requested by the app. Version-specific field that will only be set when the requested app version is found.
    
    pub permissions: Option<Vec<GoogleChromeManagementV1ChromeAppPermission>>,
    /// Output only. Every permission giving access to domains or broad host patterns. ( e.g. www.google.com). This includes the matches from content scripts as well as hosts in the permissions node of the manifest. Version-specific field that will only be set when the requested app version is found.
    #[serde(rename="siteAccess")]
    
    pub site_access: Option<Vec<GoogleChromeManagementV1ChromeAppSiteAccess>>,
    /// Output only. The app developer has enabled support for their app. Version-specific field that will only be set when the requested app version is found.
    #[serde(rename="supportEnabled")]
    
    pub support_enabled: Option<bool>,
    /// Output only. Types of an item in the Chrome Web Store
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleChromeManagementV1ChromeAppInfo {}


/// Permission requested by a Chrome app or extension.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1ChromeAppPermission {
    /// Output only. If available, whether this permissions grants the app/extension access to user data.
    #[serde(rename="accessUserData")]
    
    pub access_user_data: Option<bool>,
    /// Output only. If available, a URI to a page that has documentation for the current permission.
    #[serde(rename="documentationUri")]
    
    pub documentation_uri: Option<String>,
    /// Output only. The type of the permission.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleChromeManagementV1ChromeAppPermission {}


/// Details of an app installation request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1ChromeAppRequest {
    /// Output only. Format: app_details=customers/{customer_id}/apps/chrome/{app_id}
    #[serde(rename="appDetails")]
    
    pub app_details: Option<String>,
    /// Output only. Unique store identifier for the app. Example: "gmbmikajjgmnabiglmofipeabaddhgne" for the Save to Google Drive Chrome extension.
    #[serde(rename="appId")]
    
    pub app_id: Option<String>,
    /// Output only. The uri for the detail page of the item.
    #[serde(rename="detailUri")]
    
    pub detail_uri: Option<String>,
    /// Output only. App's display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. A link to an image that can be used as an icon for the product.
    #[serde(rename="iconUri")]
    
    pub icon_uri: Option<String>,
    /// Output only. The timestamp of the most recently made request for this app.
    #[serde(rename="latestRequestTime")]
    
    pub latest_request_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Total count of requests for this app.
    #[serde(rename="requestCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub request_count: Option<i64>,
}

impl client::Part for GoogleChromeManagementV1ChromeAppRequest {}


/// Represent one host permission.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1ChromeAppSiteAccess {
    /// Output only. This can contain very specific hosts, or patterns like "*.com" for instance.
    #[serde(rename="hostMatch")]
    
    pub host_match: Option<String>,
}

impl client::Part for GoogleChromeManagementV1ChromeAppSiteAccess {}


/// Response containing summary of requested app installations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps count chrome app requests customers](CustomerAppCountChromeAppRequestCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1CountChromeAppRequestsResponse {
    /// Token to specify the next page in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Count of requested apps matching request.
    #[serde(rename="requestedApps")]
    
    pub requested_apps: Option<Vec<GoogleChromeManagementV1ChromeAppRequest>>,
    /// Total number of matching app requests.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
}

impl client::ResponseResult for GoogleChromeManagementV1CountChromeAppRequestsResponse {}


/// Response containing a list of devices expiring in each month of a selected time frame. Counts are grouped by model and Auto Update Expiration date.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reports count chrome devices reaching auto expiration date customers](CustomerReportCountChromeDevicesReachingAutoExpirationDateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1CountChromeDevicesReachingAutoExpirationDateResponse {
    /// The list of reports sorted by auto update expiration date in ascending order.
    #[serde(rename="deviceAueCountReports")]
    
    pub device_aue_count_reports: Option<Vec<GoogleChromeManagementV1DeviceAueCountReport>>,
}

impl client::ResponseResult for GoogleChromeManagementV1CountChromeDevicesReachingAutoExpirationDateResponse {}


/// Response containing counts for devices that need attention.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reports count chrome devices that need attention customers](CustomerReportCountChromeDevicesThatNeedAttentionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1CountChromeDevicesThatNeedAttentionResponse {
    /// Number of ChromeOS devices have not synced policies in the past 28 days.
    #[serde(rename="noRecentPolicySyncCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub no_recent_policy_sync_count: Option<i64>,
    /// Number of ChromeOS devices that have not seen any user activity in the past 28 days.
    #[serde(rename="noRecentUserActivityCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub no_recent_user_activity_count: Option<i64>,
    /// Number of devices whose OS version is not compliant.
    #[serde(rename="osVersionNotCompliantCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub os_version_not_compliant_count: Option<i64>,
    /// Number of devices that are pending an OS update.
    #[serde(rename="pendingUpdate")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub pending_update: Option<i64>,
    /// Number of devices that are unable to apply a policy due to an OS version mismatch.
    #[serde(rename="unsupportedPolicyCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub unsupported_policy_count: Option<i64>,
}

impl client::ResponseResult for GoogleChromeManagementV1CountChromeDevicesThatNeedAttentionResponse {}


/// Response containing a list of devices with a specific type of hardware specification from the requested hardware type.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reports count chrome hardware fleet devices customers](CustomerReportCountChromeHardwareFleetDeviceCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1CountChromeHardwareFleetDevicesResponse {
    /// The DeviceHardwareCountReport for device cpu type (for example Intel(R) Core(TM) i7-10610U CPU @ 1.80GHz).
    #[serde(rename="cpuReports")]
    
    pub cpu_reports: Option<Vec<GoogleChromeManagementV1DeviceHardwareCountReport>>,
    /// The DeviceHardwareCountReport for device memory amount in gigabytes (for example 16).
    #[serde(rename="memoryReports")]
    
    pub memory_reports: Option<Vec<GoogleChromeManagementV1DeviceHardwareCountReport>>,
    /// The DeviceHardwareCountReport for device model type (for example Acer C7 Chromebook).
    #[serde(rename="modelReports")]
    
    pub model_reports: Option<Vec<GoogleChromeManagementV1DeviceHardwareCountReport>>,
    /// The DeviceHardwareCountReport for device storage amount in gigabytes (for example 128).
    #[serde(rename="storageReports")]
    
    pub storage_reports: Option<Vec<GoogleChromeManagementV1DeviceHardwareCountReport>>,
}

impl client::ResponseResult for GoogleChromeManagementV1CountChromeHardwareFleetDevicesResponse {}


/// Response containing requested browser versions details and counts.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reports count chrome versions customers](CustomerReportCountChromeVersionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1CountChromeVersionsResponse {
    /// List of all browser versions and their install counts.
    #[serde(rename="browserVersions")]
    
    pub browser_versions: Option<Vec<GoogleChromeManagementV1BrowserVersion>>,
    /// Token to specify the next page of the request.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Total number browser versions matching request.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
}

impl client::ResponseResult for GoogleChromeManagementV1CountChromeVersionsResponse {}


/// Response containing details of queried installed apps.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reports count installed apps customers](CustomerReportCountInstalledAppCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1CountInstalledAppsResponse {
    /// List of installed apps matching request.
    #[serde(rename="installedApps")]
    
    pub installed_apps: Option<Vec<GoogleChromeManagementV1InstalledApp>>,
    /// Token to specify the next page of the request.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Total number of installed apps matching request.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
}

impl client::ResponseResult for GoogleChromeManagementV1CountInstalledAppsResponse {}


/// CPU specifications for the device * This field provides device information, which is static and will not change over time. * Data for this field is controlled via policy: [ReportDeviceCpuInfo](https://chromeenterprise.google/policies/#ReportDeviceCpuInfo) * Data Collection Frequency: Only at Upload * Default Data Reporting Frequency: 3 hours - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: No * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1CpuInfo {
    /// Output only. Architecture type for the CPU. * This field provides device information, which is static and will not change over time. * Data for this field is controlled via policy: [ReportDeviceCpuInfo](https://chromeenterprise.google/policies/#ReportDeviceCpuInfo) * Data Collection Frequency: Only at Upload * Default Data Reporting Frequency: 3 hours - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: No * Reported for affiliated users only: N/A
    
    pub architecture: Option<String>,
    /// Output only. Whether keylocker is configured.`TRUE` = Enabled; `FALSE` = disabled. Only reported if keylockerSupported = `TRUE`.
    #[serde(rename="keylockerConfigured")]
    
    pub keylocker_configured: Option<bool>,
    /// Output only. Whether keylocker is supported.
    #[serde(rename="keylockerSupported")]
    
    pub keylocker_supported: Option<bool>,
    /// Output only. The max CPU clock speed in kHz.
    #[serde(rename="maxClockSpeed")]
    
    pub max_clock_speed: Option<i32>,
    /// Output only. The CPU model name. Example: Intel(R) Core(TM) i5-8250U CPU @ 1.60GHz
    
    pub model: Option<String>,
}

impl client::Part for GoogleChromeManagementV1CpuInfo {}


/// Provides information about the status of the CPU. * This field is telemetry information and this will change over time as the device is utilized. * Data for this field is controlled via policy: [ReportDeviceCpuInfo](https://chromeenterprise.google/policies/#ReportDeviceCpuInfo) * Data Collection Frequency: Every 10 minutes * Default Data Reporting Frequency: 3 hours - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: No * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1CpuStatusReport {
    /// Output only. CPU temperature sample info per CPU core in Celsius
    #[serde(rename="cpuTemperatureInfo")]
    
    pub cpu_temperature_info: Option<Vec<GoogleChromeManagementV1CpuTemperatureInfo>>,
    /// Output only. Sample of CPU utilization (0-100 percent).
    #[serde(rename="cpuUtilizationPct")]
    
    pub cpu_utilization_pct: Option<i32>,
    /// Output only. The timestamp in milliseconds representing time at which this report was sampled.
    #[serde(rename="reportTime")]
    
    pub report_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Frequency the report is sampled.
    #[serde(rename="sampleFrequency")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub sample_frequency: Option<client::chrono::Duration>,
}

impl client::Part for GoogleChromeManagementV1CpuStatusReport {}


/// CPU temperature of a device. Sampled per CPU core in Celsius. * This field is telemetry information and this will change over time as the device is utilized. * Data for this field is controlled via policy: [ReportDeviceCpuInfo](https://chromeenterprise.google/policies/#ReportDeviceCpuInfo) * Data Collection Frequency: Every 10 minutes * Default Data Reporting Frequency: 3 hours - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: No * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1CpuTemperatureInfo {
    /// Output only. CPU label. Example: Core 0
    
    pub label: Option<String>,
    /// Output only. CPU temperature in Celsius.
    #[serde(rename="temperatureCelsius")]
    
    pub temperature_celsius: Option<i32>,
}

impl client::Part for GoogleChromeManagementV1CpuTemperatureInfo {}


/// Describes a device reporting Chrome browser information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1Device {
    /// Output only. The ID of the device that reported this Chrome browser information.
    #[serde(rename="deviceId")]
    
    pub device_id: Option<String>,
    /// Output only. The name of the machine within its local network.
    
    pub machine: Option<String>,
}

impl client::Part for GoogleChromeManagementV1Device {}


/// Report for CountChromeDevicesPerAueDateResponse, contains the count of devices of a specific model and auto update expiration range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1DeviceAueCountReport {
    /// Enum value of month corresponding to the auto update expiration date in UTC time zone. If the device is already expired, this field is empty.
    #[serde(rename="aueMonth")]
    
    pub aue_month: Option<String>,
    /// Int value of year corresponding to the Auto Update Expiration date in UTC time zone. If the device is already expired, this field is empty.
    #[serde(rename="aueYear")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub aue_year: Option<i64>,
    /// Count of devices of this model.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
    /// Boolean value for whether or not the device has already expired.
    
    pub expired: Option<bool>,
    /// Public model name of the devices.
    
    pub model: Option<String>,
}

impl client::Part for GoogleChromeManagementV1DeviceAueCountReport {}


/// Report for CountChromeDevicesPerHardwareSpecResponse, contains the count of devices with a unique hardware specification.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1DeviceHardwareCountReport {
    /// Public name of the hardware specification.
    
    pub bucket: Option<String>,
    /// Count of devices with a unique hardware specification.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
}

impl client::Part for GoogleChromeManagementV1DeviceHardwareCountReport {}


/// Status of the single storage device.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1DiskInfo {
    /// Output only. Number of bytes read since last boot.
    #[serde(rename="bytesReadThisSession")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bytes_read_this_session: Option<i64>,
    /// Output only. Number of bytes written since last boot.
    #[serde(rename="bytesWrittenThisSession")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bytes_written_this_session: Option<i64>,
    /// Output only. Time spent discarding since last boot. Discarding is writing to clear blocks which are no longer in use. Supported on kernels 4.18+.
    #[serde(rename="discardTimeThisSession")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub discard_time_this_session: Option<client::chrono::Duration>,
    /// Output only. Disk health.
    
    pub health: Option<String>,
    /// Output only. Counts the time the disk and queue were busy, so unlike the fields above, parallel requests are not counted multiple times.
    #[serde(rename="ioTimeThisSession")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub io_time_this_session: Option<client::chrono::Duration>,
    /// Output only. Disk manufacturer.
    
    pub manufacturer: Option<String>,
    /// Output only. Disk model.
    
    pub model: Option<String>,
    /// Output only. Time spent reading from disk since last boot.
    #[serde(rename="readTimeThisSession")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub read_time_this_session: Option<client::chrono::Duration>,
    /// Output only. Disk serial number.
    #[serde(rename="serialNumber")]
    
    pub serial_number: Option<String>,
    /// Output only. Disk size.
    #[serde(rename="sizeBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub size_bytes: Option<i64>,
    /// Output only. Disk type: eMMC / NVMe / ATA / SCSI.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Output only. Disk volumes.
    #[serde(rename="volumeIds")]
    
    pub volume_ids: Option<Vec<String>>,
    /// Output only. Time spent writing to disk since last boot.
    #[serde(rename="writeTimeThisSession")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub write_time_this_session: Option<client::chrono::Duration>,
}

impl client::Part for GoogleChromeManagementV1DiskInfo {}


/// Information for a display.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1DisplayInfo {
    /// Output only. Represents the graphics card device id.
    #[serde(rename="deviceId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub device_id: Option<i64>,
    /// Output only. Indicates if display is internal or not.
    #[serde(rename="isInternal")]
    
    pub is_internal: Option<bool>,
    /// Output only. Refresh rate in Hz.
    #[serde(rename="refreshRate")]
    
    pub refresh_rate: Option<i32>,
    /// Output only. Resolution height in pixels.
    #[serde(rename="resolutionHeight")]
    
    pub resolution_height: Option<i32>,
    /// Output only. Resolution width in pixels.
    #[serde(rename="resolutionWidth")]
    
    pub resolution_width: Option<i32>,
}

impl client::Part for GoogleChromeManagementV1DisplayInfo {}


/// Response containing a list of devices with queried app installed.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reports find installed app devices customers](CustomerReportFindInstalledAppDeviceCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1FindInstalledAppDevicesResponse {
    /// A list of devices which have the app installed. Sorted in ascending alphabetical order on the Device.machine field.
    
    pub devices: Option<Vec<GoogleChromeManagementV1Device>>,
    /// Token to specify the next page of the request.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Total number of devices matching request.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
}

impl client::ResponseResult for GoogleChromeManagementV1FindInstalledAppDevicesResponse {}


/// Information of a graphics adapter (GPU).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1GraphicsAdapterInfo {
    /// Output only. Adapter name. Example: Mesa DRI Intel(R) UHD Graphics 620 (Kabylake GT2).
    
    pub adapter: Option<String>,
    /// Output only. Represents the graphics card device id.
    #[serde(rename="deviceId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub device_id: Option<i64>,
    /// Output only. Version of the GPU driver.
    #[serde(rename="driverVersion")]
    
    pub driver_version: Option<String>,
}

impl client::Part for GoogleChromeManagementV1GraphicsAdapterInfo {}


/// Information of the graphics subsystem. * This field provides device information, which is static and will not change over time. * Data for this field is controlled via policy: [ReportDeviceGraphicsStatus](https://chromeenterprise.google/policies/#ReportDeviceGraphicsStatus) * Data Collection Frequency: Only at Upload * Default Data Reporting Frequency: 3 hours - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: No * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1GraphicsInfo {
    /// Output only. Information about the graphics adapter (GPU).
    #[serde(rename="adapterInfo")]
    
    pub adapter_info: Option<GoogleChromeManagementV1GraphicsAdapterInfo>,
}

impl client::Part for GoogleChromeManagementV1GraphicsInfo {}


/// Information of the graphics subsystem. * This field is telemetry information and this will change over time as the device is utilized. * Data for this field is controlled via policy: [ReportDeviceGraphicsInfo](https://chromeenterprise.google/policies/#ReportDeviceGraphicsInfo) * Data Collection Frequency: 3 hours. * Default Data Reporting Frequency: 3 hours - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: No * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1GraphicsStatusReport {
    /// Output only. Information about the displays for the device.
    
    pub displays: Option<Vec<GoogleChromeManagementV1DisplayInfo>>,
    /// Output only. Time at which the graphics data was reported.
    #[serde(rename="reportTime")]
    
    pub report_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GoogleChromeManagementV1GraphicsStatusReport {}


/// Data that describes the result of the HTTPS latency diagnostics routine, with the HTTPS requests issued to Google websites.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1HttpsLatencyRoutineData {
    /// Output only. HTTPS latency if routine succeeded or failed because of HIGH_LATENCY or VERY_HIGH_LATENCY.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub latency: Option<client::chrono::Duration>,
    /// Output only. HTTPS latency routine problem if a problem occurred.
    
    pub problem: Option<String>,
}

impl client::Part for GoogleChromeManagementV1HttpsLatencyRoutineData {}


/// Describes an installed app.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1InstalledApp {
    /// Output only. Unique identifier of the app. For Chrome apps and extensions, the 32-character id (e.g. ehoadneljpdggcbbknedodolkkjodefl). For Android apps, the package name (e.g. com.evernote).
    #[serde(rename="appId")]
    
    pub app_id: Option<String>,
    /// Output only. How the app was installed.
    #[serde(rename="appInstallType")]
    
    pub app_install_type: Option<String>,
    /// Output only. Source of the installed app.
    #[serde(rename="appSource")]
    
    pub app_source: Option<String>,
    /// Output only. Type of the app.
    #[serde(rename="appType")]
    
    pub app_type: Option<String>,
    /// Output only. Count of browser devices with this app installed.
    #[serde(rename="browserDeviceCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub browser_device_count: Option<i64>,
    /// Output only. Description of the installed app.
    
    pub description: Option<String>,
    /// Output only. Whether the app is disabled.
    
    pub disabled: Option<bool>,
    /// Output only. Name of the installed app.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Homepage uri of the installed app.
    #[serde(rename="homepageUri")]
    
    pub homepage_uri: Option<String>,
    /// Output only. Count of ChromeOS users with this app installed.
    #[serde(rename="osUserCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub os_user_count: Option<i64>,
    /// Output only. Permissions of the installed app.
    
    pub permissions: Option<Vec<String>>,
}

impl client::Part for GoogleChromeManagementV1InstalledApp {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [telemetry devices list customers](CustomerTelemetryDeviceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1ListTelemetryDevicesResponse {
    /// Telemetry devices returned in the response.
    
    pub devices: Option<Vec<GoogleChromeManagementV1TelemetryDevice>>,
    /// Token to specify next page in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleChromeManagementV1ListTelemetryDevicesResponse {}


/// Response message for listing telemetry events for a customer.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [telemetry events list customers](CustomerTelemetryEventListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1ListTelemetryEventsResponse {
    /// Token to specify next page in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Telemetry events returned in the response.
    #[serde(rename="telemetryEvents")]
    
    pub telemetry_events: Option<Vec<GoogleChromeManagementV1TelemetryEvent>>,
}

impl client::ResponseResult for GoogleChromeManagementV1ListTelemetryEventsResponse {}


/// Memory information of a device. * This field has both telemetry and device information: - `totalRamBytes` - Device information - `availableRamBytes` - Telemetry information - `totalMemoryEncryption` - Device information * Data for this field is controlled via policy: [ReportDeviceMemoryInfo](https://chromeenterprise.google/policies/#ReportDeviceMemoryInfo) * Data Collection Frequency: - `totalRamBytes` - Only at upload - `availableRamBytes` - Every 10 minutes - `totalMemoryEncryption` - at device startup * Default Data Reporting Frequency: - `totalRamBytes` - 3 hours - `availableRamBytes` - 3 hours - `totalMemoryEncryption` - at device startup - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: only for `totalMemoryEncryption` * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1MemoryInfo {
    /// Output only. Amount of available RAM in bytes.
    #[serde(rename="availableRamBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub available_ram_bytes: Option<i64>,
    /// Output only. Total memory encryption info for the device.
    #[serde(rename="totalMemoryEncryption")]
    
    pub total_memory_encryption: Option<GoogleChromeManagementV1TotalMemoryEncryptionInfo>,
    /// Output only. Total RAM in bytes.
    #[serde(rename="totalRamBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_ram_bytes: Option<i64>,
}

impl client::Part for GoogleChromeManagementV1MemoryInfo {}


/// Contains samples of memory status reports. * This field is telemetry information and this will change over time as the device is utilized. * Data for this field is controlled via policy: [ReportDeviceMemoryInfo](https://chromeenterprise.google/policies/#ReportDeviceMemoryInfo) * Data Collection Frequency: Only at upload, SystemRamFreeByes is collected every 10 minutes * Default Data Reporting Frequency: Every 3 hours - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: No * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1MemoryStatusReport {
    /// Output only. Number of page faults during this collection
    #[serde(rename="pageFaults")]
    
    pub page_faults: Option<i32>,
    /// Output only. The timestamp in milliseconds representing time at which this report was sampled.
    #[serde(rename="reportTime")]
    
    pub report_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Frequency the report is sampled.
    #[serde(rename="sampleFrequency")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub sample_frequency: Option<client::chrono::Duration>,
    /// Output only. Amount of free RAM in bytes (unreliable due to Garbage Collection).
    #[serde(rename="systemRamFreeBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub system_ram_free_bytes: Option<i64>,
}

impl client::Part for GoogleChromeManagementV1MemoryStatusReport {}


/// Details about the network device. * This field provides device information, which is static and will not change over time. * Data for this field is controlled via policy: [ReportNetworkDeviceConfiguration](https://chromeenterprise.google/policies/#ReportNetworkDeviceConfiguration) * Data Collection Frequency: At device startup * Default Data Reporting Frequency: At device startup - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: Yes * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1NetworkDevice {
    /// Output only. The integrated circuit card ID associated with the device's sim card.
    
    pub iccid: Option<String>,
    /// Output only. IMEI (if applicable) of the corresponding network device.
    
    pub imei: Option<String>,
    /// Output only. MAC address (if applicable) of the corresponding network device.
    #[serde(rename="macAddress")]
    
    pub mac_address: Option<String>,
    /// Output only. The mobile directory number associated with the device's sim card.
    
    pub mdn: Option<String>,
    /// Output only. MEID (if applicable) of the corresponding network device.
    
    pub meid: Option<String>,
    /// Output only. Network device type.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleChromeManagementV1NetworkDevice {}


/// Network testing results to determine the health of the device's network connection, for example whether the HTTPS latency is high or normal.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1NetworkDiagnosticsReport {
    /// Output only. HTTPS latency test data.
    #[serde(rename="httpsLatencyData")]
    
    pub https_latency_data: Option<GoogleChromeManagementV1HttpsLatencyRoutineData>,
    /// Output only. Timestamp of when the diagnostics were collected.
    #[serde(rename="reportTime")]
    
    pub report_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GoogleChromeManagementV1NetworkDiagnosticsReport {}


/// Network device information. * This field provides device information, which is static and will not change over time. * Data for this field is controlled via policy: [ReportNetworkDeviceConfiguration](https://chromeenterprise.google/policies/#ReportNetworkDeviceConfiguration) * Data Collection Frequency: At device startup * Default Data Reporting Frequency: At device startup - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: Yes * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1NetworkInfo {
    /// Output only. List of network devices.
    #[serde(rename="networkDevices")]
    
    pub network_devices: Option<Vec<GoogleChromeManagementV1NetworkDevice>>,
}

impl client::Part for GoogleChromeManagementV1NetworkInfo {}


/// State of visible/configured networks. * This field is telemetry information and this will change over time as the device is utilized. * Data for this field is controlled via policy: [ReportNetworkStatus](https://chromeenterprise.google/policies/#ReportNetworkStatus) * Data Collection Frequency: 60 minutes * Default Data Reporting Frequency: 3 hours - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: Yes * Reported for affiliated users only: Yes
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1NetworkStatusReport {
    /// Output only. Current connection state of the network.
    #[serde(rename="connectionState")]
    
    pub connection_state: Option<String>,
    /// Output only. Network connection type.
    #[serde(rename="connectionType")]
    
    pub connection_type: Option<String>,
    /// Output only. Whether the wifi encryption key is turned off.
    #[serde(rename="encryptionOn")]
    
    pub encryption_on: Option<bool>,
    /// Output only. Gateway IP address.
    #[serde(rename="gatewayIpAddress")]
    
    pub gateway_ip_address: Option<String>,
    /// Output only. Network connection guid.
    
    pub guid: Option<String>,
    /// Output only. LAN IP address.
    #[serde(rename="lanIpAddress")]
    
    pub lan_ip_address: Option<String>,
    /// Output only. Receiving bit rate measured in Megabits per second.
    #[serde(rename="receivingBitRateMbps")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub receiving_bit_rate_mbps: Option<i64>,
    /// Output only. Time at which the network state was reported.
    #[serde(rename="reportTime")]
    
    pub report_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Frequency the report is sampled.
    #[serde(rename="sampleFrequency")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub sample_frequency: Option<client::chrono::Duration>,
    /// Output only. Signal strength for wireless networks measured in decibels.
    #[serde(rename="signalStrengthDbm")]
    
    pub signal_strength_dbm: Option<i32>,
    /// Output only. Transmission bit rate measured in Megabits per second.
    #[serde(rename="transmissionBitRateMbps")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub transmission_bit_rate_mbps: Option<i64>,
    /// Output only. Transmission power measured in decibels.
    #[serde(rename="transmissionPowerDbm")]
    
    pub transmission_power_dbm: Option<i32>,
    /// Output only. Wifi link quality. Value ranges from [0, 70]. 0 indicates no signal and 70 indicates a strong signal.
    #[serde(rename="wifiLinkQuality")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub wifi_link_quality: Option<i64>,
    /// Output only. Wifi power management enabled
    #[serde(rename="wifiPowerManagementEnabled")]
    
    pub wifi_power_management_enabled: Option<bool>,
}

impl client::Part for GoogleChromeManagementV1NetworkStatusReport {}


/// Contains information regarding the current OS update status. * This field is telemetry information and this will change over time as the device is utilized. * Data for this field is controlled via policy: [ReportDeviceOsUpdateStatus](https://chromeenterprise.google/policies/#ReportDeviceOsUpdateStatus) * Data Collection Frequency: Only at Upload * Default Data Reporting Frequency: 3 hours - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: No * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1OsUpdateStatus {
    /// Output only. Timestamp of the last reboot.
    #[serde(rename="lastRebootTime")]
    
    pub last_reboot_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Timestamp of the last update check.
    #[serde(rename="lastUpdateCheckTime")]
    
    pub last_update_check_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Timestamp of the last successful update.
    #[serde(rename="lastUpdateTime")]
    
    pub last_update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. New platform version of the os image being downloaded and applied. It is only set when update status is OS_IMAGE_DOWNLOAD_IN_PROGRESS or OS_UPDATE_NEED_REBOOT. Note this could be a dummy "0.0.0.0" for OS_UPDATE_NEED_REBOOT status for some edge cases, e.g. update engine is restarted without a reboot.
    #[serde(rename="newPlatformVersion")]
    
    pub new_platform_version: Option<String>,
    /// Output only. New requested platform version from the pending updated kiosk app.
    #[serde(rename="newRequestedPlatformVersion")]
    
    pub new_requested_platform_version: Option<String>,
    /// Output only. Current state of the os update.
    #[serde(rename="updateState")]
    
    pub update_state: Option<String>,
}

impl client::Part for GoogleChromeManagementV1OsUpdateStatus {}


/// Status data for storage. * This field is telemetry information and this will change over time as the device is utilized. * Data for this field is controlled via policy: [ReportDeviceStorageStatus](https://chromeenterprise.google/policies/#ReportDeviceStorageStatus) * Data Collection Frequency: Only at Upload * Default Data Reporting Frequency: 3 hours - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: No * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1StorageInfo {
    /// The available space for user data storage in the device in bytes.
    #[serde(rename="availableDiskBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub available_disk_bytes: Option<i64>,
    /// The total space for user data storage in the device in bytes.
    #[serde(rename="totalDiskBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_disk_bytes: Option<i64>,
    /// Information for disk volumes
    
    pub volume: Option<Vec<GoogleChromeManagementV1StorageInfoDiskVolume>>,
}

impl client::Part for GoogleChromeManagementV1StorageInfo {}


/// Information for disk volumes
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1StorageInfoDiskVolume {
    /// Free storage space in bytes.
    #[serde(rename="storageFreeBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub storage_free_bytes: Option<i64>,
    /// Total storage space in bytes.
    #[serde(rename="storageTotalBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub storage_total_bytes: Option<i64>,
    /// Disk volume id.
    #[serde(rename="volumeId")]
    
    pub volume_id: Option<String>,
}

impl client::Part for GoogleChromeManagementV1StorageInfoDiskVolume {}


/// Status data for storage. * This field is telemetry information and this will change over time as the device is utilized. * Data for this field is controlled via policy: [ReportDeviceStorageStatus](https://chromeenterprise.google/policies/#ReportDeviceStorageStatus) * Data Collection Frequency: Only at Upload * Default Data Reporting Frequency: 3 hours - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: No * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1StorageStatusReport {
    /// Output only. Reports on disk.
    
    pub disk: Option<Vec<GoogleChromeManagementV1DiskInfo>>,
    /// Output only. Timestamp of when the sample was collected on device
    #[serde(rename="reportTime")]
    
    pub report_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GoogleChromeManagementV1StorageStatusReport {}


/// `TelemetryAudioSevereUnderrunEvent` is triggered when a audio devices run out of buffer data for more than 5 seconds.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1TelemetryAudioSevereUnderrunEvent { _never_set: Option<bool> }

impl client::Part for GoogleChromeManagementV1TelemetryAudioSevereUnderrunEvent {}


/// Telemetry data collected from a managed device.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [telemetry devices get customers](CustomerTelemetryDeviceGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1TelemetryDevice {
    /// Output only. Audio reports collected periodically sorted in a decreasing order of report_time.
    #[serde(rename="audioStatusReport")]
    
    pub audio_status_report: Option<Vec<GoogleChromeManagementV1AudioStatusReport>>,
    /// Output only. Information on battery specs for the device.
    #[serde(rename="batteryInfo")]
    
    pub battery_info: Option<Vec<GoogleChromeManagementV1BatteryInfo>>,
    /// Output only. Battery reports collected periodically.
    #[serde(rename="batteryStatusReport")]
    
    pub battery_status_report: Option<Vec<GoogleChromeManagementV1BatteryStatusReport>>,
    /// Output only. Boot performance reports of the device.
    #[serde(rename="bootPerformanceReport")]
    
    pub boot_performance_report: Option<Vec<GoogleChromeManagementV1BootPerformanceReport>>,
    /// Output only. Information regarding CPU specs for the device.
    #[serde(rename="cpuInfo")]
    
    pub cpu_info: Option<Vec<GoogleChromeManagementV1CpuInfo>>,
    /// Output only. CPU status reports collected periodically sorted in a decreasing order of report_time.
    #[serde(rename="cpuStatusReport")]
    
    pub cpu_status_report: Option<Vec<GoogleChromeManagementV1CpuStatusReport>>,
    /// Output only. Google Workspace Customer whose enterprise enrolled the device.
    
    pub customer: Option<String>,
    /// Output only. The unique Directory API ID of the device. This value is the same as the Admin Console's Directory API ID in the ChromeOS Devices tab
    #[serde(rename="deviceId")]
    
    pub device_id: Option<String>,
    /// Output only. Contains information regarding Graphic peripherals for the device.
    #[serde(rename="graphicsInfo")]
    
    pub graphics_info: Option<GoogleChromeManagementV1GraphicsInfo>,
    /// Output only. Graphics reports collected periodically.
    #[serde(rename="graphicsStatusReport")]
    
    pub graphics_status_report: Option<Vec<GoogleChromeManagementV1GraphicsStatusReport>>,
    /// Output only. Information regarding memory specs for the device.
    #[serde(rename="memoryInfo")]
    
    pub memory_info: Option<GoogleChromeManagementV1MemoryInfo>,
    /// Output only. Memory status reports collected periodically sorted decreasing by report_time.
    #[serde(rename="memoryStatusReport")]
    
    pub memory_status_report: Option<Vec<GoogleChromeManagementV1MemoryStatusReport>>,
    /// Output only. Resource name of the device.
    
    pub name: Option<String>,
    /// Output only. Network diagnostics collected periodically.
    #[serde(rename="networkDiagnosticsReport")]
    
    pub network_diagnostics_report: Option<Vec<GoogleChromeManagementV1NetworkDiagnosticsReport>>,
    /// Output only. Network devices information.
    #[serde(rename="networkInfo")]
    
    pub network_info: Option<GoogleChromeManagementV1NetworkInfo>,
    /// Output only. Network specs collected periodically.
    #[serde(rename="networkStatusReport")]
    
    pub network_status_report: Option<Vec<GoogleChromeManagementV1NetworkStatusReport>>,
    /// Output only. Organization unit ID of the device.
    #[serde(rename="orgUnitId")]
    
    pub org_unit_id: Option<String>,
    /// Output only. Contains relevant information regarding ChromeOS update status.
    #[serde(rename="osUpdateStatus")]
    
    pub os_update_status: Option<Vec<GoogleChromeManagementV1OsUpdateStatus>>,
    /// Output only. Device serial number. This value is the same as the Admin Console's Serial Number in the ChromeOS Devices tab.
    #[serde(rename="serialNumber")]
    
    pub serial_number: Option<String>,
    /// Output only. Information of storage specs for the device.
    #[serde(rename="storageInfo")]
    
    pub storage_info: Option<GoogleChromeManagementV1StorageInfo>,
    /// Output only. Storage reports collected periodically.
    #[serde(rename="storageStatusReport")]
    
    pub storage_status_report: Option<Vec<GoogleChromeManagementV1StorageStatusReport>>,
    /// Output only. Information on Thunderbolt bus.
    #[serde(rename="thunderboltInfo")]
    
    pub thunderbolt_info: Option<Vec<GoogleChromeManagementV1ThunderboltInfo>>,
}

impl client::ResponseResult for GoogleChromeManagementV1TelemetryDevice {}


/// Information about a device associated with telemetry data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1TelemetryDeviceInfo {
    /// Output only. The unique Directory API ID of the device. This value is the same as the Admin Console's Directory API ID in the ChromeOS Devices tab.
    #[serde(rename="deviceId")]
    
    pub device_id: Option<String>,
    /// Output only. Organization unit ID of the device.
    #[serde(rename="orgUnitId")]
    
    pub org_unit_id: Option<String>,
}

impl client::Part for GoogleChromeManagementV1TelemetryDeviceInfo {}


/// Telemetry data reported by a managed device.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1TelemetryEvent {
    /// Output only. Payload for audio severe underrun event. Present only when the `event_type` field is `AUDIO_SEVERE_UNDERRUN`.
    #[serde(rename="audioSevereUnderrunEvent")]
    
    pub audio_severe_underrun_event: Option<GoogleChromeManagementV1TelemetryAudioSevereUnderrunEvent>,
    /// Output only. Information about the device associated with the event.
    
    pub device: Option<GoogleChromeManagementV1TelemetryDeviceInfo>,
    /// The event type of the current event.
    #[serde(rename="eventType")]
    
    pub event_type: Option<String>,
    /// Output only. Payload for HTTPS latency change event. Present only when `event_type` is `NETWORK_HTTPS_LATENCY_CHANGE`.
    #[serde(rename="httpsLatencyChangeEvent")]
    
    pub https_latency_change_event: Option<GoogleChromeManagementV1TelemetryHttpsLatencyChangeEvent>,
    /// Output only. Resource name of the event.
    
    pub name: Option<String>,
    /// Timestamp that represents when the event was reported.
    #[serde(rename="reportTime")]
    
    pub report_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Payload for usb peripherals event. Present only when the `event_type` field is either `USB_ADDED` or `USB_REMOVED`.
    #[serde(rename="usbPeripheralsEvent")]
    
    pub usb_peripherals_event: Option<GoogleChromeManagementV1TelemetryUsbPeripheralsEvent>,
    /// Output only. Information about the user associated with the event.
    
    pub user: Option<GoogleChromeManagementV1TelemetryUserInfo>,
}

impl client::Part for GoogleChromeManagementV1TelemetryEvent {}


/// Https latency routine is run periodically and `TelemetryHttpsLatencyChangeEvent` is triggered if a latency problem was detected or if the device has recovered from a latency problem..
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1TelemetryHttpsLatencyChangeEvent {
    /// HTTPS latency routine data that triggered the event.
    #[serde(rename="httpsLatencyRoutineData")]
    
    pub https_latency_routine_data: Option<GoogleChromeManagementV1HttpsLatencyRoutineData>,
    /// Current HTTPS latency state.
    #[serde(rename="httpsLatencyState")]
    
    pub https_latency_state: Option<String>,
}

impl client::Part for GoogleChromeManagementV1TelemetryHttpsLatencyChangeEvent {}


/// `TelemetryUsbPeripheralsEvent` is triggered USB devices are either added or removed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1TelemetryUsbPeripheralsEvent {
    /// List of usb devices that were either added or removed.
    #[serde(rename="usbPeripheralReport")]
    
    pub usb_peripheral_report: Option<Vec<GoogleChromeManagementV1UsbPeripheralReport>>,
}

impl client::Part for GoogleChromeManagementV1TelemetryUsbPeripheralsEvent {}


/// Information about a user associated with telemetry data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1TelemetryUserInfo {
    /// Output only. User's email.
    
    pub email: Option<String>,
    /// Output only. Organization unit ID of the user.
    #[serde(rename="orgUnitId")]
    
    pub org_unit_id: Option<String>,
}

impl client::Part for GoogleChromeManagementV1TelemetryUserInfo {}


/// Thunderbolt bus info. * This field provides device information, which is static and will not change over time. * Data for this field is controlled via policy: [ReportDeviceSecurityStatus](https://chromeenterprise.google/policies/#ReportDeviceSecurityStatus) * Data Collection Frequency: At device startup * Default Data Reporting Frequency: At device startup - Policy Controlled: No * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: Yes * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1ThunderboltInfo {
    /// Security level of the Thunderbolt bus.
    #[serde(rename="securityLevel")]
    
    pub security_level: Option<String>,
}

impl client::Part for GoogleChromeManagementV1ThunderboltInfo {}


/// Memory encryption information of a device. * This field provides device information, which is static and will not change over time. * Data for this field is controlled via policy: [ReportDeviceMemoryInfo](https://chromeenterprise.google/policies/#ReportDeviceMemoryInfo) * Data Collection Frequency: At device startup * Default Data Reporting Frequency: At device startup - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: Yes * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1TotalMemoryEncryptionInfo {
    /// Memory encryption algorithm.
    #[serde(rename="encryptionAlgorithm")]
    
    pub encryption_algorithm: Option<String>,
    /// The state of memory encryption on the device.
    #[serde(rename="encryptionState")]
    
    pub encryption_state: Option<String>,
    /// The length of the encryption keys.
    #[serde(rename="keyLength")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub key_length: Option<i64>,
    /// The maximum number of keys that can be used for encryption.
    #[serde(rename="maxKeys")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_keys: Option<i64>,
}

impl client::Part for GoogleChromeManagementV1TotalMemoryEncryptionInfo {}


/// USB connected peripheral report.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1UsbPeripheralReport {
    /// Output only. Categories the device belongs to https://www.usb.org/defined-class-codes
    
    pub categories: Option<Vec<String>>,
    /// Output only. Class ID https://www.usb.org/defined-class-codes
    #[serde(rename="classId")]
    
    pub class_id: Option<i32>,
    /// Output only. Firmware version
    #[serde(rename="firmwareVersion")]
    
    pub firmware_version: Option<String>,
    /// Output only. Device name, model name, or product name
    
    pub name: Option<String>,
    /// Output only. Product ID
    
    pub pid: Option<i32>,
    /// Output only. Subclass ID https://www.usb.org/defined-class-codes
    #[serde(rename="subclassId")]
    
    pub subclass_id: Option<i32>,
    /// Output only. Vendor name
    
    pub vendor: Option<String>,
    /// Output only. Vendor ID
    
    pub vid: Option<i32>,
}

impl client::Part for GoogleChromeManagementV1UsbPeripheralReport {}


/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleRpcStatus {
    /// The status code, which should be an enum value of google.rpc.Code.
    
    pub code: Option<i32>,
    /// A list of messages that carry the error details. There is a common set of message types for APIs to use.
    
    pub details: Option<Vec<HashMap<String, json::Value>>>,
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client.
    
    pub message: Option<String>,
}

impl client::Part for GoogleRpcStatus {}


/// Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values. * A month and day, with a zero year (for example, an anniversary). * A year on its own, with a zero month and a zero day. * A year and month, with a zero day (for example, a credit card expiration date). Related types: * google.type.TimeOfDay * google.type.DateTime * google.protobuf.Timestamp
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeDate {
    /// Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant.
    
    pub day: Option<i32>,
    /// Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day.
    
    pub month: Option<i32>,
    /// Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year.
    
    pub year: Option<i32>,
}

impl client::Part for GoogleTypeDate {}


